//! A long piece's plan (queue item 102; `docs/LONGFORM.md` is the design):
//! what each section of a piece past 32 bars, or of Endless, is, as a pure
//! function of the loop's seed, its request and the section's number.
//!
//! **Two journeys.** An *arc*, as a symphony's: roles in order (an opening,
//! theme A, a build, a second theme in a related key, a development, a peak
//! near two thirds, a release, the return of A changed and in the home key,
//! a close), one peak of energy, and for a finite piece of several arcs the
//! arcs as movements, the last looking back at the others' second themes.
//! And the *wander*: long holds, one thing moving at each hold's end (a
//! layer resting or returning, a layer's voice, the warmth, the space, the
//! chords' register and length, the tune's density, rarely the key or the
//! theme). A piece draws its journey from its seed, leaned by its vibes, or
//! is asked one (`journey=arc`, `journey=wander`).
//!
//! **Closed form.** Nothing here walks earlier sections. A unit of the plan
//! finds its arc by a division and its role by a handful of shares; a
//! wander's unit finds its epoch by a division (an epoch is each thing's
//! turn once, in an order drawn from the epoch's number) and its state by
//! looking at that epoch and the one before. Every draw is on a stream of
//! its own, salted by the arc's or the epoch's number, so a section costs
//! the same at number 1 and at 10^9 (promise 11), and the plan allocates
//! nothing and keeps nothing: a `Form` is a few numbers made from the spec.
//!
//! **Time, not sections.** A section is the loop's own 2 to 32 bars, eight
//! seconds or two and a half minutes, so the spans are set in time (an
//! arc's minutes, a hold's seconds) and turned into units at the loop's
//! tempo. The unit is the section; where a piece has too few sections for
//! its form it is the phrase line inside a section (`Grain`), found by the
//! same arithmetic from the section's number and the phrase's.
//!
//! **What it never reads, and never moves.** The plan is made from the
//! spec alone: never from the wander slider's amount, which is only how
//! much a section breathes inside its role. What was asked holds: a pinned
//! key is the home key (an arc visits related keys and comes home; a
//! wander asked a key never leaves it), a pinned scale rules out the
//! parallel mode, a layer asked on or off or re-rolled never rests, a
//! layer's voice asked is never handed on, a tone edited is never moved,
//! and the tempo is the loop's own throughout.
//!
//! **Heard** (`heard`): a wander's section plays its units' states as marks
//! and levels on the loop's wandered pass (session 12), and an arc's its
//! units' energy, with a finite arc's landing on the tonic (session 13),
//! and its themes, their variations and its keys (session 14).
//!
//! **Movements of different kinds** (session 14). A piece of three arcs or
//! more, and Endless, plays its movements in a repeating cycle drawn per
//! piece (`Kind`): a full arc, then a gentle one, a light one and (four or
//! more) an interlude in the wander's way, in a drawn order, each a share
//! of the full arc's length. A finite piece is whole cycles and a last full
//! arc, so its first and its last are full and the last alone closes; a
//! unit finds its movement by a division still.
//!
//! The numbers are `data/longform.txt`'s (`data::LONGFORM`).

// Units, sections and bars are whole numbers far below 2^53, and a share of
// them is rounded on purpose.
#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]

use super::canon::{Sink, Writer};
use super::choices::lean_of;
use super::data::{self, CHARACTERS, LONGFORM, Table};
use super::js;
use super::piece::{self, Piece, SECTION_BARS};
use super::request::{Edits, Journey, Length};
use super::rng::Rng;
use super::spec::Spec;
use super::theory::SCALES;
use super::{LAYERS, Layer};
use crate::math;

const JOURNEY_SALT: u32 = 0x4a52_4e59;
const MINUTES_SALT: u32 = 0x4d49_4e53;
const SECOND_SALT: u32 = 0x3254_484d;
const PARITY_SALT: u32 = 0x5041_5254;
const ARC_SALT: u32 = 0x4152_4353;
const THEME_SALT: u32 = 0x4254_484d;
const EPOCH_SALT: u32 = 0x4550_4f43;
const LAST_SALT: u32 = 0x4c41_5354;
const VALUE_SALT: u32 = 0x5641_4c00;
const KIND_SALT: u32 = 0x4b49_4e44;
const CALL_SALT: u32 = 0x4341_4c4c;
const INTERLUDE_SALT: u32 = 0x494e_544c;

/// A phrase, in bars, where four divide a section (`player::change`'s
/// `PHRASE_BARS`: the player's phrase lines are the plan's).
const PHRASE_BARS: f64 = 4.0;
/// The fewest units a full arc is laid on: one a role.
const FULL_UNITS: u64 = 9;
/// The most roles a form has.
const ROLES_MAX: usize = 9;
/// The most turns an epoch of the wander has: every thing once, the layers
/// twice (one rests, and returns).
const TURNS_MAX: usize = 10;

/// The most places a cycle of movements has: a full arc and the three
/// other kinds.
const CYCLE_MAX: usize = 4;

/// What kind a movement is (session 14; Mikey, 2026-11-12: a long piece's
/// movements should differ in kind, as a symphony's do).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// The full form: an opening to a close, with a second theme and keys.
    Full,
    /// The full form with a low peak: no build to speak of.
    Gentle,
    /// The small form: one theme, a lift, the home key.
    Light,
    /// The wander's way for a span: one thing moving at a time, each home
    /// again before the span ends.
    Interlude,
}

impl Kind {
    /// The kind's name, as the data and the map write it.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Kind::Full => "full",
            Kind::Gentle => "gentle",
            Kind::Light => "light",
            Kind::Interlude => "interlude",
        }
    }

    /// The fewest units a movement of this kind is laid on: a unit a role,
    /// or an interlude's three holds.
    const fn least(self) -> u64 {
        match self {
            Kind::Full | Kind::Gentle => FULL_UNITS,
            Kind::Light => 6,
            Kind::Interlude => 3,
        }
    }

    /// Whether a movement of this kind brings a second theme (where the
    /// piece has them).
    const fn bears(self) -> bool {
        matches!(self, Kind::Full | Kind::Gentle)
    }

    /// The kind's length, a share of a full arc's in thousandths.
    fn share(self) -> u64 {
        (js::round(1000.0 * data::lookup(LONGFORM.kinds, self.name()).unwrap_or(1.0)) as u64).max(1)
    }
}

/// A stream of the piece's own for the draw `salt` names, numbered
/// `number` (an arc's, an epoch's): the number is folded into the seed by
/// a 64-bit mix, so neighboring numbers share nothing.
fn stream(seed: u32, salt: u32, number: u64) -> Rng {
    let mut x = number ^ (u64::from(seed ^ salt) << 32) ^ u64::from(salt);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^= x >> 31;
    Rng::new((x ^ (x >> 32)) as u32)
}

/// What the plan's unit is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Grain {
    /// A section: the loop's own bars.
    Section,
    /// A phrase line inside a section, where the piece has too few sections
    /// for its form.
    Phrase,
}

impl Grain {
    /// The grain's name, as the map writes it.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Grain::Section => "section",
            Grain::Phrase => "phrase",
        }
    }
}

/// What a unit is for: an arc's roles in the full form's order, the breath
/// between arcs, the small form's lift, and the wander's two.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    /// Thin, theme A hinted.
    Opening,
    /// Theme A.
    Theme,
    /// Theme A, rising.
    Build,
    /// The second theme, in a related key.
    Second,
    /// Fragments of both, through the related keys.
    Development,
    /// The arc's height.
    Peak,
    /// The fall from it.
    Release,
    /// Theme A again, changed, in the home key.
    Return,
    /// A finite piece's end: thin, on the tonic, ringing out.
    Close,
    /// Between two arcs: no theme, a rest before the next opening.
    Breath,
    /// The small form's peak: theme A lifted.
    Lift,
    /// A wander's held state.
    Holding,
    /// The first unit after a wander's move: the thing still arriving.
    Turning,
}

impl Role {
    /// The role's name, as the data and the map write it.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Role::Opening => "opening",
            Role::Theme => "theme",
            Role::Build => "build",
            Role::Second => "second",
            Role::Development => "development",
            Role::Peak => "peak",
            Role::Release => "release",
            Role::Return => "return",
            Role::Close => "close",
            Role::Breath => "breath",
            Role::Lift => "lift",
            Role::Holding => "holding",
            Role::Turning => "turning",
        }
    }

    fn named(name: &str) -> Role {
        const ALL: [Role; 11] =
            [Role::Opening, Role::Theme, Role::Build, Role::Second, Role::Development, Role::Peak, Role::Release, Role::Return, Role::Close, Role::Breath, Role::Lift];
        // `build.rs` holds the data to these names.
        ALL.into_iter().find(|r| r.name() == name).unwrap_or(Role::Theme)
    }
}

/// Whose tune a unit plays.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    /// None: the tune rests.
    Rest,
    /// The loop's own tune.
    A,
    /// The second theme of this arc (or of the wander): another seed's tune
    /// bent into this loop (`Span::theme_seed`).
    B,
    /// The second theme of an earlier arc, by its number: the last arc of
    /// a piece in movements looks back.
    Recall(u64),
}

