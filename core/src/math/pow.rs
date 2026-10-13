//! Powers: `pow` in 64 bits, `powf` in 32.
//!
//! Ported from FreeBSD's msun (`e_pow.c`; `e_powf.c`, its conversion to
//! float by Ian Lance Taylor, Cygnus Support) by way of Rust's `libm`, the
//! code the WebAssembly build already ran. The one change: a result below
//! the smallest normal number is zero (see "Subnormal numbers" in `math`).
//!
//! The method, for both: x = 2^n (1 + f); work out log2(x) in two pieces,
//! the first with its low bits clear, multiply by y as a sum of two such
//! products, split that into an integer and |y'| <= 1/2, and return
//! 2^integer * 2^y'. Each piece carries the rounding of the last, which is
//! what keeps the result nearly correctly rounded: pow of integers gives
//! the exact integer whenever it is representable.

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

use super::exact::{scalbn, scalbnf, sqrt, sqrtf};

/// `x` with its low 32 bits cleared.
fn high(x: f64) -> f64 {
    f64::from_bits(x.to_bits() & 0xffff_ffff_0000_0000)
}

/// `x` with its high 32 bits replaced by `hi`.
fn with_high(x: f64, hi: u32) -> f64 {
    f64::from_bits((x.to_bits() & 0xffff_ffff) | (u64::from(hi) << 32))
}

/// `x` with its low 12 bits cleared.
fn highf(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & 0xffff_f000)
}

/// Whether `y`, as a power of a negative number, is an odd integer (1),
/// an even one (2), or not an integer (0). `iy` is |y|'s high word, `ly`
/// its low word.
fn integer_kind(iy: i32, ly: u32) -> i32 {
    if iy >= 0x4340_0000 {
        // |y| >= 2^53: even.
        return 2;
    }
    if iy < 0x3ff0_0000 {
        return 0;
    }
    let k = (iy >> 20) - 0x3ff;
    if k > 20 {
        let j = ly >> (52 - k);
        if j << (52 - k) == ly {
            return 2 - (j & 1) as i32;
        }
    } else if ly == 0 {
        let j = iy >> (20 - k);
        if j << (20 - k) == iy {
            return 2 - (j & 1);
        }
    }
    0
}

