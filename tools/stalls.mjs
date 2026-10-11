// Stalls while hidden (queue item 39): the real app, playing in headless
// Chromium with the tab hidden and the CPU throttled, while the clock's
// ticks are held back on purpose -- what a locked phone does to a page when
// it throttles its worker, parks its CPU or freezes its renderer for a
// moment -- and what a listener would hear.
//
// Run with:  node tools/stalls.mjs [options]
//
// Nothing in js/ is touched. The stalls are made from outside, by a script
// the harness runs before the page's own (as tools/core-paths.mjs injects
// its traps), so nothing here ships:
//
//   ticks      the page's one Worker is its clock (js/clock.js). Its
//              messages are dropped while a stall is on, as a throttled
//              worker would send none. Each stall pattern runs `--each`
//              times: "now and then" (a stall, then 6 s of normal ticks) and
//              "back to back" (stalls with one tick between them).
//   main       the main thread kept busy for the stall's length (a renderer
//              starved of CPU): the ticks queue up and arrive together.
//   freeze     the page frozen (Page.setWebLifecycleState) for the stall's
//              length, where Chromium allows it.
//
// What is counted, per stall pattern (each from its own start to 4 s after
// its last stall), as the listener has it:
//
//   gap ms       the longest the page went without a tick (the stall, seen).
//   late ticks   the engine's own counter (Diagnostics `lateTicks`,
//                `worstLateMs`): the scheduler woke after a step it should
//                already have queued, so its steps were skipped.
//   late notes   notes handed to the synth after their start time: on the
//                core, notes posted to the worklet (it starts them at once,
//                late and bunched); on ?engine=js, distinct source start
//                times already past (Web Audio starts them at once).
//   silent s     music that never sounded: gaps between notes that touch a
//                stall (or the second after it), beyond the longest gap in
//                the same loop unstalled (the control run). Rests the loop
//                has anyway can still land there, so read it beside late
//                ticks: with none, nothing was skipped.
//   fill-ins     frames the audio device had to fill (AudioContext
//                playoutStats): the audio thread itself ran late.
//
// Options:
//   --loops <names>     shatter,cinder (default), from tools/perf.mjs's set
//   --engines <list>    core,js                              (default both)
//   --throttle <list>   CPU slowdown factors                 (default 4,6)
//   --lengths <list>    stall lengths in seconds        (default 0.5,1,2,5)
//   --modes <list>      ticks,main,freeze                    (default all)
//   --each <n>          stalls per pattern                      (default 3)
//   --quality <q>       full or lite                         (default full)
//   --json <file>  --md <file>  --port <n>  --chrome <path>
//
// Chromium only, over the DevTools protocol (Playwright keeps every tab
// visible), with the Chromium Playwright installs, or --chrome.

import http from 'node:http';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');

// From tools/perf.mjs's fixed set: the busiest loops.
const LOOPS = {
  shatter: { name: 'shatter-glein-fei', code: 'DL1-0C088-1GE08-60511-HBCQY-R4DR4-V7RDR-ND7HF-X1MQ6-VHDRA-009XE-Z9G08-8ZW0G-5ZXKW-TCREH-SH000-00000-V4G0' },
  cinder: { name: 'cinder-da-yoan', code: 'DL1-0C076-20110-60H1R-PE1MF-FVF35-ZPPA7-DDQPN-9D5MS-SBK94-FAREY-9KW08-EZW10-CDR3S-2QAD0-WEFGB-00000-0006S-3R' },
  undertow: { name: 'undertow-vith-glour', code: 'DL1-0C066-0R240-80QFF-STBB1-S7BSN-XARZ1-AG3PA-793NP-FN4X2-HKYN6-DS210-9T420-Y3G51-WD020-7ZR2W-BGV5B-5G000-08000-04001-00080-00000-2B5R' },
};

