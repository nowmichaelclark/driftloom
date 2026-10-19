// Scheduling.
//
// setTimeout is far too jittery to place notes on, so it is only used to
// wake up and ask "what needs scheduling in the next fraction of a second?"
// The actual note times are absolute Web Audio clock times, which are
// sample-accurate. Standard two-clock pattern.

import { render, drift, LAYERS, characterOf, gracesOf, harmonyOf } from './generator.js';
import { Rng } from './rng.js';
import { Clock } from './clock.js';
import { draw, HOST_JITTER, HOST_ENSEMBLE, HOST_STRUM, HOST_GLASS } from './draw.js';

// Which stream of a loop's seed is its wander (what ears albums have always
// used for it).
const DRIFT_SALT = 0x5eed1e55;

// While you are looking at it, a short lookahead keeps mutes and re-rolls
// feeling immediate. Once the page is hidden the timer may be throttled to
// about one tick a second, so the queue has to be deep enough to cover the
// gap between wake-ups or the audio runs dry.
const LOOKAHEAD_VISIBLE = 0.3;
const LOOKAHEAD_HIDDEN = 3.0;
// On the Rust core, hidden, further still (queue item 39). A phone with its
// screen off can starve the page for seconds at a time -- a tester's Pixel
// paused from the lock screen seconds late, and glitched until it did --
// and notes the core already holds play on through that whatever the main
// thread is doing. Pause, Next and Previous cancel them (see `_cut`), so
// they still act as soon as the page runs.
const LOOKAHEAD_HIDDEN_CORE = 8.0;
// And on the core, visible, two seconds (queue item 40). The views that do
// the most on the main thread (the Library and Save / Share, the covers
// and lists drawn there) held the clock for up to a second on a slow phone
// (`node tools/views.mjs`), past the 0.3 s the queue held, and the music
// skipped. What used to need the short queue -- a mute, a re-roll, a tempo
// heard at once -- now cancels what is queued and decides again (`redecide`).
const LOOKAHEAD_VISIBLE_CORE = 2.0;
// A change is heard from this far ahead of the audio clock: room for the
// cancel to reach the core before the first note it drops would start.
const REDECIDE_LEAD = 0.06;
// On the core, never closer than the render-ahead plus this (queue item
// 41). The Pixel 10a's output runs about 0.45 s behind the clock and its
// core rendered past the page's clock before notes handed over 60-80 ms
// ahead reached it: they were late, started at once and bunched. The
// core says how far past the clock it has rendered when notes land
// (`CoreHost.ahead`), and a note is handed over no closer than that. A
// change on such a phone is heard that much later: its own output delay.
// The margin grows with it: the render-ahead swings by about a buffer's
// worth (`node tools/views.mjs --buffer 0.37 --delay 400`), so a quarter of
// it, and never less than this.
const AHEAD_MARGIN = 0.05;
const AHEAD_SHARE = 0.25;
// And the queue, visible, deep enough for the longest recent stall plus
// the render-ahead, with this to spare, up to the hidden depth. A stall
// is remembered and forgotten at this many seconds a second.
const STALL_MARGIN = 0.5;
const STALL_EASE = 0.02;
// Hidden, on the core, a deep output buffer is assumed at once (queue item
// 47). A tester's Pixel 9 rendered 6-21 ms past the clock on screen and
// 1.4 s past it at least once with the app minimized or the screen locked:
// Android moves a backgrounded app's audio to a deep, power-saving buffer,
// and the core renders over a second ahead in bursts. The render-ahead is
// only measured once notes have landed in it, so the notes decided near the
// clock just after the jump (Next or Previous from the lock screen, Play,
// catching up after a freeze) were planned with tens of ms to spare, landed
// inside it and were left out: the glitch. So, hidden (or frozen), the
// scheduler plans as if the render-ahead were at least this, or the worst
// this session or this device has seen (`deviceAhead`, kept in storage),
// whichever is more; never more than DEEP_MOST, so the hidden queue still
// has seconds to spare. Shown again, it plans from the measurement, as
// item 41 left it.
const DEEP_AHEAD = 1.5;
const DEEP_MOST = 4;
const TICK_VISIBLE = 50;
const TICK_HIDDEN = 250;

// A voice's graph is built only this many tick-gaps before its note, and
// never further ahead than the lookahead itself.
//
// Hidden, the scheduler still decides 3 s ahead, so a throttled timer
// cannot run the music dry. But building each voice as soon as it is
// decided wired its oscillators and filters into the mix up to 3 s early,
// and the browser renders a wired-up voice -- silence and all -- from the
// moment it is connected: a hidden tab cost the audio thread 20-100% more
// than a visible one (docs/perf-baseline.md). So a decided note waits in a
// queue and is built on the tick before it is needed, with room for three
// late ticks. The gap is measured, so if the timer does slow to a tick a
// second the window widens to match, back to 3 s. Visible, the lookahead
// is already shorter than the window and every note is built at once, as
// before.
//
// That reason is the JavaScript synth's, and only it keeps the window. On
// the Rust core a note handed over waits in its queue at no cost until it
// starts, and handing it over a few tenths of a second ahead meant that a
// locked phone holding the clock back for longer than that -- a throttled
// worker, a parked CPU, a frozen renderer -- made notes arrive late: the
// skips and glitches with the screen off (queue item 39). So on the core
// every note goes over as soon as it is decided, the whole lookahead ahead.
const BUILD_TICKS = 4;

