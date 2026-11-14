//! The generation data (`data/*.txt`, queue item 78), as compiled into the
//! core by `core/build.rs`: read-only tables, the same doubles the
//! JavaScript reads. A key a file leaves out is `None` here, as it is
//! `undefined` there, and the generator reads each the way the JavaScript
//! does (`c.airy ?? 0.25`).

/// A weighted pool of names: `scales = dorian 3, aeolian 3`.
pub type Pool = &'static [(&'static str, f64)];
/// A weighted pool of numbers: `bars = 4 7, 2 2`.
pub type NumPool = &'static [(f64, f64)];
/// A table of numbers by name: `feel = lift 0.75, energy 0.8`.
pub type Table = &'static [(&'static str, f64)];

/// A number from its bits, which is how `build.rs` writes every one.
const fn f(bits: u64) -> f64 {
    f64::from_bits(bits)
}

/// A gait: tempo band, step count and metre, drawn together.
#[derive(Clone, Copy, Debug)]
pub struct Gait {
    pub weight: f64,
    pub metre: &'static str,
    pub steps_per_bar: f64,
    pub bpm: [f64; 2],
}

/// A step of a written progression: a scale degree, or (spelled) a chord
/// with its spelling, `{ d, flat, major }` (`js/characters.js`, `degree`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Degree {
    pub d: i32,
    pub flat: bool,
    pub major: bool,
    /// Written as an object (`4M`, `6fM`) rather than a plain number.
    pub spelled: bool,
}

impl Degree {
    /// A plain degree.
    #[must_use]
    pub const fn plain(d: i32) -> Self {
        Degree { d, flat: false, major: false, spelled: false }
    }
}

/// A profile's own progressions: for every seven-note mode, or per mode.
#[derive(Clone, Copy, Debug)]
pub enum Progressions {
    List(&'static [&'static [Degree]]),
    PerMode(&'static [(&'static str, &'static [&'static [Degree]])]),
}

/// A profile (`[character id]`).
#[derive(Clone, Copy, Debug)]
pub struct Character {
    pub id: &'static str,
    pub label: Option<&'static str>,
    pub groove: Option<&'static str>,
    pub polymeter: Option<bool>,
    pub micro_loop: Option<bool>,
    pub borrowed_bright: Option<&'static [&'static str]>,
    /// warmth, space, wobble.
    pub tone: [[f64; 2]; 3],
    /// lift, energy, warmth.
    pub feel: [[f64; 2]; 3],
    pub gaits: Option<&'static [Gait]>,
    pub progressions: Option<Progressions>,
    pub bpm: Option<[f64; 2]>,
    pub swing: Option<[f64; 2]>,
    pub hat_density: Option<[f64; 2]>,
    pub steps_per_bar: Option<NumPool>,
    pub bars: Option<NumPool>,
    pub chord_size: Option<NumPool>,
    pub scales: Option<Pool>,
    pub chord_voices: Option<Pool>,
    pub melody_voices: Option<Pool>,
    pub bass_styles: Option<Pool>,
    pub bass_voices: Option<Pool>,
    pub textures: Option<Pool>,
    pub kits: Option<Pool>,
    pub weight: Option<f64>,
    pub level: Option<f64>,
    pub airy: Option<f64>,
    pub drums: Option<f64>,
    pub rest_bar: Option<f64>,
    pub flat_seven: Option<f64>,
    pub quartal: Option<f64>,
    pub pointillist: Option<f64>,
    pub skip_step: Option<f64>,
    pub glide: Option<f64>,
    pub dub_echo: Option<f64>,
    pub rolls: Option<f64>,
    pub four_floor: Option<f64>,
    pub pump: Option<f64>,
    pub drone: Option<f64>,
    pub ornament: Option<f64>,
    pub harmonize: Option<f64>,
    pub chug: Option<f64>,
}

