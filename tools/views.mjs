// Glitches with the screen on (queue item 40): the real app, playing in
// headless Chromium with the tab visible and the CPU throttled, while the
// harness goes through the views as a listener would -- Now Playing, Vibes,
// the Library with 30 saved loops and 5 albums, an album open, every loop,
// Save, Share -- scrolling the long ones, opening an album, pasting a code
// and switching views quickly. Per view, what the main thread did and what
// a listener would hear.
//
// Run with:  node tools/views.mjs [options]
//
// Nothing in js/ is touched; the page is instrumented from outside, as
// tools/stalls.mjs does it (its hook is reused), plus:
//
//   long tasks   Long Animation Frames (PerformanceObserver 'long-animation-
//                frame'): frames that took over 50 ms, with the scripts in
//                them, so the table can say what they were. Rendering
//                (style, layout, paint) is what is left of a frame once its
//                scripts are taken out.
//
// What is counted, per view (from its first action to the end of its stay):
//
//   long tasks   how many, the longest (ms), and the biggest part of it
//   gap ms       the longest the page went without a clock tick
//   late notes   notes handed to the core after their start time (the
//                core's own `late` counter beside it, in brackets)
//   late ticks   the scheduler's counter (Diagnostics `lateTicks`)
//   fill-ins     frames the audio device had to fill (playoutStats): the
//                audio thread itself ran late, whatever the page did
//   dropped      notes the core dropped (Diagnostics' `dropped`, by cause
//                where the build reports it)
//   lead ms      how far ahead of its time the closest note was handed
//                over (its time less the context's clock as it was posted);
//                and, where the core reports it (queue item 41), how far
//                ahead of what the core had rendered it arrived
//   readbacks    canvas pixel reads and GPU flushes on the main thread:
//                getImageData, putImageData, toDataURL, toBlob, and
//                drawImage from a canvas -- how many, their total and the
//                longest, and how many of them touched a GPU-backed canvas
//                (one made without willReadFrequently)
//
// Options:
//   --loops <names>     shatter,hollow (default; a busy loop and a sung one)
//   --engines <list>    core (default), js
//   --throttle <list>   CPU slowdown factors                 (default 4,6)
//   --quality <list>    full,lite                       (default full,lite)
//   --changes           also time the changes that must be heard at once
//                       (mute, unmute, re-roll, tempo, wander, a vibe loop):
//                       from the key to the first note that reflects it
//   --report            print Diagnostics at the end of each session
//   --buffer <s>        a large, bursty audio output, as on the Pixel 10a
//                       (queue item 41): the context asks for this latency
//                       (a number for latencyHint), and Chromium renders in
//                       blocks of that size -- 0.37 gives 8192 frames, 186
//                       ms bursts and about 0.56 s of output latency.
//                       (--audio-buffer-size does not reach headless
//                       Chromium's output; this does.)
//   --graphics <list>   default, software (--disable-gpu), gpu (a GPU-backed
//                       canvas on SwiftShader, so readbacks cost what they
//                       do on a GPU), nooop (out-of-process canvas raster
//                       off)                                 (default default)
//   --delay <ms>        the core takes each message this long after it
//                       arrives, in audio time (js/worklet.js is served
//                       changed so): notes reach a core that has already
//                       rendered that far past the page's clock, whatever
//                       the reason on a phone (an audio thread that takes
//                       its messages late, a core whose count has run
//                       ahead). On the audio thread, so the page's own work
//                       does not add to it.
//   --phases <names>    only these views (by the start of their names)
//   --json <file>  --md <file>  --port <n>  --chrome <path>

import fs from 'node:fs';
import {
  LOOPS, findChrome, sleep, launch, startServer, INSTRUMENT, openTab, loadCode,
} from './stalls.mjs';

