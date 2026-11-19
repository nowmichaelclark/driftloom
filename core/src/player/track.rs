//! How long the track plays (queue item 94a): the app's track length (its
//! Track length view since session 30c; its length slider before), as
//! `Engine.endStep` and `_ends` in `js/engine.js` count it (queue item 88b),
//! so the core knows when a track ends with the page asleep.
//!
//! A track is a number of whole passes (a pass of a piece is all of its
//! sections: a round), or a number of bars shorter than a pass, ending on
//! the first phrase line that many bars on. Either counts from the step the
//! listener hears when it is set (the pass playing then as the first), or
//! from a new loop's first step. A loop's own length never ends it, and
//! Endless never ends by passes.
//!
//! Or a time, to the second (session 30c, queue item 111; Mikey,
//! 2026-11-18: "Should be able to set track length to an exact second"):
//! counted on the core's clock, in the output's samples, from when it is
//! set (or from a new loop's first step), so a tempo change doesn't move
//! it, and the music is done on that second. A time's track may fade in
//! from silence at its start and fade out to silence on its last second
//! (`Fades`; `FADE_IN`, `FADE_OUT`); without the fade out it stops on the
//! second, what is sounding left to ring, as an export's ending does.

use crate::generator::js;
use crate::generator::piece::Piece;
use crate::generator::spec::Spec;

use super::change::PHRASE_BARS;

pub use crate::fades::{Fades, KIND_FADE_IN, KIND_FADE_OUT, TRACK_MOST};

/// How long the track plays.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Track {
    /// On and on: the loop never ends (the app's slider at its start).
    /// First, so that a new player's is all zero bytes.
    Always,
    /// This many whole passes (`spec.playFor`), at least 1.
    Passes(u32),
    /// This many bars, ended on the next phrase line (`spec.playBars`), at
    /// least 1.
    Bars(u32),
    /// This many seconds, 1 to `TRACK_MOST` (session 30c), on the core's
    /// clock: the player knows the second it ends on (`Player::track_at`),
    /// and none of the steps' functions here end it.
    Seconds(u32),
}

/// How long a track's fade in takes from silence, in seconds (session 30c;
/// for Mikey's ears): long enough to be heard as a rise and not a cut,
/// short enough that the loop is there by its first bar or two.
pub const FADE_IN: f64 = 3.0;

/// How long a time's fade out takes to reach silence on its last second,
/// in seconds (session 30c; for Mikey's ears): a gentle ending, the notes
/// and the mix both falling, so the last few seconds are already near
/// silence.
pub const FADE_OUT: f64 = 8.0;

/// The most of a track either fade takes: a third, so a short track still
/// plays at its full level between them.
pub const FADE_SHARE: f64 = 1.0 / 3.0;

/// The steps a pass of the whole loop takes (`roundSteps`): a piece's every
/// section, none for Endless, or the loop's own.
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // whole steps, far below 2^53
pub fn round_steps(spec: &Spec) -> Option<u64> {
    let spb = spec.steps_per_bar;
    match Piece::of(spec) {
        Some(p) => p.length().map(|bars| (bars * spb) as u64),
        None => Some((spec.bars * spb) as u64),
    }
    .filter(|&n| n > 0)
}

/// The first phrase line at or after step `at` from the load
/// (`phraseLine`): the next line of `PHRASE_BARS` bars from its section's
/// start (two bars where four don't divide a section, else the section's
/// end), or the section's end where a round's last is cut short first.
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)] // whole steps, far below 2^53
pub fn phrase_line(spec: &Spec, at: u64) -> u64 {
    let spb = spec.steps_per_bar;
    let section = spec.bars * spb;
    if !(section >= 1.0) {
        return at;
    }
    let unit = if section % (PHRASE_BARS * spb) == 0.0 {
        PHRASE_BARS * spb
    } else if section % (2.0 * spb) == 0.0 {
        2.0 * spb
    } else {
        section
    };
    let at = at as f64;
    let start = round_steps(spec).map_or(0.0, |r| at - at % r as f64);
    let k = js::floor((at - start) / section);
    let into = at - start - k * section;
    let piece = Piece::of(spec);
    let length = piece.map_or(section, |p| p.bars_of(k as u64) * spb);
    (start + k * section + js::min(js::ceil(into / unit) * unit, length)) as u64
}

