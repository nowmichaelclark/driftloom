//! How long a track plays and how it comes in and goes out, as the player
//! takes it and a code carries it (session 30c, queue item 111): shared by
//! `player::track` and `code`'s `Pace`, so the two read a host's numbers
//! the same way. Here, and not in the player, because the codes build
//! without it (the generator's checks, `tools/gen-core.sh`).

/// The longest time a track is set to: a day, as the app's old length
/// slider reached.
pub const TRACK_MOST: u32 = 86_400;

/// How a track comes in and goes out: the listener's two keys in the app's
/// Track length view. None is a track as it always played: in at once, and
/// out by the track's own close.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Fades {
    /// Up from silence at the track's start (`player::track::FADE_IN`).
    pub fade_in: bool,
    /// Down to silence on a time's last second (`player::track::FADE_OUT`;
    /// a time's only: a count of passes or bars ends on its line, as it
    /// always did).
    pub fade_out: bool,
}

impl Fades {
    /// No fades: a track as it always played. All zeros.
    pub const NONE: Fades = Fades { fade_in: false, fade_out: false };
}

/// The bit for a fade in, beside a track's kind (`player::Track::of`,
/// `code::Pace::of`).
pub const KIND_FADE_IN: u32 = 4;
/// The bit for a fade out (a time's only).
pub const KIND_FADE_OUT: u32 = 8;
