//! Ears 13b sketch: twang, a synth voice in the spirit of a tremolo baritone
//! guitar (Mikey, 2026-11-08: a synth in its spirit, not an imitation).
//!
//! Low and mid register, a bright plucked attack that falls quickly into a
//! rounder tail, and a slow amplitude tremolo over everything that rings,
//! at home in a big room with the wobble. Nothing in it is a guitar: each
//! string is two band-limited saws 4 cents apart through a two-pole lowpass
//! whose cutoff starts at `bright` times the note and falls over 0.12 s to
//! `round` times it, with a two-stage fall (most of the pluck gone in a
//! quarter second, a long tail after), and a pitch that settles from 8 cents
//! sharp over 40 ms, as a plucked string does.
//!
//! The melody channel hears no note events (the core sends a channel only
//! retunes, damps, bends and chokes, and the melody none of them), so this
//! follows the chord layer's events, as lowBell does: a source on the
//! chords channel (`where.txt`) hearing a retune at each chord through
//! `chords.RETUNE=1` (item 99). At each chord whose root changes (or the same
//! root after `REGAP` seconds) it picks the chord slowly, a beat apart: the
//! lowest note's pitch class between E2 and D#3 (82 to 156 Hz, above the 60
//! Hz guard), its fifth, then its tenth (the chord's third an octave up; the
//! octave when it has none). Six strings, the oldest taken by a new pick; a
//! release (a full stop) damps them.
//!
//! The tremolo moves the level of every string together: `rate` Hz, or, with
//! `per_beat` set, that many cycles a beat of `bpm` (tempo-synced, phase 0 at
//! the loop's start), down by `depth` at its lowest. Centered. Its level
//! follows the chord layer's (it hears the chords' signal before adding its
//! own): each pick peaks at `rel` times the chords' level over the last 1.5
//! s, never under `level`, never over `MOST`.
//!
//! `variant.py` writes the versions (soft, slow, bright) for one loop's
//! tempo into `core/sketch/`; this file is the soft one at 60 BPM.

use crate::block::{Block, Channel, Context, Event, Probe, Signal, event};
use crate::math;

/// How twang is set.
#[derive(Clone, Copy)]
pub struct Settings {
    /// A pick's peak level at the least (the chords silent).
    pub level: f64,
    /// Its peak against the chord layer's RMS level.
    pub rel: f64,
    /// The cutoff at the pick, in multiples of the note's frequency.
    pub bright: f64,
    /// The cutoff of the tail, likewise.
    pub round: f64,
    /// The tremolo's depth (0 to 1: how far the level dips).
    pub depth: f64,
    /// Its rate in Hz (when `per_beat` is 0).
    pub rate: f64,
    /// Its cycles a beat (0: free, at `rate`).
    pub per_beat: f64,
    /// The loop's tempo (the picks' spacing, and a synced tremolo's).
    pub bpm: f64,
    /// A chord root picked at the start (MIDI; NaN: none, silent until a chord).
    pub start: f64,
}

/// The settings the registry starts it with (the soft version at 60 BPM).
pub const REST: Settings = Settings { level: 0.06, rel: 2.2, bright: 18.0, round: 2.5, depth: 0.35, rate: 4.5, per_beat: 0.0, bpm: 60.0, start: f64::NAN };

/// The loudest a pick starts, whatever the chords do.
const MOST: f64 = 0.16;
/// How long the chords' level is followed over, in seconds.
const FOLLOW: f64 = 1.5;
/// The lowest root it picks (E2, 82 Hz); up to 11 above.
const LOW: f64 = 40.0;
/// The same root picked again after this many seconds.
const REGAP: f64 = 3.0;
/// How many strings ring at once.
const STRINGS: usize = 6;
/// The picks of a chord: root, fifth, tenth (or the octave).
const PICKS: usize = 3;
/// The pluck's fall and the tail's, in seconds (to 1/e), and the pluck's share.
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
/// A release's fall, in seconds to -60 dB.
const DAMP: f64 = 0.3;
/// How often the filter's coefficients are worked out, in frames.
const EVERY: usize = 16;

#[derive(Clone, Copy)]
struct Str {
    phase: [f64; 2],
    step: [f64; 2],
    pluck: f64,
    tail: f64,
    fall: [f64; 2],
    bright: f64,
    sharp: f64,
    env: f64,
    freq: f64,
    s: [f64; 2],
    a: [f64; 3],
    on: bool,
}

