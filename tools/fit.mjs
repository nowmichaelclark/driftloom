// Fitting the words to the dials (queue item 35a).
//
// Run through ranks.mjs:  node tools/ranks.mjs <file.jsonl> --fit [--dials <file.json>] [--json <out>]
//
// Mikey's answers are votes, not verdicts ("I might have selected
// different options on a different day"), so the fit is soft and says how
// sure it is. For each word, the loops it was tapped on (fits) against the
// loops it was shown on and left (doesn't fit), on each of the ten dials
// (tools/dials.mjs, scaled 0..1): the weighted means, their standardized
// difference, and an 80% bootstrap interval of it. The dials whose
// interval stays clear of zero, at most three, are the word's region; the
// rest "don't care".

import { DIALS } from './dials.mjs';
import { Rng } from '../js/rng.js';

// A tap weighs less the later it came: the first word tapped is the one
// most like the loop. Untapped words weigh 1 as doesn't-fit.
export const TAP_WEIGHTS = [1, 0.8, 0.6, 0.5, 0.4];
export const BOOT = 1000;
export const BOOT_SEED = 0x35a;
export const MIN_ANSWERS = 8;
export const MAX_REGION = 3;
// Ten dials, one 80% interval each: a word that means nothing still clears
// zero on about two of them by chance. So a dial joins a region only with
// a medium effect (|d| at least 0.5) as well as an interval clear of zero;
// a fourth or later dial only with its whole interval beyond 0.8 ("unless
// the data clearly carries more").
export const MIN_EFFECT = 0.5;
export const CLEAR_EFFECT = 0.8;
// Fit almost everything, or almost nothing: no region, apart.
export const ALWAYS = 0.9;
export const NEVER = 0.1;

// ------------------------------------------------------------ the answers

// One observation per (answer, word): fits (1) or doesn't (0), a weight,
// and which trial it came from (the bootstrap resamples trials). Five-word
// trials: answered, not skipped, not a repeat (a repeat is there to be
// compared with its original and would count one loop twice). Word-album
// answers (item 37): fits or doesn't; "not sure" left out.
export function observations(lines, tapWeights = TAP_WEIGHTS) {
  const obs = [];
  const counts = { pools: {}, albums: 0, skipped: 0, repeats: 0, unsure: 0 };
  let unit = 0;
  for (const t of lines) {
    if (!t || !t.code || !Array.isArray(t.words)) continue;
    if (t.answer || t.album) {
      if (t.answer === 'unsure' || !t.answer) { counts.unsure++; continue; }
      counts.albums++;
      const w = t.words[0];
      obs.push({ unit, code: t.code, word: w, fit: t.answer === 'fits' ? 1 : 0, weight: 1 });
      unit++;
      continue;
    }
    if (t.skip) { counts.skipped++; continue; }
    if (t.repeatOf) { counts.repeats++; continue; }
    const pool = t.pool ?? 1;
    counts.pools[pool] = (counts.pools[pool] || 0) + 1;
    const taps = t.taps || [];
    for (const w of t.words) {
      const i = taps.indexOf(w);
      obs.push({
        unit, code: t.code, word: w, fit: i >= 0 ? 1 : 0,
        weight: i >= 0 ? (tapWeights[i] ?? tapWeights[tapWeights.length - 1]) : 1,
        first: i === 0,
      });
    }
    unit++;
  }
  return { obs, units: unit, counts };
}

// ------------------------------------------------------------ the fit

