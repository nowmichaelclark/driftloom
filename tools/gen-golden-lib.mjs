// The generation goldens' cases and hashes (queue item 77): what
// tools/gen-golden.mjs runs in Node, and in Chromium through a page. Plain
// browser JavaScript, so both run the very same code; the file format, the
// canonical text and the cases are documented in tools/gen-golden.mjs.

import { newSpec, planOf, vibeSelection, LAYERS } from '../js/generator.js';
import { Engine } from '../js/engine.js';
import { Rng } from '../js/rng.js';
import { VIBE_IDS } from '../js/vibes.js';
import { kickKindOf } from '../js/choices.js';

// The corpora (tools/gen-golden.mjs, "The cases").
export const CORPUS_SEED = 1;
export const CORPUS_N = 2000;
export const VIBES_SEED = 77;
export const VIBES_N = 1000;
export const VIBE_COUNTS = [1, 3, 5];
export const ALBUM_SEED = 20261009;
// The album loops of docs/DONE.md (and docs/archive/done.md, where the
// older lines are), as of v128: "dust #7" is the loop at index 7 of corpus
// seed 20261009 (the ears' corpus), and noan-nou is the baseline's seed.
export const ALBUM_PICKS = [0, 4, 5, 6, 7, 8, 10, 12, 18, 21, 23, 25, 30, 31, 32, 33, 37, 38, 42, 45, 47, 49, 51, 52, 56, 57, 58, 59, 63, 65, 66, 67, 68, 69, 71, 77, 79, 82];
export const ALBUM_SEEDS = [['noan-nou', 4272732017]];
// How many wandered passes each case hashes, and how far it wanders (the
// app's default, `Engine.driftAmount`).
export const PASSES = 8;

// ------------------------------------------------------- canonical text

const F = new Float64Array(1);
const U = new Uint32Array(F.buffer);
const LE = new Uint8Array(new Uint16Array([1]).buffer)[0] === 1;
const hex8 = (n) => (n >>> 0).toString(16).padStart(8, '0');

function number(x) {
  if (Number.isInteger(x) && Math.abs(x) <= 2 ** 53) return Object.is(x, -0) ? '-0' : String(x);
  if (x !== x) return 'nan';
  if (x === Infinity) return 'inf';
  if (x === -Infinity) return '-inf';
  F[0] = x;
  return `x${hex8(U[LE ? 1 : 0])}${hex8(U[LE ? 0 : 1])}`;
}

/** The canonical text of a value (tools/gen-golden.mjs, "The canonical text"). */
export function canonical(v) {
  if (v === null) return 'null';
  if (v === undefined) return 'undef';
  switch (typeof v) {
    case 'number': return number(v);
    case 'boolean': return v ? 'true' : 'false';
    case 'string':
      if (!/^[\x20-\x7e]*$/.test(v)) throw new Error(`not printable ASCII: ${JSON.stringify(v)}`);
      return `"${v.replace(/[\\"]/g, (c) => `\\${c}`)}"`;
    case 'object': {
      if (Array.isArray(v)) return `[${v.map(canonical).join(',')}]`;
      if (Object.getPrototypeOf(v) !== Object.prototype) throw new Error(`not a plain object: ${v.constructor && v.constructor.name}`);
      const keys = Object.keys(v).filter((k) => v[k] !== undefined).sort();
      return `{${keys.map((k) => `${k}:${canonical(v[k])}`).join(',')}}`;
    }
    default: throw new Error(`can't write a ${typeof v}`);
  }
}

