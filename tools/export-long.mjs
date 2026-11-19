// The long export's yardstick (queue item 104b, and 104c for `--format
// small`): a three-hour lossless (or small, Opus) export in the real app, in headless Chromium, on tools/autolite.mjs's slow
// case (`?burn=1700`: the Moto-class audio thread, every block the core
// renders made 1.7 ms slower), while the music plays.
//
//   node tools/export-long.mjs [--length 10800] [--burn 1700] [--lite on|off]
//                              [--format lossless|small] [--keep DIR] [--port 8772]
//
// The app starts as the Moto ends up in autolite's slow case: "lighter on
// processor" on (so the export goes gently), unless `--lite off`. There is no
// save picker (as on Android's Chrome), so the export writes the site's
// private file system as it goes and offers a download at the end. A loop
// that plays on and on (tools/stalls.mjs's shatter, pasted as a listener
// would) is exported for `--length` seconds (one of the sheet's lengths).
//
// Sampled every 30 s through the run: the browser's memory (the resident
// size of every Chromium process, from /proc), the export's progress, and
// what plays meanwhile: the core's late, missed and dropped notes
// (Diagnostics' engine line), the audio device's fill-ins
// (AudioContext.playoutStats, where Chromium has it) and the windows in
// which the audio thread fell behind (the worklet's busy reports). Then the
// file the app downloaded is checked by tools/export.mjs --file: lossless,
// it decodes and nulls against a straight render of the job the page sent;
// small, it decodes with no warning (ffmpeg), holds exactly the frames the
// Worker rendered, and is the straight render's samples to the sample
// within the coding error.
//
// The browser runs on a profile on disk, as a phone's does (an incognito
// profile keeps the private file in memory). Passes when: the export
// finishes; memory is flat (the last quarter's mean
// within 32 MB of the second quarter's, and no sample 96 MB over the
// second quarter's mean); nothing late, missed, dropped, filled in or
// behind; and the file nulls. Linux only (it reads /proc). Chromium from
// Playwright (npm install --no-save playwright@1.56.1).

import http from 'node:http';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';
import { LOOPS } from './stalls.mjs';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const argv = process.argv.slice(2);
const flag = (name, dflt) => {
  const i = argv.indexOf(name);
  return i >= 0 ? argv[i + 1] : dflt;
};
const LENGTH = Number(flag('--length', 10800));
const BURN = Number(flag('--burn', 1700));
const LITE = flag('--lite', 'on') !== 'off';
const SMALL = flag('--format', 'lossless') === 'small';
const RATE = SMALL ? 48000 : 44100;
const PORT = Number(flag('--port', 8772));
const KEEP = flag('--keep', null);
const EVERY = 30000;

function loadPlaywright() {
  const require = createRequire(import.meta.url);
  for (const id of ['playwright', 'playwright-core', '/opt/node22/lib/node_modules/playwright']) {
    try { return require(id); } catch { /* the next one */ }
  }
  console.error('export-long: Playwright is not installed.  npm install --no-save playwright@1.56.1');
  process.exit(3);
  return null;
}

const TYPES = { '.html': 'text/html', '.js': 'text/javascript', '.mjs': 'text/javascript', '.css': 'text/css', '.json': 'application/json', '.webmanifest': 'application/manifest+json', '.flac': 'audio/flac', '.opus': 'audio/ogg', '.wav': 'audio/wav', '.png': 'image/png', '.svg': 'image/svg+xml', '.wasm': 'application/wasm', '.woff2': 'font/woff2', '.txt': 'text/plain' };
function serve(port) {
  const server = http.createServer((req, res) => {
    const url = new URL(req.url, 'http://x');
    let file = path.join(ROOT, decodeURIComponent(url.pathname));
    if (file.endsWith('/')) file = path.join(file, 'index.html');
    if (!file.startsWith(ROOT) || !fs.existsSync(file)) {
      res.writeHead(404);
      res.end();
      return;
    }
    res.writeHead(200, { 'Content-Type': TYPES[path.extname(file)] || 'application/octet-stream', 'Cache-Control': 'no-store' });
    fs.createReadStream(file).pipe(res);
  });
  return new Promise((resolve) => server.listen(port, '127.0.0.1', () => resolve(server)));
}

