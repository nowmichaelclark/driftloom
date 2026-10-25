//! subDuck (queue item 68; Ears 8, approved by Mikey 2026-10-18): the bass
//! channel alone dips at each kick and comes back, so the kick and the sub
//! take turns while the pads and the tune stay steady.
//!
//! A channel's own block on the whole bass channel (`Channel::WHOLE`), keyed
//! by the drums' channel, routed in (`Core::route`), as queue item 46c's
//! sidechain demonstration is. The sketch (`ears8-sketch.patch`,
//! `e8Duck`) dipped the bass channel's gain at each kick's time: a line down
//! to `depth` dB, scaled by the kick's velocity (full at 0.8), in 8 ms, and
//! a line back over `recover` seconds. Here the kick is heard in the key: its
//! low band (under 150 Hz, where the drums' channel is the kick: the snare's
//! and the clap's bodies, the rim and the hand drums sit above it, the hats
//! far above) rising by `RISE` from where it rested is a kick, and how loud
//! it rises in its first `PEAK_FOR` is its velocity. Then the same two lines.
//! The key is the drums' previous block (the route's order), so the dip
//! starts a block (2.7 ms at 48 kHz) after the kick.
//!
//! Its settings, set for the loop by the host (`Core::set`), are the depth
//! in dB (0 is off, and costs nothing) and the recovery in seconds.

// Times in frames, and levels in 32-bit floats, as the signal is.
#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::cast_sign_loss)]

use crate::block::{Block, Channel, Context, Mod, Mods, Probe, Setting, Signal};
use crate::math;

use super::filter::{Biquad, Kind};

/// How a subDuck is set: its depth at full velocity, in dB, and how long it
/// takes to come back, in seconds.
#[derive(Clone, Copy)]
pub struct Settings {
    /// The dip at a kick of velocity 0.8 or more, in dB; 0 is off.
    pub depth: f32,
    /// How long the line back takes, in seconds.
    pub recover: f32,
}

/// At rest: off.
pub const REST: Settings = Settings { depth: 0.0, recover: 0.2 };

/// Its settings a host sets, by number: the depth, then the recovery.
pub const DEPTH: usize = 0;
/// The recovery.
pub const RECOVER: usize = 1;
/// Its one setting a signal drives: the key, the drums' channel.
pub const KEY: usize = 0;

/// The line down, in seconds (the sketch's 8 ms).
const DOWN: f32 = 0.008;
/// Where the key is the kick: under this, in Hz.
const LOW: f32 = 150.0;
/// A kick: the key's low band rising this far over where it rested (8 dB),
/// and past `FLOOR` (-20 dBFS: a kick at velocity 0.15). A ghost hit or a
/// kick's own click can reach -28 dBFS there a few ms ahead of a kick, and
/// would take the kick's place, at its own small depth.
const RISE: f32 = 2.5;
const FLOOR: f32 = 0.1;
/// After a kick, the key is listened to again once it has fallen this far
/// under the kick's peak (6 dB), and no sooner than `REST_FOR` seconds.
const FALL: f32 = 0.5;
const REST_FOR: f32 = 0.04;
/// How long after a kick its peak is taken, in seconds: a kick and a soft
/// kick reach theirs in 4 and 9 ms.
const PEAK_FOR: f32 = 0.012;
/// The key's low-band peak of a kick at velocity 0.8, today's or the tuned
/// one (0.68, measured: under its 0.88 at full band, the lowpass and the
/// sweep from 128 Hz taking the rest): the sketch's full depth.
const FULL: f32 = 0.68;
/// A Butterworth lowpass's Q, in dB (a `BiquadFilterNode`'s Q for a lowpass).
const BUTTERWORTH_DB: f32 = -3.010_3;
/// How fast the key's level falls between peaks (a 5 ms time constant).
const LEVEL_FALL: f64 = 0.005;

/// Where the gain is going.
#[derive(Clone, Copy, PartialEq)]
enum Phase {
    /// Untouched.
    Still,
    /// The line down, from `from` at frame `at` of it.
    Down,
    /// The line back.
    Up,
}

/// The subDuck: its settings, the key's filter and level, and the gain's
/// line.
pub struct SubDuck {
    depth: f32,
    recover: f32,
    key: Biquad,
    level: f32,
    fall: f32,
    /// The key's lowest level since it was last listened to.
    low: f32,
    /// Whether it listens for a kick, and the last kick's peak.
    armed: bool,
    peak: f32,
    /// Frames since the last kick.
    since: u32,
    phase: Phase,
    /// How far the gain is taken down now (the gain is 1 less this), where
    /// its line started, and where the line goes: zeros at rest, so the
    /// static core stays all zeros and out of the `.wasm`'s data.
    cut: f32,
    from: f32,
    to: f32,
    /// Frames into the line, and its length.
    at: u32,
    len: u32,
    rate: f32,
}

