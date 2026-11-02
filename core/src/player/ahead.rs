//! Rendering ahead (queue item 87): what a host with no audio thread needs
//! to render a loop before it plays it. Godot's web build is one: music fed
//! frame by frame crackled on a phone, so Broken Friends renders passes
//! ahead and plays each as one sound.
//!
//! **Chunks.** `Player::render` takes any number of frames a call and
//! carries on exactly where the last call stopped: the core renders whole
//! blocks and hands over what the host asked for, keeping the rest of the
//! last block for the next call (`Core::render_with`), and the player
//! decides before each block on the core's own clock. So a loop rendered in
//! chunks of any size is the same samples as one rendered straight through,
//! wandering, endless, with a change waiting at a boundary (held by
//! `tests/ahead.rs`, and over chunk sizes from a block to a pass in
//! WebAssembly by `tools/ahead.mjs --sweep`). A host asks for the pass
//! playing (`pass_frames`) or for N seconds.
//!
//! **What a chunk costs.** `estimate` says how long a chunk will take to
//! render, from the loop's voices: each voice asks the budget for its cost
//! units (`data/voices.txt`, measured by render timing), so the units they
//! hold over a pass (`units`) say how much work it is, and a block costs a
//! fixed part (the mix, the master chain) and a part for each unit held.
//! The figures are in microseconds on the reference host, the player in
//! WebAssembly in Node on the machine the hands measured on (`tools/
//! ahead.mjs --estimate` checks them). A host scales them to
//! its own device: it times a chunk, divides by the chunk's estimate, and
//! multiplies the next chunk's estimate by that, so it can spread the next
//! chunk across as many frames as it needs. The units are the budget's (a
//! note holds its cost past its end, through its tail), not a count of the
//! voices sounding, so the estimate is rough for a loop not yet heard (a
//! pass's cost is within about a third of it, either way) and close once a
//! host scales it by what the last pass took; the first pass is best made
//! where there is no deadline (`ready`, a loading screen).

use super::{Player, State};
use crate::{Core, QUANTUM};

/// A block's fixed cost, in microseconds on the reference host, at full
/// quality and lite, mono and stereo (`[full, lite][mono, stereo]`): fitted
/// with `UNIT_US` by least squares over the baseline's 40 loops (corpus seed
/// 1), each block's time against the units held (`tools/ahead.mjs
/// --estimate` checks the fit).
pub const BLOCK_US: [[f64; 2]; 2] = [[26.0, 84.0], [37.0, 57.0]];
/// What each cost unit the voices hold adds to a block, in microseconds on
/// the reference host (`[full, lite][mono, stereo]`; with `BLOCK_US`).
pub const UNIT_US: [[f64; 2]; 2] = [[0.83, 0.78], [0.49, 0.51]];
/// The units the voices hold before any are measured (a loop just made):
/// the median over the same 40 loops, at full quality and at lite.
pub const TYPICAL_UNITS: [f64; 2] = [153.0, 103.0];
/// How much of a pass must be decided before its mean is read, in seconds,
/// while no whole pass has been.
const MIN_SPAN: f64 = 1.0;

impl Player {
    /// The frames from where the host stands (`Core::served`) to the end of
    /// the pass playing: the first frame at or after the next pass's first
    /// step, on the core's clock at the loop's tempo. A host rendering a
    /// pass at a time asks this before each chunk; a loop just made with
    /// nothing before it (`ready`) gives its first pass with the lead-in
    /// before its first step (`START`). None while no loop plays
    /// (one being made: `ready` makes it). A change waiting may end the
    /// loop before the pass does; a tempo set later moves the end.
    #[must_use]
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)] // frames and steps, far below 2^53
    pub fn pass_frames(&self, core: &Core) -> Option<u64> {
        if self.state != State::Playing || self.rate <= 0.0 {
            return None;
        }
        let g = &self.generator;
        let total = g.spec().bars * g.spec().steps_per_bar;
        let sd = self.seq.step_dur();
        if !(total >= 1.0 && sd.is_finite() && sd > 0.0) {
            return None;
        }
        let total_steps = total as u64;
        // A loop that began after a lead-in and has decided no step yet: its
        // first pass, lead-in and all, not the lead-in alone.
        let left = if self.lead_in && self.seq.abs_step == 0 { total_steps } else { (total_steps - self.seq.step % total_steps) % total_steps };
        let mut end = self.seq.next_step_time + left as f64 * sd;
        let served = core.served();
        let frame = |t: f64| crate::math::ceil(t * self.rate).max(0.0) as u64;
        let mut guard = 0;
        while frame(end) <= served && guard < 64 {
            end += total * sd;
            guard += 1;
        }
        Some(frame(end) - served)
    }

    /// The mean cost units the loop's voices hold, from the last whole pass
    /// decided (its notes' costs times how long each held, over the pass's
    /// length); before a whole pass is, from the part decided if it is a
    /// second or more. None for a loop just made.
    #[must_use]
    pub fn units(&self) -> Option<f64> {
        if self.units >= 0.0 {
            return Some(self.units);
        }
        if self.state != State::Playing {
            return None;
        }
        let span = self.seq.next_step_time - self.pass_from;
        (span >= MIN_SPAN).then(|| (self.synth.budget.held() - self.pass_held) / span)
    }

    /// How long rendering the next `frames` frames will take, in
    /// microseconds on the reference host (the module's doc says how a host
    /// scales it to its own): a block's fixed cost and each unit the voices
    /// hold (`units`, or `TYPICAL_UNITS` before any is measured), at the
    /// player's quality and the core's layout.
    #[must_use]
    #[allow(clippy::cast_precision_loss)] // frames, far below 2^53
    pub fn estimate(&self, core: &Core, frames: u64) -> f64 {
        let q = usize::from(!self.full);
        let l = usize::from(core.layout() == 2);
        let units = self.units().unwrap_or(TYPICAL_UNITS[q]);
        let per_block = BLOCK_US[q][l] + UNIT_US[q][l] * units;
        frames as f64 / QUANTUM as f64 * per_block
    }
}
