// Accessibility, the measurable parts (session 30f, queue item 114; the bar
// is WCAG 2.2 level AA). The real app in headless Chromium, taken through
// its screens:
//
//   contrast   as rendered, in every theme and in forced colors: text 4.5:1
//              (3:1 large) against the pixels under it, the meaningful
//              graphics and the controls' icons 3:1
//   focus      Tab reaches every control and comes round again; on each the
//              ring is drawn, whole, at 3:1, and nothing covers the control;
//              a view opened or closed keeps the focus
//   names      each control has a name, its role's state, and, a slider, its
//              value in words; the words on a key are in its name
//   targets    24 by 24 CSS px at the least (how many are under 44 is said)
//   fit        at SIZES, at 100% and 200% text: every control reachable and
//              uncovered, no words cut, nothing scrolls that shouldn't
//   motion     with reduced motion nothing animates or transitions
//
// What it can't hold (what a screen reader says in practice, a device's own
// font scale) is docs/notes/for-mikey.md's two passes.
//
//   node tools/a11y.mjs [--only contrast,focus,names,fit,motion]
//                       [--shots DIR] [--port 8774]
//
// Minutes, and a browser: not the gate. CI runs it in the export workflow's
// `a11y` job on every PR that touches the screens, and by hand:
//   gh workflow run export.yml --ref BRANCH -f jobs=a11y
// --shots: a sheet of the screens for each theme at 360 px, and for light
// and dark at a tablet's width, a short phone and a phone on its side.
//
// Chromium from Playwright (npm install --no-save playwright@1.56.1).

import http from 'node:http';
import fs from 'node:fs';
import path from 'node:path';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';
import { encodeSong, encodeAlbum, linkOf } from '../js/share.js';
import { newSpec } from '../js/gen.js';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const argv = process.argv.slice(2);
const flag = (name, dflt) => {
  const i = argv.indexOf(name);
  return i >= 0 ? argv[i + 1] : dflt;
};
const PORT = Number(flag('--port', 8774));
const SHOTS = flag('--shots', '');
const ONLY = flag('--only', 'contrast,focus,names,fit,motion').split(',');
const ORIGIN = `http://127.0.0.1:${PORT}`;

// Every theme the app has (checked against the app's own picker).
const THEMES = ['t-dawn', 't-ember', 't-tidal', 't-olive', 't-dark', 't-amoled', 't-nightlight', 't-nocturne', 't-abyss'];
const PHONE = { name: '360x780', width: 360, height: 780 };
const SIZES = [
  { name: '320x568', width: 320, height: 568 },
  { name: '360x640', width: 360, height: 640 },
  PHONE,
  { name: '412x915', width: 412, height: 915 },
  { name: '768x1024', width: 768, height: 1024 },
  { name: '568x320', width: 568, height: 320 },
  { name: '640x360', width: 640, height: 360 },
  { name: '800x360', width: 800, height: 360 },
  { name: '915x412', width: 915, height: 412 },
];
const SCALES = [100, 200];

function loadPlaywright() {
  const require = createRequire(import.meta.url);
  for (const id of ['playwright', 'playwright-core']) {
    try { return require(id); } catch { /* try the next one */ }
  }
  console.error('a11y: Playwright is not installed.  npm install --no-save playwright@1.56.1');
  process.exit(3);
  return null;
}

const TYPES = {
  '.html': 'text/html', '.js': 'text/javascript', '.css': 'text/css', '.json': 'application/json', '.txt': 'text/plain',
  '.webmanifest': 'application/manifest+json', '.png': 'image/png', '.svg': 'image/svg+xml', '.wasm': 'application/wasm', '.woff2': 'font/woff2',
};

function serve(port) {
  const server = http.createServer((req, res) => {
    let file = path.join(ROOT, decodeURIComponent(new URL(req.url, ORIGIN).pathname));
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

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const settle = (page) => page.evaluate(() => new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r))));

// ------------------------------------------------------------- the screens

const click = (page, selector) => page.evaluate((s) => {
  const n = document.querySelector(s);
  if (!n) throw new Error(`no ${s}`);
  n.click();
}, selector);
const isHidden = (page, id) => page.evaluate((i) => document.getElementById(i).hidden, id);
const toNow = async (p) => { if (await isHidden(p, 'viewNow')) await click(p, '#nowBtn'); };
const toEdit = async (p, id) => { for (let i = 0; i < 3 && await isHidden(p, id); i++) await click(p, '#viewBtn'); };
const toSave = async (p, tab) => { if (await isHidden(p, 'viewSave')) await click(p, '#saveBtn'); await click(p, `[data-stab="${tab}"]`); };
const toTab = async (p, tab) => { if (await isHidden(p, 'viewLibrary')) await click(p, '#libraryBtn'); await click(p, `[data-tab="${tab}"]`); };
const exportState = (page, which) => page.evaluate(async (w) => {
  const { WORDS } = await import(new URL('js/export.js', location.href).href);
  document.getElementById('exportFill').style.width = '42%';
  document.getElementById('exportProgress').textContent = WORDS.progress(42, '3 min');
  document.getElementById('exportNote').textContent = WORDS.background;
  document.getElementById('exportSaved').textContent = WORDS.saved('Ballad Falls.flac', '238 MB', '45 min');
  document.getElementById('exportWhy').textContent = WORDS.space('1.2 GB', '300 MB');
  for (const s of ['Choose', 'Run', 'Done', 'Failed']) document.getElementById(`export${s}`).hidden = s !== w;
}, which);