// Weighted means and sds of fits and doesn't-fits on each dial, and the
// standardized difference, for every word, given each unit's multiplicity
// (1 for the data as it is, a bootstrap's counts otherwise).
function moments(obs, dialsOf, words, mult) {
  const K = DIALS.length;
  const idx = new Map(words.map((w, i) => [w, i]));
  // [word][class][dial] -> w, wx, wxx
  const acc = new Float64Array(words.length * 2 * K * 3);
  const n = new Float64Array(words.length * 2);
  for (const o of obs) {
    const m = mult ? mult[o.unit] : 1;
    if (!m) continue;
    const wi = idx.get(o.word);
    if (!o.x) { const d = dialsOf(o.code); o.x = Float64Array.from(DIALS, (k) => d[k]); }
    const wt = o.weight * m;
    n[wi * 2 + o.fit] += m;
    for (let k = 0; k < K; k++) {
      const base = ((wi * 2 + o.fit) * K + k) * 3;
      const x = o.x[k];
      acc[base] += wt;
      acc[base + 1] += wt * x;
      acc[base + 2] += wt * x * x;
    }
  }
  const out = {};
  words.forEach((w, wi) => {
    const per = {};
    for (let k = 0; k < K; k++) {
      const a = ((wi * 2 + 1) * K + k) * 3;
      const b = ((wi * 2 + 0) * K + k) * 3;
      const wf = acc[a];
      const wn = acc[b];
      if (!wf || !wn) { per[DIALS[k]] = null; continue; }
      const mf = acc[a + 1] / wf;
      const mn = acc[b + 1] / wn;
      const vf = Math.max(0, acc[a + 2] / wf - mf * mf);
      const vn = Math.max(0, acc[b + 2] / wn - mn * mn);
      const sp = Math.sqrt((vf * wf + vn * wn) / (wf + wn));
      per[DIALS[k]] = { fit: mf, no: mn, sdFit: Math.sqrt(vf), sdNo: Math.sqrt(vn), d: sp > 1e-9 ? (mf - mn) / sp : 0 };
    }
    out[w] = { per, fits: n[wi * 2 + 1], nos: n[wi * 2] };
  });
  return out;
}

function quantile(sorted, q) {
  if (!sorted.length) return NaN;
  const i = (sorted.length - 1) * q;
  const lo = Math.floor(i);
  return sorted[lo] + (sorted[Math.min(lo + 1, sorted.length - 1)] - sorted[lo]) * (i - lo);
}

// The whole fit: per word, per dial, d and its 80% interval; the region;
// the base rate. `dialsOf(code)` gives a code's scaled dials.
export function fitWords(data, dialsOf, { boot = BOOT, seed = BOOT_SEED } = {}) {
  const { obs, units } = data;
  const words = [...new Set(obs.map((o) => o.word))].sort();
  const point = moments(obs, dialsOf, words, null);
  const draws = Object.fromEntries(words.map((w) => [w, Object.fromEntries(DIALS.map((k) => [k, []]))]));
  const r = new Rng(seed);
  const mult = new Uint16Array(units);
  for (let b = 0; b < boot; b++) {
    mult.fill(0);
    for (let i = 0; i < units; i++) mult[Math.floor(r.f() * units)]++;
    const m = moments(obs, dialsOf, words, mult);
    for (const w of words) for (const k of DIALS) {
      const c = m[w].per[k];
      if (c) draws[w][k].push(c.d);
    }
  }
  const model = {};
  for (const w of words) {
    const p = point[w];
    const answers = p.fits + p.nos;
    const rate = answers ? p.fits / answers : 0;
    const dials = {};
    for (const k of DIALS) {
      const c = p.per[k];
      const ds = draws[w][k].sort((a, b) => a - b);
      dials[k] = c ? { ...c, lo: quantile(ds, 0.1), hi: quantile(ds, 0.9) } : null;
    }
    let status = 'ok';
    if (answers < MIN_ANSWERS) status = 'too few';
    else if (rate >= ALWAYS) status = 'fits almost always';
    else if (rate <= NEVER) status = 'fits almost never';
    let region = [];
    if (status === 'ok') {
      const clear = DIALS.filter((k) => {
        const c = dials[k];
        return c && Math.abs(c.d) >= MIN_EFFECT && (c.lo > 0 || c.hi < 0);
      }).sort((a, b) => Math.abs(dials[b].d) - Math.abs(dials[a].d));
      region = clear.filter((k, i) => i < MAX_REGION
        || Math.min(Math.abs(dials[k].lo), Math.abs(dials[k].hi)) >= CLEAR_EFFECT
          && Math.sign(dials[k].lo) === Math.sign(dials[k].hi));
    }
    model[w] = {
      answers, fits: p.fits, rate, status, dials,
      region: region.map((k) => ({ dial: k, center: dials[k].fit, spread: Math.max(0.05, dials[k].sdFit), d: dials[k].d, lo: dials[k].lo, hi: dials[k].hi })),
    };
  }
  return model;
}

