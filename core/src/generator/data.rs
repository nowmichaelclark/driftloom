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

// The tables, written by build.rs: every number by its bits, which are long
// hex literals on purpose.
#[allow(clippy::unreadable_literal)]
mod tables {
    use super::{Character, Choice, Degree, Gait, Mood, NumPool, Pedal, Progressions, Vibe, f};
    include!(concat!(env!("OUT_DIR"), "/gen_data.rs"));
}
pub use tables::*;

/// A name's number in a table, by lookup.
#[must_use]
pub fn lookup(table: Table, name: &str) -> Option<f64> {
    table.iter().find(|(k, _)| *k == name).map(|x| x.1)
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
