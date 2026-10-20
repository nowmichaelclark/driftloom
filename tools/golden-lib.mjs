// The golden renders' shared code (queue item 42): the note-list text
// format, the hash, and a host that renders a note list
// through js/dlcore.wasm in Node, with the same calls the worklet makes.
//
// The format is documented where the native host reads it
// (core/examples/dlrender/notelist.rs); this file is its other reader and
// writer. Both pages and Node import it, so it uses nothing but the
// language.

// A number the way a note list writes it: the shortest text a double reads
// back from exactly; "nan" for the values a note leaves out; "-0" for
// negative zero, which String() would lose.
export function num(x) {
  if (Number.isNaN(x)) return 'nan';
  if (Object.is(x, -0)) return '-0';
  if (!Number.isFinite(x)) throw new Error(`a note list cannot hold ${x}`);
  return String(x);
}

// A note list, from the header and the calls the host made, in order. Each
// event is { type: 'param', id, op, value, time, tau }, { type: 'damp',
// channel, time }, { type: 'key', seed, pass, step, ord }, { type: 'set',
// channel, block, setting, value, time }, { type: 'event', channel, kind,
// time, value }, { type: 'retune', channel, time, notes } (six, NaN past the
// last) or { type: 'note', voice, channel, time, midi, dur, vel, parts,
// extra }.
export function writeList({ name, comment = [], rate, quality, frames, events }) {
  const out = ['dlnotes 2', ...comment.map((c) => `# ${c}`)];
  out.push(`name ${name}`, `rate ${rate}`, `quality ${quality}`, `frames ${frames}`);
  for (const e of events) {
    if (e.type === 'param') {
      out.push(`param ${e.id} ${e.op} ${num(e.value)} ${num(e.time)} ${num(e.tau ?? 0)}`);
    } else if (e.type === 'damp') {
      out.push(`damp ${e.channel} ${num(e.time)}`);
    } else if (e.type === 'key') {
      out.push(`key ${e.seed} ${e.pass} ${e.step} ${e.ord}`);
    } else if (e.type === 'set') {
      out.push(`set ${e.channel} ${e.block} ${e.setting} ${num(e.value)} ${num(e.time)}`);
    } else if (e.type === 'event') {
      out.push(`event ${e.channel} ${e.kind} ${num(e.time)} ${num(e.value)}`);
    } else if (e.type === 'retune') {
      out.push(`retune ${e.channel} ${num(e.time)} ${e.notes.map(num).join(' ')}`);
    } else {
      const extra = Array.from({ length: 12 }, (_, i) => num(e.extra?.[i] ?? NaN));
      out.push(`note ${e.voice} ${e.channel} ${num(e.time)} ${num(e.midi)} ${num(e.dur)} ${num(e.vel)} ${e.parts} ${extra.join(' ')}`);
    }
  }
  return `${out.join('\n')}\n`;
}

const NUMBER = /^(nan|[-+]?(\d+\.?\d*|\.\d+)([eE][-+]?\d+)?)$/;
function value(word, line) {
  if (!NUMBER.test(word)) throw new Error(`line ${line}: not a number: ${word}`);
  return word === 'nan' ? NaN : Number(word);
}

