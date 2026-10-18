//! A grain of the noise through a filter under a gain: the prepared
//! piano's knock (`KNOCK`) and the hat's graph (`HAT`, whose note's
//! `parts` says which: `HAT`, `OPEN_HAT` or `SHAKER`).
//!
//! The note's extra values are draws: the grain's rate and offset, and the
//! knock's band. It plays the noise.

// Web Audio's parameters are 32-bit floats, set from the host's 64-bit
// numbers: a voice narrows every value it hands one.
#![allow(clippy::cast_possible_truncation)]

use crate::QUANTUM;
use crate::block::{Block, Context, NO_EXTRA, Note, Probe, Signal, Voice};
use crate::param::Param;
use crate::rng::salt;

use super::filter::{Biquad, Kind};
use super::noise_source::NoiseSource;
use super::osc::frame_at;
use super::shared::{FILTER_TAIL, QUIET, Q_DEFAULT, noise_grain, through};

/// A grain of noise through a filter under a gain: the prepared piano's
/// knock and the hat's graph.
pub struct Struck {
    src: NoiseSource,
    filter: Biquad,
    gain: Param,
    end: u64,
}

/// The hat's kinds, by `parts`: hat, open hat, shaker.
pub const HAT: u32 = 0;
/// The open hat, which rings longer.
pub const OPEN_HAT: u32 = 1;
/// The shaker.
pub const SHAKER: u32 = 2;

impl Struck {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`;
    /// starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        Struck {
            src: NoiseSource::new(),
            filter: Biquad::new(),
            gain: Param::new(0.0),
            end: 0,
        }
    }

    /// The prepared piano's knock: `_noiseSource(time, 0.05)` (`rate_draw`,
    /// `offset_draw`) through a bandpass at a drawn frequency
    /// (`freq_draw`), struck at `vel * 0.13` and gone in 50 ms.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn knock(
        &mut self,
        time: f64,
        vel: f64,
        rate_draw: f64,
        offset_draw: f64,
        freq_draw: f64,
        noise: usize,
        rate: f64,
    ) {
        noise_grain(&mut self.src, time, 0.05, (rate_draw, offset_draw), noise, rate);
        self.filter.reset();
        self.filter.set(
            Kind::Bandpass,
            (220.0 + freq_draw * 180.0) as f32,
            3.5,
            0.0,
            rate as f32,
        );
        let g = &mut self.gain;
        g.reset_at((vel * 0.13) as f32, rate);
        g.set_value_at_time((vel * 0.13) as f32, time);
        g.exponential_ramp_to_value_at_time(0.0001, time + 0.05, time);
        self.end = self.src.end_frame() + frame_at(FILTER_TAIL, rate);
    }

    /// The hat (`drum('hat' | 'ohat' | 'shaker')`): a grain of noise
    /// through a highpass, a fast strike and an exponential fall.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn hat(
        &mut self,
        kind: u32,
        time: f64,
        vel: f64,
        rate_draw: f64,
        offset_draw: f64,
        noise: usize,
        rate: f64,
    ) {
        let v = vel.clamp(0.0, 1.0);
        let dur = match kind {
            OPEN_HAT => 0.26,
            SHAKER => 0.07,
            _ => 0.045,
        };
        noise_grain(&mut self.src, time, dur, (rate_draw, offset_draw), noise, rate);
        self.filter.reset();
        let cutoff = if kind == SHAKER { 5200.0 } else { 7400.0 };
        self.filter
            .set(Kind::Highpass, cutoff, Q_DEFAULT, 0.0, rate as f32);
        let g = &mut self.gain;
        g.reset_at(QUIET, rate);
        g.set_value_at_time(0.0001, time);
        g.exponential_ramp_to_value_at_time(
            (v * if kind == SHAKER { 0.3 } else { 0.42 }) as f32,
            time + 0.003,
            time,
        );
        g.exponential_ramp_to_value_at_time(0.0001, time + dur, time);
        self.end = self.src.end_frame() + frame_at(FILTER_TAIL, rate);
    }


    fn start_frame(&self) -> u64 {
        self.src.start_frame()
    }

    fn end_frame(&self) -> u64 {
        self.end
    }
}

impl Block for Struck {
    type Settings = Note;

    const PROBES: &'static [Probe<Note>] = &[
        Probe { name: "knock", settings: Note::probe(crate::KNOCK, crate::MELODY, 67.0, 0, NO_EXTRA) },
        Probe { name: "hat", settings: Note::probe(crate::HAT, crate::DRUMS_CHANNEL, 60.0, HAT, NO_EXTRA) },
        Probe { name: "open-hat", settings: Note::probe(crate::HAT, crate::DRUMS_CHANNEL, 60.0, OPEN_HAT, NO_EXTRA) },
        Probe { name: "shaker", settings: Note::probe(crate::HAT, crate::DRUMS_CHANNEL, 60.0, SHAKER, NO_EXTRA) },
    ];

    fn start(&mut self, note: &Note, cx: &Context) {
        let [a, b, c, ..] = note.extra;
        let (noise, rate) = (cx.noise.len(), cx.rate);
        if note.voice == crate::KNOCK {
            self.knock(note.time, note.vel, a, b, c, noise, rate);
        } else {
            self.hat(note.parts, note.time, note.vel, a, b, noise, rate);
        }
    }

    fn span(&self) -> (u64, u64) {
        (self.start_frame(), self.end_frame())
    }

    fn render(&mut self, block: u64, cx: &Context, out: &mut Signal) {
        let (rate, noise) = (cx.rate, cx.noise);
        let mut x = [0.0f32; QUANTUM];
        self.src.fill(block, noise, &mut x);
        let mut gain = [0.0f32; QUANTUM];
        self.gain.fill(block, rate, &mut gain);
        through(&x, &mut self.filter, &gain, out);
    }
}

impl Voice for Struck {
    /// Its grain's rate and offset, and the knock's band.
    fn settle(note: &mut Note) {
        let n = if note.voice == crate::KNOCK { 3 } else { 2 };
        let d = note.draws;
        for (i, x) in (0_u32..n).zip(note.extra.iter_mut()) {
            *x = d.unit(salt::GRAIN, i);
        }
    }

    fn uses_noise(_: &Note) -> bool {
        true
    }
}

impl Default for Struck {
    fn default() -> Self {
        Self::new()
    }
}
