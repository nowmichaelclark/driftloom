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
use crate::block::{Block, Context, Mods, NO_EXTRA, Note, Probe, Signal, Voice};
use crate::param::Param;

use super::lfo::Lfo;
use super::osc::{self, Oscillator, Shape};
use super::shared::{UNITY, midi_to_freq, release};

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
/// (its `FREQ` input) under an envelope that strikes, decays across the
/// note and releases to silence 1.2 s after it.
pub struct Fm {
    carrier: Oscillator<0>,
    /// The modulator and its depth: an LFO at audio rate.
    modulator: Lfo,
    envelope: Param,
}

impl Fm {
    /// A silent voice, all zeros.
    #[must_use]
    pub const fn new() -> Self {
        Fm { carrier: Oscillator::new(), modulator: Lfo::new(), envelope: Param::new(0.0) }
    }

    /// Strike a note of `f` Hz at `time` seconds, held `dur`, at `vel`.
    // The voice's settings are Web Audio's 32-bit parameters.
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn strike(&mut self, f: f64, time: f64, dur: f64, vel: f64, o: &FmOptions, rate: f64) {
        let stop = time + dur + 1.2;
        self.carrier.set(Shape::Sine, f as f32, o.detune as f32, time, stop + 0.01, rate);
        self.modulator.set((f * o.ratio) as f32, UNITY, time, stop + 0.01, rate);

        let depth = self.modulator.depth();
        depth.set_value_at_time((o.index * vel) as f32, time);
        depth.exponential_ramp_to_value_at_time(
            (o.index * 0.06).max(1.0) as f32,
            time + o.decay.min(0.9),
            time,
        );

        let peak = (vel * 0.26).max(0.001);
        let g = &mut self.envelope;
        g.reset(UNITY);
        g.set_value_at_time(0.0001, time);
        g.exponential_ramp_to_value_at_time(peak as f32, time + o.attack, time);
        let from = time + (dur * 0.7).max(0.08);
        g.exponential_ramp_to_value_at_time((peak * 0.3).max(0.0008) as f32, from, time);
        release(g, from, stop, time);
    }
}

/// Keys, the plainest options a host sends (`fm()` for `keys`).
const KEYS: [f64; crate::EXTRA] = options(2.0, 260.0, 0.006, 0.4, 0.0);
/// A bell's: a far ratio, a deep index, a little detune.
const BELL: [f64; crate::EXTRA] = options(3.51, 420.0, 0.006, 0.5, 7.0);
/// Glass's (queue item 59; `GLASS.bell` in `js/engine.js`): a soft bell at
/// ratio 3.5, a light index that settles over 1.8 s, high in the texture.
const GLASS: [f64; crate::EXTRA] = options(3.5, 150.0, 0.003, 1.8, 3.0);

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
        Probe { name: "glass", settings: Note::probe(crate::FM, crate::TEXTURE, 84.0, 0, GLASS) },
    ];

    fn start(&mut self, note: &Note, cx: &Context) {
        let [ratio, index, attack, decay, detune, ..] = note.extra;
        let options = FmOptions { ratio, index, attack, decay, detune };
        self.strike(midi_to_freq(note.midi), note.time, note.dur, note.vel, &options, cx.rate);
    }

    fn span(&self) -> (u64, u64) {
        (self.carrier.start_frame(), self.carrier.stop_frame().max(self.modulator.span().1))
    }

    /// Add this block of the note into `out`.
    fn render(&mut self, block: u64, cx: &Context, out: &mut Signal) {
        // The modulator through its depth, into the carrier's frequency, as
        // Web Audio sums an AudioParam's value and its input. Connected, so
        // the carrier's pitch always moves. The modulator starts with the
        // carrier, so its depth's events are clamped to the carrier's first
        // block, as the depth's gain node is first rendered then.
        let mut fm = [0.0f32; QUANTUM];
        self.modulator.render(block, cx, &mut fm);
        let mut wave = [0.0f32; QUANTUM];
        self.carrier.render_mod(block, cx, &mut wave, &Mods::NONE.with(osc::FREQ, &fm));
        let (lo, hi) = self.carrier.played();
        let mut envelope = [0.0; QUANTUM];
        self.envelope.fill(block, cx.rate, &mut envelope);
        for ((y, w), g) in out.iter_mut().zip(wave.iter()).zip(envelope.iter()).take(hi).skip(lo) {
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
