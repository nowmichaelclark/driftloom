// The core baseline: what the Rust core plays today, kept in the repo, and a
// check that fails on any difference (queue item 32).
//
//   node tools/baseline.mjs            render, and compare with test/core-baseline.json
//   node tools/baseline.mjs --update   render, and rewrite it
//   node tools/baseline.mjs --only loops,stereo,player,voices,refusals,costs   some parts
//
// The core is deterministic (its renders come out the same bit for bit, run
// after run, on every target), so a fingerprint that moves says that a sound
// moved. A PR that changes the sound on purpose runs --update in the same
// PR, and says what moved and why; one that does not change it must leave
// this check passing.
//
// Since queue item 96 every part is the player's: loops from their seeds on
// js/dlplayer.wasm, the module the app plays, in Node (tools/measure.mjs,
// tools/play-lib.mjs), so one way of playing is checked and no browser is
// needed. Until then the older parts were rendered in headless Chromium by
// the page's engine handing its notes to the core up front; for the same
// loops the two agree to 0.01 dB in every figure (item 96's PR has the
// table), and differ in their last bits.
//
// What it keeps:
//
//   loops     40 loops of the corpus (seed 1), full and lite quality, each
//             played 45 s from its seed, wandering as the app's does
//             (amount 1): a fingerprint of the output as it reaches the
//             speakers and of each layer as it goes into the mix, with the
//             integrated loudness, the peak and each layer's loudness beside
//             them to read what a change did
//   stereo    the same, in the core's stereo render (queue item 64), each
//             against its mono render: a fingerprint of each side, whether
//             each side is mono's bit for bit, the fold ((L + R) / 2)
//             against mono (loudness, and the residual in dB under it), and
//             the side against the mid
//   player    the same renders' output fingerprints alone, the four ways
//             (full and lite, mono and stereo), as tools/seeds.sh --browser
//             reads them: the player natively, on every target, is held to
//             them
//   voices    every voice in every layer, probed as `measure.mjs --voice
//             all` does (K-weighted level at 0.4 s notes, velocities 0.4
//             and 0.8, and for melody voices the 2-5 kHz share, the coffee
//             shop test): the level per voice, the median of each layer, and
//             a fingerprint of the notes
//   refusals  what the voice budget turns away, per layer, over 100 loops,
//             full and lite, each played a pass and 3 s more (60 s at most),
//             wandering as the app's does
//   costs     VOICE_COST as data/voices.txt states it, and the budgets
//             (core/src/player/budget.rs)
//
// The fingerprints are exact. The figures beside them are rounded
// (loudness and level to 0.01 dB, the tone share to 0.1 point) so a diff is
// readable.
//
// Needs the Rust toolchain (the voices and the refusals play the tools'
// build of the player: tools/build-core.sh --tools, made on first use).

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { Rng } from '../js/rng.js';
import { made, playLoop, spaceOf, channelGains, measureVoices, measureRefusals } from './measure.mjs';
import { LAYERS } from './play-lib.mjs';

const data = await import('../js/data.js');

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const FILE = path.join(ROOT, 'test', 'core-baseline.json');

const argv = process.argv.slice(2);
if (argv.includes('--help') || argv.includes('-h')) {
  console.log(fs.readFileSync(fileURLToPath(import.meta.url), 'utf8').split('\n').filter((l) => l.startsWith('//')).map((l) => l.slice(3)).join('\n'));
  process.exit(0);
}
const update = argv.includes('--update');
const onlyAt = argv.indexOf('--only');
const PARTS = ['loops', 'stereo', 'player', 'voices', 'refusals', 'costs'];
const only = onlyAt >= 0 ? argv[onlyAt + 1].split(',') : PARTS;
for (const part of only) if (!PARTS.includes(part)) { console.error(`baseline: no part called '${part}'`); process.exit(2); }

const LOOPS = 40;
const SEED = 1;
const SECONDS = 45;
const WANDER = 1;
const RATE = 44100;
const REFUSAL_LOOPS = 100;

const r2 = (x) => (Number.isFinite(x) ? Math.round(x * 100) / 100 : null);
const dbOf = (x) => (x > 0 ? r2(20 * Math.log10(x)) : null);
const median = (xs) => {
  const s = xs.filter(Number.isFinite).sort((a, b) => a - b);
  return s.length ? s[Math.floor((s.length - 1) / 2)] : null;
};
// FNV-1a over a string, for a fingerprint of figures.
function fnv(text) {
  let h = 0x811c9dc5;
  for (let i = 0; i < text.length; i++) { h ^= text.charCodeAt(i); h = Math.imul(h, 0x01000193); }
  return (h >>> 0).toString(16).padStart(8, '0');
}

