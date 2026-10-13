// A deep output buffer that appears when the page is hidden (queue item 47):
// the real app, playing in headless Chromium with the CPU throttled, through
// what a tester's Pixel 9 did -- minimized, locked, Next and Previous and
// Pause from the lock screen, frozen for a while, shown again -- while the
// core takes its messages as late as a phone that has moved a backgrounded
// app's audio to a deep, power-saving buffer would: from the moment the page
// is hidden until it is shown, every message reaches the core `--deep`
// seconds after it was posted, in audio time, so the core has already
// rendered that far past the page's clock (the Pixel 9's Diagnostics:
// 6-21 ms on screen, 1413 ms at worst). Shown again, the buffer drains: what
// is held still arrives in order, then everything at once again.
//
// Run with:  node tools/deep.mjs [options]
//
// Nothing in js/ is touched: js/worklet.js is served changed (as
// tools/views.mjs --delay does it) and the page instrumented from outside
// (tools/stalls.mjs's hook), so nothing here ships.
//
// The render-ahead is measured from the notes that land in it, and eases
// back down only slowly (item 41), so what is at risk is what comes before
// the first measurement: a key or a freeze just after the page is hidden.
// So each scenario runs on a fresh page (a fresh browser and profile, as a
// phone that has never been measured), the risky step at once:
//
//   next        hidden, then Next at once
//   previous    Next while visible (a loop to go back to), hidden, then
//               Previous at once
//   pause       hidden, then Pause at once and Play 0.6 s later
//   freeze      hidden, then frozen at once for 10 s (longer than the
//               hidden queue), as a locked phone may
//   tour        hidden for a while, then Next, Previous, Pause and Play,
//               frozen for 3 s and for 10 s, passes, shown, Next, hidden
//               and Next at once, shown and Next at once
//
// What is counted, per step (from its start to the next step's), as the
// listener has it: the notes that reached the core after their time (the
// core's own `late`), the ones it left out (more than 30 ms late, item 41),
// the scheduler's late ticks, and for a key, how long after it the first
// note decided by it starts (the cost of planning past the deep buffer).
//
// Options:
//   --loops <names>     shatter,hollow (default; a busy loop and a sung one)
//   --throttle <list>   CPU slowdown factors                 (default 4,6)
//   --quality <list>    full,lite                       (default full,lite)
//   --deep <s>          the deep buffer, hidden              (default 1.4)
//   --scenarios <list>  next,previous,pause,freeze,tour      (default all)
//   --report            print Diagnostics at the end of each session
//   --json <file>  --md <file>  --port <n>  --chrome <path>

import fs from 'node:fs';
import { LOOPS, findChrome, sleep, launch, startServer, openTab, loadCode } from './stalls.mjs';

function parseArgs(argv) {
  const o = {
    loops: ['shatter', 'hollow'], throttle: [4, 6], quality: ['full', 'lite'], deep: 1.4,
    scenarios: ['next', 'previous', 'pause', 'freeze', 'tour'],
    port: 8755, chrome: null, json: null, md: null, report: false,
  };
  const list = (s) => s.split(',').map((x) => x.trim()).filter(Boolean);
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    const next = () => argv[++i];
    switch (a) {
      case '--loops': o.loops = list(next()); break;
      case '--throttle': o.throttle = list(next()).map(Number); break;
      case '--quality': o.quality = list(next()); break;
      case '--deep': o.deep = Number(next()); break;
      case '--scenarios': o.scenarios = list(next()); break;
      case '--port': o.port = Number(next()); break;
      case '--chrome': o.chrome = next(); break;
      case '--json': o.json = next(); break;
      case '--md': o.md = next(); break;
      case '--report': o.report = true; break;
      default: console.error(`deep: unknown option ${a}`); process.exit(2);
    }
  }
  for (const l of o.loops) if (!LOOPS[l]) { console.error(`deep: no loop ${l} (${Object.keys(LOOPS).join(', ')})`); process.exit(2); }
  return o;
}

