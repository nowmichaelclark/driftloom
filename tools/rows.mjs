// The app on the core's player (queue item 94b): a check for each row of
// item 94a's table (every control and display against its command or
// event), in the real app, in headless Chromium, playing in real time.
//
//   node tools/rows.mjs [--long] [--port 8771]
//   node tools/rows.mjs --app [--cdp URL] [--long]
//
// `--app` (session 27, queue item 107c): the same rows against the Android
// app, its page driving the native core (android/cdp.mjs: the WebView's
// DevTools port over adb, on the one device or emulator adb sees, the app
// installed; `--cdp` an endpoint already up instead). The hooks sit on the
// app's bridge (`DriftloomNative`) instead of a worklet's node, and "is
// anything sounding" is the native output's own level. The rows that make no
// sense there are left out (the export's, Android's own in session 28; the
// plain lines: the app always has its core), and four are the app's own:
// the WebView makes no audio of its own; Diagnostics says the native core,
// with its counts and the stream's underruns; the page's main thread blocked
// for five seconds and the music plays on (docs/CORE.md, promise 1); and
// the too-slow line (item 100) can't show, however slow the reports read.
//
// Nothing in js/ is touched. The page is read from outside, before its own
// scripts run: every AudioWorkletNode it builds is kept, with what the core
// sends back (its events, its counts) and what the page posts to it; an
// analyser hangs on the node's output, for "is anything sounding"; and the
// core is asked what it plays (`probe`: its state, its step, the track's
// end, its lanes and the loop's description, `Player::describe`). Each row
// does what a listener does (a tap, a key, a slider) and checks that the
// core plays what the page shows:
//
//   Play                the core plays the loop on the card (its name); the
//                       first loop is a fixed share code (tools/stalls.mjs's
//                       shatter), pasted as a listener would, so no row
//                       depends on a random draw
//   info card           name, key, tempo as the core describes the loop
//   cover               drawn
//   cursor              lit from the core's steps, moving
//   Pause, Play again   the core idle and silent; then on from where it stood
//   Next                the core's own loop (queue item 95), crossfading;
//                       the core's loop is the card's
//   Previous            back to the loop before, by its seed, request and edits
//   roll a layer        made again in place: its seed re-rolled, the steps go on
//   mutes               all five off: silence; on: sound
//   tempo               the core's tempo and its steps' spacing
//   tone                the core's tone
//   wander and salt     the amount, and the salt across the upper range
//   volume              0: silence; back: sound
//   mono                the core renders one lane, then two
//   layers' grid        the dots of the bar shown are the core's grid
//   picks               a vibe picked: held by the core (queue item 95), the
//                       loop playing left as it is, the next loop the
//                       core's own and leaning on it; held again after a
//                       reload
//   export MIDI         a MIDI file
//   export audio        (queue item 104b and 104c) the sheet: Lossless and,
//                       where the browser can encode Opus, Small (hidden
//                       where it can't: tested with the encoder taken
//                       away), the lengths for a loop that plays on; a minute
//                       exported to the file chosen (the save picker,
//                       answered here with a file in the site's private
//                       file system), written as it is made: a FLAC that
//                       ends where the core says, the music playing on with
//                       nothing late; both speeds ("lighter on processor"
//                       off and on), measured; Cancel leaves no file and
//                       nothing broken; the free space checked first (the
//                       quota made small: the plain message, nothing
//                       started); a disk that fills mid-export (the quota
//                       made small while it runs) ends with the plain
//                       message, no file, the music playing; and where
//                       there is no save picker, the private file and a
//                       download at the end. Small (104c) has its own: a
//                       minute to the file chosen (an Ogg file, every page's
//                       checksum good, the pre-skip and the last granule
//                       position giving exactly the frames the Worker
//                       rendered, the browser decoding it to that length),
//                       its size near the constant bit rate, Cancel, the
//                       private file and a disk that fills
//   share code          the code decodes to the loop the core plays
//   lighter on processor  a new core at the other quality, the same loop
//   Diagnostics         the core's player, nothing late, nothing lost
//   plain line          a browser with no WebAssembly, and a module that
//                       never arrives: one plain line each, nothing played
//   long piece          (queue item 106b; a page of its own, opened on a
//                       32,768-bar arc) where you are: bar of how many,
//                       time played and left, in the rail's place; the
//                       markers on the line are the map's (a fresh call of
//                       the core's map, painted pixel for pixel); a tap on
//                       the line three hours in starts there within a
//                       second, the words and the place saying so, nothing
//                       late; the next-section key goes to the map's next
//                       section; the journey pick (arc, wander) reaches the
//                       core with the picks and is held whole; Endless: time
//                       played, no time left, and a wander's own markers;
//                       the Library's line says each loop's length
//   length (--long)     a minute: the track ends on its line and the core's
//                       own loop starts there, with the picks (queue item
//                       95), the card following; Now Playing counts down
//
// Chromium from Playwright (npm install --no-save playwright@1.56.1).

import http from 'node:http';
import fs from 'node:fs';
import path from 'node:path';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';
import { decodeSong } from '../js/share.js';
import { LOOPS } from './stalls.mjs';
import { pages } from './ogg-read.mjs';

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
const LONG = argv.includes('--long');
const PORT = Number(flag('--port', 8771));
const APP = argv.includes('--app');
const CDP = flag('--cdp', '');

function loadPlaywright() {
  const require = createRequire(import.meta.url);
  for (const id of ['playwright', 'playwright-core', '/opt/node22/lib/node_modules/playwright']) {
    try { return require(id); } catch { /* try the next one */ }
  }
  console.error('rows: Playwright is not installed.  npm install --no-save playwright@1.56.1');
  process.exit(3);
  return null;
}

const TYPES = {
  '.html': 'text/html', '.js': 'text/javascript', '.mjs': 'text/javascript', '.css': 'text/css', '.json': 'application/json',
  '.webmanifest': 'application/manifest+json', '.wav': 'audio/wav', '.flac': 'audio/flac', '.png': 'image/png',
  '.svg': 'image/svg+xml', '.wasm': 'application/wasm', '.woff2': 'font/woff2', '.txt': 'text/plain',
};

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

// Before the page's scripts: the nodes, their messages, an analyser on the
// output, the clipboard and the downloads.
const HOOK = `(() => {
  const R = window.__rows = { nodes: [], events: [], posts: [], probes: new Map(), id: 0, clip: null, blobs: [] };
  const Real = window.AudioWorkletNode;
  window.AudioWorkletNode = class extends Real {
    constructor(ctx, name, options) {
      super(ctx, name, options);
      R.ctx = ctx;
      R.nodes.push({ node: this, quality: options && options.processorOptions && options.processorOptions.quality });
      this.port.addEventListener('message', (e) => {
        const m = e.data;
        if (m && m.type === 'events') R.events.push({ node: R.nodes.length - 1, at: ctx.currentTime, events: m.events, grids: m.grids || [], place: m.place });
        if (m && m.type === 'probe' && R.probes.has(m.id)) { R.probes.get(m.id)(m); R.probes.delete(m.id); }
      });
      this.port.start();
    }
  };
  const post = MessagePort.prototype.postMessage;
  MessagePort.prototype.postMessage = function (m, ...rest) {
    if (m && typeof m.type === 'string' && /^play|^layout/.test(m.type)) R.posts.push({ ...m, at: R.ctx ? R.ctx.currentTime : 0, msg: m });
    return post.call(this, m, ...rest);
  };
  const connect = AudioNode.prototype.connect;
  AudioNode.prototype.connect = function (dest, ...rest) {
    if (this instanceof window.AudioWorkletNode && dest === this.context.destination) {
      const a = this.context.createAnalyser();
      a.fftSize = 2048;
      connect.call(this, a, rest[0] || 0);
      R.analyser = a;
    }
    return connect.call(this, dest, ...rest);
  };
  R.level = () => {
    if (!R.analyser) return 0;
    const buf = new Float32Array(R.analyser.fftSize);
    R.analyser.getFloatTimeDomainData(buf);
    let s = 0;
    for (const v of buf) s += v * v;
    return Math.sqrt(s / buf.length);
  };
  R.probe = () => new Promise((resolve) => {
    const n = R.nodes[R.nodes.length - 1];
    if (!n) return resolve(null);
    const id = 1e9 + ++R.id;
    R.probes.set(id, resolve);
    n.node.port.postMessage({ type: 'probe', id });
    setTimeout(() => resolve(null), 2000);
  });
  Object.defineProperty(navigator, 'clipboard', { configurable: true, value: {
    writeText: async (t) => { R.clip = t; }, readText: async () => R.clip || '' } });
  // The audio export's Worker (queue item 104b): what the page starts it
  // with, and what it says (progress counted, the rest kept). The save
  // picker answers with a file in the site's private file system, unless a
  // tab takes the picker away (then the export writes there itself).
  R.exports = [];
  R.exportProgress = 0;
  const RealWorker = window.Worker;
  window.Worker = class extends RealWorker {
    constructor(url, options) {
      super(url, options);
      if (!String(url).includes('export-worker')) return;
      this.addEventListener('message', (e) => {
        const m = e.data;
        if (m && m.type === 'progress') R.exportProgress++;
        else if (m) R.exports.push({ type: m.type, format: m.format, opus: m.opus, why: m.why, bytes: m.bytes, frames: m.frames, seconds: m.seconds, took: m.took, counts: m.counts, private: m.private });
      });
      const post = this.postMessage;
      this.postMessage = (m, t) => {
        if (m && m.type === 'start') R.exportStart = { job: m.job, sink: m.sink, pace: m.pace, name: m.name };
        return t ? post.call(this, m, t) : post.call(this, m);
      };
    }
  };
  window.showSaveFilePicker = async (opts) => {
    const d = await navigator.storage.getDirectory();
    R.picked = await d.getFileHandle(String(opts && opts.suggestedName).endsWith('.opus') ? 'picked.opus' : 'picked.flac', { create: true });
    return R.picked;
  };
  R.opfs = async () => {
    const root = await navigator.storage.getDirectory();
    const out = [];
    for await (const [k, h] of root.entries()) {
      if (h.kind === 'file') out.push(k);
      else for await (const k2 of h.keys()) out.push(k + '/' + k2);
    }
    return out;
  };
  const make = URL.createObjectURL;
  URL.createObjectURL = function (b) { if (b instanceof Blob) R.blobs.push(b); return make.call(this, b); };
})();`;

