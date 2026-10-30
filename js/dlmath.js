// The core's own math, in JavaScript (queue item 77).
//
// The generator draws only from its own seeded Rng and does plain
// arithmetic, apart from a few calls to pow, log, sin and the like. Those
// are each browser's own to the last bit (ECMAScript leaves them
// "implementation-approximated"), so a loop's decisions could differ from
// one browser to the next, and the Rust port of the generator (phase 2)
// could never be held to them exactly. So everything on the generator's
// paths calls these instead: a line-for-line mirror of the core's
// functions (`core/src/math/`, FreeBSD's msun by way of musl and Rust's
// `libm`), written with nothing but IEEE 754's basic operations, which
// JavaScript rounds correctly everywhere and never fuses, and the bits of
// a double read through a typed array. They give the core's bits on every
// machine: `test/dlmath.test.mjs` checks each against the core's own, in
// the very `js/dlcore.wasm` the app runs (its `dl_math` export), over a
// million inputs a function.
//
// Mirrored: `pow`, `exp`, `log`, `log2`, `log10`, `sin` and `cos`, in 64
// bits, as the core has them, including the core's choices: a result below
// the smallest normal number is zero, and past 2^20 pi/2 a sine is some
// number in [-1, 1] (`core/src/math/trig.rs`). `sqrt` is IEEE 754's
// correctly rounded root in both, which `Math.sqrt` is (ECMAScript does
// not leave it approximated).
//
// Keep each function as the core's is, operation for operation and in the
// same order: a reordered sum is a different last bit. A change to one
// side is a change to both.

const F = new Float64Array(1);
const U = new Uint32Array(F.buffer);
// Which 32-bit half of a double is its high word, sign and exponent.
const LE = new Uint8Array(new Uint16Array([1]).buffer)[0] === 1;
const HI = LE ? 1 : 0;
const LO = LE ? 0 : 1;

// The high word of `x`, as a signed 32-bit integer.
function hiOf(x) {
  F[0] = x;
  return U[HI] | 0;
}

// The low word of `x`, as an unsigned one.
function loOf(x) {
  F[0] = x;
  return U[LO];
}

// The double with these words.
function fromWords(hi, lo) {
  U[HI] = hi >>> 0;
  U[LO] = lo >>> 0;
  return F[0];
}

// `x` with its low 32 bits cleared.
function high(x) {
  F[0] = x;
  U[LO] = 0;
  return F[0];
}

// `x` with its high 32 bits replaced by `hi`.
function withHigh(x, hi) {
  F[0] = x;
  U[HI] = hi >>> 0;
  return F[0];
}

// Rust's `as i32` from a double: toward zero, saturating, NaN to 0.
function toI32(x) {
  if (x !== x) return 0;
  if (x >= 2147483647) return 2147483647;
  if (x <= -2147483648) return -2147483648;
  return Math.trunc(x) | 0;
}

const MIN_NORMAL = 2.2250738585072014e-308;

// `x`, or a zero of its sign if it is subnormal (`math::flush`).
function flush(x) {
  return Math.abs(x) < MIN_NORMAL ? x * 0 : x;
}

// `x * 2^n`, a result below the smallest normal number flushed to zero
// (`math::scalbn`).
function scalbn(x, n) {
  let y = x;
  if (n > 1023) {
    y *= fromWords(0x7fe00000, 0);
    n -= 1023;
    if (n > 1023) {
      y *= fromWords(0x7fe00000, 0);
      n -= 1023;
      n = Math.min(n, 1023);
    }
  } else if (n < -1022) {
    y *= fromWords(0x03600000, 0);
    n += 1022 - 53;
    if (n < -1022) {
      y *= fromWords(0x03600000, 0);
      n += 1022 - 53;
      n = Math.max(n, -1022);
    }
  }
  return flush(y * fromWords((0x3ff + n) << 20, 0));
}

/** The square root, correctly rounded (`math::sqrt`). */
export const sqrt = Math.sqrt;

// ---------------------------------------------------------------- exp

const LN2_HI = 6.93147180369123816490e-01;
const LN2_LO = 1.90821492927058770002e-10;
const INV_LN2 = 1.44269504088896340735992468100189214;
const E_P1 = 1.66666666666666019037e-01;
const E_P2 = -2.77777777770155933842e-03;
const E_P3 = 6.61375632143793436117e-05;
const E_P4 = -1.65339022054652515390e-06;
const E_P5 = 4.13813679705723846039e-08;

