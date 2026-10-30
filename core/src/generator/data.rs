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
}

/// echoTime's settings (`[setting echo]`).
#[derive(Clone, Copy, Debug)]
pub struct Echo {
    /// The times in beats, by weight.
    pub beats: NumPool,
    /// The vibes that lean each time, by time.
    pub vibes: &'static [(f64, Table)],
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
        Arc, ArcPeriods, BassLift, Character, Choice, Degree, Echo, FloorDrop, Gait, Kick, LateFloor, Mood, NumPool, PadSweep, Pedal,
        Progressions, SubDuck, Vibe, VoiceCosts, f,
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