// The worklet, served so: a message is held until the audio clock has moved
// on by the deep buffer while it is on, and taken in the order it came.
// `__deep` turns it on and off, at once.
function deepWorklet(seconds) {
  return (url, text) => {
    if (url !== '/js/worklet.js') return null;
    const take = 'this.port.onmessage = (e) => this.receive(e.data);';
    const at = '  process(inputs, outputs) {';
    if (!text.includes(take) || !text.includes(at)) throw new Error('deep: cannot find its places in js/worklet.js');
    return text
      .replace(take, `this.held = []; this.deep = false;
    this.port.onmessage = (e) => {
      if (e.data && e.data.type === '__deep') { this.deep = e.data.on; return; }
      this.held.push([currentTime + (this.deep ? ${seconds} : 0), e.data]);
      if (!this.deep) while (this.held.length && this.held[0][0] <= currentTime) this.receive(this.held.shift()[1]);
    };`)
      .replace(at, `${at}\n    while (this.held && this.held.length && this.held[0][0] <= currentTime) this.receive(this.held.shift()[1]);`);
  };
}

// Before the page's own scripts, after the stalls hook: the quality asked
// for, and the deep buffer turned on as the page is hidden and off as it is
// shown, on every core node the page builds.
const extra = (lite) => `(() => {
  try {
    const p = JSON.parse(localStorage.getItem('driftloom.prefs.v1') || '{}');
    p.lite = ${lite}; p.view = 'now';
    localStorage.setItem('driftloom.prefs.v1', JSON.stringify(p));
  } catch {}
  const nodes = [];
  const Real = window.AudioWorkletNode;
  window.AudioWorkletNode = new Proxy(Real, {
    construct(t, args, n) { const node = Reflect.construct(t, args, n); nodes.push(node); node.port.postMessage({ type: '__deep', on: document.hidden }); return node; },
  });
  document.addEventListener('visibilitychange', () => {
    for (const node of nodes) node.port.postMessage({ type: '__deep', on: document.hidden });
  });
})();
//# sourceURL=deep-harness.js`;

const coreCounts = (diag) => {
  const e = String(diag.engine || '');
  const late = e.match(/late: (\d+)/);
  const missed = e.match(/left out (\d+)/);
  return { late: late ? Number(late[1]) : 0, missed: missed ? Number(missed[1]) : 0, ticks: Number(diag.lateTicks) || 0 };
};

