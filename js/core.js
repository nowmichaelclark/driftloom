// The Rust core, main-thread side (queue item 21, roadmap 16).
//
// The core is Driftloom's engine (queue item 32): it plays by default, and
// `?engine=js` asks for the JavaScript synth instead (for measurement and
// comparison). An old `?engine=rust` link keeps working: it asks for what
// is already the default. The synth hands every voice to an
// AudioWorkletNode running js/dlcore.wasm, and since queue item 29 the mix
// as well: the channels, sends, duck, echo and reverb; since item 30 the
// wobble, the saturator, the tone and the highpass; since item 31 the bus
// compressor, the master and kill gains and the ceiling, so its output goes
// straight to the speakers. It keeps everything else: scheduling, the voice
// budget; and, since queue item 44, none of the chance in a note: where each
// note sits in its loop goes with it (a key), and the core draws from that.
// The JavaScript synth stays, as the fallback: a browser without
// AudioWorklet or WebAssembly, a core that fails to load, a core that traps
// while playing (Diagnostics says which, `js (fallback: ...)`), and what
// `?engine=js` asks for (`js (asked)`).

export const ENGINE = (() => {
  try {
    return new URLSearchParams(globalThis.location?.search || '').get('engine') === 'js' ? 'js' : 'rust';
  } catch {
    return 'rust';
  }
})();

// Why this browser cannot run the core at all, or null: it needs
// WebAssembly and an AudioWorklet (loadCore() checks the context's own).
export function coreUnsupported() {
  if (typeof WebAssembly !== 'object' || typeof WebAssembly.Module !== 'function') return 'no WebAssembly in this browser';
  if (typeof globalThis.AudioWorkletNode !== 'function') return 'no AudioWorklet in this browser';
  return null;
}

// The highest sample rate the web core holds its delay lines and noise for
// (core/src/lib.rs, MAX_RATE without the `wide` feature), and the rate asked
// for instead when a context comes up faster (queue item 45): 48 kHz, which
// the browser resamples to the device. Without it a device whose default is
// 192 kHz would play everything in JavaScript.
export const CORE_MAX_RATE = 96000;
export const CORE_FALLBACK_RATE = 48000;

// A new audio context from `Ctx` with `options`, on a rate the core can
// run at: if the browser's default is above CORE_MAX_RATE, a context at
// CORE_FALLBACK_RATE instead, and the fast one closed. If the browser will
// not make that one, the first stands and the synth falls back to
// JavaScript, as it always has for a core that will not run.
export function newContext(Ctx, options = {}) {
  const ctx = new Ctx(options);
  if (ENGINE !== 'rust' || !(ctx.sampleRate > CORE_MAX_RATE)) return ctx;
  try {
    const slower = new Ctx({ ...options, sampleRate: CORE_FALLBACK_RATE });
    try { ctx.close(); } catch { /* already gone */ }
    return slower;
  } catch {
    return ctx;
  }
}

