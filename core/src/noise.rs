//! The noise every breath, strike and drum plays. A source plays it
//! (`blocks::noise_source`).
//!
//! The core makes the noise itself, once, as it starts: two seconds of
//! white noise lightly pinked, from its own seeded generator (`rng`), so
//! every host and every run plays the same noise. Every note reads it.

use crate::rng;

/// How long the noise is, in seconds.
pub const SECONDS: f64 = 2.0;

/// The longest noise the core holds: two seconds at `MAX_RATE`. A context
/// faster than that keeps its noise voices in JavaScript.
pub const NOISE_MAX: usize = crate::MAX_FRAMES_A_SECOND * 2;

pub struct Noise {
    samples: [f32; NOISE_MAX],
    len: usize,
}

impl Noise {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`;
    /// starting it sets every value a render reads.
    // Built in a `const` for the static core, never on a stack at run time.
    #[allow(clippy::large_stack_arrays)]
    #[must_use]
    pub const fn new() -> Self {
        Noise {
            samples: [0.0; NOISE_MAX],
            len: 0,
        }
    }

    /// Make the noise for a context running at `rate` Hz: two seconds
    /// (`SECONDS`), the same length the JavaScript synth's buffer is. None,
    /// and no noise, when that is longer than the core holds.
    pub fn make(&mut self, rate: f64) {
        // A rate is a positive whole number of Hz well under what a usize
        // holds, so the cast loses nothing a sample count needs.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let len = (rate * SECONDS) as usize;
        if len > NOISE_MAX {
            self.len = 0;
            return;
        }
        self.len = len;
        if let Some(space) = self.samples.get_mut(..len) {
            rng::noise_buffer(space, rate);
        }
    }

    #[must_use]
    pub fn samples(&self) -> &[f32] {
        self.samples.get(..self.len).unwrap_or(&[])
    }

    /// Whether there is any: a voice that needs it plays in JavaScript
    /// otherwise.
    #[must_use]
    pub fn ready(&self) -> bool {
        self.len > 1
    }

    /// The buffer's length in seconds, as `AudioBuffer.duration` gives it.
    // Two seconds of frames, as a float: far below 2^52.
    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub fn duration(&self, rate: f64) -> f64 {
        self.len as f64 / rate
    }
}

impl Default for Noise {
    fn default() -> Self {
        Self::new()
    }
}
