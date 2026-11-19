//! Share codes, version 2 (queue item 105): a loop, or an album, written as a
//! short run of the 64 link-safe characters (`A-Z a-z 0-9 - _`), read back
//! by any host the same way. `core/README.md`, "Share codes", is the format,
//! small and exact; this is the one implementation of it.
//!
//! **What a loop's code holds:** its version, its seed, and only what was
//! asked or changed beside it (the request's vibes, tempo, length, journey,
//! key, mode, style, voices, layers, pins and theme; a code's edits; the
//! listener's mutes; the wander's salt of a play that wandered
//! unrepeatably; a name of its own, if it was renamed: `name.rs`). A loop
//! with nothing asked is nine characters. **An album's** holds its tracks'
//! bodies, and how long each plays before the next where its maker set it
//! (`Pace`, session 24b): the pace most tracks share once, the others each
//! their own.
//!
//! **Everything is named by its id** (`data/codes.txt`): a vibe, a profile, a
//! choice or a voice by the number given it there once, never by its place
//! in a list, and every number is variable-width, so new members fit without
//! a new format. **A code's version holds its sound:** a loop read from a
//! code is drawn at the code's version (`Request::with_version`), its pools
//! holding only what had joined by then, so an addition never moves an older
//! code's picks.
//!
//! **Refused in plain words, never played wrong:** a code from a newer build
//! (its version, or an id this build does not have) is `NEEDS_NEWER`; one
//! that is no code (a character outside the alphabet, a typo the check
//! catches, a field out of place) is `NOT_A_CODE`. Nothing a code holds can
//! panic the reader: every number is bounded and every slice checked.
//!
//! Nothing here allocates: codes are written into the caller's bytes and
//! read from them.

use crate::generator::data::{self, CHOICES, Ids, VIBE_COUNT, VIBES};
use crate::generator::request::{self, FIRST_VERSION, Journey, LAYER_NAMES, LENGTH_MOST, LayerAsk, Length, PROFILE_ORDER, Pins, Request, TONE_NAMES, VOICE_ORDER};
use crate::generator::rng::Rng;
use crate::generator::theory::SCALES;
use crate::fades::{Fades, KIND_FADE_IN, KIND_FADE_OUT, TRACK_MOST};

pub use crate::generator::request::NEEDS_NEWER;
pub use name::{BAD_NAME, MARKS_MOST, NAME_MOST, Name, TAGS_MOST};

mod classes;
mod name;

/// The line for text that is no code: a character outside the alphabet, a
/// typo the check catches, a code cut short or a field out of place.
pub const NOT_A_CODE: &str = "not a code";
/// The line for a code that does not fit the room the host gave it.
pub const NO_ROOM: &str = "no room for the code";
/// The line for a request that asks for something its own version does not
/// have (a request built by hand at an older version).
pub const NOT_AT_VERSION: &str = "asks for something its version does not have";
/// The line for an album of no tracks, or more than `ALBUM_MOST`.
pub const BAD_ALBUM: &str = "a bad album";
/// The line for a track's pace a code can't carry: a time of none, or past
/// `PACE_MOST`, or a kind the player has no such track for (whole passes and
/// bars among them, since session 30c).
pub const BAD_PACE: &str = "a bad track length";

/// The alphabet, a character a sextet (its value, 0 to 63, by place).
pub const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
/// The longest loop's code read or written, in characters: room for every
/// pin and every vibe at once, which no listener asks.
pub const LOOP_MOST: usize = 4096;
/// The most tracks an album holds.
pub const ALBUM_MOST: usize = 1024;

/// The check's modulus: the largest prime below 4096, so two characters hold
/// it, any one character changed or two side by side swapped changes it.
const CHECK_PRIME: u32 = 4093;

/// The fields' tags, in the order a code writes them (`core/README.md`).
mod tag {
    pub const VERSION: u64 = 0;
    pub const VIBE: u64 = 1;
    pub const VIBE_WEIGHTED: u64 = 2;
    pub const TEMPO: u64 = 3;
    pub const TEMPO_RANGE: u64 = 4;
    pub const LENGTH: u64 = 5;
    pub const ENDLESS: u64 = 6;
    pub const JOURNEY: u64 = 7;
    pub const KEYS: u64 = 8;
    pub const SCALES: u64 = 9;
    pub const STYLES: u64 = 10;
    pub const VOICES: u64 = 11;
    pub const LAYERS_ON: u64 = 12;
    pub const LAYERS_OFF: u64 = 13;
    pub const THEME: u64 = 14;
    pub const CHOICE: u64 = 15;
    pub const SETTING: u64 = 16;
    pub const SEED: u64 = 17;
    pub const TONE: u64 = 18;
    pub const BPM: u64 = 19;
    pub const MUTES: u64 = 20;
    pub const SALT: u64 = 21;
    pub const PACE: u64 = 22;
    pub const NAME: u64 = 30;
}

/// A loop as a code holds it: its seed, what was asked of it (its version
/// among it: `Request::version`, 0 for this build's newest), the layers the
/// listener muted (a bit each in the core's layer order), the wander's
/// salt of the play it was shared from (queue item 92; 0 for none) and its
/// own name, if it was renamed (`Name::NONE` for the one its seed gives).
#[derive(Clone, Copy)]
pub struct Loop {
    /// The loop's seed.
    pub seed: u32,
    /// What was asked of it, and the edits.
    pub request: Request,
    /// The layers muted, a bit each in the core's layer order.
    pub mutes: u8,
    /// The wander's salt, or 0.
    pub salt: u32,
    /// Its own name, or `Name::NONE`.
    pub name: Name,
}

impl Loop {
    /// The loop of `seed`, nothing asked.
    #[must_use]
    pub const fn of(seed: u32) -> Self {
        Loop { seed, request: Request::new(), mutes: 0, salt: 0, name: Name::NONE }
    }
}

/// The most seconds a track's pace counts: a day, as the player's longest
/// time (`fades::TRACK_MOST`; session 30c).
pub const PACE_MOST: u32 = TRACK_MOST;

/// How long an album's track plays before the next (session 24b; Mikey,
/// 2026-11-18: "an album's code should carry track lengths and all
/// necessary info"): the Track length view as the album's maker had it, as
/// the player takes it (`player::Track`, `player::Fades`; `spec.track` in
/// the app), so an album plays as its maker heard it. Since session 30c a
/// time to the second, with its fade in and fade out (Mikey, 2026-11-18:
/// "Should be able to set track length to an exact second"), where it was
/// whole passes or bars. A loop's own code carries none: one loop sounds
/// the same however long the listener lets it play, and when the listener's
/// app moves on from it is the listener's (#412).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Pace {
    /// On and on: the track plays until the listener moves on.
    #[default]
    On,
    /// On and on, up from silence at its start (its fade in).
    OnFadingIn,
    /// This many seconds, 1 to `PACE_MOST`, with its fades.
    Time {
        /// How long it plays.
        seconds: u32,
        /// Its fade in and its fade out.
        fades: Fades,
    },
}

/// The numbers a pace is written as, all of them (`Pace::number`): 0 and 1
/// on and on, and four for each second (its two fades).
const PACE_NUMBERS: u64 = 2 + 4 * PACE_MOST as u64;

impl Pace {
    /// The pace of the player's track kind and count (`Player::set_track_with`,
    /// `player::Track::of`, js/player.js's `trackOf`): kind 0 on and on
    /// (count 0; 4 added for its fade in), 3 a time of `count` seconds (1 to
    /// `PACE_MOST`; 4 added for a fade in, 8 for a fade out). `None` for
    /// anything else: whole passes and bars are no pace a code carries.
    #[must_use]
    pub const fn of(kind: u32, count: u32) -> Option<Pace> {
        let fades = Fades { fade_in: kind & KIND_FADE_IN != 0, fade_out: kind & KIND_FADE_OUT != 0 };
        match (kind & !(KIND_FADE_IN | KIND_FADE_OUT), count, fades.fade_out) {
            (0, 0, false) => Some(if fades.fade_in { Pace::OnFadingIn } else { Pace::On }),
            (3, 1..=PACE_MOST, _) => Some(Pace::Time { seconds: count, fades }),
            _ => None,
        }
    }

