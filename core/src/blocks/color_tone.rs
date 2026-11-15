//! colorTone (queue item 101e; batch 6's #5, Ears 13, approved by Mikey
//! 2026-11-08): a held color over the chord, a 13th over a major chord or an
//! 11th over a minor one, which the generator decides
//! (`generator::color`) and the player sends as color events
//! (`event::COLOR`: the note and how long it holds).
//!
//! The chords channel's own block, on the whole channel, as the Ears 13
//! sketch was (`tools/ears/sketches/ears13/colorTone/block.rs`), and what it
//! plays is the sketch's, number for number: a soft tone of two sines 5
//! cents apart and a little second harmonic, swelling in over 0.35 s and
//! letting go over 0.6 s, two of them so one lets go while the next swells.
//! Its level follows the chord layer's own: at each color, 1.2 times the
//! chords channel's RMS over the last 1.5 s, never under 0.02 nor over
//! 0.08. What changed is only where the colors come from: the sketch read
//! the chords as a silent organ heard them (none on arpeggiated keys, one
//! drawn for good over one held chord); here the generator knows its
//! chords. It is a voice of its own rather than a note of the chord's
//! voice, because that measures closer to the sketch: held and swelling
//! whatever the chords' voice and rhythm (a note of a stab, an arpeggio or
//! a plucked chord would be struck and fade with it), nearer the sketch's
//! level by ear against the loop (on the sketch's loops a chord's own note
//! read 2.7 to 4.9 LU off it, this 0.3 to 3.1), and outside the voice budget,
//! so it refuses no note (the PR has the figures).
//!
//! A color event with a note the tone already holds, held on, holds on
//! (the same color over a chord struck again); a color event with no note
//! (NaN), or setting `on` to 0, lets it go. Off (`on` 0, the rest), it does
//! nothing and costs nothing.

// Its levels go into the 32-bit signal.
#![allow(clippy::cast_possible_truncation)]

use crate::block::{Block, Channel, Context, Event, Probe, Setting, Signal, event};
use crate::math;

/// How the tone is set.
#[derive(Clone, Copy)]
pub struct Settings {
    /// Its level at the least (the chords silent).
    pub level: f64,
    /// Its level against the chord layer's RMS.
    pub rel: f64,
    /// A color held from the start (MIDI; NaN: none), for the kit.
    pub start: f64,
    /// Whether it listens (1) or not (0): `ON`.
    pub on: f64,
}

/// The settings the registry starts it with: the sketch's level, off.
pub const REST: Settings = Settings { level: 0.02, rel: 1.2, start: f64::NAN, on: 0.0 };

/// Its setting a host sets, by number: whether the loop plays colors.
pub const ON: usize = 0;

/// The loudest it plays.
const MOST: f64 = 0.08;
/// How long the chords' level is followed over, in seconds.
const FOLLOW: f64 = 1.5;
/// Swell and let-go, in seconds.
const SWELL: f64 = 0.35;
const GO: f64 = 0.6;
/// The two sines' detune either side of the note, in cents, and the second
/// harmonic's share.
const CENTS: f64 = 2.5;
const SECOND: f64 = 0.12;

#[derive(Clone, Copy)]
struct Tone {
    phase: [f64; 3],
    step: [f64; 3],
    gain: f64,
    peak: f64,
    on: bool,
    midi: f64,
    /// The frame it lets go at.
    until: u64,
}

const QUIET: Tone = Tone { phase: [0.0; 3], step: [0.0; 3], gain: 0.0, peak: 0.0, on: false, midi: 0.0, until: 0 };

/// The color: two tones, so one lets go while the next swells.
pub struct ColorTone {
    tones: [Tone; 2],
    next: usize,
    level: f64,
    rel: f64,
    rate: f64,
    heard: f64,
    follow: f64,
    up: f64,
    down: f64,
    active: bool,
}

impl ColorTone {
    /// Silent and off, all zeros.
    #[must_use]
    pub const fn new() -> Self {
        ColorTone { tones: [QUIET; 2], next: 0, level: 0.0, rel: 0.0, rate: 0.0, heard: 0.0, follow: 0.0, up: 0.0, down: 0.0, active: false }
    }

    fn let_go(&mut self) {
        for t in &mut self.tones {
            t.on = false;
        }
    }

    /// The color `midi` from `time`, held `seconds`: the one it holds held
    /// on, or the others let go and this one swelling in at a level from
    /// the chords' RMS.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // a frame, positive and far inside a u64
    fn sound(&mut self, midi: f64, time: f64, seconds: f64) {
        let until = (js_max0(time + seconds) * self.rate) as u64;
        // The same note as sent before, not a computed one: compared exactly.
        #[allow(clippy::float_cmp)]
        if let Some(t) = self.tones.iter_mut().find(|t| t.on && t.midi == midi) {
            t.until = t.until.max(until);
            return;
        }
        self.let_go();
        let f = 440.0 * math::pow(2.0, (midi - 69.0) / 12.0);
        let peak = (self.rel * math::sqrt(self.heard)).max(self.level).min(MOST);
        let rate = self.rate;
        let t = &mut self.tones[self.next];
        *t = Tone {
            phase: [0.0, 0.31, 0.0],
            step: [f * math::pow(2.0, -CENTS / 1200.0) / rate, f * math::pow(2.0, CENTS / 1200.0) / rate, 2.0 * f / rate],
            gain: 0.0,
            peak,
            on: true,
            midi,
            until,
        };
        self.next = 1 - self.next;
    }
}

