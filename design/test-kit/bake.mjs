// Bakes the design spec from the app itself (Painter session 5, v18;
// v19 adds the next picks, Painter session 6).
//
// The spec used to be drawn by hand and then ported into the app. Since
// v18 it goes the other way: this script runs the real app in headless
// Chromium on a 340 x 780 screen (the spec's phone), seeds a library,
// plays an album, visits every view, and copies what the app drew into
// one self-contained HTML file. So the spec looks exactly like the app,
// and a change to the app's look shows up here by baking again.
//
//   cd design/test-kit && npm install
//   node bake.mjs                      # writes ../driftloom_v19.html
//   node bake.mjs --out ../x.html      # somewhere else
//
// What it takes from the repo: index.html and js/ (run as they are),
// css/style.css (inlined whole, fonts and all), and spec.template.html
// beside this script (the page around the phone: review controls, the
// handover, the script that makes the keys work). Edit the template for
// the page; edit the app for the phone.
//
// Nothing in js/ or css/ is changed. The page is driven from outside with
// real clicks, as a person would.

import http from 'node:http';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { execSync } from 'node:child_process';

let chromium;
try { ({ chromium } = await import('playwright')); }
catch { console.error('Playwright is missing: run npm install in design/test-kit first.'); process.exit(1); }

const HERE = path.dirname(fileURLToPath(import.meta.url));
const ROOT = path.resolve(HERE, '../..');
const args = process.argv.slice(2);
const opt = (name, dflt) => { const i = args.indexOf(name); return i >= 0 ? args[i + 1] : dflt; };
const OUT = path.resolve(opt('--out', path.join(ROOT, 'design/driftloom_v19.html')));
const SIZES = (process.env.SIZES || "100,130,150,200").split(",").map(Number);
const W = 340, H = 780;

// ------------------------------------------------------------ a server

const TYPES = { '.html': 'text/html', '.js': 'text/javascript', '.mjs': 'text/javascript', '.css': 'text/css',
  '.wasm': 'application/wasm', '.woff2': 'font/woff2', '.png': 'image/png', '.json': 'application/json',
  '.webmanifest': 'application/manifest+json', '.svg': 'image/svg+xml', '.wav': 'audio/wav', '.mp3': 'audio/mpeg' };
const server = http.createServer((req, res) => {
  let p = decodeURIComponent(new URL(req.url, 'http://x').pathname);
  if (p.endsWith('/')) p += 'index.html';
  const f = path.join(ROOT, p);
  if (!f.startsWith(ROOT) || !fs.existsSync(f) || fs.statSync(f).isDirectory()) { res.writeHead(404); res.end(); return; }
  res.writeHead(200, { 'content-type': TYPES[path.extname(f)] || 'application/octet-stream' });
  fs.createReadStream(f).pipe(res);
});
await new Promise((r) => server.listen(0, '127.0.0.1', r));
const BASE = `http://127.0.0.1:${server.address().port}`;

// ------------------------------------------------- what is in the library

// Thirty saved loops and five albums of six, drawn by the app's own
// generator from fixed seeds (tools/views.mjs does the same). Names come
// from the generator, so they stay put from one bake to the next unless
// the generator changes.
const SEED_VIBES = ['merry', 'tender'];
const SEED_LIBRARY = `(async () => {
  const g = await import('/js/generator.js');
  const store = await import('/js/storage.js');
  localStorage.clear();
  const ids = [];
  for (let i = 0; i < 30; i++) ids.push(store.save(g.newSpec(1000 + i * 7919)).id);
  store.ensureAlbums();
  const titles = ['Morning', 'Rainy Afternoons', 'Late Train', 'Harbor', 'Study'];
  for (let a = 0; a < 5; a++) {
    const album = store.createAlbum(titles[a]);
    store.setAlbumIds(album.id, ids.slice(a * 6, a * 6 + 6));
  }
  for (const id of ids.slice(0, 8)) store.addToAlbum(store.FAVORITES, id);
  // The next picks of the brief's first render (design/next-picks.md):
  // two moods, 70–85 bpm and 16 bars. The app has no tempo picker yet, so
  // the tempo is seeded here, as a saved pick.
  store.setPrefs({ theme: 't-nocturne', view: 'now', edit: 'layers',
    vibes: ${JSON.stringify(SEED_VIBES)}, picks: { tempo: [70, 85], length: 16 } });
})()`;

