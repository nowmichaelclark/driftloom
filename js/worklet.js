// The Rust core's host in the browser (queue item 21, roadmap 16).
//
// One AudioWorkletProcessor around one instance of js/dlplayer.wasm: the
// core with its generator and its player (queue items 82 and 94b). The
// player decides every note itself, here on the audio thread, a block at a
// time before the block is rendered; the page sends commands (a loop's
// seed and request, a change, the controls) and is posted what the player
// decided as events (js/player.js draws them). Nothing the page does can
// make a note late: there are no notes on the page (queue item 97).
//
// The module arrives as bytes in processorOptions and is compiled here,
// synchronously: passing a compiled WebAssembly.Module across is less
// certain on iOS Safari, the target. No SharedArrayBuffer, no threads,
// nothing that needs special headers.
//
// The node has no inputs and one output: the whole of the core's mix
// through its master chain, for the speakers.
//
// Stereo (queue item 64): with `layout: 2` in processorOptions the core
// renders its stereo render, and the output carries two channels, left
// then right; otherwise it renders mono into every channel the output
// has. A `layout` message switches it while it plays (queue item 66, the
// listener's Mono switch): the core fades the side in or out, and the
// output follows the lanes it renders (`dl_lanes`).

const QUANTUM = 128;

// `n` runs of a block each, from `at` in the core's memory.
function runs(buffer, at, n) {
  return Array.from({ length: n }, (_, c) => new Float32Array(buffer, at + c * QUANTUM * 4, QUANTUM));
}
// The causes Core::dropped_by counts, in its order.
const CAUSES = 3;
// The player's events (core/src/player/events.rs, queue item 94a): the most
// that wait, the kinds by number (`Kind::ALL`), and the layers a grid has.
// They are posted to the page every EVENTS_EVERY seconds of audio: a step's
// event is made up to the player's lookahead (0.25 s) before it is heard.
const EVENTS_MAX = 256;
const STEP = 0;
const PASS = 1;
const SECTION = 2;
const LOOP = 4;
const REVISED = 5;
const LAYER_COUNT = 5;
const EVENTS_EVERY = 0.05;
// A change at the track's end (`Moment::End`).
const END = 5;
// The core's output lanes (core/src/master.rs, OUT): its out buffer holds
// this many runs whatever it renders.
const LANES = 2;
// How often it says what rendering costs (queue item 66), in seconds of
// audio: the time spent in process() against the audio it made, mono and
// stereo apart. The worklet's clock is the platform's: performance.now()
// where the scope has it, else Date.now(), whose whole milliseconds still
// add up to the time spent over many blocks (each call's start falls
// anywhere within its millisecond).
const COST_EVERY = 2;
// How busy the audio thread is (queue item 87b): the time process() took
// over each quarter-second of audio, as a share of that quarter-second (the
// mean block's cost against its deadline). The worklet's clock is whole
// milliseconds in Chromium (no performance.now() in an AudioWorklet), too
// coarse for one block of 2.7 ms but exact on average over a window's
// hundred-odd blocks. Each cost report carries the busiest window since the
// last, and how many windows took longer than the audio they made (the
// render falling behind for its own cost).
const BUSY_WINDOW = 0.25;
const clock = typeof globalThis.performance?.now === 'function' ? () => globalThis.performance.now() : () => Date.now();

