// Export the golden note lists (queue item 42).
//
//   node tools/golden-export.mjs              write core/golden/*.dlnotes from core/golden/codes.txt
//   node tools/golden-export.mjs --pick       scan a corpus (--n 1500 --window 24) and print a greedy cover of every voice;
//                                             --rows FILE keeps what each loop plays, to choose better by hand
//   options:  --only NAME   one list;  --chrome PATH;  --port N
//
// The real Synth and Engine, in headless Chromium, play each code into an
// OfflineAudioContext on the core, exactly as tools/measure.mjs does. The
// host's calls to the core (CoreHost.note, .param, .damp) are recorded in
// the order they are made, and that is the note list: everything the core
// needs to render the loop with no JavaScript, in the text format the native
// host reads (core/examples/dlrender/notelist.rs).
//
// Two things make the lists reproducible and honest:
//
//   - Math.random is seeded before the synth is built, so the same codes
//     export the same lists (the synth's articulation draws travel with
//     each note; item 44 moves them into the core), and the synth's noise
//     is the first thing it draws: the list carries only the seed, and the
//     export checks the noise the synth made against the recipe the hosts
//     use (golden-lib.mjs noiseFor).
//   - Each loop is also rendered in the page, and the list is rendered again
//     in Node through the same js/dlcore.wasm. The hashes must be equal: if
//     they are, the list holds every call that mattered, and none of the
//     loop played in JavaScript (a note the core could not take would play
//     in Web Audio and be missing from the list; the export fails on any).
//
// codes.txt is one loop a line: NAME SECONDS QUALITY CODE ('#' comments).
// Needs Playwright and Chromium, like tools/measure.mjs.

import http from 'node:http';
import fs from 'node:fs';
import path from 'node:path';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';
import { readList, renderWasm, writeList, SIGNALS } from './golden-lib.mjs';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const GOLDEN = path.join(ROOT, 'core', 'golden');
const RATE = 48000;
const argv = process.argv.slice(2);
const flag = (name, dflt) => {
  const i = argv.indexOf(name);
  return i >= 0 ? argv[i + 1] : dflt;
};
if (argv.includes('--help') || argv.includes('-h')) {
  console.log(fs.readFileSync(fileURLToPath(import.meta.url), 'utf8').split('\n').filter((l) => l.startsWith('//')).map((l) => l.slice(3)).join('\n'));
  process.exit(0);
}
const pick = argv.includes('--pick');
const only = flag('--only', null);
const PORT = Number(flag('--port', 8751));

// ------------------------------------------------------------- the page

