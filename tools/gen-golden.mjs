// The generation goldens (queue item 77): for a fixed set of seeds and
// vibes, hashes of everything the JavaScript generator decides, so the Rust
// port (phase 2, items 79-82) can be held to it step by step, as the core's
// synth was held to its golden renders.
//
//   node tools/gen-golden.mjs              write core/golden/generation.hashes
//   node tools/gen-golden.mjs --check      fail on any line that differs from the file
//   node tools/gen-golden.mjs --chromium   the same, run in headless Chromium (with
//                                          --check, or alone to print the lines)
//   node tools/gen-golden.mjs --show CASE [PART]
//                                          print a case's canonical texts (CASE as the
//                                          file names it, corpus:12; PART one of the
//                                          hashes' names), to see where a port differs
//   node tools/gen-golden.mjs --corpus N   print the lines of the first N corpus loops
//                                          (stdout; for counting what a change moves)
//   options:  --chrome PATH;  --port N
//
// The file. One case a line, after '#' comments:
//
//   SET:INDEX SEED VIBES spec=H plan=H form=H drums=H bass=H chords=H melody=H
//     texture=H meta=H choices=H load=H passes=H1,...,H8
//
// SEED is the loop's seed (unsigned), VIBES the request (queue item 85) as
// its text (`parseRequest`): the vibes asked for, in the frozen order
// (`vibeSelection`), comma-separated, each with its weight or not
// ("glacial=0.7"), the tempo ("bpm=77", "bpm=70-85"), the theme
// ("theme=1234") and the pins ("organ=on", "arc.period=240"; js/pins.js);
// '-' for nothing.
// The loop is `newSpec(SEED, VIBES)`. Each H is an FNV-1a 64 hash (16 hex
// digits) of a canonical text (below) of:
//   spec     the spec, as newSpec makes it;
//   plan     what render plans before any layer plays (`planOf`, queue item
//            79): the development's sections (null for a loop that does not
//            develop), the harmony as drawn, before the register work moves
//            it, and the entry schedules (null for none) and gaps as drawn,
//            before the repairs;
//   form     what `render` decides besides the layers' notes: form, gaps,
//            silentBars, totalSteps, stepsPerBar, cycles, harmony (the
//            progression's slots and events, its shape, longChords);
//   drums ... texture
//            each layer's events, after render (entries, rests and gaps
//            applied);
//   meta     render's meta (the voices, bass style, kit, melody cell,
//            texture);
//   choices  item 54's per-loop choices (`drawChoices`, through render);
//   load     what the engine draws and keeps at load (`Engine.load`):
//            glassScale, stab, organ, wash, arc, kick, subDuck, padSweep,
//            bassLift, late (lateFloor), space, echoBeats and pumpAmount
//            (each null where the loop's choices leave it out), and
//            `calls`, every call it makes to the synth there, in order
//            ([name, ...arguments]: the tone, the character's level, the
//            echo time, the mutes). Nothing at load may draw from
//            Math.random: the tool makes it throw.
//   passes   the first eight wandered passes (`drift`, through
//            `Engine._wander(pass)`, pass 1 to 8, at the default amount, 1):
//            the whole pattern each pass plays, less its spec, and with
//            `kick`, the tuned kick that pass plays (`kickKindOf`) where the
//            loop has one.
//
// The canonical text, which a Rust port writes the same way:
//   - a number that is an integer of at most 2^53 in size, in decimal
//     ('-0' for negative zero); any other finite number 'x' and the 16 hex
//     digits of its IEEE 754 bits (0.5 is x3fe0000000000000); NaN 'nan';
//     the infinities 'inf' and '-inf';
//   - a string in double quotes, '\' and '"' escaped with '\' (printable
//     ASCII only: anything else fails);
//   - true, false, null; undefined, in a list, 'undef';
//   - a list '[a,b,c]';
//   - an object '{k:v,k:v}', its keys sorted (by character code), unquoted,
//     and any key whose value is undefined left out.
// No spaces anywhere. FNV-1a 64 runs over the text's bytes: offset basis
// 0xcbf29ce484222325, prime 0x100000001b3.
//
// The cases (tools/gen-golden-lib.mjs):
//   corpus:0-1999   the first 2,000 loops of corpus seed 1 (Rng(1).seed32(),
//                   as tools/stats.mjs draws its corpus), no vibes;
//   vibes:0-999     1,000 seeds from Rng(77), each with 1, 3 or 5 vibes in
//                   turn, drawn from the same stream (a shuffle of VIBE_IDS);
//   golden:NAME     the 21 golden lists' seeds (core/golden/codes.txt), no
//                   vibes (their codes keep older specs: the case is the seed,
//                   generated today);
//   album:N         the album loops of docs/DONE.md: "dust #7" is loop 7 of
//                   corpus seed 20261009, the ears' corpus; and noan-nou;
//   request:0-359   360 seeds from Rng(85), each with a request of the kind
//                   its index picks (gen-golden-lib.mjs, `requestCase`):
//                   tempo pinned, ranged, weighted vibes, six or seven vibes,
//                   vibes with a pin, weights past 1 with a range;
//   pins:0-239      240 seeds from Rng(8502), each with pins of the kind its
//                   index picks (gen-golden-lib.mjs, `pinCase`; queue item
//                   85b): a choice, several choices, the arc's settings, the
//                   kick's, subDuck's and lateFloor's, padSweep's, the echo's
//                   and the space's, and pins with vibes and a tempo;
//   theme:0-239     240 seeds from Rng(8504), each with a theme of the kind
//                   its index picks (gen-golden-lib.mjs, `themeCase`; queue
//                   item 85d): the loop's own seed alone, with vibes, with a
//                   tempo, with pins, and another seed with vibes;
//   rest:0-23       24 themes whose own loop rests, each on a seed whose loop
//                   draws a choir (gen-golden-lib.mjs, `restCases`; queue
//                   item 85f): the brain's case first, then from Rng(8506),
//                   alone, with vibes, weighted vibes or a tempo.
//
// Run it after any change to the generator, the vibes, the choices or the
// engine's load. A change that is meant to move what is generated writes the
// file again in the same PR and says in the body which cases moved and why.
// CI checks the file in Node and in Chromium.