// In this order, each from where the one before left the app. The page
// opens on an album's link.
const SCREENS = [
  ['now, in an album', toNow],
  ['now, album left', async (p) => { await toNow(p); await click(p, '#nowAction'); }],
  ['vibe', async (p) => { await click(p, '#vibesBtn'); }],
  ['vibes, picked', async (p) => {
    await click(p, '#picksList .pk-line-mood .pk-val');
    for (const n of [2, 5, 9]) await click(p, `#vibesList button:nth-child(${n})`);
  }],
  ['vibe, picked', async (p) => { await click(p, '#vibesClose'); }],
  ['bars, any', async (p) => { await click(p, '#picksList .pk-line-pace button.pk-val:not(.pk-track)'); }],
  ['bars, a long piece', async (p) => {
    await p.evaluate(() => {
      const s = document.getElementById('lenSlide');
      s.value = '10';
      s.dispatchEvent(new Event('input', { bubbles: true }));
    });
    await click(p, '[data-journey="arc"]');
  }],
  ['bars, endless', async (p) => { await click(p, '#lenEndless'); }],
  ['track length, on and on', async (p) => { await click(p, '#lenCancel'); await click(p, '#picksList .pk-track'); }],
  ['track length, set', async (p) => {
    await p.evaluate(() => {
      const n = document.getElementById('trkNum');
      n.value = '1:23:45';
      n.dispatchEvent(new Event('blur'));
    });
  }],
  ['sound', async (p) => { await click(p, '#trkCancel'); await toEdit(p, 'viewSounds'); }],
  ['layers', async (p) => {
    await click(p, '#viewBtn');
    await p.evaluate(() => {
      const row = document.querySelectorAll('.layer')[1];
      if (!row.classList.contains('dim')) row.querySelector('.btn-on').click();
    });
  }],
  ['save', (p) => toSave(p, 'albums')],
  ['share', (p) => toSave(p, 'share')],
  ['export', async (p) => { await click(p, '#goExport'); await sleep(500); }],
  ['export, running', (p) => exportState(p, 'Run')],
  ['export, done', (p) => exportState(p, 'Done')],
  ['export, failed', (p) => exportState(p, 'Failed')],
  ['seed album', async (p) => { await exportState(p, 'Choose'); await click(p, '#exportBack'); await click(p, '#goSeedAlbum'); }],
  ['library, albums', (p) => toTab(p, 'albums')],
  ['album', async (p) => {
    await toTab(p, 'albums');
    await p.evaluate(() => [...document.querySelectorAll('.lrow.album')].pop().querySelector('.lrow-main').click());
  }],
  ['move a loop', async (p) => { await click(p, '.lrow.track .row-keys button'); }],
  ['add loops', async (p) => { await click(p, '#libPane .page-foot button:last-child'); await click(p, '.hero-keys button:nth-child(2)'); }],
  ['library, loops', (p) => toTab(p, 'loops')],
  ['more', (p) => toTab(p, 'more')],
  ['advanced', async (p) => {
    await click(p, '#goAdvanced');
    await p.evaluate(() => { document.querySelector('#libAdvanced .scroll').scrollTop = 0; });
  }],
  ['advanced, the end', async (p) => {
    await p.evaluate(() => { document.querySelector('#libAdvanced .scroll').scrollTop = 1e6; });
  }],
];

const ALBUM = () => linkOf(`${ORIGIN}/`, encodeAlbum('Rainy Day Mix', [newSpec(31337, ['rainy', 'mellow']), newSpec(4242), newSpec(777, 'bars=64,bpm=80')]));
const ALONE = [
  ['now, a first visit', () => `${ORIGIN}/`],
  ['now, a long piece', () => linkOf(`${ORIGIN}/`, encodeSong(newSpec(11, { length: 1024 })))],
  ['plain line', () => `${ORIGIN}/?plain`],
  ['speed test', () => `${ORIGIN}/speed.html`],
];

// ----------------------------------------------------------- in the page

