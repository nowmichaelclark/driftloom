// The player's reference (queue item 82): loops from their seeds, played in
// headless Chromium by the app's own Engine and Synth on the core, handing
// their notes to the core as the core's player does, and fingerprinted.
//
//   node tools/player.mjs [options]
//
//   --n N            the first N loops of the corpus           (default 40)
//   --seed S         the corpus's seed; Rng(S).seed32() a loop  (default 1)
//   --only I,J       only these loops of it (0 is the first)
//   --seconds S      how long each render is                    (default 45)
//   --rate HZ        the sample rate                       (default 44100)
//   --wander A       how far each loop wanders, 0 for off        (default 1)
//   --request TEXT   what each loop is asked for, as the request's text
//                    (`dlrender --request`; queue item 85b)
//   --quality Q      full, lite, or both                      (default both)
//   --layout L       mono, stereo, or both                    (default both)
//   --json FILE      write every row as JSON
//   --calls DIR      write each render's calls to the core, in the order they
//                    were made, to DIR/NAME.QUALITY.LAYOUT.calls, to set
//                    beside the player's (`dlrender --seed ... --calls`)
//   --upfront        render each loop a second time handing every note over
//                    before the render starts, as tools/measure.mjs does, and
//                    say how far its samples are from the player's way's
//   --chrome PATH, --port N, --jobs N (renders at once, default 4)
//
// Why it hands notes over as it does. The measure harness (tools/measure.mjs)
// decides a whole render before it starts and hands all of it to the core at
// once; the core keeps its waiting notes in no fixed order (a note that
// starts takes the last one's place), so which voice a note takes, and so the
// order the voices are summed in, depends on everything still waiting. A
// player cannot know the future: before each block it renders it decides the
// steps due within its lookahead, and hands those over. So this harness
// renders the same way. Before the block at frame F every step whose time is
// under F / rate + LOOKAHEAD is decided (Engine._scheduleStep, as the measure
// harness calls it, with the context's clock at F), and handed to the core
// while the render waits there (OfflineAudioContext.suspend); then the
// render goes on. The player (core/src/player/) decides the same steps before
// the same blocks, so the two render the same notes into the same core in the
// same order, and must make the same samples. LOOKAHEAD is the player's
// (`player::LOOKAHEAD`).
//
// Each render is a fresh engine on a fresh synth (as tools/measure.mjs
// makes them) with wander on at the amount asked, loaded at time 0, its first
// step at 0.05 s. Its fingerprint is FNV-1a over the words of the output's
// float samples (tools/measure.mjs, `hashOf`): `out` for a mono render, `left`
// and `right` for a stereo one, the hashes tools/baseline.mjs keeps (its
// `player` part). Every render must play everything on the core: a note that
// fell back to the JavaScript synth, or reached the core late, fails the run.
//
// Needs Playwright and Chromium, like tools/measure.mjs.

import http from 'node:http';
import fs from 'node:fs';
import path from 'node:path';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
// The player's lookahead, in seconds (core/src/player/mod.rs, LOOKAHEAD).
export const LOOKAHEAD = 0.25;

const argv = process.argv.slice(2);
if (argv.includes('--help') || argv.includes('-h')) {
  console.log(fs.readFileSync(fileURLToPath(import.meta.url), 'utf8').split('\n').filter((l) => l.startsWith('//')).map((l) => l.slice(3)).join('\n'));
  process.exit(0);
}
const flag = (name, dflt) => {
  const i = argv.indexOf(name);
  return i >= 0 ? argv[i + 1] : dflt;
};
const opts = {
  n: Number(flag('--n', 40)),
  seed: Number(flag('--seed', 1)) >>> 0,
  only: flag('--only', null),
  seconds: Number(flag('--seconds', 45)),
  rate: Number(flag('--rate', 44100)),
  wander: Number(flag('--wander', 1)),
  request: flag('--request', null),
  qualities: { full: ['full'], lite: ['lite'], both: ['full', 'lite'] }[flag('--quality', 'both')],
  layouts: { mono: [1], stereo: [2], both: [1, 2] }[flag('--layout', 'both')],
  json: flag('--json', null),
  calls: flag('--calls', null),
  upfront: argv.includes('--upfront'),
  port: Number(flag('--port', 8763)),
  chrome: flag('--chrome', null),
  jobs: Math.max(1, Number(flag('--jobs', 4))),
};
if (!opts.qualities || !opts.layouts || !(opts.seconds > 0) || !(opts.n > 0)) {
  console.error('player: --quality is full, lite or both; --layout mono, stereo or both; --seconds and --n positive');
  process.exit(2);
}

// ------------------------------------------------------------- the page

