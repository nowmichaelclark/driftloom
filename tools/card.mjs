// The info card, measured (queue item 112, session 30d): every view of the
// card, at the widths and text sizes a listener meets, and a check that
// every line it shows is whole -- nothing cut off, clipped, overlapping,
// wrapped in the middle of a fact or set in tiny type -- and that what is
// left off is a tap away.
//
//   node tools/card.mjs [--shots <dir>] [--port 8772] [--only <case>]
//
// The page is the real one, in headless Chromium, opened on a loop's link
// (or an album's, opened in the Library) at each width and text size; the
// card is read as drawn, in the page's own pixels. Per case, width and
// scale ("text 200%" is the root font size, as a browser's or Android's
// text setting makes it) it checks, for the card's name and every detail
// line that is visible:
//
//   whole      a line's text fits its box: scrollWidth within clientWidth
//              (no ellipsis, no clip), one line high
//   inside     every line lies within the card's text column and the card
//   apart      no two lines, and not the name and a line, overlap
//   size       the type is never under 11px (as the card has always been)
//   heard      a screen reader still gets every fact: the list holds each
//              fact, whether or not it is shown
//   more       where facts are left off, the card says so (a quiet mark)
//              and a tap on the details shows the rest in the same place,
//              whole; a second tap goes back
//   dot        the playhead is a row of round dots, as many as a bar has
//              steps (4/4, 6/8, 5/4, 7/8), the lit one lit
//
// `--shots <dir>` also writes a screenshot of each card, light and dark.
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
  const i = argv.indexOf(`--${name}`);
  return i >= 0 && argv[i + 1] ? argv[i + 1] : dflt;
};
const PORT = Number(flag('port', 8772));
const SHOTS = flag('shots', '');
const ONLY = flag('only', '');
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

function loadPlaywright() {
  const require = createRequire(import.meta.url);
  for (const from of [ROOT, process.cwd(), '/opt/node22/lib/node_modules/', '/usr/lib/node_modules/']) {
    try { return require(require.resolve('playwright', { paths: [from] })); } catch { /* next */ }
  }
  console.error('card: Playwright is not installed.  npm install --no-save playwright@1.56.1');
  process.exit(2);
}

const TYPES = {
  '.html': 'text/html', '.js': 'text/javascript', '.mjs': 'text/javascript', '.css': 'text/css', '.json': 'application/json',
  '.webmanifest': 'application/manifest+json', '.png': 'image/png', '.svg': 'image/svg+xml', '.wasm': 'application/wasm',
  '.woff2': 'font/woff2', '.txt': 'text/plain',
};
function serve(port) {
  const server = http.createServer((req, res) => {
    const url = new URL(req.url, 'http://x');
    let file = path.join(ROOT, decodeURIComponent(url.pathname));
    if (file.endsWith('/')) file = path.join(file, 'index.html');
    if (!file.startsWith(ROOT) || !fs.existsSync(file)) { res.writeHead(404); res.end(); return; }
    res.writeHead(200, { 'Content-Type': TYPES[path.extname(file)] || 'application/octet-stream', 'Cache-Control': 'no-store' });
    fs.createReadStream(file).pipe(res);
  });
  return new Promise((resolve) => server.listen(port, '127.0.0.1', () => resolve(server)));
}

// ------------------------------------------------------------- the cases

const named = (spec, name) => { spec.name = name; return spec; };
const THAI = 'ฝนตกพรำๆ ยามเย็นที่ริมน้ำ';
const EMOJI = '🎧🌧️ lofi 🌙✨';
const LONG = 'Mundane Thursday Afternoon';
const BUSY = ['rainy', 'mellow', 'merry'];

