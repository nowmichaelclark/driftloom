// The Rust core, main-thread side (queue item 21, roadmap 16).
//
// The core is Driftloom's engine, and the app plays the core's player: the
// core decides every note itself, on the audio thread (core/src/player/,
// through js/dlplayer.wasm in js/worklet.js), plays them, mixes them and
// finishes them, so its node goes straight to the speakers. The page sends
// it commands and draws what it reports (js/player.js). Since queue item 97
// there is nothing else: the page's own engine and the JavaScript synth it
// fell back to are gone, and a browser without AudioWorklet or WebAssembly,
// or one whose core fails to load or traps while playing, is told so in a
// plain line (Mikey, 2026-11-07; Diagnostics says which, `none (...)`).

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
// 192 kHz would play nothing.
export const CORE_MAX_RATE = 96000;
export const CORE_FALLBACK_RATE = 48000;

// A new audio context from `Ctx` with `options`, on a rate the core can
// run at: if the browser's default is above CORE_MAX_RATE, a context at
// CORE_FALLBACK_RATE instead, and the fast one closed. If the browser will
// not make that one, the first stands, the core says it cannot mix at that
// rate, and the page says it cannot play.
export function newContext(Ctx, options = {}) {
  const ctx = new Ctx(options);
  if (!(ctx.sampleRate > CORE_MAX_RATE)) return ctx;
  try {
    const slower = new Ctx({ ...options, sampleRate: CORE_FALLBACK_RATE });
    try { ctx.close(); } catch { /* already gone */ }
    return slower;
  } catch {
    return ctx;
  }
}

// The mix's parameters the page still sets, by the number the core knows
// them by (core/src/mix.rs, core/src/lib.rs): `kill`, the kill gain (the
// transport's silence, js/output.js), and the two whose events a switch of
// quality cancels, the kicks' duck (`pump`) and the tails' fades. The
// player sets everything else itself.
export const MIX = { pump: 5, tails: 6, kill: 18 };
// The AudioParam calls a parameter takes. HOLD is two: cancelScheduledValues
// at the time, then setValueAtTime there with the parameter's `value`
// attribute, holding it where it is.
export const SET = 0;
export const LINEAR = 1;
export const CANCEL = 3;
export const HOLD = 4;

// Why the core dropped a note, in Core::dropped_by's order.
export const DROP_CAUSES = ['queue full', 'voices busy', 'sung voices busy'];

// The core's module (js/dlplayer.wasm: the core with its generator and its
// player), fetched once a page and shared by every context.
const WASM_URL = new URL('./dlplayer.wasm', import.meta.url);
const WORKLET_URL = new URL('./worklet.js', import.meta.url);

let fetched = null;
export function fetchCore() {
  if (!fetched) {
    fetched = fetch(WASM_URL).then((res) => {
      if (!res.ok) throw new Error(`${WASM_URL.pathname.split('/').pop()}: HTTP ${res.status}`);
      return res.arrayBuffer();
    });
    // A failed fetch is tried again by the next context.
    fetched.catch(() => { fetched = null; });
  }
  return fetched;
}

// The contexts the worklet's module has been added to.
const modules = new WeakMap();

// What a context needs to build cores: the worklet's module added to it,
// and the core's bytes. Resolves with `{ bytes }` for CoreHost; rejects
// with why the core cannot run here.
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

