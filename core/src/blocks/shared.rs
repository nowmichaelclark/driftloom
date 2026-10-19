//! What the voices share: Web Audio's defaults, the envelope and pitch
//! helpers each voice's JavaScript twin had in common (`_release2`,
//! `speak`, `_noiseSource`, `_slideOf`), and the small constants they agree
//! on. Not a block: the parts a voice is built from are, and these are the
//! few lines of glue between them.

use crate::QUANTUM;
use crate::block::Signal;
use crate::math;
use crate::param::Param;

use super::filter::Biquad;
use super::noise_source::NoiseSource;
use super::osc::{Part, Pitch};

/// `midiToFreq` in `js/theory.js`.
#[must_use]
pub fn midi_to_freq(midi: f64) -> f64 {
    440.0 * math::pow(2.0, (midi - 69.0) / 12.0)
}

/// A `GainNode`'s gain, before anything is scheduled on it.
pub(crate) const UNITY: f32 = 1.0;
/// An `OscillatorNode`'s frequency, likewise.
pub(crate) const A440: f32 = 440.0;
/// A `BiquadFilterNode`'s frequency and Q, before anything sets them.
pub(crate) const CUTOFF_DEFAULT: f32 = 350.0;
pub(crate) const Q_DEFAULT: f32 = 1.0;
/// How long a voice with a filter is kept past its last stop, in seconds:
/// the filter rings on after its input stops, and a node keeps rendering
/// its tail. A cutoff of 200 Hz at a Q of 8 dB rings for about 30 ms;
/// this is well past that.
pub(crate) const FILTER_TAIL: f64 = 0.2;
/// Where a gain behind the noise holds before its note: its first event's
/// value (item 26b). A source can begin a frame ahead of that event, and
/// the frame it plays then goes through this, not through unity.
pub(crate) const QUIET: f32 = 0.0001;
/// A partial's level in a struck bell (`BELL_PARTIAL_VEL`).
pub(crate) const BELL_PARTIAL_VEL: f64 = 0.8;
/// Notes this long and longer get a vibrato (`VIBRATO_MIN_DUR`).
pub(crate) const VIBRATO_MIN_DUR: f64 = 0.5;
/// A slurred note is not attacked again, beyond the few ms that keep it
/// from clicking (`SLUR_ATTACK`).
pub(crate) const SLUR_ATTACK: f64 = 0.012;

/// How an oscillator whose frequency is a parameter is pitched for the
/// block starting at `block`: moving, frame by frame, while Web Audio would
/// render the parameter sample by sample, and otherwise steady at its value
/// (see `osc`). `freq` is filled when it moves. `first` is whether the
/// oscillator starts in this block: its events are clamped to the block
/// then, as Chromium does (`Param::clamp_before`) -- after deciding whether
/// it moves, as Chromium decides that first too.
// A block's first frame as seconds: well under 2^52 frames.
#[allow(clippy::cast_precision_loss)]
pub(crate) fn pitch_of<'a, const N: usize>(
    p: &mut Param<N>,
    block: u64,
    rate: f64,
    first: bool,
    freq: &'a mut Signal,
    detune: Part<'a>,
) -> Pitch<'a> {
    let from = block as f64 / rate;
    let moving = p.moving_from(from, QUANTUM as f64 / rate);
    if first {
        p.clamp_before(block, rate);
    }
    if moving {
        p.fill(block, rate, freq);
    }
    match (moving, detune) {
        (false, Part::Steady(d)) => Pitch::Steady {
            freq: p.value_at(from),
            detune: d,
        },
        (true, detune) => Pitch::Moving {
            freq: Part::Moving(freq),
            detune,
        },
        (false, detune) => Pitch::Moving {
            freq: Part::Steady(p.value_at(from)),
            detune,
        },
    }
}

/// Whether an oscillator starting at frame `start` starts in the block
/// from frame `block`.
pub(crate) fn starts_in(start: u64, block: u64) -> bool {
    start >= block && start < block + QUANTUM as u64
}

