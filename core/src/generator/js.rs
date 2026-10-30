//! JavaScript's number operations, to the bit, where Rust's differ.
//!
//! The generator is ported note for note from `js/generator.js` (queue item
//! 79), and every number in it is a JavaScript double: `+ - * /` and the
//! comparisons are IEEE 754's on both sides, so they need nothing. These
//! are the few that are not the same function in Rust: `Math.round` rounds
//! halves up (Rust's `round` rounds them away from zero) and keeps the
//! sign of a zero, `Math.max` and `Math.min` order the zeros (-0 below +0)
//! and spread a NaN, and `>>> 0` and `^` work on 32-bit integers.

use crate::math;

/// `Math.round`: the nearest integer, halves toward +infinity, and -0 for
/// anything from -0.5 up to -0.
#[must_use]
pub fn round(x: f64) -> f64 {
    let fl = math::floor(x);
    let r = if x - fl >= 0.5 { fl + 1.0 } else { fl };
    if r == 0.0 && x.is_sign_negative() { -0.0 } else { r }
}

/// `Math.floor`.
#[must_use]
pub fn floor(x: f64) -> f64 {
    math::floor(x)
}

/// `Math.ceil`.
#[must_use]
pub fn ceil(x: f64) -> f64 {
    let c = math::ceil(x);
    // ceil(-0.5) is -0 in JavaScript.
    if c == 0.0 && x.is_sign_negative() { -0.0 } else { c }
}

/// `Math.max(a, b)`.
#[must_use]
pub fn max(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else if a > b {
        a
    } else if b > a {
        b
    } else if a == 0.0 {
        // Equal zeros: +0 unless both are -0.
        if a.is_sign_negative() && b.is_sign_negative() { -0.0 } else { 0.0 }
    } else {
        a
    }
}

/// `Math.min(a, b)`.
#[must_use]
pub fn min(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else if a < b {
        a
    } else if b < a {
        b
    } else if a == 0.0 {
        if a.is_sign_negative() || b.is_sign_negative() { -0.0 } else { 0.0 }
    } else {
        a
    }
}

/// `Math.max(lo, Math.min(hi, x))`: the clamp the generator writes.
#[must_use]
pub fn clamp(x: f64, lo: f64, hi: f64) -> f64 {
    max(lo, min(hi, x))
}

/// `Math.abs`.
#[must_use]
pub fn abs(x: f64) -> f64 {
    x.abs()
}

/// The generator's salted streams: `((seed ^ salt) >>> 0) || fallback`.
#[must_use]
pub fn salted(seed: u32, salt: u32, fallback: u32) -> u32 {
    let s = seed ^ salt;
    if s == 0 { fallback } else { s }
}

/// A double as a list index, for a value the generator knows is a
/// non-negative integer below the list's length (a JavaScript index).
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // checked by the caller's bounds
pub fn index(x: f64) -> usize {
    if x >= 0.0 { x as usize } else { usize::MAX }
}

/// A small count as a double.
#[must_use]
#[allow(clippy::cast_precision_loss)] // counts far below 2^53
pub fn num(n: usize) -> f64 {
    n as f64
}
