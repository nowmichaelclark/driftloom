//! Logarithms: `log10f` (decibels) and `log2f` (octaves) in 32 bits, and
//! `log`, `log10` and `log2` in 64.
//!
//! Ported from FreeBSD's msun (`e_log10f.c`, `e_log2f.c`; the conversion to
//! float by Ian Lance Taylor, Cygnus Support) by way of Rust's `libm`, the
//! code the WebAssembly build already ran. The 64-bit ones (musl's `log.c`,
//! `log10.c` and `log2.c`, from msun, by way of `libm` too) are for the
//! generator (queue item 77): nothing that renders calls them. The
//! generator's JavaScript calls their mirror (`js/dlmath.js`), so its
//! decisions are the same in every browser and in the Rust port to come.

// fdlibm's own form, kept so it can be read against the original: its
// constants with the digits it gives them, its single-letter names, and the
// casts between floats and their bits that are the method itself, and
// its exact comparisons.
#![allow(
    clippy::excessive_precision,
    clippy::many_single_char_names,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::too_many_lines,
    clippy::unreadable_literal,
    clippy::float_cmp
)]

// ====================================================
// Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
//
// Developed at SunPro, a Sun Microsystems, Inc. business.
// Permission to use, copy, modify, and distribute this
// software is freely granted, provided that this notice
// is preserved.
// ====================================================

/// log2 of `x` split as `k + (hi + lo)/ln2`'s pieces: x = 2^k (1 + f) with
/// 1 + f in [sqrt(2)/2, sqrt(2)], and ln(1 + f) = hi + lo, `hi` with its
/// low 12 bits clear so that products with it are exact. `None` for zero,
/// negatives, infinities and NaN, which the callers answer themselves.
fn reduce(x: f32) -> Option<(f32, f32, f32)> {
    // |(log(1+s) - log(1-s))/s - Lg(s)| < 2^-34.24.
    const LG1: f32 = 0.666_666_626_93;
    const LG2: f32 = 0.400_009_721_52;
    const LG3: f32 = 0.284_987_866_88;
    const LG4: f32 = 0.242_790_788_41;

    let mut ix = x.to_bits();
    let mut k: i32 = 0;
    if ix < 0x0080_0000 || ix >> 31 != 0 {
        // Below 2^-126, zero or negative.
        if ix << 1 == 0 || ix >> 31 != 0 {
            return None;
        }
        // Subnormal (which the core never makes): scale up.
        k -= 25;
        ix = (x * f32::from_bits(0x4c00_0000)).to_bits();
    } else if ix >= 0x7f80_0000 {
        return None;
    }
    // Into [sqrt(2)/2, sqrt(2)].
    ix += 0x3f80_0000 - 0x3f35_04f3;
    k += (ix >> 23) as i32 - 0x7f;
    ix = (ix & 0x007f_ffff) + 0x3f35_04f3;
    let f = f32::from_bits(ix) - 1.0;
    let s = f / (2.0 + f);
    let z = s * s;
    let w = z * z;
    let t1 = w * (LG2 + w * LG4);
    let t2 = z * (LG1 + w * LG3);
    let r = t2 + t1;
    let hfsq = 0.5 * f * f;
    let hi = f32::from_bits((f - hfsq).to_bits() & 0xffff_f000);
    let lo = f - hi - hfsq + s * (hfsq + r);
    Some((k as f32, hi, lo))
}

/// What a logarithm is at zero, below it, at infinity and at NaN.
fn special(x: f32) -> f32 {
    if x == 0.0 {
        f32::NEG_INFINITY
    } else if x > 0.0 {
        // +infinity.
        x
    } else {
        f32::NAN
    }
}

/// The base-10 logarithm, as decibels need: under 1 ulp. 1 gives exactly 0.
#[must_use]
#[inline(never)]
pub fn log10f(x: f32) -> f32 {
    const IVLN10HI: f32 = 4.343_261_718_8e-01;
    const IVLN10LO: f32 = -3.168_997_136_5e-05;
    const LOG10_2HI: f32 = 3.010_292_053_2e-01;
    const LOG10_2LO: f32 = 7.903_415_166_8e-07;
    if x == 1.0 {
        return 0.0;
    }
    let Some((dk, hi, lo)) = reduce(x) else {
        return special(x);
    };
    dk * LOG10_2LO + (lo + hi) * IVLN10LO + lo * IVLN10HI + hi * IVLN10HI + dk * LOG10_2HI
}

/// The base-2 logarithm: under 1 ulp. Powers of two give exact integers.
#[must_use]
#[inline(never)]
pub fn log2f(x: f32) -> f32 {
    const IVLN2HI: f32 = 1.442_871_093_8e+00;
    const IVLN2LO: f32 = -1.760_528_539_3e-04;
    if x == 1.0 {
        return 0.0;
    }
    let Some((dk, hi, lo)) = reduce(x) else {
        return special(x);
    };
    (lo + hi) * IVLN2LO + lo * IVLN2HI + hi * IVLN2HI + dk
}