/// A vibe's recipe (`[vibe id]`, `docs/VIBES.md`).
#[derive(Clone, Copy, Debug)]
pub struct Vibe {
    pub id: &'static str,
    pub feel: Option<Table>,
    pub tone: Option<Table>,
    pub profiles: Option<Table>,
    pub modes: Option<Table>,
    pub melody: Option<Table>,
    pub chords: Option<Table>,
    pub bass: Option<Table>,
    pub bass_style: Option<Table>,
    pub texture: Option<Table>,
    pub tempo: Option<f64>,
    pub swing: Option<f64>,
    pub drums: Option<f64>,
    pub hats: Option<f64>,
    pub rests: Option<f64>,
    pub long: Option<f64>,
    pub big_chords: Option<f64>,
    pub six8: Option<f64>,
}

/// A mood: a region of the feeling space (`[mood id]`).
#[derive(Clone, Copy, Debug)]
pub struct Mood {
    pub id: &'static str,
    pub lift: f64,
    pub energy: f64,
    pub warmth: f64,
}

/// A per-loop choice (`[choice id]`, `js/choices.js`).
#[derive(Clone, Copy, Debug)]
pub struct Choice {
    pub id: &'static str,
    /// Today's first, then what it plays when picked.
    pub options: Pool,
    pub weight: Option<f64>,
    pub vibes: Option<Table>,
    /// The vibes that lean each option, by option.
    pub option_vibes: &'static [(&'static str, Table)],
    /// Whether the option a picked choice plays is drawn on a stream of its
    /// own (`ownStream`, queue item 101), so options added to a choice move
    /// no other choice's draw.
    pub own_stream: bool,
    /// Drawn by itself for this share of the loops it may go to, not among
    /// the changes (`alone`, queue item 101b), or `None`.
    pub alone: Option<f64>,
}

/// pedalBass's settings (`[setting pedal]`).
#[derive(Clone, Copy, Debug)]
pub struct Pedal {
    pub styles: &'static [&'static str],
    pub rhythms: Pool,
    pub neighbors: NumPool,
    pub neighbor_vibes: &'static [(f64, Table)],
    pub share: [f64; 2],
    pub low: f64,
    pub high: f64,
}

/// The arc's swing periods in seconds, by the pace a loop's vibes pick
/// (`period.*`; `arcPace`).
#[derive(Clone, Copy, Debug)]
pub struct ArcPeriods {
    pub calm: [f64; 2],
    pub bright: [f64; 2],
    pub rest: [f64; 2],
    pub glacial: [f64; 2],
    pub glacial_near: [f64; 2],
}

/// The arc's settings (`[setting arc]`; `ARC` in `js/choices.js`, with the
/// choice's own weight and vibes in `CHOICES`).
#[derive(Clone, Copy, Debug)]
pub struct Arc {
    /// deep or dip, by weight, each leaned by its `shape_vibes`.
    pub shapes: Pool,
    pub shape_vibes: &'static [(&'static str, Table)],
    pub period: ArcPeriods,
    /// The vibes that weigh a loop toward the calm period, and the bright.
    pub calm: &'static [&'static str],
    pub bright: &'static [&'static str],
    /// The vibes that, with glacial, slow its swing further.
    pub glacial_near: &'static [&'static str],
    /// A glacial loop's chance of the arc outside its choices, times
    /// glacial's weight.
    pub glacial_chance: f64,
    /// Where a deep swing starts (`from.deep`).
    pub from_deep: [f64; 2],
    pub floor: [f64; 2],
    pub dip: f64,
}

/// The tuned kick's settings (`[setting kick]`).
#[derive(Clone, Copy, Debug)]
pub struct Kick {
    /// The lowest and highest note, as MIDI notes (the data's; the note
    /// itself is worked out from the key, `kickOf`).
    pub note: [f64; 2],
    /// The falls, in dB a second.
    pub short: [f64; 2],
    pub long: [f64; 2],
    /// Level by ear against today's kick, the tail's room, the bands' room.
    pub loud: f64,
    pub tail_loud: f64,
    pub band: f64,
    /// How often a wandered pass swaps its kick.
    pub swap: f64,
}

/// subDuck's settings (`[setting subduck]`).
#[derive(Clone, Copy, Debug)]
pub struct SubDuck {
    pub depth: [f64; 2],
    pub recover: [f64; 2],
    /// The recovery's longest, as a share of a beat.
    pub beat: f64,
    /// deepDuck's own ranges (`deep.depth`, `deep.recover`; queue item
    /// 101a), its recovery held to the loop's kick spacing instead.
    pub deep_depth: [f64; 2],
    pub deep_recover: [f64; 2],
}

