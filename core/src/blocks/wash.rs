//! The wash (queue item 62; Ears 4, round 5, approved by Mikey 2026-10-16):
//! a slow swell of air on a channel of its own (`WASH_CHANNEL`), every 17 to
//! 26 seconds, whose level follows the loop.
//!
//! Two loops of the core's noise, 0.73 s apart (the sketch's left and right:
//! the mono render hears them folded, as Web Audio folds a stereo source into
//! a mono one), through two gentle lowpasses that sweep from 450 Hz up to
//! about 2.5 kHz and back, and two soft resonances tuned to the loop's key
//! (its root or fifth in 600-1200 Hz, and an octave and a fifth above it), like
//! wind across a bottle. Each swell rises and falls over 0.8 of the period;
//! the rest of it the wash is all but silent.
//!
//! Its level follows the other five layers: a signal routed in from each
//! (`Core::route`, its `MODS`), weighted by that channel's gain as the mix
//! sets it (a route taps a channel before its gain, and a muted layer's
//! weight is 0), rectified and smoothed twice over 0.7 s, sets the swell's
//! gain. So it is heard under a beat, stays behind a sparse loop, and fades
//! with the loop into a rest. Every number is the host's, set for the loop
//! (`SETTINGS`, `Core::set`), as the sketch drew them (`ears4-sketch.patch`,
//! `_washBuild`, round 5).

// Web Audio's parameters and curves are 32-bit floats, set from the host's
// 64-bit numbers; frame counts are positive and small.
#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::cast_sign_loss)]

use core::f64::consts::PI;

use crate::block::{Block, Channel, Context, Mod, Mods, Probe, Setting, Signal};
use crate::math;
use crate::QUANTUM;

use super::filter::{Biquad, Kind};
use super::noise_source::NoiseSource;

/// The settings a host sets for the loop, by number (`SETTINGS`).
pub mod set {
    /// The swell's period, in seconds. Setting it starts the swells again
    /// from the setting's time (the first half a period on), and lets the
    /// last ones fade; 0 lets the wash go.
    pub const PERIOD: usize = 0;
    /// The follower's gain: the swell's peak against the loop's level.
    pub const LEVEL: usize = 1;
    /// The low resonance, in Hz.
    pub const TINT: usize = 2;
    /// The low resonance's boost, in dB.
    pub const TINT_GAIN: usize = 3;
    /// The high resonance's (three times the low's), in dB.
    pub const TINT_HIGH: usize = 4;
    /// Both resonances' Q.
    pub const TINT_Q: usize = 5;
    /// The first of the five layers' weights in the follower, in channel
    /// order (the melody's, the chords', the bass's, the textures', the
    /// drums'): each channel's gain in the mix, 0 for one muted.
    pub const WEIGHT: usize = 6;
    /// How many settings there are.
    pub const COUNT: usize = 11;
}

/// The layers it listens to: one route each, in channel order.
pub const LAYERS: usize = 5;

/// How it is set when it starts: every setting, and a steady level the
/// follower hears with no route in (0 in a loop; the kit's probes use it as
/// the loop it would follow).
#[derive(Clone, Copy)]
pub struct Settings {
    /// The settings, by `set` number.
    pub values: [f64; set::COUNT],
    /// What the follower hears besides its routes.
    pub hear: f64,
}

/// At rest: silent, until a host sets its period.
pub const REST: Settings = Settings { values: [0.0; set::COUNT], hear: 0.0 };

/// The swell's curve: 256 points over 0.8 of the period, as the sketch's
/// `setValueCurveAtTime` takes them.
const POINTS: usize = 256;
/// Its share of the period.
const SPAN: f64 = 0.8;
/// The gain at rest, between swells and before the first.
const QUIET: f64 = 0.0001;
/// The lowpasses' corner at the swell's foot, and how far it opens
/// (450 Hz x 5.5 at the top: about 2.5 kHz).
const FOOT: f64 = 450.0;
const OPEN: f64 = 5.5;
/// Before the first swell, the sketch's filters stand at their defaults:
/// 350 Hz and 20 kHz.
const FIRST: [f64; 2] = [350.0, 20_000.0];
/// The lowpasses' Q (Web Audio's, in dB).
const Q: f32 = 0.5;
/// The two noises' offsets into the noise, in seconds.
const OFFSETS: [f64; 2] = [0.0, 0.73];
/// The follower's two smoothing stages, each a time constant in seconds.
const FOLLOW: f64 = 0.7;
/// How fast the last loop's swell fades when a new one starts (its gain's
/// time constant, in seconds), and how long its sound lasts.
const FADE: f64 = 0.3;
const FADE_FOR: f64 = 2.0;

/// A loop's swells: from when, how often.
#[derive(Clone, Copy)]
struct Schedule {
    start: f64,
    period: f64,
}

impl Schedule {
    const NONE: Schedule = Schedule { start: 0.0, period: 0.0 };

    /// Where `t` is on the curve, 0 to 1, if a swell is under way.
    fn at(self, t: f64) -> Option<f64> {
        if self.period <= 0.0 {
            return None;
        }
        let first = self.start + self.period / 2.0;
        if t < first {
            return None;
        }
        let into = (t - first) % self.period;
        (into < SPAN * self.period).then(|| into / (SPAN * self.period))
    }

