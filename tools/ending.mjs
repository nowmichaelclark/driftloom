// The length slider's end of a track (queue item 88b), rendered: the real
// Synth and Engine on the core, in an OfflineAudioContext in headless
// Chromium, so where a track ends and how it lets go can be measured.
//
//   node tools/ending.mjs [--n 4] [--minutes 5] [--bars 1024]
//
// For each loop of the corpus (seed 1, as tools/measure.mjs draws it, asked
// for `--bars` bars; a loop whose pass is shorter than the slider's time is
// skipped), stereo: the slider at `--minutes` (`spec.playBars`, as the app's
// slider stores a time shorter than one pass) and wander on. The track plays
// from 0.05 s until the engine hands over; then, rendered three ways from the
// same start:
//
//   end    where it handed over: the first phrase line past the time (a line
//          of four bars from its section's start, else two, else the
//          section's end), checked against the bars counted independently.
//   fade   the next loop never comes: the old loop's level over the 0.25 s
//          before the line, then from the line plus END_FADE plus 0.25 s
//          (its notes gone, only the reverb's and the echo's last) and the
//          same with the fade switched off (`fadeNotes` a no-op): the fade
//          takes the notes ringing over the line away.
//   join   the next loop (its seed's own, nothing asked) takes its first
//          step on the line, as the app hands over: the largest step between
//          two samples from 0.1 s before the line to a second after, against
//          the largest the render takes anywhere else (its own attacks): a
//          click shows as a step larger than the loop's own.
//   late   notes the core took late or missed, and dropped.
//
// And for the loop's own length (`--cycle`, the first `--n` corpus loops
// asked for 14 bars, wander off, the slider off): three and a half passes
// rendered, never handed over, and the render's own cycle measured: the lag
// in bars (1 to 32) at which its level envelope (10 ms frames) best matches
// itself over the last two passes is 14.
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
const N = Number(flag('--n', 4));
const MINUTES = Number(flag('--minutes', 5));
const BARS = Number(flag('--bars', 1024));
const CYCLE = Number(flag('--cycle', 14));

function loadPlaywright() {
  const require = createRequire(import.meta.url);
  for (const id of ['playwright', 'playwright-core', '/opt/node22/lib/node_modules/playwright']) {
    try { return require(id); } catch { /* try the next one */ }
  }
  console.error('ending: Playwright is not installed.  npm install --no-save playwright@1.56.1 && npx playwright install chromium');
  process.exit(3);
  return null;
}

