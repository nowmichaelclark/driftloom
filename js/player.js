// The app's engine on the core's player (queue item 94b): the default since
// then. The core decides every note itself, on the audio thread
// (core/src/player/, through js/dlplayer.wasm in js/worklet.js); this engine
// keeps no lookahead and decides no notes. It sends the core commands (a
// loop's seed, its request and a code's edits; a change, a re-roll, the
// controls) and draws what the core reports as events: the cursor from each
// step, the layers' grid of each pass, the pass and the track's end. The page
// is free to stall, hide or freeze: the worklet has what it needs.
//
// The picks (queue item 95) are the core's too: the New loops panel's
// request is sent as a command (`setPicks`) and held there, and every new
// loop is one the core makes itself with them, at Next (`next`) and when the
// length slider's time runs out with nothing of the page's armed: the page
// learns its seed from the core's events (`onOwn`) and draws nothing.
//
// It has the shape of js/engine.js's Engine (the page's engine, which
// `?engine=page` keeps for comparison until queue item 97), so js/main.js
// drives either: load, start, stop, the controls (setMute, setTempo,
// setTone, setWander, trackChanged), the cursor (visualStep), Now Playing's
// clock (audibleSteps, endStep, roundSteps) and Diagnostics (report).
//
// What it still asks of the JavaScript generator is for the page to show,
// never what plays: `render(spec)` for the info card and the first grid,
// until the core's own arrives. The spec is the page's record of a loop
// (history, saves, share codes); what the core plays is that spec's seed,
// request and edits, which make the same spec in the Rust generator (the
// generation goldens, and test/player.test.mjs).

import { render, LAYERS, requestOf, readRequest, RequestError, keyMask, scaleMask, styleMask, voiceMask, layerState, pieceOf, loopBars } from './generator.js';
import { VIBE_IDS } from './vibes.js';
import { pinNumbers } from './pins.js';

// Where "Let the loop wander" stops repeating itself (queue item 92; Mikey,
// 2026-10-18: all the way to the right it should feel experimental, not
// repeatable). From this amount up (the slider runs 0 to 2.4) each play of
// a loop draws its own salt for the wander's stream (`wanderSalt`), so the
// same code wanders another way every time it is played; below it, every
// play wanders as every play of that code always has. The conservative
// first point: only the slider's top sixth. For Mikey's ears.
export const UNREPEATABLE_FROM = 2.0;

// A new salt for a play (never 0, which is no salt).
export function freshSalt() {
  try {
    const x = new Uint32Array(1);
    globalThis.crypto.getRandomValues(x);
    if (x[0]) return x[0];
  } catch { /* no crypto here */ }
  return (Math.floor(Math.random() * 0xffffffff) >>> 0) || 1;
}

// The moments a change waits for, as the core numbers them (`Moment::ALL`).
export const MOMENT = { now: 0, beat: 1, bar: 2, phrase: 3, pass: 4, end: 5 };
// The kinds of event the player reports (`Kind::ALL`).
export const KIND = { step: 0, pass: 1, section: 2, phrase: 3, loop: 4, revised: 5, end: 6, own: 7 };
// How long the old loop takes to fall to nothing at Next, Previous or a loop
// picked while one plays, in seconds: the core's own crossfade for a change
// (`player::change::CROSSFADE`). For Mikey's ears.
export const CHANGE_FADE = 1.0;
// And when the length slider ends a track inside a pass, at a phrase line
// (queue item 88b's END_FADE; at a pass's end the old loop rings out).
export const END_FADE = 1.0;

// The cursor's and the grids' queues, at most.
const QUEUE_MOST = 512;
const GRIDS_MOST = 16;

