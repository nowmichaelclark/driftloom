//! Sine, cosine and arcsine, in 64 bits.
//!
//! Ported from FreeBSD's msun (`s_sin.c`, `s_cos.c`, `k_sin.c`, `k_cos.c`,
//! `e_rem_pio2.c`, `e_asin.c`) by way of musl and Rust's `libm`, the code
//! the WebAssembly build already ran: every argument the core passes gives
//! the same bits as before. Under 1 ulp from the true value (`tests/math.rs`
//! measures it).

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
//
// `e_rem_pio2.c` optimized by Bruce D. Evans.

use super::exact::sqrt;

/// The sine of `x`, in radians.
///
/// Accurate to under 1 ulp for |x| up to 2^20 pi/2 (about 1.6 million),
/// far past anything the core asks (its largest is the resampler's window,
/// a few hundred). Beyond that the reduction below runs out of digits of
/// pi; fdlibm then reduces with a 1,200-digit table of 2/pi, which the core
/// leaves out: there the answer is the same on every machine, kept within
/// [-1, 1], but no longer accurate.
#[must_use]
#[inline(never)]
pub fn sin(x: f64) -> f64 {
    let ix = (x.to_bits() >> 32) as u32 & 0x7fff_ffff;
    if ix <= 0x3fe9_21fb {
        // |x| <= pi/4 or so: no reduction.
        if ix < 0x3e50_0000 {
            // |x| < 2^-26: sin(x) is x to the last bit.
            return x;
        }
        return k_sin(x, 0.0, false);
    }
    if ix >= 0x7ff0_0000 {
        // Infinite or NaN.
        return f64::NAN;
    }
    let (n, y0, y1) = rem_pio2(x);
    let y = match n & 3 {
        0 => k_sin(y0, y1, true),
        1 => k_cos(y0, y1),
        2 => -k_sin(y0, y1, true),
        _ => -k_cos(y0, y1),
    };
    if ix < HUGE { y } else { within_one(y) }
}

/// The cosine of `x`, in radians; accurate over the same range as `sin`.
#[must_use]
#[inline(never)]
pub fn cos(x: f64) -> f64 {
    let ix = (x.to_bits() >> 32) as u32 & 0x7fff_ffff;
    if ix <= 0x3fe9_21fb {
        if ix < 0x3e46_a09e {
            // |x| < 2^-27 sqrt(2): cos(x) is 1 to the last bit.
            return 1.0;
        }
        return k_cos(x, 0.0);
    }
    if ix >= 0x7ff0_0000 {
        return f64::NAN;
    }
    let (n, y0, y1) = rem_pio2(x);
    let y = match n & 3 {
        0 => k_cos(y0, y1),
        1 => -k_sin(y0, y1, true),
        2 => -k_cos(y0, y1),
        _ => k_sin(y0, y1, true),
    };
    if ix < HUGE { y } else { within_one(y) }
}

/// Past 2^20 pi/2: some number in [-1, 1], whatever the reduction left
/// (NaN, where it left nothing, is 0).
fn within_one(y: f64) -> f64 {
    if y.is_nan() { 0.0 } else { y.clamp(-1.0, 1.0) }
}

/// The high word of 2^20 pi/2: where the three-part pi/2 runs out.
const HUGE: u32 = 0x4139_21fb;

// pi/2 in three parts of 33 bits (so that n times one is exact), and what
// each leaves of pi/2 after it and the parts before.
const PIO2_1: f64 = 1.570_796_326_734_125_614_17e+00;
const PIO2_1T: f64 = 6.077_100_506_506_192_249_32e-11;
const PIO2_2: f64 = 6.077_100_506_303_965_976_60e-11;
const PIO2_2T: f64 = 2.022_266_248_795_950_631_54e-21;
const PIO2_3: f64 = 2.022_266_248_711_166_455_80e-21;
const PIO2_3T: f64 = 8.478_427_660_368_899_569_97e-32;

