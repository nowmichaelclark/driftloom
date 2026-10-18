//! Tubular: a tubular bell, five sine partials (`voice('tubular')`).
//!
//! The note's extra values are five draws, its partials' detunes, in
//! partial order.

// Web Audio's parameters are 32-bit floats, set from the host's 64-bit
// numbers: a voice narrows every value it hands one.
#![allow(clippy::cast_possible_truncation)]
// The names follow synth.js's (f, g, c...), so the port reads against it.
#![allow(clippy::many_single_char_names)]

use crate::QUANTUM;
use crate::block::{Block, Context, NO_EXTRA, Note, Probe, Signal, Voice};
use crate::param::Param;
use crate::rng::salt;

use super::osc::{Pitch, TableOsc};
use super::shared::{BELL_PARTIAL_VEL, UNITY, midi_to_freq, release};

/// The tubular bell's partials: ratio to the note, and relative level.
const TUBULAR_PARTIALS: [(f64, f64); 5] = [
    (1.0, 0.8),
    (1.19, 0.6),
    (1.56, 0.4),
    (2.0, 0.5),
    (2.71, 0.22),
];
/// How many partials the tubular bell has.
pub const TUBULAR_PARTS: usize = TUBULAR_PARTIALS.len();

/// Tubular: a church or orchestral tubular bell. Five sine partials, each
/// detuned by up to three cents either way and each dying away at its own
/// rate, under one envelope that strikes at once and lets go after at least
/// five seconds.
pub struct Tubular {
    oscs: [TableOsc; TUBULAR_PARTS],
    gains: [Param; TUBULAR_PARTS],
    pitch: [f32; TUBULAR_PARTS],
    detune: [f32; TUBULAR_PARTS],
    envelope: Param,
}

impl Tubular {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`;
    /// starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        Tubular {
            oscs: [const { TableOsc::new() }; TUBULAR_PARTS],
            gains: [const { Param::new(0.0) }; TUBULAR_PARTS],
            pitch: [0.0; TUBULAR_PARTS],
            detune: [0.0; TUBULAR_PARTS],
            envelope: Param::new(0.0),
        }
    }

    /// `draws` are the partials' five detune draws, in partial order, each
    /// in [0, 1): a partial's detune is `(draw - 0.5) * 6` cents.
    pub(crate) fn play(
        &mut self,
        midi: f64,
        time: f64,
        dur: f64,
        vel: f64,
        draws: [f64; TUBULAR_PARTS],
        rate: f64,
    ) {
        let f = midi_to_freq(midi);
        let hold = dur.max(5.0);
        let stop = time + hold + 1.2;
        let g = &mut self.envelope;
        g.reset(UNITY);
        g.set_value_at_time(0.0001, time);
        g.exponential_ramp_to_value_at_time((vel * 0.26) as f32, time + 0.004, time);
        release(g, time + 0.02, stop, time);
        for (i, &(ratio, amp)) in TUBULAR_PARTIALS.iter().enumerate() {
            if let (Some(p), Some(d), Some(&draw)) =
                (self.pitch.get_mut(i), self.detune.get_mut(i), draws.get(i))
            {
                *p = (f * ratio) as f32;
                *d = ((draw - 0.5) * 6.0) as f32;
            }
            if let (Some(pg), Some(osc)) = (self.gains.get_mut(i), self.oscs.get_mut(i)) {
                pg.reset(UNITY);
                pg.set_value_at_time((BELL_PARTIAL_VEL * amp * 0.62) as f32, time);
                pg.exponential_ramp_to_value_at_time(
                    0.0001,
                    time + hold / (0.5 + ratio * 0.28),
                    time,
                );
                osc.schedule(time, stop + 0.01, rate);
            }
        }
    }


    fn start_frame(&self) -> u64 {
        self.oscs[0].start_frame()
    }

    fn end_frame(&self) -> u64 {
        self.oscs[0].stop_frame()
    }
}

impl Block for Tubular {
    type Settings = Note;

    const PROBES: &'static [Probe<Note>] = &[
        Probe { name: "bell", settings: Note::probe(crate::TUBULAR, crate::CHORDS, 55.0, 0, NO_EXTRA) },
    ];

    fn start(&mut self, note: &Note, cx: &Context) {
        let [a, b, c, d, e, ..] = note.extra;
        self.play(note.midi, note.time, note.dur, note.vel, [a, b, c, d, e], cx.rate);
    }

    fn span(&self) -> (u64, u64) {
        (self.start_frame(), self.end_frame())
    }

    fn render(&mut self, block: u64, cx: &Context, out: &mut Signal) {
        let (rate, waves) = (cx.rate, cx.waves);
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
        let mut envelope = [0.0f32; QUANTUM];
        self.envelope.fill(block, rate, &mut envelope);
        for ((y, x), g) in out.iter_mut().zip(sum.iter()).zip(envelope.iter()) {
            *y += *x * *g;
        }
    }
}

impl Voice for Tubular {
    /// Its five partials' detunes, in partial order.
    fn settle(note: &mut Note) {
        let d = note.draws;
        for (i, x) in (0_u32..).zip(note.extra.iter_mut().take(TUBULAR_PARTS)) {
            *x = d.unit(salt::TUBULAR_DETUNE, i);
        }
    }
}

impl Default for Tubular {
    fn default() -> Self {
        Self::new()
    }
}