/// How a theme is changed where it comes again. The plan says which; what
/// each does to the notes is `heard`'s (session 14).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Variation {
    /// As first stated.
    Plain,
    /// In the related key.
    Key,
    /// In another register.
    Register,
    /// Handed to another voice of the profile.
    Voice,
    /// Stretched to twice its length over held chords.
    Stretched,
    /// The same tune over the relative key's chords.
    Reharmonized,
    /// Its first half kept, its second drawn anew.
    Answered,
    /// Thinned to its outline.
    Outline,
}

impl Variation {
    /// The variation's name, as the data and the map write it.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Variation::Plain => "plain",
            Variation::Key => "key",
            Variation::Register => "register",
            Variation::Voice => "voice",
            Variation::Stretched => "stretched",
            Variation::Reharmonized => "reharmonized",
            Variation::Answered => "answered",
            Variation::Outline => "outline",
        }
    }

    fn named(name: &str) -> Variation {
        const ALL: [Variation; 8] = [
            Variation::Plain,
            Variation::Key,
            Variation::Register,
            Variation::Voice,
            Variation::Stretched,
            Variation::Reharmonized,
            Variation::Answered,
            Variation::Outline,
        ];
        ALL.into_iter().find(|v| v.name() == name).unwrap_or(Variation::Plain)
    }
}

/// How a key stands to the home key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Relation {
    /// The loop's own key.
    Home,
    /// The same mode a fifth above.
    FifthAbove,
    /// The same mode a fifth below.
    FifthBelow,
    /// The mode of the same notes with the other third (A aeolian's C
    /// ionian).
    Relative,
    /// Another mode of the profile's on the same home note, with the other
    /// third.
    Parallel,
}

impl Relation {
    /// Whether the key is the home key's mode a fifth away: the loop moved
    /// whole, as the wander moves it.
    #[must_use]
    pub const fn is_fifth(self) -> bool {
        matches!(self, Relation::FifthAbove | Relation::FifthBelow)
    }

    /// How far the loop moves, in semitones, to play in a key of this
    /// relation, the nearer way: a fifth above is a fourth down, a fifth
    /// below a fourth up (0 for any other: they keep the notes or the home
    /// note).
    #[must_use]
    pub const fn shift(self) -> i8 {
        match self {
            Relation::FifthAbove => -5,
            Relation::FifthBelow => 5,
            _ => 0,
        }
    }

    /// The relation's name, as the map writes it.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Relation::Home => "home",
            Relation::FifthAbove => "fifthAbove",
            Relation::FifthBelow => "fifthBelow",
            Relation::Relative => "relative",
            Relation::Parallel => "parallel",
        }
    }
}

/// A key: its home note (a pitch class, C 0), its mode (`theory::SCALES`)
/// and how it stands to the piece's home key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Key {
    pub root: u8,
    pub scale: usize,
    pub relation: Relation,
}

impl Key {
    /// The key's notes, a bit a pitch class.
    fn notes(self) -> u16 {
        SCALES.get(self.scale).map_or(0, |s| s.steps.iter().fold(0, |m, &step| m | 1 << ((usize::from(self.root) + step as usize) % 12)))
    }
}

/// The note names, C first, as the request reads a key.
pub const NOTE_NAMES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

/// What a wander has moved, each as it stands in a unit: all the loop's own
/// at a piece's start and through every arc.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct State {
    /// The layer resting.
    pub rest: Option<Layer>,
    /// The layer playing another voice of its pool, and which: a number
    /// from 0 to 1 over the pool's other voices.
    pub voice: Option<(Layer, f64)>,
    /// The warmth and the space, each from -1 to 1 around the loop's own.
    pub warmth: f64,
    pub space: f64,
    /// The chords' register (an octave down, their own, an octave up) and
    /// their length (shorter, their own, longer).
    pub register: i8,
    pub length: i8,
    /// The share of the tune's notes that sound: 1 its own.
    pub density: f64,
}

impl State {
    /// Everything the loop's own.
    pub const OWN: State = State { rest: None, voice: None, warmth: 0.0, space: 0.0, register: 0, length: 0, density: 1.0 };
}

/// What changed from the unit before, a bit each (`Span::changed`).
pub mod changed {
    pub const ROLE: u16 = 1;
    pub const THEME: u16 = 1 << 1;
    pub const VARIATION: u16 = 1 << 2;
    pub const KEY: u16 = 1 << 3;
    pub const LAYERS: u16 = 1 << 4;
    pub const VOICE: u16 = 1 << 5;
    pub const WARMTH: u16 = 1 << 6;
    pub const SPACE: u16 = 1 << 7;
    pub const REGISTER: u16 = 1 << 8;
    pub const LENGTH: u16 = 1 << 9;
    pub const DENSITY: u16 = 1 << 10;
    /// The bits' names, lowest first.
    pub const NAMES: [&str; 11] = ["role", "theme", "variation", "key", "layers", "voice", "warmth", "space", "register", "length", "density"];
    /// The things a wander moves: every bit but the role and the variation.
    pub const THINGS: u16 = THEME | KEY | LAYERS | VOICE | WARMTH | SPACE | REGISTER | LENGTH | DENSITY;
    /// The things an interlude moves: a wander's, the key and the theme
    /// apart.
    pub const STATE: u16 = LAYERS | VOICE | WARMTH | SPACE | REGISTER | LENGTH | DENSITY;
}

/// What one unit of a piece is: a section, or a phrase of one (`Grain`).
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(clippy::struct_excessive_bools)] // separate facts about a unit, not a state
pub struct Span {
    pub journey: Journey,
    pub role: Role,
    /// The arc's number in the piece (a movement), or the wander's hold's
    /// (an epoch's first is the epoch's number times its most turns; an
    /// epoch that takes fewer skips the rest); this unit's place in it, and
    /// how many units it has.
    pub part: u64,
    pub at: u64,
    pub of: u64,
    pub theme: Theme,
    /// The seed whose tune the theme is: none for theme A and a rest.
    pub theme_seed: Option<u32>,
    /// Whether the theme is played as a fragment (the development).
    pub fragment: bool,
    pub variation: Variation,
    pub key: Key,
    /// The tempo: the loop's own, for the whole piece.
    pub bpm: f64,
    /// From 0 to 1.
    pub energy: f64,
    /// Whether the piece ends here, on the tonic.
    pub closes: bool,
    /// The bars this unit plays: the grain's, or fewer where a round's last
    /// section is cut short.
    pub bars: f64,
    pub state: State,
    /// What changed from the unit before (`changed`); nothing at a piece's
    /// first.
    pub changed: u16,
    /// The kind of the movement it is in (session 14; a wander's: full).
    pub kind: Kind,
    /// The movement's drawn number, from 0 to 1, that its variations are a
    /// function of: which way a register moves, which voice takes the
    /// tune, which answer the theme is given.
    pub pick: f64,
    /// Whether the unit is a call and response: each phrase theme A's
    /// fragment, then `theme`'s in A's rests.
    pub calls: bool,
    /// Whether the unit prepares the return: over its last phrase the bass
    /// holds the home key's fifth.
    pub prepares: bool,
}

/// The things a wander moves, in the order their turns are numbered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Thing {
    Layers,
    Voice,
    Warmth,
    Space,
    Register,
    Length,
    Density,
    Key,
    Theme,
}

/// A finite piece's round, in units.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Round {
    sections: u64,
    units: u64,
    /// The bars of its last unit: the grain's, or fewer.
    last: f64,
}

/// A piece's plan: the few numbers every unit is found from. Made from the
/// spec (`Form::of`), kept by no one: making it again gives the same.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(clippy::struct_excessive_bools)] // four separate facts about a piece, not a state
pub struct Form {
    seed: u32,
    pub journey: Journey,
    /// Whether the journey was asked, not drawn.
    pub asked: bool,
    pub grain: Grain,
    /// A whole section's bars, a unit's, and how many units a whole section
    /// holds.
    section_bars: f64,
    pub unit_bars: f64,
    per: u64,
    /// A unit's seconds at the loop's tempo.
    pub unit_seconds: f64,
    bar_seconds: f64,
    bpm: f64,
    round: Option<Round>,
    pub home: Key,
    related: [Option<Key>; 4],
    /// The arc: how many a finite piece holds, the units of an Endless
    /// one's, whether it plays the small form, whether it has a second
    /// theme.
    arcs: u64,
    arc_units: u64,
    pub small: bool,
    pub second: bool,
    /// The movements' cycle (session 14): its kinds in order, a full arc
    /// first (one place: every movement a full arc), each place's share in
    /// thousandths of a full arc's, how many whole cycles a finite piece
    /// plays before its last full arc, and each place's units in Endless.
    cycle: [Kind; CYCLE_MAX],
    cycle_len: usize,
    shares: [u64; CYCLE_MAX],
    cycles: u64,
    lens: [u64; CYCLE_MAX],
    /// Whether the tune may be stretched (it has no cycle of its own), and
    /// whether the loop has a tune at all.
    stretchable: bool,
    tune: bool,
    /// Whether the tune's voice may be handed on (none was asked).
    hand_on: bool,
    /// The wander: an epoch's turns and its units, the layers that may rest
    /// and those whose voice may change, a bit each, and which epochs are
    /// each two-state thing's away ones (a bit a thing).
    turns: [Thing; TURNS_MAX],
    turn_count: usize,
    pub epoch_units: u64,
    resting: u8,
    voicing: u8,
    parity: u16,
    /// The layers the loop plays, and of them those an arc's energy may
    /// fade (none asked on, off or re-rolled), a bit each (session 13).
    pub playing: u8,
    pub fading: u8,
    /// Whether the code edited the warmth, and the space: an arc never
    /// moves them (session 13).
    pub toned: [bool; 2],
}

