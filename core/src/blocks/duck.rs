//! A duck: a gain on a whole channel, pressed down by another channel's
//! signal.
//!
//! The third of queue item 46c's demonstration blocks, in the tests only
//! (its line in the registry carries `#[cfg(test)]`). It proves that a
//! sidechain is one new file -- this one -- plus one line in the registry:
//! its key is one of its `MODS`, so the host routes another channel into it
//! (`Core::route`), and the test kit probes it with no test written for it.
//! subDuck (item 68) is this, on the bass channel, keyed by the drums.
//! `core/README.md` walks through it.

use crate::block::{Block, Channel, Context, Mod, Mods, Probe, Signal};
use crate::math;

/// How a duck is set: how far it presses the channel down at full key.
#[derive(Clone, Copy)]
pub struct Settings {
    /// The deepest it goes, as a share of the gain taken away: 0 to 1.
    pub depth: f32,
}

/// At rest: half the gain at full key (nothing, with no key routed in).
pub const REST: Settings = Settings { depth: 0.5 };

/// Its one setting a signal drives: the key.
pub const KEY: usize = 0;

/// How fast it follows the key down, and lets go, as time constants in
/// seconds.
const ATTACK: f64 = 0.005;
const RELEASE: f64 = 0.15;

/// The duck: its depth, and how loud the key has been.
pub struct Duck {
    depth: f32,
    level: f32,
    attack: f32,
    release: f32,
}

impl Duck {
    /// A duck at rest, all zeros.
    #[must_use]
    pub const fn new() -> Self {
        Duck { depth: 0.0, level: 0.0, attack: 0.0, release: 0.0 }
    }
}

impl Default for Duck {
    fn default() -> Self {
        Self::new()
    }
}

impl Block for Duck {
    type Settings = Settings;

    const PROBES: &'static [Probe<Settings>] = &[Probe { name: "half", settings: Settings { depth: 0.5 } }];

    const INPUT: bool = true;

    // The kit keys it with a sine of 0.5 (a fairly loud key).
    const MODS: &'static [Mod] = &[Mod { name: "key", swing: 0.5 }];

    // One-pole coefficients, 32-bit as the gain they shape.
    #[allow(clippy::cast_possible_truncation)]
    fn start(&mut self, s: &Settings, cx: &Context) {
        self.depth = s.depth;
        self.level = 0.0;
        self.attack = (1.0 - math::exp(-1.0 / (ATTACK * cx.rate))) as f32;
        self.release = (1.0 - math::exp(-1.0 / (RELEASE * cx.rate))) as f32;
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    fn render(&mut self, block: u64, cx: &Context, signal: &mut Signal) {
        self.render_mod(block, cx, signal, &Mods::NONE);
    }

    /// With no key routed in, the channel passes as it is.
    fn render_mod(&mut self, _: u64, _: &Context, signal: &mut Signal, mods: &Mods) {
        let Some(key) = mods.get(KEY) else { return };
        for (x, k) in signal.iter_mut().zip(key.iter()) {
            let k = k.abs().min(1.0);
            let speed = if k > self.level { self.attack } else { self.release };
            self.level += (k - self.level) * speed;
            *x *= 1.0 - self.depth * self.level;
        }
    }
}

// It presses down everything its channel plays.
impl Channel for Duck {
    const WHOLE: bool = true;
}
