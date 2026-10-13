// The Rust core's host in the browser (queue item 21, roadmap 16).
//
// One AudioWorkletProcessor around one instance of js/dlcore.wasm. The
// main thread decides every note, as it always has, and posts each one
// here with its absolute start time on the context's clock; the core
// queues it and starts it on its exact sample within the block. Notes
// starting at the 128-frame block boundary instead were enough to wreck
// the match against Web Audio in the brain's probe (-0.1 dB residual
// against -55.6).
//
// The module arrives as bytes in processorOptions and is compiled here,
// synchronously: it is small, and passing a compiled WebAssembly.Module
// across is less certain on iOS Safari, the target. No SharedArrayBuffer,
// no threads, nothing that needs special headers.
//
// Since queue item 29 the core mixes too: one input per channel, where the
// synth's own notes for that channel arrive (a note the core could not
// take), and one output: since item 31 the whole of it, the mix -- the
// channel gains, the sends, the duck, the echo and the reverb -- through
// the master chain -- the wobble, saturator, tone and highpass (item 30),
// the bus compressor, the master and kill gains and the ceiling -- for the
// speakers.
// With `taps`, six more outputs carry each channel as it went into the mix,
// and the signal as it reached the bus compressor (the measure harness
// reads them).

const QUANTUM = 128;
const CHANNELS = 5;

// `n` runs of a block each, from `at` in the core's memory.
function runs(buffer, at, n) {
  return Array.from({ length: n }, (_, c) => new Float32Array(buffer, at + c * QUANTUM * 4, QUANTUM));
}
// Core::note's answers (core/src/lib.rs, Taken).
const LATE = 1;
const FULL = 2;
const MISSED = 4;
// The causes Core::dropped_by counts, in its order.
const CAUSES = 3;
// How often the worklet says how early its notes arrive (queue item 41), in
// seconds of audio.
const REPORT_EVERY = 0.25;

class DriftloomCore extends AudioWorkletProcessor {
  constructor(options) {
    super();
    this.alive = true;
    this.core = null;
    this.views = null;
    this.taps = options.numberOfOutputs > 1;
    // How notes arrive (queue item 41), since the last report: the furthest
    // the core had rendered past the page's clock as it posted a note
    // (`reach`), the least time to spare any note had (`early`, negative
    // for a late one), and how far the core's count ran ahead of the
    // context's own clock. And the late notes by why: the page handed it
    // over after its time had passed (a stall), or ahead of it but behind
    // what the core had already rendered (inside the buffer).
    this.arrivals = { n: 0, reach: -Infinity, early: Infinity, lead: 0 };
    this.lateStall = 0;
    this.lateBuffer = 0;
    this.reportAt = 0;
    try {
      const module = new WebAssembly.Module(options.processorOptions.bytes);
      this.core = new WebAssembly.Instance(module, {}).exports;
      this.core.dl_init(sampleRate);
      // The mix as the synth builds it: three combs on lite, six on full.
      if (!this.core.dl_quality(options.processorOptions.quality === 'lite' ? 0 : 1)) {
        throw new Error(`no mix at ${sampleRate} Hz`);
      }
    } catch (err) {
      // Too old a browser for the module, most likely (it needs Safari 15
      // or later). The synth hears this and plays everything in JS.
      this.core = null;
      this.port.postMessage({ type: 'failed', message: String(err && err.message || err) });
    }
    this.port.onmessage = (e) => this.receive(e.data);
  }

  receive(m) {
    const core = this.core;
    switch (m.type) {
      case 'note': {
        if (!core) return;
        // Where the note sits in its loop goes first: the core draws
        // everything the note leaves to chance from it (core/src/rng.rs).
        const k = m.key;
        if (k) core.dl_key(k.seed >>> 0, k.pass >>> 0, k.step >>> 0, k.ord >>> 0);
        const rendered = (this.next !== undefined ? this.next : currentFrame) / sampleRate;
        const a = this.arrivals;
        a.n++;
        a.early = Math.min(a.early, m.time - rendered);
        if (m.sent != null) a.reach = Math.max(a.reach, rendered - m.sent);
        const x = m.extra;
        const taken = core.dl_note(
          m.voice, m.channel, m.time, m.midi, m.dur, m.vel, m.parts,
          x[0] ?? NaN, x[1] ?? NaN, x[2] ?? NaN, x[3] ?? NaN, x[4] ?? NaN, x[5] ?? NaN, x[6] ?? NaN, x[7] ?? NaN,
          x[8] ?? NaN, x[9] ?? NaN, x[10] ?? NaN, x[11] ?? NaN,
        );
        if (taken === LATE || taken === MISSED) {
          const inside = m.sent != null && m.sent < m.time;
          if (inside) this.lateBuffer++;
          else this.lateStall++;
          // Rare, so worth a message each: Diagnostics counts them, and logs
          // each late one with what decided it (queue item 47).
          this.port.postMessage({
            ...this.counts(),
            note: { time: m.time, sent: m.sent, rendered, why: m.why, missed: taken === MISSED, inside },
          });
          return;
        }
        if (taken === FULL) this.port.postMessage(this.counts());
        return;
      }
      case 'damp':
        if (core) core.dl_damp(m.channel, m.time);
        return;
      case 'cancel':
        if (core) {
          if (m.from == null) core.dl_cancel();
          else core.dl_cancel_from(m.from);
        }
        return;
      case 'param':
        if (core) core.dl_param(m.id, m.op, m.value, m.time, m.tau);
        return;
      case 'dispose':
        // Clear everything, and let go of the instance so its memory can be
        // reclaimed: a browser holds only so many WebAssembly memories at
        // once (about 128 in Chromium), and a page that builds synth after
        // synth -- the measure harness renders hundreds -- would otherwise
        // run out.
        if (core) core.dl_clear();
        this.alive = false;
        this.core = null;
        this.views = null;
        this.port.onmessage = null;
        return;
      case 'ping':
        // Every message before this one has been taken: the measure
        // harness waits for this before it renders.
        this.port.postMessage({
          ...this.counts(),
          type: 'pong',
          id: m.id,
          busy: core ? core.dl_busy() : 0,
          // The compressors' `reduction`, in dB: the bus compressor's and
          // the ceiling's.
          reduction: core ? [core.dl_reduction(0), core.dl_reduction(1)] : [0, 0],
          // And how hard the signal has pressed each since the last ping:
          // the deepest its curve has gone under the line.
          deepest: core ? [core.dl_deepest(0), core.dl_deepest(1)] : [0, 0],
        });
        return;
      default:
    }
  }

