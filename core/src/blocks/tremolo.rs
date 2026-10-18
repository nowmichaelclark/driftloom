//! A tremolo: the signal's level swung up and down by a slow sine.
//!
//! The demonstration block (queue item 46). It is in the tests only (its
//! line in the registry carries `#[cfg(test)]`), and it proves that a new
//! building block is one new file -- this one -- plus one line in the
//! registry: the test kit finds it there and checks its level, tone, cost,
//! determinism and stored render, with no test written for it.
//! `core/README.md` walks through it.

use core::f64::consts::TAU;

use crate::block::{Block, Context, Probe, Signal};
use crate::math;

/// How a tremolo is set.
#[derive(Clone, Copy)]
pub struct Settings {
    /// How many swings a second.
    pub rate: f64,
    /// How deep each swing goes: 0 leaves the level alone, 1 swings it all
    /// the way to silence.
    pub depth: f64,
}

/// The tremolo's state: where it is in its swing, and how far it moves a
/// frame.
pub struct Tremolo {
    phase: f64,
    step: f64,
    depth: f64,
}

impl Tremolo {
    /// A tremolo that leaves the signal alone, all zeros.
    #[must_use]
    pub const fn new() -> Self {
        Tremolo { phase: 0.0, step: 0.0, depth: 0.0 }
    }
}

impl Default for Tremolo {
    fn default() -> Self {
        Self::new()
    }
}

impl Block for Tremolo {
    type Settings = Settings;

    const PROBES: &'static [Probe<Settings>] = &[
        Probe { name: "slow", settings: Settings { rate: 4.0, depth: 0.5 } },
        Probe { name: "deep", settings: Settings { rate: 7.5, depth: 1.0 } },
    ];

    // It works on the signal it is given.
    const INPUT: bool = true;

    fn start(&mut self, s: &Settings, cx: &Context) {
        self.phase = 0.0;
        self.step = s.rate / cx.rate;
        self.depth = s.depth.clamp(0.0, 1.0);
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    // The gain is worked out in 64 bits and applied to a 32-bit sample.
    #[allow(clippy::cast_possible_truncation)]
    fn render(&mut self, _: u64, _: &Context, signal: &mut Signal) {
        for x in signal.iter_mut() {
            // From 1 down to 1 - depth and back, once a swing.
            let gain = 1.0 - self.depth * 0.5 * (1.0 - math::cos(TAU * self.phase));
            *x *= gain as f32;
            self.phase += self.step;
            if self.phase >= 1.0 {
                self.phase -= 1.0;
            }
        }
    }
}
