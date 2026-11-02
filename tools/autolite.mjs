// "Lighter on processor" chosen by the app (queue item 87b), live: the real
// app in headless Chromium, playing in real time, its audio thread made
// slower with `?burn=` (microseconds of work added to every block the core
// renders; the DevTools CPU throttle slows only the page's thread, not the
// audio's).
//
//   node tools/autolite.mjs [--burn 1700] [--near 850] [--minutes 10] [--only NAME]
//
// The cases, each in a fresh profile:
//
//   slow       burn: full's busiest quarter-seconds pass the line, and the app
//              moves to lite by itself within the rule's window and a pass,
//              with no late note, and Diagnostics says why; reloaded, it
//              starts on lite, still its own choice.
//   steady     no burn, --minutes of playing with Next pressed each minute:
//              full stays full.
//   yours      burn, with the listener's own choice of full stored: it stays
//              full (and the same for lite chosen, with no burn).
//   near       a burn that puts full's busiest about the line (--near: on the
//              machine measured, full's own busiest is 15-25% of a block, so
//              850 us more at 44.1 kHz puts it near 50%), for --minutes: at
//              most one move.

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
const BURN = Number(flag('--burn', 1700));
const MINUTES = Number(flag('--minutes', 10));
const NEAR = Number(flag('--near', 850));
const ONLY = flag('--only', null);
const PREFS = 'driftloom.prefs.v1';

function loadPlaywright() {
  const require = createRequire(import.meta.url);
  for (const id of ['playwright', 'playwright-core', '/opt/node22/lib/node_modules/playwright']) {
    try { return require(id); } catch { /* try the next one */ }
  }
  console.error('autolite: Playwright is not installed.  npm install --no-save playwright@1.56.1 && npx playwright install chromium');
  process.exit(3);
  return null;
}

const TYPES = {
  '.html': 'text/html', '.js': 'text/javascript', '.mjs': 'text/javascript', '.css': 'text/css', '.json': 'application/json',
  '.webmanifest': 'application/manifest+json', '.wav': 'audio/wav', '.flac': 'audio/flac', '.png': 'image/png',
  '.svg': 'image/svg+xml', '.wasm': 'application/wasm', '.woff2': 'font/woff2', '.txt': 'text/plain',
};
const server = http.createServer((req, res) => {
  let url = decodeURIComponent(req.url.split('?')[0]);
  if (url.endsWith('/')) url += 'index.html';
  const file = path.join(ROOT, path.normalize(url));
  if (!file.startsWith(ROOT) || !fs.existsSync(file) || fs.statSync(file).isDirectory()) {
    res.writeHead(404);
    return res.end('not found');
  }
  res.writeHead(200, { 'content-type': TYPES[path.extname(file)] || 'application/octet-stream' });
  return fs.createReadStream(file).pipe(res);
});
await new Promise((resolve, reject) => { server.once('error', reject); server.listen(0, '127.0.0.1', resolve); });
const BASE = `http://127.0.0.1:${server.address().port}/`;
const { chromium } = loadPlaywright();
const browser = await chromium.launch({ args: ['--no-sandbox', '--autoplay-policy=no-user-gesture-required', '--mute-audio'] });
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

let failures = 0;
function check(label, ok, detail = '') {
  console.log(ok ? `  pass  ${label}` : `  FAIL  ${label} ${detail}`);
  if (!ok) failures++;
}

// A fresh profile on the app, with `prefs` stored first, playing. The page
// is told it has eight cores, so the first guess starts it on full (the
// machine running this may have four, which starts it on lite).
async function open({ burn = 0, prefs = null, cores = 8 }) {
  const context = await browser.newContext();
  await context.addInitScript((n) => { Object.defineProperty(Navigator.prototype, 'hardwareConcurrency', { get: () => n }); }, cores);
  if (prefs) await context.addInitScript(([k, v]) => { if (!localStorage.getItem(k)) localStorage.setItem(k, v); }, [PREFS, JSON.stringify(prefs)]);
  const page = await context.newPage();
  await page.goto(`${BASE}?${burn ? `burn=${burn}` : ''}`);
  await page.waitForSelector('#playBtn');
  // The core in before Play, so the whole run is on it.
  await sleep(1500);
  await page.click('#playBtn');
  return { context, page, burn };
}
async function diag(page) {
  return page.evaluate(() => {
    document.getElementById('diagRefresh').click();
    return document.getElementById('diagOut').textContent;
  });
}
const prefsOf = (page) => page.evaluate((k) => JSON.parse(localStorage.getItem(k) || '{}'), PREFS);
const lateOf = (text) => {
  const m = text.match(/^engine: .*?late: (\d+)/m);
  return m ? Number(m[1]) : null;
};
const line = (text, key) => (text.match(new RegExp(`^${key}: .*$`, 'm')) || [''])[0];

