// Loops from their seeds on the player in WebAssembly (queue item 82):
// js/dlplayer.wasm in Node, through the exports the worklet calls with
// `?engine=player`, printing the same hash lines `dlrender --seed` prints
// natively, so the two can be held to each other (every target renders the
// same samples from a seed), and timing the blocks.
//
//   node tools/player-wasm.mjs --corpus SEED:N | --seed N[,N...]
//        [--seconds 45] [--rate 44100] [--lite] [--stereo] [--wander 1]
//        [--vibes ID,...] [--request TEXT] [--wasm FILE]
//                       one line a loop, as `dlrender --seed` prints it
//                       (--request: queue item 85's request as text,
//                       "glacial=0.7,bpm=70-85", as `dlrender --request`)
//   ... --costs         what each block costs (the load spread across blocks,
//                       as a real-time host plays it; each block's least time
//                       over three runs, or --runs N), as `dlrender --costs` measures it
//
// No browser: Node runs WebAssembly itself.

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { performance } from 'node:perf_hooks';
import { Fnv } from './golden-lib.mjs';
import { Rng } from '../js/rng.js';
import { pinNumbers } from '../js/pins.js';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const QUANTUM = 128;
const argv = process.argv.slice(2);
if (argv.includes('--help') || argv.includes('-h')) {
  console.log(fs.readFileSync(fileURLToPath(import.meta.url), 'utf8').split('\n').filter((l) => l.startsWith('//')).map((l) => l.slice(3)).join('\n'));
  process.exit(0);
}
const flag = (name, dflt) => {
  const i = argv.indexOf(name);
  return i >= 0 ? argv[i + 1] : dflt;
};
const way = {
  seconds: Number(flag('--seconds', 45)),
  rate: Number(flag('--rate', 44100)),
  full: !argv.includes('--lite'),
  layout: argv.includes('--stereo') ? 2 : 1,
  wander: Number(flag('--wander', 1)),
  vibes: flag('--vibes', '') ? flag('--vibes', '').split(',') : [],
  request: flag('--request', null),
};
const wasm = path.resolve(flag('--wasm', path.join(ROOT, 'js', 'dlplayer.wasm')));
const bytes = fs.readFileSync(wasm);
const { VIBE_IDS } = await import('../js/vibes.js');
const { parseRequest } = await import('../js/generator.js');
// The request, read as the core reads it (`Request::parse`), handed over
// call by call as the worklet hands it.
const asked = parseRequest(way.request ?? '');

const seeds = [];
if (flag('--corpus', null)) {
  const [from, n] = flag('--corpus', null).split(':').map(Number);
  const r = new Rng(from);
  for (let i = 0; i < n; i++) seeds.push([i, r.seed32()]);
}
if (flag('--seed', null)) for (const w of flag('--seed', null).split(',')) seeds.push([null, Number(w) >>> 0]);
if (!seeds.length) {
  console.error('player-wasm: --corpus SEED:N or --seed N[,N...]');
  process.exit(2);
}

// A core and the player on it, started as the worklet starts them, the loop
// asked for.
function start(seed) {
  const { exports: x } = new WebAssembly.Instance(new WebAssembly.Module(bytes), {});
  x.dl_init(way.rate);
  x.dl_quality(way.full ? 1 : 0);
  if (way.layout === 2) x.dl_layout(2);
  x.dl_play_wander(way.wander);
  x.dl_play_attach(way.full ? 1 : 0);
  for (const [id, w] of asked.vibes || []) {
    const i = VIBE_IDS.indexOf(id);
    if (!(w == null ? x.dl_play_vibe(i) : x.dl_play_weight(i, w))) throw new Error(`the core refused '${id}'`);
  }
  if (asked.tempo && !x.dl_play_ask_tempo(asked.tempo[0], asked.tempo[1])) throw new Error('the core refused the tempo');
  for (const [i, v] of pinNumbers(asked.pins)) if (!x.dl_play_pin(i, v)) throw new Error(`the core refused pin ${i}`);
  if (asked.theme != null) x.dl_play_theme(asked.theme >>> 0);
  for (const id of way.vibes) {
    const i = VIBE_IDS.indexOf(id);
    if (i < 0 || !x.dl_play_vibe(i)) throw new Error(`no vibe '${id}'`);
  }
  x.dl_play_load(seed, 0);
  return x;
}

// tools/measure.mjs's fingerprint: FNV-1a over the float samples' words.
function words32(samples) {
  const words = new Uint32Array(samples.buffer, samples.byteOffset, samples.length);
  let h = 0x811c9dc5;
  for (let i = 0; i < words.length; i++) { h ^= words[i]; h = Math.imul(h, 0x01000193); }
  return (h >>> 0).toString(16).padStart(8, '0');
}