// The app's (`--app`): the same record, kept from the native bridge. Every
// message the page posts it (an `init` is a new player, the node's place);
// every batch it posts back; any audio context the page makes (none should
// be); and a way to hand the page's own listener a batch, as the core would.
const HOOK_APP = `(() => {
  const R = window.__rows = { nodes: [], events: [], posts: [], probes: new Map(), id: 0, clip: null, blobs: [], native: null, contexts: 0, listeners: [], app: true };
  for (const name of ['AudioContext', 'webkitAudioContext']) {
    const Real = window[name];
    if (typeof Real !== 'function') continue;
    window[name] = class extends Real { constructor(...a) { super(...a); R.contexts++; } };
  }
  const hear = (text) => {
    let batch;
    try { batch = JSON.parse(text); } catch { return; }
    for (const m of [].concat(batch)) {
      if (!m) continue;
      if (m.type === 'native') R.native = m;
      if (m.type === 'events') R.events.push({ node: R.nodes.length - 1, at: 0, events: m.events, grids: m.grids || [] });
      if (m.type === 'probe' && R.probes.has(m.id)) { R.probes.get(m.id)(m); R.probes.delete(m.id); }
    }
  };
  let hooked = false;
  const hook = () => {
    const N = window.DriftloomNative;
    if (hooked || !N) return;
    hooked = true;
    const send = N.postMessage.bind(N);
    const listen = N.addEventListener.bind(N);
    R.send = (m) => send(JSON.stringify(m));
    const post = (text) => {
      try {
        const m = JSON.parse(text);
        if (m && m.type === 'init') R.nodes.push({ quality: m.quality });
        if (m && typeof m.type === 'string' && /^play|^layout/.test(m.type)) R.posts.push({ ...m });
      } catch { /* not ours to read */ }
      return send(text);
    };
    const add = (type, fn, ...rest) => {
      if (type === 'message') R.listeners.push(fn);
      return listen(type, fn, ...rest);
    };
    listen('message', (e) => hear(e.data));
    try { N.postMessage = post; N.addEventListener = add; } catch { /* read-only */ }
    if (N.postMessage !== post) {
      // An object whose methods can't be replaced: the page is given one
      // that passes everything to it.
      const shim = { postMessage: post, addEventListener: add, removeEventListener: (...a) => N.removeEventListener(...a) };
      Object.defineProperty(window, 'DriftloomNative', { configurable: true, value: shim });
    }
  };
  hook();
  document.addEventListener('readystatechange', hook);
  R.inject = (batch) => { for (const fn of R.listeners) fn({ data: JSON.stringify(batch) }); };
  R.level = () => (R.native ? R.native.level : 0);
  R.probe = () => new Promise((resolve) => {
    if (!R.send) return resolve(null);
    const id = 1e9 + ++R.id;
    R.probes.set(id, resolve);
    R.send({ type: 'probe', id });
    setTimeout(() => resolve(null), 2000);
  });
  Object.defineProperty(navigator, 'clipboard', { configurable: true, value: {
    writeText: async (t) => { R.clip = t; }, readText: async () => R.clip || '' } });
  const make = URL.createObjectURL;
  URL.createObjectURL = function (b) { if (b instanceof Blob) R.blobs.push(b); return make.call(this, b); };
})();`;

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const rows = [];
function row(name, ok, detail = '') {
  rows.push({ name, ok: !!ok, detail });
  console.log(`  ${ok ? 'pass' : 'FAIL'}  ${name}${detail ? `  (${detail})` : ''}`);
}

const about = (text) => (text ? Object.fromEntries(text.split('\n').filter(Boolean).map((l) => [l.slice(0, l.indexOf('=')), l.slice(l.indexOf('=') + 1)])) : null);

// FLAC's STREAMINFO from a file's first 42 bytes.
function streaminfo(b) {
  if (String.fromCharCode(...b.slice(0, 4)) !== 'fLaC') return null;
  const si = b.slice(8);
  return {
    rate: (si[10] << 12) | (si[11] << 4) | (si[12] >> 4),
    channels: ((si[12] >> 1) & 7) + 1,
    bits: (((si[12] & 1) << 4) | (si[13] >> 4)) + 1,
    total: (si[13] & 0x0f) * 2 ** 32 + ((si[14] << 24) >>> 0) + (si[15] << 16) + (si[16] << 8) + si[17],
  };
}

// An exported Ogg Opus file's facts (queue item 104c): every page's checksum
// is verified (the reader throws on one that fails); the headers, the end of
// stream and the granule positions as RFC 7845 has them.
function oggFacts(bytes) {
  const ps = pages(bytes);
  const head = ps[0] && ps[0].packets[0];
  const tags = ps[1] && ps[1].packets[0];
  const ascii = (b, n) => String.fromCharCode(...b.slice(0, n));
  if (!head || ascii(head, 8) !== 'OpusHead') return null;
  const preSkip = head[10] | (head[11] << 8);
  const last = ps[ps.length - 1];
  return {
    pages: ps.length,
    channels: head[9],
    preSkip,
    beginning: ps[0].flags === 2,
    tagged: !!tags && ascii(tags, 8) === 'OpusTags' && ps[1].flags === 0,
    end: (last.flags & 4) === 4 && ps.slice(0, -1).every((p) => (p.flags & 4) === 0),
    ordered: ps.every((p, i) => p.sequence === i && (i < 3 || p.granule >= ps[i - 1].granule)),
    frames: last.granule - preSkip,
    packets: ps.slice(2).reduce((n, p) => n + p.packets.length, 0),
  };
}