// The voices the core plays, by the number it knows them by
// (the registry, core/src/blocks/mod.rs), and the parts of a kalimba note
// (core/src/blocks/kalimba.rs).
export const CORE_VOICES = {
  kalimba: 0, fiddle: 1, pad: 2, fm: 3, sine: 4, tubular: 5,
  softpad: 6, analogpad: 7, analoglead: 8, sawpluck: 9, beep: 10, moog: 11, whistle: 12,
  nylon: 13, accordion: 14,
  stab: 15, ocarina: 16, flute: 17, panflute: 18, knock: 19, templebell: 20, hat: 21,
  bass: 22, texture: 23, drum: 24,
  vowel: 25, hum: 26, choir: 27,
  dubstab: 28,
};
// A channel's own blocks, by the number the core knows them by (`own` in
// core/src/blocks/mod.rs, the registry's `channel` list), for Core::set;
// and the chapel organ's settings, in its `set` order
// (core/src/blocks/organ.rs, queue item 61).
export const CORE_OWN = { organ: 3, wash: 4 };
export const ORGAN_SETTINGS = [
  'bar16', 'bar5', 'bar8', 'bar4', 'bar2_3', 'bar2', 'bar1_3_5', 'bar1_1_3', 'bar1',
  'speak', 'chiff', 'celeste', 'celesteLevel', 'release', 'tone', 'low', 'carve', 'shimmer', 'level', 'verb', 'dry',
];
// How many channels the core mixes (CHANNELS in core/src/lib.rs): the five
// layers', which the synth's channels feed (the node's inputs), and the
// wash's (WASH_CHANNEL, queue item 62), which only the core plays into.
export const CORE_CHANNEL_COUNT = 6;
export const WASH_CHANNEL = 5;
// The wash's settings (core/src/blocks/wash.rs, `set`, queue item 62): its
// period, level, tint (Hz, two gains in dB, a Q), then the follower's weight
// for each layer's channel, in the core's channel order (CORE_LAYERS), each
// routed into the wash's input of the same number.
export const WASH_SETTINGS = ['period', 'level', 'tint', 'tintGain', 'tintHigh', 'tintQ'];
export const WASH_WEIGHT = 6;
// The kinds of channel event (core/src/block.rs, `event`).
export const EVENTS = { damp: 0, release: 1, bend: 2, retune: 3 };
// A sung note's vowel and where it drifts to, as the core numbers them
// (core/src/blocks/sung.rs); 4 is a hum opening.
export const SUNG_VOWELS = { a: 0, e: 1, o: 2, u: 3 };
export const SUNG_OPEN_HUM = 4;
// The longest sung note the core takes (sung::LONGEST): its detune holds
// the jitter's steps for this long. Longer ones play in JavaScript.
export const SUNG_LONGEST = 13.5;
// A bass note's voice in `parts`, and its two flags (core/src/blocks/bass.rs); a
// name that is not here is the original, `sub`, as the JavaScript voice has it.
export const BASS_KINDS = { sub: 0, round: 1, fifths: 2, pluckbass: 3, moogbass: 4 };
export const BASS_GLIDE = 8;
export const BASS_CHUG = 16;
// A texture note's, in `parts` (core/src/blocks/texture.rs).
export const TEXTURE_KINDS = { swell: 0, drop: 1, wind: 2, waves: 3 };
// A drum's, in `parts` (core/src/blocks/drum.rs); the hats are the hat voice,
// and their kinds (core/src/blocks/struck.rs) are `parts` of their own.
export const DRUM_KINDS = {
  kick: 0, softkick: 1, snare: 2, clap: 3, rim: 4, frame: 5, tap: 6, jingle: 7, ojingle: 8,
};
export const HAT_KINDS = { hat: 0, ohat: 1, shaker: 2 };
// The longest noise the core holds (NOISE_MAX, core/src/noise.rs): two
// seconds at 96 kHz. It makes the noise itself, two seconds of it, as it
// starts; a context faster than this keeps its noise voices in JavaScript.
export const CORE_NOISE_MAX = 192000;
export const CORE_NOISE_SECONDS = 2;
export const KALIMBA_STRIKE = 1;
export const KALIMBA_BODY = 2;

// The mix's parameters and the master chain's, by the number the core
// knows them by (core/src/mix.rs, core/src/lib.rs): each one AudioParam of
// the synth's graph (the combs' one of each comb, which setTone moves
// together). `gains` is the first of five, one per channel in the core's
// order. `master` is the master gain (setVolume, setCharacterLevel) and
// `kill` the kill gain (silence, unsilence). Four are not AudioParams:
// `drive`, the value setTone hands tanhCurve() for the saturator's new
// curve; `lfoStart`, when the synth started the wobble's LFOs (the call's
// time); and `bypass`, which routes around both compressors when its value
// is not 0 (the measure harness's 'bypass'); and `cold`, which starts both
// compressors as Chromium's start, not warm, when its value is not 0, before
// the first block (the harness's, to null them against Chromium's own).
// Two more are not AudioParams either: `arcFrom` and `arcPeriod` start the arc (queue item 63, ARC_FROM and
// ARC_PERIOD in core/src/lib.rs): where on its swing it starts, 0 to 1, and
// its period from the call's time, 0 for none.
export const MIX = {
  gains: 0, pump: 5, tails: 6, echo: 7, combFb: 8, combFreq: 9, combSum: 10, reverbOut: 11,
  tone: 12, wow: 13, flutter: 14, drive: 15, lfoStart: 16, master: 17, kill: 18, bypass: 19, cold: 20,
  arcFrom: 21, arcPeriod: 22,
};
// The AudioParam calls a parameter takes. HOLD is two: cancelScheduledValues
// at the time, then setValueAtTime there with the parameter's `value`
// attribute, holding it where it is.
export const SET = 0;
export const LINEAR = 1;
export const TARGET = 2;
export const CANCEL = 3;
export const HOLD = 4;