function render(seed) {
  const x = start(seed);
  x.dl_play_ready();
  const frames = Math.ceil(way.seconds * way.rate);
  const lanes = way.layout === 2 ? [new Float32Array(frames), new Float32Array(frames)] : [new Float32Array(frames)];
  for (let frame = 0; frame < frames; frame += QUANTUM) {
    x.dl_play_process(frame);
    const used = Math.min(QUANTUM, frames - frame);
    lanes.forEach((lane, c) => lane.set(new Float32Array(x.memory.buffer, x.dl_out() + c * QUANTUM * 4, used), frame));
  }
  if (x.dl_dropped() || x.dl_late() || x.dl_missed()) throw new Error(`the core dropped ${x.dl_dropped()}, took ${x.dl_late()} late and missed ${x.dl_missed()} notes`);
  const name = new TextDecoder().decode(new Uint8Array(x.memory.buffer, x.dl_play_text(), x.dl_play_name()));
  const h = new Fnv();
  let samples = lanes[0];
  if (way.layout === 2) {
    samples = new Float32Array(frames * 2);
    for (let i = 0; i < frames; i++) { samples[2 * i] = lanes[0][i]; samples[2 * i + 1] = lanes[1][i]; }
  }
  h.samples(samples);
  const hashes = way.layout === 2 ? `left=${words32(lanes[0])} right=${words32(lanes[1])}` : `out=${words32(lanes[0])}`;
  const vibes = way.vibes.length ? ` vibes=${way.vibes.join(',')}` : '';
  const request = way.request != null ? ` request=${way.request}` : '';
  return `seed=${seed} name=${name} quality=${way.full ? 'full' : 'lite'} layout=${way.layout === 2 ? 'stereo' : 'mono'} seconds=${way.seconds} rate=${way.rate} wander=${way.wander}${vibes}${request} frames=${frames} ${hashes} hash=${h.hex()}`;
}

// Each block's time (the player's decide and the core's block together, as
// the worklet calls them), its least over `--runs` runs, and what the player was
// doing: making the loop (1, 2) or playing it (3).
function costs(seed) {
  const blocks = Math.ceil((way.seconds * way.rate) / QUANTUM);
  const best = new Float64Array(blocks).fill(Infinity);
  const states = new Uint8Array(blocks);
  for (let run = 0; run < RUNS; run++) {
    const x = start(seed);
    for (let b = 0; b < blocks; b++) {
      states[b] = x.dl_play_state();
      const t0 = performance.now();
      x.dl_play_process(b * QUANTUM);
      const t = performance.now() - t0;
      if (t < best[b]) best[b] = t;
    }
  }
  let worst = 0; let worstLoad = 0; let load = 0; let all = 0;
  for (let b = 0; b < blocks; b++) {
    all += best[b];
    worst = Math.max(worst, best[b]);
    if (states[b] === 1 || states[b] === 2) { load++; worstLoad = Math.max(worstLoad, best[b]); }
  }
  return { worst, worstLoad, load, all, seconds: (blocks * QUANTUM) / way.rate };
}

const deadline = (QUANTUM / way.rate) * 1000;
// How many runs each block's least time is taken over (`--runs`, 3 by
// default): more steadies the worst block against the machine's noise.
const RUNS = Math.max(1, Number(flag('--runs', 3)) || 3);
const costing = argv.includes('--costs');
if (costing) console.log(`# the deadline: a block of ${QUANTUM} frames at ${way.rate} Hz, ${(deadline * 1000).toFixed(0)} us; times are each block's least over ${RUNS} runs, the player's decide and the core's block together`);
let worst = 0; let worstLoad = 0;
for (const [i, seed] of seeds) {
  try {
    if (costing) {
      const c = costs(seed);
      worst = Math.max(worst, c.worst);
      worstLoad = Math.max(worstLoad, c.worstLoad);
      console.log(`${i == null ? '' : `${i} `}seed=${seed} load=${c.load} blocks; worst block ${(c.worst * 1000).toFixed(0)} us (${((100 * c.worst) / deadline).toFixed(1)}%), worst while loading ${(c.worstLoad * 1000).toFixed(0)} us; ${((100 * c.all) / (c.seconds * 1000)).toFixed(1)}% of the time it played`);
    } else {
      console.log(`${i == null ? '' : `${i} `}${render(seed)}`);
    }
  } catch (err) {
    console.error(`player-wasm: seed ${seed}: ${err.message}`);
    process.exit(1);
  }
}
if (costing) console.log(`worst block ${(worst * 1000).toFixed(0)} us (${((100 * worst) / deadline).toFixed(1)}% of the deadline); worst block while loading ${(worstLoad * 1000).toFixed(0)} us (${((100 * worstLoad) / deadline).toFixed(1)}%)`);
