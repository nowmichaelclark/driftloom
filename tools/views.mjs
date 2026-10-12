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
//   --json <file>  --md <file>  --port <n>  --chrome <path>

import fs from 'node:fs';
import {
  LOOPS, findChrome, sleep, launch, startServer, INSTRUMENT, openTab, loadCode,
} from './stalls.mjs';

function parseArgs(argv) {
  const o = {
    loops: ['shatter', 'hollow'], engines: ['core'], throttle: [4, 6], quality: ['full', 'lite'],
    port: 8754, chrome: null, json: null, md: null, changes: false, report: false,
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
      default: console.error(`views: unknown option ${a}`); process.exit(2);
    }
  }
  for (const l of o.loops) if (!LOOPS[l]) { console.error(`views: no loop ${l} (${Object.keys(LOOPS).join(', ')})`); process.exit(2); }
  return o;
}

// Before the page's own scripts, after the stalls hook: the quality asked
// for, Now Playing to start on, and the long frames.
const extra = (lite) => `(() => {
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
      const code = ${'${JSON.stringify(LOOPS[loop].code)}'};
      Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { readText: async () => code, writeText: async () => {} } });
      return true;
    })()`);
    await click(tab, '#pasteCode');
    await sleep(5000);
  } },
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

async function phase(tab, engine, ph, loop) {
  await tab.diagnostics(true);
  const before = await tab.diagnostics();
  const mark = await tab.evaluate(`(() => { const S = window.__stalls; return { ...S.mark(), frames: S.frames.length }; })()`);
  await ph.run(tab, loop);
  const diag = await tab.diagnostics();
  const after = await tab.evaluate(`(() => { const S = window.__stalls; const m = ${JSON.stringify(mark)};
    return { ...S.since(m), frames: S.frames.slice(m.frames) }; })()`);
  const events = engine === 'core' ? after.notes : after.starts;
  // Notes handed over during the phase, late or not.
  const handed = events.filter(([, h]) => h >= mark.ctx);
  const late = engine === 'core'
    ? handed.filter(([t, h]) => t < h).length
    : new Set(handed.filter(([t, h]) => t < h).map(([t]) => Math.round(t * 1000))).size;
  let gap = 0;
  for (let i = 1; i < after.ticks.length; i++) gap = Math.max(gap, after.ticks[i] - after.ticks[i - 1]);
  const lt = longTasks(after.frames);
  return {
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

// The changes that must be heard at once (queue item 40, step 2): from the
// key (on the context's clock) to the first note handed over after it that
// starts at or after the key, i.e. the first note decided anew. For a mute,
// the channel's gain moves at once and nothing new needs deciding, so the
// figure is how soon the layer is silent: the time of the gain change.
async function changes(tab, engine, loop) {
  const rows = [];
  const at = () => tab.evaluate('window.__stalls.ctx.currentTime');
  const firstAfter = async (t0) => {
    await sleep(1500);
    const S = await tab.evaluate('(() => { const S = window.__stalls; return { notes: S.notes, starts: S.starts, params: S.params || [] }; })()');
    const ev = engine === 'core' ? S.notes : S.starts;
    const fresh = ev.filter(([t, h]) => h >= t0 && t >= t0).map(([t]) => t).sort((a, b) => a - b);
    return fresh.length ? Math.round((fresh[0] - t0) * 1000) : null;
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
    const t0 = await at();
    try { await click(tab, sel); } catch (err) { rows.push({ label, error: err.message }); continue; }
    rows.push({ label, ms: await firstAfter(t0) });
  }
  await go(tab, 'sounds');
  for (const [label, id, value] of [['tempo', 'bpm', null], ['wander off', 'driftAmount', 0], ['wander on', 'driftAmount', 1]]) {
    await sleep(2500);
    const t0 = await at();
    await tab.evaluate(`(() => { const r = document.getElementById(${JSON.stringify(id)});
      r.value = ${value == null ? 'String(parseInt(r.value, 10) + 12)' : JSON.stringify(String(value))};
      r.dispatchEvent(new Event('input', { bubbles: true })); return true; })()`);
    rows.push({ label, ms: await firstAfter(t0) });
  }
  // A vibe loop starting: pick a vibe, then Next makes a new loop.
  await go(tab, 'vibes');
  await click(tab, '#vibesList .vb-key');
  await go(tab, 'now');
  await sleep(2500);
  const t0 = await at();
  await click(tab, '#nextBtn');
  rows.push({ label: 'a vibe loop (Next)', ms: await firstAfter(t0) });
  for (const r of rows) log(`  change ${r.label}: ${r.error || `${r.ms} ms to the first new note`}`);
  return rows.map((r) => ({ ...r, loop }));
}

async function session(opts, chromePath, loop, engine, throttle, quality) {
  const b = await launch(chromePath);
  try {
    const url = `http://127.0.0.1:${opts.port}/${engine === 'js' ? '?engine=js' : ''}`;
    // A phone's screen, so the views lay out (and paint) as on one.
    const tab = await openTab(b, url, throttle, {
      sources: [extra(quality === 'lite')],
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
    const rows = [];
    for (const ph of PHASES) {
      const r = await phase(tab, engine, ph, loop);
      rows.push({ loop: name, engine, throttle, quality, ...r });
      log(`  ${r.view}: long tasks ${r.longTasks} (longest ${r.longestMs} ms${r.what ? `, ${r.what}` : ''}), gap ${r.gapMs} ms, late notes ${r.lateNotes}${r.coreLate != null ? ` (core ${r.coreLate})` : ''}, late ticks ${r.lateTicks} (worst ${r.worstLateMs} ms), fill-ins ${r.fills}`);
    }
    if (opts.changes) for (const c of await changes(tab, engine, loop)) rows.push({ loop: name, engine, throttle, quality, change: true, ...c });
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
  const head = ['loop', 'cpu', 'quality', 'view', 'long tasks', 'longest ms', 'what', 'longest tick gap ms', 'late notes (core)', 'late ticks', 'fill-ins'];
  const out = [`| ${head.join(' | ')} |`, `|${head.map(() => '---').join('|')}|`];
  const changed = rows.filter((r) => r.change);
  for (const r of rows.filter((x) => !x.change)) {
    const cells = r.error
      ? [r.loop, `${r.throttle}x`, r.quality, r.view || 'session', r.error, '', '', '', '', '', '']
      : [r.loop.replace(/-.*/, ''), `${r.throttle}x`, r.quality, r.view, r.longTasks, r.longestMs, r.what || '-', r.gapMs,
        `${r.lateNotes}${r.coreLate != null ? ` (${r.coreLate})` : ''}`, r.lateTicks, r.fills ?? '-'];
    out.push(`| ${cells.join(' | ')} |`);
  }
  if (changed.length) {
    out.push('', '| loop | cpu | quality | change | first new note after, ms |', '|---|---|---|---|---|');
    for (const r of changed) out.push(`| ${r.loop.replace(/-.*/, '')} | ${r.throttle}x | ${r.quality} | ${r.label} | ${r.error || r.ms} |`);
  }
  return out.join('\n');
}

async function main() {
  const opts = parseArgs(process.argv.slice(2));
  const chromePath = findChrome(opts.chrome);
  const server = await startServer(opts.port);
  const rows = [];
  try {
    for (const loop of opts.loops) for (const engine of opts.engines) for (const throttle of opts.throttle) for (const quality of opts.quality) {
      log(`${loop} ${engine} ${throttle}x ${quality}`);
      try {
        rows.push(...await session(opts, chromePath, loop, engine, throttle, quality));
      } catch (err) {
        log(`  failed: ${err.stack || err.message}`);
        rows.push({ loop, engine, throttle, quality, error: err.message });
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
