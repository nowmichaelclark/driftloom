// Exporting audio (queue item 104b; docs/PAINTER.md, "Exporting audio"): the
// sheet in Save's Share tab, and the Worker that makes the file
// (js/export-worker.js, its own instance of the core).
//
// Safety first (Mikey, 2026-10-13): an export must never crash or break
// anything, whatever the device. So the file is written as it is made (the
// save picker where the browser has it, else the site's private file
// system and a download at the end), the free space is checked before
// anything starts, a browser that can do neither is told the most it can
// keep in memory before it starts, Cancel and every failure leave no file
// behind, and the music playing is never touched: the export's core is
// another instance on another thread, and "lighter on processor" makes it
// gentle (a quarter of one core). While it runs, the audio thread's busy
// reports are the export's to read (`watchBusy`): a busy audio thread moves
// it to gentle, and they never move the app to lite (the export's load is
// not the device's).
//
// The format is chosen each time: Lossless (FLAC, the core's own writer) or
// Small (queue item 104c: Opus through the browser's own encoder, written
// into an Ogg file by js/ogg.js, in the Worker). Small shows only where the
// Worker says the browser can encode Opus (never a fallback that could fail
// on a weak phone); a browser with AAC and not Opus has it hidden too.

import { askOf } from './player.js';
import { coreTrack, trackOfSpec } from './tracklen.js';
import { LAYERS, pieceOf, loopBars } from './request.js';
import { fetchCore } from './core.js';
import { newSpec } from './gen.js';
import * as ui from './ui.js';
import { TAIL_MAX } from './export-render.js';
import { appLink } from './app.js';
import { moduleWorkerUrl } from './worker-blob.js';

// The most a browser with no file to write to may keep in memory
// (`MEMORY_SECONDS`, with the bytes it takes stated as `MEMORY_BYTES`, its
// hard stop).
const MEMORY_SECONDS = 600;
const MEMORY_BYTES = 160 * 1024 * 1024;
// The bytes a second of FLAC takes, for the size shown and the space
// checked, by lanes: the most any of 24 corpus loops of a minute took
// (tools/export.mjs --sizes, 44.1 kHz 16-bit at full quality: 27,688 mono
// and 47,662 stereo, the medians 18,212 and 32,505; session 20), with a
// third more. And the margin the free space must hold beyond it.
const BYTES_PER_SECOND = { 1: 36000, 2: 64000 };
// And of Small's Opus, by lanes: its bit rates (js/export-opus.js: 64 and 96
// kbit/s, constant) and the Ogg pages' 0.8%, rounded up.
const SMALL_BYTES_PER_SECOND = { 1: 8100, 2: 12100 };
const MARGIN = 1.25;
// A busy audio thread while exporting (its busiest quarter-second, as a
// share of its own time; js/quality.js's reports): past this the export goes
// gently for the rest of its run.
const BUSY_LINE = 0.5;

// The sheet's words (for Mikey to approve: docs/notes/for-mikey.md).
export const WORDS = {
  intro: 'Save what plays as an audio file, to keep or to play anywhere.',
  lossless: 'Lossless: full quality, as a FLAC file.',
  small: 'Small: much smaller, as an Opus file, for sharing and for phones.',
  whole: (t) => `The whole piece, ${t}.`,
  track: (t) => `As long as the track: ${t}.`,
  ask: 'This plays on and on, so you set how long first.',
  memoryLong: (t) => `This browser can keep ${t} at most: set a shorter track length.`,
  gentle: 'Lighter on processor is on, so it goes gently: slower, and the music never stutters.',
  fast: 'It goes as fast as this device safely can.',
  picker: 'You choose where it goes.',
  space_app: (need, free) => `There isn't enough free space in that place: this needs about ${need}, and it has about ${free}. Pick another place, or a shorter length.`,
  seek: "That place can't take a file as it is made. Pick one on this device, such as Downloads or Documents.",
  private: 'It downloads when it is done.',
  memory: (t) => `This browser can't write a file as it goes, so it keeps the export in memory: ${t} at most.`,
  progress: (pct, left) => `${pct}% done${left ? `, about ${left} left` : ''}.`,
  background: 'It keeps going in the background while the music plays. Closing the app stops it, and nothing is saved.',
  saved: (name, size, t) => `Saved: ${name}, ${size}, ${t}.`,
  savedPicked: (name, size, t) => `Saved where you chose: ${name}, ${size}, ${t}.`,
  cancelled: 'Cancelled. Nothing was saved.',
  space: (need, free) => `There isn't enough free space for this: it needs about ${need}, and this device has about ${free}. A shorter length would fit.`,
  spaceLate: "This device ran out of space before the export finished, so nothing was saved. Free some space, or try a shorter length. The music wasn't touched.",
  memoryFull: 'That was more than this browser can hold, so nothing was saved. Try a shorter length.',
  failed: "Something went wrong making the file, so nothing was saved. The music wasn't touched; you can try again.",
  cant: "This browser can't export audio. A recent Chrome, Firefox or Safari can.",
};

