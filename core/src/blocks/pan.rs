//! A panner: the signal placed between the left and the right.
//!
//! The second of queue item 46b's two demonstration blocks, in the tests
//! only (its line in the registry carries `#[cfg(test)]`). It proves that a
//! block with a stereo render of its own is still one new file -- this one
//! -- plus one line in the registry: it says `STEREO` and writes
//! `render_stereo`, and the test kit's stereo checks find it there, with no
//! test written for it. `core/README.md` walks through it.
//!
//! The law is a balance: at the center both sides are the signal itself
//! (the core's rule for a centered mono block), and toward one side the
//! other side falls away in a straight line, to silence at the end.

use crate::block::{Block, Context, Probe, Signal, Stereo};

/// How a panner is set: where it places the signal.
#[derive(Clone, Copy)]
pub struct Settings {
    /// From -1 (all left) through 0 (the center) to 1 (all right).
    pub place: f32,
}

/// The panner's state: each side's gain.
pub struct Pan {
    left: f32,
    right: f32,
}

impl Pan {
    /// A panner that silences both sides, all zeros.
    #[must_use]
    pub const fn new() -> Self {
        Pan { left: 0.0, right: 0.0 }
    }
}

impl Default for Pan {
    fn default() -> Self {
        Self::new()
    }
}

impl Block for Pan {
    type Settings = Settings;

    const PROBES: &'static [Probe<Settings>] = &[
        Probe { name: "left", settings: Settings { place: -0.6 } },
        Probe { name: "right", settings: Settings { place: 0.3 } },
    ];

    // It works on the signal it is given, and it is stereo.
    const INPUT: bool = true;
    const STEREO: bool = true;

    fn start(&mut self, s: &Settings, _: &Context) {
        let place = s.place.clamp(-1.0, 1.0);
        self.left = 1.0 - place.max(0.0);
        self.right = 1.0 + place.min(0.0);
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    /// In a mono render a panner has nowhere to place the signal: it passes
    /// it as it is.
    fn render(&mut self, _: u64, _: &Context, _: &mut Signal) {}

    /// The two sides folded to one, then each side at its gain.
    fn render_stereo(&mut self, _: u64, _: &Context, signal: &mut Stereo) {
        let [l, r] = signal;
        for (a, b) in l.iter_mut().zip(r.iter_mut()) {
            let mid = f32::midpoint(*a, *b);
            *a = mid * self.left;
            *b = mid * self.right;
        }
    }
}
