//! The core's own math (`dlcore::math`, queue item 43), measured.
//!
//! Each function against a reference far more precise than it: for the
//! 64-bit functions, a double-double reference written here (about 106
//! bits, from series and Newton's method, nothing from the platform); for
//! the 32-bit ones, the 64-bit functions of the platform's own library,
//! which are 29 bits better than what they check. The error is in ulps of
//! the true value, over the ranges the core uses and well past them, and
//! the bounds the doc comments state are asserted. The exact operations
//! must equal the platform's to the bit.
//!
//! `cargo test --test math -- --nocapture` prints the table.

// A reference written out by hand: constants spelled to their last digit
// (clippy knows some of them by name), conversions between widths and
// bits, and the single-letter names of the formulas.
#![allow(
    clippy::approx_constant,
    clippy::excessive_precision,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::many_single_char_names,
    clippy::float_cmp,
    clippy::unreadable_literal
)]

use dlcore::math;

// ---------------------------------------------------------------------------
// Double-double arithmetic: a value is hi + lo, |lo| <= ulp(hi)/2. No fused
// multiply-add: products are split (Veltkamp) and summed exactly (Dekker).

#[derive(Clone, Copy, Debug)]
struct Dd(f64, f64);

fn two_sum(a: f64, b: f64) -> Dd {
    let s = a + b;
    let v = s - a;
    Dd(s, (a - (s - v)) + (b - v))
}

fn split(a: f64) -> (f64, f64) {
    let c = 134_217_729.0 * a;
    let hi = c - (c - a);
    (hi, a - hi)
}

fn two_prod(a: f64, b: f64) -> Dd {
    let p = a * b;
    let ((ah, al), (bh, bl)) = (split(a), split(b));
    Dd(p, ((ah * bh - p) + ah * bl + al * bh) + al * bl)
}

impl Dd {
    fn from(x: f64) -> Dd {
        Dd(x, 0.0)
    }
    fn add(self, o: Dd) -> Dd {
        let s = two_sum(self.0, o.0);
        let t = two_sum(self.1, o.1);
        let s = two_sum(s.0, s.1 + t.0);
        two_sum(s.0, s.1 + t.1)
    }
    fn neg(self) -> Dd {
        Dd(-self.0, -self.1)
    }
    fn sub(self, o: Dd) -> Dd {
        self.add(o.neg())
    }
    fn mul(self, o: Dd) -> Dd {
        let p = two_prod(self.0, o.0);
        two_sum(p.0, p.1 + (self.0 * o.1 + self.1 * o.0))
    }
    fn div(self, o: Dd) -> Dd {
        let q1 = self.0 / o.0;
        let r = self.sub(o.mul(Dd::from(q1)));
        let q2 = r.0 / o.0;
        let r = r.sub(o.mul(Dd::from(q2)));
        let q3 = r.0 / o.0;
        two_sum(q1, q2).add(Dd::from(q3))
    }
    fn scale(self, k: i32) -> Dd {
        // In two steps, so that neither factor leaves the normal range.
        let (a, b) = (k / 2, k - k / 2);
        let f = |e: i32| f64::from_bits(((e + 1023) as u64) << 52);
        Dd(self.0 * f(a) * f(b), self.1 * f(a) * f(b))
    }
    fn value(self) -> f64 {
        self.0 + self.1
    }
}

// ln 2 and pi/2 to well past 106 bits, in three parts each.
const LN2: [f64; 3] = [6.931_471_805_599_453e-1, 2.319_046_813_846_299_6e-17, 5.707_708_438_416_212e-34];
const PIO2: [f64; 3] = [1.570_796_326_794_896_6, 6.123_233_995_736_766e-17, -1.497_384_904_859_169_8e-33];

fn three(c: [f64; 3], k: f64) -> Dd {
    // k times a three-part constant, for |k| < 2^20.
    two_prod(c[0], k).add(two_prod(c[1], k)).add(Dd::from(c[2] * k))
}

/// e^x - 1 for |x| <= 1/2 by its series.
fn series_expm1(x: Dd) -> Dd {
    let mut term = x;
    let mut sum = x;
    for n in 2..40 {
        term = term.mul(x).div(Dd::from(f64::from(n)));
        sum = sum.add(term);
        if term.0.abs() < 1e-40 * sum.0.abs().max(1e-300) {
            break;
        }
    }
    sum
}

