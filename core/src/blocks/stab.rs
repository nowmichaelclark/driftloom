//! Stab: two detuned sawtooths through a bandpass (`voice('stab')`). It
//! reads no extra values and draws nothing.

// Web Audio's parameters are 32-bit floats, set from the host's 64-bit
// numbers: a voice narrows every value it hands one.
#![allow(clippy::cast_possible_truncation)]

use crate::QUANTUM;
use crate::block::{Block, Context, NO_EXTRA, Note, Probe, Signal, Voice};
use crate::param::Param;

use super::filter::{Biquad, Kind};
use super::osc::{Pitch, TableOsc, frame_at};
use super::shared::{FILTER_TAIL, UNITY, midi_to_freq, through};

/// Stab: two detuned sawtooths through a bandpass, under a short
/// exponential envelope (`voice('stab')`).
pub struct Stab {
    saws: [TableOsc; 2],
    pitch: f32,
    filter: Biquad,
    gain: Param,
    end: u64,
}

const STAB_CENTS: [f32; 2] = [-6.0, 7.0];

impl Stab {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`;
    /// starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        Stab {
            saws: [TableOsc::new(), TableOsc::new()],
            pitch: 0.0,
            filter: Biquad::new(),
            gain: Param::new(0.0),
            end: 0,
        }
    }

    pub(crate) fn play(&mut self, midi: f64, time: f64, dur: f64, vel: f64, rate: f64) {
        let f = midi_to_freq(midi);
        self.pitch = f as f32;
        let len = dur.min(0.22);
        self.filter.reset();
        self.filter.set(
            Kind::Bandpass,
            (f * 3.2).min(3400.0) as f32,
            2.2,
            0.0,
            rate as f32,
        );
        let g = &mut self.gain;
        g.reset_at(UNITY, rate);
        g.set_value_at_time(0.0001, time);
        g.exponential_ramp_to_value_at_time((vel * 0.3) as f32, time + 0.006, time);
        g.exponential_ramp_to_value_at_time(0.0001, time + len, time);
        let stop = time + len + 0.1;
        for saw in &mut self.saws {
            saw.schedule(time, stop, rate);
        }
        self.end = frame_at(stop + FILTER_TAIL, rate);
    }


    fn start_frame(&self) -> u64 {
        self.saws[0].start_frame()
    }

    fn end_frame(&self) -> u64 {
        self.end
    }
}

impl Block for Stab {
    type Settings = Note;

    const PROBES: &'static [Probe<Note>] = &[
        Probe { name: "stab", settings: Note::probe(crate::STAB, crate::MELODY, 67.0, 0, NO_EXTRA) },
    ];

    fn start(&mut self, note: &Note, cx: &Context) {
        self.play(note.midi, note.time, note.dur, note.vel, cx.rate);
    }

    fn span(&self) -> (u64, u64) {
        (self.start_frame(), self.end_frame())
    }

    fn render(&mut self, block: u64, cx: &Context, out: &mut Signal) {
        let (rate, waves) = (cx.rate, cx.waves);
        let r = rate as f32;
        let mut sum = [0.0f32; QUANTUM];
        let mut wave = [0.0f32; QUANTUM];
        for (saw, &detune) in self.saws.iter_mut().zip(STAB_CENTS.iter()) {
            let pitch = Pitch::Steady {
                freq: self.pitch,
                detune,
            };
            let (lo, hi) = saw.render(block, &waves.saw, &pitch, r, &mut wave);
            for (s, w) in sum.iter_mut().zip(wave.iter()).take(hi).skip(lo) {
                *s += *w;
            }
        }
        let mut gain = [0.0f32; QUANTUM];
        self.gain.fill(block, rate, &mut gain);
        through(&sum, &mut self.filter, &gain, out);
    }
}

impl Voice for Stab {}

impl Default for Stab {
    fn default() -> Self {
        Self::new()
    }
}
