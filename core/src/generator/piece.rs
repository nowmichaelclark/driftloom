//! A piece of a length asked (promise 11, queue item 88): any whole number
//! of bars from 1 up, or Endless, worked a section at a time
//! (`pieceOf` in `js/generator.js`, the same way).
//!
//! **Short lengths are the loop's own.** A length of up to `SECTION_BARS`
//! bars (the longest any profile's table draws, 32) is the loop's length:
//! the spec's `bars` is the length asked, the table's draw still made and
//! overruled (so no other draw moves), honored where the tables never draw
//! it (1, 3, 7 bars). The piece is that loop, played once round.
//!
//! **Longer ones are sections.** Past `SECTION_BARS`, and for Endless, the
//! loop keeps the length its profile drew, and the piece is that loop
//! played section after section: section `n` is the loop's pass `n` as "Let
//! the loop wander" draws it (`wander::Pass`, from the loop's seed and the
//! number alone, at the wander's amount), holding everything else the
//! request asked (the vibes, the tempo, the pins, the theme). So the
//! working memory and the cost of a section are the same for 4 bars,
//! 32,768 or Endless, and any section is reached straight from the seed and
//! its number, without making the ones before it (`Generator::section`).
//! A piece whose length the loop's doesn't divide ends on a section cut
//! short at a bar line: its first bars, as many as are left (for Mikey's
//! ears). Writing a long piece with an arc (themes that return, a journey)
//! is later composing work of its own: it changes what a section is drawn
//! from, not how a piece is counted, so it needs no rework here.
//!
//! **At its end** a piece comes round, as a loop does: its sections are
//! numbered on (each round's drawn afresh, as a loop's next pass wanders),
//! and every round is exactly the length asked, its last section cut short
//! where it was. With the wander off the rounds are the same. The app plays
//! a piece once round and moves on to the next loop; a host that wants
//! another piece asks for one. Endless never comes round: its sections are
//! numbered on, the wander's pass number wrapping past 2^32 (some ten
//! thousand years of two-bar sections) as the JavaScript's does.
//!
//! A piece's section `n` is its loop's pass `n`: the player counts them
//! from the piece's start (a loop with no length asked keeps today's first
//! pass, drawn at the count the loop before it reached).

use super::data::MAX_BARS;
use super::js;
use super::request::Length;
use super::spec::Spec;

/// The longest length that is the loop's own, in bars (`SECTION_BARS`):
/// the longest any profile draws, so the generator's lists, sized for it,
/// hold any loop it makes. A longer length is played in sections.
#[allow(clippy::cast_precision_loss)] // 32
pub const SECTION_BARS: f64 = MAX_BARS as f64;

/// A loop's piece: how many bars a section takes, and how a round ends.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    /// A section's bars: the loop's own.
    pub bars: f64,
    /// How many sections a round has and the bars its last plays (all of
    /// a section's, or fewer); none for Endless.
    pub end: Option<(u64, f64)>,
}

impl Piece {
    /// The piece of the loop of `spec`: none for a loop with no length
    /// asked, which loops as it always did.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)] // whole numbers below 2^53, checked by the request
    pub fn of(spec: &Spec) -> Option<Piece> {
        let bars = spec.bars;
        if !(bars >= 1.0) {
            return None;
        }
        let end = match spec.length? {
            Length::Endless => None,
            Length::Bars(n) => {
                // In whole numbers, so a length near 2^53 counts exactly.
                let (n, b) = (n as u64, bars as u64);
                let rest = n % b;
                let sections = n / b + u64::from(rest != 0);
                Some((sections, if rest == 0 { bars } else { rest as f64 }))
            }
        };
        Some(Piece { bars, end })
    }

    /// The bars section `n` plays: a section's, or a round's last's.
    #[must_use]
    pub fn bars_of(&self, n: u64) -> f64 {
        match self.end {
            Some((sections, last)) if n % sections == sections - 1 => last,
            _ => self.bars,
        }
    }

    /// Whether section `n` begins a round (the piece's first, or its first
    /// again after a round's last).
    #[must_use]
    pub fn begins_round(&self, n: u64) -> bool {
        self.end.is_some_and(|(sections, _)| n.is_multiple_of(sections))
    }

