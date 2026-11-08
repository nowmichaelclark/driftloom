//! How long the track plays (queue item 94a): the app's length slider, as
//! `Engine.endStep` and `_ends` in `js/engine.js` count it (queue item 88b),
//! so the core knows when a track ends with the page asleep.
//!
//! A track is a number of whole passes (a pass of a piece is all of its
//! sections: a round), or a number of bars shorter than a pass, ending on
//! the first phrase line that many bars on. Either counts from the step the
//! listener hears when it is set (the pass playing then as the first), or
//! from a new loop's first step. A loop's own length never ends it, and
//! Endless never ends by passes.

use crate::generator::js;
use crate::generator::piece::Piece;
use crate::generator::spec::Spec;

use super::change::PHRASE_BARS;

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
}

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

impl Track {
    /// The step from the load the track ends at, counted from `from`
    /// (`endStep`), or none while it plays on.
    #[must_use]
    pub fn end_step(self, spec: &Spec, from: u64) -> Option<u64> {
        match self {
            Track::Always => None,
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
    }
}