// What the core is asked for a spec's loop (`CoreHost.playLoad` and the
// rest): its request (`requestOf`: the vibes with the weights it drew, so the
// loop a selection of five or fewer made, the tempo, pins, theme, length,
// keys, modes, styles, voices and layers), and a code's edits (queue item
// 94a): every layer's seed, with whether it was re-rolled, the tone and the
// tempo. The edits are sent whole, whether or not they differ from what the
// seed draws: an edit to the value drawn makes the same loop
// (test/player.test.mjs holds it), and the page need not draw the loop to
// tell.
export function askOf(spec) {
  const asked = requestOf(spec);
  const vibes = Object.entries(asked.vibes || {}).map(([id, w]) => [VIBE_IDS.indexOf(id), w]).filter(([i]) => i >= 0);
  const seeds = spec.layerSeeds
    ? LAYERS.map((l, i) => [i, spec.layerSeeds[l] >>> 0, !!(spec.forceLayers && spec.forceLayers[l])]).filter(([, s]) => Number.isFinite(s))
    : [];
  const tone = spec.tone ? [spec.tone.warmth, spec.tone.space, spec.tone.wobble].map((v) => (Number.isFinite(v) ? v : null)) : null;
  return {
    vibes,
    tempo: asked.tempo,
    pins: pinNumbers(asked.pins),
    theme: asked.theme,
    length: asked.length,
    keys: keyMask(asked.key),
    scales: scaleMask(asked.scale),
    styles: styleMask(asked.style),
    voices: LAYERS.map((l) => voiceMask(asked.voices, l)),
    layers: LAYERS.map((l) => layerState(asked.layers, l)),
    seeds,
    tone: tone && tone.every((v) => v != null) ? tone : null,
    bpm: Number.isFinite(spec.bpm) ? spec.bpm : null,
  };
}

// The New loops panel's request as the core takes it (queue item 95,
// `Player::set_picks`): the request read (`readRequest`, the reader both
// generators share) and put in the core's numbers, the vibes a listener's
// selection, each with no weight. A request that can't be read is sent as
// its vibes alone, as the page's own draw kept the moods of one it could not
// meet: { ask, vibesOnly, why }.
export function picksAsk(request) {
  const out = (asked) => ({
    vibes: (asked.vibes || []).map(([id, w]) => [VIBE_IDS.indexOf(id), w]).filter(([i]) => i >= 0),
    tempo: asked.tempo,
    pins: pinNumbers(asked.pins),
    theme: asked.theme,
    length: asked.length,
    keys: keyMask(asked.key),
    scales: scaleMask(asked.scale),
    styles: styleMask(asked.style),
    voices: LAYERS.map((l) => voiceMask(asked.voices, l)),
    layers: LAYERS.map((l) => layerState(asked.layers, l)),
  });
  try {
    return { ask: out(readRequest(request)), vibesOnly: false, why: null };
  } catch (e) {
    if (!(e instanceof RequestError)) throw e;
    const vibes = request && Array.isArray(request.vibes) ? request.vibes : [];
    return { ask: out(readRequest(vibes)), vibesOnly: true, why: e.message };
  }
}

// The length slider's track as the core takes it (`Player::set_track`):
// [kind, count], 0 on and on, 1 whole passes (`spec.playFor`), 2 bars to the
// next phrase line (`spec.playBars`).
export function trackOf(spec) {
  if (spec && spec.playFor) return [1, spec.playFor];
  if (spec && spec.playBars) return [2, spec.playBars];
  return [0, 0];
}

