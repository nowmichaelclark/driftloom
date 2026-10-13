//! The operations IEEE 754 defines exactly: rounding to an integer, the
//! square root, and scaling by a power of two. Each has one right answer
//! for every input, so any correct implementation gives the same bits.
//! They are written out because `core` (without `std`) does not offer them
//! on stable Rust.

// Rounding and roots work on the bits and the integer significand: the
// casts between them are the method.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::unreadable_literal
)]

/// 2^52: from here up every `f64` is an integer.
const INTEGRAL: f64 = 4_503_599_627_370_496.0;

/// `x` with its fraction dropped, toward zero (`f64::trunc`).
#[must_use]
pub fn trunc(x: f64) -> f64 {
    if x.abs() < INTEGRAL {
        // Exact through a 64-bit integer; the sign is put back for the
        // zeros (trunc(-0.5) is -0).
        ((x as i64) as f64).copysign(x)
    } else {
        // Already an integer, or infinite, or NaN.
        x
    }
}

/// The largest integer not above `x` (`f64::floor`).
#[must_use]
pub fn floor(x: f64) -> f64 {
    if x.is_sign_positive() && x < INTEGRAL {
        // The usual case, a place in a table or a buffer: for a positive
        // number truncation is the floor.
        return (x as i64) as f64;
    }
    let t = trunc(x);
    // Below zero, dropping the fraction went up. `t - 1` is exact: `t` is
    // an integer under 2^52.
    if t > x { t - 1.0 } else { t }
}

/// The smallest integer not below `x` (`f64::ceil`).
#[must_use]
pub fn ceil(x: f64) -> f64 {
    let t = trunc(x);
    if t < x { t + 1.0 } else { t }
}

/// The nearest integer to `x`, halves away from zero (`f64::round`).
#[must_use]
pub fn round(x: f64) -> f64 {
    let t = trunc(x);
    // `x - t`, the fraction, is exact.
    if (x - t).abs() >= 0.5 {
        if x < 0.0 { t - 1.0 } else { t + 1.0 }
    } else {
        t
    }
}

/// The square root, correctly rounded (`f64::sqrt`): an estimate from
/// Newton's method, then corrected on the integer significand, where
/// "correctly rounded" can be checked exactly. Identical to the hardware's
/// instruction, which `core` does not offer; a few times its cost.
#[must_use]
pub fn sqrt(x: f64) -> f64 {
    if x.is_nan() || x < 0.0 {
        return f64::NAN;
    }
    if x == 0.0 || x == f64::INFINITY {
        // Zeros keep their sign.
        return x;
    }
    let bits = x.to_bits();
    let biased = ((bits >> 52) & 0x7ff) as i32;
    let (mut m, mut e) = if biased == 0 {
        // Subnormal: normalize the significand.
        let m = bits & 0x000f_ffff_ffff_ffff;
        let shift = m.leading_zeros() as i32 - 11;
        (m << shift, 1 - shift)
    } else {
        (bits & 0x000f_ffff_ffff_ffff | 1 << 52, biased)
    };
    // x = m * 2^e with m in [2^52, 2^53); make e even so it halves.
    e -= 1075;
    if e & 1 != 0 {
        m <<= 1;
        e -= 1;
    }
    // m * 2^54 lies in [2^106, 2^108), so its root has 54 bits: the 53 of
    // the answer and one more to round on.
    let mut root = estimate(m);
    // What is left, m * 2^54 - root^2: within a few units of the true root
    // it is under 2^60 either way, so the wrapping 64-bit difference is it
    // exactly. Then step to the integer root, floor(sqrt(m * 2^54)).
    let mut rest = (m << 54).wrapping_sub(root.wrapping_mul(root)) as i64;
    while rest < 0 {
        rest += (2 * root - 1) as i64;
        root -= 1;
    }
    while rest > (2 * root) as i64 {
        rest -= (2 * root + 1) as i64;
        root += 1;
    }
    let mut q = root >> 1;
    let half = root & 1 == 1;
    // A remainder means past the halfway point; exactly halfway (which a
    // square root never is) would go to even.
    if half && (rest != 0 || q & 1 == 1) {
        q += 1;
    }
    // The answer is q * 2^((e - 54) / 2 + 1), q in [2^52, 2^53].
    let mut exponent = (e - 54) / 2 + 1 + 52 + 0x3ff;
    if q == 1 << 53 {
        q >>= 1;
        exponent += 1;
    }
    f64::from_bits(((exponent as u64) << 52) | (q & 0x000f_ffff_ffff_ffff))
}

/// About sqrt(m * 2^54) for m in [2^52, 2^54), as an integer: Newton's
/// method for 1/sqrt (multiplications only), from the usual first guess
/// in the bits.
fn estimate(m: u64) -> u64 {
    // m as a number in [1, 4): exact, m has at most 54 bits.
    let v = (m as f64) * f64::from_bits(0x3cb0_0000_0000_0000);
    let mut y = f64::from_bits(0x5fe6_eb50_c7b5_37a9 - (v.to_bits() >> 1));
    // The guess is within 3.5%; each step squares the error: 2^-52 by
    // the fourth.
    for _ in 0..4 {
        y *= 1.5 - 0.5 * v * y * y;
    }
    // sqrt(v) * 2^53 = sqrt(m * 2^54); in [2^53, 2^54).
    let r = v * y * f64::from_bits(0x4340_0000_0000_0000);
    (r as u64).clamp(1 << 53, (1 << 54) - 1)
}

/// The square root of an `f32`, correctly rounded. Rounding the 64-bit root
/// to 32 bits is exact rounding here: 53 bits are more than twice 24 plus
/// two, which is what makes the double rounding harmless.
#[must_use]
pub fn sqrtf(x: f32) -> f32 {
    sqrt(f64::from(x)) as f32
}

/// `x * 2^n`, with a result below the smallest normal number flushed to
/// zero (see "Subnormal numbers" in `math`). Scaled in up to three steps so
/// that no step overflows or rounds twice, as musl's `scalbn` does.
#[must_use]
pub fn scalbn(x: f64, mut n: i32) -> f64 {
    let mut y = x;
    if n > 1023 {
        y *= f64::from_bits(0x7fe0_0000_0000_0000);
        n -= 1023;
        if n > 1023 {
            y *= f64::from_bits(0x7fe0_0000_0000_0000);
            n -= 1023;
            n = n.min(1023);
        }
    } else if n < -1022 {
        // 2^-1022 * 2^53: keeps y normal on the way down.
        y *= f64::from_bits(0x0360_0000_0000_0000);
        n += 1022 - 53;
        if n < -1022 {
            y *= f64::from_bits(0x0360_0000_0000_0000);
            n += 1022 - 53;
            n = n.max(-1022);
        }
    }
    super::flush(y * f64::from_bits(((0x3ff + n) as u64) << 52))
}

/// `x * 2^n` for `f32`, flushed the same way.
#[must_use]
pub fn scalbnf(x: f32, mut n: i32) -> f32 {
    let mut y = x;
    if n > 127 {
        y *= f32::from_bits(0x7f00_0000);
        n -= 127;
        if n > 127 {
            y *= f32::from_bits(0x7f00_0000);
            n -= 127;
            n = n.min(127);
        }
    } else if n < -126 {
        // 2^-126 * 2^24.
        y *= f32::from_bits(0x0c80_0000);
        n += 126 - 24;
        if n < -126 {
            y *= f32::from_bits(0x0c80_0000);
            n += 126 - 24;
            n = n.max(-126);
        }
    }
    super::flushf(y * f32::from_bits(((0x7f + n) as u32) << 23))
}
