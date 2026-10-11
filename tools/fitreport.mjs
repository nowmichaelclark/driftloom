// The readable report for `ranks.mjs --fit` (queue item 35a): a word's
// region first, by group (the five maps), and the honest check on top.

import fs from 'node:fs';
import { DIALS, dialsOf, loadScale } from './dials.mjs';
import {
  observations, fitWords, leaveOneOut, TAP_WEIGHTS, BOOT, MIN_ANSWERS, MIN_EFFECT,
  ALWAYS, NEVER,
} from './fit.mjs';

// The pool-3 groups, as in rank.html and ranks.mjs.
const GROUPS = {
  color: ['golden', 'frosty', 'tender', 'happy', 'joyful', 'reflective', 'merry', 'peaceful',
    'bright', 'sunny', 'starry', 'wondrous', 'triumphant', 'wistful', 'warm', 'cool',
    'heartwarming', 'comforting'],
  shimmer: ['twinkling', 'crisp', 'velvety', 'booming', 'soft', 'chiming', 'hazy', 'intricate'],
  air: ['floating', 'airy', 'intimate', 'cozy', 'spacious', 'serene', 'misty', 'steady',
    'hushed', 'foggy', 'comfy', 'restful', 'soothing', 'reverent', 'meditative', 'minimal'],
  motion: ['bouncy', 'swaying', 'lively', 'energetic', 'still', 'flowing',
    'nimble', 'skipping', 'tiptoeing', 'hypnotic', 'adventurous', 'enthusiastic'],
  flavor: ['sour', 'sweet', 'spicy', 'quirky', 'zany',
    'tangy', 'zesty', 'tart', 'mischievous', 'cheeky', 'plucky', 'mysterious', 'silly',
    'strange', 'goofy', 'experimental', 'refreshing'],
};
const groupOf = (w) => Object.keys(GROUPS).find((g) => GROUPS[g].includes(w)) ?? 'other';

// Same first word on the repeats: his own agreement with himself.
function repeatAgreement(lines) {
  const byTrial = new Map(lines.map((t) => [t.trial, t]));
  let same = 0;
  let n = 0;
  for (const b of lines.filter((t) => t.repeatOf && !t.skip)) {
    const a = byTrial.get(b.repeatOf);
    if (!a || a.skip) continue;
    n++;
    if ((a.taps[0] ?? null) === (b.taps[0] ?? null)) same++;
  }
  return { same, n };
}

const f2 = (v) => (v == null || !Number.isFinite(v) ? '-' : v.toFixed(2));
const sd = (v) => (v >= 0 ? '+' : '') + v.toFixed(2);