import http from 'node:http';
import fs from 'node:fs';
import path from 'node:path';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const FILE = path.join(ROOT, 'core', 'golden', 'generation.hashes');
const argv = process.argv.slice(2);
const flag = (name, dflt) => {
  const i = argv.indexOf(name);
  return i >= 0 ? argv[i + 1] : dflt;
};
if (argv.includes('--help') || argv.includes('-h')) {
  console.log(fs.readFileSync(fileURLToPath(import.meta.url), 'utf8').split('\n').filter((l) => l.startsWith('//')).map((l) => l.slice(3)).join('\n'));
  process.exit(0);
}

const HEADER = `# The generation goldens (queue item 77): one case a line, written by
# tools/gen-golden.mjs, which documents the format, the canonical text the
# hashes are taken over and the cases. CI checks the JavaScript generator
# against it (\`node tools/gen-golden.mjs --check\`, in Node and Chromium);
# the Rust generator (phase 2) is held to it.
`;

// The golden lists' [name, seed, vibes], from their codes.
async function goldenCodes() {
  const { decodeSong } = await import('../js/share.js');
  const out = [];
  for (const line of fs.readFileSync(path.join(ROOT, 'core', 'golden', 'codes.txt'), 'utf8').split('\n')) {
    if (!line.trim() || line.startsWith('#')) continue;
    const [name, , , code] = line.trim().split(/\s+/);
    const spec = decodeSong(code);
    out.push([name, spec.seed >>> 0, spec.vibes ? Object.keys(spec.vibes) : null]);
  }
  return out;
}

const lib = await import('./gen-golden-lib.mjs');

// ------------------------------------------------------------- --show

if (argv.includes('--show')) {
  const want = flag('--show', '');
  const part = argv[argv.indexOf('--show') + 2];
  const c = lib.cases(await goldenCodes()).find((x) => `${x.set}:${x.index}` === want);
  if (!c) {
    console.error(`gen-golden: no case ${want}`);
    process.exit(2);
  }
  const { hashes, texts } = lib.hashCase(c, true);
  for (const [k, t] of Object.entries(texts)) {
    if (part && part !== k) continue;
    if (k === 'passes') t.forEach((x, i) => console.log(`passes[${i}] ${hashes.passes[i]}\n${x}\n`));
    else console.log(`${k} ${hashes[k]}\n${t}\n`);
  }
  process.exit(0);
}

// ------------------------------------------------------------- --corpus

if (argv.includes('--corpus')) {
  const n = Number(flag('--corpus', 0));
  const { Rng } = await import('../js/rng.js');
  const r = new Rng(lib.CORPUS_SEED);
  for (let i = 0; i < n; i++) console.log(lib.lineFor({ set: 'corpus', index: i, seed: r.seed32(), vibes: null }));
  process.exit(0);
}

// ------------------------------------------------------------- the lines

async function inNode(golden) {
  return lib.cases(golden).map(lib.lineFor);
}

function loadPlaywright() {
  const require = createRequire(import.meta.url);
  for (const id of ['playwright', 'playwright-core', '/opt/node22/lib/node_modules/playwright']) {
    try { return require(id); } catch { /* try the next one */ }
  }
  try {
    const root = require('node:child_process').execSync('npm root -g', { encoding: 'utf8' }).trim();
    return require(path.join(root, 'playwright'));
  } catch { /* fall through */ }
  console.error('gen-golden: Playwright is not installed.  npm install --no-save playwright@1.56.1 && npx playwright install chromium');
  process.exit(3);
  return null;
}

