//! padWow (queue item 101b; batch 6's #3, approved by Mikey 2026-11-08): a
//! slow tape-like wow on the chord layer alone, the kick and the bass steady.
//!
//! Driftloom's wobble (`master.rs`) moves the whole mix together, 1 to 11
//! cents at 0.32 Hz. Ten of the batch's records drift the pads alone 15 to
//! 27 cents at 0.3 to 1.2 Hz. This is the Ears 13 sketch
//! (`tools/ears/sketches/ears13/padWow/block.rs`) built for the app: a
//! channel's own block on the whole chords channel (`Channel::WHOLE`, as
//! padSweep is), a short delay whose time two sines move: `depth` cents at
//! `rate` Hz, and a third of that depth at 0.37 of the rate (so the drift
//! doesn't repeat exactly), read with a cubic. Its mean delay, 3 to 5 ms, is
//! the chords channel's only shift. A delay swinging A seconds at f Hz bends
//! the pitch by 2 pi f A at most, so each swing's size follows from its
//! cents and its rate. The sketch's arithmetic is kept to the operation, so
//! a loop on from its start sounds as the clips did.
//!
//! What the sketch didn't need, the app does: a loop's settings arrive at
//! its first step (`Block::set`, at a time), and a loop with no wow sets
//! the depth to 0. From still, the line fills first and then the wow fades
//! in over `FADE`; a change while it plays glides the delay onto the new
//! loop's over `GLIDE`, keeping the phases, so the delay never jumps; to
//! still, it fades out and costs nothing again. The guards are the batch's:
//! never faster than `RATE_MAX` (faster reads as vibrato) and never deeper
//! than `DEPTH_MAX`. The host puts it on the chords channel alone, never the
//! kick's or the bass's.

// Frames and the line's places as integers, the sketch's f64 arithmetic in
// f32 samples, as the sketch did.
#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::cast_sign_loss)]

use crate::QUANTUM;
use crate::block::{Block, Channel, Context, Probe, Setting, Signal};
use crate::math;

/// How a wow is set.
#[derive(Clone, Copy)]
pub struct Settings {
    /// The main swing's depth, in cents; 0 is off.
    pub depth: f64,
    /// Its rate, in Hz.
    pub rate: f64,
}

/// At rest: off.
pub const REST: Settings = Settings { depth: 0.0, rate: 0.0 };

/// Its settings a host sets, by number: the depth.
pub const DEPTH: usize = 0;
/// The rate.
pub const RATE: usize = 1;

/// The slower swing: its share of the depth and of the rate.
const SLOW_DEPTH: f64 = 0.33;
const SLOW_RATE: f64 = 0.37;

/// The batch's guards: the chords never more than 20 cents off (past it the
/// records sound seasick), so the main swing's depth is at most what keeps
/// the two swings together there, either way (14.8: +19.6 and -19.9; a delay
/// bends a little further down than up); and no faster than 1.2 Hz (faster
/// reads as vibrato).
pub const DEPTH_MAX: f64 = 14.8;
/// See `DEPTH_MAX`.
pub const RATE_MAX: f64 = 1.2;
/// The slowest it swings, in Hz (the batch's yardstick's slowest): slower,
/// the swing's delay would outgrow the line.
const RATE_MIN: f64 = 0.3;

/// The delay line's length, a power of two: at 20 cents and 0.3 Hz the mean
/// delay is 8.6 ms (the deepest at the slowest), 1,660 frames at 192 kHz.
const LINE: usize = 4096;
/// How long the wow takes to come in from still or go back out, in seconds:
/// a crossfade with the dry channel.
const FADE: f64 = 0.02;
/// How long a change of settings glides the delay, in seconds.
const GLIDE: f64 = 0.5;

/// The wow: its line, where its swings are, and the change it is making.
pub struct PadWow {
    line: [f32; LINE],
    write: usize,
    phase: [f64; 2],
    step: [f64; 2],
    /// Each swing's size and the mean delay, in frames, now; where a glide
    /// takes them, from where, and the frames it has left of how many.
    swing: [f64; 2],
    mean: f64,
    to: [f64; 3],
    from: [f64; 3],
    left: u32,
    glide: u32,
    /// Whether it is running (a wow, or one fading out), and whether it is
    /// on its way out.
    on: bool,
    out: bool,
    /// How much of the channel is the wow's, 0 to 1, the step it moves by a
    /// frame, and the frames to wait, as the line fills, before it starts.
    wet: f32,
    fade: f32,
    hold: u32,
    /// The settings waiting for their time (`set`), and that frame.
    waiting: bool,
    next: [f64; 2],
    at: u64,
    /// The settings in force.
    depth: f64,
    rate: f64,
}

impl PadWow {
    /// A wow at rest, all zeros.
    #[must_use]
    pub const fn new() -> Self {
        PadWow {
            line: [0.0; LINE],
            write: 0,
            phase: [0.0; 2],
            step: [0.0; 2],
            swing: [0.0; 2],
            mean: 0.0,
            to: [0.0; 3],
            from: [0.0; 3],
            left: 0,
            glide: 0,
            on: false,
            out: false,
            wet: 0.0,
            fade: 0.0,
            hold: 0,
            waiting: false,
            next: [0.0; 2],
            at: 0,
            depth: 0.0,
            rate: 0.0,
        }
    }

    /// The line `back` frames (and a fraction) ago, by a cubic (the sketch's).
    fn read(&self, back: f64) -> f64 {
        let whole = back as usize;
        let frac = back - whole as f64;
        let at = |k: usize| f64::from(self.line[(self.write + 2 * LINE - whole - k) % LINE]);
        // Catmull-Rom between `here` (back = whole) and `next` (whole + 1),
        // with `newer` before them and `older` after.
        let (newer, here, next, older) = (at(0), at(1), at(2), at(3));
        here + 0.5 * frac * (next - newer + frac * (2.0 * newer - 5.0 * here + 4.0 * next - older + frac * (3.0 * (here - next) + older - newer)))
    }

