//! The tuned kick (queue item 67; Ears 8's tunedKick, approved by Mikey
//! 2026-10-18): today's kick or soft kick, the same sweep, landing on a
//! target note instead of 44 or 46 Hz, falling at a rate the loop draws,
//! and for a long boom ringing on at a tail level after today's punch. As
//! the sketch played it (`ears8-sketch.patch`, `Synth.drum`'s `K` branch):
//! a sine and its gain, no click.
//!
//! One kick at a time, as on a drum machine: the host sends a `CHOKE` to
//! the drums' channel with every hit of a loop that plays it, and a kick
//! still ringing then falls away in a few ms (the sketch's
//! `cancelAndHoldAtTime` and `setTargetAtTime(0.0001, t, 0.005)`), so a
//! boom never piles up under the next one.
//!
//! A note's `parts` say which kick (`drum::KICK` or `drum::SOFTKICK`); its
//! extra values are what the host works out at load (`js/choices.js`,
//! `kickOf`): `[0]` the target in Hz, `[1]` the fall in dB a second, `[2]`
//! the lift that keeps it at today's loudness by ear, `[3]` the tail's
//! level against the peak (1 for none: a short kick). It draws nothing.

// Web Audio's parameters are 32-bit floats, set from the host's 64-bit
// numbers: a voice narrows every value it hands one.
#![allow(clippy::cast_possible_truncation)]

use crate::block::{Block, Context, Event, Note, Probe, Signal, Voice, event};
use crate::{EXTRA, QUANTUM};
use crate::math;
use crate::param::Param;

use super::drum::{KICK, SOFTKICK};
use super::osc::{Part, TableOsc, frame_at};
use super::shared::{UNITY, pitch_of, starts_in};

/// The soft kick's level (`SOFTKICK_LEVEL`) and the kick's (1.1).
const SOFTKICK_LEVEL: f64 = 1.15;
const KICK_LEVEL: f64 = 1.1;
/// Today's fall, in dB a second, which a long boom's punch keeps down to
/// its tail (the kick's 0.0001 at 0.38 s).
const PUNCH: f64 = 213.0;
/// Where the gain ends, as today's kick ends: -80 dB.
const QUIET: f64 = 0.0001;
/// The tail's lowest level, so the ramp to it is never to zero.
const TAIL_FLOOR: f64 = 0.0002;
/// The shortest fall from the peak, in seconds.
const SHORTEST: f64 = 0.05;
/// A choke's time constant, in seconds, and how long after it the voice is
/// done (twelve time constants: under -100 dB).
const CHOKE_TAU: f64 = 0.005;
const CHOKE_DONE: f64 = 0.06;

/// A tuned kick's values, as a host sends them: the target, the fall, the
/// lift and the tail.
#[must_use]
pub const fn extra(hz: f64, dbps: f64, lift: f64, tail: f64) -> [f64; EXTRA] {
    let mut x = [f64::NAN; EXTRA];
    x[0] = hz;
    x[1] = dbps;
    x[2] = lift;
    x[3] = tail;
    x
}

/// A tuned kick note.
pub struct TunedKick {
    body: TableOsc,
    pitch: Param,
    gain: Param,
    /// When it was struck, in seconds: a choke at or before it is its own
    /// hit's, and passes it by.
    time: f64,
    /// The rate it plays at, for a choke's frame.
    rate: f64,
    /// When the soft kick's linear swell ends (the kick's start for the
    /// kick, whose rise is exponential).
    swell: f64,
    end: u64,
}

