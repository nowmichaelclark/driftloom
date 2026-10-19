//! A bass note's soft harmonics (queue item 56, roundBass): its 2nd, 3rd
//! and 4th partials as sines, under a 900 Hz lowpass and an envelope of
//! their own, so a bass note reads on a phone whose speaker does not play
//! its fundamental. Every bass voice but `rhodesbass` (an FM voice) adds
//! one, the same block for all of them, as the prototype Mikey approved
//! (2026-10-10; the `ears/prototypes` branch, `bass()`) adds one node chain.

// Web Audio's parameters are 32-bit floats, set from the host's 64-bit
// numbers.
#![allow(clippy::cast_possible_truncation)]

use crate::QUANTUM;
use crate::math;
use crate::block::{Block, Context, Probe, Signal};
use crate::param::Param;

use super::filter::{Biquad, Kind};
use super::osc::frame_at;
use super::shared::{FILTER_TAIL, UNITY, midi_to_freq};

/// The partials, as multiples of the note's frequency (the 2nd, 3rd and 4th,
/// which `render` makes from the note's phase), and each one's level
/// (the prototype's `[[2, 1], [3, 0.45], [4, 0.18]]`).
const PARTIALS: [(f64, f32); 3] = [(2.0, 1.0), (3.0, 0.45), (4.0, 0.18)];
/// The lowpass over them: 900 Hz, its Q in dB (the prototype's 0.5).
const CUTOFF: f32 = 900.0;
const Q: f32 = 0.5;

/// How a note's harmonics are set: as their bass note is.
#[derive(Clone, Copy)]
pub struct Settings {
    /// The note's pitch, as a MIDI note number.
    pub midi: f64,
    /// When it starts and how long it is held, in seconds.
    pub time: f64,
    /// How long it is held.
    pub dur: f64,
    /// How hard, after the bass voice's own trim.
    pub vel: f64,
    /// The release they let go on, as a time constant in seconds (the
    /// prototype's `letGo(0.09)`: shorter for a chug).
    pub release: f64,
    /// When the sines stop, in seconds (the bass note's own stop).
    pub stop: f64,
}

/// The harmonics: one phasor at the note's frequency, from which the three
/// partials are made (`render`), the lowpass they share, their envelope.
///
/// The partials are not three oscillators: the note's sine and cosine are
/// turned a step each frame (a rotation, in 64-bit floats, put back on the
/// unit circle each block), and the 2nd, 3rd and 4th partials follow from
/// them by the multiple-angle identities: a third of the work of three
/// table oscillators, and plain arithmetic, so the same on every target.
pub struct Harmonics {
    /// The note's phase as a point on the unit circle, and one frame's turn.
    sin: f64,
    cos: f64,
    step_sin: f64,
    step_cos: f64,
    filter: Biquad,
    gain: Param,
    /// The frames the partials sound in, and the voice's whole span (their
    /// filter rings on).
    on: (u64, u64),
    start: u64,
    end: u64,
}

impl Harmonics {
    /// Silent and all zeros.
    #[must_use]
    pub const fn new() -> Self {
        Harmonics {
            sin: 0.0,
            cos: 0.0,
            step_sin: 0.0,
            step_cos: 0.0,
            filter: Biquad::new(),
            gain: Param::new(0.0),
            on: (0, 0),
            start: 0,
            end: 0,
        }
    }

    /// Set them for a note at `rate` Hz: `Block::start` for a voice that
    /// has a rate rather than a `Context`.
    pub fn set(&mut self, s: &Settings, rate: f64) {
        let turn = core::f64::consts::TAU * midi_to_freq(s.midi) / rate;
        (self.sin, self.cos) = (0.0, 1.0);
        (self.step_sin, self.step_cos) = (math::sin(turn), math::cos(turn));
        self.filter.reset();
        self.filter.set(Kind::Lowpass, CUTOFF, Q, 0.0, rate as f32);
        let (time, vel) = (s.time, s.vel);
        let g = &mut self.gain;
        g.reset_at(UNITY, rate);
        g.set_value_at_time(0.0001, time);
        g.exponential_ramp_to_value_at_time((vel * 0.3).max(0.001) as f32, time + 0.02, time);
        g.set_target_at_time((vel * 0.2) as f32, time + 0.06, 0.3);
        g.set_target_at_time(0.0001, time + s.dur, s.release);
        self.on = (frame_at(s.time, rate), frame_at(s.stop, rate));
        self.start = self.on.0;
        self.end = self.on.1 + frame_at(FILTER_TAIL, rate);
    }
}

impl Default for Harmonics {
    fn default() -> Self {
        Self::new()
    }
}

impl Block for Harmonics {
    type Settings = Settings;

    const PROBES: &'static [Probe<Settings>] = &[Probe {
        name: "e2",
        settings: Settings { midi: 40.0, time: 0.05, dur: 0.4, vel: 0.8 * 0.55, release: 0.09, stop: 0.85 },
    }];

    fn start(&mut self, s: &Settings, cx: &Context) {
        self.set(s, cx.rate);
    }

    fn span(&self) -> (u64, u64) {
        (self.start, self.end)
    }

    /// The three partials, summed, through the lowpass (which runs every
    /// frame, as a node does, and rings on), under the envelope, added into
    /// `signal`.
    fn render(&mut self, block: u64, cx: &Context, signal: &mut Signal) {
        let mut sum = [0.0f32; QUANTUM];
        let [(_, l2), (_, l3), (_, l4)] = PARTIALS;
        let (l2, l3, l4) = (f64::from(l2), f64::from(l3), f64::from(l4));
        for (frame, y) in (block..).zip(sum.iter_mut()) {
            if frame < self.on.0 || frame >= self.on.1 {
                continue;
            }
            let (s1, c1) = (self.sin, self.cos);
            // sin 2x, cos 2x, sin 3x, sin 4x.
            let s2 = 2.0 * s1 * c1;
            let c2 = 1.0 - 2.0 * s1 * s1;
            let s3 = s2 * c1 + c2 * s1;
            let s4 = 2.0 * s2 * c2;
            *y = (l2 * s2 + l3 * s3 + l4 * s4) as f32;
            self.sin = s1 * self.step_cos + c1 * self.step_sin;
            self.cos = c1 * self.step_cos - s1 * self.step_sin;
        }
        // Back on the unit circle, so a long note keeps its level.
        let r = math::sqrt(self.sin * self.sin + self.cos * self.cos);
        if r > 0.0 {
            self.sin /= r;
            self.cos /= r;
        }
        let mut gain = [0.0f32; QUANTUM];
        self.gain.fill(block, cx.rate, &mut gain);
        for ((y, x), g) in signal.iter_mut().zip(sum.iter()).zip(gain.iter()) {
            *y += self.filter.step(*x) * *g;
        }
        self.filter.flush();
    }
}