async function session(opts, chromePath, loop, throttle, quality, scenario) {
  const b = await launch(chromePath);
  const rows = [];
  try {
    const tab = await openTab(b, `http://127.0.0.1:${opts.port}/`, throttle, { sources: [extra(quality === 'lite')] });
    await tab.press('#playBtn');
    await sleep(1500);
    const name = await loadCode(tab, LOOPS[loop].code);
    await sleep(3000);
    const diag0 = await tab.diagnostics();
    if (!/^rust/.test(String(diag0.engine))) throw new Error(`engine: ${diag0.engine}`);
    if (/lite: true/.test(diag0.text) !== (quality === 'lite')) throw new Error(`lite asked ${quality}: ${diag0.text.match(/lite: \w+/)}`);
    let blank = null;
    const hide = async () => {
      ({ targetId: blank } = await b.send('Target.createTarget', { url: 'about:blank', newWindow: false }));
      await b.send('Target.activateTarget', { targetId: blank });
      await sleep(300);
      const vis = await tab.evaluate('document.visibilityState');
      if (vis !== 'hidden') throw new Error(`the tab is ${vis}, not hidden`);
    };
    const show = async () => {
      await b.send('Target.activateTarget', { targetId: tab.targetId });
      if (blank) await b.send('Target.closeTarget', { targetId: blank }).catch(() => {});
      blank = null;
      await sleep(300);
    };
    // A media key, as the lock screen's and a headset's buttons send it; the
    // last one pressed is where the first new note is measured from (the
    // notes posted from then on, all decided by it or after it).
    let pressed = null;
    const key = async (code) => {
      pressed = await tab.evaluate(`(() => {
        const S = window.__stalls; const m = { n: S.notes.length, t: S.ctx.currentTime };
        document.body.dispatchEvent(new KeyboardEvent('keydown', { code: '${code}', bubbles: true }));
        return m;
      })()`);
    };
    const freeze = async (ms) => {
      await tab.send('Page.setWebLifecycleState', { state: 'frozen' });
      await sleep(ms);
      await tab.send('Page.setWebLifecycleState', { state: 'active' });
    };
    // Each step: what it does, then how long to listen.
    const hush = async () => {};
    const SCENARIOS = {
      next: [['visible', hush, 3000], ['hide, Next at once', async () => { await hide(); await key('MediaNextTrack'); }, 6000]],
      previous: [['Next, visible', () => key('MediaNextTrack'), 4000],
        ['hide, Previous at once', async () => { await hide(); await key('MediaPreviousTrack'); }, 6000]],
      pause: [['visible', hush, 3000], ['hide, Pause, Play', async () => {
        await hide(); await key('MediaPlayPause'); await sleep(600); await key('MediaPlayPause');
      }, 6000]],
      freeze: [['visible', hush, 3000], ['hide, freeze 10 s at once', async () => { await hide(); await freeze(10000); }, 6000]],
    };
    SCENARIOS.tour = [
      ['visible', async () => {}, 4000],
      ['hide', hide, 6000],
      ['Next, hidden', () => key('MediaNextTrack'), 6000],
      ['Previous, hidden', () => key('MediaPreviousTrack'), 6000],
      ['Pause, Play, hidden', async () => { await key('MediaPlayPause'); await sleep(600); await key('MediaPlayPause'); }, 6000],
      ['freeze 3 s', () => freeze(3000), 5000],
      ['freeze 10 s', () => freeze(10000), 6000],
      ['passes, hidden', async () => {}, 15000],
      ['show', show, 5000],
      ['Next, visible', () => key('MediaNextTrack'), 5000],
      ['hide, Next at once', async () => { await hide(); await key('MediaNextTrack'); }, 6000],
      ['show, Next at once', async () => { await show(); await key('MediaNextTrack'); }, 5000],
    ];
    for (const [label, act, wait] of SCENARIOS[scenario]) {
      const before = coreCounts(await tab.diagnostics());
      pressed = null;
      await act();
      await sleep(wait);
      const after = coreCounts(await tab.diagnostics());
      // The first note posted after the last key, and when it starts.
      const first = pressed && await tab.evaluate(`(() => {
        const fresh = window.__stalls.notes.slice(${pressed.n});
        return fresh.length ? Math.min(...fresh.map(([t]) => t)) : null;
      })()`);
      const row = {
        loop: name, throttle, quality, scenario, step: label,
        late: after.late - before.late, missed: after.missed - before.missed, lateTicks: after.ticks - before.ticks,
        firstMs: first != null ? Math.round((first - pressed.t) * 1000) : null,
      };
      rows.push(row);
      log(`  ${label}: late ${row.late} (left out ${row.missed}), late ticks ${row.lateTicks}${row.firstMs != null ? `, first new note ${row.firstMs} ms after the key` : ''}`);
    }
    if (opts.report) log((await tab.diagnostics()).text);
    return rows;
  } finally {
    await b.close();
  }
}

function log(s) { process.stderr.write(`${s}\n`); }

function table(rows) {
  const out = ['| loop | cpu | quality | step | late notes | left out | late ticks | first new note after the key, ms |', '|---|---|---|---|---|---|---|---|'];
  for (const r of rows) {
    out.push(r.error
      ? `| ${r.loop} | ${r.throttle}x | ${r.quality} | ${r.scenario} | ${r.error} | | | |`
      : `| ${r.loop.replace(/-.*/, '')} | ${r.throttle}x | ${r.quality} | ${r.step} | ${r.late} | ${r.missed} | ${r.lateTicks} | ${r.firstMs ?? '-'} |`);
  }
  return out.join('\n');
}

async function main() {
  const opts = parseArgs(process.argv.slice(2));
  const chromePath = findChrome(opts.chrome);
  const server = await startServer(opts.port, deepWorklet(opts.deep));
  const rows = [];
  try {
    for (const loop of opts.loops) for (const throttle of opts.throttle) for (const quality of opts.quality) for (const scenario of opts.scenarios) {
      log(`${loop} ${throttle}x ${quality} ${scenario}, deep buffer ${opts.deep} s while hidden`);
      try {
        rows.push(...await session(opts, chromePath, loop, throttle, quality, scenario));
      } catch (err) {
        log(`  failed: ${err.stack || err.message}`);
        rows.push({ loop, throttle, quality, scenario, error: err.message });
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
