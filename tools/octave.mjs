// The chords' octave check (queue item 84).
//
// Run with:  node tools/octave.mjs [--n 2000] [--seed 1] [--js DIR] [--save FILE] [--against FILE]
//
// The chords sit an octave (or two) below where the progression is voiced
// whenever their centre is too near the tune's. That decision is read from
// the progression, so a change that leaves the harmony alone must not move
// it. For each loop of a corpus this draws the octave, then draws the loop
// again with one choice that is not harmonic changed, and counts the loops
// whose octave moved:
//   lengths  today's chord lengths against the loop's own (longChords)
//   rests    the chords playing all the way through, with no entry schedule
//   tempo    a quarter faster
//   bass     the bass drawn again (a new style, voice and line)
// `--save` writes every loop's octave to a file; `--against` reads one
// (made on another tree with `--js`, e.g. the old generator) and lists how
// many loops the two ways disagree on. Exit 1 if any variation flips a loop.

import fs from 'node:fs';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

const args = process.argv.slice(2);
const opt = (name, def) => {
  const i = args.indexOf(`--${name}`);
  return i >= 0 ? args[i + 1] : def;
};
const N = +opt('n', 2000);
const SEED = +opt('seed', 1);
const root = path.resolve(opt('js', new URL('../js', import.meta.url).pathname));
const gen = await import(pathToFileURL(path.join(root, 'generator.js')));
const { Rng } = await import(pathToFileURL(path.join(root, 'rng.js')));
const { newSpec, render, withTodaysChords, rerollLayer, cloneSpec } = gen;

const octaveOf = (p) => p.harmony.chordShift;

const variations = {
  lengths: (spec) => withTodaysChords(spec),
  // The chords playing all the way through, no entry schedule: the one
  // layer whose forcing changes nothing else (a forced melody or drum
  // track can draw notes a loop did not have).
  rests: (spec) => render({ ...cloneSpec(spec), forceLayers: { chords: true } }),
  tempo: (spec) => render({ ...cloneSpec(spec), bpm: Math.round(spec.bpm * 1.25) }),
  bass: (spec) => {
    const s = rerollLayer(spec, 'bass');
    s.layerSeeds.bass = (spec.layerSeeds.bass ^ 0x2545f491) >>> 0 || 1;
    return render(s);
  },
};

const corpus = new Rng(SEED);
const mine = [];
const flips = Object.fromEntries(Object.keys(variations).map((k) => [k, []]));
const shifts = new Map();
for (let i = 0; i < N; i++) {
  const spec = newSpec(corpus.seed32());
  const base = octaveOf(render(spec));
  mine.push(base);
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
if (save) fs.writeFileSync(save, JSON.stringify(mine));
const against = opt('against', null);
if (against) {
  const other = JSON.parse(fs.readFileSync(against, 'utf8'));
  const moved = mine.map((v, i) => (v !== other[i] ? i : -1)).filter((i) => i >= 0);
  console.log(`against ${against}: ${moved.length} of ${N} loops take another octave (${moved.slice(0, 12).join(', ')}${moved.length > 12 ? ', ...' : ''})`);
}
process.exit(bad ? 1 : 0);
