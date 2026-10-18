//! Ocarina and flute: a nearly pure tone, a late vibrato and a looped
//! breath of noise (`voice('ocarina' | 'flute')`).
//!
//! The note's first extra value is the note it is joined to (NaN if none).
//! Whether it slides, from where and how long it takes, are drawn into the
//! first two, and the breath's rate into the third. It plays the noise.

// Web Audio's parameters are 32-bit floats, set from the host's 64-bit
// numbers: a voice narrows every value it hands one.
#![allow(clippy::cast_possible_truncation)]
// The names follow synth.js's (f, g, c...), so the port reads against it.
#![allow(clippy::many_single_char_names)]

use crate::QUANTUM;
use crate::block::{Block, Context, NO_EXTRA, Note, Probe, Signal, Voice};
use crate::param::Param;
use crate::rng::salt;

use super::filter::{Biquad, Kind};
use super::noise_source::NoiseSource;
use super::osc::{Part, Pitch, TableOsc, frame_at};
use super::shared::{
    A440, FILTER_TAIL, QUIET, UNITY, breath_rate, midi_to_freq, pitch_of, settle_slide, starts_in,
    through,
};

/// The ocarina's and the flute's levels (tone and breath scale together).
const OCARINA_LEVEL: f64 = 0.1115;
const FLUTE_LEVEL: f64 = 0.1357;
/// Their vibrato's rate, in Hz.
const WIND_VIBRATO: f32 = 5.2;

/// Ocarina and flute: a nearly pure tone (a sine, or the flute's triangle)
/// that may slide into the note, a vibrato on the detune that arrives late,
/// and a looped breath through a bandpass (`voice('ocarina' | 'flute')`).
pub struct Wind {
    flute: bool,
    osc: TableOsc,
    pitch: Param,
    lfo: TableOsc,
    depth: Param,
    gain: Param,
    air: NoiseSource,
    filter: Biquad,
    air_gain: Param,
    end: u64,
}