function parseArgs(argv) {
  const o = {
    loops: ['shatter', 'cinder'], engines: ['core', 'js'], throttle: [4, 6], lengths: [0.5, 1, 2, 5],
    modes: ['ticks', 'main', 'freeze'], each: 3, quality: 'full', port: 8753, chrome: null, json: null, md: null,
  };
  const list = (s) => s.split(',').map((x) => x.trim()).filter(Boolean);
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    const next = () => argv[++i];
    switch (a) {
      case '--loops': o.loops = list(next()); break;
      case '--engines': o.engines = list(next()); break;
      case '--throttle': o.throttle = list(next()).map(Number); break;
      case '--lengths': o.lengths = list(next()).map(Number); break;
      case '--modes': o.modes = list(next()); break;
      case '--each': o.each = Number(next()); break;
      case '--quality': o.quality = next(); break;
      case '--port': o.port = Number(next()); break;
      case '--chrome': o.chrome = next(); break;
      case '--json': o.json = next(); break;
      case '--md': o.md = next(); break;
      case '--help': case '-h':
        console.log(fs.readFileSync(fileURLToPath(import.meta.url), 'utf8').split('\n\n')[2]);
        process.exit(0);
        break;
      default: console.error(`stalls: unknown option ${a}`); process.exit(2);
    }
  }
  for (const l of o.loops) if (!LOOPS[l]) { console.error(`stalls: no loop ${l} (${Object.keys(LOOPS).join(', ')})`); process.exit(2); }
  return o;
}

// ------------------------------------------------------------- chromium

function findChrome(explicit) {
  if (explicit) return explicit;
  const roots = [process.env.PLAYWRIGHT_BROWSERS_PATH, '/opt/pw-browsers', path.join(os.homedir(), '.cache', 'ms-playwright')].filter(Boolean);
  for (const root of roots) {
    if (!fs.existsSync(root)) continue;
    for (const d of fs.readdirSync(root).filter((x) => /^chromium-\d+$/.test(x)).sort().reverse()) {
      for (const rel of ['chrome-linux/chrome', 'chrome-linux64/chrome', 'chrome-mac/Chromium.app/Contents/MacOS/Chromium']) {
        const p = path.join(root, d, rel);
        if (fs.existsSync(p)) return p;
      }
    }
  }
  console.error('stalls: no Chromium found; pass --chrome <path>');
  process.exit(3);
  return null;
}

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

async function launch(chromePath) {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'driftloom-stalls-'));
  const proc = spawn(chromePath, [
    '--headless=new', '--no-sandbox', '--remote-debugging-port=0', `--user-data-dir=${dir}`,
    '--no-first-run', '--no-default-browser-check', '--disable-extensions',
    '--disable-component-update', '--disable-sync', '--mute-audio',
    '--enable-blink-features=AudioContextPlayoutStats',
    'about:blank',
  ], { stdio: ['ignore', 'ignore', 'pipe'] });
  const wsUrl = await new Promise((resolve, reject) => {
    let buf = '';
    const timer = setTimeout(() => reject(new Error('Chromium did not start')), 20000);
    proc.stderr.on('data', (d) => {
      buf += d;
      const m = buf.match(/DevTools listening on (ws:\/\/\S+)/);
      if (m) { clearTimeout(timer); resolve(m[1]); }
    });
    proc.on('exit', (code) => reject(new Error(`Chromium exited (${code})`)));
  });
  const ws = new WebSocket(wsUrl);
  await new Promise((resolve, reject) => { ws.onopen = resolve; ws.onerror = reject; });
  let id = 0;
  const pending = new Map();
  ws.onmessage = (m) => {
    const d = JSON.parse(m.data);
    if (d.id && pending.has(d.id)) {
      const { res, rej, method } = pending.get(d.id);
      pending.delete(d.id);
      if (d.error) rej(new Error(`${method}: ${d.error.message}`));
      else res(d.result);
    }
  };
  const send = (method, params = {}, sessionId) => new Promise((res, rej) => {
    const i = ++id;
    pending.set(i, { res, rej, method });
    ws.send(JSON.stringify({ id: i, method, params, sessionId }));
  });
  const close = async () => {
    try { await Promise.race([send('Browser.close'), sleep(3000)]); } catch { /* gone */ }
    try { proc.kill('SIGKILL'); } catch { /* gone */ }
    try { ws.close(); } catch { /* gone */ }
    try { fs.rmSync(dir, { recursive: true, force: true }); } catch { /* best effort */ }
  };
  return { send, close };
}