/// Whether a mode has a minor third, a major one, or neither or both
/// (which has no other side).
fn third(scale: usize) -> Option<bool> {
    let steps = SCALES.get(scale)?.steps;
    match (steps.contains(&3.0), steps.contains(&4.0)) {
        (true, false) => Some(false),
        (false, true) => Some(true),
        _ => None,
    }
}

/// The keys related to `home` (the module's doc): the fifths, where they
/// keep enough of its notes; the relative, where the modes have one (never
/// a mode darker than the data's floor, unless home is); and the parallel,
/// for a minor home only (it brightens), where the loop's first profile has
/// a major mode that keeps enough, and no scale was asked.
fn related(spec: &Spec, home: Key) -> [Option<Key>; 4] {
    let own = home.notes();
    let near = |k: Key| f64::from((k.notes() & own).count_ones()) >= LONGFORM.common * f64::from(own.count_ones());
    let fifth = |up: u8, relation| Some(Key { root: (home.root + up) % 12, scale: home.scale, relation }).filter(|&k| near(k));
    let side = third(home.scale);
    // No related key is darker than the data's floor, unless home is.
    let bright = |scale: usize| SCALES.get(scale).map_or(0.0, |s| s.bright);
    let floor = js::min(LONGFORM.dark, bright(home.scale));
    let relative = side.and_then(|major| {
        // The modes of the same notes with the other third: the one a
        // third away where there is one (A aeolian's C ionian), else the
        // nearest in brightness.
        let usual = if major { 9 } else { 3 };
        let mut best: Option<(Key, f64)> = None;
        for up in 1..12u8 {
            for scale in 0..SCALES.len() {
                let k = Key { root: (home.root + up) % 12, scale, relation: Relation::Relative };
                if third(scale) != Some(!major) || k.notes() != own || bright(scale) < floor {
                    continue;
                }
                let far = if up == usual { -1.0 } else { js::abs(bright(scale) - bright(home.scale)) };
                if best.is_none_or(|b| far < b.1) {
                    best = Some((k, far));
                }
            }
        }
        best.map(|b| b.0)
    });
    let profile = spec.mix.iter().fold(None, |best: Option<(usize, f64)>, &(i, w)| if best.is_some_and(|b| b.1 >= w) { best } else { Some((i, w)) });
    let pool = profile.and_then(|(i, _)| CHARACTERS.get(i)).and_then(|c| c.scales).filter(|_| spec.scales == 0);
    // The parallel only brightens: a minor home's major mode, never the
    // other way (for Mikey's ears).
    let parallel = side.filter(|&major| !major).and_then(|major| {
        let mut best: Option<(Key, f64)> = None;
        for &(id, w) in pool? {
            let Some(scale) = SCALES.iter().position(|s| s.id == id) else { continue };
            let k = Key { root: home.root, scale, relation: Relation::Parallel };
            if third(scale) == Some(!major) && near(k) && bright(scale) >= floor && best.is_none_or(|b| w > b.1) {
                best = Some((k, w));
            }
        }
        best.map(|b| b.0)
    });
    [fifth(7, Relation::FifthAbove), fifth(5, Relation::FifthBelow), relative, parallel]
}

impl Form {
    /// The plan of the piece of `spec`, made with `edits` (the request's:
    /// a tone edited is never moved), whose loop plays the layers `present`
    /// (a bit each in `LAYERS`' order). None for a loop that is no long
    /// piece: no length asked, or one of up to 32 bars, which plays as it
    /// always has.
    #[must_use]
    pub fn of(spec: &Spec, edits: &Edits, present: u8) -> Option<Form> {
        Self::of_loop(spec, edits, present, 0b1_1111)
    }

    /// `of`, for a loop made: `voiceable` says, a bit a layer, which of its
    /// layers may play another voice (`heard::voiceable`: a pool with
    /// another, and no sung voice), so the wander's voice turn never falls
    /// on a layer it can't move.
    #[must_use]
    #[allow(clippy::too_many_lines)] // the piece's few numbers, each drawn once, in order
    pub fn of_loop(spec: &Spec, edits: &Edits, present: u8, voiceable: u8) -> Option<Form> {
        let piece = Piece::of(spec)?;
        let endless = match spec.length? {
            Length::Endless => true,
            Length::Bars(n) if n > SECTION_BARS => false,
            Length::Bars(_) => return None,
        };
        let seed = spec.seed;
        let v = spec.vibes.as_ref();
        let (bars, spb) = (spec.bars, spec.steps_per_bar);
        let bar_seconds = piece::seconds(1.0, spb, spec.bpm);
        if !(bar_seconds > 0.0 && bar_seconds.is_finite()) {
            return None;
        }
        // The phrase, as the player's phrase lines fall (`track::phrase_line`).
        let phrase = if bars % PHRASE_BARS == 0.0 {
            PHRASE_BARS
        } else if bars % 2.0 == 0.0 {
            2.0
        } else {
            bars
        };
        let sections = piece.end.map(|e| e.0);
        let length = piece.length();

        // The journey: asked, or drawn, the arc's weight leaned by the vibes.
        let weight = |name| data::lookup(LONGFORM.weights, name).unwrap_or(1.0);
        let arc = weight("arc") * lean_of(Some(LONGFORM.vibes), v) * if endless { LONGFORM.endless } else { 1.0 };
        let drawn = if stream(seed, JOURNEY_SALT, 0).f() * (arc + weight("wander")) < arc { Journey::Arc } else { Journey::Wander };
        let journey = spec.journey.unwrap_or(drawn);

        // An arc's length in time: between the data's two, evenly by ratio,
        // the vibes leaning it longer or shorter.
        let [lo, hi] = LONGFORM.minutes;
        let t = math::pow(stream(seed, MINUTES_SALT, 0).f(), 1.0 / lean_of(Some(LONGFORM.long), v));
        let arc_seconds = 60.0 * lo * math::pow(hi / lo, t);
        let [hold_lo, hold_hi] = LONGFORM.hold;
        let hold_seconds = (hold_lo + hold_hi) / 2.0;
        let section_seconds = bars * bar_seconds;

        // The grain: sections where the form has enough of them.
        let arcs_wanted = length.map_or(1.0, |l| js::max(1.0, js::round(l * bar_seconds / arc_seconds)));
        let arc_sections = length.map_or(arc_seconds / section_seconds, |_| sections.unwrap_or(1) as f64 / arcs_wanted);
        let fine = match journey {
            Journey::Arc => arc_sections < LONGFORM.need,
            Journey::Wander => hold_seconds / section_seconds < LONGFORM.sections,
        };
        let grain = if fine && phrase < bars { Grain::Phrase } else { Grain::Section };
        let unit_bars = if grain == Grain::Phrase { phrase } else { bars };
        let per = (bars / unit_bars) as u64;
        let unit_seconds = unit_bars * bar_seconds;
        let round = piece.end.map(|(sections, last)| {
            let cut = js::ceil(last / unit_bars);
            Round { sections, units: (sections - 1) * per + cut as u64, last: last - (cut - 1.0) * unit_bars }
        });

        // The arcs: a finite piece holds a whole number, each with room for
        // its form; Endless plays arcs of the drawn length in turn.
        let wanted = round.map_or(0, |r| (arcs_wanted as u64).min(r.units / FULL_UNITS).max(1));
        // The movements' kinds (session 14): a cycle drawn per piece, a full
        // arc first, then the others in a drawn order (three movements: one
        // other, never an interlude). A finite piece is as many whole
        // cycles as fill it and a last full arc; every movement has room
        // for its form, or the piece keeps arcs all alike.
        let mut kr = stream(seed, KIND_SALT, 0);
        let mut others = [Kind::Gentle, Kind::Light, Kind::Interlude];
        for x in (1..others.len()).rev() {
            others.swap(x, kr.pick_index(x + 1));
        }
        let mut cycle = [Kind::Full; CYCLE_MAX];
        let mut cycle_len = 1;
        if round.is_none() || wanted >= 4 {
            cycle[1..].copy_from_slice(&others);
            cycle_len = CYCLE_MAX;
        } else if wanted == 3 {
            cycle[1] = others.into_iter().find(|&k| k != Kind::Interlude).unwrap_or(Kind::Gentle);
            cycle_len = 2;
        }
        let mut shares = [0u64; CYCLE_MAX];
        for (s, k) in shares.iter_mut().zip(cycle).take(cycle_len) {
            *s = k.share();
        }
        let sum: u64 = shares.iter().sum();
        let mut cycles = 0;
        if let (Some(r), true) = (round, cycle_len > 1) {
            let want = js::max(1.0, js::round((wanted - 1) as f64 * 1000.0 / sum as f64)) as u64;
            let room = (0..cycle_len).map(|x| (r.units * shares[x] / cycle[x].least()).saturating_sub(shares[0]) / sum).min().unwrap_or(0);
            cycles = want.min(room);
            if cycles == 0 {
                cycle_len = 1;
            }
        }
        let arcs = if cycles > 0 { cycles * cycle_len as u64 + 1 } else { wanted };
        let arc_units = round.map_or_else(
            || (js::round(arc_seconds / unit_seconds) as u64).max(FULL_UNITS),
            |r| if cycles > 0 { (u128::from(r.units) * u128::from(shares[0]) / u128::from(cycles * sum + shares[0])) as u64 } else { r.units / arcs },
        );
        let mut lens = [0u64; CYCLE_MAX];
        for x in 0..cycle_len {
            lens[x] = (js::round(arc_units as f64 * shares[x] as f64 / 1000.0) as u64).max(cycle[x].least());
        }
        let small = round.is_some() && (arc_units < FULL_UNITS || arc_units as f64 * unit_seconds < LONGFORM.small * 60.0);

        // The wander's turns: every thing that was not asked, and that the
        // loop has.
        let home = Key { root: (js::index(spec.root) % 12) as u8, scale: spec.scale, relation: Relation::Home };
        let related = related(spec, home);
        let bit = |l: Layer| 1u8 << l as usize;
        let playing = present & !spec.layers_off;
        let fixed = spec.layers_on | spec.layers_off | spec.rolled;
        let resting = if playing.count_ones() >= 3 { playing & !fixed } else { 0 };
        let voicing = [Layer::Bass, Layer::Chords, Layer::Melody].into_iter().filter(|&l| playing & voiceable & bit(l) != 0 && spec.voices[l as usize] == 0).fold(0, |m, l| m | bit(l));
        let chords = playing & bit(Layer::Chords) != 0;
        let tune = playing & bit(Layer::Melody) != 0;
        // A second theme where the loop has a tune of its own to set it
        // against, and none was asked (a theme asked stays the tune).
        let second = stream(seed, SECOND_SALT, 0).f() < LONGFORM.second && tune && spec.theme.is_none();
        let stretchable = spec.cycles.of(Layer::Melody as usize).is_none_or(|c| c == 0.0 || c == bars * spb);
        let mut turns = [Thing::Layers; TURNS_MAX];
        let mut turn_count = 0;
        let mut turn = |thing, on: bool| {
            if on {
                turns[turn_count] = thing;
                turn_count += 1;
            }
        };
        turn(Thing::Layers, resting != 0);
        turn(Thing::Layers, resting != 0);
        turn(Thing::Voice, voicing != 0);
        turn(Thing::Warmth, edits.tone(0).is_none());
        turn(Thing::Space, edits.tone(1).is_none());
        turn(Thing::Register, chords);
        turn(Thing::Length, chords);
        turn(Thing::Density, tune);
        turn(Thing::Key, spec.keys == 0 && LONGFORM.key > 0.0 && related.iter().flatten().any(|k| k.relation.is_fifth()));
        // (A second theme may rest, and the tune with it: never a tune
        // asked on or re-rolled.)
        turn(Thing::Theme, tune && spec.theme.is_none() && LONGFORM.theme > 0.0 && fixed & bit(Layer::Melody) == 0);
        // An epoch is long enough for the turns it is likely to take, each a
        // mean hold: a rare thing takes a turn going and one coming back.
        let rare = |thing, chance: f64| if turns[..turn_count].contains(&thing) { 1.0 - js::min(1.0, 2.0 * chance) } else { 0.0 };
        let likely = js::max(1.0, turn_count as f64 - rare(Thing::Key, LONGFORM.key) - rare(Thing::Theme, LONGFORM.theme));
        let epoch_units = (js::round(likely * hold_seconds / unit_seconds) as u64).max(turn_count.max(1) as u64);
        let parity = (stream(seed, PARITY_SALT, 0).f() * 65536.0) as u16;

        Some(Form {
            seed,
            journey,
            asked: spec.journey.is_some(),
            grain,
            section_bars: bars,
            unit_bars,
            per,
            unit_seconds,
            bar_seconds,
            bpm: spec.bpm,
            round,
            home,
            related,
            arcs,
            arc_units,
            small,
            second,
            cycle,
            cycle_len,
            shares,
            cycles,
            lens,
            stretchable,
            tune,
            hand_on: spec.voices[Layer::Melody as usize] == 0,
            turns,
            turn_count,
            epoch_units,
            resting,
            voicing,
            parity,
            playing,
            fading: playing & !fixed,
            toned: [edits.tone(0).is_some(), edits.tone(1).is_some()],
        })
    }

