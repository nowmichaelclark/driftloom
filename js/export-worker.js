// The audio export's thread (queue item 104b; Mikey, 2026-10-13: an export
// must never crash or break anything, whatever the device).
//
// Its own instance of the core (js/dlplayer.wasm, never the one that plays,
// never on the audio thread) renders the loop or the piece a chunk at a time
// (js/export-render.js) through the core's FLAC writer, and each chunk's
// bytes go straight to the file, so memory stays the same for a minute or
// three hours. The file is one of:
//
//   handle   a file the listener chose (the File System Access API's save
//            picker, where the browser has it): written through a writable
//            stream, which the browser makes the file only when it closes.
//   private  the site's private file system (OPFS), through a synchronous
//            access handle (only a Worker has one); the page offers it as a
//            download at the end.
//   memory   neither: the bytes kept here, to a limit the page states before
//            it starts, and handed to the page as one Blob.
//   app      the Android app (session 28): a file made in Android's own
//            picker, which only the page can reach (the app's bridge is in
//            the page, not in a Worker). Each megabyte goes to the page as
//            base64, the page gives it to the app, which writes it to the
//            file as it comes, and the answer (`ack`) comes back before the
//            next is made: memory stays as it is for any length.
//
// Cancel, a failure or a full disk removes what was written: no partial file
// is left behind. A page closed mid-export takes this Worker with it; its
// private file is removed the next time an export starts.
//
// Speed ("lighter on processor"): `gentle` renders in short slices and rests
// three times as long as each took, a quarter of one core, so the music
// playing never stutters and the device stays cool; `fast` renders on,
// yielding between slices only to hear Cancel. The page can move a running
// export to gentle (the audio thread getting busy).
//
// Two formats (the job's `format`): 'flac', the core's own writer, and
// 'opus' (queue item 104c), the browser's AudioEncoder through js/ogg.js
// (js/export-opus.js), which also gets the Worker's say on whether this
// browser can encode Opus at all.
//
// Messages in: { type: 'hello' }; { type: 'start', bytes, job, sink, name,
// title, pace, limit }; { type: 'pace', pace }; { type: 'cancel' }; { type:
// 'ack', id, ok, why } (the app sink's answer).
// Out: { type: 'hello', private, opus } (whether the private file system
// works here, and whether Opus can be encoded); { type: 'progress', frames,
// expected, bytes }; { type: 'done', file (private and memory), bytes,
// frames, seconds, took, counts, format }; { type:
// 'cancelled' }; { type: 'failed', why ('space', 'write', 'render',
// 'memory'), message }; { type: 'relay', id, op ('write', 'header',
// 'close', 'abort'), data } (the app sink's, for the page to pass on).

import { Render } from './export-render.js';
import { OpusMaker, opusWorks } from './export-opus.js';
import { toBase64 } from './app.js';

// One slice of rendering before a yield, in milliseconds.
const SLICE = 25;
// Gentle rests this many times as long as each slice took.
const REST = 3;
// The bytes gathered before each write (one buffer, reused).
const GATHER = 1 << 20;
// How often progress is posted, in milliseconds.
const PROGRESS_EVERY = 250;
// Where private exports live.
const DIR = 'exports';

let job = null;

// A yield to the Worker's queue with no delay (a message on a channel of its
// own: not throttled like a timer), so Cancel is heard between slices.
const channel = new MessageChannel();
const waiting = [];
channel.port1.onmessage = () => { const r = waiting.shift(); if (r) r(); };
const yieldNow = () => new Promise((r) => { waiting.push(r); channel.port2.postMessage(0); });
const rest = (ms) => new Promise((r) => setTimeout(r, ms));

const isSpace = (err) => err && (err.name === 'QuotaExceededError' || /quota|space|disk full/i.test(String(err.message || err)));

async function privateDir() {
  const root = await navigator.storage.getDirectory();
  return root.getDirectoryHandle(DIR, { create: true });
}

// Whether the private file system works here, with a synchronous handle
// (not in Firefox before 111, Safari before 15.2, or a private window that
// refuses it).
async function privateWorks() {
  try {
    if (!navigator.storage || !navigator.storage.getDirectory) return false;
    const dir = await privateDir();
    const probe = await dir.getFileHandle('.probe', { create: true });
    if (typeof probe.createSyncAccessHandle !== 'function') return false;
    const h = await probe.createSyncAccessHandle();
    h.close();
    await dir.removeEntry('.probe');
    return true;
  } catch {
    return false;
  }
}