export class PlayerEngine {
  constructor(ctx, synth) {
    this.ctx = ctx;
    this.synth = synth;
    this.spec = null;
    // The loop as composed (the page's render, for the card and the first
    // grid) and the grid shown: the pass heard, as the core reports it.
    this.base = null;
    this.live = null;
    this.playing = false;
    this.driftOn = true;
    this.driftAmount = 1;
    // This play's wander salt (queue item 92).
    this.salt = 0;
    this.onStep = null;
    this.onLoop = null;
    this.onTrackEnd = null;
    // A loop the core made itself has started (queue item 95): (seed, tag),
    // the picks numbered `tag` its request. And the core's answer to the
    // picks: ({ tag, met, why }).
    this.onOwn = null;
    this.onPicks = null;
    // The picks sent ({ ask, tag }), the core they were sent to, the core
    // given a seed to draw from, an own loop asked while no core could take
    // it, and the picks' number of an own loop whose Loop event comes next.
    this.picks = null;
    this.picksFor = null;
    this.drawsFor = null;
    this.wantOwn = false;
    this.ownTag = null;
    this.visualQueue = [];
    this.visualOffset = 0;
    this.latency = null;
    // Kept for main.js's reports, which read them from either engine: the
    // page's clock and its spells while hidden are the page engine's.
    this.deviceAhead = 0;
    this.lateTicksEver = 0;
    this.spell = null;
    // What Play must ask the core: 'load' (a loop not yet given it),
    // 'resumed' (the same loop again on a new core: the quality switch), or
    // null (the loop it has stopped, to go on with).
    this.needs = 'load';
    // The last step decided (an event: { time, number, step }), the last
    // heard ({ time, step, abs }), the grids waiting for their time, the
    // passes heard, and the track's end (a step from the load, or null) as
    // the core says, with the step the page counts it from.
    this.decided = null;
    this.heard = null;
    this.gridQueue = [];
    this.loopCount = 0;
    this.trackEnd = null;
    this.countFrom = 0;
    // The track last sent ([kind, count]), the loop that follows when it
    // ends (a spec, armed in the core as a change at its end), and the seed
    // the core took there, until the page catches up with it.
    this.track = null;
    this.armed = null;
    this.taken = null;
    // What the core has reported, for Diagnostics.
    this.loops = 0;
    this._hooked = null;
    this._hook();
  }

  // The core's node, when it is the one playing.
  _core() {
    const c = this.synth.core;
    return c && !c.failed && c.player ? c : null;
  }

  // Listen to the core's events (once a node: a core that arrives later is
  // hooked as it does).
  _hook() {
    const c = this._core();
    if (!c || this._hooked === c) return;
    this._hooked = c;
    c.onevents = (m) => this._receive(m);
    c.onpicks = (m) => { if (this.onPicks) this.onPicks(m); };
  }

  // Every new loop is the core's own on this engine (queue item 95).
  get ownLoops() {
    return true;
  }

  // The New loops panel's request (`picksAsk(request).ask`), numbered `tag`:
  // held by the core for the loops it makes itself. Sent again to a new core
  // (a reload, the quality switch).
  setPicks(ask, tag) {
    this.picks = { ask, tag };
    this.picksFor = null;
    this._sendPicks();
  }

  _sendPicks() {
    const core = this._core();
    if (!core || !this.picks || this.picksFor === core) return;
    if (this.drawsFor !== core) {
      // A seed to draw from for this core, fresh each time one starts: the
      // core's own loops are a pure function of it and the picks.
      core.playDraws(freshSalt());
      this.drawsFor = core;
    }
    core.playPicks(this.picks.ask, this.picks.tag);
    this.picksFor = core;
  }

  // Next at the end of the history: a new loop of the core's own, with the
  // picks, crossfading as any change does. The page hears which in `onOwn`.
  // From Pause it starts playing; before the core has arrived, it waits.
  next() {
    this.salt = freshSalt();
    this.wantOwn = true;
    if (!this.playing) {
      this.playing = true;
      this.synth.unsilence();
      this._begin();
      return;
    }
    this._askOwn();
  }

  _askOwn() {
    const core = this._core();
    if (!core || !this.wantOwn) return;
    this._sendPicks();
    this.wantOwn = false;
    this.taken = null;
    core.playWander(this.driftOn ? this.driftAmount : 0, this.wanderSalt);
    core.playNext(MOMENT.now, CHANGE_FADE);
    this.visualQueue = [];
    this.gridQueue = [];
    this.countFrom = 0;
  }

  // The core has arrived (after Play, on a slow first visit): it is given the
  // loop now.
  coreArrived() {
    this._hook();
    if (this.playing) this._begin();
  }

  get stepDur() {
    return 60 / this.spec.bpm / 4;
  }

  // The salt the wander's stream takes now: this play's, across the
  // slider's upper range; else 0.
  get wanderSalt() {
    return this.driftOn && this.driftAmount >= UNREPEATABLE_FROM ? this.salt : 0;
  }

  // Where the next step falls, for js/quality.js's boundary: the step in its
  // pass after the last decided, and its time.
  get step() {
    return this.decided ? this.decided.number + 1 : 0;
  }

  get nextStepTime() {
    return this.decided ? this.decided.time + this.stepDur : this.ctx.currentTime + 0.3;
  }