// A chord's level, spread across its voicing so a five-note chord is not
// five times louder than a single note: this, over the root of the note
// count. Pads take the same figure. They reach pad(), which divides by the
// root itself, and for as long as only voice() got the 0.8 every pad chord
// sat 1.9 dB above the same chord on any other voice -- moogpad, the
// identical sound through voice(), measured exactly that much quieter.
// Applied here and not inside pad(), because voice() also reaches pad()
// for single notes, already spread.
const CHORD_SPREAD = 0.8;

// Grace notes, off the grid: each lasts GRACE seconds, and they run
// straight into the note, so a cut starts 30 ms ahead of it and a turn 60.
// Lighter than the note they decorate.
const GRACE = 0.03;
const GRACE_VEL = 0.6;
// The harmony line under the tune, a step back from it.
const HARMONY_VEL = 0.6;
// A nylon strum crosses the strings in 10-20 ms, low to high on the
// downstroke (on the beat) and high to low on the upstroke (off it), and
// the upstroke is the lighter of the two. Every other chord voice keeps the
// fixed 11 ms spread. It was 15-30 ms, heard as smeared.
// glass (queue item 59): how often a chord's first beat rings out, the
// register (E5-G6), the bell (the prototype's FM options and cost), and its
// level: the prototype's 0.62 at a chord of average velocity, following the
// chord's own velocity (the prototype scaled it by chords a bar instead).
const GLASS = {
  chance: 0.55, low: 76, high: 91, vel: 0.62, chordVel: 0.525,
  bell: { ratio: 3.5, index: 150, decay: 1.8, attack: 0.003, detune: 3, cost: 6 },
};

const STRUM = { min: 0.01, max: 0.02, up: 0.75 };

export class Engine {
  constructor(ctx, synth) {
    this.ctx = ctx;
    this.synth = synth;
    this.playing = false;
    this.spec = null;
    this.base = null; // the pattern as composed
    this.live = null; // the pattern as currently being played
    this.step = 0;
    // Never resets. Layers with different cycle lengths index off this, so
    // they keep drifting instead of resynchronising every time the pattern
    // wraps -- which is the whole point of the Eno character.
    this.absStep = 0;
    this.nextStepTime = 0;
    this.clock = new Clock(() => this._tick());
    this.driftOn = false;
    this.driftAmount = 1.0;
    this.loopCount = 0;
    this.onStep = null;
    this.onLoop = null;
    this.onTrackEnd = null;
    // Dropout counters. A stutter you cannot measure is a stutter you
    // cannot fix, and this runs on a phone that is not in front of me.
    this.lateTicks = 0;
    this.worstLateMs = 0;
    this.totalTicks = 0;
    // For the report per view (queue item 40): late ticks never cleared,
    // and the longest gap between ticks since it was last taken.
    this.lateTicksEver = 0;
    this.gapSince = 0;
    this.tailsDucked = false;
    // Notes decided but not yet built, in the order they were decided (see
    // BUILD_TICKS), and the measured gap between ticks.
    this.pending = [];
    this.tickGap = null;
    this.lastTick = null;
    // A track that ran its length asked for the next one (onTrackEnd): that
    // load follows on from what is decided, where any other cuts it short.
    this.handover = false;
    // What happened while the page was last hidden, for Diagnostics (queue
    // item 39): from when it was hidden until it was shown again.
    this.spell = null;
    // On the core, every step decided ahead, with where the scheduler stood
    // before it (queue item 40): a change that must be heard at once goes
    // back to the step sounding then and decides again. `floor`: notes asked
    // for before it were handed over already and are kept.
    this.decided = [];
    this.floor = -Infinity;
    this.redecides = 0;
    // The longest recent stall (seconds), easing back down (queue item 41),
    // counted only while the page is shown: hidden has its own queue.
    this.recentStall = 0;
    this.shownAt = 0;
    // Queue item 47: the page frozen (or being put away) while hidden, and
    // the worst render-ahead this device has seen before (seconds, from
    // storage). Both only plan the hidden notes; see DEEP_AHEAD.
    this.frozen = false;
    this.deviceAhead = 0;
    this.visualQueue = [];
    this.visualOffset = 0;   // optional manual trim, normally zero
    this.latency = null;     // measured, smoothed, seconds
  }

