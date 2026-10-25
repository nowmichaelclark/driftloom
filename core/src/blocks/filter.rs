//! A biquad filter that filters the way a Web Audio `BiquadFilterNode` does
//! in Chromium (`biquad.cc`): the Audio EQ Cookbook's coefficients worked
//! out in 64-bit floats from the node's 32-bit parameters, run in 64-bit
//! floats, with each output rounded to a 32-bit float before it is fed
//! back. A lowpass's and a highpass's Q is in dB, as Web Audio has them; a
//! bandpass's is linear.
//!
//! When a parameter moves, Chromium works the coefficients out again for
//! every sample; `set` caches the last ones, so a filter whose parameters
//! hold still costs a comparison a sample.

use crate::block::{Block, Context, Mod, Mods, Probe, Signal};
use crate::math;

/// A filter's response, as Web Audio names them.
#[derive(Clone, Copy, PartialEq)]
pub enum Kind {
    /// `'lowpass'`: Q in dB.
    Lowpass,
    /// `'highpass'`: Q in dB.
    Highpass,
    /// `'bandpass'`: Q linear.
    Bandpass,
    /// `'peaking'`: a boost or cut of `gain` dB around the frequency.
    Peaking,
}

/// One biquad: its coefficients and the two samples of memory each side.
#[derive(Clone, Copy)]
pub struct Biquad {
    b0: f64,
    b1: f64,
    b2: f64,
    a1: f64,
    a2: f64,
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
    /// The parameters the coefficients were last worked out for, once
    /// there are any (`set` has been called).
    last: (f32, f32, f32),
    ready: bool,
    /// 10^(q/20) and 10^(gain/40), kept for the q and gain they were worked
    /// out for: a cutoff that moves on every frame works the coefficients
    /// out on every frame, and the power is the slow part of that.
    resonance: Decibels,
    amplitude: Decibels,
}

/// `10^(x / per)`, kept for the last `x`. All zeros until used, like the
/// filter.
#[derive(Clone, Copy)]
struct Decibels {
    ready: bool,
    of: f32,
    value: f64,
}

impl Decibels {
    const fn new() -> Self {
        Decibels { ready: false, of: 0.0, value: 0.0 }
    }

    fn get(&mut self, x: f32, per: f64) -> f64 {
        if !self.ready || self.of.to_bits() != x.to_bits() {
            self.value = math::pow(10.0, x as f64 / per);
            self.of = x;
            self.ready = true;
        }
        self.value
    }
}

