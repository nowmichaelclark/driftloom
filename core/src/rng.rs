//! Seeded draws: every bit of chance in a note's sound comes from here.
//!
//! The core never asks the platform for a random number. A draw is a pure
//! function of where a note sits in its loop (`Key`), the channel it plays
//! in, a fixed number for the kind of draw (`salt`) and which draw of that
//! kind it is (`index`). So the same loop makes the same sound on every
//! device and every time, "Let the loop wander" still varies from pass to
//! pass (the pass is in the key), and adding a new kind of draw later
//! cannot shift any other: draws are looked up by name, never taken in turn
//! from a stream, so how many a note makes (a long sung note takes more
//! steps than a short one) changes nothing after it.
//!
//! The hash is plain 32-bit integer arithmetic (multiply, xor, shift), which
//! is exact on every target. A draw is the top 24 bits of the hash, over
//! 2^24: a number in [0, 1) a 32-bit float holds exactly, so no rounding
//! can differ anywhere.
//!
//! `js/draw.js` is the same function in JavaScript, for the few decisions the
//! page still takes itself (when a note starts, whether a sung vowel drifts;
//! see the list in `salt`). `test/draw-vectors.txt` holds values both must
//! make, and both sides' tests check them.

use crate::math;

/// Where a note sits in its loop: the loop's seed, which pass of the loop
/// it is (the pass count rises each time the loop comes round), the step
/// within the pass, and the note's place among the notes its layer plays at
/// that step. The host numbers them; the channel (the layer) comes with the
/// note. A note's key does not depend on what the voice budget did to the
/// notes before it, nor on whether a layer is muted, so the same note makes
/// the same draws in full and lite quality and with the drums off.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Key {
    pub seed: u32,
    pub pass: u32,
    pub step: u32,
    pub ord: u32,
}

/// The kinds of draw, each a number fixed for good: changing one changes the
/// sound of every loop that draws it, so a new kind takes the next free
/// number and the old ones never move. `HOST_` kinds are the ones the page
/// draws itself, from `js/draw.js`, for decisions it makes from them
/// (`js/draw.js` repeats these numbers).
pub mod salt {
    /// The noise every breath, strike and drum plays (`noise_buffer`).
    pub const NOISE: u32 = 1;

    /// The fiddle's vibrato: its rate, where the rate drifts to, its depth.
    pub const FIDDLE_RATE: u32 = 10;
    pub const FIDDLE_END: u32 = 11;
    pub const FIDDLE_DEPTH: u32 = 12;
    /// Whether a wind note slides into its pitch.
    pub const SLIDE: u32 = 13;
    /// The rate its breath of noise plays at.
    pub const BREATH_RATE: u32 = 14;
    /// The pan flute's vibrato rate.
    pub const PAN_VIBRATO: u32 = 15;
    /// How late the accordion's second reed comes in.
    pub const REED_LATE: u32 = 16;
    /// The five partials of a tubular bell, one index each.
    pub const TUBULAR_DETUNE: u32 = 17;
    /// The temple bell's eight partials, one index each.
    pub const TEMPLE_DETUNE: u32 = 18;
    /// The grain of noise a strike, a click or a drum plays: its rate and
    /// where in the noise it starts (index 0 and 1), and the knock's and the
    /// drop's band (index 2).
    pub const GRAIN: u32 = 19;
    /// A tambourine's five zils, one index each.
    pub const ZIL: u32 = 20;
    /// The wind texture's slow sine, and the waves' rate.
    pub const TEXTURE_RATE: u32 = 21;

    /// A sung note's choices. Each singer's are numbered from
    /// `SUNG_SINGER * singer`: the choir's spread, the scoop's start and end,
    /// every step of the jitter (a step's value is index `2 * step`, its wait
    /// the one after), and the vibrato's rate, depth and arrival; the breath's
    /// rate is the note's.
    pub const SUNG_SPREAD: u32 = 30;
    pub const SUNG_SCOOP: u32 = 31;
    pub const SUNG_JITTER: u32 = 32;
    pub const SUNG_VIBRATO: u32 = 33;
    pub const SUNG_BREATH: u32 = 34;
    /// Indices between one singer's draws and the next's.
    pub const SUNG_SINGER: u32 = 1000;

    /// The page's: a drum's timing jitter, a layer's ensemble slip, a chord's
    /// strum span, whether a held vowel drifts and where to, whether a chord
    /// rings out as glass and on which notes (queue item 59), and whether an
    /// offbeat takes a dubStab (queue item 60).
    pub const HOST_JITTER: u32 = 100;
    pub const HOST_ENSEMBLE: u32 = 101;
    pub const HOST_STRUM: u32 = 102;
    pub const HOST_DRIFT: u32 = 103;
    pub const HOST_DRIFT_TO: u32 = 104;
    pub const HOST_GLASS: u32 = 105;
    pub const HOST_STAB: u32 = 106;
}

