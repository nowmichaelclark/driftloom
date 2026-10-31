//! The demonstration sketch: a veil, a gentle lowpass mixed under the dry
//! signal, on the melody (`where.txt`).
//!
//! This is what a sketch looks like: the kit's interface, and nothing else.
//! `struct Sketch` with a `const fn new()` (all zeros), `pub const REST` (the
//! settings the registry starts it with), `impl Block` (at least one probe)
//! and `impl Channel` (where on the channel it works). Replace the file with
//! your own; `core/sketch/README.md` says how to hear and probe it.

use crate::block::{Block, Channel, Context, Probe, Setting, Signal};
use crate::math;

/// How the veil is set.
#[derive(Clone, Copy)]
pub struct Settings {
    /// The lowpass's corner, in Hz.
    pub corner: f64,
    /// How much of the lowpassed signal replaces the dry: 0 leaves it as it
    /// is, 1 is all lowpass.
    pub mix: f64,
}

/// The settings the registry starts it with.
pub const REST: Settings = Settings { corner: 1_800.0, mix: 0.5 };

/// The veil: a one-pole lowpass and its memory.
pub struct Sketch {
    coeff: f32,
    mix: f32,
    low: f32,
}

impl Sketch {
    /// A veil at rest, all zeros.
    #[must_use]
    pub const fn new() -> Self {
        Sketch { coeff: 0.0, mix: 0.0, low: 0.0 }
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
        Probe { name: "soft", settings: REST },
        Probe { name: "dark", settings: Settings { corner: 600.0, mix: 1.0 } },
    ];

    // It works on the signal it is given.
    const INPUT: bool = true;

    const SETTINGS: &'static [Setting] = &[Setting { name: "mix", probe: 1.0 }];

    // A one-pole's coefficient and mix are 32-bit, as the signal is.
    #[allow(clippy::cast_possible_truncation)]
    fn start(&mut self, s: &Settings, cx: &Context) {
        self.coeff = (1.0 - math::exp(-core::f64::consts::TAU * s.corner / cx.rate)) as f32;
        self.mix = s.mix.clamp(0.0, 1.0) as f32;
        self.low = 0.0;
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn set(&mut self, setting: usize, value: f64, _: f64, _: &Context) {
        if setting == 0 {
            self.mix = value.clamp(0.0, 1.0) as f32;
        }
    }

    fn render(&mut self, _: u64, _: &Context, signal: &mut Signal) {
        for x in signal.iter_mut() {
            self.low += self.coeff * (*x - self.low);
            *x += self.mix * (self.low - *x);
        }
    }
}

// It works on everything its channel plays.
impl Channel for Sketch {
    const WHOLE: bool = true;
}
