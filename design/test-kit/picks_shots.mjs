// Screenshots of the next picks in the app (Painter session 6): Now Playing
// outside an album, the Vibe panel (the brief's "New loops") and the Length picker, in any
// theme and text size, with any picks. Also measures what the brief asks
// for: targets, nothing scrolling, the panel fitting its zone.
//
//   node picks_shots.mjs out/picks --themes all --sizes 100,200
//   node picks_shots.mjs out/picks --themes t-dawn --sizes 100 --set long
//
// Sets of picks: everyday (merry, tender · 70–85 bpm · 16 bars), long
// (four moods · 4,096 bars, tempo any), nothing (all any; Endless in the
// picker). Writes one PNG per theme, size and set (the three views side by
// side) and prints a JSON line of measurements for each.

import http from 'node:http';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { chromium } from 'playwright';

const HERE = path.dirname(fileURLToPath(import.meta.url));
const ROOT = path.resolve(HERE, '../..');
const args = process.argv.slice(2);
const opt = (name, dflt) => { const i = args.indexOf(name); return i >= 0 ? args[i + 1] : dflt; };
const OUT = path.resolve(args[0] && !args[0].startsWith('--') ? args[0] : 'out/picks');
const ALL = ['t-nocturne', 't-dark', 't-amoled', 't-nightlight', 't-abyss', 't-dawn', 't-ember', 't-tidal', 't-olive'];
const THEMES = opt('--themes', 'all') === 'all' ? ALL : opt('--themes').split(',');
const SIZES = opt('--sizes', '100,200').split(',').map(Number);
const SETS = opt('--set', 'everyday,long,nothing').split(',');
fs.mkdirSync(OUT, { recursive: true });

const PICKS = {
  everyday: { vibes: ['merry', 'tender'], picks: { tempo: [70, 85], length: 16 } },
  long: { vibes: ['merry', 'tender', 'starry', 'wistful'], picks: { tempo: null, length: 4096 } },
  nothing: { vibes: [], picks: { tempo: null, length: null } },
  // The widest words there are, both pace values, the longest length.
  worst: { vibes: ['adventurous', 'enthusiastic', 'experimental', 'heartwarming'], picks: { tempo: [100, 120], length: 32768 } },
};

const TYPES = { '.html': 'text/html', '.js': 'text/javascript', '.mjs': 'text/javascript', '.css': 'text/css',
  '.wasm': 'application/wasm', '.woff2': 'font/woff2', '.png': 'image/png', '.json': 'application/json',
  '.webmanifest': 'application/manifest+json', '.svg': 'image/svg+xml' };
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

const browser = await chromium.launch({ args: ['--autoplay-policy=no-user-gesture-required'] });
const ctx = await browser.newContext({ viewport: { width: 340, height: 780 }, deviceScaleFactor: 2 });

async function settle(page, ms = 200) {
  await page.evaluate(() => new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r))));
  await page.waitForTimeout(ms);
}

// Every check the brief asks of these views, measured on the page.
const MEASURE = `(() => {
  const v = [...document.querySelectorAll('.view')].find((x) => !x.hidden);
  const zone = document.querySelector('.dynamic-inner').getBoundingClientRect();
  const out = { view: v.dataset.view, small: [], overflow: [], scrolls: [], outside: [] };
  for (const n of v.querySelectorAll('button, input')) {
    if (n.closest('[hidden]') || n.type === 'hidden') continue;
    const r = n.getBoundingClientRect();
    if (!r.width) continue;
    if (r.width < 44 - 0.5 || r.height < 44 - 0.5) out.small.push(\`\${n.id || n.className || n.textContent.trim()} \${r.width.toFixed(1)}×\${r.height.toFixed(1)}\`);
    if (r.bottom > zone.bottom + 0.5 || r.top < zone.top - 0.5 || r.left < zone.left - 0.5 || r.right > zone.right + 0.5) out.outside.push(n.id || n.textContent.trim());
  }
  for (const n of [v, ...v.querySelectorAll('*')]) {
    if (n.classList.contains('vh')) continue; // screen-reader-only text
    const cs = getComputedStyle(n);
    if (n.scrollHeight > n.clientHeight + 1 && /auto|scroll/.test(cs.overflowY)) out.scrolls.push(n.className || n.id);
    if (n.scrollWidth > n.clientWidth + 1 && n.clientWidth && !/visible/.test(cs.overflowX) && n.tagName !== 'INPUT') out.overflow.push(n.className || n.id);
  }
  for (const n of v.querySelectorAll('.pk-pill, .pk-any, .ln-time, .ln-ticks span, .ln-inf, .pk-anykey')) {
    const r = n.getBoundingClientRect();
    if (r.width && (r.right > zone.right + 0.5 || r.left < zone.left - 0.5 || r.bottom > zone.bottom + 0.5)) out.outside.push(n.textContent.trim());
  }
  // Anything past the bottom of the view itself (not just the panel):
  // the view is where nothing may spill.
  const vb = v.getBoundingClientRect().bottom;
  for (const n of v.querySelectorAll('*')) {
    const r = n.getBoundingClientRect();
    if (r.height && n.getClientRects().length && r.bottom > vb + 0.5) out.outside.push(\`past the view: \${n.className || n.tagName} +\${(r.bottom - vb).toFixed(1)}\`);
  }
  const list = v.querySelector('.pk-list');
  if (list && list.scrollHeight > list.clientHeight + 0.5) out.overflow.push(\`pk-list \${list.scrollHeight}>\${list.clientHeight}\`);
  const last = [...v.querySelectorAll('*')].filter((n) => n.getClientRects().length).reduce((m, n) => Math.max(m, n.getBoundingClientRect().bottom), 0);
  out.room = +(zone.bottom - last).toFixed(1);
  return out;
})()`;