    /// Whether a swell has begun yet.
    fn begun(self, t: f64) -> bool {
        self.period > 0.0 && t >= self.start + self.period / 2.0
    }
}

/// The wash.
pub struct Wash {
    values: [f64; set::COUNT],
    hear: f64,
    on: bool,
    now: Schedule,
    /// The last loop's swells, fading from `faded` (when the new ones were
    /// set) at the gain they had then.
    last: Schedule,
    faded: f64,
    from: f64,
    noise: [NoiseSource; 2],
    lows: [Biquad; 2],
    tints: [Biquad; 2],
    follow: [f64; 2],
    a: f64,
    rate: f64,
    noise_len: usize,
    /// The curve's points: its shape, 0 to 1 (`sin(pi x)^1.4`).
    shape: [f32; POINTS],
}

impl Wash {
    /// Silent and all zeros (the core holds one in every channel, in its
    /// static memory).
    #[must_use]
    pub const fn new() -> Self {
        Wash {
            values: [0.0; set::COUNT],
            hear: 0.0,
            on: false,
            now: Schedule::NONE,
            last: Schedule::NONE,
            faded: 0.0,
            from: 0.0,
            noise: [NoiseSource::unstarted(), NoiseSource::unstarted()],
            lows: [Biquad::new(), Biquad::new()],
            tints: [Biquad::new(), Biquad::new()],
            follow: [0.0; 2],
            a: 0.0,
            rate: 0.0,
            noise_len: 0,
            shape: [0.0; POINTS],
        }
    }

    fn value(&self, n: usize) -> f64 {
        self.values.get(n).copied().unwrap_or(0.0)
    }

    /// The curve at `u` (0 to 1), as Web Audio reads a value curve: the
    /// points drawn straight between.
    fn curve(&self, at: f64) -> f64 {
        let place = at * (POINTS - 1) as f64;
        let point = place as usize;
        let frac = place - point as f64;
        let here = f64::from(self.shape.get(point).copied().unwrap_or(0.0));
        let next = f64::from(self.shape.get(point + 1).copied().unwrap_or(0.0));
        here + (next - here) * frac
    }

    /// The swell's gain at `t`: this loop's, and the last's fading.
    fn gain(&self, t: f64) -> f64 {
        let ours = self.now.at(t).map_or(QUIET, |u| QUIET + self.curve(u));
        let fade = if self.faded > 0.0 && t >= self.from && t < self.from + FADE_FOR {
            (self.faded - QUIET) * math::exp(-(t - self.from) / FADE)
        } else {
            0.0
        };
        ours + fade
    }

    /// The lowpasses' corners at `t`, the first's and the second's: the
    /// last loop's swell's while it fades, else this one's.
    fn corners(&self, t: f64) -> [f64; 2] {
        let s = if self.faded > 0.0 && t < self.from + FADE_FOR { self.last } else { self.now };
        match s.at(t) {
            Some(u) => {
                let f = FOOT * math::pow(OPEN, self.curve(u));
                [f, f]
            }
            None if s.begun(t) => [FOOT, FOOT],
            None => FIRST,
        }
    }

    /// The swells from `time` on, every `period` seconds; the ones before
    /// fade (`_washBuild`, a new loop's).
    fn restart(&mut self, period: f64, time: f64) {
        if self.on {
            self.faded = self.gain(time);
            self.from = time;
            self.last = self.now;
        }
        self.now = Schedule { start: time, period };
        if period > 0.0 {
            for (n, &offset) in self.noise.iter_mut().zip(OFFSETS.iter()) {
                n.looped_at(time, offset, f64::INFINITY, 1.0, self.noise_len, self.rate);
            }
            self.on = true;
        }
        self.set_tints();
    }

    fn set_tints(&mut self) {
        let rate = self.rate as f32;
        let hz = self.value(set::TINT).max(10.0);
        let q = self.value(set::TINT_Q).max(0.1) as f32;
        let (low_gain, high_gain) = (self.value(set::TINT_GAIN) as f32, self.value(set::TINT_HIGH) as f32);
        let [low, high] = &mut self.tints;
        low.set(Kind::Peaking, hz as f32, q, low_gain, rate);
        high.set(Kind::Peaking, (hz * 3.0) as f32, q, high_gain, rate);
    }

