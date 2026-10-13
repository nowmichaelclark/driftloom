//! What a note leaves to chance, and how the core settles it.
//!
//! A voice's code is written as the synth's always was, taking a few small
//! numbers it calls its draws: where in the noise a strike starts, how far
//! a bell partial is out of tune, how late the second reed comes in. Until
//! queue item 44 the page made those numbers with `Math.random` and sent
//! them with every note. Now the host sends only where the note sits in its
//! loop (`rng::Key`), and `settle` fills them in here, from the seeded
//! generator, before the note is queued. A draw is a function of the key
//! and the kind of draw (`rng::salt`), so a note makes the same draws
//! whenever and wherever it is played.
//!
//! Every draw settled here keeps the range the JavaScript voice drew it in,
//! uniform, so what the ear heard before is what it hears now, only fixed to
//! the loop. `every_moved_draw_keeps_its_range` below lists them all and
//! checks each one's range, mean and spread against the old formula.

use crate::breath::VIBRATO_MIN_DUR;
use crate::rng::{Draws, salt};
use crate::voice::midi_to_freq;
use crate::{
    ACCORDION, BASS, DRUM, EXTRA, FIDDLE, FLUTE, HAT, KNOCK, OCARINA, PANFLUTE, TEMPLEBELL,
    TEXTURE, TUBULAR, WHISTLE, bass, drums, math, texture,
};

/// A wind note this short or shorter than this never slides, so a staccato
/// line stays clean (`SLIDE_MIN_DUR`); the share of the longer ones that do
/// (`SLIDE_CHANCE`); the widest step it slides across, in semitones, before
/// it scoops from just under instead (`SLIDE_MAX_STEP`); and how far
/// under, in semitones (`SLIDE_SCOOP`).
const SLIDE_MIN_DUR: f64 = 0.18;
const SLIDE_CHANCE: f64 = 0.22;
const SLIDE_MAX_STEP: f64 = 5.0;
const SLIDE_SCOOP: f64 = 1.5;

/// Whether a wind note slides into its pitch, and if so where from in Hz
/// and how long it takes (`_slideOf`). `prev` is the note it is joined to,
/// NaN if none: a joined note within `SLIDE_MAX_STEP` semitones slides from
/// it, any other from just under itself. `chance` is the note's draw.
#[must_use]
pub fn slide_of(midi: f64, dur: f64, prev: f64, chance: f64) -> Option<(f64, f64)> {
    if dur < SLIDE_MIN_DUR || chance >= SLIDE_CHANCE {
        return None;
    }
    let step = midi - prev;
    let from = if prev.is_finite() && step != 0.0 && step.abs() <= SLIDE_MAX_STEP {
        midi_to_freq(prev)
    } else {
        midi_to_freq(midi) * math::pow(2.0, -SLIDE_SCOOP / 12.0)
    };
    Some((from, (dur * 0.22).clamp(0.03, 0.09)))
}