/// sin(x + y) for |x| <= pi/4, `y` the tail of `x` (when `tail`): a
/// degree-13 odd polynomial whose error is under 2^-58.
fn k_sin(x: f64, y: f64, tail: bool) -> f64 {
    const S1: f64 = -1.666_666_666_666_663_243_48e-01;
    const S2: f64 = 8.333_333_333_322_489_461_24e-03;
    const S3: f64 = -1.984_126_982_985_794_931_34e-04;
    const S4: f64 = 2.755_731_370_707_006_767_89e-06;
    const S5: f64 = -2.505_076_025_340_686_341_95e-08;
    const S6: f64 = 1.589_690_995_211_550_102_21e-10;
    let z = x * x;
    let w = z * z;
    let r = S2 + z * (S3 + z * S4) + z * w * (S5 + z * S6);
    let v = z * x;
    if tail {
        x - ((z * (0.5 * y - v * r) - y) - v * S1)
    } else {
        x + v * (S1 + z * r)
    }
}

/// cos(x + y) for |x| <= pi/4: a degree-14 even polynomial, error under
/// 2^-58, with `1 - x^2/2` split so that its rounding is not lost.
fn k_cos(x: f64, y: f64) -> f64 {
    const C1: f64 = 4.166_666_666_666_660_190_37e-02;
    const C2: f64 = -1.388_888_888_887_410_957_49e-03;
    const C3: f64 = 2.480_158_728_947_672_941_78e-05;
    const C4: f64 = -2.755_731_435_139_066_330_35e-07;
    const C5: f64 = 2.087_572_321_298_174_827_90e-09;
    const C6: f64 = -1.135_964_755_778_819_482_65e-11;
    let z = x * x;
    let w = z * z;
    let r = z * (C1 + z * (C2 + z * C3)) + w * w * (C4 + z * (C5 + z * C6));
    let hz = 0.5 * z;
    let w = 1.0 - hz;
    w + (((1.0 - w) - hz) + (z * r - x * y))
}

/// `x` less the nearest multiple `n` of pi/2, as `y0 + y1`; returns
/// `(n, y0, y1)`. For |x| > pi/4.
///
/// pi/2 is held in parts of 33 bits, so that `n` times a part is exact,
/// and taken away one part at a time while the remainder is cancelling:
/// good to 151 bits, enough for any |x| < 2^20 pi/2.
fn rem_pio2(x: f64) -> (i32, f64, f64) {
    let sign = x.to_bits() >> 63 != 0;
    let ix = (x.to_bits() >> 32) as u32 & 0x7fff_ffff;

    // The nearest multiple of pi/2 for |x| up to 9 pi/4, worked out with one
    // part unless x is near one (where the remainder cancels).
    let near = |k: f64| {
        let (z, y0) = if sign {
            let z = x + k * PIO2_1;
            (z, z + k * PIO2_1T)
        } else {
            let z = x - k * PIO2_1;
            (z, z - k * PIO2_1T)
        };
        let y1 = if sign {
            (z - y0) + k * PIO2_1T
        } else {
            (z - y0) - k * PIO2_1T
        };
        let n = k as i32;
        (if sign { -n } else { n }, y0, y1)
    };

    if ix <= 0x400f_6a7a {
        // |x| <= 5 pi/4.
        if ix & 0xf_ffff == 0x9_21fb {
            // |x| is about pi/2 or pi.
            return medium(x, ix);
        }
        return if ix <= 0x4002_d97c {
            near(1.0)
        } else {
            near(2.0)
        };
    }
    if ix <= 0x401c_463b {
        // |x| <= 9 pi/4.
        if ix <= 0x4015_fdbc {
            if ix == 0x4012_d97c {
                // About 3 pi/2.
                return medium(x, ix);
            }
            return near(3.0);
        }
        if ix == 0x4019_21fb {
            // About 2 pi.
            return medium(x, ix);
        }
        return near(4.0);
    }
    medium(x, ix)

    // (Infinite and NaN arguments never reach here: `sin` and `cos` return
    // first.)
}

