//! Logarithms in 32 bits: `log10f` (decibels) and `log2f` (octaves).
//!
//! Ported from FreeBSD's msun (`e_log10f.c`, `e_log2f.c`; the conversion to
//! float by Ian Lance Taylor, Cygnus Support) by way of Rust's `libm`, the
//! code the WebAssembly build already ran.

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