const QUIET: Str = Str { phase: [0.0; 2], step: [0.0; 2], pluck: 0.0, tail: 0.0, fall: [0.0; 2], bright: 0.0, sharp: 0.0, env: 0.0, freq: 0.0, s: [0.0; 2], a: [0.0; 3], on: false };

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

/// twang: its strings, the picks waiting, and what it heard.
pub struct Sketch {
    strings: [Str; STRINGS],
    next: usize,
    s: Settings,
    rate: f64,
    root: f64,
    at: f64,
    heard: f64,
    follow: f64,
    waiting: [(f64, f64); PICKS],
    trem: f64,
    trem_step: f64,
    k: [f64; 4],
}

impl Sketch {
    /// Silent, all zeros.
    #[must_use]
    pub const fn new() -> Self {
        Sketch {
            strings: [QUIET; STRINGS],
            next: 0,
            s: Settings { level: 0.0, rel: 0.0, bright: 0.0, round: 0.0, depth: 0.0, rate: 0.0, per_beat: 0.0, bpm: 0.0, start: f64::NAN },
            rate: 0.0,
            root: 0.0,
            at: 0.0,
            heard: 0.0,
            follow: 0.0,
            waiting: [(f64::NAN, 0.0); PICKS],
            trem: 0.0,
            trem_step: 0.0,
            k: [0.0; 4],
        }
    }

    /// Pick `midi` on the next string.
    fn pick(&mut self, midi: f64) {
        let f = 440.0 * math::pow(2.0, (midi - 69.0) / 12.0);
        let peak = (self.s.rel * math::sqrt(self.heard)).max(self.s.level).min(MOST);
        let rate = self.rate;
        let st = &mut self.strings[self.next];
        st.freq = f;
        st.step = [f / rate, f * math::pow(2.0, DETUNE / 1200.0) / rate];
        st.pluck = peak * PLUCK_SHARE;
        st.tail = peak * (1.0 - PLUCK_SHARE);
        st.fall = [self.k[0], self.k[1]];
        st.bright = 1.0;
        st.sharp = 1.0;
        st.env = 0.0;
        st.on = true;
        self.next = (self.next + 1) % STRINGS;
    }

    /// The picks of the chord on `root` (a pitch class over E2) with `third`
    /// (3, 4, or none), from `time`, a beat apart.
    fn schedule(&mut self, root: f64, third: Option<f64>, time: f64) {
        let beat = 60.0 / self.s.bpm.max(20.0);
        let low = LOW + root;
        let tops = [low, low + 7.0, low + 12.0 + third.unwrap_or(0.0)];
        let mut at = time;
        for (w, &m) in self.waiting.iter_mut().zip(tops.iter()) {
            *w = (at, m);
            at += beat;
        }
    }
}

impl Default for Sketch {
    fn default() -> Self {
        Self::new()
    }
}

impl Block for Sketch {
    type Settings = Settings;

