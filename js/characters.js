// Sound profiles.
//
// A profile is a set of constraints -- tempo band, scale pool, instruments,
// density, metre, form -- that move together. On their own they would be
// sixteen boxes. They are not used that way: every loop draws a *weight*
// across several of them, so the palette is a continuous space rather than
// sixteen points, and a loop can be mostly one thing with three others
// colouring it.
//
// Names are deliberately about the sound rather than where it came from.

import { pow } from './dlmath.js';
import { blocksOf, number, readBlock } from './data.js';

// The profiles themselves are data (queue item 78): data/characters.txt, one
// [character id] block each, in this order; docs/VIBES.md and the file's own
// comments say how to edit one.

// What each key of a profile holds (js/data.js, READ): a number, a range, a
// weighted pool, and so on. A key not named here is an error.
const SCHEMA = {
  label: 'word',
  groove: 'word',
  polymeter: 'flag',
  microLoop: 'flag',
  borrowedBright: 'words',
  'tone.*': 'range',
  'feel.*': 'range',
  gaits: (rows) => rows.map((r) => {
    if (r.length !== 5) throw new Error('expected weight, metre, steps a bar and a tempo range');
    return { weight: number(r[0]), metre: r[1], stepsPerBar: number(r[2]), bpm: [number(r[3]), number(r[4])] };
  }),
  progressions: (rows) => rows.map((r) => r.map(degree)),
  'progressions.*': (rows) => rows.map((r) => r.map(degree)),
};
for (const k of ['bpm', 'swing', 'hatDensity']) SCHEMA[k] = 'range';
for (const k of ['stepsPerBar', 'bars', 'chordSize']) SCHEMA[k] = 'numpool';
for (const k of ['scales', 'chordVoices', 'melodyVoices', 'bassStyles', 'bassVoices', 'textures', 'kits']) SCHEMA[k] = 'pool';
for (const k of ['weight', 'level', 'airy', 'drums', 'restBar', 'flatSeven', 'quartal', 'pointillist', 'skipStep',
  'glide', 'dubEcho', 'rolls', 'fourFloor', 'pump', 'drone', 'ornament', 'harmonize', 'chug']) SCHEMA[k] = 'num';

// A scale degree in a progression: a number, or a chord that is spelled out,
// '4M' (degree 4, a major chord) and '6fM' (degree 6 flattened, major).
function degree(token) {
  const m = /^(\d+)(f?)(M?)$/.exec(token);
  if (!m) throw new Error(`'${token}' is not a degree`);
  if (!m[2] && !m[3]) return Number(m[1]);
  const d = { d: Number(m[1]) };
  if (m[2]) d.flat = true;
  if (m[3]) d.major = true;
  return d;
}

export const CHARACTERS = Object.fromEntries(blocksOf('characters', 'character').map(
  (b) => [b.name, readBlock(b, SCHEMA, 'data/characters.txt')],
));

// Saved loops from before the rename still resolve.
export const LEGACY_NAMES = {
  tape: 'dust',
  hyrule: 'glade',
  field: 'thaw',
  postcard: 'haven',
  plantasia: 'bloom',
  airports: 'vapor',
};

export function resolveKey(key) {
  if (CHARACTERS[key]) return key;
  return LEGACY_NAMES[key] || 'dust';
}

export const CHARACTER_WEIGHTS = Object.entries(CHARACTERS).map(([k, c]) => [k, c.weight]);

export const POLY_CYCLES = [3, 4, 5, 7, 8, 9, 11];

// ---------------------------------------------------------------- mixing

const NUM_RANGES = ['bpm', 'swing', 'hatDensity'];
const NUM_SCALARS = [
  'drums', 'restBar', 'level', 'flatSeven', 'quartal', 'pointillist',
  'skipStep', 'glide', 'airy', 'pump', 'rolls', 'fourFloor', 'dubEcho',
];
const WEIGHTED = [
  'scales', 'stepsPerBar', 'bars', 'chordVoices', 'melodyVoices',
  'bassStyles', 'bassVoices', 'chordSize', 'textures',
];
const FEEL_AXES = ['lift', 'energy', 'warmth'];

