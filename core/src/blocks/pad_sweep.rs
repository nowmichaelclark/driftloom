//! padSweep (queue item 72; Ears 8, approved by Mikey 2026-10-18): a
//! lowpass on the chord layer swinging open and shut every 12-28 s while the
//! rest stays as bright as it was.
//!
//! A channel's own block on the whole chords channel (`Channel::WHOLE`): its
//! own `Lfo` into its own `Filter`'s frequency, as the swell (queue item 46c's
//! demonstration) carries an `Lfo` into a gain. The sketch
//! (`ears8-sketch.patch`, `e8Sweep`) set a `BiquadFilterNode` at `lo` times
//! 2^(oct / 2) and drove its detune with an oscillator of wave -cos, by oct / 2
//! octaves, so the cutoff is lo * 2^(oct * (1 - cos) / 2): `lo` when closed,
//! where it starts, `lo` times 2^oct when open, half a period later. Here the
//! `Lfo` runs at half the swing's rate and its square is that curve's
//! (1 - cos) / 2, starting at 0: closed.
//!
//! Its settings, set for the loop by the host (`Core::set`): where it closes
//! (`LO`, Hz), how far it opens (`OCT`, octaves), its period (seconds) and its
//! Q (dB, as a `BiquadFilterNode`'s). A new sweep starts closed at its time.
//! Across a load the cutoff glides from where it was onto the new loop's
//! curve over `GLIDE` seconds, and a loop with no sweep (an `OCT` of 0) has
//! it glide open and then stand still, costing nothing.

// Times in frames, and settings in 32-bit floats, as Web Audio's are.
#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::cast_sign_loss)]

use crate::QUANTUM;
use crate::block::{Block, Channel, Context, Mods, Probe, Setting, Signal};
use crate::math;

use super::filter::{FREQ, Filter, Kind, Settings as FilterSettings};
use super::lfo::Lfo;

/// How a sweep is set.
#[derive(Clone, Copy)]
pub struct Settings {
    /// Where it closes, in Hz.
    pub lo: f32,
    /// How far it opens, in octaves above `lo`; 0 is off.
    pub oct: f32,
    /// One swing, closed to open and back, in seconds.
    pub period: f32,
    /// Its resonance, in dB.
    pub q: f32,
}

/// At rest: off.
pub const REST: Settings = Settings { lo: 0.0, oct: 0.0, period: 0.0, q: 0.0 };

/// Its settings a host sets, by number.
pub const LO: usize = 0;
/// How far it opens.
pub const OCT: usize = 1;
/// Its period.
pub const PERIOD: usize = 2;
/// Its Q.
pub const Q: usize = 3;

/// How long the cutoff glides across a load, in seconds.
const GLIDE: f64 = 0.5;
/// Where an ending sweep glides to, in Hz: open, past what the chords play.
const OPEN: f32 = 20000.0;
/// How long the filter takes to come in from still, and to go back out, in
/// seconds: a crossfade with the dry channel, since a filter starting from
/// rest under a signal already playing jumps.
const FADE: f64 = 0.02;

/// The sweep: its settings, its `Lfo` and `Filter`, and the glide across a
/// change.
pub struct PadSweep {
    lo: f32,
    oct: f32,
    period: f32,
    q: f32,
    lfo: Lfo<0>,
    filter: Filter,
    /// Whether the filter is running (a sweep, or one gliding out).
    on: bool,
    /// The cutoff the last frame had, in octaves above 1 Hz, and the glide
    /// from there: frames left, and how many it takes.
    last: f32,
    from: f32,
    left: u32,
    glide: u32,
    /// How much of the channel is the filter's, 0 to 1, and the step it moves
    /// by a frame.
    wet: f32,
    fade: f32,
}

impl PadSweep {
    /// A sweep at rest, all zeros.
    #[must_use]
    pub const fn new() -> Self {
        PadSweep {
            lo: 0.0,
            oct: 0.0,
            period: 0.0,
            q: 0.0,
            lfo: Lfo::new(),
            filter: Filter::new(),
            on: false,
            last: 0.0,
            from: 0.0,
            left: 0,
            glide: 0,
            wet: 0.0,
            fade: 0.0,
        }
    }

