//! The arc (queue item 63; the prototypes, approved by Mikey 2026-10-10): a
//! second lowpass after the master's tone filter that breathes over minutes,
//! and lifts the echo's feedback a little as it opens. It has two shapes
//! (queue item 63b, Mikey 2026-10-21), which the host draws per loop:
//!
//! - **Deep**, the prototype's swing (`startArc` on the `ears/prototypes`
//!   branch): from 300 Hz up to 9 kHz and back, lingering on the dark side.
//!   At a point `u` of the swing the cutoff is `300 x 30^(p^2)`, where
//!   `p = (1 - cos(2 pi u)) / 2`, and the echo's feedback rises by `0.14 p`.
//!   Its cutoff sits under 700 Hz for about half of every swing, which is why
//!   it is the rarer shape: a listener who doesn't know it returns hears the
//!   instruments go missing.
//! - **Dip**, mostly open: at 9 kHz for all but a share of the swing (`dip`,
//!   about 35%), where it dips once, smoothly, to a `floor` (800 Hz to 1.5
//!   kHz) and back: a raised cosine in log frequency, darkest at `u = 0` as
//!   the deep swing is. The echo's feedback rises with how open it is as the
//!   deep swing's does: the same cutoff, the same lift.
//!
//! A loop starts at a point of the swing the host draws, a bright one (the
//! prototype started every loop closed, at 300 Hz). It glides into a new
//! loop's swing, or out of the last one, over half a second, so a change
//! never jumps the cutoff; with no arc it is no filter at all, and the chain
//! renders as it always has.
//!
//! The swing is worked out from the time, at each block's two ends, rather
//! than run by an `Lfo`: a loop starts mid-swing, at a phase an oscillator
//! (which starts at 0, as Web Audio's does) cannot start at, and over
//! minutes a block is a millionth of the swing, so the cutoff is set once a
//! block and the feedback drawn straight across it. The master holds one in
//! each of its lanes (a processor on each side, `Pair`'s way) and the host
//! starts it through `Core::param` (`ARC_FROM`, `ARC_PERIOD`).

// Web Audio's parameters are 32-bit floats, set from 64-bit numbers.
#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]

use core::f64::consts::TAU;

use crate::block::{Block, Context, Probe, Signal};
use crate::math;
use crate::QUANTUM;

use super::filter::{Biquad, Kind};

/// The cutoff at the swing's dark end, and how far it opens (9 kHz).
const LOW: f64 = 300.0;
const HIGH: f64 = 9000.0;
/// The lowpass's Q (Web Audio's, in dB): the prototype's.
const Q: f32 = 0.7;
/// How far the echo's feedback rises at the top of the swing.
const LIFT: f64 = 0.14;
/// How long a change glides, in seconds.
const GLIDE: f64 = 0.5;
/// Where the filter stands with no arc, as a cutoff (above anything the
/// chain lets through), and the share of the rate it is held under.
const OPEN: f64 = 20_000.0;
const NYQUIST_SHARE: f64 = 0.45;

/// The swing's shape (queue item 63b).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// The prototype's: 300 Hz to 9 kHz and back, lingering dark.
    Deep,
    /// Open at 9 kHz but for one smooth dip to `Settings::floor`.
    Dip,
}

impl Shape {
    /// The shape a host's number names (`ARC_SHAPE`): 1 the dip, 0 (or
    /// anything up to 0.5) the deep swing.
    #[must_use]
    pub fn from_number(value: f64) -> Shape {
        if value > 0.5 { Shape::Dip } else { Shape::Deep }
    }
}

/// How it is set when it starts: a swing of `period` seconds (0 for none),
/// at point `from` (0 to 1: 0 and 1 the darkest, 0.5 the brightest) at time
/// 0, of either shape.
#[derive(Clone, Copy)]
pub struct Settings {
    /// The swing's period, in seconds; 0 for no arc.
    pub period: f64,
    /// Where on it the start is, 0 to 1.
    pub from: f64,
    /// Its shape.
    pub shape: Shape,
    /// The dip's floor, in Hz (held to 300 Hz to 9 kHz); the deep swing
    /// doesn't read it.
    pub floor: f64,
    /// The share of the swing the dip takes, around its darkest point (held
    /// to 0 to 1); the deep swing doesn't read it.
    pub dip: f64,
}