impl TunedKick {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`;
    /// starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        TunedKick {
            body: TableOsc::new(),
            pitch: Param::new(0.0),
            gain: Param::new(0.0),
            time: 0.0,
            rate: 0.0,
            swell: 0.0,
            end: 0,
        }
    }

    /// One kick: `soft` the soft kick's sweep and swell, landing on `hz`,
    /// at `vel` times `lift`; with a `tail` under 1, today's fall down to
    /// it, then the drawn fall `dbps` from there; without, the drawn fall
    /// from the peak.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn play(&mut self, soft: bool, time: f64, vel: f64, hz: f64, dbps: f64, lift: f64, tail: f64, rate: f64) {
        let v = vel.clamp(0.0, 1.0);
        let tail = if tail.is_finite() && tail > 0.0 { tail.min(1.0) } else { 1.0 };
        let lift = if lift.is_finite() && lift > 0.0 { lift } else { 1.0 };
        let dbps = if dbps.is_finite() && dbps > 0.0 { dbps } else { PUNCH };
        let (from, sweep, att) = if soft { (96.0, 0.14, 0.009) } else { (128.0, 0.09, 0.004) };
        let peak = v * if soft { SOFTKICK_LEVEL } else { KICK_LEVEL } * lift;
        let db = |x: f64| 20.0 * f64::from(math::log10f(x as f32));
        let t1 = att + -db(tail) / PUNCH;
        let len = t1 + SHORTEST.max(db(peak * tail / QUIET) / dbps);
        // Its pitch rests at where it starts, as the frame drum's does (queue
        // item 55), so where a hit falls in its block never adds a blip of
        // 440 Hz to it.
        let p = &mut self.pitch;
        p.reset(from as f32);
        p.set_value_at_time(from as f32, time);
        p.exponential_ramp_to_value_at_time(hz as f32, time + sweep, time);
        let g = &mut self.gain;
        g.reset(UNITY);
        if soft {
            g.set_value_at_time(0.0, time);
            g.linear_ramp_to_value_at_time(peak as f32, time + att, time);
        } else {
            g.set_value_at_time(QUIET as f32, time);
            g.exponential_ramp_to_value_at_time(peak as f32, time + att, time);
        }
        if tail < 1.0 {
            g.exponential_ramp_to_value_at_time(TAIL_FLOOR.max(peak * tail) as f32, time + t1, time);
        }
        g.exponential_ramp_to_value_at_time(QUIET as f32, time + len, time);
        self.body.schedule(time, time + len + 0.02, rate);
        self.time = time;
        self.rate = rate;
        self.swell = if soft { time + att } else { time };
        self.end = self.body.stop_frame();
    }

    /// The next kick at `time`: from where its gain has got to, a fall in a
    /// few ms, and the voice done soon after. The ramp under way is cut at
    /// `time` and kept up to it (`cancelAndHoldAtTime`): the same curve to
    /// the value it has reached there, linear in the soft kick's swell,
    /// exponential after it.
    fn choke(&mut self, time: f64, rate: f64) {
        let g = &mut self.gain;
        let now = g.value_at(time);
        g.cancel_scheduled_values(time);
        if self.swell > time {
            g.linear_ramp_to_value_at_time(now, time, time);
        } else {
            g.exponential_ramp_to_value_at_time(now, time, time);
        }
        g.set_target_at_time(QUIET as f32, time, CHOKE_TAU);
        self.end = self.end.min(frame_at(time + CHOKE_DONE, rate).max(self.body.start_frame()));
    }
}

/// The probes: a short kick and a long boom on A#1, and the soft kick's
/// boom, each at today's loudness (lift 1).
const A_SHARP_1: f64 = 58.27;

impl Block for TunedKick {
    type Settings = Note;

    const PROBES: &'static [Probe<Note>] = &[
        Probe { name: "short", settings: Note::probe(crate::TUNEDKICK, crate::DRUMS_CHANNEL, 0.0, KICK, extra(A_SHARP_1, 220.0, 1.0, 1.0)) },
        Probe { name: "long", settings: Note::probe(crate::TUNEDKICK, crate::DRUMS_CHANNEL, 0.0, KICK, extra(A_SHARP_1, 62.0, 1.0, 0.25)) },
        Probe { name: "soft-long", settings: Note::probe(crate::TUNEDKICK, crate::DRUMS_CHANNEL, 0.0, SOFTKICK, extra(A_SHARP_1, 62.0, 1.0, 0.25)) },
    ];

    const HEARS: &'static [u32] = &[event::CHOKE];

    fn start(&mut self, note: &Note, cx: &Context) {
        let [hz, dbps, lift, tail, ..] = note.extra;
        self.play(note.parts == SOFTKICK, note.time, note.vel, hz, dbps, lift, tail, cx.rate);
    }

    /// The next kick's choke, at its time: never its own hit's.
    fn event(&mut self, event: &Event) {
        if event.kind == event::CHOKE && event.time > self.time {
            self.choke(event.time, self.rate);
        }
    }

    fn span(&self) -> (u64, u64) {
        (self.body.start_frame(), self.end)
    }

    fn render(&mut self, block: u64, cx: &Context, out: &mut Signal) {
        // Choked, it is done: the core keeps its slot to the end it was
        // started with, and it plays nothing there.
        if block >= self.end {
            return;
        }
        let rate = cx.rate;
        let mut freq = [0.0f32; QUANTUM];
        let first = starts_in(self.body.start_frame(), block);
        let pitch = pitch_of(&mut self.pitch, block, rate, first, &mut freq, Part::Steady(0.0));
        let mut tone = [0.0f32; QUANTUM];
        let (lo, hi) = self.body.render(block, &cx.waves.sine, &pitch, rate as f32, &mut tone);
        let mut gain = [0.0f32; QUANTUM];
        self.gain.fill(block, rate, &mut gain);
        for ((y, x), g) in out.iter_mut().zip(tone.iter()).zip(gain.iter()).take(hi).skip(lo) {
            *y += *x * *g;
        }
    }
}

impl Voice for TunedKick {}

impl Default for TunedKick {
    fn default() -> Self {
        Self::new()
    }
}
