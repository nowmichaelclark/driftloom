//! An oscillator that starts, runs and stops the way a Web Audio
//! `OscillatorNode` does in Chromium (`oscillator_handler.cc`), checked
//! against its output. It plays a `Wave`: a `PeriodicWave`, or one of the
//! built-in shapes, which Chromium builds the same way -- even the sine is
//! a table of one partial, read like the rest. The JavaScript synth is the
//! reference, and these are the places a plain oscillator would not null
//! against it:
//!
//! - **Start and stop** land on whole sample frames: the first frame at or
//!   after the start time, and the stop time likewise, each rounded to
//!   1/1024 of a frame first (`frame_at`). So a start a hair past a frame
//!   is on that frame, and a stop a hair past one stops there. A start on
//!   a frame but after it writes nothing there; the wave begins on the
//!   next. The wave starts at the top of its table and moves on after each
//!   sample, by the step at that sample.
//! - **The fraction of a frame.** A note rarely starts on a frame. When the
//!   oscillator's pitch is steady in the render block it starts in
//!   (nothing automated, nothing connected to it), Chromium starts the
//!   wave where it would have been at that frame had the note begun
//!   between frames. Not in the very first block a context renders, and
//!   not when the pitch moves (below): there it starts at the top.
//! - **The first block of a moving pitch.** When the frequency or detune
//!   moves in the block the note starts in (something is connected to it,
//!   as FM is, or it has an automation event at or after the block's
//!   start), the block reads its steps from the start of the block rather
//!   than from the note's first frame: the note's first sample takes the
//!   block's first step, and so on. So a frequency set at the note's start
//!   time does not reach it until a block later, and the oscillator runs
//!   at whatever came before -- an automated frequency's intrinsic value,
//!   or an FM carrier's own pitch with no modulation -- for up to a block.
//!   That is in every JavaScript note today, so it is here too. From the
//!   next block on, frames line up.
//! - **Precision.** The place in the table is a 64-bit float; the steps,
//!   the pitch, the detune and the blend between tables are 32-bit floats,
//!   worked out in Chromium's order.
//!
//! Blocks are Web Audio's render quantum, `QUANTUM` frames, on frame
//! numbers that are multiples of it: the host hands every block to the
//! core in order, and these rules are counted from frame 0.

// The arithmetic follows Chromium's, f32 and f64 mixed as its code mixes
// them, and frame counts, indices and sample values cross between ints
// and floats on purpose; a checked conversion would add a branch to every
// sample and change no result.
#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::cast_sign_loss)]

use crate::math;
use crate::QUANTUM;
use crate::block::{Block, Context, Mod, Mods, Probe, Signal};
use crate::param::{MAX_EVENTS, Param};
use crate::wave::{Wave, Waves};

use super::shared::starts_in;

/// `TimeToSampleFrame`'s first step: the time in frames, to 1/1024 of one.
#[must_use]
pub fn frames(time: f64, rate: f64) -> f64 {
    math::round(time * rate * 1024.0) / 1024.0
}

/// The first frame at or after `time` seconds, as Chromium schedules every
/// source, oscillators and buffer sources alike
/// (`AudioScheduledSourceHandler::UpdateSchedulingInfo`): the time is
/// rounded to 1/1024 of a frame first, so a time a hair past a frame is
/// that frame.
#[must_use]
pub fn frame_at(time: f64, rate: f64) -> u64 {
    let f = math::ceil(frames(time, rate));
    if f > 0.0 { f as u64 } else { 0 }
}

/// How a wavetable oscillator's pitch is set for a block: steady (one
/// frequency and one detune for the whole block, read once), or moving, with
/// the frequency and the detune in cents at every frame. Either part of a
/// moving pitch may itself be steady.
pub enum Pitch<'a> {
    /// One frequency in Hz and one detune in cents for the whole block.
    Steady {
        /// The frequency, in Hz.
        freq: f32,
        /// The detune, in cents.
        detune: f32,
    },
    /// A frequency and a detune that may change at every frame.
    Moving {
        /// The frequency, in Hz.
        freq: Part<'a>,
        /// The detune, in cents.
        detune: Part<'a>,
    },
}

/// One part of a moving pitch: steady for the block, or a value a frame.
pub enum Part<'a> {
    /// The same value at every frame.
    Steady(f32),
    /// A value for each frame of the block.
    Moving(&'a [f32; QUANTUM]),
}