/// lateFloor's settings (`[setting latefloor]`).
#[derive(Clone, Copy, Debug)]
pub struct LateFloor {
    pub seconds: [f64; 2],
    /// The wait is rounded to this many bars.
    pub round: f64,
    /// The eligibility test: something else plays within this many seconds.
    pub first: f64,
    pub orders: Pool,
    /// The gap between the two arrivals, in bars, by weight.
    pub gap: NumPool,
    pub max_gap: f64,
    /// The gaps from this many bars up are the long end the vibes lean.
    pub gap_long: f64,
    pub long_vibes: Table,
    /// slowOpening's own wait and fade, in seconds (`slow.seconds`,
    /// `slow.fade`; queue item 101a).
    pub slow_seconds: [f64; 2],
    pub slow_fade: [f64; 2],
}

/// floorDrop's shapes (`[setting floordrop]`).
#[derive(Clone, Copy, Debug)]
pub struct FloorDrop {
    pub shapes: Pool,
    /// The shapes that need a bass line.
    pub need_bass: &'static [&'static str],
    pub shape_vibes: &'static [(&'static str, Table)],
    pub stagger_short: f64,
    pub stutter_short: f64,
    pub stutter: [f64; 2],
    pub long_seconds: f64,
    pub long_bars: [f64; 2],
    pub whole_under: f64,
}

/// padSweep's settings (`[setting padsweep]`).
#[derive(Clone, Copy, Debug)]
pub struct PadSweep {
    /// The chord voices it sweeps (its eligibility test).
    pub sustained: &'static [&'static str],
    pub lo: [f64; 2],
    pub oct: [f64; 2],
    pub period: [f64; 2],
    pub q: [f64; 2],
    /// wideSweep's two (`wide.*`, `fast.*`; queue item 101a): each the same
    /// four ranges.
    pub wide: SweepRanges,
    pub fast: SweepRanges,
}

/// One kind of padSweep's ranges: where it closes, how far it opens, one
/// swing and its resonance.
#[derive(Clone, Copy, Debug)]
pub struct SweepRanges {
    pub lo: [f64; 2],
    pub oct: [f64; 2],
    pub period: [f64; 2],
    pub q: [f64; 2],
}

/// padWow's settings (`[setting padwow]`, queue item 101b): the ranges a
/// loop draws its depth (cents) and rate (Hz) from.
#[derive(Clone, Copy, Debug)]
pub struct Wow {
    pub depth: [f64; 2],
    pub rate: [f64; 2],
}

/// dottedPulse's settings (`[setting dottedpulse]`, queue item 101d): the
/// pulse's velocity on the rim, and the 3-3-2's room over the loop's own
/// kick, in dB (the low guard).
#[derive(Clone, Copy, Debug)]
pub struct Dotted {
    pub pulse: f64,
    pub low: f64,
}

/// echoTime's settings (`[setting echo]`).
#[derive(Clone, Copy, Debug)]
pub struct Echo {
    /// The times in beats, by weight.
    pub beats: NumPool,
    /// The vibes that lean each time, by time.
    pub vibes: &'static [(f64, Table)],
}

/// tuningOffset's settings (`[setting tuning]`, queue item 101b): how
/// often a loop plays off A440, how far, and the vibes that lean how often.
#[derive(Clone, Copy, Debug)]
pub struct Tuning {
    /// The share of loops off A440, before the vibes lean it.
    pub share: f64,
    /// How far off, in cents, either way.
    pub cents: [f64; 2],
    /// The vibes that lean the share.
    pub vibes: Table,
}