  // What the listener is hearing right now, on the audio clock.
  //
  // ctx.currentTime is the time of audio being handed to the output, not of
  // audio arriving at the ear; the gap is the output latency, which on
  // Android can exceed 300ms and varies by device, buffer size, and whether
  // headphones or Bluetooth are connected. Lighting the cursor at
  // currentTime therefore runs ahead of the music by an unknown amount, and
  // asking the user to dial that in by hand is not a fix.
  //
  // getOutputTimestamp exists for exactly this. It returns a correlated
  // pair: the audio-clock time of the sample being played at the output,
  // and the performance-clock time it happened. Interpolating from that pair
  // with performance.now() gives the true playback position, self-correcting
  // as latency changes underneath us -- which it does the moment Bluetooth
  // headphones connect.
  heardTime() {
    const ctx = this.ctx;
    let heard = null;
    if (typeof ctx.getOutputTimestamp === 'function') {
      const ts = ctx.getOutputTimestamp();
      if (ts && ts.contextTime > 0 && ts.performanceTime > 0) {
        const since = (performance.now() - ts.performanceTime) / 1000;
        // A stale timestamp would otherwise extrapolate without bound.
        heard = ts.contextTime + Math.max(0, Math.min(0.5, since));
      }
    }
    if (heard === null) {
      // No timestamp: fall back to the declared latency figures.
      const declared = ctx.outputLatency || ctx.baseLatency || 0;
      heard = ctx.currentTime - declared;
    }
    // Smooth the implied latency rather than the position, so the cursor
    // never jumps backwards when a measurement wobbles.
    const raw = Math.max(0, Math.min(0.6, ctx.currentTime - heard));
    this.latency = this.latency === null ? raw : this.latency * 0.92 + raw * 0.08;
    return ctx.currentTime - this.latency;
  }

  // The step that should be lit right now, or null if nothing has changed.
  visualStep() {
    const now = this.heardTime() + this.visualOffset;
    let found = null;
    while (this.visualQueue.length && this.visualQueue[0].time <= now) {
      found = this.visualQueue.shift().step;
    }
    return found;
  }

  load(spec, { keepPosition = false } = {}) {
    // Next, Previous or a loop picked while playing: what the old loop has
    // decided ahead does not play out first.
    if (this.playing && !keepPosition && !this.handover) this._cut();
    // A re-roll keeps its place: on the core, from the step sounding now,
    // decided again with the new layer, not after the queue has run out.
    const back = keepPosition ? this._rewindPoint() : null;
    this.handover = false;
    this.spec = spec;
    this.base = render(spec);
    this.live = this.driftOn ? this._wander(this.loopCount) : this.base;
    this.synth.setTone(spec.tone);
    const character = characterOf(spec);
    this.synth.setCharacterLevel(character.level ?? 1);
    this.pumpAmount = character.pump ?? 0;
    this.synth.setEchoTime((60 / spec.bpm) * 0.75);
    for (const [layer, muted] of Object.entries(spec.mutes || {})) {
      this.synth.setMute(layer, muted);
    }
    if (!keepPosition) {
      this.step = 0;
      this.absStep = 0;
      this.loopCount = 0;
      this.decided = [];
    } else {
      this.step = this.step % this.base.totalSteps;
    }
    if (back) this._redecideFrom(back, this.live);
    return this.base;
  }

  get stepDur() {
    return 60 / this.spec.bpm / 4;
  }

  get hidden() {
    return typeof document !== 'undefined' && document.hidden;
  }

  // Hidden, or frozen: where a deep output buffer is assumed (queue item 47).
  get away() {
    return this.frozen || this.hidden;
  }

  // The page is being frozen or put away (`freeze`, `pagehide`), or is back
  // (`resume`, `pageshow`).
  setFrozen(on) {
    this.frozen = !!on;
  }

  get lookahead() {
    if (!this.synth.queuesAhead) return this.hidden ? LOOKAHEAD_HIDDEN : LOOKAHEAD_VISIBLE;
    if (this.away) return LOOKAHEAD_HIDDEN_CORE;
    // Deep enough, too, that the step after the queue's end is not already
    // inside the deep buffer assumed the moment the page is hidden.
    const deep = Math.max(this.recentStall + this.ahead, this.deepAhead) + STALL_MARGIN;
    return Math.min(LOOKAHEAD_HIDDEN_CORE, Math.max(LOOKAHEAD_VISIBLE_CORE, deep));
  }

  // How far past the audio clock the core has already rendered by the time
  // a note handed over now reaches it (queue item 41); 0 on the JavaScript
  // synth, whose nodes start on the clock itself. Hidden or frozen, never
  // less than the deep buffer assumed (queue item 47).
  get ahead() {
    if (!this.synth.queuesAhead) return 0;
    const measured = finite(this.synth.renderAhead);
    return this.away ? Math.max(measured, this.deepAhead) : measured;
  }

  // The render-ahead assumed while hidden: DEEP_AHEAD, or the worst this
  // session (`worstAhead`) or this device has seen, up to DEEP_MOST.
  get deepAhead() {
    if (!this.synth.queuesAhead) return 0;
    const worst = Math.max(DEEP_AHEAD, finite(this.synth.worstAhead), finite(this.deviceAhead));
    return Math.min(DEEP_MOST, worst);
  }

