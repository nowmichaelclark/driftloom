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
//! **Not yet heard.** Until session 12 the plan is computed and reported
//! (`dlgen --map`, `tools/longform.mjs`), and a section still plays as
//! `piece` says: the loop's wandered pass.
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

/// How a theme is changed where it comes again. What each does to the notes
/// is session 14's; the plan says which.
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
}

/// What one unit of a piece is: a section, or a phrase of one (`Grain`).
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[allow(clippy::too_many_lines)] // the piece's few numbers, each drawn once, in order
    pub fn of(spec: &Spec, edits: &Edits, present: u8) -> Option<Form> {
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
        let arcs = round.map_or(0, |r| (arcs_wanted as u64).min(r.units / FULL_UNITS).max(1));
        let arc_units = round.map_or_else(|| (js::round(arc_seconds / unit_seconds) as u64).max(FULL_UNITS), |r| r.units / arcs);
        let small = round.is_some() && (arc_units < FULL_UNITS || arc_units as f64 * unit_seconds < LONGFORM.small * 60.0);
        let second = stream(seed, SECOND_SALT, 0).f() < LONGFORM.second;

        // The wander's turns: every thing that was not asked, and that the
        // loop has.
        let home = Key { root: (js::index(spec.root) % 12) as u8, scale: spec.scale, relation: Relation::Home };
        let related = related(spec, home);
        let bit = |l: Layer| 1u8 << l as usize;
        let playing = present & !spec.layers_off;
        let fixed = spec.layers_on | spec.layers_off | spec.rolled;
        let resting = if playing.count_ones() >= 3 { playing & !fixed } else { 0 };
        let voicing = [Layer::Bass, Layer::Chords, Layer::Melody].into_iter().filter(|&l| playing & bit(l) != 0 && spec.voices[l as usize] == 0).fold(0, |m, l| m | bit(l));
        let chords = playing & bit(Layer::Chords) != 0;
        let tune = playing & bit(Layer::Melody) != 0;
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
        turn(Thing::Key, spec.keys == 0 && LONGFORM.key > 0.0 && related.iter().any(Option::is_some));
        turn(Thing::Theme, tune && spec.theme.is_none() && LONGFORM.theme > 0.0);
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
            hand_on: spec.voices[Layer::Melody as usize] == 0,
            turns,
            turn_count,
            epoch_units,
            resting,
            voicing,
            parity,
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
            if self.journey == Journey::Wander && i > 0 && span.role == Role::Holding && span.changed & changed::THINGS != 0 {
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

    /// The start of arc `k` of a finite piece's `arcs`, in units.
    fn arc_start(&self, r: Round, k: u64) -> u64 {
        (u128::from(k) * u128::from(r.units) / u128::from(self.arcs)) as u64
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

    fn arc_unit(&self, i: u64) -> Span {
        // Which arc, and where in it.
        let (k, j, n, last) = match self.round {
            Some(r) => {
                let mut k = (u128::from(i) * u128::from(self.arcs) / u128::from(r.units)) as u64;
                if k + 1 < self.arcs && self.arc_start(r, k + 1) <= i {
                    k += 1;
                }
                let start = self.arc_start(r, k);
                (k, i - start, self.arc_start(r, k + 1) - start, k + 1 == self.arcs)
            }
            None => (i / self.arc_units, i % self.arc_units, self.arc_units, false),
        };
        // The arc's own draws, in a fixed order.
        let mut r = stream(self.seed, ARC_SALT, k);
        let peak = r.range(LONGFORM.peak[0], LONGFORM.peak[1]);
        let height = r.range(LONGFORM.height[0], LONGFORM.height[1]);
        let keys = self.related.iter().flatten().count();
        let first = r.pick_index(keys.max(1));
        let key = |q: usize| self.related.iter().flatten().nth(q % keys.max(1)).copied().unwrap_or(self.home);
        let second_key = key(first);
        let vary = |table: Table, r: &mut Rng| {
            let allowed = |name: &str| self.hand_on || name != Variation::Voice.name();
            r.weighted_index(table.iter().map(|&(name, w)| if allowed(name) { w } else { 0.0 })).and_then(|x| table.get(x)).map_or(Variation::Register, |x| Variation::named(x.0))
        };
        let returns = vary(LONGFORM.returns, &mut r);
        let builds = vary(LONGFORM.builds, &mut r);

        let form = if self.small { LONGFORM.short } else { LONGFORM.full };
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
            (e.1[0] + (e.1[1] - e.1[0]) * t) * height
        });
        let b = if self.second { Theme::B } else { Theme::A };
        let mut fragment = false;
        let (theme, variation, key) = match role {
            Role::Opening | Role::Close => (Theme::A, Variation::Outline, self.home),
            Role::Theme | Role::Holding | Role::Turning => (Theme::A, Variation::Plain, self.home),
            Role::Build => (Theme::A, builds, self.home),
            Role::Lift => (Theme::A, if builds == Variation::Plain { Variation::Register } else { builds }, self.home),
            Role::Second if self.second => (Theme::B, Variation::Plain, second_key),
            Role::Second => (Theme::A, if second_key == self.home { Variation::Register } else { Variation::Key }, second_key),
            Role::Development => {
                fragment = true;
                let d = j - from;
                // An A and its answer, unit by unit; a key lasts a pair of
                // them at least, and the data's least time.
                let stay = 2 * js::max(1.0, js::ceil(LONGFORM.stay / (2.0 * self.unit_seconds))) as u64;
                let at = key(first + 1 + (d / stay) as usize);
                let at = if keys == 1 && (d / stay) % 2 == 1 { self.home } else { at };
                if d % 2 == 0 {
                    (Theme::A, Variation::Plain, at)
                } else if !self.second {
                    (Theme::A, Variation::Answered, at)
                } else if last && self.arcs > 1 {
                    (Theme::Recall((d / 2) % (self.arcs - 1)), Variation::Plain, at)
                } else {
                    (Theme::B, Variation::Plain, at)
                }
            }
            Role::Peak => (Theme::A, Variation::Register, second_key),
            Role::Release => (b, Variation::Outline, second_key),
            Role::Return => (Theme::A, returns, self.home),
            Role::Breath => (Theme::Rest, Variation::Plain, self.home),
        };
        let theme_seed = match theme {
            Theme::B => Some(self.second_seed(k)),
            Theme::Recall(of) => Some(self.second_seed(of)),
            Theme::A | Theme::Rest => None,
        };
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
        // The holds: each a drawn length, scaled to the epoch's units.
        let mut lengths = [0.0; TURNS_MAX];
        let mut total = 0.0;
        for l in lengths.iter_mut().take(n) {
            *l = r.range(LONGFORM.hold[0], LONGFORM.hold[1]);
            total += *l;
        }
        let mut sum = 0.0;
        let mut before = 0;
        for q in 0..n {
            sum += lengths[q];
            let room = self.epoch_units.saturating_sub((n - 1 - q) as u64);
            ends[q] = (js::round(sum / total * self.epoch_units as f64) as u64).max(before + 1).min(room).max(before);
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

    /// Whether epoch `e` is one of thing `bit`'s away ones: a two-state
    /// thing is the loop's own every other epoch, so it always moves.
    fn away(&self, bit: u16, e: u64) -> bool {
        (e & 1 == 1) != (self.parity & bit != 0)
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
            among(self.voicing, &mut r).map(|l| (l, r.f())).filter(|_| self.away(changed::VOICE, e))
        };
        let voice = voice_of(e);
        // A layer never rests while its voice is changed: not the layer
        // this epoch voices, nor the one the epoch before did.
        let voiced = |v: Option<(Layer, f64)>| v.map_or(0, |(l, _)| 1u8 << l as usize);
        let resting = self.resting & !voiced(voice) & !voiced(e.checked_sub(1).and_then(voice_of));
        let mut r = draw(Thing::Key);
        let keys = self.related.iter().flatten().count();
        let visit = r.f() < LONGFORM.key;
        let key = self.related.iter().flatten().nth(r.pick_index(keys.max(1))).copied().filter(|_| visit && !homeward && has(Thing::Key)).unwrap_or(self.home);
        let state = State {
            rest: among(resting, &mut draw(Thing::Layers)),
            voice,
            warmth: if has(Thing::Warmth) { draw(Thing::Warmth).range(-1.0, 1.0) } else { 0.0 },
            space: if has(Thing::Space) { draw(Thing::Space).range(-1.0, 1.0) } else { 0.0 },
            register: if has(Thing::Register) && self.away(changed::REGISTER, e) { sign(&mut draw(Thing::Register)) } else { 0 },
            length: if has(Thing::Length) && self.away(changed::LENGTH, e) { sign(&mut draw(Thing::Length)) } else { 0 },
            density: if has(Thing::Density) { draw(Thing::Density).range(thin, own) } else { 1.0 },
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
        w.key("of");
        w.num(self.of as f64);
        w.key("part");
        w.num(self.part as f64);
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