/// x = 2^k (1 + f) with 1 + f in [sqrt(2)/2, sqrt(2)], and the pieces of
/// ln(1 + f) the 64-bit logarithms share: `(k, f, hfsq, s, r)`, where
/// hfsq = f^2/2, s = f/(2 + f), and ln(1 + f) = f - hfsq + s (hfsq + r).
/// `Err` holds the answer itself, for zero, negatives, one, infinity and
/// NaN.
fn reduce64(x: f64) -> Result<(f64, f64, f64, f64, f64), f64> {
    // |(log(1+s) - log(1-s))/s - Lg(s)| < 2^-58.45.
    const LG1: f64 = 6.666_666_666_666_735_130e-01;
    const LG2: f64 = 3.999_999_999_940_941_908e-01;
    const LG3: f64 = 2.857_142_874_366_239_149e-01;
    const LG4: f64 = 2.222_219_843_214_978_396e-01;
    const LG5: f64 = 1.818_357_216_161_805_012e-01;
    const LG6: f64 = 1.531_383_769_920_937_332e-01;
    const LG7: f64 = 1.479_819_860_511_658_591e-01;

    let mut x = x;
    let mut ui = x.to_bits();
    let mut hx = (ui >> 32) as u32;
    let mut k: i32 = 0;
    if hx < 0x0010_0000 || hx >> 31 != 0 {
        // Below 2^-1022, zero or negative.
        if ui << 1 == 0 {
            return Err(f64::NEG_INFINITY);
        }
        if hx >> 31 != 0 {
            return Err(f64::NAN);
        }
        // Subnormal (which nothing the generator makes): scale up by 2^54.
        k -= 54;
        x *= f64::from_bits(0x4350_0000_0000_0000);
        ui = x.to_bits();
        hx = (ui >> 32) as u32;
    } else if hx >= 0x7ff0_0000 {
        // +infinity or NaN.
        return Err(x);
    } else if hx == 0x3ff0_0000 && ui << 32 == 0 {
        return Err(0.0);
    }
    // Into [sqrt(2)/2, sqrt(2)].
    hx += 0x3ff0_0000 - 0x3fe6_a09e;
    k += (hx >> 20) as i32 - 0x3ff;
    hx = (hx & 0x000f_ffff) + 0x3fe6_a09e;
    let x = f64::from_bits((u64::from(hx) << 32) | (ui & 0xffff_ffff));
    let f = x - 1.0;
    let hfsq = 0.5 * f * f;
    let s = f / (2.0 + f);
    let z = s * s;
    let w = z * z;
    let t1 = w * (LG2 + w * (LG4 + w * LG6));
    let t2 = z * (LG1 + w * (LG3 + w * (LG5 + w * LG7)));
    Ok((f64::from(k), f, hfsq, s, t2 + t1))
}

/// ln(1 + f), from `reduce64`'s pieces, split as `hi + lo` with `hi`'s low
/// 32 bits clear, so that its products with the two-part constants of
/// `log10` and `log2` lose nothing.
fn split(f: f64, hfsq: f64, s: f64, r: f64) -> (f64, f64) {
    let hi = f64::from_bits((f - hfsq).to_bits() & 0xffff_ffff_0000_0000);
    let lo = f - hi - hfsq + s * (hfsq + r);
    (hi, lo)
}

/// The natural logarithm, under 1 ulp. 1 gives exactly 0.
#[must_use]
#[inline(never)]
pub fn log(x: f64) -> f64 {
    const LN2_HI: f64 = 6.931_471_803_691_238_164_90e-01;
    const LN2_LO: f64 = 1.908_214_929_270_587_700_02e-10;
    match reduce64(x) {
        Ok((dk, f, hfsq, s, r)) => s * (hfsq + r) + dk * LN2_LO - hfsq + f + dk * LN2_HI,
        Err(y) => y,
    }
}

/// The base-10 logarithm, under 1 ulp. 1 gives exactly 0.
#[must_use]
#[inline(never)]
pub fn log10(x: f64) -> f64 {
    const IVLN10HI: f64 = 4.342_944_818_781_688_809_39e-01;
    const IVLN10LO: f64 = 2.508_294_671_164_527_522_98e-11;
    const LOG10_2HI: f64 = 3.010_299_956_636_117_713_06e-01;
    const LOG10_2LO: f64 = 3.694_239_077_158_930_786_16e-13;
    let (dk, f, hfsq, s, r) = match reduce64(x) {
        Ok(v) => v,
        Err(y) => return y,
    };
    let (hi, lo) = split(f, hfsq, s, r);
    // val_hi + val_lo ~ log10(1 + f) + k log10(2), the sum taken with its
    // rounding carried.
    let val_hi = hi * IVLN10HI;
    let y = dk * LOG10_2HI;
    let val_lo = dk * LOG10_2LO + (lo + hi) * IVLN10LO + lo * IVLN10HI;
    let w = y + val_hi;
    let val_lo = val_lo + ((y - w) + val_hi);
    val_lo + w
}

/// The base-2 logarithm, under 1 ulp. Powers of two give exact integers.
#[must_use]
#[inline(never)]
pub fn log2(x: f64) -> f64 {
    const IVLN2HI: f64 = 1.442_695_040_721_446_275_71e+00;
    const IVLN2LO: f64 = 1.675_171_316_488_651_183_53e-10;
    let (dk, f, hfsq, s, r) = match reduce64(x) {
        Ok(v) => v,
        Err(y) => return y,
    };
    let (hi, lo) = split(f, hfsq, s, r);
    let val_hi = hi * IVLN2HI;
    let val_lo = (lo + hi) * IVLN2LO + lo * IVLN2HI;
    let w = dk + val_hi;
    let val_lo = val_lo + ((dk - w) + val_hi);
    val_lo + w
}