/** e^x (`math::exp`): zero below about -708.4, where it would be subnormal. */
export function exp(x) {
  let hx = hiOf(x);
  const negative = hx < 0;
  hx &= 0x7fffffff;
  if (hx >= 0x4086232b) {
    if (x !== x) return x;
    if (x > 709.782712893383973096) return Infinity;
    if (x < -708.39641853226410622) return 0;
  }
  let hi;
  let lo;
  let k;
  if (hx > 0x3fd62e42) {
    if (hx >= 0x3ff0a2b2) k = toI32(INV_LN2 * x + (negative ? -0.5 : 0.5));
    else k = negative ? -1 : 1;
    hi = x - k * LN2_HI;
    lo = k * LN2_LO;
  } else if (hx > 0x3e300000) {
    hi = x;
    lo = 0;
    k = 0;
  } else {
    return 1 + x;
  }
  const r = k === 0 ? hi : hi - lo;
  const rr = r * r;
  const c = r - rr * (E_P1 + rr * (E_P2 + rr * (E_P3 + rr * (E_P4 + rr * E_P5))));
  const y = 1 + (r * c / (2 - c) - lo + hi);
  return k === 0 ? y : scalbn(y, k);
}

// ---------------------------------------------------------------- pow

const DP_H1 = 5.84962487220764160156e-01;
const DP_L1 = 1.35003920212974897128e-08;
const TWO53 = 9007199254740992;
const L1 = 5.99999999999994648725e-01;
const L2 = 4.28571428578550184252e-01;
const L3 = 3.33333329818377432918e-01;
const L4 = 2.72728123808534006489e-01;
const L5 = 2.30660745775561754067e-01;
const L6 = 2.06975017800338417784e-01;
const LG2 = 0.693147180559945309417232121458176568;
const LG2_H = 6.93147182464599609375e-01;
const LG2_L = -1.90465429995776804525e-09;
const OVT = 8.0085662595372944372e-17;
const CP = 9.61796693925975554329e-01;
const CP_H = 9.61796700954437255859e-01;
const CP_L = -7.02846165095275826516e-09;
const IVLN2 = 1.44269504088896340735992468100189214;
const IVLN2_H = 1.44269502162933349609e+00;
const IVLN2_L = 1.92596299112661746887e-08;

// Whether `y`, as a power of a negative number, is an odd integer (1), an
// even one (2), or not an integer (0); `iy` its high word less the sign,
// `ly` its low word.
function integerKind(iy, ly) {
  if (iy >= 0x43400000) return 2;
  if (iy < 0x3ff00000) return 0;
  const k = (iy >> 20) - 0x3ff;
  if (k > 20) {
    const j = ly >>> (52 - k);
    if (((j << (52 - k)) >>> 0) === ly) return 2 - (j & 1);
  } else if (ly === 0) {
    const j = iy >> (20 - k);
    if ((j << (20 - k)) === iy) return 2 - (j & 1);
  }
  return 0;
}