// ------------------------------------------- loops, stereo and the player

// The 40 loops from their seeds, each played once in mono and (for the
// stereo and player parts) once in stereo: the three parts read the same
// renders. By name, so a diff says which loop moved (the seed breaks a tie).
function renders(quality, stereoToo) {
  const gains = channelGains();
  const way = { rate: RATE, full: quality === 'full', wander: WANDER };
  const opts = { seconds: SECONDS, rate: RATE };
  const master = new Rng(SEED);
  const parts = { loops: {}, stereo: {}, player: {} };
  for (let i = 0; i < LOOPS; i++) {
    const seed = master.seed32();
    const mono = playLoop(made(seed, { ...way, layout: 1 }), opts, gains);
    const name = parts.loops[mono.name] ? `${mono.name}#${seed}` : mono.name;
    parts.loops[name] = {
      seed,
      profile: mono.profile,
      seconds: mono.seconds,
      out: mono.hashes.out,
      layers: mono.hashes.layers,
      lufs: r2(mono.lufs),
      peakDb: dbOf(mono.peak),
      layerLufs: Object.fromEntries(LAYERS.map((l) => [l, r2(mono.layers[l].lufs ?? NaN)])),
    };
    parts.player[name] = { seed, out: mono.hashes.out };
    if (!stereoToo) continue;
    const wide = playLoop(made(seed, { ...way, layout: 2 }), opts, gains);
    const space = spaceOf(wide.out, mono.out[0], RATE);
    parts.stereo[name] = {
      left: wide.hashes.left,
      right: wide.hashes.right,
      monoOnEachSide: space.exact,
      foldLU: r2(space.foldLU),
      foldResidDb: r2(space.foldResid),
      sideMidDb: r2(space.sideMid),
    };
    Object.assign(parts.player[name], { left: wide.hashes.left, right: wide.hashes.right });
  }
  return parts;
}

// ---------------------------------------------------------------- voices

function voices() {
  // Every third note of each window, one render of each: the core renders
  // the same notes the same way every time, and a change to a voice moves
  // many of them.
  const probed = measureVoices({ voice: ['all'], note: 0.4, reps: 1, stride: 3, rate: RATE });
  const rows = {};
  const layers = {};
  for (const row of probed.rows) {
    const key = `${row.layer}/${row.voice}`;
    const at = (vel) => (row.notes[vel] || []).map((n) => r2(n.lufs));
    const lo = at(0.4);
    const hi = at(0.8);
    const tone = row.tone ? row.tone.map((t) => Math.round(t.share * 1000) / 10) : null;
    rows[key] = {
      level04: r2(median(lo)),
      level08: r2(median(hi)),
      tone: tone ? Math.round(10 * tone.reduce((a, b) => a + b, 0) / tone.length) / 10 : null,
      print: fnv(JSON.stringify([lo, hi, tone])),
    };
    (layers[row.layer] ||= []).push(median(lo));
  }
  return {
    window: 'every 3rd note of each layer\'s window, 0.4 s notes, one render each',
    layerMedian04: Object.fromEntries(Object.entries(layers).map(([k, v]) => [k, r2(median(v))])),
    rows,
  };
}

// -------------------------------------------------------------- refusals

function refusals() {
  const out = {};
  for (const quality of ['full', 'lite']) {
    const rows = measureRefusals({ n: REFUSAL_LOOPS, seed: SEED, seconds: 'pass', wander: WANDER, rate: RATE, quality, request: '', profile: null, moods: null });
    const sum = (key) => Object.fromEntries(LAYERS.map((l) => [l, rows.reduce((a, r) => a + r[key][l], 0)]));
    const melody = rows.map((r) => (r.asked.melody ? r.refused.melody / r.asked.melody : null)).filter((x) => x != null);
    out[quality] = {
      loops: rows.length,
      asked: sum('asked'),
      refused: sum('refused'),
      letGo: sum('stolen'),
      meanMelodyRefusal: Math.round(1e4 * melody.reduce((a, b) => a + b, 0) / Math.max(1, melody.length)) / 1e4,
      loopsLosingMelody: melody.filter((x) => x > 0.001).length,
    };
  }
  return out;
}

// ----------------------------------------------------------------- costs