/** FNV-1a, 64 bits, over a string of ASCII, as 16 hex digits. */
export function fnv1a64(text) {
  // The state in two 32-bit halves; the prime is 2^40 + 0x1b3, so a step
  // is the state times 0x1b3, plus its low half shifted into the high one
  // by 8 (40 - 32). Every product stays below 2^53, so it is exact.
  let hi = 0xcbf29ce4;
  let lo = 0x84222325;
  for (let i = 0; i < text.length; i++) {
    const c = text.charCodeAt(i);
    if (c > 0x7f) throw new Error('not ASCII');
    lo = (lo ^ c) >>> 0;
    const l = lo * 0x1b3;
    const carry = Math.floor(l / 4294967296);
    hi = (hi * 0x1b3 + carry + ((lo << 8) >>> 0)) >>> 0;
    lo = l >>> 0;
  }
  return hex8(hi) + hex8(lo);
}

export const hashOf = (v) => fnv1a64(canonical(v));

// ------------------------------------------------------------ the cases

/**
 * Every case, in the file's order: { set, index, seed, vibes } (vibes a
 * list of ids, or null). `goldenCodes` is the golden lists' [name, seed,
 * vibes] (read from core/golden/codes.txt by the caller).
 */
export function cases(goldenCodes) {
  const out = [];
  const corpus = new Rng(CORPUS_SEED);
  for (let i = 0; i < CORPUS_N; i++) out.push({ set: 'corpus', index: i, seed: corpus.seed32(), vibes: null });
  const vibed = new Rng(VIBES_SEED);
  for (let i = 0; i < VIBES_N; i++) {
    const seed = vibed.seed32();
    const count = VIBE_COUNTS[i % VIBE_COUNTS.length];
    const vibes = vibeSelection(vibed.shuffle(VIBE_IDS.slice()).slice(0, count));
    out.push({ set: 'vibes', index: i, seed, vibes });
  }
  for (const [name, seed, vibes] of goldenCodes) out.push({ set: 'golden', index: name, seed, vibes });
  const album = new Rng(ALBUM_SEED);
  const seeds = Array.from({ length: Math.max(...ALBUM_PICKS) + 1 }, () => album.seed32());
  for (const n of ALBUM_PICKS) out.push({ set: 'album', index: n, seed: seeds[n], vibes: null });
  for (const [name, seed] of ALBUM_SEEDS) out.push({ set: 'album', index: name, seed, vibes: null });
  return out;
}

// ------------------------------------------------------------ one case

// A synth that does nothing but write down what the engine tells it.
function recorder() {
  const calls = [];
  const target = {
    queuesAhead: true,
    renderAhead: null,
    aheadMeasured: true,
    worstAhead: 0,
    channels: Object.fromEntries(LAYERS.map((l) => [l, { gain: l }])),
  };
  const synth = new Proxy(target, {
    get(t, prop) {
      if (prop in t) return t[prop];
      return (...args) => { calls.push([prop, ...args]); };
    },
  });
  return { synth, calls };
}

// What the engine decided at load, besides the pattern: every per-loop draw
// it keeps (each null where the loop's choices leave it out).
const LOAD_FIELDS = ['glassScale', 'stab', 'organ', 'wash', 'arc', 'kick', 'subDuck', 'padSweep', 'bassLift', 'late', 'space', 'echoBeats', 'pumpAmount'];

/**
 * What one case decides, as plain values, in the file's order: { spec,
 * plan, form, <each layer>, meta, choices, load, passes: [...] }.
 */
export function partsOf(c) {
  const spec = newSpec(c.seed, c.vibes);
  const parts = { spec, plan: planOf(spec) };
  // The engine renders the pattern itself, at load; nothing it decides
  // there may come from chance.
  const { synth, calls } = recorder();
  const engine = new Engine({ currentTime: 0, state: 'running', sampleRate: 48000 }, synth);
  const random = Math.random;
  Math.random = () => { throw new Error('Math.random at load'); };
  try {
    engine.load(spec);
  } finally {
    Math.random = random;
  }
  const p = engine.base;
  parts.form = {
    form: p.form, gaps: p.gaps, silentBars: p.silentBars, totalSteps: p.totalSteps,
    stepsPerBar: p.stepsPerBar, cycles: p.cycles, harmony: p.harmony,
  };
  for (const layer of LAYERS) parts[layer] = p.tracks[layer];
  parts.meta = p.meta;
  parts.choices = p.choices;
  parts.load = { ...Object.fromEntries(LOAD_FIELDS.map((k) => [k, engine[k] ?? null])), calls };
  parts.passes = [];
  for (let pass = 1; pass <= PASSES; pass++) {
    const w = engine._wander(pass);
    const { spec: _, ...rest } = w;
    parts.passes.push(engine.kick ? { ...rest, kick: kickKindOf(spec.seed, engine.kick.kind, pass) } : rest);
  }
  // In a browser the engine's clock holds a worker.
  engine.clock.dispose();
  return parts;
}