// A note list, read back: the same shape writeList takes.
export function readList(text) {
  const list = { name: '', rate: 0, quality: 'full', frames: 0, events: [] };
  let header = false;
  text.split('\n').forEach((raw, n) => {
    const line = n + 1;
    const words = raw.trim().split(/\s+/);
    if (!words[0] || words[0].startsWith('#')) return;
    const [kind, ...a] = words;
    if (!header) {
      if (kind !== 'dlnotes' || a.join(' ') !== '2') throw new Error(`line ${line}: a note list starts with: dlnotes 2`);
      header = true;
      return;
    }
    switch (kind) {
      case 'name': list.name = a[0]; break;
      case 'rate': list.rate = value(a[0], line); break;
      case 'quality': list.quality = a[0]; break;
      case 'frames': list.frames = value(a[0], line); break;
      case 'param':
        list.events.push({ type: 'param', id: value(a[0], line), op: value(a[1], line), value: value(a[2], line), time: value(a[3], line), tau: value(a[4], line) });
        break;
      case 'damp': list.events.push({ type: 'damp', channel: value(a[0], line), time: value(a[1], line) }); break;
      case 'key':
        list.events.push({ type: 'key', seed: value(a[0], line), pass: value(a[1], line), step: value(a[2], line), ord: value(a[3], line) });
        break;
      case 'set':
        list.events.push({ type: 'set', channel: value(a[0], line), block: value(a[1], line), setting: value(a[2], line), value: value(a[3], line), time: value(a[4], line) });
        break;
      case 'event':
        list.events.push({ type: 'event', channel: value(a[0], line), kind: value(a[1], line), time: value(a[2], line), value: value(a[3], line) });
        break;
      case 'retune':
        list.events.push({ type: 'retune', channel: value(a[0], line), time: value(a[1], line), notes: a.slice(2, 8).map((w) => value(w, line)) });
        break;
      case 'note':
        list.events.push({
          type: 'note', voice: value(a[0], line), channel: value(a[1], line), time: value(a[2], line), midi: value(a[3], line),
          dur: value(a[4], line), vel: value(a[5], line), parts: value(a[6], line), extra: a.slice(7, 19).map((w) => value(w, line)),
        });
        break;
      default: throw new Error(`line ${line}: no such line: ${kind}`);
    }
  });
  if (!header) throw new Error('empty: a note list starts with: dlnotes 2');
  return list;
}

// FNV-1a, 64 bits, over bytes: the hash a render is kept by (see `Fnv` in
// the native host). The state is two 32-bit halves, so no BigInt: the
// prime is 2^40 + 0x1b3, and a multiply by it is a shift and a small
// multiply, each exact in a double.
export class Fnv {
  constructor() {
    this.hi = 0xcbf29ce4;
    this.lo = 0x84222325;
  }

  bytes(bytes) {
    let { hi, lo } = this;
    for (let i = 0; i < bytes.length; i++) {
      lo = (lo ^ bytes[i]) >>> 0;
      const p = lo * 0x1b3;
      const plo = p % 4294967296;
      hi = (hi * 0x1b3 + (p - plo) / 4294967296 + ((lo << 8) >>> 0)) % 4294967296;
      lo = plo;
    }
    this.hi = hi;
    this.lo = lo;
  }

  // Float32 samples as their four little-endian bytes each (this assumes
  // the machine running JavaScript is little-endian, as every one that
  // runs these tools is).
  samples(samples) {
    this.bytes(new Uint8Array(samples.buffer, samples.byteOffset, samples.byteLength));
  }

  hex() {
    return this.hi.toString(16).padStart(8, '0') + this.lo.toString(16).padStart(8, '0');
  }
}

// The signals a render hashes, in the hash line's order (the native host's
// SIGNALS).
export const SIGNALS = ['out', 'melody', 'chords', 'bass', 'texture', 'drums', 'bus'];
const QUANTUM = 128;
const CHANNELS = 5;

// The same list at another sample rate, as the native host's `at_rate`: a
// list is in seconds and Hz, so only the length in frames changes.
export function atRate(list, rate) {
  return { ...list, name: `${list.name}@${rate}`, rate, frames: Math.round(list.frames * rate / list.rate) };
}

// Hand a list's calls to a fresh core, as the worklet's host does.
function load(x, list) {
  x.dl_init(list.rate);
  if (!x.dl_quality(list.quality === 'lite' ? 0 : 1)) throw new Error(`the core does not mix at ${list.rate} Hz`);
  list.events.forEach((e, n) => {
    if (e.type === 'param') {
      if (!x.dl_param(e.id, e.op, e.value, e.time, e.tau)) throw new Error(`event ${n + 1}: the core refused parameter ${e.id}`);
    } else if (e.type === 'damp') {
      x.dl_damp(e.channel, e.time);
    } else if (e.type === 'key') {
      x.dl_key(e.seed, e.pass, e.step, e.ord);
    } else if (e.type === 'set') {
      if (!x.dl_set(e.channel, e.block, e.setting, e.value, e.time)) throw new Error(`event ${n + 1}: the core refused setting ${e.setting} of block ${e.block}`);
    } else if (e.type === 'event') {
      if (!x.dl_event(e.channel, e.kind, e.time, e.value)) throw new Error(`event ${n + 1}: the core refused an event of kind ${e.kind}`);
    } else if (e.type === 'retune') {
      const m = e.notes;
      if (!x.dl_retune(e.channel, e.time, m[0], m[1], m[2], m[3], m[4], m[5])) throw new Error(`event ${n + 1}: the core refused a retune`);
    } else {
      const v = e.extra;
      const taken = x.dl_note(e.voice, e.channel, e.time, e.midi, e.dur, e.vel, e.parts, v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8], v[9], v[10], v[11]);
      if (taken !== 0) throw new Error(`event ${n + 1}: the core took the note as ${taken}, not on time`);
    }
  });
}