const PAGE = `<!doctype html><meta charset="utf-8"><title>golden</title>
<script type="module">
import { Engine } from '/js/engine.js';
import { Synth } from '/js/synth.js';
import { loadCore, CoreHost, CORE_VOICES } from '/js/core.js';
import { decodeSong } from '/js/share.js';
import { newSpec } from '/js/generator.js';
import { Rng, mulberry32 } from '/js/rng.js';
import { Fnv, noiseFor } from '/tools/golden-lib.mjs';

const QUANTUM = 128;
const NAMES = Object.fromEntries(Object.entries(CORE_VOICES).map(([k, v]) => [v, k]));

// Play a spec into a synth on the core, the way tools/measure.mjs does, and
// record what the host sent. Returns what it needs to export or to pick.
async function play(spec, seconds, quality, noiseSeed, { render }) {
  const frames = Math.ceil((seconds * ${RATE}) / QUANTUM) * QUANTUM;
  const ctx = new OfflineAudioContext(render ? 7 : 1, frames, ${RATE});
  const core = await loadCore(ctx);
  const events = [];
  const was = {};
  for (const name of ['note', 'damp', 'param', 'cancel']) was[name] = CoreHost.prototype[name];
  CoreHost.prototype.note = function (voice, channel, time, midi, dur, vel, parts, extra, draws) {
    events.push({ type: 'note', voice, channel, time, midi, dur, vel, parts, extra: extra ? Array.from(extra) : [], draws: draws ? Array.from(draws) : null });
    return was.note.apply(this, arguments);
  };
  CoreHost.prototype.damp = function (channel, time) {
    events.push({ type: 'damp', channel, time });
    return was.damp.apply(this, arguments);
  };
  CoreHost.prototype.param = function (id, op, value, time, tau = 0) {
    events.push({ type: 'param', id, op, value, time, tau });
    return was.param.apply(this, arguments);
  };
  CoreHost.prototype.cancel = function () {
    throw new Error('the synth cancelled notes in an offline render');
  };
  try {
    Math.random = mulberry32(noiseSeed);
    const synth = new Synth(ctx, quality, { engine: 'rust', core, taps: true });
    if (!synth.core) throw new Error('the synth has no core: ' + synth.coreError);
    const noise = new Fnv();
    noise.samples(synth.noise.getChannelData(0));
    const recipe = new Fnv();
    recipe.samples(noiseFor(noiseSeed, ${RATE}));
    const engine = new Engine(ctx, synth);
    if (engine.clock.worker) engine.clock.worker.terminate();
    let hashes = null;
    if (render) {
      const node = synth.core.node;
      node.disconnect(0);
      const merger = ctx.createChannelMerger(7);
      merger.connect(ctx.destination);
      node.connect(merger, 0, 0);
      for (let c = 0; c < 5; c++) node.connect(merger, synth.core.tapOf(c), 1 + c);
      node.connect(merger, synth.core.beforeComp(), 6);
    }
    engine.load(spec);
    engine.playing = true;
    engine.nextStepTime = 0.05;
    let guard = 0;
    while (engine.nextStepTime < seconds && guard++ < 200000) {
      engine._scheduleStep(engine.step, engine.nextStepTime);
      engine._advance();
    }
    await synth.core.flush();
    if (render) {
      const buf = await ctx.startRendering();
      hashes = Array.from({ length: 7 }, (_, c) => {
        const h = new Fnv();
        h.samples(buf.getChannelData(c));
        return h.hex();
      });
    }
    const fallbacks = synth.fallbacks;
    synth.core.dispose();
    return { events, frames, hashes, fallbacks, noiseOk: noise.hex() === recipe.hex() };
  } finally {
    Object.assign(CoreHost.prototype, was);
  }
}

window.exportLoop = async ({ code, seconds, quality, noiseSeed }) => {
  const spec = decodeSong(code);
  const r = await play(spec, seconds, quality, noiseSeed, { render: true });
  return { ...r, bars: spec.bars, bpm: spec.bpm, seed: spec.seed };
};

// What a note is, for coverage: its voice (and which of a voice's kinds)
// and the channel it plays on.
function keyOf(e) {
  const name = NAMES[e.voice];
  const kind = ['bass', 'texture', 'drum', 'hat'].includes(name) ? ':' + (name === 'bass' ? e.parts & 7 : e.parts) : '';
  return name + kind + '@' + e.channel;
}

window.scanCorpus = async ({ seed, from, to, seconds, quality }) => {
  const master = new Rng(seed);
  const out = [];
  for (let i = 0; i < to; i++) {
    const spec = newSpec(master.seed32());
    if (i < from) continue;
    const loopDur = spec.bars * (spec.stepsPerBar || 16) * (60 / spec.bpm / 4);
    const window = Math.min(seconds, loopDur);
    const r = await play(spec, window, quality, spec.seed >>> 0 || 1, { render: false });
    const keys = [...new Set(r.events.filter((e) => e.type === 'note').map(keyOf))];
    const notes = r.events.filter((e) => e.type === 'note').length;
    out.push({ i, seed: spec.seed, loopDur, window, keys, notes, fallbacks: r.fallbacks });
  }
  return out;
};
window.ready = true;
</script>`;

// ------------------------------------------------------------- the server

function loadPlaywright() {
  const require = createRequire(import.meta.url);
  for (const id of ['playwright', 'playwright-core', '/opt/node22/lib/node_modules/playwright']) {
    try { return require(id); } catch { /* try the next one */ }
  }
  try {
    const root = require('node:child_process').execSync('npm root -g', { encoding: 'utf8' }).trim();
    return require(path.join(root, 'playwright'));
  } catch { /* fall through */ }
  console.error('golden-export: Playwright is not installed.  npm install -g playwright && npx playwright install chromium');
  process.exit(3);
  return null;
}

const TYPES = { '.html': 'text/html', '.js': 'text/javascript', '.mjs': 'text/javascript', '.json': 'application/json', '.wasm': 'application/wasm', '.wav': 'audio/wav', '.flac': 'audio/flac' };
const server = http.createServer((req, res) => {
  const url = decodeURIComponent(req.url.split('?')[0]);
  if (url === '/__golden.html') {
    res.writeHead(200, { 'content-type': 'text/html' });
    return res.end(PAGE);
  }
  const file = path.join(ROOT, path.normalize(url));
  if (!file.startsWith(ROOT) || !fs.existsSync(file) || fs.statSync(file).isDirectory()) {
    res.writeHead(404);
    return res.end('not found');
  }
  res.writeHead(200, { 'content-type': TYPES[path.extname(file)] || 'application/octet-stream' });
  return fs.createReadStream(file).pipe(res);
});
await new Promise((resolve, reject) => {
  server.once('error', reject);
  server.listen(PORT, '127.0.0.1', resolve);
});
const { chromium } = loadPlaywright();
const browser = await chromium.launch({ executablePath: flag('--chrome', undefined), args: ['--no-sandbox', '--autoplay-policy=no-user-gesture-required'] });
const page = await browser.newPage();
const errors = [];
page.on('pageerror', (e) => errors.push(e.message));
page.on('crash', () => { console.error('golden-export: the page crashed'); process.exit(1); });
await page.goto(`http://127.0.0.1:${PORT}/__golden.html`);
try {
  await page.waitForFunction(() => window.ready === true, null, { timeout: 20000 });
} catch {
  console.error(`golden-export: the page never loaded the app modules.\n  ${errors.join('\n  ')}`);
  process.exit(1);
}
const done = async (code) => { await browser.close(); server.close(); process.exit(code); };

// ------------------------------------------------------------ --pick