// Why the core dropped a note, in Core::dropped_by's order.
export const DROP_CAUSES = ['queue full', 'voices busy', 'sung voices busy'];

// The render-ahead (queue item 41): how far past the page's clock the core
// has already rendered by the time a note posted now reaches it. Taken as
// the worst of what the core reports, at once, and let back down only this
// fast (seconds a second), so one quiet spell does not undo a phone's
// buffer.
const AHEAD_EASE = 0.02;

// A log of the render-ahead's jumps and of each late note (queue item 47),
// for Diagnostics: kept for the page, across every core it builds, the last
// LOG_MOST of them. Each entry has `at`, when it happened on the page's
// clock (performance.now(), ms since the page began), and its `kind`:
// 'jump', the render-ahead measured past what was planned from by more than
// JUMP (`reachMs`, `wasMs`); or 'late', a note that reached the core after
// its time (`lateMs` behind what the core had rendered, `spareMs` ahead of
// the page's clock as it was posted, `why` it was decided, whether it was
// `missed`: left out, and whether it came `inside` the buffer rather than
// after a stall). `onjump(reach)` is called with each jump, so the page can
// keep the worst for this device.
export const coreLog = { entries: [], total: 0, onjump: null };
const LOG_MOST = 40;
const JUMP = 0.1;

function logEntry(entry) {
  coreLog.total++;
  coreLog.entries.push(entry);
  if (coreLog.entries.length > LOG_MOST) coreLog.entries.shift();
}

const WASM_URL = new URL('./dlcore.wasm', import.meta.url);
const WORKLET_URL = new URL('./worklet.js', import.meta.url);

// Fetched once per page, however many contexts and synths come and go.
let bytes = null;
export function fetchCore() {
  if (!bytes) {
    bytes = fetch(WASM_URL).then((res) => {
      if (!res.ok) throw new Error(`dlcore.wasm: HTTP ${res.status}`);
      return res.arrayBuffer();
    });
    // A failed fetch can be tried again.
    bytes.catch(() => { bytes = null; });
  }
  return bytes;
}

// The worklet module, added once per context.
const modules = new WeakMap();

// Everything a Synth needs to build its core node on this context: the
// module's bytes, with the worklet already added. Async, so the app calls
// it as soon as it has a context and the synth takes it when it arrives.
export async function loadCore(ctx) {
  if (typeof WebAssembly !== 'object') throw new Error('no WebAssembly in this browser');
  if (!ctx.audioWorklet) throw new Error('no AudioWorklet in this browser');
  let added = modules.get(ctx);
  if (!added) {
    added = ctx.audioWorklet.addModule(WORKLET_URL);
    modules.set(ctx, added);
  }
  const [wasm] = await Promise.all([fetchCore(), added]);
  return { bytes: wasm };
}

// A note carries up to twelve of its voice's own values (EXTRA in
// core/src/lib.rs); what it leaves out reaches the core as NaN.
const NO_EXTRA = [];

// Where a note sits when nothing says (core/src/rng.rs, Key).
const NO_KEY = { seed: 0, pass: 0, step: 0, ord: 0 };

