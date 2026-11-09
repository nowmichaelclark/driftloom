//! Ears 13b sketch: dropSpotlight, the low bell alone in a floor drop.
//!
//! Batch 6's #11 (batch 5's five records: Polygon Window, Bradley's Robot,
//! Analogue Bubblebath 3, Caustic Window, Alberto Balsalm): when the kick and
//! the bass leave, one part steps forward. Its instrument is batch 6's
//! approved lowBell (the same partials and register, D2 to C#3), heard only
//! in a drop.
//!
//! A source on the chords channel (`where.txt`) that declares its route
//! (item 99): mod 0 is the drums layer and mod 1 the bass, routed in by the
//! clip (`"route": {"sketch": "drums", "sketch.1": "bass"}`). It hears the
//! floor as the drums' kick band (twice through a 150 Hz one-pole lowpass,
//! so hats that keep playing in a drop do not count) plus the bass, smoothed
//! over 50 ms, against the loudest it has been lately (a peak that falls over
//! 6 s). When both stay 25 dB under that for `HOLD` seconds a drop has begun:
//! it strikes the chord's root, and the root of each chord after while the
//! drop lasts. Once the floor is back (12 dB under the peak or louder) every
//! ring is damped over `DAMP` seconds: the bell belongs to the drop. A full
//! stop (the chords silent too) strikes nothing.
//!
//! It hears the chords through `chords.RETUNE=1` (item 99), no silent organ.
//! Centered. Its level follows the chord layer's (it hears the chords'
//! signal before adding its own): each strike's first partial peaks at `rel`
//! times the chords' level over the last 1.5 s, never under `level`, never
//! over `MOST`.

use crate::block::{Block, Channel, Context, Event, Mod, Mods, Probe, Signal, event};
use crate::math;

/// How the spotlight is set.
#[derive(Clone, Copy)]
pub struct Settings {
    /// The first partial's peak level at the least (the chords silent).
    pub level: f64,
    /// Its peak against the chord layer's RMS level.
    pub rel: f64,
    /// How long a strike takes to reach its peak, in seconds.
    pub attack: f64,
    /// A note struck at the start (MIDI; NaN: none).
    pub start: f64,
    /// 1: it starts in a drop (for the kit's probes); 0: it waits for one.
    pub dropped: f64,
}

/// The settings the registry starts it with: waiting for a drop.
pub const REST: Settings = Settings { level: 0.05, rel: 2.0, attack: 0.035, start: f64::NAN, dropped: 0.0 };

/// The loudest a strike starts, whatever the chords do.
const MOST: f64 = 0.15;
/// How long the chords' level is followed over, in seconds.
const FOLLOW: f64 = 1.5;
/// The partials (lowBell's): ratio to the fundamental, level, fall in dB a second.
const PARTIALS: [(f64, f64, f64); 4] = [(1.0, 1.0, 11.0), (2.08, 0.42, 19.0), (3.11, 0.2, 29.0), (5.2, 0.08, 44.0)];
/// The lowest note it strikes (D2, 73 Hz); it strikes up to 11 above.
const LOW: f64 = 38.0;
/// The same root struck again after this many seconds.
const REGAP: f64 = 4.0;
/// How many notes ring at once.
const RINGS: usize = 2;
/// The floor's damp, in seconds to -60 dB.
const DAMP: f64 = 1.5;
/// How long the floor must be gone before a drop begins, in seconds.
const HOLD: f64 = 1.2;
/// Under the peak by this much (as a power ratio, -25 dB): the floor is gone.
const GONE: f64 = 0.003_162;
/// Within this much of the peak (-12 dB): the floor is back.
const BACK: f64 = 0.063_1;
/// The floor's smoothing and the peak's fall, in seconds.
const SMOOTH: f64 = 0.05;
const PEAK_FALL: f64 = 6.0;
/// The kick band's lowpass, in Hz.
const KICK_BAND: f64 = 150.0;
/// The mods: the drums layer, and the bass.
const DRUMS: usize = 0;
const BASS: usize = 1;

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

/// The spotlight: its rings, the chord it last heard, and the floor.
pub struct Sketch {
    rings: [Ring; RINGS],
    next: usize,
    level: f64,
    rel: f64,
    rise: f64,
    rate: f64,
    root: f64,
    struck: f64,
    at: f64,
    heard: f64,
    follow: f64,
    lp: [f64; 2],
    lp_k: f64,
    floor: f64,
    peak: f64,
    smooth_k: f64,
    peak_k: f64,
    quiet: f64,
    dropped: bool,
    pending: bool,
}