// Put in every page before its scripts: `window.A`.
function pageSide() {
  const CONTROLS = 'button, a[href], input:not([type="hidden"]), [role="button"], [role="slider"]';
  const SCROLLERS = '.scroll, .vb-list';
  // The graphics that mean something. `fill`: the middle of the shape;
  // `edge`: a ring or a stroke somewhere in it.
  const MARKS = [
    ['.playhead i.beat:not(.on)', 'fill'], ['.playhead i.on', 'lit'],
    ['.layer:not(.dim) .pattern span.on:not([data-shade])', 'fill'],
    ['.pattern span.on[data-shade], .layer.dim .pattern span.on', 'edge'],
    ['.np-dots i.on', 'fill'], ['[aria-selected="true"] > .led, [aria-pressed="true"] > .led', 'fill'],
    ['.meta-dots i.on', 'fill'], ['.now-mark', 'edge'], ['.pick', 'edge'],
    ['button:not(:disabled) svg, .vkey:not(:disabled) svg', 'edge'],
    ['.btn-play svg', 'halves'], ['.pc-head', 'head'], ['input[type="range"]:not(:disabled)', 'thumb'],
    ['.export-bar span', 'fill'], ['.ln-inf[aria-pressed="true"] > span', 'edge'],
  ];
  const one = document.createElement('canvas');
  one.width = one.height = 1;
  const g = one.getContext('2d', { willReadFrequently: true });
  // Any computed color as [r, g, b, a].
  const rgba = (c) => {
    g.clearRect(0, 0, 1, 1);
    g.fillStyle = '#000';
    g.fillStyle = c;
    g.fillRect(0, 0, 1, 1);
    const d = g.getImageData(0, 0, 1, 1).data;
    return [d[0], d[1], d[2], d[3] / 255];
  };
  const seen = (e) => e.checkVisibility({ visibilityProperty: true, opacityProperty: true }) && !e.closest('.vh');
  const nameOf = (e) => (e.id ? `#${e.id}` : `${e.tagName.toLowerCase()}.${String(e.getAttribute('class') || '').trim().replace(/\s+/g, '.')}`);
  // What of `r` its clipping ancestors leave on screen.
  const clipped = (e, r, from = e.parentElement) => {
    let box = { left: r.left, top: r.top, right: r.right, bottom: r.bottom };
    for (let p = from; p && p !== document.body; p = p.parentElement) {
      const cs = getComputedStyle(p);
      if (cs.overflowX === 'visible' && cs.overflowY === 'visible') continue;
      const b = p.getBoundingClientRect();
      const m = cs.overflowX === 'clip' ? parseFloat(cs.overflowClipMargin) || 0 : 0;
      box = { left: Math.max(box.left, b.left - m), top: Math.max(box.top, b.top - m), right: Math.min(box.right, b.right + m), bottom: Math.min(box.bottom, b.bottom + m) };
    }
    box = { left: Math.max(0, box.left), top: Math.max(0, box.top), right: Math.min(innerWidth, box.right), bottom: Math.min(innerHeight, box.bottom) };
    return box.right - box.left >= 1 && box.bottom - box.top >= 1 ? box : null;
  };
  const opacityOf = (e) => {
    let o = 1;
    for (let p = e; p; p = p.parentElement) o *= Number(getComputedStyle(p).opacity);
    return o;
  };
  // A control that is off is exempt (1.4.3 and 1.4.11), and what labels it.
  const off = (e) => !!e.closest(':disabled, [aria-disabled="true"], .ln-journey.off, #viewLength.endless .ln-slide');
  const textRuns = () => {
    const out = [];
    const walk = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT);
    for (let n = walk.nextNode(); n; n = walk.nextNode()) {
      const e = n.parentElement;
      if (!n.textContent.trim() || e.closest('script, style, svg') || !seen(e)) continue;
      const range = document.createRange();
      range.selectNodeContents(n);
      for (const r of range.getClientRects()) if (r.width >= 1) out.push({ e, r, text: n.textContent.trim().slice(0, 30) });
    }
    for (const e of document.querySelectorAll('input[type="text"]')) if (seen(e) && e.value) out.push({ e, r: e.getBoundingClientRect(), text: e.value, input: true });
    return out;
  };
  let order = [];
  window.A = {
    rgba,
    contrastTargets() {
      const text = [];
      for (const { e, r, text: words, input } of textRuns()) {
        if (off(e)) continue;
        const box = clipped(e, r);
        if (!box || box.bottom - box.top < Math.min(8, r.height - 1)) continue;
        if (input) {
          // The number's digits, not the whole box.
          const inX = Math.min(r.width * 0.4, r.width / 2 - 8);
          box.left += inX; box.right -= inX; box.top += r.height * 0.3; box.bottom -= r.height * 0.3;
        }
        const cs = getComputedStyle(e);
        const px = parseFloat(cs.fontSize);
        const fg = rgba(cs.color);
        fg[3] *= opacityOf(e);
        text.push({ what: `${nameOf(e)} "${words}"`, box, fg, need: px >= 24 || (px >= 18.66 && Number(cs.fontWeight) >= 700) ? 3 : 4.5 });
      }
      const marks = [];
      for (const [sel, how] of MARKS) {
        for (const e of document.querySelectorAll(sel)) {
          if (!e.checkVisibility() || off(e)) continue;
          let r = e.getBoundingClientRect();
          if (how === 'lit') r = { left: r.left + r.width / 2 - 4, right: r.left + r.width / 2 + 4, top: r.top + r.height / 2 - 4, bottom: r.top + r.height / 2 + 4 };
          if (how === 'head') r = { left: r.left - 3, right: r.left + 3, top: r.bottom - 8, bottom: r.bottom - 1 };
          if (how === 'thumb') {
            const t = (Number(e.value) - Number(e.min)) / (Number(e.max) - Number(e.min));
            const x = r.left + 12 + t * (r.width - 24);
            const y = r.top + r.height / 2;
            r = { left: x - 12, right: x + 12, top: y - 12, bottom: y + 12 };
          }
          if (how === 'halves') {
            const mid = r.top + r.height / 2;
            for (const [top, bottom] of [[r.top, mid], [mid, r.bottom]]) marks.push({ what: `${nameOf(e)} of Play`, how: 'edge', box: { left: r.left, right: r.right, top, bottom } });
            continue;
          }
          const box = clipped(e, r);
          if (!box || box.right - box.left < Math.min(6, r.right - r.left - 1) || box.bottom - box.top < Math.min(6, r.bottom - r.top - 1)) continue;
          marks.push({ what: `${nameOf(e)} in ${nameOf(e.parentElement)}`, how: how === 'thumb' || how === 'edge' ? 'edge' : 'fill', box });
        }
      }
      return { text, marks };
    },
    // The screen without its words and marks: what is under them.
    bare(on) {
      let s = document.getElementById('a11y-bare');
      if (!on) { if (s) s.remove(); return; }
      if (!s) {
        s = document.createElement('style');
        s.id = 'a11y-bare';
        document.head.appendChild(s);
      }
      s.textContent = `* { color: transparent !important; -webkit-text-fill-color: transparent !important; text-shadow: none !important; text-decoration-color: transparent !important; }
        ${MARKS.map(([sel]) => sel).join(', ')} { visibility: hidden !important; }
        .ln-ticks, .ln-inf > span { visibility: hidden !important; }`;
    },
    // What Tab should reach now.
    reachable() {
      order = [...document.querySelectorAll(`${CONTROLS}, [tabindex="0"]`)].filter((e) => !e.disabled && e.tabIndex >= 0 && e.checkVisibility({ visibilityProperty: true }));
      return order.length;
    },
    focused() {
      const e = document.activeElement;
      if (!e || e === document.body || e === document.documentElement) return null;
      const r = e.getBoundingClientRect();
      const x = Math.min(innerWidth - 1, Math.max(0, r.left + r.width / 2));
      const y = Math.min(innerHeight - 1, Math.max(0, r.top + r.height / 2));
      const hit = document.elementFromPoint(x, y);
      return { i: order.indexOf(e), what: nameOf(e), zone: e.closest('section, nav, main')?.className || '', covered: !(hit && (hit === e || e.contains(hit) || hit.contains(e))) };
    },
    ring(i) {
      const e = order[i];
      if (!e || !e.isConnected || !e.checkVisibility()) return null;
      e.scrollIntoView({ block: 'nearest', inline: 'nearest' });
      e.focus({ preventScroll: true, focusVisible: true });
      const cs = getComputedStyle(e);
      const r = e.getBoundingClientRect();
      const w = parseFloat(cs.outlineWidth) || 0;
      const o = parseFloat(cs.outlineOffset) || 0;
      const at = o + w / 2;
      const out = Math.max(0, o + w);
      const outer = { left: r.left - out, right: r.right + out, top: r.top - out, bottom: r.bottom + out };
      const shown = clipped(e, outer);
      const inner = cs.boxShadow.match(/(rgba?\([^)]*\)|color\([^)]*\)) 0px 0px 0px 2px/);
      return {
        what: nameOf(e),
        drawn: cs.outlineStyle !== 'none' && w >= 2,
        whole: !!shown && shown.left <= outer.left + 1 && shown.right >= outer.right - 1 && shown.top <= outer.top + 1 && shown.bottom >= outer.bottom - 1,
        color: rgba(cs.outlineColor),
        inner: inner ? rgba(inner[1]) : null,
        band: { left: r.left - at, right: r.right + at, top: r.top - at, bottom: r.bottom + at },
      };
    },
    blur() {
      if (document.activeElement) document.activeElement.blur();
    },
    states() {
      const bad = [];
      for (const e of document.querySelectorAll(CONTROLS)) {
        if (!seen(e)) continue;
        const role = e.getAttribute('role') || '';
        if ((role === 'switch' || role === 'checkbox' || role === 'radio') && !e.hasAttribute('aria-checked')) bad.push(`${nameOf(e)} has no checked state`);
        if (role === 'tab' && !e.hasAttribute('aria-selected')) bad.push(`${nameOf(e)} has no selected state`);
        if (e.classList.contains('armed') && e.classList.contains('word') && !e.hasAttribute('aria-pressed') && !e.getAttribute('aria-label')) bad.push(`${nameOf(e)} is lit with no state`);
        if ((e.type === 'range' || role === 'slider') && !e.getAttribute('aria-valuetext') && e.getAttribute('aria-disabled') !== 'true') bad.push(`${nameOf(e)} doesn't say its value`);
        // 2.5.3: the words on it are in its name (symbols and steps aside).
        const label = e.getAttribute('aria-label');
        const words = [...e.childNodes].filter((n) => !(n.nodeType === 1 && (n.matches('[aria-hidden="true"], .vh, svg') || !n.checkVisibility())))
          .map((n) => n.textContent).join(' ').trim().toLowerCase().replace(/\s+/g, ' ');
        if (label && /[a-z]{2}/.test(words) && !/^[+−]1|[…·]|\d:\d\d/.test(words)) {
          const said = label.toLowerCase().replace(/\s+/g, ' ');
          if (!said.includes(words) && !words.split(/[ ,]+/).every((w) => said.includes(w))) bad.push(`${nameOf(e)} shows "${words}" and is named "${label}"`);
        }
      }
      for (const e of document.querySelectorAll('[aria-hidden="true"]')) {
        if (e.querySelector('button, a[href], input, [tabindex="0"]') && e.checkVisibility()) bad.push(`${nameOf(e)} hides something that takes focus`);
      }
      return bad;
    },
    fit() {
      const bad = [];
      let small = 0;
      let count = 0;
      for (const e of document.querySelectorAll(CONTROLS)) {
        if (!e.checkVisibility({ visibilityProperty: true })) continue;
        const sc = e.closest(SCROLLERS);
        if (sc) {
          if (sc.clientHeight < 40) { bad.push(`${nameOf(sc)} is ${sc.clientHeight} px to scroll in`); continue; }
          e.scrollIntoView({ block: 'center', inline: 'nearest' });
        }
        const r = e.getBoundingClientRect();
        count++;
        if (r.width < 24 || r.height < 24) bad.push(`${nameOf(e)} is ${Math.round(r.width)}x${Math.round(r.height)}`);
        else if (r.width < 44 || r.height < 44) small++;
        const box = sc ? null : e.closest('.dynamic-inner, .info-inner, .zone-buttons');
        const b = box ? box.getBoundingClientRect() : { left: 0, top: 0, right: innerWidth, bottom: innerHeight };
        if (document.getElementById('app') && (r.left < b.left - 1 || r.right > b.right + 1 || (!sc && (r.top < b.top - 1 || r.bottom > b.bottom + 1)))) bad.push(`${nameOf(e)} is outside its panel`);
        for (const y of [r.top + 3, r.top + r.height / 2, r.bottom - 3]) {
          const x = r.left + r.width / 2;
          if (sc) {
            const s = sc.getBoundingClientRect();
            if (y <= s.top || y >= s.bottom) continue;
          } else if (!document.getElementById('app') && (y < 0 || y >= innerHeight)) continue;
          const hit = document.elementFromPoint(x, y);
          if (!(hit && (hit === e || e.contains(hit)))) { bad.push(`${nameOf(e)} is under ${hit ? nameOf(hit) : 'nothing'}`); break; }
        }
      }
      // A key is as tall as its row lets it be, up to a finger's 44.
      for (const e of document.querySelectorAll('.layer button, .pk-anykey')) {
        if (!e.checkVisibility()) continue;
        const row = e.closest('.layer, .pk-line');
        if (!row) continue;
        const room = Math.min(44, row.getBoundingClientRect().height - 4);
        if (e.getBoundingClientRect().height < room - 1) bad.push(`${nameOf(e)} is shorter than its row lets it be`);
      }
      for (const { e, r, text, input } of textRuns()) {
        if (input) {
          if (e.scrollWidth > e.clientWidth + 1) bad.push(`"${text}" is cut in ${nameOf(e)}`);
          continue;
        }
        if (e.closest('.toast, .sw, .diag') || !document.getElementById('app')) continue;
        const sc = e.closest(SCROLLERS);
        for (let p = e; p && p.id !== 'app' && p !== sc; p = p.parentElement) {
          const cs = getComputedStyle(p);
          if (cs.overflowX === 'visible' && cs.overflowY === 'visible') continue;
          const b = p.getBoundingClientRect();
          // A line box is taller than its letters: 3 px of it may go.
          if (r.right > b.right + 1 || r.left < b.left - 1 || r.bottom > b.bottom + 4 || r.top < b.top - 4) { bad.push(`"${text}" is cut by ${nameOf(p)}`); break; }
        }
        const key = e.closest('button, .tab, a.vkey');
        if (key) {
          const k = key.getBoundingClientRect();
          if (r.right > k.right + 1 || r.left < k.left - 1 || r.bottom > k.bottom + 3 || r.top < k.top - 3) bad.push(`"${text}" runs out of ${nameOf(key)}`);
        }
      }
      // Two pieces of text over each other.
      const runs = textRuns().filter((t) => !t.e.closest('.toast, .scroll, .vb-list, .sw') && !t.input);
      for (let i = 0; i < runs.length; i++) {
        for (let j = i + 1; j < runs.length; j++) {
          if (runs[i].e === runs[j].e) continue;
          const a = runs[i].r;
          const b = runs[j].r;
          if (Math.min(a.right, b.right) - Math.max(a.left, b.left) > 2 && Math.min(a.bottom, b.bottom) - Math.max(a.top, b.top) > 4) bad.push(`"${runs[i].text}" is over "${runs[j].text}"`);
        }
      }
      const app = document.getElementById('app');
      if (app) {
        for (const e of app.querySelectorAll('*')) {
          if (!/auto|scroll/.test(getComputedStyle(e).overflowY) || !e.checkVisibility()) continue;
          if (e.scrollHeight > e.clientHeight + 1 && !e.matches(SCROLLERS)) bad.push(`${nameOf(e)} scrolls`);
        }
        const share = document.querySelector('#saveShare .scroll');
        if (share && share.checkVisibility() && share.scrollHeight > share.clientHeight + 1) bad.push('Share scrolls');
        if (app.scrollHeight > app.clientHeight + 1 || document.documentElement.scrollWidth > innerWidth + 1) bad.push('the screen overflows');
      } else if (document.documentElement.scrollWidth > innerWidth + 1) bad.push('the page is wider than the screen');
      return { bad, small, count };
    },
    motion() {
      const bad = document.getAnimations().filter((a) => a.playState === 'running').map((a) => `${a.animationName || a.transitionProperty || 'an animation'} runs`);
      for (const e of document.querySelectorAll('#app *')) {
        const cs = getComputedStyle(e);
        if (cs.transitionDuration.split(',').some((d) => parseFloat(d) > 0)) bad.push(`${nameOf(e)} has a transition`);
      }
      return bad;
    },
  };
}