function parseArgs(argv) {
  const o = {
    loops: ['shatter', 'hollow'], engines: ['core'], throttle: [4, 6], quality: ['full', 'lite'],
    port: 8754, chrome: null, json: null, md: null, changes: false, report: false,
    buffer: null, graphics: ['default'], phases: null, delay: 0,
  };
  const list = (s) => s.split(',').map((x) => x.trim()).filter(Boolean);
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    const next = () => argv[++i];
    switch (a) {
      case '--loops': o.loops = list(next()); break;
      case '--engines': o.engines = list(next()); break;
      case '--throttle': o.throttle = list(next()).map(Number); break;
      case '--quality': o.quality = list(next()); break;
      case '--port': o.port = Number(next()); break;
      case '--chrome': o.chrome = next(); break;
      case '--json': o.json = next(); break;
      case '--md': o.md = next(); break;
      case '--changes': o.changes = true; break;
      case '--report': o.report = true; break;
      case '--buffer': o.buffer = Number(next()); break;
      case '--graphics': o.graphics = list(next()); break;
      case '--phases': o.phases = list(next()); break;
      case '--delay': o.delay = Number(next()); break;
      default: console.error(`views: unknown option ${a}`); process.exit(2);
    }
  }
  for (const l of o.loops) if (!LOOPS[l]) { console.error(`views: no loop ${l} (${Object.keys(LOOPS).join(', ')})`); process.exit(2); }
  for (const g of o.graphics) if (!GRAPHICS[g]) { console.error(`views: no graphics ${g} (${Object.keys(GRAPHICS).join(', ')})`); process.exit(2); }
  return o;
}

// Chromium's switches for each graphics mode. Headless Chromium draws a
// canvas in software unless told otherwise; `gpu` keeps a 2D canvas
// GPU-backed (on SwiftShader, a GPU in software), so a pixel read back from
// it flushes and copies as it does on a phone's GPU.
const GRAPHICS = {
  default: [],
  software: ['--disable-gpu', '--disable-gpu-compositing'],
  gpu: ['--use-angle=swiftshader', '--enable-unsafe-swiftshader', '--enable-gpu-rasterization', '--ignore-gpu-blocklist',
    '--enable-features=Canvas2DGPUAcceleration'],
  nooop: ['--disable-features=CanvasOopRasterization'],
};

