//! Ears 13 sketch: colorTone, a held tone past the seventh over the chord.
//!
//! Piles 5 and 6 (`docs/ears/batch-6.md`; Ears 12b's colors reading): one or
//! two of a 9, 11 or 13 held over 26 to 46% of the chord time on Pyramid, 38
//! to 46% on Bubble Universe!, 51 to 95% on Holographic Universe(s)!, 41% on
//! Conscious Agents, 52 to 60% on two Triangular Reformat tracks and 86% on
//! Synthesist's So Weit, So Gut: mostly a 13 over major and dominant
//! sevenths and an 11 over minor chords. Driftloom's chords already add a
//! ninth now and then (`plan.rs`), never an 11 or a 13.
//!
//! A whole-channel source on the chords channel (`where.txt`) that hears the
//! chords through the silent organ (as lowBell does). On a chord with a
//! major third it holds the 13 (a major sixth over the root), on one with a
//! minor third the 11 (a fourth over the root): in every diatonic mode both
//! are in the scale over those chords, so no key is needed. Chords with
//! neither third (quartal, power, sus), with a flat fifth, or that hold the
//! color already, get none. It sounds on about half of the chords (a fixed
//! sequence, the same every render), voiced just above the chord's top note
//! (C6 at the most), a soft tone of two sines 5 cents apart and a little
//! second harmonic that swells in over 0.35 s and lets go over 0.6 s at the
//! next chord. Its level follows the chord layer's own, as lowBell's does.
//!
//! A loop whose keys arpeggiate sends one note a chord event, so the chord is
//! read from every note heard in the last `WINDOW` seconds, and the tone
//! changes only when that chord's root or third changes.

use crate::block::{Block, Channel, Context, Event, Probe, Signal, event};
use crate::math;

/// How the tone is set.
#[derive(Clone, Copy)]
pub struct Settings {
    /// Its level at the least (the chords silent).
    pub level: f64,
    /// Its level against the chord layer's RMS.
    pub rel: f64,
    /// How often a chord gets it, 0 to 1.
    pub share: f64,
    /// A chord held from the start (its root and third, MIDI; NaN: none).
    pub start: f64,
}

/// The settings the registry starts it with.
pub const REST: Settings = Settings { level: 0.02, rel: 1.2, share: 0.6, start: f64::NAN };

/// How long a heard note counts toward the chord, in seconds.
const WINDOW: f64 = 2.0;
/// How many heard notes it keeps.
const HEARD: usize = 12;

/// The loudest it plays.
const MOST: f64 = 0.08;
/// How long the chords' level is followed over, in seconds.
const FOLLOW: f64 = 1.5;
/// Swell and let-go, in seconds.
const SWELL: f64 = 0.35;
const GO: f64 = 0.6;
/// The highest it is voiced (C6).
const TOP: f64 = 84.0;

#[derive(Clone, Copy)]
struct Tone {
    phase: [f64; 3],
    step: [f64; 3],
    gain: f64,
    peak: f64,
    on: bool,
    midi: f64,
}

const QUIET: Tone = Tone { phase: [0.0; 3], step: [0.0; 3], gain: 0.0, peak: 0.0, on: false, midi: f64::NAN };

/// The tone: two of them, so one lets go while the next swells.
pub struct Sketch {
    tones: [Tone; 2],
    next: usize,
    level: f64,
    rel: f64,
    share: f64,
    rate: f64,
    heard: f64,
    follow: f64,
    up: f64,
    down: f64,
    draws: u32,
    notes: [(f64, f64); HEARD],
    write: usize,
    held: f64,
}

impl Sketch {
    /// Silent, all zeros.
    #[must_use]
    pub const fn new() -> Self {
        Sketch {
            tones: [QUIET; 2],
            next: 0,
            level: 0.0,
            rel: 0.0,
            share: 0.0,
            rate: 0.0,
            heard: 0.0,
            follow: 0.0,
            up: 0.0,
            down: 0.0,
            draws: 0,
            notes: [(0.0, 0.0); HEARD],
            write: 0,
            held: 0.0,
        }
    }

    /// A chord's color, voiced, or NaN. The root is the note with a third
    /// and a fifth over it (an inversion keeps its root), else the lowest
    /// with a third; a chord with neither third gets none.
    fn color(chord: &[f64]) -> f64 {
        let pc = |m: f64| {
            let mut d = m % 12.0;
            if d < 0.0 {
                d += 12.0;
            }
            d
        };
        let has = |root: f64, iv: f64| chord.iter().any(|&m| pc(m - root) == iv);
        let top = chord.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let third = |r: f64| has(r, 4.0) || has(r, 3.0);
        let mut sorted = [f64::NAN; HEARD];
        for (s, &m) in sorted.iter_mut().zip(chord) {
            *s = m;
        }
        let n = chord.len().min(HEARD);
        sorted[..n].sort_unstable_by(f64::total_cmp);
        let root = sorted[..n]
            .iter()
            .copied()
            .find(|&r| third(r) && has(r, 7.0))
            .or_else(|| sorted[..n].iter().copied().find(|&r| third(r)));
        let Some(root) = root else { return f64::NAN };
        if has(root, 6.0) && !has(root, 7.0) {
            return f64::NAN;
        }
        let iv = if has(root, 4.0) { 9.0 } else { 5.0 };
        if has(root, iv) {
            return f64::NAN;
        }
        let mut m = root + iv;
        while m <= top {
            m += 12.0;
        }
        while m > TOP {
            m -= 12.0;
        }
        m
    }

