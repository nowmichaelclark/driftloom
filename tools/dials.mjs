// The ten dials, measured (queue item 34).
//
// Run with:  node tools/dials.mjs <export.jsonl | DL1-code ...> [--json <file>]
//
// Every ranked loop needs a position on the ten dials (docs/MOODS.md,
// "Dials" and "Maps and clouds") before Mikey's answers can be turned into
// word regions. This measures them from the loop's own notes: decodeSong,
// then render(spec) as stats.mjs does, so a dial reads the pattern as heard
// -- after entry schedules and gaps, every layer's own cycle unrolled across
// the loop, no wander (`/rank` plays the loop as composed). No audio is
// rendered: the two dials that are about sound rather than notes (warmth
// and edge) read the spec's tone and the voices' known character instead.
//
// Each dial is 0 to 1 twice over. The raw value is the formula at its
// function below; the scaled value is that raw value's percentile in a
// fixed corpus (2,000 loops at stats.mjs's default seed, test/dial-scale.json),
// so 0.5 is the catalog median. The scale does not move between runs:
// regenerating it is a deliberate step (`--write-scale`), like the balance
// lock's, taken when a generator change moves the corpus.
//
// Six dials are also a cloud, bar by bar, each bar weighted by how much
// sounds in it, and the loop's point is the weighted mean of its bars: lift,
// energy, ground, height, bounce and tang. The other four stay loop-wide
// because what they read is fixed for the whole loop (warmth: the tone dial
// and the voices; room: the space dial, the voicing, the tune's span; edge:
// the voices and their note lengths; oddness: the mode and the pairing).

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { newSpec, render, STEPS_PER_BAR, metreOf } from '../js/generator.js';
import { Rng } from '../js/rng.js';
import { SCALE_BRIGHTNESS, SCALES } from '../js/theory.js';
import { decodeSong } from '../js/share.js';

const HERE = path.dirname(fileURLToPath(import.meta.url));
export const SCALE_FILE = path.join(HERE, '..', 'test', 'dial-scale.json');

export const DIALS = ['lift', 'energy', 'warmth', 'room', 'ground', 'edge', 'height', 'bounce', 'tang', 'oddness'];
export const CLOUD_DIALS = ['lift', 'energy', 'ground', 'height', 'bounce', 'tang'];

// The corpus the scale is taken from: stats.mjs's default (--n 2000 --seed 1).
export const SCALE_N = 2000;
export const SCALE_SEED = 1;

const clamp01 = (v) => Math.max(0, Math.min(1, v));

// ------------------------------------------------------------ the voices

// What each voice is like, for the two dials that are about sound. Two
// figures, both the hands' reading of the voice as built (docs/EARS.md and
// the voice comments in js/synth.js), not measurements:
//   attack  1 struck or plucked (the note starts with a click or a hit and
//           decays), 0 sustained (bowed, blown, sung, a pad), 0.5 between;
//   bright  0 dark and round to 1 glassy, where its energy sits.
// The 2-5 kHz share the core baseline measures per melody voice
// (test/core-baseline.json, `tone`) is read beside them for edge; it is
// near zero for every voice but two (pluck 11.1%, stab 13.8%), so on its
// own it cannot tell the others apart.
const VOICE = {
  pluck: [1, 0.7], bell: [1, 0.9], keys: [1, 0.4], saw: [0, 0.8], ocarina: [0, 0.4],
  flute: [0, 0.5], harp: [1, 0.6], musicbox: [1, 0.9], fiddle: [0, 0.6], accordion: [0, 0.5],
  piano: [1, 0.5], sine: [0, 0.1], rhodes: [1, 0.3], moog: [0.5, 0.6], whistle: [0, 0.7],
  softpad: [0, 0.1], analoglead: [0.5, 0.7], prepared: [1, 0.7], celeste: [1, 0.85],
  kalimba: [1, 0.6], marimba: [1, 0.5], vowel: [0, 0.4], hum: [0, 0.15], choir: [0, 0.3],
  templebell: [1, 0.7], tubular: [1, 0.75], stab: [1, 0.9], panflute: [0, 0.5],
  pad: [0, 0.3], moogpad: [0, 0.3], analogpad: [0, 0.35], nylon: [1, 0.45],
  // Bass voices.
  sub: [0, 0], round: [0, 0.1], pluckbass: [1, 0.3], moogbass: [0.5, 0.35], fifths: [0, 0.2], rhodesbass: [1, 0.2],
  // Texture kinds (an event's `kind`).
  bells: [1, 0.9], chime: [1, 0.9], swell: [0, 0.2], drops: [1, 0.6], wind: [0, 0.5], waves: [0, 0.4],
};
// Drums: every hit is struck; hats and the jingle are the bright ones.
const DRUM_BRIGHT = { hat: 0.9, ohat: 0.9, jingle: 0.85, ojingle: 0.85, shaker: 0.8, rim: 0.6, clap: 0.6, snare: 0.5, tap: 0.4, kick: 0.1, frame: 0.2 };

