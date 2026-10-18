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
use crate::block::{Block, Context, NO_EXTRA, Note, Probe, Signal, Voice};
use crate::param::Param;

use super::filter::{Biquad, Kind};
use super::osc::{Part, Pitch, TableOsc, frame_at};
use super::shared::{
    A440, CUTOFF_DEFAULT, FILTER_TAIL, UNITY, midi_to_freq, pitch_of, settle_slide, starts_in,
};

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
/// vibrato on its detune that ramps in, through a resonant lowpass whose
/// cutoff moves, under a gain. The whistle may scoop or slide into the
/// note (`_slide`); where from and how long it takes are drawn
/// (`Voice::settle`).
pub struct Lead {
    kind: LeadKind,
    osc: TableOsc,
    pitch: Param,
    lfo: TableOsc,
    depth: Param,
    cutoff: Param,
    q: f32,
    filter: Biquad,
    gain: Param,
    end: u64,
}

impl Lead {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`;
    /// starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        Lead {
            kind: LeadKind::Moog,
            osc: TableOsc::new(),
            pitch: Param::new(0.0),
            lfo: TableOsc::new(),
            depth: Param::new(0.0),
            cutoff: Param::new(0.0),
            q: 0.0,
            filter: Biquad::new(),
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
        self.kind = kind;
        let stop = time + dur + 0.5;
        let p = &mut self.pitch;
        p.reset(A440);
        match slide.filter(|_| whistle) {
            Some((from, reach)) => {
                p.set_value_at_time(from as f32, time);
                p.exponential_ramp_to_value_at_time(f as f32, time + reach, time);
            }
            None => {
                p.set_value_at_time(f as f32, time);
            }
        }
        let d = &mut self.depth;
        d.reset(UNITY);
        d.set_value_at_time(0.0, time);
        d.linear_ramp_to_value_at_time(if whistle { 12.0 } else { 5.0 }, time + dur.min(0.6), time);
        let c = &mut self.cutoff;
        c.reset(CUTOFF_DEFAULT);
        if whistle {
            self.q = 8.0;
            c.set_value_at_time((f * 7.0).min(9000.0) as f32, time);
            c.exponential_ramp_to_value_at_time(
                (f * 1.6).max(220.0) as f32,
                time + (dur * 0.8).max(0.12),
                time,
            );
        } else {
            self.q = 5.0;
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
        self.filter.reset();
        self.osc.schedule(time, stop, rate);
        self.lfo.schedule(time, stop, rate);
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
        let (rate, waves) = (cx.rate, cx.waves);
        let r = rate as f32;
        // The detune, in cents: the vibrato's sine through its depth. It
        // moves for the whole note, even before the depth has risen.
        let steady = Pitch::Steady {
            freq: LEAD_VIBRATO,
            detune: 0.0,
        };
        let mut sine = [0.0f32; QUANTUM];
        let (lo, hi) = self.lfo.render(block, &waves.sine, &steady, r, &mut sine);
        // Feeding only the detune, the depth is first rendered when the
        // note starts: its events are clamped to that block.
        if starts_in(self.lfo.start_frame(), block) {
            self.depth.clamp_before(block, rate);
        }
        let mut depth = [0.0f32; QUANTUM];
        self.depth.fill(block, rate, &mut depth);
        let mut cents = [0.0f32; QUANTUM];
        for ((c, w), d) in cents
            .iter_mut()
            .zip(sine.iter())
            .zip(depth.iter())
            .take(hi)
            .skip(lo)
        {
            *c = *w * *d;
        }
        let mut freq = [0.0f32; QUANTUM];
        let first = starts_in(self.osc.start_frame(), block);
        let pitch = pitch_of(
            &mut self.pitch,
            block,
            rate,
            first,
            &mut freq,
            Part::Moving(&cents),
        );
        let wave = match self.kind {
            LeadKind::Moog => &waves.lead_moog,
            LeadKind::Whistle => &waves.triangle,
        };
        let mut tone = [0.0f32; QUANTUM];
        self.osc.render(block, wave, &pitch, r, &mut tone);
        let mut cutoff = [0.0f32; QUANTUM];
        self.cutoff.fill(block, rate, &mut cutoff);
        let mut gain = [0.0f32; QUANTUM];
        self.gain.fill(block, rate, &mut gain);
        for (((y, x), c), g) in out
            .iter_mut()
            .zip(tone.iter())
            .zip(cutoff.iter())
            .zip(gain.iter())
        {
            self.filter.set(Kind::Lowpass, *c, self.q, 0.0, r);
            *y += self.filter.step(*x) * *g;
        }
        self.filter.flush();
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

