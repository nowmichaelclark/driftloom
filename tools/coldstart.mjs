// The first loop after the app starts (queue item 76): the real app in
// headless Chromium, started from nothing for each run, Play pressed, the
// first loop played to its end, then Diagnostics' late notes read. A phone
// whose core renders further past the clock than the first notes were
// planned for (item 41's render-ahead, unmeasured until a note reaches the
// core) loses those notes: late, left out.
//
// Run with:  node tools/coldstart.mjs [options]
//
//   --runs <n>          fresh starts per quality (default 3; each is a new
//                       random first loop)
//   --quality <list>    full,lite                       (default full,lite)
//   --buffer <s>        a large, bursty audio output, as views.mjs --buffer
//   --delay <ms>        the core takes each message this long after it
//                       arrives, in audio time, as views.mjs --delay: notes
//                       reach a core already that far past the page's clock
//   --seconds <n>       how long to play (default 12, past the first loop's
//                       start; the late notes of item 76 came at 3.8 s)
//   --throttle <n>      CPU slowdown                              (default 4)
//   --report            print the late-note log of each run
//   --port <n>  --chrome <path>
//
// Per run: the late notes the core counted, the log's entries and whether
// each was planned from the cold-start margin or from a measurement
// (Diagnostics' "decided by ... [margin]" or "[measured]"). Exit 1 if any
// run lost a note.

import { findChrome, sleep, launch, startServer, openTab } from './stalls.mjs';

function parseArgs(argv) {
  const o = { runs: 3, quality: ['full', 'lite'], buffer: null, delay: 0, seconds: 12, throttle: 4, port: 8755, chrome: null, report: false };
  const list = (s) => s.split(',').map((x) => x.trim()).filter(Boolean);
  for (let i = 0; i < argv.length; i++) {
    const next = () => argv[++i];
    switch (argv[i]) {
      case '--runs': o.runs = Number(next()); break;
      case '--quality': o.quality = list(next()); break;
      case '--buffer': o.buffer = Number(next()); break;
      case '--delay': o.delay = Number(next()); break;
      case '--seconds': o.seconds = Number(next()); break;
      case '--throttle': o.throttle = Number(next()); break;
      case '--port': o.port = Number(next()); break;
      case '--chrome': o.chrome = next(); break;
      case '--report': o.report = true; break;
      default: console.error(`coldstart: unknown option ${argv[i]}`); process.exit(2);
    }
  }
  return o;
}

// The context asks for this latency, so Chromium renders in blocks of it.
const bufferHook = (buffer) => `(() => {
  const Inner = window.AudioContext;
  window.AudioContext = new Proxy(Inner, {
    construct(t, args, n) { return Reflect.construct(t, [{ ...(args[0] || {}), latencyHint: ${Number(buffer)} }], n); },
  });
  window.webkitAudioContext = window.AudioContext;
})()`;

// js/worklet.js served so it takes each message `ms` of audio time late
// (the same change as tools/views.mjs --delay).
function delayWorklet(ms) {
  if (!ms) return null;
  return (url, text) => {
    if (url !== '/js/worklet.js') return null;
    const take = 'this.port.onmessage = (e) => this.receive(e.data);';
    const at = '  process(inputs, outputs) {';
    if (!text.includes(take) || !text.includes(at)) throw new Error('coldstart: --delay cannot find its places in js/worklet.js');
    return text
      .replace(take, `this.held = []; this.port.onmessage = (e) => this.held.push([currentTime + ${ms / 1000}, e.data]);`)
      .replace(at, `${at}\n    while (this.held && this.held.length && this.held[0][0] <= currentTime) this.receive(this.held.shift()[1]);`);
  };
}

async function run(o, chromePath, quality) {
  const b = await launch(chromePath);
  try {
    const tab = await openTab(b, `http://127.0.0.1:${o.port}/`, o.throttle, {
      sources: o.buffer ? [bufferHook(o.buffer)] : [],
    });
    if (quality === 'lite') {
      const on = await tab.evaluate("document.getElementById('liteMode').checked");
      if (!on) await tab.evaluate("document.getElementById('liteMode').click()");
    }
    await tab.press('#playBtn');
    await sleep(o.seconds * 1000);
    const d = await tab.diagnostics();
    const lines = d.text.split('\n');
    const from = lines.findIndex((l) => l.startsWith('late notes and render-ahead jumps'));
    const log = from < 0 ? [] : lines.slice(from + 1).filter((l) => l.startsWith('  ') && l.includes('late note, '));
    return { late: d.coreLate, engine: d.engine, log, render: lines.find((l) => l.startsWith('renderAheadMs')) };
  } finally {
    await b.close();
  }
}

async function main() {
  const o = parseArgs(process.argv.slice(2));
  const chromePath = findChrome(o.chrome);
  const server = await startServer(o.port, delayWorklet(o.delay));
  let lost = 0;
  try {
    for (const quality of o.quality) {
      for (let i = 1; i <= o.runs; i++) {
        const r = await run(o, chromePath, quality);
        const none = r.late === 0;
        if (!none) lost++;
        console.log(`${quality} #${i}: late ${r.late}${r.log.length ? `, ${r.log.length} in the log` : ''}  ${r.render || ''}`);
        if (o.report || !none) for (const l of r.log) console.log(l);
      }
    }
  } finally {
    server.close();
  }
  process.exit(lost ? 1 : 0);
}

await main();