    /// How many units section `n` holds: one at the section's grain, its
    /// phrases at the phrase's (fewer in a round's last, cut short).
    #[must_use]
    pub fn units_in(&self, n: u64) -> u64 {
        match self.round {
            Some(r) if n % r.sections == r.sections - 1 => r.units - (r.sections - 1) * self.per,
            _ => self.per,
        }
    }

    /// How many bars there are from section `n`'s first to its round's
    /// end (a finite piece's); none for Endless. From the numbers alone.
    #[must_use]
    pub fn to_end(&self, n: u64) -> Option<f64> {
        let r = self.round?;
        let i = n % r.sections;
        let last = (r.units - (r.sections - 1) * self.per - 1) as f64 * self.unit_bars + r.last;
        Some((r.sections - 1 - i) as f64 * self.section_bars + last)
    }

    /// A bar's seconds at the loop's tempo.
    #[must_use]
    pub fn bar_seconds(&self) -> f64 {
        self.bar_seconds
    }

    /// How many arcs a finite arc piece holds (its movements); 0 for
    /// Endless, whose arcs go on.
    #[must_use]
    pub fn arcs(&self) -> u64 {
        self.arcs
    }

    /// An arc's units: a finite piece's first's (the others are within one
    /// of it), or every one of Endless's.
    #[must_use]
    pub fn arc_units(&self) -> u64 {
        self.arc_units
    }

    /// The movements' cycle: its kinds in order (one, a full arc: every
    /// movement alike).
    #[must_use]
    pub fn cycle(&self) -> &[Kind] {
        &self.cycle[..self.cycle_len]
    }

    /// How many units the movement holding unit `phrase` of section `n`
    /// has (an arc piece's).
    #[must_use]
    pub fn movement_units(&self, n: u64, phrase: u64) -> u64 {
        self.movement(self.place(n, phrase).1).2
    }

    /// The seed of movement `k`'s second theme, where it brings one.
    #[must_use]
    pub fn second_of(&self, k: u64) -> Option<u32> {
        let light = self.small || self.kind_of(k) == Kind::Light;
        (self.second && !light && self.kind_of(k).bears()).then(|| self.second_seed(k))
    }

