//! Ears 13 sketch: widePad, the chord layer made wide on the side alone.
//!
//! Batch 5's widePad (eight records of piles 1 and 2, the pad's left and
//! right correlating 0.31 to 0.46 over a mono sub), and piles 4 to 6
//! (`docs/ears/batch-6.md`): pads with a correlation of 0.0 to 0.5 above 120 Hz
//! over a centered low end on Far Away Trains Passing By, Kankyo Ongaku,
//! Mirror's Edge, Windows 96, Bubble Universe!, Holographic Universe(s)!,
//! Triangular Reformat, Conscious Agents, Synthesist and Pyramid. Driftloom's
//! approved stereo (the space) keeps the side about 18 to 20 dB under the mid.
//!
//! A whole-channel block on the chords channel (`where.txt`) that leaves the
//! channel's signal (the mid, and so the mono render and the fold) bit for
//! bit as it was, and adds to the side a copy of it through three allpasses
//! (so it is unrelated to the mid, not a delayed echo of it), highpassed at
//! 250 Hz (the low end stays centered), at `width`. With `open` above 0 the
//! width grows from nothing over that many seconds from the start (the
//! records' width that opens late), else it is there from the start. In a
//! mono render it does nothing.

use crate::block::{Block, Channel, Context, Mods, Probe, Signal};
use crate::math;

/// How wide.
#[derive(Clone, Copy)]
pub struct Settings {
    /// The side's level against the chords (1: as loud as the mid).
    pub width: f64,
    /// Seconds to open over from the start; 0: open from the start.
    pub open: f64,
}

/// The settings the registry starts it with.
pub const REST: Settings = Settings { width: 0.55, open: 0.0 };

/// The allpasses' delays (ms) and their gain.
const DELAYS: [f64; 3] = [4.7, 7.9, 12.3];
const AP: f64 = 0.6;
/// Each allpass's line (enough at 48 kHz for 21 ms).
const LINE: usize = 1024;
/// The side's highpass, in Hz.
const LOW: f64 = 250.0;

/// The width: its allpasses and highpass.
pub struct Sketch {
    lines: [[f32; LINE]; 3],
    len: [usize; 3],
    at: usize,
    hp: [f64; 5],
    mem: [f64; 4],
    width: f64,
    grow: f64,
    gain: f64,
}

impl Sketch {
    /// At rest, all zeros.
    #[must_use]
    pub const fn new() -> Self {
        Sketch { lines: [[0.0; LINE]; 3], len: [0; 3], at: 0, hp: [0.0; 5], mem: [0.0; 4], width: 0.0, grow: 0.0, gain: 0.0 }
    }
}

impl Default for Sketch {
    fn default() -> Self {
        Self::new()
    }
}

impl Block for Sketch {
    type Settings = Settings;

    const PROBES: &'static [Probe<Settings>] = &[Probe { name: "wide", settings: REST }, Probe { name: "open", settings: Settings { width: 0.55, open: 0.5 } }];

    const INPUT: bool = true;

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn start(&mut self, s: &Settings, cx: &Context) {
        *self = Sketch::new();
        for (k, &ms) in DELAYS.iter().enumerate() {
            self.len[k] = ((ms * 0.001 * cx.rate) as usize).clamp(1, LINE - 1);
        }
        // An RBJ highpass, Q 0.707.
        let w = core::f64::consts::TAU * LOW / cx.rate;
        let (sn, cs) = (math::sin(w), math::cos(w));
        let alpha = sn / (2.0 * core::f64::consts::FRAC_1_SQRT_2);
        let a0 = 1.0 + alpha;
        self.hp = [(1.0 + cs) / 2.0 / a0, -(1.0 + cs) / a0, (1.0 + cs) / 2.0 / a0, -2.0 * cs / a0, (1.0 - alpha) / a0];
        self.width = s.width;
        if s.open > 0.0 {
            self.gain = 0.0;
            self.grow = 1.0 / (s.open * cx.rate);
        } else {
            self.gain = 1.0;
            self.grow = 0.0;
        }
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    // The mono render: nothing changes.
    fn render(&mut self, _: u64, _: &Context, _: &mut Signal) {}
}

impl Channel for Sketch {
    const WHOLE: bool = true;

    #[allow(clippy::cast_possible_truncation)]
    fn render_side(&mut self, _: u64, _: &Context, signal: &mut Signal, _: &Mods, side: &mut Signal) {
        let [b0, b1, b2, a1, a2] = self.hp;
        for (x, s) in signal.iter().zip(side.iter_mut()) {
            let mut y = f64::from(*x);
            for k in 0..3 {
                let back = (self.at + LINE - self.len[k]) % LINE;
                let d = f64::from(self.lines[k][back]);
                let v = y + AP * d;
                self.lines[k][self.at] = v as f32;
                y = d - AP * v;
            }
            self.at = (self.at + 1) % LINE;
            let [x1, x2, y1, y2] = self.mem;
            let h = b0 * y + b1 * x1 + b2 * x2 - a1 * y1 - a2 * y2;
            self.mem = [y, x1, h, y1];
            self.gain = (self.gain + self.grow).min(1.0);
            *s += (self.width * self.gain * h) as f32;
        }
    }
}
