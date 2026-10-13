//! Exponentials: `exp`, `expm1` and `tanh` in 64 bits, `exp2f` in 32.
//!
//! Ported from FreeBSD's msun (`e_exp.c`, `s_expm1.c`, `s_exp2f.c`) and
//! musl's `tanh.c`, by way of Rust's `libm`, the code the WebAssembly build
//! already ran. The one change: a result below the smallest normal number
//! is zero (see "Subnormal numbers" in `math`).

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
// Copyright (C) 1993, 2004 by Sun Microsystems, Inc. All rights reserved.
//
// Developed at SunPro, a Sun Microsystems, Inc. business.
// Permission to use, copy, modify, and distribute this
// software is freely granted, provided that this notice
// is preserved.
// ====================================================
//
// `exp2f` (`s_exp2f.c`): Copyright (c) 2005 David Schultz
// <das@FreeBSD.ORG>. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
// OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
// SUCH DAMAGE.

use super::exact::scalbn;

/// ln 2 in two parts, the first with its low bits clear so that `k` times
/// it is exact for any `k` an exponential needs.
const LN2_HI: f64 = 6.931_471_803_691_238_164_90e-01;
const LN2_LO: f64 = 1.908_214_929_270_587_700_02e-10;
const INV_LN2: f64 = core::f64::consts::LOG2_E;

/// e^x, under 1 ulp (an error analysis bounds it; `tests/math.rs`
/// measures it).
///
/// x is reduced to `k ln2 + r` with |r| <= ln2/2, e^r comes from a
/// degree-5 rational form whose error is under 2^-59, and 2^k scales it.
/// Below about -708.4 the true answer is subnormal, and this returns 0.
#[must_use]
#[inline(never)]
pub fn exp(x: f64) -> f64 {
    const P1: f64 = 1.666_666_666_666_660_190_37e-01;
    const P2: f64 = -2.777_777_777_701_559_338_42e-03;
    const P3: f64 = 6.613_756_321_437_934_361_17e-05;
    const P4: f64 = -1.653_390_220_546_525_153_90e-06;
    const P5: f64 = 4.138_136_797_057_238_460_39e-08;

    let hx = (x.to_bits() >> 32) as u32;
    let negative = hx >> 31 != 0;
    let hx = hx & 0x7fff_ffff;
    if hx >= 0x4086_232b {
        // |x| >= 708.39...
        if x.is_nan() {
            return x;
        }
        if x > 709.782_712_893_383_973_096 {
            return f64::INFINITY;
        }
        if x < -708.396_418_532_264_106_22 {
            // Subnormal or zero: zero.
            return 0.0;
        }
    }
    let (hi, lo, k) = if hx > 0x3fd6_2e42 {
        // |x| > ln2/2.
        let k = if hx >= 0x3ff0_a2b2 {
            // |x| >= 1.5 ln2.
            (INV_LN2 * x + if negative { -0.5 } else { 0.5 }) as i32
        } else if negative {
            -1
        } else {
            1
        };
        let kf = f64::from(k);
        (x - kf * LN2_HI, kf * LN2_LO, k)
    } else if hx > 0x3e30_0000 {
        // |x| > 2^-28.
        (x, 0.0, 0)
    } else {
        // Tiny: 1 + x is e^x to the last bit.
        return 1.0 + x;
    };
    let r = if k == 0 { hi } else { hi - lo };
    let rr = r * r;
    let c = r - rr * (P1 + rr * (P2 + rr * (P3 + rr * (P4 + rr * P5))));
    let y = 1.0 + (r * c / (2.0 - c) - lo + hi);
    if k == 0 { y } else { scalbn(y, k) }
}

/// e^x - 1, accurate near zero where `exp(x) - 1` would cancel; under
/// 1 ulp. `tanh` is built on it.
#[must_use]
#[inline(never)]
pub fn expm1(x: f64) -> f64 {
    const O_THRESHOLD: f64 = 7.097_827_128_933_839_730_96e+02;
    // Scaled: Q_n here is 2^n times Q_n of R(2z), z = x^2/2.
    const Q1: f64 = -3.333_333_333_333_313_164_28e-02;
    const Q2: f64 = 1.587_301_587_254_814_601_65e-03;
    const Q3: f64 = -7.936_507_578_674_879_424_73e-05;
    const Q4: f64 = 4.008_217_827_329_362_395_52e-06;
    const Q5: f64 = -2.010_992_181_836_243_713_26e-07;

    let mut x = x;
    let hx = ((x.to_bits() >> 32) & 0x7fff_ffff) as u32;
    let negative = x.to_bits() >> 63 != 0;
    if hx >= 0x4043_687a {
        // |x| >= 56 ln2.
        if x.is_nan() {
            return x;
        }
        if negative {
            return -1.0;
        }
        if x > O_THRESHOLD {
            return f64::INFINITY;
        }
    }
    let (k, c) = if hx > 0x3fd6_2e42 {
        // |x| > ln2/2.
        let (hi, lo, k) = if hx < 0x3ff0_a2b2 {
            // |x| < 1.5 ln2.
            if negative {
                (x + LN2_HI, -LN2_LO, -1)
            } else {
                (x - LN2_HI, LN2_LO, 1)
            }
        } else {
            let k = (INV_LN2 * x + if negative { -0.5 } else { 0.5 }) as i32;
            let t = f64::from(k);
            (x - t * LN2_HI, t * LN2_LO, k)
        };
        x = hi - lo;
        (k, (hi - x) - lo)
    } else if hx < 0x3c90_0000 {
        // |x| < 2^-54: e^x - 1 is x to the last bit.
        return x;
    } else {
        (0, 0.0)
    };
    let hfx = 0.5 * x;
    let hxs = x * hfx;
    let r1 = 1.0 + hxs * (Q1 + hxs * (Q2 + hxs * (Q3 + hxs * (Q4 + hxs * Q5))));
    let t = 3.0 - r1 * hfx;
    let mut e = hxs * ((r1 - t) / (6.0 - x * t));
    if k == 0 {
        return x - (x * e - hxs);
    }
    e = x * (e - c) - c;
    e -= hxs;
    // e^x is about 2^k (x - e + 1).
    if k == -1 {
        return 0.5 * (x - e) - 0.5;
    }
    if k == 1 {
        if x < -0.25 {
            return -2.0 * (e - (x + 0.5));
        }
        return 1.0 + 2.0 * (x - e);
    }
    let two_k = f64::from_bits(((0x3ff + k) as u64) << 52);
    if !(0..=56).contains(&k) {
        let y = x - e + 1.0;
        let y = if k == 1024 {
            y * 2.0 * f64::from_bits(0x7fe0_0000_0000_0000)
        } else {
            y * two_k
        };
        return y - 1.0;
    }
    let two_minus_k = f64::from_bits(((0x3ff - k) as u64) << 52);
    if k < 20 {
        (x - e + (1.0 - two_minus_k)) * two_k
    } else {
        (x - (e + two_minus_k) + 1.0) * two_k
    }
}