// Multiplying and shifting a word into the hash, in the order of the
// finalizer of MurmurHash3 (a bijection, so no two words collide for the
// same state).
const fn fold(h: u32, word: u32) -> u32 {
    let mut h = (h ^ word).wrapping_mul(0x85eb_ca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2_ae35);
    h ^ (h >> 16)
}

/// The hash of one draw: 32 bits. The same function as `bits` in
/// `js/draw.js`.
#[must_use]
pub const fn bits(key: Key, channel: u32, salt: u32, index: u32) -> u32 {
    let mut h = 0x9e37_79b9;
    h = fold(h, key.seed);
    h = fold(h, key.pass);
    h = fold(h, key.step);
    h = fold(h, channel);
    h = fold(h, key.ord);
    h = fold(h, salt);
    fold(h, index)
}

/// A draw in [0, 1): the hash's top 24 bits over 2^24. Exact in a 32-bit
/// float, so it is the same number on every target.
#[must_use]
pub fn unit(key: Key, channel: u32, salt: u32, index: u32) -> f64 {
    f64::from(bits(key, channel, salt, index) >> 8) / 16_777_216.0
}

/// One note's source of draws: its key and its channel.
#[derive(Clone, Copy, Debug)]
pub struct Draws {
    key: Key,
    channel: u32,
}

impl Draws {
    #[must_use]
    pub const fn new(key: Key, channel: u32) -> Self {
        Draws { key, channel }
    }

    /// The draw of kind `salt`, number `index`, in [0, 1).
    #[must_use]
    pub fn unit(&self, salt: u32, index: u32) -> f64 {
        unit(self.key, self.channel, salt, index)
    }
}

/// The key the noise buffer's draws use: nothing to do with any loop, so
/// the noise is the same for every loop and every host.
const NOISE_KEY: Key = Key {
    seed: 0x6e6f_6973,
    pass: 0,
    step: 0,
    ord: 0,
};

/// The rate the noise's colour was tuned at, in Hz: at this rate the noise
/// is exactly what the synth's `_makeNoise` made.
const NOISE_TUNED_AT: f64 = 48_000.0;

/// Fill `out` with the noise every breath, strike and drum plays at `rate`
/// Hz: white noise lightly pinked, so it sounds like air and not a hiss.
/// The recipe is the JavaScript synth's `_makeNoise`, with the white drawn
/// from the core's own generator, carried to any rate: it was written as
/// three one-pole filters whose poles are per-sample numbers, which at
/// another rate would put the noise's colour somewhere else in Hz. Each
/// pole is moved to keep its time constant (`p ^ (48000 / rate)`), its
/// input gain to keep its gain at low frequencies, and the white's level to
/// keep its density per Hz, so the noise sounds the same, band for band
/// below 20 kHz, at every rate. At 48,000 Hz every factor is exactly 1, so
/// the noise is what it was there.
pub fn noise_buffer(out: &mut [f32], rate: f64) {
    let ratio = NOISE_TUNED_AT / rate;
    // (pole, input gain) of the three one-pole sections, moved to `rate`.
    let section = |pole: f64, gain: f64| {
        let moved = math::pow(pole, ratio);
        (moved, gain * (1.0 - moved) / (1.0 - pole))
    };
    let ((p0, g0), (p1, g1), (p2, g2)) = (section(0.99765, 0.099), section(0.963, 0.293), section(0.57, 1.0526));
    let white_level = math::sqrt(1.0 / ratio);
    let (mut b0, mut b1, mut b2) = (0.0_f64, 0.0_f64, 0.0_f64);
    for (i, sample) in (0_u32..).zip(out.iter_mut()) {
        let white = (unit(NOISE_KEY, 0, salt::NOISE, i) * 2.0 - 1.0) * white_level;
        b0 = p0 * b0 + white * g0;
        b1 = p1 * b1 + white * g1;
        b2 = p2 * b2 + white * g2;
        // Stored as the JavaScript buffer stores it: a 32-bit float.
        #[allow(clippy::cast_possible_truncation)]
        let value = ((b0 + b1 + b2 + white * 0.1848) * 0.22) as f32;
        *sample = value;
    }
}

// Tests count and bucket with casts of values they have just bounded.
#[cfg(test)]
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::type_complexity
)]
mod tests {
    use super::*;

    fn key(seed: u32, pass: u32, step: u32, ord: u32) -> Key {
        Key { seed, pass, step, ord }
    }