    /// The new sweep from `time`: closed then, gliding from where the last one
    /// was if it was running.
    fn begin(&mut self, time: f64, cx: &Context) {
        let open = self.oct > 0.0 && self.period > 0.0 && self.lo > 0.0;
        if open {
            self.lfo.set(0.5 / self.period, 1.0, time, f64::INFINITY, cx.rate);
            let settings = FilterSettings { kind: Kind::Lowpass, freq: 0.0, q: self.q, gain: 0.0 };
            // A sweep under way keeps its filter's memory, so the change
            // doesn't click; a new one starts from rest.
            if self.on {
                self.filter.retarget(settings);
            } else {
                self.filter.set(settings, cx.rate as f32);
            }
        }
        if self.on {
            self.from = self.last;
            self.glide = ((GLIDE * cx.rate) as u32).max(1);
            self.left = self.glide;
        } else if open {
            self.left = 0;
        }
        self.on = self.on || open;
        self.fade = (1.0 / (FADE * cx.rate)) as f32;
    }
}

impl Default for PadSweep {
    fn default() -> Self {
        Self::new()
    }
}

impl Block for PadSweep {
    type Settings = Settings;

    const PROBES: &'static [Probe<Settings>] = &[Probe { name: "sweep", settings: Settings { lo: 600.0, oct: 3.0, period: 0.8, q: 1.0 } }];

    const INPUT: bool = true;

    // The kit sets each to a value a listener would hear differ: lower,
    // wider, slower, more resonant.
    const SETTINGS: &'static [Setting] = &[
        Setting { name: "lo", probe: 300.0 },
        Setting { name: "oct", probe: 4.0 },
        Setting { name: "period", probe: 0.5 },
        Setting { name: "q", probe: 6.0 },
    ];

    fn start(&mut self, s: &Settings, cx: &Context) {
        *self = PadSweep::new();
        self.lo = s.lo;
        self.oct = s.oct;
        self.period = s.period;
        self.q = s.q;
        self.begin(0.0, cx);
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    /// A new setting takes hold at `time`: the sweep starts again there,
    /// closed (the host sets all four at a loop's first step, together).
    fn set(&mut self, setting: usize, value: f64, time: f64, cx: &Context) {
        let v = value as f32;
        match setting {
            LO => self.lo = v,
            OCT => self.oct = v.max(0.0),
            PERIOD => self.period = v,
            Q => self.q = v,
            _ => return,
        }
        self.begin(time, cx);
    }

    fn render(&mut self, block: u64, cx: &Context, signal: &mut Signal) {
        self.render_mod(block, cx, signal, &Mods::NONE);
    }

    /// Still and open, it passes the channel as it is and costs nothing.
    fn render_mod(&mut self, block: u64, cx: &Context, signal: &mut Signal, _: &Mods) {
        if !self.on {
            return;
        }
        let sweeping = self.oct > 0.0 && self.period > 0.0 && self.lo > 0.0;
        let mut wave = [0.0f32; QUANTUM];
        if sweeping {
            self.lfo.render(block, cx, &mut wave);
        }
        // The cutoff at each frame, in octaves above 1 Hz, gliding from the
        // last sweep's onto this one's (or open) across a change.
        let base = if sweeping { math::log2f(self.lo) } else { math::log2f(OPEN) };
        let mut freq = [0.0f32; QUANTUM];
        let mut done = !sweeping;
        for (f, w) in freq.iter_mut().zip(wave.iter()) {
            let mut oct = if sweeping { base + self.oct * *w * *w } else { base };
            if self.left > 0 {
                let k = 1.0 - self.left as f32 / self.glide as f32;
                oct = self.from + (oct - self.from) * k;
                self.left -= 1;
                done = false;
            }
            self.last = oct;
            *f = math::exp2f(oct);
        }
        let dry = *signal;
        self.filter.render_mod(block, cx, signal, &Mods::NONE.with(FREQ, &freq));
        // In from still, and, glided open with no sweep to follow, out to
        // still, by a crossfade with the dry channel.
        let to = if done { 0.0 } else { 1.0 };
        if self.wet != to || to < 1.0 {
            for (y, x) in signal.iter_mut().zip(dry.iter()) {
                self.wet = if to > self.wet { (self.wet + self.fade).min(1.0) } else { (self.wet - self.fade).max(0.0) };
                *y = *x + (*y - *x) * self.wet;
            }
        }
        if done && self.wet == 0.0 {
            self.on = false;
        }
    }
}

// It works on everything the chords' channel plays.
impl Channel for PadSweep {
    const WHOLE: bool = true;
}