    /// Its kind, as `of` takes it.
    #[must_use]
    pub const fn kind(self) -> u32 {
        match self {
            Pace::On => 0,
            Pace::OnFadingIn => KIND_FADE_IN,
            Pace::Time { fades, .. } => 3 | if fades.fade_in { KIND_FADE_IN } else { 0 } | if fades.fade_out { KIND_FADE_OUT } else { 0 },
        }
    }

    /// Its count, as `of` takes it.
    #[must_use]
    pub const fn count(self) -> u32 {
        match self {
            Pace::On | Pace::OnFadingIn => 0,
            Pace::Time { seconds, .. } => seconds,
        }
    }

    /// The number a code writes: 0 on and on, 1 on and on fading in, and
    /// `2 + 4 (s - 1) + f` for `s` seconds with fades `f` (1 the fade in, 2
    /// the fade out), so a track of up to four seconds is one character,
    /// and of up to an hour and a quarter three. `None` for a time a code
    /// can't carry.
    const fn number(self) -> Option<u32> {
        match self {
            Pace::On => Some(0),
            Pace::OnFadingIn => Some(1),
            Pace::Time { seconds: s @ 1..=PACE_MOST, fades } => {
                Some(2 + 4 * (s - 1) + if fades.fade_in { 1 } else { 0 } + if fades.fade_out { 2 } else { 0 })
            }
            Pace::Time { .. } => None,
        }
    }

    /// The pace of a number `number` wrote (every number below
    /// `PACE_NUMBERS` is one: a code made before session 30c, whose numbers
    /// were passes and bars, still reads, as times of a few seconds).
    const fn from_number(p: u32) -> Pace {
        match p {
            0 => Pace::On,
            1 => Pace::OnFadingIn,
            _ => {
                let v = p - 2;
                Pace::Time { seconds: v / 4 + 1, fades: Fades { fade_in: v % 4 & 1 != 0, fade_out: v % 4 & 2 != 0 } }
            }
        }
    }
}

/// A track as a host writes it beside its pace, in text (`dl_album_write`,
/// `dl_code_album_write`, `node tools/codes.mjs album`): the loop's code,
/// then `:t` and the seconds where it has a time, `i` after them for its
/// fade in and `o` for its fade out (`ENBAAAAEmBO_7:t5025o`), `:i` alone for
/// one that plays on and on fading in, nothing where it plays on. Split
/// here into the code and its pace.
///
/// # Errors
/// `BAD_PACE` for a pace written any other way (a time of 0, one with a 0
/// before it, one past `PACE_MOST`, the fades out of order).
pub fn paced(text: &[u8]) -> Result<(&[u8], Pace), &'static str> {
    let Some(at) = text.iter().rposition(|&b| b == b':') else { return Ok((text, Pace::On)) };
    let (code, rest) = (&text[..at], &text[at + 1..]);
    if rest == b"i" {
        return Ok((code, Pace::OnFadingIn));
    }
    let Some(rest) = rest.strip_prefix(b"t") else { return Err(BAD_PACE) };
    let digits = rest.iter().take_while(|b| b.is_ascii_digit()).count();
    let (digits, flags) = rest.split_at(digits);
    if digits.is_empty() || digits.len() > 10 || digits[0] == b'0' {
        return Err(BAD_PACE);
    }
    let fades = match flags {
        b"" => Fades::NONE,
        b"i" => Fades { fade_in: true, fade_out: false },
        b"o" => Fades { fade_in: false, fade_out: true },
        b"io" => Fades { fade_in: true, fade_out: true },
        _ => return Err(BAD_PACE),
    };
    let n = digits.iter().fold(0u64, |n, &d| n * 10 + u64::from(d - b'0'));
    let seconds = u32::try_from(n).ok().filter(|n| (1..=PACE_MOST).contains(n)).ok_or(BAD_PACE)?;
    Ok((code, Pace::Time { seconds, fades }))
}

/// The pace most of `paces` share (their numbers), the smaller on a tie:
/// the one an album writes once for all (`write_album`). Sorts `paces`.
fn most_common(paces: &mut [u32]) -> u32 {
    paces.sort_unstable();
    let (mut best, mut most) = (0, 0);
    let mut i = 0;
    while i < paces.len() {
        let run = paces[i..].iter().take_while(|&&p| p == paces[i]).count();
        if run > most {
            (best, most) = (paces[i], run);
        }
        i += run;
    }
    best
}

/// What a code is, by its first character: a loop's or an album's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// One loop.
    Loop,
    /// A list of loops, or an album drawn from one seed.
    Album,
}

// ------------------------------------------------------------------ writing

/// Sextets written into the caller's bytes, as characters.
struct Out<'a> {
    bytes: &'a mut [u8],
    len: usize,
    full: bool,
}

impl<'a> Out<'a> {
    fn new(bytes: &'a mut [u8]) -> Self {
        Out { bytes, len: 0, full: false }
    }

    fn put(&mut self, sextet: u8) {
        match self.bytes.get_mut(self.len) {
            Some(b) => {
                *b = ALPHABET[usize::from(sextet & 63)];
                self.len += 1;
            }
            None => self.full = true,
        }
    }

    /// A whole number: five bits a character, the lowest first, 32 added to
    /// every character but the last.
    fn uv(&mut self, mut x: u64) {
        while x >= 32 {
            #[allow(clippy::cast_possible_truncation)] // five bits
            self.put((x & 31) as u8 | 32);
            x >>= 5;
        }
        #[allow(clippy::cast_possible_truncation)] // below 32
        self.put(x as u8);
    }

    /// A 32-bit seed: six characters, the lowest six bits first.
    fn seed(&mut self, s: u32) {
        for k in 0..6 {
            #[allow(clippy::cast_possible_truncation)] // six bits
            self.put(((s >> (6 * k)) & 63) as u8);
        }
    }

    /// A number as its shortest decimal (`decimal`): the digits, then the
    /// power of ten.
    fn dec(&mut self, x: f64) {
        let (m, e) = decimal(x);
        self.uv(zigzag(m));
        self.uv(e);
    }

    /// A set of ids: how many, then the first, then each gap less one.
    fn set(&mut self, ids: &[u32]) {
        self.uv(ids.len() as u64);
        let mut last: Option<u32> = None;
        for &id in ids {
            self.uv(u64::from(last.map_or(id, |l| id - l - 1)));
            last = Some(id);
        }
    }

    /// The check over every character written, then nothing more.
    fn check(&mut self) -> Result<usize, &'static str> {
        let h = check_of(&self.bytes[..self.len.min(self.bytes.len())]);
        #[allow(clippy::cast_possible_truncation)] // six bits each
        {
            self.put((h & 63) as u8);
            self.put((h >> 6) as u8);
        }
        if self.full { Err(NO_ROOM) } else { Ok(self.len) }
    }
}

/// The check of characters already known to be the alphabet's: each one's
/// value plus one, read as a number in base 64, modulo `CHECK_PRIME`.
fn check_of(chars: &[u8]) -> u32 {
    chars.iter().fold(0, |h, &c| (h * 64 + u32::from(sextet(c).unwrap_or(0)) + 1) % CHECK_PRIME)
}

/// A character's value, or `None` for one outside the alphabet.
fn sextet(c: u8) -> Option<u8> {
    match c {
        b'A'..=b'Z' => Some(c - b'A'),
        b'a'..=b'z' => Some(c - b'a' + 26),
        b'0'..=b'9' => Some(c - b'0' + 52),
        b'-' => Some(62),
        b'_' => Some(63),
        _ => None,
    }
}