/// The long pieces' numbers (`data/longform.txt`, queue item 102): what
/// `longform` draws a piece's plan by. The file says what each is.
#[derive(Clone, Copy, Debug)]
pub struct Longform {
    /// The journeys' weights (`arc`, `wander`).
    pub weights: Table,
    /// The arc's weight in Endless, as a share of its own.
    pub endless: f64,
    /// The vibes that lean the arc's weight.
    pub vibes: Table,
    /// An arc's length, in minutes.
    pub minutes: [f64; 2],
    /// The vibes that lean an arc longer.
    pub long: Table,
    /// An arc shorter than this, in minutes, plays the small form.
    pub small: f64,
    /// An arc of fewer sections than this is planned in phrases.
    pub need: f64,
    /// The full form's roles and their shares, in order.
    pub full: Table,
    /// The small form's.
    pub short: Table,
    /// Each role's energy, from its first unit to its last.
    pub energy: &'static [(&'static str, [f64; 2])],
    /// Where in the arc the peak ends.
    pub peak: [f64; 2],
    /// How high an arc's peak is.
    pub height: [f64; 2],
    /// The share of arc pieces with a second theme.
    pub second: f64,
    /// How the return changes theme A, by weight.
    pub returns: Table,
    /// What the build and the lift do to it.
    pub builds: Table,
    /// The least a close lasts, in seconds.
    pub close: f64,
    /// The least share of the home key's notes a related key keeps.
    pub common: f64,
    /// No related key's mode is darker than this, unless home is.
    pub dark: f64,
    /// The least a key lasts in the development, in seconds.
    pub stay: f64,
    /// The movements' kinds and each one's length, a share of a full arc's.
    pub kinds: Table,
    /// A gentle movement's energy: its floor and its peak.
    pub gentle: [f64; 2],
    /// How often a movement's return is prepared on the home key's fifth.
    pub prepare: f64,
    /// How often a pair of the development's units is a call and response.
    pub call: f64,
    /// The least share of its notes a theme's outline leaves out.
    pub outline: f64,
    /// A wander's hold, in seconds.
    pub hold: [f64; 2],
    /// A mean hold of fewer sections than this is planned in phrases.
    pub sections: f64,
    /// How often an epoch of the wander visits a related key.
    pub key: f64,
    /// How often one hands the tune to the second theme.
    pub theme: f64,
    /// The share of the tune's notes that sound, thinnest to its own.
    pub density: [f64; 2],
    /// How far out of the loop's own a turn of the warmth, the space or
    /// the tune's density takes it, at least (a share of the way).
    pub apart: f64,
    /// How far the tone's two move the Sound panel's warmth and space (0 to
    /// 1) at their furthest.
    pub reach: f64,
    /// The chords' lengths, shorter and longer: a share of their own.
    pub lengths: [f64; 2],
    /// How long a move glides, in seconds, where it can.
    pub glide: f64,
}

/// How a long piece's units meet (`[longform joins]`, session 15): how
/// long a join lasts and what it plays, as `longform` and `heard` read it.
#[derive(Clone, Copy, Debug)]
pub struct Joins {
    /// How far the energy rises or falls across a line for a fill or the
    /// floor out where a theme changes (twice this where only a role does).
    pub step: f64,
    /// How long a join lasts before its line, in bars: this many at most,
    /// and at most `share` of its unit.
    pub bars: f64,
    pub share: f64,
    /// A breath's share of a join's length.
    pub breath: f64,
    /// A fill's hits, from the first to the last, as shares of the
    /// instrument's loudest in the loop.
    pub fill: [f64; 2],
    /// How many beats a pivot's chord holds past the line.
    pub hold: f64,
    /// How far under the unit, in dB by its notes (`Heard::under`), a join
    /// may leave the mix before it falls back to a lighter one, or to a
    /// glide (session 16b: never a hole).
    pub hole: f64,
}

/// How an arc's energy is heard (`[longform energy]`, queue item 102c):
/// what a unit of a given energy plays, as `heard` reads it.
#[derive(Clone, Copy, Debug)]
pub struct Energy {
    /// The layers that fade with the energy, each with the energy it is
    /// gone at, the one it is at its floor at, the one it is its own from,
    /// and the floor (a share of its level).
    pub levels: &'static [(&'static str, [f64; 4])],
    /// The chords' and the tune's swell: their level's share at the second
    /// energy and under, their own from the third.
    pub swell: [f64; 3],
    /// The kit's shimmer is silent under the first energy and its own from
    /// the second.
    pub kit: [f64; 2],
    /// The kit's shimmer: the pieces that come in with the energy.
    pub shimmer: Table,
    /// The kit's backbeat, which comes in over the same energies from
    /// `beat`'s share of its velocity.
    pub backbeat: Table,
    pub beat: f64,
    /// How much softer the kit's hits are at the lowest energy.
    pub soft: f64,
    /// The energy the tune thins under, and its thinnest (the share left
    /// out at nothing).
    pub thin: [f64; 2],
    /// Under this energy the chords sound longer.
    pub held: f64,
    /// The tune's double an octave up: silent at the first energy, at the
    /// third's share of the tune's velocity from the second.
    pub double: [f64; 3],
    /// The energy the arc's filter is its own at, and how far either side
    /// its aim is all the way.
    pub aim: [f64; 2],
    /// How much darker the filter sits at the aim's darkest, and how much
    /// shallower the swing is at its brightest: shares of its range.
    pub dark: f64,
    pub open: f64,
    /// How far the space moves the Sound panel's at the aim's furthest.
    pub space: f64,
    /// The least a finite arc's landing lasts, in seconds (a bar at least).
    pub land: f64,
}