let presenceCache = null;
function presence(voice) {
  if (!presenceCache) {
    presenceCache = {};
    try {
      const base = JSON.parse(fs.readFileSync(path.join(HERE, '..', 'test', 'core-baseline.json'), 'utf8'));
      for (const [k, r] of Object.entries(base.voices.rows)) {
        const [layer, name] = k.split('/');
        if (layer === 'melody' && typeof r.tone === 'number') presenceCache[name] = r.tone;
      }
    } catch { /* no baseline: the term reads 0 */ }
  }
  return presenceCache[voice] || 0;
}

// How unusual each mode sounds next to the major scale, 0 (ionian) to 1
// (whole-tone): the hands' ordering from the brief's examples (whole-tone,
// hirajoshi and phrygian far from ionian), the scales' own intervals (a
// semitone above the root, an augmented second, no perfect fifth) and how
// often the catalog draws them.
const MODE_ODD = {
  ionian: 0, majorPent: 0.1, mixolydian: 0.2, minorPent: 0.2, dorian: 0.25, aeolian: 0.3,
  lydian: 0.4, yo: 0.4, ritusen: 0.4, lydianDominant: 0.5, kumoi: 0.5, dorianSharp4: 0.6,
  harmonicMinor: 0.6, akebono: 0.6, insen: 0.7, phrygian: 0.7, phrygianDominant: 0.7,
  hirajoshi: 0.8, wholeTone: 1,
};

// ------------------------------------------------------------ the notes

// Every sounding note of every layer across the whole loop, a layer's own
// cycle unrolled, in steps from the loop's start.
function soundingNotes(p) {
  const total = p.totalSteps;
  const out = [];
  for (const layer of Object.keys(p.tracks)) {
    const cycle = (p.cycles && p.cycles[layer]) || total;
    for (let off = 0; off < total; off += cycle) {
      for (const e of p.tracks[layer]) {
        if (!(e.vel > 0)) continue;
        const step = e.step + off;
        if (step >= total) continue;
        const pitches = e.notes || (e.midi != null ? [e.midi] : []);
        out.push({
          layer, step, dur: e.dur || 1, vel: e.vel, pitches,
          voice: layer === 'texture' ? e.kind : e.voice, inst: e.inst || null,
        });
      }
    }
  }
  return out.sort((a, b) => a.step - b.step);
}

const sum = (xs) => xs.reduce((a, b) => a + b, 0);
function wmean(pairs) {
  let w = 0;
  let s = 0;
  for (const [v, wt] of pairs) { if (v == null || !(wt > 0)) continue; s += v * wt; w += wt; }
  return w ? s / w : null;
}
function wmedian(pairs) {
  const xs = pairs.filter(([v, w]) => v != null && w > 0).sort((a, b) => a[0] - b[0]);
  const total = sum(xs.map(([, w]) => w));
  if (!total) return null;
  let acc = 0;
  for (const [v, w] of xs) { acc += w; if (acc >= total / 2) return v; }
  return xs[xs.length - 1][0];
}

// A chord's quality from the root the harmony gave it (its slot's
// `rootMidi`; the lowest note when there is none, which an inversion would
// misread): 1 with a major third above the root and no minor third, 0 the
// other way round, 0.5 with both or neither (a sus, a quartal stack, a
// cluster).
function majorness(pitches, root = null) {
  if (!pitches.length) return null;
  const low = root ?? Math.min(...pitches);
  const pcs = new Set(pitches.map((n) => (((n - low) % 12) + 12) % 12));
  const maj = pcs.has(4);
  const min = pcs.has(3);
  return maj && !min ? 1 : min && !maj ? 0 : 0.5;
}

// How tart one chord is: 1 with a semitone in it (a b9, a cluster: two
// pitch classes a semitone apart), 0.6 with a close second (two notes of
// the voicing two semitones apart, an added second), 0 for a plain triad or
// a seventh voiced open.
function tartness(pitches) {
  if (pitches.length < 2) return 0;
  const pcs = [...new Set(pitches.map((n) => ((n % 12) + 12) % 12))].sort((a, b) => a - b);
  for (let i = 0; i < pcs.length; i++) {
    const d = (pcs[(i + 1) % pcs.length] - pcs[i] + 12) % 12;
    if (d === 1) return 1;
  }
  const sorted = pitches.slice().sort((a, b) => a - b);
  for (let i = 0; i + 1 < sorted.length; i++) if (sorted[i + 1] - sorted[i] === 2) return 0.6;
  return 0;
}

