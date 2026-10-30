//! Pitch and scale maths: `js/theory.js`. MIDI note numbers throughout, as
//! doubles, as the JavaScript keeps them.

use super::js;
use crate::math;

/// A mode: its name, its steps in semitones from the root, and where it
/// sits on the character axis (`SCALE_BRIGHTNESS`; 0.5 for none).
#[derive(Clone, Copy, Debug)]
pub struct Scale {
    pub id: &'static str,
    pub steps: &'static [f64],
    pub bright: f64,
}

/// `SCALES`, with `SCALE_BRIGHTNESS`.
pub static SCALES: [Scale; 19] = [
    Scale { id: "dorian", steps: &[0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0], bright: 0.46 },
    Scale { id: "aeolian", steps: &[0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0], bright: 0.24 },
    Scale { id: "ionian", steps: &[0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 11.0], bright: 0.92 },
    Scale { id: "mixolydian", steps: &[0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 10.0], bright: 0.75 },
    Scale { id: "lydian", steps: &[0.0, 2.0, 4.0, 6.0, 7.0, 9.0, 11.0], bright: 1.0 },
    Scale { id: "phrygian", steps: &[0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 10.0], bright: 0.08 },
    Scale { id: "harmonicMinor", steps: &[0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 11.0], bright: 0.14 },
    Scale { id: "minorPent", steps: &[0.0, 3.0, 5.0, 7.0, 10.0], bright: 0.34 },
    Scale { id: "majorPent", steps: &[0.0, 2.0, 4.0, 7.0, 9.0], bright: 0.9 },
    Scale { id: "kumoi", steps: &[0.0, 2.0, 3.0, 7.0, 9.0], bright: 0.52 },
    Scale { id: "hirajoshi", steps: &[0.0, 2.0, 3.0, 7.0, 8.0], bright: 0.28 },
    Scale { id: "insen", steps: &[0.0, 1.0, 5.0, 7.0, 10.0], bright: 0.3 },
    Scale { id: "wholeTone", steps: &[0.0, 2.0, 4.0, 6.0, 8.0, 10.0], bright: 0.44 },
    Scale { id: "lydianDominant", steps: &[0.0, 2.0, 4.0, 6.0, 7.0, 9.0, 10.0], bright: 0.78 },
    Scale { id: "phrygianDominant", steps: &[0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 10.0], bright: 0.2 },
    Scale { id: "dorianSharp4", steps: &[0.0, 2.0, 3.0, 6.0, 7.0, 9.0, 10.0], bright: 0.4 },
    Scale { id: "yo", steps: &[0.0, 2.0, 5.0, 7.0, 9.0], bright: 0.82 },
    Scale { id: "ritusen", steps: &[0.0, 2.0, 5.0, 7.0, 10.0], bright: 0.68 },
    Scale { id: "akebono", steps: &[0.0, 2.0, 3.0, 7.0, 9.0], bright: 0.48 },
];

/// A mode by name.
#[must_use]
pub fn scale(id: &str) -> Option<&'static Scale> {
    SCALES.iter().find(|s| s.id == id)
}

/// `scalePitch`: a degree of the mode (wrapping into octaves either way) as
/// a MIDI note.
#[must_use]
pub fn scale_pitch(root: f64, steps: &[f64], degree: f64, octave: f64) -> f64 {
    let n = js::num(steps.len());
    let oct = js::floor(degree / n) + octave;
    let idx = ((degree % n) + n) % n;
    root + steps[js::index(idx).min(steps.len() - 1)] + 12.0 * oct
}

/// `GLAD_FROM`: the lift above which the bright modes are leaned toward.
pub const GLAD_FROM: f64 = 0.66;

/// `moodWeighted` for one mode: its weight `w`, its brightness, the lift.
#[must_use]
pub fn mood_weighted(w: f64, bright: f64, mood: f64) -> f64 {
    let glad = js::max(0.0, mood - GLAD_FROM);
    let inward = js::max(0.0, 0.5 - mood);
    let near = 0.2 + 2.1 * (1.0 - js::abs(bright - mood));
    let tilt = math::exp(20.0 * glad * (bright - 0.5) - 20.0 * inward * math::pow((bright - 0.4) / 0.3, 2.0));
    w * near * tilt
}

/// A mode's brightness, by name (0.5 for one the table lacks).
#[must_use]
pub fn brightness(id: &str) -> f64 {
    scale(id).map_or(0.5, |s| s.bright)
}