const fn zigzag(m: i64) -> u64 {
    #[allow(clippy::cast_sign_loss)] // the zigzag's own bit trick
    let z = ((m << 1) ^ (m >> 63)) as u64;
    z
}

const fn unzigzag(z: u64) -> i64 {
    #[allow(clippy::cast_possible_wrap)] // the zigzag's own bit trick
    let m = (z >> 1) as i64 ^ -((z & 1) as i64);
    m
}

/// A finite number's shortest decimal, the one that reads back to it
/// exactly (`{:e}`, the shortest that round-trips): its digits as a whole
/// number with no zero at its end, and the power of ten, zigzagged. Zero is
/// digits 0 and power 0; minus zero, digits 0 and power 1 (the one place a
/// sign of zero is kept: a pin may be -0).
fn decimal(x: f64) -> (i64, u64) {
    if x == 0.0 {
        return (0, u64::from(x.is_sign_negative()));
    }
    let mut buf = Text::<40>::new();
    let _ = core::fmt::write(&mut buf, format_args!("{x:e}"));
    let s = buf.as_bytes();
    let (neg, s) = match s.split_first() {
        Some((b'-', rest)) => (true, rest),
        _ => (false, s),
    };
    let e_at = s.iter().position(|&b| b == b'e').unwrap_or(s.len());
    let (mant, exp) = s.split_at(e_at);
    let mut m: i64 = 0;
    let mut frac = 0i64;
    let mut seen_point = false;
    for &b in mant {
        if b == b'.' {
            seen_point = true;
        } else if b.is_ascii_digit() {
            m = m * 10 + i64::from(b - b'0');
            if seen_point {
                frac += 1;
            }
        }
    }
    let exp = exp.get(1..).unwrap_or(&[]);
    let (eneg, exp) = match exp.split_first() {
        Some((b'-', rest)) => (true, rest),
        _ => (false, exp),
    };
    let mut e = exp.iter().filter(|b| b.is_ascii_digit()).fold(0i64, |a, &b| a * 10 + i64::from(b - b'0'));
    if eneg {
        e = -e;
    }
    e -= frac;
    while m != 0 && m % 10 == 0 {
        m /= 10;
        e += 1;
    }
    (if neg { -m } else { m }, zigzag(e))
}

/// The number `decimal` wrote, or `None` for digits and a power that are
/// not one's shortest decimal (so every number has one code).
fn from_decimal(m: i64, e: u64) -> Option<f64> {
    if m == 0 {
        return match e {
            0 => Some(0.0),
            1 => Some(-0.0),
            _ => None,
        };
    }
    let p = unzigzag(e);
    // Seventeen digits and a power within the doubles' own reach.
    if m.unsigned_abs() >= 100_000_000_000_000_000 || !(-400..=400).contains(&p) || m % 10 == 0 {
        return None;
    }
    let mut buf = Text::<48>::new();
    let _ = core::fmt::write(&mut buf, format_args!("{m}e{p}"));
    let x: f64 = core::str::from_utf8(buf.as_bytes()).ok()?.parse().ok()?;
    (x.is_finite() && decimal(x) == (m, e)).then_some(x)
}

/// A little text on the stack, for the numbers' digits.
struct Text<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> Text<N> {
    const fn new() -> Self {
        Text { buf: [0; N], len: 0 }
    }
    fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}

impl<const N: usize> core::fmt::Write for Text<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let end = self.len + s.len();
        let slot = self.buf.get_mut(self.len..end).ok_or(core::fmt::Error)?;
        slot.copy_from_slice(s.as_bytes());
        self.len = end;
        Ok(())
    }
}

/// `name`'s id in `list`.
fn id_of(list: Ids, name: &str, version: u32) -> Result<u32, &'static str> {
    let e = list.iter().find(|e| e.0 == name).ok_or(NOT_AT_VERSION)?;
    if data::joined(e.2, version) { Ok(e.1) } else { Err(NOT_AT_VERSION) }
}

/// The ids of `names`, sorted.
fn ids_sorted<const N: usize>(list: Ids, names: impl Iterator<Item = &'static str>, version: u32) -> Result<([u32; N], usize), &'static str> {
    let mut out = [0u32; N];
    let mut n = 0;
    for name in names {
        let slot = out.get_mut(n).ok_or(NOT_AT_VERSION)?;
        *slot = id_of(list, name, version)?;
        n += 1;
    }
    out[..n].sort_unstable();
    Ok((out, n))
}

/// The names of the bits set in `bits`, by place in `names`.
fn bit_names(bits: u32, names: &'static [&'static str]) -> impl Iterator<Item = &'static str> {
    names.iter().enumerate().filter(move |(i, _)| *i < 32 && bits & (1 << i) != 0).map(|(_, &n)| n)
}

/// The layers' names of a layer bit set.
fn layer_bits(bits: u8) -> impl Iterator<Item = &'static str> {
    bit_names(u32::from(bits), &LAYER_NAMES)
}

/// The version a request writes: its own, or this build's newest.
fn version_of(r: &Request) -> u32 {
    if r.version() == 0 { data::CODE_NEWEST } else { r.version() }
}

