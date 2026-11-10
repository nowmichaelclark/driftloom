// The chords' octave check (queue item 84).
//
// Run with:  node tools/octave.mjs [--n 2000] [--seed 1] [--save FILE] [--against FILE]
//
// The chords sit an octave (or two) below where the progression is voiced
// whenever their centre is too near the tune's. That decision is read from
// the progression, so a change that leaves the harmony alone must not move
// it. For each loop of a corpus this reads the octave from the core's
// generator (tools/gen-lib.mjs: the chords as planned against the chords as
// rendered), then draws the loop again with one choice that is not harmonic
// changed, and counts the loops whose octave moved:
//   rests    the chords playing all the way through, with no entry schedule
//   tempo    a quarter faster
//   bass     the bass drawn again (a new style, voice and line)
// (Today's chord lengths against the loop's own, the variation the rule was
// written for, is not a thing a request can ask: the core reads the octave
// from today's lengths itself, `Render::chords_octave`.)
// `--save` writes every loop's octave and the gap between the tune's and
// the chords' centres to a file; `--against` reads one (made on another
// tree) and lists how many loops the two disagree on. Exit 1 if any
// variation flips a loop. Needs the tools' build (tools/build-core.sh
// --tools).

import fs from 'node:fs';
import { parts } from './gen-lib.mjs';
import { Rng } from '../js/rng.js';

const args = process.argv.slice(2);
const opt = (name, def) => {
  const i = args.indexOf(`--${name}`);
  return i >= 0 ? args[i + 1] : def;
};
const N = +opt('n', 2000);
const SEED = +opt('seed', 1);

// How many semitones the rendered chords sit from where the progression was
// voiced (the plan's harmony is as drawn, the form's after the register
// work); 0 for a loop with no chords.
function octaveOf(p) {
  const a = p.plan.harmony.events.find((e) => e.notes && e.notes.length);
  const b = p.form.harmony.events.find((e) => e.notes && e.notes.length);
  return a && b ? b.notes[0] - a.notes[0] : 0;
}

// The centre of a layer's sounding notes (by pitch, each note once), or null.
function centre(events) {
  const pitches = events.filter((e) => e.vel > 0).flatMap((e) => (e.notes ? e.notes : e.midi != null ? [e.midi] : []));
  return pitches.length ? pitches.reduce((x, y) => x + y, 0) / pitches.length : null;
}

/** The tune's centre less the chords', in semitones, or null where either rests. */
export function gapOf(p) {
  const m = centre(p.melody);
  const c = centre(p.chords);
  return m == null || c == null ? null : m - c;
}

const variations = {
  // The chords playing all the way through, no entry schedule: the one
  // layer whose forcing changes nothing else (a forced melody or drum
  // track can draw notes a loop did not have).
  rests: (spec) => parts(spec.seed, `edit.roll.chords=${spec.layerSeeds.chords >>> 0}`),
  tempo: (spec) => parts(spec.seed, `edit.bpm=${Math.min(190, Math.round(spec.bpm * 1.25))}`),
  bass: (spec) => parts(spec.seed, `edit.roll.bass=${(spec.layerSeeds.bass ^ 0x2545f491) >>> 0 || 1}`),
};

const corpus = new Rng(SEED);
const mine = [];
const gaps = [];
const flips = Object.fromEntries(Object.keys(variations).map((k) => [k, []]));
const shifts = new Map();
for (let i = 0; i < N; i++) {
  const made = parts(corpus.seed32());
  const spec = made.spec;
  const base = octaveOf(made);
  mine.push(base);
  gaps.push(gapOf(made));
  shifts.set(base, (shifts.get(base) || 0) + 1);
  for (const [name, make] of Object.entries(variations)) {
    if (octaveOf(make(spec)) !== base) flips[name].push(i);
  }
}

console.log(`${N} loops, corpus seed ${SEED}; the chords' octave: ${[...shifts].sort((a, b) => a[0] - b[0]).map(([k, v]) => `${k} x${v}`).join(', ')}`);
let bad = 0;
for (const [name, list] of Object.entries(flips)) {
  console.log(`${name.padEnd(8)} flips ${list.length}${list.length ? `  (loops ${list.slice(0, 10).join(', ')}${list.length > 10 ? ', ...' : ''})` : ''}`);
  bad += list.length;
}
const save = opt('save', null);
if (save) fs.writeFileSync(save, JSON.stringify({ octaves: mine, gaps }));
const against = opt('against', null);
if (against) {
  const other = JSON.parse(fs.readFileSync(against, 'utf8')).octaves;
  const moved = mine.map((v, i) => (v !== other[i] ? i : -1)).filter((i) => i >= 0);
  console.log(`against ${against}: ${moved.length} of ${N} loops take another octave (${moved.slice(0, 12).join(', ')}${moved.length > 12 ? ', ...' : ''})`);
}
process.exit(bad ? 1 : 0);
