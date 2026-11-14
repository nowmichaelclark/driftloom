// The Android app's smoke test (queue item 107a): the installed app starts,
// its page loads, Play is pressed, and the page's Diagnostics shows the
// core's player playing, with no error. Headless: it drives the app's own
// WebView through the Chrome DevTools protocol, which a debuggable (test)
// build offers over adb (MainActivity turns it on). No test library, no
// accessibility tree: the page is read as the listener's page is.
//
//   node android/smoke.mjs                  the app on the one device or
//                                           emulator adb sees (installed)
//   node android/smoke.mjs --cdp http://127.0.0.1:9222
//                                           no adb: a DevTools endpoint that
//                                           is already up (a desktop Chromium
//                                           on the same files: how the
//                                           script itself is tried)
//   --package ID    the app's id (default: android/gradle.properties's)
//   --shot FILE     a screenshot of the page after the check (a PNG)
//   --timeout S     seconds to wait for each step (default 90)
//
// Needs Node 22 (its WebSocket) and, for the device, adb on the path.

import { execFileSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const HERE = path.dirname(fileURLToPath(import.meta.url));
const argv = process.argv.slice(2);
const opt = (name, fallback) => {
  const i = argv.indexOf(name);
  return i >= 0 && argv[i + 1] ? argv[i + 1] : fallback;
};
const STEP_MS = Number(opt('--timeout', '90')) * 1000;
const props = fs.readFileSync(path.join(HERE, 'gradle.properties'), 'utf8');
const PACKAGE = opt('--package', (props.match(/^driftloom\.applicationId=(.+)$/m) || [])[1]);
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const log = (...a) => console.log(...a);

function adb(...args) {
  return execFileSync('adb', args, { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }).trim();
}

async function until(what, fn, ms = STEP_MS) {
  const end = Date.now() + ms;
  let last;
  while (Date.now() < end) {
    try {
      const v = await fn();
      if (v) return v;
    } catch (e) { last = e; }
    await sleep(500);
  }
  throw new Error(`timed out (${ms / 1000} s) waiting for ${what}${last ? `: ${last.message}` : ''}`);
}

// ---- Reach the app's page ------------------------------------------------

async function endpoint() {
  const given = opt('--cdp', '');
  if (given) return given;
  if (!PACKAGE) throw new Error('no package id: pass --package');
  log(`starting ${PACKAGE}`);
  adb('shell', 'monkey', '-p', PACKAGE, '-c', 'android.intent.category.LAUNCHER', '1');
  const pid = await until('the app to run', () => adb('shell', 'pidof', PACKAGE).split(/\s+/)[0]);
  const socket = `webview_devtools_remote_${pid}`;
  await until('the WebView to open its inspector', () => adb('shell', 'cat', '/proc/net/unix').includes(socket));
  const port = adb('forward', 'tcp:0', `localabstract:${socket}`);
  return `http://127.0.0.1:${port}`;
}

class Page {
  constructor(ws) {
    this.ws = ws;
    this.next = 1;
    this.waiting = new Map();
    this.problems = [];
    ws.addEventListener('message', (m) => {
      const msg = JSON.parse(m.data);
      if (msg.id && this.waiting.has(msg.id)) {
        const { ok, fail } = this.waiting.get(msg.id);
        this.waiting.delete(msg.id);
        msg.error ? fail(new Error(msg.error.message)) : ok(msg.result);
      } else if (msg.method === 'Runtime.exceptionThrown') {
        const d = msg.params.exceptionDetails;
        this.problems.push(`uncaught: ${(d.exception && d.exception.description) || d.text}`);
      } else if (msg.method === 'Runtime.consoleAPICalled' && msg.params.type === 'error') {
        this.problems.push(`console.error: ${msg.params.args.map((a) => a.value ?? a.description).join(' ')}`);
      } else if (msg.method === 'Log.entryAdded' && msg.params.entry.level === 'error') {
        this.problems.push(`log: ${msg.params.entry.text} ${msg.params.entry.url || ''}`);
      }
    });
  }
  send(method, params = {}) {
    const id = this.next++;
    this.ws.send(JSON.stringify({ id, method, params }));
    return new Promise((ok, fail) => this.waiting.set(id, { ok, fail }));
  }
  // `gesture`: counts as a tap, which is what lets the page start its audio.
  async eval(expression, gesture = false) {
    const r = await this.send('Runtime.evaluate', { expression, returnByValue: true, awaitPromise: true, userGesture: gesture });
    if (r.exceptionDetails) throw new Error(`in the page: ${r.exceptionDetails.exception?.description || r.exceptionDetails.text}`);
    return r.result.value;
  }
}

const base = await endpoint();
log(`DevTools at ${base}`);
const target = await until('the page', async () => {
  const list = await (await fetch(`${base}/json/list`)).json();
  return list.find((t) => t.type === 'page' && /^https?:\/\/(appassets\.androidplatform\.net|localhost|127\.0\.0\.1)/.test(t.url));
});
log(`page: ${target.url}`);
const ws = new WebSocket(target.webSocketDebuggerUrl);
await new Promise((ok, fail) => { ws.onopen = ok; ws.onerror = () => fail(new Error('the DevTools socket would not open')); });
const page = new Page(ws);
await page.send('Runtime.enable');
await page.send('Log.enable');
// What the page logged before the socket opened is read again from a reload,
// so nothing the start does is missed (the page's time origin says it is the
// new document that is answering).
const origin = await page.eval('performance.timeOrigin');
await page.send('Page.reload', { ignoreCache: true });
await until('the page to reload', async () => (await page.eval('performance.timeOrigin')) !== origin);

const diag = () => page.eval("document.getElementById('diagRefresh').click(); document.getElementById('diagOut').textContent");
const checks = [];
const check = (name, ok, detail = '') => {
  checks.push(ok);
  log(`${ok ? 'ok  ' : 'FAIL'} ${name}${detail ? `: ${detail}` : ''}`);
};
let text = '';
try {
  await until('the page to be built', () => page.eval("document.readyState === 'complete' && !!document.getElementById('playBtn') && !!document.getElementById('diagOut')"));
  text = await until('the core to load', async () => {
    const t = await diag();
    return /^core: loaded/m.test(t) || /^core: not loaded/m.test(t) ? t : '';
  });
  const line = (t, key) => (t.split('\n').find((l) => l.startsWith(`${key}:`)) || '').trim();
  check('the core loaded', /^core: loaded/m.test(text), line(text, 'core'));
  check('no plain line in the card\'s place', await page.eval("document.getElementById('plainLine').hidden"));

  await page.eval("document.getElementById('playBtn').click()", true);
  text = await until('the player to play', async () => {
    const t = await diag();
    const m = /player: playing, step (\d+)/.exec(line(t, 'engine'));
    return m && Number(m[1]) > 0 ? t : '';
  });
  const first = Number(/player: playing, step (\d+)/.exec(line(text, 'engine'))[1]);
  await sleep(4000);
  text = await diag();
  const second = Number((/player: playing, step (\d+)/.exec(line(text, 'engine')) || [])[1] || 0);
  const engine = line(text, 'engine');
  check('Diagnostics shows the core\'s player', /^engine: rust \(the core's player\)/.test(engine), engine);
  check('no error in the engine line', !/none \(|error|fail/i.test(engine));
  check('the player plays on (its step moves)', second !== first, `step ${first}, then ${second}`);
  check('the page has a loop named', (await page.eval("document.getElementById('loopName').textContent.trim()")).length > 1);
} catch (e) {
  check('the check ran to its end', false, e.message);
  try { text = text || (await diag()); } catch { /* the page may be gone */ }
}

log('\n---- Diagnostics ----');
log(text);
log('---------------------');
if (page.problems.length) log(`the page logged ${page.problems.length} problem(s):\n  ${page.problems.join('\n  ')}`);
const shot = opt('--shot', '');
if (shot) {
  try {
    const r = await page.send('Page.captureScreenshot', { format: 'png' });
    fs.writeFileSync(shot, Buffer.from(r.data, 'base64'));
    log(`screenshot: ${shot}`);
  } catch (e) { log(`no screenshot: ${e.message}`); }
}
check('no uncaught error in the page', !page.problems.some((p) => p.startsWith('uncaught')), page.problems.filter((p) => p.startsWith('uncaught')).join('; '));
ws.close();
const failed = checks.filter((c) => !c).length;
log(failed ? `\n${failed} check(s) failed` : '\nthe app\'s page plays');
process.exit(failed ? 1 : 0);