/// A loop's seed and fields (everything but its head and check), at
/// `version`; a track of an album at another version writes its own first,
/// and a track whose pace is not its album's writes its own (`pace`, its
/// number) before its name.
#[allow(clippy::too_many_lines)] // one field after another, in their order
fn body(w: &mut Out, song: &Loop, version: u32, own_version: Option<u32>, picks_only: bool, pace: Option<u32>) -> Result<(), &'static str> {
    let req = &song.request;
    w.seed(song.seed);
    if let Some(v) = own_version {
        w.uv(tag::VERSION);
        w.uv(u64::from(v));
    }
    // The vibes, by id, each with its weight or none.
    let mut vibes = [(0u32, f64::NAN); VIBE_COUNT];
    let mut asked = 0;
    for (i, v) in VIBES.iter().enumerate() {
        if let Some(weight) = req.vibe(i) {
            vibes[asked] = (id_of(data::CODE_VIBES, v.id, version)?, weight.unwrap_or(f64::NAN));
            asked += 1;
        }
    }
    vibes[..asked].sort_unstable_by_key(|e| e.0);
    // Those asked without a weight first, then those with one.
    for weighted in [false, true] {
        for &(id, weight) in vibes[..asked].iter().filter(|e| e.1.is_nan() != weighted) {
            w.uv(if weighted { tag::VIBE_WEIGHTED } else { tag::VIBE });
            w.uv(u64::from(id));
            if weighted {
                w.dec(weight);
            }
        }
    }
    // A tempo pinned, its bpm; a range, its low and how far past it.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // whole bpm, 32 to 190
    if let Some([lo, hi]) = req.tempo() {
        let low = (lo - request::TEMPO_LIMIT[0]) as u64;
        let span = (hi - lo) as u64;
        if span == 0 {
            w.uv(tag::TEMPO);
            w.uv(low);
        } else {
            w.uv(tag::TEMPO_RANGE);
            w.uv(low);
            w.uv(span - 1);
        }
    }
    match req.length() {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // whole bars, 1 to 2^53 - 1
        Some(Length::Bars(bars)) => {
            w.uv(tag::LENGTH);
            w.uv(bars as u64 - 1);
        }
        Some(Length::Endless) => w.uv(tag::ENDLESS),
        None => {}
    }
    if let Some(j) = req.journey() {
        w.uv(tag::JOURNEY);
        w.uv(u64::from(id_of(data::CODE_JOURNEYS, j.name(), version)?));
    }
    if req.keys() != 0 {
        let mut keys = [0u32; 12];
        let mut len = 0;
        for note in 0..12 {
            if req.keys() & (1 << note) != 0 {
                keys[len] = note;
                len += 1;
            }
        }
        w.uv(tag::KEYS);
        w.set(&keys[..len]);
    }
    if req.scales() != 0 {
        let names = SCALES.iter().enumerate().filter(|(i, _)| req.scales() & (1 << i) != 0).map(|(_, s)| s.id);
        let (ids, len) = ids_sorted::<32>(data::CODE_SCALES, names, version)?;
        w.uv(tag::SCALES);
        w.set(&ids[..len]);
    }
    if req.styles() != 0 {
        let (ids, len) = ids_sorted::<32>(data::CODE_PROFILES, bit_names(req.styles(), &PROFILE_ORDER), version)?;
        w.uv(tag::STYLES);
        w.set(&ids[..len]);
    }
    // Each layer asked voices, by the layer's id.
    let mut layers = [(0u32, 0usize); 5];
    for (k, name) in LAYER_NAMES.iter().enumerate() {
        layers[k] = (id_of(data::CODE_LAYERS, name, version)?, k);
    }
    layers.sort_unstable_by_key(|e| e.0);
    for &(id, layer) in &layers {
        let bits = req.voices(layer);
        if bits != 0 {
            let (ids, len) = ids_sorted::<32>(data::CODE_VOICES[layer], bit_names(bits, VOICE_ORDER[layer]), version)?;
            w.uv(tag::VOICES);
            w.uv(u64::from(id));
            w.set(&ids[..len]);
        }
    }
    for (t, bits) in [(tag::LAYERS_ON, req.layers_on()), (tag::LAYERS_OFF, req.layers_off())] {
        if bits != 0 {
            let (ids, len) = ids_sorted::<8>(data::CODE_LAYERS, layer_bits(bits), version)?;
            w.uv(t);
            w.set(&ids[..len]);
        }
    }
    if let Some(theme) = req.theme() {
        w.uv(tag::THEME);
        w.seed(theme);
    }
    pins(w, req.pins(), version)?;
    if picks_only {
        return Ok(());
    }
    // The edits: each layer's seed, by the layer's id (re-rolled or not).
    let edits = req.edits();
    for &(id, layer) in &layers {
        if let Some(seed) = edits.seed(layer) {
            w.uv(tag::SEED);
            w.uv(u64::from(id) * 2 + u64::from(edits.rolled() & (1 << layer) != 0));
            w.seed(seed);
        }
    }
    let mut tones = [(0u32, 0usize); 3];
    for (k, name) in TONE_NAMES.iter().enumerate() {
        tones[k] = (id_of(data::CODE_TONES, name, version)?, k);
    }
    tones.sort_unstable_by_key(|t| t.0);
    for &(id, k) in &tones {
        if let Some(x) = edits.tone(k) {
            w.uv(tag::TONE);
            w.uv(u64::from(id));
            w.dec(x);
        }
    }
    if let Some(bpm) = edits.bpm() {
        w.uv(tag::BPM);
        w.dec(bpm);
    }
    if song.mutes & 31 != 0 {
        let (ids, len) = ids_sorted::<8>(data::CODE_LAYERS, layer_bits(song.mutes & 31), version)?;
        w.uv(tag::MUTES);
        w.set(&ids[..len]);
    }
    if song.salt != 0 {
        w.uv(tag::SALT);
        w.seed(song.salt);
    }
    if let Some(p) = pace {
        w.uv(tag::PACE);
        w.uv(u64::from(p));
    }
    if !song.name.is_none() {
        w.uv(tag::NAME);
        song.name.write(w);
    }
    Ok(())
}

/// A pin's value as a code writes it: an option's or a word's id, or a
/// number.
#[derive(Clone, Copy)]
enum Value {
    Word(u32),
    Number(f64),
}

/// The pins: the choices by id, each its option's id; then the settings by
/// id, each its word's id or its number.
fn pins(w: &mut Out, p: &Pins, version: u32) -> Result<(), &'static str> {
    let mut choices = [(0u32, 0u32); request::PINS_MAX];
    let mut settings = [(0u32, Value::Number(0.0)); request::PINS_MAX];
    let (mut nc, mut ns) = (0, 0);
    for number in 0..request::PINS_MAX {
        let Some(x) = p.at(number) else { continue };
        let place = crate::generator::js::index(x);
        if number < request::CHOICE_PINS {
            let c = CHOICES.get(number).ok_or(NOT_AT_VERSION)?;
            let option = c.options.get(place).ok_or(NOT_AT_VERSION)?.0;
            choices[nc] = (id_of(data::CODE_CHOICES, c.id, version)?, id_of(options_of(c.id), option, version)?);
            nc += 1;
        } else {
            let mut full = Text::<64>::new();
            let (prefix, name) = Pins::name(number).ok_or(NOT_AT_VERSION)?;
            let _ = core::fmt::write(&mut full, format_args!("{prefix}{name}"));
            let full = core::str::from_utf8(full.as_bytes()).unwrap_or("");
            let id = id_of(data::CODE_SETTINGS, full, version)?;
            let value = match Pins::words(number) {
                Some(words) => Value::Word(id_of(words_of(full), words.get(place).ok_or(NOT_AT_VERSION)?.0, version)?),
                None => Value::Number(x),
            };
            settings[ns] = (id, value);
            ns += 1;
        }
    }
    choices[..nc].sort_unstable_by_key(|e| e.0);
    settings[..ns].sort_unstable_by_key(|e| e.0);
    for &(id, option) in &choices[..nc] {
        w.uv(tag::CHOICE);
        w.uv(u64::from(id));
        w.uv(u64::from(option));
    }
    for &(id, value) in &settings[..ns] {
        w.uv(tag::SETTING);
        w.uv(u64::from(id));
        match value {
            Value::Word(word) => w.uv(u64::from(word)),
            Value::Number(x) => w.dec(x),
        }
    }
    Ok(())
}

/// Choice `id`'s options' list in `data/codes.txt`.
fn options_of(id: &str) -> Ids {
    data::CODE_OPTIONS.iter().find(|e| e.0 == id).map_or(&[], |e| e.1)
}

/// Setting `name`'s words' list in `data/codes.txt`.
fn words_of(name: &str) -> Ids {
    data::CODE_WORDS.iter().find(|e| e.0 == name).map_or(&[], |e| e.1)
}

/// Write `l`'s code into `out`: its length in bytes (plain ASCII).
///
/// # Errors
/// `NO_ROOM` if it does not fit, `NOT_AT_VERSION` for a request that asks
/// for something its own version does not have.
pub fn write_loop(l: &Loop, out: &mut [u8]) -> Result<usize, &'static str> {
    let version = version_of(&l.request);
    let mut w = Out::new(out);
    w.uv(u64::from(version) * 2);
    body(&mut w, l, version, None, false, None)?;
    w.check()
}

// ------------------------------------------------------------------ reading

/// A code's characters as sextets, read one field at a time.
struct In<'a> {
    chars: &'a [u8],
    at: usize,
}

