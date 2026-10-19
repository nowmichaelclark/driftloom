//! The wave-table voices whose pitch moves: the moog and the whistle, with
//! a vibrato that ramps in.
//!
//! The note's first extra value is the note it is joined to (NaN if
//! none). The whistle then draws whether it slides into its pitch, from
//! where and how long it takes, into the first two; the moog draws
//! nothing.

// Web Audio's parameters are 32-bit floats, set from the host's 64-bit
// numbers: a voice narrows every value it hands one.
#![allow(clippy::cast_possible_truncation)]
// The names follow synth.js's (f, g, c...), so the port reads against it.
#![allow(clippy::many_single_char_names)]

use crate::QUANTUM;
use crate::block::{Block, Context, Mods, NO_EXTRA, Note, Probe, Signal, Voice};
use crate::param::Param;

use super::filter::{self, Filter, Kind};
use super::lfo::Lfo;
use super::osc::{self, Oscillator, Shape, frame_at};
use super::shared::{A440, CUTOFF_DEFAULT, FILTER_TAIL, UNITY, midi_to_freq, settle_slide};

/// The vibrato of the moog and the whistle, in Hz.
const LEAD_VIBRATO: f32 = 5.6;

/// Which of the two a `Lead` is.
#[derive(Clone, Copy, PartialEq)]
pub enum LeadKind {
    /// The moog: its own wave, no slide.
    Moog,
    /// The whistle: the triangle, sliding in on some notes.
    Whistle,
}

/// `voice('moog')` and `voice('whistle')`: one oscillator (the moog's own
/// wave, or the built-in triangle) whose pitch is automated, with a
/// vibrato into its detune that ramps in, through a resonant lowpass whose
/// cutoff an envelope sweeps (into its `FREQ`), under a gain. The whistle
/// may scoop or slide into the note (`_slide`); where from and how long it
/// takes are drawn (`Voice::settle`).
pub struct Lead {
    osc: Oscillator,
    lfo: Lfo,
    cutoff: Param,
    filter: Filter,
    gain: Param,
    end: u64,
}

impl Lead {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`;
    /// starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        Lead {
            osc: Oscillator::new(),
            lfo: Lfo::new(),
            cutoff: Param::new(0.0),
            filter: Filter::new(),
            gain: Param::new(0.0),
            end: 0,
        }
    }

    /// `slide` is where the pitch comes from in Hz and how long it takes to
    /// arrive, in seconds, if the whistle slides into this note.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn play(
        &mut self,
        kind: LeadKind,
        midi: f64,
        time: f64,
        dur: f64,
        vel: f64,
        slide: Option<(f64, f64)>,
        rate: f64,
    ) {
        let whistle = kind == LeadKind::Whistle;
        let f = midi_to_freq(midi);
        let stop = time + dur + 0.5;
        let shape = if whistle { Shape::Triangle } else { Shape::LeadMoog };
        self.osc.set(shape, A440, 0.0, time, stop, rate);
        let p = self.osc.freq();
        match slide.filter(|_| whistle) {
            Some((from, reach)) => {
                p.set_value_at_time(from as f32, time);
                p.exponential_ramp_to_value_at_time(f as f32, time + reach, time);
            }
            None => {
                p.set_value_at_time(f as f32, time);
            }
        }
        self.lfo.set(LEAD_VIBRATO, UNITY, time, stop, rate);
        let d = self.lfo.depth();
        d.set_value_at_time(0.0, time);
        d.linear_ramp_to_value_at_time(if whistle { 12.0 } else { 5.0 }, time + dur.min(0.6), time);
        let c = &mut self.cutoff;
        c.reset(CUTOFF_DEFAULT);
        let q = if whistle { 8.0 } else { 5.0 };
        // The cutoff is all envelope, into the filter's frequency.
        self.filter.set(filter::Settings { kind: Kind::Lowpass, freq: 0.0, q, gain: 0.0 }, rate as f32);
        if whistle {
            c.set_value_at_time((f * 7.0).min(9000.0) as f32, time);
            c.exponential_ramp_to_value_at_time(
                (f * 1.6).max(220.0) as f32,
                time + (dur * 0.8).max(0.12),
                time,
            );
        } else {
            c.set_value_at_time(1600.0, time);
            c.exponential_ramp_to_value_at_time(1100.0, time + (dur * 0.8).max(0.12), time);
        }
        let g = &mut self.gain;
        g.reset(UNITY);
        g.set_value_at_time(0.0001, time);
        g.exponential_ramp_to_value_at_time(
            (vel * if whistle { 0.1121 } else { 0.0947 }) as f32,
            time + 0.014,
            time,
        );
        g.set_target_at_time(0.0001, time + dur * 0.75, 0.1);
        self.end = frame_at(stop + FILTER_TAIL, rate);
    }

    fn start_frame(&self) -> u64 {
        self.osc.start_frame()
    }

    fn end_frame(&self) -> u64 {
        self.end
    }
}

impl Block for Lead {
    type Settings = Note;

    const PROBES: &'static [Probe<Note>] = &[
        Probe { name: "moog", settings: Note::probe(crate::MOOG, crate::MELODY, 67.0, 0, NO_EXTRA) },
        Probe { name: "whistle", settings: Note::probe(crate::WHISTLE, crate::MELODY, 67.0, 0, NO_EXTRA) },
    ];

    fn start(&mut self, note: &Note, cx: &Context) {
        let kind = if note.voice == crate::MOOG { LeadKind::Moog } else { LeadKind::Whistle };
        let [from, reach, ..] = note.extra;
        let slide = if from.is_finite() && reach.is_finite() { Some((from, reach)) } else { None };
        self.play(kind, note.midi, note.time, note.dur, note.vel, slide, cx.rate);
    }

    fn span(&self) -> (u64, u64) {
        (self.start_frame(), self.end_frame())
    }

    fn render(&mut self, block: u64, cx: &Context, out: &mut Signal) {
        let rate = cx.rate;
        // The vibrato, in cents, into the detune: connected, it moves for the
        // whole note, even before the depth has risen. Feeding only the
        // detune, the depth is first rendered when the note starts.
        let mut cents = [0.0f32; QUANTUM];
        self.lfo.render(block, cx, &mut cents);
        let mut tone = [0.0f32; QUANTUM];
        self.osc.render_mod(block, cx, &mut tone, &Mods::NONE.with(osc::DETUNE, &cents));
        // The filter runs every frame of the block, its cutoff swept.
        let mut cutoff = [0.0f32; QUANTUM];
        self.cutoff.fill(block, rate, &mut cutoff);
        self.filter.render_mod(block, cx, &mut tone, &Mods::NONE.with(filter::FREQ, &cutoff));
        let mut gain = [0.0f32; QUANTUM];
        self.gain.fill(block, rate, &mut gain);
        for ((y, x), g) in out.iter_mut().zip(tone.iter()).zip(gain.iter()) {
            *y += *x * *g;
        }
    }
}

impl Voice for Lead {
    /// The whistle's slide (the moog draws nothing).
    fn settle(note: &mut Note) {
        if note.voice == crate::WHISTLE {
            settle_slide(note);
        }
    }
}

impl Default for Lead {
    fn default() -> Self {
        Self::new()
    }
}

