// Contact sheets of the v18 to v20 spec: chosen places x themes x text sizes,
// one picture. Uses the page's own window.spec, so it shows exactly what
// the review controls would.
//
//   node sheet18.mjs ../driftloom_v20.html out/sheet.png \
//     --places now,layers,library:more --themes all --sizes 100 [--paused] [--insets android]
//
// --themes all | t-dawn,t-nocturne,…   --places all | now,vibes,…
import { chromium } from 'playwright';
import path from 'node:path';
import fs from 'node:fs';

const [file, out, ...rest] = process.argv.slice(2);
if (!file || !out) { console.error('usage: node sheet18.mjs <spec.html> <out.png> [--places …] [--themes …] [--sizes …]'); process.exit(1); }
const opt = (n, d) => { const i = rest.indexOf(n); return i >= 0 ? rest[i + 1] : d; };
const flag = (n) => rest.includes(n);

const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1200, height: 1100 }, deviceScaleFactor: 2 });
// Nothing leaves the machine: the file carries its own fonts and pictures.
await page.route(/^https?:/, (r) => r.abort());
const url = 'file://' + path.resolve(file) + (opt('--insets') ? `#insets=${opt('--insets')}` : '');
await page.goto(url);
await page.waitForFunction(() => window.spec);

const all = await page.evaluate(() => ({ places: window.spec.places, themes: window.spec.themes, sizes: window.spec.sizes }));
const pick = (v, list) => (!v || v === 'all' ? list : v.split(','));
const places = pick(opt('--places', 'now'), all.places);
const themes = pick(opt('--themes', 't-nocturne'), all.themes);
const sizes = pick(opt('--sizes', '100'), all.sizes.map(String)).map(Number);
if (flag('--paused')) await page.evaluate(() => window.spec.setPlaying(false));

const shots = [];
for (const size of sizes) {
  await page.evaluate((s) => window.spec.setSize(s), size);
  for (const place of places) {
    for (const theme of themes) {
      await page.evaluate(([p, t]) => { window.spec.setTheme(t); window.spec.show(p); }, [place, theme]);
      await page.waitForTimeout(150);
      const png = await page.locator('#app').screenshot();
      shots.push({ label: `${theme.replace('t-', '')} · ${place} · ${size}%`, data: png.toString('base64') });
    }
  }
}

// Lay them out on one page and photograph that.
const cols = Math.min(shots.length, Number(opt('--cols', themes.length > 1 ? Math.min(themes.length, 5) : Math.min(shots.length, 5))));
const sheet = await browser.newPage({ viewport: { width: cols * 360 + 40, height: 800 }, deviceScaleFactor: 1 });
await sheet.setContent(`<body style="margin:0;padding:20px;background:#0a0a0c;font:12px ui-monospace,monospace;color:#aaa">
  <div style="display:grid;grid-template-columns:repeat(${cols},340px);gap:20px">
  ${shots.map((s) => `<figure style="margin:0"><img style="width:340px;display:block" src="data:image/png;base64,${s.data}"><figcaption style="padding-top:6px">${s.label}</figcaption></figure>`).join('')}
  </div></body>`);
await sheet.waitForTimeout(300);
fs.mkdirSync(path.dirname(path.resolve(out)), { recursive: true });
await sheet.screenshot({ path: out, fullPage: true });
await browser.close();
console.log(`${out}: ${shots.length} phones`);
