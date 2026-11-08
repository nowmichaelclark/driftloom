//! Forceable generator levers that are not in the data (queue item 99), in
//! the `sketch` build only.
//!
//! Item 83's `--force` forces one of `data/choices.txt`'s choices. A choice
//! added there changes the app (its draws, its golden hashes, its bytes),
//! and these are for hearing before anything is approved. So the levers
//! live here, beside [`tuning`](crate::tuning)'s constants, and `Render::force`
//! falls back to them for an id that is not a choice: `dlrender --force
//! tripletKit=on` and a clip job's `"force": {"tripletKit": "on"}` work as
//! they do for `lateFloor`. Nothing draws them: a lever is off (its first
//! option, `today`) until a clip forces it, and off it changes nothing, so a
//! sketch build renders every loop bit for bit as the plain build does.
//!
//! The values are process-wide, like the constants': a render sets them
//! before it starts and leaves them alone. Each clip is its own process.
//! Tests that force one hold a lock of their own for as long as they do
//! (`tests/sketch.rs`).
//!
//! - **tripletKit** (`on`): the kit's hats and ghost hits on the thirds of
//!   the beat instead of its sixteenths, and the kick on the beat
//!   ([`triplet_kit`]).
//! - **dottedPulse** (`pulse`, `kick332`): a soft rim every three eighths,
//!   or the kick on a 3-3-2 ([`dotted_pulse`]).
//! - **planingSevenths** (`on`): every chord a major seventh, the roots a
//!   whole step or a minor third apart ([`PLANING_STEPS`]); drawn in
//!   `plan::gen_harmony`.

use core::sync::atomic::{AtomicU8, Ordering};

use crate::generator::js;
use crate::generator::layers::{DrumEvent, Drums, NO_DRUM};
use crate::generator::plan::{Notes, Slot};
use crate::generator::rng::Rng;
use crate::generator::theory::scale_pitch;

/// A lever: its name, and its options, the first being `today` (off).
pub struct Lever {
    pub id: &'static str,
    pub options: &'static [&'static str],
}

/// The levers' numbers in [`LEVERS`].
pub const TRIPLET_KIT: usize = 0;
pub const DOTTED_PULSE: usize = 1;
pub const PLANING_SEVENTHS: usize = 2;