  // What decided the notes handed over next, for Diagnostics' log of late
  // notes (queue item 47): the synth passes it to the core with each note.
  _why(cause) {
    if (this.synth.queuesAhead) this.synth.why = cause;
  }

  // The soonest a note decided now may start: `least` ahead of the clock,
  // and on the core never inside its render-ahead.
  _soonest(least) {
    const a = this.ahead;
    return this.ctx.currentTime + (a > 0 ? Math.max(least, a + Math.max(AHEAD_MARGIN, a * AHEAD_SHARE)) : least);
  }

  // Called when the page is shown or hidden, so the tick rate follows.
  retune() {
    if (this.hidden) this._beginSpell();
    else {
      this._endSpell();
      this.shownAt = performance.now();
    }
    if (this.playing) this.clock.start(this.hidden ? TICK_HIDDEN : TICK_VISIBLE);
  }

  // The engine's part of the "while hidden" report: the longest the clock
  // went without a tick, and the late ticks.
  _beginSpell() {
    if (this.spell && this.spell.open) return;
    this.spell = { open: true, from: performance.now(), to: null, worstGapMs: 0, lateTicks: 0, worstLateMs: 0, ticks: 0 };
  }

  _endSpell() {
    const s = this.spell;
    if (!s || !s.open) return;
    const now = performance.now();
    // A clock held back until the page came back is a gap too.
    if (this.playing && this.lastTick != null) s.worstGapMs = Math.max(s.worstGapMs, Math.round(now - this.lastTick));
    s.open = false;
    s.to = now;
  }

  // Everything decided ahead of the JavaScript synth's visible lookahead,
  // dropped: the notes not yet handed over, and on the core the ones that
  // have been but have not started. The next step is decided from now, as
  // on Play. The JavaScript synth, visible, has nothing that far ahead, so
  // nothing changes there.
  //
  // Hidden on the core, the new loop starts past the deep buffer assumed
  // (queue item 47), about two seconds on, and the old one plays on until
  // then rather than leaving that silent: only what it holds from there on
  // is dropped. A phone with a deep buffer has rendered most of that
  // already anyway.
  _cut() {
    const now = this.ctx.currentTime;
    if (!this.spec || this.nextStepTime <= now + LOOKAHEAD_VISIBLE + this.stepDur) return;
    this.pending = [];
    this.decided = [];
    this.floor = -Infinity;
    const start = this._soonest(0.08);
    if (this.synth.queuesAhead && this.away) {
      this.synth.cancelFrom(start);
      this.synth.restoreTails(start);
      this.visualQueue = this.visualQueue.filter((v) => v.time < start);
    } else {
      this.synth.cancelAhead(now);
      this.synth.restoreTails(now);
      this.visualQueue = [];
    }
    this.tailsDucked = false;
    this.nextStepTime = start;
    // The new loop starts from when the page has finished loading it, not
    // from now: drawing and rendering it takes a slow phone a good part of
    // a second, and its first notes would be late. And it is decided then,
    // not on the next tick, which hidden can be a quarter of a second
    // further on. (After the caller is done: a new loop resets its
    // position after loading it.)
    Promise.resolve().then(() => {
      if (!this.playing) return;
      this.nextStepTime = Math.max(start, this._soonest(0.08));
      this._tick('jump');
    });
  }

  // A change that must be heard at once, with notes handed to the core
  // seconds ahead (queue item 40): a mute, a re-roll, the tempo, wander.
  // What the core holds from just ahead of now is dropped, and the
  // scheduler goes back to the step sounding then and decides again from
  // there with the change in. `live`: the pattern to decide with (wander
  // switched off goes back to the loop as composed), else the one each step
  // had. On the JavaScript synth there is nothing that far ahead to redo;
  // false, and the change is heard as the next steps are decided, as ever.
  redecide(live = null) {
    const back = this._rewindPoint();
    if (!back) return false;
    this._redecideFrom(back, live);
    return true;
  }

  // Where to decide again from: the first step whose notes can reach the
  // time the change is heard from (a step's notes run up to a step after it,
  // with swing, strums and rolls).
  _rewindPoint() {
    if (!this.playing || !this.spec || this.handover || !this.synth.queuesAhead) return null;
    const from = this._soonest(REDECIDE_LEAD);
    if (this.nextStepTime <= from) return null;
    const early = from - this.stepDur - 0.08;
    const index = this.decided.findIndex((d) => d.time >= early);
    if (index < 0) return null;
    // Nothing to go back to before the first step kept: the loop in the
    // queue began there (a track that ran its length handed over to the
    // next, seconds ahead). The change is heard from where it starts, its
    // first notes (graces run 0.07 s ahead) and on, and the old loop's last
    // notes play as they were decided.
    const first = this.decided[index];
    const at = index === 0 && first.time > early ? Math.max(from, first.time - 0.08) : from;
    return { ...first, index, from: at };
  }