// Before the page's own scripts, after the stalls hook: the quality asked
// for, Now Playing to start on, the long frames, the canvas readbacks, and
// the audio output's size when one is asked for.
const extra = (lite, buffer) => `(() => {
  const buffer = ${buffer == null ? 'null' : Number(buffer)};
  if (buffer) {
    const Inner = window.AudioContext;
    window.AudioContext = new Proxy(Inner, {
      construct(t, args, n) { return Reflect.construct(t, [{ ...(args[0] || {}), latencyHint: buffer }], n); },
    });
    window.webkitAudioContext = window.AudioContext;
  }
  // Every 2D context, and whether it is GPU-backed (made without
  // willReadFrequently), by its canvas.
  const gpu = new WeakMap();
  const canvases = [window.HTMLCanvasElement, window.OffscreenCanvas].filter(Boolean);
  for (const C of canvases) {
    const get = C.prototype.getContext;
    C.prototype.getContext = function (type, attrs) {
      const ctx = get.call(this, type, attrs);
      if (ctx && /2d/.test(type) && !gpu.has(this)) gpu.set(this, !(attrs && attrs.willReadFrequently));
      return ctx;
    };
  }
  const reads = window.__reads = [];
  const timed = (proto, name, gpuOf) => {
    if (!proto || !proto[name]) return;
    const f = proto[name];
    proto[name] = function (...a) {
      const t0 = performance.now();
      try { return f.apply(this, a); } finally {
        const ms = performance.now() - t0;
        reads.push([t0, name, ms, gpuOf(this, a) ? 1 : 0]);
      }
    };
  };
  const ownGpu = (ctx) => gpu.get(ctx.canvas) === true;
  const srcGpu = (ctx, a) => gpu.get(a[0]) === true && !ownGpu(ctx);
  for (const P of [window.CanvasRenderingContext2D, window.OffscreenCanvasRenderingContext2D].filter(Boolean)) {
    timed(P.prototype, 'getImageData', ownGpu);
    timed(P.prototype, 'putImageData', ownGpu);
    // From a canvas only (an image is decoded, not read back): a GPU canvas
    // drawn into a CPU one is a readback.
    const draw = P.prototype.drawImage;
    P.prototype.drawImage = function (...a) {
      const src = a[0];
      if (!(src instanceof HTMLCanvasElement || (window.OffscreenCanvas && src instanceof OffscreenCanvas))) return draw.apply(this, a);
      const t0 = performance.now();
      try { return draw.apply(this, a); } finally {
        reads.push([t0, 'drawImage', performance.now() - t0, srcGpu(this, a) ? 1 : 0]);
      }
    };
  }
  // The core's reports of how its notes arrived (queue item 41; nothing
  // on a build before it): the least time to spare, and how far past the
  // page's clock it had rendered.
  const early = window.__early = [];
  const desc = Object.getOwnPropertyDescriptor(MessagePort.prototype, 'onmessage');
  if (desc && desc.set) {
    Object.defineProperty(MessagePort.prototype, 'onmessage', {
      configurable: true,
      get() { return desc.get.call(this); },
      set(fn) {
        desc.set.call(this, typeof fn !== 'function' ? fn : function (e) {
          const m = e && e.data;
          if (m && m.type === 'arrivals') early.push([m.early, m.reach, m.lead]);
          return fn.call(this, e);
        });
      },
    });
  }
  timed(window.HTMLCanvasElement && HTMLCanvasElement.prototype, 'toDataURL', (c) => gpu.get(c) === true);
  timed(window.HTMLCanvasElement && HTMLCanvasElement.prototype, 'toBlob', (c) => gpu.get(c) === true);

  try {
    const k = 'driftloom.prefs.v1';
    const p = JSON.parse(localStorage.getItem(k) || '{}');
    localStorage.setItem(k, JSON.stringify({ ...p, lite: ${lite}, view: 'now', drift: true, driftAmount: 1 }));
  } catch { /* no storage */ }
  const S = window.__stalls;
  S.frames = [];
  try {
    new PerformanceObserver((list) => {
      for (const f of list.getEntries()) {
        const scripts = (f.scripts || []).map((s) => ({
          d: Math.round(s.duration),
          what: (s.sourceFunctionName || s.invoker || '?') + ' ' + String(s.sourceURL || '').replace(/^.*\\//, ''),
          inv: s.invokerType,
        }));
        S.frames.push({ at: f.startTime, d: Math.round(f.duration), block: Math.round(f.blockingDuration || 0),
          style: f.styleAndLayoutStart ? Math.round(f.startTime + f.duration - f.styleAndLayoutStart) : 0, scripts });
      }
    }).observe({ type: 'long-animation-frame', buffered: true });
  } catch { S.noLoaf = true; }
})();
//# sourceURL=views-harness.js`;

// Thirty saved loops and five albums of six, drawn by the app's own
// generator from a fixed seed, written where the app keeps them.
const SEED_LIBRARY = `(async () => {
  const g = await import('/js/generator.js');
  const store = await import('/js/storage.js');
  if (store.loadAll().length >= 30) return store.loadAll().length;
  const ids = [];
  for (let i = 0; i < 30; i++) {
    const e = store.save(g.newSpec(1000 + i * 7919));
    ids.push(e.id);
  }
  store.ensureAlbums();
  for (let a = 0; a < 5; a++) {
    const album = store.createAlbum(['Morning', 'Rain', 'Late train', 'Harbor', 'Study'][a]);
    store.setAlbumIds(album.id, ids.slice(a * 6, a * 6 + 6));
  }
  // Favorite Loops holds a few too, as anyone's would.
  for (const id of ids.slice(0, 8)) store.addToAlbum(store.FAVORITES, id);
  return store.loadAll().length;
})()`;