/// `_release2`: from wherever the envelope has got to, an exponential fall
/// to `floor`, then a short line to true zero at `to`.
pub(crate) fn release(g: &mut Param, from: f64, to: f64, now: f64) {
    const FLOOR: f32 = 0.0006;
    g.exponential_ramp_to_value_at_time(FLOOR, (from + 0.01).max(to - 0.025), now);
    g.linear_ramp_to_value_at_time(0.0, to, now);
}

/// `speak`: how long a note takes to arrive, by its length.
pub(crate) fn speak(dur: f64) -> f64 {
    (0.015 + dur * 0.06).clamp(0.02, 0.1)
}

/// A source's playback rate from its draw: `0.9 + Math.random() * 0.25`.
pub(crate) fn breath_rate(draw: f64) -> f64 {
    0.9 + draw * 0.25
}

/// `_noiseSource(time, dur)`: a grain of the noise at a drawn rate, from a
/// drawn place, `dur + 0.05` long; `rate_draw` and `offset_draw` are its two
/// draws, in that order.
// The noise's length in frames, as seconds: two seconds of frames.
#[allow(clippy::cast_precision_loss)]
pub(crate) fn noise_grain(
    src: &mut NoiseSource,
    time: f64,
    dur: f64,
    (rate_draw, offset_draw): (f64, f64),
    len: usize,
    rate: f64,
) {
    let duration = len as f64 / rate;
    let offset = offset_draw * (duration - dur - 0.05);
    src.grain(
        time,
        offset.max(0.0),
        dur + 0.05,
        breath_rate(rate_draw),
        len,
        rate,
    );
}

/// `x` through `filter` and `gain`, added into `out`. The filter runs on
/// every frame of the block: it rings on past its source.
pub(crate) fn through(x: &Signal, filter: &mut Biquad, gain: &Signal, out: &mut Signal) {
    for ((y, &v), &g) in out.iter_mut().zip(x.iter()).zip(gain.iter()) {
        *y += filter.step(v) * g;
    }
    filter.flush();
}

/// A wind note this short or shorter than this never slides, so a staccato
/// line stays clean (`SLIDE_MIN_DUR`); the share of the longer ones that do
/// (`SLIDE_CHANCE`); the widest step it slides across, in semitones, before
/// it scoops from just under instead (`SLIDE_MAX_STEP`); and how far
/// under, in semitones (`SLIDE_SCOOP`).
const SLIDE_MIN_DUR: f64 = 0.18;
pub(crate) const SLIDE_CHANCE: f64 = 0.22;
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

/// A note's slide, settled into its first two extra values: where from in
/// Hz and how long it takes, or NaN in both for none. The first extra value
/// comes in as the note it is joined to.
pub(crate) fn settle_slide(note: &mut crate::block::Note) {
    let chance = note.draws.unit(crate::rng::salt::SLIDE, 0);
    let slide = slide_of(note.midi, note.dur, note.extra[0], chance);
    note.extra[0] = slide.map_or(f64::NAN, |s| s.0);
    note.extra[1] = slide.map_or(f64::NAN, |s| s.1);
}

/// A finite value as `Some`, NaN (the host's "none") as `None`.
pub(crate) fn given(x: f64) -> Option<f64> {
    if x.is_finite() { Some(x) } else { None }
}

// Tests count with casts of values they have just bounded.
#[cfg(test)]
#[allow(clippy::cast_precision_loss)]
mod tests {
    use super::*;
    use crate::block::{NO_EXTRA, Note};
    use crate::blocks::{bass, drum, settle, texture};
    use crate::rng::{Draws, Key};
    use crate::{
        ACCORDION, BASS, CHORDS, DRUM, EXTRA, FIDDLE, FLUTE, HAT, KNOCK, MELODY, OCARINA, PANFLUTE,
        TEMPLEBELL, TEXTURE, TUBULAR, WHISTLE,
    };

    const N: u32 = 20_000;
    const NO: [f64; EXTRA] = NO_EXTRA;

