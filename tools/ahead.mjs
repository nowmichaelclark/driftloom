// Rendering ahead on the web host (queue item 87): the player in
// js/dlplayer.wasm, in Node and in headless Chromium.
//
//   node tools/ahead.mjs [--loops vei-koan,...] [--seconds S] [--runs N] [--rate HZ]
//                       the speed test (js/speed.js) in Node: per loop, full and
//                       lite, mono and stereo, the worst block as a % of its
//                       deadline and the render-ahead speed as times real time
//   ... --chromium      the same, in headless Chromium, through the page Mikey
//                       runs on his phone (speed.html, its worker and all)
//   node tools/ahead.mjs --sweep
//                       chunked equals straight through: loops rendered ahead
//                       in chunks of every size from 128 to 256 frames, then
//                       sizes rising by 15% up to a pass, and a pass at a
//                       time (dl_play_pass_frames), each against the same
//                       render straight through, sample for sample; wandering
//                       pass after pass, and with a change waiting at a pass's
//                       end. Fails on any difference.
//   node tools/ahead.mjs --estimate [--corpus SEED:N]
//                       the core's estimate of a chunk's cost
//                       (dl_play_estimate) against the time it took, a pass at
//                       a time: raw, and as a host uses it (scaled by the last
//                       pass's measured over estimated)
//
// The module's render export takes at most 4,096 frames a call (2,048 a
// side in stereo), so a bigger chunk is rendered as calls of that size and
// its rest: what is tested is where chunks end, which is all that differs.

import fs from 'node:fs';
import path from 'node:path';
import http from 'node:http';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';
import { performance } from 'node:perf_hooks';
import { LOOPS, WAYS, measure, line, tick, warm } from '../js/speed.js';
import { Rng } from '../js/rng.js';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const argv = process.argv.slice(2);
if (argv.includes('--help') || argv.includes('-h')) {
  console.log(fs.readFileSync(fileURLToPath(import.meta.url), 'utf8').split('\n').filter((l) => l.startsWith('//')).map((l) => l.slice(3)).join('\n'));
  process.exit(0);
}
const flag = (name, dflt) => {
  const i = argv.indexOf(name);
  return i >= 0 ? argv[i + 1] : dflt;
};
const bytes = fs.readFileSync(path.join(ROOT, 'js', 'dlplayer.wasm'));
const module = new WebAssembly.Module(bytes);
const QUANTUM = 128;
const MAX = 4096;

// A core and the player on it, the loop made at once.
function start(seed, { rate = 44100, full = true, stereo = false } = {}) {
  const x = new WebAssembly.Instance(module, {}).exports;
  x.dl_init(rate);
  x.dl_quality(full ? 1 : 0);
  if (stereo) x.dl_layout(2);
  x.dl_play_wander(1);
  x.dl_play_attach(full ? 1 : 0);
  x.dl_play_load(seed >>> 0, 0);
  x.dl_play_ready();
  return x;
}

// `frames` frames of the loop of `seed`, stereo, in chunks: `cut(x, k)` says
// how long chunk k is; a change to `change.seed` at the next pass is asked
// once `change.at` frames are rendered (a chunk ends there).
function chunked(seed, frames, cut, change) {
  const x = start(seed, { stereo: true });
  const left = new Float32Array(frames);
  const right = new Float32Array(frames);
  let done = 0; let k = 0;
  while (done < frames) {
    if (change && done === change.at && !x.dl_play_change(change.seed, 4, 1)) throw new Error('the core refused the change');
    let n = Math.min(cut(x, k++), frames - done);
    if (change && done < change.at) n = Math.min(n, change.at - done);
    if (!(n > 0)) throw new Error(`a chunk of ${n} frames`);
    // One chunk: calls of at most the module's room, then its rest.
    for (let i = 0; i < n;) {
      const m = Math.min(MAX / 2, n - i);
      const at = x.dl_play_render(m);
      if (!at) throw new Error('the core would not render');
      left.set(new Float32Array(x.memory.buffer, at, m), done + i);
      right.set(new Float32Array(x.memory.buffer, at + m * 4, m), done + i);
      i += m;
    }
    done += n;
  }
  if (x.dl_dropped() || x.dl_late() || x.dl_missed()) throw new Error(`the core dropped ${x.dl_dropped()}, took ${x.dl_late()} late and missed ${x.dl_missed()}`);
  return [left, right];
}

