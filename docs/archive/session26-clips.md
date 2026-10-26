# Session 26: the brain's clip renderer (for item 83)

The brain rendered Session 26's clips (`docs/ALBUMS.md`) with this script, in a scratch
worktree of `main` at v123. Item 83 builds `tools/clips.mjs` from it and reproduces
its forced clips sample for sample. Two changes in the scratch worktree only:

- `js/choices.js`, at the end of `drawChoices`, before `return out;`:
  `if (globalThis.FORCE_CHOICES) Object.assign(out, globalThis.FORCE_CHOICES);`
- for the bass clips, `LIFT_DB` in `core/src/blocks/bass.rs` set to 3.0 or 5.5 and
  `tools/build-core.sh` run in that worktree.

Run as `node clip.mjs WORKTREE JOBS.json OUTDIR [PORT]`, with Playwright 1.56.1.
It reproduces the hands' figure for undertow #45 with lateFloor forced (the floor
arrives at 0:29-0:30).

## The jobs

```json
[
 {"name":"bass-dust7-today","corpus":[20261009,7],"seconds":30,"layout":2,"drift":1},
 {"name":"bass-dust42-today","corpus":[20261009,42],"seconds":30,"layout":2,"drift":1},
 {"name":"bass-noannou-today","seed":4272732017,"seconds":30,"layout":2,"drift":1},
 {"name":"late-undertow45-forced","corpus":[20261009,45],"seconds":42,"layout":2,"drift":1,"force":{"lateFloor":"on"}},
 {"name":"pedal-dust69-forced","corpus":[20261009,69],"seconds":80,"from":50,"layout":2,"drift":1,"force":{"pedalBass":"on"}},
 {"name":"pedal-dust69-today","corpus":[20261009,69],"seconds":80,"from":50,"layout":2,"drift":1},
 {"name":"sweep-thaw71-forced","corpus":[20261009,71],"seconds":45,"layout":2,"drift":1,"force":{"padSweep":"on"}},
 {"name":"stutter-cinder93","corpus":[20261009,93],"seconds":130,"from":108,"layout":2,"drift":1},
 {"name":"longdrop-vapor4","corpus":[20261009,4],"seconds":110,"from":60,"layout":2,"drift":1},
 {"name":"sm1-s","corpus":[20261009,57],"seconds":30,"layout":2,"drift":1},
 {"name":"sm1-m","corpus":[20261009,57],"seconds":30,"layout":1,"drift":1},
 {"name":"sm2-s","corpus":[20261009,30],"seconds":30,"from":5,"layout":2,"drift":1},
 {"name":"sm2-m","corpus":[20261009,30],"seconds":30,"from":5,"layout":1,"drift":1},
 {"name":"sm3-s","corpus":[20261009,56],"seconds":30,"layout":2,"drift":1},
 {"name":"sm3-m","corpus":[20261009,56],"seconds":30,"layout":1,"drift":1},
 {"name":"sm4-s","corpus":[20261009,82],"seconds":30,"layout":2,"drift":1},
 {"name":"sm4-m","corpus":[20261009,82],"seconds":30,"layout":1,"drift":1},
 {"name":"sm5-s","corpus":[20261009,31],"seconds":30,"layout":2,"drift":1},
 {"name":"sm5-m","corpus":[20261009,31],"seconds":30,"layout":1,"drift":1},
 {"name":"sm6-s","corpus":[20261009,23],"seconds":30,"layout":2,"drift":1},
 {"name":"sm6-m","corpus":[20261009,23],"seconds":30,"layout":1,"drift":1},
 {"name":"sm7-m","corpus":[20261009,52],"seconds":24,"layout":1,"drift":1}
]
```

The bass jobs (run in each lifted worktree):

```json
[
 {"name":"dust7","corpus":[20261009,7],"seconds":30,"layout":2,"drift":1},
 {"name":"dust42","corpus":[20261009,42],"seconds":30,"layout":2,"drift":1},
 {"name":"noannou","seed":4272732017,"seconds":30,"layout":2,"drift":1}
]
```

## clip.mjs

