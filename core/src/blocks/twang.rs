//! twang (queue item 101h; batch 6's #13, Ears 13b, approved by Mikey
//! 2026-11-09, all three versions): a synth in the spirit of a tremolo
//! baritone guitar (`docs/DECISIONS.md`, 2026-11-08: a synth in its spirit,
//! never an imitation), picking each chord low and slow. The player picks it
//! (`Synth::twang`) at each chord whose root changes, or the same root again
//! after a while: the root's pitch class between E2 and D#3, then its fifth
//! a beat later, then its tenth a beat after that; this file is what a pick
//! plays.
//!
//! What plays is the Ears 13b sketch's (`tools/ears/sketches/ears13/twang/`),
//! number for number: two band-limited saws 4 cents apart through a two-pole
//! lowpass whose cutoff starts at the version's `bright` times the note and
//! falls over 0.12 s to its `round` times it; a two-stage fall (55% of the
//! pick gone in a quarter second, the rest a tail of 2.8 s), a 2 ms attack,
//! and a pitch settling from 8 cents sharp over 40 ms. Six strings, as the
//! sketch's: a pick takes the next, and its saws start where that string's
//! ran to since its last pick (`PHASES`, from the player), as the sketch's
//! strings, which a pick never reset, did; so the pair's slow beat comes in
//! at a phase of its own (all in phase, every pick would start 3 dB louder,
//! at the beat's top). Its level is the
//! sketch's rule: each pick peaks at 2.2 times the chords channel's RMS over
//! the last 1.5 s, never under 0.06 nor over 0.16, so a sparse loop does not
//! drown under it and a busy one does not bury it. The tremolo moves every
//! string together: the note's phase is read from the core's clock against
//! the loop's start (`ZERO` in the extra values), so every pick that rings
//! dips at the same moment, as the sketch's one tremolo over its six strings
//! did.
//!
//! **Three versions** (the note's parts, `SLOW` and `BRIGHT`; soft is 0):
//! soft (a 4.5 Hz tremolo, 35% deep; the pick's cutoff 18 times the note, the
//! tail's 2.5), slow (tempo-synced, two cycles a beat of the loop's tempo,
//! `BEATS` in the extra values; 55% deep, rounder: 12 and 2), bright (5.5 Hz,
//! 45% deep, the brightest pick: 30 and 3.5).
//!
//! Where the core's version differs from the sketch, and why: the sketch was
//! a block on the whole chords channel that heard the chords through
//! `chords.RETUNE`; this is a voice the player plays from the chords it
//! knows, and, as the low bell does, it follows its channel
//! (`Voice::FOLLOWS`): its notes sound into the chords channel after the
//! channel's own blocks, each stamped with the channel's level without it
//! (`block::HEARD`), the mean square the sketch followed. The sketch rang six
//! strings, a new pick taking the oldest at once; here each pick is a note,
//! and the player lets the oldest go as a seventh starts (`event::LET_GO`),
//! over 50 ms rather than at once, so the taken string does not click. A
//! string that is not taken ends once its fall is 60 dB under its peak (17
//! s; the sketch rang it to 1e-6). A let-go at a full stop damps every string
//! over 0.3 s, as the sketch's release did. Centered: it has no width of its
//! own, and it draws nothing.

// Its levels go into the 32-bit signal.
#![allow(clippy::cast_possible_truncation)]

use crate::QUANTUM;
use crate::block::{Block, Context, Event, HEARD, NO_EXTRA, Note, Probe, Signal, Voice, event};
use crate::math;

use super::midi_to_freq;

/// The note's parts: the slow version (a tempo-synced tremolo, rounder).
pub const SLOW: u32 = 1;
/// The note's parts: the bright version (a brighter pick).
pub const BRIGHT: u32 = 2;

/// Where the note's extra values hold the loop's beats a second (the slow
/// version's tremolo is two cycles a beat; NaN: 60 BPM).
pub const BEATS: usize = 0;
/// Where they hold the time, on the core's clock, the tremolo's phase is 0
/// at: the loop's start (NaN: 0), so every pick of a loop shares one
/// tremolo.
pub const ZERO: usize = 1;
/// Where they hold the two saws' phases at the pick (0 to 1; NaN: 0): the
/// player hands each pick the phases its string's saws have run to since
/// that string's last pick (`string_phases`), as the sketch's six strings,
/// never reset, carried them.
pub const PHASES: usize = 2;

