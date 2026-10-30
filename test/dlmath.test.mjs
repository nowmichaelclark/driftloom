// The generator's math (js/dlmath.js, queue item 77) against the core's own,
// bit for bit: each function over a million inputs, called in the module the
// app runs (js/dlcore.wasm, its `dl_math` export) and in the mirror.
// Run with:  node test/dlmath.test.mjs
//
// The inputs: the ranges the generator calls each function on (decibels,
// pitches, the exponential draws, the arc's angles, the kick's fit), wide
// spreads over every binade, any 64-bit pattern at all (NaNs, infinities,
// subnormals, huge angles), and the special cases one by one. NaN equals NaN
// here whatever its payload: nothing reads one.

import fs from 'node:fs';
import { mulberry32 } from '../js/rng.js';
import * as m from '../js/dlmath.js';

const N = 1_000_000;
let failures = 0;
function check(label, condition, detail = '') {
  console.log(condition ? `  pass  ${label}` : `  FAIL  ${label} ${detail}`);
  if (!condition) failures++;
}

const bytes = fs.readFileSync(new URL('../js/dlcore.wasm', import.meta.url));
const { exports: core } = new WebAssembly.Instance(new WebAssembly.Module(bytes), {});
const WHICH = { pow: 0, exp: 1, log: 2, log2: 3, log10: 4, sin: 5, cos: 6, sqrt: 7 };

const F = new Float64Array(2);
const B = new BigUint64Array(F.buffer);
function same(a, b) {
  if (a !== a && b !== b) return true;
  F[0] = a;
  F[1] = b;
  return B[0] === B[1];
}

const r = mulberry32(0x77d1a7);
const unit = () => r();
const within = (lo, hi) => lo + (hi - lo) * unit();
const binades = (lo, hi) => Math.exp(Math.log(lo) + (Math.log(hi) - Math.log(lo)) * unit());
const U = new Uint32Array(F.buffer);
// Any 64-bit pattern.
const any = () => {
  U[0] = (r() * 4294967296) >>> 0;
  U[1] = (r() * 4294967296) >>> 0;
  return F[0];
};

const SPECIAL = [0, -0, 1, -1, 0.5, -0.5, 2, -2, 3, 10, 0.1, Infinity, -Infinity, NaN,
  Number.MIN_VALUE, -Number.MIN_VALUE, 2.2250738585072014e-308, Number.MAX_VALUE, -Number.MAX_VALUE,
  Math.PI, -Math.PI, Math.PI / 2, Math.PI / 4, 3 * Math.PI / 2, 2 * Math.PI, 5 * Math.PI / 4, 9 * Math.PI / 4,
  1e7, -3e9, 1e15, 1e22, 1.7e308, 708.39, -708.39, -708.5, 709.78, 709.79, -745, 1024, -1075, 1e-300, 4e-320];

// Each function: its draws, mixed, a million in all; `pair` for pow.
const CASES = {
  pow: [
    () => [10, within(-200, 200) / 20],
    () => [2, within(-100, 100) / 12],
    () => [unit(), within(0.2, 5)],
    () => [binades(1e-3, 1e3), within(-30, 30)],
    () => [within(0.5, 200), 4],
    () => [binades(1e-6, 1e6), 2.2],
    () => [within(0.2, 3), within(-2, 2)],
    () => [any(), any()],
    () => [-within(0, 100), Math.round(within(-40, 40))],
    () => [SPECIAL[(r() * SPECIAL.length) | 0], SPECIAL[(r() * SPECIAL.length) | 0]],
  ],
  exp: [() => within(-10, 10), () => within(-750, 750), () => any(), () => SPECIAL[(r() * SPECIAL.length) | 0]],
  log: [() => 1 - unit() * 0.999, () => binades(1e-300, 1e300), () => within(0.5, 2), () => any(), () => SPECIAL[(r() * SPECIAL.length) | 0]],
  log2: [() => Math.round(within(1, 65536)), () => binades(1e-300, 1e300), () => within(0.5, 2), () => any(), () => SPECIAL[(r() * SPECIAL.length) | 0]],
  log10: [() => within(0.001, 1), () => binades(1e-300, 1e300), () => within(0.5, 2), () => any(), () => SPECIAL[(r() * SPECIAL.length) | 0]],
  sin: [() => within(0, Math.PI), () => within(-4, 4), () => within(-1e4, 1e4), () => within(-2e6, 2e6), () => any(), () => SPECIAL[(r() * SPECIAL.length) | 0]],
  cos: [() => within(0, 2 * Math.PI), () => within(-4, 4), () => within(-1e4, 1e4), () => within(-2e6, 2e6), () => any(), () => SPECIAL[(r() * SPECIAL.length) | 0]],
  sqrt: [() => within(0, 1e6), () => binades(1e-300, 1e300), () => any(), () => SPECIAL[(r() * SPECIAL.length) | 0]],
};

console.log('\nThe mirror against the core, a million inputs a function');
for (const [name, draws] of Object.entries(CASES)) {
  const which = WHICH[name];
  const mirror = m[name];
  let bad = 0;
  let first = '';
  // Every special value, and every pair of them for pow, first.
  const inputs = name === 'pow' ? SPECIAL.flatMap((x) => SPECIAL.map((y) => [x, y])) : SPECIAL.map((x) => [x, 0]);
  for (const [x, y] of inputs) {
    const a = core.dl_math(which, x, y);
    const b = name === 'pow' ? mirror(x, y) : mirror(x);
    if (!same(a, b)) {
      bad++;
      if (!first) first = `at ${x}${name === 'pow' ? `, ${y}` : ''}: core ${a}, mirror ${b}`;
    }
  }
  for (let i = inputs.length; i < N; i++) {
    const d = draws[i % draws.length]();
    const [x, y] = name === 'pow' ? d : [d, 0];
    const a = core.dl_math(which, x, y);
    const b = name === 'pow' ? mirror(x, y) : mirror(x);
    if (!same(a, b)) {
      bad++;
      if (!first) first = `at ${x}${name === 'pow' ? `, ${y}` : ''}: core ${a}, mirror ${b}`;
    }
  }
  check(`${name}: equal on all ${N.toLocaleString('en-US')}`, bad === 0, `${bad} differ, first ${first}`);
}

console.log('\nWhat the mirror promises on its own');
check('log2 of powers of two is exact', Array.from({ length: 2098 }, (_, i) => i - 1074).every((k) => m.log2(2 ** k) === k));
check('log10 of powers of ten is exact', Array.from({ length: 23 }, (_, i) => i).every((k) => m.log10(10 ** k) === k));
check('pow of small integers is exact', [[2, 10, 1024], [3, 4, 81], [10, 3, 1000], [7, 2, 49]].every(([x, y, z]) => m.pow(x, y) === z));
check('no subnormal comes out', m.exp(-708.5) === 0 && m.pow(2, -1060) === 0 && m.pow(0.5, 1030.5) === 0);
check('huge angles stay in [-1, 1]', [1e7, -3e9, 1e15, 1e22, 1.7e308].every((x) => Math.abs(m.sin(x)) <= 1 && Math.abs(m.cos(x)) <= 1));

console.log(failures ? `\n${failures} FAILED` : '\nall passed');
process.exit(failures ? 1 : 0);