```js
// Brain, Session 26: render listening clips through the app's real Engine and
// Synth on the core, in headless Chromium, to 16-bit WAV.
//
//   node clip.mjs REPO JOBS.json OUTDIR [PORT]
//
// JOBS.json: [{ "name": "t1", "seed": <loop seed> | "corpus": [corpusSeed, index],
//               "seconds": 60, "from": 0, "layout": 2, "drift": 1, "force": {"lateFloor": "on"} }]
// `from` trims the head of the clip (rendered from the start, written from `from`).
// `force` needs REPO's js/choices.js patched to read globalThis.FORCE_CHOICES.
import fs from 'node:fs';
import http from 'node:http';
import path from 'node:path';
import { createRequire } from 'node:module';

const [REPO, JOBS, OUT, PORT = '8790'] = process.argv.slice(2);
const ROOT = path.resolve(REPO);
const jobs = JSON.parse(fs.readFileSync(JOBS, 'utf8'));
fs.mkdirSync(OUT, { recursive: true });
const RATE = 48000;

const PAGE = `<!doctype html><meta charset="utf-8"><script type="module">
import { Engine } from '/js/engine.js';
import { Synth } from '/js/synth.js';
import { loadCore } from '/js/core.js';
import { newSpec, render } from '/js/generator.js';
import { Rng, mulberry32 } from '/js/rng.js';
window.specOf = (j) => {
  if (j.seed !== undefined) return newSpec(j.seed);
  const m = new Rng(j.corpus[0]);
  let s; for (let i = 0; i <= j.corpus[1]; i++) s = newSpec(m.seed32());
  return s;
};
window.renderOne = async (j, rate) => {
  globalThis.FORCE_CHOICES = j.force || null;
  const spec = window.specOf(j);
  const ctx = new OfflineAudioContext(2, Math.ceil(j.seconds * rate), rate);
  const core = await loadCore(ctx);
  Math.random = mulberry32(spec.seed >>> 0 || 1);
  const synth = new Synth(ctx, j.quality || 'full', { engine: 'rust', core, layout: j.layout || 2 });
  const engine = new Engine(ctx, synth);
  if (engine.clock.worker) engine.clock.worker.terminate();
  engine.driftOn = (j.drift ?? 1) > 0;
  engine.driftAmount = (j.drift ?? 1) > 0 ? j.drift ?? 1 : 1;
  engine.load(spec);
  engine.playing = true;
  engine.nextStepTime = 0.05;
  let guard = 0;
  while (engine.nextStepTime < j.seconds && guard++ < 400000) {
    engine._scheduleStep(engine.step, engine.nextStepTime);
    engine._advance();
  }
  await synth.core.flush();
  const buf = await ctx.startRendering();
  synth.core.dispose();
  const r = render(spec);
  const choices = Object.entries(r.choices).filter(([, v]) => v !== 'today').map(([k, v]) => v === 'on' ? k : k + ':' + v);
  const L = buf.getChannelData(0), R = buf.getChannelData(1);
  const from = Math.floor((j.from || 0) * rate);
  const n = L.length - from;
  const pcm = new Int16Array(n * 2);
  let peak = 0;
  for (let i = 0; i < n; i++) {
    const a = L[from + i], b = R[from + i];
    peak = Math.max(peak, Math.abs(a), Math.abs(b));
    pcm[2 * i] = Math.max(-32768, Math.min(32767, Math.round(a * 32767)));
    pcm[2 * i + 1] = Math.max(-32768, Math.min(32767, Math.round(b * 32767)));
  }
  let bin = '';
  const bytes = new Uint8Array(pcm.buffer);
  for (let i = 0; i < bytes.length; i += 0x8000) bin += String.fromCharCode.apply(null, bytes.subarray(i, i + 0x8000));
  return { b64: btoa(bin), peak, choices, profile: Object.entries(spec.mix || {}).sort((a, b) => b[1] - a[1])[0]?.[0], bars: spec.bars, bpm: spec.bpm, seed: spec.seed };
};
window.ready = true;
</script>`;

const TYPES = { '.html': 'text/html', '.js': 'text/javascript', '.mjs': 'text/javascript', '.json': 'application/json', '.wasm': 'application/wasm' };
const server = http.createServer((req, res) => {
  const url = decodeURIComponent(req.url.split('?')[0]);
  if (url === '/__clip.html') { res.writeHead(200, { 'content-type': 'text/html' }); return res.end(PAGE); }
  const file = path.join(ROOT, path.normalize(url));
  if (!file.startsWith(ROOT) || !fs.existsSync(file) || fs.statSync(file).isDirectory()) { res.writeHead(404); return res.end('nf'); }
  res.writeHead(200, { 'content-type': TYPES[path.extname(file)] || 'application/octet-stream' });
  fs.createReadStream(file).pipe(res);
});
await new Promise((ok) => server.listen(+PORT, '127.0.0.1', ok));
const require = createRequire('/home/claude/driftloom/package.json');
const { chromium } = require('playwright');
const browser = await chromium.launch({ args: ['--no-sandbox'] });
const page = await browser.newPage();
page.on('pageerror', (e) => console.error('pageerror', e.message));
await page.goto(`http://127.0.0.1:${PORT}/__clip.html`);
await page.waitForFunction(() => window.ready === true);

function wav(pcmBytes) {
  const h = Buffer.alloc(44);
  h.write('RIFF', 0); h.writeUInt32LE(36 + pcmBytes.length, 4); h.write('WAVE', 8);
  h.write('fmt ', 12); h.writeUInt32LE(16, 16); h.writeUInt16LE(1, 20); h.writeUInt16LE(2, 22);
  h.writeUInt32LE(RATE, 24); h.writeUInt32LE(RATE * 4, 28); h.writeUInt16LE(4, 32); h.writeUInt16LE(16, 34);
  h.write('data', 36); h.writeUInt32LE(pcmBytes.length, 40);
  return Buffer.concat([h, pcmBytes]);
}
const index = [];
for (const j of jobs) {
  const t0 = Date.now();
  const r = await page.evaluate(([job, rate]) => window.renderOne(job, rate), [j, RATE]);
  fs.writeFileSync(path.join(OUT, j.name + '.wav'), wav(Buffer.from(r.b64, 'base64')));
  delete r.b64;
  index.push({ ...j, ...r, peakDb: +(20 * Math.log10(r.peak || 1e-9)).toFixed(2) });
  console.log(j.name, r.profile, r.choices.join(' '), `peak ${(20 * Math.log10(r.peak)).toFixed(2)} dB`, `${((Date.now() - t0) / 1000).toFixed(1)} s`);
}
fs.writeFileSync(path.join(OUT, 'index.json'), JSON.stringify(index, null, 1));
await browser.close();
server.close();
```