    /// Each swing's size and the mean delay for `depth` cents at `rate` Hz,
    /// in frames, and each phase's step (the sketch's `set_up`).
    fn shape(depth: f64, rate: f64, sr: f64) -> ([f64; 3], [f64; 2]) {
        let tau = core::f64::consts::TAU;
        let rates = [rate, rate * SLOW_RATE];
        let depths = [depth, depth * SLOW_DEPTH];
        let swing = [0, 1].map(|k| (math::pow(2.0, depths[k] / 1200.0) - 1.0) / (tau * rates[k].max(0.01)) * sr);
        let step = rates.map(|r| r / sr);
        ([swing[0], swing[1], swing[0] + swing[1] + 4.0], step)
    }

    /// The settings `depth` and `rate` take hold now: on from still, a glide
    /// onto them while it plays, or out to still.
    fn apply(&mut self, depth: f64, rate: f64, sr: f64) {
        self.depth = depth.clamp(0.0, DEPTH_MAX);
        self.rate = rate.clamp(0.0, RATE_MAX);
        if self.depth <= 0.0 || self.rate <= 0.0 {
            // Out to still, from wherever it is.
            self.out = self.on;
            return;
        }
        let (to, step) = Self::shape(self.depth, self.rate.max(RATE_MIN), sr);
        self.step = step;
        self.fade = (1.0 / (FADE * sr)) as f32;
        if self.on && !self.out {
            self.from = [self.swing[0], self.swing[1], self.mean];
            self.to = to;
            self.glide = ((GLIDE * sr) as u32).max(1);
            self.left = self.glide;
        } else {
            // From still (or on its way out, which starts again from still):
            // an empty line, filled before the wow fades in.
            if !self.on {
                self.line = [0.0; LINE];
                self.write = 0;
                self.wet = 0.0;
            }
            self.phase = [0.0, 0.25];
            [self.swing[0], self.swing[1], self.mean] = to;
            self.to = to;
            self.left = 0;
            self.hold = if self.on { 0 } else { to[2] as u32 + 4 };
            self.on = true;
            self.out = false;
        }
    }
}

impl Default for PadWow {
    fn default() -> Self {
        Self::new()
    }
}

impl Block for PadWow {
    type Settings = Settings;

    const PROBES: &'static [Probe<Settings>] = &[
        Probe { name: "gentle", settings: Settings { depth: 14.0, rate: 0.5 } },
        Probe { name: "deepest", settings: Settings { depth: DEPTH_MAX, rate: RATE_MIN } },
    ];

    const INPUT: bool = true;

    // The kit sets each to a value a listener would hear differ: the
    // deepest, and the fastest.
    const SETTINGS: &'static [Setting] = &[Setting { name: "depth", probe: DEPTH_MAX }, Setting { name: "rate", probe: RATE_MAX }];

    /// Started on, it plays at once, as the sketch did from the loop's start.
    fn start(&mut self, s: &Settings, cx: &Context) {
        *self = PadWow::new();
        self.apply(s.depth, s.rate, cx.rate);
        self.hold = 0;
        self.wet = 1.0;
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    /// A new setting takes hold at `time` (the host sets both at a loop's
    /// first step, together).
    fn set(&mut self, setting: usize, value: f64, time: f64, cx: &Context) {
        if !self.waiting {
            self.next = [self.depth, self.rate];
        }
        match setting {
            DEPTH => self.next[0] = value,
            RATE => self.next[1] = value,
            _ => return,
        }
        self.waiting = true;
        self.at = (time.max(0.0) * cx.rate) as u64;
    }

    /// Still, it passes the channel as it is and costs nothing.
    fn render(&mut self, block: u64, cx: &Context, signal: &mut Signal) {
        if !(self.on || self.waiting && self.at < block + QUANTUM as u64) {
            return;
        }
        let tau = core::f64::consts::TAU;
        for (i, x) in signal.iter_mut().enumerate() {
            if self.waiting && block + i as u64 >= self.at {
                self.waiting = false;
                self.apply(self.next[0], self.next[1], cx.rate);
            }
            if !self.on {
                continue;
            }
            if self.left > 0 {
                let k = 1.0 - f64::from(self.left) / f64::from(self.glide);
                let g = |n: usize| self.from[n] + (self.to[n] - self.from[n]) * k;
                [self.swing[0], self.swing[1], self.mean] = [g(0), g(1), g(2)];
                self.left -= 1;
            }
            let dry = *x;
            self.line[self.write] = dry;
            let back = self.mean + self.swing[0] * math::sin(tau * self.phase[0]) + self.swing[1] * math::sin(tau * self.phase[1]);
            let wow = self.read(back.clamp(1.0, (LINE - 4) as f64)) as f32;
            *x = if self.wet >= 1.0 { wow } else { dry + (wow - dry) * self.wet };
            self.write = (self.write + 1) % LINE;
            for (p, s) in self.phase.iter_mut().zip(self.step) {
                *p += s;
                if *p >= 1.0 {
                    *p -= 1.0;
                }
            }
            if self.out {
                self.wet = (self.wet - self.fade).max(0.0);
                if self.wet == 0.0 {
                    self.on = false;
                    self.out = false;
                }
            } else if self.hold > 0 {
                self.hold -= 1;
            } else {
                self.wet = (self.wet + self.fade).min(1.0);
            }
        }
    }
}

// It works on everything the chords' channel plays.
impl Channel for PadWow {
    const WHOLE: bool = true;
}