const PAGE = `<!doctype html><meta charset="utf-8"><title>ending</title>
<script type="module">
import { Synth } from '/js/synth.js';
import { Engine, END_FADE } from '/js/engine.js';
import { loadCore } from '/js/core.js';
import { newSpec } from '/js/generator.js';
import { Rng } from '/js/rng.js';

const RATE = 48000;
const START = 0.05;

// Every step up to \`end\`, as the engine decides them; at a hand-over,
// \`next\` (the app's next loop, loaded at once) or nothing.
function decide(engine, end, next) {
  let guard = 0;
  while (engine.nextStepTime < end && guard++ < 1e7) {
    if (engine.handover) {
      if (!next || engine.spec === next) return;
      engine.load(next);
      continue;
    }
    engine._scheduleStep(engine.step, engine.nextStepTime);
    engine._advance();
  }
}

// Plays \`spec\` from START to \`seconds\` on a fresh core; returns the render,
// where it handed over, and the core's counts.
async function play(spec, seconds, { next = null, fade = true, drift = true } = {}) {
  const ctx = new OfflineAudioContext(2, Math.ceil(seconds * RATE), RATE);
  const core = await loadCore(ctx);
  const synth = new Synth(ctx, 'full', { engine: 'rust', core, layout: 2 });
  if (!fade) synth.fadeNotes = () => {};
  const engine = new Engine(ctx, synth);
  if (engine.clock.worker) engine.clock.worker.terminate();
  engine.driftOn = drift;
  engine.load(spec);
  engine.playing = true;
  engine.nextStepTime = START;
  let end = null;
  let endStep = null;
  engine.onTrackEnd = () => { end = engine.nextStepTime; endStep = engine.absStep; engine._letGo(); };
  // Decided as the app does, a lookahead ahead of the render (a whole long
  // track handed over up front would overflow the core's queue), the
  // render suspended each second while the next is decided.
  const state = { next };
  const ahead = (t) => {
    decide(engine, Math.min(seconds, t + 2), state.next);
  };
  ahead(0);
  await synth.core.flush();
  for (let t = 1; t < seconds; t++) {
    ctx.suspend(t).then(async () => {
      ahead(t);
      await synth.core.flush();
      ctx.resume();
    });
  }
  const buf = await ctx.startRendering();
  const counts = await synth.core.flush();
  synth.core.dispose();
  return { buf, end, endStep, counts, engine };
}

function rms(buf, a, b) {
  let s = 0;
  let n = 0;
  for (let c = 0; c < 2; c++) {
    const d = buf.getChannelData(c);
    for (let k = Math.max(0, Math.floor(a * RATE)); k < Math.min(d.length, Math.floor(b * RATE)); k++) { s += d[k] * d[k]; n++; }
  }
  return n ? Math.sqrt(s / n) : 0;
}
const db = (x) => (x > 0 ? 20 * Math.log10(x) : -Infinity);

window.ending = async (n, minutes, bars) => {
  const rng = new Rng(1);
  const out = [];
  let tried = 0;
  while (out.length < n && tried++ < 200) {
    const seed = rng.seed32();
    const spec = newSpec(seed, 'bars=' + bars);
    const bar = (spec.stepsPerBar || 16) * (60 / spec.bpm / 4);
    const secs = minutes * 60;
    if (bars * bar <= secs) continue;
    spec.playBars = Math.max(1, Math.ceil(secs / bar - 1e-9));
    const seconds = START + secs + 4 * 4 * bar + END_FADE + 6;
    const alone = await play(spec, seconds);
    if (alone.end == null) { out.push({ seed, name: spec.name, error: 'never handed over' }); continue; }
    const at = alone.end;
    // The bars counted here, not by the engine: the first line past the
    // time, of four bars (two, or the section) from the section's start.
    const spb = spec.stepsPerBar || 16;
    const section = spec.bars;
    const unit = section % 4 === 0 ? 4 : section % 2 === 0 ? 2 : section;
    const played = (at - START) / bar;
    const want = (() => {
      const from = spec.playBars;
      const k = Math.floor(from / section);
      const into = from - k * section;
      return k * section + Math.min(Math.ceil(into / unit) * unit, section);
    })();
    const ringing = await play(spec, seconds, { fade: false });
    const after = [at + END_FADE + 0.25, at + END_FADE + 1.25];
    const joined = await play(spec, at + 3, { next: newSpec(seed ^ 0x5eed) });
    let join = 0;
    let own = 0;
    for (let c = 0; c < 2; c++) {
      const d = joined.buf.getChannelData(c);
      const j0 = Math.floor((at - 0.1) * RATE);
      const j1 = Math.ceil((at + 1) * RATE);
      for (let k = Math.floor(START * RATE) + 1; k < d.length; k++) {
        const s = Math.abs(d[k] - d[k - 1]);
        if (k >= j0 && k <= j1) join = Math.max(join, s);
        else own = Math.max(own, s);
      }
    }
    out.push({
      seed, name: spec.name, section, playBars: spec.playBars, secs, at, played, want,
      before: db(rms(alone.buf, at - 0.25, at)), faded: db(rms(alone.buf, ...after)), rang: db(rms(ringing.buf, ...after)),
      join, own,
      late: alone.counts.late + joined.counts.late, missed: (alone.counts.missed || 0) + (joined.counts.missed || 0), dropped: alone.counts.dropped + joined.counts.dropped,
    });
  }
  return out;
};

// The loop's own cycle, from the render: the lag in bars at which the level
// envelope best matches itself.
window.cycle = async (n, bars) => {
  const rng = new Rng(1);
  const out = [];
  for (let i = 0; i < n; i++) {
    const seed = rng.seed32();
    const spec = newSpec(seed, 'bars=' + bars);
    const bar = (spec.stepsPerBar || 16) * (60 / spec.bpm / 4);
    const pass = bars * bar;
    const seconds = START + 3.5 * pass;
    const r = await play(spec, seconds, { drift: false });
    const passes = r.engine.loopCount;
    const frame = Math.round(0.01 * RATE);
    const env = [];
    const d0 = r.buf.getChannelData(0);
    const d1 = r.buf.getChannelData(1);
    for (let k = 0; k + frame <= d0.length; k += frame) {
      let s = 0;
      for (let j = k; j < k + frame; j++) s += d0[j] * d0[j] + d1[j] * d1[j];
      env.push(Math.sqrt(s / frame));
    }
    const fpb = bar / 0.01;
    const from = Math.floor((START + 1.5 * pass) / 0.01);
    let best = 0;
    let bestLag = 0;
    const scores = [];
    for (let lag = 1; lag <= 32; lag++) {
      const L = Math.round(lag * fpb);
      let ab = 0, aa = 0, bb = 0;
      for (let k = from; k + L < env.length; k++) { ab += env[k] * env[k + L]; aa += env[k] * env[k]; bb += env[k + L] * env[k + L]; }
      const score = aa && bb ? ab / Math.sqrt(aa * bb) : 0;
      scores.push(score);
      if (score > best + 1e-9) { best = score; bestLag = lag; }
    }
    out.push({ seed, name: spec.name, bars: spec.bars, passes, handedOver: r.end != null, bestLag, best, at14: scores[bars - 1] });
  }
  return out;
};
window.ready = true;
</script>`;