  // `keepPosition`: a re-roll, made again in its place. `resume`: the same
  // loop again under a rebuilt engine (the quality switch), started from its
  // top without its lateFloor's wait.
  load(spec, { keepPosition = false, resume = false } = {}) {
    this.spec = spec;
    this.base = render(spec);
    this.live = this.base;
    // A new play draws a new salt; a loop the core already took (at the
    // track's end, or its own) keeps the one it was drawn with.
    const caught = !keepPosition && this.taken != null && this.taken === spec.seed >>> 0;
    if (!keepPosition && !resume && !caught) this.salt = freshSalt();
    if (!keepPosition) {
      this.gridQueue = [];
      this.countFrom = 0;
    }
    const core = this._core();
    if (!this.playing || !core) {
      this.needs = resume ? 'resumed' : 'load';
      return this.base;
    }
    const seed = spec.seed >>> 0;
    if (keepPosition) {
      core.playRevise(seed, askOf(spec));
    } else if (this.taken === seed) {
      // The core took it at the track's end already: the page catches up.
      this.taken = null;
    } else {
      this.taken = null;
      core.playChange(seed, askOf(spec), MOMENT.now, CHANGE_FADE);
      this.visualQueue = [];
    }
    this._controls();
    return this.base;
  }

  // The controls that hold across loops, as the spec and the sliders have
  // them: each layer's mute, the wander and its salt, the track's length.
  _controls() {
    const core = this._core();
    if (!core || !this.spec || this.wantOwn) return;
    const mutes = this.spec.mutes || {};
    LAYERS.forEach((layer, i) => core.playMute(i, !!mutes[layer]));
    core.playWander(this.driftOn ? this.driftAmount : 0, this.wanderSalt);
    const t = trackOf(this.spec);
    if (!this.track || t[0] !== this.track[0] || t[1] !== this.track[1]) this._sendTrack(t);
  }

  _sendTrack(t = trackOf(this.spec)) {
    const core = this._core();
    if (!core) return;
    this.track = t;
    core.playTrack(t[0], t[1], this._arm());
  }

  // The armed loop as the core takes it, with the track it plays (its own
  // length slider's), or null.
  _arm() {
    const s = this.armed;
    return s ? { seed: s.seed >>> 0, ask: askOf(s), fade: END_FADE, track: trackOf(s) } : null;
  }

  // The loop that follows when the track ends (`spec`): armed in the core,
  // so it starts there with the page asleep. Null for none of the page's:
  // the one waiting is dropped, and a loop of the core's own follows, with
  // the picks (queue item 95).
  arm(spec) {
    this.armed = spec || null;
    const core = this._core();
    if (!core || !this.playing) return;
    core.playArm(this._arm());
  }

  start() {
    if (this.playing || !this.spec) return;
    this.playing = true;
    this.synth.unsilence();
    this._begin();
  }

  // Play: the loop to the core (or the one it has, from where it stood), the
  // controls, the track and the loop armed for its end. Waits for a core
  // still on its way.
  _begin() {
    this._hook();
    const core = this._core();
    if (!core || !this.playing) return;
    if (this.synth.userVolume != null) core.playVolume(this.synth.userVolume);
    // The picks, held by this core for the loops it makes itself.
    this._sendPicks();
    // The wander and its salt before the loop: its first pass is drawn with them.
    core.playWander(this.driftOn ? this.driftAmount : 0, this.wanderSalt);
    if (this.wantOwn) {
      // Next asked from Pause or before the core arrived: the core's own
      // loop is the one it plays; the page catches up at its Loop event.
      this._askOwn();
      this.track = null;
      this.needs = null;
      this._controls();
      return;
    }
    if (this.needs) {
      core.playLoad(this.spec.seed >>> 0, askOf(this.spec), 0, this.needs === 'resumed');
      this.taken = null;
      this.visualQueue = [];
      // A new loop's track counts from its first step: sent again, with the
      // loop armed for its end (a load drops the change waiting there).
      this.track = null;
      this.needs = null;
      this._controls();
      return;
    }
    // Going on where it stood: the track's count goes on too, and only the
    // loop armed for its end, which Pause dropped, is sent again.
    core.playResume(0);
    this._controls();
    if (this.armed) core.playArm(this._arm());
  }