class DriftloomCore extends AudioWorkletProcessor {
  constructor(options) {
    super();
    this.alive = true;
    this.core = null;
    this.views = null;
    // What rendering has cost since the node was built, mono then stereo:
    // milliseconds spent, and seconds of audio made.
    this.cost = { ms: [0, 0], seconds: [0, 0] };
    this.costAt = 0;
    // The window being counted, and since the last report: the busiest
    // window's share, the windows over their whole time, and how many.
    this.busy = { ms: 0, frames: 0, max: 0, behind: 0, windows: 0 };
    // A test's slow device (queue item 87b, `?burn=` in the page): this
    // many microseconds of plain arithmetic added to every block, inside the
    // timed render, so the measuring and the rule see an audio thread that
    // much slower. The count of operations follows the clock as it goes
    // (every 64 blocks, the time they took against the time asked), since
    // the browser's compiler speeds the loop up as it runs.
    this.burnUs = Number(options.processorOptions && options.processorOptions.burn) || 0;
    this.burn = 0;
    this.burnMs = 0;
    this.burnBlocks = 0;
    this.sink = 1;
    if (this.burnUs > 0) {
      // A first count from ten milliseconds of it, as the node starts.
      let n = 0;
      let x = 1;
      const t0 = clock();
      while (clock() - t0 < 10) {
        for (let i = 0; i < 1000; i++) x = x * 1.0000001 + 1e-9;
        n += 1000;
      }
      this.sink = x;
      this.burn = Math.max(1, Math.round((this.burnUs * n) / ((clock() - t0) * 1000)));
    }
    try {
      const module = new WebAssembly.Module(options.processorOptions.bytes);
      this.core = new WebAssembly.Instance(module, {}).exports;
      this.core.dl_init(sampleRate);
      // The mix: three combs on lite, six on full.
      if (!this.core.dl_quality(options.processorOptions.quality === 'lite' ? 0 : 1)) {
        throw new Error(`no mix at ${sampleRate} Hz`);
      }
      if (options.processorOptions.layout === 2 && !this.core.dl_layout(2)) {
        throw new Error('no stereo render');
      }
      // The player's events waiting to be posted, the grids read with them,
      // the requests it refused, and when the events are next posted.
      this.events = [];
      this.grids = [];
      this.urgent = false;
      this.eventsAt = 0;
      this.refused = 0;
      if (typeof this.core.dl_play_attach !== 'function') throw new Error('no player in this module');
      // The core's player on the core: it decides each block's steps
      // before the block (dl_play_process).
      this.core.dl_play_attach(options.processorOptions.quality === 'lite' ? 0 : 1);
    } catch (err) {
      // Too old a browser for the module, most likely (it needs Safari 15
      // or later). The page hears this and says it cannot play.
      this.core = null;
      this.port.postMessage({ type: 'failed', message: String(err && err.message || err) });
    }
    this.port.onmessage = (e) => this.receive(e.data);
  }

