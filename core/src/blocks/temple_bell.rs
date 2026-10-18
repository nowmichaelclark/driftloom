//! Temple bell: four pairs of beating sines and a strike of noise
//! (`voice('templebell')`).
//!
//! The note's extra values are draws: the eight partials' detunes, in
//! partial order, then the strike's rate and offset. It plays the noise.

// Web Audio's parameters are 32-bit floats, set from the host's 64-bit
// numbers: a voice narrows every value it hands one.
#![allow(clippy::cast_possible_truncation)]

use crate::QUANTUM;
use crate::block::{Block, Context, NO_EXTRA, Note, Probe, Signal, Voice};
use crate::math;
use crate::param::Param;
use crate::rng::salt;

use super::filter::{Biquad, Kind};
use super::noise_source::NoiseSource;
use super::osc::{Pitch, TableOsc, frame_at};
use super::shared::{BELL_PARTIAL_VEL, QUIET, UNITY, midi_to_freq, noise_grain, release, through};

/// The temple bell's lift as a chord voice.
fn chord_lift() -> f64 {
    math::pow(10.0, 1.9 / 20.0)
}
/// A bowl's partials: ratio to the note and relative level. Each is a pair
/// of sines 4 cents either side, which is what beats.
const TEMPLE_PARTIALS: [(f64, f64); 4] = [(1.0, 1.0), (2.02, 0.5), (2.76, 0.32), (5.4, 0.14)];
const TEMPLE_PAIR: [f64; 2] = [-4.0, 4.0];
/// How many sines the bell has: four partials, two each.
pub const TEMPLE_SINES: usize = 8;

/// Temple bell: four pairs of sines a few cents apart, each dying at its own
/// rate, and a bandpassed strike of noise, under one envelope that lets go
/// after at least six and a half seconds (`voice('templebell')`).
pub struct TempleBell {
    oscs: [TableOsc; TEMPLE_SINES],
    pitch: [f32; TEMPLE_SINES],
    detune: [f32; TEMPLE_SINES],
    gains: [Param; TEMPLE_SINES],
    envelope: Param,
    strike: NoiseSource,
    filter: Biquad,
    strike_gain: Param,
    end: u64,
}

