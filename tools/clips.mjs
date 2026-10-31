// Listening clips from the core's own player, with the levers a sketch or an
// A/B needs (queue item 83).
//
//   node tools/clips.mjs JOBS.json OUTDIR [--opus] [--bin PATH] [--no-build]
//
// Each job is rendered by `dlrender --seed`/`--corpus` (core/examples/dlrender)
// built with the `sketch` feature: the core's generator and player, natively,
// no JavaScript and no Chromium. That is the better base for a clip, since it
// is what phase 3 and the game play. Output per job: OUTDIR/NAME.wav
// (32-bit float, as dlrender writes it), NAME.opus with --opus (ffmpeg, 96
// kbps), and OUTDIR/index.json and index.md: per clip the loop's name and
// seed, how it was rendered, its hash, and what it drew among its choices.
//
// A job (everything but name, and one of seed and corpus, is optional):
//
//   { "name": "late-undertow45-forced",
//     "corpus": [20261009, 45],      // corpus seed and index (Rng(seed).seed32()
//                                    // a loop; tools/ears/ scan.mjs's corpus)
//     "seed": 982035031,             // or a loop's own seed
//     "seconds": 42,                 // length written, from the loop's start
//     "from": 0,                     // the clip starts here (rendered from 0)
//     "layout": 2,                   // 2 stereo, 1 mono (default 2)
//     "wander": 1,                   // "Let the loop wander" amount; 0 off (default 1)
//     "vibes": ["peaceful"],         // vibe ids, as the app's panel picks them
//     "request": "bpm=180",          // a request as text (item 85: dlrender --request)
//     "lite": false,                 // the lite mix (a phone's)
//     "force": { "lateFloor": "on" },        // choices forced ("today": none)
//     "set": { "bass.LIFT_DB": 3 },          // constants moved (core/src/tuning.rs)
//     "pin": { "sub_duck.depth": 0.5 },      // a channel block's setting pinned
//     "sketch": true }                       // core/sketch/block.rs on, where.txt says where
//
// Forced choices, constants and pins are the sketch build's: "force" moves
// onto the request once item 85 lands. A clip with none of them is the
// player's plain render, bit for bit run to run (and the same bits as the
// build without the feature: the sketch slot is off unless a job turns it
// on). The index lists the options a loop drew other than today's; a forced
// one shows up there.

import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const CORE = path.join(ROOT, 'core');

const args = process.argv.slice(2);
const flag = (name) => {
  const i = args.indexOf(name);
  if (i < 0) return false;
  args.splice(i, 1);
  return true;
};
const valueOf = (name) => {
  const i = args.indexOf(name);
  if (i < 0) return null;
  const [, v] = args.splice(i, 2);
  return v;
};
const opus = flag('--opus');
const noBuild = flag('--no-build');
const binArg = valueOf('--bin');
const [JOBS, OUT] = args;
if (!JOBS || !OUT) {
  console.error('usage: node tools/clips.mjs JOBS.json OUTDIR [--opus] [--bin PATH] [--no-build]');
  process.exit(2);
}

const run = (cmd, argv, opts = {}) => {
  const r = spawnSync(cmd, argv, { encoding: 'utf8', maxBuffer: 1 << 28, ...opts });
  if (r.status !== 0) {
    throw new Error(`${cmd} ${argv.join(' ')}\n${r.stderr || r.stdout || r.error}`);
  }
  return r.stdout;
};

let bin = binArg;
if (!bin) {
  if (!noBuild) {
    run('cargo', ['build', '--release', '--locked', '--features', 'sketch', '--example', 'dlrender'], { cwd: CORE, stdio: ['ignore', 'ignore', 'inherit'] });
  }
  bin = path.join(CORE, 'target', 'release', 'examples', 'dlrender');
}

const jobs = JSON.parse(fs.readFileSync(JOBS, 'utf8'));
fs.mkdirSync(OUT, { recursive: true });

