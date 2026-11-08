//! Ears 13 sketch: lowBell, its swelling kind (a 0.4 s attack, as Kankyo Ongaku's Variation III swells in). Otherwise lowBell/block.rs exactly.
//!
//! Piles 3 to 6 (`docs/ears/batch-6.md`): a bell-like, inharmonic note in
//! the bass-to-tenor register (65 to 165 Hz) on eleven records (Peek, Yellow
//! Calx, Mummy Mix, Equation, Windowlicker, Flow Coma, Orban Eq Trx 4,
//! Inkey$, Crowsmangegus, Lifeforms' four picks, 2814's 05, Bowery
//! Electric's Looped, Kankyo Ongaku's Variation III). Driftloom's bells
//! (glass, templebell, tubular, the chimes) all ring above it.
//!
//! A source on the chords channel (`where.txt`) that hears the chords: the
//! core sends a retune at each chord only while the chapel organ plays, so
//! the clips force the organ on and pin its level, dry and room to 0 (a
//! silent organ renders the loop bit for bit as without it). At each chord
//! whose root changes (or the same root again after `REGAP` seconds) it
//! strikes the root's pitch class between D2 and C#3, so the fundamental
//! stays above the 60 Hz guard. Four partials 60 to 68 cents off whole
//! multiples (the records' bells read 60 to 92 cents off), the higher ones
//! falling faster; two rings, the older taken by a new strike. A release
//! (the organ letting go at a full stop) damps it over a quarter second.
//! Centered: it has no width of its own. Its level follows the chord
//! layer's (a whole-channel source: it hears the chords' signal before it
//! adds its own), so a sparse loop does not drown under it and a busy one
//! does not bury it: each strike's first partial peaks at `rel` times the
//! chords' level over the last 1.5 s, never under `level`.

use crate::block::{Block, Channel, Context, Event, Probe, Signal, event};
use crate::math;

/// How the bell is set.
#[derive(Clone, Copy)]
pub struct Settings {
    /// The first partial's peak level at the least (the chords silent).
    pub level: f64,
    /// Its peak against the chord layer's RMS level.
    pub rel: f64,
    /// How long a strike takes to reach its peak, in seconds.
    pub attack: f64,
    /// A note struck at the start (MIDI; NaN: none, silent until a chord).
    pub start: f64,
}

/// The settings the registry starts it with: silent until a chord.
pub const REST: Settings = Settings { level: 0.045, rel: 1.4, attack: 0.4, start: f64::NAN };

/// The loudest a strike starts, whatever the chords do.
const MOST: f64 = 0.12;
/// How long the chords' level is followed over, in seconds.
const FOLLOW: f64 = 1.5;

/// The partials: ratio to the fundamental, level, fall in dB a second.
const PARTIALS: [(f64, f64, f64); 4] = [(1.0, 1.0, 11.0), (2.08, 0.42, 19.0), (3.11, 0.2, 29.0), (5.2, 0.08, 44.0)];
/// The lowest note it strikes (D2, 73 Hz); it strikes up to 11 above.
const LOW: f64 = 38.0;
/// The same root struck again after this many seconds.
const REGAP: f64 = 4.0;
/// How many notes ring at once.
const RINGS: usize = 2;
/// A release's fall, in seconds to -60 dB.
const DAMP: f64 = 0.25;

#[derive(Clone, Copy)]
struct Ring {
    phase: [f64; 4],
    step: [f64; 4],
    amp: [f64; 4],
    fall: [f64; 4],
    env: f64,
    on: bool,
}

const QUIET: Ring = Ring { phase: [0.0; 4], step: [0.0; 4], amp: [0.0; 4], fall: [0.0; 4], env: 0.0, on: false };

/// The bell: its rings, which one strikes next, and the last root.
pub struct Sketch {
    rings: [Ring; RINGS],
    next: usize,
    level: f64,
    rel: f64,
    rise: f64,
    rate: f64,
    root: f64,
    at: f64,
    heard: f64,
    follow: f64,
}

