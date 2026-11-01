//! Changing loops at a musical moment (queue item 86): what a host asks for
//! (`Player::change`), and the boundaries a change waits for. Broken
//! Friends changes its music at moments, not on a timer (a new room, a
//! friend made well, a mood): the next loop starts on the loop playing's
//! beat, bar, phrase or pass, and the old one hands over to it by a
//! crossfade or by letting its notes ring out, on one core, with no reset.
//!
//! **How it lands on the moment.** The player keeps one generator, so the
//! next loop can only be made once the loop playing has nothing left to
//! decide. A change waits until its boundary is `Player::lead` away (the
//! lookahead and the time the longest load takes, `LOAD_BLOCKS` blocks);
//! then the loop playing decides every step up to the boundary at once (its
//! notes handed over ahead, as a lookahead does), the next loop is made a
//! stage a block, and it takes its first step where the boundary falls. A
//! boundary nearer than that is not one a change can make: it takes the
//! next one of its kind (`Moment::Beat` the beat after, and so on). Should a
//! load ever take longer, the loop starts as soon as it is made, as `load`
//! starts one.

/// The most blocks a load takes, its stages and the first pass's draw
/// (`Generator::step`, `Player::decide`), with room to spare: the most
/// over 300 corpus loops is 92 (a loop with a tuned kick's fit). A change
/// begins making its loop this many blocks before its boundary, plus the
/// lookahead.
pub const LOAD_BLOCKS: usize = 160;

/// How long a crossfade takes by default, in seconds: the old loop's notes
/// fall from the boundary to nothing over it while the new one plays from
/// the boundary at its own level. For Mikey's ears: long enough that a
/// chord ringing over the moment is let go rather than cut, short enough
/// that two keys don't sound together for long.
pub const CROSSFADE: f64 = 1.0;
/// The longest crossfade asked for is held to this, in seconds.
pub const CROSSFADE_MAX: f64 = 30.0;

/// The bars a phrase takes, at most (`Moment::Phrase`): the tune's phrases
/// are two or four bars (`genMelody`'s `barsPerPhrase`), and a four-bar
/// line is a phrase's end in either.
pub const PHRASE_BARS: f64 = 4.0;

/// Where a change waits for: the boundary of the loop playing it starts on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Moment {
    /// As soon as the next loop is made, where the loop playing's next step
    /// would have fallen (`Player::load`'s moment).
    Now,
    /// The next beat: four steps (a quarter note), six in 6/8.
    Beat,
    /// The next bar line.
    Bar,
    /// The end of the phrase: the next line of `PHRASE_BARS` bars from the
    /// pass's start (two bars in a loop whose length four doesn't divide, a
    /// pass in one two doesn't either).
    Phrase,
    /// The end of the pass, where the loop would start again.
    Pass,
}

impl Moment {
    /// Every moment, in the order the hosts number them (0 to 4).
    pub const ALL: [Moment; 5] = [Moment::Now, Moment::Beat, Moment::Bar, Moment::Phrase, Moment::Pass];

    /// The moment a host numbers `n` (`ALL`'s order), if there is one.
    #[must_use]
    pub fn of(n: u32) -> Option<Moment> {
        Moment::ALL.get(n as usize).copied()
    }

    /// Its place among the changes that wait (`Now` waits for nothing).
    pub(super) fn slot(self) -> Option<usize> {
        match self {
            Moment::Now => None,
            Moment::Beat => Some(0),
            Moment::Bar => Some(1),
            Moment::Phrase => Some(2),
            Moment::Pass => Some(3),
        }
    }
}

/// How the loop playing hands over to the next at the boundary.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Handover {
    /// Everything the old loop handed over falls from the boundary to
    /// nothing over this many seconds (`Core::fade_all`), its organ letting
    /// go over the same; the new loop plays from the boundary at its own
    /// level.
    Crossfade(f64),
    /// The old loop's notes ring out as they would (`Player::load`'s way).
    RingOut,
}

impl Handover {
    /// The crossfade's length, held to 0 to `CROSSFADE_MAX` seconds (none
    /// to ring out). A length that is not a number is the default's.
    #[must_use]
    pub fn fade(self) -> Option<f64> {
        match self {
            Handover::Crossfade(x) if x.is_finite() => Some(x.clamp(0.0, CROSSFADE_MAX)),
            Handover::Crossfade(_) => Some(CROSSFADE),
            Handover::RingOut => None,
        }
    }
}

impl Default for Handover {
    /// A crossfade of `CROSSFADE` seconds.
    fn default() -> Self {
        Handover::Crossfade(CROSSFADE)
    }
}

/// A change waiting for its boundary (an event: a request attached to a
/// moment, promise 11): the loop asked for, how it hands over, when it was
/// asked (the latest for a boundary wins), and the step it starts on, from
/// the loop playing's load (none until worked out against the loop that
/// plays).
#[derive(Clone, Copy)]
pub(super) struct Change {
    pub seed: u32,
    pub request: super::Request,
    pub handover: Handover,
    pub asked: u32,
    pub target: Option<u64>,
}

/// The grid a moment's boundaries lie on, in steps of the loop playing:
/// `beat` steps a beat, `spb` a bar, `total` a pass. None for `Now`.
#[must_use]
pub fn unit(moment: Moment, beat: f64, spb: f64, total: f64) -> Option<f64> {
    let phrase = if total % (PHRASE_BARS * spb) == 0.0 {
        PHRASE_BARS * spb
    } else if total % (2.0 * spb) == 0.0 {
        2.0 * spb
    } else {
        total
    };
    let u = match moment {
        Moment::Now => return None,
        Moment::Beat => beat,
        Moment::Bar => spb,
        Moment::Phrase => phrase,
        Moment::Pass => total,
    };
    (u.is_finite() && u >= 1.0).then_some(u)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_grids_divide_the_pass() {
        // Four bars of 4/4: a beat 4 steps, a bar 16, the phrase the pass.
        assert_eq!(unit(Moment::Beat, 4.0, 16.0, 64.0), Some(4.0));
        assert_eq!(unit(Moment::Bar, 4.0, 16.0, 64.0), Some(16.0));
        assert_eq!(unit(Moment::Phrase, 4.0, 16.0, 64.0), Some(64.0));
        assert_eq!(unit(Moment::Phrase, 4.0, 16.0, 128.0), Some(64.0));
        // Six bars: two-bar phrases; five: the pass.
        assert_eq!(unit(Moment::Phrase, 4.0, 16.0, 96.0), Some(32.0));
        assert_eq!(unit(Moment::Phrase, 6.0, 12.0, 60.0), Some(60.0));
        assert_eq!(unit(Moment::Pass, 6.0, 12.0, 60.0), Some(60.0));
        assert_eq!(unit(Moment::Now, 4.0, 16.0, 64.0), None);
    }

    #[test]
    fn a_crossfade_is_held_to_its_range() {
        assert_eq!(Handover::Crossfade(2.0).fade(), Some(2.0));
        assert_eq!(Handover::Crossfade(-1.0).fade(), Some(0.0));
        assert_eq!(Handover::Crossfade(f64::NAN).fade(), Some(CROSSFADE));
        assert_eq!(Handover::Crossfade(1e9).fade(), Some(CROSSFADE_MAX));
        assert_eq!(Handover::RingOut.fade(), None);
        assert_eq!(Handover::default(), Handover::Crossfade(CROSSFADE));
        assert_eq!(Moment::of(4), Some(Moment::Pass));
        assert_eq!(Moment::of(5), None);
    }
}