/// `x`, or 0 where it is under 0 or not a number.
fn js_max0(x: f64) -> f64 {
    if x > 0.0 { x } else { 0.0 }
}

impl Default for ColorTone {
    fn default() -> Self {
        Self::new()
    }
}

impl Block for ColorTone {
    type Settings = Settings;

    const PROBES: &'static [Probe<Settings>] = &[Probe { name: "thirteen", settings: Settings { level: 0.03, rel: 0.32, start: 69.0, on: 1.0 } }];

    const HEARS: &'static [u32] = &[event::COLOR];

    // Off lets the color go before it has swelled: silence, which a probe
    // cannot hear as a change (the tests below hold it).
    const SETTINGS: &'static [Setting] = &[Setting { name: "on", probe: f64::NAN }];

    fn start(&mut self, s: &Settings, cx: &Context) {
        *self = ColorTone::new();
        self.rate = cx.rate;
        self.level = s.level;
        self.rel = s.rel;
        self.active = s.on > 0.0;
        #[allow(clippy::cast_precision_loss)]
        let frames = crate::QUANTUM as f64;
        self.follow = 1.0 - math::exp(-frames / (FOLLOW * cx.rate));
        self.up = 1.0 / (SWELL * cx.rate);
        self.down = 1.0 / (GO * cx.rate);
        if !s.start.is_nan() {
            self.sound(s.start, 0.0, f64::INFINITY);
        }
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    fn set(&mut self, setting: usize, value: f64, _: f64, _: &Context) {
        if setting == ON {
            self.active = value > 0.0;
            if !self.active {
                self.let_go();
            }
        }
    }

    fn event(&mut self, e: &Event) {
        if e.kind != event::COLOR || !self.active {
            return;
        }
        if e.value.is_nan() {
            self.let_go();
        } else {
            self.sound(e.value, e.time, e.notes[0]);
        }
    }

    fn render(&mut self, block: u64, _: &Context, signal: &mut Signal) {
        let silent = self.tones.iter().all(|t| !t.on && t.gain == 0.0);
        if !self.active && silent {
            return;
        }
        let mut ms = 0.0;
        for &x in signal.iter() {
            ms += f64::from(x) * f64::from(x);
        }
        #[allow(clippy::cast_precision_loss)]
        let n = signal.len() as f64;
        self.heard += self.follow * (ms / n - self.heard);
        if silent {
            return;
        }
        let (up, down) = (self.up, self.down);
        let tau = core::f64::consts::TAU;
        for t in &mut self.tones {
            if !t.on && t.gain == 0.0 {
                continue;
            }
            for (i, x) in signal.iter_mut().enumerate() {
                if t.on && block + i as u64 >= t.until {
                    t.on = false;
                }
                t.gain = if t.on { (t.gain + up).min(1.0) } else { (t.gain - down).max(0.0) };
                let y = 0.5 * math::sin(tau * t.phase[0]) + 0.5 * math::sin(tau * t.phase[1]) + SECOND * math::sin(tau * t.phase[2]);
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

// It adds its own sound to its channel's whole signal, whose level it
// follows.
impl Channel for ColorTone {
    const WHOLE: bool = true;
}

#[cfg(test)]
mod tests {
    use super::{ColorTone, ON, REST, Settings};
    use crate::QUANTUM;
    use crate::block::{Block, Context, Event};
    use crate::wave::Waves;

    /// The energy of the next `blocks` blocks it renders from `at` (a block
    /// number, moved on past them).
    fn rms(block: &mut ColorTone, cx: &Context, at: &mut u64, blocks: u64) -> f64 {
        let mut sum = 0.0;
        for _ in 0..blocks {
            let mut signal = [0.0f32; QUANTUM];
            block.render(*at * QUANTUM as u64, cx, &mut signal);
            *at += 1;
            sum += signal.iter().map(|&x| f64::from(x) * f64::from(x)).sum::<f64>();
        }
        sum
    }

    // Off (the rest), a color sent is not heard; on, it sounds and lets go
    // when it is held no longer; set off, it lets go.
    #[test]
    fn it_plays_colors_only_when_on_and_lets_them_go() {
        // It reads no wave, but a context holds them: 1.8 MB, made on a
        // thread with room for them.
        let waves = std::thread::Builder::new()
            .stack_size(1 << 26)
            .spawn(|| Box::new(Waves::new()))
            .and_then(|t| t.join().map_err(|_| std::io::Error::other("panicked")))
            .expect("waves");
        let cx = Context { rate: 48_000.0, waves: &waves, noise: &[] };
        let mut c = Box::new(ColorTone::new());
        let mut at = 0;
        c.start(&REST, &cx);
        c.event(&Event::color(0.0, 69.0, 1.0));
        assert_eq!(rms(&mut c, &cx, &mut at, 400), 0.0, "off: nothing");
        c.start(&Settings { on: 1.0, ..REST }, &cx);
        at = 0;
        c.event(&Event::color(0.0, 69.0, 0.5));
        assert!(rms(&mut c, &cx, &mut at, 200) > 0.0, "on: a color");
        rms(&mut c, &cx, &mut at, 250);
        assert_eq!(rms(&mut c, &cx, &mut at, 10), 0.0, "let go after its half second and 0.6 s of letting go");
        c.event(&Event::color(1.2, 72.0, 10.0));
        assert!(rms(&mut c, &cx, &mut at, 100) > 0.0);
        c.set(ON, 0.0, 0.0, &cx);
        rms(&mut c, &cx, &mut at, 250);
        assert_eq!(rms(&mut c, &cx, &mut at, 10), 0.0, "set off, it lets go");
    }
}