/// The phrase line at or before step `at` from the load (session 17: where
/// a jump lands), on `phrase_line`'s grid: the line `phrase_line` gives
/// where `at` is on one, else the one before it.
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)] // whole steps, far below 2^53
pub fn phrase_before(spec: &Spec, at: u64) -> u64 {
    let spb = spec.steps_per_bar;
    let section = spec.bars * spb;
    if !(section >= 1.0) {
        return at;
    }
    let unit = if section % (PHRASE_BARS * spb) == 0.0 {
        PHRASE_BARS * spb
    } else if section % (2.0 * spb) == 0.0 {
        2.0 * spb
    } else {
        section
    };
    let at = at as f64;
    let start = round_steps(spec).map_or(0.0, |r| at - at % r as f64);
    let k = js::floor((at - start) / section);
    let into = at - start - k * section;
    (start + k * section + js::floor(into / unit) * unit) as u64
}

impl Track {
    /// The track a host's two numbers say (`dl_play_track`, and the native
    /// app's and the C host's same command): `kind` 0 on and on, 1 whole
    /// passes, 2 bars to a phrase line (`count` at least 1, as given), 3 a
    /// time of `count` seconds (1 to `TRACK_MOST`); and its fades, a bit
    /// each beside the kind (`KIND_FADE_IN`, `KIND_FADE_OUT`, the fade out a
    /// time's only). One place reads them, so every host takes the same.
    /// None for anything else.
    #[must_use]
    pub fn of(kind: u32, count: u32) -> Option<(Track, Fades)> {
        if kind >= 16 {
            return None;
        }
        let fades = Fades { fade_in: kind & KIND_FADE_IN != 0, fade_out: kind & KIND_FADE_OUT != 0 };
        let track = match kind & 3 {
            0 => Track::Always,
            1 => Track::Passes(count.max(1)),
            2 => Track::Bars(count.max(1)),
            _ if (1..=TRACK_MOST).contains(&count) => Track::Seconds(count),
            _ => return None,
        };
        if fades.fade_out && !matches!(track, Track::Seconds(_)) {
            return None;
        }
        Some((track, fades))
    }

    /// How long its fade in and its fade out take, in seconds, with `fades`
    /// (0 for one that is off): `FADE_IN` and `FADE_OUT`, each at most a
    /// third of a time.
    #[must_use]
    pub fn fade_lengths(self, fades: Fades) -> (f64, f64) {
        let most = match self {
            Track::Seconds(n) => f64::from(n) * FADE_SHARE,
            _ => f64::INFINITY,
        };
        let fade_in = if fades.fade_in { FADE_IN.min(most) } else { 0.0 };
        let fade_out = if fades.fade_out && matches!(self, Track::Seconds(_)) { FADE_OUT.min(most) } else { 0.0 };
        (fade_in, fade_out)
    }

    /// The step from the load the track ends at, counted from `from`
    /// (`endStep`), or none while it plays on (or for a time: the player
    /// counts it, `Player::track_end`).
    #[must_use]
    pub fn end_step(self, spec: &Spec, from: u64) -> Option<u64> {
        match self {
            Track::Always | Track::Seconds(_) => None,
            Track::Passes(n) => {
                let round = round_steps(spec)?;
                Some((from / round + u64::from(n.max(1))) * round)
            }
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // whole steps
            Track::Bars(n) => Some(phrase_line(spec, from + u64::from(n.max(1)) * spec.steps_per_bar as u64)),
        }
    }

    /// Whether the track ends at step `at` (`_ends`): at or past its end, on
    /// a pass's line (whole passes) or a phrase line (bars). One set to a
    /// time already passed ends at the next such line.
    #[must_use]
    pub fn ends_at(self, spec: &Spec, from: u64, at: u64) -> bool {
        let Some(end) = self.end_step(spec, from) else { return false };
        if at < end {
            return false;
        }
        match self {
            Track::Passes(_) => round_steps(spec).is_some_and(|r| at.is_multiple_of(r)),
            _ => phrase_line(spec, at) == at,
        }
    }