const same = (a, b) => a[0].length === b[0].length && a.every((lane, c) => {
  const p = new Uint32Array(lane.buffer); const q = new Uint32Array(b[c].buffer);
  for (let i = 0; i < p.length; i++) if (p[i] !== q[i]) return false;
  return true;
});

function sweep() {
  // Short loops of the corpus (passes of 3 to 5 s at 44.1 kHz), so a sweep
  // renders a few passes a size: shaisk-sha, sheing-shoa, rei-saisk.
  const cases = [
    { name: 'shaisk-sha', seed: 1027928796, every: true },
    { name: 'sheing-shoa', seed: 209265687 },
    { name: 'rei-saisk', seed: 663542962, change: 3730978585 },
  ];
  let renders = 0;
  for (const c of cases) {
    const pass = start(c.seed, { stereo: true }).dl_play_pass_frames();
    // The first pass (lead-in and all) and the next, and a little: the
    // wander's second pass drawn and played.
    const frames = Math.round(pass * 2) + 5000;
    const change = c.change ? { seed: c.change, at: Math.round(pass / 3) } : null;
    const straight = chunked(c.seed, frames, () => frames, change);
    const sizes = [];
    // Every size from 128 to 256 (over a second and a half: a chunk's end
    // falls at every place in a block, again and again).
    const short = Math.min(frames, 66150);
    if (c.every) for (let n = 128; n <= 256; n++) sizes.push([n, short]);
    for (let n = c.every ? 282 : 128; n < pass; n = Math.ceil(n * 1.15)) sizes.push([n, frames]);
    sizes.push([pass, frames], [pass + 1, frames]);
    const cuts = sizes.map(([n, len]) => [`${n}`, () => n, len]);
    cuts.push(['a pass at a time', (x) => x.dl_play_pass_frames(), frames]);
    cuts.push(['1, 4410, 777, a pass, 3', (x, k) => [1, 4410, 777, x.dl_play_pass_frames(), 3][k % 5], frames]);
    for (const [label, cut, n] of cuts) {
      const want = n === frames ? straight : straight.map((l) => l.slice(0, n));
      const got = chunked(c.seed, n, cut, change && change.at < n ? change : null);
      renders++;
      if (!same(got, want)) {
        console.error(`ahead: ${c.name}: chunks of ${label} frames differ from straight through`);
        process.exit(1);
      }
    }
    console.log(`ahead: ${c.name} (a pass of ${pass} frames${change ? ', a change waiting at the first pass\'s end' : ''}): ${cuts.length} ways of chunking, every one the same samples as straight through`);
  }
  console.log(`ahead: all ${renders} chunked renders equal straight through, sample for sample`);
}

function estimate() {
  const [from, n] = (flag('--corpus', '1:40')).split(':').map(Number);
  const r = new Rng(from);
  const seeds = [];
  for (let i = 0; i < n; i++) seeds.push(r.seed32());
  const passes = Number(flag('--passes', 4));
  for (const way of WAYS) {
    const raw = []; const first = []; const scaled = [];
    for (const seed of seeds) {
      const x = start(seed, { full: way.full, stereo: way.stereo });
      let last = null;
      for (let p = 0; p < passes; p++) {
        const n = x.dl_play_pass_frames();
        const est = x.dl_play_estimate(n);
        const t0 = performance.now();
        for (let i = 0; i < n; i += MAX / 2) x.dl_play_render(Math.min(MAX / 2, n - i));
        const took = (performance.now() - t0) * 1000;
        (p === 0 ? first : raw).push(est / took);
        if (last) scaled.push((est * (last.took / last.est)) / took);
        last = { est, took };
      }
    }
    const q = (a) => {
      a.sort((p, s) => p - s);
      const at = (f) => a[Math.min(a.length - 1, Math.floor(f * a.length))].toFixed(2);
      return `median ${at(0.5)}, 10-90% ${at(0.1)}-${at(0.9)}, ${at(0)}-${at(1)} at the ends`;
    };
    console.log(`${way.full ? 'full' : 'lite'} ${way.stereo ? 'stereo' : 'mono'}: estimate / took, first pass (no units measured): ${q(first)}`);
    console.log(`  later passes, raw: ${q(raw)}`);
    console.log(`  later passes, scaled by the last pass (as a host uses it): ${q(scaled)}`);
  }
}