// ------------------------------------------------------------ the views

async function click(tab, selector) {
  const ok = await tab.evaluate(`(() => {
    const n = document.querySelector(${JSON.stringify(selector)});
    if (!n) return false;
    n.scrollIntoView({ block: 'nearest' });
    return true;
  })()`);
  if (!ok) throw new Error(`no ${selector}`);
  await tab.press(selector);
}

// Scroll the visible list down and back up, with a finger (a touch scroll
// gesture, as a phone sends it), so the compositor does what it would.
async function scroll(tab, distance = 900) {
  const box = await tab.evaluate(`(() => {
    const sc = [...document.querySelectorAll('.view:not([hidden]) .scroll, .view:not([hidden]) .vb-list')]
      .find((s) => s.offsetParent && s.clientHeight > 40);
    if (!sc) return null;
    const r = sc.getBoundingClientRect();
    return { x: r.left + r.width / 2, y: r.top + r.height / 2 };
  })()`);
  if (!box) return false;
  for (const d of [-distance, distance]) {
    try {
      await tab.send('Input.synthesizeScrollGesture', {
        x: Math.round(box.x), y: Math.round(box.y), yDistance: d, speed: 1200, gestureSourceType: 'touch', repeatCount: 1,
      });
    } catch {
      await tab.send('Input.dispatchMouseEvent', { type: 'mouseWheel', x: box.x, y: box.y, deltaX: 0, deltaY: -d });
    }
    await sleep(400);
  }
  return true;
}

async function view(tab, name) {
  return tab.evaluate(`(() => {
    const want = ${JSON.stringify(name)};
    const shown = [...document.querySelectorAll('.view')].find((v) => !v.hidden);
    return shown ? shown.dataset.view === want : false;
  })()`);
}

async function go(tab, name) {
  if (await view(tab, name)) return;
  const key = { library: '#libraryBtn', save: '#saveBtn', now: '#nowBtn', vibes: '#vibesBtn' }[name];
  if (name === 'vibes' && !(await view(tab, 'now'))) await go(tab, 'now');
  if (key) await click(tab, key);
  else {
    // Sound and Layers: the switch, once or twice.
    for (let i = 0; i < 3 && !(await view(tab, name)); i++) await click(tab, '#viewBtn');
  }
  if (!(await view(tab, name))) throw new Error(`could not reach ${name}`);
}

const PHASES = [
  { view: 'now playing', run: async (tab) => { await go(tab, 'now'); await sleep(8000); } },
  { view: 'vibes', run: async (tab) => { await go(tab, 'vibes'); await sleep(1500); await scroll(tab, 500); await sleep(4000); } },
  { view: 'library: albums', run: async (tab) => {
    await go(tab, 'library');
    await click(tab, '[data-tab="albums"]');
    await sleep(1500); await scroll(tab, 500); await sleep(4000);
  } },
  { view: 'library: an album open', run: async (tab) => {
    await click(tab, '#libPane .lrow.album:nth-of-type(3) .lrow-main');
    await sleep(1500); await scroll(tab, 600); await sleep(2000);
    // Back up, and into another.
    await click(tab, '#libPane .page-head .back');
    await sleep(800);
    await click(tab, '#libPane .lrow.album:nth-of-type(5) .lrow-main');
    await sleep(3000);
  } },
  { view: 'library: every loop', run: async (tab) => {
    await click(tab, '[data-tab="loops"]');
    await sleep(1500); await scroll(tab, 1500); await sleep(4000);
  } },
  { view: 'save', run: async (tab) => {
    await go(tab, 'save');
    await click(tab, '[data-stab="albums"]');
    await sleep(1500); await scroll(tab, 400); await sleep(4000);
  } },
  { view: 'share, a code pasted', run: async (tab, loop) => {
    await click(tab, '[data-stab="share"]');
    await sleep(2000);
    // The playing loop's own code: it is saved (in Imported Loops), so it
    // plays again from where it lives, as a pasted code does. Tapped, so
    // the long frame can say what ran.
    await tab.evaluate(`(() => {
      const code = ${JSON.stringify(LOOPS[loop].code)};
      Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { readText: async () => code, writeText: async () => {} } });
      return true;
    })()`);
    await click(tab, '#pasteCode');
    await sleep(5000);
  } },
  { view: 'sounds', run: async (tab) => { await go(tab, 'sounds'); await sleep(1500); await scroll(tab, 600); await sleep(3000); } },
  { view: 'layers', run: async (tab) => { await go(tab, 'layers'); await sleep(6000); } },
  { view: 'switching quickly', run: async (tab) => {
    for (let i = 0; i < 2; i++) {
      for (const v of ['library', 'save', 'now', 'library', 'save', 'sounds', 'layers', 'now']) {
        await go(tab, v);
        await sleep(250);
      }
    }
    await sleep(3000);
  } },
];