/** `x` to the power `y` (`math::pow`), with C's special cases. */
export function pow(x, y) {
  const hx = hiOf(x);
  const lx = loOf(x);
  const hy = hiOf(y);
  const ly = loOf(y);
  let ix = hx & 0x7fffffff;
  const iy = hy & 0x7fffffff;

  if ((iy | ly) === 0 || (hx === 0x3ff00000 && lx === 0)) return 1;
  if (ix > 0x7ff00000 || (ix === 0x7ff00000 && lx !== 0) || iy > 0x7ff00000 || (iy === 0x7ff00000 && ly !== 0)) {
    return NaN;
  }
  const yisint = hx < 0 ? integerKind(iy, ly) : 0;

  if (ly === 0) {
    if (iy === 0x7ff00000) {
      if (((ix - 0x3ff00000) | lx) === 0) return 1;
      if (ix >= 0x3ff00000) return hy >= 0 ? y : 0;
      return hy >= 0 ? 0 : -y;
    }
    if (iy === 0x3ff00000) return hy >= 0 ? x : 1 / x;
    if (hy === 0x40000000) return x * x;
    if (hy === 0x3fe00000 && hx >= 0) return sqrt(x);
  }

  let ax = Math.abs(x);
  if (lx === 0 && (ix === 0x7ff00000 || ix === 0 || ix === 0x3ff00000)) {
    let z = hy < 0 ? 1 / ax : ax;
    if (hx < 0) {
      if (((ix - 0x3ff00000) | yisint) === 0) z = NaN;
      else if (yisint === 1) z = -z;
    }
    return z;
  }

  let s = 1;
  if (hx < 0) {
    if (yisint === 0) return NaN;
    if (yisint === 1) s = -1;
  }

  let t1;
  let t2;
  if (iy > 0x41e00000) {
    if (iy > 0x43f00000) {
      if (ix <= 0x3fefffff) return hy < 0 ? Infinity : 0;
      if (ix >= 0x3ff00000) return hy > 0 ? Infinity : 0;
    }
    if (ix < 0x3fefffff) return hy < 0 ? s * Infinity : s * 0;
    if (ix > 0x3ff00000) return hy > 0 ? s * Infinity : s * 0;
    const t = ax - 1;
    const w = (t * t) * (0.5 - t * (0.3333333333333333333333 - t * 0.25));
    const u = IVLN2_H * t;
    const v = t * IVLN2_L - w * IVLN2;
    t1 = high(u + v);
    t2 = v - (t1 - u);
  } else {
    let n = 0;
    if (ix < 0x00100000) {
      ax *= TWO53;
      n -= 53;
      ix = hiOf(ax);
    }
    n += (ix >> 20) - 0x3ff;
    const j = ix & 0x000fffff;
    ix = j | 0x3ff00000;
    let k;
    if (j <= 0x3988e) {
      k = 0;
    } else if (j < 0xbb67a) {
      k = 1;
    } else {
      k = 0;
      n += 1;
      ix -= 0x00100000;
    }
    const bp = k === 0 ? 1 : 1.5;
    const dpH = k === 0 ? 0 : DP_H1;
    const dpL = k === 0 ? 0 : DP_L1;
    ax = withHigh(ax, ix);

    const u = ax - bp;
    const v = 1 / (ax + bp);
    const ss = u * v;
    const sH = high(ss);
    let tH = fromWords(((ix >>> 1) | 0x20000000) + 0x00080000 + (k << 18), 0);
    let tL = ax - (tH - bp);
    const sL = v * ((u - sH * tH) - sH * tL);
    let s2 = ss * ss;
    let r = s2 * s2 * (L1 + s2 * (L2 + s2 * (L3 + s2 * (L4 + s2 * (L5 + s2 * L6)))));
    r += sL * (sH + ss);
    s2 = sH * sH;
    tH = high(3 + s2 + r);
    tL = r - ((tH - 3) - s2);
    const u2 = sH * tH;
    const v2 = sL * tH + tL * ss;
    const pH = high(u2 + v2);
    const pL = v2 - (pH - u2);
    const zH = CP_H * pH;
    const zL = CP_L * pH + pL * CP + dpL;
    const t = n;
    t1 = high(((zH + zL) + dpH) + t);
    t2 = zL - (((t1 - t) - dpH) - zH);
  }

  const y1 = high(y);
  const pL = (y - y1) * t1 + y * t2;
  let pH = y1 * t1;
  let z = pL + pH;
  const j = hiOf(z);
  const i = loOf(z) | 0;
  if (j >= 0x40900000) {
    if (((j - 0x40900000) | i) !== 0 || pL + OVT > z - pH) return s * Infinity;
  } else if ((j & 0x7fffffff) >= 0x4090cc00) {
    if ((((j >>> 0) - 0xc090cc00) | i) !== 0 || pL <= z - pH) return s * 0;
  }

  const ia = j & 0x7fffffff;
  let k = (ia >> 20) - 0x3ff;
  let n = 0;
  if (ia > 0x3fe00000) {
    n = (j + (0x00100000 >> (k + 1))) | 0;
    k = ((n & 0x7fffffff) >> 20) - 0x3ff;
    const t = fromWords(n & ~(0x000fffff >> k), 0);
    n = ((n & 0x000fffff) | 0x00100000) >> (20 - k);
    if (j < 0) n = -n;
    pH -= t;
  }
  const t = high(pL + pH);
  const u = t * LG2_H;
  const v = (pL - (t - pH)) * LG2 + t * LG2_L;
  z = u + v;
  const w = v - (z - u);
  const tt = z * z;
  const tt1 = z - tt * (E_P1 + tt * (E_P2 + tt * (E_P3 + tt * (E_P4 + tt * E_P5))));
  const r = (z * tt1) / (tt1 - 2) - (w + z * w);
  z = 1 - (r - z);
  const jj = (hiOf(z) + (n << 20)) | 0;
  if (jj >> 20 <= 0) z = scalbn(z, n);
  else z = withHigh(z, jj);
  return s * z;
}

// ---------------------------------------------------------------- logarithms

const LG_1 = 6.666666666666735130e-01;
const LG_2 = 3.999999999940941908e-01;
const LG_3 = 2.857142874366239149e-01;
const LG_4 = 2.222219843214978396e-01;
const LG_5 = 1.818357216161805012e-01;
const LG_6 = 1.531383769920937332e-01;
const LG_7 = 1.479819860511658591e-01;