// The audio export's rows (queue item 104b), on the page playing.
async function exportRows({ page, ev, probe, level, diag, clickId, browser }) {
  const origin = `http://127.0.0.1:${PORT}`;
  const cdp = await page.context().newCDPSession(page);
  const share = () => ev(() => {
    if (document.getElementById('viewSave').hidden) document.getElementById('saveBtn').click();
    document.querySelector('[data-stab="share"]').click();
  });
  const sheet = () => ev(() => ({
    choose: !document.getElementById('exportChoose').hidden,
    run: !document.getElementById('exportRun').hidden,
    done: !document.getElementById('exportDone').hidden,
    failed: !document.getElementById('exportFailed').hidden,
    why: document.getElementById('exportWhy').textContent,
    saved: document.getElementById('exportSaved').textContent,
    about: document.getElementById('exportAbout').textContent,
    lengths: [...document.querySelectorAll('#exportLengths .vkey')].map((b) => `${b.dataset.length}${b.getAttribute('aria-checked') === 'true' ? '*' : ''}`),
    lossless: !document.querySelector('[data-format="lossless"]').hidden,
    small: !document.querySelector('[data-format="small"]').hidden,
  }));
  // The Worker's next word of `types` after `from`, or null after `ms`.
  const said = async (types, from, ms) => {
    const t0 = Date.now();
    while (Date.now() - t0 < ms) {
      const m = await ev((n) => window.__rows.exports.slice(n), from);
      const hit = m.find((x) => types.includes(x.type));
      if (hit) return hit;
      await sleep(250);
    }
    return null;
  };
  const count = () => ev(() => window.__rows.exports.length);
  const lateLine = async () => (await diag()).split('\n').find((l) => l.startsWith('engine:')) || '';
  const nothingLate = (line) => /late: 0\s+missed: 0\s+dropped: 0/.test(line);
  const playing = async () => {
    const p = await probe();
    return p && p.player.state === 3 && (await level(1500)) > 1e-4;
  };
  const setLite = async (on) => {
    const now = await ev(() => document.getElementById('liteMode').getAttribute('aria-checked') === 'true');
    if (now !== on) {
      await clickId('liteMode');
      await sleep(3500);
    }
  };

  console.log('\n  export audio (queue items 104b and 104c)');
  await share();
  await sleep(1200);
  let s = await sheet();
  row('export audio: the sheet offers Lossless and Small (this browser can encode Opus), and the lengths for a loop that plays on', s.choose && s.lossless && s.small && s.lengths.length === 7 && s.lengths.includes('600*') && /^About \d+ MB\./.test(s.about), `${s.lengths.join(' ')}; ${s.about}`);

  // A minute, both speeds.
  const speeds = {};
  for (const lite of [false, true]) {
    await setLite(lite);
    await share();
    await ev(() => document.querySelector('#exportLengths [data-length="60"]').click());
    const from = await count();
    const progress = await ev(() => window.__rows.exportProgress);
    await clickId('exportAudio');
    const done = await said(['done', 'failed', 'cancelled'], from, 240000);
    const meanwhile = await playing();
    const start = await ev(() => window.__rows.exportStart);
    const file = await ev(async () => {
      const f = await window.__rows.picked.getFile();
      return { size: f.size, head: [...new Uint8Array(await f.slice(0, 42).arrayBuffer())] };
    });
    const si = streaminfo(file.head);
    s = await sheet();
    const moved = (await ev(() => window.__rows.exportProgress)) - progress;
    const line = await lateLine();
    const pace = lite ? 'gentle' : 'fast';
    speeds[pace] = done && done.took ? done.seconds / done.took : 0;
    row(`export audio (${pace}): a minute to the file chosen, written as it went, a FLAC of the loop playing that ends where the core says, the music on and nothing late`,
      done && done.type === 'done' && start.pace === pace && start.sink === 'handle' && file.size === done.bytes && si && si.rate === 44100 && si.bits === 16 && si.total === done.frames
      && done.seconds >= 60 && done.seconds < 60 + 30 && moved >= 2 && !done.counts.late && !done.counts.missed && !done.counts.dropped && s.done && meanwhile && nothingLate(line),
      `${done && done.type} ${done && done.why || ''}, ${file.size} bytes, ${si && si.total} frames, ${done && done.seconds && done.seconds.toFixed(1)} s in ${done && done.took && done.took.toFixed(1)} s (${speeds[pace].toFixed(1)}x), ${moved} progress reports; ${s.saved}; ${line}`);
    await clickId('exportDoneBtn');
  }
  row('export audio: gentle ("lighter on processor" on) is slower than fast, and both faster than real time', speeds.fast > speeds.gentle && speeds.gentle > 1, `fast ${speeds.fast.toFixed(1)}x, gentle ${speeds.gentle.toFixed(1)}x real time`);
  await setLite(false);

  // Cancel.
  await share();
  await ev(() => document.querySelector('#exportLengths [data-length="600"]').click());
  let from = await count();
  await clickId('exportAudio');
  await sleep(1500);
  await clickId('exportCancel');
  let m = await said(['cancelled', 'done', 'failed'], from, 30000);
  s = await sheet();
  let files = await ev(() => window.__rows.opfs());
  row('export audio: Cancel leaves no file, the music playing and nothing late', m && m.type === 'cancelled' && s.failed && /Nothing was saved/.test(s.why) && !files.includes('picked.flac') && (await playing()) && nothingLate(await lateLine()),
    `${m && m.type}; ${s.why}; files ${files.join(',') || 'none'}`);
  await clickId('exportOk');

  // The free space checked first.
  const usage = await ev(async () => (await navigator.storage.estimate()).usage);
  await cdp.send('Storage.overrideQuotaForOrigin', { origin, quotaSize: usage + 1e6 });
  await ev(() => document.querySelector('#exportLengths [data-length="3600"]').click());
  from = await count();
  const before = await ev(() => window.__rows.exportStart);
  await clickId('exportAudio');
  await sleep(1500);
  s = await sheet();
  const after = await ev(() => window.__rows.exportStart);
  row('export audio: free space checked first: too little says so plainly, and nothing starts', s.failed && /^There isn't enough free space for this/.test(s.why) && JSON.stringify(before) === JSON.stringify(after) && (await count()) === from,
    s.why);
  await cdp.send('Storage.overrideQuotaForOrigin', { origin });
  await clickId('exportOk');
  await probe();

  // Small (104c): a minute to the file chosen, then Cancel.
  await share();
  await ev(() => document.querySelector('[data-format="small"]').click());
  await ev(() => document.querySelector('#exportLengths [data-length="60"]').click());
  s = await sheet();
  const smallAbout = s.about;
  from = await count();
  const progress0 = await ev(() => window.__rows.exportProgress);
  await clickId('exportAudio');
  const small = await said(['done', 'failed', 'cancelled'], from, 240000);
  const smallMeanwhile = await playing();
  const smallStart = await ev(() => window.__rows.exportStart);
  const smallGot = await ev(async () => {
    const f = await window.__rows.picked.getFile();
    const b = new Uint8Array(await f.arrayBuffer());
    let bin = '';
    for (let i = 0; i < b.length; i += 0x8000) bin += String.fromCharCode(...b.subarray(i, i + 0x8000));
    let decoded;
    try { decoded = (await new OfflineAudioContext(2, 1, 48000).decodeAudioData(b.buffer.slice(0))).length; } catch (e) { decoded = String(e); }
    return { name: f.name, size: f.size, base64: bin ? btoa(bin) : '', decoded };
  });
  let ogg = null;
  let oggWhy = '';
  try { ogg = oggFacts(new Uint8Array(Buffer.from(smallGot.base64, 'base64'))); } catch (e) { oggWhy = e.message; }
  s = await sheet();
  const smallMoved = (await ev(() => window.__rows.exportProgress)) - progress0;
  const smallLine = await lateLine();
  const rate = small && small.seconds ? smallGot.size / small.seconds : 0;
  row('export audio (small): a minute to the file chosen, written as it went, an Ogg Opus file whose end and length are the Worker\'s frames to the sample, the browser decodes it, the music on and nothing late',
    small && small.type === 'done' && small.format === 'opus' && smallStart.sink === 'handle' && smallStart.job.format === 'opus' && /\.opus$/.test(smallStart.name) && smallGot.name === 'picked.opus' && smallGot.size === small.bytes
    && ogg && ogg.beginning && ogg.tagged && ogg.end && ogg.ordered && ogg.channels === 2 && ogg.preSkip > 0 && ogg.frames === small.frames && smallGot.decoded === small.frames
    && small.seconds >= 60 && small.seconds < 60 + 30 && smallMoved >= 2 && !small.counts.late && !small.counts.missed && !small.counts.dropped && s.done && smallMeanwhile && nothingLate(smallLine),
    `${small && small.type} ${small && small.why || ''}, ${smallGot.size} bytes, ${ogg ? `${ogg.pages} pages, ${ogg.packets} packets, pre-skip ${ogg.preSkip}, ${ogg.frames} frames` : oggWhy}; the browser decoded ${smallGot.decoded} frames; ${small && small.seconds && small.seconds.toFixed(1)} s in ${small && small.took && small.took.toFixed(1)} s; ${smallMoved} progress reports; ${s.saved}; ${smallLine}`);
  row('export audio (small): the size is the constant bit rate (96 kbit/s stereo, 12,000 bytes a second and the pages) and the sheet said about so', rate > 11500 && rate < 12700 && /^About \d+ MB\./.test(smallAbout), `${Math.round(rate)} bytes a second; ${smallAbout}`);
  await clickId('exportDoneBtn');
  await share();
  await ev(() => document.querySelector('#exportLengths [data-length="600"]').click());
  from = await count();
  await clickId('exportAudio');
  await sleep(1500);
  await clickId('exportCancel');
  m = await said(['cancelled', 'done', 'failed'], from, 30000);
  s = await sheet();
  files = await ev(() => window.__rows.opfs());
  row('export audio (small): Cancel leaves no file, the music playing and nothing late', m && m.type === 'cancelled' && s.failed && /Nothing was saved/.test(s.why) && !files.includes('picked.opus') && (await playing()) && nothingLate(await lateLine()),
    `${m && m.type}; ${s.why}; files ${files.join(',') || 'none'}`);
  await clickId('exportOk');
  await ev(() => document.querySelector('[data-format="lossless"]').click());
  await probe();

  // The private file, and a disk that fills while it is written: another
  // tab, with no save picker (as Android's Chrome and Safari have none).
  const tab = await browser.newPage({ viewport: { width: 412, height: 915 } });
  const bad = [];
  tab.on('pageerror', (e) => bad.push(e.message));
  await tab.addInitScript(HOOK);
  await tab.addInitScript(() => { window.showSaveFilePicker = undefined; });
  await tab.goto(`${origin}/`);
  await tab.waitForFunction(() => document.readyState === 'complete' && document.getElementById('playBtn'));
  await sleep(1500);
  const tev = (fn, arg) => tab.evaluate(fn, arg);
  await tev((code) => { window.__rows.clip = code; }, LOOPS.shatter.code);
  await tev(() => document.getElementById('pasteCode').click());
  await sleep(3000);
  await tev(() => {
    if (document.getElementById('viewSave').hidden) document.getElementById('saveBtn').click();
    document.querySelector('[data-stab="share"]').click();
  });
  await sleep(1200);
  await tev(() => document.querySelector('#exportLengths [data-length="60"]').click());
  const blobs = await tev(() => window.__rows.blobs.length);
  await tev(() => document.getElementById('exportAudio').click());
  let t0 = Date.now();
  let done = null;
  while (Date.now() - t0 < 240000 && !done) {
    await sleep(500);
    done = (await tev(() => window.__rows.exports)).find((x) => x.type === 'done' || x.type === 'failed');
  }
  const got = await tev(async (n) => {
    const b = window.__rows.blobs[n];
    return b ? { size: b.size, head: [...new Uint8Array(await b.slice(0, 42).arrayBuffer())] } : null;
  }, blobs);
  const start = await tev(() => window.__rows.exportStart);
  const si = got && streaminfo(got.head);
  row('export audio, with no save picker: written to the private file as it went, a download at the end', done && done.type === 'done' && start.sink === 'private' && got && got.size === done.bytes && si && si.total === done.frames,
    `${done && done.type} ${done && done.why || ''}, ${got && got.size} bytes`);
  await tev(() => document.getElementById('exportDoneBtn').click());

  // A disk that fills mid-export: the quota made small once it is writing.
  const tcdp = await tab.context().newCDPSession(tab);
  await tev(() => document.querySelector('#exportLengths [data-length="1800"]').click());
  const n0 = (await tev(() => window.__rows.exports)).length;
  await tev(() => document.getElementById('exportAudio').click());
  t0 = Date.now();
  while (Date.now() - t0 < 30000 && (await tev(() => window.__rows.exportProgress)) < 3) await sleep(250);
  const used = await tev(async () => (await navigator.storage.estimate()).usage);
  await tcdp.send('Storage.overrideQuotaForOrigin', { origin, quotaSize: used + 256 * 1024 });
  t0 = Date.now();
  let end = null;
  while (Date.now() - t0 < 120000 && !end) {
    await sleep(500);
    end = (await tev(() => window.__rows.exports)).slice(n0).find((x) => ['done', 'failed', 'cancelled'].includes(x.type));
  }
  await tcdp.send('Storage.overrideQuotaForOrigin', { origin });
  files = await tev(() => window.__rows.opfs());
  const why = await tev(() => document.getElementById('exportWhy').textContent);
  const tp = await tev(() => window.__rows.probe());
  const tl = await tev(async () => { let w = 0; for (let i = 0; i < 15; i++) { w = Math.max(w, window.__rows.level()); await new Promise((r) => setTimeout(r, 100)); } return w; });
  row('export audio: a disk that fills mid-export ends with the plain message, no file left, the music playing', end && end.type === 'failed' && end.why === 'space' && /ran out of space/.test(why) && !files.some((f) => f.startsWith('exports/') && f.endsWith('.flac')) && tp && tp.player.state === 3 && tl > 1e-4,
    `${end && end.type} ${end && end.why || ''}; ${why}; files ${files.join(',') || 'none'}`);
  await tev(() => document.getElementById('exportOk').click());

  // Small in the same tab: the private file, and a disk that fills.
  await tev(() => document.querySelector('[data-format="small"]').click());
  await tev(() => document.querySelector('#exportLengths [data-length="60"]').click());
  const blobs2 = await tev(() => window.__rows.blobs.length);
  const n1 = (await tev(() => window.__rows.exports)).length;
  await tev(() => document.getElementById('exportAudio').click());
  t0 = Date.now();
  let done2 = null;
  while (Date.now() - t0 < 240000 && !done2) {
    await sleep(500);
    done2 = (await tev(() => window.__rows.exports)).slice(n1).find((x) => x.type === 'done' || x.type === 'failed');
  }
  const got2 = await tev(async (n) => {
    const b = window.__rows.blobs[n];
    if (!b) return null;
    const u = new Uint8Array(await b.arrayBuffer());
    let bin = '';
    for (let i = 0; i < u.length; i += 0x8000) bin += String.fromCharCode(...u.subarray(i, i + 0x8000));
    return { size: b.size, base64: btoa(bin) };
  }, blobs2);
  const start2 = await tev(() => window.__rows.exportStart);
  let ogg2 = null;
  try { ogg2 = got2 && oggFacts(new Uint8Array(Buffer.from(got2.base64, 'base64'))); } catch { ogg2 = null; }
  row('export audio (small), with no save picker: written to the private file as it went, an Ogg Opus file downloaded at the end', done2 && done2.type === 'done' && done2.format === 'opus' && start2.sink === 'private' && got2 && got2.size === done2.bytes && ogg2 && ogg2.end && ogg2.ordered && ogg2.frames === done2.frames,
    `${done2 && done2.type} ${done2 && done2.why || ''}, ${got2 && got2.size} bytes, ${ogg2 ? ogg2.frames : '?'} frames`);
  await tev(() => document.getElementById('exportDoneBtn').click());
  await tev(() => document.querySelector('#exportLengths [data-length="3600"]').click());
  const n2 = (await tev(() => window.__rows.exports)).length;
  await tev(() => document.getElementById('exportAudio').click());
  t0 = Date.now();
  while (Date.now() - t0 < 30000 && (await tev(() => window.__rows.exportProgress)) < 6) await sleep(250);
  const used2 = await tev(async () => (await navigator.storage.estimate()).usage);
  await tcdp.send('Storage.overrideQuotaForOrigin', { origin, quotaSize: used2 + 128 * 1024 });
  t0 = Date.now();
  let end2 = null;
  while (Date.now() - t0 < 120000 && !end2) {
    await sleep(500);
    end2 = (await tev(() => window.__rows.exports)).slice(n2).find((x) => ['done', 'failed', 'cancelled'].includes(x.type));
  }
  await tcdp.send('Storage.overrideQuotaForOrigin', { origin });
  files = await tev(() => window.__rows.opfs());
  const why2 = await tev(() => document.getElementById('exportWhy').textContent);
  const tp2 = await tev(() => window.__rows.probe());
  row('export audio (small): a disk that fills mid-export ends with the plain message, no file left, the music playing', end2 && end2.type === 'failed' && end2.why === 'space' && /ran out of space/.test(why2) && !files.some((f) => f.startsWith('exports/') && f.endsWith('.opus')) && tp2 && tp2.player.state === 3,
    `${end2 && end2.type} ${end2 && end2.why || ''}; ${why2}; files ${files.join(',') || 'none'}`);
  row('export audio: no page errors in the tab with no save picker', bad.length === 0, bad.join('; '));
  await tab.close();

  // A browser with no Opus encoder (the Worker's taken away: the sheet's
  // choice is the Worker's word; what the page posts before the Worker's
  // module has loaded is kept and handed to it): Small is not offered,
  // Lossless is.
  const bare = await browser.newPage({ viewport: { width: 412, height: 915 } });
  const bareBad = [];
  bare.on('pageerror', (e) => bareBad.push(e.message));
  await bare.addInitScript(HOOK);
  await bare.addInitScript(() => {
    const Real = window.Worker;
    window.Worker = class extends Real {
      constructor(url, options) {
        let u = url;
        if (String(url).includes('export-worker')) {
          const abs = new URL(String(url), location.href).href;
          u = URL.createObjectURL(new Blob([`self.AudioEncoder = undefined; const early = []; const keep = (e) => early.push(e); self.addEventListener('message', keep); await import(${JSON.stringify(abs)}); self.removeEventListener('message', keep); for (const e of early) self.onmessage(e);`], { type: 'text/javascript' }));
        }
        super(u, options);
      }
    };
  });
  await bare.goto(`${origin}/`);
  await bare.waitForFunction(() => document.readyState === 'complete' && document.getElementById('playBtn'));
  await sleep(1500);
  await bare.evaluate((code) => { window.__rows.clip = code; }, LOOPS.shatter.code);
  await bare.evaluate(() => document.getElementById('pasteCode').click());
  await sleep(3000);
  await bare.evaluate(() => {
    if (document.getElementById('viewSave').hidden) document.getElementById('saveBtn').click();
    document.querySelector('[data-stab="share"]').click();
  });
  await sleep(1500);
  const seen = await bare.evaluate(() => ({
    lossless: !document.querySelector('[data-format="lossless"]').hidden,
    small: !document.querySelector('[data-format="small"]').hidden,
    go: !document.getElementById('exportAudio').disabled,
    about: document.getElementById('exportAbout').textContent,
  }));
  row('export audio: where the browser has no Opus encoder, Small is not offered and Lossless still is', seen.lossless && !seen.small && seen.go && /^About \d+ MB\./.test(seen.about) && bareBad.length === 0, `${JSON.stringify(seen)} ${bareBad.join('; ')}`);
  await bare.close();
}