// ------------------------------------------------------------ the figures

function longTasks(frames) {
  const long = frames.filter((f) => f.d > 50);
  if (!long.length) return { count: 0, longest: 0, what: '' };
  const worst = long.reduce((a, b) => (b.d > a.d ? b : a));
  const script = worst.scripts.reduce((a, b) => (b.d > (a ? a.d : -1) ? b : a), null);
  const scripted = worst.scripts.reduce((t, s) => t + s.d, 0);
  const what = script && script.d >= (worst.d - scripted)
    ? `${script.what.trim()} ${script.d} ms`
    : `rendering ${Math.max(0, worst.d - scripted)} ms${script ? `, ${script.what.trim()} ${script.d}` : ''}`;
  return { count: long.length, longest: worst.d, what };
}

// The core's dropped notes, from Diagnostics' engine line, and by cause
// where the build says (queue item 41).
function dropped(diag) {
  const e = String(diag.engine || '');
  const m = e.match(/dropped: (\d+)/);
  return m ? Number(m[1]) : 0;
}

function readbacks(list) {
  if (!list.length) return { n: 0, ms: 0, worst: 0, gpu: 0, gpuMs: 0 };
  let ms = 0; let worst = 0; let g = 0; let gpuMs = 0;
  for (const [, , d, onGpu] of list) {
    ms += d; worst = Math.max(worst, d);
    if (onGpu) { g++; gpuMs += d; }
  }
  return { n: list.length, ms: Math.round(ms), worst: Math.round(worst), gpu: g, gpuMs: Math.round(gpuMs) };
}

