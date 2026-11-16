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
//! - **planingSevenths** (`on`): the loop planes as if it drew it (queue item
//!   101f made it the progression's per-loop draw, `generator::planing`; a
//!   request pins it with `progression.kind`). The lever stays so that Ears
//!   13b's job lists render again, now with the tune following the chords.

use core::sync::atomic::{AtomicU8, Ordering};

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

/// The velocity ramp of a floor that has just arrived (`latefloor.FADE`, in
/// `tuning`): `since` steps after its arrival, of `sd` seconds each; 0 at the
/// arrival and 1 once the fade has passed, and 1 when the fade is 0, as the
/// data has it (`Sequencer::schedule_step`).
#[must_use]
pub fn late_fade(since: f64, sd: f64) -> f64 {
    let fade = crate::tuning::get("latefloor.FADE", 0.0);
    if fade > 0.0 { (since * sd / fade).clamp(0.0, 1.0) } else { 1.0 }
}