// The pixels, read in a blank page: `jobs` against one or two screenshots.
async function readPixels({ bare, full, jobs }) {
  const lum = (r, g, b) => {
    const f = (v) => (v <= 10.31 ? v / 3294.6 : ((v / 255 + 0.055) / 1.055) ** 2.4);
    return 0.2126 * f(r) + 0.7152 * f(g) + 0.0722 * f(b);
  };
  const ratio = (a, b) => (Math.max(a, b) + 0.05) / (Math.min(a, b) + 0.05);
  const load = async (b64) => {
    const im = new Image();
    im.src = `data:image/png;base64,${b64}`;
    await im.decode();
    const c = document.createElement('canvas');
    c.width = im.width;
    c.height = im.height;
    const x = c.getContext('2d', { willReadFrequently: true });
    x.drawImage(im, 0, 0);
    return { w: im.width, h: im.height, d: x.getImageData(0, 0, im.width, im.height).data };
  };
  const B = await load(bare);
  const F = full ? await load(full) : null;
  const k = B.w / innerWidth;
  const over = (fg, i) => {
    if (!fg || !fg[3]) return null;
    const at = (c) => fg[c] * fg[3] + B.d[i + c] * (1 - fg[3]);
    return lum(at(0), at(1), at(2));
  };
  const under = (i) => lum(B.d[i], B.d[i + 1], B.d[i + 2]);
  const pick = (v, q) => {
    v.sort((a, b) => a - b);
    return v.length ? v[Math.min(v.length - 1, Math.floor(v.length * q))] : null;
  };
  return jobs.map((j) => {
    const v = [];
    let { left, right, top, bottom } = j.box;
    const at = (x, y) => {
      const px = Math.round(x);
      const py = Math.round(y);
      return px >= 0 && py >= 0 && px < B.w && py < B.h ? 4 * (py * B.w + px) : -1;
    };
    if (j.kind === 'ring') {
      const points = [];
      for (let t = 0.1; t < 0.95; t += 0.05) {
        const x = left + (right - left) * t;
        const y = top + (bottom - top) * t;
        points.push([x, top], [x, bottom], [left, y], [right, y]);
      }
      for (const [x, y] of points) {
        const i = at(x * k, y * k);
        if (i < 0) continue;
        const line = ratio(over(j.fg, i), under(i));
        // A two-color ring: either band against what is behind it.
        const band = over(j.inner, i);
        v.push(band == null ? line : Math.max(line, ratio(band, under(i))));
      }
      return pick(v, 0.1);
    }
    if (j.kind === 'fill') {
      const inX = (right - left) * 0.3;
      const inY = (bottom - top) * 0.3;
      left += inX; right -= inX; top += inY; bottom -= inY;
    }
    for (let y = Math.ceil(top * k); y < bottom * k; y++) {
      for (let x = Math.ceil(left * k); x < right * k; x++) {
        const i = at(x, y);
        if (i < 0) continue;
        v.push(j.kind === 'text' ? ratio(over(j.fg, i), under(i)) : ratio(lum(F.d[i], F.d[i + 1], F.d[i + 2]), under(i)));
      }
    }
    // Text: its worst background (a hairline in the box aside); a fill: its
    // middle; an edge: the strokes are the box's most different pixels.
    return pick(v, j.kind === 'text' ? 0.05 : j.kind === 'fill' ? 0.5 : 0.93);
  });
}