impl TempleBell {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`;
    /// starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        TempleBell {
            oscs: [const { TableOsc::new() }; TEMPLE_SINES],
            pitch: [0.0; TEMPLE_SINES],
            detune: [0.0; TEMPLE_SINES],
            gains: [const { Param::new(0.0) }; TEMPLE_SINES],
            envelope: Param::new(0.0),
            strike: NoiseSource::new(),
            filter: Biquad::new(),
            strike_gain: Param::new(0.0),
            end: 0,
        }
    }

    /// `draws` are the eight detune draws, partial by partial, the lower of
    /// each pair first; then the strike's rate and offset draws. `chords`
    /// is whether it plays into the chords channel.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn play(
        &mut self,
        midi: f64,
        time: f64,
        dur: f64,
        vel: f64,
        chords: bool,
        draws: &[f64; TEMPLE_SINES],
        strike: (f64, f64),
        noise: usize,
        rate: f64,
    ) {
        let f = midi_to_freq(midi);
        let hold = dur.max(6.5);
        let stop = time + hold + 1.4;
        let g = &mut self.envelope;
        g.reset_at(QUIET, rate);
        g.set_value_at_time(0.0001, time);
        let lift = if chords { chord_lift() } else { 1.0 };
        g.exponential_ramp_to_value_at_time((vel * 0.3 * lift) as f32, time + 0.006, time);
        release(g, time + 0.02, stop, time);
        let mut n = 0;
        for &(ratio, amp) in &TEMPLE_PARTIALS {
            for &cents in &TEMPLE_PAIR {
                if let (Some(p), Some(d), Some(pg), Some(osc), Some(&draw)) = (
                    self.pitch.get_mut(n),
                    self.detune.get_mut(n),
                    self.gains.get_mut(n),
                    self.oscs.get_mut(n),
                    draws.get(n),
                ) {
                    *p = (f * ratio) as f32;
                    *d = (cents + (draw - 0.5) * 3.0) as f32;
                    pg.reset_at(UNITY, rate);
                    pg.set_value_at_time((BELL_PARTIAL_VEL * amp * 0.5) as f32, time);
                    pg.exponential_ramp_to_value_at_time(
                        0.0001,
                        time + hold / (0.55 + ratio * 0.3),
                        time,
                    );
                    osc.schedule(time, stop + 0.01, rate);
                }
                n += 1;
            }
        }
        noise_grain(&mut self.strike, time, 0.03, strike, noise, rate);
        self.filter.reset();
        self.filter
            .set(Kind::Bandpass, (f * 6.0) as f32, 1.2, 0.0, rate as f32);
        let s = &mut self.strike_gain;
        s.reset_at((BELL_PARTIAL_VEL * 0.18) as f32, rate);
        s.set_value_at_time((BELL_PARTIAL_VEL * 0.18) as f32, time);
        s.exponential_ramp_to_value_at_time(0.0001, time + 0.04, time);
        self.end = frame_at(stop + 0.01, rate);
    }


    fn start_frame(&self) -> u64 {
        self.oscs[0].start_frame()
    }

    fn end_frame(&self) -> u64 {
        self.end
    }
}

impl Block for TempleBell {
    type Settings = Note;

    const PROBES: &'static [Probe<Note>] = &[
        Probe { name: "melody", settings: Note::probe(crate::TEMPLEBELL, crate::MELODY, 67.0, 0, NO_EXTRA) },
        Probe { name: "chords", settings: Note::probe(crate::TEMPLEBELL, crate::CHORDS, 55.0, 0, NO_EXTRA) },
    ];

    fn start(&mut self, note: &Note, cx: &Context) {
        let mut draws = [0.0; TEMPLE_SINES];
        for (d, &x) in draws.iter_mut().zip(note.extra.iter()) {
            *d = x;
        }
        let strike = (note.extra[TEMPLE_SINES], note.extra[TEMPLE_SINES + 1]);
        let chords = note.channel == crate::CHORDS;
        let (noise, rate) = (cx.noise.len(), cx.rate);
        self.play(note.midi, note.time, note.dur, note.vel, chords, &draws, strike, noise, rate);
    }

    fn span(&self) -> (u64, u64) {
        (self.start_frame(), self.end_frame())
    }

    fn render(&mut self, block: u64, cx: &Context, out: &mut Signal) {
        let (rate, waves, noise) = (cx.rate, cx.waves, cx.noise);
        let r = rate as f32;
        let mut sum = [0.0f32; QUANTUM];
        let mut wave = [0.0f32; QUANTUM];
        let mut gain = [0.0f32; QUANTUM];
        for ((osc, pg), (&freq, &detune)) in self
            .oscs
            .iter_mut()
            .zip(self.gains.iter())
            .zip(self.pitch.iter().zip(self.detune.iter()))
        {
            let pitch = Pitch::Steady { freq, detune };
            let (lo, hi) = osc.render(block, &waves.sine, &pitch, r, &mut wave);
            pg.fill(block, rate, &mut gain);
            for ((s, w), g) in sum
                .iter_mut()
                .zip(wave.iter())
                .zip(gain.iter())
                .take(hi)
                .skip(lo)
            {
                *s += *w * *g;
            }
        }
        let mut x = [0.0f32; QUANTUM];
        self.strike.fill(block, noise, &mut x);
        self.strike_gain.fill(block, rate, &mut gain);
        through(&x, &mut self.filter, &gain, &mut sum);
        let mut envelope = [0.0f32; QUANTUM];
        self.envelope.fill(block, rate, &mut envelope);
        for ((y, x), g) in out.iter_mut().zip(sum.iter()).zip(envelope.iter()) {
            *y += *x * *g;
        }
    }
}

impl Voice for TempleBell {
    /// The eight partials' detunes, in partial order, then the strike's
    /// rate and offset.
    fn settle(note: &mut Note) {
        let d = note.draws;
        for (i, x) in (0_u32..).zip(note.extra.iter_mut().take(TEMPLE_SINES)) {
            *x = d.unit(salt::TEMPLE_DETUNE, i);
        }
        note.extra[TEMPLE_SINES] = d.unit(salt::GRAIN, 0);
        note.extra[TEMPLE_SINES + 1] = d.unit(salt::GRAIN, 1);
    }

    fn uses_noise(_: &Note) -> bool {
        true
    }
}

impl Default for TempleBell {
    fn default() -> Self {
        Self::new()
    }
}