/// Settle the draws of a note of `voice`: overwrite, in `x` (its extra
/// values), the places the voice reads its draws from. The host's own
/// values (the note a fiddle is joined to, a texture's band) are left as
/// they came.
pub fn settle(voice: u32, parts: u32, midi: f64, dur: f64, d: &Draws, x: &mut [f64; EXTRA]) {
    let u = |salt: u32, i: u32| d.unit(salt, i);
    match voice {
        FIDDLE => {
            if dur >= VIBRATO_MIN_DUR {
                let rate = 4.8 + u(salt::FIDDLE_RATE, 0) * 1.3;
                x[1] = rate;
                x[2] = rate * (0.9 + u(salt::FIDDLE_END, 0) * 0.2);
                x[3] = 10.0 + u(salt::FIDDLE_DEPTH, 0) * 10.0;
            } else {
                x[1] = f64::NAN;
                x[2] = f64::NAN;
                x[3] = f64::NAN;
            }
        }
        OCARINA | FLUTE => {
            let slide = slide_of(midi, dur, x[0], u(salt::SLIDE, 0));
            x[0] = slide.map_or(f64::NAN, |s| s.0);
            x[1] = slide.map_or(f64::NAN, |s| s.1);
            x[2] = u(salt::BREATH_RATE, 0);
        }
        WHISTLE => {
            let slide = slide_of(midi, dur, x[0], u(salt::SLIDE, 0));
            x[0] = slide.map_or(f64::NAN, |s| s.0);
            x[1] = slide.map_or(f64::NAN, |s| s.1);
        }
        ACCORDION => x[1] = u(salt::REED_LATE, 0),
        PANFLUTE => {
            x[0] = if dur >= VIBRATO_MIN_DUR { u(salt::PAN_VIBRATO, 0) } else { f64::NAN };
            x[1] = u(salt::BREATH_RATE, 0);
        }
        TUBULAR => {
            for (i, v) in (0_u32..).zip(x.iter_mut().take(5)) {
                *v = u(salt::TUBULAR_DETUNE, i);
            }
        }
        TEMPLEBELL => {
            for (i, v) in (0_u32..).zip(x.iter_mut().take(8)) {
                *v = u(salt::TEMPLE_DETUNE, i);
            }
            x[8] = u(salt::GRAIN, 0);
            x[9] = u(salt::GRAIN, 1);
        }
        KNOCK => {
            x[0] = u(salt::GRAIN, 0);
            x[1] = u(salt::GRAIN, 1);
            x[2] = u(salt::GRAIN, 2);
        }
        HAT => {
            x[0] = u(salt::GRAIN, 0);
            x[1] = u(salt::GRAIN, 1);
        }
        BASS => {
            if parts & bass::KIND == bass::PLUCKBASS {
                x[0] = u(salt::GRAIN, 0);
                x[1] = u(salt::GRAIN, 1);
            }
        }
        DRUM => {
            if parts == drums::JINGLE || parts == drums::OPEN_JINGLE {
                // A tambourine's five zils come first, then its noise.
                for (i, v) in (0_u32..).zip(x.iter_mut().take(5)) {
                    *v = u(salt::ZIL, i);
                }
                x[5] = u(salt::GRAIN, 0);
                x[6] = u(salt::GRAIN, 1);
            } else if drums::uses_noise(parts) {
                x[0] = u(salt::GRAIN, 0);
                x[1] = u(salt::GRAIN, 1);
            }
        }
        TEXTURE => match parts {
            texture::DROP => {
                x[0] = u(salt::GRAIN, 0);
                x[1] = u(salt::GRAIN, 1);
                x[2] = u(salt::GRAIN, 2);
            }
            texture::WIND => x[0] = u(salt::TEXTURE_RATE, 0),
            texture::WAVES => {
                x[0] = u(salt::GRAIN, 0);
                x[1] = u(salt::GRAIN, 1);
            }
            _ => {}
        },
        _ => {}
    }
}

// Tests count with casts of values they have just bounded.
#[cfg(test)]
#[allow(clippy::cast_precision_loss)]
mod tests {
    use super::*;
    use crate::rng::Key;
    use crate::{CHORDS, MELODY};

    const N: u32 = 20_000;
    const NO: [f64; EXTRA] = [f64::NAN; EXTRA];

    fn settled(voice: u32, parts: u32, midi: f64, dur: f64, channel: u32, n: u32) -> [f64; EXTRA] {
        let key = Key { seed: n.wrapping_mul(2_654_435_761), pass: n % 5, step: n % 64, ord: n % 9 };
        let mut x = NO;
        settle(voice, parts, midi, dur, &Draws::new(key, channel), &mut x);
        x
    }

    /// Each value `pick` reads from a voice's settled extras is uniform over
    /// `[lo, hi)`, the range the JavaScript voice drew it in: it reaches
    /// both ends, its mean is the middle and its spread a twelfth of the
    /// range squared.
    fn uniform(voice: u32, parts: u32, dur: f64, pick: impl Fn(&[f64; EXTRA]) -> f64, lo: f64, hi: f64, what: &str) {
        let (mut sum, mut sq, mut min, mut max) = (0.0, 0.0, f64::MAX, f64::MIN);
        for n in 0..N {
            let v = pick(&settled(voice, parts, 64.0, dur, MELODY, n));
            assert!(v.is_finite(), "{what}: {v}");
            sum += v;
            sq += v * v;
            min = min.min(v);
            max = max.max(v);
        }
        let n = f64::from(N);
        let (mean, sd) = (sum / n, (sq / n - (sum / n) * (sum / n)).sqrt());
        let span = hi - lo;
        assert!(min >= lo && max < hi, "{what}: range {min} to {max}, not {lo} to {hi}");
        assert!(min < lo + span * 0.002 && max > hi - span * 0.002, "{what}: {min} to {max} leaves the ends of {lo} to {hi}");
        assert!((mean - f64::midpoint(lo, hi)).abs() < span * 0.01, "{what}: mean {mean}");
        assert!((sd - span / 12.0_f64.sqrt()).abs() < span * 0.01, "{what}: spread {sd}");
    }

