//! A held instrument of four voices, retuned chord by chord.
//!
//! The second of queue item 46c's demonstration blocks, in the tests only
//! (its line in the registry carries `#[cfg(test)]`). It proves that a held
//! instrument a channel keeps for the loop is one new file -- this one --
//! plus one line in the registry: it hears retunes (`HEARS`), which reach it
//! every time they are sent, and moves by the shared rule (`retune`): a note
//! two chords share sounds on, untouched, and only the changed notes are let
//! go and spoken again. The test kit sends it five chords with no test
//! written for it. The chapel organ (item 61) is this, with pipes for sines.
//! `core/README.md` walks through it.

use core::f64::consts::TAU;

use crate::block::{Block, Channel, Context, Event, Probe, Signal, event};
use crate::math;

use super::shared::{Turn, midi_to_freq, retune};

/// How many voices it has.
const VOICES: usize = 4;
/// Each voice's level while it sounds.
const LEVEL: f64 = 0.08;
/// How long a voice takes to speak or let go, in seconds.
const SPEECH: f64 = 0.02;

/// How it is set: the chord it holds from the start (NaN for a silent
/// voice).
#[derive(Clone, Copy)]
pub struct Settings {
    /// The chord, as MIDI note numbers.
    pub chord: [f64; VOICES],
}

/// At rest: silent until it hears a chord.
pub const REST: Settings = Settings { chord: [f64::NAN; VOICES] };

/// One voice: a sine, its note, and the note it speaks once it has let go.
#[derive(Clone, Copy)]
struct Pipe {
    midi: f64,
    next: f64,
    phase: f64,
    step: f64,
    gain: f64,
    on: bool,
}

const SILENT: Pipe = Pipe { midi: f64::NAN, next: f64::NAN, phase: 0.0, step: 0.0, gain: 0.0, on: false };

/// The instrument: its voices, and how far a gain moves a frame.
pub struct Chorale {
    pipes: [Pipe; VOICES],
    speed: f64,
    rate: f64,
}

impl Chorale {
    /// Silent, all zeros.
    #[must_use]
    pub const fn new() -> Self {
        Chorale { pipes: [SILENT; VOICES], speed: 0.0, rate: 0.0 }
    }

    /// Speak `midi` on `pipe` from silence.
    fn speak(pipe: &mut Pipe, midi: f64, rate: f64) {
        *pipe = Pipe { midi, next: f64::NAN, phase: 0.0, step: midi_to_freq(midi) / rate, gain: 0.0, on: true };
    }
}

impl Default for Chorale {
    fn default() -> Self {
        Self::new()
    }
}

impl Block for Chorale {
    type Settings = Settings;

    const PROBES: &'static [Probe<Settings>] = &[Probe { name: "triad", settings: Settings { chord: [60.0, 64.0, 67.0, f64::NAN] } }];

    const HEARS: &'static [u32] = &[event::RETUNE];

    fn start(&mut self, s: &Settings, cx: &Context) {
        self.rate = cx.rate;
        self.speed = 1.0 / (SPEECH * cx.rate);
        for (pipe, &m) in self.pipes.iter_mut().zip(s.chord.iter()) {
            *pipe = SILENT;
            if !m.is_nan() {
                Self::speak(pipe, m, cx.rate);
            }
        }
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    /// A retune, from the start of the block it falls in: each voice keeps
    /// its note, lets go to speak another, or lets go.
    fn event(&mut self, e: &Event) {
        if e.kind != event::RETUNE {
            return;
        }
        let held = self.pipes.map(|p| if p.on { if p.next.is_nan() { p.midi } else { p.next } } else { f64::NAN });
        for (pipe, turn) in self.pipes.iter_mut().zip(retune(&held, e.chord())) {
            match turn {
                Turn::Keep => {}
                Turn::Speak(m) if pipe.gain == 0.0 => Self::speak(pipe, m, self.rate),
                Turn::Speak(m) => {
                    pipe.next = m;
                    pipe.on = true;
                }
                Turn::Rest => {
                    pipe.on = false;
                    pipe.next = f64::NAN;
                }
            }
        }
    }

    // A sample of a sine, in 32 bits as the signal is.
    #[allow(clippy::cast_possible_truncation)]
    fn render(&mut self, _: u64, _: &Context, signal: &mut Signal) {
        let rate = self.rate;
        for pipe in &mut self.pipes {
            if !pipe.on && pipe.gain == 0.0 {
                continue;
            }
            for x in signal.iter_mut() {
                // Speaking up, or letting go: of its note, or to speak the next.
                let up = pipe.on && pipe.next.is_nan();
                pipe.gain = if up { (pipe.gain + self.speed).min(1.0) } else { (pipe.gain - self.speed).max(0.0) };
                if pipe.gain == 0.0 && !pipe.next.is_nan() {
                    let next = pipe.next;
                    Self::speak(pipe, next, rate);
                }
                *x += (LEVEL * pipe.gain * math::sin(TAU * pipe.phase)) as f32;
                pipe.phase += pipe.step;
                if pipe.phase >= 1.0 {
                    pipe.phase -= 1.0;
                }
            }
        }
    }
}

// It adds its own sound to its channel.
impl Channel for Chorale {}