  // Pause: the core stops deciding, what it handed over and has not started
  // is dropped, and the output falls silent. Play goes on where it stood.
  stop() {
    if (!this.playing) return;
    this.playing = false;
    const core = this._core();
    if (core) {
      core.playStop();
      core.cancel();
    }
    this.synth.silence();
    this.visualQueue = [];
    this.gridQueue = [];
  }

  setMute(layer, muted) {
    const core = this._core();
    const i = LAYERS.indexOf(layer);
    if (core && i >= 0) core.playMute(i, !!muted);
  }

  // The tempo slider: from the next step the core decides (the spec holds
  // it already, and a change or a re-roll sends it as an edit).
  setTempo(bpm) {
    const core = this._core();
    if (core) core.playTempo(bpm);
  }

  setTone(tone) {
    const core = this._core();
    if (core) core.playTone(tone);
  }

  // "Let the loop wander": 0 is off, else how far (from the next pass).
  setWander(amount) {
    this.driftOn = amount > 0;
    if (this.driftOn) this.driftAmount = amount;
    const core = this._core();
    if (core) core.playWander(this.driftOn ? this.driftAmount : 0, this.wanderSalt);
  }

  // The length slider moved (the spec holds it): the core counts from the
  // step heard now, and the loop armed for its end goes with it.
  trackChanged(next = this.armed) {
    this.armed = next || null;
    this.countFrom = this.heard ? this.heard.abs : 0;
    this.trackEnd = null;
    if (this.playing) this._sendTrack();
  }

  // Kept for main.js, which asks either engine: the core hears a change at
  // once on its own.
  redecide() {
    return false;
  }

  reset() {}

  // The page shown or hidden, frozen or back: nothing to plan here, the core
  // decides on the audio thread.
  retune() {}

  setFrozen() {}

  clearMetrics() {}

  takeGap() {
    return 0;
  }

  // What the core reports (CoreHost.onevents): its events, in order, and
  // the grids read with them.
  _receive(m) {
    const ev = m.events || [];
    const grids = new Map((m.grids || []).map((g) => [g.at, g.grid]));
    for (let i = 0; i < ev.length / 4; i++) {
      const time = ev[4 * i];
      const kind = ev[4 * i + 1];
      const number = ev[4 * i + 2];
      const step = ev[4 * i + 3];
      if (kind === KIND.step) {
        this.decided = { time, number, step };
        this.visualQueue.push({ time, step: number, abs: step });
        if (this.visualQueue.length > QUEUE_MOST) this.visualQueue.shift();
      } else if (kind === KIND.pass || kind === KIND.section) {
        this.loopCount = number;
        if (kind === KIND.pass && this.onLoop) this.onLoop(number);
      } else if (kind === KIND.own) {
        // The loop whose Loop event comes next is the core's own.
        this.ownTag = number;
      } else if (kind === KIND.loop) {
        this.loops++;
        this.countFrom = 0;
        this.loopCount = 0;
        if (this.ownTag != null) {
          // The core made it with the picks numbered `ownTag`: the page
          // catches up (main.js's onOwn draws the same spec for its card and
          // history and loads it, and `load` sees the core has it).
          const tag = this.ownTag;
          this.ownTag = null;
          this.taken = number >>> 0;
          if (this.onOwn) this.onOwn(number >>> 0, tag);
        }
      } else if (kind === KIND.end) {
        // The track has run: the core has started the loop armed for it, if
        // one was, and the page catches up (main.js's onTrackEnd loads that
        // loop, and `load` sees it is the one the core took). With none of
        // the page's armed, the core makes one of its own (KIND.own).
        if (this.armed) {
          this.taken = this.armed.seed >>> 0;
          if (this.onTrackEnd) this.onTrackEnd();
        }
      }
      const grid = grids.get(i);
      if (grid && this.spec) {
        this.gridQueue.push({ time, pattern: this._gridPattern(grid) });
        if (this.gridQueue.length > GRIDS_MOST) this.gridQueue.shift();
      }
    }
    if (m.trackEnd != null) this.trackEnd = m.trackEnd >= 0 ? m.trackEnd : null;
  }