    /// One block into `out`, following `mods` (the five layers).
    fn play(&mut self, block: u64, cx: &Context, out: &mut Signal, mods: &Mods) {
        let start = block as f64 / cx.rate;
        if !self.on {
            return;
        }
        let end = (block + QUANTUM as u64) as f64 / cx.rate;
        // Let go: nothing to come, and the last swell faded out.
        if self.now.period <= 0.0 && end >= self.from + FADE_FOR {
            self.on = false;
            self.faded = 0.0;
            return;
        }
        if self.faded > 0.0 && start >= self.from + FADE_FOR {
            self.faded = 0.0;
        }
        // The corners for the block, at its middle: the swell takes seconds
        // where a block is 2.7 ms (48 kHz), so this is within a hertz of
        // moving them every frame.
        let mid = f64::midpoint(start, end);
        let corners = self.corners(mid);
        let rate = cx.rate as f32;
        for (lp, &f) in self.lows.iter_mut().zip(corners.iter()) {
            lp.set(Kind::Lowpass, f.min(cx.rate / 2.0) as f32, Q, 0.0, rate);
        }
        let mut sum = [0.0f32; QUANTUM];
        let mut grain = [0.0f32; QUANTUM];
        for n in &mut self.noise {
            n.fill(block, cx.noise, &mut grain);
            for (y, &x) in sum.iter_mut().zip(grain.iter()) {
                *y += 0.5 * x;
            }
        }
        let weights: [f64; LAYERS] = core::array::from_fn(|i| self.value(set::WEIGHT + i));
        let level = self.value(set::LEVEL);
        let a = self.a;
        for (i, (y, &x)) in out.iter_mut().zip(sum.iter()).enumerate() {
            let mut heard = self.hear;
            for (k, &w) in weights.iter().enumerate() {
                if let Some(s) = mods.get(k).and_then(|s| s.get(i)) {
                    heard += w * f64::from(*s);
                }
            }
            let rect = heard.abs().min(1.0);
            let [s1, s2] = self.follow;
            let s1 = (1.0 - a) * rect + a * s1;
            let s2 = (1.0 - a) * s1 + a * s2;
            self.follow = [s1, s2];
            let t = (block + i as u64) as f64 / cx.rate;
            let g = self.gain(t);
            let [l1, l2] = &mut self.lows;
            let [t1, t2] = &mut self.tints;
            let v = t2.step(t1.step(l2.step(l1.step(x))));
            *y += v * (g * level * s2) as f32;
        }
        for f in self.lows.iter_mut().chain(self.tints.iter_mut()) {
            f.flush();
        }
        for s in &mut self.follow {
            *s = math::flush(*s);
        }
    }
}

impl Default for Wash {
    fn default() -> Self {
        Self::new()
    }
}

/// The kit's probe: a swell every 6 s (so one rises inside the probe's 12
/// s), tuned to A (880 Hz, and 2640), following a steady loop.
const PROBE: [f64; set::COUNT] = [6.0, 0.9, 880.0, 5.5, 3.0, 4.0, 0.58, 0.44, 0.62, 0.5, 0.82];

impl Block for Wash {
    type Settings = Settings;

    const PROBES: &'static [Probe<Settings>] = &[Probe { name: "swell", settings: Settings { values: PROBE, hear: 0.1 } }];

    // The kit swings each layer's route by 0.3, on top of the steady loop.
    const MODS: &'static [Mod] = &[
        Mod { name: "melody", swing: 0.3 },
        Mod { name: "chords", swing: 0.3 },
        Mod { name: "bass", swing: 0.3 },
        Mod { name: "texture", swing: 0.3 },
        Mod { name: "drums", swing: 0.3 },
    ];

    const SETTINGS: &'static [Setting] = &[
        Setting { name: "period", probe: 4.0 },
        Setting { name: "level", probe: 0.3 },
        Setting { name: "tint", probe: 660.0 },
        Setting { name: "tint-gain", probe: 12.0 },
        Setting { name: "tint-high", probe: 12.0 },
        Setting { name: "tint-q", probe: 1.0 },
        // A weight changes nothing with no route in (the kit sets settings
        // with nothing routed): the kit's routed probes hear them instead.
        Setting { name: "melody", probe: f64::NAN },
        Setting { name: "chords", probe: f64::NAN },
        Setting { name: "bass", probe: f64::NAN },
        Setting { name: "texture", probe: f64::NAN },
        Setting { name: "drums", probe: f64::NAN },
    ];

    fn start(&mut self, s: &Settings, cx: &Context) {
        *self = Wash::new();
        self.rate = cx.rate;
        self.noise_len = cx.noise.len();
        self.a = math::exp(-1.0 / (FOLLOW * cx.rate));
        for (i, p) in self.shape.iter_mut().enumerate() {
            *p = math::pow(math::sin(PI * i as f64 / (POINTS - 1) as f64), 1.4) as f32;
        }
        self.values = s.values;
        self.hear = s.hear;
        self.restart(self.value(set::PERIOD), 0.0);
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    fn set(&mut self, setting: usize, value: f64, time: f64, _: &Context) {
        if let Some(v) = self.values.get_mut(setting) {
            *v = value;
        }
        match setting {
            set::PERIOD => self.restart(value, time),
            set::TINT | set::TINT_GAIN | set::TINT_HIGH | set::TINT_Q => self.set_tints(),
            _ => {}
        }
    }

    fn render(&mut self, block: u64, cx: &Context, signal: &mut Signal) {
        self.play(block, cx, signal, &Mods::NONE);
    }

    fn render_mod(&mut self, block: u64, cx: &Context, signal: &mut Signal, mods: &Mods) {
        self.play(block, cx, signal, mods);
    }
}

// A source on its own bus in the wash's channel.
impl Channel for Wash {}
