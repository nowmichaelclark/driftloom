//! Fiddle: a bowed string through the violin's body (`voice('fiddle')`).
//!
//! The note's extra values: the note it is joined to (NaN if none), then
//! three draws, its vibrato's rate, rate at the end and depth (none for a
//! note under half a second). It plays through the fiddle body
//! (`own::FIDDLE_BODY`).

// Web Audio's parameters are 32-bit floats, set from the host's 64-bit
// numbers: a voice narrows every value it hands one.
#![allow(clippy::cast_possible_truncation)]

use crate::QUANTUM;
use crate::block::{Block, Context, Mods, NO_EXTRA, Note, Probe, Signal, Voice};
use crate::param::Param;
use crate::rng::salt;

use super::lfo::Lfo;
use super::own;
use super::osc::{self, Oscillator, Shape};
use super::shared::{A440, SLUR_ATTACK, UNITY, VIBRATO_MIN_DUR, given, midi_to_freq, speak};

/// The fiddle's level, to its layer's median (`FIDDLE_LEVEL` in synth.js).
const FIDDLE_LEVEL: f64 = 0.0736;
/// The widest step a slur glides across, in semitones (`SLIDE_MAX_STEP`).
const SLUR_MAX_STEP: f64 = 5.0;

/// The vibrato a long fiddle note draws (`Voice::settle`): its rate at the
/// start and the end of the note, in Hz, and its depth in cents.
#[derive(Clone, Copy)]
pub struct Vibrato {
    /// Its rate at the start, in Hz.
    pub rate: f64,
    /// Its rate at the end of the note, in Hz.
    pub rate_end: f64,
    /// Its depth, in cents.
    pub depth: f64,
}

/// Fiddle: a bowed string. The `bowed` wave through the violin's body (one
/// per channel, in `Core`), an attack sized to the note, a slur from the
/// note before when it is joined to it, and on long notes a vibrato (an
/// LFO into its `DETUNE`) arriving once the note has settled.
pub struct Fiddle {
    osc: Oscillator,
    vibrato: bool,
    lfo: Lfo,
    envelope: Param,
}

impl Fiddle {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`;
    /// starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        Fiddle { osc: Oscillator::new(), vibrato: false, lfo: Lfo::new(), envelope: Param::new(0.0) }
    }

    /// `prev` is the note this one is joined to, if any (`opts.prev`).
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn play(
        &mut self,
        midi: f64,
        time: f64,
        dur: f64,
        vel: f64,
        prev: Option<f64>,
        vib: Option<Vibrato>,
        rate: f64,
    ) {
        let f = midi_to_freq(midi);
        // `_slur`: from the last note to this one, a little quicker than a
        // slide, unless the step is a repeat or wider than a fourth.
        // A repeat is the same number the host sent twice.
        #[allow(clippy::float_cmp)]
        match prev {
            Some(p) if midi != p && (midi - p).abs() <= SLUR_MAX_STEP => {
                self.osc.set(Shape::Bowed, A440, 0.0, time, time + dur + 0.4, rate);
                let pitch = self.osc.freq();
                pitch.set_value_at_time(midi_to_freq(p) as f32, time);
                let reach = (dur * 0.12).clamp(0.02, 0.045);
                pitch.exponential_ramp_to_value_at_time(f as f32, time + reach, time);
            }
            _ => self.osc.set(Shape::Bowed, f as f32, 0.0, time, time + dur + 0.4, rate),
        }
        self.vibrato = vib.is_some();
        if let Some(v) = vib {
            self.lfo.set(A440, UNITY, time, time + dur + 0.4, rate);
            let r = self.lfo.rate();
            r.set_value_at_time(v.rate as f32, time);
            r.linear_ramp_to_value_at_time(v.rate_end as f32, time + dur, time);
            let onset = (dur * 0.3).min(0.3);
            let d = self.lfo.depth();
            d.set_value_at_time(0.0, time);
            d.set_value_at_time(0.0, time + onset);
            d.linear_ramp_to_value_at_time((v.depth * 0.6) as f32, time + onset + 0.2, time);
            d.linear_ramp_to_value_at_time(v.depth as f32, time + dur, time);
        }
        let attack = if prev.is_some() {
            SLUR_ATTACK
        } else {
            speak(dur)
        };
        let g = &mut self.envelope;
        g.reset(UNITY);
        g.set_value_at_time(0.0001, time);
        g.linear_ramp_to_value_at_time((vel * FIDDLE_LEVEL) as f32, time + attack, time);
        g.set_target_at_time(0.0001, time + dur * 0.9, 0.07);
    }

    fn start_frame(&self) -> u64 {
        self.osc.start_frame()
    }

    fn end_frame(&self) -> u64 {
        self.osc.stop_frame()
    }
}

impl Block for Fiddle {
    type Settings = Note;

    const PROBES: &'static [Probe<Note>] = &[
        Probe { name: "bowed", settings: Note::probe(crate::FIDDLE, crate::MELODY, 67.0, 0, NO_EXTRA) },
        Probe { name: "long", settings: Note { dur: 1.2, ..Note::probe(crate::FIDDLE, crate::MELODY, 67.0, 0, NO_EXTRA) } },
    ];

    fn start(&mut self, note: &Note, cx: &Context) {
        let [prev, rate, rate_end, depth, ..] = note.extra;
        let vibrato = if rate.is_finite() && rate_end.is_finite() && depth.is_finite() {
            Some(Vibrato { rate, rate_end, depth })
        } else {
            None
        };
        self.play(note.midi, note.time, note.dur, note.vel, given(prev), vibrato, cx.rate);
    }

    fn span(&self) -> (u64, u64) {
        (self.start_frame(), self.end_frame())
    }

    /// Add this block into `out`, the fiddle's bus for its channel: the body
    /// comes after, shared by every fiddle note in the channel.
    fn render(&mut self, block: u64, cx: &Context, out: &mut Signal) {
        // The vibrato, in cents, into the detune. Connected to it, the
        // detune moves for the whole note, even before the vibrato's own
        // start.
        let mut cents = [0.0f32; QUANTUM];
        let mods = if self.vibrato {
            self.lfo.render(block, cx, &mut cents);
            Mods::NONE.with(osc::DETUNE, &cents)
        } else {
            Mods::NONE
        };
        let mut wave = [0.0f32; QUANTUM];
        self.osc.render_mod(block, cx, &mut wave, &mods);
        let (lo, hi) = self.osc.played();
        let mut envelope = [0.0f32; QUANTUM];
        self.envelope.fill(block, cx.rate, &mut envelope);
        for ((y, w), g) in out.iter_mut().zip(wave.iter()).zip(envelope.iter()).take(hi).skip(lo) {
            *y += *w * *g;
        }
    }
}

impl Voice for Fiddle {
    const THROUGH: Option<usize> = Some(own::FIDDLE_BODY);

    /// The vibrato's rate, where it ends and its depth, for a note of half
    /// a second or more.
    fn settle(note: &mut Note) {
        let d = note.draws;
        if note.dur >= VIBRATO_MIN_DUR {
            let rate = 4.8 + d.unit(salt::FIDDLE_RATE, 0) * 1.3;
            note.extra[1] = rate;
            note.extra[2] = rate * (0.9 + d.unit(salt::FIDDLE_END, 0) * 0.2);
            note.extra[3] = 10.0 + d.unit(salt::FIDDLE_DEPTH, 0) * 10.0;
        } else {
            note.extra[1] = f64::NAN;
            note.extra[2] = f64::NAN;
            note.extra[3] = f64::NAN;
        }
    }
}

impl Default for Fiddle {
    fn default() -> Self {
        Self::new()
    }
}