const TYPES = {
  '.html': 'text/html', '.js': 'text/javascript', '.mjs': 'text/javascript', '.css': 'text/css',
  '.json': 'application/json', '.webmanifest': 'application/manifest+json', '.wav': 'audio/wav',
  '.flac': 'audio/flac', '.png': 'image/png', '.wasm': 'application/wasm',
};
function startServer(port) {
  const server = http.createServer((req, res) => {
    let url = decodeURIComponent(req.url.split('?')[0]);
    if (url.endsWith('/')) url += 'index.html';
    const file = path.join(ROOT, path.normalize(url));
    if (!file.startsWith(ROOT) || !fs.existsSync(file) || fs.statSync(file).isDirectory()) {
      res.writeHead(404);
      return res.end('not found');
    }
    res.writeHead(200, { 'content-type': TYPES[path.extname(file)] || 'application/octet-stream', 'cache-control': 'no-store' });
    return fs.createReadStream(file).pipe(res);
  });
  return new Promise((resolve, reject) => {
    server.once('error', reject);
    server.listen(port, '127.0.0.1', () => resolve(server));
  });
}

// ------------------------------------------------------ the test-only hook

// Runs before the page's own scripts; never shipped. The first loop is
// seeded as tools/perf.mjs seeds it.
const INSTRUMENT = `(() => {
  const native = Math.random;
  let a = 20260926;
  Math.random = function () {
    a |= 0; a = (a + 0x6d2b79f5) | 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
  document.addEventListener('DOMContentLoaded', () => { Math.random = native; }, { once: true });
  const S = window.__stalls = { ctx: null, ticks: [], dropped: 0, until: 0, plan: null, notes: [], starts: [], windows: [] };
  // Each stall, on the context's clock, from its start to its end.
  S.window = (ms) => { if (S.ctx) S.windows.push([S.ctx.currentTime, S.ctx.currentTime + ms / 1000]); };
  const RealCtx = window.AudioContext;
  window.AudioContext = new Proxy(RealCtx, {
    construct(t, args, n) { const c = Reflect.construct(t, args, n); S.ctx = c; return c; },
  });
  // The clock's worker. Its ticks are dropped while a stall is on; a plan
  // starts the next stall once its gap has passed and a tick got through.
  const RealWorker = window.Worker;
  window.Worker = new Proxy(RealWorker, {
    construct(t, args, n) {
      const w = Reflect.construct(t, args, n);
      let handler = null;
      Object.defineProperty(w, 'onmessage', { configurable: true, get: () => handler, set: (fn) => { handler = fn; } });
      w.addEventListener('message', (e) => {
        const now = performance.now();
        if (now < S.until) { S.dropped++; return; }
        S.ticks.push(now);
        if (handler) handler.call(w, e);
        const p = S.plan;
        if (p && p.left > 0 && now - p.lastEnd >= p.gapMs) {
          p.left--;
          S.until = now + p.ms;
          S.window(p.ms);
          p.lastEnd = S.until;
        }
      });
      return w;
    },
  });
  S.stall = (ms, count, gapMs) => {
    S.plan = { ms, left: count, gapMs, lastEnd: -Infinity };
    return true;
  };
  // Notes posted to the core, and sources started in Web Audio: when each
  // was asked for, and when it was handed over, on the context's clock.
  const post = MessagePort.prototype.postMessage;
  MessagePort.prototype.postMessage = function (m, ...rest) {
    if (m && m.type === 'note' && S.ctx) S.notes.push([m.time, S.ctx.currentTime]);
    return post.call(this, m, ...rest);
  };
  const start = AudioScheduledSourceNode.prototype.start;
  AudioScheduledSourceNode.prototype.start = function (when = 0, ...rest) {
    if (S.ctx && this.context === S.ctx && when > 0) S.starts.push([when, S.ctx.currentTime]);
    return start.call(this, when, ...rest);
  };
  S.fills = () => {
    try { return S.ctx && S.ctx.playoutStats ? S.ctx.playoutStats.toJSON().fallbackFramesEvents : null; } catch { return null; }
  };
  S.mark = () => ({ now: performance.now(), ctx: S.ctx ? S.ctx.currentTime : 0, ticks: S.ticks.length, notes: S.notes.length, starts: S.starts.length, windows: S.windows.length, fills: S.fills() });
  S.since = (m) => ({
    ticks: S.ticks.slice(Math.max(0, m.ticks - 1)), notes: S.notes.slice(m.notes), starts: S.starts.slice(m.starts),
    windows: S.windows.slice(m.windows),
    ctx: S.ctx ? S.ctx.currentTime : 0, fills: S.fills(),
  });
})();
//# sourceURL=stalls-harness.js`;

