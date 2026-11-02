// The generation goldens' cases and hashes (queue item 77): what
// tools/gen-golden.mjs runs in Node, and in Chromium through a page. Plain
// browser JavaScript, so both run the very same code; the file format, the
// canonical text and the cases are documented in tools/gen-golden.mjs.

import { newSpec, planOf, vibeSelection, choirOf, themeRests, LAYERS } from '../js/generator.js';
import { Engine } from '../js/engine.js';
import { Rng } from '../js/rng.js';
import { VIBE_IDS } from '../js/vibes.js';
import { kickKindOf } from '../js/choices.js';
import { PINS } from '../js/pins.js';

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
// The requests (queue item 85): seeds from their own stream, each with a
// request of the kind its index picks (`requestCase`).
export const REQUEST_SEED = 85;
export const REQUEST_N = 360;
// The pins (queue item 85b): seeds from their own stream, each with pins of
// the kind its index picks (`pinCase`).
export const PIN_SEED = 8502;
export const PIN_N = 240;
// The themes (queue item 85d): seeds from their own stream, each with a
// theme of the kind its index picks (`themeCase`).
export const THEME_SEED = 8504;
export const THEME_N = 240;
// A theme's rest, choir or not (queue item 85f): themes whose own loop rests,
// each bent onto a seed whose loop draws a choir, from their own stream, the
// brain's case first (`restCases`).
export const REST_SEED = 8506;
export const REST_N = 24;
export const REST_FIRST = [2428270433, 'theme=3570153259,booming=0.31,cheeky=0.26,experimental'];
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
  const asked = new Rng(REQUEST_SEED);
  for (let i = 0; i < REQUEST_N; i++) {
    const seed = asked.seed32();
    out.push({ set: 'request', index: i, seed, vibes: null, request: requestCase(asked, i) });
  }
  const pinned = new Rng(PIN_SEED);
  for (let i = 0; i < PIN_N; i++) {
    const seed = pinned.seed32();
    out.push({ set: 'pins', index: i, seed, vibes: null, request: pinCase(pinned, i) });
  }
  const themed = new Rng(THEME_SEED);
  for (let i = 0; i < THEME_N; i++) {
    const seed = themed.seed32();
    out.push({ set: 'theme', index: i, seed, vibes: null, request: themeCase(themed, i, seed) });
  }
  restCases().forEach(([seed, request], i) => out.push({ set: 'rest', index: i, seed, vibes: null, request }));
  return out;
}

// The rest cases (queue item 85f): the brain's, then resting themes on choir
// seeds, the theme alone, with vibes, with weighted vibes, with a tempo.
function restCases() {
  const r = new Rng(REST_SEED);
  const next = (wanted) => {
    for (;;) {
      const s = r.seed32();
      if (wanted(s)) return s;
    }
  };
  const vibes = (n) => vibeSelection(r.shuffle(VIBE_IDS.slice()).slice(0, n));
  const out = [REST_FIRST];
  for (let i = 1; i < REST_N; i++) {
    const theme = next((s) => themeRests({ theme: s }));
    const seed = next((s) => !!choirOf(newSpec(s)));
    const parts = [`theme=${theme}`];
    if (i % 4 === 1) parts.unshift(...vibes(1 + (i % 3)));
    if (i % 4 === 2) parts.unshift(...vibes(2).map((id) => `${id}=${Math.floor(r.f() * 100) / 100}`));
    if (i % 4 === 3) parts.unshift(`bpm=${40 + Math.floor(r.f() * 120)}`);
    out.push([seed, parts.join(',')]);
  }
  return out;
}

// A theme, of the kind `i` picks, drawn from `r`: the loop's own seed alone,
// its own with vibes, another seed with weighted vibes, its own with a
// tempo pinned, another with vibes and a range, and its own with vibes and
// pins.
function themeCase(r, i, seed) {
  const other = () => r.seed32();
  const vibes = (n) => vibeSelection(r.shuffle(VIBE_IDS.slice()).slice(0, n));
  const weight = () => String(Math.floor(r.f() * 100) / 100);
  switch (i % 6) {
    case 0: return `theme=${seed}`;
    case 1: return [...vibes(1 + (i % 3)), `theme=${seed}`].join(',');
    case 2: return [...vibes(1 + (i % 4)).map((id) => `${id}=${weight()}`), `theme=${other()}`].join(',');
    case 3: return `bpm=${32 + Math.floor(r.f() * 159)},theme=${seed}`;
    case 4: {
      const lo = 40 + Math.floor(r.f() * 120);
      return [...vibes(2), `bpm=${lo}-${lo + 20}`, `theme=${other()}`].join(',');
    }
    default: return [...vibes(1 + (i % 2)), `theme=${seed}`, 'organ=on', 'echo.beats=1'].join(',');
  }
}