// What the logarithms share (`reduce64` in `core/src/math/log.rs`): x =
// 2^k (1 + f), 1 + f in [sqrt(2)/2, sqrt(2)], and ln(1 + f)'s pieces, left
// in R; or the answer itself (zero, negatives, one, infinity, NaN), as the
// return value. Returns null when R holds the pieces.
const R = { dk: 0, f: 0, hfsq: 0, s: 0, r: 0 };

function reduce64(x) {
  F[0] = x;
  let lo = U[LO];
  let hx = U[HI];
  let k = 0;
  if (hx < 0x00100000 || hx >>> 31 !== 0) {
    if (((hx << 1) | lo) === 0) return -Infinity;
    if (hx >>> 31 !== 0) return NaN;
    k -= 54;
    x *= fromWords(0x43500000, 0);
    F[0] = x;
    lo = U[LO];
    hx = U[HI];
  } else if (hx >= 0x7ff00000) {
    return x;
  } else if (hx === 0x3ff00000 && lo === 0) {
    return 0;
  }
  hx = (hx + (0x3ff00000 - 0x3fe6a09e)) >>> 0;
  k += (hx >>> 20) - 0x3ff;
  hx = (hx & 0x000fffff) + 0x3fe6a09e;
  const f = fromWords(hx, lo) - 1;
  const hfsq = 0.5 * f * f;
  const s = f / (2 + f);
  const z = s * s;
  const w = z * z;
  const t1 = w * (LG_2 + w * (LG_4 + w * LG_6));
  const t2 = z * (LG_1 + w * (LG_3 + w * (LG_5 + w * LG_7)));
  R.dk = k;
  R.f = f;
  R.hfsq = hfsq;
  R.s = s;
  R.r = t2 + t1;
  return null;
}

/** The natural logarithm (`math::log`). */
export function log(x) {
  const special = reduce64(x);
  if (special !== null) return special;
  const { dk, f, hfsq, s, r } = R;
  return s * (hfsq + r) + dk * LN2_LO - hfsq + f + dk * LN2_HI;
}

const IVLN10HI = 4.34294481878168880939e-01;
const IVLN10LO = 2.50829467116452752298e-11;
const LOG10_2HI = 3.01029995663611771306e-01;
const LOG10_2LO = 3.69423907715893078616e-13;

/** The base-10 logarithm (`math::log10`). */
export function log10(x) {
  const special = reduce64(x);
  if (special !== null) return special;
  const { dk, f, hfsq, s, r } = R;
  const hi = high(f - hfsq);
  const lo = f - hi - hfsq + s * (hfsq + r);
  const valHi = hi * IVLN10HI;
  const y = dk * LOG10_2HI;
  let valLo = dk * LOG10_2LO + (lo + hi) * IVLN10LO + lo * IVLN10HI;
  const w = y + valHi;
  valLo = valLo + ((y - w) + valHi);
  return valLo + w;
}

const IVLN2HI = 1.44269504072144627571e+00;
const IVLN2LO = 1.67517131648865118353e-10;

/** The base-2 logarithm (`math::log2`): powers of two give exact integers. */
export function log2(x) {
  const special = reduce64(x);
  if (special !== null) return special;
  const { dk, f, hfsq, s, r } = R;
  const hi = high(f - hfsq);
  const lo = f - hi - hfsq + s * (hfsq + r);
  const valHi = hi * IVLN2HI;
  let valLo = (lo + hi) * IVLN2LO + lo * IVLN2HI;
  const w = dk + valHi;
  valLo = valLo + ((dk - w) + valHi);
  return valLo + w;
}

// ---------------------------------------------------------------- sin, cos

// The high word of 2^20 pi/2: where the three-part pi/2 runs out.
const HUGE = 0x413921fb;
const PIO2_1 = 1.57079632673412561417e+00;
const PIO2_1T = 6.07710050650619224932e-11;
const PIO2_2 = 6.07710050630396597660e-11;
const PIO2_2T = 2.02226624879595063154e-21;
const PIO2_3 = 2.02226624871116645580e-21;
const PIO2_3T = 8.47842766036889956997e-32;
const INV_PIO2 = 0.636619772367581343075535053490057448;
const TO_INT = 6755399441055744;

const S1 = -1.66666666666666324348e-01;
const S2 = 8.33333333332248946124e-03;
const S3 = -1.98412698298579493134e-04;
const S4 = 2.75573137070700676789e-06;
const S5 = -2.50507602534068634195e-08;
const S6 = 1.58969099521155010221e-10;