impl Biquad {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`;
    /// starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        Biquad {
            b0: 0.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
            last: (0.0, 0.0, 0.0),
            ready: false,
            resonance: Decibels::new(),
            amplitude: Decibels::new(),
        }
    }

    /// Silence, and no coefficients yet. All zeros, like `new`, so a pool
    /// of filters costs nothing in the `.wasm`.
    pub fn reset(&mut self) {
        *self = Biquad::new();
    }

    /// Work out the coefficients for `kind` at `freq` Hz, `q` and `gain` dB,
    /// unless they are what they were.
    pub fn set(&mut self, kind: Kind, freq: f32, q: f32, gain: f32, rate: f32) {
        if self.ready && self.last == (freq, q, gain) {
            return;
        }
        self.last = (freq, q, gain);
        self.ready = true;
        let nyquist = rate as f64 / 2.0;
        let w = (freq as f64 / nyquist).clamp(0.0, 1.0);
        let (b0, b1, b2, a0, a1, a2) = match kind {
            Kind::Lowpass => {
                if w >= 1.0 {
                    (1.0, 0.0, 0.0, 1.0, 0.0, 0.0)
                } else if w > 0.0 {
                    let resonance = self.resonance.get(q, 20.0);
                    let theta = core::f64::consts::PI * w;
                    let alpha = math::sin(theta) / (2.0 * resonance);
                    let cosw = math::cos(theta);
                    let beta = (1.0 - cosw) / 2.0;
                    (
                        beta,
                        2.0 * beta,
                        beta,
                        1.0 + alpha,
                        -2.0 * cosw,
                        1.0 - alpha,
                    )
                } else {
                    (0.0, 0.0, 0.0, 1.0, 0.0, 0.0)
                }
            }
            Kind::Highpass => {
                if w >= 1.0 {
                    // The z-transform is 0.
                    (0.0, 0.0, 0.0, 1.0, 0.0, 0.0)
                } else if w > 0.0 {
                    let resonance = self.resonance.get(q, 20.0);
                    let theta = core::f64::consts::PI * w;
                    let alpha = math::sin(theta) / (2.0 * resonance);
                    let cosw = math::cos(theta);
                    let beta = (1.0 + cosw) / 2.0;
                    (
                        beta,
                        -2.0 * beta,
                        beta,
                        1.0 + alpha,
                        -2.0 * cosw,
                        1.0 - alpha,
                    )
                } else {
                    // At zero the poles and zeros meet: the z-transform is 1.
                    (1.0, 0.0, 0.0, 1.0, 0.0, 0.0)
                }
            }
            Kind::Bandpass => {
                let q = (q as f64).max(0.0);
                if w > 0.0 && w < 1.0 {
                    if q > 0.0 {
                        let w0 = core::f64::consts::PI * w;
                        let alpha = math::sin(w0) / (2.0 * q);
                        let k = math::cos(w0);
                        (alpha, 0.0, -alpha, 1.0 + alpha, -2.0 * k, 1.0 - alpha)
                    } else {
                        // The limit as Q goes to 0 is 1.
                        (1.0, 0.0, 0.0, 1.0, 0.0, 0.0)
                    }
                } else {
                    // At 0 and at the Nyquist frequency it passes nothing.
                    (0.0, 0.0, 0.0, 1.0, 0.0, 0.0)
                }
            }
            Kind::Peaking => {
                let q = (q as f64).max(0.0);
                let a = self.amplitude.get(gain, 40.0);
                if w > 0.0 && w < 1.0 {
                    if q > 0.0 {
                        let w0 = core::f64::consts::PI * w;
                        let alpha = math::sin(w0) / (2.0 * q);
                        let k = math::cos(w0);
                        (
                            1.0 + alpha * a,
                            -2.0 * k,
                            1.0 - alpha * a,
                            1.0 + alpha / a,
                            -2.0 * k,
                            1.0 - alpha / a,
                        )
                    } else {
                        (a * a, 0.0, 0.0, 1.0, 0.0, 0.0)
                    }
                } else {
                    (1.0, 0.0, 0.0, 1.0, 0.0, 0.0)
                }
            }
        };
        let inv = 1.0 / a0;
        self.b0 = b0 * inv;
        self.b1 = b1 * inv;
        self.b2 = b2 * inv;
        self.a1 = a1 * inv;
        self.a2 = a2 * inv;
    }

    /// Take another filter's coefficients, as worked out for its last
    /// `set`: filters that share their parameters work them out once.
    pub fn follow(&mut self, other: &Biquad) {
        self.b0 = other.b0;
        self.b1 = other.b1;
        self.b2 = other.b2;
        self.a1 = other.a1;
        self.a2 = other.a2;
        self.last = other.last;
        self.ready = other.ready;
    }

    /// One sample through the filter.
    pub fn step(&mut self, x: f32) -> f32 {
        let x = x as f64;
        let wide = self.b0 * x + self.b1 * self.x1 + self.b2 * self.x2
            - self.a1 * self.y1
            - self.a2 * self.y2;
        // A ringing tail decays toward the subnormal numbers; it stops at
        // the silence floor instead, before it is rounded to 32 bits, so
        // that no subnormal is ever made (`math`, "Subnormal numbers").
        let y = if wide.abs() < f64::from(math::SILENCE) { 0.0 } else { wide as f32 };
        self.x2 = self.x1;
        self.x1 = x;
        self.y2 = self.y1;
        self.y1 = y as f64;
        y
    }

    /// Whether the filter is still: nothing in its memory, so zeros in give
    /// zeros out.
    pub fn at_rest(&self) -> bool {
        self.x1 == 0.0 && self.x2 == 0.0 && self.y1 == 0.0 && self.y2 == 0.0
    }

    /// At the end of every block, as Chromium does: state too small for a
    /// 32-bit float goes to zero.
    pub fn flush(&mut self) {
        for v in [&mut self.x1, &mut self.x2, &mut self.y1, &mut self.y2] {
            if v.abs() < f32::MIN_POSITIVE as f64 {
                *v = 0.0;
            }
        }
    }
}

/// A filter's settings when it plays as a block: its kind, its frequency
/// in Hz, its Q (in dB for a lowpass or a highpass, as Web Audio has it)
/// and its gain in dB (a peaking filter's).
#[derive(Clone, Copy)]
pub struct Settings {
    /// Which response.
    pub kind: Kind,
    /// The corner or center frequency, in Hz.
    pub freq: f32,
    /// The resonance.
    pub q: f32,
    /// A peaking filter's boost, in dB.
    pub gain: f32,
}

/// The filter's settings a signal can drive (`Block::MODS`): its
/// frequency, in Hz -- a sweep, an envelope or an LFO into the cutoff.
pub const FREQ: usize = 0;
/// Its Q.
pub const Q: usize = 1;
/// Its gain, in dB.
pub const GAIN: usize = 2;

/// A filter as a block: a `Biquad` and its settings, a signal through it.
/// Undriven, it holds its settings still (coefficients worked out once, at
/// `start`). With a signal driving a setting (`FREQ`, `Q`, `GAIN`), each
/// frame's setting is its own value plus the signal's, and the
/// coefficients are worked out again whenever the three move, as a
/// `BiquadFilterNode` does under a moving parameter. The raw `Biquad`
/// stays the arithmetic the mix, the master and the bodies step directly.
#[derive(Clone, Copy)]
pub struct Filter {
    biquad: Biquad,
    settings: Settings,
}

impl Filter {
    /// Silent and all zeros.
    #[must_use]
    pub const fn new() -> Self {
        Filter { biquad: Biquad::new(), settings: Settings { kind: Kind::Lowpass, freq: 0.0, q: 0.0, gain: 0.0 } }
    }

    /// Set it to `settings`, at rest, at `rate` Hz: `Block::start` with a
    /// rate rather than a `Context`. A voice whose cutoff is all signal
    /// sets `freq` to zero and drives `FREQ` with the whole curve.
    pub fn set(&mut self, settings: Settings, rate: f32) {
        self.settings = settings;
        self.biquad.reset();
        self.biquad.set(settings.kind, settings.freq, settings.q, settings.gain, rate);
    }

    /// New settings for a filter already running, keeping its memory: the
    /// coefficients follow at the next frame they are worked out (a driven
    /// setting's, at once), as a `BiquadFilterNode`'s do when its parameters
    /// change under a signal (padSweep, queue item 72).
    pub fn retarget(&mut self, settings: Settings) {
        self.settings = settings;
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::new()
    }
}

impl Block for Filter {
    type Settings = Settings;

    const PROBES: &'static [Probe<Settings>] = &[
        Probe { name: "lowpass", settings: Settings { kind: Kind::Lowpass, freq: 1000.0, q: -3.0, gain: 0.0 } },
        Probe { name: "highpass", settings: Settings { kind: Kind::Highpass, freq: 2000.0, q: 6.0, gain: 0.0 } },
        Probe { name: "bandpass", settings: Settings { kind: Kind::Bandpass, freq: 1200.0, q: 2.2, gain: 0.0 } },
        Probe { name: "peaking", settings: Settings { kind: Kind::Peaking, freq: 800.0, q: 1.2, gain: 9.0 } },
    ];

    const INPUT: bool = true;

    const MODS: &'static [Mod] =
        &[Mod { name: "freq", swing: 600.0 }, Mod { name: "q", swing: 1.0 }, Mod { name: "gain", swing: 6.0 }];

    // Web Audio's filter parameters are 32-bit floats.
    #[allow(clippy::cast_possible_truncation)]
    fn start(&mut self, s: &Settings, cx: &Context) {
        self.set(*s, cx.rate as f32);
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    fn render(&mut self, block: u64, cx: &Context, signal: &mut Signal) {
        self.render_mod(block, cx, signal, &Mods::NONE);
    }

    // Web Audio's filter parameters are 32-bit floats.
    #[allow(clippy::cast_possible_truncation)]
    fn render_mod(&mut self, _: u64, cx: &Context, signal: &mut Signal, mods: &Mods) {
        let s = self.settings;
        match (mods.get(FREQ), mods.get(Q), mods.get(GAIN)) {
            (None, None, None) => {
                for x in signal.iter_mut() {
                    *x = self.biquad.step(*x);
                }
            }
            // A sweep, the common case: the frequency alone.
            (Some(freq), None, None) => {
                let r = cx.rate as f32;
                for (x, f) in signal.iter_mut().zip(freq.iter()) {
                    self.biquad.set(s.kind, s.freq + *f, s.q, s.gain, r);
                    *x = self.biquad.step(*x);
                }
            }
            (freq, q, gain) => {
                let r = cx.rate as f32;
                let at = |m: Option<&Signal>, i: usize, own: f32| m.and_then(|m| m.get(i)).map_or(own, |v| own + *v);
                for (i, x) in signal.iter_mut().enumerate() {
                    self.biquad.set(s.kind, at(freq, i, s.freq), at(q, i, s.q), at(gain, i, s.gain), r);
                    *x = self.biquad.step(*x);
                }
            }
        }
        self.biquad.flush();
    }
}

impl Default for Biquad {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_lowpass_passes_dc_and_stops_the_top() {
        let mut f = Biquad::new();
        f.set(Kind::Lowpass, 1000.0, -3.0, 0.0, 44100.0);
        let mut y = 0.0;
        for _ in 0..10000 {
            y = f.step(1.0);
        }
        assert!((y - 1.0).abs() < 1e-4);
        let mut g = Biquad::new();
        g.set(Kind::Lowpass, 1000.0, -3.0, 0.0, 44100.0);
        let mut peak = 0.0f32;
        for k in 0..10000 {
            let y = g.step(if k % 2 == 0 { 1.0 } else { -1.0 });
            if k > 1000 {
                peak = peak.max(y.abs());
            }
        }
        assert!(peak < 1e-3);
    }

    #[test]
    fn a_peaking_filter_lifts_its_band_by_its_gain() {
        let rate = 44100.0f32;
        let mut f = Biquad::new();
        f.set(Kind::Peaking, 1000.0, 1.0, 6.0, rate);
        let mut peak = 0.0f32;
        for k in 0..44100 {
            let x = (2.0 * core::f64::consts::PI * 1000.0 * k as f64 / rate as f64).sin() as f32;
            let y = f.step(x);
            if k > 22050 {
                peak = peak.max(y.abs());
            }
        }
        assert!((20.0 * peak.log10() - 6.0).abs() < 0.05, "{peak}");
    }
}
