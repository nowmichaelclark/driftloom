//! The core's own math (queue item 43; `docs/CORE.md`, promise 3).
//!
//! Every platform computes `sin`, `exp` or `powf` its own way, and the last
//! bits differ: the same notes made different samples on a phone, a laptop
//! and in the browser. So the core carries its own functions, written with
//! nothing but IEEE 754's basic operations (`+ - * /`, comparisons,
//! conversions) and bit manipulation. Those are correctly rounded on every
//! machine the core targets, so the functions give the same bits
//! everywhere, and the core needs no `std` (no `libm`) to have them.
//!
//! **Which functions.** The ones the core uses, no more: `sin`, `cos`,
//! `asin`, `exp`, `expm1`, `tanh` and `pow` in 64 bits; `powf`, `exp2f`,
//! `log10f` and `log2f` in 32; and the exact operations (`floor`, `ceil`,
//! `round`, `trunc`, `sqrt`), which `core` does not offer on stable Rust.
//! And for the generator (queue item 77; phase 2 ports it here), `log`,
//! `log10` and `log2` in 64 bits. The generator's JavaScript calls the
//! same functions through their mirror, `js/dlmath.js`, which
//! `test/dlmath.test.mjs` holds to these bit for bit (through `dl_math`).
//!
//! **Where they come from.** FreeBSD's msun (fdlibm), as musl and Rust's
//! `libm` carry it: the very code the WebAssembly build ran before (its
//! `std` has no system library to call, so it uses `libm`). So the app's
//! own engine sounds exactly as it did, and the native builds, which used
//! glibc's functions until now, move to it. Each file keeps its notice.
//!
//! **How accurate, and why that much.** Measured by `tests/math.rs`
//! against a double-double reference, over the ranges the core uses and
//! beyond: `sin`, `cos`, `asin`, `exp`, `expm1` and `pow` at most 0.86 ulp
//! from the true value, `tanh` 2.03 (only the saturator's curve uses it);
//! `powf`, `log10f` and `log2f` at most 0.82 ulp and `exp2f` 0.501. That is
//! more than audio needs in most places, and it is right where errors
//! accumulate: pitches and phases (`pow`, `exp2f`), filter coefficients
//! near their poles (`sin`, `cos`), envelope curves multiplied into
//! themselves every frame (`exp`, `pow`). Where nobody could hear an error
//! -- the compressors' decibel conversions (`powf`, `log10f`), where a few
//! ulps are still 140 dB down -- a cheaper form would do. It was not taken:
//! these take fewer instructions a call than the `libm` calls they replace,
//! and keeping them keeps the app's renders as they were. A faster
//! approximation is a change of sound, to be measured and heard on its own
//! (the charter's phase 8), not part of making the core deterministic.
//!
//! **Out of line.** The functions with a series in them are
//! `#[inline(never)]`: `libm`'s were calls, and inlined into the
//! oscillators' per-frame loops they made V8 compile those loops worse
//! (measured on the golden renders).
//!
//! **No fused multiply-add.** `a * b + c` is two roundings here, never
//! one: Rust does not fuse floating-point operations unless asked
//! (`mul_add`), and nothing in the core asks. The functions were derived
//! for two roundings, and a fused form would give different bits on the
//! machines that have one. Keep it that way: no `mul_add`, and no
//! fast-math tricks that would allow contraction.
//!
//! **Subnormal numbers.** Some machines flush numbers below 2^-126 (in
//! 32 bits) to zero, in hardware or because the host asks (Chromium turns
//! on flush-to-zero on its audio threads); others keep them, slowly. Either
//! way the same code would differ, in its samples or its cost. So the core
//! never makes one. These functions return zero where the true answer is
//! below the smallest normal number (`exp` under about -708, `exp2f` under
//! -126, `pow` and `powf` that small). And the DSP's tails stop at the
//! silence floor (`SILENCE`, -301 dB): a filter's output when its ringing
//! has decayed that far, before it is rounded to 32 bits (`filter.rs`, the
//! one place the golden renders found subnormals being made), and each
//! channel as it enters the mix, so nothing downstream starts from a
//! tail that small. Checked two ways: every golden render, run with the
//! processor's underflow and denormal exceptions unmasked, never raises
//! one; and rendered with the processor flushing subnormals and without,
//! the goldens hash the same (`tools/golden.sh --flush`, in CI).

mod exact;
mod exp;
mod log;
mod pow;
mod trig;

pub use exact::{ceil, floor, round, scalbn, scalbnf, sqrt, sqrtf, trunc};
pub use exp::{exp, exp2f, expm1, tanh};
pub use log::{log, log2, log2f, log10, log10f};
pub use pow::{pow, powf};
pub use trig::{asin, cos, sin};

/// The silence floor, 2^-50 (about 8.9e-16, or -301 dB under full scale):
/// below it a value the core keeps is zero. It is far under anything
/// audible (a 24-bit converter's own floor is -144 dB) and far enough over
/// the subnormal numbers (below 2^-126) that a product of two values at or
/// above it, and any sum of such products, is still a normal number.
pub const SILENCE: f32 = f32::from_bits(0x2680_0000);

/// `x`, or zero if it is under the silence floor (`SILENCE`). Applied where
/// a value is kept from one frame or one stage to the next: a parameter's
/// value, a filter's output, a channel as it enters the mix.
#[must_use]
#[inline]
pub fn silence(x: f32) -> f32 {
    if x.abs() < SILENCE { 0.0 } else { x }
}

/// `x`, or a zero of its sign if it is subnormal: what a processor that
/// flushes subnormals to zero would give, so that one that keeps them
/// gives it too.
#[must_use]
#[inline]
pub fn flush(x: f64) -> f64 {
    if x.abs() < f64::MIN_POSITIVE {
        f64::from_bits(x.to_bits() & (1 << 63))
    } else {
        x
    }
}

/// `flush` for `f32`.
#[must_use]
#[inline]
pub fn flushf(x: f32) -> f32 {
    if x.abs() < f32::MIN_POSITIVE {
        f32::from_bits(x.to_bits() & (1 << 31))
    } else {
        x
    }
}