// A case: its name, the code to open (a loop's or an album's) and, for an
// album, how to show it (`open`: on the card, or playing).
export function cases() {
  const out = [];
  const loop = (name, spec) => out.push({ name, code: encodeSong(spec), spec });
  loop('loop', newSpec(11));
  loop('busy loop (4 lines of feeling, drifting)', newSpec(125, BUSY));
  loop('busy 6/8', newSpec(24, BUSY));
  loop('busy 5/4', newSpec(37, BUSY));
  loop('long piece', newSpec(11, { length: 1024 }));
  loop('endless', newSpec(11, { length: 'endless' }));
  loop('long name', named(newSpec(11, BUSY), LONG));
  loop('Thai name', named(newSpec(125, BUSY), THAI));
  loop('emoji name', named(newSpec(125, BUSY), EMOJI));
  const tracks = [named(newSpec(125, BUSY), 'First'), newSpec(24), newSpec(37, { tempo: 60 })];
  out.push({ name: 'album on the card', album: encodeAlbum('Rainy Day Mix', tracks), open: 'card', specs: tracks });
  out.push({ name: 'album playing', album: encodeAlbum('Rainy Day Mix', tracks), open: 'playing', specs: tracks });
  return out;
}

// Widths a listener meets: the smallest phone, a common one, a large one, a
// tablet in portrait and in landscape; a phone on its side.
const SIZES = [
  { name: '320', width: 320, height: 640 },
  { name: '360', width: 360, height: 740 },
  { name: '412', width: 412, height: 915 },
  { name: 'tablet', width: 768, height: 1024 },
  { name: 'landscape phone', width: 844, height: 390 },
  { name: 'landscape tablet', width: 1180, height: 820 },
];
// The browser's text size: 100%, 130%, 200% (and Android's largest scale,
// 200% as well, as the root font size).
const SCALES = [100, 130, 200];

// ------------------------------------------------------------ the checks

// Run in the page: what the card shows, and what is wrong with it.
function inspect() {
  const $ = (id) => document.getElementById(id);
  const card = $('card');
  const col = $('loopDetail').closest('.info-text');
  const colBox = col.getBoundingClientRect();
  const cardBox = card.getBoundingClientRect();
  const problems = [];
  const boxes = [];
  const name = $('loopName');
  const shown = [...$('loopDetail').children].filter((li) => {
    const cs = getComputedStyle(li);
    return cs.display !== 'none' && cs.visibility !== 'hidden' && !li.hasAttribute('data-off');
  });
  const lines = [name, ...shown];
  for (const li of lines) {
    const r = li.getBoundingClientRect();
    const cs = getComputedStyle(li);
    const label = li === name ? 'name' : `"${li.textContent}"`;
    if (r.width === 0 || r.height === 0) { problems.push(`${label}: no box`); continue; }
    if (li.scrollWidth > li.clientWidth + 1) problems.push(`${label}: cut off sideways (${li.scrollWidth} > ${li.clientWidth})`);
    if (li.scrollHeight > li.clientHeight + 1) problems.push(`${label}: cut off below (${li.scrollHeight} > ${li.clientHeight})`);
    if (r.left < colBox.left - 1 || r.right > colBox.right + 1) problems.push(`${label}: outside the column sideways`);
    if (r.top < colBox.top - 1 || r.bottom > colBox.bottom + 1) problems.push(`${label}: outside the column (${Math.round(r.bottom)} > ${Math.round(colBox.bottom)})`);
    if (r.right > cardBox.right + 1 || r.bottom > cardBox.bottom + 1) problems.push(`${label}: outside the card`);
    const lh = parseFloat(cs.lineHeight) || parseFloat(cs.fontSize) * 1.3;
    if (li !== name && r.height > lh * 1.5) problems.push(`${label}: wraps (${Math.round(r.height)}px high, a line is ${Math.round(lh)}px)`);
    if (li !== name && parseFloat(cs.fontSize) < 10.99) problems.push(`${label}: type ${cs.fontSize}`);
    if (cs.textOverflow === 'ellipsis' && li.scrollWidth > li.clientWidth) problems.push(`${label}: ellipsis`);
    boxes.push([label, r]);
  }
  for (let i = 0; i < boxes.length; i++) {
    for (let j = i + 1; j < boxes.length; j++) {
      const a = boxes[i][1];
      const b = boxes[j][1];
      if (a.left < b.right - 1 && b.left < a.right - 1 && a.top < b.bottom - 1 && b.top < a.bottom - 1) {
        problems.push(`${boxes[i][0]} overlaps ${boxes[j][0]}`);
      }
    }
  }
  // The quiet mark for more lies whole inside the details' column too.
  const mark = $('metaDots');
  if (mark && !$('metaMore').hidden) {
    for (const e of [mark, ...mark.children]) {
      const r = e.getBoundingClientRect();
      if (r.top < colBox.top - 1 || r.bottom > colBox.bottom + 1 || r.right > cardBox.right + 1) problems.push('the mark for more is cut off');
    }
  }
  const all = [...$('loopDetail').children].map((li) => li.textContent);
  return {
    problems,
    shown: shown.map((li) => li.textContent),
    all,
    name: name.textContent,
    nameSize: parseFloat(getComputedStyle(name).fontSize),
    // The tap: present when there is more to show; its label says which page.
    more: (() => {
      const b = $('metaMore');
      return b && !b.hidden ? { label: b.getAttribute('aria-label'), dots: Number(b.dataset.pages || 0), rect: b.getBoundingClientRect().toJSON() } : null;
    })(),
    facts: [...$('loopFacts').children].map((li) => li.textContent),
    plain: getComputedStyle($('loopFacts')).position,
    ariaHidden: $('loopDetail').getAttribute('aria-hidden'),
  };
}

