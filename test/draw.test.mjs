// Tests for the page's copy of the core's draws (queue item 44): js/draw.js
// against test/draw-vectors.txt, which the core's own test
// (core/tests/draws.rs) reads too, and the shape of the draws it makes.
// Needs no browser. Run with:  node test/draw.test.mjs

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { bits, unit, draw, HOST_JITTER, HOST_ENSEMBLE, HOST_STRUM, HOST_DRIFT, HOST_DRIFT_TO, HOST_GLASS, HOST_STAB } from '../js/draw.js';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
let failures = 0;
function check(label, condition, detail = '') {
  console.log(condition ? `  pass  ${label}` : `  FAIL  ${label} ${detail}`);
  if (!condition) failures++;
}

console.log('\nThe hashes the core makes');
{
  const lines = fs.readFileSync(path.join(ROOT, 'test', 'draw-vectors.txt'), 'utf8')
    .split('\n').filter((l) => l.trim() && !l.startsWith('#'));
  let wrong = 0;
  for (const line of lines) {
    const w = line.split(/\s+/);
    const n = w.slice(0, 7).map(Number);
    const want = parseInt(w[7], 16);
    if (bits(...n) !== want) wrong++;
    if (unit(...n) !== (want >>> 8) / 16777216) wrong++;
  }
  check(`${lines.length} vectors`, lines.length >= 80 && wrong === 0, `${wrong} wrong`);
}

console.log('\nThe draws');
{
  const n = 100000;
  let sum = 0, sq = 0, lo = 1, hi = 0;
  for (let i = 0; i < n; i++) {
    const x = unit(Math.imul(i, 2654435761) >>> 0, i % 7, i % 64, i % 5, i % 40, HOST_JITTER, i % 3);
    sum += x; sq += x * x; lo = Math.min(lo, x); hi = Math.max(hi, x);
  }
  const mean = sum / n;
  const sd = Math.sqrt(sq / n - mean * mean);
  check('in [0, 1), reaching both ends', lo >= 0 && hi < 1 && lo < 0.001 && hi > 0.999, `${lo} ${hi}`);
  check('mean a half, spread a twelfth', Math.abs(mean - 0.5) < 0.005 && Math.abs(sd - Math.sqrt(1 / 12)) < 0.003, `${mean} ${sd}`);
  check('the same inputs make the same draw', unit(1, 2, 3, 4, 5, HOST_STRUM, 0) === unit(1, 2, 3, 4, 5, HOST_STRUM, 0));
  check('the kinds differ',
    new Set([HOST_JITTER, HOST_ENSEMBLE, HOST_STRUM, HOST_DRIFT, HOST_DRIFT_TO, HOST_GLASS, HOST_STAB].map((s) => bits(1, 2, 3, 4, 5, s, 0))).size === 7);
  const key = { seed: 77, pass: 1, step: 9, ord: 3 };
  check('a layer\'s draws come from its channel', draw(key, 'drums', HOST_JITTER) === unit(77, 1, 9, 4, 3, HOST_JITTER, 0)
    && draw(key, 'melody', HOST_JITTER) !== draw(key, 'drums', HOST_JITTER));
}

if (failures) {
  console.log(`\n${failures} failed`);
  process.exit(1);
}
console.log('\nAll passed');