async function phase(tab, engine, ph, loop) {
  await tab.diagnostics(true);
  const before = await tab.diagnostics();
  const mark = await tab.evaluate(`(() => { const S = window.__stalls; return { ...S.mark(), frames: S.frames.length, reads: window.__reads.length,
    early: (window.__early || []).length }; })()`);
  await ph.run(tab, loop);
  const diag = await tab.diagnostics();
  const after = await tab.evaluate(`(() => { const S = window.__stalls; const m = ${JSON.stringify(mark)};
    return { ...S.since(m), frames: S.frames.slice(m.frames), reads: window.__reads.slice(m.reads),
      early: (window.__early || []).slice(m.early) }; })()`);
  const events = engine === 'core' ? after.notes : after.starts;
  // Notes handed over during the phase, late or not.
  const handed = events.filter(([, h]) => h >= mark.ctx);
  const late = engine === 'core'
    ? handed.filter(([t, h]) => t < h).length
    : new Set(handed.filter(([t, h]) => t < h).map(([t]) => Math.round(t * 1000))).size;
  let gap = 0;
  for (let i = 1; i < after.ticks.length; i++) gap = Math.max(gap, after.ticks[i] - after.ticks[i - 1]);
  const lt = longTasks(after.frames);
  const leads = handed.map(([t, h]) => t - h);
  const early = after.early.map(([e]) => e);
  const reach = after.early.map(([, r]) => r).filter((r) => r != null);
  return {
    reachMs: reach.length ? Math.round(Math.max(...reach) * 1000) : null,
    dropped: dropped(diag) - dropped(before),
    minLeadMs: leads.length ? Math.round(Math.min(...leads) * 1000) : null,
    minEarlyMs: early.length ? Math.round(Math.min(...early) * 1000) : null,
    reads: readbacks(after.reads),
    view: ph.view,
    longTasks: lt.count, longestMs: lt.longest, what: lt.what,
    gapMs: Math.round(gap),
    lateNotes: late,
    coreLate: before.coreLate != null && diag.coreLate != null ? diag.coreLate - before.coreLate : null,
    lateTicks: diag.lateTicks, worstLateMs: diag.worstLateMs,
    fills: after.fills != null && mark.fills != null ? after.fills - mark.fills : null,
    notes: handed.length,
  };
}

// The changes that must be heard at once (queue item 40, step 2), from the
// key (on the context's clock):
//   heard from   when the music has the change: the time the core's queue is
//                cut from and decided again (a cancel from a time); with
//                none, the first note decided anew
//   first note   the first note handed over after the key that starts after
//                it, i.e. the first note decided anew (it waits for the
//                loop's next note as well)
// A mute's channel gain moves at the key besides, whatever the queue holds.
async function changes(tab, engine, loop) {
  const rows = [];
  const at = () => tab.evaluate('window.__stalls.ctx.currentTime');
  // The core's late notes over each change (queue item 41): a change heard
  // from inside the render-ahead lands late.
  const coreLate = async () => (await tab.diagnostics()).coreLate;
  let lateMark = null;
  const firstAfter = async (t0) => {
    await sleep(1500);
    const lateNow = await coreLate();
    const late = lateMark != null && lateNow != null ? lateNow - lateMark : null;
    const S = await tab.evaluate('(() => { const S = window.__stalls; return { notes: S.notes, starts: S.starts, froms: S.froms || [] }; })()');
    const ev = engine === 'core' ? S.notes : S.starts;
    const fresh = ev.filter(([t, h]) => h >= t0 && t >= t0).map(([t]) => t).sort((a, b) => a - b);
    const first = fresh.length ? Math.round((fresh[0] - t0) * 1000) : null;
    const cut = S.froms.find(([sent]) => sent >= t0);
    return { ms: cut ? Math.round((cut[1] - t0) * 1000) : first, first, late };
  };
  await go(tab, 'layers');
  await sleep(2000);
  const layerKey = (layer, cls) => `.layer[data-layer="${layer}"] ${cls}`;
  const steps = [
    ['mute melody', layerKey('melody', '.btn-on')],
    ['unmute melody', layerKey('melody', '.btn-on')],
    ['re-roll drums', layerKey('drums', '.btn-roll, .btn-reroll, [data-act="reroll"]')],
  ];
  for (const [label, sel] of steps) {
    await sleep(2500);
    lateMark = await coreLate();
    const t0 = await at();
    try { await click(tab, sel); } catch (err) { rows.push({ label, error: err.message }); continue; }
    rows.push({ label, ...await firstAfter(t0) });
  }
  await go(tab, 'sounds');
  for (const [label, id, value] of [['tempo', 'bpm', null], ['wander off', 'driftAmount', 0], ['wander on', 'driftAmount', 1]]) {
    await sleep(2500);
    lateMark = await coreLate();
    const t0 = await at();
    await tab.evaluate(`(() => { const r = document.getElementById(${JSON.stringify(id)});
      r.value = ${value == null ? 'String(parseInt(r.value, 10) + 12)' : JSON.stringify(String(value))};
      r.dispatchEvent(new Event('input', { bubbles: true })); return true; })()`);
    rows.push({ label, ...await firstAfter(t0) });
  }
  // A vibe loop starting: pick a vibe, then Next makes a new loop.
  await go(tab, 'vibes');
  await click(tab, '#vibesList .vb-key');
  await go(tab, 'now');
  await sleep(2500);
  lateMark = await coreLate();
  const t0 = await at();
  await click(tab, '#nextBtn');
  rows.push({ label: 'a vibe loop (Next)', ...await firstAfter(t0) });
  for (const r of rows) log(`  change ${r.label}: ${r.error || `heard from ${r.ms} ms, first new note ${r.first} ms, late notes ${r.late ?? '-'}`}`);
  return rows.map((r) => ({ ...r, loop }));
}