// The app's own rows (`--app`, session 27), on the page playing.
async function appRows({ ev, diag, clickId, level, probe }) {
  const line = (t, k) => t.split('\n').find((l) => l.startsWith(`${k}:`)) || '';
  {
    const seen = await ev(() => ({ contexts: window.__rows.contexts, audio: document.querySelectorAll('audio').length }));
    const keep = line(await diag(), 'keepalive');
    row('app: the WebView plays no audio of its own (no audio context, no keepalive element)', seen.contexts === 0 && seen.audio === 0 && /native/.test(keep), `${seen.contexts} contexts, ${seen.audio} audio elements; ${keep}`);
  }
  {
    const t = await diag();
    const engine = line(t, 'engine');
    const stream = line(t, 'stream');
    row('app: Diagnostics says the native core, its late, missed and dropped counts and the stream\'s underruns', /^engine: native \(the core's player\)\s+late: \d+\s+missed: \d+\s+dropped: \d+.*underruns: \d+/.test(engine) && /^stream: running/.test(stream), `${engine} | ${stream}`);
  }
  {
    // Promise 1: the page's main thread blocked for five seconds; the core
    // plays on through it, its clock keeping time with the wall's, its
    // output sounding throughout, no underrun, nothing late.
    const n0 = await ev(() => window.__rows.native);
    const t0 = Date.now();
    await ev(() => { const t = Date.now(); while (Date.now() - t < 5000) { /* busy */ } });
    const blocked = Date.now() - t0;
    await sleep(2500);
    const n1 = await ev(() => window.__rows.native);
    const wall = (Date.now() - t0) / 1000;
    const clock = n1.time - n0.time;
    const sounding = n1.sounding - n0.sounding;
    const p = await probe();
    row('app: the page\'s main thread blocked for 5 s, the music plays on (promise 1)', blocked >= 4900 && Math.abs(clock - wall) < 1.0 && sounding >= clock - 0.5 && n1.underruns === n0.underruns && p && p.player.state === 3 && p.late === 0,
      `blocked ${blocked} ms; the core's clock ${clock.toFixed(2)} s in ${wall.toFixed(2)} s, ${sounding.toFixed(2)} s sounding; underruns ${n0.underruns} -> ${n1.underruns}; late ${p && p.late}`);
  }
  {
    // Item 100's too-slow line can't show in the app: on lite, a minute of
    // reports with every quarter-second behind (in a browser, the line
    // after twenty seconds), handed to the page as the core hands them.
    const isLite = () => ev(() => document.getElementById('liteMode').getAttribute('aria-checked'));
    const was = await isLite();
    if (was !== 'true') {
      await clickId('liteMode');
      await sleep(3000);
    }
    const lite = await isLite();
    await ev(() => {
      const busy = { max: 1.5, behind: 8, windows: 8, seconds: 2 };
      for (let i = 0; i < 30; i++) window.__rows.inject([{ type: 'cost', ms: [0, 0], seconds: [0, 0], busy }]);
    });
    await sleep(500);
    const shown = await ev(() => !document.getElementById('plainLine').hidden);
    const slow = line(await diag(), 'slow');
    row('app: the too-slow line can\'t show (lite, a minute of reports every quarter-second behind)', lite === 'true' && !shown && slow === 'slow: no', `lite ${lite}, line ${shown ? 'shown' : 'hidden'}, ${slow}`);
    if (was !== 'true') {
      await clickId('liteMode');
      await sleep(3000);
    }
    row('app: and the music plays on', (await level(1500)) > 1e-4);
  }
}

// The app's page (`--app`), with the hooks in, loaded afresh.
async function openApp() {
  const { AppPage } = await import('../android/cdp.mjs');
  const page = await AppPage.open({ cdp: CDP, ms: 90000, log: (...a) => console.log(' ', ...a) });
  await page.addInitScript(HOOK_APP);
  await page.reload();
  return page;
}

// A long piece on screen (queue item 106b), in a page of its own: opened on
// the 32,768-bar arc a listener's picks make (kept in the prefs as the app
// keeps them), played in real time. Every row reads what the core says (its
// `place` on each events post, its Jump event) against what the page shows.
async function longRows(browser, port, errors) {
  const page = await browser.newPage({ viewport: { width: 412, height: 915 } });
  page.on('pageerror', (e) => errors.push(e.message));
  await page.addInitScript(HOOK);
  await page.addInitScript(() => {
    try {
      if (!localStorage.getItem('driftloom.prefs.v1')) {
        localStorage.setItem('driftloom.prefs.v1', JSON.stringify({ view: 'now', picks: { length: 32768, journey: 'arc' } }));
      }
    } catch (e) { /* no storage: the rows say so */ }
  });
  await page.goto(`http://127.0.0.1:${port}/`);
  await page.waitForFunction(() => document.readyState === 'complete' && document.getElementById('playBtn'));
  await sleep(1500);
  const ev = (fn, arg) => page.evaluate(fn, arg);
  const probe = async () => {
    const m = await ev(() => window.__rows.probe());
    return m ? { ...m, d: about(m.about) } : null;
  };
  const diag = () => ev(() => { document.getElementById('diagRefresh').click(); return document.getElementById('diagOut').textContent; });
  const nothingLate = (text) => /late: 0\s+missed: 0\s+dropped: 0/.test(text.split('\n').find((l) => l.startsWith('engine:')) || '');
  // The core's last word on where it is, and the page's words for it.
  const place = () => ev(() => { const R = window.__rows; for (let i = R.events.length - 1; i >= 0; i--) if (R.events[i].place) return R.events[i].place; return null; });
  const words = () => ev(() => ({
    shown: !document.getElementById('nowPiece').hidden,
    rail: !document.getElementById('nowRail').hidden,
    bars: document.getElementById('pieceBars').textContent,
    times: document.getElementById('pieceTimes').textContent,
  }));
  // "1:12:03 played" in seconds.
  const secs = (t) => { const m = /^([\d:]+) played/.exec(t); return m ? m[1].split(':').reduce((a, v) => a * 60 + Number(v), 0) : NaN; };
  const jumps = () => ev(() => window.__rows.posts.filter((m) => m.type === 'playJump'));
  // Seconds from the page's jump to the core's Jump event arriving.
  const landed = async (count) => {
    for (let i = 0; i < 40; i++) {
      const r = await ev((n) => {
        const R = window.__rows;
        const posts = R.posts.filter((m) => m.type === 'playJump');
        if (posts.length < n) return null;
        const post = posts[n - 1];
        for (const b of R.events) if (b.at >= post.at) for (let k = 1; k < b.events.length; k += 4) if (b.events[k] === 8) return { after: b.at - post.at, at: post.at };
        return null;
      }, count);
      if (r) return r;
      await sleep(250);
    }
    return null;
  };

  console.log(`\nA long piece on screen (queue item 106b)`);
  await page.click('#playBtn');
  await sleep(4500);
  let p = await probe();
  const spb = Number(p.d.steps_per_bar);
  const perBar = spb * (60 / Number(p.d.bpm) / 4);
  let w = await words();
  row('long piece: Now Playing says where you are: bar of how many, time played and time left, the line in the rail\'s place',
    w.shown && !w.rail && /^bar [\d,]+ of 32,768/.test(w.bars) && /^[\d:]+ played \u00b7 [\d:]+ left$/.test(w.times), `${w.bars}; ${w.times}`);

  // The markers are the map's: a fresh call of the core's map, drawn.
  const m = await ev(async () => {
    const piece = await import('/js/piece.js');
    const st = piece.pieceState();
    const model = st.model;
    if (!model) return null;
    const again = piece.modelNow(model.spec);
    const line = document.getElementById('pieceLine');
    const canvas = document.getElementById('pieceCanvas');
    const W = Math.round(line.clientWidth);
    const { heights } = piece.columns(model, W);
    const dpr = canvas.width / canvas.clientWidth;
    const ctx = canvas.getContext('2d');
    const y = Math.round((canvas.clientHeight - 4.5) * dpr);
    const row = ctx.getImageData(0, y, canvas.width, 1).data;
    let missing = 0;
    let stray = 0;
    let ticks = 0;
    for (let x = 0; x < W; x++) {
      const alpha = row[Math.min(canvas.width - 1, Math.floor((x + 0.5) * dpr)) * 4 + 3];
      if (heights[x] > 0) { ticks++; if (alpha < 40) missing++; } else if (!(x > 0 && heights[x - 1] > 0) && alpha > 10) stray++;
    }
    const roles = {};
    for (const k of model.marks) roles[k.role] = (roles[k.role] || 0) + 1;
    return { same: JSON.stringify(again.marks) === JSON.stringify(model.marks), marks: model.marks.length, first: model.marks[0].bar, last: model.last, ticks, missing, stray, W, roles };
  });
  row('long piece: the markers are the map\'s: the core\'s marks asked again are the line\'s, every tick is painted and nothing else is',
    m && m.same && m.marks > 100 && m.first === 0 && m.last === 32768 && m.ticks > 20 && m.missing === 0 && m.stray === 0 && m.roles.peak > 0 && m.roles.opening > 0, JSON.stringify(m && { ...m, roles: undefined }));

  // A jump to three hours in: a tap on the line.
  const box = await page.locator('#pieceLine').boundingBox();
  const target = Math.floor(10800 / perBar);
  const x = box.x + (target / 32768) * box.width;
  await page.mouse.click(x, box.y + box.height / 2);
  const hit = await landed(1);
  await sleep(1500);
  const sent = (await jumps())[0];
  const pl = await place();
  const pixel = 32768 / box.width;
  row('long piece: a tap three hours in starts there within a second (the core\'s Jump event, its place three hours in)',
    hit && hit.after < 1 && sent && sent.kind === 1 && Math.abs(sent.msg.at - target) <= pixel + 1 && pl && Math.abs(pl.played - 10800) < 120 + pixel * perBar, `after ${hit && hit.after.toFixed(2)} s, jump to bar ${sent && sent.msg.at}, place ${pl && Math.round(pl.played)} s`);
  w = await words();
  const now = await place();
  const shownBar = Number((/^bar ([\d,]+)/.exec(w.bars) || [])[1] && /^bar ([\d,]+)/.exec(w.bars)[1].replace(/,/g, ''));
  const word = await ev(async (bar) => {
    const piece = await import('/js/piece.js');
    const mark = piece.markAt(piece.pieceState().model, bar);
    return { want: mark ? piece.ROLE_WORDS[mark.role] : null, section: mark ? mark.section : null };
  }, now.bar);
  row('long piece: where you are after the jump: the bar, the time played and left, and the section\'s word are the core\'s place and the map\'s',
    Math.abs(shownBar - (Math.floor(now.bar) + 1)) <= 3 && Math.abs(secs(w.times) - now.played) < 6 && /left$/.test(w.times) && word.want && w.bars.endsWith(word.want), `${w.bars}; ${w.times}; place bar ${now.bar.toFixed(1)}`);
  row('long piece: nothing late across the jump', nothingLate(await diag()), (await diag()).split('\n').find((l) => l.startsWith('engine:')));

  // The next-section key goes to the map's next section.
  const want = await ev(async (bar) => {
    const piece = await import('/js/piece.js');
    const next = piece.neighbor(piece.pieceState().model, bar, 1);
    return next ? { bar: next.bar, role: next.role } : null;
  }, now.bar);
  await ev(() => document.getElementById('pieceNext').click());
  const second = await landed(2);
  await sleep(1500);
  const after = await place();
  const sent2 = (await jumps())[1];
  row('long piece: the next-section key goes to the map\'s next section, on its line', want && second && sent2 && sent2.msg.at === want.bar && after && after.bar >= want.bar - 0.1 && after.bar < want.bar + 12, `to ${sent2 && sent2.msg.at} (${want && want.role}), place ${after && after.bar.toFixed(1)}`);

  // The journey pick reaches the core with the picks (through the Bars
  // picker, as a listener sets it), and is held whole.
  await ev(() => document.getElementById('vibesBtn').click());
  await sleep(300);
  await ev(() => [...document.querySelectorAll('.pk-line-pace button')].find((b) => /^Bars/.test(b.getAttribute('aria-label') || '')).click());
  await sleep(300);
  await ev(() => { document.getElementById('lenEndless').click(); document.querySelector('#lenJourney [data-journey="wander"]').click(); });
  const lit = await ev(() => ({ wander: document.querySelector('#lenJourney [data-journey="wander"]').getAttribute('aria-pressed'), arc: document.querySelector('#lenJourney [data-journey="arc"]').getAttribute('aria-pressed'), dim: document.getElementById('lenJourney').classList.contains('off') }));
  await ev(() => document.getElementById('lenDone').click());
  await sleep(600);
  const picks = await ev(() => window.__rows.posts.filter((m) => m.type === 'playPicks').pop() || null);
  p = await probe();
  const cant = await ev(() => { const n = document.querySelector('.pk-cant'); return n ? n.textContent : ''; });
  const pills = await ev(() => [...document.querySelectorAll('#picksList .pk-line-pace .pk-pill')].map((x) => x.textContent));
  row('long piece: the journey pick (a wander, with Endless) is sent with the picks, held whole by the core, and shown beside the bars',
    lit.wander === 'true' && lit.arc === 'false' && !lit.dim && picks && picks.ask.journey === 2 && picks.ask.length === 'endless' && p.picksTag === picks.tag && !cant && pills.includes('wander'),
    `lit ${JSON.stringify(lit)}, ask journey ${picks && picks.ask.journey}, tag ${p && p.picksTag}/${picks && picks.tag}, ${cant}, ${pills.join('|')}`);

  // Next: the core's own loop takes them, Endless, a wander: time played, no
  // time left, and a wander's own markers.
  await ev(() => document.getElementById('nowBtn').click());
  await sleep(300);
  await ev(() => document.getElementById('nextBtn').click());
  await sleep(5000);
  p = await probe();
  w = await words();
  const end = await ev(async () => {
    const piece = await import('/js/piece.js');
    const model = piece.pieceState().model;
    const roles = new Set(model ? model.marks.map((k) => k.role) : []);
    return { endless: !!(model && model.endless), wander: roles.has('holding') && !roles.has('peak') && !roles.has('opening'), window: model ? model.last - model.first : 0 };
  });
  row('long piece: Endless: bar and time played, no time left, a window of the map with a wander\'s holds',
    end.endless && end.wander && end.window > 0 && /^bar [\d,]+( \u00b7 [a-z ]+)?$/.test(w.bars) && !/ of /.test(w.bars) && /^[\d:]+ played$/.test(w.times), `${w.bars}; ${w.times}; ${JSON.stringify(end)}`);

  // The Library says each loop's length: from 4 bars to 27 hours, or
  // Endless. Four loops saved (the page's own storage, as a Save would), the
  // album opened, the lines read.
  await ev(async () => {
    const store = await import('/js/storage.js');
    const gen = await import('/js/gen.js');
    [[11, 'bars=4'], [12, 'bars=3072,journey=arc'], [13, 'bars=32768,journey=wander'], [14, 'bars=endless,journey=arc'], [15, '-']].forEach(([seed, request]) => {
      const entry = store.save(gen.newSpec(seed, request));
      store.addToAlbum(store.FAVORITES, entry.id);
    });
  });
  await ev(() => document.getElementById('libraryBtn').click());
  await sleep(600);
  await ev(() => document.querySelector('#libPane .lrow-main').click());
  await sleep(600);
  const lines = await ev(() => [...document.querySelectorAll('#libPane .lrow-meta')].map((x) => x.textContent.replace(/\u00a0/g, ' ')));
  const has = (re) => lines.some((t) => re.test(t));
  row('Library: a list\'s line says a loop\'s length: 4 bars and its time, 3,072 bars and hours, 32,768 bars and about a day, endless, a plain loop\'s own bars',
    has(/ bpm · 4 bars · \d+s$/) && has(/ bpm · 3,072 bars · \d+h( \d+m)?$/) && has(/ bpm · 32,768 bars · \d+h( \d+m)?$/) && has(/ bpm · endless$/) && has(/ bpm · \d+ bars$/), lines.join(' | '));
  row('long piece: no page errors in its page', errors.length === 0, errors.join('; '));
  await page.close();
}

async function main() {
  const { chromium } = APP ? {} : loadPlaywright();
  const server = APP ? null : await serve(PORT);
  const browser = APP ? null : await chromium.launch({ args: ['--autoplay-policy=no-user-gesture-required'] });
  const errors = [];
  try {
    const page = APP ? await openApp() : await browser.newPage({ viewport: { width: 412, height: 915 } });
    page.on('pageerror', (e) => errors.push(e.message));
    const ready = async () => {
      for (let t = 0; t < 60000; t += 250) {
        if (await page.evaluate(() => document.readyState === 'complete' && !!document.getElementById('playBtn'))) return;
        await sleep(250);
      }
      throw new Error('the page did not load');
    };
    if (!APP) {
      await page.addInitScript(HOOK);
      await page.goto(`http://127.0.0.1:${PORT}/`);
    }
    await ready();
    await sleep(1500);
    const ev = (fn, arg) => page.evaluate(fn, arg);
    const probe = async () => {
      const m = await ev(() => window.__rows.probe());
      return m ? { ...m, d: about(m.about) } : null;
    };
    const name = () => ev(() => document.getElementById('loopName').textContent);
    const level = async (ms = 600) => {
      let worst = 0;
      for (let t = 0; t < ms; t += 100) {
        worst = Math.max(worst, await ev(() => window.__rows.level()));
        await sleep(100);
      }
      return worst;
    };
    const diag = () => ev(() => { document.getElementById('diagRefresh').click(); return document.getElementById('diagOut').textContent; });
    const setRange = (id, value) => ev(([i, v]) => {
      const r = document.getElementById(i);
      r.value = String(v);
      r.dispatchEvent(new Event('input', { bubbles: true }));
    }, [id, value]);
    const clickId = (id) => ev((i) => document.getElementById(i).click(), id);
    const key = (k) => page.keyboard.press(k);
    // The step events from the latest node, as [time, kind, number, step].
    const steps = () => ev(() => {
      const R = window.__rows;
      const last = R.nodes.length - 1;
      const out = [];
      for (const b of R.events) if (b.node === last) for (let i = 0; i < b.events.length; i += 4) out.push(b.events.slice(i, i + 4));
      return out;
    });
    const kinds = { step: 0, pass: 1, section: 2, phrase: 3, loop: 4, revised: 5, end: 6, own: 7 };
    // The last picks the page sent the core (queue item 95).
    const lastPicks = () => ev(() => window.__rows.posts.filter((m) => m.type === 'playPicks').pop() || null);
    // The seed of each loop the core made itself, with the picks' number.
    const owns = async () => {
      const e = await steps();
      const out = [];
      for (let i = 0; i + 1 < e.length; i++) if (e[i][1] === kinds.own && e[i + 1][1] === kinds.loop) out.push({ tag: e[i][2], seed: e[i + 1][2] });
      return out;
    };

    console.log(`\nThe app's controls and displays on the core's player (queue item 94b)`);
    // Play, from a fixed loop: a share code pasted through the page's own
    // "paste a code" (the clipboard is the harness's), which plays it. A
    // loop drawn at random could be a sparse one, or ring on after a mute.
    await ev((code) => { window.__rows.clip = code; }, LOOPS.shatter.code);
    await clickId('pasteCode');
    await sleep(3500);
    let p = await probe();
    const first = await name();
    {
      row('Play: the core plays the loop on the card', p && p.player.state === 3 && p.d.name === first, `${p && p.d && p.d.name} / ${first}`);
      const detail = await ev(() => document.getElementById('loopDetail').textContent);
      row('info card: its key, tempo and name as the core describes the loop', detail.includes(`${p.d.bpm} bpm`) && p.d.name === first, detail.replace(/\s+/g, ' '));
    }
    row('cover: drawn', await ev(() => {
      const c = document.getElementById('cover');
      const d = c.getContext('2d').getImageData(0, 0, c.width, c.height).data;
      return d.some((v, i) => i % 4 !== 3 && v > 0);
    }));
    const lit = async () => ev(() => [...document.getElementById('playhead').children].findIndex((c) => c.classList.contains('on')));
    const seen = new Set();
    for (let i = 0; i < 12; i++) {
      seen.add(await lit());
      await sleep(70);
    }
    row('cursor: lit and moving with the steps', seen.size >= 3 && !seen.has(-1), [...seen].join(','));
    // Listen for six seconds: even this loop has its rests.
    row('sound: something plays', (await level(6000)) > 1e-4);

    // Pause and Play again.
    const before = (await steps()).filter((e) => e[1] === kinds.step).pop();
    await key('Space');
    await sleep(800);
    p = await probe();
    const quiet = await level();
    row('Pause: the core stops, the output silent', p.player.state === 0 && quiet < 1e-4, `state ${p.player.state}, level ${quiet.toExponential(1)}`);
    await key('Space');
    await sleep(1500);
    {
      p = await probe();
      const after = (await steps()).filter((e) => e[1] === kinds.step).pop();
      row('Play again: the core goes on from where it stood', p.player.state === 3 && after[3] > before[3] && (await name()) === first, `step ${before[3]} -> ${after[3]}`);
    }

    // Next and Previous.
    await clickId('nextBtn');
    await sleep(3000);
    const second = await name();
    p = await probe();
    const asked = await ev(() => window.__rows.posts.filter((m) => m.type === 'playNext' && m.moment === 0).pop());
    {
      row('Next: the core\'s own loop, crossfading, the card following', second !== first && p.d.name === second && asked && asked.fade === 1, `${p.d.name} / ${second}`);
      const own = (await owns()).pop();
      const sent = await lastPicks();
      row('Next: the core says its own loop began, by its seed and the picks it took', own && own.seed === Number(p.d.seed) && sent && own.tag === sent.tag, JSON.stringify(own));
    }
    await clickId('prevBtn');
    await sleep(3000);
    p = await probe();
    row('Previous: back to the loop before', (await name()) === first && p.d.name === first, p.d.name);

    // Roll a layer (key 1, the drums): made again in place.
    await ev(() => document.getElementById('viewBtn').click());
    const seedsBefore = p.d.layer_seeds.split('|');
    const stepBefore = (await steps()).filter((e) => e[1] === kinds.step).pop()[3];
    await key('1');
    await sleep(2000);
    {
      p = await probe();
      const seeds = p.d.layer_seeds.split('|');
      const revised = (await steps()).some((e) => e[1] === kinds.revised);
      const stepAfter = (await steps()).filter((e) => e[1] === kinds.step).pop()[3];
      row('roll a layer: the drums re-rolled in place, the steps going on', seeds[0] !== seedsBefore[0] && seeds.slice(1).join() === seedsBefore.slice(1).join()
        && (p.d.rolled || '').split('|').includes('drums') && revised && stepAfter > stepBefore, `${seedsBefore[0]} -> ${seeds[0]}, rolled ${p.d.rolled}, step ${stepBefore} -> ${stepAfter}`);
    }

    // Mutes: all five off, then on.
    const mutes = () => ev(() => { for (const b of document.querySelectorAll('.layer .btn-on')) b.click(); });
    await mutes();
    // A tail still ringing (the room's reverb) is not yet silence: wait for
    // it to fall, up to twelve seconds, rather than read once.
    let muted = Infinity;
    for (let t = 0; t < 12000 && !(muted < 2e-3); t += 500) {
      await sleep(500);
      muted = await level(400);
    }
    await mutes();
    await sleep(1500);
    const back = await level(1500);
    row('mutes: all five layers off is silence, on again is sound', muted < 2e-3 && back > muted * 4, `${muted.toExponential(1)} -> ${back.toExponential(1)}`);

    // Tempo and tone.
    await setRange('bpm', 120);
    await setRange('warmth', 0.13);
    await setRange('space', 0.77);
    await setRange('wobble', 0.21);
    await sleep(2500);
    {
      p = await probe();
      const st = (await steps()).filter((e) => e[1] === kinds.step).slice(-6);
      const gaps = st.slice(1).map((e, i) => e[0] - st[i][0]);
      row('tempo: the core plays at the slider\'s tempo, its steps that far apart', Number(p.d.bpm) === 120 && gaps.every((g) => Math.abs(g - 60 / 120 / 4) < 1e-6), `${p.d.bpm} bpm, ${gaps.map((g) => g.toFixed(4)).join(' ')}`);
      row('tone: the core\'s warmth, space and wobble are the sliders\'', p.d.tone === '0.13|0.77|0.21', p.d.tone);
    }

    // Wander and its salt.
    await setRange('driftAmount', 2.4);
    await sleep(300);
    const wander = await ev(() => window.__rows.posts.filter((m) => m.type === 'playWander').pop());
    const wline = (await diag()).split('\n').find((l) => l.startsWith('wander:'));
    row('wander: the amount, and a salt across the upper range', wander && wander.amount === 2.4 && wander.salt > 0 && /unrepeatable/.test(wline), wline);
    await setRange('driftAmount', 0);
    await sleep(300);
    const off = await ev(() => window.__rows.posts.filter((m) => m.type === 'playWander').pop());
    row('wander off: none', off && off.amount === 0 && off.salt === 0);
    await setRange('driftAmount', 1);

    // Volume.
    await setRange('volume', 0);
    await sleep(1200);
    const silent = await level();
    await setRange('volume', 0.85);
    await sleep(1200);
    const loud = await level(1200);
    row('volume: 0 is silence, back is sound', silent < 1e-4 && loud > 1e-3, `${silent.toExponential(1)} -> ${loud.toExponential(1)}`);

    // Mono.
    await clickId('monoMode');
    await sleep(600);
    const mono = (await probe()).lanes;
    await clickId('monoMode');
    await sleep(600);
    const stereo = (await probe()).lanes;
    row('mono: one lane, then two again', mono === 1 && stereo === 2, `${mono}, ${stereo}`);

    // The layers' grid: the dots of the bar shown are a bar of the core's
    // grid of the pass heard.
    await ev(() => { if (document.querySelector('.view:not([hidden])').dataset.view !== 'layers') document.getElementById('viewBtn').click(); });
    await sleep(2500);
    {
      const res = await ev(() => {
        const R = window.__rows;
        const last = R.nodes.length - 1;
        const grids = R.events.filter((b) => b.node === last).flatMap((b) => b.grids.map((g) => g.grid)).slice(-3);
        const layers = ['drums', 'bass', 'chords', 'melody', 'texture'];
        const dots = layers.map((l) => [...document.querySelectorAll(`.layer[data-layer="${l}"] .pattern span`)].map((d) => (d.classList.contains('on') ? 1 : 0)));
        const spb = dots[0].length;
        for (const g of grids) {
          for (let bar = 0; bar < 64; bar++) {
            const fits = layers.every((l, i) => {
              const row = g[i];
              const cycle = row.length;
              return dots[i].every((on, k) => on === (row[(bar * spb + k) % cycle] > 0 ? 1 : 0));
            });
            if (fits) return { ok: true, bar };
          }
        }
        return { ok: false, grids: grids.length, dots: dots.map((d) => d.join('')).join(' ') };
      });
      row('layers\' grid: the bar shown is the core\'s grid of the pass', res.ok, JSON.stringify(res));
    }

    // Picks: a vibe, then Next.
    const playingSeed = (await probe()).d.seed;
    const vibe = await ev(() => {
      document.getElementById('vibesBtn').click();
      document.querySelector('#picksList .pk-line-mood .pk-val').click();
      const b = document.querySelector('#vibesList .vb-key');
      b.click();
      return b.dataset.vibe;
    });
    {
      await sleep(500);
      p = await probe();
      const sent = await lastPicks();
      const index = sent ? sent.ask.vibes.map(([i]) => i) : [];
      row('picks: sent as a command and held by the core, the loop playing left as it is', sent && p.picksTag === sent.tag && index.length === 1 && p.d.seed === playingSeed, `tag ${p.picksTag}, ${JSON.stringify(sent && sent.ask.vibes)}`);
    }
    // N, a new loop (Next would walk forward through the history first).
    await key('n');
    await sleep(3000);
    {
      p = await probe();
      const own = (await owns()).pop();
      row('picks: the next new loop is the core\'s own and leans on them', (p.d.vibes || '').split('|').some((x) => x.startsWith(`${vibe}:`)) && p.d.name === (await name())
        && own && own.seed === Number(p.d.seed), `${vibe}: ${p.d.vibes}`);
    }
    await ev(() => document.getElementById('vibesClear').click());

    // Export MIDI and a share code.
    await clickId('exportMidi');
    await sleep(500);
    const midi = await ev(async () => {
      const b = window.__rows.blobs.pop();
      if (!b) return null;
      const t = new Uint8Array(await b.arrayBuffer());
      return String.fromCharCode(...t.slice(0, 4));
    });
    row('export MIDI: a MIDI file', midi === 'MThd', String(midi));
    // The export's rows are the browser's (in the app, session 28's).
    if (!APP) await exportRows({ page, ev, probe, level, diag, clickId, browser });
    await clickId('copySong');
    await sleep(500);
    const code = await ev(() => window.__rows.clip);
    let shared = null;
    try { shared = decodeSong(code); } catch (err) { shared = { error: err.message }; }
    p = await probe();
    row('share code: it decodes to the loop the core plays', shared && shared.seed >>> 0 === Number(p.d.seed) && shared.name === p.d.name, `${code && code.slice(0, 24)}...`);

    // Lighter on processor: a new core at the other quality, the loop again.
    const nodes = await ev(() => window.__rows.nodes.length);
    const lite = await ev(() => document.getElementById('liteMode').getAttribute('aria-checked'));
    const card = await name();
    await clickId('liteMode');
    await sleep(3500);
    const built = await ev(() => window.__rows.nodes.slice(-1)[0].quality);
    {
      p = await probe();
      const loops = (await steps()).filter((e) => e[1] === kinds.loop);
      row('lighter on processor: a new core at the other quality plays the same loop from its top', (await ev(() => window.__rows.nodes.length)) === nodes + 1
        && built === (lite === 'true' ? 'full' : 'lite') && p.player.state === 3 && p.d.name === card && loops.length === 1, `${lite} -> ${built}`);
    }
    await clickId('liteMode');
    await sleep(3000);

    // Diagnostics.
    const text = await diag();
    const engineLine = text.split('\n').find((l) => l.startsWith('engine:')) || '';
    {
      row('Diagnostics: the core\'s player, nothing late, missed or dropped, no event lost, no edit refused', /the core's player\)\s+late: 0\s+missed: 0\s+dropped: 0/.test(engineLine)
        && /playerEventsLost: 0/.test(text) && /playerRefused: 0/.test(text), `${engineLine}; ${text.split('\n').filter((l) => /^player(EventsLost|Refused)/.test(l)).join(', ')}`);
    }
    if (APP) await appRows({ ev, diag, clickId, level, probe });

    // The length slider: the shortest stop, a minute.
    if (LONG) {
      await ev(() => document.getElementById('nowBtn').click());
      await sleep(500);
      await setRange('trackLen', 1);
      await sleep(800);
      p = await probe();
      const track = await ev(() => window.__rows.posts.filter((m) => m.type === 'playTrack').pop());
      const clockText = await ev(() => document.getElementById('nowBig').textContent);
      // At the end of the history nothing of the page's follows: the core
      // makes its own (queue item 95).
      row('length: the core counts the track, its own loop to follow (nothing of the page\'s armed)', p.trackEnd >= 0 && track && track.kind > 0 && track.arm === null && clockText.startsWith('−'), `end step ${p.trackEnd}, clock ${clockText}`);
      const before = (await owns()).length;
      const t0 = Date.now();
      let took = null;
      while (Date.now() - t0 < 150000 && !took) {
        await sleep(2000);
        const e = await steps();
        const at = e.findIndex((x) => x[1] === kinds.end);
        const own = (await owns()).slice(before)[0];
        took = at >= 0 && own ? own : null;
      }
      await sleep(1500);
      p = await probe();
      const sent = await lastPicks();
      row('length: the track ends on its line, the core\'s own loop starts there with the picks, the card follows', took && Number(p.d.seed) === took.seed && sent && took.tag === sent.tag
        && p.d.name === (await name()) && p.trackEnd < 0, `${Math.round((Date.now() - t0) / 1000)} s`);
      const after = (await diag()).split('\n').find((l) => l.startsWith('engine:'));
      row('length: nothing late across the hand-over', /late: 0\s+missed: 0\s+dropped: 0/.test(after), after);
    }
    // The picks survive a reload (kept as the vibes are, in the prefs), and
    // go to the new core as it starts.
    {
      await ev(() => {
        document.getElementById('vibesBtn').click();
        document.querySelector('#picksList .pk-line-mood .pk-val').click();
        document.querySelector('#vibesList .vb-key').click();
      });
      await sleep(300);
      const kept = await lastPicks();
      await page.reload();
      await ready();
      await sleep(1500);
      await page.click('#playBtn');
      await sleep(3000);
      p = await probe();
      const again = await lastPicks();
      row('picks: held again by the core after a reload', kept && again && p.picksTag === again.tag && JSON.stringify(again.ask.vibes) === JSON.stringify(kept.ask.vibes), JSON.stringify(again && again.ask.vibes));
    }
    row('no page errors', errors.length === 0, errors.join('; '));
    // A long piece on screen (queue item 106b), in a page of its own.
    if (!APP) await longRows(browser, PORT, errors);
    if (!APP) await plainRows(browser);
  } finally {
    if (browser) await browser.close();
    if (server) server.close();
  }
  report();
}

// The plain lines, each in a tab of its own.
async function plainRows(browser) {
  {
    // The plain line (queue item 97; Mikey, 2026-11-07): a browser that
    // can't run the core says so in the card's place and plays nothing;
    // there is no fallback synth. A page with WebAssembly taken away
    // before its scripts run, and one whose module never arrives.
    const plain = async (setup) => {
      const tab = await browser.newPage({ viewport: { width: 412, height: 915 } });
      const bad = [];
      tab.on('pageerror', (e) => bad.push(e.message));
      await setup(tab);
      await tab.goto(`http://127.0.0.1:${PORT}/`);
      await sleep(2500);
      const seen = await tab.evaluate(() => {
        const line = document.getElementById('plainLine');
        return { text: line.textContent.trim(), shown: !line.hidden && getComputedStyle(line).display !== 'none' };
      });
      await tab.close();
      return { ...seen, bad };
    };
    const none = await plain((tab) => tab.addInitScript(() => { Object.defineProperty(globalThis, 'WebAssembly', { value: undefined }); }));
    row('plain line: a browser with no WebAssembly is told it can\'t play', none.shown && none.text === 'This browser can\'t play Driftloom; try a recent Chrome, Firefox or Safari' && !none.bad.length, `${none.text} ${none.bad.join('; ')}`);
    const lost = await plain((tab) => tab.route('**/js/dlplayer.wasm', (r) => r.abort('failed')));
    row('plain line: a module that never arrives says to refresh', lost.shown && lost.text === 'Please refresh.' && !lost.bad.length, `${lost.text} ${lost.bad.join('; ')}`);
  }
}

function report() {
  const failed = rows.filter((r) => !r.ok);
  console.log(failed.length ? `\n${failed.length} of ${rows.length} failed` : `\nall ${rows.length} passed`);
  process.exit(failed.length ? 1 : 0);
}

await main();