/// The bass layer's level in dB, with a kick and without
/// (`[setting basslift]`).
#[derive(Clone, Copy, Debug)]
pub struct BassLift {
    pub kick: f64,
    pub no_kick: f64,
}

/// What each voice costs the voice budget (`[voices costs]` in
/// `data/voices.txt`; `VOICE_COST` in `js/synth.js`), which the player asks
/// as the JavaScript synth does (queue item 82): a voice the table leaves
/// out costs what each call says, `default` where it says nothing.
#[derive(Clone, Copy, Debug)]
pub struct VoiceCosts {
    pub default: f64,
    /// dubStab's: it yields, asking for room and holding none.
    pub dubstab: f64,
    pub cost: Table,
}

// The tables, written by build.rs: every number by its bits, which are long
// hex literals on purpose.
#[allow(clippy::unreadable_literal)]
mod tables {
    use super::{
        Arc, ArcPeriods, BassLift, Character, Choice, Degree, Dotted, Echo, Energy, FloorDrop, Gait, Joins, Kick, LateFloor, Mood, NumPool, PadSweep, Pedal,
        Longform, Progressions, SubDuck, SweepRanges, Tuning, Vibe, VoiceCosts, Wow, f,
    };
    include!(concat!(env!("OUT_DIR"), "/gen_data.rs"));
}
pub use tables::*;

/// A name's number in a table, by lookup.
#[must_use]
pub fn lookup(table: Table, name: &str) -> Option<f64> {
    table.iter().find(|(k, _)| *k == name).map(|x| x.1)
}

/// A name's table among tables by name (`ARC.shapeVibes[id]`), or none.
#[must_use]
pub fn table_named(tables: &[(&'static str, Table)], name: &str) -> Option<Table> {
    tables.iter().find(|(k, _)| *k == name).map(|x| x.1)
}

/// A number's table among tables by number (`ECHO.vibes[b]`), or none.
#[must_use]
pub fn table_at(tables: &[(f64, Table)], x: f64) -> Option<Table> {
    tables.iter().find(|(k, _)| *k == x).map(|t| t.1)
}

/// A profile's index by id.
#[must_use]
pub fn character_index(id: &str) -> Option<usize> {
    CHARACTERS.iter().position(|c| c.id == id)
}

/// `resolveKey`: an id the profiles know, through the legacy names, else
/// dust.
#[must_use]
pub fn resolve_key(key: &str) -> usize {
    const LEGACY: [(&str, &str); 6] =
        [("tape", "dust"), ("hyrule", "glade"), ("field", "thaw"), ("postcard", "haven"), ("plantasia", "bloom"), ("airports", "vapor")];
    if let Some(i) = character_index(key) {
        return i;
    }
    let to = LEGACY.iter().find(|(k, _)| *k == key).map_or("dust", |x| x.1);
    character_index(to).unwrap_or(0)
}

/// A vibe's number (its index in the share-code order) by id.
#[must_use]
pub fn vibe_index(id: &str) -> Option<usize> {
    VIBES.iter().position(|v| v.id == id)
}

impl super::list::Empty for Gait {
    const EMPTY: Self = Gait { weight: 0.0, metre: "", steps_per_bar: 0.0, bpm: [0.0; 2] };
}

impl super::list::Empty for Degree {
    const EMPTY: Self = Degree::plain(0);
}
