//! The wave-table voices with a steady pitch: the soft pad, the analog
//! pad, the analog lead, the saw pluck and the square beep, each a port of
//! its JavaScript twin in `js/synth.js`, call for call, and each the same
//! shape -- a few oscillators on one wave, through a lowpass, under an
//! envelope -- so one type plays five voice numbers. They read no extra
//! values and draw nothing.

// Web Audio's parameters are 32-bit floats, set from the host's 64-bit
// numbers: a voice narrows every value it hands one.
#![allow(clippy::cast_possible_truncation)]

use crate::QUANTUM;
use crate::block::{Block, Context, Mods, NO_EXTRA, Note, Probe, Signal, Voice};
use crate::param::Param;

use super::filter::{self, Filter, Kind};
use super::lfo::Lfo;
use super::osc::{self, Oscillator, Shape, frame_at};
use super::shared::{CUTOFF_DEFAULT, FILTER_TAIL, Q_DEFAULT, UNITY, midi_to_freq, release};

/// The most oscillators a `Tone` has (the pads' three saws).
const TONES: usize = 3;

/// A tone with a steady pitch: `count` oscillators on one table, some
/// cents apart, summed into a lowpass whose cutoff may move (an envelope
/// into its `FREQ`), then a gain. With a vibrato, an LFO at a fixed rate
/// and depth drives every oscillator's `DETUNE`.
pub struct Tone {
    count: usize,
    oscs: [Oscillator<0>; TONES],
    vibrato: bool,
    lfo: Lfo<0>,
    cutoff: Param,
    q: f32,
    filter: Filter,
    gain: Param,
    end: u64,
}

