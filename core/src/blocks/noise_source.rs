//! A source that plays the core's noise (`noise`) the way Chromium plays an
//! `AudioBufferSourceNode`: a grain of it, or a loop.
//!
//! The source is Chromium's (`audio_buffer_source_handler.cc`, version 141,
//! the one the measure harness runs), checked against it sample for sample
//! on grains, loops, sub-frame starts and offsets:
//!
//! - It starts on the first frame at or after its start time, and if the
//!   start fell between frames it first skips ahead by the fraction it
//!   missed, at its rate. Its stop is the first frame at or after the stop
//!   time. Both times go through Chromium's `TimeToSampleFrame`, which
//!   rounds the time to 1/1024 of a frame first, so a time a hair past a
//!   frame is that frame.
//! - It reads from a whole frame: the offset, in frames, rounded to the
//!   nearest. (Newer Chromium keeps the fraction.)
//! - It moves by its playback rate, a 32-bit float, each frame, and reads
//!   between two samples by linear interpolation in 64-bit floats. A grain
//!   (`start(when, offset, duration)`) ends when the read position reaches
//!   the offset plus the duration in buffer frames, rounded to the nearest;
//!   a loop wraps around the whole buffer, keeping the fraction.

// The arithmetic follows Chromium's, f32 and f64 mixed as its code mixes
// them, and frame counts, indices and sample values cross between ints
// and floats on purpose; a checked conversion would add a branch to every
// sample and change no result.
#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::cast_sign_loss)]

use crate::math;
use crate::QUANTUM;
use crate::block::{Block, Context, Probe, Signal};
use super::osc::{frame_at, frames};

/// A source playing the noise.
#[derive(Clone, Copy)]
pub struct NoiseSource {
    start: u64,
    stop: u64,
    /// Where it reads, in buffer frames.
    index: f64,
    /// Buffer frames a frame.
    step: f64,
    /// Where a grain ends, or the buffer's length for a loop.
    end: f64,
    looped: bool,
    done: bool,
}

impl NoiseSource {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`;
    /// starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        NoiseSource {
            start: 0,
            stop: 0,
            index: 0.0,
            step: 0.0,
            end: 0.0,
            looped: false,
            done: true,
        }
    }

    /// One never started, all zeros, for a block the core holds in its
    /// static memory (a channel's own block): it plays nothing until a
    /// `grain` or a loop starts it.
    #[must_use]
    pub const fn unstarted() -> Self {
        NoiseSource { done: false, ..NoiseSource::new() }
    }

    /// `src.playbackRate.value = playback; src.start(when, offset, duration)`.
    #[allow(clippy::too_many_arguments)]
    pub fn grain(
        &mut self,
        when: f64,
        offset: f64,
        duration: f64,
        playback: f64,
        len: usize,
        rate: f64,
    ) {
        let buffer = len as f64 / rate;
        let offset = offset.max(0.0).min(buffer);
        let end = math::round(frames(offset + duration.max(0.0), rate));
        self.begin(when, math::round(frames(offset, rate)), playback, rate);
        self.end = end.min(len as f64);
        self.looped = false;
        self.stop = u64::MAX;
    }

    /// `src.loop = true; src.playbackRate.value = playback; src.start(when);
    /// src.stop(stop)`.
    pub fn looped(&mut self, when: f64, stop: f64, playback: f64, len: usize, rate: f64) {
        self.looped_at(when, 0.0, stop, playback, len, rate);
    }

    /// The same, from `offset` seconds in: `src.start(when, offset)`.
    #[allow(clippy::too_many_arguments)]
    pub fn looped_at(
        &mut self,
        when: f64,
        offset: f64,
        stop: f64,
        playback: f64,
        len: usize,
        rate: f64,
    ) {
        let buffer = len as f64 / rate;
        let offset = offset.max(0.0).min(buffer);
        self.begin(when, math::round(frames(offset, rate)), playback, rate);
        self.end = len as f64;
        self.looped = true;
        self.stop = frame_at(stop, rate);
    }

    fn begin(&mut self, when: f64, index: f64, playback: f64, rate: f64) {
        self.start = frame_at(when, rate);
        // A k-rate AudioParam: a 32-bit float.
        self.step = f64::from(playback as f32);
        self.index = index;
        // The fraction of a frame the start fell before its first frame.
        let missed = when * rate - self.start as f64;
        if missed < 0.0 {
            self.index += (missed * self.step).abs();
        }
        self.done = false;
    }