/// The reduction for any other |x|: `n` rounded from x * 2/pi, then up to
/// three rounds of taking away n times a part of pi/2, the next only while
/// the remainder keeps cancelling.
fn medium(x: f64, ix: u32) -> (i32, f64, f64) {
    const INV_PIO2: f64 = core::f64::consts::FRAC_2_PI;
    // 1.5 * 2^52: adding it rounds to an integer (round to nearest, which
    // Rust always uses).
    const TO_INT: f64 = 1.5 / f64::EPSILON;

    let f_n = (x * INV_PIO2 + TO_INT) - TO_INT;
    let n = f_n as i32;
    let mut r = x - f_n * PIO2_1;
    // Good to 85 bits.
    let mut w = f_n * PIO2_1T;
    let mut y0 = r - w;
    let exponent = |y: f64| ((y.to_bits() >> 52) & 0x7ff) as i32;
    let ex = (ix >> 20) as i32;
    if ex - exponent(y0) > 16 {
        // Good to 118 bits.
        let t = r;
        w = f_n * PIO2_2;
        r = t - w;
        w = f_n * PIO2_2T - ((t - r) - w);
        y0 = r - w;
        if ex - exponent(y0) > 49 {
            // Good to 151 bits: every case.
            let t = r;
            w = f_n * PIO2_3;
            r = t - w;
            w = f_n * PIO2_3T - ((t - r) - w);
            y0 = r - w;
        }
    }
    let y1 = (r - y0) - w;
    (n, y0, y1)
}

/// The arcsine of `x`, in radians: NaN outside [-1, 1].
///
/// Under 0.5 ulp from 0.5 up a rational approximation of
/// `(asin(x) - x) / x^3` (error under 2^-58.75); from 0.5 to 1 through
/// asin(x) = pi/2 - 2 asin(sqrt((1 - x) / 2)), with the root split in two
/// so its rounding is carried.
#[must_use]
#[inline(never)]
pub fn asin(x: f64) -> f64 {
    const PIO2_HI: f64 = core::f64::consts::FRAC_PI_2;
    const PIO2_LO: f64 = 6.123_233_995_736_766_035_87e-17;
    let hx = (x.to_bits() >> 32) as u32;
    let ix = hx & 0x7fff_ffff;
    if ix >= 0x3ff0_0000 {
        // |x| >= 1, or NaN.
        let lx = x.to_bits() as u32;
        if (ix - 0x3ff0_0000) | lx == 0 {
            // asin(+-1) = +-pi/2 (the tiny term rounds it as fdlibm does).
            return x * PIO2_HI + f64::from_bits(0x3870_0000_0000_0000);
        }
        return f64::NAN;
    }
    if ix < 0x3fe0_0000 {
        // |x| < 0.5.
        if (0x0010_0000..0x3e50_0000).contains(&ix) {
            // |x| < 2^-26: asin(x) is x to the last bit.
            return x;
        }
        return x + x * asin_r(x * x);
    }
    // 0.5 <= |x| < 1.
    let z = (1.0 - x.abs()) * 0.5;
    let s = sqrt(z);
    let r = asin_r(z);
    let y = if ix >= 0x3fef_3333 {
        // |x| > 0.975.
        PIO2_HI - (2.0 * (s + s * r) - PIO2_LO)
    } else {
        // f + c = sqrt(z), f with its low word cleared.
        let f = f64::from_bits(s.to_bits() & 0xffff_ffff_0000_0000);
        let c = (z - f * f) / (s + f);
        0.5 * PIO2_HI - (2.0 * s * r - (PIO2_LO - 2.0 * c) - (0.5 * PIO2_HI - 2.0 * f))
    };
    if hx >> 31 != 0 { -y } else { y }
}

/// `(asin(x) - x) / x^3` at z = x^2, rationally.
fn asin_r(z: f64) -> f64 {
    const P_S0: f64 = 1.666_666_666_666_666_574_15e-01;
    const P_S1: f64 = -3.255_658_186_224_009_154_05e-01;
    const P_S2: f64 = 2.012_125_321_348_629_258_81e-01;
    const P_S3: f64 = -4.005_553_450_067_941_140_27e-02;
    const P_S4: f64 = 7.915_349_942_898_145_321_76e-04;
    const P_S5: f64 = 3.479_331_075_960_211_675_70e-05;
    const Q_S1: f64 = -2.403_394_911_734_414_218_78e+00;
    const Q_S2: f64 = 2.020_945_760_233_505_694_71e+00;
    const Q_S3: f64 = -6.882_839_716_054_532_930_30e-01;
    const Q_S4: f64 = 7.703_815_055_590_193_527_91e-02;
    let p = z * (P_S0 + z * (P_S1 + z * (P_S2 + z * (P_S3 + z * (P_S4 + z * P_S5)))));
    let q = 1.0 + z * (Q_S1 + z * (Q_S2 + z * (Q_S3 + z * Q_S4)));
    p / q
}