    /// Whether some section is cut short: a round's last plays fewer bars
    /// than a section's.
    #[must_use]
    pub fn cut(&self) -> bool {
        self.end.is_some_and(|(_, last)| last < self.bars)
    }

    /// The piece's length in bars, once round; none for Endless.
    #[must_use]
    #[allow(clippy::cast_precision_loss)] // sections, far below 2^53 bars
    pub fn length(&self) -> Option<f64> {
        self.end.map(|(sections, last)| (sections - 1) as f64 * self.bars + last)
    }
}

/// The wander's pass number for section `n` (`pass >>> 0` in the
/// JavaScript): the section's number, wrapping past 2^32.
#[must_use]
#[allow(clippy::cast_possible_truncation)] // wrapping on purpose
pub fn pass_number(n: u64) -> u32 {
    n as u32
}

/// How long `bars` bars of `steps_per_bar` steps play at `bpm`, in seconds
/// (`pieceSeconds` in `js/generator.js`, the same operations in the same
/// order): what the app's Length picker shows ("16 bars, about 45-55 s"),
/// so the app and the core never disagree.
#[must_use]
pub fn seconds(bars: f64, steps_per_bar: f64, bpm: f64) -> f64 {
    bars * steps_per_bar * (60.0 / bpm / 4.0)
}

/// How long `bars` bars play across a tempo range `[lo, hi]` bpm, shortest
/// first (`pieceTime`): the fastest tempo's and the slowest's.
#[must_use]
pub fn time(bars: f64, steps_per_bar: f64, tempo: [f64; 2]) -> [f64; 2] {
    [seconds(bars, steps_per_bar, js::max(tempo[0], tempo[1])), seconds(bars, steps_per_bar, js::min(tempo[0], tempo[1]))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn piece(bars: f64, length: Option<Length>) -> Option<Piece> {
        let mut spec = Spec::EMPTY;
        spec.bars = bars;
        spec.length = length;
        Piece::of(&spec)
    }

    #[test]
    fn a_piece_counts_its_sections() {
        assert_eq!(piece(8.0, None), None);
        let p = piece(8.0, Some(Length::Bars(1024.0))).expect("a piece");
        assert_eq!(p.end, Some((128, 8.0)));
        assert_eq!(p.length(), Some(1024.0));
        assert_eq!((p.bars_of(0), p.bars_of(127), p.begins_round(128), p.cut()), (8.0, 8.0, true, false));
        // 100 bars of 16: six whole sections and four bars.
        let p = piece(16.0, Some(Length::Bars(100.0))).expect("a piece");
        assert_eq!(p.end, Some((7, 4.0)));
        assert_eq!((p.bars_of(5), p.bars_of(6), p.bars_of(13), p.bars_of(14), p.length()), (16.0, 4.0, 4.0, 16.0, Some(100.0)));
        assert!(p.cut() && p.begins_round(7) && !p.begins_round(6));
        // A short length is the loop's own: one section, all of it.
        let p = piece(3.0, Some(Length::Bars(3.0))).expect("a piece");
        assert_eq!((p.end, p.bars_of(9), p.cut()), (Some((1, 3.0)), 3.0, false));
        let p = piece(4.0, Some(Length::Endless)).expect("a piece");
        assert_eq!((p.end, p.bars_of(1 << 40), p.begins_round(0), p.length()), (None, 4.0, false, None));
        // The longest length counts exactly.
        let p = piece(24.0, Some(Length::Bars(super::super::request::LENGTH_MOST))).expect("a piece");
        assert_eq!(p.length(), Some(super::super::request::LENGTH_MOST));
        assert_eq!(pass_number((1 << 32) + 5), 5);
    }

    #[test]
    fn the_play_time() {
        // Sixteen bars of 4/4 at 120 bpm: 32 s; at 60, 64 s.
        assert_eq!(seconds(16.0, 16.0, 120.0), 32.0);
        assert_eq!(time(16.0, 16.0, [60.0, 120.0]), [32.0, 64.0]);
    }
}