/// No arc: the chain as it was. Its floor and dip are the host's defaults
/// (`ARC` in `js/choices.js`), for a host that starts a dip without naming
/// them.
pub const REST: Settings = Settings { period: 0.0, from: 0.0, shape: Shape::Deep, floor: 1000.0, dip: 0.35 };

/// No arc, all zeros: what a core built in a static holds until it starts
/// (a nonzero field would put the whole core in the `.wasm`'s data).
pub const ZERO: Settings = Settings { period: 0.0, from: 0.0, shape: Shape::Deep, floor: 0.0, dip: 0.0 };

/// One loop's swing: from when, how long, from where, and its shape. For a
/// dip, `depth` is how far down it goes, as a share of the deep swing's
/// whole range in log frequency (from 9 kHz to 300 Hz), worked out once.
#[derive(Clone, Copy, PartialEq)]
struct Swing {
    start: f64,
    period: f64,
    from: f64,
    shape: Shape,
    floor: f64,
    dip: f64,
    depth: f64,
}

impl Swing {
    // All zeros, as `ZERO` is.
    const NONE: Swing = Swing { start: 0.0, period: 0.0, from: 0.0, shape: Shape::Deep, floor: 0.0, dip: 0.0, depth: 0.0 };

    fn new(s: &Settings, start: f64) -> Swing {
        let floor = s.floor.clamp(LOW, HIGH);
        let depth = f64::from(math::log2f((HIGH / floor) as f32) / math::log2f((HIGH / LOW) as f32));
        Swing { start, period: s.period.max(0.0), from: s.from - math::floor(s.from), shape: s.shape, floor, dip: s.dip.clamp(0.0, 1.0), depth }
    }

    /// The cutoff and the feedback's lift at `t`.
    fn at(self, t: f64, rate: f64) -> (f64, f64) {
        let top = OPEN.min(NYQUIST_SHARE * rate);
        if self.period <= 0.0 {
            return (top, 0.0);
        }
        let u = self.from + (t - self.start) / self.period;
        match self.shape {
            Shape::Deep => {
                // How open it is, 0 to 1.
                let p = (1.0 - math::cos(TAU * u)) / 2.0;
                ((LOW * math::pow(HIGH / LOW, p * p)).min(top), LIFT * p)
            }
            Shape::Dip => {
                // How far down, 0 (open) to 1 (the floor): a raised cosine
                // across the dip's share of the swing, centered on the
                // swing's darkest point (`off`, in swings from it).
                let off = u - math::floor(u + 0.5);
                let down = if off.abs() < self.dip / 2.0 { f64::midpoint(1.0, math::cos(TAU * off / self.dip)) } else { 0.0 };
                let cutoff = HIGH * math::pow(self.floor / HIGH, down);
                // The deep swing's lift for the same cutoff: there the cutoff
                // sits `p^2` of the way from 300 Hz to 9 kHz in log
                // frequency, and the lift is `0.14 p`.
                (cutoff.min(top), LIFT * math::sqrt(1.0 - down * self.depth))
            }
        }
    }
}

/// The arc: the swing now, the one before it (gliding from), and the next a
/// host has asked for (from its time).
#[derive(Clone)]
pub struct Arc {
    now: Swing,
    was: Swing,
    next: Swing,
    pending: bool,
    filter: Biquad,
    rate: f64,
}

impl Arc {
    /// No arc, all zeros.
    #[must_use]
    pub const fn new() -> Self {
        Arc { now: Swing::NONE, was: Swing::NONE, next: Swing::NONE, pending: false, filter: Biquad::new(), rate: 0.0 }
    }