// ------------------------------------------------------------ measuring

// The raw dials of one spec, the per-bar clouds and per-layer height and
// edge. Deterministic: render is.
export function measureSpec(spec, scale = null) {
  const p = render(spec);
  const spb = p.stepsPerBar || spec.stepsPerBar || STEPS_PER_BAR;
  const bars = Math.max(1, Math.round(p.totalSteps / spb));
  const sd = 60 / spec.bpm / 4; // seconds per step
  const barSec = spb * sd;
  const beat = metreOf(spec) === '6/8' ? 6 : 4;
  const notes = soundingNotes(p);
  const pitched = notes.filter((n) => n.layer !== 'drums' && n.pitches.length);
  const melody = notes.filter((n) => n.layer === 'melody');
  const chords = notes.filter((n) => n.layer === 'chords');
  const bright = SCALE_BRIGHTNESS[spec.scale] ?? 0.5;
  const barOf = (n) => Math.min(bars - 1, Math.floor(n.step / spb));
  const inBar = Array.from({ length: bars }, () => []);
  for (const n of notes) inBar[barOf(n)].push(n);

  // The sounding notes of the melody in order, for leaps and odd steps.
  const line = melody.slice().sort((a, b) => a.step - b.step);
  const leapsAt = new Map();
  for (let i = 1; i < line.length; i++) {
    const d = Math.abs(line[i].pitches[0] - line[i - 1].pitches[0]);
    leapsAt.set(line[i], d);
  }

  const cloud = [];
  for (let b = 0; b < bars; b++) {
    const here = inBar[b];
    // How much sounds in the bar: note-steps times velocity, every layer.
    const weight = sum(here.map((n) => n.vel * Math.min(n.dur, spb) * Math.max(1, n.pitches.length || 1)));
    const from = b * spb;
    const to = from + spb;

    // lift: the mode's brightness, and the share of chord time in this bar
    // on chords with a major third. 0.7 and 0.3: at half each the chords'
    // quality, which moves bar to bar whatever the mode, blurred the words
    // the mode separates best (happy, sunny, reflective in Mikey's 181); at
    // 0.7 the dial separates them as well as the mode alone and still moves
    // with the chords. A bar without chords reads the mode alone.
    const chordTime = [];
    for (const n of chords) {
      const overlap = Math.min(n.step + n.dur, to) - Math.max(n.step, from);
      if (overlap > 0) chordTime.push([majorness(n.pitches, rootAt(p, n.step)), overlap]);
    }
    const major = wmean(chordTime);
    const lift = major == null ? bright : 0.7 * bright + 0.3 * major;

    // energy: tempo, onsets a second over every layer, drum hits a bar.
    // Tempo on 50-160 bpm, onsets on a log scale (16 a second reads 1),
    // drum hits on 0-16. Two parts note density to one part tempo: two
    // loops at one tempo differ most in how much happens in them.
    const onsets = here.length;
    const drumHits = here.filter((n) => n.layer === 'drums').length;
    const tempoN = clamp01((spec.bpm - 50) / 110);
    const rateN = clamp01(Math.log1p(onsets / barSec) / Math.log1p(16));
    const drumN = clamp01(drumHits / 16);
    const energy = 0.3 * tempoN + 0.45 * rateN + 0.25 * drumN;

    // ground: the share of beats that land on a kick or a bass onset, three
    // parts, and one part the share of beats not held over by a melody or
    // chord note tied across them (a line floating over the beat).
    let grounded = 0;
    let tied = 0;
    let beats = 0;
    for (let s = from; s < to; s += beat) {
      beats++;
      if (here.some((n) => n.step === s && (n.layer === 'bass' || n.inst === 'kick' || n.inst === 'frame'))) grounded++;
      if (notes.some((n) => (n.layer === 'melody' || n.layer === 'chords') && n.step < s && n.step + n.dur > s)) tied++;
    }
    const ground = 0.75 * (grounded / beats) + 0.25 * (1 - tied / beats);

    // height: the sounding-weighted median pitch of melody and texture,
    // MIDI 36 to 96 onto 0 to 1.
    const high = here.filter((n) => n.layer === 'melody' || n.layer === 'texture')
      .flatMap((n) => n.pitches.map((m) => [m, n.vel * n.dur]));
    const hm = wmedian(high);
    const height = hm == null ? null : clamp01((hm - 36) / 60);

    // bounce: dotted and syncopated onsets, short notes and leaps of a
    // fourth or more, over the tune and the bass. Each a share of their
    // onsets in the bar; the mean of the four.
    const tuneBass = here.filter((n) => n.layer === 'melody' || n.layer === 'bass');
    let bounce = null;
    if (tuneBass.length) {
      const pos = (n) => (n.step - from) % beat;
      const synco = sum(tuneBass.map((n) => (pos(n) === 0 ? 0 : pos(n) % 2 === 1 ? 1 : 0.5))) / tuneBass.length;
      const dotted = tuneBass.filter((n) => n.dur === 3 || n.dur === 6).length / tuneBass.length;
      const short = tuneBass.filter((n) => n.dur <= 1).length / tuneBass.length;
      const tune = here.filter((n) => n.layer === 'melody' && leapsAt.has(n));
      const leaps = tune.length ? tune.filter((n) => leapsAt.get(n) >= 5).length / tune.length : 0;
      bounce = (synco + dotted + short + leaps) / 4;
    }

    // tang: chord time on tart chords (tartness, above), against plain.
    const tang = chordTime.length
      ? wmean(chords.map((n) => [tartness(n.pitches), Math.min(n.step + n.dur, to) - Math.max(n.step, from)]))
      : null;

    cloud.push({ bar: b, weight, lift, energy, ground, height, bounce, tang });
  }

  const point = {};
  for (const d of CLOUD_DIALS) point[d] = wmean(cloud.map((c) => [c[d], c.weight]));

  // warmth: the Sound panel's warmth, and how dark the voices sounding are
  // (1 - bright above, weighted by how much each sounds). Loop-wide.
  const voiceBright = wmean(notes.map((n) => [
    n.layer === 'drums' ? (DRUM_BRIGHT[n.inst] ?? 0.5) : (VOICE[n.voice] ? VOICE[n.voice][1] : 0.5),
    n.vel * Math.min(n.dur, spb) * Math.max(1, n.pitches.length),
  ]));
  const warmth = clamp01(0.55 * (spec.tone ? spec.tone.warmth : 0.6) + 0.45 * (1 - (voiceBright ?? 0.5)));

  // room: the Sound panel's space, the chords' spread (lowest to highest
  // note, 0 to 36 semitones) and the tune's span (0 to 24). Loop-wide.
  const spread = wmean(chords.map((n) => [Math.max(...n.pitches) - Math.min(...n.pitches), n.dur]));
  const tunePitches = line.map((n) => n.pitches[0]);
  const span = tunePitches.length ? Math.max(...tunePitches) - Math.min(...tunePitches) : 0;
  const room = clamp01(0.5 * (spec.tone ? spec.tone.space : 0.5) + 0.25 * clamp01((spread ?? 0) / 36) + 0.25 * clamp01(span / 24));

  // edge: struck and plucked against sustained (by onsets, every layer,
  // drums struck), short notes (the median note length, 1 step reads 1 and
  // a bar reads 0) and the 2-5 kHz share measured per melody voice (10%
  // reads 1). Loop-wide.
  const attack = wmean(notes.map((n) => [n.layer === 'drums' ? 1 : (VOICE[n.voice] ? VOICE[n.voice][0] : 0.5), n.vel]));
  const medDur = wmedian(pitched.map((n) => [n.dur, 1])) ?? spb;
  const shortness = clamp01(1 - Math.log(medDur) / Math.log(spb));
  const pres = clamp01(presence(p.meta.melodyVoice) / 10);
  const edge = clamp01(0.55 * (attack ?? 0.5) + 0.3 * shortness + 0.15 * pres);

  // oddness: the mode (MODE_ODD), tritones and augmented seconds in the
  // tune (their share of its steps, doubled), and how rare the loop's pair
  // of melody and keys voices is in the catalog (from the scale's counts).
  let odd = 0;
  for (let i = 1; i < line.length; i++) {
    const d = Math.abs(line[i].pitches[0] - line[i - 1].pitches[0]);
    if (d === 6 || d === 3) {
      // A minor third is only odd when it is a step of the scale (an
      // augmented second); a tritone always is.
      if (d === 6) odd++;
      else if (isStep(spec, line[i - 1].pitches[0], line[i].pitches[0])) odd++;
    }
  }
  const tuneOdd = line.length > 1 ? clamp01((2 * odd) / (line.length - 1)) : 0;
  const pairKey = pairOf(p);
  const rarity = scale ? pairRarity(scale, pairKey) : 0.5;
  const oddness = clamp01(0.5 * (MODE_ODD[spec.scale] ?? 0.5) + 0.25 * tuneOdd + 0.25 * rarity);

  const raw = {
    lift: point.lift ?? bright,
    energy: point.energy ?? 0,
    warmth,
    room,
    ground: point.ground ?? 0,
    edge,
    height: point.height ?? 0.5,
    bounce: point.bounce ?? 0,
    tang: point.tang ?? 0,
    oddness,
  };

  // Per layer, where it is cheap: height and edge for each pitched layer.
  const layers = {};
  for (const layer of ['bass', 'chords', 'melody', 'texture']) {
    const ns = notes.filter((n) => n.layer === layer);
    if (!ns.length) continue;
    const hm = wmedian(ns.flatMap((n) => n.pitches.map((m) => [m, n.vel * n.dur])));
    const at = wmean(ns.map((n) => [VOICE[n.voice] ? VOICE[n.voice][0] : 0.5, n.vel]));
    layers[layer] = { height: hm == null ? null : clamp01((hm - 36) / 60), edge: at };
  }

  return { raw, cloud, layers, pair: pairKey };
}