    /// The first step at or after `at` the track ends on (none while it plays
    /// on).
    #[must_use]
    pub fn next_end(self, spec: &Spec, from: u64, at: u64) -> Option<u64> {
        let end = self.end_step(spec, from)?;
        let at = at.max(end);
        Some(match self {
            Track::Passes(_) => {
                let r = round_steps(spec)?;
                at.div_ceil(r) * r
            }
            _ => phrase_line(spec, at),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::request::Length;

    fn spec(bars: f64, length: Option<Length>) -> Spec {
        let mut s = Spec::EMPTY;
        s.bars = bars;
        s.steps_per_bar = 16.0;
        s.length = length;
        s
    }

    #[test]
    fn a_track_ends_where_the_app_ends_it() {
        // Four bars: a pass of 64 steps, the phrase the pass.
        let s = spec(4.0, None);
        assert_eq!(Track::Always.end_step(&s, 0), None);
        assert_eq!(Track::Passes(2).end_step(&s, 0), Some(128));
        // Counted from the pass heard: set at step 70, the pass of 64-127 is
        // the first.
        assert_eq!(Track::Passes(2).end_step(&s, 70), Some(192));
        assert!(Track::Passes(2).ends_at(&s, 70, 192) && !Track::Passes(2).ends_at(&s, 70, 200));
        assert_eq!(Track::Passes(2).next_end(&s, 0, 130), Some(192));
        // Eight bars: phrase lines every four. Three bars on from step 0
        // ends on the line at 64.
        let s = spec(8.0, None);
        assert_eq!(Track::Bars(3).end_step(&s, 0), Some(64));
        assert_eq!(Track::Bars(5).end_step(&s, 10), Some(128));
        assert_eq!(Track::Bars(1).next_end(&s, 0, 65), Some(128));
        assert!(Track::Bars(1).ends_at(&s, 0, 64) && !Track::Bars(1).ends_at(&s, 0, 70));
        // A piece of 100 bars of 16: a round of 1,600 steps; its last
        // section is 4 bars, so the line after its start is the round's end.
        let p = spec(16.0, Some(Length::Bars(100.0)));
        assert_eq!(round_steps(&p), Some(1600));
        assert_eq!(phrase_line(&p, 1536 + 1), 1600);
        assert_eq!(Track::Passes(1).end_step(&p, 5), Some(1600));
        // Endless never ends by passes, but does by bars.
        let e = spec(4.0, Some(Length::Endless));
        assert_eq!((round_steps(&e), Track::Passes(3).end_step(&e, 0)), (None, None));
        assert_eq!(Track::Bars(4).end_step(&e, 0), Some(64));
        // A time is the player's to count, never a step's here.
        assert_eq!(Track::Seconds(90).end_step(&s, 0), None);
        assert!(!Track::Seconds(1).ends_at(&s, 0, 64));
    }

    #[test]
    fn every_host_reads_a_track_the_same_way() {
        let none = Fades::NONE;
        assert_eq!(Track::of(0, 0), Some((Track::Always, none)));
        assert_eq!(Track::of(1, 0), Some((Track::Passes(1), none)));
        assert_eq!(Track::of(2, 8), Some((Track::Bars(8), none)));
        assert_eq!(Track::of(3, 5025), Some((Track::Seconds(5025), none)));
        let both = Fades { fade_in: true, fade_out: true };
        assert_eq!(Track::of(3 | KIND_FADE_IN | KIND_FADE_OUT, 5025), Some((Track::Seconds(5025), both)));
        // A fade in plays on and on as well; a fade out is a time's alone.
        assert_eq!(Track::of(KIND_FADE_IN, 0), Some((Track::Always, Fades { fade_in: true, fade_out: false })));
        assert_eq!(Track::of(KIND_FADE_OUT, 0), None);
        assert_eq!(Track::of(1 | KIND_FADE_OUT, 2), None);
        // A time of none, or past a day, and kinds there are none of.
        assert_eq!(Track::of(3, 0), None);
        assert_eq!(Track::of(3, TRACK_MOST + 1), None);
        assert_eq!(Track::of(16, 1), None);
        // The fades' lengths: each its own, at most a third of a time.
        assert_eq!(Track::Seconds(5025).fade_lengths(both), (FADE_IN, FADE_OUT));
        assert_eq!(Track::Seconds(6).fade_lengths(both), (2.0, 2.0));
        assert_eq!(Track::Always.fade_lengths(both), (FADE_IN, 0.0));
        assert_eq!(Track::Seconds(60).fade_lengths(none), (0.0, 0.0));
    }
}