  _redecideFrom(back, live) {
    this.synth.cancelFrom(back.from);
    this.floor = back.from;
    this.step = back.step % this.base.totalSteps;
    this.absStep = back.absStep;
    this.loopCount = back.loopCount;
    this.live = live || back.live;
    this.tailsDucked = back.tailsDucked;
    this.nextStepTime = back.time;
    this.decided.length = back.index;
    this.visualQueue = this.visualQueue.filter((v) => v.time < back.time);
    this.redecides++;
    this._decide('change');
  }

  start() {
    if (this.playing || !this.spec) return;
    this.playing = true;
    this.decided = [];
    this.floor = -Infinity;
    this.synth.unsilence();
    this.nextStepTime = this._soonest(0.08);
    this.clock.start(this.hidden ? TICK_HIDDEN : TICK_VISIBLE);
    this._tick('play');
  }

  stop() {
    this.playing = false;
    this.handover = false;
    this.clock.stop();
    this.pending = [];
    this.decided = [];
    // Handed to the core ahead and not started: a Play soon after must not
    // hear them on top of its own.
    this.synth.cancelAhead();
    this.lastTick = null;
    this.visualQueue = [];
    this.synth.silence();
    if (this.tailsDucked) {
      this.synth.restoreTails(this.ctx.currentTime);
      this.tailsDucked = false;
    }
  }

  report() {
    return {
      clock: this.clock.usingWorker ? 'worker' : 'timer',
      hidden: this.hidden,
      lookahead: +this.lookahead.toFixed(2),
      redecides: this.redecides,
      ticks: this.totalTicks,
      lateTicks: this.lateTicks,
      worstLateMs: this.worstLateMs,
      ctxState: this.ctx.state,
      sampleRate: this.ctx.sampleRate,
      baseLatency: this.ctx.baseLatency ? +this.ctx.baseLatency.toFixed(4) : '-',
      outputLatency: this.ctx.outputLatency ? +this.ctx.outputLatency.toFixed(4) : '-',
      measuredLatencyMs: this.latency === null ? '-' : Math.round(this.latency * 1000),
      timestampApi: typeof this.ctx.getOutputTimestamp === 'function' ? 'yes' : 'no',
      ...this._aheadReport(),
    };
  }

  // On the core (queue item 41): the render-ahead planned from now, as
  // measured, and the most it has been; the least time any note had to spare as it reached
  // the core (negative: one was late); how far the core's count has run
  // past the context's clock; and the longest recent stall the queue is
  // sized for.
  _aheadReport() {
    const c = this.synth.queuesAhead ? this.synth.core : null;
    if (!c || typeof c !== 'object') return { renderAheadMs: '-' };
    const ms = (x) => (Number.isFinite(x) ? Math.round(x * 1000) : '-');
    return {
      // Planned from (at least a render callback), and as measured.
      renderAheadMs: `${ms(this.ahead)} (measured ${c.ahead == null ? '- no notes yet' : ms(c.ahead)})`,
      worstRenderAheadMs: ms(c.worstAhead),
      // Queue item 47: what is planned from while hidden, and this device's
      // worst before this session.
      hiddenRenderAheadMs: `${ms(this.deepAhead)} (assumed ${ms(DEEP_AHEAD)}, this device before ${ms(finite(this.deviceAhead))})`,
      leastSpareMs: ms(c.earliest),
      coreClockLeadMs: ms(c.clockLead),
      recentStallMs: ms(this.recentStall),
    };
  }

  // The longest gap between ticks since the last call, in ms; a clock held
  // back right now counts up to now.
  takeGap() {
    const now = performance.now();
    const open = this.playing && this.lastTick != null ? now - this.lastTick : 0;
    const worst = Math.round(Math.max(this.gapSince, open));
    this.gapSince = 0;
    return worst;
  }

  clearMetrics() {
    this.lateTicks = 0;
    this.worstLateMs = 0;
    this.totalTicks = 0;
  }

  reset() {
    this.step = 0;
    this.absStep = 0;
    this.loopCount = 0;
    this.synth.setTone(this.spec.tone);
  }

  // How far ahead of its note a voice is built.
  get buildAhead() {
    if (!this.tickGap) return this.lookahead;
    return Math.min(this.lookahead, Math.max(0.3, this.tickGap * BUILD_TICKS));
  }

  // Build a voice now if its note is close enough, otherwise queue it.
  // Visible, or on the core, every voice is built at once; whatever a
  // hidden spell left queued goes first, so the order holds.
  _play(time, build, key = null) {
    // Deciding again after a change: what is asked for before the change is
    // heard was handed over already and plays as it was.
    if (time < this.floor) return;
    // The note's place in its loop goes to the synth as it is built, and
    // only then (queue item 44): the core draws everything the note leaves
    // to chance from it.
    if (key) {
      const play = build;
      build = () => {
        this.synth.key = key;
        try {
          play();
        } finally {
          this.synth.key = null;
        }
      };
    }
    if (!this.hidden || !this.pending || this.synth.queuesAhead) {
      if (this.pending && this.pending.length) for (const p of this.pending.splice(0)) p.build();
      build();
    } else if (this.pending.length || time > this.ctx.currentTime + this.buildAhead) {
      this.pending.push({ time, build });
    } else {
      build();
    }
  }

