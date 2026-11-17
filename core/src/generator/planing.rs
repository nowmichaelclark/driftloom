//! planingSevenths (queue item 101f; batch 6's #10, Ears 13b, approved by
//! Mikey 2026-11-09): an option of the progression's per-loop draw, today's
//! first. Every chord a major seventh, the roots a whole step or a minor
//! third apart, either way: the shape sliding rather than a progression in
//! a key, as four of the batch's records plane theirs.
//!
//! The chords are Ears 13b's lever, walk for walk ([`Planer`], on the
//! chords' seed salted as the lever salted it), so a loop that planes plays
//! the chords Mikey heard. The lever left the tune in the key's scale, and
//! it rubbed: up to 47% of its time a semitone over a chord tone ("What the
//! levers did"). Here the rest follows the planed chord. Each slot carries
//! its chord's own scale, the Lydian on its root (its major pentatonic in a
//! mode of five notes), note for note against the loop's mode, as a spelled
//! chord's steps are ([`spelling`], `Slot::steps`), and `Render::spell_over`
//! moves the tune along it, every note to its partner, a semitone at most,
//! so the tune keeps its shape and every note it plays is a tone of the
//! chord, its ninth, or a note of the chord's scale; the bass goes to the
//! nearest note of the scale, a chord tone where two are as near ([`snap`]).
//! Lydian rather than Ionian: Ionian's fourth sits a semitone over the
//! chord's third, the rub the batch measured; the Lydian's raised fourth is
//! the planed chord's own color.
//!
//! What else reads the key follows too: the bass plays the planed roots (the
//! lever's), the pedal and a walking bass are moved into the chord's scale
//! with the tune, glass and the organ ring the chords' own notes, colorTone
//! reads the chord's scale for its 13th, a wandered note and a harmony line
//! step within it, and a long piece's landing and key changes keep the
//! planed sound (`heard`). The tuned kick keeps the key's note (the walk
//! turns around the tonic), for Mikey's ears.
//!
//! The draw ([`planes`]) is per loop, on the loop's seed salted on a stream
//! of its own, outside the list of choices and counted against a loop's
//! changes (queue item 101h), as tuningOffset is (`[setting progression]`); a request may pin
//! it (`progression.kind`). A loop that does not plane draws and plays
//! exactly as before.

use super::choices::lean_of;
use super::data::{PROGRESSION, Pool};
use super::js;
use super::list::List;
use super::plan::{Notes, Slot};
use super::rng::Rng;
use super::spec::Spec;
use super::theory::scale_pitch;

/// The progression's kinds, today's first: the words the request's
/// `progression.kind` pin takes (`js/pins.js`).
pub const KINDS: Pool = &[("today", 1.0), ("planing", 1.0)];

/// The draw's own stream (a salt on the loop's seed).
const DRAW_SALT: u32 = 0x706c_616e;

/// The walk's own stream (a salt on the chords' seed): the lever's, so the
/// chords are the ones Ears 13b sketched.
const WALK_SALT: u32 = 0x504c_4e47;

/// The roots' moves, in semitones: a whole step or a minor third, either
/// way. The walk draws one a chord.
pub const STEPS: [f64; 4] = [2.0, -2.0, 3.0, -3.0];

/// The farthest the walk goes from the tonic, in semitones: a move that would
/// take it farther goes the other way.
const FARTHEST: f64 = 5.0;

/// A major seventh chord, in semitones over its root.
pub const CHORD: [f64; 4] = [0.0, 4.0, 7.0, 11.0];

/// The chord's own scale, the Lydian, in semitones over its root: a loop
/// in a mode of seven notes.
pub const LYDIAN: [f64; 7] = [0.0, 2.0, 4.0, 6.0, 7.0, 9.0, 11.0];

/// The chord's own scale for a loop in a mode of five notes: its major
/// pentatonic (the Lydian's), so the tune stays as gapped as its mode.
pub const PENTATONIC: [f64; 5] = [0.0, 2.0, 4.0, 7.0, 9.0];

/// The chord's own scale for a loop in a mode of six (the whole tone): the
/// Lydian without its raised fourth.
pub const HEXATONIC: [f64; 6] = [0.0, 2.0, 4.0, 7.0, 9.0, 11.0];

/// The chord's own scale with as many notes as the loop's mode has.
#[must_use]
pub fn chord_scale(notes: usize) -> &'static [f64] {
    match notes {
        5 => &PENTATONIC,
        6 => &HEXATONIC,
        _ => &LYDIAN,
    }
}

/// Whether the loop of `spec` planes: drawn on its own stream, for
/// `[setting progression] planing` of the loops before its vibes lean it
/// (never to none), unless the request pinned `progression.kind`.
#[must_use]
pub fn planes(spec: &Spec) -> bool {
    let mut r = Rng::new(js::salted(spec.seed, DRAW_SALT, 1));
    let base = PROGRESSION.planing;
    let share = base * lean_of(Some(PROGRESSION.vibes), spec.vibes.as_ref());
    let drawn = r.f() * (1.0 - base + share) < share;
    let planing = spec.pins.word("progression.kind").map_or(drawn, |k| k == "planing");
    forced(planing)
}

/// A pitch class, 0 to 12 (not included), of `x` semitones.
fn pc(x: f64) -> f64 {
    ((x % 12.0) + 12.0) % 12.0
}

/// A planed chord: its root (a MIDI note), and the scale degree nearest it.
#[derive(Clone, Copy)]
struct Planed {
    root: f64,
    degree: f64,
}

