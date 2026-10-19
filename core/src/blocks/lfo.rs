//! A low-frequency oscillator: a sine at a rate, through a depth. The
//! signal every vibrato drives an oscillator's detune with, the texture
//! wind's slow sine drives its filter's band with and, at audio rate, the
//! modulator FM drives a carrier's frequency with (queue item 46b): a
//! source of signals for other blocks' `MODS`.
//!
//! As Web Audio builds one: an `OscillatorNode` (a sine; its frequency is
//! the rate) into a `GainNode` (its gain is the depth). Both are `Param`s,
//! since the fiddle's rate and every vibrato's depth ramp; `N` is how many
//! events each holds, so a steady one (`N = 0`) pays nothing for them. The
//! depth is first rendered when the sine starts, so its events are clamped
//! to that block (`Param::clamp_before`), as the oscillator's are.

use crate::QUANTUM;
use crate::block::{Block, Context, Probe, Signal};
use crate::param::{MAX_EVENTS, Param};

use super::osc::{Oscillator, Shape};
use super::shared::starts_in;

/// How an LFO is set: its rate and depth (their values before any
/// automation), and when it starts and stops.
#[derive(Clone, Copy)]
pub struct Settings {
    /// Swings a second, in Hz.
    pub rate: f32,
    /// How far it swings, in the unit of the setting it drives (cents for a
    /// detune, Hz for a frequency).
    pub depth: f32,
    /// When it starts, in seconds.
    pub start: f64,
    /// When it stops, in seconds.
    pub stop: f64,
}

/// The LFO: its sine and its depth.
pub struct Lfo<const N: usize = MAX_EVENTS> {
    sine: Oscillator<N>,
    depth: Param<N>,
}

impl<const N: usize> Lfo<N> {
    /// A still LFO, all zeros.
    #[must_use]
    pub const fn new() -> Self {
        Lfo { sine: Oscillator::new(), depth: Param::new(0.0) }
    }

    /// Set it to swing `rate` times a second by `depth`, from `start` to
    /// `stop` seconds, at `rate_hz`: `Block::start` with the numbers, for a
    /// voice that has a rate rather than a `Context`.
    pub fn set(&mut self, rate: f32, depth: f32, start: f64, stop: f64, rate_hz: f64) {
        self.sine.set(Shape::Sine, rate, 0.0, start, stop, rate_hz);
        self.depth.reset(depth);
    }

    /// Its rate, to automate after `set`.
    pub fn rate(&mut self) -> &mut Param<N> {
        self.sine.freq()
    }

    /// Its depth, to automate after `set`.
    pub fn depth(&mut self) -> &mut Param<N> {
        &mut self.depth
    }

    /// The first frame it swings in.
    #[must_use]
    pub fn start_frame(&self) -> u64 {
        self.sine.start_frame()
    }
}

impl<const N: usize> Default for Lfo<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Block for Lfo<N> {
    type Settings = Settings;

    const PROBES: &'static [Probe<Settings>] = &[
        Probe { name: "vibrato", settings: Settings { rate: 5.5, depth: 0.5, start: 0.05, stop: 1.05 } },
        Probe { name: "fm", settings: Settings { rate: 440.0, depth: 0.25, start: 0.05, stop: 0.55 } },
    ];

    fn start(&mut self, s: &Settings, cx: &Context) {
        self.set(s.rate, s.depth, s.start, s.stop, cx.rate);
    }

    fn span(&self) -> (u64, u64) {
        self.sine.span()
    }

    /// Add the sine through its depth into `signal`.
    fn render(&mut self, block: u64, cx: &Context, signal: &mut Signal) {
        let first = starts_in(self.sine.start_frame(), block);
        let mut wave = [0.0f32; QUANTUM];
        self.sine.render(block, cx, &mut wave);
        let (lo, hi) = self.sine.played();
        if first {
            self.depth.clamp_before(block, cx.rate);
        }
        let (Some(y), Some(w)) = (signal.get_mut(lo..hi), wave.get(lo..hi)) else {
            return;
        };
        if let Some(d) = self.depth.steady() {
            for (y, w) in y.iter_mut().zip(w.iter()) {
                *y += *w * d;
            }
        } else {
            let mut depth = [0.0f32; QUANTUM];
            self.depth.fill(block, cx.rate, &mut depth);
            for ((y, w), d) in y.iter_mut().zip(w.iter()).zip(depth.get(lo..hi).unwrap_or(&[])) {
                *y += *w * *d;
            }
        }
    }
}