async function session(opts, chromePath, loop, engine, throttle, quality, graphics) {
  const b = await launch(chromePath, GRAPHICS[graphics]);
  try {
    const url = `http://127.0.0.1:${opts.port}/${engine === 'js' ? '?engine=js' : ''}`;
    // A phone's screen, so the views lay out (and paint) as on one.
    const tab = await openTab(b, url, throttle, {
      sources: [extra(quality === 'lite', opts.buffer)],
      setup: async (send) => {
        await send('Emulation.setDeviceMetricsOverride', { width: 412, height: 915, deviceScaleFactor: 2.625, mobile: true });
        await send('Emulation.setTouchEmulationEnabled', { enabled: true });
      },
    });
    await tab.evaluate(SEED_LIBRARY);
    await tab.press('#playBtn');
    await sleep(1500);
    const name = await loadCode(tab, LOOPS[loop].code);
    await sleep(3000);
    const diag = await tab.diagnostics();
    const lite = /lite: true/.test(diag.text);
    if (lite !== (quality === 'lite')) throw new Error(`lite is ${lite}, asked for ${quality}`);
    if (engine === 'core' && !/^rust/.test(String(diag.engine))) throw new Error(`engine: ${diag.engine}`);
    const out = await tab.evaluate(`(() => { const c = window.__stalls.ctx; return { base: c.baseLatency, out: c.outputLatency, rate: c.sampleRate }; })()`);
    log(`  audio: baseLatency ${out.base.toFixed(4)} s, outputLatency ${out.out.toFixed(3)} s, ${out.rate} Hz; graphics ${graphics}`);
    const rows = [];
    const phases = opts.phases ? PHASES.filter((p) => opts.phases.some((n) => p.view.startsWith(n))) : PHASES;
    for (const ph of phases) {
      let r;
      try {
        r = await phase(tab, engine, ph, loop);
      } catch (err) {
        log(`  ${ph.view}: failed: ${err.message}`);
        rows.push({ loop: name, engine, throttle, quality, graphics, view: ph.view, error: err.message });
        continue;
      }
      rows.push({ loop: name, engine, throttle, quality, graphics, ...r });
      log(`  ${r.view}: long tasks ${r.longTasks} (longest ${r.longestMs} ms${r.what ? `, ${r.what}` : ''}), gap ${r.gapMs} ms, late notes ${r.lateNotes}${r.coreLate != null ? ` (core ${r.coreLate})` : ''}, dropped ${r.dropped}, late ticks ${r.lateTicks} (worst ${r.worstLateMs} ms), fill-ins ${r.fills}, lead ${r.minLeadMs} ms${r.minEarlyMs != null ? ` (core ${r.minEarlyMs}, render-ahead ${r.reachMs})` : ''}, readbacks ${r.reads.n} (${r.reads.ms} ms, longest ${r.reads.worst}; GPU ${r.reads.gpu}, ${r.reads.gpuMs} ms)`);
    }
    if (opts.changes) for (const c of await changes(tab, engine, loop)) rows.push({ loop: name, engine, throttle, quality, graphics, change: true, ...c });
    if (opts.report) {
      await go(tab, 'library');
      log((await tab.diagnostics()).text);
    }
    return rows;
  } finally {
    await b.close();
  }
}

