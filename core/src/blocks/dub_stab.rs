//! dubStab (queue item 60; Ears 4, round 4, approved by Mikey 2026-10-16):
//! a soft chord stab on the offbeat of a busy groove, thrown into the echo.
//!
//! One note is the whole chord, up to three notes (the note's own and two
//! in its extra values): for each, a triangle six cents sharp under a
//! sawtooth six cents flat, all through one lowpass closing from its open
//! frequency to 700 Hz over the tail, a highpass at 320 Hz, and a gain that
//! speaks in 22 ms and dies away over the tail (`dubStab()` in
//! `tools/ears/sketches/ears4-sketch.patch`, round 4's options). Its throw
//! into the echo, darker and lighter than the chords' own send, and a
//! little reverb, are its sends (`Voice::SENDS`).
//!
//! Its peak is `vel` times its level, shared between its notes (the
//! sketch's `vel * level / sqrt(notes)`). Its extra values, all set by the
//! host, drawn per loop from its seed:
//!
//! - 0, 1: the chord's second and third notes (NaN for none);
//! - 2: the tail, in seconds (0.24 to 0.34);
//! - 3: the lowpass's open frequency, in Hz (2,000 to 2,800);
//! - 4: the sawtooth's level against the triangle's (0.6 to 0.9);
//! - 5: the throw into the echo (0.3 to 0.42);
//! - 6: the throw's lowpass, in Hz (1,000 to 1,300);
//! - 7: the reverb send (0.12);
//! - 8: its level (0.19, brought down by up to 8 dB in a loop of under four
//!   notes a second).

// Web Audio's parameters are 32-bit floats, set from the host's 64-bit
// numbers: a voice narrows every value it hands one.
#![allow(clippy::cast_possible_truncation)]

use crate::block::{Block, Context, Mods, Note, Probe, Sends, Signal, Voice};
use crate::param::Param;
use crate::{EXTRA, QUANTUM};

use super::filter::{self, Biquad, Filter, Kind};
use super::osc::{Pitch, TableOsc, frame_at};
use super::shared::{FILTER_TAIL, midi_to_freq};

/// The most notes a stab plays.
const NOTES: usize = 3;
/// Its attack, in seconds; where its lowpass closes to, in Hz; the lowpass's
/// Q (dB); the highpass's corner and Q; each layer's detune, in cents.
const ATTACK: f64 = 0.022;
const CLOSE: f32 = 700.0;
const LOWPASS_Q: f32 = 0.8;
const HIGHPASS: f32 = 320.0;
const HIGHPASS_Q: f32 = 0.6;
const TRIANGLE_CENTS: f32 = 6.0;
const SAW_CENTS: f32 = -6.0;
/// Its level before the loop's density brings it down (round 4's 0.19).
const LEVEL: f64 = 0.19;
/// The throw's lowpass Q, Web Audio's default.
const THROW_Q: f32 = 1.0;

/// A dubStab note: its oscillators, filters and gain, and its sends.
pub struct DubStab {
    triangles: [TableOsc; NOTES],
    saws: [TableOsc; NOTES],
    freqs: [f32; NOTES],
    notes: usize,
    saw: f32,
    cutoff: Param,
    lowpass: Filter,
    highpass: Biquad,
    gain: Param,
    throw: f32,
    verb: f32,
    dark: Biquad,
    start: u64,
    end: u64,
}

