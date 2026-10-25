//! The width ceiling (queue item 64; `docs/ears/batch-3.md`, "Stereo"): the
//! side of a stereo signal turned down wherever it rises above a target
//! against the mid, so no loop gets wider than its space recipe means, however
//! its parts happen to fall. The ears sketched it offline on finished renders
//! (`tools/ears/width_ceiling.py`); this is that sketch as a block.
//!
//! Each block it measures the mid, `(left + right) / 2`, and the side,
//! `(left - right) / 2`, above `cutoff` (a fourth-order Butterworth
//! highpass, as the sketch's), and keeps their energy over the last `window`
//! seconds. Where the side's share is above `target` (in dB against the mid),
//! the gain on the side that would bring it to the target is where the
//! side's gain heads: down with a time constant of `down` seconds, back up
//! with one of `up`. The mid passes untouched, so what a mono speaker hears
//! (the fold) is exactly what came in.
//!
//! Where the core must differ from the sketch: the sketch's window was
//! centered on each moment (it read a second ahead in the finished file); a
//! block can't look ahead, so its window is the last `window` seconds. It
//! moves its gain once a block (the sketch, every 10 ms) and glides it across
//! the block, as the sketch interpolated between its steps.
//!
//! In a mono render it does nothing: there is no side.

// Coefficients are worked out in 64-bit and run in 32, as the filters' are;
// the window's count of blocks is a clamped, positive number.
#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::cast_sign_loss)]

use crate::block::{Block, Context, Probe, Setting, Signal, Stereo};
use crate::math;
use crate::QUANTUM;

use super::filter::{Biquad, Kind};

/// The longest window it keeps, in seconds, and its blocks at the highest
/// rate the build takes.
pub const WINDOW_MAX: f64 = 4.0;
const SLOTS: usize = 4 * crate::MAX_FRAMES_A_SECOND / QUANTUM + 1;

/// The two sections of a fourth-order Butterworth highpass, as their Q in
/// Web Audio's dB: 0.5412 and 1.3066.
const BUTTERWORTH_Q: [f32; 2] = [-5.333, 2.323];

/// How it is set.
#[derive(Clone, Copy)]
pub struct Settings {
    /// How wide it may get: the side's energy against the mid's, in dB,
    /// above `cutoff`. The ears' recipes: -7 on busy loops, -9 sparse, -12
    /// very sparse.
    pub target: f64,
    /// What it measures above, in Hz (the low end stays centered anyway).
    pub cutoff: f64,
    /// How long it listens back, in seconds (up to `WINDOW_MAX`).
    pub window: f64,
    /// How fast the side comes down, a time constant in seconds.
    pub down: f64,
    /// How slowly it comes back, a time constant in seconds.
    pub up: f64,
}

/// The sketch's: -7 dB above 200 Hz over 2 s, down in about half a second,
/// back over about three.
pub const REST: Settings = Settings { target: -7.0, cutoff: 200.0, window: 2.0, down: 0.5, up: 3.0 };

/// All zeros: what a core built in a static holds until it starts.
pub const ZERO: Settings = Settings { target: 0.0, cutoff: 0.0, window: 0.0, down: 0.0, up: 0.0 };

/// Its settings by name, for a host that sets them (`Width::set`). The kit
/// can't hear any of them (it probes settings in a mono render, and its
/// stereo probe feeds both sides the same, so there is no side), so their
/// probes are NaN; the tests below check them.
pub const SETTINGS: &[Setting] = &[
    Setting { name: "target", probe: f64::NAN },
    Setting { name: "cutoff", probe: f64::NAN },
    Setting { name: "window", probe: f64::NAN },
    Setting { name: "down", probe: f64::NAN },
    Setting { name: "up", probe: f64::NAN },
];

/// The width ceiling's state.
pub struct Width {
    settings: Settings,
    rate: f64,
    mid: [Biquad; 2],
    side: [Biquad; 2],
    /// Each block's energy above the cutoff, mid and side, for the window.
    energy: [[f64; 2]; SLOTS],
    at: usize,
    slots: usize,
    sum: [f64; 2],
    /// The side's gain now, and the share of the way toward a target it
    /// moves in a block, down and up.
    gain: f64,
    fall: f64,
    rise: f64,
}