export async function fitReport(lines, { dials = null, json = null, boot = BOOT } = {}) {
  const scale = loadScale();
  const fromFile = dials ? JSON.parse(fs.readFileSync(dials, 'utf8')) : null;
  const memo = new Map();
  const dialsFor = (code) => {
    if (!memo.has(code)) {
      memo.set(code, fromFile && fromFile[code] ? fromFile[code].scaled : dialsOf(code, scale).scaled);
    }
    return memo.get(code);
  };

  const data = observations(lines);
  const model = fitWords(data, dialsFor, { boot });
  const loo = leaveOneOut(data, dialsFor, { boot: Math.min(boot, 200) });
  const base = loo.n ? loo.baseHits / loo.n : NaN;
  const fitRate = loo.n ? loo.hits / loo.n : NaN;
  const rep = repeatAgreement(lines);

  const out = [''];
  const pools = Object.entries(data.counts.pools).map(([p, c]) => `pool ${p}: ${c}`).join(', ') || 'no pools';
  out.push(`driftloom: fitting the words to the dials -- ${pools}; word albums: ${data.counts.albums} answers`
    + ` (${data.counts.unsure} not sure left out); ${data.counts.skipped} skipped, ${data.counts.repeats} repeats left out`);
  out.push(`  ${memo.size} distinct loops measured${fromFile ? ' (dials read from file where present)' : ''}; taps weigh ${TAP_WEIGHTS.join(', ')} in the order tapped, untapped 1`);
  out.push('');
  const verdict = fitRate > base
    ? `The fit beats base rates: its top word is the first tap ${loo.hits} of ${loo.n} times (${(100 * fitRate).toFixed(0)}%), base rates alone ${loo.baseHits} (${(100 * base).toFixed(0)}%).`
    : `The fit does NOT beat base rates: its top word is the first tap ${loo.hits} of ${loo.n} times (${(100 * fitRate).toFixed(0)}%), base rates alone ${loo.baseHits} (${(100 * base).toFixed(0)}%).`;
  out.push('  ' + verdict);
  out.push(`  Leave one trial out, fit from the rest (bootstrap ${Math.min(boot, 200)} there), order its five words by how likely each fits the loop:`);
  out.push(`    the fit             ${(100 * fitRate).toFixed(0)}%  (${loo.hits}/${loo.n} trials with a tap)`);
  out.push(`    base rates alone    ${(100 * base).toFixed(0)}%  (each word's fit rate, ignoring the loop)`);
  out.push('    chance              20%');
  out.push(`    his own repeats     ${rep.n ? `${rep.same} of ${rep.n} (${(100 * rep.same / rep.n).toFixed(0)}%)` : '-'} on the first word`);
  const pp = loo.pairs;
  if (pp && pp.n) {
    out.push(`  And a gentler check on the same held-out trials: a word he tapped scores above one he left, in ${pp.n} such pairs:`);
    out.push(`    the fit ${(100 * pp.fit / pp.n).toFixed(0)}%, base rates alone ${(100 * pp.base / pp.n).toFixed(0)}%, chance 50%`);
  }
  out.push('');
  out.push(`  A region: dials whose 80% bootstrap interval (${boot} resamples of trials) stays clear of zero with |d| at least ${MIN_EFFECT}, at most three;`);
  out.push('  d is fits less doesn\'t-fit over the pooled sd; center and spread are the fits\' weighted mean and sd (dials 0..1, 0.5 the catalog median).');
  out.push(`  Fewer than ${MIN_ANSWERS} answers: too few. Fits ${100 * ALWAYS}% or more, or ${100 * NEVER}% or less, of the times shown: listed apart, no region.`);

  const apart = [];
  for (const g of [...Object.keys(GROUPS), 'other']) {
    const words = Object.keys(model).filter((w) => groupOf(w) === g).sort();
    if (!words.length) continue;
    out.push('');
    out.push(`-- ${g}`);
    for (const w of words) {
      const m = model[w];
      const head = `  ${w.padEnd(13)} ${String(m.answers).padStart(3)} answers, fits ${(100 * m.rate).toFixed(0).padStart(3)}%`;
      if (m.status !== 'ok') {
        if (m.status !== 'too few') apart.push(`${w} (${(100 * m.rate).toFixed(0)}% of ${m.answers})`);
        out.push(`${head}   ${m.status}`);
        continue;
      }
      if (!m.region.length) {
        const best = DIALS.filter((k) => m.dials[k]).sort((a, b) => Math.abs(m.dials[b].d) - Math.abs(m.dials[a].d))[0];
        const c = best && m.dials[best];
        out.push(`${head}   no region${c ? ` (nearest: ${best} d ${sd(c.d)}, 80% ${f2(c.lo)}..${f2(c.hi)})` : ''}`);
        continue;
      }
      out.push(`${head}   ` + m.region.map((r) => `${r.dial} ${f2(r.center)} +/- ${f2(r.spread)} (d ${sd(r.d)}, 80% ${f2(r.lo)}..${f2(r.hi)})`).join('; '));
    }
  }
  out.push('');
  out.push(`  apart (fit almost everything or almost nothing): ${apart.join(', ') || 'none'}`);
  out.push('');

  if (json) {
    const doc = {
      about: 'Queue item 35a: per word, the dials of fits against doesn\'t-fits (d, 80% bootstrap interval), the region, and the honest check.',
      counts: data.counts, tapWeights: TAP_WEIGHTS, boot,
      check: { trials: loo.n, fit: loo.hits, baseRates: loo.baseHits, chance: 0.2, repeats: rep, pairs: loo.pairs },
      words: model,
    };
    fs.writeFileSync(json, JSON.stringify(doc, null, 1) + '\n');
    out.push(`  wrote ${json}`);
  }
  return out.join('\n');
}
