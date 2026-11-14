#!/usr/bin/env node
// No join a hole, on any piece (queue item 102g, session 16b): the wide run
// of session 16's yardstick (tools/longform-played.mjs, `joinDepths`). Every
// join of many pieces, as played, against the loop as itself at the same
// moment: Endless arcs (their first 30 minutes) and 1,024-bar arcs (whole),
// rendered by `dlrender` at the wander's amount 0 (the plan with nothing
// breathing, so the two renders play the same notes but for the plan's). A
// hole is session 16's (a join whose median 50 ms frame is HOLE_DB under the
// loop there, made by the join, not the unit's own quiet before it) that
// stays that far under for more than half a beat: a half-beat breath (a
// four-bar unit's, the shortest there is) is the gesture Mikey kept in
// moan-teil's look-back (docs/notes/102-mikey.md), and half a beat is what
// brain B3 held session 05's three places to. Those are listed, not held.
// tools/longform.mjs plays a small set of its own (the three places session
// 05 found among them); this is the nightly's (.github/workflows/nightly.yml).
//
//   node tools/longform-holes.mjs                 140 Endless arcs and 60 of
//                                                 1,024 bars: about 90
//                                                 minutes on four cores
//   node tools/longform-holes.mjs --endless 20 --long 4    fewer
//   node tools/longform-holes.mjs --from 100      the corpus's seeds from the
//                                                 hundredth (0)
//   node tools/longform-holes.mjs --shard 2/4     every fourth seed, from the
//                                                 second (the nightly's jobs)
//   node tools/longform-holes.mjs --only 51,109   those corpus seeds alone
//   node tools/longform-holes.mjs --jobs 4        pieces played at once (the
//                                                 cores)
//   node tools/longform-holes.mjs --minutes 30    how long an Endless arc plays
//   node tools/longform-holes.mjs --json FILE     every join as a JSON line
//   node tools/longform-holes.mjs --check         exit 1 on a hole
//
// The corpus is the corpus seed's (20261009) stream, as tools/longform.mjs
// draws it: ta-muin is its #121 and yi-yoar its #51.

import { execFile } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { pieceMap } from './gen-lib.mjs';
import { HOLE_DB, dlrender, joinDepths, readWav } from './longform-played.mjs';
import { Rng } from '../js/rng.js';

const args = process.argv.slice(2);
const flag = (name) => args.includes(name);
const value = (name, fallback) => (args.includes(name) ? args[args.indexOf(name) + 1] : fallback);
const SELF = fileURLToPath(import.meta.url);
const RATE = 44100;
const ENDLESS = 'bars=endless,journey=arc';
const LONG = 'bars=1024,journey=arc';

/** One seed's pieces, played (a worker): each request's joins against the loop as itself, one render of the loop for all. */
function playSeed(seed, requests, minutes) {
  const tmp = fs.mkdtempSync(path.join(os.tmpdir(), `holes-${seed}-`));
  const one = (request, seconds, file) => {
    dlrender(['--seed', String(seed), ...(request ? ['--request', request] : []), '--seconds', String(seconds), '--rate', String(RATE), '--wander', '0', '--wav', file]);
    return readWav(file).data;
  };
  const plans = requests.map((request) => {
    const { form } = pieceMap(seed, request, 0, 1);
    const sectionSeconds = form.sectionBars / form.unitBars * form.unitSeconds;
    const sections = form.sections ?? Math.ceil(minutes * 60 / sectionSeconds);
    return { request, sections, seconds: Math.ceil(sections * sectionSeconds + 2) };
  });
  const out = [];
  try {
    const loop = one(null, Math.max(...plans.map((p) => p.seconds)), path.join(tmp, 'loop.wav'));
    for (const p of plans) {
      const piece = one(p.request, p.seconds, path.join(tmp, 'piece.wav'));
      for (const d of joinDepths(seed, p.request, p.sections, 0.05, piece, loop, RATE)) out.push({ seed, request: p.request, ...d });
    }
  } finally {
    fs.rmSync(tmp, { recursive: true, force: true });
  }
  return out;
}

if (flag('--worker')) {
  const i = args.indexOf('--worker');
  const seed = Number(args[i + 1]);
  const requests = args[i + 2].split(';');
  process.stdout.write(JSON.stringify(playSeed(seed, requests, Number(value('--minutes', 30)))));
  process.exit(0);
}