  // Build every queued voice whose note is within reach, in the order the
  // notes were decided, so the voice budget sees them in the same order.
  _flush() {
    if (!this.pending || !this.pending.length) return;
    const horizon = this.hidden ? this.ctx.currentTime + this.buildAhead : Infinity;
    let i = 0;
    while (i < this.pending.length && this.pending[i].time <= horizon) i++;
    for (const { build } of this.pending.splice(0, i)) build();
  }

  // `cause`: what asked for the notes decided now (Play, a jump), else the
  // steady lookahead.
  _tick(cause = 'lookahead') {
    if (!this.playing) return;
    this.totalTicks++;
    const now = performance.now();
    if (this.lastTick != null) {
      const gap = (now - this.lastTick) / 1000;
      // Quick to widen when ticks slow down, slow to narrow again.
      this.tickGap = this.tickGap == null || gap > this.tickGap ? gap : this.tickGap * 0.9 + gap * 0.1;
    }
    const spell = this.spell && this.spell.open ? this.spell : null;
    if (spell) {
      spell.ticks++;
      if (this.lastTick != null) spell.worstGapMs = Math.max(spell.worstGapMs, Math.round(now - this.lastTick));
    }
    if (this.lastTick != null) {
      this.gapSince = Math.max(this.gapSince, now - this.lastTick);
      // The longest recent stall, for the depth of the queue: up at once,
      // down slowly.
      const gap = (now - this.lastTick) / 1000;
      const shown = !this.hidden && this.lastTick >= this.shownAt;
      this.recentStall = Math.max(shown ? gap : 0, this.recentStall - STALL_EASE * gap);
    }
    this.lastTick = now;
    this._flush();
    // If the next step was already due before we woke up, the queue ran dry
    // and something audible was missed. On the core it is due once it is
    // inside the render-ahead: it could only arrive late.
    const ahead = this.ahead;
    const behind = this.ctx.currentTime + ahead - this.nextStepTime;
    if (behind > 0) {
      this.lateTicks++;
      this.lateTicksEver++;
      this.worstLateMs = Math.max(this.worstLateMs, Math.round(behind * 1000));
      if (spell) {
        spell.lateTicks++;
        spell.worstLateMs = Math.max(spell.worstLateMs, Math.round(behind * 1000));
      }
      // Do not try to catch up by cramming the missed steps in at once;
      // that turns a gap into a burst. Skip to now and carry on. On the
      // core every one of them would be late (or left out), so it always
      // skips, to the soonest a note can still be on time.
      if (ahead > 0 || behind > 0.25) {
        this.nextStepTime = this._soonest(0.02);
        if (cause === 'lookahead') cause = 'catch-up';
      }
    }
    this._decide(cause);
  }

  // Every step up to the lookahead, and on the core where each began.
  _decide(cause = 'lookahead') {
    const now = this.ctx.currentTime;
    const horizon = now + this.lookahead;
    const keep = this.synth.queuesAhead;
    if (!keep) this.decided.length = 0;
    // A second of what has sounded is plenty to go back to.
    let old = 0;
    while (old < this.decided.length && this.decided[old].time < now - 1) old++;
    if (old) this.decided.splice(0, old);
    let guard = 0;
    while (this.nextStepTime < horizon && guard++ < 256) {
      if (keep) {
        this.decided.push({
          time: this.nextStepTime, step: this.step, absStep: this.absStep, loopCount: this.loopCount,
          live: this.live, tailsDucked: this.tailsDucked,
        });
        // A loop's first step, or a pass's, says so in the log.
        const top = this.absStep === 0 ? 'new loop' : this.step === 0 ? 'new pass' : null;
        this._why(top ? `${cause}, ${top}` : cause);
      }
      this._scheduleStep(this.step, this.nextStepTime);
      this._advance();
    }
    if (keep) this._why('lookahead');
    this._flush();
  }

  // How the loop wanders on pass `pass` ("Let the loop wander"): a variation
  // of the stored pattern drawn from the loop's seed and the pass number, so
  // the same code wanders the same way every time it is played (queue item
  // 44), and any pass can be reached without playing the ones before it.
  _wander(pass) {
    const seed = (((this.spec.seed ^ DRIFT_SALT) >>> 0) + Math.imul(pass, 0x9e3779b1)) >>> 0;
    return drift(this.base, new Rng(seed || 1), this.driftAmount);
  }

  _advance() {
    this.nextStepTime += this.stepDur;
    this.step++;
    this.absStep++;
    if (this.step >= this.live.totalSteps) {
      this.step = 0;
      this.loopCount++;
      // Fresh variation at the top of each pass. The base pattern is never
      // touched, so the piece always returns to the version you saved.
      this.live = this.driftOn ? this._wander(this.loopCount) : this.base;
      if (this.onLoop) this.onLoop(this.loopCount);
      // A track with a set length hands over once it has run its passes.
      const limit = this.spec && this.spec.playFor;
      if (limit && this.loopCount >= limit && this.onTrackEnd) {
        this.handover = true;
        this.onTrackEnd();
      }
    }
  }