  receive(m) {
    const core = this.core;
    switch (m.type) {
      // The listener's Mono switch (queue item 66): Core::set_layout, which
      // fades while the core plays.
      case 'layout':
        if (core) core.dl_layout(m.channels);
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
      // The core's player (queue items 82 and 94b; CoreHost.playLoad and
      // the rest): what a loop asks for (`ask`), then the load, the change or
      // the re-roll that takes it.
      case 'playLoad':
        if (core) {
          this.ask(m.ask);
          if (m.resumed) core.dl_play_load_resumed(m.seed >>> 0, m.at || 0);
          else core.dl_play_load(m.seed >>> 0, m.at || 0);
        }
        return;
      case 'playChange':
        if (core) {
          this.ask(m.ask);
          core.dl_play_change(m.seed >>> 0, m.moment >>> 0, m.fade);
        }
        return;
      case 'playRevise':
        if (core) {
          this.ask(m.ask);
          core.dl_play_revise(m.seed >>> 0);
        }
        return;
      // The length slider (queue item 94b): the track's length, and the loop
      // that follows when it has run (a change at the track's end, 5).
      case 'playTrack':
        if (core) {
          core.dl_play_track(m.kind >>> 0, m.count >>> 0);
          this.arm(m.arm);
          this.urgent = true;
        }
        return;
      case 'playArm':
        if (core) this.arm(m.arm);
        return;
      // The picks (queue item 95): the New loops panel's request, held by the
      // core for every loop it makes itself, numbered by the page. The page
      // hears whether they could be met whole (if not, the core holds their
      // vibes only) and why not, in the core's own words.
      case 'playPicks':
        if (core) {
          this.ask(m.ask, false);
          const met = core.dl_play_picks(m.tag >>> 0) === 1;
          const n = core.dl_play_refusal();
          const why = n ? String.fromCharCode(...new Uint8Array(core.memory.buffer, core.dl_play_refusal_at(), n)) : null;
          this.port.postMessage({ type: 'picks', tag: m.tag >>> 0, met, why });
        }
        return;
      // The seed the core draws its own loops' seeds from (queue item 95).
      case 'playDraws':
        if (core) core.dl_play_draws(m.seed >>> 0);
        return;
      // A new loop of the core's own (the app's Next at the end of its
      // history), with the picks.
      case 'playNext':
        if (core) {
          core.dl_play_next(m.moment >>> 0, m.fade);
          this.urgent = true;
        }
        return;
      case 'playTone':
        if (core) core.dl_play_tone(m.warmth, m.space, m.wobble);
        return;
      // What the player says about the loop now, for the checks.
      case 'probe':
        if (core) {
          const n = core.dl_play_describe();
          const text = String.fromCharCode(...new Uint8Array(core.memory.buffer, core.dl_play_about_at(), n));
          this.port.postMessage({
            ...this.counts(),
            type: 'probe', id: m.id, about: text, lanes: core.dl_lanes(),
            player: { state: core.dl_play_state(), step: core.dl_play_step(), time: core.dl_play_time() },
            trackEnd: core.dl_play_track_end(), nextChange: core.dl_play_next_change(), lost: core.dl_play_events_lost(), refused: this.refused,
            picksTag: core.dl_play_picks_tag(),
          });
        } else {
          this.port.postMessage({ type: 'probe', id: m.id, about: null });
        }
        return;
      case 'playStop':
        if (core) core.dl_play_stop();
        return;
      case 'playResume':
        if (core) core.dl_play_resume(m.at || 0);
        return;
      case 'playWander':
        if (core) {
          core.dl_play_wander(m.amount);
          // And its salt (queue item 92): 0, every code's own wander.
          core.dl_play_wander_salt((m.salt || 0) >>> 0);
        }
        return;
      case 'playMute':
        if (core) core.dl_play_mute(m.layer, m.muted ? 1 : 0);
        return;
      case 'playTempo':
        if (core) core.dl_play_tempo(m.bpm);
        return;
      case 'playVolume':
        if (core) core.dl_play_volume(m.volume);
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
          // The core's player (queue item 82): where it is (0 idle, 1 making a
          // loop, 2 its first pass, 3 playing) and the step it decides next.
          player: core ? { state: core.dl_play_state(), step: core.dl_play_step(), time: core.dl_play_time() } : null,
        });
        return;
      default:
    }
  }

  // What a loop asks for (queue item 94b; `askOf` in js/player.js): the
  // request, then a code's edits, each a call the core may refuse (counted
  // in `refused`, for Diagnostics, unless `count` is false: the picks say
  // their own refusals).
  ask(a, count = true) {
    const core = this.core;
    if (!core || !a) return;
    let refused = 0;
    const take = (ok) => { if (!ok) refused++; };
    // Each vibe with its weight, or none (a listener's pick, queue item 95),
    // the tempo (queue item 85).
    for (const [i, w] of a.vibes || []) take(w == null ? core.dl_play_vibe(i) : core.dl_play_weight(i, w));
    if (a.tempo) take(core.dl_play_ask_tempo(a.tempo[0], a.tempo[1]));
    // Its pins (queue item 85b): [number, value] pairs.
    for (const [i, v] of a.pins || []) take(core.dl_play_pin(i, v));
    // Its theme (queue item 85d): a seed.
    if (a.theme != null) core.dl_play_theme(a.theme >>> 0);
    // Its length (queue item 88): bars, or Endless.
    if (a.length != null) take(core.dl_play_length(a.length === 'endless' ? Infinity : a.length));
    // Its keys and modes (queue item 89), and styles (90): a bit each.
    if (a.keys) take(core.dl_play_keys(a.keys));
    if (a.scales) take(core.dl_play_scales(a.scales));
    if (a.styles) take(core.dl_play_styles(a.styles));
    // Its voices and layers (queue item 91): a mask and a state for each
    // layer, in the layers' order.
    (a.voices || []).forEach((mask, layer) => { if (mask) take(core.dl_play_voices(layer, mask)); });
    (a.layers || []).forEach((state, layer) => { if (state) take(core.dl_play_layer(layer, state)); });
    // A code's edits (queue item 94a): each layer's seed, re-rolled or not,
    // the tone and the tempo the slider set.
    for (const [layer, seed, rolled] of a.seeds || []) take(core.dl_play_seed(layer, seed >>> 0, rolled ? 1 : 0));
    (a.tone || []).forEach((v, which) => take(core.dl_play_edit_tone(which, v)));
    if (a.bpm != null) take(core.dl_play_edit_bpm(a.bpm));
    if (count) this.refused += refused;
  }

  // The loop that follows when the track ends (queue item 94b): a change
  // waiting for the track's end, the old loop falling to nothing over `fade`
  // at a phrase line inside a pass (ringing out at a pass's end), with the
  // track that loop plays (`track`, [kind, count]; queue item 95). None
  // (null) drops the one waiting, so the core's own loop follows instead.
  arm(arm) {
    const core = this.core;
    if (!core) return;
    if (!arm) {
      core.dl_play_drop(END);
      return;
    }
    this.ask(arm.ask);
    if (arm.track) core.dl_play_change_track(arm.seed >>> 0, END, arm.fade, arm.track[0] >>> 0, arm.track[1] >>> 0);
    else core.dl_play_change(arm.seed >>> 0, END, arm.fade);
  }

  // The player's events from the block just rendered (queue item 94b): taken
  // after every block, so none is lost, and posted to the page every
  // EVENTS_EVERY seconds of audio, or at once with one that is not a step.
  // With an event that begins a pass (a pass, a section, a new loop, a
  // re-roll), the layers' grid of that pass, read now (`Player::grid` is the
  // pass being decided).
  takeEvents(block) {
    const core = this.core;
    const n = core.dl_play_events();
    if (n) {
      const views = this.views;
      if (!views.events) views.events = new Float64Array(views.buffer, core.dl_play_event_at(), 4 * EVENTS_MAX);
      const f = views.events;
      for (let i = 0; i < n; i++) {
        const kind = f[4 * i + 1];
        if (kind !== STEP) this.urgent = true;
        if (kind === PASS || kind === SECTION || kind === LOOP || kind === REVISED) {
          const grid = [];
          for (let l = 0; l < LAYER_COUNT; l++) {
            const k = core.dl_play_grid(l);
            grid.push(Array.from(new Float64Array(views.buffer, core.dl_play_grid_at(), k)));
          }
          this.grids.push({ at: this.events.length / 4, grid });
        }
        this.events.push(f[4 * i], kind, f[4 * i + 2], f[4 * i + 3]);
      }
    }
    if (!(this.urgent || (this.events.length && block >= this.eventsAt))) return;
    this.eventsAt = block + Math.round(EVENTS_EVERY * sampleRate);
    this.urgent = false;
    this.port.postMessage({
      type: 'events', events: this.events, grids: this.grids,
      trackEnd: core.dl_play_track_end(), lost: core.dl_play_events_lost(), refused: this.refused,
    });
    this.events = [];
    this.grids = [];
  }

  // The late and dropped notes so far, by kind.
  counts() {
    const core = this.core;
    const by = [];
    for (let c = 0; c < CAUSES; c++) by.push(core ? core.dl_dropped_by(c) : 0);
    return {
      type: 'counts',
      late: core ? core.dl_late() : 0,
      missed: core ? core.dl_missed() : 0,
      dropped: core ? core.dl_dropped() : 0,
      droppedBy: by,
    };
  }

  // What the block just rendered cost, counted with the lanes it had; and
  // every COST_EVERY seconds of audio, the totals so far.
  costReport(lanes, ms, block) {
    const i = lanes === 2 ? 1 : 0;
    this.cost.ms[i] += ms;
    this.cost.seconds[i] += QUANTUM / sampleRate;
    const b = this.busy;
    b.ms += ms;
    b.frames += QUANTUM;
    if (b.frames >= BUSY_WINDOW * sampleRate) {
      const share = b.ms / ((b.frames / sampleRate) * 1000);
      if (share > b.max) b.max = share;
      if (share >= 1) b.behind++;
      b.windows++;
      b.ms = 0;
      b.frames = 0;
    }
    if (block < this.costAt) return;
    this.costAt = block + Math.round(COST_EVERY * sampleRate);
    const core = this.core;
    // And where the player is.
    const player = core ? { state: core.dl_play_state(), step: core.dl_play_step(), time: core.dl_play_time() } : null;
    const busy = { max: b.max, behind: b.behind, windows: b.windows, seconds: b.windows * BUSY_WINDOW };
    b.max = 0;
    b.behind = 0;
    b.windows = 0;
    // The core's counts ride with it: its late, left-out and dropped notes.
    this.port.postMessage({ ...this.counts(), type: 'cost', ms: this.cost.ms.slice(), seconds: this.cost.seconds.slice(), player, busy });
  }

  // The test's slow device (`burn`): arithmetic the optimizer can't drop,
  // its count kept to the time asked.
  burnFor() {
    const t0 = clock();
    let x = this.sink;
    for (let i = 0; i < this.burn; i++) x = x * 1.0000001 + 1e-9;
    this.sink = x;
    this.burnMs += clock() - t0;
    if (++this.burnBlocks >= 64) {
      const took = (this.burnMs * 1000) / this.burnBlocks;
      const scale = took > 0 ? this.burnUs / took : 2;
      this.burn = Math.max(1, Math.round(this.burn * Math.min(1.5, Math.max(0.67, scale))));
      this.burnMs = 0;
      this.burnBlocks = 0;
    }
  }

  process(_inputs, outputs) {
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
        out: runs(buffer, core.dl_out(), LANES),
      };
    }
    const views = this.views;
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
    const began = clock();
    core.dl_play_process(block);
    this.takeEvents(block);
    if (this.burn) this.burnFor();
    // Each of the output's channels from a lane the core rendered: the
    // mono render in every one, or left and right.
    const lanes = core.dl_lanes();
    const out = outputs[0];
    if (out) {
      for (let c = 0; c < out.length; c++) {
        if (out[c].length === QUANTUM) out[c].set(views.out[Math.min(c, lanes - 1)]);
      }
    }
    this.costReport(lanes, clock() - began, block);
    return true;
  }
}

registerProcessor('driftloom-core', DriftloomCore);