  // The layers' grid as a pattern js/ui.js draws (`renderGrids`): each
  // layer's loudest note on each step of its cycle.
  _gridPattern(grid) {
    const base = this.base;
    const tracks = {};
    const cycles = {};
    LAYERS.forEach((layer, i) => {
      const row = grid[i] || [];
      cycles[layer] = row.length || base.totalSteps;
      tracks[layer] = [];
      row.forEach((vel, step) => { if (vel) tracks[layer].push({ step, vel }); });
    });
    return { stepsPerBar: base.stepsPerBar, totalSteps: base.totalSteps, cycles, tracks };
  }

  // What the listener is hearing now, on the audio clock (Engine.heardTime:
  // the output timestamp, or the declared latency, smoothed).
  heardTime() {
    const ctx = this.ctx;
    let heard = null;
    if (typeof ctx.getOutputTimestamp === 'function') {
      const ts = ctx.getOutputTimestamp();
      if (ts && ts.contextTime > 0 && ts.performanceTime > 0) {
        const since = (performance.now() - ts.performanceTime) / 1000;
        heard = ts.contextTime + Math.max(0, Math.min(0.5, since));
      }
    }
    if (heard === null) heard = ctx.currentTime - (ctx.outputLatency || ctx.baseLatency || 0);
    const raw = Math.max(0, Math.min(0.6, ctx.currentTime - heard));
    this.latency = this.latency === null ? raw : this.latency * 0.92 + raw * 0.08;
    return ctx.currentTime - this.latency;
  }

  // The step that should be lit now, or null if nothing has changed; and the
  // grid of the pass heard, from its time.
  visualStep() {
    const now = this.heardTime() + this.visualOffset;
    let found = null;
    while (this.visualQueue.length && this.visualQueue[0].time <= now) {
      const v = this.visualQueue.shift();
      this.heard = v;
      found = v.step;
    }
    while (this.gridQueue.length && this.gridQueue[0].time <= now) this.live = this.gridQueue.shift().pattern;
    return found;
  }

  // Where the listener is in this loop, in steps from its load.
  audibleSteps() {
    if (!this.playing || !this.heard || !this.spec) return null;
    const into = (this.heardTime() - this.heard.time) / this.stepDur;
    return this.heard.abs + Math.max(0, Math.min(1, into));
  }

  // The step the track ends at, from the loop's load, as the core says; null
  // while it plays on.
  endStep() {
    return this.trackEnd;
  }

  // The steps a pass of the whole loop takes (Engine.roundSteps).
  roundSteps() {
    const spb = this.base.stepsPerBar || 16;
    return pieceOf(this.spec) ? loopBars(this.spec) * spb : this.base.totalSteps;
  }

  report() {
    const c = this._core();
    const p = c && c.playerAt;
    const states = ['idle', 'making a loop', 'its first pass', 'playing'];
    return {
      clock: 'none (the core decides, on the audio thread)',
      hidden: typeof document !== 'undefined' && document.hidden,
      lookahead: 0,
      redecides: 0,
      ticks: 0,
      lateTicks: 0,
      worstLateMs: 0,
      player: p ? `${states[p.state] || p.state}, step ${p.step}` : c ? 'starting' : 'waiting for the core',
      playerLoops: this.loops,
      playerEventsLost: c ? c.eventsLost : '-',
      playerRefused: c ? c.refused : '-',
      trackEndStep: this.trackEnd == null ? 'none' : this.trackEnd,
      armed: this.armed ? this.armed.name : 'none',
      picksSent: this.picks ? this.picks.tag : 'none',
      ctxState: this.ctx.state,
      sampleRate: this.ctx.sampleRate,
      baseLatency: this.ctx.baseLatency ? +this.ctx.baseLatency.toFixed(4) : '-',
      outputLatency: this.ctx.outputLatency ? +this.ctx.outputLatency.toFixed(4) : '-',
      measuredLatencyMs: this.latency === null ? '-' : Math.round(this.latency * 1000),
      timestampApi: typeof this.ctx.getOutputTimestamp === 'function' ? 'yes' : 'no',
    };
  }
}