    /// A draw for a color, 0 to 1: from its pitch and how many colors came
    /// before, so a loop's chords decide it, not the render.
    fn draw_for(&mut self, color: f64) -> f64 {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let m = color as u32;
        self.draws = self.draws.wrapping_add(1);
        let mut h = m.wrapping_mul(0x9e37_79b9) ^ self.draws.wrapping_mul(0x85eb_ca6b);
        h ^= h >> 15;
        h = h.wrapping_mul(0x2c1b_3c6d);
        h ^= h >> 12;
        f64::from(h >> 8) / f64::from(1u32 << 24)
    }

    fn sound(&mut self, midi: f64) {
        for t in &mut self.tones {
            t.on = false;
        }
        let f = 440.0 * math::pow(2.0, (midi - 69.0) / 12.0);
        let peak = (self.rel * math::sqrt(self.heard)).max(self.level).min(MOST);
        let rate = self.rate;
        let t = &mut self.tones[self.next];
        *t = Tone {
            phase: [0.0, 0.31, 0.0],
            step: [f * math::pow(2.0, -2.5 / 1200.0) / rate, f * math::pow(2.0, 2.5 / 1200.0) / rate, 2.0 * f / rate],
            gain: 0.0,
            peak,
            on: true,
            midi,
        };
        self.next = 1 - self.next;
    }
}

impl Default for Sketch {
    fn default() -> Self {
        Self::new()
    }
}

impl Block for Sketch {
    type Settings = Settings;

    const PROBES: &'static [Probe<Settings>] = &[Probe { name: "thirteen", settings: Settings { level: 0.03, rel: 0.32, share: 1.0, start: 60.0 } }];

    const HEARS: &'static [u32] = &[event::RETUNE, event::RELEASE];

    fn start(&mut self, s: &Settings, cx: &Context) {
        *self = Sketch::new();
        self.rate = cx.rate;
        self.level = s.level;
        self.rel = s.rel;
        self.share = s.share;
        self.draws = 0;
        self.notes = [(f64::NAN, f64::NEG_INFINITY); HEARD];
        self.held = f64::NAN;
        #[allow(clippy::cast_precision_loss)]
        let frames = crate::QUANTUM as f64;
        self.follow = 1.0 - math::exp(-frames / (FOLLOW * cx.rate));
        self.up = 1.0 / (SWELL * cx.rate);
        self.down = 1.0 / (GO * cx.rate);
        if !s.start.is_nan() {
            self.sound(Self::color(&[s.start, s.start + 4.0, s.start + 7.0]));
        }
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    fn event(&mut self, e: &Event) {
        match e.kind {
            event::RETUNE => {
                for &m in e.chord() {
                    self.notes[self.write] = (m, e.time);
                    self.write = (self.write + 1) % HEARD;
                }
                let mut chord = [f64::NAN; HEARD];
                let mut n = 0;
                for &(m, at) in &self.notes {
                    if !m.is_nan() && e.time - at < WINDOW {
                        chord[n] = m;
                        n += 1;
                    }
                }
                let color = Self::color(&chord[..n]);
                // The same color as it holds (or none again): nothing moves.
                if color == self.held || (color.is_nan() && self.held.is_nan()) {
                    return;
                }
                self.held = color;
                if !color.is_nan() && self.draw_for(color) < self.share {
                    self.sound(color);
                } else {
                    for t in &mut self.tones {
                        t.on = false;
                    }
                }
            }
            event::RELEASE => {
                for t in &mut self.tones {
                    t.on = false;
                }
                self.notes = [(f64::NAN, f64::NEG_INFINITY); HEARD];
                self.held = f64::NAN;
            }
            _ => {}
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn render(&mut self, _: u64, _: &Context, signal: &mut Signal) {
        let mut ms = 0.0;
        for &x in signal.iter() {
            ms += f64::from(x) * f64::from(x);
        }
        #[allow(clippy::cast_precision_loss)]
        let n = signal.len() as f64;
        self.heard += self.follow * (ms / n - self.heard);
        let (up, down) = (self.up, self.down);
        for t in &mut self.tones {
            if !t.on && t.gain == 0.0 {
                continue;
            }
            for x in signal.iter_mut() {
                t.gain = if t.on { (t.gain + up).min(1.0) } else { (t.gain - down).max(0.0) };
                let tau = core::f64::consts::TAU;
                let y = 0.5 * math::sin(tau * t.phase[0]) + 0.5 * math::sin(tau * t.phase[1]) + 0.12 * math::sin(tau * t.phase[2]);
                *x += (t.peak * t.gain * y) as f32;
                for k in 0..3 {
                    t.phase[k] += t.step[k];
                    if t.phase[k] >= 1.0 {
                        t.phase[k] -= 1.0;
                    }
                }
            }
        }
    }
}

// It adds its own sound to its channel's whole signal, which it listens to.
impl Channel for Sketch {
    const WHOLE: bool = true;
}