// -------------------------------------------------- copying what is drawn

// Runs in the page. A copy of a node with every canvas turned into an
// <img> (its pixels kept once, in window.__art, and pointed at by
// data-art), every input's live value written into its attributes, and
// the moment-to-moment marks (the lit cell, the halo column, the clock)
// taken out: the spec's own script puts those back while it runs.
const SNAP = `(() => {
  const art = window.__art || (window.__art = []);
  const index = (url) => { let i = art.indexOf(url); if (i < 0) { art.push(url); i = art.length - 1; } return i; };
  const pix = (node) => {
    try {
      if (node.tagName === 'CANVAS') return node.width ? node.toDataURL('image/webp', 0.9) : null;
      if (node.tagName === 'IMG' && node.complete && node.naturalWidth && /^blob:/.test(node.src)) {
        const c = document.createElement('canvas');
        c.width = node.naturalWidth; c.height = node.naturalHeight;
        c.getContext('2d').drawImage(node, 0, 0);
        return c.toDataURL('image/webp', 0.9);
      }
    } catch (e) { return null; }
    return null;
  };
  window.__snap = (node) => {
    const live = [node, ...node.querySelectorAll('*')];
    const copy = node.cloneNode(true);
    const dead = [copy, ...copy.querySelectorAll('*')];
    for (let i = 0; i < live.length; i++) {
      const a = live[i], b = dead[i];
      if (a.tagName === 'CANVAS' || (a.tagName === 'IMG' && /^blob:/.test(a.src))) {
        const url = pix(a);
        const img = document.createElement('img');
        for (const at of a.attributes) if (at.name !== 'width' && at.name !== 'height' && at.name !== 'src') img.setAttribute(at.name, at.value);
        img.setAttribute('alt', '');
        if (a.getAttribute('aria-label') && !a.getAttribute('aria-hidden')) { img.setAttribute('role', 'img'); }
        if (url) img.dataset.art = index(url);
        b.replaceWith(img);
        dead[i] = img;
      } else if (a.tagName === 'INPUT') {
        if (a.type === 'checkbox' || a.type === 'radio') { if (a.checked) b.setAttribute('checked', ''); else b.removeAttribute('checked'); }
        else b.setAttribute('value', a.value);
      }
    }
    for (const s of copy.querySelectorAll('script')) s.remove();
    for (const n of copy.querySelectorAll('.playhead i.on')) n.classList.remove('on');
    for (const n of copy.querySelectorAll('.pattern span.at')) n.classList.remove('at');
    const big = copy.querySelector('#nowBig'); if (big) big.textContent = '0:00';
    const fill = copy.querySelector('#nowFill'); if (fill) fill.removeAttribute('style');
    for (const n of copy.querySelectorAll('.toast')) { n.textContent = ''; n.classList.remove('show'); }
    // Measured scroll positions are not kept; the hint lines are worked
    // out again by the spec's script.
    for (const n of copy.querySelectorAll('.scroll')) n.classList.remove('more-up', 'more-down');
    return copy;
  };
})()`;

// ------------------------------------------------------------- the visits

async function settle(page, ms = 250) {
  await page.evaluate(() => new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r))));
  await page.waitForTimeout(ms);
}

async function shown(page) {
  return page.evaluate(() => {
    const v = [...document.querySelectorAll('.view')].find((x) => !x.hidden);
    return v ? v.dataset.view : null;
  });
}

async function press(page, selector) {
  await page.locator(selector).first().click();
  await settle(page);
}