    /// The keys the piece may visit, the home key apart.
    pub fn related(&self) -> impl Iterator<Item = Key> + '_ {
        self.related.iter().flatten().copied()
    }

    /// How many turns an epoch of the wander has: the things that may move
    /// (a layer's rest and its return are two).
    #[must_use]
    pub fn turns(&self) -> usize {
        self.turn_count
    }

    /// The unit's place: its number counted from the piece's start, and in
    /// its round (the same for Endless).
    fn place(&self, n: u64, phrase: u64) -> (u64, u64) {
        if let Some(r) = self.round {
            let i = (n % r.sections) * self.per + phrase.min(self.units_in(n) - 1);
            ((n / r.sections).saturating_mul(r.units).saturating_add(i), i)
        } else {
            let g = n.saturating_mul(self.per).saturating_add(phrase.min(self.per - 1));
            (g, g)
        }
    }

    /// The unit before unit `phrase` of section `n` (`at`'s numbers), none
    /// at the piece's very first: the last of the section before, at a
    /// section's first.
    #[must_use]
    pub fn before(&self, n: u64, phrase: u64) -> Option<(u64, u64)> {
        let p = phrase.min(self.units_in(n) - 1);
        if p > 0 {
            Some((n, p - 1))
        } else {
            n.checked_sub(1).map(|b| (b, self.units_in(b) - 1))
        }
    }

    /// Where unit `phrase` of section `n` sits in its round's close, if it
    /// closes: its place among the close's units (0 the first) and how many
    /// they are.
    #[must_use]
    pub fn close_place(&self, n: u64, phrase: u64) -> Option<(u64, u64)> {
        let r = self.round?;
        let (_, i) = self.place(n, phrase);
        let count = self.close_units(r).min(r.units);
        let first = r.units - count;
        (i >= first).then(|| (i - first, count))
    }

    /// Whether section `n` begins a round after another (a finite piece
    /// come round): it starts from the close before it.
    #[must_use]
    pub fn comes_round(&self, n: u64) -> bool {
        self.round.is_some_and(|r| n > 0 && n.is_multiple_of(r.sections))
    }

    /// Section `n`, at its first unit (the whole of it, at the section's
    /// grain).
    #[must_use]
    pub fn section(&self, n: u64) -> Span {
        self.at(n, 0)
    }

    /// Unit `phrase` of section `n` (0 the section's first; past its last
    /// is its last): what it is, and what changed from the unit before it.
    /// Found from the numbers alone, at the same cost for any `n`.
    #[must_use]
    pub fn at(&self, n: u64, phrase: u64) -> Span {
        let (g, i) = self.place(n, phrase);
        let mut span = self.unit(i);
        if g > 0 {
            let before = self.unit(match self.round {
                Some(r) if i == 0 => r.units - 1,
                _ => i - 1,
            });
            span.changed = difference(&before, &span);
            // (A round's first unit is a start, not a turn.)
            // (An interlude's first follows a breath: only its own things
            // turn it.)
            let things = if self.journey == Journey::Wander { changed::THINGS } else { changed::STATE };
            if span.journey == Journey::Wander && i > 0 && span.role == Role::Holding && span.changed & things != 0 {
                span.role = Role::Turning;
                span.changed |= changed::ROLE;
            }
        }
        span
    }

    /// Unit `i` of a round (or of Endless), before what changed is read.
    fn unit(&self, i: u64) -> Span {
        let mut span = match self.journey {
            Journey::Arc => self.arc_unit(i),
            Journey::Wander => self.wander_unit(i),
        };
        span.bars = match self.round {
            Some(r) if i == r.units - 1 => r.last,
            _ => self.unit_bars,
        };
        span
    }

    /// How many units from a round's end a close takes to last the data's
    /// least: one, or more where the last is cut short.
    fn close_units(&self, r: Round) -> u64 {
        let short = LONGFORM.close / self.bar_seconds - r.last;
        1 + if short > 0.0 { js::ceil(short / self.unit_bars) as u64 } else { 0 }
    }

    // ------------------------------------------------------------ the arc

    /// The start of movement `k` of a finite piece's `arcs`, in units:
    /// the units shared evenly, or by the cycle's shares.
    fn arc_start(&self, r: Round, k: u64) -> u64 {
        if self.cycles == 0 {
            return (u128::from(k) * u128::from(r.units) / u128::from(self.arcs)) as u64;
        }
        if k >= self.arcs {
            return r.units;
        }
        let len = self.cycle_len;
        let sum: u64 = self.shares.iter().take(len).sum();
        let before: u64 = self.shares.iter().take((k % len as u64) as usize).sum();
        let at = (k / len as u64) * sum + before;
        (u128::from(at) * u128::from(r.units) / u128::from(self.cycles * sum + self.shares[0])) as u64
    }

    /// The seed of arc `k`'s second theme: its own draw, so an earlier
    /// arc's is found by the number alone (the last arc looks back).
    fn second_seed(&self, k: u64) -> u32 {
        (stream(self.seed, THEME_SALT, k).f() * 4_294_967_296.0) as u32
    }

    /// The roles' bounds in an arc of `n` units (role `x` from `bounds[x]`
    /// to `bounds[x + 1]`), and how many roles: the form's shares laid on
    /// the units, the peak ending where this arc drew it, every role a unit
    /// at least, and the `last` arc's close lasting its least.
    fn bounds(&self, form: Table, n: u64, peak: f64, last: bool) -> ([u64; ROLES_MAX + 1], usize) {
        let roles = form.len().min(ROLES_MAX);
        let total: f64 = form.iter().take(roles).map(|x| x.1).sum();
        let at_peak = form.iter().position(|x| x.0 == Role::Peak.name());
        let before: f64 = at_peak.map_or(total, |p| form.iter().take(p + 1).map(|x| x.1).sum());
        let mut bounds = [0u64; ROLES_MAX + 1];
        let mut sum = 0.0;
        for (x, &(_, share)) in form.iter().take(roles).enumerate() {
            sum += share;
            let c = match at_peak {
                Some(p) if before < total && x <= p => sum / before * peak,
                Some(_) if before < total => peak + (sum - before) / (total - before) * (1.0 - peak),
                _ => sum / total,
            };
            let room = n.saturating_sub((roles - 1 - x) as u64);
            bounds[x + 1] = (js::round(c * n as f64) as u64).max(bounds[x] + 1).min(room).max(bounds[x]);
        }
        bounds[roles] = n;
        if let (true, Some(round)) = (last && roles >= 2, self.round) {
            // The close lasts its least, taking from the role before it.
            let from = n.saturating_sub(self.close_units(round)).max(bounds[roles - 2] + 1);
            bounds[roles - 1] = bounds[roles - 1].min(from).max(bounds[roles - 2]);
        }
        (bounds, roles)
    }

    /// The movement unit `i` of a round (or of Endless) is in: its number,
    /// the unit's place in it, its units, whether it is a finite piece's
    /// last, and its kind. By division: whole cycles, then a place.
    fn movement(&self, i: u64) -> (u64, u64, u64, bool, Kind) {
        let len = self.cycle_len;
        let Some(r) = self.round else {
            if len == 1 {
                return (i / self.arc_units, i % self.arc_units, self.arc_units, false, Kind::Full);
            }
            let whole: u64 = self.lens.iter().take(len).sum();
            let (c, mut x) = (i / whole, i % whole);
            let mut place = 0;
            while place + 1 < len && x >= self.lens[place] {
                x -= self.lens[place];
                place += 1;
            }
            return (c.saturating_mul(len as u64).saturating_add(place as u64), x, self.lens[place], false, self.cycle[place]);
        };
        if self.cycles == 0 {
            let mut k = (u128::from(i) * u128::from(self.arcs) / u128::from(r.units)) as u64;
            if k + 1 < self.arcs && self.arc_start(r, k + 1) <= i {
                k += 1;
            }
            let start = self.arc_start(r, k);
            return (k, i - start, self.arc_start(r, k + 1) - start, k + 1 == self.arcs, Kind::Full);
        }
        // A guess by the shares, set right against the starts themselves.
        let sum: u64 = self.shares.iter().take(len).sum();
        let whole = u128::from(self.cycles * sum + self.shares[0]);
        let at = (u128::from(i) * whole / u128::from(r.units)) as u64;
        let (c, mut x) = (at / sum, at % sum);
        let mut place = 0;
        while place + 1 < len && x >= self.shares[place] {
            x -= self.shares[place];
            place += 1;
        }
        let mut k = (c * len as u64 + place as u64).min(self.arcs - 1);
        while k + 1 < self.arcs && self.arc_start(r, k + 1) <= i {
            k += 1;
        }
        while k > 0 && self.arc_start(r, k) > i {
            k -= 1;
        }
        let start = self.arc_start(r, k);
        (k, i - start, self.arc_start(r, k + 1) - start, k + 1 == self.arcs, self.kind_of(k))
    }

    /// The kind of movement `k`: its place in the cycle's.
    #[must_use]
    pub fn kind_of(&self, k: u64) -> Kind {
        self.cycle[(k % self.cycle_len as u64) as usize]
    }

    /// How many of a finite piece's movements before its last bring a
    /// second theme: what the last looks back on.
    fn bearers(&self) -> u64 {
        if self.cycles == 0 {
            return self.arcs.saturating_sub(1);
        }
        self.cycles * self.cycle.iter().take(self.cycle_len).filter(|k| k.bears()).count() as u64
    }

    /// The number of the `x`th movement that brings a second theme, by
    /// division.
    fn bearer(&self, x: u64) -> u64 {
        if self.cycles == 0 {
            return x;
        }
        let each = self.cycle.iter().take(self.cycle_len).filter(|k| k.bears()).count().max(1) as u64;
        let place = (0..self.cycle_len).filter(|&p| self.cycle[p].bears()).nth((x % each) as usize).unwrap_or(0);
        (x / each) * self.cycle_len as u64 + place as u64
    }

    #[allow(clippy::too_many_lines)] // a movement's draws in their fixed order, then its roles
    fn arc_unit(&self, i: u64) -> Span {
        // Which movement, and where in it.
        let (k, j, n, last, kind) = self.movement(i);
        if kind == Kind::Interlude {
            return self.interlude_unit(k, j, n);
        }
        // The arc's own draws, in a fixed order.
        let mut r = stream(self.seed, ARC_SALT, k);
        let peak = r.range(LONGFORM.peak[0], LONGFORM.peak[1]);
        let height = r.range(LONGFORM.height[0], LONGFORM.height[1]);
        let keys = self.related.iter().flatten().count();
        let first = r.pick_index(keys.max(1));
        let key = |q: usize| self.related.iter().flatten().nth(q % keys.max(1)).copied().unwrap_or(self.home);
        let second_key = key(first);
        let relative = self.related.iter().flatten().any(|k| k.relation == Relation::Relative);
        let vary = |table: Table, r: &mut Rng| {
            // A variation the loop can't play is never drawn: the tune's
            // voice asked, no relative key, a tune with a cycle of its own.
            let allowed = |name: &str| {
                (self.hand_on || name != Variation::Voice.name())
                    && (relative || name != Variation::Reharmonized.name())
                    && (self.stretchable || name != Variation::Stretched.name())
            };
            r.weighted_index(table.iter().map(|&(name, w)| if allowed(name) { w } else { 0.0 })).and_then(|x| table.get(x)).map_or(Variation::Register, |x| Variation::named(x.0))
        };
        let returns = vary(LONGFORM.returns, &mut r);
        let builds = vary(LONGFORM.builds, &mut r);
        // (Session 14's draws, after the others, so none of those moved.)
        let pick = r.f();
        let prepared = r.f() < LONGFORM.prepare;

        let light = self.small || kind == Kind::Light;
        let two = self.second && !light;
        let form = if light { LONGFORM.short } else { LONGFORM.full };
        let (bounds, roles) = self.bounds(form, n, peak, last);
        let x = (0..roles).find(|&x| j < bounds[x + 1]).unwrap_or(roles - 1);
        let (from, to) = (bounds[x], bounds[x + 1]);
        let mut role = Role::named(form[x].0);
        if role == Role::Close && !last {
            role = Role::Breath;
        }

        // What the role plays.
        let energy = data::LONGFORM.energy.iter().find(|e| e.0 == role.name()).map_or(0.5, |e| {
            let t = ((j - from) as f64 + 0.5) / (to - from).max(1) as f64;
            let e = e.1[0] + (e.1[1] - e.1[0]) * t;
            // A gentle movement's sits between its two, but its breath.
            if kind == Kind::Gentle && role != Role::Breath {
                let [lo, hi] = LONGFORM.gentle;
                lo + (hi - lo) * e
            } else {
                e * height
            }
        });
        let b = if two { Theme::B } else { Theme::A };
        let mut fragment = false;
        let mut calls = false;
        let (theme, variation, key) = match role {
            Role::Opening | Role::Close => (Theme::A, Variation::Outline, self.home),
            Role::Theme | Role::Holding | Role::Turning => (Theme::A, Variation::Plain, self.home),
            Role::Build => (Theme::A, builds, self.home),
            Role::Lift => (Theme::A, if builds == Variation::Plain { Variation::Register } else { builds }, self.home),
            Role::Second if two => (Theme::B, Variation::Plain, second_key),
            Role::Second => (Theme::A, if second_key == self.home { Variation::Register } else { Variation::Key }, second_key),
            Role::Development => {
                fragment = true;
                let d = j - from;
                // An A and its answer, unit by unit; a key lasts a pair of
                // them at least, and the data's least time.
                let stay = 2 * js::max(1.0, js::ceil(LONGFORM.stay / (2.0 * self.unit_seconds))) as u64;
                let at = key(first + 1 + (d / stay) as usize);
                let at = if keys == 1 && (d / stay) % 2 == 1 { self.home } else { at };
                // The answer: the second theme, in a finale an earlier
                // movement's (one a section, so a section makes one tune),
                // or theme A answered. Now and then the pair is a call and
                // response: A's fragment, then the answer's in A's rests.
                let bearers = self.bearers();
                let answer = if !two {
                    (Theme::A, Variation::Answered)
                } else if last && bearers > 0 {
                    let x = if self.grain == Grain::Phrase { i / self.per } else { d / 2 };
                    (Theme::Recall(self.bearer(x % bearers)), Variation::Plain)
                } else {
                    (Theme::B, Variation::Plain)
                };
                calls = self.tune && to - from >= 2 && (d / 2) * 2 + 1 < to - from && stream(self.seed, CALL_SALT, k.wrapping_mul(0x1_0000).wrapping_add(d / 2)).f() < LONGFORM.call;
                if d % 2 == 0 && !calls { (Theme::A, Variation::Plain, at) } else { (answer.0, answer.1, at) }
            }
            Role::Peak => (Theme::A, Variation::Register, second_key),
            // (The small form stays at home throughout.)
            Role::Release => (b, Variation::Outline, if light { self.home } else { second_key }),
            Role::Return => (Theme::A, returns, self.home),
            Role::Breath => (Theme::Rest, Variation::Plain, self.home),
        };
        let theme_seed = match theme {
            Theme::B => Some(self.second_seed(k)),
            Theme::Recall(of) => Some(self.second_seed(of)),
            Theme::A | Theme::Rest => None,
        };
        // The return prepared: the last unit before theme A comes home.
        let held = prepared && j + 1 == to && form.get(x + 1).is_some_and(|next| next.0 == Role::Return.name());
        Span {
            journey: Journey::Arc,
            role,
            part: k,
            at: j,
            of: n,
            theme,
            theme_seed,
            fragment,
            variation,
            key,
            bpm: self.bpm,
            energy,
            closes: role == Role::Close,
            bars: self.unit_bars,
            state: State::OWN,
            changed: 0,
            kind,
            pick,
            calls,
            prepares: held,
        }
    }

    /// Unit `j` of the `n` of movement `k`, an interlude (session 14): the
    /// wander's way for the span. Up to three of the things a wander moves
    /// (never the key or the theme) go out one at a time, a hold apart, and
    /// come home in the same order, so one thing moves at each hold's end
    /// and everything is the loop's own at both ends. The things, their
    /// order and where each goes are drawn from the movement's number.
    fn interlude_unit(&self, k: u64, j: u64, n: u64) -> Span {
        let mut things = [Thing::Layers; TURNS_MAX];
        let mut count = 0;
        for &t in self.turns.iter().take(self.turn_count) {
            if !matches!(t, Thing::Key | Thing::Theme) && !things[..count].contains(&t) {
                things[count] = t;
                count += 1;
            }
        }
        let mut r = stream(self.seed, INTERLUDE_SALT, k);
        for x in (1..count).rev() {
            things.swap(x, r.pick_index(x + 1));
        }
        // As many things as leave each hold the data's least, three at most.
        let room = js::max(0.0, (n as f64 * self.unit_seconds / LONGFORM.hold[0] - 1.0) / 2.0) as usize;
        let out = room.clamp(1, 3).min(count).min((n.saturating_sub(1) / 2) as usize);
        let holds = 2 * out as u64 + 1;
        let hold = (u128::from(j) * u128::from(holds) / u128::from(n.max(1))) as u64;
        let line = |h: u64| (u128::from(h) * u128::from(n)).div_ceil(u128::from(holds)) as u64;
        let (from, to) = (line(hold), line(hold + 1).max(line(hold) + 1));
        // Thing `q` is out from the end of hold `q` to the end of hold
        // `out + q`.
        let away = |thing: Thing| things[..out].iter().position(|&t| t == thing).is_some_and(|q| (q as u64) < hold && hold <= (out + q) as u64);
        let draw = |thing: Thing| stream(self.seed, VALUE_SALT + thing as u32, k ^ 0x8000_0000_0000_0000);
        let among = |mask: u8, r: &mut Rng| {
            let pick = r.pick_index(mask.count_ones().max(1) as usize);
            LAYERS.into_iter().filter(|&l| mask & (1 << l as usize) != 0).nth(pick)
        };
        let sign = |r: &mut Rng| if r.f() < 0.5 { -1 } else { 1 };
        let [thin, own] = LONGFORM.density;
        let apart = LONGFORM.apart;
        let voice = {
            let mut r = draw(Thing::Voice);
            among(self.voicing, &mut r).map(|l| (l, r.f())).filter(|_| things[..out].contains(&Thing::Voice))
        };
        // A layer never rests while its voice is changed.
        let resting = self.resting & !voice.map_or(0, |(l, _)| 1u8 << l as usize);
        let tone = |thing: Thing| {
            let mut r = draw(thing);
            f64::from(sign(&mut r)) * r.range(apart, 1.0)
        };
        let state = State {
            rest: among(resting, &mut draw(Thing::Layers)).filter(|_| away(Thing::Layers)),
            voice: voice.filter(|_| away(Thing::Voice)),
            warmth: if away(Thing::Warmth) { tone(Thing::Warmth) } else { 0.0 },
            space: if away(Thing::Space) { tone(Thing::Space) } else { 0.0 },
            register: if away(Thing::Register) { sign(&mut draw(Thing::Register)) } else { 0 },
            length: if away(Thing::Length) { sign(&mut draw(Thing::Length)) } else { 0 },
            density: if away(Thing::Density) { draw(Thing::Density).range(thin, own - apart * (own - thin)) } else { own },
        };
        let thinned = if own > thin { (own - state.density) / (own - thin) } else { 0.0 };
        Span {
            journey: Journey::Wander,
            role: Role::Holding,
            part: k,
            at: j - from.min(j),
            of: to - from,
            theme: Theme::A,
            theme_seed: None,
            fragment: false,
            variation: Variation::Plain,
            key: self.home,
            bpm: self.bpm,
            energy: js::clamp(0.5 - 0.12 * f64::from(u8::from(state.rest.is_some())) - 0.15 * thinned + 0.05 * f64::from(state.register), 0.0, 1.0),
            closes: false,
            bars: self.unit_bars,
            state,
            changed: 0,
            kind: Kind::Interlude,
            pick: 0.0,
            calls: false,
            prepares: false,
        }
    }

    // --------------------------------------------------------- the wander

    /// Whether a finite piece's epoch `e` is one of its last two, which
    /// bring the key and the theme home for the close.
    fn homeward(&self, e: u64) -> bool {
        self.round.is_some_and(|r| e + 2 >= r.units.div_ceil(self.epoch_units))
    }

    /// What the things are once epoch `e`'s turns are all taken, and before
    /// any is (the epoch before's).
    fn sides(&self, e: u64) -> [(State, Key, Theme); 2] {
        [self.settled(e.checked_sub(1)), self.settled(Some(e))]
    }

    /// The things that take a turn in an epoch, the layers apart: of those
    /// that may move, those that do (a key that stays, or a thing at its own
    /// in the first epoch, takes none), so every turn is a move. And whether
    /// a layer rests in it.
    fn turns_of(&self, [was, now]: &[(State, Key, Theme); 2]) -> ([Thing; TURNS_MAX], usize, bool) {
        let mut turns = [Thing::Layers; TURNS_MAX];
        let mut count = 0;
        for &thing in self.turns.iter().take(self.turn_count) {
            let moves = match thing {
                Thing::Layers => false,
                Thing::Voice => was.0.voice != now.0.voice,
                Thing::Warmth => was.0.warmth != now.0.warmth,
                Thing::Space => was.0.space != now.0.space,
                Thing::Register => was.0.register != now.0.register,
                Thing::Length => was.0.length != now.0.length,
                Thing::Density => was.0.density != now.0.density,
                Thing::Key => was.1 != now.1,
                Thing::Theme => was.2 != now.2,
            };
            if moves {
                turns[count] = thing;
                count += 1;
            }
        }
        (turns, count, now.0.rest.is_some())
    }

    /// Which of an epoch's `count` things (the layers apart) it ends on:
    /// drawn on a stream of its own, so the epoch after reads it without
    /// the epoch's order.
    fn last_turn(&self, e: u64, count: usize) -> usize {
        stream(self.seed, LAST_SALT, e).pick_index(count.max(1))
    }

    /// Epoch `e`'s turns in the order they are taken (`order[q]` the thing
    /// that moves at the end of hold `q`), how many, and where each hold
    /// ends, in units from the epoch's start. The first turn is never the
    /// thing the epoch before ended on, where there are things enough to
    /// choose; a layer's rest and its return are a hold or two apart, and
    /// never an epoch's last turn where another thing can be.
    fn epoch(&self, e: u64, sides: &[(State, Key, Theme); 2]) -> ([Thing; TURNS_MAX], usize, [u64; TURNS_MAX]) {
        let (turns, count, layers) = self.turns_of(sides);
        let n = (count + if layers { 2 } else { 0 }).min(TURNS_MAX);
        let mut order = [Thing::Layers; TURNS_MAX];
        let mut ends = [self.epoch_units; TURNS_MAX];
        if n == 0 {
            return (order, 0, ends);
        }
        let mut r = stream(self.seed, EPOCH_SALT, e);
        // The holds: each a drawn length, fitted to the epoch (`fit_holds`:
        // as drawn around the epoch's mean, never past the data's range
        // where the epoch has turns enough), in units.
        let mut lengths = [0.0; TURNS_MAX];
        for l in lengths.iter_mut().take(n) {
            *l = r.range(LONGFORM.hold[0], LONGFORM.hold[1]) / self.unit_seconds;
        }
        // The longest in whole units, a little under: a hold's two ends each
        // round to a unit line, so it may gain up to a unit.
        let total = self.epoch_units as f64;
        let most = js::max(1.0, js::floor(LONGFORM.hold[1] / self.unit_seconds)) - 1e-6;
        fit_holds(&mut lengths[..n], total, LONGFORM.hold[0] / self.unit_seconds, most);
        let mut sum = 0.0;
        let mut before = 0;
        for q in 0..n {
            sum += lengths[q];
            let room = self.epoch_units.saturating_sub((n - 1 - q) as u64);
            ends[q] = (js::round(sum) as u64).max(before + 1).min(room).max(before);
            before = ends[q];
        }
        ends[n - 1] = self.epoch_units;
        // The things' order: the last as drawn, the first not what moved
        // last, the rest shuffled.
        let mut rest = turns;
        if count > 0 {
            let moved = e.checked_sub(1).and_then(|b| {
                let (theirs, count, layers) = self.turns_of(&self.sides(b));
                if count > 0 { Some(theirs[self.last_turn(b, count)]) } else { layers.then_some(Thing::Layers) }
            });
            rest.swap(self.last_turn(e, count), count - 1);
            let free = rest.iter().take(count - 1).filter(|&&t| Some(t) != moved).count();
            if count > 1 {
                let pick = r.pick_index(if free > 0 { free } else { count - 1 });
                let first = (0..count - 1).filter(|&x| free == 0 || Some(rest[x]) != moved).nth(pick).unwrap_or(0);
                rest.swap(0, first);
                for x in (2..count - 1).rev() {
                    rest.swap(x, 1 + r.pick_index(x));
                }
            }
        }
        // The layer's rest: its two turns set among the others, one hold
        // apart or two, before the epoch's last.
        let (out, back) = if layers && n == count + 2 {
            let apart = if count >= 2 { 1 + r.pick_index(2) } else { 1 };
            let out = r.pick_index((count + 1).saturating_sub(apart).max(1));
            (out, out + apart)
        } else {
            (TURNS_MAX, TURNS_MAX)
        };
        let mut next = 0;
        for (q, turn) in order.iter_mut().enumerate().take(n) {
            if q != out && q != back {
                *turn = rest[next.min(TURNS_MAX - 1)];
                next += 1;
            }
        }
        (order, n, ends)
    }

    /// Whether epoch `e` is one of the away ones: a two-state thing is the
    /// loop's own every other epoch, so it always moves. Every thing goes
    /// out in the even epochs, the first among them, and comes home in the
    /// odd ones, one turn at a time (session 12): so every epoch has all its
    /// turns, and none is stretched to fill it. (102a gave each thing its
    /// own parity, so the first epoch moved about half of them, and its
    /// holds outlasted 300 s.)
    fn away(e: u64) -> bool {
        e & 1 == 0
    }

    /// The side of the loop's own the warmth or the space (`bit`) takes in
    /// epoch `e`: the other each epoch, the first drawn per piece.
    fn side(&self, bit: u16, e: u64) -> f64 {
        if Self::away(e) == (self.parity & bit != 0) { -1.0 } else { 1.0 }
    }

    /// What the wander's things are once they have all taken epoch `e`'s
    /// turn (`None`: before the first, the loop's own).
    fn settled(&self, e: Option<u64>) -> (State, Key, Theme) {
        let Some(e) = e else { return (State::OWN, self.home, Theme::A) };
        let homeward = self.homeward(e);
        let draw = |thing: Thing| stream(self.seed, VALUE_SALT + thing as u32, e);
        let among = |mask: u8, r: &mut Rng| {
            let pick = r.pick_index(mask.count_ones().max(1) as usize);
            LAYERS.into_iter().filter(|&l| mask & (1 << l as usize) != 0).nth(pick)
        };
        let [thin, own] = LONGFORM.density;
        let sign = |r: &mut Rng| if r.f() < 0.5 { -1 } else { 1 };
        // A thing that takes no turns (asked, or not the loop's) stays the
        // loop's own.
        let has = |thing: Thing| self.turns[..self.turn_count].contains(&thing);
        // The layer whose voice an epoch changes, if it is an away one.
        let voice_of = |e: u64| {
            let mut r = stream(self.seed, VALUE_SALT + Thing::Voice as u32, e);
            among(self.voicing, &mut r).map(|l| (l, r.f())).filter(|_| Self::away(e))
        };
        let voice = voice_of(e);
        // A layer never rests while its voice is changed: not the layer
        // this epoch voices, nor the one the epoch before did.
        let voiced = |v: Option<(Layer, f64)>| v.map_or(0, |(l, _)| 1u8 << l as usize);
        let resting = self.resting & !voiced(voice) & !voiced(e.checked_sub(1).and_then(voice_of));
        let mut r = draw(Thing::Key);
        // A wander visits a key a fifth away only: the whole loop moved as
        // it is, every chord and note the same in the new key (session 12;
        // the relative and the parallel change what the chords are, which
        // is the arc's to sound, session 14).
        let fifths = || self.related.iter().flatten().filter(|k| k.relation.is_fifth());
        let keys = fifths().count();
        let visit = r.f() < LONGFORM.key;
        let key = fifths().nth(r.pick_index(keys.max(1))).copied().filter(|_| visit && !homeward && has(Thing::Key)).unwrap_or(self.home);
        // The tone's two take the other side of the loop's own every epoch,
        // at least `apart` of the way out, so each turn is a step that is
        // heard (a fresh draw anywhere, as 102a drew them, moved by less
        // than a fifth of the range about one turn in five); the tune thins
        // every other epoch and is its own between.
        let apart = LONGFORM.apart;
        let side = |bit: u16| self.side(bit, e);
        let state = State {
            rest: among(resting, &mut draw(Thing::Layers)),
            voice,
            warmth: if has(Thing::Warmth) { side(changed::WARMTH) * draw(Thing::Warmth).range(apart, 1.0) } else { 0.0 },
            space: if has(Thing::Space) { side(changed::SPACE) * draw(Thing::Space).range(apart, 1.0) } else { 0.0 },
            register: if has(Thing::Register) && Self::away(e) { sign(&mut draw(Thing::Register)) } else { 0 },
            length: if has(Thing::Length) && Self::away(e) { sign(&mut draw(Thing::Length)) } else { 0 },
            density: if has(Thing::Density) && Self::away(e) { draw(Thing::Density).range(thin, own - apart * (own - thin)) } else { own },
        };
        let theme = if has(Thing::Theme) && !homeward && draw(Thing::Theme).f() < LONGFORM.theme { Theme::B } else { Theme::A };
        (state, key, theme)
    }

    fn wander_unit(&self, i: u64) -> Span {
        let (e, x) = (i / self.epoch_units, i % self.epoch_units);
        let sides = self.sides(e);
        let (order, n, ends) = self.epoch(e, &sides);
        let hold = (0..n).find(|&q| x < ends[q]).unwrap_or(n.saturating_sub(1));
        let from = if hold == 0 { 0 } else { ends[hold - 1] };
        // Each thing is as this epoch's turn left it, where that turn is
        // taken, and as the epoch before left it where it is not.
        let [(was, was_key, was_theme), (now, now_key, now_theme)] = sides;
        let pick = |thing: Thing| (0..hold).any(|q| order[q] == thing);
        // The layers' two turns: the layer rests from the first to the
        // second.
        let mut layer_turns = (0..n).filter(|&q| order[q] == Thing::Layers);
        let resting = match (layer_turns.next(), layer_turns.next()) {
            (Some(out), Some(back)) => out < hold && hold <= back,
            _ => false,
        };
        let state = State {
            rest: now.rest.filter(|_| resting),
            voice: if pick(Thing::Voice) { now.voice } else { was.voice },
            warmth: if pick(Thing::Warmth) { now.warmth } else { was.warmth },
            space: if pick(Thing::Space) { now.space } else { was.space },
            register: if pick(Thing::Register) { now.register } else { was.register },
            length: if pick(Thing::Length) { now.length } else { was.length },
            density: if pick(Thing::Density) { now.density } else { was.density },
        };
        let key = if pick(Thing::Key) { now_key } else { was_key };
        let theme = if pick(Thing::Theme) { now_theme } else { was_theme };
        let closes = self.round.is_some_and(|r| i + self.close_units(r).min(r.units) >= r.units);
        let [thin, own] = LONGFORM.density;
        let thinned = if own > thin { (own - state.density) / (own - thin) } else { 0.0 };
        let energy = if closes { 0.2 } else { js::clamp(0.5 - 0.12 * f64::from(u8::from(state.rest.is_some())) - 0.15 * thinned + 0.05 * f64::from(state.register), 0.0, 1.0) };
        Span {
            journey: Journey::Wander,
            role: if closes { Role::Close } else { Role::Holding },
            part: e.saturating_mul(self.turn_count.max(1) as u64).saturating_add(hold as u64),
            at: x - from,
            of: ends[hold.min(TURNS_MAX - 1)] - from,
            theme,
            theme_seed: (theme == Theme::B).then(|| self.second_seed(0)),
            fragment: false,
            variation: if closes { Variation::Outline } else { Variation::Plain },
            key,
            bpm: self.bpm,
            energy,
            closes,
            bars: self.unit_bars,
            state,
            changed: 0,
            kind: Kind::Full,
            pick: 0.0,
            calls: false,
            prepares: false,
        }
    }

    // ------------------------------------------------------------ the map

    /// The piece's own line of the map (`dlgen --map`'s first), as one
    /// object of the generator's canonical text.
    pub fn write<S: Sink>(&self, w: &mut Writer<S>) {
        w.obj();
        w.key("arcUnits");
        w.num(self.arc_units as f64);
        w.key("arcs");
        w.num(self.arcs as f64);
        w.key("asked");
        w.bool(self.asked);
        w.key("cycle");
        w.list();
        for k in self.cycle.iter().take(self.cycle_len) {
            w.str(k.name());
        }
        w.end_list();
        w.key("epochUnits");
        w.num(self.epoch_units as f64);
        w.key("grain");
        w.str(self.grain.name());
        w.key("home");
        write_key(w, self.home);
        w.key("journey");
        w.str(self.journey.name());
        w.key("related");
        w.list();
        for k in self.related() {
            write_key(w, k);
        }
        w.end_list();
        w.key("second");
        w.bool(self.second);
        w.key("sectionBars");
        w.num(self.section_bars);
        w.key("sections");
        match self.round {
            Some(r) => w.num(r.sections as f64),
            None => w.null(),
        }
        w.key("small");
        w.bool(self.small);
        w.key("turns");
        w.num(self.turn_count as f64);
        w.key("unitBars");
        w.num(self.unit_bars);
        w.key("unitSeconds");
        w.num(self.unit_seconds);
        w.key("units");
        match self.round {
            Some(r) => w.num(r.units as f64),
            None => w.null(),
        }
        w.end_obj();
    }
}

