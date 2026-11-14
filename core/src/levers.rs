//! Forceable generator levers that are not in the data (queue item 99), in
//! the `sketch` build only.
//!
//! Item 83's `--force` forces one of `data/choices.txt`'s choices. A choice
//! added there changes the app (its draws, its golden hashes, its bytes),
//! and these are for hearing before anything is approved. So the levers
//! live here, beside [`tuning`](crate::tuning)'s constants, and `Render::force`
//! falls back to them for an id that is not a choice: `dlrender --force
//! planingSevenths=on` and a clip job's `"force": {"planingSevenths": "on"}`
//! work as they do for `lateFloor`. Nothing draws them: a lever is off (its first
//! option, `today`) until a clip forces it, and off it changes nothing, so a
//! sketch build renders every loop bit for bit as the plain build does.
//!
//! The values are process-wide, like the constants': a render sets them
//! before it starts and leaves them alone. Each clip is its own process.
//! Tests that force one hold a lock of their own for as long as they do
//! (`tests/sketch.rs`).
//!
//! - tripletKit and dottedPulse were levers until queue item 101d made them
//!   choices of the data (`generator::kit`), which `--force tripletKit=on`
//!   and `--force dottedPulse=kick332` force as any other.
//! - **planingSevenths** (`on`): every chord a major seventh, the roots a
//!   whole step or a minor third apart ([`PLANING_STEPS`]); drawn in
//!   `plan::gen_harmony`.

use core::sync::atomic::{AtomicU8, Ordering};

use crate::generator::js;
use crate::generator::plan::{Notes, Slot};
use crate::generator::rng::Rng;
use crate::generator::theory::scale_pitch;

/// A lever: its name, and its options, the first being `today` (off).
pub struct Lever {
    pub id: &'static str,
    pub options: &'static [&'static str],
}

/// The levers' numbers in [`LEVERS`].
pub const PLANING_SEVENTHS: usize = 0;

/// The levers, by number. A name here must not be one of the data's choices
/// (`Render::force` looks there first).
pub const LEVERS: [Lever; 1] = [
    Lever { id: "planingSevenths", options: &["today", "on"] },
];

/// The option number each lever is forced to (0: off).
static FORCED: [AtomicU8; LEVERS.len()] = [const { AtomicU8::new(0) }; LEVERS.len()];

/// Force lever `id` to `option` for the loops made from now on; `None` (or
/// `today`) puts it back.
///
/// # Errors
/// There is no such lever, or it has no such option.
pub fn force(id: &str, option: Option<&str>) -> Result<(), &'static str> {
    let i = LEVERS.iter().position(|l| l.id == id).ok_or("no such choice")?;
    let o = match option {
        None => 0,
        Some(name) => LEVERS[i].options.iter().position(|o| *o == name).ok_or("no such option")?,
    };
    FORCED[i].store(u8::try_from(o).map_err(|_| "no such option")?, Ordering::Relaxed);
    Ok(())
}

/// Lever number `which`'s option number now (0: off).
#[must_use]
pub fn option(which: usize) -> usize {
    FORCED.get(which).map_or(0, |o| usize::from(o.load(Ordering::Relaxed)))
}

/// The option lever `id` is at, by name: none for a name that is not a lever.
#[must_use]
pub fn named(id: &str) -> Option<&'static str> {
    let i = LEVERS.iter().position(|l| l.id == id)?;
    LEVERS[i].options.get(option(i)).copied()
}

/// The roots' moves for planingSevenths, in semitones: a whole step or a
/// minor third, either way. The harmony draws one a chord.
pub const PLANING_STEPS: [f64; 4] = [2.0, -2.0, 3.0, -3.0];

/// The velocity ramp of a floor that has just arrived (`latefloor.FADE`, in
/// `tuning`): `since` steps after its arrival, of `sd` seconds each; 0 at the
/// arrival and 1 once the fade has passed, and 1 when the fade is 0, as the
/// data has it (`Sequencer::schedule_step`).
#[must_use]
pub fn late_fade(since: f64, sd: f64) -> f64 {
    let fade = crate::tuning::get("latefloor.FADE", 0.0);
    if fade > 0.0 { (since * sd / fade).clamp(0.0, 1.0) } else { 1.0 }
}

/// planingSevenths' own stream (a salt on the chords' seed).
const PLANING_SALT: u32 = 0x504c_4e47;

/// A planed chord: its root (a MIDI note), and the scale degree nearest it.
#[derive(Clone, Copy)]
struct Planed {
    root: f64,
    degree: f64,
}

/// planingSevenths' progression, drawn as `plan::gen_harmony` goes along its
/// slots (`Planer::notes`, then `Planer::apply` to the slot made): the first
/// chord is the tonic's; each after moves a whole step or a minor third,
/// either way ([`PLANING_STEPS`]), turning back where that would take it more
/// than a fourth from the tonic. Off (the lever at `today`), it does nothing.
pub struct Planer {
    on: bool,
    rng: Rng,
    offset: f64,
    chord: Option<Planed>,
}

impl Planer {
    /// The planer of a harmony drawn from `chords_seed`, on if the lever is.
    #[must_use]
    pub fn new(chords_seed: u32) -> Self {
        Planer { on: option(PLANING_SEVENTHS) != 0, rng: Rng::new(js::salted(chords_seed, PLANING_SALT, 1)), offset: 0.0, chord: None }
    }

    /// The next slot's chord, replacing `notes` with a major seventh on the
    /// planed root (`first`: the first slot; `tonic` the loop's root; `scale`
    /// its steps).
    #[allow(clippy::cast_possible_truncation)] // MIDI notes, whole and small
    pub fn notes(&mut self, first: bool, tonic: f64, scale: &[f64], notes: &mut Notes) {
        if !self.on {
            return;
        }
        if !first {
            let by = self.rng.pick(&PLANING_STEPS).unwrap_or(2.0);
            self.offset += if (self.offset + by).abs() > 5.0 { -by } else { by };
        }
        let mut degree = 0.0;
        let mut nearest = f64::MAX;
        for (i, &step) in scale.iter().enumerate() {
            let apart = ((self.offset - step) % 12.0 + 12.0) % 12.0;
            let apart = apart.min(12.0 - apart);
            if apart < nearest {
                nearest = apart;
                degree = js::num(i);
            }
        }
        let root = scale_pitch(tonic, scale, 0.0, 0.0) + self.offset;
        self.chord = Some(Planed { root, degree });
        notes.clear();
        for x in [0.0, 4.0, 7.0, 11.0] {
            notes.push((root + x) as i16);
        }
    }

    /// Point the slot just made at the planed root, over the scale degree
    /// nearest it, for the bass and the tune to follow.
    pub fn apply(&self, slot: Option<&mut Slot>) {
        if let (Some(p), Some(slot)) = (self.chord.filter(|_| self.on), slot) {
            slot.degree = p.degree;
            slot.root_midi = p.root;
            slot.bass_degree = p.degree;
            slot.bass_midi = p.root;
        }
    }
}
