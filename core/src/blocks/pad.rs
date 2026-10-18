//! Pad: one note of `pad()`, two triangles through an opening lowpass.
//!
//! The note's first extra value is how many notes the chord has. It draws
//! nothing.

// Web Audio's parameters are 32-bit floats, set from the host's 64-bit
// numbers: a voice narrows every value it hands one.
#![allow(clippy::cast_possible_truncation)]

use crate::QUANTUM;
use crate::block::{Block, Context, NO_EXTRA, Note, Probe, Signal, Voice};
use crate::math;
use crate::param::Param;

use super::filter::{Biquad, Kind};
use super::osc::{Pitch, TableOsc};
use super::shared::{CUTOFF_DEFAULT, UNITY, midi_to_freq, release};

/// The two triangles' detunes, in cents.
const PAD_DETUNE: [f32; 2] = [-7.0, 6.0];
const PAD_Q: f32 = 0.8;

/// One note of `pad()`: two triangles 13 cents apart through a lowpass
/// that opens across the first half of the note, under a slow swell and a
/// release to silence 1.6 s after it.
pub struct PadNote {
    oscs: [TableOsc; 2],
    pitch: f32,
    gain: Param,
    cutoff: Param,
    filter: Biquad,
}

impl PadNote {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`;
    /// starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        PadNote {
            oscs: [TableOsc::new(), TableOsc::new()],
            pitch: 0.0,
            gain: Param::new(0.0),
            cutoff: Param::new(0.0),
            filter: Biquad::new(),
        }
    }

    /// `count` is how many notes the chord has: the level is spread across
    /// them.
    pub(crate) fn play(&mut self, midi: f64, time: f64, dur: f64, vel: f64, count: f64, rate: f64) {
        self.pitch = midi_to_freq(midi) as f32;
        let stop = time + dur + 1.6;
        let g = &mut self.gain;
        g.reset(UNITY);
        g.set_value_at_time(0.0001, time);
        g.linear_ramp_to_value_at_time(
            ((vel * 0.22) / math::sqrt(count)) as f32,
            time + (dur * 0.4).min(0.9),
            time,
        );
        release(g, time + dur * 0.8, stop, time);
        let c = &mut self.cutoff;
        c.reset(CUTOFF_DEFAULT);
        c.set_value_at_time(700.0, time);
        c.linear_ramp_to_value_at_time(1900.0, time + dur * 0.5, time);
        self.filter.reset();
        for osc in &mut self.oscs {
            osc.schedule(time, stop + 0.01, rate);
        }
    }


    fn start_frame(&self) -> u64 {
        self.oscs[0].start_frame()
    }

    fn end_frame(&self) -> u64 {
        self.oscs[0].stop_frame()
    }
}

impl Block for PadNote {
    type Settings = Note;

    const PROBES: &'static [Probe<Note>] = &[
        Probe { name: "chord", settings: Note::probe(crate::PAD, crate::CHORDS, 55.0, 0, NO_EXTRA) },
    ];

    fn start(&mut self, note: &Note, cx: &Context) {
        self.play(note.midi, note.time, note.dur, note.vel, note.extra[0].max(1.0), cx.rate);
    }

    fn span(&self) -> (u64, u64) {
        (self.start_frame(), self.end_frame())
    }

    fn render(&mut self, block: u64, cx: &Context, out: &mut Signal) {
        let (rate, waves) = (cx.rate, cx.waves);
        let r = rate as f32;
        let mut sum = [0.0f32; QUANTUM];
        let mut wave = [0.0f32; QUANTUM];
        for (osc, &detune) in self.oscs.iter_mut().zip(PAD_DETUNE.iter()) {
            let pitch = Pitch::Steady {
                freq: self.pitch,
                detune,
            };
            osc.render(block, &waves.triangle, &pitch, r, &mut wave);
            for (s, w) in sum.iter_mut().zip(wave.iter()) {
                *s += *w;
            }
        }
        // The filter runs every frame of the block, as a node does: before
        // the triangles start its input is silence.
        let mut cutoff = [0.0f32; QUANTUM];
        self.cutoff.fill(block, rate, &mut cutoff);
        let mut gain = [0.0f32; QUANTUM];
        self.gain.fill(block, rate, &mut gain);
        for (((y, x), c), g) in out
            .iter_mut()
            .zip(sum.iter())
            .zip(cutoff.iter())
            .zip(gain.iter())
        {
            self.filter.set(Kind::Lowpass, *c, PAD_Q, 0.0, r);
            *y += self.filter.step(*x) * *g;
        }
        self.filter.flush();
    }
}

impl Voice for PadNote {}

impl Default for PadNote {
    fn default() -> Self {
        Self::new()
    }
}

