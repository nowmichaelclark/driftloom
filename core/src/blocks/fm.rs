//! `fm()`: the two-operator FM voice. Every voice that is an `fm()` call in
//! synth.js -- keys, bell, celeste, musicbox, rhodes, marimba, harp, piano,
//! the rhodes bass, and the bell and chime textures -- is this voice, with
//! the call's options in the note; the kalimba's strike is one too.
//!
//! The note's extra values are `fm()`'s options, resolved, in this order:
//! ratio, index, attack, decay, detune (cents). It draws nothing.

// Web Audio's parameters are 32-bit floats, set from the host's 64-bit
// numbers: a voice narrows every value it hands one.
#![allow(clippy::cast_possible_truncation)]

use crate::QUANTUM;
use crate::block::{Block, Context, NO_EXTRA, Note, Probe, Signal, Voice};
use crate::param::Param;

use super::osc::{Part, Pitch, TableOsc};
use super::shared::{UNITY, midi_to_freq, release, starts_in};

/// The options `fm()` reads.
pub struct FmOptions {
    /// The modulator's frequency, as a ratio of the carrier's.
    pub ratio: f64,
    /// How far the modulator swings the carrier, in Hz at full velocity.
    pub index: f64,
    /// The envelope's attack, in seconds.
    pub attack: f64,
    /// How long the modulation takes to fall away, in seconds.
    pub decay: f64,
    /// The carrier's detune, in cents.
    pub detune: f64,
}

/// `fm()`: two-operator FM. A sine modulator, its depth falling from
/// `index * vel` Hz across the decay, into the frequency of a sine carrier
/// under an envelope that strikes, decays across the note and releases to
/// silence 1.2 s after it.
pub struct Fm {
    carrier: TableOsc,
    modulator: TableOsc,
    depth: Param,
    envelope: Param,
    pitch: f32,
    mod_pitch: f32,
    detune: f32,
}

impl Fm {
    /// A silent voice, all zeros.
    #[must_use]
    pub const fn new() -> Self {
        Fm {
            carrier: TableOsc::new(),
            modulator: TableOsc::new(),
            depth: Param::new(0.0),
            envelope: Param::new(0.0),
            pitch: 0.0,
            mod_pitch: 0.0,
            detune: 0.0,
        }
    }

    /// Strike a note of `f` Hz at `time` seconds, held `dur`, at `vel`.
    // The voice's settings are Web Audio's 32-bit parameters.
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn strike(&mut self, f: f64, time: f64, dur: f64, vel: f64, o: &FmOptions, rate: f64) {
        self.pitch = f as f32;
        self.mod_pitch = (f * o.ratio) as f32;
        self.detune = o.detune as f32;

        let depth = &mut self.depth;
        depth.reset(UNITY);
        depth.set_value_at_time((o.index * vel) as f32, time);
        depth.exponential_ramp_to_value_at_time(
            (o.index * 0.06).max(1.0) as f32,
            time + o.decay.min(0.9),
            time,
        );

        let peak = (vel * 0.26).max(0.001);
        let stop = time + dur + 1.2;
        let g = &mut self.envelope;
        g.reset(UNITY);
        g.set_value_at_time(0.0001, time);
        g.exponential_ramp_to_value_at_time(peak as f32, time + o.attack, time);
        let from = time + (dur * 0.7).max(0.08);
        g.exponential_ramp_to_value_at_time((peak * 0.3).max(0.0008) as f32, from, time);
        release(g, from, stop, time);

        self.carrier.schedule(time, stop + 0.01, rate);
        self.modulator.schedule(time, stop + 0.01, rate);
    }

}

/// Keys, the plainest options a host sends (`fm()` for `keys`).
const KEYS: [f64; crate::EXTRA] = options(2.0, 260.0, 0.006, 0.4, 0.0);
/// A bell's: a far ratio, a deep index, a little detune.
const BELL: [f64; crate::EXTRA] = options(3.51, 420.0, 0.006, 0.5, 7.0);

const fn options(ratio: f64, index: f64, attack: f64, decay: f64, detune: f64) -> [f64; crate::EXTRA] {
    let mut x = NO_EXTRA;
    x[0] = ratio;
    x[1] = index;
    x[2] = attack;
    x[3] = decay;
    x[4] = detune;
    x
}

impl Block for Fm {
    type Settings = Note;

    const PROBES: &'static [Probe<Note>] = &[
        Probe { name: "keys", settings: Note::probe(crate::FM, crate::MELODY, 67.0, 0, KEYS) },
        Probe { name: "bell", settings: Note::probe(crate::FM, crate::MELODY, 67.0, 0, BELL) },
    ];

    fn start(&mut self, note: &Note, cx: &Context) {
        let [ratio, index, attack, decay, detune, ..] = note.extra;
        let options = FmOptions { ratio, index, attack, decay, detune };
        self.strike(midi_to_freq(note.midi), note.time, note.dur, note.vel, &options, cx.rate);
    }

    fn span(&self) -> (u64, u64) {
        (self.carrier.start_frame(), self.carrier.stop_frame().max(self.modulator.stop_frame()))
    }

    /// Add this block of the note into `out`.
    fn render(&mut self, block: u64, cx: &Context, out: &mut Signal) {
        let (rate, waves) = (cx.rate, cx.waves);
        let r = rate as f32;
        let mut wave = [0.0; QUANTUM];
        // The modulator's frequency never moves.
        let steady = Pitch::Steady {
            freq: self.mod_pitch,
            detune: 0.0,
        };
        let (lo, hi) = self
            .modulator
            .render(block, &waves.sine, &steady, r, &mut wave);
        // The carrier's frequency at every frame of the block: its own pitch
        // plus the modulator through its depth, as Web Audio sums an
        // AudioParam's value and its input. Connected, so it always moves.
        // The depth's gain node feeds only the carrier's frequency, so like
        // the carrier's own events it is first rendered when the carrier
        // starts, and its events are clamped to that block.
        if starts_in(self.carrier.start_frame(), block) {
            self.depth.clamp_before(block, rate);
        }
        let mut depth = [0.0; QUANTUM];
        self.depth.fill(block, rate, &mut depth);
        let mut freq = [self.pitch; QUANTUM];
        for ((f, w), d) in freq
            .iter_mut()
            .zip(wave.iter())
            .zip(depth.iter())
            .take(hi)
            .skip(lo)
        {
            *f += *w * *d;
        }
        let carrier = Pitch::Moving {
            freq: Part::Moving(&freq),
            detune: Part::Steady(self.detune),
        };
        let (lo, hi) = self
            .carrier
            .render(block, &waves.sine, &carrier, r, &mut wave);
        let mut envelope = [0.0; QUANTUM];
        self.envelope.fill(block, rate, &mut envelope);
        for ((y, w), g) in out
            .iter_mut()
            .zip(wave.iter())
            .zip(envelope.iter())
            .take(hi)
            .skip(lo)
        {
            *y += *w * *g;
        }
    }
}

impl Voice for Fm {}

impl Default for Fm {
    fn default() -> Self {
        Self::new()
    }
}
