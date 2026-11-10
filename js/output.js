// The core's node, into the speakers (queue item 97).
//
// The core plays everything: its player decides the notes, its voices make
// them, its mix and master chain finish them (core/, through
// js/dlplayer.wasm in js/worklet.js). What is left for the page is where
// that sound goes and the few things only a listener's device decides: the
// volume, the transport's silence, mono or stereo, and the quality the core
// mixes at. This is that: one core's node on one context, and nothing of
// Web Audio besides. Until item 97 it was a JavaScript synth of its own
// (js/synth.js), which played whatever the core could not; a browser that
// cannot run the core now plays nothing, and the page says so in a plain
// line (Mikey, 2026-11-07).

import { CoreHost, MIX, CANCEL, HOLD, LINEAR, SET, DROP_CAUSES } from './core.js';

export class Output {
  // `core` is what loadCore() gave for this context, if it has arrived; one
  // that arrives later is handed over with attachCore(). `onCoreError`
  // hears it, once, if the core turns out not to run, at any time.
  constructor(ctx, quality = 'full', { core = null, coreError = null, onCoreError = null, layout = 2, burn = 0 } = {}) {
    this.ctx = ctx;
    this.quality = quality;
    // The core's render: 1 mono, 2 stereo (queue item 64; `setLayout`).
    this.layout = layout === 2 ? 2 : 1;
    this.onCoreError = onCoreError;
    // How busy the core's audio thread is (queue item 87b): called with each
    // of its reports ({ max, behind, windows, seconds }); and a test's slow
    // device, microseconds added to each of its blocks (`?burn=`).
    this.onBusy = null;
    this._burn = burn;
    this.core = null;
    // Why the core is not coming, if it is not: Diagnostics shows it.
    this.coreError = coreError;
    // What reaches the speakers: the core's node, once there is one.
    this.output = null;
    // The listener's volume, and whether the transport silenced the output.
    this.userVolume = null;
    this._killed = false;
    if (core) this.attachCore(core);
  }

  attachCore(core) {
    if (this.core || this.coreError) return;
    try {
      this.core = new CoreHost(this.ctx, core, { quality: this.quality, layout: this.layout, burn: this._burn });
    } catch (err) {
      this.coreFailed(err);
      return;
    }
    this.core.onbusy = (busy) => { if (this.onBusy) this.onBusy(busy); };
    this.core.onfail = () => {
      // The app keeps this answer for the session: a later output (the
      // quality toggle) does not build a core that has failed.
      this.coreError = this.core.failed;
      if (this.onCoreError) this.onCoreError(this.coreError);
    };
    this.core.node.connect(this.ctx.destination, 0);
    this.output = this.core.node;
    if (this._killed) this.core.param(MIX.kill, SET, 0, 0);
    if (this.userVolume != null) this.core.playVolume(this.userVolume);
  }

  // The core will not come (it did not load, or its node could not be
  // built): nothing plays, and the page says why.
  coreFailed(err) {
    if (this.core) return;
    this.coreError = String((err && err.message) || err);
    if (this.onCoreError) this.onCoreError(this.coreError);
  }

  // The core, when it is running.
  _core() {
    return this.core && !this.core.failed ? this.core : null;
  }

  // The listener's Mono switch (queue item 66): the core's render from
  // now, mono (1) or stereo (2), faded so a loop playing goes on; a core
  // still on its way starts in it.
  setLayout(layout) {
    this.layout = layout === 2 ? 2 : 1;
    const core = this._core();
    if (core) core.setLayout(this.layout);
  }

  // The core's player sets the master gain itself, at each loop it starts
  // and when it is told the listener's volume.
  setVolume(v) {
    this.userVolume = v;
    const core = this._core();
    if (core) core.playVolume(v);
  }

  // A switch of quality hands over at `from` (queue item 87b): what the
  // player handed the core for `from` or later is dropped, and the mix's
  // own events from then on with it (the tails' fades and the ducks).
  cancelFrom(from) {
    const core = this._core();
    if (!core) return;
    core.cancel(from);
    core.param(MIX.tails, CANCEL, 0, from);
    core.param(MIX.pump, CANCEL, 0, from);
  }

  // Silence everything already in flight. Not a mute: the transport stopped.
  silence(when = this.ctx.currentTime) {
    this._killed = true;
    const core = this._core();
    if (!core) return;
    core.param(MIX.kill, HOLD, 0, when);
    core.param(MIX.kill, LINEAR, 0, when + 0.06);
  }

  // The whole output falls to nothing from `from` to `to` and stays there
  // (queue item 87b: the quality switch hands over at a boundary, the old
  // core fading out before it). Like `silence`, over a fade of its own.
  fadeOut(from, to) {
    this._killed = true;
    const core = this._core();
    if (!core) return;
    core.param(MIX.kill, HOLD, 0, from);
    core.param(MIX.kill, LINEAR, 0, to);
  }

  unsilence(when = this.ctx.currentTime) {
    this._killed = false;
    const core = this._core();
    if (!core) return;
    core.param(MIX.kill, HOLD, 0, when);
    core.param(MIX.kill, LINEAR, 1, when + 0.02);
  }

  // Let the core's node go: its notes stop with it.
  dispose() {
    if (this.core) this.core.dispose();
    this.output = null;
  }

  // For Diagnostics: the core's player, with the notes it took late, left
  // out or dropped; or why there is none.
  engineReport() {
    const c = this.core;
    if (!c) return this.coreError ? `none (${this.coreError})` : 'rust (loading)';
    if (c.failed) return `none (${c.failed})`;
    const p = c.playerAt;
    const at = p ? `  player: ${['idle', 'making a loop', 'its first pass', 'playing'][p.state] || p.state}, step ${p.step}` : '';
    const dropped = c.dropped ? ` (${DROP_CAUSES.map((cause, i) => `${cause} ${c.droppedBy[i] || 0}`).join(', ')})` : '';
    return `rust (the core's player)  late: ${c.late}  missed: ${c.missed}  dropped: ${c.dropped}${dropped}${at}`;
  }
}
