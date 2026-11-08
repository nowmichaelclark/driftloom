//! Ears 13 sketch: padWow, a slow tape-like wow on the chord layer alone.
//!
//! Piles 4 to 6 (`docs/ears/batch-6.md`): the pads drift 15 to 27 cents at
//! 0.3 to 1.2 Hz while the kick and the bass stay put, on Smokes Quantity
//! (19 cents, 0.6 Hz), Synthesist (17, 1.2; 1847 - Earth 27, 0.3), The
//! Future Sound of Music (23, 0.3), My Incomplete T.O.E. (23, 0.3), Dam
//! Ambience (17, 0.4), Bubble Universe! (15 to 22, 0.7 to 1.7), Spiral Era
//! (16, 1.1), and drifts of 20 to 50 cents over 4 s on Structures from
//! Silence, Windows 96 and One Very Important Thought. Driftloom's wobble
//! (`master.rs`) moves the whole mix, kick and bass with it, by 1 to 11
//! cents at 0.32 Hz.
//!
//! A whole-channel processor on the chords channel (`where.txt`): a short
//! delay whose time two sines move, `depth` cents at `rate` Hz and a third of
//! it at 0.37 of that rate (so the drift does not repeat exactly), read with
//! a cubic. Its mean delay is 3 to 4 ms, the chords channel's only shift.
//! Nothing else moves; it has no width.

use crate::block::{Block, Channel, Context, Probe, Setting, Signal};
use crate::math;

/// How the wow is set.
#[derive(Clone, Copy)]
pub struct Settings {
    /// The main swing's depth, in cents.
    pub depth: f64,
    /// Its rate, in Hz.
    pub rate: f64,
}

/// The settings the registry starts it with.
pub const REST: Settings = Settings { depth: 14.0, rate: 0.5 };

/// The delay line's length, a power of two (enough at 48 kHz for 40 ms).
const LINE: usize = 2048;
/// The slower swing: its share of the depth and of the rate.
const SLOW_DEPTH: f64 = 0.33;
const SLOW_RATE: f64 = 0.37;

/// The wow: its line and where its swings are.
pub struct Sketch {
    line: [f32; LINE],
    write: usize,
    phase: [f64; 2],
    step: [f64; 2],
    swing: [f64; 2],
    mean: f64,
}

impl Sketch {
    /// At rest, all zeros.
    #[must_use]
    pub const fn new() -> Self {
        Sketch { line: [0.0; LINE], write: 0, phase: [0.0; 2], step: [0.0; 2], swing: [0.0; 2], mean: 0.0 }
    }

    /// The line `back` frames (and a fraction) ago, by a cubic.
    fn read(&self, back: f64) -> f64 {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let whole = back as usize;
        #[allow(clippy::cast_precision_loss)]
        let t = back - whole as f64;
        let at = |k: usize| f64::from(self.line[(self.write + 2 * LINE - whole - k) % LINE]);
        let (y0, y1, y2, y3) = (at(0), at(1), at(2), at(3));
        // Catmull-Rom between y1 (back = whole) and y2 (whole + 1), y0 the newer.
        let (a, b, c, d) = (y1, y2, y0, y3);
        a + 0.5 * t * (b - c + t * (2.0 * c - 5.0 * a + 4.0 * b - d + t * (3.0 * (a - b) + d - c)))
    }

    fn set_up(&mut self, depth: f64, rate: f64, sr: f64) {
        let tau = core::f64::consts::TAU;
        let rates = [rate, rate * SLOW_RATE];
        let depths = [depth, depth * SLOW_DEPTH];
        for k in 0..2 {
            // A delay swinging A s at f Hz bends the pitch by 2 pi f A at most.
            let frac = math::pow(2.0, depths[k] / 1200.0) - 1.0;
            self.swing[k] = frac / (tau * rates[k].max(0.01)) * sr;
            self.step[k] = rates[k] / sr;
        }
        self.mean = self.swing[0] + self.swing[1] + 4.0;
    }
}

impl Default for Sketch {
    fn default() -> Self {
        Self::new()
    }
}

impl Block for Sketch {
    type Settings = Settings;

    const PROBES: &'static [Probe<Settings>] = &[
        Probe { name: "gentle", settings: REST },
        Probe { name: "records", settings: Settings { depth: 20.0, rate: 0.4 } },
    ];

    const INPUT: bool = true;

    const SETTINGS: &'static [Setting] = &[Setting { name: "depth", probe: 30.0 }];

    fn start(&mut self, s: &Settings, cx: &Context) {
        *self = Sketch::new();
        self.phase = [0.0, 0.25];
        self.set_up(s.depth, s.rate, cx.rate);
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    fn set(&mut self, setting: usize, value: f64, _: f64, cx: &Context) {
        if setting == 0 {
            let rate = self.step[0] * cx.rate;
            self.set_up(value.clamp(0.0, 60.0), rate, cx.rate);
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn render(&mut self, _: u64, _: &Context, signal: &mut Signal) {
        let tau = core::f64::consts::TAU;
        for x in signal.iter_mut() {
            self.line[self.write] = *x;
            let back = self.mean + self.swing[0] * math::sin(tau * self.phase[0]) + self.swing[1] * math::sin(tau * self.phase[1]);
            *x = self.read(back.clamp(1.0, (LINE - 4) as f64)) as f32;
            self.write = (self.write + 1) % LINE;
            for k in 0..2 {
                self.phase[k] += self.step[k];
                if self.phase[k] >= 1.0 {
                    self.phase[k] -= 1.0;
                }
            }
        }
    }
}

// It works on everything its channel plays.
impl Channel for Sketch {
    const WHOLE: bool = true;
}