// The harmony's root under a step (a pitch class), or null.
function rootAt(p, step) {
  const h = p.harmony;
  if (!h || !h.slots || !h.slots.length) return null;
  const s = step % (h.cycleSteps || p.totalSteps);
  const slot = h.slots.find((x) => s >= x.startStep && s < x.startStep + x.lengthSteps);
  return slot && slot.rootMidi != null ? slot.rootMidi : null;
}

function isStep(spec, a, b) {
  const steps = SCALES[spec.scale] && SCALES[spec.scale].steps;
  if (!steps) return false;
  const deg = (m) => {
    const rel = m - spec.root;
    const pc = ((rel % 12) + 12) % 12;
    const i = steps.indexOf(pc);
    return i < 0 ? null : Math.floor(rel / 12) * steps.length + i;
  };
  const da = deg(a);
  const db = deg(b);
  return da != null && db != null && Math.abs(da - db) === 1;
}

function pairOf(p) {
  const keys = p.tracks.chords.find((e) => e.vel > 0);
  return `${p.meta.melodyVoice || '-'}|${keys ? keys.voice : '-'}`;
}

// -log of the pair's share of the corpus, onto 0 (the commonest pair) to 1
// (a pair the corpus never drew).
function pairRarity(scale, key) {
  const counts = scale.pairs || {};
  const total = scale.n || 1;
  const max = Math.max(1, ...Object.values(counts));
  const c = counts[key] || 0;
  const hi = Math.log(total / 0.5);
  const lo = Math.log(total / max);
  return clamp01((Math.log(total / (c || 0.5)) - lo) / (hi - lo));
}