const results = [];
for (const size of SIZES) {
  for (const set of SETS) {
    const page = await ctx.newPage();
    const errs = [];
    page.on('pageerror', (e) => errs.push(e.message));
    await page.addInitScript(`(() => {
      const set = () => { document.documentElement.style.fontSize = '${size}%'; };
      if (document.documentElement) { set(); return; }
      new MutationObserver((m, o) => { if (document.documentElement) { set(); o.disconnect(); } }).observe(document, { childList: true });
    })();`);
    await page.goto(`${BASE}/index.html`);
    await page.evaluate(async (p) => {
      const g = await import('/js/generator.js');
      const store = await import('/js/storage.js');
      if (!store.loadAll().length) {
        const ids = [];
        for (let i = 0; i < 12; i++) ids.push(store.save(g.newSpec(1000 + i * 7919)).id);
        store.ensureAlbums();
        const album = store.createAlbum('Harbor');
        store.setAlbumIds(album.id, ids.slice(0, 6));
      }
      store.setPrefs({ view: 'now', edit: 'layers', vibes: p.vibes, picks: p.picks });
    }, PICKS[set]);
    await page.reload();
    await settle(page, 500);
    // Play Harbor, then leave it: Now Playing outside an album, with the
    // way back.
    await page.click('#libraryBtn'); await settle(page);
    await page.click('[data-tab="loops"]'); await settle(page);
    await page.click('[data-tab="albums"]'); await settle(page);
    await page.locator('#libPane .lrow.album', { hasText: 'Harbor' }).locator('button', { hasText: 'play' }).click();
    await settle(page, 800);
    await page.click('#nowBtn'); await settle(page);
    if ((await page.evaluate(() => document.querySelector('.view:not([hidden])').dataset.view)) !== 'now') { await page.click('#nowBtn'); await settle(page); }
    const inAlbum = await page.locator('#nowNext:not([hidden])').count();
    await page.click('#nowAction'); await settle(page, 400);
    // Pause, so every theme's shot holds the same moment.
    await page.click('#playBtn'); await settle(page, 2600);

    for (const theme of THEMES) {
      await page.evaluate((t) => {
        const app = document.getElementById('app');
        for (const c of [...app.classList]) if (c.startsWith('t-')) app.classList.remove(c);
        app.classList.add(t);
      }, theme);
      if ((await page.evaluate(() => document.querySelector('.view:not([hidden])').dataset.view)) !== 'now') { await page.click('#nowBtn'); await settle(page); }
      await settle(page, 150);
      const shots = [];
      const m = {};
      shots.push(await page.locator('#app').screenshot());
      m.now = await page.evaluate(MEASURE);
      m.nowName = await page.getAttribute('#nowPicks', 'aria-label');
      await page.click('#nowPicks'); await settle(page);
      shots.push(await page.locator('#app').screenshot());
      m.picks = await page.evaluate(MEASURE);
      await page.locator('#picksList .pk-line-pace button.pk-val').last().click(); await settle(page);
      // nothing: the picker opens with any lit; worst: the ∞ key pressed.
      if (set === 'worst') { await page.click('#lenEndless'); await settle(page); }
      shots.push(await page.locator('#app').screenshot());
      m.length = await page.evaluate(MEASURE);
      await page.click('#lenCancel'); await settle(page);
      await page.click('#picksClose'); await settle(page);
      const name = `${theme}_${size}_${set}`;
      shots.forEach((b, i) => fs.writeFileSync(path.join(OUT, `${name}_${i}.png`), b));
      results.push({ name, inAlbumBefore: !!inAlbum, errs, ...m });
    }
    await page.close();
  }
}
await browser.close();
server.close();
for (const r of results) console.log(JSON.stringify(r));
