//! The arc (queue item 63; the prototypes, approved by Mikey 2026-10-10): a
//! second lowpass after the master's tone filter that breathes over minutes,
//! from 300 Hz up to 9 kHz and back, lingering on the dark side, and lifts
//! the echo's feedback a little as it opens.
//!
//! Its swing is the prototype's (`startArc` on the `ears/prototypes`
//! branch): over a period of 2 to 5 minutes, the cutoff at a point `u` of it
//! is `300 x 30^(p^2)`, where `p = (1 - cos(2 pi u)) / 2`, and the echo's
//! feedback rises by `0.14 p`. A loop starts at a point of the swing the
//! host draws, a bright one (the prototype started every loop closed, at 300
//! Hz). It glides into a new loop's swing, or out of the last one, over half
//! a second, so a change never jumps the cutoff; with no arc it is no filter
//! at all, and the chain renders as it always has.
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

/// How it is set when it starts: a swing of `period` seconds (0 for none),
/// at point `from` (0 to 1: 0 and 1 the dark end, 0.5 the bright) at time 0.
#[derive(Clone, Copy)]
pub struct Settings {
    /// The swing's period, in seconds; 0 for no arc.
    pub period: f64,
    /// Where on it the start is, 0 to 1.
    pub from: f64,
}

/// No arc: the chain as it was.
pub const REST: Settings = Settings { period: 0.0, from: 0.0 };

/// One loop's swing: from when, how long, from where.
#[derive(Clone, Copy, PartialEq)]
struct Swing {
    start: f64,
    period: f64,
    from: f64,
}

impl Swing {
    const NONE: Swing = Swing { start: 0.0, period: 0.0, from: 0.0 };

    /// How open it is at `t`, 0 to 1 (`p`), or `None` for no arc.
    fn open(self, t: f64) -> Option<f64> {
        (self.period > 0.0).then(|| {
            let u = self.from + (t - self.start) / self.period;
            (1.0 - math::cos(TAU * u)) / 2.0
        })
    }

    /// The cutoff and the feedback's lift at `t`.
    fn at(self, t: f64, rate: f64) -> (f64, f64) {
        let top = OPEN.min(NYQUIST_SHARE * rate);
        match self.open(t) {
            Some(p) => ((LOW * math::pow(HIGH / LOW, p * p)).min(top), LIFT * p),
            None => (top, 0.0),
        }
    }
}

/// The arc: the swing now, the one before it (gliding from), and the next a
/// host has asked for (from its time).
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

    /// A swing of `period` seconds (0 for none) from `time`, starting at
    /// point `from` of it: a loop's arc, or the end of one.
    pub fn begin(&mut self, period: f64, from: f64, time: f64) {
        self.next = Swing { start: time, period: period.max(0.0), from: from - math::floor(from) };
        self.pending = true;
    }

    /// The cutoff and the feedback's lift at `t`: the swing's, gliding from
    /// the last one's for `GLIDE` seconds after it starts (geometric for the
    /// cutoff, straight for the feedback).
    fn at(&self, t: f64) -> (f64, f64) {
        let (f, lift) = self.now.at(t, self.rate);
        let into = t - self.now.start;
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
    /// no arc.
    pub fn process(&mut self, block: u64, signal: &mut Signal) {
        self.advance(block);
        let (start, end) = self.ends(block);
        if !self.active(start) {
            return;
        }
        let (f, _) = self.at(f64::midpoint(start, end));
        self.filter.set(Kind::Lowpass, f as f32, Q, 0.0, self.rate as f32);
        for x in signal.iter_mut() {
            *x = self.filter.step(*x);
        }
        self.filter.flush();
        // Glided out and rung down: no filter from here on.
        if self.now.period <= 0.0 && end >= self.now.start + GLIDE {
            self.filter.reset();
        }
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
    // noise: a swing of 1.2 s from the bright point.
    const PROBES: &'static [Probe<Settings>] = &[Probe { name: "breath", settings: Settings { period: 1.2, from: 0.5 } }];

    const INPUT: bool = true;

    fn start(&mut self, s: &Settings, cx: &Context) {
        self.ready(cx.rate);
        self.begin(s.period, s.from, 0.0);
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    fn render(&mut self, block: u64, _: &Context, signal: &mut Signal) {
        self.process(block, signal);
    }
}