impl In<'_> {
    fn done(&self) -> bool {
        self.at >= self.chars.len()
    }

    fn take(&mut self) -> Result<u8, &'static str> {
        let c = *self.chars.get(self.at).ok_or(NOT_A_CODE)?;
        self.at += 1;
        sextet(c).ok_or(NOT_A_CODE)
    }

    /// A whole number (`Out::uv`), written in as few characters as it
    /// needs (a last character of 0 after another is no code's), at most 64
    /// bits.
    fn uv(&mut self) -> Result<u64, &'static str> {
        let mut x: u64 = 0;
        for k in 0..13 {
            let s = self.take()?;
            let bits = u64::from(s & 31);
            if k == 12 && bits > 15 {
                return Err(NOT_A_CODE);
            }
            x |= bits << (5 * k);
            if s & 32 == 0 {
                if k > 0 && bits == 0 {
                    return Err(NOT_A_CODE);
                }
                return Ok(x);
            }
        }
        Err(NOT_A_CODE)
    }

    /// A whole number below `most`.
    fn below(&mut self, most: u64) -> Result<u64, &'static str> {
        let x = self.uv()?;
        if x < most { Ok(x) } else { Err(NOT_A_CODE) }
    }

    /// An id (any 32-bit number: one this build lacks is a newer build's).
    fn id(&mut self) -> Result<u32, &'static str> {
        u32::try_from(self.uv()?).map_err(|_| NEEDS_NEWER)
    }

    fn seed(&mut self) -> Result<u32, &'static str> {
        let mut s: u64 = 0;
        for k in 0..6 {
            s |= u64::from(self.take()?) << (6 * k);
        }
        u32::try_from(s).map_err(|_| NOT_A_CODE)
    }

    fn dec(&mut self) -> Result<f64, &'static str> {
        let m = unzigzag(self.uv()?);
        let e = self.uv()?;
        from_decimal(m, e).ok_or(NOT_A_CODE)
    }

    /// A set (`Out::set`): its ids, ascending, at least one.
    fn set(&mut self, out: &mut [u32; 64]) -> Result<usize, &'static str> {
        let n = usize::try_from(self.below(65)?).map_err(|_| NOT_A_CODE)?;
        if n == 0 {
            return Err(NOT_A_CODE);
        }
        let mut last: Option<u64> = None;
        for slot in out.iter_mut().take(n) {
            let gap = self.uv()?;
            let id = match last {
                None => gap,
                Some(l) => l.checked_add(gap).and_then(|x| x.checked_add(1)).ok_or(NOT_A_CODE)?,
            };
            *slot = u32::try_from(id).map_err(|_| NEEDS_NEWER)?;
            last = Some(id);
        }
        Ok(n)
    }
}

/// The name of `id` in `list`, at `version`: an id this build lacks is a
/// newer build's; one that joined after the code's version, no code's.
fn name_of(list: Ids, id: u32, version: u32) -> Result<&'static str, &'static str> {
    let e = list.iter().find(|e| e.1 == id).ok_or(NEEDS_NEWER)?;
    if data::joined(e.2, version) { Ok(e.0) } else { Err(NOT_A_CODE) }
}

/// A name's place in `names` (the request's own lists), or a newer build's.
fn place(names: &[&str], name: &str) -> Result<usize, &'static str> {
    names.iter().position(|&n| n == name).ok_or(NEEDS_NEWER)
}

/// A layer by its id.
fn layer_of(id: u32, version: u32) -> Result<usize, &'static str> {
    place(&LAYER_NAMES, name_of(data::CODE_LAYERS, id, version)?)
}

/// A refusal of the request's own (a value outside what it takes) is no
/// code's.
fn met<T>(r: Result<T, &'static str>) -> Result<T, &'static str> {
    r.map_err(|why| if why == NEEDS_NEWER { why } else { NOT_A_CODE })
}

/// The characters of a code, trimmed of the spaces a paste brings, checked:
/// the alphabet's only, long enough, and the check right. Returns them less
/// the check.
fn checked(code: &[u8], most: usize) -> Result<&[u8], &'static str> {
    let start = code.iter().position(|b| !b.is_ascii_whitespace()).unwrap_or(code.len());
    let end = code.iter().rposition(|b| !b.is_ascii_whitespace()).map_or(start, |e| e + 1);
    let code = &code[start..end];
    if code.len() < 9 || code.len() > most || code.iter().any(|&c| sextet(c).is_none()) {
        return Err(NOT_A_CODE);
    }
    let (chars, check) = code.split_at(code.len() - 2);
    let given = u32::from(sextet(check[0]).unwrap_or(0)) | u32::from(sextet(check[1]).unwrap_or(0)) << 6;
    if check_of(chars) != given {
        return Err(NOT_A_CODE);
    }
    Ok(chars)
}

/// The head: the version and whether it is an album's. A version past this
/// build's is a newer build's; one before the first, no code's.
fn head(r: &mut In) -> Result<(u32, Kind), &'static str> {
    let h = r.uv()?;
    let version = u32::try_from(h / 2).map_err(|_| NEEDS_NEWER)?;
    if version > data::CODE_NEWEST {
        return Err(NEEDS_NEWER);
    }
    if version < FIRST_VERSION {
        return Err(NOT_A_CODE);
    }
    Ok((version, if h % 2 == 0 { Kind::Loop } else { Kind::Album }))
}

/// What kind of code `code` is, read from its head alone (no check made):
/// `None` for text that is no code's head, or a newer build's.
#[must_use]
pub fn kind(code: &[u8]) -> Option<Kind> {
    let start = code.iter().position(|b| !b.is_ascii_whitespace())?;
    head(&mut In { chars: &code[start..], at: 0 }).ok().map(|h| h.1)
}

/// The fields a body may hold.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Fields {
    /// A loop's: everything.
    Loop,
    /// An album's track: a loop's, and its own version first.
    Track,
    /// An album from one seed: the picks the tracks share, and the album's
    /// name.
    Picks,
}

/// A track's own pace as its body holds it: its number, and where its
/// field is in the body's characters (from its tag to its end).
#[derive(Clone, Copy)]
struct OwnPace {
    number: u32,
    from: usize,
    to: usize,
}