impl Width {
    /// All zeros; `ready` builds it.
    #[must_use]
    // Built in place in the core's static, never on a stack.
    #[allow(clippy::large_stack_arrays)]
    pub const fn new() -> Self {
        Width {
            settings: ZERO,
            rate: 0.0,
            mid: [Biquad::new(), Biquad::new()],
            side: [Biquad::new(), Biquad::new()],
            energy: [[0.0; 2]; SLOTS],
            at: 0,
            slots: 0,
            sum: [0.0; 2],
            gain: 0.0,
            fall: 0.0,
            rise: 0.0,
        }
    }

    /// At `rate` Hz with `settings`, nothing heard yet, the side at full
    /// width.
    pub fn ready(&mut self, settings: &Settings, rate: f64) {
        *self = Width::new();
        self.rate = rate;
        self.settings = *settings;
        self.configure();
        self.gain = 1.0;
    }

    /// Setting number `setting` (`SETTINGS`) becomes `value`, from the next
    /// block. A new window or cutoff starts listening afresh.
    pub fn set(&mut self, setting: usize, value: f64) {
        if !value.is_finite() {
            return;
        }
        match setting {
            0 => self.settings.target = value,
            1 => self.settings.cutoff = value,
            2 => self.settings.window = value,
            3 => self.settings.down = value,
            4 => self.settings.up = value,
            _ => return,
        }
        if setting == 1 || setting == 2 {
            self.energy.fill([0.0; 2]);
            self.sum = [0.0; 2];
            self.at = 0;
        }
        self.configure();
    }

    fn configure(&mut self) {
        let s = self.settings;
        let rate = self.rate.max(1.0);
        let cutoff = s.cutoff.clamp(10.0, 0.45 * rate) as f32;
        for (m, q) in self.mid.iter_mut().zip(BUTTERWORTH_Q) {
            m.set(Kind::Highpass, cutoff, q, 0.0, rate as f32);
        }
        for (m, q) in self.side.iter_mut().zip(BUTTERWORTH_Q) {
            m.set(Kind::Highpass, cutoff, q, 0.0, rate as f32);
        }
        let block = QUANTUM as f64 / rate;
        self.slots = ((s.window.clamp(0.0, WINDOW_MAX) / block) as usize).clamp(1, SLOTS);
        self.fall = 1.0 - math::exp(-block / s.down.max(block));
        self.rise = 1.0 - math::exp(-block / s.up.max(block));
    }

    /// The side's gain now, 0 to 1.
    #[must_use]
    pub fn gain(&self) -> f64 {
        self.gain
    }

    /// One block of a stereo signal, in place: the side measured and turned
    /// down to the target, the mid untouched.
    pub fn process(&mut self, signal: &mut Stereo) {
        let [left, right] = signal;
        let [m0, m1] = &mut self.mid;
        let [s0, s1] = &mut self.side;
        let mut mid = [0.0f32; QUANTUM];
        let mut side = [0.0f32; QUANTUM];
        let mut energy = [0.0f64; 2];
        for (((m, s), &l), &r) in mid.iter_mut().zip(side.iter_mut()).zip(left.iter()).zip(right.iter()) {
            *m = f32::midpoint(l, r);
            *s = (l - r) * 0.5;
            let hm = m1.step(m0.step(*m));
            let hs = s1.step(s0.step(*s));
            energy[0] += f64::from(hm) * f64::from(hm);
            energy[1] += f64::from(hs) * f64::from(hs);
        }
        for f in self.mid.iter_mut().chain(self.side.iter_mut()) {
            f.flush();
        }
        // The window: the oldest block out, this one in.
        if let Some(slot) = self.energy.get_mut(self.at) {
            for ((sum, old), new) in self.sum.iter_mut().zip(slot.iter_mut()).zip(energy) {
                *sum = (*sum - *old + new).max(0.0);
                *old = new;
            }
        }
        self.at = (self.at + 1) % self.slots.max(1);
        let [em, es] = self.sum;
        let allowed = math::pow(10.0, self.settings.target / 10.0) * em;
        let target = if es <= allowed || es <= 0.0 { 1.0 } else { math::sqrt(allowed / es) };
        let was = self.gain;
        let k = if target < was { self.fall } else { self.rise };
        self.gain = math::flush(was + (target - was) * k);
        // Glide across the block, from the last gain to this one. A frame
        // with no side is left as it came (its two sides are already the
        // mid, and `m + 0` would turn a -0 into a +0).
        let step = (self.gain - was) / QUANTUM as f64;
        for (i, ((l, r), (&m, &s))) in left.iter_mut().zip(right.iter_mut()).zip(mid.iter().zip(side.iter())).enumerate() {
            if s != 0.0 {
                let s = s * (was + step * (i + 1) as f64) as f32;
                *l = m + s;
                *r = m - s;
            }
        }
    }
}

