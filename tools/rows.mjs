// The app on the core's player (queue item 94b): a check for each row of
// item 94a's table (every control and display against its command or
// event), in the real app, in headless Chromium, playing in real time.
//
//   node tools/rows.mjs [--long] [--port 8771]
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
//   Play                the core plays the loop on the card (its name)
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
//   share code          the code decodes to the loop the core plays
//   lighter on processor  a new core at the other quality, the same loop
//   Diagnostics         the core's player, nothing late, nothing lost
//   plain line          a browser with no WebAssembly, and a module that
//                       never arrives: one plain line each, nothing played
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
        if (m && m.type === 'events') R.events.push({ node: R.nodes.length - 1, at: ctx.currentTime, events: m.events, grids: m.grids || [] });
        if (m && m.type === 'probe' && R.probes.has(m.id)) { R.probes.get(m.id)(m); R.probes.delete(m.id); }
      });
      this.port.start();
    }
  };
  const post = MessagePort.prototype.postMessage;
  MessagePort.prototype.postMessage = function (m, ...rest) {
    if (m && typeof m.type === 'string' && /^play|^layout/.test(m.type)) R.posts.push({ ...m, at: R.ctx ? R.ctx.currentTime : 0 });
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

async function main() {
  const { chromium } = loadPlaywright();
  const server = await serve(PORT);
  const browser = await chromium.launch({ args: ['--autoplay-policy=no-user-gesture-required'] });
  const errors = [];
  try {
    const page = await browser.newPage({ viewport: { width: 412, height: 915 } });
    page.on('pageerror', (e) => errors.push(e.message));
    await page.addInitScript(HOOK);
    await page.goto(`http://127.0.0.1:${PORT}/`);
    await page.waitForFunction(() => document.readyState === 'complete' && document.getElementById('playBtn'));
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
    // Play.
    await page.click('#playBtn');
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
    // The first loop is drawn at random, and a sparse one can rest for a
    // second: listen for six.
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
    await sleep(1800);
    const muted = await level(800);
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
      await page.waitForFunction(() => document.readyState === 'complete' && document.getElementById('playBtn'));
      await sleep(1500);
      await page.click('#playBtn');
      await sleep(3000);
      p = await probe();
      const again = await lastPicks();
      row('picks: held again by the core after a reload', kept && again && p.picksTag === again.tag && JSON.stringify(again.ask.vibes) === JSON.stringify(kept.ask.vibes), JSON.stringify(again && again.ask.vibes));
    }
    row('no page errors', errors.length === 0, errors.join('; '));

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
    row('plain line: a module that never arrives says to reload', lost.shown && lost.text === 'Driftloom couldn\'t load. Check the connection and reload.' && !lost.bad.length, `${lost.text} ${lost.bad.join('; ')}`);
  } finally {
    await browser.close();
    server.close();
  }
  const failed = rows.filter((r) => !r.ok);
  console.log(failed.length ? `\n${failed.length} of ${rows.length} failed` : `\nall ${rows.length} passed`);
  process.exit(failed.length ? 1 : 0);
}

await main();