// Before the page's scripts: the context, the worklet's busy reports, the
// export Worker's words and the job it was given; no save picker; the
// clipboard the harness's; eight cores (so only the burn makes it slow).
const HOOK = `(() => {
  const R = window.__long = { behind: 0, busiest: 0, exports: [], progress: null, start: null, clip: null };
  const Ctx = window.AudioContext;
  window.AudioContext = class extends Ctx { constructor(o) { super(o); R.ctx = this; } };
  const Node = window.AudioWorkletNode;
  window.AudioWorkletNode = class extends Node {
    constructor(ctx, name, options) {
      super(ctx, name, options);
      this.port.addEventListener('message', (e) => {
        const m = e.data;
        if (m && m.type === 'cost' && m.busy) { R.behind += m.busy.behind || 0; R.busiest = Math.max(R.busiest, m.busy.max || 0); }
      });
      this.port.start();
    }
  };
  const W = window.Worker;
  window.Worker = class extends W {
    constructor(url, options) {
      super(url, options);
      if (!String(url).includes('export-worker')) return;
      this.addEventListener('message', (e) => {
        const m = e.data;
        if (m && m.type === 'progress') R.progress = m;
        else if (m) R.exports.push({ type: m.type, why: m.why, message: m.message, bytes: m.bytes, frames: m.frames, seconds: m.seconds, took: m.took, counts: m.counts, stop: m.stop, fade: m.fade });
      });
      const post = this.postMessage;
      this.postMessage = (m, t) => {
        if (m && m.type === 'start') R.start = { job: m.job, sink: m.sink, pace: m.pace };
        return t ? post.call(this, m, t) : post.call(this, m);
      };
    }
  };
  window.showSaveFilePicker = undefined;
  Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { writeText: async (t) => { R.clip = t; }, readText: async () => R.clip || '' } });
  Object.defineProperty(Navigator.prototype, 'hardwareConcurrency', { get: () => 8 });
  R.fills = () => { try { const s = R.ctx && R.ctx.playoutStats; return s && typeof s.fallbackFramesEvents === 'number' ? s.fallbackFramesEvents : null; } catch { return null; } };
})();`;

// The resident memory of every Chromium process, in bytes.
function chromiumRss() {
  let total = 0;
  for (const pid of fs.readdirSync('/proc').filter((p) => /^\d+$/.test(p))) {
    try {
      const cmd = fs.readFileSync(`/proc/${pid}/cmdline`, 'utf8');
      if (!/chrom/i.test(cmd)) continue;
      const m = /VmRSS:\s+(\d+) kB/.exec(fs.readFileSync(`/proc/${pid}/status`, 'utf8'));
      if (m) total += Number(m[1]) * 1024;
    } catch { /* gone */ }
  }
  return total;
}

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const mb = (b) => (b / 1e6).toFixed(0);