/// A string's two saws' phases at `time`, from its last pick of `midi` at
/// `then` with phases `was` (a fresh string, `then` NaN: both 0): where
/// the sketch's string, never reset, would have run them to, its pitch
/// settle included. The player keeps the six strings' last picks.
#[must_use]
pub fn string_phases(was: [f64; 2], midi: f64, then: f64, time: f64) -> [f64; 2] {
    if !then.is_finite() || !midi.is_finite() {
        return [0.0; 2];
    }
    let f = midi_to_freq(midi);
    let fs = [f, f * math::pow(2.0, DETUNE / 1200.0)];
    let held = (time - then).max(0.0);
    // The settle's extra turn: 8 cents sharp falling to 0 over 40 ms.
    let settle = (math::pow(2.0, SHARP / 1200.0) - 1.0) * SHARP_FALL * (1.0 - math::exp(-held / SHARP_FALL));
    let mut out = [0.0; 2];
    for ((o, &w), &f) in out.iter_mut().zip(was.iter()).zip(fs.iter()) {
        let x = w + f * (held + settle);
        *o = x - math::floor(x);
    }
    out
}

/// A version's pick and tremolo: the cutoff at the pick and of the tail,
/// in multiples of the note; the tremolo's depth, its rate in Hz (when not
/// synced) and its cycles a beat (0: free). The sketch's `variant.py`.
struct Version {
    bright: f64,
    round: f64,
    depth: f64,
    hz: f64,
    per_beat: f64,
}

/// soft, slow and bright, by the note's parts.
const VERSIONS: [Version; 3] = [
    Version { bright: 18.0, round: 2.5, depth: 0.35, hz: 4.5, per_beat: 0.0 },
    Version { bright: 12.0, round: 2.0, depth: 0.55, hz: 0.0, per_beat: 2.0 },
    Version { bright: 30.0, round: 3.5, depth: 0.45, hz: 5.5, per_beat: 0.0 },
];

/// A pick's peak at the least (the chords silent), its peak against the
/// chords' RMS, and the most it peaks at, whatever they do.
const LEVEL: f64 = 0.06;
const REL: f64 = 2.2;
const MOST: f64 = 0.16;
/// The pluck's fall and the tail's, in seconds (to 1/e), and the pluck's
/// share of the peak.
const PLUCK: f64 = 0.25;
const TAIL: f64 = 2.8;
const PLUCK_SHARE: f64 = 0.55;
/// The cutoff's fall from bright to round, in seconds (to 1/e).
const SETTLE: f64 = 0.12;
/// The pitch's settle: cents sharp at the pick, and its fall (to 1/e).
const SHARP: f64 = 8.0;
const SHARP_FALL: f64 = 0.04;
/// The second saw's detune, in cents.
const DETUNE: f64 = 4.0;
/// The highest cutoff, in Hz.
const TOP: f64 = 7000.0;
/// The filter's damping (1/Q).
const DAMPING: f64 = 1.25;
/// The attack, in seconds.
const ATTACK: f64 = 0.002;
/// A let-go's fall when it names none, in seconds to -60 dB (the sketch's
/// release at a full stop).
const DAMP: f64 = 0.3;
/// How far under its peak a string falls before the note ends, in dB.
const END_DB: f64 = 60.0;
/// How often the filter's coefficients are worked out, in frames.
const EVERY: u64 = 16;

/// How long a pick rings at most, in seconds: its tail 60 dB under the
/// peak (the player holds the note this long, `Synth::twang`).
#[must_use]
pub fn ring() -> f64 {
    ATTACK + TAIL * math::log(math::pow(10.0, END_DB / 20.0) * (1.0 - PLUCK_SHARE))
}

/// A band-limited saw's correction at phase `t` for a step of `dt` (polyBLEP).
fn blep(t: f64, dt: f64) -> f64 {
    if t < dt {
        let x = t / dt;
        x + x - x * x - 1.0
    } else if t > 1.0 - dt {
        let x = (t - 1.0) / dt;
        x * x + x + x + 1.0
    } else {
        0.0
    }
}

