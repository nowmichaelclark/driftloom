// Building a word album (queue item 35b).
//
// Run with:
//   node tools/wordalbum.mjs --word cozy --target "warmth>0.7,lift 0.55-0.8,energy<0.5"
//   node tools/wordalbum.mjs --word cozy --fit <export.jsonl>
//
// One word, about eight loops: six guesses, as close to the word's target as
// the corpus has, and two decoys far from it on its main dials, shuffled. The
// brain sends it to Mikey as "Cozy?" and he says which tracks don't fit
// (item 37 on /rank, or as a plain album). The target is either the word's
// fitted region (item 35a, from an export) or dial ranges the brain writes
// itself where the data is thin. --seed makes it reproducible, so the key
// (which track is which) is re-derived by running the same line again and
// never needs writing down.

import { newSpec, render, patternSteps } from '../js/generator.js';
import { encodeAlbum } from '../js/share.js';
import { SCALES } from '../js/theory.js';
import { CHARACTERS } from '../js/characters.js';
import { Rng } from '../js/rng.js';
import { DIALS, measureSpec, scaled, loadScale } from './dials.mjs';
import { observations, fitWords } from './fit.mjs';

const USAGE = `
driftloom word albums (queue item 35b)

  node tools/wordalbum.mjs --word <w> (--target "<ranges>" | --fit <export.jsonl>) [options]

  --target <ranges>   dial ranges, comma-separated: "warmth>0.7", "energy<0.5",
                      "lift 0.55-0.8"; the first is the main dial
  --fit <file>        the word's fitted region (item 35a) from a /rank export
  --guesses <n>       loops nearest the target                      (6)
  --decoys <n>        loops far from it on its main dials            (2)
  --n <count>         corpus to draw from                           (3000)
  --seed <number>     corpus seed; same seed, same album               (1)
  --seconds <s>       about how long each track plays                 (40)
  --help              this
`;

// "warmth>0.7,lift 0.55-0.8,energy<0.5" -> [{dial, lo, hi}], in order.
export function parseTarget(text) {
  return String(text).split(',').map((part) => part.trim()).filter(Boolean).map((part) => {
    let m = part.match(/^([a-z]+)\s*(>=?|<=?)\s*([0-9.]+)$/);
    if (m) {
      const v = Number(m[3]);
      return m[2].startsWith('>') ? { dial: m[1], lo: v, hi: 1 } : { dial: m[1], lo: 0, hi: v };
    }
    m = part.match(/^([a-z]+)\s+([0-9.]+)\s*-\s*([0-9.]+)$/);
    if (m) return { dial: m[1], lo: Number(m[2]), hi: Number(m[3]) };
    throw new Error(`cannot read the target "${part}"`);
  }).map((t) => {
    if (!DIALS.includes(t.dial)) throw new Error(`no dial called ${t.dial}`);
    return t;
  });
}

// A fitted region as ranges: center +/- spread on each of its dials.
export function regionTarget(region) {
  return region.map((r) => ({ dial: r.dial, lo: Math.max(0, r.center - r.spread), hi: Math.min(1, r.center + r.spread) }));
}

// How far a loop is from the target: on each dial, how far outside its
// range (0 inside), the main dial counting double.
export function distance(d, target) {
  let s = 0;
  target.forEach((t, i) => {
    const x = d[t.dial];
    const out = x < t.lo ? t.lo - x : x > t.hi ? x - t.hi : 0;
    s += (i === 0 ? 2 : 1) * out * out;
  });
  return Math.sqrt(s);
}

// How far a loop is from the target's middle on its main dials (the first
// two), for the decoys: far from what the word is about.
function farness(d, target) {
  const main = target.slice(0, 2);
  return Math.sqrt(main.reduce((a, t) => a + (d[t.dial] - (t.lo + t.hi) / 2) ** 2, 0));
}

const lead = (spec) => Object.entries(spec.mix || {}).sort((a, b) => b[1] - a[1])[0][0];