// ------------------------------------------------------------ the scale

// The percentile of v in a dial's 101 quantiles (0th to 100th), linear in
// between, so 0.5 is the corpus median.
export function percentile(q, v) {
  if (v <= q[0]) return 0;
  if (v >= q[q.length - 1]) return 1;
  let lo = 0;
  let hi = q.length - 1;
  while (hi - lo > 1) {
    const mid = (lo + hi) >> 1;
    if (q[mid] <= v) lo = mid; else hi = mid;
  }
  // Ties (a dial with many loops on one value) take the middle of the run,
  // so a value shared by a third of the corpus reads as its middle.
  let first = lo;
  while (first > 0 && q[first - 1] === q[lo]) first--;
  let last = lo;
  while (last + 1 < q.length && q[last + 1] === q[lo]) last++;
  if (q[lo] === v && last > first) return ((first + last) / 2) / (q.length - 1);
  const frac = q[hi] === q[lo] ? 0 : (v - q[lo]) / (q[hi] - q[lo]);
  return (lo + frac) / (q.length - 1);
}

export function loadScale(file = SCALE_FILE) {
  return JSON.parse(fs.readFileSync(file, 'utf8'));
}

export function scaled(raw, scale) {
  const out = {};
  for (const d of DIALS) out[d] = percentile(scale.quantiles[d], raw[d]);
  return out;
}

export function corpusSpecs(n = SCALE_N, seed = SCALE_SEED) {
  const master = new Rng(seed);
  return Array.from({ length: n }, () => newSpec(master.seed32()));
}