    /// Every draw moved from JavaScript into the core, with the range it
    /// keeps (queue item 44's list; each is `Math.random() * a + b` in the
    /// voice it came from).
    #[test]
    fn every_moved_draw_keeps_its_range() {
        use crate::{ACCORDION, FLUTE, HAT, KNOCK, OCARINA, PANFLUTE, TEMPLEBELL, TUBULAR};
        // The fiddle's vibrato: rate, where it ends (a share of the rate),
        // depth.
        uniform(FIDDLE, 0, 1.0, |x| x[1], 4.8, 6.1, "fiddle vibrato rate");
        uniform(FIDDLE, 0, 1.0, |x| x[2] / x[1], 0.9, 1.1, "fiddle vibrato end");
        uniform(FIDDLE, 0, 1.0, |x| x[3], 10.0, 20.0, "fiddle vibrato depth");
        // The breath of the winds and the pan flute.
        uniform(OCARINA, 0, 1.0, |x| x[2], 0.0, 1.0, "ocarina breath");
        uniform(FLUTE, 0, 1.0, |x| x[2], 0.0, 1.0, "flute breath");
        uniform(PANFLUTE, 0, 1.0, |x| x[0], 0.0, 1.0, "pan flute vibrato");
        uniform(PANFLUTE, 0, 1.0, |x| x[1], 0.0, 1.0, "pan flute breath");
        uniform(ACCORDION, 0, 1.0, |x| x[1], 0.0, 1.0, "accordion second reed");
        for i in 0..5 {
            uniform(TUBULAR, 0, 1.0, |x| x[i], 0.0, 1.0, "tubular partial");
        }
        for i in 0..10 {
            uniform(TEMPLEBELL, 0, 1.0, |x| x[i], 0.0, 1.0, "temple bell");
        }
        for i in 0..3 {
            uniform(KNOCK, 0, 0.5, |x| x[i], 0.0, 1.0, "knock");
        }
        for i in 0..2 {
            uniform(HAT, 0, 0.1, |x| x[i], 0.0, 1.0, "hat grain");
            uniform(BASS, bass::PLUCKBASS, 0.5, |x| x[i], 0.0, 1.0, "pluck bass click");
            uniform(DRUM, drums::SNARE, 0.1, |x| x[i], 0.0, 1.0, "drum grain");
            uniform(DRUM, drums::JINGLE, 0.1, |x| x[5 + i], 0.0, 1.0, "jingle grain");
            uniform(TEXTURE, texture::WAVES, 2.0, |x| x[i], 0.0, 1.0, "waves");
        }
        for i in 0..5 {
            uniform(DRUM, drums::OPEN_JINGLE, 0.1, |x| x[i], 0.0, 1.0, "zil");
        }
        for i in 0..3 {
            uniform(TEXTURE, texture::DROP, 0.5, |x| x[i], 0.0, 1.0, "drop");
        }
        uniform(TEXTURE, texture::WIND, 2.0, |x| x[0], 0.0, 1.0, "wind");
    }

    /// The winds slide on 22% of the notes long enough, and never on a short
    /// one; the slide's reach and where it comes from follow the old rules.
    #[test]
    fn winds_slide_as_often_as_they_did() {
        for voice in [OCARINA, FLUTE, WHISTLE] {
            let slid = (0..N)
                .filter(|&n| settled(voice, 0, 64.0, 0.5, MELODY, n)[0].is_finite())
                .count();
            let share = slid as f64 / f64::from(N);
            assert!((share - SLIDE_CHANCE).abs() < 0.01, "voice {voice}: {share}");
            assert!((0..N).all(|n| settled(voice, 0, 64.0, 0.17, MELODY, n)[0].is_nan()));
        }
    }

    /// The pan flute and fiddle take no vibrato under half a second.
    #[test]
    fn short_notes_take_no_vibrato() {
        assert!(settled(FIDDLE, 0, 64.0, 0.49, MELODY, 1)[1].is_nan());
        assert!(settled(PANFLUTE, 0, 64.0, 0.49, MELODY, 1)[0].is_nan());
        assert!(settled(FIDDLE, 0, 64.0, 0.5, MELODY, 1)[1].is_finite());
        assert!(settled(PANFLUTE, 0, 64.0, 0.5, MELODY, 1)[0].is_finite());
    }

    /// The host's own values are left alone: the note a fiddle is joined to,
    /// a texture's band.
    #[test]
    fn the_hosts_values_stay() {
        let mut x = NO;
        x[0] = 62.0;
        settle(FIDDLE, 0, 64.0, 1.0, &Draws::new(Key::default(), CHORDS), &mut x);
        assert!((x[0] - 62.0).abs() < f64::EPSILON);
        x = NO;
        x[1] = 800.0;
        settle(TEXTURE, texture::WIND, 0.0, 4.0, &Draws::new(Key::default(), CHORDS), &mut x);
        assert!((x[1] - 800.0).abs() < f64::EPSILON);
    }
}
