// Tests for the ten dials (queue item 34, tools/dials.mjs).
// Run with:  node test/dials.test.mjs

import { newSpec, render } from '../js/generator.js';
import { encodeSong } from '../js/share.js';
import { DIALS, CLOUD_DIALS, dialsOf, loadScale, buildScale } from '../tools/dials.mjs';

let failures = 0;
function check(label, condition, detail = '') {
  console.log(condition ? `  pass  ${label}` : `  FAIL  ${label} ${detail}`);
  if (!condition) failures++;
}

const scale = loadScale();

console.log('\nThe dials');

// A still loop and a lively one, found in the catalog rather than built by
// hand, so the test reads loops the generator really makes.
let still = null;
let lively = null;
for (let s = 1; s < 4000 && !(still && lively); s++) {
  const spec = newSpec(s);
  const p = render(spec);
  const drums = p.tracks.drums.some((e) => e.vel > 0);
  if (!still && spec.bpm <= 62 && !drums) still = spec;
  if (!lively && spec.bpm >= 130 && drums) lively = spec;
}
check('found a still loop and a lively one', !!(still && lively));
if (still && lively) {
  const a = dialsOf(still, scale).scaled;
  const b = dialsOf(lively, scale).scaled;
  check('a still loop reads low on energy', a.energy < 0.25, a.energy.toFixed(2));
  check('a fast loop with drums reads high on energy', b.energy > 0.75, b.energy.toFixed(2));
}

// Percentiles inside 0..1, raw values inside 0..1, clouds too.
{
  const bad = [];
  for (let s = 100; s < 400; s++) {
    const d = dialsOf(newSpec(s), scale);
    for (const k of DIALS) {
      if (!(d.scaled[k] >= 0 && d.scaled[k] <= 1)) bad.push(`${s} ${k} scaled ${d.scaled[k]}`);
      if (!(d.raw[k] >= 0 && d.raw[k] <= 1)) bad.push(`${s} ${k} raw ${d.raw[k]}`);
    }
    for (const c of d.cloud) {
      for (const k of CLOUD_DIALS) if (c[k] != null && !(c[k] >= 0 && c[k] <= 1)) bad.push(`${s} bar ${c.bar} ${k} ${c[k]}`);
    }
  }
  check('every dial, raw and scaled, and every bar of every cloud, is inside 0..1 (300 loops)', !bad.length, bad.slice(0, 4).join('; '));
}

// The same code twice, the same numbers.
{
  const code = encodeSong(newSpec(19000));
  check('deterministic: the same code twice gives the same dials',
    JSON.stringify(dialsOf(code, scale)) === JSON.stringify(dialsOf(code, scale)));
}

// The scale is the corpus's: a generator change that moves it fails here
// until test/dial-scale.json is rewritten on purpose (--write-scale).
check('test/dial-scale.json is the current corpus\'s scale (else: node tools/dials.mjs --write-scale)',
  JSON.stringify(buildScale().quantiles) === JSON.stringify(scale.quantiles));

console.log(failures === 0 ? '\nAll good.\n' : `\n${failures} failing.\n`);
process.exit(failures === 0 ? 0 : 1);