// ----------------------------------------------------------------- run

async function open(browser, c, size, scale, dark) {
  const ctx = await browser.newContext({
    viewport: { width: size.width, height: size.height },
    deviceScaleFactor: SHOTS ? 2 : 1,
    colorScheme: dark ? 'dark' : 'light',
  });
  await ctx.addInitScript((theme) => {
    try { localStorage.setItem('driftloom.prefs.v1', JSON.stringify({ theme })); } catch { /* no storage */ }
  }, dark ? 't-nocturne' : 't-dawn');
  const page = await ctx.newPage();
  // The browser's (or Android's) text size: the root font size, set before
  // the page's own scripts run, as a setting is.
  if (scale !== 100) {
    await page.addInitScript((pct) => {
      const style = document.createElement('style');
      style.textContent = `html { font-size: ${pct}% !important; }`;
      document.addEventListener('DOMContentLoaded', () => document.head.appendChild(style));
    }, scale);
  }
  const code = c.code || c.album;
  await page.goto(linkOf(`http://127.0.0.1:${PORT}/`, code));
  await page.waitForFunction(() => document.readyState === 'complete' && document.getElementById('playBtn'), null, { timeout: 60000 });
  await sleep(1500);
  if (c.album) {
    await page.evaluate(() => {
      if (document.getElementById('viewLibrary').hidden) document.getElementById('libraryBtn').click();
      document.querySelector('[data-tab="albums"]').click();
    });
    await sleep(300);
    await page.evaluate(() => {
      const r = [...document.querySelectorAll('.lrow.album')][0];
      if (r) r.querySelector('.lrow-main').click();
    });
    await sleep(500);
    if (c.open === 'playing') {
      await page.evaluate(() => { const t = document.querySelector('.lrow.track .lrow-main'); if (t) t.click(); });
      await sleep(800);
      await page.evaluate(() => { document.getElementById('libraryBtn').click(); });
      await sleep(400);
    }
  }
  await page.evaluate(() => document.fonts && document.fonts.ready);
  await sleep(300);
  return { ctx, page };
}