const PAGE = `<!doctype html><meta charset="utf-8"><title>player</title>
<script type="module">
import { Engine } from '/js/engine.js';
import { Synth } from '/js/synth.js';
import { loadCore, CoreHost } from '/js/core.js';
import { newSpec } from '/js/generator.js';
import { mulberry32 } from '/js/rng.js';

const QUANTUM = 128;
const LOOKAHEAD = ${LOOKAHEAD};

// tools/measure.mjs's fingerprint: FNV-1a over the float samples' words.
function hashOf(d) {
  const words = new Uint32Array(d.buffer, d.byteOffset, d.length);
  let h = 0x811c9dc5;
  for (let i = 0; i < words.length; i++) { h ^= words[i]; h = Math.imul(h, 0x01000193); }
  return (h >>> 0).toString(16).padStart(8, '0');
}

// A number as the player's call lists write it (core/src/player, Trace): a
// whole number in decimal, NaN 'nan', anything else 'x' and its bits.
const bitsView = new DataView(new ArrayBuffer(8));
function num(x) {
  if (Number.isNaN(x)) return 'nan';
  if (Number.isInteger(x) && Math.abs(x) <= 2 ** 53) return Object.is(x, -0) ? '-0' : String(x);
  bitsView.setFloat64(0, x);
  return 'x' + bitsView.getBigUint64(0).toString(16).padStart(16, '0');
}
const nums = (xs) => xs.map(num).join(' ');

// Every call the host makes to the core, as text, while \`lines\` is set.
let lines = null;
const was = {};
for (const name of ['note', 'damp', 'param', 'set', 'event', 'retune', 'route', 'bassLift', 'cancel', 'spaceRange']) was[name] = CoreHost.prototype[name];
CoreHost.prototype.note = function (voice, channel, time, midi, dur, vel, parts, extra = []) {
  if (!lines) return was.note.apply(this, arguments);
  // The key is asked for once: a note without one is numbered by the asking.
  const k = this.keyOf();
  lines.push('key ' + nums([k.seed, k.pass, k.step, k.ord]));
  lines.push('note ' + nums([voice, channel, time, midi, dur, vel, parts, ...Array.from({ length: 12 }, (_, i) => extra[i] ?? NaN)]));
  const keyOf = this.keyOf;
  this.keyOf = () => k;
  try {
    return was.note.apply(this, arguments);
  } finally {
    this.keyOf = keyOf;
  }
};
CoreHost.prototype.damp = function (channel, time) {
  if (lines) lines.push('damp ' + nums([channel, time]));
  return was.damp.apply(this, arguments);
};
CoreHost.prototype.param = function (id, op, value, time, tau = 0) {
  if (lines) lines.push('param ' + nums([id, op, value, time, tau]));
  return was.param.apply(this, arguments);
};
CoreHost.prototype.set = function (channel, block, setting, value, time) {
  if (lines) lines.push('set ' + nums([channel, block, setting, value, time]));
  return was.set.apply(this, arguments);
};
CoreHost.prototype.event = function (channel, kind, time, value = 0) {
  if (lines) lines.push('event ' + nums([channel, kind, time, value]));
  return was.event.apply(this, arguments);
};
CoreHost.prototype.retune = function (channel, time, notes) {
  if (lines) lines.push('retune ' + nums([channel, time, ...Array.from({ length: 6 }, (_, i) => notes[i] ?? NaN)]));
  return was.retune.apply(this, arguments);
};
CoreHost.prototype.route = function (channel, block, setting, from, time) {
  if (lines) lines.push('route ' + nums([channel, block, setting, from == null ? 4294967295 : from, time]));
  return was.route.apply(this, arguments);
};
CoreHost.prototype.bassLift = function (db) {
  if (lines) lines.push('lift ' + nums([db]));
  return was.bassLift.apply(this, arguments);
};
CoreHost.prototype.spaceRange = function (which, lo, hi) {
  if (lines) lines.push('space ' + nums([which, lo, hi]));
  return was.spaceRange.apply(this, arguments);
};
CoreHost.prototype.cancel = function () {
  throw new Error('the synth called cancel in a player render');
};

// One loop, one way: what it plays, as the player hands it over (or, with
// \`upfront\`, every note handed over before the render starts).
async function play({ seed, request = null, seconds, rate, quality, layout, wander, calls, upfront = false, keep = false }) {
  const frames = Math.ceil(seconds * rate);
  const ctx = new OfflineAudioContext(2, frames, rate);
  const core = await loadCore(ctx);
  const random = Math.random;
  // The synth builds its JavaScript noise as it starts, which the core's
  // notes never play; after that nothing may draw from the platform.
  Math.random = mulberry32((seed >>> 0) || 1);
  const own = calls ? [] : null;
  lines = own;
  let synth;
  let engine;
  try {
    synth = new Synth(ctx, quality, { engine: 'rust', core, layout });
    if (!synth.core) throw new Error('the synth has no core: ' + synth.coreError);
    engine = new Engine(ctx, synth);
    if (engine.clock.worker) engine.clock.worker.terminate();
    engine.driftOn = wander > 0;
    if (wander > 0) engine.driftAmount = wander;
    Math.random = () => { throw new Error('Math.random in the sound path'); };
    engine.load(newSpec(seed, request));
    engine.playing = true;
    engine.nextStepTime = 0.05;
  } finally {
    Math.random = random;
    lines = null;
  }
  // Before the block at frame f: every step due before f / rate + LOOKAHEAD.
  const due = (f) => engine.nextStepTime < f / rate + LOOKAHEAD;
  const decide = (f) => {
    lines = own;
    if (own) own.push('at ' + f);
    Math.random = () => { throw new Error('Math.random in the sound path'); };
    try {
      let guard = 0;
      while (due(f) && guard++ < 100000) {
        engine._scheduleStep(engine.step, engine.nextStepTime);
        engine._advance();
      }
    } finally {
      Math.random = random;
      lines = null;
    }
  };
  const after = (f) => {
    let g = f + QUANTUM;
    while (g < frames && !due(g)) g += QUANTUM;
    return g;
  };
  let failure = null;
  const hop = async (f) => {
    try {
      const at = Math.round(ctx.currentTime * rate);
      if (at !== f) throw new Error('the render waited at frame ' + at + ', not ' + f);
      decide(f);
      await synth.core.flush();
      const g = after(f);
      if (g < frames) ctx.suspend(g / rate).then(() => hop(g));
    } catch (err) {
      failure = failure || err;
    }
    ctx.resume();
  };
  if (upfront) {
    decide(Math.ceil(seconds * rate) + Math.ceil(LOOKAHEAD * rate));
  } else {
    decide(0);
    const first = after(0);
    if (first < frames) ctx.suspend(first / rate).then(() => hop(first));
  }
  await synth.core.flush();
  const buf = await ctx.startRendering();
  const counts = await synth.core.flush();
  synth.core.dispose();
  if (failure) throw failure;
  const hashes = layout === 2
    ? { left: hashOf(buf.getChannelData(0)), right: hashOf(buf.getChannelData(1)) }
    : { out: hashOf(buf.getChannelData(0)) };
  return {
    hashes, frames, fallbacks: synth.fallbacks, late: counts.late, missed: counts.missed, dropped: counts.dropped,
    name: engine.spec.name, bars: engine.spec.bars, bpm: engine.spec.bpm, calls: own ? own.join('\\n') + '\\n' : null,
    buf: keep ? buf : null,
  };
}

// How far the up-front render's samples are from the player's way's: the
// frames that differ, the first, and the largest difference and the
// difference's whole level, in dB under full scale and under the render.
async function upfrontOf(job) {
  const a = await play({ ...job, keep: true, upfront: false });
  const b = await play({ ...job, keep: true, upfront: true });
  let differ = 0, first = -1, most = 0, sum = 0, level = 0, n = 0;
  for (let c = 0; c < a.buf.numberOfChannels && c < job.layout; c++) {
    const x = a.buf.getChannelData(c), y = b.buf.getChannelData(c);
    for (let i = 0; i < x.length; i++) {
      const d = Math.abs(x[i] - y[i]);
      if (d > 0) {
        differ++;
        if (first < 0 || i < first) first = i;
      }
      most = Math.max(most, d);
      sum += d * d;
      level += x[i] * x[i];
      n++;
    }
  }
  const db = (v) => (v > 0 ? 20 * Math.log10(v) : -Infinity);
  return {
    hashes: a.hashes, upfront: b.hashes, frames: a.frames, name: a.name, fallbacks: a.fallbacks + b.fallbacks,
    late: a.late + b.late, missed: a.missed + b.missed, dropped: a.dropped + b.dropped,
    differ, first: first < 0 ? null : first / job.rate, most, mostDb: db(most),
    underDb: sum > 0 ? db(Math.sqrt(sum / n)) - db(Math.sqrt(level / n)) : -Infinity,
  };
}

window.playLoops = async (jobs, width) => {
  const out = new Array(jobs.length);
  let next = 0;
  const worker = async () => {
    while (next < jobs.length) {
      const i = next++;
      try {
        out[i] = { ...jobs[i], ...(await (jobs[i].upfront ? upfrontOf(jobs[i]) : play(jobs[i]))) };
      } catch (err) {
        out[i] = { ...jobs[i], error: String((err && err.stack) || err) };
      }
    }
  };
  await Promise.all(Array.from({ length: width }, worker));
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
  console.error('player: Playwright is not installed.  npm install -g playwright && npx playwright install chromium');
  process.exit(3);
  return null;
}

const TYPES = { '.html': 'text/html', '.js': 'text/javascript', '.mjs': 'text/javascript', '.json': 'application/json', '.wasm': 'application/wasm', '.txt': 'text/plain' };
const server = http.createServer((req, res) => {
  const url = decodeURIComponent(req.url.split('?')[0]);
  if (url === '/__player.html') {
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
  server.listen(opts.port, '127.0.0.1', resolve);
});
const { chromium } = loadPlaywright();
const browser = await chromium.launch({ executablePath: opts.chrome || undefined, args: ['--no-sandbox', '--autoplay-policy=no-user-gesture-required'] });
const page = await browser.newPage();
const errors = [];
page.on('pageerror', (e) => errors.push(e.message));
page.on('crash', () => { console.error('player: the page crashed (out of memory?)'); process.exit(1); });
async function load() {
  await page.goto(`http://127.0.0.1:${opts.port}/__player.html`);
  await page.waitForFunction(() => window.ready === true, null, { timeout: 20000 });
}