/// `x` to the power `y`, under 1 ulp (nearly correctly rounded), with
/// C's special cases: anything to 0 is 1, 1 to anything is 1, and so on.
#[must_use]
#[inline(never)]
pub fn pow(x: f64, y: f64) -> f64 {
    const DP_H1: f64 = 5.849_624_872_207_641_601_56e-01;
    const DP_L1: f64 = 1.350_039_202_129_748_971_28e-08;
    const TWO53: f64 = 9_007_199_254_740_992.0;
    // Polynomial for (3/2) (log(x) - 2s - (2/3) s^3).
    const L1: f64 = 5.999_999_999_999_946_487_25e-01;
    const L2: f64 = 4.285_714_285_785_501_842_52e-01;
    const L3: f64 = 3.333_333_298_183_774_329_18e-01;
    const L4: f64 = 2.727_281_238_085_340_064_89e-01;
    const L5: f64 = 2.306_607_457_755_617_540_67e-01;
    const L6: f64 = 2.069_750_178_003_384_177_84e-01;
    const P1: f64 = 1.666_666_666_666_660_190_37e-01;
    const P2: f64 = -2.777_777_777_701_559_338_42e-03;
    const P3: f64 = 6.613_756_321_437_934_361_17e-05;
    const P4: f64 = -1.653_390_220_546_525_153_90e-06;
    const P5: f64 = 4.138_136_797_057_238_460_39e-08;
    const LG2: f64 = core::f64::consts::LN_2;
    const LG2_H: f64 = 6.931_471_824_645_996_093_75e-01;
    const LG2_L: f64 = -1.904_654_299_957_768_045_25e-09;
    // -(1024 - log2(overflow + 0.5 ulp)).
    const OVT: f64 = 8.008_566_259_537_294_437_2e-17;
    // 2/(3 ln2), whole and in two parts.
    const CP: f64 = 9.617_966_939_259_755_543_29e-01;
    const CP_H: f64 = 9.617_967_009_544_372_558_59e-01;
    const CP_L: f64 = -7.028_461_650_952_758_265_16e-09;
    // 1/ln2, whole and in two parts.
    const IVLN2: f64 = core::f64::consts::LOG2_E;
    const IVLN2_H: f64 = 1.442_695_021_629_333_496_09e+00;
    const IVLN2_L: f64 = 1.925_962_991_126_617_468_87e-08;

    let (hx, lx) = ((x.to_bits() >> 32) as i32, x.to_bits() as u32);
    let (hy, ly) = ((y.to_bits() >> 32) as i32, y.to_bits() as u32);
    let mut ix = hx & 0x7fff_ffff;
    let iy = hy & 0x7fff_ffff;

    // x^0 = 1, even for NaN; 1^y = 1, even for NaN.
    if (iy as u32) | ly == 0 || (hx == 0x3ff0_0000 && lx == 0) {
        return 1.0;
    }
    if ix > 0x7ff0_0000
        || (ix == 0x7ff0_0000 && lx != 0)
        || iy > 0x7ff0_0000
        || (iy == 0x7ff0_0000 && ly != 0)
    {
        return f64::NAN;
    }
    let yisint = if hx < 0 { integer_kind(iy, ly) } else { 0 };

    if ly == 0 {
        if iy == 0x7ff0_0000 {
            // y is +-infinity.
            return if (ix - 0x3ff0_0000) | lx as i32 == 0 {
                1.0
            } else if ix >= 0x3ff0_0000 {
                if hy >= 0 { y } else { 0.0 }
            } else if hy >= 0 {
                0.0
            } else {
                -y
            };
        }
        if iy == 0x3ff0_0000 {
            return if hy >= 0 { x } else { 1.0 / x };
        }
        if hy == 0x4000_0000 {
            return x * x;
        }
        if hy == 0x3fe0_0000 && hx >= 0 {
            return sqrt(x);
        }
    }

    let mut ax = x.abs();
    if lx == 0 && (ix == 0x7ff0_0000 || ix == 0 || ix == 0x3ff0_0000) {
        // x is +-0, +-infinity or +-1.
        let mut z = if hy < 0 { 1.0 / ax } else { ax };
        if hx < 0 {
            if (ix - 0x3ff0_0000) | yisint == 0 {
                // (-1)^non-integer.
                z = f64::NAN;
            } else if yisint == 1 {
                z = -z;
            }
        }
        return z;
    }

    let mut s = 1.0;
    if hx < 0 {
        if yisint == 0 {
            // A negative number to a non-integer power.
            return f64::NAN;
        }
        if yisint == 1 {
            s = -1.0;
        }
    }

    let (t1, t2);
    if iy > 0x41e0_0000 {
        // |y| > 2^31: over- or underflows unless x is very near 1.
        if iy > 0x43f0_0000 {
            // |y| > 2^64.
            if ix <= 0x3fef_ffff {
                return if hy < 0 { f64::INFINITY } else { 0.0 };
            }
            if ix >= 0x3ff0_0000 {
                return if hy > 0 { f64::INFINITY } else { 0.0 };
            }
        }
        if ix < 0x3fef_ffff {
            return if hy < 0 { s * f64::INFINITY } else { s * 0.0 };
        }
        if ix > 0x3ff0_0000 {
            return if hy > 0 { s * f64::INFINITY } else { s * 0.0 };
        }
        // |1 - x| <= 2^-20: log(x) is x - x^2/2 + x^3/3 - x^4/4.
        let t = ax - 1.0;
        let w = (t * t) * (0.5 - t * (0.333_333_333_333_333_333_333_3 - t * 0.25));
        let u = IVLN2_H * t;
        let v = t * IVLN2_L - w * IVLN2;
        t1 = high(u + v);
        t2 = v - (t1 - u);
    } else {
        let mut n: i32 = 0;
        if ix < 0x0010_0000 {
            // Subnormal x (which the core never makes).
            ax *= TWO53;
            n -= 53;
            ix = (ax.to_bits() >> 32) as i32;
        }
        n += (ix >> 20) - 0x3ff;
        let j = ix & 0x000f_ffff;
        // Which interval: |x| < sqrt(3/2), < sqrt(3), or up a binade.
        ix = j | 0x3ff0_0000;
        let k = if j <= 0x3_988e {
            0
        } else if j < 0xb_b67a {
            1
        } else {
            n += 1;
            ix -= 0x0010_0000;
            0
        };
        let (bp, dp_h, dp_l) = if k == 0 {
            (1.0, 0.0, 0.0)
        } else {
            (1.5, DP_H1, DP_L1)
        };
        ax = with_high(ax, ix as u32);

        // ss = s_h + s_l = (x - 1)/(x + 1) or (x - 1.5)/(x + 1.5).
        let u = ax - bp;
        let v = 1.0 / (ax + bp);
        let ss = u * v;
        let s_h = high(ss);
        // t_h = ax + bp, high part.
        let t_h = with_high(
            0.0,
            ((ix as u32 >> 1) | 0x2000_0000) + 0x0008_0000 + ((k as u32) << 18),
        );
        let t_l = ax - (t_h - bp);
        let s_l = v * ((u - s_h * t_h) - s_h * t_l);
        // log(ax).
        let s2 = ss * ss;
        let mut r = s2 * s2 * (L1 + s2 * (L2 + s2 * (L3 + s2 * (L4 + s2 * (L5 + s2 * L6)))));
        r += s_l * (s_h + ss);
        let s2 = s_h * s_h;
        let t_h = high(3.0 + s2 + r);
        let t_l = r - ((t_h - 3.0) - s2);
        // u + v = ss (1 + ...).
        let u = s_h * t_h;
        let v = s_l * t_h + t_l * ss;
        // 2/(3 log2) (ss + ...).
        let p_h = high(u + v);
        let p_l = v - (p_h - u);
        let z_h = CP_H * p_h;
        let z_l = CP_L * p_h + p_l * CP + dp_l;
        // log2(ax) = n + dp_h + z_h + z_l.
        let t = f64::from(n);
        t1 = high(((z_h + z_l) + dp_h) + t);
        t2 = z_l - (((t1 - t) - dp_h) - z_h);
    }

    // (y1 + y2) (t1 + t2), y1 the high part of y.
    let y1 = high(y);
    let p_l = (y - y1) * t1 + y * t2;
    let mut p_h = y1 * t1;
    let z = p_l + p_h;
    let j = (z.to_bits() >> 32) as i32;
    let i = z.to_bits() as i32;
    if j >= 0x4090_0000 {
        // z >= 1024.
        if (j - 0x4090_0000) | i != 0 || p_l + OVT > z - p_h {
            return s * f64::INFINITY;
        }
    } else if (j & 0x7fff_ffff) >= 0x4090_cc00 {
        // z <= -1075.
        if ((j as u32).wrapping_sub(0xc090_cc00)) | (i as u32) != 0 || p_l <= z - p_h {
            return s * 0.0;
        }
    }

    // 2^(p_h + p_l).
    let i = j & 0x7fff_ffff;
    let mut k = (i >> 20) - 0x3ff;
    let mut n: i32 = 0;
    if i > 0x3fe0_0000 {
        // |z| > 0.5: n = [z + 0.5].
        n = j + (0x0010_0000 >> (k + 1));
        k = ((n & 0x7fff_ffff) >> 20) - 0x3ff;
        let t = with_high(0.0, (n & !(0x000f_ffff >> k)) as u32);
        n = ((n & 0x000f_ffff) | 0x0010_0000) >> (20 - k);
        if j < 0 {
            n = -n;
        }
        p_h -= t;
    }
    let t = high(p_l + p_h);
    let u = t * LG2_H;
    let v = (p_l - (t - p_h)) * LG2 + t * LG2_L;
    let mut z = u + v;
    let w = v - (z - u);
    let t = z * z;
    let t1 = z - t * (P1 + t * (P2 + t * (P3 + t * (P4 + t * P5))));
    let r = (z * t1) / (t1 - 2.0) - (w + z * w);
    z = 1.0 - (r - z);
    let j = ((z.to_bits() >> 32) as i32) + (n << 20);
    if j >> 20 <= 0 {
        // Below the normal range: `scalbn` makes it zero.
        z = scalbn(z, n);
    } else {
        z = with_high(z, j as u32);
    }
    s * z
}