// One core node for one synth. `dests` are the synth's five channels in
// the core's order, each a voice's destination: what a voice the core
// takes would have played into, and, for a note it cannot take, where the
// synth plays it instead -- connected by the synth into the node's input
// for that channel. The node's one output is the mix through the whole
// master chain, for the speakers; with `taps`, five more carry the channels
// as they went into the mix, and one more the signal as it reached the bus
// compressor.
// `quality` is the synth's: the mix has six combs on 'full', three on
// 'lite'. `key` is a function giving where the note about to be sent sits
// in its loop, `{ seed, pass, step, ord }` (core/src/rng.rs): the core
// draws everything a note leaves to chance from it.
export class CoreHost {
  constructor(ctx, core, dests, { quality = 'full', taps = false, key = null } = {}) {
    this.ctx = ctx;
    this.dests = dests;
    this.taps = taps;
    this.keyOf = key || (() => NO_KEY);
    // Called once if the core turns out not to run (too old a browser).
    this.onfail = null;
    // Whether the core has the noise its noise voices play: it makes two
    // seconds of it as it starts, if it holds that many samples.
    this.noise = Math.floor(ctx.sampleRate * CORE_NOISE_SECONDS) <= CORE_NOISE_MAX;
    this.late = 0;
    this.dropped = 0;
    // Queue item 41: the late notes left out (more than the core's MISS
    // late), and the late ones by why (the page's stall, or a note handed
    // over ahead of its time that still landed behind what the core had
    // rendered); the dropped ones by cause (DROP_CAUSES).
    this.missed = 0;
    this.lateStall = 0;
    this.lateBuffer = 0;
    this.droppedBy = DROP_CAUSES.map(() => 0);
    // The render-ahead, measured (null until notes have arrived), the most
    // it has been, the least time any note had to spare when it reached
    // the core, and how far the core's count has run past the context's
    // clock.
    this.ahead = null;
    this.worstAhead = 0;
    this.earliest = Infinity;
    this.clockLead = 0;
    this._aheadAt = 0;
    // What decided the notes posted next (queue item 47): the engine says,
    // through the synth, and each note carries it.
    this.why = null;
    // Called with each report of how notes arrive (`arrivals`).
    this.onarrivals = null;
    // The compressors' gain reduction in dB, as of the last flush(); and
    // how hard the signal has pressed each since this node was built, the
    // deepest its curve has gone under the line.
    this.reduction = [0, 0];
    this.deepest = [0, 0];
    this.failed = null;
    this._pings = new Map();
    this._ping = 0;
    const outputs = taps ? 2 + CORE_CHANNEL_COUNT : 1;
    this.node = new AudioWorkletNode(ctx, 'driftloom-core', {
      numberOfInputs: dests.length,
      numberOfOutputs: outputs,
      outputChannelCount: new Array(outputs).fill(1),
      // The graph is mono end to end.
      channelCount: 1,
      channelCountMode: 'explicit',
      // Copied, not moved: the page keeps its copy for the next synth.
      processorOptions: { bytes: core.bytes, quality },
    });
    this.node.port.onmessage = (e) => this._receive(e.data);
    // The processor threw while playing (a trap in the core, say): the node
    // is silent for good, and the synth's JavaScript mix takes over.
    this.node.onprocessorerror = (e) => this._fail((e && e.message) || 'processor error');
  }

  // The output carrying channel `channel` as it went into the mix (`taps`).
  tapOf(channel) {
    return this.taps ? 1 + channel : -1;
  }

  // The output carrying the signal as it reached the bus compressor
  // (`taps`).
  beforeComp() {
    return this.taps ? 1 + CORE_CHANNEL_COUNT : -1;
  }

  // Move one of the mix's parameters (MIX) by an AudioParam call (SET,
  // LINEAR, TARGET, CANCEL), as the synth would move its own node's:
  // `value` at `time`, with time constant `tau` for a target.
  param(id, op, value, time, tau = 0) {
    this.node.port.postMessage({ type: 'param', id, op, value, time, tau });
  }

  // The core's channel number for this destination, or -1.
  channelOf(dest) {
    return this.failed ? -1 : this.dests.indexOf(dest);
  }

  // `extra` is the voice's own numbers (core/src/lib.rs): for the fiddle,
  // the note it is joined to and its vibrato; for the pad, how many notes
  // the chord has; for fm, its five options. What a voice leaves to chance
  // is not here: the core draws it from the note's `key`.
  note(voice, channel, time, midi, dur, vel, parts, extra = NO_EXTRA) {
    // `sent`: the page's clock as it posts, so the worklet can say how far
    // past it the core had already rendered when the note arrived.
    this.node.port.postMessage({ type: 'note', voice, channel, time, midi, dur, vel, parts, extra, key: this.keyOf(), sent: this.ctx.currentTime, why: this.why });
  }

  // A new strum into `channel` at `time`: the last strum's nylon strings
  // still held let go (Synth.damp, Core::damp).
  damp(channel, time) {
    this.node.port.postMessage({ type: 'damp', channel, time });
  }

  // Setting number `setting` of `channel`'s own block number `block`
  // (CORE_OWN) to `value` at `time`, for the loop (Core::set, queue item
  // 46c).
  set(channel, block, setting, value, time) {
    this.node.port.postMessage({ type: 'set', channel, block, setting, value, time });
  }

  // Route `from`'s signal (a channel, as it goes into the mix) into setting
  // `setting` (its MODS) of `channel`'s own block number `block` from `time`
  // (Core::route); `from` null undoes the route.
  route(channel, block, setting, from, time) {
    const none = 4294967295;
    this.node.port.postMessage({ type: 'route', channel, block, setting, from: from == null ? none : from, bus: none, time });
  }

  // A channel event of `kind` (EVENTS) at `time`, with `value`: a release's
  // fall, in seconds (Core::event).
  event(channel, kind, time, value = 0) {
    this.node.port.postMessage({ type: 'event', channel, kind, time, value });
  }

