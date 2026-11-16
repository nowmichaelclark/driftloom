//! lowBell (queue item 101g; batch 6's #2, Ears 13, approved by Mikey
//! 2026-11-08, both kinds): a struck low bell on the chord's root, in a
//! register Driftloom's other bells never reach. The player strikes it
//! (`Synth::low_bell`) at each chord whose root changes, or the same root
//! again after a while, between D2 and C#3, so its fundamental stays above
//! the 60 Hz guard; this file is what it plays.
//!
//! What plays is the Ears 13 sketch's (`tools/ears/sketches/ears13/lowBell/`),
//! number for number: four sine partials at 1, 2.08, 3.11 and 5.2 times the
//! note (60 to 68 cents off whole multiples, the records' bells read 60 to
//! 92), the higher falling faster (11, 19, 29 and 44 dB a second), under a
//! linear attack of 35 ms, or 0.4 s for the swelling kind (`SWELL` in the
//! note's parts). Its level is the sketch's rule: each strike's first
//! partial peaks at 1.4 times the chords channel's RMS over the last 1.5 s,
//! never under 0.045 nor over 0.12, times the note's velocity (1 as the
//! player sends it), so a sparse loop does not drown under it and a busy one
//! does not bury it.
//!
//! Where the core's version differs from the sketch, and why: the sketch was
//! a block on the whole chords channel that heard the chords through a
//! silent organ; this is a voice the player plays from the chords it knows.
//! To keep the sketch's level rule and its place in the channel, it is the
//! first voice that follows its channel (`Voice::FOLLOWS`): its notes sound
//! into the chords channel after the channel's own blocks (padSweep, padWow
//! and colorTone leave it be, as they left the sketch), and each note is
//! stamped as it starts with the channel's level heard so far
//! (`block::HEARD`), the same mean square the sketch followed. The sketch
//! rang two notes at most, the older taken by a new strike; here each strike
//! is a note of its own that ends once its first partial is 60 dB under its
//! peak (5.5 s), so a ring is never cut off.
//!
//! dropSpotlight (queue item 101g; batch 6's #11, Ears 13b) is the same bell
//! stepping forward in a floor drop (`SPOT` in the parts, either kind): the
//! spotlight sketch's louder rule, 2 times the chords' RMS, never under 0.05
//! nor over 0.15.
//!
//! A choke (`event::CHOKE`, its value the fall in seconds to -60 dB, the
//! player's 0.25 at a full stop, 1.5 as a drop's floor comes back) damps every partial from its time; a choke
//! at or before the note's own time passes it by. Centered: it has no width
//! of its own, and it draws nothing.

// Its levels go into the 32-bit signal.
#![allow(clippy::cast_possible_truncation)]

use crate::QUANTUM;
use crate::block::{Block, Context, Event, HEARD, NO_EXTRA, Note, Probe, Signal, Voice, event};
use crate::math;

use super::midi_to_freq;

/// The note's parts: the swelling kind (a 0.4 s attack).
pub const SWELL: u32 = 1;
/// The note's parts: the bell stepping forward in a floor drop
/// (dropSpotlight, Ears 13b): the spotlight sketch's louder level rule.
pub const SPOT: u32 = 2;
/// The note's parts: struck where the chords have left too (a drop's first
/// strike over a chord gone quiet): its least level, whatever the channel
/// held a moment before, as the sketch, coming in a second late, heard.
pub const ALONE: u32 = 4;

/// The partials: ratio to the fundamental, level, fall in dB a second.
const PARTIALS: [(f64, f64, f64); 4] = [(1.0, 1.0, 11.0), (2.08, 0.42, 19.0), (3.11, 0.2, 29.0), (5.2, 0.08, 44.0)];
/// How many partials it has.
const PARTS: usize = PARTIALS.len();
/// The attacks, in seconds: struck, and swelling.
const STRUCK: f64 = 0.035;
const SWELLING: f64 = 0.4;
/// The first partial's peak at the least (the chords silent), its peak
/// against the chords' RMS, and the most it peaks at, whatever they do.
const LEVEL: f64 = 0.045;
const REL: f64 = 1.4;
const MOST: f64 = 0.12;
/// The same three in a floor drop (`SPOT`): the Ears 13b dropSpotlight
/// sketch's, a bell that steps forward as the kit and the bass leave.
const SPOT_LEVEL: f64 = 0.05;
const SPOT_REL: f64 = 2.0;
const SPOT_MOST: f64 = 0.15;
/// How far under its peak the first partial falls before the note ends, in
/// dB.
const END_DB: f64 = 60.0;
/// A choke's fall when it names none, in seconds to -60 dB (the sketch's
/// damp at a full stop).
const DAMP: f64 = 0.25;