function costs() {
  const source = fs.readFileSync(path.join(ROOT, 'core', 'src', 'player', 'budget.rs'), 'utf8');
  // The voice table is data (queue item 78): data/voices.txt.
  const { blockOf, readBlock } = data;
  const table = readBlock(blockOf('voices', 'voices', 'costs'), { default: 'num', dubstab: 'num', cost: 'table' }, 'data/voices.txt').cost;
  const budgets = {};
  for (const name of ['MAX_BUDGET', 'LITE_BUDGET']) {
    const m = source.match(new RegExp(`pub const ${name}: f64 = (\\d+)`));
    budgets[name] = m ? Number(m[1]) : null;
  }
  return { budgets, voices: table };
}

// ------------------------------------------------------------------ run

const now = {};
const log = (m) => console.error(`baseline: ${m}`);
if (only.some((p) => ['loops', 'stereo', 'player'].includes(p))) {
  const stereoToo = only.includes('stereo') || only.includes('player');
  const by = { loops: {}, stereo: {}, player: { seconds: SECONDS, wander: WANDER } };
  for (const quality of ['full', 'lite']) {
    log(`${LOOPS} loops from their seeds, ${quality}${stereoToo ? ', mono and stereo' : ''}`);
    const r = renders(quality, stereoToo);
    for (const part of ['loops', 'stereo', 'player']) by[part][quality] = r[part];
  }
  for (const part of ['loops', 'stereo', 'player']) if (only.includes(part)) now[part] = by[part];
}
if (only.includes('voices')) { log('every voice'); now.voices = voices(); }
if (only.includes('refusals')) { log(`refusals over ${REFUSAL_LOOPS} loops, full and lite`); now.refusals = refusals(); }
if (only.includes('costs')) now.costs = costs();

// ------------------------------------------------------------ comparing

// Every leaf of a value, as 'a.b[2].c' -> value.
function leaves(value, prefix = '', out = new Map()) {
  if (value && typeof value === 'object') {
    for (const [k, v] of Object.entries(value)) leaves(v, Array.isArray(value) ? `${prefix}[${k}]` : prefix ? `${prefix}.${k}` : k, out);
  } else {
    out.set(prefix, value);
  }
  return out;
}

if (update) {
  // A part not rendered this time keeps what was there; a part the
  // baseline no longer has goes.
  const kept = fs.existsSync(FILE) ? JSON.parse(fs.readFileSync(FILE, 'utf8')) : {};
  const next = { about: 'The Rust core\'s render fingerprints and measured figures, on the player (tools/baseline.mjs). Rewritten by a PR that changes the sound on purpose; checked by every other.' };
  for (const part of PARTS) if (now[part] ?? kept[part]) next[part] = now[part] ?? kept[part];
  fs.writeFileSync(FILE, `${JSON.stringify(next, null, 1)}\n`);
  console.log(`baseline: wrote ${path.relative(ROOT, FILE)} (${only.join(', ')})`);
  process.exit(0);
}

if (!fs.existsSync(FILE)) {
  console.error('baseline: test/core-baseline.json does not exist; run  node tools/baseline.mjs --update');
  process.exit(1);
}
const kept = JSON.parse(fs.readFileSync(FILE, 'utf8'));
const lines = [];
let moved = 0;
for (const part of only) {
  const was = leaves(kept[part] ?? null);
  const is = leaves(now[part]);
  const keys = [...new Set([...was.keys(), ...is.keys()])];
  const diffs = keys.filter((k) => was.get(k) !== is.get(k));
  moved += diffs.length;
  if (diffs.length) {
    lines.push(`  ${part}: ${diffs.length} figure(s) moved`);
    // Fingerprints first, then the figures that explain them.
    for (const k of diffs.slice(0, 60)) lines.push(`    ${k}: ${was.has(k) ? JSON.stringify(was.get(k)) : '(none)'} -> ${is.has(k) ? JSON.stringify(is.get(k)) : '(gone)'}`);
    if (diffs.length > 60) lines.push(`    ... and ${diffs.length - 60} more`);
  } else {
    lines.push(`  ${part}: unchanged (${keys.length} figures)`);
  }
}
console.log(`\nThe core against its baseline (test/core-baseline.json)\n${lines.join('\n')}\n`);
if (moved) {
  console.log('The sound moved. If that is the point of this change, run  node tools/baseline.mjs --update\nin the same PR, and say in it what moved and why.\n');
  process.exit(1);
}
console.log('Nothing moved.\n');
