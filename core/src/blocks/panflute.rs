//! Pan flute: the `pipe` wave, a vibrato on long notes and a breath with a
//! chiff (`voice('panflute')`).
//!
//! The note's extra values are two draws: the vibrato's rate (none for a
//! note under half a second) and the breath's rate. It plays the noise.

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
use super::osc::{Part, Pitch, TableOsc, frame_at};
use super::shared::{
    A440, FILTER_TAIL, QUIET, UNITY, VIBRATO_MIN_DUR, breath_rate, given, midi_to_freq, pitch_of,
    starts_in, through,
};

/// `PANFLUTE_LEVEL`, the breath and the chiff as shares of it, and the dip
/// in cents the pipe speaks from.
const PANFLUTE_LEVEL: f64 = 0.1011;
const PANFLUTE_BREATH: f64 = 0.5;
const PANFLUTE_CHIFF: f64 = 0.8;
const PANFLUTE_DIP: f64 = 30.0;

/// Pan flute: the `pipe` wave, a little flat and rising onto the note, a
/// vibrato on long notes, and a looped breath through a bandpass that spikes
/// as the pipe catches -- the chiff -- and settles (`voice('panflute')`).
pub struct Panflute {
    osc: TableOsc,
    pitch: Param,
    vibrato: bool,
    lfo: TableOsc,
    lfo_rate: f32,
    depth: Param,
    gain: Param,
    air: NoiseSource,
    filter: Biquad,
    air_gain: Param,
    end: u64,
}

impl Panflute {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`;
    /// starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        Panflute {
            osc: TableOsc::new(),
            pitch: Param::new(0.0),
            vibrato: false,
            lfo: TableOsc::new(),
            lfo_rate: 0.0,
            depth: Param::new(0.0),
            gain: Param::new(0.0),
            air: NoiseSource::new(),
            filter: Biquad::new(),
            air_gain: Param::new(0.0),
            end: 0,
        }
    }

    /// `vibrato` is the vibrato's rate draw, for a note long enough to have
    /// one; `breath`, the breath's rate draw.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn play(
        &mut self,
        midi: f64,
        time: f64,
        dur: f64,
        vel: f64,
        vibrato: Option<f64>,
        breath: f64,
        noise: usize,
        rate: f64,
    ) {
        let f = midi_to_freq(midi);
        let level = vel * PANFLUTE_LEVEL;
        let p = &mut self.pitch;
        p.reset_at(A440, rate);
        p.set_value_at_time((f * math::pow(2.0, -PANFLUTE_DIP / 1200.0)) as f32, time);
        p.exponential_ramp_to_value_at_time(f as f32, time + 0.05, time);
        let stop = time + dur + 0.4;
        self.vibrato = vibrato.is_some();
        if let Some(draw) = vibrato {
            self.lfo_rate = (4.8 + draw * 0.8) as f32;
            let onset = (dur * 0.3).min(0.3);
            let d = &mut self.depth;
            d.reset_at(UNITY, rate);
            d.set_value_at_time(0.0, time);
            d.set_value_at_time(0.0, time + onset);
            d.linear_ramp_to_value_at_time(8.0, time + onset + 0.3, time);
            self.lfo.schedule(time, stop, rate);
        }
        let let_go = time + dur * 0.85;
        let g = &mut self.gain;
        g.reset_at(UNITY, rate);
        g.set_value_at_time(0.0001, time);
        g.linear_ramp_to_value_at_time(level as f32, (time + 0.03).min(let_go), time);
        g.set_target_at_time(0.0001, let_go, 0.08);
        self.air
            .looped(time, stop, breath_rate(breath), noise, rate);
        self.filter.reset();
        self.filter
            .set(Kind::Bandpass, (f * 1.5) as f32, 0.9, 0.0, rate as f32);
        let a = &mut self.air_gain;
        a.reset_at(QUIET, rate);
        a.set_value_at_time(0.0001, time);
        a.exponential_ramp_to_value_at_time(
            (level * PANFLUTE_CHIFF) as f32,
            (time + 0.004).min(let_go),
            time,
        );
        a.exponential_ramp_to_value_at_time(
            (level * PANFLUTE_BREATH) as f32,
            (time + 0.03).min(let_go),
            time,
        );
        a.set_target_at_time(0.0001, let_go, 0.08);
        self.osc.schedule(time, stop, rate);
        self.end = frame_at(stop + FILTER_TAIL, rate);
    }


    fn start_frame(&self) -> u64 {
        self.osc.start_frame()
    }

    fn end_frame(&self) -> u64 {
        self.end
    }
}

impl Block for Panflute {
    type Settings = Note;

    const PROBES: &'static [Probe<Note>] = &[
        Probe { name: "panflute", settings: Note::probe(crate::PANFLUTE, crate::MELODY, 67.0, 0, NO_EXTRA) },
    ];

    fn start(&mut self, note: &Note, cx: &Context) {
        let [vibrato, breath, ..] = note.extra;
        let (noise, rate) = (cx.noise.len(), cx.rate);
        self.play(note.midi, note.time, note.dur, note.vel, given(vibrato), breath, noise, rate);
    }

    fn span(&self) -> (u64, u64) {
        (self.start_frame(), self.end_frame())
    }

    fn render(&mut self, block: u64, cx: &Context, out: &mut Signal) {
        let (rate, waves, noise) = (cx.rate, cx.waves, cx.noise);
        let r = rate as f32;
        let mut cents = [0.0f32; QUANTUM];
        if self.vibrato {
            let steady = Pitch::Steady {
                freq: self.lfo_rate,
                detune: 0.0,
            };
            let mut sine = [0.0f32; QUANTUM];
            let (lo, hi) = self.lfo.render(block, &waves.sine, &steady, r, &mut sine);
            if starts_in(self.lfo.start_frame(), block) {
                self.depth.clamp_before(block, rate);
            }
            let mut depth = [0.0f32; QUANTUM];
            self.depth.fill(block, rate, &mut depth);
            for ((c, w), d) in cents
                .iter_mut()
                .zip(sine.iter())
                .zip(depth.iter())
                .take(hi)
                .skip(lo)
            {
                *c = *w * *d;
            }
        }
        let detune = if self.vibrato {
            Part::Moving(&cents)
        } else {
            Part::Steady(0.0)
        };
        let mut freq = [0.0f32; QUANTUM];
        let first = starts_in(self.osc.start_frame(), block);
        let pitch = pitch_of(&mut self.pitch, block, rate, first, &mut freq, detune);
        let mut tone = [0.0f32; QUANTUM];
        let (lo, hi) = self.osc.render(block, &waves.pipe, &pitch, r, &mut tone);
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

impl Voice for Panflute {
    /// The vibrato's rate (none for a note under half a second) and the
    /// breath's.
    fn settle(note: &mut Note) {
        let d = note.draws;
        note.extra[0] = if note.dur >= VIBRATO_MIN_DUR { d.unit(salt::PAN_VIBRATO, 0) } else { f64::NAN };
        note.extra[1] = d.unit(salt::BREATH_RATE, 0);
    }

    fn uses_noise(_: &Note) -> bool {
        true
    }
}

impl Default for Panflute {
    fn default() -> Self {
        Self::new()
    }
}