fn dd_exp(x: Dd) -> Dd {
    let k = (x.0 / LN2[0]).round();
    // r = x - k ln2, |r| <= ln2/2, then e^r = (e^(r/16))^16.
    let r = x.sub(three(LN2, k)).scale(-4);
    let mut e = series_expm1(r).add(Dd::from(1.0));
    for _ in 0..4 {
        e = e.mul(e);
    }
    e.scale(k as i32)
}

fn dd_expm1(x: f64) -> Dd {
    if x.abs() <= 0.5 { series_expm1(Dd::from(x)) } else { dd_exp(Dd::from(x)).sub(Dd::from(1.0)) }
}

/// ln x by Newton's method on e^y = x, from the platform's guess.
fn dd_ln(x: f64) -> Dd {
    let mut y = Dd::from(x.ln());
    for _ in 0..2 {
        // y + x e^-y - 1.
        let e = dd_exp(y.neg());
        y = y.add(Dd::from(x).mul(e).sub(Dd::from(1.0)));
    }
    y
}

/// sin and cos by their series after taking away the nearest multiple of
/// pi/2.
fn dd_sincos(x: f64) -> (Dd, Dd) {
    let k = (x / PIO2[0]).round();
    let r = Dd::from(x).sub(three(PIO2, k));
    let r2 = r.mul(r);
    let (mut s, mut c) = (r, Dd::from(1.0));
    let (mut ts, mut tc) = (r, Dd::from(1.0));
    for n in 1..30 {
        let n = f64::from(n);
        ts = ts.mul(r2).div(Dd::from(-(2.0 * n) * (2.0 * n + 1.0)));
        tc = tc.mul(r2).div(Dd::from(-(2.0 * n - 1.0) * (2.0 * n)));
        s = s.add(ts);
        c = c.add(tc);
    }
    match (k as i64).rem_euclid(4) {
        0 => (s, c),
        1 => (c, s.neg()),
        2 => (s.neg(), c.neg()),
        _ => (c.neg(), s),
    }
}

/// asin by Newton's method on sin y = x.
fn dd_asin(x: f64) -> Dd {
    let mut y = Dd::from(x.asin());
    for _ in 0..2 {
        let (s, c) = dd_sincos(y.0);
        // sin(y0 + y1) = s + c y1, to the precision needed.
        let s = s.add(c.mul(Dd::from(y.1)));
        y = y.add(Dd::from(x).sub(s).div(c));
    }
    y
}

fn dd_tanh(x: f64) -> Dd {
    let e = dd_expm1(2.0 * x);
    e.div(e.add(Dd::from(2.0)))
}

fn dd_pow(x: f64, y: f64) -> Dd {
    let l = dd_ln(x).mul(Dd::from(y));
    dd_exp(l)
}

// ---------------------------------------------------------------------------

/// The error of `got` in ulps of the reference, at the reference's binade.
fn ulps(got: f64, want: Dd) -> f64 {
    let w = want.value();
    (Dd::from(got).sub(want)).value().abs() / ulp64(w)
}

fn ulp64(x: f64) -> f64 {
    let x = x.abs();
    if x < f64::MIN_POSITIVE {
        return f64::from_bits(1);
    }
    let e = ((x.to_bits() >> 52) & 0x7ff) as i64;
    if e <= 52 {
        return f64::from_bits(1 << (e - 1).max(0));
    }
    f64::from_bits(((e - 52) as u64) << 52)
}

fn ulps32(got: f32, want: f64) -> f64 {
    let x = want.abs() as f32;
    let e = ((x.to_bits() >> 23) & 0xff) as i32 - 127;
    let ulp = if x < f32::MIN_POSITIVE { 2f64.powi(-149) } else { 2f64.powi(e - 23) };
    (f64::from(got) - want).abs() / ulp
}

struct Rng(u64);