// Two passes: the voice pairs first (oddness reads their rarity), then the
// dials.
export function buildScale(n = SCALE_N, seed = SCALE_SEED) {
  const specs = corpusSpecs(n, seed);
  const pairs = {};
  for (const s of specs) {
    const k = pairOf(render(s));
    pairs[k] = (pairs[k] || 0) + 1;
  }
  const partial = { n, pairs };
  const cols = Object.fromEntries(DIALS.map((d) => [d, []]));
  for (const s of specs) {
    const { raw } = measureSpec(s, partial);
    for (const d of DIALS) cols[d].push(raw[d]);
  }
  const quantiles = {};
  for (const d of DIALS) {
    const xs = cols[d].sort((a, b) => a - b);
    quantiles[d] = Array.from({ length: 101 }, (_, i) => {
      const x = (xs.length - 1) * (i / 100);
      const lo = Math.floor(x);
      const v = xs[lo] + (xs[Math.min(lo + 1, xs.length - 1)] - xs[lo]) * (x - lo);
      return Math.round(v * 1e6) / 1e6;
    });
  }
  return {
    about: `Queue item 34: each dial's 0th to 100th percentile over ${n} loops (newSpec at stats.mjs's corpus seed ${seed}), and the corpus's melody|keys voice pairs. Regenerate with node tools/dials.mjs --write-scale, on purpose, when a generator change moves the corpus.`,
    n, seed, quantiles, pairs,
  };
}

// One code (or spec) in, everything out.
export function dialsOf(specOrCode, scale = loadScale()) {
  const spec = typeof specOrCode === 'string' ? decodeSong(specOrCode) : specOrCode;
  const m = measureSpec(spec, scale);
  const s = scaled(m.raw, scale);
  const cloud = m.cloud.map((c) => {
    const o = { bar: c.bar, weight: Math.round(c.weight * 1000) / 1000 };
    for (const d of CLOUD_DIALS) o[d] = c[d] == null ? null : percentile(scale.quantiles[d], c[d]);
    return o;
  });
  return { raw: m.raw, scaled: s, cloud, layers: m.layers };
}

// ------------------------------------------------------------ reading input

export function readExport(file) {
  return fs.readFileSync(file, 'utf8').split('\n').map((l) => l.trim()).filter(Boolean)
    .map((l) => { try { return JSON.parse(l); } catch { return null; } })
    .filter((o) => o && o.code);
}

// ------------------------------------------------------------ yardsticks

function pearson(xs, ys) {
  const n = xs.length;
  const mx = sum(xs) / n;
  const my = sum(ys) / n;
  let sxy = 0;
  let sxx = 0;
  let syy = 0;
  for (let i = 0; i < n; i++) {
    sxy += (xs[i] - mx) * (ys[i] - my);
    sxx += (xs[i] - mx) ** 2;
    syy += (ys[i] - my) ** 2;
  }
  return sxx && syy ? sxy / Math.sqrt(sxx * syy) : 0;
}

function lead(spec) {
  return Object.entries(spec.mix || {}).sort((a, b) => b[1] - a[1])[0]?.[0] || '?';
}

function yardsticks(scale, n) {
  const specs = corpusSpecs(n, SCALE_SEED);
  const rows = specs.map((s) => ({ spec: s, d: scaled(measureSpec(s, scale).raw, scale) }));
  const out = [''];
  out.push(`driftloom dials: the yardsticks over ${n} loops (corpus seed ${SCALE_SEED})`);
  out.push('');
  out.push('  correlation (Pearson, scaled dials); * above 0.7');
  out.push('  ' + ''.padEnd(9) + DIALS.map((d) => d.slice(0, 7).padStart(8)).join(''));
  const flagged = [];
  for (const a of DIALS) {
    const cells = DIALS.map((b) => {
      const r = pearson(rows.map((x) => x.d[a]), rows.map((x) => x.d[b]));
      if (a < b && Math.abs(r) > 0.7) flagged.push(`${a}/${b} ${r.toFixed(2)}`);
      return (a === b ? '1' : r.toFixed(2) + (Math.abs(r) > 0.7 ? '*' : ' ')).padStart(8);
    });
    out.push('  ' + a.padEnd(9) + cells.join(''));
  }
  out.push(flagged.length ? `  pairs above 0.7: ${flagged.join(', ')}` : '  no pair above 0.7');
  out.push('');
  out.push('  mean scaled dial by leading profile');
  out.push('  ' + 'profile'.padEnd(11) + 'loops'.padStart(6) + DIALS.map((d) => d.slice(0, 7).padStart(8)).join(''));
  const profiles = [...new Set(rows.map((r) => lead(r.spec)))].sort();
  for (const pr of profiles) {
    const here = rows.filter((r) => lead(r.spec) === pr);
    out.push('  ' + pr.padEnd(11) + String(here.length).padStart(6)
      + DIALS.map((d) => (sum(here.map((r) => r.d[d])) / here.length).toFixed(2).padStart(8)).join(''));
  }
  out.push('');
  out.push(`  lift against feel.lift: r = ${pearson(rows.map((r) => r.d.lift), rows.map((r) => r.spec.feel.lift)).toFixed(2)}`);
  out.push(`  energy against feel.energy: r = ${pearson(rows.map((r) => r.d.energy), rows.map((r) => r.spec.feel.energy)).toFixed(2)}`);
  out.push(`  warmth against feel.warmth: r = ${pearson(rows.map((r) => r.d.warmth), rows.map((r) => r.spec.feel.warmth)).toFixed(2)}`);
  out.push('');
  return out.join('\n');
}

