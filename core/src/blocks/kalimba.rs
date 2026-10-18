//! Kalimba: a plucked metal tine (`voice('kalimba')` in synth.js).
//!
//! The note's `parts` are which parts of it the voice budget let through,
//! `STRIKE` and `BODY`; it reads no extra values and draws nothing. Its
//! strike is the FM voice (`fm`), composed in.

// Web Audio's parameters are 32-bit floats, set from the host's 64-bit
// numbers: a voice narrows every value it hands one.
#![allow(clippy::cast_possible_truncation)]

use crate::QUANTUM;
use crate::block::{Block, Context, NO_EXTRA, Note, Probe, Signal, Voice};
use crate::param::Param;

use super::fm::{Fm, FmOptions};
use super::osc::{Part, TableOsc};
use super::shared::{A440, UNITY, midi_to_freq, pitch_of, starts_in};

/// The kalimba's FM strike, as `voice('kalimba')` asks `fm()` for it.
const KALIMBA_STRIKE: FmOptions = FmOptions {
    ratio: 3.7,
    index: 260.0,
    decay: 0.09,
    attack: 0.002,
    detune: 0.0,
};
/// The longest note the strike is given.
pub const KALIMBA_LONGEST: f64 = 1.1;

/// Which parts of a kalimba note the voice budget let through: in the
/// JavaScript voice the strike and the body ask for room separately, and
/// either can be refused alone.
pub const STRIKE: u32 = 1;
/// The wooden thump from the box.
pub const BODY: u32 = 2;

/// Kalimba: a plucked metal tine. A bright inharmonic FM strike that dies
/// away almost at once, over a soft wooden thump from the box -- a sine an
/// octave down, gone in a fifth of a second.
pub struct Kalimba {
    strike: Fm,
    body: TableOsc,
    body_pitch: Param,
    body_gain: Param,
    parts: u32,
}

impl Kalimba {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`;
    /// starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        Kalimba {
            strike: Fm::new(),
            body: TableOsc::new(),
            body_pitch: Param::new(0.0),
            body_gain: Param::new(0.0),
            parts: 0,
        }
    }

    pub(crate) fn play(&mut self, midi: f64, time: f64, dur: f64, vel: f64, parts: u32, rate: f64) {
        self.parts = parts;
        if parts & STRIKE != 0 {
            let f = midi_to_freq(midi);
            self.strike.strike(
                f,
                time,
                dur.min(KALIMBA_LONGEST),
                vel,
                &KALIMBA_STRIKE,
                rate,
            );
        }
        if parts & BODY != 0 {
            self.body_pitch.reset(A440);
            self.body_pitch
                .set_value_at_time(midi_to_freq(midi - 12.0) as f32, time);
            let g = &mut self.body_gain;
            g.reset(UNITY);
            g.set_value_at_time(0.0001, time);
            g.exponential_ramp_to_value_at_time((vel * 0.12) as f32, time + 0.004, time);
            g.exponential_ramp_to_value_at_time(0.0001, time + 0.16, time);
            self.body.schedule(time, time + 0.2, rate);
        }
    }


    fn start_frame(&self) -> u64 {
        let mut at = u64::MAX;
        if self.parts & STRIKE != 0 {
            at = at.min(self.strike.span().0);
        }
        if self.parts & BODY != 0 {
            at = at.min(self.body.start_frame());
        }
        at
    }

    fn end_frame(&self) -> u64 {
        let mut at = 0;
        if self.parts & STRIKE != 0 {
            at = at.max(self.strike.span().1);
        }
        if self.parts & BODY != 0 {
            at = at.max(self.body.stop_frame());
        }
        at
    }
}

impl Block for Kalimba {
    type Settings = Note;

    const PROBES: &'static [Probe<Note>] = &[
        Probe { name: "whole", settings: Note::probe(crate::KALIMBA, crate::MELODY, 67.0, STRIKE | BODY, NO_EXTRA) },
        Probe { name: "body", settings: Note::probe(crate::KALIMBA, crate::MELODY, 67.0, BODY, NO_EXTRA) },
    ];

    fn start(&mut self, note: &Note, cx: &Context) {
        self.play(note.midi, note.time, note.dur, note.vel, note.parts, cx.rate);
    }

    fn span(&self) -> (u64, u64) {
        (self.start_frame(), self.end_frame())
    }

    fn render(&mut self, block: u64, cx: &Context, out: &mut Signal) {
        let (rate, waves) = (cx.rate, cx.waves);
        if self.parts & STRIKE != 0 {
            self.strike.render(block, cx, out);
        }
        if self.parts & BODY != 0 && block < self.body.stop_frame() {
            let mut freq = [0.0; QUANTUM];
            let first = starts_in(self.body.start_frame(), block);
            let pitch = pitch_of(
                &mut self.body_pitch,
                block,
                rate,
                first,
                &mut freq,
                Part::Steady(0.0),
            );
            let mut wave = [0.0; QUANTUM];
            let (lo, hi) = self
                .body
                .render(block, &waves.sine, &pitch, rate as f32, &mut wave);
            let mut gain = [0.0; QUANTUM];
            self.body_gain.fill(block, rate, &mut gain);
            for ((y, w), g) in out
                .iter_mut()
                .zip(wave.iter())
                .zip(gain.iter())
                .take(hi)
                .skip(lo)
            {
                *y += *w * *g;
            }
        }
    }
}

impl Voice for Kalimba {}

impl Default for Kalimba {
    fn default() -> Self {
        Self::new()
    }
}