// ---------------------------------------------------------------- the run

const failures = new Map();
const tally = { text: 0, marks: 0, rings: 0, controls: 0, under44: 0, fits: 0 };
const worst = {};

function fail(check, where, what) {
  const key = `${check}: ${what}`;
  if (!failures.has(key)) {
    failures.set(key, []);
    console.log(`  FAIL  ${check}: ${where}: ${what}`);
  }
  failures.get(key).push(where);
}

async function open(browser, { url, size = PHONE, scale = 100, forced = false, dpr = 1 } = {}) {
  const ctx = await browser.newContext({
    viewport: { width: size.width, height: size.height }, deviceScaleFactor: dpr, serviceWorkers: 'block',
    reducedMotion: 'reduce', forcedColors: forced ? 'active' : 'none',
  });
  await ctx.addInitScript(pageSide);
  if (url.endsWith('?plain')) await ctx.addInitScript(() => { Object.defineProperty(globalThis, 'WebAssembly', { value: undefined }); });
  const page = await ctx.newPage();
  page.on('pageerror', (e) => fail('page', url, e.message));
  await page.goto(url);
  await page.waitForFunction(() => document.readyState === 'complete', null, { timeout: 60000 });
  await page.evaluate(() => document.fonts && document.fonts.ready);
  if (scale !== 100) {
    await page.evaluate((s) => {
      document.documentElement.style.fontSize = `${s}%`;
      dispatchEvent(new Event('resize'));
    }, scale);
  }
  await sleep(700);
  return { ctx, page };
}