impl Tone {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`;
    /// starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        Tone {
            count: 0,
            oscs: [const { Oscillator::new() }; TONES],
            vibrato: false,
            lfo: Lfo::new(),
            cutoff: Param::new(0.0),
            q: 0.0,
            filter: Filter::new(),
            gain: Param::new(0.0),
            end: 0,
        }
    }

    // What every recipe shares: the oscillators, started at `time` and
    // stopped at `stop`, and the gain and filter state.
    fn start(
        &mut self,
        shape: Shape,
        detunes: &[f32],
        midi: f64,
        time: f64,
        stop: f64,
        rate: f64,
    ) {
        self.count = detunes.len().min(TONES);
        let pitch = midi_to_freq(midi) as f32;
        self.vibrato = false;
        for (osc, &cents) in self.oscs.iter_mut().zip(detunes) {
            osc.set(shape, pitch, cents, time, stop, rate);
        }
        self.gain.reset(UNITY);
        self.end = frame_at(stop + FILTER_TAIL, rate);
    }

    /// `softpad`: three saws (-9, 0, +11 cents) under a lowpass that opens
    /// across the first half of the note.
    pub(crate) fn soft_pad(&mut self, midi: f64, time: f64, dur: f64, vel: f64, rate: f64) {
        let stop = time + dur + 1.8;
        self.start(
            Shape::Saw,
            &[-9.0, 0.0, 11.0],
            midi,
            time,
            stop + 0.01,
            rate,
        );
        self.q = Q_DEFAULT;
        let c = &mut self.cutoff;
        c.reset(CUTOFF_DEFAULT);
        c.set_value_at_time(700.0, time);
        c.linear_ramp_to_value_at_time(1500.0, time + dur * 0.5, time);
        let g = &mut self.gain;
        g.set_value_at_time(0.0001, time);
        g.linear_ramp_to_value_at_time((vel * 0.16) as f32, time + (dur * 0.45).min(1.4), time);
        release(g, time + dur * 0.7, stop, time);
    }

    /// `analogpad`: three saws (-11, 0, +9) under a lowpass that opens to
    /// six times the note.
    pub(crate) fn analog_pad(&mut self, midi: f64, time: f64, dur: f64, vel: f64, rate: f64) {
        let f = midi_to_freq(midi);
        let stop = time + dur + 1.6;
        self.start(
            Shape::Saw,
            &[-11.0, 0.0, 9.0],
            midi,
            time,
            stop + 0.01,
            rate,
        );
        self.q = 1.6;
        let c = &mut self.cutoff;
        c.reset(CUTOFF_DEFAULT);
        c.set_value_at_time((f * 3.0).min(900.0) as f32, time);
        c.linear_ramp_to_value_at_time(
            (f * 6.0).min(2600.0) as f32,
            time + (dur * 0.6).min(2.0),
            time,
        );
        let g = &mut self.gain;
        g.set_value_at_time(0.0001, time);
        g.linear_ramp_to_value_at_time((vel * 0.17) as f32, time + (dur * 0.3).min(0.9), time);
        release(g, time + dur * 0.75, stop, time);
    }

    /// `analoglead`: two soft saws (-7, +6) under a fixed lowpass.
    pub(crate) fn analog_lead(&mut self, midi: f64, time: f64, dur: f64, vel: f64, rate: f64) {
        self.start(
            Shape::LeadAnalog,
            &[-7.0, 6.0],
            midi,
            time,
            time + dur + 0.8,
            rate,
        );
        self.q = 0.0;
        self.cutoff.reset(1600.0);
        let let_go = time + dur * 0.7;
        let g = &mut self.gain;
        g.set_value_at_time(0.0001, time);
        g.exponential_ramp_to_value_at_time((vel * 0.0861) as f32, (time + 0.03).min(let_go), time);
        g.set_target_at_time(0.0001, let_go, 0.18);
    }

    /// `pluck('saw')`: one soft saw under a fixed lowpass with a little edge.
    pub(crate) fn saw_pluck(&mut self, midi: f64, time: f64, dur: f64, vel: f64, rate: f64) {
        self.start(Shape::LeadSaw, &[0.0], midi, time, time + dur + 0.6, rate);
        self.q = 4.0;
        self.cutoff.reset(1400.0);
        let g = &mut self.gain;
        g.set_value_at_time(0.0001, time);
        g.exponential_ramp_to_value_at_time((vel * 0.1055) as f32, time + 0.01, time);
        g.set_target_at_time(0.0001, time + dur * 0.6, 0.15);
    }

    /// `pluck()`'s default, the square beep: a square wave with a constant
    /// 5.4 Hz vibrato of 3.5 cents, under a lowpass at 2.6 kHz.
    pub(crate) fn beep(&mut self, midi: f64, time: f64, dur: f64, vel: f64, rate: f64) {
        let stop = time + dur + 0.5;
        self.start(Shape::Square, &[0.0], midi, time, stop, rate);
        self.vibrato = true;
        self.lfo.set(5.4, 3.5, time, stop, rate);
        self.q = Q_DEFAULT;
        self.cutoff.reset(2600.0);
        let g = &mut self.gain;
        g.set_value_at_time(0.0001, time);
        g.exponential_ramp_to_value_at_time((vel * 0.095) as f32, time + 0.008, time);
        g.set_target_at_time(0.0001, time + dur * 0.55, 0.12);
    }

    fn start_frame(&self) -> u64 {
        self.oscs[0].start_frame()
    }

    fn end_frame(&self) -> u64 {
        self.end
    }
}

impl Block for Tone {
    type Settings = Note;

    const PROBES: &'static [Probe<Note>] = &[
        Probe { name: "softpad", settings: Note::probe(crate::SOFTPAD, crate::CHORDS, 55.0, 0, NO_EXTRA) },
        Probe { name: "analogpad", settings: Note::probe(crate::ANALOGPAD, crate::CHORDS, 55.0, 0, NO_EXTRA) },
        Probe { name: "analoglead", settings: Note::probe(crate::ANALOGLEAD, crate::MELODY, 67.0, 0, NO_EXTRA) },
        Probe { name: "saw", settings: Note::probe(crate::SAWPLUCK, crate::MELODY, 67.0, 0, NO_EXTRA) },
        Probe { name: "pluck", settings: Note::probe(crate::BEEP, crate::MELODY, 67.0, 0, NO_EXTRA) },
    ];

    fn start(&mut self, note: &Note, cx: &Context) {
        let (midi, time, dur, vel, rate) = (note.midi, note.time, note.dur, note.vel, cx.rate);
        match note.voice {
            crate::SOFTPAD => self.soft_pad(midi, time, dur, vel, rate),
            crate::ANALOGPAD => self.analog_pad(midi, time, dur, vel, rate),
            crate::ANALOGLEAD => self.analog_lead(midi, time, dur, vel, rate),
            crate::SAWPLUCK => self.saw_pluck(midi, time, dur, vel, rate),
            _ => self.beep(midi, time, dur, vel, rate),
        }
        // The cutoff is all envelope, into the filter's frequency.
        self.filter.set(filter::Settings { kind: Kind::Lowpass, freq: 0.0, q: self.q, gain: 0.0 }, rate as f32);
    }

    fn span(&self) -> (u64, u64) {
        (self.start_frame(), self.end_frame())
    }

    fn render(&mut self, block: u64, cx: &Context, out: &mut Signal) {
        let rate = cx.rate;
        // The vibrato's detune, in cents, into every oscillator.
        let mut cents = [0.0f32; QUANTUM];
        let mods = if self.vibrato {
            self.lfo.render(block, cx, &mut cents);
            Mods::NONE.with(osc::DETUNE, &cents)
        } else {
            Mods::NONE
        };
        let mut sum = [0.0f32; QUANTUM];
        for osc in self.oscs.iter_mut().take(self.count) {
            osc.render_mod(block, cx, &mut sum, &mods);
        }
        // The filter runs every frame of the block, as a node does.
        let mut cutoff = [0.0f32; QUANTUM];
        self.cutoff.fill(block, rate, &mut cutoff);
        self.filter.render_mod(block, cx, &mut sum, &Mods::NONE.with(filter::FREQ, &cutoff));
        let mut gain = [0.0f32; QUANTUM];
        self.gain.fill(block, rate, &mut gain);
        for ((y, x), g) in out.iter_mut().zip(sum.iter()).zip(gain.iter()) {
            *y += *x * *g;
        }
    }
}

impl Voice for Tone {}

impl Default for Tone {
    fn default() -> Self {
        Self::new()
    }
}
