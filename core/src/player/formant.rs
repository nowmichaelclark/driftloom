//! The sung voices' trims (`_formantTrim`, `formantEnergy` and
//! `biquadPower` in `js/synth.js`), as the player works them out (queue
//! item 82). A sung note whose fundamental sits on a formant's peak comes
//! out much louder than its neighbors, so each note is trimmed by how much
//! energy the vocal tract hands its harmonics against the voice's usual
//! note on that vowel: the median over MIDI 43-91. Worked out from the
//! filters' own response (the RBJ biquads Web Audio uses) at each harmonic,
//! K-weighted roughly as the loudness meter weighs them; no draws, and on
//! the core's math (`crate::math`, which `js/dlmath.js` mirrors), so the
//! page and the player trim each note alike to the bit.

use core::f64::consts::PI;

use crate::blocks::midi_to_freq;
use crate::generator::js;
use crate::math;

/// A formant: its center in Hz, its bandwidth in Hz, its boost in dB.
pub type Formant = [f64; 3];
/// A vowel's three formants.
pub type Vowel = [Formant; 3];

/// The vowels a sung voice sings (`VOWELS`), in the core's order
/// (`SUNG_VOWELS`: a, e, o, u).
pub const VOWELS: [Vowel; 4] = [
    [[800.0, 80.0, 16.0], [1150.0, 90.0, 13.0], [2900.0, 130.0, 9.0]],
    [[400.0, 60.0, 16.0], [1600.0, 80.0, 13.0], [2700.0, 130.0, 9.0]],
    [[450.0, 70.0, 17.0], [800.0, 80.0, 12.0], [2830.0, 120.0, 7.0]],
    [[325.0, 50.0, 17.0], [700.0, 60.0, 11.0], [2530.0, 170.0, 6.0]],
];
/// A closed mouth: one low nasal resonance, nothing up top (a hum).
pub const HUM: Vowel = [[280.0, 60.0, 18.0], [1100.0, 100.0, 8.0], [2200.0, 160.0, 3.0]];
/// The same tract relaxing, where a long hum opens to (`OPEN_HUM`).
pub const OPEN_HUM: Vowel = [[330.0, 70.0, 16.5], [1200.0, 110.0, 10.0], [2350.0, 170.0, 5.0]];
/// How much of a note's excess comes off (`FORMANT_TAME`), and the range a
/// trim stays in (`FORMANT_TRIM_RANGE`): it only ever cuts.
const TAME: f64 = 0.85;
const TRIM_LO: f64 = 0.25;
const TRIM_HI: f64 = 1.0;
/// The notes the voice's usual note is the median of.
const MEDIAN_FROM: i32 = 43;
const MEDIAN_TO: i32 = 91;
/// The sung voices' references: the four vowels', then the hum's.
pub const REFS: usize = 5;
/// The hum's place among them.
pub const HUM_REF: usize = 4;

/// `biquadPower(type, f, fc, q, gainDb, fs)`: the power response at `f` Hz
/// of a peaking filter at `fc` (`q`, `gain_db`), or (`peaking` false) of a
/// lowpass at `fc` with its Q in dB, as Web Audio takes it.
fn biquad_power(peaking: bool, f: f64, fc: f64, q: f64, gain_db: f64, fs: f64) -> f64 {
    let w0 = (2.0 * PI * fc) / fs;
    let cw = math::cos(w0);
    let (b0, b1, b2, a0, a1, a2);
    if peaking {
        let a = math::pow(10.0, gain_db / 40.0);
        let alpha = math::sin(w0) / (2.0 * q);
        b0 = 1.0 + alpha * a;
        b1 = -2.0 * cw;
        b2 = 1.0 - alpha * a;
        a0 = 1.0 + alpha / a;
        a1 = -2.0 * cw;
        a2 = 1.0 - alpha / a;
    } else {
        let alpha = math::sin(w0) / (2.0 * math::pow(10.0, q / 20.0));
        b0 = (1.0 - cw) / 2.0;
        b1 = 1.0 - cw;
        b2 = (1.0 - cw) / 2.0;
        a0 = 1.0 + alpha;
        a1 = -2.0 * cw;
        a2 = 1.0 - alpha;
    }
    let w = (2.0 * PI * f) / fs;
    let (c1, s1) = (math::cos(w), math::sin(w));
    let (c2, s2) = (math::cos(2.0 * w), math::sin(2.0 * w));
    let nr = b0 + b1 * c1 + b2 * c2;
    let ni = -(b1 * s1 + b2 * s2);
    let dr = a0 + a1 * c1 + a2 * c2;
    let di = -(a1 * s1 + a2 * s2);
    (nr * nr + ni * ni) / (dr * dr + di * di)
}

