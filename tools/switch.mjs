// The quality switch the app makes by itself (queue item 87b), rendered: the
// real Synth, Engine and js/quality.js's hand-off, in an OfflineAudioContext
// in headless Chromium, so the join can be measured sample by sample.
//
//   node tools/switch.mjs [--n 20] [--from lite] [--gap 0.12]
//
// For each loop of the corpus (seed 1, as tools/measure.mjs draws it), stereo,
// on the core: the loop plays at full quality (or lite with --from lite); at
// the first pass boundary after 2.5 s the old quality hands off as the app's
// does (`quality.handOff`: nothing decided from the boundary, what was handed
// over for it dropped, a fade of `quality.FADE` to it), and a new synth at the
// other quality takes the loop from its top `--gap` seconds after the boundary
// (the time the app takes to build it once the old one is gone: the alarm, the
// node, the engine's first lead). Reported per loop and overall:
//
//   step   the largest step between two samples from the fade's start to half
//          a second into the new quality, against the largest the loop takes
//          anywhere else in the same render (its own attacks): a click shows
//          as a step larger than the loop's own.
//   late   notes either core took late or missed, and dropped.
//
// Nothing in js/ is touched.

import fs from 'node:fs';
import path from 'node:path';
import http from 'node:http';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';

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
const N = Number(flag('--n', 20));
const FROM = flag('--from', 'full');
const GAP = Number(flag('--gap', 0.12));

function loadPlaywright() {
  const require = createRequire(import.meta.url);
  for (const id of ['playwright', 'playwright-core', '/opt/node22/lib/node_modules/playwright']) {
    try { return require(id); } catch { /* try the next one */ }
  }
  console.error('switch: Playwright is not installed.  npm install --no-save playwright@1.56.1 && npx playwright install chromium');
  process.exit(3);
  return null;
}

const PAGE = `<!doctype html><meta charset="utf-8"><title>switch</title>
<script type="module">
import { Synth } from '/js/synth.js';
import { Engine } from '/js/engine.js';
import { loadCore } from '/js/core.js';
import { newSpec } from '/js/generator.js';
import { Rng } from '/js/rng.js';
import * as quality from '/js/quality.js';

const RATE = 48000;
// Every step from the engine's next up to \`end\` (and its \`until\`).
function decide(engine, end) {
  let guard = 0;
  while (engine.nextStepTime < end && engine.nextStepTime < engine.until && guard++ < 200000) {
    engine._scheduleStep(engine.step, engine.nextStepTime);
    engine._advance();
  }
}
function engineFor(ctx, synth, spec, resume) {
  const engine = new Engine(ctx, synth);
  if (engine.clock.worker) engine.clock.worker.terminate();
  engine.load(spec, { resume });
  engine.playing = true;
  return engine;
}

window.run = async (n, from, gap) => {
  const rng = new Rng(1);
  const out = [];
  for (let i = 0; i < n; i++) {
    const seed = rng.seed32();
    const spec = newSpec(seed);
    const pass = spec.bars * (spec.stepsPerBar || 16) * (60 / spec.bpm / 4);
    // The boundary: the first pass's end after 2.5 s.
    const at = 0.05 + Math.max(1, Math.ceil((2.5 - 0.05) / pass)) * pass;
    const seconds = at + gap + Math.min(8, Math.max(3, pass));
    const ctx = new OfflineAudioContext(2, Math.ceil(seconds * RATE), RATE);
    const core = await loadCore(ctx);
    const to = from === 'full' ? 'lite' : 'full';
    const a = new Synth(ctx, from, { engine: 'rust', core, layout: 2 });
    const ea = engineFor(ctx, a, spec, false);
    ea.nextStepTime = 0.05;
    // What the engine had decided ahead when the switch was asked (the
    // core's lookahead, two seconds), then the hand-off.
    decide(ea, at + 2);
    quality.handOff({ engine: ea, synth: a }, at);
    decide(ea, seconds);
    const b = new Synth(ctx, to, { engine: 'rust', core, layout: 2 });
    const eb = engineFor(ctx, b, spec, true);
    eb.nextStepTime = at + gap;
    decide(eb, seconds);
    await a.core.flush();
    await b.core.flush();
    const buf = await ctx.startRendering();
    const ca = await a.core.flush();
    const cb = await b.core.flush();
    a.core.dispose();
    b.core.dispose();
    const from0 = Math.floor((at - quality.FADE - 0.02) * RATE);
    const to0 = Math.ceil((at + gap + 0.5) * RATE);
    let join = 0;
    let own = 0;
    for (let c = 0; c < 2; c++) {
      const d = buf.getChannelData(c);
      for (let k = 1; k < d.length; k++) {
        const s = Math.abs(d[k] - d[k - 1]);
        if (k >= from0 && k <= to0) join = Math.max(join, s);
        else own = Math.max(own, s);
      }
    }
    out.push({ i, seed, name: spec.name, at, join, own, late: ca.late + cb.late, missed: (ca.missed || 0) + (cb.missed || 0), dropped: ca.dropped + cb.dropped });
  }
  return out;
};
window.ready = true;
</script>`;

const TYPES = { '.html': 'text/html', '.js': 'text/javascript', '.mjs': 'text/javascript', '.wasm': 'application/wasm', '.txt': 'text/plain' };
const server = http.createServer((req, res) => {
  const url = decodeURIComponent(req.url.split('?')[0]);
  if (url === '/__switch.html') {
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
await new Promise((resolve, reject) => { server.once('error', reject); server.listen(0, '127.0.0.1', resolve); });
const { chromium } = loadPlaywright();
const browser = await chromium.launch({ args: ['--no-sandbox'] });
let failed = false;
try {
  const page = await browser.newPage();
  const errors = [];
  page.on('pageerror', (e) => errors.push(e.message));
  await page.goto(`http://127.0.0.1:${server.address().port}/__switch.html`);
  await page.waitForFunction(() => window.ready === true, null, { timeout: 30000 }).catch(() => {
    console.error(`switch: the page never loaded.\n  ${errors.join('\n  ')}`);
    process.exit(1);
  });
  const rows = await page.evaluate(({ n, from, gap }) => window.run(n, from, gap), { n: N, from: FROM, gap: GAP });
  let worst = 0;
  let late = 0;
  for (const r of rows) {
    const ratio = r.own > 0 ? r.join / r.own : r.join > 0 ? Infinity : 0;
    worst = Math.max(worst, ratio);
    late += r.late + r.missed;
    console.log(`${String(r.i).padStart(2)} ${r.name.padEnd(14)} switch at ${r.at.toFixed(2)} s: step ${r.join.toFixed(4)} against the loop's own ${r.own.toFixed(4)} (${ratio.toFixed(2)}x); late ${r.late}, missed ${r.missed}, dropped ${r.dropped}`);
  }
  console.log(`${FROM} to ${FROM === 'full' ? 'lite' : 'full'}, ${rows.length} loops: the join's largest step ${worst.toFixed(2)}x the loop's own at most; ${late} notes late or missed`);
  failed = worst > 1 || late > 0;
} finally {
  await browser.close();
  server.close();
}
process.exit(failed ? 1 : 0);