// The sinks: open, write(bytes), header(bytes at 0), close() -> what the
// page is given, remove().
class PrivateSink {
  async open(name) {
    const dir = await privateDir();
    // Nothing older stays: a page closed mid-export, or the last export,
    // already offered.
    for await (const key of dir.keys()) {
      try { await dir.removeEntry(key); } catch { /* still open elsewhere */ }
    }
    this.dir = dir;
    this.name = name;
    this.file = await dir.getFileHandle(name, { create: true });
    this.h = await this.file.createSyncAccessHandle();
    this.h.truncate(0);
    this.at = 0;
  }

  write(bytes) {
    const n = this.h.write(bytes, { at: this.at });
    if (n !== bytes.length) {
      const e = new Error('the disk took only part of a write');
      e.name = 'QuotaExceededError';
      throw e;
    }
    this.at += n;
  }

  header(bytes) {
    this.h.write(bytes, { at: 0 });
  }

  async close() {
    this.h.flush();
    this.h.close();
    this.h = null;
    return this.file.getFile();
  }

  async remove() {
    try { if (this.h) this.h.close(); } catch { /* closed already */ }
    this.h = null;
    try { await this.dir.removeEntry(this.name); } catch { /* never made */ }
  }
}

class HandleSink {
  constructor(handle) {
    this.handle = handle;
  }

  async open() {
    this.w = await this.handle.createWritable({ keepExistingData: false });
  }

  async write(bytes) {
    await this.w.write(bytes);
  }

  async header(bytes) {
    await this.w.write({ type: 'write', position: 0, data: bytes });
  }

  async close() {
    await this.w.close();
    this.w = null;
    return null;
  }

  async remove() {
    try { if (this.w) await this.w.abort(); } catch { /* closed already */ }
    this.w = null;
    // The picker made the file, empty, when it was chosen.
    try { if (typeof this.handle.remove === 'function') await this.handle.remove(); } catch { /* not allowed */ }
  }
}

// The app's file, through the page (see the top). Each call waits for the
// app's answer, so what the disk can take is what is made.
const acks = new Map();
let relayed = 0;

class RelaySink {
  open() {}

  write(bytes) {
    return this.relay('write', bytes);
  }

  header(bytes) {
    return this.relay('header', bytes);
  }

  async close() {
    await this.relay('close');
    return null;
  }

  async remove() {
    try { await this.relay('abort'); } catch { /* nothing was open */ }
  }

  relay(op, bytes) {
    const id = ++relayed;
    return new Promise((resolve, reject) => {
      acks.set(id, { resolve, reject });
      self.postMessage({ type: 'relay', id, op, data: bytes ? toBase64(bytes) : '' });
    });
  }
}

class MemorySink {
  constructor(limit, type) {
    this.limit = limit;
    this.type = type;
  }

  open() {
    this.parts = [];
    this.size = 0;
  }

  write(bytes) {
    if (this.size + bytes.length > this.limit) {
      const e = new Error('more than this browser can keep in memory');
      e.name = 'MemoryLimit';
      throw e;
    }
    this.parts.push(bytes.slice());
    this.size += bytes.length;
  }

  header(bytes) {
    // The file's first part is its header alone (the first write), so the
    // finished one takes its place.
    this.parts[0] = bytes.slice();
  }

  close() {
    const blob = new Blob(this.parts, { type: this.type });
    this.parts = null;
    return blob;
  }

  remove() {
    this.parts = null;
  }
}

// The bytes, gathered into one buffer and written a megabyte at a time.
class Gather {
  constructor(sink) {
    this.sink = sink;
    this.buf = new Uint8Array(GATHER);
    this.n = 0;
    this.total = 0;
  }

  async add(bytes) {
    this.total += bytes.length;
    if (this.n + bytes.length > this.buf.length) await this.flush();
    if (bytes.length > this.buf.length) {
      await this.sink.write(bytes.slice());
      return;
    }
    this.buf.set(bytes, this.n);
    this.n += bytes.length;
  }

  async flush() {
    if (!this.n) return;
    // A copy for a stream that may keep it past the call; the private file
    // and memory copy or keep their own.
    await this.sink.write(this.sink instanceof HandleSink ? this.buf.slice(0, this.n) : this.buf.subarray(0, this.n));
    this.n = 0;
  }
}