    /// At rest at `rate` Hz.
    pub fn ready(&mut self, rate: f64) {
        *self = Arc::new();
        self.rate = rate;
    }

    /// The swing `s` describes from `time` (a period of 0 for none): a
    /// loop's arc, or the end of one.
    pub fn begin(&mut self, s: &Settings, time: f64) {
        self.next = Swing::new(s, time);
        self.pending = true;
    }

    /// The cutoff and the feedback's lift at `t`: the swing's, gliding from
    /// the last one's for `GLIDE` seconds after it starts (geometric for the
    /// cutoff, straight for the feedback), and the last one's before it
    /// starts (queue item 86: a swing is taken in the block it starts in, and
    /// read before its start there it jumped the cutoff for a block, open
    /// for an arc's end, which rang).
    fn at(&self, t: f64) -> (f64, f64) {
        let (f, lift) = self.now.at(t, self.rate);
        let into = t - self.now.start;
        if into < 0.0 && self.was != self.now {
            return self.was.at(t, self.rate);
        }
        if into >= GLIDE || into < 0.0 || self.was == self.now {
            return (f, lift);
        }
        let w = into / GLIDE;
        let (f0, lift0) = self.was.at(t, self.rate);
        (math::pow(f0, 1.0 - w) * math::pow(f, w), lift0 + (lift - lift0) * w)
    }

    /// Whether it is a filter at all in a block that starts at `start`: an
    /// arc, or one gliding out.
    fn active(&self, start: f64) -> bool {
        self.now.period > 0.0 || (self.was.period > 0.0 && start < self.now.start + GLIDE)
    }

    /// Take the swing a host asked for, once its time has come.
    fn advance(&mut self, block: u64) {
        if self.pending && super::osc::frame_at(self.next.start, self.rate) < block + QUANTUM as u64 {
            self.was = self.now;
            self.now = self.next;
            self.pending = false;
        }
    }

    /// What the echo's feedback rises by, frame by frame, over the block
    /// from `block`; `None` for no arc (the mix as it was).
    pub fn lift(&mut self, block: u64) -> Option<Signal> {
        self.advance(block);
        let (start, end) = self.ends(block);
        if !self.active(start) {
            return None;
        }
        let (a, b) = (self.at(start).1, self.at(end).1);
        let mut out = [0.0f32; QUANTUM];
        for (i, y) in out.iter_mut().enumerate() {
            *y = (a + (b - a) * (i as f64 / QUANTUM as f64)) as f32;
        }
        Some(out)
    }

    fn ends(&self, block: u64) -> (f64, f64) {
        (block as f64 / self.rate, (block + QUANTUM as u64) as f64 / self.rate)
    }

    /// Filter `signal`, the block from `block`, in place: nothing at all with
    /// no arc. With none the filter still runs, open, on the signal, which it
    /// leaves as it is (queue item 86): so an arc that starts while the music
    /// plays (a loop with one after a loop without) takes over from a filter
    /// already in step with it, not from rest, which stepped the output.
    /// Before any music its state is all zeros, as it always was.
    pub fn process(&mut self, block: u64, signal: &mut Signal) {
        self.advance(block);
        let (start, end) = self.ends(block);
        let (f, _) = self.at(f64::midpoint(start, end));
        self.filter.set(Kind::Lowpass, f as f32, Q, 0.0, self.rate as f32);
        if self.active(start) {
            for x in signal.iter_mut() {
                *x = self.filter.step(*x);
            }
        } else {
            for &x in signal.iter() {
                self.filter.step(x);
            }
        }
        self.filter.flush();
    }
}

impl Default for Arc {
    fn default() -> Self {
        Self::new()
    }
}

impl Block for Arc {
    type Settings = Settings;