impl Sketch {
    /// Silent, all zeros.
    #[must_use]
    pub const fn new() -> Self {
        Sketch {
            rings: [QUIET; RINGS],
            next: 0,
            level: 0.0,
            rel: 0.0,
            rise: 0.0,
            rate: 0.0,
            root: 0.0,
            struck: 0.0,
            at: 0.0,
            heard: 0.0,
            follow: 0.0,
            lp: [0.0; 2],
            lp_k: 0.0,
            floor: 0.0,
            peak: 0.0,
            smooth_k: 0.0,
            peak_k: 0.0,
            quiet: 0.0,
            dropped: false,
            pending: false,
        }
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

    /// Strike the chord's root if it differs from the last struck (or the
    /// same one after `REGAP`), at `time`.
    #[allow(clippy::float_cmp)] // pitch classes: whole numbers
    fn spot(&mut self, time: f64) {
        if self.root.is_nan() || (self.root == self.struck && time - self.at < REGAP) {
            return;
        }
        self.struck = self.root;
        self.at = time;
        self.strike(LOW + self.root);
    }

    fn damp(&mut self, seconds: f64) {
        let fall = math::pow(10.0, -3.0 / (seconds * self.rate));
        for ring in &mut self.rings {
            for f in &mut ring.fall {
                *f = f.min(fall);
            }
        }
    }

    /// The floor (the drums' kick band and the bass) over this block.
    fn hear_floor(&mut self, n: f64, mods: &Mods) -> Option<f64> {
        let (drums, bass) = (mods.get(DRUMS), mods.get(BASS));
        if drums.is_none() && bass.is_none() {
            return None;
        }
        let mut ms = 0.0;
        if let Some(d) = drums {
            for &x in d {
                self.lp[0] += self.lp_k * (f64::from(x) - self.lp[0]);
                self.lp[1] += self.lp_k * (self.lp[0] - self.lp[1]);
                ms += self.lp[1] * self.lp[1];
            }
        }
        if let Some(b) = bass {
            for &x in b {
                ms += f64::from(x) * f64::from(x);
            }
        }
        Some(ms / n)
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
        Probe { name: "struck", settings: Settings { level: 0.03, rel: 2.0, attack: 0.035, start: 45.0, dropped: 1.0 } },
        Probe { name: "swell", settings: Settings { level: 0.03, rel: 2.0, attack: 0.4, start: 40.0, dropped: 1.0 } },
    ];

    const MODS: &'static [Mod] = &[Mod { name: "drums", swing: 0.5 }, Mod { name: "bass", swing: 0.5 }];

    const HEARS: &'static [u32] = &[event::RETUNE, event::RELEASE];

    fn start(&mut self, s: &Settings, cx: &Context) {
        *self = Sketch::new();
        self.rate = cx.rate;
        self.level = s.level;
        self.rel = s.rel;
        #[allow(clippy::cast_precision_loss)]
        let frames = crate::QUANTUM as f64;
        self.follow = 1.0 - math::exp(-frames / (FOLLOW * cx.rate));
        self.smooth_k = 1.0 - math::exp(-frames / (SMOOTH * cx.rate));
        self.peak_k = math::exp(-frames / (PEAK_FALL * cx.rate));
        self.lp_k = 1.0 - math::exp(-core::f64::consts::TAU * KICK_BAND / cx.rate);
        self.rise = 1.0 / (s.attack.max(0.001) * cx.rate);
        self.root = f64::NAN;
        self.struck = f64::NAN;
        self.at = -REGAP;
        self.dropped = s.dropped != 0.0;
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
                let Some(low) = e.chord().iter().copied().reduce(f64::min) else { return };
                let mut pc = (low - LOW) % 12.0;
                if pc < 0.0 {
                    pc += 12.0;
                }
                self.root = pc;
                // Struck as the block that hears it renders, in a drop.
                self.pending = self.dropped;
            }
            event::RELEASE => {
                self.damp(0.25);
                self.root = f64::NAN;
            }
            _ => {}
        }
    }

    fn render(&mut self, block: u64, cx: &Context, signal: &mut Signal) {
        self.render_mod(block, cx, signal, &Mods::NONE);
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    fn render_mod(&mut self, block: u64, _: &Context, signal: &mut Signal, mods: &Mods) {
        let n = signal.len() as f64;
        let now = block as f64 * n / self.rate;
        // The chords' own level, before the bell adds to it.
        let mut ms = 0.0;
        for &x in signal.iter() {
            ms += f64::from(x) * f64::from(x);
        }
        self.heard += self.follow * (ms / n - self.heard);
        if let Some(floor) = self.hear_floor(n, mods) {
            self.floor += self.smooth_k * (floor - self.floor);
            self.peak = (self.peak * self.peak_k).max(self.floor);
            if self.floor > self.peak * BACK {
                if self.dropped || self.rings.iter().any(|r| r.on) {
                    self.damp(DAMP);
                }
                self.dropped = false;
                self.quiet = 0.0;
            } else if self.floor < self.peak * GONE && self.peak > 1e-9 {
                self.quiet += n / self.rate;
                if !self.dropped && self.quiet >= HOLD && self.heard > 1e-8 {
                    self.dropped = true;
                    self.pending = true;
                }
            }
        }
        if self.pending {
            self.pending = false;
            if self.dropped {
                self.spot(now);
            }
        }
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