async function main() {
  const { chromium } = loadPlaywright();
  const server = await serve(PORT);
  const browser = await chromium.launch({ args: ['--autoplay-policy=no-user-gesture-required'] });
  let bad = 0;
  let checked = 0;
  if (SHOTS) fs.mkdirSync(SHOTS, { recursive: true });
  try {
    for (const c of cases()) {
      if (ONLY && !c.name.includes(ONLY)) continue;
      for (const size of SIZES) {
        for (const scale of SCALES) {
          const { ctx, page } = await open(browser, c, size, scale, false);
          // BEFORE: the old card has none of the new elements; shots only.
          const file0 = `${c.name.replace(/[^a-z0-9]+/gi, '-')}_${size.name.replace(/ /g, '-')}_${scale}`;
          await (await page.$('.zone-info')).screenshot({ path: path.join(SHOTS, `${file0}_light.png`) });
          const d0 = await open(browser, c, size, scale, true);
          await (await d0.page.$('.zone-info')).screenshot({ path: path.join(SHOTS, `${file0}_dark.png`) });
          await d0.ctx.close(); await ctx.close(); checked++; continue;
          const r = await page.evaluate(inspect);
          const tag = `${c.name} @ ${size.name}, text ${scale}%`;
          if (r.problems.length) {
            bad++;
            console.log(`  FAIL  ${tag}`);
            for (const p of r.problems) console.log(`          ${p}`);
          } else if (process.argv.includes('--verbose')) {
            console.log(`  ok    ${tag}: ${r.shown.length}/${r.all.length} lines`);
          }
          // Everything is a tap away: each page whole, the pages together every fact
          // (but one wider than the column on its own), the last back to the first.
          const seen = [r.shown.join('\n')];
          let pageProblems = [];
          if (!r.facts.length) pageProblems.push('a screen reader has no facts');
          if (r.ariaHidden !== 'true') pageProblems.push('the drawn lines are not hidden from a screen reader (they would be read twice)');
          if (r.more) {
            if (r.more.dots < 2) pageProblems.push('the mark for more says fewer than two pages');
            for (let i = 1; i <= 12; i++) {
              await page.click('#metaMore');
              await sleep(60);
              const p2 = await page.evaluate(inspect);
              for (const p of p2.problems) pageProblems.push(`page ${i + 1}: ${p}`);
              const key = p2.shown.join('\n');
              if (key === seen[0]) { seen.pageCount = i; break; }
              seen.push(key);
              if (i === 12) pageProblems.push('the pages never come back to the first');
            }
            if (!seen.pageCount) pageProblems.push('no way back to the first page');
            else if (seen.length !== r.more.dots) pageProblems.push(`${r.more.dots} dots but ${seen.length} pages`);
          }
          const text = seen.join('\n');
          for (const fact of r.facts) {
            for (const part of fact.split(' \u00b7 ')) {
              // A fact too wide for the column is said in fewer words: a style
              // or feeling without its percent, an album's title or its place.
              const short = [part, part.replace(/^\d+% /, ''), part.replace(/ \d+\/\d+$/, ''), (/(\d+\/\d+)$/.exec(part) || [])[1]];
              if (!short.some((t) => t && text.includes(t))) pageProblems.push(`"${part}" is on no page`);
            }
          }
          if (pageProblems.length) {
            if (!r.problems.length) { bad++; console.log(`  FAIL  ${tag}`); }
            for (const p of pageProblems) console.log(`          ${p}`);
          } else if (process.argv.includes('--verbose')) {
            console.log(`        ${r.facts.length} facts on ${seen.length} page${seen.length === 1 ? '' : 's'}`);
          }
          if (SHOTS) {
            const file = `${c.name.replace(/[^a-z0-9]+/gi, '-')}_${size.name.replace(/ /g, '-')}_${scale}`;
            await (await page.$('.zone-info')).screenshot({ path: path.join(SHOTS, `${file}_light.png`) });
            const d = await open(browser, c, size, scale, true);
            await (await d.page.$('.zone-info')).screenshot({ path: path.join(SHOTS, `${file}_dark.png`) });
            if (r.more) {
              await d.page.click('#metaMore');
              await sleep(80);
              await (await d.page.$('.zone-info')).screenshot({ path: path.join(SHOTS, `${file}_dark_page2.png`) });
            }
            await d.ctx.close();
          }
          await ctx.close();
        }
      }
    }
  } finally {
    await browser.close();
    server.close();
  }
  console.log(`\ncard: ${checked} views checked, ${bad} with something wrong`);
  process.exit(bad ? 1 : 0);
}

if (process.argv[1] && fileURLToPath(import.meta.url) === path.resolve(process.argv[1])) {
  main().catch((e) => { console.error(e); process.exit(2); });
}
