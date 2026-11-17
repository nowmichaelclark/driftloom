//! colorTone (queue item 101e; batch 6's #5, approved by Mikey 2026-11-08):
//! a held 13th over the major chords and an 11th over the minor ones, on a
//! share of the chords, decided here, where the chords are made, once the
//! choices are drawn and the chords' octave is taken (`Render::finish_part`).
//!
//! The Ears 13 sketch heard the chords through a silent organ, so a loop
//! whose keys arpeggiate (one note an event) formed no chord and drew no
//! color, and a loop holding one chord drew one color or none for good. The
//! generator knows its chords: each slot of the progression is read whole,
//! and drawn on its own, so both limits go. What it sounds like is the
//! sketch's (`blocks::color_tone`, the chords channel's own block): voiced
//! just above the chord's top note (C6 at the most), soft, swelling in over
//! 0.35 s and letting go over 0.6 s, its level following the chord layer's.
//!
//! Over a chord with a major third, the 13 (a major sixth over the root);
//! over one with a minor third, the 11 (a perfect fourth). A chord with no
//! third, a fifth that is not perfect (a flat or a sharp one), or the color
//! already in it gets none, nor one whose color is not in its own scale (the
//! loop's mode, or the chord's spelled one): so never a flat ninth, a flat
//! thirteenth over a minor chord or a natural 11 over a major one, and no
//! color the chord's mode lacks. Every chord is drawn on the loop's own
//! stream, so nothing else the loop draws moves.

use super::data::COLORTONE;
use super::js;
use super::list::{Empty, List};
use super::plan::{ChordEvent, NOTES_MAX, SLOTS_MAX, Slot};
use super::render;
use super::rng::Rng;
use super::spec::Spec;

/// colorTone's own stream (a salt on the loop's seed).
const COLOR_SALT: u32 = 0x636f_6c72;

/// The highest a color is voiced: C6, the sketch's.
pub const TOP: i16 = 84;

/// The 13 over a major chord and the 11 over a minor one, in semitones over
/// the root.
const THIRTEENTH: i16 = 9;
const ELEVENTH: i16 = 5;

/// One held color: where it starts in the chords layer, how long it holds
/// (steps), its pitch (MIDI), and the chord event it sounds with (its index
/// in the layer): silent where that event is (a gap, a rest, a pass's
/// breath).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub step: f64,
    pub dur: f64,
    pub midi: i16,
    pub anchor: u16,
}

impl Empty for Color {
    const EMPTY: Self = Color { step: 0.0, dur: 0.0, midi: 0, anchor: 0 };
}

/// A loop's colors, a slot's at most each.
pub type Colors = List<Color, SLOTS_MAX>;

/// A pitch class, 0 to 11.
fn pc(n: i32) -> i32 {
    n.rem_euclid(12)
}

#[allow(clippy::cast_possible_truncation)] // MIDI notes, whole and small
fn whole(x: f64) -> i32 {
    js::round(x) as i32
}

/// The color slot `slot`'s chord takes, as semitones over its root (the 13
/// or the 11), or none: the chord's third says which; a chord with no
/// third, a fifth that is not perfect, the color already in it, or a color
/// its scale lacks takes none.
#[must_use]
pub fn interval(slot: &Slot, spec: &Spec) -> Option<i16> {
    let root = whole(slot.root_midi);
    let has = |iv: i32| slot.notes.iter().any(|&n| pc(i32::from(n) - root) == iv);
    let iv = if has(4) {
        THIRTEENTH
    } else if has(3) {
        ELEVENTH
    } else {
        return None;
    };
    if ((has(6) || has(8)) && !has(7)) || has(i32::from(iv)) {
        return None;
    }
    let scale = slot.steps.as_ref().map_or(spec.steps(), |s| &s[..]);
    let at = pc(root + i32::from(iv) - whole(spec.root));
    scale.iter().any(|&s| whole(s) == at).then_some(iv)
}

/// Whether chord event `e` is slot `slot`'s chord sounding: struck in the
/// slot or held into it, and every note of it one of the slot's, in any
/// octave (the chords' octave moves them all).
fn sounds_in(e: &ChordEvent, slot: &Slot) -> bool {
    let end = slot.start_step + slot.length_steps;
    e.vel != 0.0
        && !e.notes.is_empty()
        && e.step < end
        && e.step + e.dur > slot.start_step
        && e.notes.iter().all(|&n| slot.notes.iter().any(|&m| pc(i32::from(n) - i32::from(m)) == 0))
}

/// The color voiced over the chord `notes` (as played): the lowest of its
/// pitch over the top note, never above C6. Where C6 puts it inside the
/// chord, none if a note of the chord is a semitone from it.
fn voiced(notes: &[i16], root: i32, iv: i16) -> Option<i16> {
    let top = i32::from(notes.iter().copied().max()?);
    let want = pc(root + i32::from(iv));
    let mut m = top + 1 + pc(want - (top + 1));
    while m > i32::from(TOP) {
        m -= 12;
    }
    if m <= top && notes.iter().any(|&n| matches!(pc(i32::from(n) - m), 1 | 11)) {
        return None;
    }
    i16::try_from(m).ok()
}

/// Whether any chord of the loop can take a color (the choice's test).
#[must_use]
pub fn colorable(slots: &[Slot], chords: &[ChordEvent], spec: &Spec) -> bool {
    slots.iter().any(|s| interval(s, spec).is_some() && chords.iter().any(|e| sounds_in(e, s)))
}

/// The loop's colors, into `out`: each slot drawn on colorTone's own stream
/// (`[setting colortone] share`), one draw a slot whether it can take one or
/// not, so a chord's draw is its place's. A slot that takes one holds it from
/// where its chord first sounds in it to its end, voiced over the chord as
/// the layer plays it (`shift`, the chords' octave, `render::lower`); one held on from the
/// slot before (the same color, the same chord event sounding: a long chord)
/// holds on.
pub fn colors(slots: &[Slot], chords: &[ChordEvent], spec: &Spec, shift: f64, out: &mut Colors) {
    out.clear();
    let mut r = Rng::new(js::salted(spec.seed, COLOR_SALT, 1));
    for slot in slots {
        let drawn = r.f() < COLORTONE.share;
        let Some(iv) = interval(slot, spec).filter(|_| drawn) else { continue };
        let mut anchor: Option<(usize, f64)> = None;
        for (i, e) in chords.iter().enumerate() {
            if sounds_in(e, slot) && anchor.is_none_or(|(_, at)| e.step < at) {
                anchor = Some((i, e.step));
            }
        }
        let Some((i, at)) = anchor else { continue };
        let Ok(anchor) = u16::try_from(i) else { continue };
        let mut notes: List<i16, NOTES_MAX> = List::new();
        for &n in slot.notes.iter() {
            notes.push(n);
        }
        render::lower(&mut notes, &slot.notes, shift);
        let Some(midi) = voiced(&notes, whole(slot.root_midi), iv) else { continue };
        let step = js::max(slot.start_step, at);
        let end = slot.start_step + slot.length_steps;
        if let Some(last) = out.last_mut()
            && last.midi == midi
            && last.anchor == anchor
            && last.step + last.dur == step
        {
            last.dur = end - last.step;
            continue;
        }
        out.push(Color { step, dur: end - step, midi, anchor });
    }
}