// Each screen in turn, then those with a page of their own.
async function walk(browser, options, see) {
  const { ctx, page } = await open(browser, { ...options, url: ALBUM() });
  for (const [name, go] of SCREENS) {
    try {
      await go(page);
      await sleep(200);
      await settle(page);
      await see(page, name, ctx);
    } catch (e) {
      fail('screen', name, `could not be checked: ${e.message.split('\n')[0]}`);
    }
  }
  await ctx.close();
  for (const [name, url] of ALONE) {
    const own = await open(browser, { ...options, url: url() });
    try {
      await see(own.page, name, own.ctx);
    } catch (e) {
      fail('screen', name, `could not be checked: ${e.message.split('\n')[0]}`);
    }
    await own.ctx.close();
  }
}

const setTheme = (page, theme) => page.evaluate((t) => {
  const app = document.getElementById('app');
  if (!app) return;
  app.classList.remove(...[...app.classList].filter((c) => /^t-/.test(c)));
  app.classList.add(t);
  // As playing, held still: the lit dot and the layers' column.
  app.classList.add('playing');
  const light = document.querySelector('#playhead i');
  if (light) light.classList.add('on');
}, theme);

async function measure(page, lab, theme, where, { rings }) {
  await setTheme(page, theme);
  await settle(page);
  const { text, marks } = await page.evaluate(() => window.A.contrastTargets());
  const full = marks.length ? (await page.screenshot()).toString('base64') : null;
  await page.evaluate(() => window.A.bare(true));
  await settle(page);
  const bare = (await page.screenshot()).toString('base64');
  await page.evaluate(() => window.A.bare(false));
  await lab.setViewportSize(page.viewportSize());
  const jobs = [
    ...text.map((t) => ({ kind: 'text', what: t.what, box: t.box, fg: t.fg, need: t.need })),
    ...marks.map((m) => ({ kind: m.how, what: m.what, box: m.box, need: 3 })),
  ];
  if (rings) {
    // Keyboard modality, so focus() draws the ring.
    await page.keyboard.press('Shift');
    const count = await page.evaluate(() => window.A.reachable());
    const before = await page.evaluate(() => [...document.querySelectorAll('.scroll, .vb-list')].map((s) => s.scrollTop));
    for (let i = 0; i < count; i++) {
      const r = await page.evaluate((n) => window.A.ring(n), i);
      if (!r) continue;
      tally.rings++;
      if (!r.drawn) fail('focus', where, `${r.what} shows no ring`);
      else {
        if (!r.whole) fail('focus', where, `${r.what}'s ring is cut`);
        // (Only where nothing scrolled to show it: the pixels are the first screen's.)
        const still = await page.evaluate((was) => [...document.querySelectorAll('.scroll, .vb-list')].every((s, j) => s.scrollTop === was[j]), before);
        if (still) jobs.push({ kind: 'ring', what: `the ring on ${r.what}`, box: r.band, fg: r.color, inner: r.inner, need: 3 });
      }
    }
    await page.evaluate((was) => {
      window.A.blur();
      document.querySelectorAll('.scroll, .vb-list').forEach((s, j) => { s.scrollTop = was[j]; });
    }, before);
  }
  const got = await lab.evaluate(readPixels, { bare, full, jobs });
  jobs.forEach((j, i) => {
    const r = got[i];
    if (r == null) return;
    if (j.kind === 'text') tally.text++;
    else if (j.kind !== 'ring') tally.marks++;
    const kind = j.kind === 'text' ? (j.need === 3 ? 'large text' : 'text') : j.kind === 'ring' ? 'focus ring' : 'graphic';
    const key = `${theme} ${kind}`;
    if (worst[key] === undefined || r < worst[key]) worst[key] = r;
    if (r < j.need) fail('contrast', where, `${j.what} is ${r.toFixed(2)}:1 in ${theme}, under ${j.need}:1`);
  });
}