const el = (id) => document.getElementById(id);

// A duration in words: "45 s", "12 min", "2 h 5 min".
export function words(secs) {
  if (!(secs >= 0) || !Number.isFinite(secs)) return '';
  if (secs < 60) return `${Math.max(1, Math.round(secs))} s`;
  if (secs < 3600) return `${Math.round(secs / 60)} min`;
  const h = Math.floor(secs / 3600);
  const m = Math.round((secs - h * 3600) / 60);
  return m ? `${h} h ${m} min` : `${h} h`;
}

// A size in words: "38 MB", "1.2 GB".
export function size(bytes) {
  if (bytes >= 1e9) return `${(bytes / 1e9).toFixed(1)} GB`;
  return `${Math.max(1, Math.round(bytes / 1e6))} MB`;
}

const bar = (spec) => (spec.stepsPerBar || 16) * (60 / spec.bpm / 4);

// What an export of `spec` is (session 30c: the export is the track
// length; Mikey, 2026-11-18): { key, seconds, track }, the track as the core
// takes it ([kind, count]). The time the Track length view set, with its
// fades; with none, a finite piece whole (its fade in, if set); else null:
// a loop that plays on and on, or Endless, asks how long first (the export
// key opens the Track length view).
export function planOf(spec) {
  const t = trackOfSpec(spec);
  if (t.secs != null) return { key: 'track', seconds: t.secs, track: coreTrack(spec) };
  const piece = pieceOf(spec);
  if (piece && Number.isFinite(piece.sections)) {
    return { key: 'whole', seconds: loopBars(spec) * bar(spec), track: [1 | (t.fadeIn ? 4 : 0), 1] };
  }
  return null;
}

