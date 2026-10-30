//! The generator's random stream: `js/rng.js`'s `Rng`, to the bit.
//!
//! Mulberry32, a 32-bit integer hash run as a counter, and each draw its
//! output over 2^32. Every method draws exactly as the JavaScript does, in
//! the same order, so a seed makes the same loop on both sides. (The
//! synth's per-note draws are another thing, `crate::rng`: looked up by
//! name, never taken in turn.)

use super::js;

/// A seeded stream (`new Rng(seed)`).
#[derive(Clone, Copy, Debug)]
pub struct Rng {
    a: u32,
}

impl Rng {
    /// `new Rng(seed)`: a seed of 0 is 1, as `(seed >>> 0) || 1` says.
    #[must_use]
    pub const fn new(seed: u32) -> Self {
        Rng { a: if seed == 0 { 1 } else { seed } }
    }

    /// The next draw, in [0, 1) (`f`, `next`). (0x3d is mulberry32's 61.)
    pub fn f(&mut self) -> f64 {
        self.a = self.a.wrapping_add(0x6d2b_79f5);
        let a = self.a;
        let mut t = (a ^ (a >> 15)).wrapping_mul(1 | a);
        t = t.wrapping_add((t ^ (t >> 7)).wrapping_mul(0x3d | t)) ^ t;
        f64::from(t ^ (t >> 14)) / 4_294_967_296.0
    }

    /// `range(a, b)`.
    pub fn range(&mut self, a: f64, b: f64) -> f64 {
        a + self.f() * (b - a)
    }

    /// `int(a, b)`, inclusive on both ends.
    pub fn int(&mut self, a: f64, b: f64) -> f64 {
        js::floor(a + self.f() * (b - a + 1.0))
    }

    /// `pick`'s index into a list of `len`.
    pub fn pick_index(&mut self, len: usize) -> usize {
        js::index(js::floor(self.f() * js::num(len)))
    }

    /// `pick(list)`. An empty list draws and returns `None` (JavaScript's
    /// `undefined`).
    pub fn pick<T: Copy>(&mut self, list: &[T]) -> Option<T> {
        let i = self.pick_index(list.len());
        list.get(i).copied()
    }

    /// `chance(p)`: false for a `p` that is NaN (JavaScript's undefined).
    pub fn chance(&mut self, p: f64) -> bool {
        self.f() < p
    }

    /// `weighted(items)`'s index: which of `weights` the draw lands on (the
    /// last if rounding carries it past the end). `None` for an empty list,
    /// where JavaScript would throw.
    pub fn weighted_index(&mut self, weights: impl Iterator<Item = f64> + Clone) -> Option<usize> {
        let mut total = 0.0;
        let mut n: usize = 0;
        for w in weights.clone() {
            total += w;
            n += 1;
        }
        let mut r = self.f() * total;
        for (i, w) in weights.enumerate() {
            r -= w;
            if r <= 0.0 {
                return Some(i);
            }
        }
        n.checked_sub(1)
    }

    /// `weighted([[value, weight], ...])`.
    pub fn weighted<T: Copy>(&mut self, items: &[(T, f64)]) -> Option<T> {
        self.weighted_index(items.iter().map(|x| x.1)).map(|i| items[i].0)
    }

    /// `shuffle`, in place (the JavaScript shuffles a copy).
    pub fn shuffle<T>(&mut self, a: &mut [T]) {
        let mut i = a.len();
        while i > 1 {
            i -= 1;
            let j = js::index(js::floor(self.f() * js::num(i + 1)));
            a.swap(i, j.min(i));
        }
    }

    /// `seed32`: a fresh 32-bit seed.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // in [0, 2^32)
    pub fn seed32(&mut self) -> u32 {
        js::floor(self.f() * 4_294_967_296.0) as u32
    }
}

const ONSETS: [&str; 17] = ["k", "s", "t", "m", "n", "h", "r", "w", "y", "v", "l", "d", "f", "sh", "th", "br", "gl"];
const NUCLEI: [&str; 10] = ["a", "e", "i", "o", "u", "ai", "ei", "oa", "ui", "ou"];
const CODAS: [&str; 10] = ["", "", "", "n", "m", "l", "r", "sk", "th", "ng"];

/// A loop's name (`seedName`): two syllables drawn from its seed. At most
/// 15 bytes, so it fits `Name`.
#[must_use]
pub fn seed_name(seed: u32) -> super::list::Name {
    let mut r = Rng::new(seed ^ 0x9e37_79b9);
    let mut out = super::list::Name::new();
    for k in 0..2 {
        if k == 1 {
            out.push_str("-");
        }
        out.push_str(r.pick(&ONSETS).unwrap_or(""));
        out.push_str(r.pick(&NUCLEI).unwrap_or(""));
        out.push_str(r.pick(&CODAS).unwrap_or(""));
    }
    out
}