/// A loop's seed and fields, read from `r` up to its end, at `version`
/// (a track's own first field may change it), and a track's own pace.
#[allow(clippy::too_many_lines)] // one field after another, in their order
fn read_body(r: &mut In, version: u32, fields: Fields) -> Result<(Loop, Option<OwnPace>), &'static str> {
    let seed = r.seed()?;
    let mut version = version;
    let mut req = Request::new();
    let (mut mutes, mut salt, mut name) = (0u8, 0u32, Name::NONE);
    // Each tag once, ascending; a keyed one (a vibe, a layer's voices, a
    // pin, a layer's seed, a tone) may repeat, its key ascending.
    let mut last: Option<(u64, u64)> = None;
    let mut ids = [0u32; 64];
    let mut first = true;
    let mut pace = None;
    while !r.done() {
        let from = r.at;
        let t = r.uv()?;
        // A field this build does not know is a newer build's.
        if t > tag::PACE && t != tag::NAME {
            return Err(NEEDS_NEWER);
        }
        // The key read with the tag, to hold the order.
        let (k, keyed) = match t {
            tag::VIBE | tag::VIBE_WEIGHTED | tag::VOICES | tag::CHOICE | tag::SETTING | tag::SEED | tag::TONE => (r.uv()?, true),
            _ => (0, false),
        };
        match last {
            Some((lt, lk)) if t < lt || (t == lt && (!keyed || k <= lk)) => return Err(NOT_A_CODE),
            _ => {}
        }
        last = Some((t, k));
        let picks = t != tag::VERSION && t <= tag::SETTING;
        let allowed = match fields {
            Fields::Loop => t != tag::VERSION && t != tag::PACE,
            Fields::Track => t != tag::VERSION || first,
            Fields::Picks => picks || t == tag::NAME,
        };
        first = false;
        if !allowed {
            return Err(NOT_A_CODE);
        }
        match t {
            tag::VERSION => {
                let v = u32::try_from(r.uv()?).map_err(|_| NEEDS_NEWER)?;
                if v > data::CODE_NEWEST {
                    return Err(NEEDS_NEWER);
                }
                if v < FIRST_VERSION || v == version {
                    return Err(NOT_A_CODE);
                }
                version = v;
            }
            tag::VIBE | tag::VIBE_WEIGHTED => {
                let id = u32::try_from(k).map_err(|_| NEEDS_NEWER)?;
                let i = data::vibe_index(name_of(data::CODE_VIBES, id, version)?).ok_or(NEEDS_NEWER)?;
                // A vibe twice (with a weight and without) is no code's.
                if req.vibe(i).is_some() {
                    return Err(NOT_A_CODE);
                }
                req = if t == tag::VIBE_WEIGHTED { met(req.with_weight(i, r.dec()?))? } else { req.with(i) };
            }
            tag::TEMPO | tag::TEMPO_RANGE => {
                let lo = r.below(200)?;
                let hi = if t == tag::TEMPO { lo } else { lo + 1 + r.below(200)? };
                #[allow(clippy::cast_precision_loss)] // below 400
                let (lo, hi) = (lo as f64 + request::TEMPO_LIMIT[0], hi as f64 + request::TEMPO_LIMIT[0]);
                req = met(req.with_tempo(lo, hi))?;
            }
            tag::LENGTH | tag::ENDLESS => {
                if req.length().is_some() {
                    return Err(NOT_A_CODE);
                }
                let length = if t == tag::ENDLESS {
                    Length::Endless
                } else {
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // LENGTH_MOST is 2^53 - 1
                    let most = LENGTH_MOST as u64;
                    #[allow(clippy::cast_precision_loss)] // below 2^53: exact
                    let bars = r.below(most)? as f64 + 1.0;
                    Length::Bars(bars)
                };
                req = met(req.with_length(length))?;
            }
            tag::JOURNEY => {
                req = req.with_journey(match name_of(data::CODE_JOURNEYS, r.id()?, version)? {
                    "arc" => Journey::Arc,
                    "wander" => Journey::Wander,
                    _ => return Err(NEEDS_NEWER),
                });
            }
            tag::KEYS => {
                let n = r.set(&mut ids)?;
                let mut keys = 0u32;
                for &note in &ids[..n] {
                    if note > 11 {
                        return Err(NOT_A_CODE);
                    }
                    keys |= 1 << note;
                }
                req = met(req.with_keys(keys))?;
            }
            tag::SCALES | tag::STYLES => {
                let n = r.set(&mut ids)?;
                let mut bits = 0u32;
                for &id in &ids[..n] {
                    bits |= 1 << if t == tag::SCALES {
                        let name = name_of(data::CODE_SCALES, id, version)?;
                        SCALES.iter().position(|s| s.id == name).ok_or(NEEDS_NEWER)?
                    } else {
                        place(&PROFILE_ORDER, name_of(data::CODE_PROFILES, id, version)?)?
                    };
                }
                req = met(if t == tag::SCALES { req.with_scales(bits) } else { req.with_styles(bits) })?;
            }
            tag::VOICES => {
                let layer = layer_of(u32::try_from(k).map_err(|_| NEEDS_NEWER)?, version)?;
                let n = r.set(&mut ids)?;
                let mut bits = 0u32;
                for &id in &ids[..n] {
                    bits |= 1 << place(VOICE_ORDER[layer], name_of(data::CODE_VOICES[layer], id, version)?)?;
                }
                req = met(req.with_voices(layer, bits))?;
            }
            tag::LAYERS_ON | tag::LAYERS_OFF | tag::MUTES => {
                let n = r.set(&mut ids)?;
                let mut bits = 0u8;
                for &id in &ids[..n] {
                    bits |= 1 << layer_of(id, version)?;
                }
                if t == tag::MUTES {
                    mutes = bits;
                } else {
                    let ask = if t == tag::LAYERS_ON { LayerAsk::On } else { LayerAsk::Off };
                    for layer in 0..LAYER_NAMES.len() {
                        if bits & (1 << layer) != 0 {
                            // A layer both on and off is no code's.
                            if req.layer(layer) != LayerAsk::Chance {
                                return Err(NOT_A_CODE);
                            }
                            req = met(req.with_layer(layer, ask))?;
                        }
                    }
                }
            }
            tag::THEME => req = req.with_theme(r.seed()?),
            tag::CHOICE => {
                let name = name_of(data::CODE_CHOICES, u32::try_from(k).map_err(|_| NEEDS_NEWER)?, version)?;
                let number = CHOICES.iter().position(|c| c.id == name).ok_or(NEEDS_NEWER)?;
                if number >= request::CHOICE_PINS {
                    return Err(NEEDS_NEWER);
                }
                let option = name_of(options_of(name), r.id()?, version)?;
                let o = CHOICES[number].options.iter().position(|x| x.0 == option).ok_or(NEEDS_NEWER)?;
                req = met(req.with_pin(number, crate::generator::js::num(o)))?;
            }
            tag::SETTING => {
                let name = name_of(data::CODE_SETTINGS, u32::try_from(k).map_err(|_| NEEDS_NEWER)?, version)?;
                let number = Pins::number(name).ok_or(NEEDS_NEWER)?;
                let value = match Pins::words(number) {
                    Some(words) => {
                        let word = name_of(words_of(name), r.id()?, version)?;
                        crate::generator::js::num(words.iter().position(|x| x.0 == word).ok_or(NEEDS_NEWER)?)
                    }
                    None => r.dec()?,
                };
                req = met(req.with_pin(number, value))?;
            }
            tag::SEED => {
                let layer = layer_of(u32::try_from(k / 2).map_err(|_| NEEDS_NEWER)?, version)?;
                if req.edits().seed(layer).is_some() {
                    return Err(NOT_A_CODE);
                }
                req = met(req.with_layer_seed(layer, r.seed()?, k % 2 == 1))?;
            }
            tag::TONE => {
                let name = name_of(data::CODE_TONES, u32::try_from(k).map_err(|_| NEEDS_NEWER)?, version)?;
                req = met(req.with_tone(place(&TONE_NAMES, name)?, r.dec()?))?;
            }
            tag::BPM => req = met(req.with_bpm(r.dec()?))?,
            tag::SALT => {
                salt = r.seed()?;
                if salt == 0 {
                    return Err(NOT_A_CODE);
                }
            }
            tag::PACE => {
                #[allow(clippy::cast_possible_truncation)] // below 2^32
                let number = r.below(PACE_NUMBERS)? as u32;
                pace = Some(OwnPace { number, from, to: r.at });
            }
            tag::NAME => name = Name::read(r)?,
            _ => return Err(NEEDS_NEWER),
        }
    }
    let request = met(req.with_version(version))?;
    Ok((Loop { seed, request, mutes, salt, name }, pace))
}

/// Read a loop's code (spaces around it are let be).
///
/// # Errors
/// `NEEDS_NEWER` for a code from a newer build, `NOT_A_CODE` for text that
/// is no loop's code (an album's among it).
pub fn read_loop(code: &[u8]) -> Result<Loop, &'static str> {
    let chars = checked(code, LOOP_MOST)?;
    let mut r = In { chars, at: 0 };
    let (version, kind) = head(&mut r)?;
    if kind != Kind::Loop {
        return Err(NOT_A_CODE);
    }
    read_body(&mut r, version, Fields::Loop).map(|b| b.0)
}

// ------------------------------------------------------------------- albums

/// An album as its code holds it: how many tracks, its version, the seed it
/// is drawn from (an album from one seed) or none (a list of loops), and its
/// name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Album {
    /// How many tracks.
    pub count: usize,
    /// The album's version: its newest track's.
    pub version: u32,
    /// The seed it is drawn from, or `None` for a list of loops.
    pub seed: Option<u32>,
    /// Its name, or `Name::NONE`.
    pub name: Name,
}

/// The seed of track `index` (from 0) of an album drawn from `seed`: the
/// generator's own stream of `seed` (mulberry32, `Rng`), its `index + 1`th
/// fresh seed.
#[must_use]
pub fn track_seed(seed: u32, index: usize) -> u32 {
    let mut r = Rng::new(seed);
    let mut s = r.seed32();
    for _ in 0..index {
        s = r.seed32();
    }
    s
}

/// Where an album's tracks are: each its characters (a track's body).
struct Tracks<'a> {
    album: Album,
    /// From one seed: the picks the tracks share.
    picks: Option<Request>,
    /// A list: the characters from the first track's length on.
    list: &'a [u8],
    /// A list: the pace its tracks share (its number; 0 on and on), each
    /// track's but those that write their own.
    shared: u32,
}