// --------------------------------------------------------------- one tab

async function openTab(b, url, throttle) {
  const { targetInfos } = await b.send('Target.getTargets');
  const target = targetInfos.find((t) => t.type === 'page');
  const { sessionId } = await b.send('Target.attachToTarget', { targetId: target.targetId, flatten: true });
  const send = (m, p) => b.send(m, p, sessionId);
  const evaluate = async (expression) => {
    const r = await send('Runtime.evaluate', { expression, returnByValue: true, awaitPromise: true });
    if (r.exceptionDetails) throw new Error(`page: ${r.exceptionDetails.exception?.description || r.exceptionDetails.text}`);
    return r.result.value;
  };
  await send('Page.enable');
  await send('Runtime.enable');
  await send('Page.addScriptToEvaluateOnNewDocument', { source: INSTRUMENT });
  if (throttle > 1) await send('Emulation.setCPUThrottlingRate', { rate: throttle });
  await send('Page.navigate', { url });
  await evaluate(`new Promise((r) => (function wait() {
    document.getElementById('playBtn') && document.readyState === 'complete' ? r(true) : setTimeout(wait, 50);
  })())`);
  const press = async (selector) => {
    const box = await evaluate(`(() => {
      const r = document.querySelector(${JSON.stringify(selector)}).getBoundingClientRect();
      return { x: r.left + r.width / 2, y: r.top + r.height / 2 };
    })()`);
    const base = { x: box.x, y: box.y, button: 'left', clickCount: 1, pointerType: 'mouse' };
    await send('Input.dispatchMouseEvent', { type: 'mousePressed', ...base });
    await send('Input.dispatchMouseEvent', { type: 'mouseReleased', ...base });
  };
  const diagnostics = async (reset = false) => {
    const text = await evaluate(`(() => {
      ${reset ? "document.getElementById('diagReset').click();" : ''}
      document.getElementById('diagRefresh').click();
      return document.getElementById('diagOut').textContent;
    })()`);
    const out = { text };
    for (const line of text.split('\n')) {
      const m = line.match(/^([A-Za-z]+):\s*(.*)$/);
      if (m && !(m[1] in out)) out[m[1]] = /^-?\d+(\.\d+)?$/.test(m[2]) ? Number(m[2]) : m[2];
    }
    const late = String(out.engine || '').match(/late: (\d+)/);
    out.coreLate = late ? Number(late[1]) : null;
    return out;
  };
  return { send, evaluate, press, diagnostics, targetId: target.targetId };
}

async function loadCode(tab, code) {
  await tab.evaluate(`(() => {
    const code = ${JSON.stringify(code)};
    Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { readText: async () => code, writeText: async () => {} } });
    document.getElementById('pasteCode').click();
    return true;
  })()`);
  await sleep(400);
  return tab.evaluate("document.getElementById('loopName').textContent");
}

