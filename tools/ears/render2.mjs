// render.mjs, extended for this session: a job list instead of one setting,
// prototypes (PROTO) and space recipes (SPACE) per job, refused-note
// counts from the voice budget, chord changes a minute from the generator,
// and the render's wall time (a rough cost proxy).
//
// Usage: node render2.mjs REPO OUTDIR JOBS.json
// JOBS.json: { "seed": 20261009, "n": 80, "jobs": [ { "pick": 8, "seconds": 60,
//   "tag": "mono", "proto": {...}, "space": {...} }, ... ] }
import fs from 'node:fs';
import http from 'node:http';
import path from 'node:path';
import { createRequire } from 'node:module';
import { execSync } from 'node:child_process';

const [REPO, OUT, JOBS] = process.argv.slice(2);
const ROOT = path.resolve(REPO);
const job = JSON.parse(fs.readFileSync(JOBS, 'utf8'));
fs.mkdirSync(OUT, { recursive: true });
const RATE = 44100;

const PAGE = `<!doctype html><meta charset="utf-8"><script type="module">
import { Engine } from '/js/engine.js';
import { Synth } from '/js/synth.js';
import { newSpec, render } from '/js/generator.js';
import { Rng, mulberry32 } from '/js/rng.js';

function leadOf(spec) {
  return Object.entries(spec.mix || {}).sort((a, b) => b[1] - a[1])[0]?.[0] || '?';
}
window.specsFor = (seed, n) => {
  const master = new Rng(seed);
  return Array.from({ length: n }, () => newSpec(master.seed32()));
};
function chordStats(spec) {
  const r = render(spec);
  const ev = (r.tracks?.chords || []).filter((e) => e.vel && e.notes && e.notes.length);
  let changes = 0, prev = null;
  for (const e of ev) {
    const k = e.notes.slice().sort((a, b) => a - b).join(',');
    if (prev !== null && k !== prev) changes++;
    prev = k;
  }
  const sec = (r.totalSteps || 0) * 60 / spec.bpm / 4;
  return { chordEvents: ev.length, changesPerMin: sec ? changes / sec * 60 : null, loopSec: sec };
}
window.renderOne = async (spec, seconds, rate, proto, space) => {
  globalThis.PROTO = proto || {};
  globalThis.SPACE = space || null;
  const ctx = new OfflineAudioContext(2, Math.ceil(seconds * rate), rate);
  Math.random = mulberry32(spec.seed >>> 0 || 1);
  const synth = new Synth(ctx, 'full');
  let asked = 0, refused = 0; const who = {};
  const orig = synth._budget.bind(synth);
  synth._budget = (t, s, c) => { asked++; const ok = orig(t, s, c); if (!ok) { refused++; const f = (new Error().stack.split('\\n')[2] || '').trim().split(' ')[1] || '?'; who[f] = (who[f] || 0) + 1; } return ok; };
  const engine = new Engine(ctx, synth);
  if (engine.clock.worker) engine.clock.worker.terminate();
  engine.driftOn = true;
  engine.driftAmount = 1;
  engine.load(spec);
  engine.playing = true;
  engine.nextStepTime = 0.05;
  let guard = 0;
  while (engine.nextStepTime < seconds && guard++ < 400000) {
    engine._scheduleStep(engine.step, engine.nextStepTime);
    engine._advance();
  }
  const t0 = performance.now();
  const buf = await ctx.startRendering();
  const ms = performance.now() - t0;
  const L = buf.getChannelData(0), R = buf.getChannelData(1);
  const f32 = new Float32Array(L.length * 2);
  for (let i = 0; i < L.length; i++) { f32[2 * i] = L[i]; f32[2 * i + 1] = R[i]; }
  let s = '';
  const bytes = new Uint8Array(f32.buffer);
  for (let i = 0; i < bytes.length; i += 0x8000) s += String.fromCharCode.apply(null, bytes.subarray(i, i + 0x8000));
  const cs = chordStats(spec);
  const r = render(spec);
  return {
    b64: btoa(s), lead: leadOf(spec), seed: spec.seed, bpm: spec.bpm ?? null,
    meta: r.meta || {}, bars: r.totalSteps ? r.totalSteps / 16 : null,
    asked, refused, who, ms, ...cs,
  };
};
window.ready = true;
</script>`;

const TYPES = { '.js': 'text/javascript', '.mjs': 'text/javascript', '.wasm': 'application/wasm', '.json': 'application/json', '.html': 'text/html' };
const server = http.createServer((req, res) => {
  const url = decodeURIComponent(req.url.split('?')[0]);
  if (url === '/__render.html') { res.writeHead(200, { 'content-type': 'text/html' }); return res.end(PAGE); }
  const file = path.join(ROOT, path.normalize(url));
  if (!file.startsWith(ROOT) || !fs.existsSync(file) || fs.statSync(file).isDirectory()) { res.writeHead(404); return res.end(); }
  res.writeHead(200, { 'content-type': TYPES[path.extname(file)] || 'application/octet-stream' });
  fs.createReadStream(file).pipe(res);
});
await new Promise((r) => server.listen(0, '127.0.0.1', r));
const port = server.address().port;

const require = createRequire(import.meta.url);
const globalRoot = execSync('npm root -g').toString().trim();
const { chromium } = require(path.join(globalRoot, 'playwright'));
const browser = await chromium.launch({ args: ['--no-sandbox'] });
const page = await browser.newPage();
page.on('pageerror', (e) => console.error('page error:', e.message));
page.on('console', (m) => { if (m.type() === 'error') console.error('console:', m.text()); });
await page.goto(`http://127.0.0.1:${port}/__render.html`);
await page.waitForFunction(() => window.ready === true, null, { timeout: 20000 });

const specs = await page.evaluate(([s, n]) => window.specsFor(s, n), [job.seed ?? 20261009, job.n ?? 80]);
const index = [];
for (const j of job.jobs) {
  const spec = specs[j.pick];
  const r = await page.evaluate(([spec, sec, rate, proto, space]) => window.renderOne(spec, sec, rate, proto, space),
    [spec, j.seconds ?? 60, RATE, j.proto || {}, j.space || null]);
  const f32 = Buffer.from(r.b64, 'base64');
  // 32-bit float WAV, so nothing clips or rounds before measuring.
  const header = Buffer.alloc(44);
  header.write('RIFF', 0); header.writeUInt32LE(36 + f32.length, 4); header.write('WAVE', 8);
  header.write('fmt ', 12); header.writeUInt32LE(16, 16); header.writeUInt16LE(3, 20); header.writeUInt16LE(2, 22);
  header.writeUInt32LE(RATE, 24); header.writeUInt32LE(RATE * 8, 28); header.writeUInt16LE(8, 32); header.writeUInt16LE(32, 34);
  header.write('data', 36); header.writeUInt32LE(f32.length, 40);
  const name = `${String(j.pick).padStart(3, '0')}-${r.lead}-${j.tag}.wav`;
  fs.writeFileSync(path.join(OUT, name), Buffer.concat([header, f32]));
  delete r.b64;
  index.push({ file: name, pick: j.pick, tag: j.tag, proto: j.proto || {}, space: j.space || null, ...r });
  console.log(name, `refused ${r.refused}/${r.asked} ${JSON.stringify(r.who)}`, `chords/min ${r.changesPerMin?.toFixed(1)}`, `${Math.round(r.ms)} ms`);
}
fs.writeFileSync(path.join(OUT, `index-${path.basename(JOBS, '.json')}.json`), JSON.stringify(index, null, 1));
await browser.close();
server.close();