// ------------------------------------------------------------- the run

// The corpus as tools/measure.mjs and tools/gen-golden.mjs draw it.
const { Rng } = await import('../js/rng.js');
const master = new Rng(opts.seed);
const seeds = Array.from({ length: opts.n }, () => master.seed32());
const picked = opts.only ? opts.only.split(',').map(Number) : seeds.map((_, i) => i);
const jobs = [];
for (const i of picked) {
  for (const quality of opts.qualities) {
    for (const layout of opts.layouts) {
      jobs.push({ index: i, seed: seeds[i], request: opts.request, seconds: opts.seconds, rate: opts.rate, quality, layout, wander: opts.wander, calls: !!opts.calls, upfront: opts.upfront });
    }
  }
}
// Chromium keeps every offline render that loaded the core until the page
// goes away (tools/measure.mjs, `inPages`): a fresh page every few renders.
const BATCH = 8;
const rows = [];
for (let i = 0; i < jobs.length; i += BATCH) {
  await load();
  rows.push(...(await page.evaluate(([j, w]) => window.playLoops(j, w), [jobs.slice(i, i + BATCH), opts.jobs])));
  process.stderr.write(`player: ${Math.min(jobs.length, i + BATCH)} of ${jobs.length} renders\r`);
}
process.stderr.write('\n');
await browser.close();
server.close();

