// Before and after shots for item 119: node shots.mjs <beforeRoot> <afterRoot> <outDir>
import http from 'node:http';
import fs from 'node:fs';
import path from 'node:path';
import { createRequire } from 'node:module';
const [BEFORE, AFTER, OUT] = process.argv.slice(2).map((p) => path.resolve(p));
const require = createRequire(import.meta.url);
const { chromium } = require('playwright');
const { encodeSong, linkOf } = await import(path.join(AFTER, 'js/share.js'));
const { newSpec } = await import(path.join(AFTER, 'js/gen.js'));
const TYPES = { '.html': 'text/html', '.js': 'text/javascript', '.css': 'text/css', '.json': 'application/json', '.txt': 'text/plain', '.webmanifest': 'application/manifest+json', '.png': 'image/png', '.svg': 'image/svg+xml', '.wasm': 'application/wasm', '.woff2': 'font/woff2' };
function serve(root, port) {
  return new Promise((ok) => {
    const s = http.createServer((req, res) => {
      let f = path.join(root, decodeURIComponent(new URL(req.url, 'http://x').pathname));
      if (f.endsWith('/')) f = path.join(f, 'index.html');
      if (!f.startsWith(root) || !fs.existsSync(f)) { res.writeHead(404); res.end(); return; }
      res.writeHead(200, { 'Content-Type': TYPES[path.extname(f)] || 'application/octet-stream', 'Cache-Control': 'no-store' });
      fs.createReadStream(f).pipe(res);
    }).listen(port, '127.0.0.1', () => ok(s));
  });
}
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const SIZES = [
  { name: '360x640', width: 360, height: 640, touch: true },
  { name: '320x568', width: 320, height: 568, touch: true },
  { name: 'side-640x360', width: 640, height: 360, touch: true },
  { name: 'tablet-800x1280', width: 800, height: 1280, touch: true },
];
const THEMES = { light: 't-dawn', dark: 't-nocturne' };
const servers = [await serve(BEFORE, 8791), await serve(AFTER, 8792)];
const browser = await chromium.launch({ args: ['--autoplay-policy=no-user-gesture-required'] });
fs.mkdirSync(OUT, { recursive: true });
const code = encodeSong(newSpec(11));
const all = {};
for (const [tag, port] of [['before', 8791], ['after', 8792]]) {
  for (const size of SIZES) for (const mode of ['light', 'dark']) {
    const ctx = await browser.newContext({ viewport: { width: size.width, height: size.height }, colorScheme: mode, serviceWorkers: 'block', deviceScaleFactor: 2 });
    await ctx.addInitScript((theme) => { try { localStorage.setItem('driftloom.prefs.v1', JSON.stringify({ theme })); } catch {} }, THEMES[mode]);
    const page = await ctx.newPage();
    page.on('pageerror', (e) => console.error(tag, size.name, mode, e.message));
    await page.goto(linkOf(`http://127.0.0.1:${port}/`, code));
    await page.waitForFunction(() => document.readyState === 'complete');
    await sleep(1500);
    await page.click('#playBtn');
    await sleep(2500);
    const shot = async (view) => {
      const f = `${view}_${size.name}_${mode}_${tag}.png`;
      await page.screenshot({ path: path.join(OUT, f) });
      (all[`${view}_${size.name}_${mode}`] ||= {})[tag] = f;
    };
    await shot('card');
    for (let i = 0; i < 4 && await page.evaluate(() => document.getElementById('viewSounds').hidden); i++) { await page.click('#viewBtn'); await sleep(400); }
    await sleep(500); await shot('sound');
    await page.click('#viewBtn'); await sleep(900);
    if (await page.evaluate(() => document.getElementById('viewLayers').hidden)) console.error('layers not shown', tag, size.name);
    await page.click('.layer[data-layer="bass"] .btn-on'); await sleep(500);
    await shot('layers');
    await ctx.close();
  }
}
// One sheet per size and mode: before above, after below, three views across.
for (const size of SIZES) for (const mode of ['light', 'dark']) {
  const ctx = await browser.newContext({ viewport: { width: 1400, height: 900 } });
  const p = await ctx.newPage();
  const img = (f) => `<img src="data:image/png;base64,${fs.readFileSync(path.join(OUT, f)).toString('base64')}">`;
  const row = (tag) => `<h2>${tag === 'before' ? 'Before (main, v206)' : 'After (#468, v209)'}</h2><main>${['card', 'sound', 'layers'].map((v) => `<figure>${img(all[`${v}_${size.name}_${mode}`][tag])}<figcaption>${v}</figcaption></figure>`).join('')}</main>`;
  await p.setContent(`<style>body{margin:12px;font:14px system-ui;background:#fff;color:#111}h1{font-size:18px}h2{font-size:15px;margin:10px 0 4px}main{display:grid;grid-template-columns:repeat(3,1fr);gap:12px}figure{margin:0}img{width:100%;display:block;border:1px solid #ccc}</style><h1>Item 119: ${size.name}, ${mode} (${THEMES[mode]})</h1>${row('before')}${row('after')}`);
  await p.screenshot({ path: path.join(OUT, `sheet_${size.name}_${mode}.png`), fullPage: true });
  await ctx.close();
}
await browser.close();
servers.forEach((s) => s.close());
console.log('done', Object.keys(all).length);