// Mikey's taps against the dials: for each word, the mean scaled dial on
// the loops where it was shown and tapped (fits) and shown but left
// (doesn't fit).
function earsReport(file, scale, words, dials) {
  const lines = readExport(file).filter((l) => Array.isArray(l.words) && !l.skip);
  const memo = new Map();
  const of = (code) => {
    if (!memo.has(code)) memo.set(code, scaled(measureSpec(decodeSong(code), scale).raw, scale));
    return memo.get(code);
  };
  const out = [''];
  out.push(`driftloom dials against the taps: ${lines.length} trials, ${new Set(lines.map((l) => l.code)).size} loops (${path.basename(file)})`);
  out.push('');
  out.push('  ' + 'word'.padEnd(14) + 'fits'.padStart(5) + ' no'.padStart(5)
    + dials.map((d) => `${d} fit / no`.padStart(20)).join(''));
  for (const w of words) {
    const fit = lines.filter((l) => l.words.includes(w) && (l.taps || []).includes(w));
    const no = lines.filter((l) => l.words.includes(w) && !(l.taps || []).includes(w));
    const mean = (ls, d) => (ls.length ? sum(ls.map((l) => of(l.code)[d])) / ls.length : NaN);
    out.push('  ' + w.padEnd(14) + String(fit.length).padStart(5) + String(no.length).padStart(5)
      + dials.map((d) => {
        const a = mean(fit, d);
        const b = mean(no, d);
        const cell = `${Number.isFinite(a) ? a.toFixed(2) : ' -  '} / ${Number.isFinite(b) ? b.toFixed(2) : ' -  '}`;
        const sign = Number.isFinite(a) && Number.isFinite(b) ? (a - b >= 0 ? ' +' : ' -') : '  ';
        return (cell + sign).padStart(20);
      }).join(''));
  }
  // The same separation as a standardized difference (fit mean less
  // doesn't-fit mean, over their pooled sd), the dial beside the raw
  // feature the brain's scratch check used, so "at least as clearly" is a
  // number: energy against tempo and notes a second, lift against the
  // mode's brightness.
  const feat = new Map();
  const featOf = (code) => {
    if (!feat.has(code)) {
      const spec = decodeSong(code);
      const p = render(spec);
      const secs = p.totalSteps * (60 / spec.bpm / 4);
      const onsets = soundingNotes(p).length;
      feat.set(code, { tempo: spec.bpm, perSecond: onsets / secs, brightness: SCALE_BRIGHTNESS[spec.scale] ?? 0.5 });
    }
    return feat.get(code);
  };
  const effect = (fit, no, f) => {
    const a = fit.map(f);
    const b = no.map(f);
    if (a.length < 2 || b.length < 2) return NaN;
    const m = (xs) => sum(xs) / xs.length;
    const v = (xs) => sum(xs.map((x) => (x - m(xs)) ** 2)) / (xs.length - 1);
    const sp = Math.sqrt(((a.length - 1) * v(a) + (b.length - 1) * v(b)) / (a.length + b.length - 2));
    return sp ? (m(a) - m(b)) / sp : NaN;
  };
  // Raw and scaled both: a percentile is not a linear map, so a dial's
  // scaled value is compared with the features' raw values only loosely;
  // its raw value is the like-for-like one.
  const rawOf = (code) => {
    if (!feat.has(code + '#raw')) feat.set(code + '#raw', measureSpec(decodeSong(code), scale).raw);
    return feat.get(code + '#raw');
  };
  const cols = [
    ['energy', (l) => of(l.code).energy],
    ['energy raw', (l) => rawOf(l.code).energy],
    ['tempo', (l) => featOf(l.code).tempo],
    ['notes/s', (l) => featOf(l.code).perSecond],
    ['lift', (l) => of(l.code).lift],
    ['lift raw', (l) => rawOf(l.code).lift],
    ['mode bright', (l) => featOf(l.code).brightness],
  ];
  out.push('');
  out.push('  standardized difference, fits less doesn\'t-fit (pooled sd)');
  out.push('  ' + 'word'.padEnd(14) + cols.map(([c]) => c.padStart(13)).join(''));
  for (const w of words) {
    const fit = lines.filter((l) => l.words.includes(w) && (l.taps || []).includes(w));
    const no = lines.filter((l) => l.words.includes(w) && !(l.taps || []).includes(w));
    out.push('  ' + w.padEnd(14) + cols.map(([, f]) => {
      const e = effect(fit, no, f);
      return (Number.isFinite(e) ? (e >= 0 ? '+' : '') + e.toFixed(2) : '-').padStart(13);
    }).join(''));
  }
  out.push('');
  return out.join('\n');
}