impl Rng {
    fn unit(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn within(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.unit()
    }
    /// Spread evenly over the binades of [lo, hi], lo > 0.
    fn binades(&mut self, lo: f64, hi: f64) -> f64 {
        (lo.ln() + (hi.ln() - lo.ln()) * self.unit()).exp()
    }
}

const DRAWS: usize = 20_000;

/// The largest error of `f` over `DRAWS` points from `draw`, printed.
fn worst(name: &str, range: &str, draw: impl Fn(&mut Rng) -> f64, f: impl Fn(f64) -> f64, reference: impl Fn(f64) -> Dd) -> f64 {
    let mut rng = Rng(0x2545_f491_4f6c_dd1d);
    let (mut most, mut at) = (0.0f64, 0.0);
    for _ in 0..DRAWS {
        let x = draw(&mut rng);
        let e = ulps(f(x), reference(x));
        if e > most {
            (most, at) = (e, x);
        }
    }
    eprintln!("{name:<8} {range:<28} max {most:.3} ulp (at {at:e})");
    most
}

fn worst32(name: &str, range: &str, draw: impl Fn(&mut Rng) -> f32, f: impl Fn(f32) -> f32, reference: impl Fn(f64) -> f64) -> f64 {
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    let (mut most, mut at) = (0.0f64, 0.0f32);
    for _ in 0..DRAWS {
        let x = draw(&mut rng);
        let e = ulps32(f(x), reference(f64::from(x)));
        if e > most {
            (most, at) = (e, x);
        }
    }
    eprintln!("{name:<8} {range:<28} max {most:.3} ulp (at {at:e})");
    most
}

#[test]
fn the_reference_is_right() {
    // Against values worked out to 60 digits (Python's `decimal`).
    // 2^-90 is plenty: an ulp measured to a thousandth needs 2^-63.
    let close = |got: Dd, want: f64, tail: f64| (got.sub(Dd(want, tail))).value().abs() < 8e-28 * want.abs();
    assert!(close(dd_exp(Dd::from(1.0)), 2.718_281_828_459_045, 1.445_646_891_729_250_2e-16));
    assert!(close(dd_ln(10.0), 2.302_585_092_994_046, -2.170_756_223_382_249_4e-16));
    let (s, c) = dd_sincos(1.0);
    assert!(close(s, 8.414_709_848_078_965e-1, 1.776_845_092_935_536e-18));
    assert!(close(c, 5.403_023_058_681_398e-1, -4.760_954_612_604_417e-17));
}

/// An exponent in (-30, 30) that depends on x, for `pow`'s widest test.
fn exponent(x: f64) -> f64 {
    ((x * 7919.0).fract() - 0.5) * 60.0
}

#[test]
fn the_64_bit_functions_are_under_their_bounds() {
    let sin = worst("sin", "[-pi, pi]", |r| r.within(-3.2, 3.2), math::sin, |x| dd_sincos(x).0);
    let sin_far = worst("sin", "[-10^4, 10^4]", |r| r.within(-1e4, 1e4), math::sin, |x| dd_sincos(x).0);
    let cos = worst("cos", "[-pi, pi]", |r| r.within(-3.2, 3.2), math::cos, |x| dd_sincos(x).1);
    let cos_far = worst("cos", "[-10^4, 10^4]", |r| r.within(-1e4, 1e4), math::cos, |x| dd_sincos(x).1);
    let asin = worst("asin", "[-1, 1]", |r| r.within(-1.0, 1.0), math::asin, dd_asin);
    let exp = worst("exp", "[-700, 709]", |r| r.within(-700.0, 709.0), math::exp, |x| dd_exp(Dd::from(x)));
    let exp_near = worst("exp", "[-1, 1]", |r| r.within(-1.0, 1.0), math::exp, |x| dd_exp(Dd::from(x)));
    let expm1 = worst("expm1", "[-40, 40]", |r| r.within(-40.0, 40.0), math::expm1, dd_expm1);
    let expm1_near = worst("expm1", "[-0.5, 0.5]", |r| r.within(-0.5, 0.5), math::expm1, dd_expm1);
    let tanh = worst("tanh", "[-20, 20]", |r| r.within(-20.0, 20.0), math::tanh, dd_tanh);
    let tanh_near = worst("tanh", "[-1, 1]", |r| r.within(-1.0, 1.0), math::tanh, dd_tanh);
    let ln = worst("(ln)", "reference check, [1e-3, 1e3]", |r| r.binades(1e-3, 1e3), f64::ln, dd_ln);
    let pow_db = worst("pow", "10^(x/20), x in [-200, 200]", |r| r.within(-200.0, 200.0), |x| math::pow(10.0, x / 20.0), |x| dd_pow(10.0, x / 20.0));
    let pow_pitch = worst("pow", "2^(x/12), x in [-100, 100]", |r| r.within(-100.0, 100.0), |x| math::pow(2.0, x / 12.0), |x| dd_pow(2.0, x / 12.0));
    let pow_any = worst(
        "pow",
        "x^y, x in [1e-3, 1e3], |y| < 30",
        |r| r.binades(1e-3, 1e3),
        |x| math::pow(x, exponent(x)),
        |x| dd_pow(x, exponent(x)),
    );
    // The generator's logarithms (queue item 77), over its ranges (the
    // vibes' exponential draws, decibels, octaves) and far past them.
    let ln10 = dd_ln(10.0);
    let ln2 = Dd(LN2[0], LN2[1]);
    let natural = worst("log", "[1e-300, 1e300]", |r| r.binades(1e-300, 1e300), math::log, dd_ln);
    let natural_near = worst("log", "[0.5, 2]", |r| r.within(0.5, 2.0), math::log, dd_ln);
    let log10 = worst("log10", "[1e-300, 1e300]", |r| r.binades(1e-300, 1e300), math::log10, |x| dd_ln(x).div(ln10));
    let log10_near = worst("log10", "[0.5, 2]", |r| r.within(0.5, 2.0), math::log10, |x| dd_ln(x).div(ln10));
    let log2 = worst("log2", "[1e-300, 1e300]", |r| r.binades(1e-300, 1e300), math::log2, |x| dd_ln(x).div(ln2));
    let log2_near = worst("log2", "[0.5, 2]", |r| r.within(0.5, 2.0), math::log2, |x| dd_ln(x).div(ln2));
    for e in [sin, sin_far, cos, cos_far, asin, exp, exp_near, expm1, expm1_near, pow_db, pow_pitch, pow_any, natural, natural_near, log10, log10_near, log2, log2_near] {
        assert!(e < 1.0, "{e} ulp");
    }
    for e in [tanh, tanh_near] {
        assert!(e < 2.1, "{e} ulp");
    }
    // The platform's own ln, as a check on the reference: glibc's is under
    // 1 ulp too.
    assert!(ln < 1.0, "{ln} ulp");
}

#[test]
fn the_32_bit_functions_are_under_their_bounds() {
    let powf = worst32("powf", "10^(x/20), x in [-120, 120]", |r| r.within(-120.0, 120.0) as f32, |x| math::powf(10.0, 0.05 * x), |x| 10f64.powf(f64::from(0.05 * x as f32)));
    let powf_any = worst32("powf", "x^0.35, x in [1e-6, 1e6]", |r| r.binades(1e-6, 1e6) as f32, |x| math::powf(x, 0.35), |x| x.powf(f64::from(0.35f32)));
    let exp2f = worst32("exp2f", "[-126, 127]", |r| r.within(-126.0, 127.0) as f32, math::exp2f, f64::exp2);
    let exp2f_near = worst32("exp2f", "[-1, 1]", |r| r.within(-1.0, 1.0) as f32, math::exp2f, f64::exp2);
    let log10f = worst32("log10f", "[1e-30, 1e30]", |r| r.binades(1e-30, 1e30) as f32, math::log10f, f64::log10);
    let log10f_near = worst32("log10f", "[0.5, 2]", |r| r.within(0.5, 2.0) as f32, math::log10f, f64::log10);
    let log2f = worst32("log2f", "[1e-30, 1e30]", |r| r.binades(1e-30, 1e30) as f32, math::log2f, f64::log2);
    let log2f_near = worst32("log2f", "[0.5, 2]", |r| r.within(0.5, 2.0) as f32, math::log2f, f64::log2);
    for e in [powf, powf_any, exp2f, exp2f_near, log10f, log10f_near, log2f, log2f_near] {
        assert!(e < 1.0, "{e} ulp");
    }
}

#[test]
fn the_exact_operations_are_exact() {
    let mut rng = Rng(7);
    let check = |x: f64| {
        let same = |a: f64, b: f64| a.to_bits() == b.to_bits() || (a.is_nan() && b.is_nan());
        assert!(same(math::floor(x), x.floor()), "floor {x:e}");
        assert!(same(math::ceil(x), x.ceil()), "ceil {x:e}");
        assert!(same(math::round(x), x.round()), "round {x:e}");
        assert!(same(math::trunc(x), x.trunc()), "trunc {x:e}");
        assert!(same(math::sqrt(x), x.sqrt()), "sqrt {x:e}");
        let f = x as f32;
        assert!(same(f64::from(math::sqrtf(f)), f64::from(f.sqrt())), "sqrtf {f:e}");
    };
    for x in [0.0, -0.0, 0.5, -0.5, 1.5, -1.5, 2.5, -2.5, 4_503_599_627_370_495.5, -4_503_599_627_370_495.5, 1e300, f64::MIN_POSITIVE, f64::from_bits(1), f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
        check(x);
    }
    for _ in 0..200_000 {
        // Every binade, both signs, and plain numbers.
        check(f64::from_bits(rng.0.wrapping_mul(0x9e37_79b9_7f4a_7c15)));
        check(rng.within(-1e4, 1e4));
        rng.unit();
    }
}

#[test]
fn subnormal_results_are_zero() {
    // The policy in `math`, "Subnormal numbers".
    for (got, what) in [
        (math::exp(-708.5), "exp"),
        (math::exp(-745.0), "exp"),
        (math::pow(2.0, -1060.0), "pow"),
        (math::pow(0.5, 1030.5), "pow"),
        (math::scalbn(1.0, -1030), "scalbn"),
        (f64::from(math::exp2f(-126.5)), "exp2f"),
        (f64::from(math::powf(2.0, -130.0)), "powf"),
        (f64::from(math::scalbnf(1.0, -130)), "scalbnf"),
    ] {
        assert_eq!(got, 0.0, "{what}");
    }
    // Just above the normal range, the true value.
    assert_eq!(math::exp(-708.39), (-708.39f64).exp());
    assert_eq!(math::pow(2.0, -1022.0), f64::MIN_POSITIVE);
    assert_eq!(math::exp2f(-126.0), f32::MIN_POSITIVE);
    assert_eq!(math::silence(1e-15), 1e-15);
    assert_eq!(math::silence(-8e-16), 0.0);
    assert_eq!(math::flush(-1e-310).to_bits(), (-0.0f64).to_bits());
}

#[test]
fn the_logarithms_are_exact_where_they_can_be() {
    // 1 to 0, powers of two and ten to their integers, and C's special
    // cases.
    assert_eq!(math::log(1.0).to_bits(), 0);
    assert_eq!(math::log10(1.0).to_bits(), 0);
    assert_eq!(math::log2(1.0).to_bits(), 0);
    for k in -1074..1024 {
        let x = if k >= -1022 { f64::from_bits(((k + 1023) as u64) << 52) } else { f64::from_bits(1 << (k + 1074)) };
        assert_eq!(math::log2(x), f64::from(k), "log2 2^{k}");
    }
    for k in 0..23 {
        assert_eq!(math::log10(10f64.powi(k)), f64::from(k), "log10 10^{k}");
    }
    for x in [0.0, -0.0] {
        assert_eq!(math::log(x), f64::NEG_INFINITY);
        assert_eq!(math::log10(x), f64::NEG_INFINITY);
        assert_eq!(math::log2(x), f64::NEG_INFINITY);
    }
    for x in [-1.0, -1e-300, f64::NEG_INFINITY, f64::NAN] {
        assert!(math::log(x).is_nan() && math::log10(x).is_nan() && math::log2(x).is_nan(), "{x:e}");
    }
    assert_eq!(math::log(f64::INFINITY), f64::INFINITY);
}

#[test]
fn huge_angles_stay_in_range() {
    // Beyond 2^20 pi/2 the answer is not accurate, but it is a sine.
    for x in [1e7, -3e9, 1e15, 1e22, 1.7e308] {
        assert!((-1.0..=1.0).contains(&math::sin(x)), "sin {x:e}");
        assert!((-1.0..=1.0).contains(&math::cos(x)), "cos {x:e}");
    }
}

#[test]
fn no_input_panics() {
    // Debug builds check every integer operation for overflow: any bit
    // pattern, through every function, must come back with a number.
    let mut rng = Rng(11);
    for _ in 0..100_000 {
        rng.unit();
        let (a, b) = (f64::from_bits(rng.0), f64::from_bits(rng.0.rotate_left(29)));
        let (fa, fb) = (f32::from_bits(rng.0 as u32), f32::from_bits((rng.0 >> 32) as u32));
        let k = (rng.0 >> 40) as i32 - (1 << 23);
        let all = [
            math::sin(a), math::cos(a), math::asin(a), math::exp(a), math::expm1(a), math::tanh(a),
            math::pow(a, b), math::sqrt(a), math::floor(a), math::ceil(a), math::round(a), math::trunc(a),
            math::scalbn(a, k), f64::from(math::powf(fa, fb)), f64::from(math::exp2f(fa)),
            f64::from(math::log10f(fa)), f64::from(math::log2f(fa)), f64::from(math::sqrtf(fa)), f64::from(math::scalbnf(fa, k)),
            math::log(a), math::log10(a), math::log2(a),
        ];
        assert_eq!(all.len(), 22);
    }
}