/// The levers, by number. A name here must not be one of the data's choices
/// (`Render::force` looks there first).
pub const LEVERS: [Lever; 3] = [
    Lever { id: "tripletKit", options: &["today", "on"] },
    Lever { id: "dottedPulse", options: &["today", "pulse", "kick332"] },
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

/// A beat's steps in the metre's grid: 6 in 6/8, else 4.
fn beat_of(six_eight: bool) -> f64 {
    if six_eight { 6.0 } else { 4.0 }
}

/// Move `e` to where a third of the beat falls, which is a step and the
/// fraction of the next the sequencer plays it past it (`micro`), or drop it
/// where it falls on the straight eighth, the middle of the beat.
fn to_third(e: &mut DrumEvent, beat: f64) {
    let at = e.step % beat;
    let third = at * 3.0 / beat;
    if (third - js::floor(third) - 0.5).abs() < 1e-9 {
        e.vel = 0.0;
        e.vel0 = 0.0;
        return;
    }
    let to = js::round(third) * beat / 3.0 + (e.step - at);
    e.step = js::floor(to);
    let rest = to - e.step;
    e.micro = if rest > 1e-9 { Some(rest) } else { None };
}

/// tripletKit: every hit that is not the kick and not on the beat is put on
/// a third of the beat (the hats, the ghost hits and the fill's ticks; a
/// roll keeps the place it was drawn at), the straight eighth (the middle of
/// the beat) is dropped, and a kick off the beat moves to its beat, or goes
/// where another kick already sits there. The hats at an odd step still take
/// the loop's swing, as they do.
pub fn triplet_kit(drums: &mut Drums, six_eight: bool) {
    let beat = beat_of(six_eight);
    for i in 0..drums.len() {
        let mut e = drums[i];
        if e.vel == 0.0 || e.roll || e.step % beat == 0.0 {
            continue;
        }
        if e.inst == "kick" || e.inst == "softkick" {
            let on_beat = e.step - e.step % beat;
            let taken = drums.iter().any(|o| o.vel != 0.0 && (o.step - on_beat).abs() < 1e-9 && o.inst == e.inst);
            if taken {
                e.vel = 0.0;
                e.vel0 = 0.0;
            } else {
                e.step = on_beat;
            }
        } else {
            to_third(&mut e, beat);
        }
        drums[i] = e;
    }
}

/// The levers that re-time the kit, in order (`Render::part`'s last): tripletKit,
/// then dottedPulse. `total` is the kit layer's steps, `steps_per_bar` the
/// loop's. Off, they change nothing.
pub fn drum_levers(drums: &mut Drums, six_eight: bool, total: f64, steps_per_bar: f64) {
    if option(TRIPLET_KIT) != 0 {
        triplet_kit(drums, six_eight);
    }
    let pulse = option(DOTTED_PULSE);
    if pulse != 0 {
        dotted_pulse(drums, pulse, total, steps_per_bar);
    }
}

/// The soft part's velocity for dottedPulse's `pulse`, quiet as the ghost
/// hits are.
const PULSE_VEL: f64 = 0.2;

/// dottedPulse: `pulse` adds a soft rim every three eighths (six sixteenths)
/// across the loop, on the loop's own grid and unaware of its bars (the
/// lilt is against them); `kick332` puts the loop's kick on a 3-3-2 of
/// eight eighths, 0, 6 and 12 of a 16-step bar (the bar's last quarter
/// holds the beat's pickup). Rolls and fills stay as drawn.
pub fn dotted_pulse(drums: &mut Drums, option: usize, total: f64, steps_per_bar: f64) {
    if option == 1 {
        let mut s = 0.0;
        while s < total {
            let mut hit = NO_DRUM;
            hit.step = s;
            hit.inst = "rim".into();
            hit.vel = PULSE_VEL;
            hit.vel0 = PULSE_VEL;
            drums.push(hit);
            s += 6.0;
        }
    } else if option == 2 {
        for e in drums.iter_mut().filter(|e| e.inst == "kick" || e.inst == "softkick") {
            if e.vel == 0.0 || e.roll {
                continue;
            }
            let bar = e.step - e.step % steps_per_bar;
            let at = e.step - bar;
            // The nearest of 0, 6 and 12 in a 16-step bar; the same
            // fractions of the bar in another.
            let unit = steps_per_bar / 16.0;
            let nearest = [0.0, 6.0, 12.0].into_iter().min_by(|a, b| (at - a * unit).abs().total_cmp(&(at - b * unit).abs())).unwrap_or(0.0);
            e.step = bar + nearest * unit;
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn hit(step: f64, inst: &'static str, vel: f64) -> DrumEvent {
        DrumEvent { step, inst: inst.into(), vel, vel0: vel, ..NO_DRUM }
    }

    fn steps(d: &Drums, inst: &str) -> Vec<(f64, f64)> {
        d.iter().filter(|e| e.vel != 0.0 && e.inst == inst).map(|e| (e.step, e.micro.unwrap_or(0.0))).collect()
    }

    #[test]
    fn the_triplet_kit_puts_hats_on_thirds_drops_the_eighth_and_snaps_the_kick() {
        let mut d = Drums::new();
        for s in 0..8 {
            d.push(hit(f64::from(s), "hat", 0.4));
        }
        d.push(hit(0.0, "kick", 0.8));
        d.push(hit(6.0, "kick", 0.6));
        d.push(hit(7.0, "kick", 0.6));
        d.push(hit(4.0, "snare", 0.6));
        triplet_kit(&mut d, false);
        let hats = steps(&d, "hat");
        // Per beat: offsets 0, 1 and 3 stay (0, 4/3, 8/3 of a beat's four
        // steps), the middle one is gone.
        let at = |step: f64, micro: f64| hats.iter().any(|&(s, m)| (s + m - (step + micro)).abs() < 1e-9);
        assert!(at(0.0, 0.0) && at(1.0, 1.0 / 3.0) && at(2.0, 2.0 / 3.0) && at(4.0, 0.0) && at(5.0, 1.0 / 3.0) && at(6.0, 2.0 / 3.0));
        assert_eq!(hats.len(), 6, "{hats:?}");
        // The kicks at 6 and 7 fall to the beat at 4, once.
        assert_eq!(steps(&d, "kick"), vec![(0.0, 0.0), (4.0, 0.0)]);
        assert_eq!(steps(&d, "snare"), vec![(4.0, 0.0)], "the snare was on the beat");
    }

    #[test]
    fn the_triplet_kit_in_six_eight_keeps_the_dotted_beats_thirds() {
        let mut d = Drums::new();
        for s in 0..6 {
            d.push(hit(f64::from(s), "hat", 0.4));
        }
        triplet_kit(&mut d, true);
        assert_eq!(steps(&d, "hat"), vec![(0.0, 0.0), (2.0, 0.0), (4.0, 0.0)]);
    }

    #[test]
    fn a_dotted_pulse_adds_a_rim_every_six_steps_or_moves_the_kick_to_3_3_2() {
        let mut d = Drums::new();
        d.push(hit(0.0, "kick", 0.8));
        d.push(hit(10.0, "kick", 0.6));
        dotted_pulse(&mut d, 1, 32.0, 16.0);
        assert_eq!(steps(&d, "rim").iter().map(|x| x.0).collect::<Vec<_>>(), vec![0.0, 6.0, 12.0, 18.0, 24.0, 30.0]);
        assert_eq!(steps(&d, "kick"), vec![(0.0, 0.0), (10.0, 0.0)], "the pulse leaves the kick");
        let mut d = Drums::new();
        d.push(hit(0.0, "kick", 0.8));
        d.push(hit(10.0, "kick", 0.6));
        d.push(hit(16.0 + 14.0, "kick", 0.6));
        dotted_pulse(&mut d, 2, 32.0, 16.0);
        assert_eq!(steps(&d, "kick"), vec![(0.0, 0.0), (12.0, 0.0), (28.0, 0.0)]);
    }
}
