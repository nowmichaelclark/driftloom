//! Sine: one sine under a swell (`voice('sine')`). It reads no extra
//! values and draws nothing.

// Web Audio's parameters are 32-bit floats, set from the host's 64-bit
// numbers: a voice narrows every value it hands one.
#![allow(clippy::cast_possible_truncation)]

use crate::QUANTUM;
use crate::block::{Block, Context, NO_EXTRA, Note, Probe, Signal, Voice};
use crate::param::Param;

use super::osc::{Pitch, TableOsc};
use super::shared::{UNITY, midi_to_freq, release};

/// Sine: one sine under a linear swell and a release to silence 1 s after
/// the note (`voice('sine')`).
pub struct SineNote {
    osc: TableOsc,
    pitch: f32,
    gain: Param,
}

impl SineNote {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`;
    /// starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        SineNote {
            osc: TableOsc::new(),
            pitch: 0.0,
            gain: Param::new(0.0),
        }
    }

    pub(crate) fn play(&mut self, midi: f64, time: f64, dur: f64, vel: f64, rate: f64) {
        self.pitch = midi_to_freq(midi) as f32;
        let stop = time + dur + 1.0;
        let g = &mut self.gain;
        g.reset(UNITY);
        g.set_value_at_time(0.0001, time);
        g.linear_ramp_to_value_at_time((vel * 0.24) as f32, time + (dur * 0.3).min(0.5), time);
        release(g, time + dur * 0.7, stop, time);
        self.osc.schedule(time, stop + 0.01, rate);
    }


    fn start_frame(&self) -> u64 {
        self.osc.start_frame()
    }

    fn end_frame(&self) -> u64 {
        self.osc.stop_frame()
    }
}

impl Block for SineNote {
    type Settings = Note;

    const PROBES: &'static [Probe<Note>] = &[
        Probe { name: "sine", settings: Note::probe(crate::SINE, crate::MELODY, 67.0, 0, NO_EXTRA) },
    ];

    fn start(&mut self, note: &Note, cx: &Context) {
        self.play(note.midi, note.time, note.dur, note.vel, cx.rate);
    }

    fn span(&self) -> (u64, u64) {
        (self.start_frame(), self.end_frame())
    }

    fn render(&mut self, block: u64, cx: &Context, out: &mut Signal) {
        let (rate, waves) = (cx.rate, cx.waves);
        let pitch = Pitch::Steady {
            freq: self.pitch,
            detune: 0.0,
        };
        let mut wave = [0.0f32; QUANTUM];
        let (lo, hi) = self
            .osc
            .render(block, &waves.sine, &pitch, rate as f32, &mut wave);
        let mut gain = [0.0f32; QUANTUM];
        self.gain.fill(block, rate, &mut gain);
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

impl Voice for SineNote {}

impl Default for SineNote {
    fn default() -> Self {
        Self::new()
    }
}