// The speaker output of a list rendered a host's way: `size` frames a call
// (a number, or 'random' for a different number each call, from 1 to 2000, in
// a fixed order), through `dl_render`. Returns the samples' hash, which must
// be the `out` hash of any other way of rendering the same list.
export function renderWasmBlocks(bytes, list, size) {
  const { exports: x } = new WebAssembly.Instance(new WebAssembly.Module(bytes), {});
  load(x, list);
  const hash = new Fnv();
  let state = 0x2545f491;
  for (let done = 0; done < list.frames;) {
    let want = size;
    if (size === 'random') {
      state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
      want = 1 + ((state >>> 8) % 2000);
    }
    const take = Math.min(want, 2000, list.frames - done);
    const at = x.dl_render(take);
    if (!at) throw new Error('the core would not render');
    hash.samples(new Float32Array(x.memory.buffer, at, take));
    done += take;
  }
  if (x.dl_dropped() || x.dl_late() || x.dl_missed()) throw new Error('the core dropped or missed notes');
  return hash.hex();
}

// Render a note list through the .wasm, the way the worklet drives it: the
// exports, one block at a time. `keep` is the name of a signal whose
// samples to return as well.
export function renderWasm(bytes, list, keep = null) {
  const { exports: x } = new WebAssembly.Instance(new WebAssembly.Module(bytes), {});
  load(x, list);
  const hashes = SIGNALS.map(() => new Fnv());
  const index = keep == null ? -1 : SIGNALS.indexOf(keep);
  const kept = index >= 0 ? new Float32Array(list.frames) : null;
  for (let frame = 0; frame < list.frames; frame += QUANTUM) {
    const taps = x.dl_process(frame);
    const used = Math.min(QUANTUM, list.frames - frame);
    const buffer = x.memory.buffer;
    const blocks = [x.dl_out(), ...Array.from({ length: CHANNELS }, (_, c) => taps + c * QUANTUM * 4), x.dl_bus()];
    blocks.forEach((p, i) => {
      const block = new Float32Array(buffer, p, used);
      hashes[i].samples(block);
      if (i === index) kept.set(block, frame);
    });
  }
  if (x.dl_dropped() || x.dl_late() || x.dl_missed()) {
    throw new Error(`the core dropped ${x.dl_dropped()}, took ${x.dl_late()} late and missed ${x.dl_missed()} notes`);
  }
  return { hashes: hashes.map((h) => h.hex()), kept };
}

// The hash line, as the native host prints it.
export function hashLine(list, hashes) {
  return `${list.name} frames=${list.frames}${SIGNALS.map((s, i) => ` ${s}=${hashes[i]}`).join('')}`;
}

// A mono 32-bit float WAV, as the native host writes it.
export function wavBytes(samples, rate) {
  const data = samples.length * 4;
  const out = new DataView(new ArrayBuffer(44 + data));
  const text = (at, s) => [...s].forEach((c, i) => out.setUint8(at + i, c.charCodeAt(0)));
  text(0, 'RIFF'); out.setUint32(4, 36 + data, true); text(8, 'WAVEfmt ');
  out.setUint32(16, 16, true); out.setUint16(20, 3, true); out.setUint16(22, 1, true);
  out.setUint32(24, rate, true); out.setUint32(28, rate * 4, true); out.setUint16(32, 4, true); out.setUint16(34, 32, true);
  text(36, 'data'); out.setUint32(40, data, true);
  new Float32Array(out.buffer, 44, samples.length).set(samples);
  return new Uint8Array(out.buffer);
}