    const PROBES: &'static [Probe<Settings>] = &[
        Probe { name: "soft", settings: Settings { start: 45.0, ..REST } },
        Probe { name: "slow", settings: Settings { bright: 12.0, round: 2.0, depth: 0.55, rate: 0.0, per_beat: 2.0, bpm: 58.0, start: 40.0, ..REST } },
        Probe { name: "bright", settings: Settings { bright: 30.0, round: 3.5, depth: 0.45, rate: 5.5, per_beat: 0.0, bpm: 67.0, start: 47.0, ..REST } },
    ];

    const HEARS: &'static [u32] = &[event::RETUNE, event::RELEASE];

    fn start(&mut self, s: &Settings, cx: &Context) {
        *self = Sketch::new();
        self.s = *s;
        self.rate = cx.rate;
        #[allow(clippy::cast_precision_loss)]
        let frames = crate::QUANTUM as f64;
        self.follow = 1.0 - math::exp(-frames / (FOLLOW * cx.rate));
        self.k = [math::exp(-1.0 / (PLUCK * cx.rate)), math::exp(-1.0 / (TAIL * cx.rate)), math::exp(-1.0 / (SETTLE * cx.rate)), math::exp(-1.0 / (SHARP_FALL * cx.rate))];
        let hz = if s.per_beat > 0.0 { s.per_beat * s.bpm / 60.0 } else { s.rate };
        self.trem_step = hz / cx.rate;
        self.root = f64::NAN;
        self.at = -REGAP;
        if !s.start.is_nan() {
            let mut pc = (s.start - LOW) % 12.0;
            if pc < 0.0 {
                pc += 12.0;
            }
            self.schedule(pc, Some(4.0), 0.0);
        }
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    #[allow(clippy::float_cmp)] // pitch classes: whole numbers
    fn event(&mut self, e: &Event) {
        match e.kind {
            event::RETUNE => {
                let chord = e.chord();
                let Some(low) = chord.iter().copied().reduce(f64::min) else { return };
                let mut pc = (low - LOW) % 12.0;
                if pc < 0.0 {
                    pc += 12.0;
                }
                if pc == self.root && e.time - self.at < REGAP {
                    return;
                }
                let has = |d: f64| chord.iter().any(|&m| ((m - low) % 12.0 + 12.0) % 12.0 == d);
                let third = if has(4.0) {
                    Some(4.0)
                } else if has(3.0) {
                    Some(3.0)
                } else {
                    None
                };
                self.root = pc;
                self.at = e.time;
                self.schedule(pc, third, e.time);
            }
            event::RELEASE => {
                let fall = math::pow(10.0, -3.0 / (DAMP * self.rate));
                for st in &mut self.strings {
                    st.fall = [st.fall[0].min(fall), st.fall[1].min(fall)];
                }
                self.waiting = [(f64::NAN, 0.0); PICKS];
                self.root = f64::NAN;
            }
            _ => {}
        }
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::float_cmp)] // a fresh string's 1.0 is exact
    fn render(&mut self, block: u64, _: &Context, signal: &mut Signal) {
        // The chords' own level, before twang adds to it.
        let mut ms = 0.0;
        for &x in signal.iter() {
            ms += f64::from(x) * f64::from(x);
        }
        let n = signal.len() as f64;
        let clock = block as f64 * n / self.rate;
        self.heard += self.follow * (ms / n - self.heard);
        let rate = self.rate;
        let (settle, sharp_fall) = (self.k[2], self.k[3]);
        let (bright, round) = (self.s.bright, self.s.round);
        let up = 1.0 / (ATTACK * rate);
        let sharp = math::pow(2.0, SHARP / 1200.0) - 1.0;
        for (i, x) in signal.iter_mut().enumerate() {
            let now = clock + i as f64 / rate;
            for w in 0..PICKS {
                let (due, midi) = self.waiting[w];
                if !due.is_nan() && due <= now {
                    self.waiting[w].0 = f64::NAN;
                    self.pick(midi);
                }
            }
            let mut y = 0.0;
            for st in &mut self.strings {
                if !st.on {
                    continue;
                }
                if i % EVERY == 0 || st.bright == 1.0 {
                    let fc = (st.freq * (round + (bright - round) * st.bright)).min(TOP).min(0.45 * rate);
                    let w = core::f64::consts::PI * fc / rate;
                    let g = math::sin(w) / math::cos(w);
                    let a1 = 1.0 / (1.0 + g * (g + DAMPING));
                    st.a = [a1, g * a1, g * g * a1];
                }
                let bend = 1.0 + sharp * st.sharp;
                let mut saw = 0.0;
                for o in 0..2 {
                    let dt = st.step[o] * bend;
                    let p = st.phase[o];
                    saw += 2.0 * p - 1.0 - blep(p, dt);
                    st.phase[o] += dt;
                    if st.phase[o] >= 1.0 {
                        st.phase[o] -= 1.0;
                    }
                }
                st.env = (st.env + up).min(1.0);
                let v = 0.5 * saw * st.env * (st.pluck + st.tail);
                // The lowpass (a two-pole state-variable filter, trapezoidal).
                let v3 = v - st.s[1];
                let v1 = st.a[0] * st.s[0] + st.a[1] * v3;
                let v2 = st.s[1] + st.a[1] * st.s[0] + st.a[2] * v3;
                st.s = [2.0 * v1 - st.s[0], 2.0 * v2 - st.s[1]];
                y += v2;
                st.pluck *= st.fall[0];
                st.tail *= st.fall[1];
                st.bright *= settle;
                st.sharp *= sharp_fall;
                if st.pluck + st.tail < 1e-6 {
                    st.on = false;
                    st.s = [0.0; 2];
                }
            }
            let dip = 0.5 * self.s.depth * (1.0 - math::cos(core::f64::consts::TAU * self.trem));
            self.trem += self.trem_step;
            if self.trem >= 1.0 {
                self.trem -= 1.0;
            }
            *x += (y * (1.0 - dip)) as f32;
        }
    }
}

// It adds its own sound to its channel's whole signal, which it listens to.
impl Channel for Sketch {
    const WHOLE: bool = true;
}