impl DubStab {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`.
    #[must_use]
    pub const fn new() -> Self {
        DubStab {
            triangles: [TableOsc::new(), TableOsc::new(), TableOsc::new()],
            saws: [TableOsc::new(), TableOsc::new(), TableOsc::new()],
            freqs: [0.0; NOTES],
            notes: 0,
            saw: 0.0,
            cutoff: Param::new(0.0),
            lowpass: Filter::new(),
            highpass: Biquad::new(),
            gain: Param::new(0.0),
            throw: 0.0,
            verb: 0.0,
            dark: Biquad::new(),
            start: 0,
            end: 0,
        }
    }

    /// The stab, unsent: its notes through the filters under the gain.
    fn stab(&mut self, block: u64, cx: &Context, out: &mut Signal) {
        let r = cx.rate as f32;
        let mut sum = [0.0f32; QUANTUM];
        let mut wave = [0.0f32; QUANTUM];
        for ((tri, saw), &freq) in self.triangles.iter_mut().zip(self.saws.iter_mut()).zip(self.freqs.iter()).take(self.notes) {
            let pitch = Pitch::Steady { freq, detune: TRIANGLE_CENTS };
            let (lo, hi) = tri.render(block, &cx.waves.triangle, &pitch, r, &mut wave);
            for (s, w) in sum.iter_mut().zip(wave.iter()).take(hi).skip(lo) {
                *s += *w;
            }
            let pitch = Pitch::Steady { freq, detune: SAW_CENTS };
            let (lo, hi) = saw.render(block, &cx.waves.saw, &pitch, r, &mut wave);
            for (s, w) in sum.iter_mut().zip(wave.iter()).take(hi).skip(lo) {
                *s += *w * self.saw;
            }
        }
        let mut cutoff = [0.0f32; QUANTUM];
        self.cutoff.fill(block, cx.rate, &mut cutoff);
        self.lowpass.render_mod(block, cx, &mut sum, &Mods::NONE.with(filter::FREQ, &cutoff));
        let mut gain = [0.0f32; QUANTUM];
        self.gain.fill(block, cx.rate, &mut gain);
        for ((y, x), g) in out.iter_mut().zip(sum.iter()).zip(gain.iter()) {
            *y = self.highpass.step(*x) * *g;
        }
        self.highpass.flush();
    }
}

impl Default for DubStab {
    fn default() -> Self {
        Self::new()
    }
}

/// A probe's chord and the options round 4 draws in the middle of its
/// ranges: C4 with E4 and G4, a 0.29 s tail, open at 2.4 kHz, the saw at
/// 0.75, a throw of 0.36 darkened at 1.15 kHz, 0.12 into the room, at its
/// full level.
const PROBE_EXTRA: [f64; EXTRA] = {
    let mut x = [f64::NAN; EXTRA];
    x[0] = 64.0;
    x[1] = 67.0;
    x[2] = 0.29;
    x[3] = 2400.0;
    x[4] = 0.75;
    x[5] = 0.36;
    x[6] = 1150.0;
    x[7] = 0.12;
    x[8] = LEVEL;
    x
};

impl Block for DubStab {
    type Settings = Note;

    const PROBES: &'static [Probe<Note>] = &[Probe {
        name: "c4",
        settings: Note::probe(crate::DUBSTAB, crate::CHORDS, 60.0, 0, PROBE_EXTRA),
    }];

    fn start(&mut self, note: &Note, cx: &Context) {
        let rate = cx.rate;
        let [n2, n3, decay, open, saw, throw, tone, verb, level, ..] = note.extra;
        // The chord's notes, the ones it has, in order.
        let mut notes = [0.0; NOTES];
        let mut count = 0;
        for m in [note.midi, n2, n3] {
            if m.is_finite()
                && let Some(slot) = notes.get_mut(count)
            {
                *slot = m;
                count += 1;
            }
        }
        self.notes = count;
        let time = note.time;
        let dec = if decay.is_finite() { decay } else { 0.29 };
        let end = time + ATTACK + dec * 1.8;
        for ((tri, saw_osc), (freq, &m)) in self.triangles.iter_mut().zip(self.saws.iter_mut()).zip(self.freqs.iter_mut().zip(notes.iter())).take(count) {
            *freq = midi_to_freq(m) as f32;
            tri.schedule(time, end, rate);
            saw_osc.schedule(time, end, rate);
        }
        self.saw = if saw.is_finite() { saw as f32 } else { 0.75 };
        let open = if open.is_finite() { open } else { 2400.0 };
        let c = &mut self.cutoff;
        c.reset(0.0);
        c.set_value_at_time(open as f32, time);
        c.exponential_ramp_to_value_at_time(CLOSE, time + dec, time);
        self.lowpass.set(filter::Settings { kind: Kind::Lowpass, freq: 0.0, q: LOWPASS_Q, gain: 0.0 }, rate as f32);
        self.highpass.reset();
        self.highpass.set(Kind::Highpass, HIGHPASS, HIGHPASS_Q, 0.0, rate as f32);
        // The peak, shared between the notes.
        #[allow(clippy::cast_precision_loss)] // three at most
        let level = if level.is_finite() { level } else { LEVEL };
        let peak = note.vel * level / crate::math::sqrt(count.max(1) as f64);
        let g = &mut self.gain;
        g.reset(0.0);
        g.set_value_at_time(0.0, time);
        g.linear_ramp_to_value_at_time(peak as f32, time + ATTACK, time);
        g.set_target_at_time(0.0, time + ATTACK, dec / 3.5);
        self.throw = if throw.is_finite() { throw as f32 } else { 0.0 };
        self.verb = if verb.is_finite() { verb as f32 } else { 0.0 };
        let tone = if tone.is_finite() { tone } else { 1100.0 };
        self.dark.reset();
        self.dark.set(Kind::Lowpass, tone as f32, THROW_Q, 0.0, rate as f32);
        self.start = self.triangles[0].start_frame();
        self.end = frame_at(end + FILTER_TAIL, rate);
    }

    fn span(&self) -> (u64, u64) {
        (self.start, self.end)
    }

    fn render(&mut self, block: u64, cx: &Context, out: &mut Signal) {
        let mut stab = [0.0f32; QUANTUM];
        self.stab(block, cx, &mut stab);
        for (y, x) in out.iter_mut().zip(stab.iter()) {
            *y += *x;
        }
    }
}

impl Voice for DubStab {
    // Its own throw into the echo and its room.
    const SENDS: bool = true;

    fn render_sends(&mut self, block: u64, cx: &Context, out: &mut Signal, sends: &mut Sends) {
        let mut stab = [0.0f32; QUANTUM];
        self.stab(block, cx, &mut stab);
        let [verb, echo] = sends;
        for (((y, v), e), x) in out.iter_mut().zip(verb.iter_mut()).zip(echo.iter_mut()).zip(stab.iter()) {
            *y += *x;
            *v += *x * self.verb;
            *e += self.dark.step(*x) * self.throw;
        }
        self.dark.flush();
    }
}
