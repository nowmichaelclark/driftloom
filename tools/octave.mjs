// The chords' octave check (queue item 84).
//
// Run with:  node tools/octave.mjs [--n 2000] [--seed 1] [--save FILE] [--against FILE [--list]]
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
// It prints the gap's distribution (the tune's centre less the chords', on
// the sounding notes, as the blind album measured it: queue item 84b), for
// every loop and for the loops whose chords were moved down, and how many
// of those sit more than an octave under the tune.
// `--save` writes every loop's octave and that gap to a file; `--against`
// reads one (made on another tree) and lists how many loops the two
// disagree on: which way they moved, and the gap's spread on them before
// and after. Exit 1 if any variation flips a loop. Needs the tools' build
// (tools/build-core.sh --tools).

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
// work), as the move the register work was given. A whole octave moves
// every note; since item 84b a move of less than an octave down takes the
// notes at or over a line in each chord an octave down (`render::lower`:
// the line from the slot's chord, its top notes, as many as take its centre
// at least that far), so the move is read back by replaying that rule: the
// moves (whole octaves, and one between each pair of the partial rule's
// steps) that give every chord's notes exactly. `at` is the loop's figure:
// a whole octave where one explains it, else the middle of the moves that do.
const STEPS = [...new Set(Array.from({ length: 8 }, (_, n) => Array.from({ length: n + 1 }, (_, d) => (-12 * d) / (n + 1))).flat())].sort((a, b) => a - b);
const CANDIDATES = [-24, -12, 0, 12, 24, ...STEPS.slice(0, -1).map((x, i) => (x + STEPS[i + 1]) / 2)];
function lineOf(chord, by) {
  const s = [...chord].sort((a, b) => a - b);
  const down = Math.min(s.length, Math.max(1, Math.ceil((s.length * -by) / 12)));
  return s.length ? s[s.length - down] : null;
}
function lowered(notes, over, by) {
  if (by % 12 === 0) return notes.map((n) => n + by);
  const at = lineOf(over.length ? over : notes, by);
  return notes.map((n) => (at != null && n >= at ? n - 12 : n));
}
function slotAt(h, step) {
  const c = h.cycleSteps;
  const q = c ? ((step % c) + c) % c : step;
  for (let i = h.slots.length - 1; i >= 0; i--) if (q >= h.slots[i].startStep) return h.slots[i];
  return h.slots[0];
}
function octaveOf(p) {
  const a = p.plan.harmony.events;
  const b = p.form.harmony.events;
  let can = CANDIDATES;
  for (let i = 0; i < a.length; i++) {
    if (!a[i].notes || !a[i].notes.length || !b[i] || !b[i].notes) continue;
    const over = (slotAt(p.plan.harmony, a[i].step) || {}).notes || [];
    can = can.filter((by) => lowered(a[i].notes, over, by).every((n, j) => n === b[i].notes[j]));
  }
  const whole = can.filter((x) => x % 12 === 0).sort((x, y) => Math.abs(x) - Math.abs(y));
  const part = can.filter((x) => x % 12 !== 0).sort((x, y) => x - y);
  const at = whole.length ? whole[0] : part.length ? Math.round(100 * part[part.length >> 1]) / 100 : NaN;
  return { can, at };
}
// Two readings agree where some one move gives both.
const agree = (x, y) => x.can.some((v) => y.can.includes(v));

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
  const read = octaveOf(made);
  const base = read.at;
  mine.push(base);
  gaps.push(gapOf(made));
  shifts.set(base, (shifts.get(base) || 0) + 1);
  for (const [name, make] of Object.entries(variations)) {
    if (!agree(octaveOf(make(spec)), read)) flips[name].push(i);
  }
}

const whole = (o) => o % 12 === 0 || Number.isNaN(o);
const octs = [...shifts].filter(([k]) => whole(k)).sort((a, b) => a[0] - b[0]).map(([k, v]) => `${k} x${v}`);
const part = [...shifts].filter(([k]) => !whole(k)).reduce((a, [, v]) => a + v, 0);
console.log(`${N} loops, corpus seed ${SEED}; the chords' octave: ${octs.join(', ')}${part ? `, less than an octave down (item 84b) x${part}` : ''}`);
let bad = 0;
for (const [name, list] of Object.entries(flips)) {
  console.log(`${name.padEnd(8)} flips ${list.length}${list.length ? `  (loops ${list.slice(0, 10).join(', ')}${list.length > 10 ? ', ...' : ''})` : ''}`);
  bad += list.length;
}
// The gap's spread: its bins and its percentiles, over the loops given.
const BINS = [[-Infinity, 0], [0, 5], [5, 7], [7, 10], [10, 12.5], [12.5, 14], [14, 17], [17, Infinity]];
const binName = ([a, b]) => (a === -Infinity ? `<${b}` : b === Infinity ? `${a}+` : `${a}-${b}`);
function spread(label, xs) {
  const v = xs.filter((x) => x != null).sort((a, b) => a - b);
  if (!v.length) return `${label.padEnd(30)} none`;
  const pc = (q) => v[Math.min(v.length - 1, Math.floor(q * v.length))].toFixed(1);
  const bins = BINS.map((b) => `${binName(b)} ${v.filter((x) => x >= b[0] && x < b[1]).length}`).join(', ');
  return `${label.padEnd(30)} ${String(v.length).padStart(5)} loops; p10 ${pc(0.1)}, p25 ${pc(0.25)}, median ${pc(0.5)}, p75 ${pc(0.75)}, p90 ${pc(0.9)}; ${bins}`;
}
const down = (oct) => gaps.map((g, i) => (oct[i] < 0 ? g : null));
console.log('the gap (tune less chords, sounding notes, semitones):');
console.log(spread('  every loop', gaps));
console.log(spread('  chords moved down', down(mine)));
console.log(spread('  chords not moved', gaps.map((g, i) => (mine[i] === 0 ? g : null))));
const far = mine.filter((o, i) => o < 0 && gaps[i] != null && gaps[i] > 12.5).length;
console.log(`  moved down and more than an octave under the tune (gap over 12.5): ${far}`);
const save = opt('save', null);
if (save) fs.writeFileSync(save, JSON.stringify({ octaves: mine, gaps }));
const against = opt('against', null);
if (against) {
  const was = JSON.parse(fs.readFileSync(against, 'utf8'));
  const other = was.octaves;
  const moved = mine.map((v, i) => (v !== other[i] ? i : -1)).filter((i) => i >= 0);
  console.log(`against ${against}: ${moved.length} of ${N} loops take another octave (${moved.slice(0, 12).join(', ')}${moved.length > 12 ? ', ...' : ''})`);
  const way = (i) => Math.sign(mine[i] - other[i]);
  console.log(`  up ${moved.filter((i) => way(i) > 0).length}, down ${moved.filter((i) => way(i) < 0).length}`);
  console.log(spread('  moved, the gap before', moved.map((i) => was.gaps[i])));
  console.log(spread('  moved, the gap after', moved.map((i) => gaps[i])));
  console.log(spread('  every loop, the gap before', was.gaps));
  console.log(spread('  every loop, the gap after', gaps));
  if (args.includes('--list')) for (const i of moved) console.log(`  #${i} octave ${other[i]} to ${mine[i]}, gap ${was.gaps[i]?.toFixed(1)} to ${gaps[i]?.toFixed(1)}`);
}
process.exit(bad ? 1 : 0);