let bad = 0;
for (const r of rows) {
  const label = `${r.index}`.padStart(3) + ` ${String(r.name || '?').padEnd(15)} ${r.quality.padEnd(4)} ${r.layout === 2 ? 'stereo' : 'mono  '}`;
  if (r.error) {
    bad++;
    console.log(`${label}  FAILED  ${r.error.split('\n')[0]}`);
    continue;
  }
  const problems = [];
  if (r.fallbacks) problems.push(`${r.fallbacks} note(s) played in JavaScript`);
  if (r.late || r.missed) problems.push(`${r.late} late, ${r.missed} missed`);
  if (problems.length) bad++;
  const upfront = r.upfront
    ? (r.differ
      ? `  up front: ${r.differ} of ${r.frames * r.layout} samples differ, from ${r.first.toFixed(3)} s; the most by ${r.most.toExponential(2)} (${r.mostDb.toFixed(1)} dBFS); in all ${(-r.underDb).toFixed(1)} dB under the render`
      : '  up front: the same samples')
    : '';
  console.log(`${label}  ${Object.entries(r.hashes).map(([k, v]) => `${k}=${v}`).join(' ')}${r.dropped ? `  dropped ${r.dropped}` : ''}${upfront}${problems.length ? `  FAILS: ${problems.join('; ')}` : ''}`);
  if (opts.calls && r.calls) {
    fs.mkdirSync(opts.calls, { recursive: true });
    fs.writeFileSync(path.join(opts.calls, `${r.index}-${r.name}.${r.quality}.${r.layout === 2 ? 'stereo' : 'mono'}.calls`), r.calls);
  }
}
if (opts.json) {
  fs.writeFileSync(opts.json, `${JSON.stringify(rows.map(({ calls, ...r }) => r), null, 1)}\n`);
  console.error(`player: wrote ${opts.json}`);
}
if (errors.length) console.error(`player: errors on the page:\n  ${errors.join('\n  ')}`);
process.exit(bad || errors.length ? 1 : 0);
