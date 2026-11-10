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

// The profiles in their frozen order (append only): the order a share code
// numbers them in, and a request's set of styles is written in (queue item
// 90). The Rust core's `request::PROFILE_ORDER` is the same list.
export const PROFILE_ORDER = [
  'dust', 'glade', 'thaw', 'haven', 'bloom', 'vapor',
  'halcyon', 'clockwork', 'shatter', 'undertow',
  'grove', 'hollow', 'shrine', 'tide', 'cinder', 'wayfare',
];

// The voices a request can ask for, by layer, each in its frozen order
// (append only): the order a share code numbers them in, and a request's
// set of voices is written in (queue item 91). The names a layer's pools
// draw from, less the choir's (vowel, hum and choir): the rarities stay
// rare, and are not offered until Mikey says so. The Rust core's
// `request::VOICE_ORDER` is the same lists. For the drums the voice is the
// kit, for the texture its kind (less "none": that is the layer left out).
export const VOICE_ORDER = {
  drums: ['brush', 'tape', 'hand'],
  bass: ['sub', 'round', 'pluckbass', 'moogbass', 'fifths', 'rhodesbass'],
  chords: ['keys', 'pad', 'harp', 'accordion', 'piano', 'rhodes', 'moogpad', 'softpad', 'analogpad', 'prepared', 'musicbox',
    'kalimba', 'marimba', 'templebell', 'tubular', 'stab', 'nylon'],
  melody: ['pluck', 'bell', 'keys', 'saw', 'ocarina', 'flute', 'harp', 'musicbox', 'fiddle', 'accordion', 'piano', 'sine', 'rhodes', 'moog',
    'whistle', 'softpad', 'analoglead', 'prepared', 'celeste', 'kalimba', 'marimba', 'templebell', 'tubular', 'stab', 'panflute'],
  texture: ['bells', 'swell', 'drops', 'chime', 'wind', 'waves'],
};