  // glass (queue item 59): on a chord's first beat, 55% of the time, two or
  // three of its notes ring out in E5-G6 as a soft FM bell (ratio 3.5), a
  // dotted eighth apart, into the texture channel. Every decision is a draw
  // from where the chord sits (js/draw.js), so a step decided again sounds
  // the same and each pass rings its own way, as the prototype's did. Its
  // level follows the chord it comes from (GLASS.vel).
  _glass(p, s, t, sd, key) {
    let ord = 0;
    for (const e of p.tracks.chords) {
      if (e.step !== s || !e.vel || !e.notes || !e.notes.length) continue;
      const at = { ...key, ord: ord++ };
      if (draw(at, 'texture', HOST_GLASS, 0) >= GLASS.chance) continue;
      const count = draw(at, 'texture', HOST_GLASS, 1) < 0.5 ? 3 : 2;
      const pcs = e.notes.slice().sort((a, b) => a - b);
      const first = Math.floor(draw(at, 'texture', HOST_GLASS, 2) * pcs.length);
      for (let i = 0; i < count; i++) {
        let m = pcs[(first + i) % pcs.length];
        while (m < GLASS.low) m += 12;
        while (m > GLASS.high) m -= 12;
        const when = t + i * 3 * sd;
        const vel = GLASS.vel * (e.vel / GLASS.chordVel) * (1 - i * 0.18);
        this._play(when, () => this.synth.fm(m, when, Math.max(0.6, 2.5 * sd), vel, { ...GLASS.bell, out: this.synth.channels.texture.gain }));
      }
    }
  }

  // True when every layer is scheduled out for the bar containing this step
  // by the loop's entry schedule. (Drift's full stop, the other way a bar
  // fell silent, became a floor drop in queue item 58: the loop plays on.)
  _isSilentBar(absStep) {
    const p = this.live;
    if (!p) return false;
    const spb = p.stepsPerBar || 16;
    const bar = Math.floor((absStep % p.totalSteps) / spb);
    if (p.silentBars && p.silentBars.indexOf(bar) !== -1) return true;
    if (!p.form) return false;
    for (const layer of LAYERS) {
      const sched = p.form[layer];
      if (!sched) return false; // a freshly rolled layer plays throughout
      const cycle = (p.cycles && p.cycles[layer]) || p.totalSteps;
      const b = Math.floor((absStep % cycle) / spb) % sched.length;
      if (sched[b]) return false;
    }
    return true;
  }