function normalise(mix) {
  const out = {};
  let total = 0;
  for (const [k, w] of Object.entries(mix)) {
    if (w > 0) total += w;
  }
  if (!total) return { dust: 1 };
  for (const [k, w] of Object.entries(mix)) {
    if (w > 0) out[resolveKey(k)] = (out[resolveKey(k)] || 0) + w / total;
  }
  return out;
}

// Combine any number of profiles by weight. Pure, so a spec only stores the
// weights and the blend rebuilds identically every time.
export function blendMix(mix) {
  const w = normalise(mix);
  const keys = Object.keys(w);
  if (keys.length === 1) return CHARACTERS[keys[0]];

  const out = { blended: true };
  out.label = keys
    .slice()
    .sort((a, b) => w[b] - w[a])
    .map((k) => CHARACTERS[k].label)
    .join('/');

  // Numbers blend with the dominant profile weighted far more heavily than
  // its share suggests.
  //
  // A plain weighted average is what a mix "should" be, and it was quietly
  // destroying the thing profiles exist for. Averaging four tempo bands
  // lands near the middle every time: measured, single-profile loops had a
  // tempo spread of sd 29.7 while three- and four-profile loops had 20.5.
  // Since most loops blend, most loops were being pulled toward a middling
  // tempo, middling energy and middling density -- audible as everything
  // sounding like a lullaby. Raising the weights to a power before
  // normalising keeps a mostly-Shatter loop actually fast while still
  // letting the other profiles colour it.
  const sharp = {};
  let sharpTotal = 0;
  for (const key of keys) {
    sharp[key] = pow(w[key], 2.2);
    sharpTotal += sharp[key];
  }
  for (const key of keys) sharp[key] /= sharpTotal;

  for (const k of NUM_RANGES) {
    let lo = 0;
    let hi = 0;
    for (const key of keys) {
      const v = CHARACTERS[key][k] || [0, 0];
      lo += v[0] * sharp[key];
      hi += v[1] * sharp[key];
    }
    out[k] = [lo, hi];
  }

  for (const k of NUM_SCALARS) {
    let v = 0;
    for (const key of keys) v += (CHARACTERS[key][k] ?? 0) * sharp[key];
    if (v) out[k] = v;
  }

  // Pools are unioned, not replaced, so a mostly-Dust loop with a little
  // Glade in it can still reach for an ocarina.
  for (const k of WEIGHTED) {
    const acc = new Map();
    for (const key of keys) {
      for (const [name, weight] of CHARACTERS[key][k] || []) {
        acc.set(name, (acc.get(name) || 0) + weight * w[key]);
      }
    }
    out[k] = [...acc].filter(([, v]) => v > 0.001);
  }

  out.tone = {};
  for (const k of ['warmth', 'space', 'wobble']) {
    let lo = 0;
    let hi = 0;
    for (const key of keys) {
      lo += CHARACTERS[key].tone[k][0] * w[key];
      hi += CHARACTERS[key].tone[k][1] * w[key];
    }
    out.tone[k] = [lo, hi];
  }

  out.feel = {};
  for (const axis of FEEL_AXES) {
    let lo = 0;
    let hi = 0;
    for (const key of keys) {
      const range = CHARACTERS[key].feel[axis];
      lo += range[0] * sharp[key];
      hi += range[1] * sharp[key];
    }
    out.feel[axis] = [lo, hi];
  }

  // All-or-nothing traits go to whichever profile dominates.
  const lead = keys.reduce((a, b) => (w[a] >= w[b] ? a : b));
  out.polymeter = !!CHARACTERS[lead].polymeter;
  out.microLoop = !!CHARACTERS[lead].microLoop;
  // So are the things a profile brings of its own: how tempo and metre go
  // together, which kits it plays, its progressions, the drone, ornaments
  // and the harmony line, the chug and the grooves. A loop that tide merely
  // colours does not start waltzing.
  for (const k of ['gaits', 'kits', 'progressions', 'drone', 'ornament', 'harmonize', 'chug', 'groove']) {
    if (CHARACTERS[lead][k] !== undefined) out[k] = CHARACTERS[lead][k];
  }
  return out;
}