/**
 * One case's hashes, in the file's order: { spec, plan, form, <each layer>, meta,
 * choices, load, passes: [...] }; with `texts`, the canonical texts too.
 */
export function hashCase(c, texts = false) {
  const { passes, ...parts } = partsOf(c);
  const out = {};
  const text = {};
  for (const [k, v] of Object.entries(parts)) {
    const t = canonical(v);
    out[k] = fnv1a64(t);
    if (texts) text[k] = t;
  }
  out.passes = passes.map((v) => {
    const t = canonical(v);
    if (texts) (text.passes ||= []).push(t);
    return fnv1a64(t);
  });
  return texts ? { hashes: out, texts: text } : out;
}

/** The file's line for a case. */
export function lineFor(c) {
  const h = hashCase(c);
  const named = Object.entries(h).filter(([k]) => k !== 'passes').map(([k, v]) => `${k}=${v}`);
  return [`${c.set}:${c.index}`, c.seed >>> 0, c.vibes && c.vibes.length ? c.vibes.join(',') : '-', ...named, `passes=${h.passes.join(',')}`].join(' ');
}

// ------------------------------------------------------- reading it back

/**
 * A canonical text read back into a value (the inverse of `canonical`, for
 * tools that read what the Rust generator made: `tools/stats.mjs --core`).
 * An object's keys come back in the text's (sorted) order.
 */
export function parseCanonical(text) {
  let i = 0;
  const fail = (why) => { throw new Error(`canonical text, at ${i}: ${why}`); };
  const word = () => {
    const start = i;
    while (i < text.length && /[A-Za-z0-9_.+-]/.test(text[i])) i++;
    return text.slice(start, i);
  };
  const value = () => {
    const c = text[i];
    if (c === '{') {
      i++;
      const out = {};
      if (text[i] === '}') { i++; return out; }
      for (;;) {
        const k = word();
        if (text[i++] !== ':') fail('expected :');
        out[k] = value();
        if (text[i] === ',') { i++; continue; }
        if (text[i++] !== '}') fail('expected }');
        return out;
      }
    }
    if (c === '[') {
      i++;
      const out = [];
      if (text[i] === ']') { i++; return out; }
      for (;;) {
        out.push(value());
        if (text[i] === ',') { i++; continue; }
        if (text[i++] !== ']') fail('expected ]');
        return out;
      }
    }
    if (c === '"') {
      let s = '';
      i++;
      while (text[i] !== '"') {
        if (i >= text.length) fail('an unended string');
        if (text[i] === '\\') i++;
        s += text[i++];
      }
      i++;
      return s;
    }
    const w = word();
    switch (w) {
      case 'null': return null;
      case 'undef': return undefined;
      case 'true': return true;
      case 'false': return false;
      case 'nan': return NaN;
      case 'inf': return Infinity;
      case '-inf': return -Infinity;
      case '-0': return -0;
      default:
        if (/^x[0-9a-f]{16}$/.test(w)) {
          U[LE ? 1 : 0] = parseInt(w.slice(1, 9), 16);
          U[LE ? 0 : 1] = parseInt(w.slice(9), 16);
          return F[0];
        }
        if (/^-?\d+$/.test(w)) return Number(w);
        return fail(`not a value: ${w}`);
    }
  };
  const v = value();
  if (i !== text.length) fail('text after the value');
  return v;
}