function loadPlaywright() {
  const require = createRequire(import.meta.url);
  for (const id of ['playwright', 'playwright-core', '/opt/node22/lib/node_modules/playwright']) {
    try { return require(id); } catch { /* try the next one */ }
  }
  console.error('ahead: Playwright is not installed.  npm install --no-save playwright@1.56.1 && npx playwright install chromium');
  process.exit(3);
  return null;
}

// The speed test in headless Chromium: speed.html served as the site serves
// it (cross-origin isolated, as _headers asks), run with ?auto.
async function inChromium(query) {
  const TYPES = { '.html': 'text/html', '.js': 'text/javascript', '.mjs': 'text/javascript', '.wasm': 'application/wasm' };
  const server = http.createServer((req, res) => {
    const url = decodeURIComponent(req.url.split('?')[0]);
    const file = path.join(ROOT, path.normalize(url));
    if (!file.startsWith(ROOT) || !fs.existsSync(file) || fs.statSync(file).isDirectory()) {
      res.writeHead(404);
      return res.end('not found');
    }
    res.writeHead(200, {
      'content-type': TYPES[path.extname(file)] || 'application/octet-stream',
      'cross-origin-opener-policy': 'same-origin',
      'cross-origin-embedder-policy': 'require-corp',
    });
    return fs.createReadStream(file).pipe(res);
  });
  await new Promise((resolve, reject) => { server.once('error', reject); server.listen(0, '127.0.0.1', resolve); });
  const { chromium } = loadPlaywright();
  const browser = await chromium.launch({ args: ['--no-sandbox', '--autoplay-policy=no-user-gesture-required'] });
  try {
    const page = await browser.newPage();
    const errors = [];
    page.on('pageerror', (e) => errors.push(e.message));
    await page.goto(`http://127.0.0.1:${server.address().port}/speed.html?auto&${query}`);
    await page.waitForFunction(() => document.title.endsWith('done') || /error:/.test(document.getElementById('out').textContent), null, { timeout: 0 });
    const text = await page.$eval('#out', (e) => e.textContent);
    if (errors.length) console.error(errors.join('\n'));
    return text;
  } finally {
    await browser.close();
    server.close();
  }
}

if (argv.includes('--sweep')) {
  sweep();
} else if (argv.includes('--estimate')) {
  estimate();
} else {
  const names = flag('--loops', null);
  const seconds = flag('--seconds', null);
  const runs = flag('--runs', null);
  const rate = Number(flag('--rate', 44100));
  if (argv.includes('--chromium')) {
    const query = new URLSearchParams({ rate: String(rate), ...(names ? { loops: names } : {}), ...(seconds ? { seconds } : {}), ...(runs ? { runs } : {}) });
    console.log(await inChromium(query.toString()));
  } else {
    const now = () => performance.now();
    console.log(`# Node ${process.version}, rate ${rate} Hz (deadline ${(QUANTUM / rate * 1e6).toFixed(0)} us a block), clock step ${tick(now).toFixed(2)} us`);
    warm(module, { rate, now });
    const pick = names ? LOOPS.filter((l) => names.split(',').includes(l.name)) : LOOPS;
    const speeds = {};
    for (const loop of pick) {
      for (const way of WAYS) {
        const r = measure(module, { seed: loop.seed, way, rate, seconds: Number(seconds || 45), runs: Number(runs || 3), aheadRuns: 3, now });
        console.log(line(loop, way, r));
        speeds[`${loop.name} ${way.full} ${way.stereo}`] = r.speed;
      }
      const m = speeds[`${loop.name} false false`]; const s = speeds[`${loop.name} false true`];
      if (m && s) console.log(`  ${loop.name}: lite, stereo costs ${(m / s).toFixed(2)}x mono rendering ahead`);
    }
  }
}