function log(s) { process.stderr.write(`${s}\n`); }

function table(rows) {
  const head = ['loop', 'cpu', 'quality', 'graphics', 'view', 'long tasks', 'longest ms', 'what', 'longest tick gap ms', 'late notes (core)', 'dropped',
    'late ticks', 'fill-ins', 'closest lead ms (at the core)', 'readbacks: n / ms / longest (GPU n)'];
  const out = [`| ${head.join(' | ')} |`, `|${head.map(() => '---').join('|')}|`];
  const changed = rows.filter((r) => r.change);
  for (const r of rows.filter((x) => !x.change)) {
    const cells = r.error
      ? [r.loop, `${r.throttle}x`, r.quality, r.graphics || '-', r.view || 'session', r.error, '', '', '', '', '', '', '', '', '']
      : [r.loop.replace(/-.*/, ''), `${r.throttle}x`, r.quality, r.graphics, r.view, r.longTasks, r.longestMs, r.what || '-', r.gapMs,
        `${r.lateNotes}${r.coreLate != null ? ` (${r.coreLate})` : ''}`, r.dropped, r.lateTicks, r.fills ?? '-',
        `${r.minLeadMs ?? '-'}${r.minEarlyMs != null ? ` (${r.minEarlyMs})` : ''}`,
        `${r.reads.n} / ${r.reads.ms} / ${r.reads.worst} (${r.reads.gpu})`];
    out.push(`| ${cells.join(' | ')} |`);
  }
  if (changed.length) {
    out.push('', '| loop | cpu | quality | graphics | change | heard from, ms | first new note, ms | late notes (core) |', '|---|---|---|---|---|---|---|---|');
    for (const r of changed) {
      out.push(`| ${r.loop.replace(/-.*/, '')} | ${r.throttle}x | ${r.quality} | ${r.graphics} | ${r.label} | ${r.error || r.ms} | ${r.first ?? '-'} | ${r.late ?? '-'} |`);
    }
  }
  return out.join('\n');
}

// --delay: the worklet holds each message until the audio clock has moved
// on by the delay, then takes it, before rendering the block.
function delayWorklet(ms) {
  if (!ms) return null;
  return (url, text) => {
    if (url !== '/js/worklet.js') return null;
    const take = 'this.port.onmessage = (e) => this.receive(e.data);';
    const at = '  process(inputs, outputs) {';
    if (!text.includes(take) || !text.includes(at)) throw new Error('views: --delay cannot find its places in js/worklet.js');
    return text
      .replace(take, `this.held = []; this.port.onmessage = (e) => this.held.push([currentTime + ${ms / 1000}, e.data]);`)
      .replace(at, `${at}\n    while (this.held && this.held.length && this.held[0][0] <= currentTime) this.receive(this.held.shift()[1]);`);
  };
}

async function main() {
  const opts = parseArgs(process.argv.slice(2));
  const chromePath = findChrome(opts.chrome);
  const server = await startServer(opts.port, delayWorklet(opts.delay));
  const rows = [];
  try {
    for (const graphics of opts.graphics) for (const loop of opts.loops) for (const engine of opts.engines) for (const throttle of opts.throttle) for (const quality of opts.quality) {
      log(`${loop} ${engine} ${throttle}x ${quality} ${graphics}${opts.buffer ? ` buffer ${opts.buffer}` : ''}${opts.delay ? ` delay ${opts.delay} ms` : ''}`);
      try {
        rows.push(...await session(opts, chromePath, loop, engine, throttle, quality, graphics));
      } catch (err) {
        log(`  failed: ${err.stack || err.message}`);
        rows.push({ loop, engine, throttle, quality, graphics, error: err.message });
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