fn write_key<S: Sink>(w: &mut Writer<S>, k: Key) {
    w.obj();
    w.key("relation");
    w.str(k.relation.name());
    w.key("root");
    w.str(NOTE_NAMES[usize::from(k.root) % 12]);
    w.key("scale");
    w.str(SCALES.get(k.scale).map_or("", |s| s.id));
    w.end_obj();
}

/// An epoch's holds, drawn as `holds` (any unit), fitted to fill `total`:
/// each kept as far from the epoch's mean as it was drawn from the draws'
/// mean, unless that would take it under `lo` or past `hi`, where every
/// hold is drawn in toward the mean alike, just enough. So the holds still
/// sum to the epoch, and no hold outlasts the data's longest where the
/// epoch has turns enough to fill it (`total` at most `hi` a turn); an
/// epoch with fewer splits itself evenly. (Scaling the draws to the epoch,
/// as 102a did, let one long draw beside short ones outlast 300 s by
/// nearly twice: session 12 measured 587 s.)
fn fit_holds(holds: &mut [f64], total: f64, lo: f64, hi: f64) {
    let n = holds.len();
    if n == 0 {
        return;
    }
    let count = n as f64;
    let mean = total / count;
    let drawn = holds.iter().sum::<f64>() / count;
    let (most, least) = holds.iter().fold((f64::NEG_INFINITY, f64::INFINITY), |(m, l), &x| (js::max(m, x), js::min(l, x)));
    // How far the draws may stand from the mean: as drawn, or less.
    let mut k: f64 = 1.0;
    if most > drawn {
        k = k.min(js::max(0.0, hi - mean) / (most - drawn));
    }
    if least < drawn {
        k = k.min(js::max(0.0, mean - lo) / (drawn - least));
    }
    for h in holds.iter_mut() {
        *h = mean + (*h - drawn) * k;
    }
}