  // A retune of `channel` to `notes` (six at most) at `time`: what the
  // instrument it holds plays from then (Event::retune).
  retune(channel, time, notes) {
    this.node.port.postMessage({ type: 'retune', channel, time, notes: notes.slice(0, 6) });
  }

  // Every note handed over that has not started yet, dropped; the ones
  // sounding carry on (queue item 39). With `from`, only those asked for
  // at that time or later (queue item 40).
  cancel(from = null) {
    this.node.port.postMessage(from == null ? { type: 'cancel' } : { type: 'cancel', from });
  }

  // Resolves once the worklet has taken every message sent before it.
  flush() {
    const id = ++this._ping;
    return new Promise((resolve) => {
      this._pings.set(id, resolve);
      this.node.port.postMessage({ type: 'ping', id });
    });
  }

  // Nothing the node says afterwards is heard: an error queued just before a
  // rebuild must not reach the synth that is gone (it would put the
  // JavaScript chain back on a disposed synth).
  dispose() {
    this.onfail = null;
    this.node.onprocessorerror = null;
    this.node.port.postMessage({ type: 'dispose' });
    try { this.node.disconnect(); } catch { /* already detached */ }
    this.node.port.onmessage = null;
  }

  _receive(m) {
    if (m.type === 'counts' || m.type === 'pong') {
      this.late = m.late;
      this.dropped = m.dropped;
      if (m.missed != null) this.missed = m.missed;
      if (m.lateStall != null) this.lateStall = m.lateStall;
      if (m.lateBuffer != null) this.lateBuffer = m.lateBuffer;
      if (m.droppedBy) this.droppedBy = m.droppedBy;
    }
    if (m.note) this._late(m.note);
    if (m.type === 'arrivals') this._arrivals(m);
    if (m.type === 'pong') {
      if (m.reduction) this.reduction = m.reduction;
      if (m.deepest) this.deepest = this.deepest.map((d, i) => Math.min(d, m.deepest[i]));
      const resolve = this._pings.get(m.id);
      this._pings.delete(m.id);
      if (resolve) resolve(m);
    }
    if (m.type === 'failed') this._fail(m.message);
  }

  // How the last few notes reached the core: the render-ahead follows the
  // worst at once and eases down slowly.
  _arrivals(m) {
    const now = this.ctx.currentTime;
    if (m.reach != null && Number.isFinite(m.reach)) {
      const eased = this.ahead == null ? 0 : Math.max(0, this.ahead - AHEAD_EASE * Math.max(0, now - this._aheadAt));
      // Past what the page was planning from: a jump, for the log.
      if (m.reach > eased + JUMP) {
        logEntry({ kind: 'jump', at: this._pageTime(m.rendered), reachMs: Math.round(m.reach * 1000), wasMs: Math.round(eased * 1000) });
        if (coreLog.onjump) coreLog.onjump(m.reach);
      }
      this.ahead = Math.max(eased, m.reach);
      this._aheadAt = now;
      this.worstAhead = Math.max(this.worstAhead, m.reach);
    }
    if (Number.isFinite(m.early)) this.earliest = Math.min(this.earliest, m.early);
    if (Number.isFinite(m.lead)) this.clockLead = Math.max(this.clockLead, m.lead);
    if (this.onarrivals) this.onarrivals(m);
  }

  // A note that reached the core after its time (queue item 47), for the
  // log: how far behind what the core had rendered, how far ahead of the
  // page's clock it was posted, and why it was decided.
  _late(n) {
    const ms = (x) => (Number.isFinite(x) ? Math.round(x * 1000) : null);
    logEntry({
      kind: 'late', at: this._pageTime(n.rendered), lateMs: ms(n.rendered - n.time), spareMs: ms(n.time - n.sent),
      why: n.why || null, missed: !!n.missed, inside: !!n.inside,
    });
  }

  // When the core was at `rendered` (its own count, seconds), on the page's
  // clock: as long before now as the context's clock is past it.
  _pageTime(rendered) {
    const behind = Number.isFinite(rendered) ? Math.max(0, this.ctx.currentTime - rendered) : 0;
    return Math.round(performance.now() - behind * 1000);
  }

  // The core does not run, at load or while playing. Once: the synth hears
  // it and plays everything in JS.
  _fail(message) {
    if (this.failed) return;
    this.failed = message;
    if (this.onfail) this.onfail();
  }
}