const ENDLESS_N = Number(value('--endless', 140));
const LONG_N = Number(value('--long', 60));
const FROM = Number(value('--from', 0));
const [shard, shards] = (value('--shard', '1/1')).split('/').map(Number);
const JOBS = Number(value('--jobs', os.availableParallelism?.() ?? 4));
const MINUTES = Number(value('--minutes', 30));
const json = value('--json', null);

const rng = new Rng(20261009);
const corpus = Array.from({ length: FROM + Math.max(ENDLESS_N, LONG_N) }, () => Math.floor(rng.f() * 4294967296));
const ONLY = value('--only', null)?.split(',').map(Number);
const work = [];
for (let k = FROM; k < corpus.length; k++) {
  if ((k - FROM) % shards !== shard - 1 || (ONLY && !ONLY.includes(k))) continue;
  const requests = [...(k - FROM < ENDLESS_N ? [ENDLESS] : []), ...(k - FROM < LONG_N ? [LONG] : [])];
  if (requests.length) work.push({ k, seed: corpus[k], requests });
}

// The release dlrender, built once before the workers share it.
dlrender(['--help']);
process.env.LONGFORM_BUILD = 'no';

const t0 = Date.now();
const joins = [];
let done = 0;
let pieces = 0;
const run = (w) => new Promise((resolve, reject) => {
  execFile(process.execPath, [SELF, '--worker', String(w.seed), w.requests.join(';'), '--minutes', String(MINUTES)], { maxBuffer: 1 << 28, env: process.env }, (err, stdout) => {
    if (err) return reject(new Error(`${w.seed}: ${err.message}`));
    for (const j of JSON.parse(stdout)) joins.push({ k: w.k, ...j });
    done += 1;
    pieces += w.requests.length;
    if (!flag('--quiet')) process.stderr.write(`\r  ${done} of ${work.length} seeds, ${((Date.now() - t0) / 60000).toFixed(1)} min`);
    resolve();
  });
});
const queue = [...work];
await Promise.all(Array.from({ length: Math.min(JOBS, queue.length) }, async () => {
  while (queue.length) await run(queue.shift());
}));
if (!flag('--quiet')) process.stderr.write('\n');
joins.sort((a, b) => a.k - b.k || a.request.localeCompare(b.request) || a.t - b.t);
if (json) fs.writeFileSync(json, joins.map((j) => JSON.stringify(j)).join('\n') + '\n');

const med = (xs) => [...xs].sort((a, b) => a - b)[Math.floor(xs.length / 2)];
// (Half a beat at most HOLE_DB under: a breath's, listed apart.)
const holes = joins.filter((j) => j.hole && j.under > 0.5 + 1e-6);
const brief = joins.filter((j) => j.hole && !(j.under > 0.5 + 1e-6));
const kinds = [...new Set(joins.map((j) => j.join))].sort();
const at = (j) => `${j.seed} (#${j.k}) "${j.request}" at ${j.t.toFixed(1)} s, section ${j.section}.${j.phrase}: a ${j.join} ${j.frames.toFixed(1)} dB under, from ${j.before.toFixed(1)}`;
console.log(`longform-holes: ${pieces} pieces (${work.filter((w) => w.requests.includes(ENDLESS)).length} Endless arcs, ${MINUTES} minutes; ${work.filter((w) => w.requests.includes(LONG)).length} at 1,024 bars), ${joins.length} joins, in ${((Date.now() - t0) / 60000).toFixed(1)} min`);
console.log(`  dB under the loop as itself, median / deepest (count): ${kinds.map((k) => { const xs = joins.filter((j) => j.join === k).map((j) => j.frames); return `${k} ${med(xs).toFixed(1)} / ${Math.max(...xs).toFixed(1)} (${xs.length})`; }).join(', ')}`);
const deepest = [...joins].sort((a, b) => b.frames - a.frames).slice(0, 8);
console.log('  the deepest:');
for (const j of deepest) console.log(`    ${at(j)}${j.inherited ? ' (the unit\'s own quiet)' : ''}`);
if (brief.length) console.log(`  ${HOLE_DB} dB under for half a beat or less (a breath's own; listed, not held):\n${brief.map((j) => `    ${at(j)}, ${j.under.toFixed(2)} beats`).join('\n')}`);
console.log(holes.length ? `  FAIL  ${holes.length} holes (${HOLE_DB} dB under for more than half a beat, made by the join):\n${holes.map((j) => `    ${at(j)}, ${j.under.toFixed(2)} beats`).join('\n')}` : `  ok    no join a hole (${HOLE_DB} dB under the loop there for more than half a beat, made by the join)`);
if (holes.length && flag('--check')) process.exit(1);