/// `x` to the power `y` in 32 bits: the same method as `pow`, under 1 ulp.
#[must_use]
#[inline(never)]
pub fn powf(x: f32, y: f32) -> f32 {
    const DP_H1: f32 = 5.849_609_38e-01;
    const DP_L1: f32 = 1.563_220_85e-06;
    const TWO24: f32 = 16_777_216.0;
    const L1: f32 = 6.000_000_238_4e-01;
    const L2: f32 = 4.285_714_328_3e-01;
    const L3: f32 = 3.333_333_432_7e-01;
    const L4: f32 = 2.727_281_153_2e-01;
    const L5: f32 = 2.306_607_514_6e-01;
    const L6: f32 = 2.069_750_130_2e-01;
    const P1: f32 = 1.666_666_716_3e-01;
    const P2: f32 = -2.777_777_845_0e-03;
    const P3: f32 = 6.613_755_977_0e-05;
    const P4: f32 = -1.653_390_199_9e-06;
    const P5: f32 = 4.138_136_944_2e-08;
    const LG2: f32 = 6.931_471_824_6e-01;
    const LG2_H: f32 = 6.931_457_52e-01;
    const LG2_L: f32 = 1.428_606_54e-06;
    // -(128 - log2(overflow + 0.5 ulp)).
    const OVT: f32 = 4.299_566_569_4e-08;
    const CP: f32 = 9.617_967_009_5e-01;
    const CP_H: f32 = 9.619_140_625_0e-01;
    const CP_L: f32 = -1.173_685_740_2e-04;
    const IVLN2: f32 = 1.442_695_021_6e+00;
    const IVLN2_H: f32 = 1.442_687_988_3e+00;
    const IVLN2_L: f32 = 7.052_607_543_3e-06;

    let hx = x.to_bits() as i32;
    let hy = y.to_bits() as i32;
    let mut ix = hx & 0x7fff_ffff;
    let iy = hy & 0x7fff_ffff;

    if iy == 0 || hx == 0x3f80_0000 {
        return 1.0;
    }
    if ix > 0x7f80_0000 || iy > 0x7f80_0000 {
        return f32::NAN;
    }
    let mut yisint = 0;
    if hx < 0 {
        if iy >= 0x4b80_0000 {
            yisint = 2;
        } else if iy >= 0x3f80_0000 {
            let k = (iy >> 23) - 0x7f;
            let j = iy >> (23 - k);
            if j << (23 - k) == iy {
                yisint = 2 - (j & 1);
            }
        }
    }
    if iy == 0x7f80_0000 {
        // y is +-infinity.
        return match ix.cmp(&0x3f80_0000) {
            core::cmp::Ordering::Equal => 1.0,
            core::cmp::Ordering::Greater => {
                if hy >= 0 {
                    y
                } else {
                    0.0
                }
            }
            core::cmp::Ordering::Less => {
                if hy >= 0 {
                    0.0
                } else {
                    -y
                }
            }
        };
    }
    if iy == 0x3f80_0000 {
        return if hy >= 0 { x } else { 1.0 / x };
    }
    if hy == 0x4000_0000 {
        return x * x;
    }
    if hy == 0x3f00_0000 && hx >= 0 {
        return sqrtf(x);
    }

    let mut ax = x.abs();
    if ix == 0x7f80_0000 || ix == 0 || ix == 0x3f80_0000 {
        let mut z = if hy < 0 { 1.0 / ax } else { ax };
        if hx < 0 {
            if (ix - 0x3f80_0000) | yisint == 0 {
                z = f32::NAN;
            } else if yisint == 1 {
                z = -z;
            }
        }
        return z;
    }

    let mut sn = 1.0;
    if hx < 0 {
        if yisint == 0 {
            return f32::NAN;
        }
        if yisint == 1 {
            sn = -1.0;
        }
    }

    let (t1, t2);
    if iy > 0x4d00_0000 {
        // |y| > 2^27.
        if ix < 0x3f7f_fff8 {
            return if hy < 0 { sn * f32::INFINITY } else { sn * 0.0 };
        }
        if ix > 0x3f80_0007 {
            return if hy > 0 { sn * f32::INFINITY } else { sn * 0.0 };
        }
        let t = ax - 1.0;
        let w = (t * t) * (0.5 - t * (0.333_333_333_333 - t * 0.25));
        let u = IVLN2_H * t;
        let v = t * IVLN2_L - w * IVLN2;
        t1 = highf(u + v);
        t2 = v - (t1 - u);
    } else {
        let mut n: i32 = 0;
        if ix < 0x0080_0000 {
            ax *= TWO24;
            n -= 24;
            ix = ax.to_bits() as i32;
        }
        n += (ix >> 23) - 0x7f;
        let j = ix & 0x007f_ffff;
        ix = j | 0x3f80_0000;
        let k = if j <= 0x1c_c471 {
            0
        } else if j < 0x5d_b3d7 {
            1
        } else {
            n += 1;
            ix -= 0x0080_0000;
            0
        };
        let (bp, dp_h, dp_l) = if k == 0 {
            (1.0, 0.0, 0.0)
        } else {
            (1.5, DP_H1, DP_L1)
        };
        ax = f32::from_bits(ix as u32);

        let u = ax - bp;
        let v = 1.0 / (ax + bp);
        let s = u * v;
        let s_h = highf(s);
        let t_h = f32::from_bits(
            ((((ix as u32 >> 1) & 0xffff_f000) | 0x2000_0000) + 0x0040_0000) + ((k as u32) << 21),
        );
        let t_l = ax - (t_h - bp);
        let s_l = v * ((u - s_h * t_h) - s_h * t_l);
        let s2 = s * s;
        let mut r = s2 * s2 * (L1 + s2 * (L2 + s2 * (L3 + s2 * (L4 + s2 * (L5 + s2 * L6)))));
        r += s_l * (s_h + s);
        let s2 = s_h * s_h;
        let t_h = highf(3.0 + s2 + r);
        let t_l = r - ((t_h - 3.0) - s2);
        let u = s_h * t_h;
        let v = s_l * t_h + t_l * s;
        let p_h = highf(u + v);
        let p_l = v - (p_h - u);
        let z_h = CP_H * p_h;
        let z_l = CP_L * p_h + p_l * CP + dp_l;
        let t = n as f32;
        t1 = highf(((z_h + z_l) + dp_h) + t);
        t2 = z_l - (((t1 - t) - dp_h) - z_h);
    }

    let y1 = highf(y);
    let p_l = (y - y1) * t1 + y * t2;
    let mut p_h = y1 * t1;
    let z = p_l + p_h;
    let j = z.to_bits() as i32;
    if j > 0x4300_0000 {
        // z > 128.
        return sn * f32::INFINITY;
    } else if j == 0x4300_0000 {
        if p_l + OVT > z - p_h {
            return sn * f32::INFINITY;
        }
    } else if (j & 0x7fff_ffff) > 0x4316_0000 {
        // z < -150.
        return sn * 0.0;
    } else if j as u32 == 0xc316_0000 && p_l <= z - p_h {
        return sn * 0.0;
    }

    let i = j & 0x7fff_ffff;
    let mut k = (i >> 23) - 0x7f;
    let mut n: i32 = 0;
    if i > 0x3f00_0000 {
        n = j + (0x0080_0000 >> (k + 1));
        k = ((n & 0x7fff_ffff) >> 23) - 0x7f;
        let t = f32::from_bits(n as u32 & !(0x007f_ffff >> k));
        n = ((n & 0x007f_ffff) | 0x0080_0000) >> (23 - k);
        if j < 0 {
            n = -n;
        }
        p_h -= t;
    }
    let t = f32::from_bits((p_l + p_h).to_bits() & 0xffff_8000);
    let u = t * LG2_H;
    let v = (p_l - (t - p_h)) * LG2 + t * LG2_L;
    let mut z = u + v;
    let w = v - (z - u);
    let t = z * z;
    let t1 = z - t * (P1 + t * (P2 + t * (P3 + t * (P4 + t * P5))));
    let r = (z * t1) / (t1 - 2.0) - (w + z * w);
    z = 1.0 - (r - z);
    let j = (z.to_bits() as i32) + (n << 23);
    if j >> 23 <= 0 {
        z = scalbnf(z, n);
    } else {
        z = f32::from_bits(j as u32);
    }
    sn * z
}