  _scheduleStep(step, time) {
    const p = this.live;
    const cyc = p.cycles || {};
    const abs = this.absStep;

    // At each bar line, decide whether the tails should still be ringing.
    const spbNow = p.stepsPerBar || 16;
    if (abs % spbNow === 0) {
      const quiet = this._isSilentBar(abs);
      // Deciding again after a change, a bar line before it is heard was
      // handed over already, as it was.
      const kept = time < this.floor;
      if (quiet && !this.tailsDucked) {
        if (!kept) this.synth.fadeTails(time);
        this.tailsDucked = true;
      } else if (!quiet && this.tailsDucked) {
        // Restore just before the bar starts, so the first note is not dry.
        if (!kept) this.synth.restoreTails(Math.max(this.ctx.currentTime, time - 0.12));
        this.tailsDucked = false;
      }
    }
    // Where each layer is inside its own loop.
    const at = (layer) => {
      const len = cyc[layer] || p.totalSteps;
      return len === p.totalSteps ? step : ((abs % len) + len) % len;
    };
    const swing = step % 2 === 1 ? this.spec.swing * this.stepDur : 0;
    const t = time + swing;
    const sd = this.stepDur;
    const mutes = this.spec.mutes || {};
    // Where each note sits in its loop (core/src/rng.rs): the loop's seed,
    // the pass, the step, and the note's place among its layer's notes at
    // this step. Counted here, as the notes are decided, so it does not
    // depend on what the voice budget turns away, on a layer being muted,
    // or on when the voice is built.
    const seed = this.spec.seed >>> 0;
    const pass = this.loopCount;
    const placed = { drums: 0, bass: 0, chords: 0, melody: 0, texture: 0 };
    const keyOf = (layer) => ({ seed, pass, step, ord: placed[layer]++ });

    if (!mutes.drums) {
      const s = at('drums');
      const pump = this.pumpAmount || 0;
      for (const e of p.tracks.drums) {
        if (e.step !== s || !e.vel) continue;
        // Rolls sit between the steps, so they carry a fractional offset and
        // skip the humanising jitter that would smear them.
        const micro = e.micro ? e.micro * sd : 0;
        const key = keyOf('drums');
        const jitter = e.roll ? 0 : (draw(key, 'drums', HOST_JITTER) - 0.5) * 0.008;
        const at = t + micro + jitter;
        this._play(at, () => this.synth.drum(e.inst, at, e.vel), key);
        if (pump && (e.inst === 'kick' || e.inst === 'softkick')) this._play(at, () => this.synth.duck(t + micro, pump * e.vel));
      }
    }
    if (!mutes.bass) {
      const s = at('bass');
      for (const e of p.tracks.bass) {
        if (e.step !== s || !e.vel) continue;
        this._play(t, () => this.synth.bass(e.midi, t, e.dur * sd, e.vel, e.glide, e.voice, e.chug), keyOf('bass'));
      }
    }
    // An ensemble does not attack together. The composer's detune keeps
    // the two layers apart in pitch; this keeps them apart in time, and it
    // has to be drawn per note rather than written into the spec, because
    // a layer late by the same three milliseconds every bar has simply
    // been delayed. Drawn once per layer per step so the two scatter
    // against each other as well as against the grid, and symmetric like
    // the drum jitter above so the music does not walk late.
    const ensemble = (layer) => (p.meta && p.meta.choir ? (draw({ seed, pass, step, ord: 0 }, layer, HOST_ENSEMBLE) - 0.5) * 0.016 : 0);

    if (!mutes.chords) {
      const s = at('chords');
      const slip = ensemble('chords');
      for (const e of p.tracks.chords) {
        if (e.step !== s || !e.vel) continue;
        if (e.voice === 'pad') {
          this._play(t, () => this.synth.pad(e.notes, t, e.dur * sd, e.vel * CHORD_SPREAD), keyOf('chords'));
        } else {
          const spread = CHORD_SPREAD / Math.sqrt(e.notes.length);
          if (e.strum) {
            const span = STRUM.min + draw(keyOf('chords'), 'chords', HOST_STRUM) * (STRUM.max - STRUM.min);
            const strings = e.strum === 'down' ? e.notes : e.notes.slice().reverse();
            const weight = e.strum === 'down' ? 1 : STRUM.up;
            // The hand that strikes the chord stops the last one's strings.
            this._play(t + slip, () => this.synth.damp(this.synth.channels.chords.gain, t + slip));
            strings.forEach((n, i) => {
              const at = strings.length > 1 ? span * i / (strings.length - 1) : 0;
              this._play(t + slip + at, () => this.synth.voice(e.voice, n, t + slip + at, e.dur * sd, e.vel * spread * weight,
                this.synth.channels.chords.gain, { vowel: e.vowel, detune: e.detune, strum: true }), keyOf('chords'));
            });
            continue;
          }
          e.notes.forEach((n, i) => {
            // Spread the notes of a chord by a few milliseconds so it
            // sounds like fingers rather than a switch closing.
            this._play(t + slip + i * 0.011, () => this.synth.voice(e.voice, n, t + slip + i * 0.011, e.dur * sd, e.vel * spread,
              this.synth.channels.chords.gain, { vowel: e.vowel, detune: e.detune }), keyOf('chords'));
          });
        }
      }
      if (p.choices && p.choices.glass === 'on') this._glass(p, s, t, sd, { seed, pass, step });
    }
    if (!mutes.melody) {
      const s = at('melody');
      const slip = ensemble('melody');
      for (const e of p.tracks.melody) {
        if (e.step !== s || !e.vel) continue;
        const out = this.synth.channels.melody.gain;
        this._play(t + slip, () => this.synth.voice(e.voice, e.midi, t + slip, e.dur * sd, e.vel, out,
          { glide: e.glide, vowel: e.vowel, detune: e.detune, prev: e.prev }), keyOf('melody'));
        // Asked for after the note, so where the budget is short it is the
        // decoration that yields, never the tune.
        const under = harmonyOf(e, p.spec);
        if (under != null) {
          this._play(t + slip, () => this.synth.voice(e.voice, under, t + slip, e.dur * sd, e.vel * HARMONY_VEL, out,
            { vowel: e.vowel, detune: e.detune }), keyOf('melody'));
        }
        const graces = gracesOf(e, p.spec);
        graces.forEach((g, i) => {
          const at = t + slip - GRACE * (graces.length - i);
          this._play(at, () => this.synth.voice(e.voice, g, Math.max(this.ctx.currentTime, at), GRACE, e.vel * GRACE_VEL, out,
            { vowel: e.vowel, detune: e.detune }), keyOf('melody'));
        });
      }
    }
    if (!mutes.texture) {
      const s = at('texture');
      for (const e of p.tracks.texture) {
        if (e.step !== s || !e.vel) continue;
        this._play(t, () => this.synth.texture(e.kind, e.notes, t, e.dur * sd, e.vel, { band: e.band }), keyOf('texture'));
      }
    }

    // The cursor used to get a setTimeout per step. Timer callbacks drift,
    // arrive in bursts and are the first thing the main thread drops under
    // load, so the playhead wandered away from the music. Record when each
    // step is due and let a frame loop read it off the audio clock instead.
    this.visualQueue.push({ step, time });
    if (this.visualQueue.length > 512) this.visualQueue.shift();
  }
}

function finite(x) {
  return typeof x === 'number' && Number.isFinite(x) ? x : 0;
}
