//! An amplifier whose gain a signal can drive.
//!
//! The first of queue item 46b's two demonstration blocks, in the tests only
//! (its line in the registry carries `#[cfg(test)]`). It proves that a
//! block taking a modulation input is still one new file -- this one --
//! plus one line in the registry: it says which of its settings a signal
//! can drive (`MODS`), and the test kit finds it there and probes that
//! input, with no test written for it. `core/README.md` walks through it.

use crate::block::{Block, Context, Mod, Mods, Probe, Signal};

/// How an amplifier is set: its gain.
#[derive(Clone, Copy)]
pub struct Settings {
    /// The gain, as a factor: 1 leaves the signal as it is.
    pub gain: f32,
}

/// Its one setting a signal can drive: the gain. An LFO into it is a
/// tremolo; an envelope into it, a note's shape.
pub const GAIN: usize = 0;

/// The amplifier's state: only its gain.
pub struct Amp {
    gain: f32,
}

impl Amp {
    /// An amplifier at zero gain, all zeros.
    #[must_use]
    pub const fn new() -> Self {
        Amp { gain: 0.0 }
    }
}

impl Default for Amp {
    fn default() -> Self {
        Self::new()
    }
}

impl Block for Amp {
    type Settings = Settings;

    const PROBES: &'static [Probe<Settings>] = &[Probe { name: "half", settings: Settings { gain: 0.5 } }];

    // It works on the signal it is given.
    const INPUT: bool = true;

    // The gain swings by 0.4 either side when the kit drives it.
    const MODS: &'static [Mod] = &[Mod { name: "gain", swing: 0.4 }];

    fn start(&mut self, s: &Settings, _: &Context) {
        self.gain = s.gain;
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    fn render(&mut self, block: u64, cx: &Context, signal: &mut Signal) {
        self.render_mod(block, cx, signal, &Mods::NONE);
    }

    fn render_mod(&mut self, _: u64, _: &Context, signal: &mut Signal, mods: &Mods) {
        match mods.get(GAIN) {
            // The gain, plus the signal driving it, frame by frame.
            Some(m) => {
                for (x, g) in signal.iter_mut().zip(m.iter()) {
                    *x *= self.gain + *g;
                }
            }
            None => {
                for x in signal.iter_mut() {
                    *x *= self.gain;
                }
            }
        }
    }
}