function kSin(x, y, tail) {
  const z = x * x;
  const w = z * z;
  const r = S2 + z * (S3 + z * S4) + z * w * (S5 + z * S6);
  const v = z * x;
  if (tail) return x - ((z * (0.5 * y - v * r) - y) - v * S1);
  return x + v * (S1 + z * r);
}

const C1 = 4.16666666666666019037e-02;
const C2 = -1.38888888888741095749e-03;
const C3 = 2.48015872894767294178e-05;
const C4 = -2.75573143513906633035e-07;
const C5 = 2.08757232129817482790e-09;
const C6 = -1.13596475577881948265e-11;

function kCos(x, y) {
  const z = x * x;
  const w = z * z;
  const r = z * (C1 + z * (C2 + z * C3)) + w * w * (C4 + z * (C5 + z * C6));
  const hz = 0.5 * z;
  const w1 = 1 - hz;
  return w1 + (((1 - w1) - hz) + (z * r - x * y));
}

// rem_pio2's answer: x less n times pi/2, as y0 + y1.
const P = { n: 0, y0: 0, y1: 0 };

function exponentOf(y) {
  return (hiOf(y) >>> 20) & 0x7ff;
}

function medium(x, ix) {
  const fN = (x * INV_PIO2 + TO_INT) - TO_INT;
  const n = toI32(fN);
  let r = x - fN * PIO2_1;
  let w = fN * PIO2_1T;
  let y0 = r - w;
  const ex = ix >>> 20;
  if (ex - exponentOf(y0) > 16) {
    let t = r;
    w = fN * PIO2_2;
    r = t - w;
    w = fN * PIO2_2T - ((t - r) - w);
    y0 = r - w;
    if (ex - exponentOf(y0) > 49) {
      t = r;
      w = fN * PIO2_3;
      r = t - w;
      w = fN * PIO2_3T - ((t - r) - w);
      y0 = r - w;
    }
  }
  P.n = n;
  P.y0 = y0;
  P.y1 = (r - y0) - w;
}

function near(x, sign, k) {
  let z;
  let y0;
  if (sign) {
    z = x + k * PIO2_1;
    y0 = z + k * PIO2_1T;
  } else {
    z = x - k * PIO2_1;
    y0 = z - k * PIO2_1T;
  }
  P.y1 = sign ? (z - y0) + k * PIO2_1T : (z - y0) - k * PIO2_1T;
  P.y0 = y0;
  P.n = sign ? -k : k;
}

function remPio2(x) {
  const hx = hiOf(x);
  const sign = hx < 0;
  const ix = hx & 0x7fffffff;
  if (ix <= 0x400f6a7a) {
    if ((ix & 0xfffff) === 0x921fb) return medium(x, ix);
    return near(x, sign, ix <= 0x4002d97c ? 1 : 2);
  }
  if (ix <= 0x401c463b) {
    if (ix <= 0x4015fdbc) {
      if (ix === 0x4012d97c) return medium(x, ix);
      return near(x, sign, 3);
    }
    if (ix === 0x401921fb) return medium(x, ix);
    return near(x, sign, 4);
  }
  return medium(x, ix);
}

function withinOne(y) {
  return y !== y ? 0 : Math.min(1, Math.max(-1, y));
}

/** The sine of `x`, in radians (`math::sin`). */
export function sin(x) {
  const ix = hiOf(x) & 0x7fffffff;
  if (ix <= 0x3fe921fb) {
    if (ix < 0x3e500000) return x;
    return kSin(x, 0, false);
  }
  if (ix >= 0x7ff00000) return NaN;
  remPio2(x);
  const { y0, y1 } = P;
  let y;
  switch (P.n & 3) {
    case 0: y = kSin(y0, y1, true); break;
    case 1: y = kCos(y0, y1); break;
    case 2: y = -kSin(y0, y1, true); break;
    default: y = -kCos(y0, y1);
  }
  return ix < HUGE ? y : withinOne(y);
}

/** The cosine of `x`, in radians (`math::cos`). */
export function cos(x) {
  const ix = hiOf(x) & 0x7fffffff;
  if (ix <= 0x3fe921fb) {
    if (ix < 0x3e46a09e) return 1;
    return kCos(x, 0);
  }
  if (ix >= 0x7ff00000) return NaN;
  remPio2(x);
  const { y0, y1 } = P;
  let y;
  switch (P.n & 3) {
    case 0: y = kCos(y0, y1); break;
    case 1: y = -kSin(y0, y1, true); break;
    case 2: y = -kCos(y0, y1); break;
    default: y = kSin(y0, y1, true);
  }
  return ix < HUGE ? y : withinOne(y);
}