// Every place the spec can show. Each is reached with real presses, from
// Now Playing, so the app's own rules decide what is on screen.
async function goto(page, state) {
  const [view, sub, n] = state.split(':');
  // Home first, so every visit starts from the same place.
  if ((await shown(page)) !== 'now') await press(page, '#nowBtn');
  if ((await shown(page)) !== 'now') await press(page, '#nowBtn');
  // Now Playing outside an album, and the next picks it opens: leave the
  // album first (these are visited after every place that shows it).
  if ((view === 'now' && sub === 'free') || view === 'picks' || view === 'length') {
    if (await page.locator('#nowNext:not([hidden])').count()) {
      await press(page, '#nowAction');
      await page.waitForTimeout(2400); // the "left the album" note fades
    }
    if (view === 'now') return;
    await press(page, '#nowPicks');
    if (view === 'length') await press(page, '#picksList .pk-line-pace button.pk-val');
    return;
  }
  if (view === 'now') return;
  if (view === 'vibes') return press(page, '#vibesBtn');
  if (view === 'layers' || view === 'sounds') {
    await press(page, '#viewBtn');
    if ((await shown(page)) !== view) await press(page, '#viewBtn');
    return;
  }
  if (view === 'library') {
    await press(page, '#libraryBtn');
    const tab = sub === 'album' ? 'albums' : sub === 'advanced' ? 'more' : sub;
    // Pressing the tab you are on goes up a level, so press another first.
    await press(page, `[data-tab="${tab === 'loops' ? 'albums' : 'loops'}"]`);
    await press(page, `[data-tab="${tab}"]`);
    // Coming back to a tab returns to the album (or Advanced) you were
    // in; the tab's own key once more goes up to its list.
    if (tab === 'albums' && !(await page.locator('#libPane .lrow.album').count())) await press(page, '[data-tab="albums"]');
    if (tab === 'more' && await page.locator('#libAdvanced:not([hidden])').count()) await press(page, '[data-tab="more"]');
    if (sub === 'album') { await page.locator('#libPane .lrow.album').nth(Number(n)).locator('.lrow-main').click(); await settle(page); }
    if (sub === 'advanced') await press(page, '#goAdvanced');
    await page.waitForTimeout(600); // album art is drawn off the main thread
    return;
  }
  if (view === 'save') {
    await press(page, '#saveBtn');
    await press(page, `[data-stab="${sub}"]`);
    await page.waitForTimeout(400);
  }
}

async function capture(page, state) {
  await goto(page, state);
  const want = state.split(':')[0];
  const got = await shown(page);
  if (got !== want) throw new Error(`${state}: the app shows ${got}`);
  return page.evaluate(() => {
    const v = [...document.querySelectorAll('.view')].find((x) => !x.hidden);
    const view = window.__snap(v);
    const card = window.__snap(document.getElementById('card'));
    const keys = window.__snap(document.querySelector('.zone-buttons'));
    return { view: v.id, html: view.innerHTML, card: card.innerHTML,
      keys: keys.innerHTML, playheadHidden: document.getElementById('playhead').style.visibility === 'hidden' };
  });
}

// ----------------------------------------------------------------- bake

const browser = await chromium.launch({ args: ['--autoplay-policy=no-user-gesture-required'] });
const ctx = await browser.newContext({ viewport: { width: W, height: H }, deviceScaleFactor: 2 });
const errors = [];

let page = await ctx.newPage();
await page.goto(`${BASE}/index.html`);
await page.evaluate(SEED_LIBRARY);