impl Default for Width {
    fn default() -> Self {
        Self::new()
    }
}

impl Block for Width {
    type Settings = Settings;

    const PROBES: &'static [Probe<Settings>] = &[Probe { name: "ceiling", settings: REST }];

    const INPUT: bool = true;

    const STEREO: bool = true;

    const SETTINGS: &'static [Setting] = SETTINGS;

    fn start(&mut self, s: &Settings, cx: &Context) {
        self.ready(s, cx.rate);
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    /// Mono: there is no side, so nothing to do.
    fn render(&mut self, _: u64, _: &Context, _: &mut Signal) {}

    fn render_stereo(&mut self, _: u64, _: &Context, signal: &mut Stereo) {
        self.process(signal);
    }

    fn set(&mut self, setting: usize, value: f64, _: f64, _: &Context) {
        Width::set(self, setting, value);
    }
}

#[cfg(test)]
#[allow(clippy::needless_range_loop)]
mod tests {
    use super::*;

    const RATE: f64 = 48_000.0;

    /// A 1 kHz tone as the mid, and `side` of it, 90 degrees round, as the
    /// side, one block at a time.
    fn tone(block: usize, side: f32) -> Stereo {
        let mut out = [[0.0; QUANTUM]; 2];
        for i in 0..QUANTUM {
            let t = (block * QUANTUM + i) as f64 / RATE;
            let m = (0.3 * math::sin(core::f64::consts::TAU * 1000.0 * t)) as f32;
            let s = side * (0.3 * math::cos(core::f64::consts::TAU * 1000.0 * t)) as f32;
            out[0][i] = m + s;
            out[1][i] = m - s;
        }
        out
    }

    fn ratio_db(signal: &[Stereo]) -> f64 {
        let (mut m, mut s) = (0.0, 0.0);
        for b in signal {
            for (&l, &r) in b[0].iter().zip(b[1].iter()) {
                m += f64::from(f32::midpoint(l, r)).powi(2);
                s += f64::from((l - r) * 0.5).powi(2);
            }
        }
        10.0 * (s / m).log10()
    }

    #[test]
    fn centered_passes_bit_for_bit() {
        let mut w = Width::new();
        w.ready(&REST, RATE);
        for b in 0..800 {
            let mut input = tone(b, 0.0);
            input[0][3] = -0.0;
            input[1][3] = -0.0;
            let mut out = input;
            w.process(&mut out);
            let bits = |x: &Stereo| x.map(|side| side.map(f32::to_bits));
            assert_eq!(bits(&out), bits(&input), "block {b}");
        }
    }

    #[test]
    fn a_wide_signal_comes_down_to_its_target_and_the_mid_stays() {
        let mut w = Width::new();
        w.ready(&Settings { target: -12.0, ..REST }, RATE);
        // Side 0 dB against the mid: 12 dB too wide.
        let mut tail = Vec::new();
        for b in 0..(8.0 * RATE / QUANTUM as f64) as usize {
            let input = tone(b, 1.0);
            let mut out = input;
            w.process(&mut out);
            for i in 0..QUANTUM {
                let m_in = f32::midpoint(input[0][i], input[1][i]);
                let m_out = f32::midpoint(out[0][i], out[1][i]);
                assert!((m_in - m_out).abs() <= 1e-7, "the mid moved at block {b}");
            }
            if b as f64 * QUANTUM as f64 / RATE > 6.0 {
                tail.push(out);
            }
        }
        let r = ratio_db(&tail);
        assert!((r + 12.0).abs() < 0.3, "{r}");
    }

    #[test]
    fn under_the_target_nothing_moves() {
        let mut w = Width::new();
        w.ready(&REST, RATE);
        // Side -14 dB: under -7.
        for b in 0..1000 {
            let mut out = tone(b, 0.2);
            w.process(&mut out);
        }
        assert!((w.gain() - 1.0).abs() < 1e-12, "{}", w.gain());
    }
}