// ------------------------------------------------------------ the figures

// Seconds of music that never sounded because of a stall: each gap between
// sounding times (a late note sounds when it is handed over) that overlaps a
// stall (or the second after it) and is longer than `allow`, less `allow`.
// A loop's own rests away from the stalls are not counted.
function silent(events, from, to, allow, windows = []) {
  const at = events.map(([time, handed]) => Math.max(time, handed)).filter((t) => t >= from && t <= to).sort((x, y) => x - y);
  const points = [from, ...at, to];
  let sum = 0;
  let worst = 0;
  for (let i = 1; i < points.length; i++) {
    const a = points[i - 1];
    const b = points[i];
    worst = Math.max(worst, b - a);
    const near = windows.some(([w0, w1]) => a < w1 + 1 && b > w0);
    if (near && b - a > allow) sum += b - a - allow;
  }
  return { sum, worst };
}

function lateCount(events, from, to, engine) {
  const late = events.filter(([time, handed]) => time >= from && time <= to && time < handed);
  if (engine === 'core') return late.length;
  return new Set(late.map(([time]) => Math.round(time * 1000))).size;
}

async function measure(tab, engine, { label, run, settle = 4000, control = null }) {
  await tab.diagnostics(true);
  const before = await tab.diagnostics();
  const mark = await tab.evaluate('window.__stalls.mark()');
  const info = await run();
  await sleep(settle);
  const diag = await tab.diagnostics();
  const after = await tab.evaluate(`window.__stalls.since(${JSON.stringify(mark)})`);
  const events = engine === 'core' ? after.notes : after.starts;
  const from = mark.ctx + 0.5;
  // Notes are decided ahead, so the last few seconds are not all in yet.
  const to = after.ctx;
  let gap = 0;
  for (let i = 1; i < after.ticks.length; i++) gap = Math.max(gap, after.ticks[i] - after.ticks[i - 1]);
  const quiet = silent(events, from, to, control ? control.worstGap + 0.05 : Infinity, after.windows);
  return {
    label, ...info,
    gapMs: Math.round(gap),
    lateTicks: diag.lateTicks, worstLateMs: diag.worstLateMs,
    lateNotes: lateCount(events, from, to, engine),
    coreLate: before.coreLate != null && diag.coreLate != null ? diag.coreLate - before.coreLate : null,
    silentS: control ? +quiet.sum.toFixed(2) : null,
    worstGap: +quiet.worst.toFixed(3),
    fills: after.fills != null && mark.fills != null ? after.fills - mark.fills : null,
    engineLine: diag.engine,
    notes: events.length,
  };
}