/// A twang pick: one string.
pub struct Twang {
    /// The two saws' phases and steps.
    phase: [f64; 2],
    step: [f64; 2],
    /// The pluck's and the tail's levels, and their falls a frame.
    pluck: f64,
    tail: f64,
    fall: [f64; 2],
    /// The cutoff's and the pitch's settles (1 at the pick, falling to 0),
    /// and their falls a frame.
    bright: f64,
    sharp: f64,
    settle: f64,
    sharp_fall: f64,
    /// The cutoff's range, in multiples of the note.
    hi: f64,
    lo: f64,
    /// The attack's envelope and its rise a frame.
    env: f64,
    up: f64,
    /// The note's frequency, the filter's state and coefficients.
    freq: f64,
    s: [f64; 2],
    a: [f64; 3],
    /// The tremolo: its depth, and its phase as a turning pair (cosine,
    /// sine) and the turn a frame.
    depth: f64,
    trem: [f64; 2],
    turn: [f64; 2],
    /// When it was picked, in seconds and as a frame; when it ends; the
    /// peak it falls from.
    time: f64,
    begin: u64,
    end: u64,
    peak: f64,
    rate: f64,
    /// A let-go's frame and its fall a frame (0: none).
    damp_at: u64,
    damp: f64,
}

impl Twang {
    /// Silent and all zeros, so a pool slot holding one costs nothing in the
    /// `.wasm`; starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        Twang {
            phase: [0.0; 2],
            step: [0.0; 2],
            pluck: 0.0,
            tail: 0.0,
            fall: [0.0; 2],
            bright: 0.0,
            sharp: 0.0,
            settle: 0.0,
            sharp_fall: 0.0,
            hi: 0.0,
            lo: 0.0,
            env: 0.0,
            up: 0.0,
            freq: 0.0,
            s: [0.0; 2],
            a: [0.0; 3],
            depth: 0.0,
            trem: [0.0; 2],
            turn: [0.0; 2],
            time: 0.0,
            begin: 0,
            end: 0,
            peak: 0.0,
            rate: 0.0,
            damp_at: 0,
            damp: 0.0,
        }
    }

    /// Where a time in seconds falls, as a frame.
    #[allow(clippy::cast_sign_loss)] // clamped at 0 first
    fn frame(&self, time: f64) -> u64 {
        math::round(time.max(0.0) * self.rate) as u64
    }

    /// The filter's coefficients for the cutoff now.
    fn tune(&mut self) {
        let fc = (self.freq * (self.lo + (self.hi - self.lo) * self.bright)).min(TOP).min(0.45 * self.rate);
        let w = core::f64::consts::PI * fc / self.rate;
        let g = math::sin(w) / math::cos(w);
        let a1 = 1.0 / (1.0 + g * (g + DAMPING));
        self.a = [a1, g * a1, g * g * a1];
    }
}

/// A probe's extra values: 60 BPM, the tremolo's phase 0 at the start.
const PROBE: [f64; crate::EXTRA] = {
    let mut x = NO_EXTRA;
    x[BEATS] = 1.0;
    x[ZERO] = 0.0;
    x
};

impl Block for Twang {
    type Settings = Note;