let firstPass = null;
async function open(size) {
  const p = await ctx.newPage();
  p.on('pageerror', (e) => errors.push(`${size}%: ${e.message} ${(e.stack || '').split('\n').slice(1, 3).join(' ')}`));
  // The text-size setting, as a phone applies it: before the first paint.
  await p.addInitScript(`(() => {
    const set = () => { document.documentElement.style.fontSize = '${size}%'; };
    if (document.documentElement) { set(); return; }
    new MutationObserver((m, o) => { if (document.documentElement) { set(); o.disconnect(); } }).observe(document, { childList: true });
  })();`);
  await p.goto(`${BASE}/index.html`);
  await p.evaluate(SNAP);
  await settle(p, 600);
  // Play the second album from the top, so Now Playing has an album to
  // show and the lists have a loop marked as playing.
  await press(p, '#libraryBtn');
  await press(p, '[data-tab="loops"]');
  await press(p, '[data-tab="albums"]');
  await p.locator('#libPane .lrow.album').nth(2).locator('button', { hasText: 'play' }).click();
  await settle(p, 1500);
  // The Layers rows as the app draws them now, in the loop's first pass:
  // with Wander on (the default) later passes drift away from it.
  if (size === 100) firstPass = await p.evaluate(() => {
    const drawn = {};
    for (const row of document.querySelectorAll('.layer')) {
      drawn[row.dataset.layer] = [...row.querySelectorAll('.pattern span')].map((d) => (d.classList.contains('big') ? 2 : d.classList.contains('on') ? 1 : 0)).join('');
    }
    return drawn;
  });
  await press(p, '#nowBtn');
  return p;
}

await page.close();
page = await open(100);

// The albums the list shows (Imported Loops only once it has something).
await press(page, '#libraryBtn');
await press(page, '[data-tab="albums"]');
if (!(await page.locator('#libPane .lrow.album').count())) await press(page, '[data-tab="albums"]');
const albumCount = await page.locator('#libPane .lrow.album').count();
await press(page, '#nowBtn');

const STATES = ['now', 'vibes', 'layers', 'sounds',
  'library:albums', 'library:loops', 'library:more', 'library:advanced',
  ...Array.from({ length: albumCount }, (_, i) => `library:album:${i}`),
  'save:albums', 'save:share',
  // Last, since they leave the album: the next picks (v19).
  'now:free', 'picks', 'length'];

// The app's whole screen once (every view is in it, most hidden), then
// each place's own view and card at every text size.
const skeleton = await page.evaluate(() => window.__snap(document.getElementById('app')).outerHTML);
const states = {};
// One table of pictures for the whole spec. Each page kept its own list;
// its data-art numbers are moved into this one by content.
const allArt = [];
function adopt(pageArt, text) {
  return text.replace(/data-art="(\d+)"/g, (_, i) => {
    const url = pageArt[Number(i)];
    let at = allArt.indexOf(url);
    if (at < 0) { allArt.push(url); at = allArt.length - 1; }
    return `data-art="${at}"`;
  });
}
let skeletonArt = await page.evaluate(() => window.__art.slice());
const skeletonHtml = adopt(skeletonArt, skeleton);
for (const size of SIZES) {
  const p = size === 100 ? page : await open(size);
  const caps = [];
  for (const s of STATES) caps.push([s, await capture(p, s)]);
  const pageArt = await p.evaluate(() => window.__art);
  for (const [s, c] of caps) {
    states[s] ??= { view: c.view, sizes: {} };
    states[s].sizes[size] = { html: adopt(pageArt, c.html), card: adopt(pageArt, c.card),
      keys: adopt(pageArt, c.keys), playheadHidden: c.playheadHidden };
  }
  if (size !== 100) await p.close();
}

// Keep only what differs from 100%: the later sizes mostly repeat it.
for (const s of Object.values(states)) {
  const base = s.sizes[100];
  for (const size of SIZES.slice(1)) {
    const v = s.sizes[size];
    for (const k of ['html', 'card', 'keys']) if (v[k] === base[k]) delete v[k];
    if (v.playheadHidden === base.playheadHidden) delete v.playheadHidden;
    if (!Object.keys(v).length) delete s.sizes[size];
  }
}

// ------------------------------------------- the loop's notes, bar by bar