/// Read an album's head and walk its tracks once (each checked, so a bad
/// track refuses the album).
fn album_of(code: &[u8]) -> Result<Tracks<'_>, &'static str> {
    let chars = checked(code, usize::MAX)?;
    let mut r = In { chars, at: 0 };
    let (version, kind) = head(&mut r)?;
    if kind != Kind::Album {
        return Err(NOT_A_CODE);
    }
    match r.uv()? {
        0 => {
            let count = usize::try_from(r.below(ALBUM_MOST as u64 + 1)?).map_err(|_| NOT_A_CODE)?;
            if count == 0 {
                return Err(NOT_A_CODE);
            }
            let start = r.at;
            let mut newest = FIRST_VERSION;
            // Each track's own pace, or none (`u32::MAX`, past any number).
            let mut paces = [u32::MAX; ALBUM_MOST];
            for own in paces.iter_mut().take(count) {
                let len = usize::try_from(r.below(LOOP_MOST as u64)?).map_err(|_| NOT_A_CODE)?;
                let end = r.at.checked_add(len).filter(|&e| e <= chars.len()).ok_or(NOT_A_CODE)?;
                let mut t = In { chars: &chars[..end], at: r.at };
                let (l, pace) = read_body(&mut t, version, Fields::Track)?;
                // A track's own version is older than the album's (the
                // album's is its newest).
                if l.request.version() > version {
                    return Err(NOT_A_CODE);
                }
                newest = newest.max(l.request.version());
                if let Some(p) = pace {
                    *own = p.number;
                }
                r.at = end;
            }
            if newest != version {
                return Err(NOT_A_CODE);
            }
            // The album's fields: the pace its tracks share, then its name,
            // each at most once.
            let (mut shared, mut name) = (0, Name::NONE);
            let mut last = None;
            while !r.done() {
                let t = r.uv()?;
                if t > tag::PACE && t != tag::NAME {
                    return Err(NEEDS_NEWER);
                }
                if last.is_some_and(|l| t <= l) {
                    return Err(NOT_A_CODE);
                }
                last = Some(t);
                match t {
                    tag::PACE => {
                        #[allow(clippy::cast_possible_truncation)] // below 2^32
                        let p = r.below(PACE_NUMBERS)? as u32;
                        // On and on is no field's: an album without one.
                        if p == 0 {
                            return Err(NOT_A_CODE);
                        }
                        shared = p;
                    }
                    tag::NAME => name = Name::read(&mut r)?,
                    _ => return Err(NOT_A_CODE),
                }
            }
            // One code for each album: a track writes its own pace only
            // where it is not the album's, and the album's is the one most
            // of its tracks share (the smaller on a tie), as `write_album`
            // writes it.
            for p in paces.iter_mut().take(count) {
                if *p == shared {
                    return Err(NOT_A_CODE);
                }
                if *p == u32::MAX {
                    *p = shared;
                }
            }
            if most_common(&mut paces[..count]) != shared {
                return Err(NOT_A_CODE);
            }
            Ok(Tracks { album: Album { count, version, seed: None, name }, picks: None, list: &chars[start..], shared })
        }
        1 => {
            let seed = r.seed()?;
            let count = usize::try_from(r.below(ALBUM_MOST as u64 + 1)?).map_err(|_| NOT_A_CODE)?;
            if count == 0 {
                return Err(NOT_A_CODE);
            }
            // The picks read as a body whose seed is the album's.
            let mut picks = [0u8; LOOP_MOST + 6];
            let rest = &chars[r.at..];
            let room = picks.get_mut(..rest.len() + 6).ok_or(NOT_A_CODE)?;
            room[..6].fill(b'A');
            room[6..].copy_from_slice(rest);
            let (l, _) = read_body(&mut In { chars: room, at: 0 }, version, Fields::Picks)?;
            Ok(Tracks { album: Album { count, version, seed: Some(seed), name: l.name }, picks: Some(l.request), list: &[], shared: 0 })
        }
        _ => Err(NEEDS_NEWER),
    }
}

/// Read an album's code: how many tracks, its version, its seed (an album
/// from one seed) and its name. Every track is checked.
///
/// # Errors
/// `NEEDS_NEWER` for a newer build's album, `NOT_A_CODE` for text that is
/// no album's code.
pub fn read_album(code: &[u8]) -> Result<Album, &'static str> {
    album_of(code).map(|t| t.album)
}

/// Track `index`'s body in a list album `t` (its seed and fields, its own
/// version and pace among them), and its own pace.
fn body_of<'a>(t: &Tracks<'a>, index: usize) -> Result<(&'a [u8], Option<OwnPace>), &'static str> {
    let mut r = In { chars: t.list, at: 0 };
    for _ in 0..index {
        let len = usize::try_from(r.uv()?).map_err(|_| NOT_A_CODE)?;
        r.at += len;
    }
    let len = usize::try_from(r.uv()?).map_err(|_| NOT_A_CODE)?;
    let body = t.list.get(r.at..r.at + len).ok_or(NOT_A_CODE)?;
    let (_, pace) = read_body(&mut In { chars: body, at: 0 }, t.album.version, Fields::Track)?;
    Ok((body, pace))
}

/// Write track `index` (from 0) of the album `code` into `out` as its own
/// loop's code: its length. Its pace is not in it (a loop's code carries
/// none: `album_pace` reads it).
///
/// # Errors
/// As `read_album`; `BAD_ALBUM` for an index past the last track, `NO_ROOM`
/// if the code does not fit.
pub fn album_track(code: &[u8], index: usize, out: &mut [u8]) -> Result<usize, &'static str> {
    let t = album_of(code)?;
    if index >= t.album.count {
        return Err(BAD_ALBUM);
    }
    if let (Some(seed), Some(picks)) = (t.album.seed, t.picks) {
        return write_loop(&Loop { request: picks, ..Loop::of(track_seed(seed, index)) }, out);
    }
    let (body, pace) = body_of(&t, index)?;
    // The track's own version, where it writes one, comes off its body, and
    // its own pace.
    let mut b = In { chars: body, at: 6 };
    let mut version = t.album.version;
    let mut from = 6;
    if !b.done() && b.uv()? == tag::VERSION {
        version = u32::try_from(b.uv()?).map_err(|_| NOT_A_CODE)?;
        from = b.at;
    }
    let (cut, to) = pace.map_or((body.len(), body.len()), |p| (p.from, p.to));
    let mut w = Out::new(out);
    w.uv(u64::from(version) * 2);
    for &c in body[..6].iter().chain(&body[from..cut]).chain(&body[to..]) {
        w.put(sextet(c).unwrap_or(0));
    }
    w.check()
}

/// How long track `index` (from 0) of the album `code` plays before the
/// next, as its maker set it: its own pace, or the one its album's tracks
/// share. An album from one seed sets none.
///
/// # Errors
/// As `album_track`.
pub fn album_pace(code: &[u8], index: usize) -> Result<Pace, &'static str> {
    let t = album_of(code)?;
    if index >= t.album.count {
        return Err(BAD_ALBUM);
    }
    if t.album.seed.is_some() {
        return Ok(Pace::On);
    }
    let (_, pace) = body_of(&t, index)?;
    Ok(Pace::from_number(pace.map_or(t.shared, |p| p.number)))
}