if (pick) {
  // Every loop of a corpus, scanned for what it plays in its first seconds;
  // then a greedy cover: the loop that adds the most voices not yet heard,
  // shorter loops first on a tie, until every voice is covered.
  const WINDOW = Number(flag('--window', 16));
  const COUNT = Number(flag('--n', 150));
  const rows = await page.evaluate((o) => window.scanCorpus(o), { seed: 1, from: 0, to: COUNT, seconds: WINDOW, quality: 'full' });
  if (flag('--rows', null)) fs.writeFileSync(flag('--rows', null), `${JSON.stringify(rows)}\n`);
  const bad = rows.filter((r) => r.fallbacks);
  const all = new Set(rows.flatMap((r) => r.keys));
  const left = new Set(all);
  const chosen = [];
  while (left.size) {
    let best = null;
    for (const r of rows) {
      if (r.fallbacks) continue;
      const gain = r.keys.filter((k) => left.has(k)).length;
      if (gain && (!best || gain > best.gain || (gain === best.gain && r.window < best.row.window))) best = { row: r, gain };
    }
    if (!best) break;
    chosen.push(best.row);
    best.row.keys.forEach((k) => left.delete(k));
  }
  const busiest = rows.filter((r) => !r.fallbacks).sort((a, b) => b.notes / b.window - a.notes / a.window)[0];
  console.log(JSON.stringify({ scanned: rows.length, withFallbacks: bad.length, keys: [...all].sort(), chosen: chosen.map((r) => ({ i: r.i, seed: r.seed, window: r.window, notes: r.notes, keys: r.keys.length })), uncovered: [...left], busiest: busiest && { i: busiest.i, seed: busiest.seed, notes: busiest.notes, window: busiest.window } }, null, 1));
  await done(0);
}

// ------------------------------------------------------------ export

const codesFile = path.join(GOLDEN, 'codes.txt');
const loops = fs.readFileSync(codesFile, 'utf8').split('\n').filter((l) => l.trim() && !l.startsWith('#')).map((l) => {
  const [name, seconds, quality, code] = l.trim().split(/\s+/);
  return { name, seconds: Number(seconds), quality, code };
}).filter((l) => !only || l.name === only);
if (!loops.length) { console.error('golden-export: no loops'); await done(1); }

const wasm = fs.readFileSync(path.join(ROOT, 'js', 'dlcore.wasm'));
let failed = 0;
for (const loop of loops) {
  // The noise's seed is the loop's own, as measure.mjs seeds its renders.
  const r = await page.evaluate((o) => window.exportLoop(o), { ...loop, noiseSeed: 0 });
  const noiseSeed = r.seed >>> 0 || 1;
  // Seeded now that the loop's seed is known: the first page played it with
  // a placeholder, so play it again for the real export.
  const real = await page.evaluate((o) => window.exportLoop(o), { ...loop, noiseSeed });
  const problems = [];
  if (!real.noiseOk) problems.push('the synth\'s noise is not the recipe\'s');
  if (real.fallbacks) problems.push(`${real.fallbacks} note(s) played in JavaScript`);
  const list = {
    name: loop.name, rate: RATE, quality: loop.quality, noiseSeed, frames: real.frames, events: real.events,
    comment: [`${loop.name}: ${real.bars} bars at ${real.bpm} bpm, seed ${real.seed}, ${loop.seconds} s of it, ${loop.quality} quality.`, 'Exported by tools/golden-export.mjs from the code in codes.txt; do not edit by hand.'],
  };
  const text = writeList(list);
  // The list must read back to the same, and render in Node to what the page
  // rendered.
  const back = readList(text);
  const node = renderWasm(wasm, back);
  // What reaches the speakers and the bus compressor must match to the bit.
  // The channels' taps may not, and need not: Chromium turns on flush-to-zero
  // for its audio threads, which the worklet's WebAssembly runs on, so a
  // tail decaying through subnormal numbers is zero in the page and not in
  // Node (queue item 43 decides how the core treats them). The list checks
  // the master output and the bus; the taps are only counted.
  const differ = SIGNALS.filter((_, i) => node.hashes[i] !== real.hashes[i]);
  const tapsDiffer = differ.filter((s) => s !== 'out' && s !== 'bus');
  if (differ.includes('out') || differ.includes('bus')) problems.push(`the list renders differently from the page in: ${differ.join(', ')}`);
  if (problems.length) {
    failed++;
    console.error(`  FAIL  ${loop.name}: ${problems.join('; ')}`);
    continue;
  }
  fs.writeFileSync(path.join(GOLDEN, `${loop.name}.dlnotes`), text);
  const notes = real.events.filter((e) => e.type === 'note').length;
  console.log(`  ok    ${loop.name.padEnd(18)} ${String(loop.seconds).padStart(4)} s  ${String(notes).padStart(5)} notes  ${String(real.events.length - notes).padStart(4)} other  ${(text.length / 1024).toFixed(0).padStart(5)} KB${tapsDiffer.length ? `  (taps differ from the page: ${tapsDiffer.join(', ')})` : ''}`);
}
await done(failed ? 1 : 0);