async function run(m) {
  const started = performance.now();
  const opus = m.job.format === 'opus';
  const sink = m.sink === 'handle' ? new HandleSink(m.handle) : m.sink === 'app' ? new RelaySink() : m.sink === 'private' ? new PrivateSink() : new MemorySink(m.limit, opus ? 'audio/ogg' : 'audio/flac');
  job = { cancelled: false, pace: m.pace, sink };
  let render;
  let maker = null;
  try {
    const module = await WebAssembly.compile(m.bytes);
    const x = (await WebAssembly.instantiate(module, {})).exports;
    render = new Render(x, m.job);
    if (opus) {
      maker = new OpusMaker(render, { title: m.title || '' });
      await maker.open();
    }
  } catch (err) {
    if (maker) maker.close();
    self.postMessage({ type: 'failed', why: 'render', message: String((err && err.message) || err) });
    job = null;
    return;
  }
  try {
    await sink.open(m.name);
    const out = new Gather(sink);
    // FLAC: the header first, its own write: the memory sink swaps it at the
    // end. Opus has no header to come back to: the file is written once,
    // front to back.
    if (render.first) {
      if (sink instanceof MemorySink) sink.write(render.first);
      if (!(sink instanceof MemorySink)) await out.add(render.first);
      else out.total = render.first.length;
    }
    let posted = 0;
    let more = true;
    while (more) {
      const t0 = performance.now();
      while (performance.now() - t0 < SLICE) {
        const bytes = maker ? maker.step() : render.step();
        if (!bytes) {
          more = false;
          break;
        }
        if (bytes.length) await out.add(bytes);
      }
      if (job.cancelled) throw new Cancelled();
      const now = performance.now();
      if (now - posted >= PROGRESS_EVERY) {
        posted = now;
        self.postMessage({ type: 'progress', frames: render.frames, expected: render.expectedFrames, bytes: out.total });
      }
      // The encoder's outputs come between slices; it is not let to queue
      // more than a few chunks.
      if (maker) await maker.settle();
      if (more) {
        if (job.pace === 'gentle') await rest(REST * (performance.now() - t0));
        else await yieldNow();
        if (job.cancelled) throw new Cancelled();
      }
    }
    await out.add(maker ? await maker.last() : render.last());
    await out.flush();
    if (!maker) await sink.header(render.header().slice());
    const file = await sink.close();
    self.postMessage({
      type: 'done', file, format: render.format, bytes: out.total, frames: render.frames, seconds: render.frames / render.rate,
      took: (performance.now() - started) / 1000, counts: render.counts(), lateEnd: render.lateEnd,
      // Where the core was told to stop, and where the tail's fade began:
      // what a straight render needs to null the file (tools/export.mjs).
      stop: render.stopFrame == null ? null : { frame: render.stopFrame, time: render.endTime }, fade: render.fadeFrom,
    });
  } catch (err) {
    if (maker) maker.close();
    await sink.remove();
    if (err instanceof Cancelled) self.postMessage({ type: 'cancelled' });
    else if (isSpace(err)) self.postMessage({ type: 'failed', why: 'space', message: String(err.message || err) });
    else if (err && err.name === 'MemoryLimit') self.postMessage({ type: 'failed', why: 'memory', message: err.message });
    else self.postMessage({ type: 'failed', why: 'write', message: String((err && err.message) || err) });
  } finally {
    if (maker) maker.close();
    job = null;
  }
}

class Cancelled extends Error {}

self.onmessage = async (e) => {
  const m = e.data || {};
  if (m.type === 'hello') self.postMessage({ type: 'hello', private: await privateWorks(), opus: await opusWorks() });
  else if (m.type === 'start' && !job) run(m);
  else if (m.type === 'pace' && job) job.pace = m.pace;
  else if (m.type === 'cancel' && job) job.cancelled = true;
  else if (m.type === 'ack') {
    const a = acks.get(m.id);
    acks.delete(m.id);
    if (!a) return;
    if (m.ok) a.resolve();
    else {
      // The disk filling is the one failure the page says in its own words.
      const err = new Error(m.why || 'the app could not write the file');
      if (m.why === 'space') err.name = 'QuotaExceededError';
      a.reject(err);
    }
  }
};