export function buildAlbum({ word, target, guesses = 6, decoys = 2, n = 3000, seed = 1, seconds = 40, scale = loadScale() }) {
  const master = new Rng(seed);
  const pool = [];
  for (let i = 0; i < n; i++) {
    const spec = newSpec(master.seed32());
    // Four bars or longer: a two-bar loop is over before it says anything.
    // And one pass no longer than half again the track: a track is whole
    // passes, so a three-minute loop cannot be played for forty seconds.
    if (spec.bars < 4) continue;
    if (patternSteps(spec) * (60 / spec.bpm / 4) > seconds * 1.5) continue;
    const d = scaled(measureSpec(spec, scale).raw, scale);
    pool.push({ spec, d, dist: distance(d, target), far: farness(d, target) });
  }
  // Each from a different profile where the corpus allows: take the nearest
  // of each profile first, then fill from the rest.
  const pick = (list, count, key) => {
    const sorted = list.slice().sort((a, b) => a[key] - b[key]);
    const out = [];
    const seen = new Set();
    for (const c of sorted) if (out.length < count && !seen.has(lead(c.spec))) { out.push(c); seen.add(lead(c.spec)); }
    for (const c of sorted) if (out.length < count && !out.includes(c)) out.push(c);
    return out;
  };
  const near = pick(pool, guesses, 'dist').map((c) => ({ ...c, role: 'guess' }));
  const rest = pool.filter((c) => !near.some((g) => g.spec === c.spec) && c.dist > 0);
  const far = pick(rest.map((c) => ({ ...c, negFar: -c.far })), decoys, 'negFar').map((c) => ({ ...c, role: 'decoy' }));
  const tracks = new Rng(((seed ^ 0x41b) >>> 0) || 1).shuffle([...near, ...far]);
  for (const t of tracks) {
    // About `seconds` a track: whole passes of the loop as composed.
    const loopSeconds = patternSteps(t.spec) * (60 / t.spec.bpm / 4);
    t.spec.playFor = Math.max(1, Math.round(seconds / loopSeconds));
  }
  const title = `${word[0].toUpperCase()}${word.slice(1)}?`;
  const code = encodeAlbum(title, tracks.map((t) => t.spec));
  return { title, code, tracks, target };
}

function describe(album) {
  const out = [''];
  out.push(`${album.title}  (${album.tracks.length} tracks)`);
  out.push('');
  out.push(album.code);
  out.push('');
  out.push(`  /rank: rank.html?album=${album.code}`);
  out.push('');
  out.push(`  target: ${album.target.map((t) => `${t.dial} ${t.lo.toFixed(2)}-${t.hi.toFixed(2)}`).join(', ')}`);
  out.push('  ' + ['track', 'role', 'profile', 'mode', 'bpm', 'secs', ...album.target.map((t) => t.dial)].map((h, i) => (i < 3 ? h.padEnd(i === 2 ? 11 : 7) : i === 3 ? h.padEnd(18) : h.padStart(9))).join(''));
  album.tracks.forEach((t, i) => {
    const secs = Math.round(t.spec.playFor * patternSteps(t.spec) * (60 / t.spec.bpm / 4));
    out.push('  ' + [String(i + 1).padEnd(7), t.role.padEnd(7), (CHARACTERS[lead(t.spec)]?.label || lead(t.spec)).padEnd(11),
      (SCALES[t.spec.scale]?.label || t.spec.scale).padEnd(18), String(t.spec.bpm).padStart(9), String(secs).padStart(9),
      ...album.target.map((tg) => t.d[tg.dial].toFixed(2).padStart(9))].join(''));
  });
  out.push('');
  return out.join('\n');
}

async function main(argv) {
  const opts = { word: null, target: null, fit: null, guesses: 6, decoys: 2, n: 3000, seed: 1, seconds: 40 };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    const v = () => argv[++i];
    if (a === '--word') opts.word = v();
    else if (a === '--target') opts.target = v();
    else if (a === '--fit') opts.fit = v();
    else if (a === '--guesses') opts.guesses = Number(v());
    else if (a === '--decoys') opts.decoys = Number(v());
    else if (a === '--n') opts.n = Number(v());
    else if (a === '--seed') opts.seed = Number(v()) >>> 0;
    else if (a === '--seconds') opts.seconds = Number(v());
    else if (a === '--help' || a === '-h') { console.log(USAGE); return 0; }
    else { console.error(`wordalbum: unknown option ${a}\n${USAGE}`); return 2; }
  }
  if (!opts.word || !(opts.target || opts.fit)) { console.log(USAGE); return 2; }
  let target;
  if (opts.target) target = parseTarget(opts.target);
  else {
    const fs = await import('node:fs');
    const { dialsOf } = await import('./dials.mjs');
    const lines = fs.readFileSync(opts.fit, 'utf8').split('\n').filter((l) => l.trim()).map((l) => JSON.parse(l));
    const scale = loadScale();
    const memo = new Map();
    const model = fitWords(observations(lines), (c) => {
      if (!memo.has(c)) memo.set(c, dialsOf(c, scale).scaled);
      return memo.get(c);
    });
    const m = model[opts.word];
    if (!m || !m.region.length) {
      console.error(`wordalbum: ${opts.word} has no fitted region (${m ? m.status === 'ok' ? 'no dial clears' : m.status : 'never shown'}); give --target instead`);
      return 1;
    }
    target = regionTarget(m.region);
  }
  console.log(describe(buildAlbum({ ...opts, target })));
  return 0;
}

const isMain = process.argv[1] && import.meta.url === new URL(`file://${process.argv[1]}`).href;
if (isMain) process.exitCode = await main(process.argv.slice(2));
