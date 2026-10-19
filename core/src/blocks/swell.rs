//! A swell on a whole channel, whose rate the host sets for the loop.
//!
//! The first of queue item 46c's three demonstration blocks, in the tests
//! only (its line in the registry carries `#[cfg(test)]`). It proves that a
//! channel's own block with per-loop settings is still one new file -- this
//! one -- plus one line in the registry: it works on its channel's whole
//! signal (`Channel::WHOLE`), carries its own `Lfo`, and says which of its
//! settings a host sets (`SETTINGS`); `Core::set` reaches them, and the test
//! kit finds and probes them with no test written for it. padSweep (item
//! 72) and the arc (item 63) are this, with a filter in place of the gain.
//! `core/README.md` walks through it.

use crate::QUANTUM;
use crate::block::{Block, Channel, Context, Probe, Setting, Signal};

use super::lfo::Lfo;

/// How a swell is set: how fast its gain swings and how far.
#[derive(Clone, Copy)]
pub struct Settings {
    /// Swings a second, in Hz.
    pub rate: f32,
    /// How far the gain swings either side of 1; 0 leaves the channel as it
    /// is, and costs nothing.
    pub depth: f32,
}

/// At rest: no swing, so the channel passes as it is.
pub const REST: Settings = Settings { rate: 1.0, depth: 0.0 };

/// Its settings a host sets, by number: the rate, then the depth.
pub const RATE: usize = 0;
/// The depth.
pub const DEPTH: usize = 1;

/// The swell: its LFO, and what it was last set to.
pub struct Swell {
    lfo: Lfo<0>,
    rate: f32,
    depth: f32,
}

impl Swell {
    /// A swell at rest, all zeros.
    #[must_use]
    pub const fn new() -> Self {
        Swell { lfo: Lfo::new(), rate: 0.0, depth: 0.0 }
    }

    /// Swing from `time` on, at the rate and depth it holds.
    fn swing_from(&mut self, time: f64, cx: &Context) {
        self.lfo.set(self.rate, self.depth, time, f64::INFINITY, cx.rate);
    }
}

impl Default for Swell {
    fn default() -> Self {
        Self::new()
    }
}

impl Block for Swell {
    type Settings = Settings;

    const PROBES: &'static [Probe<Settings>] = &[Probe { name: "slow", settings: Settings { rate: 4.0, depth: 0.4 } }];

    const INPUT: bool = true;

    // The kit sets the rate to a slower swing, and the depth to a deeper one.
    const SETTINGS: &'static [Setting] = &[Setting { name: "rate", probe: 1.5 }, Setting { name: "depth", probe: 0.8 }];

    fn start(&mut self, s: &Settings, cx: &Context) {
        self.rate = s.rate;
        self.depth = s.depth;
        self.swing_from(0.0, cx);
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    /// A new rate or depth starts the swing again from `time`.
    // A setting is a 32-bit float, as an `AudioParam`'s value is.
    #[allow(clippy::cast_possible_truncation)]
    fn set(&mut self, setting: usize, value: f64, time: f64, cx: &Context) {
        match setting {
            RATE => self.rate = value as f32,
            DEPTH => self.depth = value as f32,
            _ => return,
        }
        self.swing_from(time, cx);
    }

    fn render(&mut self, block: u64, cx: &Context, signal: &mut Signal) {
        if self.depth == 0.0 {
            return;
        }
        let mut swing = [0.0; QUANTUM];
        self.lfo.render(block, cx, &mut swing);
        for (x, s) in signal.iter_mut().zip(swing.iter()) {
            *x *= 1.0 + *s;
        }
    }
}

// It works on everything its channel plays.
impl Channel for Swell {
    const WHOLE: bool = true;
}