async function main() {
  const { chromium } = loadPlaywright();
  const server = await serve(PORT);
  const out = KEEP || fs.mkdtempSync(path.join(os.tmpdir(), 'dlexport-long-'));
  fs.mkdirSync(out, { recursive: true });
  // A profile on disk (a persistent context): an incognito one, which
  // `newContext()` makes, keeps the site's storage in memory, so the private
  // file the export writes would count as the browser's memory (the first
  // CI run grew by exactly the file's 653 MB that way). A phone writes it
  // to disk.
  const profile = fs.mkdtempSync(path.join(os.tmpdir(), 'dlexport-profile-'));
  const context = await chromium.launchPersistentContext(profile, {
    args: ['--autoplay-policy=no-user-gesture-required', '--mute-audio'],
    acceptDownloads: true,
    viewport: { width: 412, height: 915 },
  });
  let failures = 0;
  const check = (label, ok, detail = '') => {
    console.log(`  ${ok ? 'pass' : 'FAIL'}  ${label}${detail ? `  (${detail})` : ''}`);
    if (!ok) failures++;
  };
  try {
    await context.addInitScript(HOOK);
    await context.addInitScript((lite) => { if (!localStorage.getItem('driftloom.prefs.v1')) localStorage.setItem('driftloom.prefs.v1', JSON.stringify({ lite })); }, LITE);
    const page = await context.newPage();
    const errors = [];
    page.on('pageerror', (e) => errors.push(e.message));
    await page.goto(`http://127.0.0.1:${PORT}/${BURN ? `?burn=${BURN}` : ''}`);
    await page.waitForFunction(() => document.readyState === 'complete' && document.getElementById('playBtn'));
    await sleep(1500);
    const ev = (fn, arg) => page.evaluate(fn, arg);
    await ev((code) => { window.__long.clip = code; }, LOOPS.shatter.code);
    await ev(() => document.getElementById('pasteCode').click());
    await sleep(4000);
    const diag = () => ev(() => { document.getElementById('diagRefresh').click(); return document.getElementById('diagOut').textContent; });
    const engine = async () => {
      const line = (await diag()).split('\n').find((l) => l.startsWith('engine:')) || '';
      const n = (k) => Number((new RegExp(`${k}: (\\d+)`).exec(line) || [])[1] ?? NaN);
      return { line, late: n('late'), missed: n('missed'), dropped: n('dropped') };
    };
    // The export is as long as the track (session 30c): set in the Track
    // length view, to the second, as a listener sets it.
    const set = await ev((t) => {
      document.getElementById('vibesBtn').click();
      document.querySelector('#picksList .pk-track').click();
      const n = document.getElementById('trkNum');
      const two = (x) => String(x).padStart(2, '0');
      n.value = `${Math.floor(t / 3600)}:${two(Math.floor((t % 3600) / 60))}:${two(t % 60)}`;
      n.dispatchEvent(new Event('blur'));
      document.getElementById('trkDone').click();
      return document.querySelector('#picksList .pk-track').textContent;
    }, LENGTH);
    await ev(() => {
      if (document.getElementById('viewSave').hidden) document.getElementById('saveBtn').click();
      document.querySelector('[data-stab="share"]').click();
    });
    await sleep(1500);
    if (SMALL) {
      const small = await ev(() => { const b = document.querySelector('[data-format="small"]'); if (b && !b.hidden) b.click(); return !!b && !b.hidden; });
      if (!small) throw new Error('the sheet does not offer Small in this browser');
    }
    if (set === 'plays on') throw new Error(`no track length of ${LENGTH} s`);
    const before = await engine();
    const fills0 = await ev(() => window.__long.fills());
    const behind0 = await ev(() => window.__long.behind);
    const download = page.waitForEvent('download', { timeout: 0 });
    console.log(`export-long: ${LENGTH / 3600} hours of ${LOOPS.shatter.name}, ${SMALL ? 'small (Opus)' : 'lossless'}, burn ${BURN} us, lighter on processor ${LITE ? 'on' : 'off'}`);
    await ev(() => document.getElementById('exportAudio').click());
    const t0 = Date.now();
    const samples = [];
    let end = null;
    while (!end) {
      await sleep(EVERY);
      const s = await ev(() => ({ progress: window.__long.progress, exports: window.__long.exports, behind: window.__long.behind, busiest: window.__long.busiest, fills: window.__long.fills() }));
      const e = await engine();
      const rss = chromiumRss();
      const minutes = s.progress ? s.progress.frames / RATE / 60 : 0;
      samples.push({ at: (Date.now() - t0) / 1000, minutes, rss, late: e.late, fills: s.fills, behind: s.behind });
      console.log(`  ${((Date.now() - t0) / 60000).toFixed(1).padStart(6)} min: ${minutes.toFixed(1).padStart(6)} min exported, memory ${mb(rss)} MB, late ${e.late} missed ${e.missed} dropped ${e.dropped}, fill-ins ${s.fills ?? '-'}, behind ${s.behind - behind0}, busiest ${(100 * s.busiest).toFixed(0)}%`);
      end = s.exports.find((x) => ['done', 'failed', 'cancelled'].includes(x.type)) || null;
      if (Date.now() - t0 > 6 * 3600 * 1000) throw new Error('six hours and not done');
    }
    const took = (Date.now() - t0) / 1000;
    const after = await engine();
    const fills1 = await ev(() => window.__long.fills());
    const behind1 = await ev(() => window.__long.behind);
    const start = await ev(() => window.__long.start);
    const playing = await ev(() => document.getElementById('playBtn').getAttribute('aria-label'));
    check('the export finished', end.type === 'done', `${end.type} ${end.why || ''} ${end.message || ''}`);
    if (end.type === 'done') console.log(`  ${(end.seconds / 3600).toFixed(2)} hours, ${mb(end.bytes)} MB, in ${(took / 60).toFixed(0)} min: ${(end.seconds / end.took).toFixed(1)}x real time (${start.pace})`);
    // Memory: the second quarter against the last.
    const q = Math.floor(samples.length / 4);
    const mean = (xs) => xs.reduce((a, b) => a + b, 0) / Math.max(1, xs.length);
    const second = mean(samples.slice(q, 2 * q).map((s) => s.rss));
    const last = mean(samples.slice(3 * q).map((s) => s.rss));
    const peak = Math.max(...samples.slice(q).map((s) => s.rss));
    check('memory flat through the run', samples.length >= 8 && last - second < 32e6 && peak - second < 96e6, `${samples.length} samples; second quarter ${mb(second)} MB, last ${mb(last)} MB, peak ${mb(peak)} MB`);
    check('nothing late, missed or dropped in what played meanwhile', after.late === before.late && after.missed === before.missed && after.dropped === before.dropped, `${before.line} -> ${after.line}`);
    check('no fill-ins, the audio thread never behind', (fills0 == null || fills1 === fills0) && behind1 === behind0, `fill-ins ${fills0 ?? '-'} -> ${fills1 ?? '-'}, behind ${behind1 - behind0}`);
    check('the music still playing', playing === 'Pause', playing);
    check('no page errors', errors.length === 0, errors.join('; '));
    if (end.type === 'done') {
      const d = await download;
      const file = path.join(out, SMALL ? 'export.opus' : 'export.flac');
      await d.saveAs(file);
      const job = path.join(out, 'export.json');
      fs.writeFileSync(job, JSON.stringify({ job: start.job, stop: end.stop, fade: end.fade, frames: end.frames }));
      check('the download is the file the Worker wrote', fs.statSync(file).size === end.bytes, `${fs.statSync(file).size} bytes`);
      const r = spawnSync('node', [path.join(ROOT, 'tools', 'export.mjs'), '--file', file, '--job', job], { stdio: 'inherit' });
      check(SMALL ? 'the file decodes with no warning, holds the frames the Worker rendered, and is the straight render within the coding error (tools/export.mjs --file)' : 'the file decodes and nulls against a straight render (tools/export.mjs --file)', r.status === 0);
      if (!KEEP) fs.rmSync(out, { recursive: true, force: true });
    }
  } finally {
    await context.close();
    server.close();
    fs.rmSync(profile, { recursive: true, force: true });
  }
  console.log(failures ? `\nexport-long: ${failures} failed` : '\nexport-long: every check passed');
  process.exit(failures ? 1 : 0);
}

await main();