/// The hyperbolic tangent, from `expm1`: about 2 ulp at worst, between
/// 0.1 and 0.26 (musl's own note; `tests/math.rs` measures 2.03). Only the
/// saturator's curve uses it, worked out when the tone is set.
#[must_use]
#[inline(never)]
pub fn tanh(x: f64) -> f64 {
    let negative = x.to_bits() >> 63 != 0;
    let x = x.abs();
    let w = (x.to_bits() >> 32) as u32;
    let t = if w > 0x3fe1_93ea {
        // |x| > ln(3)/2, about 0.5493, or NaN.
        if w > 0x4034_0000 {
            // |x| > 20, or NaN: 1 to the last bit (and NaN stays NaN).
            1.0 - 0.0 / x
        } else {
            let t = expm1(2.0 * x);
            1.0 - 2.0 / (t + 2.0)
        }
    } else if w > 0x3fd0_58ae {
        // |x| > ln(5/3)/2, about 0.2554.
        let t = expm1(2.0 * x);
        t / (t + 2.0)
    } else if w >= 0x0010_0000 {
        let t = expm1(-2.0 * x);
        -t / (t + 2.0)
    } else {
        x
    };
    if negative { -t } else { t }
}

/// 2^x in 32 bits, under 0.501 ulp (Schultz's bound): a 16-entry table of
/// 2^(i/16) and a degree-4 polynomial for the rest, both in 64 bits, which
/// makes their rounding negligible. Below -126 the true answer is
/// subnormal, and this returns 0.
#[must_use]
#[inline(never)]
pub fn exp2f(x: f32) -> f32 {
    const TABLE: [u64; 16] = [
        0x3fe6_a09e_667f_3bcd,
        0x3fe7_a114_73eb_0187,
        0x3fe8_ace5_422a_a0db,
        0x3fe9_c491_82a3_f090,
        0x3fea_e89f_995a_d3ad,
        0x3fec_199b_dd85_529c,
        0x3fed_5818_dcfb_a487,
        0x3fee_a4af_a2a4_90da,
        0x3ff0_0000_0000_0000,
        0x3ff0_b558_6cf9_890f,
        0x3ff1_72b8_3c7d_517b,
        0x3ff2_387a_6e75_6238,
        0x3ff3_06fe_0a31_b715,
        0x3ff3_dea6_4c12_3422,
        0x3ff4_bfda_d536_2a27,
        0x3ff5_ab07_dd48_5429,
    ];
    let redux = f32::from_bits(0x4b40_0000) / 16.0;
    let p1 = f32::from_bits(0x3f31_7218);
    let p2 = f32::from_bits(0x3e75_fdf0);
    let p3 = f32::from_bits(0x3d63_59a4);
    let p4 = f32::from_bits(0x3c1d_964e);

    let ui = x.to_bits();
    let ix = ui & 0x7fff_ffff;
    if ix > 0x42fc_0000 {
        // |x| > 126.
        if ix > 0x7f80_0000 {
            // NaN.
            return x;
        }
        if (0x4300_0000..0x8000_0000).contains(&ui) {
            // x >= 128.
            return f32::INFINITY;
        }
        if ui >= 0x8000_0000 {
            // x < -126: subnormal or zero.
            return 0.0;
        }
    } else if ix <= 0x3300_0000 {
        // |x| <= 2^-25.
        return 1.0 + x;
    }
    // x = k + i/16 + z, |z| <= 1/32.
    let ui = (x + redux).to_bits();
    let i0 = ui.wrapping_add(8);
    let k = i0 / 16;
    let two_k = f64::from_bits(u64::from(0x3ff_u32.wrapping_add(k)) << 52);
    let z = f64::from(x - (f32::from_bits(ui) - redux));
    let r = f64::from_bits(TABLE[(i0 & 15) as usize]);
    let t = r * z;
    let r = r
        + t * (f64::from(p1) + z * f64::from(p2))
        + t * (z * z) * (f64::from(p3) + z * f64::from(p4));
    (r * two_k) as f32
}