const PAGE = `<!doctype html><meta charset="utf-8"><title>gen-golden</title>
<script type="module">
import * as lib from '/tools/gen-golden-lib.mjs';
window.lines = (golden) => lib.cases(golden).map(lib.lineFor);
window.ready = true;
</script>`;

async function inChromium(golden) {
  const TYPES = { '.html': 'text/html', '.js': 'text/javascript', '.mjs': 'text/javascript', '.json': 'application/json', '.wasm': 'application/wasm' };
  const server = http.createServer((req, res) => {
    const url = decodeURIComponent(req.url.split('?')[0]);
    if (url === '/__gen.html') {
      res.writeHead(200, { 'content-type': 'text/html' });
      return res.end(PAGE);
    }
    const file = path.join(ROOT, path.normalize(url));
    if (!file.startsWith(ROOT) || !fs.existsSync(file) || fs.statSync(file).isDirectory()) {
      res.writeHead(404);
      return res.end('not found');
    }
    res.writeHead(200, { 'content-type': TYPES[path.extname(file)] || 'application/octet-stream' });
    return fs.createReadStream(file).pipe(res);
  });
  const port = Number(flag('--port', 8777));
  await new Promise((resolve, reject) => {
    server.once('error', reject);
    server.listen(port, '127.0.0.1', resolve);
  });
  const { chromium } = loadPlaywright();
  const browser = await chromium.launch({ executablePath: flag('--chrome', undefined), args: ['--no-sandbox'] });
  try {
    const page = await browser.newPage();
    const errors = [];
    page.on('pageerror', (e) => errors.push(e.message));
    await page.goto(`http://127.0.0.1:${port}/__gen.html`);
    try {
      await page.waitForFunction(() => window.ready === true, null, { timeout: 20000 });
    } catch {
      console.error(`gen-golden: the page never loaded the modules.\n  ${errors.join('\n  ')}`);
      process.exit(1);
    }
    console.error(`gen-golden: in ${browser.version ? `Chromium ${browser.version()}` : 'Chromium'}`);
    return await page.evaluate((g) => window.lines(g), golden);
  } finally {
    await browser.close();
    server.close();
  }
}

const golden = await goldenCodes();
const chromium = argv.includes('--chromium');
const t0 = Date.now();
const lines = chromium ? await inChromium(golden) : await inNode(golden);
const seconds = ((Date.now() - t0) / 1000).toFixed(1);

if (argv.includes('--check')) {
  const kept = fs.readFileSync(FILE, 'utf8').split('\n').filter((l) => l && !l.startsWith('#'));
  const byCase = new Map(kept.map((l) => [l.split(' ')[0], l]));
  let bad = 0;
  for (const line of lines) {
    const name = line.split(' ')[0];
    const was = byCase.get(name);
    byCase.delete(name);
    if (was === line) continue;
    bad++;
    if (bad > 40) continue;
    if (!was) {
      console.log(`  new   ${name}`);
      continue;
    }
    // Which hashes moved, by name (a pass by its number).
    const fields = (l) => Object.fromEntries(l.split(' ').slice(3).map((f) => f.split('=')));
    const [a, b] = [fields(was), fields(line)];
    const moved = Object.keys(b).filter((k) => k !== 'passes' && a[k] !== b[k]);
    const pa = (a.passes || '').split(',');
    const pb = b.passes.split(',');
    pb.forEach((h, i) => { if (pa[i] !== h) moved.push(`pass ${i + 1}`); });
    if (was.split(' ').slice(0, 3).join(' ') !== line.split(' ').slice(0, 3).join(' ')) moved.unshift('the case itself');
    console.log(`  moved ${name}: ${moved.join(', ')}`);
  }
  for (const name of byCase.keys()) {
    bad++;
    console.log(`  gone  ${name}`);
  }
  if (bad > 40) console.log(`  ... and ${bad - 40} more`);
  console.log(bad ? `gen-golden: ${bad} of ${lines.length} cases differ from ${path.relative(ROOT, FILE)} (${seconds} s)`
    : `gen-golden: all ${lines.length} cases equal ${path.relative(ROOT, FILE)}${chromium ? ', in Chromium' : ''} (${seconds} s)`);
  process.exit(bad ? 1 : 0);
}

if (chromium) {
  console.log(lines.join('\n'));
  process.exit(0);
}
fs.writeFileSync(FILE, HEADER + lines.join('\n') + '\n');
console.log(`gen-golden: wrote ${lines.length} cases to ${path.relative(ROOT, FILE)} (${seconds} s)`);