/// `formantEnergy(f0, formants, corner, humming, fs)`: the energy a note of
/// fundamental `f0` comes out with, every harmonic below 6 kHz through the
/// formants and the lowpass at `corner`, with the meter's high shelf. A
/// hum's source is a triangle (odd harmonics, 1/n^2), the others' the
/// glottal wave (every harmonic, 1/n^2.5).
#[must_use]
pub fn energy(f0: f64, formants: &Vowel, corner: f64, humming: bool, fs: f64) -> f64 {
    let mut e = 0.0;
    let mut h = 1.0;
    while h * f0 < 6000.0 {
        if humming && h % 2.0 == 0.0 {
            h += 1.0;
            continue;
        }
        let amp = if humming { 1.0 / (h * h) } else { 1.0 / math::pow(h, 2.5) };
        let f = h * f0;
        let mut p = amp * amp * biquad_power(false, f, corner, 0.7, 0.0, fs);
        for &[hz, bw, gain_db] in formants {
            p *= biquad_power(true, f, hz, hz / bw, gain_db, fs);
        }
        let r = f / 1681.0;
        let x = r * r;
        e += p * (1.0 + 2.51 * x) / (1.0 + x);
        h += 1.0;
    }
    e
}

/// The references the trims are against, at one sample rate: each vowel's
/// and the hum's median energy over MIDI 43-91 (`_formantTrim`'s `ref`).
/// Worked out once, as the player starts (about a millisecond, so never on
/// an audio block).
#[derive(Clone, Copy)]
pub struct Refs {
    refs: [f64; REFS],
    rate: f64,
}

impl Refs {
    /// None yet.
    #[must_use]
    pub const fn new() -> Self {
        Refs { refs: [0.0; REFS], rate: 0.0 }
    }

    /// Work them out for `rate`, unless they already are.
    pub fn at(&mut self, rate: f64) {
        if self.rate == rate {
            return;
        }
        for (i, r) in self.refs.iter_mut().enumerate() {
            let (formants, corner, humming) = if i == HUM_REF { (&HUM, 1700.0, true) } else { (&VOWELS[i], 2600.0, false) };
            let mut all = [0.0; (MEDIAN_TO - MEDIAN_FROM + 1) as usize];
            for (m, e) in (MEDIAN_FROM..=MEDIAN_TO).zip(all.iter_mut()) {
                *e = energy(midi_to_freq(f64::from(m)), formants, corner, humming, rate);
            }
            all.sort_unstable_by(f64::total_cmp);
            *r = all[all.len() / 2];
        }
        self.rate = rate;
    }

    /// `_formantTrim(key, ...)`: the trim of a note at `midi` on reference
    /// `which` (a vowel's number, or `HUM_REF`), through `formants`.
    #[must_use]
    pub fn trim(&self, which: usize, formants: &Vowel, corner: f64, humming: bool, midi: f64) -> f64 {
        let e = energy(midi_to_freq(midi), formants, corner, humming, self.rate);
        let r = self.refs.get(which).copied().unwrap_or(0.0);
        js::min(TRIM_HI, js::max(TRIM_LO, math::pow(r / e, TAME / 2.0)))
    }

    /// A moving vowel's trim (`voice`, the sung voices): `trim` for where it
    /// starts, and for where it lands (`landed`, `depth` of the way to it)
    /// against the same median, half each.
    #[must_use]
    pub fn moving(&self, trim: f64, f: f64, formants: &Vowel, target: &Vowel, depth: f64, corner: f64, humming: bool) -> f64 {
        let mut landed = *formants;
        for (l, (&[hz, bw, g], &[thz, tbw, tg])) in landed.iter_mut().zip(formants.iter().zip(target.iter())) {
            *l = [hz + (thz - hz) * depth, bw + (tbw - bw) * depth, g + (tg - g) * depth];
        }
        let e0 = energy(f, formants, corner, humming, self.rate);
        let e1 = energy(f, &landed, corner, humming, self.rate);
        (trim + trim * js::min(TRIM_HI, js::max(TRIM_LO, math::pow(e0 / e1, TAME / 2.0)))) / 2.0
    }
}

impl Default for Refs {
    fn default() -> Self {
        Self::new()
    }
}