    const PROBES: &'static [Probe<Note>] = &[
        Probe { name: "soft", settings: Note::probe(crate::TWANG, crate::CHORDS, 45.0, 0, PROBE) },
        Probe { name: "slow", settings: Note::probe(crate::TWANG, crate::CHORDS, 40.0, SLOW, PROBE) },
        Probe { name: "bright", settings: Note::probe(crate::TWANG, crate::CHORDS, 47.0, BRIGHT, PROBE) },
    ];

    const HEARS: &'static [u32] = &[event::LET_GO];

    fn start(&mut self, note: &Note, cx: &Context) {
        *self = Twang::new();
        let rate = cx.rate;
        self.rate = rate;
        self.time = note.time;
        let v = VERSIONS.get(note.parts as usize).unwrap_or(&VERSIONS[0]);
        let heard = note.extra[HEARD];
        let heard = if heard > 0.0 { heard } else { 0.0 };
        self.peak = (REL * math::sqrt(heard)).clamp(LEVEL, MOST) * note.vel.clamp(0.0, 1.0);
        let f = midi_to_freq(note.midi);
        self.freq = f;
        self.step = [f / rate, f * math::pow(2.0, DETUNE / 1200.0) / rate];
        self.begin = self.frame(note.time);
        // The saws start where the player says its string's ran to, so the
        // pair's beat (4 cents apart) comes in at a phase of its own.
        for (k, p) in self.phase.iter_mut().enumerate() {
            let w = note.extra[PHASES + k];
            *p = if w.is_finite() { w - math::floor(w) } else { 0.0 };
        }
        self.pluck = self.peak * PLUCK_SHARE;
        self.tail = self.peak * (1.0 - PLUCK_SHARE);
        self.fall = [math::exp(-1.0 / (PLUCK * rate)), math::exp(-1.0 / (TAIL * rate))];
        self.bright = 1.0;
        self.sharp = 1.0;
        self.settle = math::exp(-1.0 / (SETTLE * rate));
        self.sharp_fall = math::exp(-1.0 / (SHARP_FALL * rate));
        self.hi = v.bright;
        self.lo = v.round;
        self.up = 1.0 / (ATTACK * rate);
        self.end = self.begin + self.frame(ring());
        // The tremolo, its phase read from the clock at the pick's first
        // frame against the loop's start.
        let beats = if note.extra[BEATS] > 0.0 { note.extra[BEATS] } else { 1.0 };
        let zero = if note.extra[ZERO].is_finite() { note.extra[ZERO] } else { 0.0 };
        let hz = if v.per_beat > 0.0 { v.per_beat * beats } else { v.hz };
        #[allow(clippy::cast_precision_loss)] // a frame of the clock, exact to 2^53
        let at = (self.begin as f64 / rate - zero) * hz;
        let at = at - math::floor(at);
        let w = core::f64::consts::TAU * at;
        self.trem = [math::cos(w), math::sin(w)];
        let dw = core::f64::consts::TAU * hz / rate;
        self.turn = [math::cos(dw), math::sin(dw)];
        self.depth = v.depth;
        self.tune();
    }

    fn span(&self) -> (u64, u64) {
        (self.begin, self.end)
    }

    fn event(&mut self, e: &Event) {
        // A let-go reaches the strings picked at or before its value; the
        // pick it comes with passes it by.
        if e.kind != event::LET_GO || self.time > e.value || e.time < self.time {
            return;
        }
        let seconds = if e.notes[0] > 0.0 { e.notes[0] } else { DAMP };
        let at = self.frame(e.time);
        if self.damp == 0.0 || at < self.damp_at {
            self.damp_at = at;
            self.damp = math::pow(10.0, -3.0 / (seconds * self.rate));
            self.end = self.end.min(at + self.frame(seconds));
        }
    }

    #[allow(clippy::float_cmp)] // a fresh string's 1.0 is exact
    fn render(&mut self, block: u64, _: &Context, out: &mut Signal) {
        #[allow(clippy::cast_possible_truncation)] // a frame within the block
        let lo = (self.begin.saturating_sub(block)).min(QUANTUM as u64) as usize;
        let sharp = math::pow(2.0, SHARP / 1200.0) - 1.0;
        let gone = self.peak * math::pow(10.0, -END_DB / 20.0);
        for (frame, y) in (block + lo as u64..).zip(out.iter_mut().skip(lo)) {
            if frame >= self.end {
                break;
            }
            if self.damp > 0.0 && frame == self.damp_at {
                self.fall = [self.fall[0].min(self.damp), self.fall[1].min(self.damp)];
            }
            if frame % EVERY == 0 && self.bright != 1.0 {
                self.tune();
            }
            let bend = 1.0 + sharp * self.sharp;
            let mut saw = 0.0;
            for o in 0..2 {
                let dt = self.step[o] * bend;
                let p = self.phase[o];
                saw += 2.0 * p - 1.0 - blep(p, dt);
                self.phase[o] += dt;
                if self.phase[o] >= 1.0 {
                    self.phase[o] -= 1.0;
                }
            }
            self.env = (self.env + self.up).min(1.0);
            let v = 0.5 * saw * self.env * (self.pluck + self.tail);
            // The lowpass (a two-pole state-variable filter, trapezoidal).
            let v3 = v - self.s[1];
            let v1 = self.a[0] * self.s[0] + self.a[1] * v3;
            let v2 = self.s[1] + self.a[1] * self.s[0] + self.a[2] * v3;
            self.s = [2.0 * v1 - self.s[0], 2.0 * v2 - self.s[1]];
            self.pluck *= self.fall[0];
            self.tail *= self.fall[1];
            self.bright *= self.settle;
            self.sharp *= self.sharp_fall;
            // The tremolo, the same dip as every string of the loop.
            let dip = 0.5 * self.depth * (1.0 - self.trem[0]);
            let [c, s] = self.trem;
            self.trem = [c * self.turn[0] - s * self.turn[1], s * self.turn[0] + c * self.turn[1]];
            *y += (v2 * (1.0 - dip)) as f32;
            if self.pluck + self.tail < gone {
                self.end = frame + 1;
                break;
            }
        }
    }
}

impl Voice for Twang {
    /// It sounds after its channel's own blocks, knowing the channel's level.
    const FOLLOWS: bool = true;
}

impl Default for Twang {
    fn default() -> Self {
        Self::new()
    }
}