/// What differs between a unit and the one after it (`changed`).
fn difference(a: &Span, b: &Span) -> u16 {
    let bit = |differs: bool, bit: u16| if differs { bit } else { 0 };
    bit(a.role != b.role, changed::ROLE)
        | bit((a.theme, a.theme_seed, a.fragment) != (b.theme, b.theme_seed, b.fragment), changed::THEME)
        | bit(a.variation != b.variation, changed::VARIATION)
        | bit(a.key != b.key, changed::KEY)
        | bit(a.state.rest != b.state.rest, changed::LAYERS)
        | bit(a.state.voice != b.state.voice, changed::VOICE)
        | bit(a.state.warmth != b.state.warmth, changed::WARMTH)
        | bit(a.state.space != b.state.space, changed::SPACE)
        | bit(a.state.register != b.state.register, changed::REGISTER)
        | bit(a.state.length != b.state.length, changed::LENGTH)
        | bit(a.state.density != b.state.density, changed::DENSITY)
}

impl Span {
    /// The unit as one object of the generator's canonical text: a line of
    /// the map (`dlgen --map`, `tools/longform.mjs`).
    pub fn write<S: Sink>(&self, w: &mut Writer<S>) {
        w.obj();
        w.key("at");
        w.num(self.at as f64);
        w.key("bars");
        w.num(self.bars);
        w.key("bpm");
        w.num(self.bpm);
        w.key("calls");
        w.bool(self.calls);
        w.key("changed");
        w.list();
        for (x, name) in changed::NAMES.iter().enumerate() {
            if self.changed & (1 << x) != 0 {
                w.str(name);
            }
        }
        w.end_list();
        w.key("closes");
        w.bool(self.closes);
        w.key("density");
        w.num(self.state.density);
        w.key("energy");
        w.num(js::round(self.energy * 1000.0) / 1000.0);
        w.key("fragment");
        w.bool(self.fragment);
        w.key("journey");
        w.str(self.journey.name());
        w.key("key");
        write_key(w, self.key);
        w.key("length");
        w.num(f64::from(self.state.length));
        w.key("movement");
        match self.journey {
            Journey::Arc => w.str(self.kind.name()),
            Journey::Wander if self.kind == Kind::Interlude => w.str(self.kind.name()),
            Journey::Wander => w.null(),
        }
        w.key("of");
        w.num(self.of as f64);
        w.key("part");
        w.num(self.part as f64);
        w.key("pick");
        w.num(js::round(self.pick * 1000.0) / 1000.0);
        w.key("prepares");
        w.bool(self.prepares);
        w.key("recall");
        match self.theme {
            Theme::Recall(k) => w.num(k as f64),
            _ => w.null(),
        }
        w.key("register");
        w.num(f64::from(self.state.register));
        w.key("rest");
        match self.state.rest {
            Some(l) => w.str(l.name()),
            None => w.null(),
        }
        w.key("role");
        w.str(self.role.name());
        w.key("space");
        w.num(self.state.space);
        w.key("theme");
        w.str(match self.theme {
            Theme::Rest => "rest",
            Theme::A => "A",
            Theme::B | Theme::Recall(_) => "B",
        });
        w.key("themeSeed");
        match self.theme_seed {
            Some(s) => w.num(f64::from(s)),
            None => w.null(),
        }
        w.key("variation");
        w.str(self.variation.name());
        w.key("voice");
        match self.state.voice {
            Some((l, _)) => w.str(l.name()),
            None => w.null(),
        }
        w.key("voicePick");
        match self.state.voice {
            Some((_, pick)) => w.num(pick),
            None => w.null(),
        }
        w.key("warmth");
        w.num(self.state.warmth);
        w.end_obj();
    }
}