async function slow() {
  console.log(`\nslow: burn ${BURN} us a block`);
  const t = await open({ burn: BURN });
  const t0 = Date.now();
  let moved = null;
  while (Date.now() - t0 < 150000) {
    await sleep(2000);
    const p = await prefsOf(t.page);
    if (p.liteAuto) { moved = { ...p.liteAuto, after: (Date.now() - t0) / 1000 }; break; }
  }
  check('it moves to lite by itself', moved && moved.lite === true, JSON.stringify(moved));
  if (moved) console.log(`        after ${moved.after.toFixed(0)} s: ${moved.why}`);
  await sleep(8000);
  const text = await diag(t.page);
  console.log(`        ${line(text, 'cores')}\n        ${line(text, 'busiest')}`);
  check('Diagnostics says why', /lite: true, auto \(busiest \d+% at full\)/.test(text) || /lite: true, auto \(fell behind/.test(text), line(text, 'cores'));
  const playing = await t.page.evaluate(() => document.getElementById('playBtn').getAttribute('aria-label'));
  check('it plays on, on lite', /\(lite;/.test(line(text, 'busiest')) && playing === 'Pause', `${line(text, 'busiest')}, ${playing}`);
  check('no note late', lateOf(text) === 0, `${lateOf(text)}: ${line(text, 'engine')}`);
  const shown = await t.page.evaluate(() => { const h = document.getElementById('liteAuto'); return h && !h.hidden ? h.textContent : null; });
  check('the switch\'s line says it was set by itself', !!shown, shown);
  await t.page.reload();
  await t.page.waitForSelector('#playBtn');
  await sleep(500);
  const again = await diag(t.page);
  check('reloaded, it starts on lite, its own choice', /lite: true, auto \(/.test(again), line(again, 'cores'));
  await t.context.close();
}

async function steady() {
  console.log(`\nsteady: no burn, ${MINUTES} minutes, Next each minute`);
  const t = await open({});
  for (let m = 0; m < MINUTES; m++) {
    await sleep(60000);
    await t.page.click('#nextBtn');
  }
  const text = await diag(t.page);
  console.log(`        ${line(text, 'cores')}\n        ${line(text, 'busiest')}`);
  const p = await prefsOf(t.page);
  check('full stays full', !p.liteAuto && /lite: false/.test(text), JSON.stringify(p.liteAuto));
  check('no note late', lateOf(text) === 0, `${lateOf(text)}`);
  await t.context.close();
}

async function yours() {
  console.log(`\nyours: the listener's own choice, burn ${BURN} us`);
  const t = await open({ burn: BURN, prefs: { lite: false } });
  await sleep(75000);
  let p = await prefsOf(t.page);
  let text = await diag(t.page);
  console.log(`        ${line(text, 'busiest')}`);
  check('full, chosen, stays full however busy', !p.liteAuto && /lite: false, yours/.test(text), line(text, 'cores'));
  await t.context.close();
  const u = await open({ prefs: { lite: true } });
  await sleep(75000);
  p = await prefsOf(u.page);
  text = await diag(u.page);
  check('lite, chosen, stays lite however quiet', !p.liteAuto && /lite: true, yours/.test(text), line(text, 'cores'));
  await u.context.close();
}

async function near() {
  // About the line: the measured busiest hovers around it.
  const burn = NEAR;
  console.log(`\nnear: burn ${burn} us, ${MINUTES} minutes`);
  const t = await open({ burn });
  const moves = new Set();
  const seen = [];
  for (let s = 0; s < MINUTES * 60; s += 10) {
    await sleep(10000);
    const p = await prefsOf(t.page);
    if (p.liteAuto) moves.add(p.liteAuto.at);
    if (s % 60 === 50) {
      const m = line(await diag(t.page), 'busiest').match(/busiest: (\S+) over 20 s/);
      if (m) seen.push(m[1]);
    }
  }
  const text = await diag(t.page);
  console.log(`        ${line(text, 'cores')}\n        ${line(text, 'busiest')}`);
  console.log(`        the busiest over 20 s, each minute: ${seen.join(' ')}`);
  check('at most one move', moves.size <= 1, `${moves.size}`);
  await t.context.close();
}

const cases = { slow, steady, yours, near };
try {
  for (const [name, fn] of Object.entries(cases)) if (!ONLY || ONLY.split(',').includes(name)) await fn();
} finally {
  await browser.close();
  server.close();
}
console.log(failures ? `\n${failures} failed` : '\nall passed');
process.exit(failures ? 1 : 0);