const flagsOf = (j) => {
  const a = [];
  if (j.corpus) {
    const [s, i] = j.corpus;
    a.push('--corpus', `${s}:${i + 1}`, '--pick', String(i));
  } else if (j.seed !== undefined) {
    a.push('--seed', String(j.seed));
  } else {
    throw new Error(`${j.name}: a job needs "seed" or "corpus"`);
  }
  a.push('--seconds', String(j.seconds ?? 30), '--wander', String(j.wander ?? 1), '--choices');
  if ((j.layout ?? 2) === 2) a.push('--stereo');
  if (j.lite) a.push('--lite');
  if (j.from) a.push('--from', String(j.from));
  if (j.vibes?.length) a.push('--vibes', j.vibes.join(','));
  if (j.request) a.push('--request', j.request);
  for (const [id, option] of Object.entries(j.force ?? {})) a.push('--force', `${id}=${option}`);
  for (const [name, v] of Object.entries(j.set ?? {})) a.push('--set', `${name}=${v}`);
  for (const [name, v] of Object.entries(j.pin ?? {})) a.push('--pin', `${name}=${v}`);
  if (j.sketch) a.push('--sketch');
  return a;
};

const index = [];
for (const j of jobs) {
  if (!j.name || /[^\w.-]/.test(j.name)) throw new Error(`a job's name is letters, digits, _ . -: ${JSON.stringify(j.name)}`);
  const wav = path.join(OUT, `${j.name}.wav`);
  // dlrender writes a corpus loop into a directory (I-NAME.wav) and a seed to
  // the file named: both go through a directory of their own, then to NAME.wav.
  const tmp = fs.mkdtempSync(path.join(OUT, '.clip-'));
  const line = run(bin, [...flagsOf(j), '--wav', j.corpus ? tmp : path.join(tmp, 'clip.wav')]).trim();
  const [made] = fs.readdirSync(tmp).filter((f) => f.endsWith('.wav'));
  fs.renameSync(path.join(tmp, made), wav);
  fs.rmSync(tmp, { recursive: true });
  const get = (k) => line.match(new RegExp(`(?:^| )${k}=(\\S+)`))?.[1] ?? null;
  const row = {
    name: j.name,
    loop: get('name'),
    seed: Number(get('seed')),
    corpus: j.corpus ?? null,
    seconds: j.seconds ?? 30,
    from: j.from ?? 0,
    layout: (j.layout ?? 2) === 2 ? 'stereo' : 'mono',
    quality: j.lite ? 'lite' : 'full',
    wander: j.wander ?? 1,
    choices: (get('choices') ?? '-') === '-' ? [] : get('choices').split(','),
    forced: j.force ?? {},
    set: j.set ?? {},
    pin: j.pin ?? {},
    sketch: Boolean(j.sketch),
    hash: get('hash'),
  };
  if (opus) {
    const out = path.join(OUT, `${j.name}.opus`);
    run('ffmpeg', ['-y', '-loglevel', 'error', '-i', wav, '-c:a', 'libopus', '-b:a', '96k', out]);
    row.opus = path.basename(out);
  }
  index.push(row);
  console.log(`${j.name}: ${row.loop} ${row.choices.join(',') || '-'}`);
}

fs.writeFileSync(path.join(OUT, 'index.json'), `${JSON.stringify(index, null, 2)}\n`);
const md = ['| clip | loop | seed | drew | levers |', '|---|---|---|---|---|'];
for (const r of index) {
  const levers = [
    ...Object.entries(r.forced).map(([k, v]) => `force ${k}=${v}`),
    ...Object.entries(r.set).map(([k, v]) => `set ${k}=${v}`),
    ...Object.entries(r.pin).map(([k, v]) => `pin ${k}=${v}`),
    ...(r.sketch ? ['sketch'] : []),
  ].join(', ') || '-';
  const where = r.corpus ? ` (corpus ${r.corpus[0]} #${r.corpus[1]})` : '';
  md.push(`| ${r.name} | ${r.loop}${where} | ${r.seed} | ${r.choices.join(', ') || '-'} | ${levers} |`);
}
fs.writeFileSync(path.join(OUT, 'index.md'), `${md.join('\n')}\n`);