    // Fast enough for the kit to hear it close and open within a second of
    // noise: a swing of 1.2 s from the bright point, of each shape (the dip
    // to its lowest floor, from 0.39 s to 0.81 s).
    const PROBES: &'static [Probe<Settings>] = &[
        Probe { name: "breath", settings: Settings { period: 1.2, from: 0.5, ..REST } },
        Probe { name: "dip", settings: Settings { period: 1.2, from: 0.5, shape: Shape::Dip, floor: 800.0, ..REST } },
    ];

    const INPUT: bool = true;

    fn start(&mut self, s: &Settings, cx: &Context) {
        self.ready(cx.rate);
        self.begin(s, 0.0);
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    fn render(&mut self, block: u64, _: &Context, signal: &mut Signal) {
        self.process(block, signal);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATE: f64 = 48_000.0;

    /// The cutoff and lift across one swing, at `n` points.
    fn swing(s: &Settings, n: usize) -> impl Iterator<Item = (f64, f64)> + '_ {
        let w = Swing::new(s, 0.0);
        (0..n).map(move |i| w.at(i as f64 / n as f64 * s.period, RATE))
    }

    #[test]
    fn deep_is_the_prototypes_swing() {
        let s = Settings { period: 180.0, from: 0.47, ..REST };
        for (i, (f, lift)) in swing(&s, 1000).enumerate() {
            let u = 0.47 + (i as f64 / 1000.0 * 180.0) / 180.0;
            let p = (1.0 - math::cos(TAU * u)) / 2.0;
            assert_eq!(f.to_bits(), (LOW * math::pow(HIGH / LOW, p * p)).to_bits());
            assert_eq!(lift.to_bits(), (LIFT * p).to_bits());
        }
    }

    #[test]
    fn the_dip_stays_open_and_never_under_its_floor() {
        for floor in [800.0, 1000.0, 1500.0] {
            let s = Settings { period: 200.0, from: 0.5, shape: Shape::Dip, floor, dip: 0.35 };
            let n = 20_000;
            let (mut low, mut under_2k, mut open, mut near) = (f64::MAX, 0, 0, 0);
            for (f, _) in swing(&s, n) {
                low = low.min(f);
                under_2k += usize::from(f < 2000.0);
                open += usize::from(f == HIGH);
                near += usize::from(f <= 2.0 * floor);
            }
            let share = |k: usize| k as f64 / n as f64;
            assert!(low >= floor * (1.0 - 1e-12) && low < floor * 1.001, "{floor}: {low}");
            assert!(share(under_2k) <= 0.15, "{floor}: {}", share(under_2k));
            assert!((share(open) - 0.65).abs() < 0.001, "{floor}: {}", share(open));
            // Within twice the floor: 12.6% of the swing at 800 Hz, 15.0% at
            // 1.5 kHz (25 to 30 s of a 200 s swing).
            assert!((0.12..0.155).contains(&share(near)), "{floor}: {}", share(near));
        }
    }

    #[test]
    fn the_dip_lifts_the_echo_as_the_deep_swing_does_for_the_same_cutoff() {
        let s = Settings { period: 120.0, from: 0.0, shape: Shape::Dip, floor: 800.0, dip: 0.35 };
        for (f, lift) in swing(&s, 2000) {
            // The deep swing's p for this cutoff.
            let p = math::sqrt(f64::from(math::log2f((f / LOW) as f32) / math::log2f((HIGH / LOW) as f32)));
            assert!((lift - LIFT * p).abs() < 1e-5, "{f}: {lift} against {}", LIFT * p);
        }
    }

    #[test]
    fn the_dip_is_darkest_at_the_swings_darkest_point() {
        let s = Settings { period: 100.0, from: 0.0, shape: Shape::Dip, floor: 900.0, dip: 0.35 };
        let w = Swing::new(&s, 0.0);
        assert!((w.at(0.0, RATE).0 - 900.0).abs() < 1e-9);
        assert!((w.at(100.0, RATE).0 - 900.0).abs() < 1e-6);
        assert_eq!(w.at(50.0, RATE).0, HIGH);
    }
}