async function contrast(browser, lab) {
  await walk(browser, { dpr: 2 }, async (page, name) => {
    for (const theme of THEMES) await measure(page, lab, theme, `${name}, ${theme}`, { rings: ONLY.includes('focus') });
    await setTheme(page, 't-nocturne');
  });
  // A popup over each theme.
  const { ctx, page } = await open(browser, { url: `${ORIGIN}/`, dpr: 2 });
  const got = await page.evaluate(() => [...document.querySelectorAll('#themes .sw')].map((b) => b.dataset.theme).sort().join(','));
  if (got !== [...THEMES].sort().join(',')) fail('contrast', 'More', `the themes are ${got}: this check's list is behind`);
  await page.evaluate(() => {
    const t = document.getElementById('toast');
    t.textContent = 'Saved';
    t.classList.add('show');
  });
  for (const theme of THEMES) await measure(page, lab, theme, `a popup, ${theme}`, { rings: false });
  await ctx.close();
  await walk(browser, { forced: true, dpr: 2 }, (p, name) => measure(p, lab, 't-nocturne', `${name}, forced colors`, { rings: ONLY.includes('focus') }));
}

// Tab from the top: every control once, the zones in order, and out again.
async function tabOrder(page, where) {
  await page.evaluate(() => {
    window.A.blur();
    const top = document.createElement('i');
    top.id = 'a11y-top';
    top.tabIndex = -1;
    document.body.prepend(top);
    top.focus();
  });
  const count = await page.evaluate(() => window.A.reachable());
  const got = new Set();
  let cameRound = false;
  let zone = -1;
  let last = null;
  for (let n = 0; n < count + 12; n++) {
    await page.keyboard.press('Tab');
    const f = await page.evaluate(() => window.A.focused());
    if (!f) { cameRound = true; break; }
    if (f.i >= 0 && got.has(f.i)) {
      cameRound = got.size >= count;
      if (f.what === last && !cameRound) fail('focus', where, `Tab is stuck on ${f.what}`);
      break;
    }
    last = f.what;
    if (f.i >= 0) got.add(f.i);
    if (f.covered) fail('focus', where, `${f.what} has the focus and is covered`);
    const z = ['zone-info', 'zone-dynamic', 'zone-buttons'].findIndex((c) => f.zone.includes(c));
    if (z >= 0) {
      if (z < zone) fail('focus', where, `${f.what} comes out of order`);
      zone = z;
    }
  }
  if (!cameRound && got.size < count) fail('focus', where, 'Tab does not come round');
  if (got.size < count) fail('focus', where, `Tab reaches ${got.size} of ${count}`);
  await page.evaluate(() => {
    window.A.blur();
    document.getElementById('a11y-top').remove();
  });
}

// The views a key opens keep the focus, going in and coming back.
async function focusFollows(browser) {
  const { ctx, page } = await open(browser, { url: ALBUM() });
  await toNow(page);
  for (const [key, expected] of [
    ['#vibesBtn', '#picksClose'],
    ['#picksList .pk-line-mood .pk-val', '#vibesClose'],
    ['#vibesClose', '#picksList .pk-line-mood .pk-val'],
    ['#picksList .pk-line-pace button.pk-val:not(.pk-track)', '#lenCancel'],
    ['#lenCancel', '#picksList .pk-line-pace button.pk-val:not(.pk-track)'],
    ['#picksList .pk-track', '#trkCancel'],
    ['#trkCancel', '#picksList .pk-track'],
    ['#picksClose', '#vibesBtn'],
  ]) {
    await page.focus(key);
    await page.keyboard.press('Enter');
    await sleep(250);
    const on = await page.evaluate((s) => document.activeElement === document.querySelector(s), expected);
    if (!on) fail('focus', key, `pressing ${key} doesn't leave the focus on ${expected}`);
  }
  // Space presses the key the keyboard is on.
  await page.focus('#vibesBtn');
  await page.keyboard.press(' ');
  await sleep(250);
  if (await isHidden(page, 'viewVibe')) fail('focus', 'vibe', 'Space does not press the key the keyboard is on');
  await ctx.close();
}