/// A low bell note.
pub struct LowBell {
    phase: [f64; PARTS],
    step: [f64; PARTS],
    amp: [f64; PARTS],
    fall: [f64; PARTS],
    /// The attack's envelope and its rise a frame.
    env: f64,
    rise: f64,
    /// When it was struck, in seconds and as a frame; when it ends.
    time: f64,
    begin: u64,
    end: u64,
    rate: f64,
    /// A choke's frame and its fall a frame (0: none).
    damp_at: u64,
    damp: f64,
}

impl LowBell {
    /// Silent and all zeros, so a pool slot holding one costs nothing in the
    /// `.wasm`; starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        LowBell {
            phase: [0.0; PARTS],
            step: [0.0; PARTS],
            amp: [0.0; PARTS],
            fall: [0.0; PARTS],
            env: 0.0,
            rise: 0.0,
            time: 0.0,
            begin: 0,
            end: 0,
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
}

impl Block for LowBell {
    type Settings = Note;

    const PROBES: &'static [Probe<Note>] = &[
        Probe { name: "struck", settings: Note::probe(crate::LOWBELL, crate::CHORDS, 45.0, 0, NO_EXTRA) },
        Probe { name: "swell", settings: Note::probe(crate::LOWBELL, crate::CHORDS, 40.0, SWELL, NO_EXTRA) },
        Probe { name: "spot", settings: Note::probe(crate::LOWBELL, crate::CHORDS, 43.0, SPOT, NO_EXTRA) },
    ];

    const HEARS: &'static [u32] = &[event::CHOKE];

    fn start(&mut self, note: &Note, cx: &Context) {
        *self = LowBell::new();
        let rate = cx.rate;
        self.rate = rate;
        self.time = note.time;
        let heard = note.extra[HEARD];
        let heard = if heard > 0.0 && note.parts & ALONE == 0 { heard } else { 0.0 };
        let (level, rel, most) = if note.parts & SPOT != 0 { (SPOT_LEVEL, SPOT_REL, SPOT_MOST) } else { (LEVEL, REL, MOST) };
        let peak = (rel * math::sqrt(heard)).clamp(level, most) * note.vel.clamp(0.0, 1.0);
        let f = midi_to_freq(note.midi);
        for (k, &(ratio, amp, dbps)) in PARTIALS.iter().enumerate() {
            self.step[k] = f * ratio / rate;
            self.amp[k] = amp * peak;
            self.fall[k] = math::pow(10.0, -dbps / 20.0 / rate);
        }
        let attack = if note.parts & SWELL != 0 { SWELLING } else { STRUCK };
        self.rise = 1.0 / (attack * rate);
        self.begin = self.frame(note.time);
        self.end = self.begin + self.frame(attack + END_DB / PARTIALS[0].2);
    }

    fn span(&self) -> (u64, u64) {
        (self.begin, self.end)
    }

    fn event(&mut self, e: &Event) {
        if e.kind != event::CHOKE || e.time <= self.time {
            return;
        }
        let seconds = if e.value > 0.0 { e.value } else { DAMP };
        let at = self.frame(e.time);
        if self.damp == 0.0 || at < self.damp_at {
            self.damp_at = at;
            self.damp = math::pow(10.0, -3.0 / (seconds * self.rate));
            self.end = self.end.min(at + self.frame(seconds));
        }
    }

    fn render(&mut self, block: u64, _: &Context, out: &mut Signal) {
        #[allow(clippy::cast_possible_truncation)] // a frame within the block
        let lo = (self.begin.saturating_sub(block)).min(QUANTUM as u64) as usize;
        for (frame, y) in (block + lo as u64..).zip(out.iter_mut().skip(lo)) {
            if frame >= self.end {
                break;
            }
            if self.damp > 0.0 && frame == self.damp_at {
                for f in &mut self.fall {
                    *f = f.min(self.damp);
                }
            }
            self.env = (self.env + self.rise).min(1.0);
            let mut x = 0.0;
            for k in 0..PARTS {
                x += self.amp[k] * math::sin(core::f64::consts::TAU * self.phase[k]);
                self.amp[k] *= self.fall[k];
                self.phase[k] += self.step[k];
                if self.phase[k] >= 1.0 {
                    self.phase[k] -= 1.0;
                }
            }
            *y += (self.env * x) as f32;
        }
    }
}

impl Voice for LowBell {
    /// It sounds after its channel's own blocks, knowing the channel's level.
    const FOLLOWS: bool = true;
}

impl Default for LowBell {
    fn default() -> Self {
        Self::new()
    }
}