// ------------------------------------------------------------ the command

const USAGE = `
driftloom dials (queue item 34)

  node tools/dials.mjs <export.jsonl | DL1-code ...> [options]

  --json <file>      also write the raw and scaled dials, the per-bar clouds
                     and the per-layer height and edge, keyed by code
  --raw              print the raw values in the table instead of the scaled
  --write-scale      rebuild test/dial-scale.json from the corpus (2,000
                     loops at stats.mjs's default seed); a deliberate step
  --yardsticks [n]   the correlation table, the dials by leading profile and
                     lift against feel.lift, over n corpus loops (2000)
  --ears <export>    each energy and lift word's mean dials on the loops it
                     was tapped on and the ones it was left on
  --help             this

  Every dial is 0 to 1, scaled to its percentile in the corpus (0.5 is the
  catalog median). An export is read one line a trial; each distinct code
  is measured once. Score dials only: no audio is rendered.
`;

const EAR_WORDS = [
  'energetic', 'bouncy', 'lively', 'enthusiastic', 'adventurous', 'nimble',
  'cozy', 'tiptoeing', 'meditative', 'peaceful', 'soothing', 'foggy',
  'happy', 'sunny', 'reflective',
];

async function main(argv) {
  const opts = { inputs: [], json: null, raw: false, writeScale: false, yard: null, ears: null };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a === '--json') opts.json = argv[++i];
    else if (a === '--raw') opts.raw = true;
    else if (a === '--write-scale') opts.writeScale = true;
    else if (a === '--yardsticks') opts.yard = Number.isFinite(Number(argv[i + 1])) ? Number(argv[++i]) : SCALE_N;
    else if (a === '--ears') opts.ears = argv[++i];
    else if (a === '--help' || a === '-h') { console.log(USAGE); return 0; }
    else if (a.startsWith('--')) { console.error(`dials: unknown option ${a}\n${USAGE}`); return 2; }
    else opts.inputs.push(a);
  }

  if (opts.writeScale) {
    const t = Date.now();
    const scale = buildScale();
    fs.writeFileSync(SCALE_FILE, JSON.stringify(scale) + '\n');
    console.log(`wrote ${path.relative(process.cwd(), SCALE_FILE)}: ${scale.n} loops, ${Object.keys(scale.pairs).length} voice pairs, ${((Date.now() - t) / 1000).toFixed(1)} s`);
    return 0;
  }
  const scale = loadScale();
  if (opts.yard) { console.log(yardsticks(scale, opts.yard)); return 0; }
  if (opts.ears) { console.log(earsReport(opts.ears, scale, EAR_WORDS, ['energy', 'lift'])); return 0; }
  if (!opts.inputs.length) { console.log(USAGE); return 2; }

  const codes = [];
  for (const inp of opts.inputs) {
    if (fs.existsSync(inp)) for (const l of readExport(inp)) codes.push(l.code);
    else codes.push(inp);
  }
  const distinct = [...new Set(codes)];
  const t = Date.now();
  const results = {};
  for (const code of distinct) results[code] = dialsOf(code, scale);
  const ms = Date.now() - t;

  const head = DIALS.map((d) => d.slice(0, 7).padStart(8)).join('');
  console.log('');
  console.log(`driftloom dials, ${distinct.length} loops${opts.raw ? ' (raw)' : ' (percentile in the catalog; 0.5 is its median)'}, ${ms} ms`);
  console.log('');
  console.log('  ' + 'loop'.padEnd(16) + head);
  for (const code of distinct) {
    const r = results[code];
    const v = opts.raw ? r.raw : r.scaled;
    const name = decodeSong(code).name;
    console.log('  ' + name.padEnd(16) + DIALS.map((d) => v[d].toFixed(2).padStart(8)).join(''));
  }
  console.log('');
  if (opts.json) {
    fs.writeFileSync(opts.json, JSON.stringify(results, null, 1) + '\n');
    console.log(`  wrote ${opts.json}`);
  }
  return 0;
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  process.exitCode = await main(process.argv.slice(2));
}
