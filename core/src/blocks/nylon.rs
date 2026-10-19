//! Nylon guitar: a plucked string through the guitar's body
//! (`voice('nylon')`).
//!
//! The note's first extra value says whether it is a string of a strum (1)
//! or not (0 or NaN): a strummed string hears the next strum into its
//! channel, a `DAMP` channel event (`Core::damp`, `Voice::event`). It draws
//! nothing, and plays through the channel's nylon body (`own::NYLON_BODY`).

// Web Audio's parameters are 32-bit floats, set from the host's 64-bit
// numbers: a voice narrows every value it hands one.
#![allow(clippy::cast_possible_truncation)]

use crate::QUANTUM;
use crate::block::{Block, Context, Event, NO_EXTRA, Note, Probe, Signal, Voice, event};
use crate::math;
use crate::param::Param;

use super::own;
use super::osc::{Pitch, TableOsc};
use super::shared::{UNITY, midi_to_freq};

/// The nylon guitar's level (`NYLON_LEVEL`), the bright layer's share of
/// it and how fast each layer dies away (`NYLON_BRIGHT`, `NYLON_MELLOW`),
/// and the release a strummed string is damped on (`NYLON_DAMP`).
const NYLON_LEVEL: f64 = 0.1029;
const NYLON_BRIGHT_SHARE: f64 = 0.6;
const NYLON_BRIGHT_TAU: f64 = 0.08;
const NYLON_MELLOW_TAU: f64 = 0.55;
/// The release a strummed string is damped on, as a time constant in
/// seconds.
pub const NYLON_DAMP: f64 = 0.08;

/// Nylon guitar: a plucked string, two layers -- a mellow one that rings
/// on, lower strings longer, and a bright one that dies in a tenth of a
/// second -- each with its own envelope, into the guitar's body
/// (`voice('nylon')`). A strummed string can be damped by the next strum
/// (`damp`).
pub struct Nylon {
    /// Mellow, then bright, in the order JavaScript builds them.
    oscs: [TableOsc; 2],
    gains: [Param; 2],
    pitch: f32,
    /// Whether it is a string of a strum, which the next strum damps.
    strummed: bool,
}

impl Nylon {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`;
    /// starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        Nylon {
            oscs: [TableOsc::new(), TableOsc::new()],
            gains: [Param::new(0.0), Param::new(0.0)],
            pitch: 0.0,
            strummed: false,
        }
    }

    pub(crate) fn play(&mut self, midi: f64, time: f64, dur: f64, vel: f64, rate: f64) {
        let f = midi_to_freq(midi);
        self.pitch = f as f32;
        let level = vel * NYLON_LEVEL;
        let ring = NYLON_MELLOW_TAU * math::pow(261.6 / f, 0.35);
        let stop = time + dur + 0.4;
        for ((osc, g), (peak, tau)) in self.oscs.iter_mut().zip(self.gains.iter_mut()).zip([
            (level, ring),
            (level * NYLON_BRIGHT_SHARE, NYLON_BRIGHT_TAU),
        ]) {
            g.reset_at(UNITY, rate);
            g.set_value_at_time(0.0001, time);
            g.exponential_ramp_to_value_at_time(peak as f32, time + 0.004, time);
            g.set_target_at_time(0.0001, time + 0.004, tau);
            g.set_target_at_time(0.0001, time + dur, 0.08);
            osc.schedule(time, stop, rate);
        }
    }

    /// Let the string go at `time`, on the note's own release
    /// (`damp()` in synth.js).
    fn damp(&mut self, time: f64) {
        for g in &mut self.gains {
            g.set_target_at_time(0.0001, time, NYLON_DAMP);
        }
    }

    fn start_frame(&self) -> u64 {
        self.oscs[0].start_frame()
    }

    fn end_frame(&self) -> u64 {
        self.oscs[0].stop_frame()
    }
}

impl Block for Nylon {
    type Settings = Note;

    const PROBES: &'static [Probe<Note>] = &[
        Probe { name: "pluck", settings: Note::probe(crate::NYLON, crate::MELODY, 67.0, 0, NO_EXTRA) },
    ];

    fn start(&mut self, note: &Note, cx: &Context) {
        self.play(note.midi, note.time, note.dur, note.vel, cx.rate);
        self.strummed = note.extra[0] > 0.0;
    }

    fn span(&self) -> (u64, u64) {
        (self.start_frame(), self.end_frame())
    }

    /// Add this block into `out`, the nylon's bus for its channel.
    fn render(&mut self, block: u64, cx: &Context, out: &mut Signal) {
        let (rate, waves) = (cx.rate, cx.waves);
        let r = rate as f32;
        let pitch = Pitch::Steady {
            freq: self.pitch,
            detune: 0.0,
        };
        for ((osc, g), wave) in self
            .oscs
            .iter_mut()
            .zip(self.gains.iter())
            .zip([&waves.nylon_mellow, &waves.nylon_bright])
        {
            let mut w = [0.0f32; QUANTUM];
            let (lo, hi) = osc.render(block, wave, &pitch, r, &mut w);
            let mut gain = [0.0f32; QUANTUM];
            g.fill(block, rate, &mut gain);
            for ((y, x), k) in out
                .iter_mut()
                .zip(w.iter())
                .zip(gain.iter())
                .take(hi)
                .skip(lo)
            {
                *y += *x * *k;
            }
        }
    }
}

impl Voice for Nylon {
    const THROUGH: Option<usize> = Some(own::NYLON_BODY);

    /// A strummed string lets go at a damp: the next strum.
    fn event(&mut self, event: &Event) {
        if self.strummed && event.kind == event::DAMP {
            self.damp(event.time);
        }
    }
}

impl Default for Nylon {
    fn default() -> Self {
        Self::new()
    }
}