impl SubDuck {
    /// A subDuck at rest, all zeros.
    #[must_use]
    pub const fn new() -> Self {
        SubDuck {
            depth: 0.0,
            recover: 0.0,
            key: Biquad::new(),
            level: 0.0,
            fall: 0.0,
            low: 0.0,
            armed: false,
            peak: 0.0,
            since: 0,
            phase: Phase::Still,
            cut: 0.0,
            from: 0.0,
            to: 0.0,
            at: 0,
            len: 0,
            rate: 0.0,
        }
    }

    /// The dip for a kick whose key peaked at `peak`: how far the gain is
    /// taken down.
    fn floor_for(&self, peak: f32) -> f32 {
        let db = self.depth * (peak / FULL).min(1.0);
        1.0 - math::powf(10.0, -db / 20.0)
    }

    /// Hear one frame of the key's low band, `k`: a new kick starts the line
    /// down, and the peak of the one under way sets how deep it goes.
    fn hear(&mut self, k: f32) {
        let a = k.abs();
        self.level = if a > self.level { a } else { self.level * self.fall };
        self.since = self.since.saturating_add(1);
        let peaking = (self.since as f32) < PEAK_FOR * self.rate;
        if peaking && self.level > self.peak {
            self.peak = self.level;
            if self.phase == Phase::Down {
                self.to = self.floor_for(self.peak);
            }
        }
        if !self.armed {
            if !peaking && (self.since as f32) >= REST_FOR * self.rate && self.level <= FALL * self.peak {
                self.armed = true;
                self.low = self.level;
            }
            return;
        }
        self.low = self.low.min(self.level);
        if self.level > FLOOR && self.level > RISE * self.low {
            self.armed = false;
            self.since = 0;
            self.peak = self.level;
            self.phase = Phase::Down;
            self.from = self.cut;
            self.to = self.floor_for(self.peak);
            self.at = 0;
            self.len = ((DOWN * self.rate) as u32).max(1);
        }
    }

    /// The gain at the next frame, along its line.
    fn step(&mut self) -> f32 {
        match self.phase {
            Phase::Still => {}
            Phase::Down | Phase::Up => {
                self.at += 1;
                let t = (self.at as f32 / self.len as f32).min(1.0);
                self.cut = self.from + (self.to - self.from) * t;
                if self.at >= self.len {
                    if self.phase == Phase::Down {
                        self.phase = Phase::Up;
                        self.from = self.cut;
                        self.to = 0.0;
                        self.at = 0;
                        self.len = ((self.recover * self.rate) as u32).max(1);
                    } else {
                        self.phase = Phase::Still;
                        self.cut = 0.0;
                    }
                }
            }
        }
        1.0 - self.cut
    }
}

impl Default for SubDuck {
    fn default() -> Self {
        Self::new()
    }
}

impl Block for SubDuck {
    type Settings = Settings;

    const PROBES: &'static [Probe<Settings>] = &[Probe { name: "kick", settings: Settings { depth: 7.0, recover: 0.2 } }];

    const INPUT: bool = true;

    // The kit keys it with a 3 Hz sine of 0.5: a kick at about half
    // velocity, six times a second.
    const MODS: &'static [Mod] = &[Mod { name: "key", swing: 0.5 }];

    // The kit can only hear these with a key routed in, which it never
    // routes with a setting, so they are not probed (NaN); the probe's key
    // is heard through `MODS`.
    const SETTINGS: &'static [Setting] = &[Setting { name: "depth", probe: f64::NAN }, Setting { name: "recover", probe: f64::NAN }];

    fn start(&mut self, s: &Settings, cx: &Context) {
        *self = SubDuck::new();
        self.depth = s.depth;
        self.recover = s.recover;
        self.rate = cx.rate as f32;
        self.fall = math::exp(-1.0 / (LEVEL_FALL * cx.rate)) as f32;
        // A Butterworth lowpass: its Q in dB, as a `BiquadFilterNode`'s (1/sqrt 2).
        self.key.set(Kind::Lowpass, LOW, BUTTERWORTH_DB, 0.0, self.rate);
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    // A setting is a 32-bit float, as an `AudioParam`'s value is.
    fn set(&mut self, setting: usize, value: f64, _time: f64, _cx: &Context) {
        match setting {
            DEPTH => self.depth = value as f32,
            RECOVER => self.recover = value as f32,
            _ => {}
        }
    }

    fn render(&mut self, block: u64, cx: &Context, signal: &mut Signal) {
        self.render_mod(block, cx, signal, &Mods::NONE);
    }

    /// Off (no depth), or with no key routed in, the channel passes as it is
    /// and it costs nothing.
    fn render_mod(&mut self, _: u64, _: &Context, signal: &mut Signal, mods: &Mods) {
        if self.depth <= 0.0 {
            return;
        }
        let Some(key) = mods.get(KEY) else { return };
        for (x, k) in signal.iter_mut().zip(key.iter()) {
            let low = self.key.step(*k);
            self.hear(low);
            *x *= self.step();
        }
        self.key.flush();
    }
}

// It dips everything its channel plays.
impl Channel for SubDuck {
    const WHOLE: bool = true;
}