impl Wind {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`;
    /// starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        Wind {
            flute: false,
            osc: TableOsc::new(),
            pitch: Param::new(0.0),
            lfo: TableOsc::new(),
            depth: Param::new(0.0),
            gain: Param::new(0.0),
            air: NoiseSource::new(),
            filter: Biquad::new(),
            air_gain: Param::new(0.0),
            end: 0,
        }
    }

    /// `slide` is where the pitch comes from in Hz and how long it takes to
    /// arrive, if the note slides (`_slideOf`); `breath` is the breath's
    /// rate draw.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn play(
        &mut self,
        flute: bool,
        midi: f64,
        time: f64,
        dur: f64,
        vel: f64,
        slide: Option<(f64, f64)>,
        breath: f64,
        noise: usize,
        rate: f64,
    ) {
        self.flute = flute;
        let f = midi_to_freq(midi);
        let level = if flute { FLUTE_LEVEL } else { OCARINA_LEVEL };
        let p = &mut self.pitch;
        match slide {
            Some((from, reach)) => {
                p.reset_at(A440, rate);
                p.set_value_at_time(from as f32, time);
                p.exponential_ramp_to_value_at_time(f as f32, time + reach, time);
            }
            None => p.reset_at(f as f32, rate),
        }
        let d = &mut self.depth;
        d.reset_at(UNITY, rate);
        d.set_value_at_time(0.0, time);
        d.linear_ramp_to_value_at_time(if flute { 9.0 } else { 6.0 }, time + dur.min(0.5), time);
        let let_go = time + dur * 0.8;
        let g = &mut self.gain;
        g.reset_at(UNITY, rate);
        g.set_value_at_time(0.0001, time);
        g.linear_ramp_to_value_at_time((vel * level) as f32, (time + 0.05).min(let_go), time);
        g.set_target_at_time(0.0001, let_go, 0.09);
        let air_stop = time + dur + 0.5;
        self.air
            .looped(time, air_stop, breath_rate(breath), noise, rate);
        self.filter.reset();
        self.filter
            .set(Kind::Bandpass, (f * 2.0) as f32, 1.2, 0.0, rate as f32);
        let a = &mut self.air_gain;
        a.reset_at(QUIET, rate);
        a.set_value_at_time(0.0001, time);
        let share = if flute { 1.0 / 3.0 } else { 0.15 };
        a.linear_ramp_to_value_at_time(
            (vel * level * share) as f32,
            (time + 0.06).min(let_go),
            time,
        );
        a.set_target_at_time(0.0001, let_go, 0.09);
        let stop = time + dur + 0.4;
        self.osc.schedule(time, stop, rate);
        self.lfo.schedule(time, stop, rate);
        self.end = frame_at(air_stop + FILTER_TAIL, rate);
    }


    fn start_frame(&self) -> u64 {
        self.osc.start_frame()
    }

    fn end_frame(&self) -> u64 {
        self.end
    }
}

impl Block for Wind {
    type Settings = Note;

    const PROBES: &'static [Probe<Note>] = &[
        Probe { name: "ocarina", settings: Note::probe(crate::OCARINA, crate::MELODY, 67.0, 0, NO_EXTRA) },
        Probe { name: "flute", settings: Note::probe(crate::FLUTE, crate::MELODY, 67.0, 0, NO_EXTRA) },
    ];

    fn start(&mut self, note: &Note, cx: &Context) {
        let [from, reach, breath, ..] = note.extra;
        let slide = if from.is_finite() && reach.is_finite() { Some((from, reach)) } else { None };
        let flute = note.voice == crate::FLUTE;
        let (noise, rate) = (cx.noise.len(), cx.rate);
        self.play(flute, note.midi, note.time, note.dur, note.vel, slide, breath, noise, rate);
    }

    fn span(&self) -> (u64, u64) {
        (self.start_frame(), self.end_frame())
    }

    fn render(&mut self, block: u64, cx: &Context, out: &mut Signal) {
        let (rate, waves, noise) = (cx.rate, cx.waves, cx.noise);
        let r = rate as f32;
        let steady = Pitch::Steady {
            freq: WIND_VIBRATO,
            detune: 0.0,
        };
        let mut sine = [0.0f32; QUANTUM];
        let (lo, hi) = self.lfo.render(block, &waves.sine, &steady, r, &mut sine);
        // Feeding only the detune, the depth is first rendered when the
        // note starts.
        if starts_in(self.lfo.start_frame(), block) {
            self.depth.clamp_before(block, rate);
        }
        let mut depth = [0.0f32; QUANTUM];
        self.depth.fill(block, rate, &mut depth);
        let mut cents = [0.0f32; QUANTUM];
        for ((c, w), d) in cents
            .iter_mut()
            .zip(sine.iter())
            .zip(depth.iter())
            .take(hi)
            .skip(lo)
        {
            *c = *w * *d;
        }
        let mut freq = [0.0f32; QUANTUM];
        let first = starts_in(self.osc.start_frame(), block);
        let pitch = pitch_of(
            &mut self.pitch,
            block,
            rate,
            first,
            &mut freq,
            Part::Moving(&cents),
        );
        let wave = if self.flute {
            &waves.triangle
        } else {
            &waves.sine
        };
        let mut tone = [0.0f32; QUANTUM];
        let (lo, hi) = self.osc.render(block, wave, &pitch, r, &mut tone);
        let mut gain = [0.0f32; QUANTUM];
        self.gain.fill(block, rate, &mut gain);
        for ((y, x), g) in out
            .iter_mut()
            .zip(tone.iter())
            .zip(gain.iter())
            .take(hi)
            .skip(lo)
        {
            *y += *x * *g;
        }
        let mut air = [0.0f32; QUANTUM];
        self.air.fill(block, noise, &mut air);
        let mut air_gain = [0.0f32; QUANTUM];
        self.air_gain.fill(block, rate, &mut air_gain);
        through(&air, &mut self.filter, &air_gain, out);
    }
}

impl Voice for Wind {
    /// Whether it slides into its pitch, from where and how long it takes,
    /// and the rate its breath plays at.
    fn settle(note: &mut Note) {
        settle_slide(note);
        note.extra[2] = note.draws.unit(salt::BREATH_RATE, 0);
    }

    fn uses_noise(_: &Note) -> bool {
        true
    }
}

impl Default for Wind {
    fn default() -> Self {
        Self::new()
    }
}