async function session(opts, chromePath, loop, engine, throttle) {
  const b = await launch(chromePath);
  const rows = [];
  try {
    const url = `http://127.0.0.1:${opts.port}/${engine === 'js' ? '?engine=js' : ''}`;
    const tab = await openTab(b, url, throttle);
    if (opts.quality === 'lite') {
      const on = await tab.evaluate("document.getElementById('liteMode').checked");
      if (!on) await tab.evaluate("document.getElementById('liteMode').click()");
    }
    await tab.press('#playBtn');
    await sleep(1500);
    const name = await loadCode(tab, LOOPS[loop].code);
    await sleep(3000);
    // Hidden: another tab in front, as perf.mjs does it.
    const { targetId } = await b.send('Target.createTarget', { url: 'about:blank', newWindow: false });
    await b.send('Target.activateTarget', { targetId });
    await sleep(4000);
    const vis = await tab.evaluate('document.visibilityState');
    if (vis !== 'hidden') throw new Error(`the tab is ${vis}, not hidden`);
    const control = await measure(tab, engine, { label: 'control', run: async () => { await sleep(30000); return {}; }, settle: 0 });
    rows.push(control);
    const base = { loop: name, engine, throttle };
    log(`  control: worst note gap ${control.worstGap} s, ${control.notes} notes, late ticks ${control.lateTicks}`);
    for (const mode of opts.modes) {
      for (const len of opts.lengths) {
        const ms = len * 1000;
        const patterns = mode === 'ticks'
          ? [['now and then', opts.each, 6000], ['back to back', opts.each + 1, 0]]
          : [['now and then', opts.each, 6000]];
        for (const [pattern, count, gapMs] of patterns) {
          const row = await measure(tab, engine, {
            label: `${mode} ${len} s, ${pattern}`,
            control,
            run: async () => {
              if (mode === 'ticks') {
                await tab.evaluate(`window.__stalls.stall(${ms}, ${count}, ${gapMs})`);
                // Stalls start on the next tick that gets through.
                await sleep(count * ms + (count - 1) * gapMs + 1500);
                await tab.evaluate('window.__stalls.plan = null, true');
                return {};
              }
              for (let i = 0; i < count; i++) {
                if (mode === 'main') {
                  await tab.evaluate(`(() => { window.__stalls.window(${ms}); const t = performance.now() + ${ms}; while (performance.now() < t); return true; })()`);
                } else {
                  await tab.evaluate(`window.__stalls.window(${ms})`);
                  try {
                    await tab.send('Page.setWebLifecycleState', { state: 'frozen' });
                  } catch (err) {
                    return { error: err.message };
                  }
                  await sleep(ms);
                  await tab.send('Page.setWebLifecycleState', { state: 'active' });
                }
                if (i < count - 1) await sleep(gapMs);
              }
              return {};
            },
          });
          rows.push({ ...base, ...row });
          log(`  ${row.label}: ${row.error ? row.error : `gap ${row.gapMs} ms, late ticks ${row.lateTicks} (worst ${row.worstLateMs} ms), late notes ${row.lateNotes}${row.coreLate != null ? ` (core ${row.coreLate})` : ''}, silent ${row.silentS} s, fill-ins ${row.fills}`}`);
        }
      }
    }
    control.loop = name;
    control.engine = engine;
    control.throttle = throttle;
    return rows;
  } finally {
    await b.close();
  }
}

function log(s) { process.stderr.write(`${s}\n`); }

function table(rows) {
  const head = ['loop', 'engine', 'cpu', 'stall', 'longest gap ms', 'late ticks', 'worst late ms', 'late notes', 'silent s', 'fill-ins'];
  const out = [`| ${head.join(' | ')} |`, `|${head.map(() => '---').join('|')}|`];
  for (const r of rows) {
    const cells = r.error
      ? [r.loop, r.engine, `${r.throttle}x`, r.label, r.error, '', '', '', '', '']
      : [r.loop, r.engine, `${r.throttle}x`, r.label, r.gapMs, r.lateTicks, r.worstLateMs, r.lateNotes, r.silentS ?? '-', r.fills ?? '-'];
    out.push(`| ${cells.join(' | ')} |`);
  }
  return out.join('\n');
}

async function main() {
  const opts = parseArgs(process.argv.slice(2));
  const chromePath = findChrome(opts.chrome);
  const server = await startServer(opts.port);
  const rows = [];
  try {
    for (const loop of opts.loops) for (const engine of opts.engines) for (const throttle of opts.throttle) {
      log(`${loop} ${engine} ${throttle}x ${opts.quality}`);
      try {
        rows.push(...await session(opts, chromePath, loop, engine, throttle));
      } catch (err) {
        log(`  failed: ${err.message}`);
        rows.push({ loop, engine, throttle, label: 'session', error: err.message });
      }
    }
  } finally {
    server.close();
  }
  const text = `${table(rows)}\n`;
  console.log(text);
  if (opts.json) fs.writeFileSync(opts.json, `${JSON.stringify(rows, null, 1)}\n`);
  if (opts.md) fs.writeFileSync(opts.md, text);
}

await main();