// The loop's name for the file: its own, or (a loop with none of its own)
// the name the core makes for it, made safe (as ui.download makes it).
function loopName(spec) {
  let name = String(spec.name || '').trim();
  if (!name) {
    try { name = newSpec(spec.seed >>> 0, null).name || ''; } catch { name = ''; }
  }
  return String(name).replace(/[\\/:*?"<>|\u0000-\u001f]/g, '-').replace(/\s+/g, ' ').replace(/^[.\s]+/, '').slice(0, 100) || 'loop';
}

// What each format is called in a file: its extension and media type.
export const FORMATS = {
  lossless: { ext: 'flac', type: 'audio/flac', description: 'FLAC audio', bytes: BYTES_PER_SECOND },
  small: { ext: 'opus', type: 'audio/ogg', description: 'Opus audio', bytes: SMALL_BYTES_PER_SECOND },
};

export function fileName(spec, format = 'lossless') {
  return `${loopName(spec)}.${FORMATS[format].ext}`;
}

// The sheet's states: 'choose', 'run', 'done', 'failed'.
export class AudioExport {
  // `host`: { spec(), engine(), mono(), lite() } from the app.
  constructor(host) {
    this.host = host;
    this.worker = null;
    // How the file is written here: 'app' (the Android app's own picker, its
    // bytes handed to the app as they are made: session 28), 'picker',
    // 'private' or 'memory'; null until the Worker has said, false if it
    // can't export at all.
    this.mode = null;
    // The format chosen ('lossless' or 'small'), and whether this browser can
    // make Small (the Worker says: false until it has).
    this.format = 'lossless';
    this.opus = false;
    this.running = null;
    this.result = null;
    this.timer = null;
    // The Android app's end (js/app.js), or null in a browser.
    this.app = appLink();
    this.wire();
  }

  wire() {
    el('exportIntro').textContent = WORDS.intro;
    el('exportFormat').textContent = WORDS.lossless;
    for (const b of document.querySelectorAll('[data-format]')) {
      b.addEventListener('click', () => {
        if (b.hidden || this.running) return;
        this.format = b.dataset.format;
        this.refresh();
      });
    }
    el('exportAudio').addEventListener('click', () => this.start());
    el('exportCancel').addEventListener('click', () => this.cancel());
    el('exportOk').addEventListener('click', () => this.show('choose'));
    el('exportDoneBtn').addEventListener('click', () => this.show('choose'));
    el('exportSave').addEventListener('click', () => this.save());
    el('exportOpen').addEventListener('click', () => this.open());
    el('exportShare').addEventListener('click', () => this.share());
    for (const b of document.querySelectorAll('[data-stab="share"]')) b.addEventListener('click', () => this.refresh());
    // While the sheet's choice shows, it follows the loop playing (a track
    // that ended, a loop picked elsewhere): a look every second and a half,
    // and none while it is out of sight.
    this.timer = setInterval(() => {
      const sheet = el('exportChoose');
      if (sheet && !sheet.hidden && sheet.offsetParent !== null) this.refresh();
    }, 1500);
    this.show('choose');
  }

  // The Worker, started once; it says whether the private file system works.
  // In the Android app its imports are loaded by the page (js/worker-blob.js).
  boot() {
    if (this.worker || this.booting || this.mode === false) return;
    this.booting = true;
    const address = new URL('./export-worker.js', import.meta.url).href;
    Promise.resolve(this.app ? moduleWorkerUrl(address) : address).then((url) => {
      this.booting = false;
      this.worker = new Worker(url, { type: 'module' });
      this.worker.onmessage = (e) => this.receive(e.data);
      this.worker.onerror = (e) => {
        // A Worker that won't start (no module workers) or that died.
        e.preventDefault?.();
        console.error(`export worker: ${e.message || 'would not start'} ${e.filename || ''}:${e.lineno || 0}`);
        if (this.running) this.fail('write', String(e.message || 'the export stopped'));
        else {
          this.mode = false;
          this.worker = null;
          this.refresh();
        }
      };
      this.worker.postMessage({ type: 'hello' });
    }).catch((err) => {
      this.booting = false;
      console.error(`export worker: ${err && err.message}`);
      this.mode = false;
      this.refresh();
    });
  }

  receive(m) {
    if (m.type === 'hello') {
      this.mode = this.app ? 'app' : typeof window.showSaveFilePicker === 'function' ? 'picker' : m.private ? 'private' : 'memory';
      this.opus = !!m.opus;
      this.refresh();
    } else if (m.type === 'relay' && this.running) {
      this.relay(m);
    } else if (m.type === 'progress' && this.running) {
      this.progress(m);
    } else if (m.type === 'done' && this.running) {
      this.done(m);
    } else if (m.type === 'cancelled' && this.running) {
      this.running = null;
      if (this.app) this.app.exporting(false);
      el('exportWhy').textContent = WORDS.cancelled;
      this.show('failed');
    } else if (m.type === 'failed' && this.running) {
      this.fail(m.why, m.message);
    }
  }

  // The app's file (session 28): the Worker's bytes go to the app, which
  // writes them where the listener chose, and the answer goes back.
  relay(m) {
    this.app.request({ type: `save-${m.op}`, data: m.data }).then((r) => {
      if (this.worker) this.worker.postMessage({ type: 'ack', id: m.id, ok: !!(r && r.ok), why: r && r.why });
    });
  }

  // Which states show.
  show(which) {
    for (const [id, state] of [['exportChoose', 'choose'], ['exportRun', 'run'], ['exportDone', 'done'], ['exportFailed', 'failed']]) el(id).hidden = which !== state;
    if (which === 'choose') this.refresh();
  }

  // The choice, for the loop playing now.
  refresh() {
    this.boot();
    const spec = this.host.spec();
    const about = el('exportAbout');
    const go = el('exportAudio');
    if (this.mode === false || !spec) {
      about.textContent = this.mode === false ? WORDS.cant : '';
      go.disabled = true;
      return;
    }
    // Small shows only where it works; chosen where it doesn't, Lossless.
    if (!this.opus) this.format = 'lossless';
    const small = document.querySelector('[data-format="small"]');
    small.hidden = !this.opus;
    for (const b of document.querySelectorAll('[data-format]')) {
      b.setAttribute('aria-checked', String(b.dataset.format === this.format));
      b.disabled = !!this.running;
    }
    el('exportFormat').textContent = this.format === 'small' ? WORDS.small : WORDS.lossless;
    const plan = planOf(spec);
    const memory = this.mode === 'memory';
    const tooLong = memory && plan && plan.seconds > MEMORY_SECONDS;
    const lanes = this.host.mono() ? 1 : 2;
    const lines = [];
    if (!plan) lines.push(WORDS.ask);
    else if (plan.key === 'whole') lines.push(WORDS.whole(words(plan.seconds)));
    else lines.push(WORDS.track(words(plan.seconds)));
    if (tooLong) lines.push(WORDS.memoryLong(words(MEMORY_SECONDS)));
    else if (plan) lines.push(`About ${size((plan.seconds + TAIL_MAX) * FORMATS[this.format].bytes[lanes])}.`);
    lines.push(this.mode === 'picker' || this.mode === 'app' ? WORDS.picker : this.mode === 'private' ? WORDS.private : this.mode === 'memory' && !tooLong ? WORDS.memory(words(MEMORY_SECONDS)) : '');
    lines.push(this.pace() === 'gentle' ? WORDS.gentle : WORDS.fast);
    about.textContent = lines.filter(Boolean).join(' ');
    // With no end, or too long to keep, the key opens the Track length view.
    go.disabled = this.mode == null || !!this.running;
    go.dataset.asks = !plan || tooLong ? 'length' : '';
  }

  // "Lighter on processor" on: gentle. Off: as fast as is safe, which on a
  // device of two cores or fewer is gentle too.
  pace() {
    if (this.host.lite()) return 'gentle';
    const cores = navigator.hardwareConcurrency || 2;
    return cores <= 2 ? 'gentle' : 'fast';
  }

  // The switch moved while an export runs.
  setPace() {
    if (this.running && !this.running.slowed) this.worker.postMessage({ type: 'pace', pace: this.pace() });
    if (!this.running) this.refresh();
  }

  // A report from the audio thread (js/main.js's onBusy) while an export
  // runs: true if the export took it (the app's own rule leaves it alone).
  watchBusy(report) {
    if (!this.running) return false;
    if (report && report.max > BUSY_LINE && !this.running.slowed && this.worker) {
      this.running.slowed = true;
      this.worker.postMessage({ type: 'pace', pace: 'gentle' });
    }
    return true;
  }

  async start() {
    if (this.running || !this.mode) return;
    const spec = this.host.spec();
    const engine = this.host.engine();
    if (!spec) return;
    const chosen = planOf(spec);
    // Nothing would end it, or it is longer than this browser can keep: how
    // long, first (the Track length view, which exports on done).
    if (!chosen || (this.mode === 'memory' && chosen.seconds > MEMORY_SECONDS)) {
      this.host.openTrack();
      return;
    }
    const lanes = this.host.mono() ? 1 : 2;
    const format = this.format === 'small' && this.opus ? 'small' : 'lossless';
    const need = (chosen.seconds + TAIL_MAX) * FORMATS[format].bytes[lanes] * MARGIN;
    let name = fileName(spec, format);
    // Free space first (the origin's share of the disk, as the browser
    // counts it; the picker's file lands outside it, but on the same disk).
    if (this.mode !== 'memory' && this.mode !== 'app') {
      try {
        const est = navigator.storage && navigator.storage.estimate ? await navigator.storage.estimate() : null;
        const free = est && Number.isFinite(est.quota) && Number.isFinite(est.usage) ? est.quota - est.usage : null;
        if (free != null && free < need) {
          el('exportWhy').textContent = WORDS.space(size(need), size(Math.max(0, free)));
          this.show('failed');
          return;
        }
      } catch { /* no estimate here: the write itself says if the disk fills */ }
    } else if (need / MARGIN > MEMORY_BYTES) {
      el('exportWhy').textContent = WORDS.memoryFull;
      this.show('failed');
      return;
    }
    let handle = null;
    if (this.mode === 'picker') {
      try {
        handle = await window.showSaveFilePicker({ suggestedName: name, types: [{ description: FORMATS[format].description, accept: { [FORMATS[format].type]: [`.${FORMATS[format].ext}`] } }] });
      } catch (err) {
        // Closed without choosing: nothing to do. Anything else: the
        // private file system instead.
        if (err && err.name === 'AbortError') return;
        handle = null;
      }
    }
    let saved = null;
    if (this.mode === 'app') {
      // Android's own picker, then the platform's free space for the place
      // chosen; FLAC's header is written again at the end, which a place
      // that only streams can't do (the app says so, and removes the file).
      const r = await this.app.request({ type: 'save', name, mime: FORMATS[format].type, need: Math.ceil(need), seek: format === 'lossless' });
      if (!r || !r.ok) {
        if (r && r.cancelled) return;
        el('exportWhy').textContent = r && r.why === 'space' ? WORDS.space_app(size(need), size(Math.max(0, r.free))) : r && r.why === 'seek' ? WORDS.seek : WORDS.failed;
        this.show('failed');
        return;
      }
      saved = r;
    }
    let bytes;
    try {
      bytes = await fetchCore();
    } catch {
      if (saved) this.app.send({ type: 'save-abort' });
      this.fail('render', 'no core');
      return;
    }
    const job = {
      seed: spec.seed >>> 0,
      ask: askOf(spec),
      mutes: LAYERS.map((l) => !!(spec.mutes && spec.mutes[l])),
      wander: engine && engine.driftOn ? engine.driftAmount : 0,
      salt: engine ? engine.wanderSalt : 0,
      layout: lanes,
      track: chosen.track,
      format: format === 'small' ? 'opus' : 'flac',
    };
    const sink = saved ? 'app' : handle ? 'handle' : this.mode === 'memory' ? 'memory' : 'private';
    // The picker may have changed the name (a file of that name was there).
    if (saved && saved.name) name = saved.name;
    this.running = { name, sink, handle, chosen, format, began: performance.now(), slowed: false, spec };
    if (saved) this.app.exporting(true, name, 0);
    this.result = null;
    el('exportFill').style.width = '0%';
    el('exportProgress').textContent = WORDS.progress(0, null);
    el('exportNote').textContent = WORDS.background;
    this.show('run');
    this.worker.postMessage({ type: 'start', bytes, job, sink, handle, name, title: loopName(spec), pace: this.pace(), limit: MEMORY_BYTES });
  }

  progress(m) {
    const r = this.running;
    const share = m.expected ? Math.min(0.99, m.frames / m.expected) : 0;
    const pct = Math.floor(share * 100);
    const took = (performance.now() - r.began) / 1000;
    const left = share > 0.02 ? (took / share) * (1 - share) : null;
    if (r.sink === 'app') this.app.exporting(true, r.name, pct);
    el('exportFill').style.width = `${pct}%`;
    el('exportProgress').textContent = WORDS.progress(pct, left != null ? words(left) : null);
    r.progress = m;
  }

  cancel() {
    if (!this.running) return;
    this.worker.postMessage({ type: 'cancel' });
    el('exportProgress').textContent = 'Cancelling...';
  }

  done(m) {
    const r = this.running;
    this.running = null;
    if (this.app) this.app.exporting(false);
    this.result = { ...m, name: r.name, sink: r.sink, handle: r.handle, format: r.format, url: null };
    const t = words(m.seconds);
    const placed = r.sink === 'handle' || r.sink === 'app';
    el('exportSaved').textContent = placed ? WORDS.savedPicked(r.name, size(m.bytes), t) : WORDS.saved(r.name, size(m.bytes), t);
    el('exportSave').hidden = placed;
    this.show('done');
    el('exportShare').hidden = !this.canShare();
    // The private file and the memory's are offered as a download now.
    if (!placed) this.save();
  }

  fail(why, message) {
    this.running = null;
    if (this.app) this.app.exporting(false);
    el('exportWhy').textContent = why === 'space' ? WORDS.spaceLate : why === 'memory' ? WORDS.memoryFull : WORDS.failed;
    if (message) console.warn('export:', why, message);
    this.show('failed');
  }

  // The finished file, as a File or Blob.
  async file() {
    const r = this.result;
    if (!r) return null;
    if (r.file) return r.file;
    if (r.handle) return r.handle.getFile();
    return null;
  }

  async save() {
    const f = await this.file();
    if (f) ui.download(f, this.result.name);
  }

  async open() {
    if (this.result && this.result.sink === 'app') {
      this.app.send({ type: 'save-view' });
      return;
    }
    const f = await this.file();
    if (!f) return;
    if (this.result.url) URL.revokeObjectURL(this.result.url);
    this.result.url = URL.createObjectURL(f instanceof File ? f : new File([f], this.result.name, { type: FORMATS[this.result.format || 'lossless'].type }));
    window.open(this.result.url, '_blank', 'noopener');
  }

  canShare() {
    if (this.result && this.result.sink === 'app') return true;
    try {
      const kind = FORMATS[(this.result && this.result.format) || 'lossless'];
      const probe = new File([new Uint8Array(4)], `x.${kind.ext}`, { type: kind.type });
      return !!(navigator.canShare && navigator.canShare({ files: [probe] }));
    } catch {
      return false;
    }
  }

  async share() {
    if (this.result && this.result.sink === 'app') {
      this.app.send({ type: 'save-share' });
      return;
    }
    const f = await this.file();
    if (!f) return;
    const file = f instanceof File && f.name === this.result.name ? f : new File([f], this.result.name, { type: FORMATS[this.result.format || 'lossless'].type });
    try {
      await navigator.share({ files: [file], title: this.result.name });
    } catch { /* closed, or not allowed: the file is still saved */ }
  }
}