/// The oscillator. It keeps its place in table samples.
pub struct TableOsc {
    /// Position in the table, in table samples, in [0, size).
    index: f64,
    start: u64,
    stop: u64,
    at: f64,
}

impl TableOsc {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`;
    /// starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        TableOsc {
            index: 0.0,
            start: 0,
            stop: 0,
            at: 0.0,
        }
    }

    /// Start at `start` seconds and stop at `stop`, at `rate` Hz, from the
    /// top of the table (`OscillatorNode.start`, `.stop`).
    pub fn schedule(&mut self, start: f64, stop: f64, rate: f64) {
        self.start = frame_at(start, rate);
        self.stop = frame_at(stop, rate);
        self.at = start * rate;
        self.index = 0.0;
    }

    /// Where the oscillator is in the table, in table samples.
    #[must_use]
    pub fn index(&self) -> f64 {
        self.index
    }

    /// The first frame it plays.
    #[must_use]
    pub fn start_frame(&self) -> u64 {
        self.start
    }

    /// Where a steady wave starts, in the block it starts in: where it
    /// would have been at its first sample had it begun on its start time,
    /// between frames. Not in the very first block a context renders.
    fn lead(&mut self, f: f32, rate_scale: f32) {
        if self.start >= QUANTUM as u64 {
            let first = self.start + u64::from(self.at > self.start as f64);
            self.index = (first as f64 - self.at) * f64::from(f) * f64::from(rate_scale);
        }
    }

    /// The frame it stops on: it plays nothing from here on.
    #[must_use]
    pub fn stop_frame(&self) -> u64 {
        self.stop
    }

    /// Render the block from frame `block` into `out` (every sample written,
    /// zero where it does not play). Returns the range it played in.
    pub fn render(
        &mut self,
        block: u64,
        wave: &Wave,
        pitch: &Pitch,
        rate: f32,
        out: &mut [f32; QUANTUM],
    ) -> (usize, usize) {
        out.fill(0.0);
        self.play::<false>(block, wave, pitch, rate, out)
    }

    /// The same, but added into `out` where it plays and leaving the rest
    /// alone: the `Oscillator` block's way, with no copy between.
    pub fn add(&mut self, block: u64, wave: &Wave, pitch: &Pitch, rate: f32, out: &mut [f32; QUANTUM]) -> (usize, usize) {
        self.play::<true>(block, wave, pitch, rate, out)
    }

    // `render` and `add`: each sample written, or added (`ADD`). Frame
    // counts and table sizes are far below 2^23, so their casts to floats
    // are exact; the 32-bit narrowing is Chromium's own.
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    fn play<const ADD: bool>(
        &mut self,
        block: u64,
        wave: &Wave,
        pitch: &Pitch,
        rate: f32,
        out: &mut [f32; QUANTUM],
    ) -> (usize, usize) {
        let end = block + QUANTUM as u64;
        let first = self.start >= block && self.start < end;
        // A start a hair past its frame (see `frame_at`) starts on that
        // frame but writes nothing there: the wave begins on the next.
        let skip = u64::from(first && self.at > self.start as f64);
        let lo = self.start.max(block) + skip;
        let hi = self.stop.min(end);
        let size = wave.size() as f64;
        let nyquist = rate / 2.0;
        let rate_scale = wave.rate_scale();
        if lo >= hi {
            // Started on a block's last frame, a hair past it: Chromium
            // still sets the wave's place in this block, at its pitch here.
            if skip == 1
                && lo == end
                && let Pitch::Steady { freq, detune } = pitch
            {
                self.lead(within(freq * math::exp2f(detune / 1200.0), nyquist), rate_scale);
            }
            return (0, 0);
        }
        let (lo, hi) = ((lo - block) as usize, (hi - block) as usize);
        let n = hi - lo;
        match pitch {
            Pitch::Steady { freq, detune } => {
                let f = within(freq * math::exp2f(detune / 1200.0), nyquist);
                let incr = f * rate_scale;
                if first {
                    self.lead(f, rate_scale);
                }
                let pick = wave.pick(f);
                let mut index = self.index;
                for y in out.iter_mut().skip(lo).take(n) {
                    let v = wave.read(index, pick);
                    *y = if ADD { *y + v } else { v };
                    index = wrap_to(index + f64::from(incr), size);
                }
                // Chromium moves on by the whole block at once.
                self.index = wrap_to(self.index + f64::from(n as f32 * incr), size);
            }
            Pitch::Moving { freq, detune } => {
                // The step at every frame, as Chromium works it out: the
                // frequencies, times 2^(cents/1200), clamped to the Nyquist
                // frequency, times the table's rate; in 32-bit floats.
                let mut incr = [0.0f32; QUANTUM];
                let mut scale = rate_scale;
                match freq {
                    Part::Moving(f) => incr = **f,
                    Part::Steady(f) => scale *= *f,
                }
                match detune {
                    Part::Moving(d) => {
                        let k = (1.0f64 / 1200.0) as f32;
                        let moving = matches!(freq, Part::Moving(_));
                        for (i, c) in incr.iter_mut().zip(d.iter()) {
                            let m = math::exp2f(c * k);
                            *i = if moving { m * *i } else { m };
                        }
                    }
                    Part::Steady(c) => scale *= math::exp2f(c / 1200.0),
                }
                for i in &mut incr {
                    *i = within(*i, nyquist) * scale;
                }
                // The first block reads its steps from the block's start;
                // see the module notes.
                let shift = if first { lo } else { 0 };
                let inverse = 1.0 / rate_scale;
                let steps = incr.get(lo - shift..hi - shift).unwrap_or(&[]);
                let mut index = self.index;
                for (y, &step) in out.iter_mut().skip(lo).zip(steps) {
                    let v = wave.read(index, wave.pick(inverse * step));
                    *y = if ADD { *y + v } else { v };
                    index = wrap_to(index + f64::from(step), size);
                }
                self.index = index;
            }
        }
        (lo, hi)
    }
}

impl TableOsc {
    /// The block from frame `block` at a steady `freq`, as `render` would
    /// play it, without reading the table: only its place in the table
    /// moves. For a host that starts rendering after the oscillator began
    /// (the wobble's LFOs, `master`).
    pub fn skip(&mut self, block: u64, wave: &Wave, freq: f32, rate: f32) {
        let end = block + QUANTUM as u64;
        let first = self.start >= block && self.start < end;
        let skip = u64::from(first && self.at > self.start as f64);
        let lo = self.start.max(block) + skip;
        let hi = self.stop.min(end);
        let f = within(freq, rate / 2.0);
        let rate_scale = wave.rate_scale();
        if lo >= hi {
            if skip == 1 && lo == end {
                self.lead(f, rate_scale);
            }
            return;
        }
        if first {
            self.lead(f, rate_scale);
        }
        let n = (hi - lo) as f32;
        self.index = wrap_to(self.index + f64::from(n * (f * rate_scale)), wave.size() as f64);
    }
}

impl TableOsc {
    /// `blocks` whole blocks at a steady `freq` after `skip` last moved the
    /// oscillator, all in one step: only its place in the table moves, as
    /// `skip` would move it block by block. Each block moves it on by the
    /// same `QUANTUM * incr`, a 32-bit float, and a sum of such steps is
    /// exact in 64 bits (24 bits of mantissa a step, tens of bits of room),
    /// so the one multiplication equals the repeated addition; what differs
    /// from stepping is only the rounding the stepped sum collects from a
    /// start that fell between two frames (`lead`): measured at about 7e-12
    /// of a table sample after a thousand blocks and 5.4e-9 after 1.24
    /// million (an hour in), far
    /// under the 1e-7 the tests allow and inaudible. Only for blocks after
    /// the one the oscillator started in, and before it stops.
    pub fn skip_whole(&mut self, blocks: u64, wave: &Wave, freq: f32, rate: f32) {
        let f = within(freq, rate / 2.0);
        let step = f64::from(QUANTUM as f32 * (f * wave.rate_scale()));
        self.index = wrap_to(self.index + blocks as f64 * step, wave.size() as f64);
    }
}

/// The tables an `Oscillator` block can play: Web Audio's built-in shapes
/// and the core's own waves (`wave::Waves`).
#[derive(Clone, Copy)]
pub enum Shape {
    /// `'sine'`.
    Sine,
    /// `'triangle'`.
    Triangle,
    /// `'sawtooth'`.
    Saw,
    /// `'square'`.
    Square,
    /// The fiddle's bowed string.
    Bowed,
    /// `_makeWave(1.5)`, the saw lead's.
    LeadSaw,
    /// The moog's.
    LeadMoog,
    /// `_makeWave(1.7)`, the analog lead's.
    LeadAnalog,
    /// The accordion's reed.
    Reed,
    /// The pan flute's pipe.
    Pipe,
    /// The nylon guitar's mellow layer.
    NylonMellow,
    /// The nylon guitar's bright layer.
    NylonBright,
    /// The sung voices' glottal pulse.
    Glottal,
}

impl Shape {
    /// Its table among `waves`.
    #[must_use]
    pub fn table(self, waves: &Waves) -> &Wave {
        match self {
            Shape::Sine => &waves.sine,
            Shape::Triangle => &waves.triangle,
            Shape::Saw => &waves.saw,
            Shape::Square => &waves.square,
            Shape::Bowed => &waves.bowed,
            Shape::LeadSaw => &waves.lead_saw,
            Shape::LeadMoog => &waves.lead_moog,
            Shape::LeadAnalog => &waves.lead_analog,
            Shape::Reed => &waves.reed,
            Shape::Pipe => &waves.pipe,
            Shape::NylonMellow => &waves.nylon_mellow,
            Shape::NylonBright => &waves.nylon_bright,
            Shape::Glottal => &waves.glottal,
        }
    }
}

/// An oscillator's settings when it plays as a block: a table, a pitch
/// (automate the frequency after starting it, through `Oscillator::freq`),
/// and when it starts and stops.
#[derive(Clone, Copy)]
pub struct Settings {
    /// The table.
    pub shape: Shape,
    /// The frequency, in Hz: its value before any automation.
    pub freq: f32,
    /// The detune, in cents.
    pub detune: f32,
    /// When it starts, in seconds.
    pub start: f64,
    /// When it stops, in seconds.
    pub stop: f64,
}

/// The oscillator's setting a signal can drive (`Block::MODS`): its
/// frequency, in Hz -- FM, where a modulator drives a carrier.
pub const FREQ: usize = 0;
/// Its detune, in cents -- a vibrato.
pub const DETUNE: usize = 1;

/// The oscillator as a block: `TableOsc` playing one of the core's tables,
/// added into the signal. Its frequency is a `Param` (`freq`: an
/// `OscillatorNode`'s `frequency`, which a voice automates for a slur or a
/// slide), holding up to `N` events, so a steady oscillator (`N = 0`) pays
/// nothing for one; its detune is steady. A signal can drive either
/// (`FREQ`, `DETUNE`), added to it frame by frame, as Web Audio sums an
/// `AudioParam` and the node connected into it.
///
/// The pitch is steady for a block (one frequency, read once, with the
/// start rules that go with it; see the module notes) when the frequency's
/// automation is not moving and no signal drives it; otherwise it moves,
/// frame by frame -- a driven setting moves even where its signal is zero.
pub struct Oscillator<const N: usize = MAX_EVENTS> {
    osc: TableOsc,
    shape: Shape,
    freq: Param<N>,
    detune: f32,
    /// The frames of the last block it rendered that it sounded in.
    played: (usize, usize),
}

impl<const N: usize> Oscillator<N> {
    /// A silent oscillator, all zeros.
    #[must_use]
    pub const fn new() -> Self {
        Oscillator { osc: TableOsc::new(), shape: Shape::Sine, freq: Param::new(0.0), detune: 0.0, played: (0, 0) }
    }

    /// Set it to play `shape` at `freq` Hz (the frequency's value before
    /// any automation) and `detune` cents, from `start` to `stop` seconds,
    /// at `rate` Hz, from the top of the table: `Block::start` with the
    /// numbers, for a voice that has a rate rather than a `Context`.
    pub fn set(&mut self, shape: Shape, freq: f32, detune: f32, start: f64, stop: f64, rate: f64) {
        self.shape = shape;
        self.freq.reset(freq);
        self.detune = detune;
        self.osc.schedule(start, stop, rate);
        self.played = (0, 0);
    }

    /// Its frequency, to automate after `set` (a slur, a slide).
    pub fn freq(&mut self) -> &mut Param<N> {
        &mut self.freq
    }

    /// The first frame it plays.
    #[must_use]
    pub fn start_frame(&self) -> u64 {
        self.osc.start_frame()
    }

    /// The frame it stops on.
    #[must_use]
    pub fn stop_frame(&self) -> u64 {
        self.osc.stop_frame()
    }

    /// The frames of the last block it rendered that it sounded in, from
    /// and to: zeros outside them. A voice applies its envelope there.
    #[must_use]
    pub fn played(&self) -> (usize, usize) {
        self.played
    }
}

impl<const N: usize> Default for Oscillator<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Block for Oscillator<N> {
    type Settings = Settings;

    const PROBES: &'static [Probe<Settings>] = &[
        Probe { name: "sine", settings: Settings { shape: Shape::Sine, freq: 440.0, detune: 0.0, start: 0.05, stop: 0.55 } },
        Probe { name: "triangle", settings: Settings { shape: Shape::Triangle, freq: 330.0, detune: 0.0, start: 0.0123, stop: 0.5 } },
        Probe { name: "saw", settings: Settings { shape: Shape::Saw, freq: 220.0, detune: -7.0, start: 0.05, stop: 0.55 } },
        Probe { name: "square", settings: Settings { shape: Shape::Square, freq: 196.0, detune: 3.0, start: 0.05, stop: 0.55 } },
    ];

    const MODS: &'static [Mod] = &[Mod { name: "freq", swing: 40.0 }, Mod { name: "detune", swing: 50.0 }];

    fn start(&mut self, s: &Settings, cx: &Context) {
        self.set(s.shape, s.freq, s.detune, s.start, s.stop, cx.rate);
    }

    fn span(&self) -> (u64, u64) {
        (self.osc.start_frame(), self.osc.stop_frame())
    }

    fn render(&mut self, block: u64, cx: &Context, signal: &mut Signal) {
        self.render_mod(block, cx, signal, &Mods::NONE);
    }

    // The table is read at a 32-bit rate, as Web Audio's is; a block's
    // first frame as seconds is well under 2^52 frames.
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    fn render_mod(&mut self, block: u64, cx: &Context, signal: &mut Signal, mods: &Mods) {
        let rate = cx.rate;
        let from = block as f64 / rate;
        // Whether the automation moves is decided before its events are
        // clamped to the block the oscillator starts in, as Chromium
        // decides it first (`shared::pitch_of`, which this generalizes).
        let moving = self.freq.moving_from(from, QUANTUM as f64 / rate);
        if starts_in(self.osc.start_frame(), block) {
            self.freq.clamp_before(block, rate);
        }
        // The buffers are only made when a part moves.
        let driven = mods.get(FREQ);
        let freq: Signal;
        let freq_part = if moving || driven.is_some() {
            let mut f = if moving {
                let mut f = [0.0f32; QUANTUM];
                self.freq.fill(block, rate, &mut f);
                f
            } else {
                [self.freq.value_at(from); QUANTUM]
            };
            if let Some(m) = driven {
                for (f, x) in f.iter_mut().zip(m.iter()) {
                    *f += *x;
                }
            }
            freq = f;
            Part::Moving(&freq)
        } else {
            Part::Steady(self.freq.value_at(from))
        };
        let cents: Signal;
        let detune_part = match mods.get(DETUNE) {
            Some(m) => {
                cents = core::array::from_fn(|i| self.detune + m.get(i).copied().unwrap_or(0.0));
                Part::Moving(&cents)
            }
            None => Part::Steady(self.detune),
        };
        let pitch = match (freq_part, detune_part) {
            (Part::Steady(freq), Part::Steady(detune)) => Pitch::Steady { freq, detune },
            (freq, detune) => Pitch::Moving { freq, detune },
        };
        let table = self.shape.table(cx.waves);
        self.played = self.osc.add(block, table, &pitch, rate as f32, signal);
    }
}

impl Default for TableOsc {
    fn default() -> Self {
        Self::new()
    }
}

/// `f` held within plus or minus `limit`. Not `f32::clamp`, which checks
/// its bounds and would bring the panic machinery into the `.wasm`.
fn within(f: f32, limit: f32) -> f32 {
    f.max(-limit).min(limit)
}

/// `index` wrapped into the table, `index - floor(index / size) * size`.
/// Most frames it is already there, and then that is `index` to the bit
/// (the floor is +0); the division, the slow part, is only done when not.
fn wrap_to(index: f64, size: f64) -> f64 {
    if index.is_sign_positive() && index < size {
        index
    } else {
        index - math::floor(index / size) * size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wave::{Fft, Waves};

    const RATE: f64 = 44100.0;

    // The waves are 1.8 MB, built on a thread with room for them.
    fn waves() -> Box<Waves> {
        std::thread::Builder::new()
            .stack_size(1 << 26)
            .spawn(|| {
                let mut w = Box::new(Waves::new());
                w.build(RATE as f32, &mut Box::new(Fft::new()));
                w
            })
            .and_then(|t| t.join().map_err(|_| std::io::Error::other("panicked")))
            .expect("waves")
    }

    // A sine through the table: Chromium's own interpolation error is
    // about 1e-5 at most, so that is the tolerance here.
    fn sine(phase: f64) -> f32 {
        (phase * core::f64::consts::TAU).sin() as f32
    }

    fn run(
        osc: &mut TableOsc,
        w: &Waves,
        freq: impl Fn(u64) -> f32,
        moving: bool,
        frames: usize,
    ) -> Vec<f32> {
        let mut all = Vec::new();
        let mut block = 0u64;
        while all.len() < frames {
            let mut f = [0.0; QUANTUM];
            for (i, v) in f.iter_mut().enumerate() {
                *v = freq(block + i as u64);
            }
            let mut out = [0.0; QUANTUM];
            let pitch = if moving {
                Pitch::Moving {
                    freq: Part::Moving(&f),
                    detune: Part::Steady(0.0),
                }
            } else {
                Pitch::Steady {
                    freq: f[0],
                    detune: 0.0,
                }
            };
            osc.render(block, &w.sine, &pitch, RATE as f32, &mut out);
            all.extend_from_slice(&out);
            block += QUANTUM as u64;
        }
        all
    }

    #[test]
    fn starts_and_stops_on_the_next_whole_frame() {
        let w = waves();
        let mut osc = TableOsc::new();
        // Inside the first block, so no lead.
        osc.schedule(100.37 / RATE, 3000.6 / RATE, RATE);
        let out = run(&mut osc, &w, |_| 1000.0, false, 4096);
        assert_eq!(out[100], 0.0);
        assert!(out[101].abs() < 1e-6); // the top of the table
        assert!((out[102] - sine(1000.0 / RATE)).abs() < 1e-5);
        assert!(out[3000] != 0.0);
        assert_eq!(out[3001], 0.0);
    }

    #[test]
    fn a_hair_past_a_frame_is_that_frame() {
        let w = waves();
        let mut osc = TableOsc::new();
        // Starts on frame 1000 but plays from 1001, as it would have from
        // 1000.0003; stops on frame 3000, which it does not play.
        osc.schedule(1000.0003 / RATE, 3000.0003 / RATE, RATE);
        assert_eq!(osc.start_frame(), 1000);
        assert_eq!(osc.stop_frame(), 3000);
        let out = run(&mut osc, &w, |_| 1000.0, false, 4096);
        assert_eq!(out[1000], 0.0);
        assert!((out[1001] - sine(1000.0 * 0.9997 / RATE)).abs() < 1e-5);
        assert!(out[2999] != 0.0);
        assert_eq!(out[3000], 0.0);
    }

    #[test]
    fn a_start_on_a_blocks_last_frame_takes_that_blocks_pitch() {
        let w = waves();
        let mut osc = TableOsc::new();
        // Frame 1023 is the last of the block 896..1024: the note starts
        // there and sounds from 1024, from where 1000 Hz (that block's
        // pitch, not the next one's) would have put it.
        osc.schedule(1023.0003 / RATE, 1.0, RATE);
        let out = run(
            &mut osc,
            &w,
            |k| if k >= 1024 { 2000.0 } else { 1000.0 },
            false,
            2048,
        );
        assert_eq!(out[1023], 0.0);
        assert!((out[1024] - sine(1000.0 * 0.9997 / RATE)).abs() < 1e-5);
    }

    #[test]
    fn a_steady_pitch_starts_where_it_would_have_been() {
        let w = waves();
        let mut osc = TableOsc::new();
        osc.schedule(1000.3 / RATE, 1.0, RATE);
        let out = run(&mut osc, &w, |_| 1000.0, false, 2048);
        assert!((out[1001] - sine(1000.0 * 0.7 / RATE)).abs() < 1e-5);
    }

    #[test]
    fn a_moving_pitch_reads_its_first_block_from_the_block_start() {
        let w = waves();
        let mut osc = TableOsc::new();
        osc.schedule(1000.3 / RATE, 1.0, RATE);
        // 440 until frame 1001, 110 after: the first block (896..1024)
        // reads frames 896.. for its 23 samples, all of them 440.
        let out = run(
            &mut osc,
            &w,
            |k| if k >= 1001 { 110.0 } else { 440.0 },
            true,
            2048,
        );
        let mut phase = 0.0f64;
        for k in 1001..1100u64 {
            assert!((out[k as usize] - sine(phase)).abs() < 1e-5, "frame {k}");
            phase += if k < 1024 { 440.0 } else { 110.0 } / RATE;
        }
    }
}