  // The late and dropped notes so far, by kind.
  counts() {
    const core = this.core;
    const by = [];
    for (let c = 0; c < CAUSES; c++) by.push(core && core.dl_dropped_by ? core.dl_dropped_by(c) : 0);
    return {
      type: 'counts',
      late: core ? core.dl_late() : 0,
      missed: core && core.dl_missed ? core.dl_missed() : 0,
      lateStall: this.lateStall,
      lateBuffer: this.lateBuffer,
      dropped: core ? core.dl_dropped() : 0,
      droppedBy: by,
    };
  }

  // How the notes since the last report arrived, every REPORT_EVERY
  // seconds while any are arriving.
  arrivalReport(block) {
    const a = this.arrivals;
    a.lead = Math.max(a.lead, block - currentFrame);
    if (block < this.reportAt) return;
    this.reportAt = block + Math.round(REPORT_EVERY * sampleRate);
    if (!a.n) return;
    this.port.postMessage({
      type: 'arrivals', n: a.n, reach: a.reach === -Infinity ? null : a.reach, early: a.early,
      lead: a.lead / sampleRate, rendered: block / sampleRate,
    });
    this.arrivals = { n: 0, reach: -Infinity, early: Infinity, lead: 0 };
  }

  process(inputs, outputs) {
    if (!this.alive) return false;
    const core = this.core;
    if (!core) return true;
    // Views onto the core's buffers, made once: the buffers never move, and
    // the memory never grows, since nothing allocates after init. A new
    // view every block would be garbage every block.
    if (!this.views || this.views.buffer !== core.memory.buffer) {
      const buffer = core.memory.buffer;
      this.views = {
        buffer,
        inputs: runs(buffer, core.dl_inputs(), CHANNELS),
        out: runs(buffer, core.dl_out(), core.dl_outputs()),
        bus: runs(buffer, core.dl_bus(), core.dl_outputs()),
        channels: null,
      };
    }
    const views = this.views;
    // The synth's own notes, channel by channel. An input with nothing
    // connected has no channels; the core clears what it took.
    for (let c = 0; c < CHANNELS; c++) {
      const input = inputs[c] && inputs[c][0];
      if (input && input.length === QUANTUM) views.inputs[c].set(input);
    }
    // The block to render: `currentFrame`, unless it says a block the core
    // has already rendered. Chromium sometimes hands a process() call the
    // last block's frame while it renders the next (seen in offline
    // renders: 9088 again where 9216 was due, then 9344), and a block
    // rendered as the last one would play every envelope in it 128 frames
    // late. A real gap -- a block the node was not asked for -- still moves
    // the core on. Time never goes backward, however stale the clock is for
    // however long: a block rendered again starts the voices that began in
    // it a second time, and the pads' and leads' filters spiked to thirty
    // times their level (queue item 32). A count the clock never catches up
    // with is a lead, and costs nothing but the few notes scheduled inside
    // it counted late.
    const block = this.next !== undefined && this.next > currentFrame ? this.next : currentFrame;
    this.next = block + QUANTUM;
    this.arrivalReport(block);
    const at = core.dl_process(block);
    // A run for each of the output's channels (one: the graph is mono).
    const out = outputs[0];
    if (out) {
      for (let c = 0; c < out.length && c < views.out.length; c++) {
        if (out[c].length === QUANTUM) out[c].set(views.out[c]);
      }
    }
    if (this.taps) {
      if (!views.channels) views.channels = runs(views.buffer, at, CHANNELS);
      for (let c = 0; c < CHANNELS; c++) {
        const tap = outputs[c + 1] && outputs[c + 1][0];
        if (tap && tap.length === QUANTUM) tap.set(views.channels[c]);
      }
      const bus = outputs[CHANNELS + 1] && outputs[CHANNELS + 1][0];
      if (bus && bus.length === QUANTUM) bus.set(views.bus[0]);
    }
    return true;
  }
}

registerProcessor('driftloom-core', DriftloomCore);