impl Sketch {
    /// Silent, all zeros.
    #[must_use]
    pub const fn new() -> Self {
        Sketch { rings: [QUIET; RINGS], next: 0, level: 0.0, rel: 0.0, rise: 0.0, rate: 0.0, root: 0.0, at: 0.0, heard: 0.0, follow: 0.0 }
    }

    fn strike(&mut self, midi: f64) {
        let f = 440.0 * math::pow(2.0, (midi - 69.0) / 12.0);
        let rate = self.rate;
        let peak = (self.rel * math::sqrt(self.heard)).max(self.level).min(MOST);
        let ring = &mut self.rings[self.next];
        for (k, &(ratio, amp, dbps)) in PARTIALS.iter().enumerate() {
            ring.phase[k] = 0.0;
            ring.step[k] = f * ratio / rate;
            ring.amp[k] = amp * peak;
            ring.fall[k] = math::pow(10.0, -dbps / 20.0 / rate);
        }
        ring.env = 0.0;
        ring.on = true;
        self.next = (self.next + 1) % RINGS;
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
        Probe { name: "struck", settings: Settings { level: 0.03, rel: 1.4, attack: 0.035, start: 45.0 } },
        Probe { name: "swell", settings: Settings { level: 0.03, rel: 1.4, attack: 0.4, start: 40.0 } },
    ];

    const HEARS: &'static [u32] = &[event::RETUNE, event::RELEASE];

    fn start(&mut self, s: &Settings, cx: &Context) {
        *self = Sketch::new();
        self.rate = cx.rate;
        self.level = s.level;
        self.rel = s.rel;
        #[allow(clippy::cast_precision_loss)]
        let frames = crate::QUANTUM as f64;
        self.follow = 1.0 - math::exp(-frames / (FOLLOW * cx.rate));
        self.rise = 1.0 / (s.attack.max(0.001) * cx.rate);
        self.root = f64::NAN;
        self.at = -REGAP;
        if !s.start.is_nan() {
            self.strike(s.start);
        }
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    fn event(&mut self, e: &Event) {
        match e.kind {
            event::RETUNE => {
                let Some(root) = e.chord().iter().copied().reduce(f64::min) else { return };
                let mut pc = (root - LOW) % 12.0;
                if pc < 0.0 {
                    pc += 12.0;
                }
                if pc == self.root && e.time - self.at < REGAP {
                    return;
                }
                self.root = pc;
                self.at = e.time;
                self.strike(LOW + pc);
            }
            event::RELEASE => {
                let fall = math::pow(10.0, -3.0 / (DAMP * self.rate));
                for ring in &mut self.rings {
                    for f in &mut ring.fall {
                        *f = f.min(fall);
                    }
                }
                self.root = f64::NAN;
            }
            _ => {}
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn render(&mut self, _: u64, _: &Context, signal: &mut Signal) {
        // The chords' own level, before the bell adds to it.
        let mut ms = 0.0;
        for &x in signal.iter() {
            ms += f64::from(x) * f64::from(x);
        }
        #[allow(clippy::cast_precision_loss)]
        let n = signal.len() as f64;
        self.heard += self.follow * (ms / n - self.heard);
        let rise = self.rise;
        for ring in &mut self.rings {
            if !ring.on {
                continue;
            }
            for x in signal.iter_mut() {
                ring.env = (ring.env + rise).min(1.0);
                let mut y = 0.0;
                for k in 0..4 {
                    y += ring.amp[k] * math::sin(core::f64::consts::TAU * ring.phase[k]);
                    ring.amp[k] *= ring.fall[k];
                    ring.phase[k] += ring.step[k];
                    if ring.phase[k] >= 1.0 {
                        ring.phase[k] -= 1.0;
                    }
                }
                *x += (ring.env * y) as f32;
            }
            if ring.amp[0] < 1e-6 {
                ring.on = false;
            }
        }
    }
}

// It adds its own sound to its channel's whole signal, which it listens to.
impl Channel for Sketch {
    const WHOLE: bool = true;
}