// The Layers rows follow the playhead bar by bar. The pattern comes from
// the app's own generator, read the way ui.js reads it (stepsForBar).
const loop = await page.evaluate(async () => {
  const g = await import('/js/generator.js');
  const store = await import('/js/storage.js');
  const ui = await import('/js/ui.js');
  const name = document.getElementById('loopName').textContent;
  const entry = store.loadAll().find((e) => e.spec.name === name);
  const spec = entry.spec;
  const pattern = g.render(spec);
  const spb = pattern.stepsPerBar || 16;
  const bars = Math.max(1, Math.round(pattern.totalSteps / spb));
  const layers = {};
  for (const layer of g.LAYERS) {
    const cycle = (pattern.cycles && pattern.cycles[layer]) || pattern.totalSteps;
    layers[layer] = [];
    for (let bar = 0; bar < bars; bar++) {
      const out = new Array(spb).fill(0);
      const start = (bar * spb) % cycle;
      for (const e of pattern.tracks[layer]) {
        if (!e.vel) continue;
        const idx = ((e.step - start) % cycle + cycle) % cycle;
        if (idx < spb) out[idx] = Math.max(out[idx], e.vel);
      }
      layers[layer].push(out.map((v) => (v > 0.55 ? 2 : v ? 1 : 0)).join(''));
    }
  }
  return { name, bpm: spec.bpm, spb, bars, layers,
    icons: { play: ui.icon('play'), pause: ui.icon('pause'), check: ui.icon('check'), sounds: ui.icon('sounds'), layers: ui.icon('layers') } };
});

// The first pass, as read from the generator, must hold the bar the app
// was drawing during its first pass.
const drawnRow = Object.values(firstPass).join('|');
const someBar = Array.from({ length: loop.bars }, (_, b) => Object.keys(firstPass).map((l) => loop.layers[l][b]).join('|'));
const patternOk = someBar.includes(drawnRow);

// ------------------------------------------- Now Playing's clock

// Read twice, a second apart: counting down (a set length) or up.
await goto(page, 'now');
const readClock = () => page.evaluate(() => ({
  text: document.getElementById('nowBig').textContent,
  fill: document.getElementById('nowFill').style.transform,
}));
const c1 = await readClock();
const secs = (t) => t.replace('\u2212', '').split(':').reduce((a, x) => a * 60 + Number(x), 0);
const now = { sign: c1.text.startsWith('\u2212') ? -1 : 1, t0: secs(c1.text),
  frac0: Number((c1.fill.match(/scaleX\(([\d.]+)\)/) || [0, 0])[1]) };

// ------------------------------------------------- measurements for tables

const measured = await page.evaluate(() => {
  const r = (sel) => { const n = document.querySelector(sel); return n ? n.getBoundingClientRect() : null; };
  const cs = (sel) => { const n = document.querySelector(sel); return n ? getComputedStyle(n) : null; };
  const px = (v) => Math.round(v * 10) / 10;
  const zone = (sel) => px(r(sel).height);
  const keys = [...document.querySelectorAll('.zone-buttons .btn')].map((b) => { const q = b.getBoundingClientRect(); return `${px(q.width)}×${px(q.height)}`; });
  const name = cs('#loopName'), meta = cs('#loopDetail');
  return {
    zones: { info: zone('.zone-info'), dynamic: zone('.zone-dynamic'), buttons: zone('.zone-buttons') },
    gap: getComputedStyle(document.getElementById('app')).rowGap,
    cover: `${px(r('.cover').width)}×${px(r('.cover').height)}`,
    name: `${name.fontFamily.split(',')[0].replace(/'/g, '')} ${name.fontWeight} · ${name.fontSize}`,
    meta: `${meta.fontFamily.split(',')[0].replace(/'/g, '')} ${meta.fontWeight} · ${meta.fontSize} / ${meta.lineHeight}`,
    keys,
    radius: cs('.zone-buttons .btn').borderRadius,
    panel: cs('#card').borderRadius,
  };
});

await browser.close();
server.close();

// ------------------------------------------------------------ write it

const css = forSpec(inlineFonts(fs.readFileSync(path.join(ROOT, 'css/style.css'), 'utf8')));