// One core and its player in one AudioWorkletNode. `layout` is 1 for the
// mono render or 2 for the stereo one (queue item 64), and `setLayout`
// switches it while it plays (item 66, the Mono switch): the node's output
// carries two channels either way, the mono render in both, so the switch
// never rebuilds it. The node takes the player's commands (`playLoad` and
// the rest) and reports what it decides as events (`onevents`).
export class CoreHost {
  constructor(ctx, core, { quality = 'full', layout = 1, burn = 0 } = {}) {
    this.ctx = ctx;
    this.layout = layout === 2 ? 2 : 1;
    // Called once if the core turns out not to run (too old a browser).
    this.onfail = null;
    // What the core says of its own notes: those it took late, those it
    // left out for being too late, and those it dropped, by cause.
    this.late = 0;
    this.missed = 0;
    this.dropped = 0;
    this.droppedBy = DROP_CAUSES.map(() => 0);
    this.failed = null;
    // What the core's render costs, each layout apart (queue item 66): the
    // milliseconds its blocks took and the seconds of sound they made.
    this.cost = { ms: [0, 0], seconds: [0, 0] };
    // How busy the audio thread is (queue item 87b): called with each of
    // the worklet's reports.
    this.onbusy = null;
    this._pings = new Map();
    this._ping = 0;
    // Where the player is, as last reported ({ state, step, time }); its
    // events (`onevents`), its answer to the picks (`onpicks`), the events
    // it had no room for and the parts of a request it refused.
    this.playerAt = null;
    this.onevents = null;
    this.onpicks = null;
    this.eventsLost = 0;
    this.refused = 0;
    this._probes = new Map();
    this.node = new AudioWorkletNode(ctx, 'driftloom-core', {
      numberOfInputs: 0,
      numberOfOutputs: 1,
      outputChannelCount: [2],
      processorOptions: { bytes: core.bytes, quality, layout: this.layout, burn },
    });
    this.node.port.onmessage = (e) => this._receive(e.data);
    // A throw inside the worklet (the module would not compile, a trap).
    this.node.onprocessorerror = (e) => this._fail((e && e.message) || 'processor error');
  }

  // The render from now: mono (1) or stereo (2), faded by the core.
  setLayout(layout) {
    this.layout = layout === 2 ? 2 : 1;
    this.node.port.postMessage({ type: 'layout', channels: this.layout });
  }

  // For Diagnostics: what the render costs this device, each layout apart.
  costReport() {
    const { ms, seconds } = this.cost;
    const time = (x) => (x < 10 ? x.toFixed(1) : Math.round(x));
    const one = (i) => (seconds[i] > 0 ? `${((ms[i] / 1000 / seconds[i]) * 100).toFixed(1)}% of real time over ${time(seconds[i])} s` : 'not played');
    return `mono ${one(0)}, stereo ${one(1)}`;
  }

  // One of the mix's parameters (`MIX`), by one of the AudioParam calls
  // (`SET` and the rest), at `time` on the context's clock.
  param(id, op, value, time, tau = 0) {
    this.node.port.postMessage({ type: 'param', id, op, value, time, tau });
  }

  // The core's player (queue items 82 and 94b). `ask` is a loop's request
  // and a code's edits, as `askOf` in js/player.js builds them; `seed` its
  // seed. `playLoad` starts the loop afresh, its first step at `at` on the
  // context's clock (0: as soon as it is made), `resumed` a loop that was
  // playing on another core (the quality switch: its lateFloor does not wait
  // again). `playChange` changes to it at a moment (`MOMENT` in
  // js/player.js), the old loop falling to nothing over `fade` seconds;
  // `playRevise` makes the loop playing again in its place (a re-roll).
  playLoad(seed, ask, at = 0, resumed = false) {
    this.node.port.postMessage({ type: 'playLoad', seed, ask, at, resumed });
  }

  playChange(seed, ask, moment, fade) {
    this.node.port.postMessage({ type: 'playChange', seed, ask, moment, fade });
  }

  playRevise(seed, ask) {
    this.node.port.postMessage({ type: 'playRevise', seed, ask });
  }

  // How long the track plays (`kind` 0 on and on, 1 `count` passes, 2
  // `count` bars), counted from what is heard now; with `arm` ({ seed, ask,
  // fade }), the loop that follows when it ends, in the same message so that
  // no block falls between them.
  playTrack(kind, count, arm = null) {
    this.node.port.postMessage({ type: 'playTrack', kind, count, arm });
  }