// ------------------------------------------------------------ predicting

// How likely a word is to fit a loop: its base rate, moved by each region
// dial's likelihood ratio (fits and doesn't-fits as two normal clouds on
// that dial). A word without a region is its base rate.
export const SD_FLOOR = 0.2;
export const SHRINK_N = 20;
export function fitChance(model, word, d, { sdFloor = SD_FLOOR, shrinkN = SHRINK_N } = {}) {
  const m = model[word];
  if (!m) return null;
  const rate = Math.min(0.98, Math.max(0.02, m.rate));
  let logit = Math.log(rate / (1 - rate));
  const shrink = m.answers / (m.answers + shrinkN);
  for (const r of m.region) {
    const c = m.dials[r.dial];
    const x = d[r.dial];
    const sf = Math.max(sdFloor, c.sdFit);
    const sn = Math.max(sdFloor, c.sdNo);
    logit += shrink * ((-0.5 * ((x - c.fit) / sf) ** 2 - Math.log(sf)) - (-0.5 * ((x - c.no) / sn) ** 2 - Math.log(sn)));
  }
  return 1 / (1 + Math.exp(-logit));
}

// ------------------------------------------------------------ the honest check

// Leave one trial out at a time, fit from the rest, order that trial's five
// words by how likely each is to fit the loop, and count how often the top
// word is the first tapped. Beside it: chance (one in five) and base rates
// alone (each word's fit rate from the rest, ignoring the loop).
export function leaveOneOut(data, dialsOf, { boot = BOOT, seed = BOOT_SEED, chance = {} } = {}) {
  const trials = new Map();
  for (const o of data.obs) {
    if (o.first == null) continue; // album answers have no first tap
    if (!trials.has(o.unit)) trials.set(o.unit, []);
    trials.get(o.unit).push(o);
  }
  let hits = 0;
  let baseHits = 0;
  let n = 0;
  // Beside the brief's check, a gentler one: in the held-out trial, does a
  // word he tapped score above a word he left? (Ties count half.)
  const pairs = { fit: 0, base: 0, n: 0 };
  for (const [unit, list] of trials) {
    const first = list.find((o) => o.first);
    if (!first) continue; // nothing tapped: no first word to predict
    const rest = { obs: data.obs.filter((o) => o.unit !== unit), units: data.units };
    const model = fitWords(rest, dialsOf, { boot, seed: seed + unit });
    const d = dialsOf(first.code);
    const score = (w, f) => (model[w] ? f(w) : 0);
    const top = (f) => list.map((o) => o.word).sort((a, b) => score(b, f) - score(a, f) || a.localeCompare(b))[0];
    if (top((w) => fitChance(model, w, d, chance)) === first.word) hits++;
    if (top((w) => model[w].rate) === first.word) baseHits++;
    n++;
    const tapped = list.filter((o) => o.fit);
    const left = list.filter((o) => !o.fit);
    const cmp = (a, b) => (a > b ? 1 : a === b ? 0.5 : 0);
    for (const t of tapped) for (const u of left) {
      if (!model[t.word] || !model[u.word]) continue;
      pairs.n++;
      pairs.fit += cmp(fitChance(model, t.word, d, chance), fitChance(model, u.word, d, chance));
      pairs.base += cmp(model[t.word].rate, model[u.word].rate);
    }
  }
  return { n, hits, baseHits, pairs };
}