const TYPES = { '.html': 'text/html', '.js': 'text/javascript', '.mjs': 'text/javascript', '.wasm': 'application/wasm', '.txt': 'text/plain' };
const server = http.createServer((req, res) => {
  const url = decodeURIComponent(req.url.split('?')[0]);
  if (url === '/__ending.html') {
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
  await page.goto(`http://127.0.0.1:${server.address().port}/__ending.html`);
  await page.waitForFunction(() => window.ready === true, null, { timeout: 30000 }).catch(() => {
    console.error(`ending: the page never loaded.\n  ${errors.join('\n  ')}`);
    process.exit(1);
  });
  page.setDefaultTimeout(0);
  const rows = await page.evaluate(({ n, minutes, bars }) => window.ending(n, minutes, bars), { n: N, minutes: MINUTES, bars: BARS });
  for (const r of rows) {
    if (r.error) { console.log(`${r.name}: ${r.error}`); failed = true; continue; }
    const onLine = Math.abs(r.played - r.want) < 1e-6;
    const ratio = r.own > 0 ? r.join / r.own : 0;
    console.log(`${r.name.padEnd(14)} ${BARS} bars of ${r.section}, ${r.playBars} bars set (${r.secs} s): ended at ${r.at.toFixed(2)} s, bar ${r.played.toFixed(2)} (want ${r.want})`
      + `; before the line ${r.before.toFixed(1)} dB, ${r.faded.toFixed(1)} dB after the fade against ${r.rang.toFixed(1)} dB unfaded`
      + `; join step ${r.join.toFixed(4)} against ${r.own.toFixed(4)} (${ratio.toFixed(2)}x); late ${r.late}, missed ${r.missed}, dropped ${r.dropped}`);
    if (!onLine || r.played < r.playBars - 1e-6 || r.faded > r.rang + 0.5 || ratio > 1 || r.late + r.missed > 0) failed = true;
  }
  const cycles = await page.evaluate(({ n, bars }) => window.cycle(n, bars), { n: N, bars: CYCLE });
  for (const c of cycles) {
    console.log(`${c.name.padEnd(14)} ${CYCLE} bars asked, ${c.bars} made, slider off: ${c.passes} passes, ${c.handedOver ? 'handed over' : 'never handed over'}; the render's cycle ${c.bestLag} bars (match ${c.best.toFixed(4)}; at ${CYCLE}: ${c.at14.toFixed(4)})`);
    if (c.handedOver || c.passes < 3 || c.bestLag % CYCLE !== 0 && c.at14 < c.best - 1e-3) failed = true;
  }
  console.log(failed ? 'ending: FAILED' : 'ending: all good');
} finally {
  await browser.close();
  server.close();
}
process.exit(failed ? 1 : 0);