// Two changes for the spec's phone, both noted at the top of the file:
// the short-screen rule never applies to it (it is 780px tall plus
// insets, never under 700), and the reduced-motion rules are copied
// under html.sim-reduced-motion for the review switch.
function forSpec(text) {
  const shortRule = /@media \(max-height: 700px\)/g;
  if (!shortRule.test(text)) console.log('note: the short-screen rule was not found; nothing switched off');
  text = text.replace(shortRule, '@media (max-height: 0px) /* spec: off, see the top of the file */');
  let copies = '';
  const open = /@media \(prefers-reduced-motion: reduce\)\s*\{/g;
  let m;
  while ((m = open.exec(text))) {
    let i = open.lastIndex, depth = 1;
    const start = i;
    while (depth && i < text.length) { if (text[i] === '{') depth++; else if (text[i] === '}') depth--; i++; }
    copies += prefixRules(text.slice(start, i - 1), 'sim-reduced-motion');
  }
  return `${text}\n\n/* spec: the reduced-motion rules again, for the review switch */\n${copies}`;
}
function prefixRules(body, cls) {
  body = body.replace(/\/\*[\s\S]*?\*\//g, '');
  if (/@/.test(body)) throw new Error('nested at-rule inside a reduced-motion block: teach prefixRules about it');
  return body.replace(/([^{}]+)\{([^{}]*)\}/g, (_, sel, decl) => sel.split(',').map((x) => {
    x = x.trim();
    if (x.startsWith('html')) return x.replace(/^html/, `html.${cls}`);
    if (x.startsWith(':root')) return x.replace(/^:root/, `:root.${cls}`);
    return `html.${cls} ${x}`;
  }).join(', ') + ` {${decl}}\n`);
}
function inlineFonts(text) {
  return text.replace(/url\('\.\.\/fonts\/([^']+)'\)/g, (_, f) =>
    `url(data:font/woff2;base64,${fs.readFileSync(path.join(ROOT, 'fonts', f)).toString('base64')})`);
}

const commit = (() => {
  try {
    const sha = execSync('git rev-parse --short HEAD', { cwd: ROOT }).toString().trim();
    const dirty = execSync('git status --porcelain -- index.html css js design/test-kit/bake.mjs design/test-kit/spec.template.html', { cwd: ROOT }).toString().trim();
    return `commit ${sha}${dirty ? ' plus uncommitted changes' : ''}`;
  } catch { return ''; }
})();
const DATA = {
  baked: new Date().toISOString().slice(0, 10),
  sizes: SIZES, states, art: allArt, loop: { name: loop.name, bpm: loop.bpm, spb: loop.spb, bars: loop.bars, layers: loop.layers },
  icons: loop.icons, measured, now, commit, vibes: SEED_VIBES,
};

const template = fs.readFileSync(path.join(HERE, 'spec.template.html'), 'utf8');
const out = template
  .replace('/*BAKE:CSS*/', () => css)
  .replace('<!--BAKE:PHONE-->', () => skeletonHtml)
  .replace('/*BAKE:DATA*/null', () => JSON.stringify(DATA).replace(/<\/(script)/gi, '<\\/$1'))
  .replace(/<!--BAKE:DATE-->/g, () => DATA.baked);
fs.writeFileSync(OUT, out);

const kb = (n) => `${Math.round(n / 1024)} KB`;
console.log(`baked ${path.relative(process.cwd(), OUT)} · ${kb(out.length)} · ${STATES.length} places × ${SIZES.length} sizes · ${allArt.length} pictures (${kb(allArt.join('').length)})`);
console.log(`loop "${loop.name}" · ${loop.bpm} bpm · ${loop.bars} bars of ${loop.spb} · pattern check ${patternOk ? 'ok' : 'MISMATCH'}`);
console.log(`zones ${measured.zones.info} / ${measured.zones.dynamic} / ${measured.zones.buttons} · ${albumCount} albums`);
console.log(errors.length ? `page errors:\n  ${errors.join('\n  ')}` : 'page errors: none');
if (!patternOk) process.exitCode = 1;