    /// The same as `new`, but all zeros, for a pool that must cost nothing
    /// in the `.wasm`: with no stop it plays nothing until `grain` or
    /// `looped`.
    #[must_use]
    pub const fn idle() -> Self {
        NoiseSource {
            done: false,
            ..NoiseSource::new()
        }
    }

    /// The first frame it plays.
    #[must_use]
    pub fn start_frame(&self) -> u64 {
        self.start
    }

    /// The frame it has certainly finished by: its stop, or for a grain
    /// the frame its read position reaches the grain's end.
    #[must_use]
    pub fn end_frame(&self) -> u64 {
        if self.looped {
            return self.stop;
        }
        let left = (self.end - self.index).max(0.0);
        let frames = if self.step > 0.0 {
            math::ceil(left / self.step) as u64
        } else {
            0
        };
        self.start + frames + 1
    }

    /// Render the block from frame `block` into `out`, every sample
    /// written, zero where it does not play. Returns the range it played
    /// in.
    pub fn fill(
        &mut self,
        block: u64,
        noise: &[f32],
        out: &mut [f32; QUANTUM],
    ) -> (usize, usize) {
        out.fill(0.0);
        let end = block + QUANTUM as u64;
        let lo = self.start.max(block);
        let hi = self.stop.min(end);
        if self.done || lo >= hi {
            return (0, 0);
        }
        let len = noise.len();
        let size = len as f64;
        if len < 2 || self.step <= 0.0 {
            self.done = true;
            return (0, 0);
        }
        let (lo, hi) = ((lo - block) as usize, (hi - block) as usize);
        let mut last = lo;
        for y in out.iter_mut().take(hi).skip(lo) {
            if !self.looped && self.index >= self.end {
                self.done = true;
                break;
            }
            let at = math::floor(self.index);
            let fraction = self.index - at;
            let i = (at as usize).min(len - 1);
            let mut j = i + 1;
            if j >= len {
                j = if self.looped {
                    (math::floor(self.index + 1.0 - size).max(0.0) as usize).min(len - 1)
                } else {
                    i
                };
            }
            let (a, b) = (
                f64::from(noise.get(i).copied().unwrap_or(0.0)),
                f64::from(noise.get(j).copied().unwrap_or(0.0)),
            );
            let sample = if i == j && i >= 1 {
                // The end of the buffer: on from the last two samples.
                let before = f64::from(noise.get(i - 1).copied().unwrap_or(0.0));
                a + (a - before) * fraction
            } else {
                (1.0 - fraction) * a + fraction * b
            };
            *y = sample as f32;
            last += 1;
            self.index += self.step;
            if self.index >= self.end {
                if self.looped {
                    self.index -= size;
                } else {
                    self.done = true;
                    break;
                }
            }
        }
        (lo, last)
    }
}

/// A grain of the noise, as a block plays one: from `time` seconds, from
/// `offset` seconds into the noise, `duration` long, at `playback` times
/// its speed.
#[derive(Clone, Copy)]
pub struct Settings {
    /// When it starts, in seconds.
    pub time: f64,
    /// Where in the noise it starts, in seconds.
    pub offset: f64,
    /// How long it plays, in seconds of the noise.
    pub duration: f64,
    /// How fast it reads the noise: 1 is as made.
    pub playback: f64,
}

/// A source of the noise as a block: a grain, added into the signal.
impl Block for NoiseSource {
    type Settings = Settings;

    const PROBES: &'static [Probe<Settings>] = &[
        Probe { name: "grain", settings: Settings { time: 0.05, offset: 0.3, duration: 0.4, playback: 1.0 } },
        Probe { name: "fast", settings: Settings { time: 0.0123, offset: 1.1, duration: 0.25, playback: 1.13 } },
    ];

    fn start(&mut self, s: &Settings, cx: &Context) {
        self.grain(s.time, s.offset, s.duration, s.playback, cx.noise.len(), cx.rate);
    }

    fn span(&self) -> (u64, u64) {
        (self.start_frame(), self.end_frame())
    }

    fn render(&mut self, block: u64, cx: &Context, signal: &mut Signal) {
        let mut x = [0.0; QUANTUM];
        let (lo, hi) = self.fill(block, cx.noise, &mut x);
        for (y, v) in signal.iter_mut().zip(x.iter()).take(hi).skip(lo) {
            *y += *v;
        }
    }
}

impl Default for NoiseSource {
    fn default() -> Self {
        Self::new()
    }
}