/// The planed progression, drawn as `plan::gen_harmony` goes along its
/// slots (`Planer::notes`, then `Planer::apply` to the slot made): the first
/// chord is the tonic's; each after moves a whole step or a minor third,
/// either way ([`STEPS`]), turning back where that would take it more than a
/// fourth from the tonic. Off, it does nothing, and draws nothing.
pub struct Planer {
    on: bool,
    rng: Rng,
    offset: f64,
    chord: Option<Planed>,
}

impl Planer {
    /// The planer of a harmony drawn from `chords_seed`, on if the loop
    /// planes.
    #[must_use]
    pub fn new(on: bool, chords_seed: u32) -> Self {
        Planer { on, rng: Rng::new(js::salted(chords_seed, WALK_SALT, 1)), offset: 0.0, chord: None }
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
            let by = self.rng.pick(&STEPS).unwrap_or(2.0);
            self.offset += if (self.offset + by).abs() > FARTHEST { -by } else { by };
        }
        let mut degree = 0.0;
        let mut nearest = f64::MAX;
        for (i, &step) in scale.iter().enumerate() {
            let apart = pc(self.offset - step);
            let apart = apart.min(12.0 - apart);
            if apart < nearest {
                nearest = apart;
                degree = js::num(i);
            }
        }
        let root = scale_pitch(tonic, scale, 0.0, 0.0) + self.offset;
        self.chord = Some(Planed { root, degree });
        notes.clear();
        for x in CHORD {
            notes.push((root + x) as i16);
        }
    }

    /// Point the slot just made at the planed root, over the scale degree
    /// nearest it, for the bass and the tune to follow, with the chord's
    /// own scale for the spelling ([`spelling`] of the loop's mode `scale`
    /// in `tonic`).
    pub fn apply(&self, slot: Option<&mut Slot>, tonic: f64, scale: &[f64]) {
        if let (Some(p), Some(slot)) = (self.chord.filter(|_| self.on), slot) {
            slot.degree = p.degree;
            slot.root_midi = p.root;
            slot.bass_degree = p.degree;
            slot.bass_midi = p.root;
            slot.steps = Some(spelling(scale, p.root - tonic));
        }
    }
}

/// The nearer way from `from` to `to` semitones, -6 to 5.
fn near(to: f64, from: f64) -> f64 {
    pc(to - from + 6.0) - 6.0
}

/// The chord's own scale ([`chord_scale`] on the root `root` semitones over
/// the tonic) note for note against the loop's mode `scale`, as a spelled
/// chord's steps are (`Slot::steps`, relative to the tonic): each of the
/// mode's notes, in order, to one of the chord scale's, in order, turned to
/// where they move least (the least in all, then the least at most), so a
/// tune spelled over it keeps its shape (a step stays a step, a note's
/// neighbors its neighbors) and moves a semitone at most in a mode of seven
/// (two in a mode of five).
#[must_use]
pub fn spelling(scale: &[f64], root: f64) -> List<f64, 7> {
    let n = scale.len();
    let mut ch: List<f64, 7> = List::new();
    for &x in chord_scale(n) {
        ch.push(pc(root + x));
    }
    ch.sort_stable_by(|a, b| a < b);
    let mut best = (f64::INFINITY, f64::INFINITY, 0);
    for k in 0..ch.len() {
        let (mut all, mut most) = (0.0, 0.0f64);
        for (i, &s) in scale.iter().enumerate() {
            let by = near(ch[(i + k) % ch.len()], s).abs();
            all += by;
            most = most.max(by);
        }
        if all < best.0 || (all == best.0 && most < best.1) {
            best = (all, most, k);
        }
    }
    let mut out: List<f64, 7> = List::new();
    for i in 0..n.min(ch.len()) {
        out.push(ch[(i + best.2) % ch.len()]);
    }
    out
}

/// `midi` moved into a planed slot's scale (one `Planer::apply` made, of a
/// loop in `tonic`): itself if it is in the chord's scale, else the nearest
/// note that is, the chord's own tone where two are as near (the lower where
/// neither or both are). Every chord scale here has a note within a
/// semitone of any other, and where two are, one is a tone of the chord: so
/// a note moves a semitone at most, onto the chord where it can. The bass
/// takes this (its notes are the planed root's, not the mode's), and any
/// note of the tune outside the mode.
#[must_use]
pub fn snap(midi: f64, tonic: f64, slot: &Slot) -> f64 {
    let Some(st) = &slot.steps else { return midi };
    let p = pc(midi - tonic);
    if st.contains(&p) {
        return midi;
    }
    let root = pc(slot.root_midi - tonic);
    let chord = |q: f64| CHORD.iter().any(|&x| pc(root + x) == q);
    let mut by = 1.0;
    while by < 12.0 {
        let (down, up) = (st.contains(&pc(p - by)), st.contains(&pc(p + by)));
        match (down, up) {
            (true, true) => return if chord(pc(p + by)) && !chord(pc(p - by)) { midi + by } else { midi - by },
            (true, false) => return midi - by,
            (false, true) => return midi + by,
            (false, false) => by += 1.0,
        }
    }
    midi
}

/// The sketch build's lever (queue item 99, `levers`): `--force
/// planingSevenths=on` plans the loop as if it drew it.
#[cfg(feature = "sketch")]
fn forced(drawn: bool) -> bool {
    drawn || crate::levers::option(crate::levers::PLANING_SEVENTHS) != 0
}

/// Without the sketch build, what the loop drew.
#[cfg(not(feature = "sketch"))]
fn forced(drawn: bool) -> bool {
    drawn
}