    #[test]
    fn a_draw_is_a_pure_function_of_where_it_is_and_what_kind() {
        let k = key(7, 1, 2, 3);
        assert_eq!(unit(k, 0, salt::SLIDE, 0).to_bits(), unit(k, 0, salt::SLIDE, 0).to_bits());
        // Each word of the key, the channel, the kind and the index matter.
        let base = bits(k, 0, salt::SLIDE, 0);
        for other in [
            bits(key(8, 1, 2, 3), 0, salt::SLIDE, 0),
            bits(key(7, 2, 2, 3), 0, salt::SLIDE, 0),
            bits(key(7, 1, 3, 3), 0, salt::SLIDE, 0),
            bits(key(7, 1, 2, 4), 0, salt::SLIDE, 0),
            bits(k, 1, salt::SLIDE, 0),
            bits(k, 0, salt::BREATH_RATE, 0),
            bits(k, 0, salt::SLIDE, 1),
        ] {
            assert_ne!(base, other);
        }
    }

    /// Draws are in [0, 1), spread evenly: mean 1/2, spread 1/sqrt(12),
    /// every tenth of the range about a tenth of the draws.
    #[test]
    fn draws_are_even_over_the_range() {
        let n = 200_000_u32;
        let (mut sum, mut sq, mut lo, mut hi) = (0.0, 0.0, f64::MAX, f64::MIN);
        let mut tenths = [0_u32; 10];
        for i in 0..n {
            // Walk every word of the key, as notes do.
            let k = key(i.wrapping_mul(2_654_435_761), i % 7, i % 64, i % 40);
            let x = unit(k, i % 5, salt::GRAIN, i % 3);
            sum += x;
            sq += x * x;
            lo = lo.min(x);
            hi = hi.max(x);
            tenths[(x * 10.0) as usize] += 1;
        }
        let mean = sum / f64::from(n);
        let sd = (sq / f64::from(n) - mean * mean).sqrt();
        assert!((mean - 0.5).abs() < 0.003, "mean {mean}");
        assert!((sd - (1.0 / 12.0_f64).sqrt()).abs() < 0.002, "sd {sd}");
        assert!(lo >= 0.0 && hi < 1.0, "range {lo} {hi}");
        assert!(lo < 0.001 && hi > 0.999, "range {lo} {hi}");
        for t in tenths {
            let share = f64::from(t) / f64::from(n);
            assert!((share - 0.1).abs() < 0.003, "tenth {share}");
        }
    }

    /// Neighbouring notes must not make neighbouring draws: no serial
    /// correlation along any one word of the key.
    #[test]
    fn neighbours_are_not_alike() {
        let n = 100_000_u32;
        let walks: [fn(u32) -> (Key, u32, u32); 4] = [
            |i| (key(1, 0, 0, i), 0, 0),
            |i| (key(1, 0, i, 0), 0, 0),
            |i| (key(1, i, 0, 0), 0, 0),
            |i| (key(i, 0, 0, 0), 0, 0),
        ];
        for walk in walks {
            let xs: Vec<f64> = (0..n)
                .map(|i| {
                    let (k, c, x) = walk(i);
                    unit(k, c, salt::SLIDE, x)
                })
                .collect();
            let mean = xs.iter().sum::<f64>() / f64::from(n);
            let var = xs.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / f64::from(n);
            let cov = xs.windows(2).map(|w| (w[0] - mean) * (w[1] - mean)).sum::<f64>() / f64::from(n - 1);
            assert!((cov / var).abs() < 0.01, "serial correlation {}", cov / var);
        }
    }

    /// Flipping one bit of any input flips about half the 32 bits of the
    /// hash: an avalanche, so a one-step change of a key is not a small
    /// change of the draw.
    #[test]
    fn one_changed_bit_changes_half_the_hash() {
        let mut flipped = 0_u32;
        let mut trials = 0_u32;
        for s in 0..64_u32 {
            let k = key(s.wrapping_mul(0x9e37_79b1), s, s + 3, s + 5);
            let h = bits(k, s % 5, salt::GRAIN, s);
            for bit in 0..32 {
                let m = 1_u32 << bit;
                for other in [
                    bits(Key { seed: k.seed ^ m, ..k }, s % 5, salt::GRAIN, s),
                    bits(Key { pass: k.pass ^ m, ..k }, s % 5, salt::GRAIN, s),
                    bits(Key { step: k.step ^ m, ..k }, s % 5, salt::GRAIN, s),
                    bits(Key { ord: k.ord ^ m, ..k }, s % 5, salt::GRAIN, s),
                    bits(k, (s % 5) ^ m, salt::GRAIN, s),
                    bits(k, s % 5, salt::GRAIN ^ m, s),
                    bits(k, s % 5, salt::GRAIN, s ^ m),
                ] {
                    flipped += (h ^ other).count_ones();
                    trials += 1;
                }
            }
        }
        let mean = f64::from(flipped) / f64::from(trials);
        assert!((mean - 16.0).abs() < 0.5, "{mean} bits flip on average");
    }