/// Write the album of the loops' codes `tracks`, each with its pace (how
/// long it plays before the next), in their order, named `name`
/// (`Name::NONE` for none), into `out`: its length. Each track keeps its
/// own version, and its own name; the album's version is its newest
/// track's. The pace most tracks share is written once, for the album (none
/// where most play on and on, so an album with no pace set costs nothing),
/// and a track whose pace is another writes its own.
///
/// # Errors
/// What `read_loop` says of a track, `BAD_ALBUM` for no tracks or more than
/// `ALBUM_MOST`, `BAD_PACE` for a pace of no count or past `PACE_MOST`,
/// `NO_ROOM` if the album does not fit.
pub fn write_album<'a>(tracks: impl Iterator<Item = (&'a [u8], Pace)> + Clone, name: &Name, out: &mut [u8]) -> Result<usize, &'static str> {
    let mut count = 0;
    let mut version = FIRST_VERSION;
    let mut paces = [0u32; ALBUM_MOST];
    for (t, pace) in tracks.clone() {
        let l = read_loop(t)?;
        version = version.max(l.request.version());
        let p = pace.number().ok_or(BAD_PACE)?;
        if let Some(slot) = paces.get_mut(count) {
            *slot = p;
        }
        count += 1;
    }
    if count == 0 || count > ALBUM_MOST {
        return Err(BAD_ALBUM);
    }
    let shared = most_common(&mut paces[..count]);
    let mut w = Out::new(out);
    w.uv(u64::from(version) * 2 + 1);
    w.uv(0);
    w.uv(count as u64);
    let mut scratch = [0u8; LOOP_MOST];
    for (t, pace) in tracks {
        let l = read_loop(t)?;
        let own = l.request.version();
        let number = pace.number().ok_or(BAD_PACE)?;
        let mut b = Out::new(&mut scratch);
        body(&mut b, &l, own, (own != version).then_some(own), false, (number != shared).then_some(number))?;
        let n = b.len;
        if b.full {
            return Err(NO_ROOM);
        }
        w.uv(n as u64);
        for &c in &scratch[..n] {
            w.put(sextet(c).unwrap_or(0));
        }
    }
    if shared != 0 {
        w.uv(tag::PACE);
        w.uv(u64::from(shared));
    }
    if !name.is_none() {
        w.uv(tag::NAME);
        name.write(&mut w);
    }
    w.check()
}

/// Write the album of `count` tracks drawn from `seed` (`track_seed`), each
/// asked `picks` (its edits left out: an album's tracks share picks, not a
/// loop's edits), named `name`, into `out`: its length.
///
/// # Errors
/// `BAD_ALBUM` for no tracks or more than `ALBUM_MOST`, `NOT_AT_VERSION` as
/// `write_loop`, `NO_ROOM` if it does not fit.
pub fn write_seed_album(seed: u32, count: usize, picks: &Request, name: &Name, out: &mut [u8]) -> Result<usize, &'static str> {
    if count == 0 || count > ALBUM_MOST {
        return Err(BAD_ALBUM);
    }
    let picks = picks.asked();
    let version = version_of(&picks);
    let mut w = Out::new(out);
    w.uv(u64::from(version) * 2 + 1);
    w.uv(1);
    w.seed(seed);
    w.uv(count as u64);
    // The picks as a body less its seed: written after a seed of nothing,
    // which is then dropped.
    let mut scratch = [0u8; LOOP_MOST];
    let mut b = Out::new(&mut scratch);
    body(&mut b, &Loop { request: picks, ..Loop::of(0) }, version, None, true, None)?;
    if b.full {
        return Err(NO_ROOM);
    }
    let n = b.len;
    for &c in &scratch[6..n] {
        w.put(sextet(c).unwrap_or(0));
    }
    if !name.is_none() {
        w.uv(tag::NAME);
        name.write(&mut w);
    }
    w.check()
}

// ------------------------------------------------------------------- as text

/// The names of a set's bits, `|` between (`request_text`).
fn names<W: core::fmt::Write>(w: &mut W, bits: u32, names: impl Iterator<Item = &'static str>) -> core::fmt::Result {
    let mut first = true;
    for (i, n) in names.enumerate() {
        if i < 32 && bits & (1 << i) != 0 {
            if !first {
                w.write_char('|')?;
            }
            w.write_str(n)?;
            first = false;
        }
    }
    Ok(())
}

/// `r` as the text `Request::parse` reads (its asks, then its edits, a comma
/// between each, and its version where it has one): what a host that keeps
/// a loop as a record of its request reads back from a code (`dl_code_request`,
/// the app's js/share.js), so the record says what was asked, in the
/// request's own words. Every request reads back to itself
/// (`tests/codes.rs`). Nothing asked is "".
///
/// # Errors
/// The writer's.
pub fn request_text<W: core::fmt::Write>(r: &Request, w: &mut W) -> core::fmt::Result {
    use crate::generator::longform::NOTE_NAMES;
    let mut first = true;
    let mut item = |w: &mut W, args: core::fmt::Arguments| -> core::fmt::Result {
        if !first {
            w.write_char(',')?;
        }
        first = false;
        w.write_fmt(args)
    };
    if r.version() != 0 {
        item(w, format_args!("version={}", r.version()))?;
    }
    for (i, v) in VIBES.iter().enumerate() {
        match r.vibe(i) {
            Some(Some(weight)) => item(w, format_args!("{}={weight}", v.id))?,
            Some(None) => item(w, format_args!("{}", v.id))?,
            None => {}
        }
    }
    if let Some([lo, hi]) = r.tempo() {
        #[allow(clippy::float_cmp)] // whole bpm: a tempo pinned is one number
        let pinned = lo == hi;
        if pinned {
            item(w, format_args!("bpm={lo}"))?;
        } else {
            item(w, format_args!("bpm={lo}-{hi}"))?;
        }
    }
    match r.length() {
        Some(Length::Bars(bars)) => item(w, format_args!("bars={bars}"))?,
        Some(Length::Endless) => item(w, format_args!("bars=endless"))?,
        None => {}
    }
    if let Some(j) = r.journey() {
        item(w, format_args!("journey={}", j.name()))?;
    }
    if r.keys() != 0 {
        item(w, format_args!("key="))?;
        names(w, r.keys(), NOTE_NAMES.iter().copied())?;
    }
    if r.scales() != 0 {
        item(w, format_args!("scale="))?;
        names(w, r.scales(), SCALES.iter().map(|s| s.id))?;
    }
    if r.styles() != 0 {
        item(w, format_args!("style="))?;
        names(w, r.styles(), PROFILE_ORDER.iter().copied())?;
    }
    for (layer, name) in LAYER_NAMES.iter().enumerate() {
        if r.voices(layer) != 0 {
            item(w, format_args!("voice.{name}="))?;
            names(w, r.voices(layer), VOICE_ORDER[layer].iter().copied())?;
        }
    }
    for (layer, name) in LAYER_NAMES.iter().enumerate() {
        match r.layer(layer) {
            LayerAsk::On => item(w, format_args!("layer.{name}=on"))?,
            LayerAsk::Off => item(w, format_args!("layer.{name}=off"))?,
            LayerAsk::Chance => {}
        }
    }
    if let Some(theme) = r.theme() {
        item(w, format_args!("theme={theme}"))?;
    }
    let p = r.pins();
    for number in 0..request::PINS_MAX {
        let (Some(x), Some((prefix, name))) = (p.at(number), Pins::name(number)) else { continue };
        match Pins::words(number) {
            Some(words) => item(w, format_args!("{prefix}{name}={}", words.get(crate::generator::js::index(x)).map_or("", |e| e.0)))?,
            None => item(w, format_args!("{prefix}{name}={x}"))?,
        }
    }
    let e = r.edits();
    for (layer, name) in LAYER_NAMES.iter().enumerate() {
        if let Some(seed) = e.seed(layer) {
            let kind = if e.rolled() & (1 << layer) != 0 { "roll" } else { "seed" };
            item(w, format_args!("edit.{kind}.{name}={seed}"))?;
        }
    }
    for (k, name) in TONE_NAMES.iter().enumerate() {
        if let Some(x) = e.tone(k) {
            item(w, format_args!("edit.{name}={x}"))?;
        }
    }
    if let Some(bpm) = e.bpm() {
        item(w, format_args!("edit.bpm={bpm}"))?;
    }
    Ok(())
}
