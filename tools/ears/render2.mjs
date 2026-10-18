// render.mjs, extended for the ears sessions: a job list instead of one
// setting, prototypes (PROTO) and space recipes (SPACE) per job, refused-note
// counts from the voice budget (and asked counts) per voice, chord changes a
// minute from the generator, and the render's wall time (a rough cost proxy).
//
// Usage: node render2.mjs REPO OUTDIR JOBS.json
// JOBS.json: { "seed": 20261009, "n": 80, "jobs": [ { "pick": 8, "seconds": 60,
//   "tag": "mono", "proto": {...}, "space": {...} }, ... ] }
// Per job, optionally (Ears 4):
//   "stems": true      also write <name>-stems.wav: one mono channel per layer
//                      (dry, after its channel gain) and per sketch, from the
//                      same render; names in the index ("stemNames")
//   "then": 12, "thenAt": 40   load another corpus loop partway through, as
//                      Next or a reroll would (checks what a sketch leaves
//                      behind across loads)
//   "reserve": 14      every budget check as if 14 more units were spent (what
//                      a fixed reservation for a sketch would cost the rest)
// PLAYWRIGHT_ROOT points at a folder holding the playwright package when it
// isn't in `npm root -g`.
// Ears 8: the index's "ears8" lists what each load's Ears 8 sketches drew
// (tunedKick, subDuck, lateFloor, padSweep, echoTime) and every floor drop
// with its pass, start time and shape; meta.e8pedal is the pedal bass's draw.
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
const STEM_LAYERS = ['drums', 'bass', 'chords', 'melody', 'texture'];
const STEM_SKETCHES = ['dubStab', 'organ', 'wash'];
// Also tapped with stems: what reaches the master saturator (a WaveShaper,
// which clamps its input at +-1), for checking whether the mix clips there.
const STEM_TAPS = ['satIn'];
window.renderOne = async (spec, seconds, rate, proto, space, opts = {}) => {
  globalThis.PROTO = proto || {};
  globalThis.SPACE = space || null;
  // Ears 8: what each load's sketches drew, and where each floor drop fell.
  globalThis.EARS8_LOG = [];
  const nStems = opts.stems ? STEM_LAYERS.length + STEM_SKETCHES.length + STEM_TAPS.length : 0;
  const ctx = new OfflineAudioContext(2 + nStems, Math.ceil(seconds * rate), rate);
  Math.random = mulberry32(spec.seed >>> 0 || 1);
  const synth = new Synth(ctx, 'full');
  let asked = 0, refused = 0; const who = {}, askedWho = {}, yields = {};
  const orig = synth._budget.bind(synth);
  const reserve = opts.reserve || 0;
  synth._budget = (t, s, c, probe) => {
    const f = (new Error().stack.split('\\n')[2] || '').trim().split(' ')[1] || '?';
    const ok = orig(t, s, (c ?? 12) + reserve, probe);
    // A probe (a sketch asking whether there is room, holding nothing) is
    // not a note asked for; when the answer is no, the sketch yields.
    if (probe) { if (!ok) yields[f] = (yields[f] || 0) + 1; return ok; }
    asked++;
    askedWho[f] = (askedWho[f] || 0) + 1;
    if (!ok) { refused++; who[f] = (who[f] || 0) + 1; }
    return ok;
  };
  const engine = new Engine(ctx, synth);
  if (engine.clock.worker) engine.clock.worker.terminate();
  engine.driftOn = true;
  engine.driftAmount = 1;
  engine.load(spec);
  engine.playing = true;
  engine.nextStepTime = 0.05;
  let guard = 0;
  let then = opts.thenSpec ? opts.thenAt : Infinity;
  while (engine.nextStepTime < seconds && guard++ < 400000) {
    if (engine.nextStepTime >= then) {
      // As Next does, offline: the new loop from its first step, now.
      then = Infinity;
      const at = engine.nextStepTime;
      engine.playing = false;
      engine.load(opts.thenSpec);
      engine.playing = true;
      engine.nextStepTime = at;
    }
    engine._scheduleStep(engine.step, engine.nextStepTime);
    engine._advance();
  }
  const stemNames = [];
  if (nStems) {
    // Every voice is scheduled by now, so every sketch node exists.
    const merger = ctx.createChannelMerger(2 + nStems);
    let k = 2;
    const tap = (node, gain, name) => {
      const g = ctx.createGain();
      g.gain.value = gain;
      g.channelCount = 1;
      g.channelCountMode = 'explicit';
      node.connect(g).connect(merger, 0, k++);
      stemNames.push(name);
    };
    for (const l of STEM_LAYERS) tap(synth.channels[l].gain, 1, l);
    for (const n of STEM_SKETCHES) {
      const sk = synth._sketch && synth._sketch[n];
      if (sk) tap(sk.node, synth.channels[sk.channel].base.gain, n);
      else { k++; stemNames.push(n + ' (none)'); }
    }
    if (synth.wobble) tap(synth.wobble, 1, 'satIn'); else { k++; stemNames.push('satIn (none)'); }
    ctx.destination.channelInterpretation = 'discrete';
    merger.connect(ctx.destination);
  }
  const t0 = performance.now();
  const buf = await ctx.startRendering();
  const ms = performance.now() - t0;
  const inter = (chans) => {
    const n = buf.length, c = chans.length;
    const f32 = new Float32Array(n * c);
    const d = chans.map((i) => buf.getChannelData(i));
    for (let i = 0; i < n; i++) for (let j = 0; j < c; j++) f32[c * i + j] = d[j][i];
    let s = '';
    const bytes = new Uint8Array(f32.buffer);
    for (let i = 0; i < bytes.length; i += 0x8000) s += String.fromCharCode.apply(null, bytes.subarray(i, i + 0x8000));
    return btoa(s);
  };
  const b64 = inter([0, 1]);
  const stemsB64 = nStems ? inter(Array.from({ length: nStems }, (_, i) => 2 + i)) : null;
  const E = engine._ears || null;
  const sketch = E ? {
    nps: +E.nps.toFixed(2), busy: E.busy,
    stab: E.stab && (E.stab === 'r1' ? 'r1' : { kind: E.stab.kind, ...E.stab.opts }),
    organ: E.organ && (E.organ === 'r1' ? 'r1' : { reg: E.organ.reg, rate: E.organ.rate, drive: E.organ.drive, tone: E.organ.tone, swell: E.organ.swell, click: E.organ.click }),
    wash: E.wash || (proto && proto.wash === 'r1' ? 'r1' : null),
  } : null;
  const cs = chordStats(spec);
  const r = render(spec);
  return {
    b64, stemsB64, stemNames, sketch, lead: leadOf(spec), seed: spec.seed, bpm: spec.bpm ?? null,
    meta: r.meta || {}, bars: r.totalSteps ? r.totalSteps / 16 : null,
    asked, refused, who, askedWho, yields, ms, ...cs,
    ears8: globalThis.EARS8_LOG.length ? globalThis.EARS8_LOG : null,
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
const globalRoot = process.env.PLAYWRIGHT_ROOT || execSync('npm root -g').toString().trim();
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
  const opts = { stems: !!j.stems, reserve: j.reserve || 0, thenSpec: j.then != null ? specs[j.then] : null, thenAt: j.thenAt ?? 30 };
  const r = await page.evaluate(([spec, sec, rate, proto, space, opts]) => window.renderOne(spec, sec, rate, proto, space, opts),
    [spec, j.seconds ?? 60, RATE, j.proto || {}, j.space || null, opts]);
  // 32-bit float WAV, so nothing clips or rounds before measuring.
  const wav = (b64, ch) => {
    const f32 = Buffer.from(b64, 'base64');
    const header = Buffer.alloc(44);
    header.write('RIFF', 0); header.writeUInt32LE(36 + f32.length, 4); header.write('WAVE', 8);
    header.write('fmt ', 12); header.writeUInt32LE(16, 16); header.writeUInt16LE(3, 20); header.writeUInt16LE(ch, 22);
    header.writeUInt32LE(RATE, 24); header.writeUInt32LE(RATE * 4 * ch, 28); header.writeUInt16LE(4 * ch, 32); header.writeUInt16LE(32, 34);
    header.write('data', 36); header.writeUInt32LE(f32.length, 40);
    return Buffer.concat([header, f32]);
  };
  const name = `${String(j.pick).padStart(3, '0')}-${r.lead}-${j.tag}.wav`;
  fs.writeFileSync(path.join(OUT, name), wav(r.b64, 2));
  if (r.stemsB64) fs.writeFileSync(path.join(OUT, name.replace('.wav', '-stems.wav')), wav(r.stemsB64, r.stemNames.length));
  delete r.b64;
  delete r.stemsB64;
  index.push({ file: name, pick: j.pick, tag: j.tag, seconds: j.seconds ?? 60, proto: j.proto || {}, space: j.space || null, then: j.then ?? null, ...r });
  // Written after every job, so a batch cut short keeps what it rendered.
  fs.writeFileSync(path.join(OUT, `index-${path.basename(JOBS, '.json')}.json`), JSON.stringify(index, null, 1));
  console.log(name, `refused ${r.refused}/${r.asked} ${JSON.stringify(r.who)}`, Object.keys(r.yields || {}).length ? `yields ${JSON.stringify(r.yields)}` : '', `chords/min ${r.changesPerMin?.toFixed(1)}`, `${Math.round(r.ms)} ms`);
}
fs.writeFileSync(path.join(OUT, `index-${path.basename(JOBS, '.json')}.json`), JSON.stringify(index, null, 1));
await browser.close();
server.close();