// A pin's value drawn from `r`: one of its words or values, or a number in
// its range (a whole one where it takes only those), to three places so the
// text reads back to the same number in both languages.
function pinDraw(r, p) {
  if (p.words) return p.words[Math.floor(r.f() * p.words.length)];
  if (p.values) return String(p.values[Math.floor(r.f() * p.values.length)]);
  const [lo, hi] = p.range;
  if (p.whole) return String(lo + Math.floor(r.f() * (hi - lo + 1)));
  return String(Math.min(hi, Math.max(lo, Math.round((lo + r.f() * (hi - lo)) * 1000) / 1000)));
}

// Pins, of the kind `i` picks, drawn from `r`: one choice at any of its
// options (today's too), two to four choices, the arc on with its settings,
// the tuned kick, subDuck and lateFloor with theirs, padSweep, the echo and
// some of the space's, and a mix with vibes and a tempo.
function pinCase(r, i) {
  const byName = (n) => PINS.find((p) => p.name === n);
  const pin = (n, v = null) => `${n}=${v ?? pinDraw(r, byName(n))}`;
  const choices = PINS.filter((p) => !p.name.includes('.'));
  const some = (list, n) => r.shuffle(list.slice()).slice(0, n);
  const space = PINS.filter((p) => p.name.startsWith('space.'));
  switch (i % 6) {
    case 0: return pin(choices[Math.floor(r.f() * choices.length)].name);
    case 1: return some(choices, 2 + (i % 3)).map((p) => pin(p.name)).join(',');
    case 2: return [pin('arc', 'on'), ...some(['arc.shape', 'arc.period', 'arc.from', 'arc.floor'], 1 + (i % 4)).map((n) => pin(n))].join(',');
    case 3: return [pin('tunedKick'), pin('subDuck', 'on'), pin('lateFloor', 'on'),
      ...some(['kick.note', 'kick.short', 'kick.long', 'subDuck.depth', 'subDuck.recover', 'lateFloor.order', 'lateFloor.seconds', 'lateFloor.gap'], 2 + (i % 5)).map((n) => pin(n))].join(',');
    case 4: return [pin('padSweep', 'on'), ...some(['padSweep.lo', 'padSweep.oct', 'padSweep.period', 'padSweep.q', 'echo.beats'], 1 + (i % 5)).map((n) => pin(n)),
      ...some(space, 1 + (i % 4)).map((p) => pin(p.name))].join(',');
    default: {
      const vibes = vibeSelection(r.shuffle(VIBE_IDS.slice()).slice(0, 1 + (i % 3)));
      return [...vibes, `bpm=${60 + Math.floor(r.f() * 100)}`, ...some(choices, 1 + (i % 2)).map((p) => pin(p.name)), pin('echo.beats')].join(',');
    }
  }
}

// A request's text, of the kind `i` picks, drawn from `r`: a pinned tempo
// (anywhere the core plays), a range, weighted vibes (one to three), six or
// seven vibes some weighted (the five heaviest steer), unweighted vibes with
// a pin, and weights past 1 (scaled) with a range.
function requestCase(r, i) {
  const bpm = () => 32 + Math.floor(r.f() * 159);
  const range = () => {
    const lo = 32 + Math.floor(r.f() * 139);
    return `bpm=${lo}-${Math.min(190, lo + Math.floor(r.f() * 41))}`;
  };
  const weight = (most) => String(Math.floor(r.f() * most * 100) / 100);
  const vibes = (n) => vibeSelection(r.shuffle(VIBE_IDS.slice()).slice(0, n));
  switch (i % 6) {
    case 0: return `bpm=${bpm()}`;
    case 1: return range();
    case 2: return vibes(1 + (i % 3)).map((id) => `${id}=${weight(1)}`).join(',');
    case 3: return vibes(6 + (i % 2)).map((id, k) => (k % 3 ? `${id}=${weight(1)}` : id)).join(',');
    case 4: return [...vibes(1 + (i % 5)), `bpm=${bpm()}`].join(',');
    default: return [...vibes(2 + (i % 3)).map((id) => `${id}=${weight(2)}`), range()].join(',');
  }
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
  const spec = newSpec(c.seed, c.request ?? c.vibes);
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
    parts.passes.push(engine.kick ? { ...rest, kick: kickKindOf(spec.seed, engine.kick.kind, pass, !!(spec.pins && spec.pins.tunedKick)) } : rest);
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
  const asked = c.request ?? (c.vibes && c.vibes.length ? c.vibes.join(',') : '-');
  return [`${c.set}:${c.index}`, c.seed >>> 0, asked, ...named, `passes=${h.passes.join(',')}`].join(' ');
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