    /// No two kinds of draw share a number.
    #[test]
    fn kinds_have_numbers_of_their_own() {
        let all = [
            salt::NOISE, salt::FIDDLE_RATE, salt::FIDDLE_END, salt::FIDDLE_DEPTH, salt::SLIDE,
            salt::BREATH_RATE, salt::PAN_VIBRATO, salt::REED_LATE, salt::TUBULAR_DETUNE,
            salt::TEMPLE_DETUNE, salt::GRAIN, salt::ZIL, salt::TEXTURE_RATE, salt::SUNG_SPREAD,
            salt::SUNG_SCOOP, salt::SUNG_JITTER, salt::SUNG_VIBRATO, salt::SUNG_BREATH,
            salt::HOST_JITTER, salt::HOST_ENSEMBLE, salt::HOST_STRUM, salt::HOST_DRIFT,
            salt::HOST_DRIFT_TO, salt::HOST_GLASS, salt::HOST_STAB,
        ];
        let mut sorted = all.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), all.len());
    }

    /// The noise is the synth's: the same recipe fed by the platform's
    /// random numbers has a level (rms) of 0.37 to 0.39 and a peak of 1.4 to
    /// 1.6, and a mean that wanders by a few hundredths (its slowest pole
    /// holds a few hundred samples); and it is the same every time.
    #[test]
    fn the_noise_is_pinked_white() {
        let mut a = vec![0.0_f32; 96_000];
        let mut b = vec![0.0_f32; 96_000];
        noise_buffer(&mut a, 48_000.0);
        noise_buffer(&mut b, 48_000.0);
        assert_eq!(a, b);
        let mean = a.iter().map(|x| f64::from(*x)).sum::<f64>() / a.len() as f64;
        let rms = (a.iter().map(|x| f64::from(*x * *x)).sum::<f64>() / a.len() as f64).sqrt();
        let peak = a.iter().fold(0.0_f32, |m, x| m.max(x.abs()));
        assert!(mean.abs() < 0.08, "mean {mean}");
        assert!((0.33..0.43).contains(&rms), "rms {rms}");
        assert!(peak > 1.2 && peak < 1.9, "peak {peak}");
    }

    /// The noise has the same density per Hz at every rate, in the bands
    /// that all of them hold: its colour is in Hz, not in samples. Welch's
    /// estimate at six frequencies, in dB under the one at 1 kHz, within 2 dB
    /// of the 48 kHz noise's (the estimates themselves scatter by about 1 dB;
    /// left in samples, the colour is several dB out at 22 and 192 kHz).
    #[test]
    fn the_noise_sounds_the_same_at_any_rate() {
        let density = |rate: f64, hz: f64| -> f64 {
            let mut noise = vec![0.0_f32; (rate * 8.0) as usize];
            noise_buffer(&mut noise, rate);
            // Goertzel over 0.1 s windows, averaged.
            let n = (rate / 10.0) as usize;
            let k = (hz * n as f64 / rate).round();
            let w = 2.0 * std::f64::consts::PI * k / n as f64;
            let c = 2.0 * w.cos();
            let (mut sum, mut count) = (0.0, 0);
            for chunk in noise.chunks_exact(n) {
                let (mut s1, mut s2) = (0.0_f64, 0.0_f64);
                for (i, x) in chunk.iter().enumerate() {
                    // A Hann window.
                    let hann = 0.5 - 0.5 * (2.0 * std::f64::consts::PI * i as f64 / n as f64).cos();
                    let s0 = f64::from(*x) * hann + c * s1 - s2;
                    s2 = s1;
                    s1 = s0;
                }
                sum += s1 * s1 + s2 * s2 - c * s1 * s2;
                count += 1;
            }
            // Per Hz: the rate sets the scale (a density is per second).
            10.0 * (sum / f64::from(count) / rate).log10()
        };
        let bands = [100.0, 300.0, 1000.0, 3000.0, 6000.0, 9000.0];
        let at = |rate: f64| bands.map(|hz| density(rate, hz) - density(rate, 1000.0));
        let reference = at(48_000.0);
        for rate in [22_050.0, 44_100.0, 96_000.0, 192_000.0] {
            for ((got, want), hz) in at(rate).iter().zip(reference).zip(bands) {
                assert!((got - want).abs() < 2.0, "{rate} Hz at {hz} Hz: {got:.1} dB against {want:.1}");
            }
        }
    }
}