    /// A note's extra values once the core has settled its draws, as
    /// `Core::note` does when the note arrives.
    fn settled_with(voice: u32, parts: u32, midi: f64, dur: f64, channel: u32, key: Key, extra: [f64; EXTRA]) -> [f64; EXTRA] {
        let draws = Draws::new(key, channel);
        let mut note = Note { dur, draws, ..Note::probe(voice, channel, midi, parts, extra) };
        settle(&mut note);
        note.extra
    }

    fn settled(voice: u32, parts: u32, midi: f64, dur: f64, channel: u32, n: u32) -> [f64; EXTRA] {
        let key = Key { seed: n.wrapping_mul(2_654_435_761), pass: n % 5, step: n % 64, ord: n % 9 };
        settled_with(voice, parts, midi, dur, channel, key, NO)
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
            uniform(DRUM, drum::SNARE, 0.1, |x| x[i], 0.0, 1.0, "drum grain");
            uniform(DRUM, drum::JINGLE, 0.1, |x| x[5 + i], 0.0, 1.0, "jingle grain");
            uniform(TEXTURE, texture::WAVES, 2.0, |x| x[i], 0.0, 1.0, "waves");
        }
        for i in 0..5 {
            uniform(DRUM, drum::OPEN_JINGLE, 0.1, |x| x[i], 0.0, 1.0, "zil");
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
        let x = settled_with(FIDDLE, 0, 64.0, 1.0, CHORDS, Key::default(), x);
        assert!((x[0] - 62.0).abs() < f64::EPSILON);
        let mut x = NO;
        x[1] = 800.0;
        let x = settled_with(TEXTURE, texture::WIND, 0.0, 4.0, CHORDS, Key::default(), x);
        assert!((x[1] - 800.0).abs() < f64::EPSILON);
    }
}

/// What one of a held instrument's voices does at a retune (`retune`).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Turn {
    /// Its note is in the new chord too: it sounds on, untouched.
    Keep,
    /// Its note is not: it lets go and speaks this note instead, or speaks
    /// it from silence.
    Speak(f64),
    /// No note of the new chord is left for it: it lets go, or stays
    /// silent.
    Rest,
}

/// The retune rule (queue items 46c and 61): a held instrument with `N`
/// voices holding `held` (NaN for a silent voice) moves to `chord`. A note
/// two chords share keeps sounding, untouched, on the voice that holds it;
/// each new note goes to the first voice left over, in order; a voice with
/// none lets go. Notes past the voices are left out, and a note named twice
/// sounds once. The same chords give the same turns on every device.
// Notes are matched exactly: a chord's notes are the host's numbers, sent
// again as they were.
#[allow(clippy::float_cmp)]
#[must_use]
pub fn retune<const N: usize>(held: &[f64; N], chord: &[f64]) -> [Turn; N] {
    let mut turns = [Turn::Rest; N];
    let mut placed = [false; crate::block::event::NOTES];
    let chord = chord.get(..crate::block::event::NOTES).unwrap_or(chord);
    // A note named twice is placed once, and a NaN never.
    for (i, &m) in chord.iter().enumerate() {
        if (m.is_nan() || chord.get(..i).is_some_and(|before| before.contains(&m)))
            && let Some(p) = placed.get_mut(i)
        {
            *p = true;
        }
    }
    // The notes the chords share stay where they are.
    for (turn, &h) in turns.iter_mut().zip(held.iter()) {
        if let Some(i) = chord.iter().position(|&m| m == h)
            && let Some(p) = placed.get_mut(i)
            && !*p
        {
            *p = true;
            *turn = Turn::Keep;
        }
    }
    // The new notes go to the voices left, in order.
    let mut free = turns.iter_mut().filter(|t| **t == Turn::Rest);
    for (&m, p) in chord.iter().zip(placed.iter()) {
        if !*p
            && let Some(turn) = free.next()
        {
            *turn = Turn::Speak(m);
        }
    }
    turns
}