  // The loop that follows when the track ends (`MOMENT.end`), alone; with
  // `arm.track` ([kind, count]) the track it plays; null drops the one
  // waiting, so the core's own loop follows (queue item 95).
  playArm(arm) {
    this.node.port.postMessage({ type: 'playArm', arm });
  }

  // The picks (queue item 95): the New loops panel's request (`picksAsk` in
  // js/player.js), held by the core for every loop it makes itself and
  // numbered `tag`; the core answers (`onpicks`) whether it can meet them.
  playPicks(ask, tag) {
    this.node.port.postMessage({ type: 'playPicks', ask, tag });
  }

  // The seed the core draws its own loops' seeds from (0, none of its own).
  playDraws(seed) {
    this.node.port.postMessage({ type: 'playDraws', seed });
  }

  // A new loop of the core's own at `moment`, the old falling over `fade`.
  playNext(moment, fade) {
    this.node.port.postMessage({ type: 'playNext', moment, fade });
  }

  // The Sound panel's tone, gliding there from now.
  playTone(tone) {
    this.node.port.postMessage({ type: 'playTone', warmth: tone.warmth, space: tone.space, wobble: tone.wobble });
  }

  playStop() {
    this.node.port.postMessage({ type: 'playStop' });
  }

  playResume(at) {
    this.node.port.postMessage({ type: 'playResume', at });
  }

  // `salt` (queue item 92): the wander's stream salted, 0 for none.
  playWander(amount, salt = 0) {
    this.node.port.postMessage({ type: 'playWander', amount, salt });
  }

  playMute(layer, muted) {
    this.node.port.postMessage({ type: 'playMute', layer, muted });
  }

  playTempo(bpm) {
    this.node.port.postMessage({ type: 'playTempo', bpm });
  }

  playVolume(volume) {
    this.node.port.postMessage({ type: 'playVolume', volume });
  }

  // Resolves with what the player says about the loop now: its state, step,
  // track's end, events lost, lanes and description (`describe`'s lines as
  // an object). For the checks (tools/rows.mjs reads the same from outside).
  probe() {
    const id = ++this._ping;
    return new Promise((resolve) => {
      this._probes.set(id, resolve);
      this.node.port.postMessage({ type: 'probe', id });
    });
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
  // rebuild must not reach an output that is gone.
  dispose() {
    this.onfail = null;
    this.onevents = null;
    this.node.onprocessorerror = null;
    this.node.port.postMessage({ type: 'dispose' });
    try { this.node.disconnect(); } catch { /* already detached */ }
    this.node.port.onmessage = null;
  }

  _receive(m) {
    if (m.type === 'counts' || m.type === 'pong' || m.type === 'cost' || m.type === 'probe') {
      if (m.late != null) this.late = m.late;
      if (m.dropped != null) this.dropped = m.dropped;
      if (m.missed != null) this.missed = m.missed;
      if (m.droppedBy) this.droppedBy = m.droppedBy;
      if (m.player) this.playerAt = m.player;
    }
    if (m.type === 'cost') {
      this.cost = { ms: m.ms, seconds: m.seconds };
      if (m.busy && this.onbusy) this.onbusy(m.busy);
    }
    if (m.type === 'pong') {
      const resolve = this._pings.get(m.id);
      this._pings.delete(m.id);
      if (resolve) resolve(m);
    }
    if (m.type === 'events') {
      if (m.lost != null) this.eventsLost = m.lost;
      if (m.refused != null) this.refused = m.refused;
      if (this.onevents) this.onevents(m);
    }
    if (m.type === 'picks' && this.onpicks) this.onpicks(m);
    if (m.type === 'probe') {
      const resolve = this._probes.get(m.id);
      this._probes.delete(m.id);
      if (resolve) resolve(m);
    }
    if (m.type === 'failed') this._fail(m.message);
  }

  // The core does not run, at load or while playing. Once: the output hears
  // it, and the page says it cannot play.
  _fail(message) {
    if (this.failed) return;
    this.failed = message;
    if (this.onfail) this.onfail();
  }
}
