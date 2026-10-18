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
use crate::block::{Block, Context, NO_EXTRA, Note, Probe, Signal, Voice};
use crate::param::Param;
use crate::wave::{Wave, Waves};

use super::filter::{Biquad, Kind};
use super::osc::{Part, Pitch, TableOsc, frame_at};
use super::shared::{CUTOFF_DEFAULT, FILTER_TAIL, Q_DEFAULT, UNITY, midi_to_freq, release};

/// Which table a `Tone` plays.
#[derive(Clone, Copy, PartialEq)]
pub enum Source {
    /// The built-in sawtooth.
    Saw,
    /// The built-in square.
    Square,
    /// `_makeWave(1.5)`, the saw lead's.
    LeadSaw,
    /// `_makeWave(1.7)`, the analoglead's.
    LeadAnalog,
}

fn table(source: Source, waves: &Waves) -> &Wave {
    match source {
        Source::Saw => &waves.saw,
        Source::Square => &waves.square,
        Source::LeadSaw => &waves.lead_saw,
        Source::LeadAnalog => &waves.lead_analog,
    }
}

/// The most oscillators a `Tone` has (the pads' three saws).
const TONES: usize = 3;

/// A tone with a steady pitch: `count` oscillators on one table, `detune`
/// cents apart, summed into a lowpass whose cutoff may move, then a gain.
/// With a vibrato, one sine at a fixed rate through a fixed depth drives
/// every oscillator's detune.
pub struct Tone {
    source: Source,
    count: usize,
    oscs: [TableOsc; TONES],
    detune: [f32; TONES],
    pitch: f32,
    vibrato: bool,
    lfo: TableOsc,
    lfo_rate: f32,
    lfo_depth: f32,
    cutoff: Param,
    q: f32,
    filter: Biquad,
    gain: Param,
    end: u64,
}

impl Tone {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`;
    /// starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        Tone {
            source: Source::Saw,
            count: 0,
            oscs: [const { TableOsc::new() }; TONES],
            detune: [0.0; TONES],
            pitch: 0.0,
            vibrato: false,
            lfo: TableOsc::new(),
            lfo_rate: 0.0,
            lfo_depth: 0.0,
            cutoff: Param::new(0.0),
            q: 0.0,
            filter: Biquad::new(),
            gain: Param::new(0.0),
            end: 0,
        }
    }

    // What every recipe shares: the oscillators, started at `time` and
    // stopped at `stop`, and the gain and filter state.
    fn start(
        &mut self,
        source: Source,
        detunes: &[f32],
        midi: f64,
        time: f64,
        stop: f64,
        rate: f64,
    ) {
        self.source = source;
        self.count = detunes.len().min(TONES);
        self.pitch = midi_to_freq(midi) as f32;
        self.vibrato = false;
        self.filter.reset();
        for ((osc, d), &cents) in self
            .oscs
            .iter_mut()
            .zip(self.detune.iter_mut())
            .zip(detunes)
        {
            *d = cents;
            osc.schedule(time, stop, rate);
        }
        self.gain.reset(UNITY);
        self.end = frame_at(stop + FILTER_TAIL, rate);
    }

    /// `softpad`: three saws (-9, 0, +11 cents) under a lowpass that opens
    /// across the first half of the note.
    pub(crate) fn soft_pad(&mut self, midi: f64, time: f64, dur: f64, vel: f64, rate: f64) {
        let stop = time + dur + 1.8;
        self.start(
            Source::Saw,
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
            Source::Saw,
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
            Source::LeadAnalog,
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
        self.start(Source::LeadSaw, &[0.0], midi, time, time + dur + 0.6, rate);
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
        self.start(Source::Square, &[0.0], midi, time, stop, rate);
        self.vibrato = true;
        self.lfo_rate = 5.4;
        self.lfo_depth = 3.5;
        self.lfo.schedule(time, stop, rate);
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
    }

    fn span(&self) -> (u64, u64) {
        (self.start_frame(), self.end_frame())
    }

    fn render(&mut self, block: u64, cx: &Context, out: &mut Signal) {
        let (rate, waves) = (cx.rate, cx.waves);
        let r = rate as f32;
        let wave = table(self.source, waves);
        // The vibrato's detune, in cents: its sine through its depth.
        let mut cents = [0.0f32; QUANTUM];
        if self.vibrato {
            let steady = Pitch::Steady {
                freq: self.lfo_rate,
                detune: 0.0,
            };
            let mut sine = [0.0f32; QUANTUM];
            let (lo, hi) = self.lfo.render(block, &waves.sine, &steady, r, &mut sine);
            for (c, w) in cents.iter_mut().zip(sine.iter()).take(hi).skip(lo) {
                *c = *w * self.lfo_depth;
            }
        }
        let mut sum = [0.0f32; QUANTUM];
        let mut one = [0.0f32; QUANTUM];
        for (osc, &detune) in self
            .oscs
            .iter_mut()
            .zip(self.detune.iter())
            .take(self.count)
        {
            if self.vibrato {
                let pitch = Pitch::Moving {
                    freq: Part::Steady(self.pitch),
                    detune: Part::Moving(&cents),
                };
                osc.render(block, wave, &pitch, r, &mut one);
            } else {
                let pitch = Pitch::Steady {
                    freq: self.pitch,
                    detune,
                };
                osc.render(block, wave, &pitch, r, &mut one);
            }
            for (s, w) in sum.iter_mut().zip(one.iter()) {
                *s += *w;
            }
        }
        // The filter runs every frame of the block, as a node does.
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
            self.filter.set(Kind::Lowpass, *c, self.q, 0.0, r);
            *y += self.filter.step(*x) * *g;
        }
        self.filter.flush();
    }
}

impl Voice for Tone {}

impl Default for Tone {
    fn default() -> Self {
        Self::new()
    }
}