const ROLES = new Set(['button', 'slider', 'switch', 'tab', 'radio', 'checkbox', 'textbox', 'link']);
async function names(page, where, ctx) {
  const cdp = await ctx.newCDPSession(page);
  const { nodes } = await cdp.send('Accessibility.getFullAXTree');
  for (const n of nodes) {
    if (n.ignored || !n.role || !ROLES.has(n.role.value)) continue;
    if (!/[\p{L}\p{N}]/u.test(String((n.name && n.name.value) || ''))) fail('names', where, `a ${n.role.value} has no name`);
  }
  await cdp.detach();
  for (const m of await page.evaluate(() => window.A.states())) fail('names', where, m);
}

async function fit(browser) {
  for (const size of SIZES) {
    for (const scale of SCALES) {
      await walk(browser, { size, scale }, async (page, name) => {
        const { bad, small, count } = await page.evaluate(() => window.A.fit());
        tally.fits++;
        if (scale === 100 && size === PHONE) {
          tally.controls += count;
          tally.under44 += small;
        }
        for (const b of new Set(bad)) fail('fit', `${name} @ ${size.name}, text ${scale}%`, `${b} (${size.name}, ${scale}%)`);
      });
    }
  }
}

async function shots(browser) {
  fs.mkdirSync(SHOTS, { recursive: true });
  const sets = [
    ...THEMES.map((theme) => [theme, PHONE]),
    ...[['768x1024', 768, 1024], ['360x640', 360, 640], ['800x360', 800, 360]]
      .flatMap(([name, width, height]) => ['t-dawn', 't-nocturne'].map((theme) => [theme, { name, width, height }])),
  ];
  for (const [theme, size] of sets) {
    const tiles = [];
    await walk(browser, { size }, async (page, name) => {
      await setTheme(page, theme);
      await settle(page);
      tiles.push([name, (await page.screenshot({ type: 'jpeg', quality: 85 })).toString('base64')]);
    });
    const ctx = await browser.newContext({ viewport: { width: 1600, height: 900 } });
    const sheet = await ctx.newPage();
    const figures = tiles.map(([name, jpg]) => `<figure><img src="data:image/jpeg;base64,${jpg}"><figcaption>${name}</figcaption></figure>`).join('');
    await sheet.setContent(`<style>body{margin:8px;font:12px system-ui;background:#fff;color:#111}main{display:grid;grid-template-columns:repeat(${size.width > 700 ? 5 : 8},1fr);gap:8px}figure{margin:0}img{width:100%;display:block}figcaption{padding:3px 0}</style><main>${figures}</main>`);
    await sheet.screenshot({ path: path.join(SHOTS, `sheet_${size.name}_${theme}.jpg`), fullPage: true, type: 'jpeg', quality: 85 });
    await ctx.close();
  }
}

async function main() {
  const { chromium } = loadPlaywright();
  const server = await serve(PORT);
  const browser = await chromium.launch({ args: ['--autoplay-policy=no-user-gesture-required'] });
  const labCtx = await browser.newContext({ viewport: { width: PHONE.width, height: PHONE.height } });
  const lab = await labCtx.newPage();
  const began = Date.now();
  try {
    if (SHOTS) await shots(browser);
    if (ONLY.includes('contrast')) await contrast(browser, lab);
    if (ONLY.includes('fit')) await fit(browser);
    if (ONLY.includes('focus') || ONLY.includes('names') || ONLY.includes('motion')) {
      await walk(browser, {}, async (page, name, ctx) => {
        if (ONLY.includes('names')) await names(page, name, ctx);
        if (ONLY.includes('focus')) await tabOrder(page, name);
        if (ONLY.includes('motion')) {
          await setTheme(page, 't-dawn');
          await settle(page);
          for (const m of await page.evaluate(() => window.A.motion())) fail('motion', name, m);
          await setTheme(page, 't-nocturne');
        }
      });
      if (ONLY.includes('focus')) await focusFollows(browser);
    }
  } finally {
    await browser.close();
    server.close();
  }
  if (Object.keys(worst).length) {
    console.log('\n| theme | text | large text | graphic | focus ring |\n|---|--:|--:|--:|--:|');
    for (const t of THEMES) {
      const cell = (k) => (worst[`${t} ${k}`] === undefined ? '' : worst[`${t} ${k}`].toFixed(2));
      console.log(`| ${t.slice(2)} | ${cell('text')} | ${cell('large text')} | ${cell('graphic')} | ${cell('focus ring')} |`);
    }
  }
  for (const [what, where] of failures) console.log(`  failed ${where.length}x  ${what}`);
  console.log(`\na11y: ${tally.text} looks at text, ${tally.marks} at graphics, ${tally.rings} at rings in ${THEMES.length} themes and forced colors; ${tally.fits} fits; ${tally.under44} of ${tally.controls} targets under 44 px at 360x780; ${Math.round((Date.now() - began) / 1000)} s; ${failures.size} failed`);
  process.exit(failures.size ? 1 : 0);
}

main().catch((e) => {
  console.error(`a11y: ${e.stack}`);
  process.exit(1);
});
