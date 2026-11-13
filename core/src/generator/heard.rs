//! The wander heard (queue item 102b; `docs/LONGFORM.md`, "The wander"): what
//! a long piece's plan asks of a section, as marks and levels on the loop's
//! own events and the mix, the way a wandered pass is.
//!
//! A section of a piece whose journey is the wander is cut into the plan's
//! units (the section, or its phrase lines: `longform::Grain`), and each
//! unit plays the plan's state there: which layer rests, a layer's voice
//! from its profile's pool, the tone (the warmth and the space, around the
//! loop's own), the chords' register and length, how many of the tune's
//! notes sound, and the key (a fifth away, the whole loop moved). What can
//! glide glides over the unit after a move (the plan's `turning` unit): a
//! layer's level as it rests and returns, the tone, the tune thinning or
//! filling. What can't (a voice, a register, a length, a key) changes on the
//! unit's first step, a phrase line or a section line. A unit's start is
//! the unit before's end, so a held unit is level, and nothing jumps.
//!
//! **Nothing new is made.** A layer's level is its channel's gain in the
//! mix (`Synth::set_plan_level`, beside the listener's mute), and its notes
//! are left out only once it is fully at rest; a voice is a name the
//! sequencer plays the same notes with; a register, a key and a length are
//! numbers on the notes it plays; the tune thins by leaving out its weakest
//! notes, a rank drawn once a note. So a section still costs one loop's
//! making and a few hundred bytes, and any section is found from its number
//! (`Form::at` for each of its units and the one before).
//!
//! **What was asked is never moved**: the plan never picks it (`Form`), and
//! a voice is only ever one the profile's pool has for that layer, never a
//! sung one (those carry vowels of their own) nor one where the loop sings
//! as a choir.
//!
//! **A finite wander closes softly**: over its close every layer eases down
//! (to `CLOSE_LEVEL`) and the tune thins toward its thinnest; the round after
//! starts as the loop's own, every layer easing back up over its first
//! unit. Endless never closes.
//!
//! **An arc is heard by its energy** (queue item 102c, session 13: `Lift`,
//! `hear_arc`): each unit's energy, smoothed over its neighbors, played the
//! same way, as marks and levels gliding over the unit: the kit, the bass
//! and the texture down a little (gone only as a close thins), the chords
//! and the tune swelling by under a decibel, the kit's shimmer (the hats,
//! the shakers) arriving with the energy and every hit softer below the
//! loop's own, the tune thinner and the chords held longer when it is low,
//! the tune doubled an octave up, softly, toward the peak, and the arc's
//! filter (the loop's own slow swing, `blocks::arc`) aimed darker below
//! the peak and its swing shallower at it, the space opening with it. The
//! loop as composed is the arc's fullest, near its peak. **A finite arc
//! lands**: its last bars play the loop's tonic chord and the tune on the
//! tonic alone (`Landing`), held for half of them and ringing out, after
//! the tune's last phrase, whole; the round after starts at its opening,
//! from the close. Endless plays arc after arc, a breath between, and never
//! lands. A pass of a loop that is no piece is exactly as before.
//!
//! **An arc's themes, variations and keys** (queue item 102d, session 14:
//! `Lift::themed`). A unit plays the loop's own tune, the piece's second
//! (`Tune`: another seed's tune bent into this loop, or the loop's own
//! answered, made by the generator a section ahead, `Generator::prepare`),
//! or one then the other by the halves of each phrase: a fragment (the
//! phrase's first half, then a rest), a call and response (theme A's
//! fragment, the other's in its rests), a theme answered (its first half
//! kept, the second the answer's). One voice throughout: never two tunes
//! at once. A variation is a mark on the notes, a pure function of the
//! theme and the movement's drawn number: the tune an octave away, in
//! another voice of its pool, stretched (each phrase's first half at half
//! speed over held chords), over the relative key's chords, thinned to its
//! outline. A related key a fifth away moves the whole loop (the wander's
//! way); the relative and the parallel move every note along the mode
//! (`key_map`: a table by pitch class, so the chords, the bass, the tune,
//! the texture and a tuned kick move together and stay in the key). Before
//! a return the bass may hold the home key's fifth for a phrase. **An
//! interlude** (a movement in the wander's way) is its energy and the
//! wander's state over it. **A section plays one tune beside the loop's
//! own** (so the generator keeps two: the section playing and the next):
//! where its units name more, the first is the section's and stands in for
//! the others.

// Steps and units are whole numbers far below 2^53.
#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]

use super::canon::{Sink, Writer};
use super::character::Names;
use super::data::{ENERGY, JOINS, LONGFORM};
use super::js;
use super::list::{Empty, Word};
use super::longform::{self, Form, Join, Key, Relation, Role, Span, State, Theme, Variation};
use super::melody::{MELODY_MAX, Melody};
use super::plan::{self, Cx, Notes};
use super::render::Render;
use super::request::Journey;
use super::spec::Metre;
use super::theory::SCALES;
use super::{LAYERS, Layer};

/// The most units a section has: a section of up to 32 bars in phrases of
/// two bars (a length four doesn't divide).
pub const UNITS_MAX: usize = 16;
/// How far every layer eases down over a finite wander's close: its level.
pub const CLOSE_LEVEL: f64 = 0.7;
/// The voices a layer never takes on, nor leaves: sung ones carry vowels of
/// their own, written with their notes.
const SUNG: [&str; 3] = ["vowel", "hum", "choir"];
/// Where the chords may sit once moved an octave: their lowest note and
/// their highest.
const CHORDS_RANGE: [i16; 2] = [36, 96];
/// Where each layer may sit once moved to another key, in `LAYERS`' order
/// (the texture anywhere).
const KEY_RANGE: [[f64; 2]; 5] = [[0.0, 127.0], [33.0, 60.0], [36.0, 96.0], [48.0, 96.0], [0.0, 127.0]];
/// The most chords `has_room` reads one by one, where they are out of order.
const ROOM_MOST: usize = 64;
/// The stream a tune's notes draw their rank on, for thinning.
const THIN_SALT: u32 = 0x5448_494e;
/// The same for the second tune's notes, and the salt an answer's seed is
/// made with.
const SECOND_SALT: u32 = 0x3254_4e45;
const ANSWER_SALT: u32 = 0x414e_5357;
/// Where the tune may sit once a variation moves it an octave: up where its
/// highest stays here or under (then anywhere under the doubling's 96),
/// down where its lowest stays here or over.
const TUNE_RANGE: [f64; 2] = [48.0, 91.0];
/// The layers a key's map moves, a bit each: every pitched one.
const PITCHED: u8 = 0b1_1110;

/// What a phrase half of a unit plays (`Unit::tune`).
pub mod tune {
    /// The loop's own tune.
    pub const OWN: u8 = 0;
    /// The section's second tune (`Heard::second`).
    pub const SECOND: u8 = 1;
    /// Nothing: the tune rests.
    pub const REST: u8 = 2;
}

/// A tune beside the loop's own, as the generator makes it (session 14):
/// which, and from what seed. All zero bytes is none.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tune {
    /// 0 none; 1 a second theme, another seed's tune bent into this loop;
    /// 2 the loop's own tune answered (`melody::answer_figure`).
    pub kind: u8,
    pub seed: u32,
}

impl Tune {
    /// No tune.
    pub const NONE: Tune = Tune { kind: 0, seed: 0 };
    /// A second theme's.
    pub const SECOND: u8 = 1;
    /// An answer's.
    pub const ANSWER: u8 = 2;

    /// The tune unit `span` of a piece asks for beside the loop's own, if
    /// it asks for one.
    #[must_use]
    pub fn wanted(span: &Span) -> Option<Tune> {
        match (span.theme, span.theme_seed) {
            (Theme::B | Theme::Recall(_), Some(seed)) => Some(Tune { kind: Tune::SECOND, seed }),
            (Theme::A, _) if span.variation == Variation::Answered => {
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // a number from 0 to 1 as 32 bits
                let seed = (span.pick * 4_294_967_296.0) as u32 ^ ANSWER_SALT;
                Some(Tune { kind: Tune::ANSWER, seed })
            }
            _ => None,
        }
    }

    /// The one tune section `n` of the piece `form` plays beside the loop's
    /// own: its first unit's that asks for one.
    #[must_use]
    pub fn of(form: &Form, n: u64) -> Tune {
        (0..form.units_in(n)).find_map(|p| Tune::wanted(&form.at(n, p))).unwrap_or(Tune::NONE)
    }
}

/// A unit of a section as the plan has it heard: its steps, and each thing
/// at the unit's start and its end (the gliding ones) or throughout. Every
/// field is the loop's own at zero, so a unit of nothing is all zero bytes.
#[derive(Clone, Copy, Debug)]
pub struct Unit {
    /// Its steps in the section: from its first, to its end.
    pub from: f64,
    pub to: f64,
    /// Each layer's level, as how far it is down (0 its own, 1 at rest), at
    /// the unit's start and its end, in `LAYERS`' order.
    pub cut: [[f64; 2]; 5],
    /// The warmth and the space, as how far each is moved on the Sound
    /// panel's 0 to 1 (`reach` at most), at the start and the end.
    pub tone: [[f64; 2]; 2],
    /// How many of the tune's notes are left out (0 none), at the start and
    /// the end.
    pub thin: [f64; 2],
    /// The voice each layer plays in place of its own (empty: its own).
    pub voice: [Word; 5],
    /// The chords moved, in semitones (an octave, or none).
    pub register: i16,
    /// The chords' lengths, as a share of their own (0: their own).
    pub stretch: f64,
    /// Each layer moved to another key, in semitones (0 none).
    pub shift: [i16; 5],
    /// An arc's (session 13): how much softer the kit's hits are, a share
    /// of their velocity, at the start and the end (0 their own).
    pub soft: [f64; 2],
    /// How far down the kit's shimmer is (the hats, the shakers, the
    /// jingles: `[longform energy] shimmer`), a share of its velocity, at
    /// the start and the end (0 its own, 1 silent).
    pub shimmer: [f64; 2],
    /// How far down the kit's backbeat is (the snares, the claps, the rims,
    /// the brushes: `[longform energy] backbeat`), likewise.
    pub backbeat: [f64; 2],
    /// The tune doubled this many semitones up (0 none: an octave, toward
    /// the peak), and the double's share of the tune's velocity at the
    /// start and the end.
    pub double: i16,
    pub doubled: [f64; 2],
    /// The arc's filter's aim, at the start and the end: -1 its darkest, 0
    /// the loop's own swing, 1 its brightest.
    pub aim: [f64; 2],
    /// What each half of a phrase plays (session 14, `tune`): the loop's
    /// own tune, the section's second, or a rest.
    pub tune: [u8; 2],
    /// The tune moved, in semitones (an octave, a variation's register).
    pub lift: i16,
    /// Whether the tune is stretched: each phrase's first half at half
    /// speed.
    pub stretched: bool,
    /// Another mode's key: how far each pitch class, counted from the home
    /// note, moves (`key_map`), and the layers it moves, a bit each.
    pub map: [i8; 12],
    pub mapped: u8,
    /// The bass holds this note (0: its own) from section step
    /// `pedal_from` to the unit's end: a return prepared.
    pub pedal: i16,
    pub pedal_from: f64,
    /// How the unit meets the next at its end (session 15, `Join`), as
    /// played: from section step `join_from` to the unit's end, the layers
    /// `out` leave their notes out (a bit each, `LAYERS`' order; the tails
    /// ring).
    pub join: Join,
    pub join_from: f64,
    pub out: u8,
    /// A fill: the kit's instrument `fill` struck every `fill_every` steps
    /// from `join_from`, rising from the first velocity to the second.
    pub fill: Word,
    pub fill_every: f64,
    pub fill_vel: [f64; 2],
    /// A pivot: the chord both keys share (`pivot_count` notes), struck on
    /// `join_from` in the voice of the loop's chord `pivot_from`, at
    /// `pivot_vel`, held `pivot_hold` steps (across the line).
    pub pivot: [i16; 3],
    pub pivot_count: usize,
    pub pivot_from: usize,
    pub pivot_vel: f64,
    pub pivot_hold: f64,
}

impl Empty for Unit {
    const EMPTY: Self = Unit {
        from: 0.0,
        to: 0.0,
        cut: [[0.0; 2]; 5],
        tone: [[0.0; 2]; 2],
        thin: [0.0; 2],
        voice: [Word::EMPTY; 5],
        register: 0,
        stretch: 0.0,
        shift: [0; 5],
        soft: [0.0; 2],
        shimmer: [0.0; 2],
        backbeat: [0.0; 2],
        double: 0,
        doubled: [0.0; 2],
        aim: [0.0; 2],
        tune: [0; 2],
        lift: 0,
        stretched: false,
        map: [0; 12],
        mapped: 0,
        pedal: 0,
        pedal_from: 0.0,
        join: Join::None,
        join_from: 0.0,
        out: 0,
        fill: Word::EMPTY,
        fill_every: 0.0,
        fill_vel: [0.0; 2],
        pivot: [0; 3],
        pivot_count: 0,
        pivot_from: 0,
        pivot_vel: 0.0,
        pivot_hold: 0.0,
    };
}

impl Unit {
    /// Where `step` is in the unit, from 0 at its first to 1 at its end.
    fn along(&self, step: f64) -> f64 {
        if self.to > self.from { js::clamp((step - self.from) / (self.to - self.from), 0.0, 1.0) } else { 1.0 }
    }

    /// A gliding thing at `step`: its start and its end, met in a line.
    fn glide(pair: [f64; 2], t: f64) -> f64 {
        pair[0] + (pair[1] - pair[0]) * t
    }

    /// Layer `layer`'s level at `step` (1 its own).
    #[must_use]
    pub fn level(&self, layer: Layer, step: f64) -> f64 {
        1.0 - Self::glide(self.cut[layer as usize], self.along(step))
    }

    /// Whether `layer` rests throughout the unit: its notes are left out.
    #[must_use]
    pub fn rests(&self, layer: Layer) -> bool {
        self.cut[layer as usize] == [1.0, 1.0]
    }

    /// Whether `layer`'s notes are left out at `step` for the join at the
    /// unit's end (session 15): the floor out, the tune alone, a breath,
    /// the chords under a pivot.
    #[must_use]
    pub fn joins_out(&self, layer: Layer, step: f64) -> bool {
        self.out & (1 << layer as usize) != 0 && step >= self.join_from && step < self.to
    }

    /// The fill's hit at `step`, if one falls there: its instrument and its
    /// share of the instrument's loudest (before the arc's kit marks).
    #[must_use]
    pub fn fill_at(&self, step: f64) -> Option<(&'static str, f64)> {
        if self.join != Join::Fill || step < self.join_from || step >= self.to || !(self.fill_every > 0.0) || (step - self.join_from) % self.fill_every != 0.0 {
            return None;
        }
        let span = self.to - self.join_from - self.fill_every;
        let t = if span > 0.0 { (step - self.join_from) / span } else { 1.0 };
        Some((self.fill.get(), Self::glide(self.fill_vel, js::clamp(t, 0.0, 1.0))))
    }

    /// The pivot's chord, where it is struck at `step`.
    #[must_use]
    pub fn pivot_at(&self, step: f64) -> Option<&[i16]> {
        (self.join == Join::Pivot && self.pivot_count > 0 && step == self.join_from).then(|| &self.pivot[..self.pivot_count])
    }

    /// The warmth's and the space's moves at `step`.
    #[must_use]
    pub fn tone_at(&self, step: f64) -> [f64; 2] {
        let t = self.along(step);
        [Self::glide(self.tone[0], t), Self::glide(self.tone[1], t)]
    }

    /// Whether anything glides in the unit (its levels, its tone, its tune,
    /// an arc's aim).
    #[must_use]
    pub fn glides(&self) -> bool {
        let moves = |p: &[f64; 2]| p[0] != p[1];
        self.cut.iter().any(moves) || self.tone.iter().any(moves) || moves(&self.thin) || moves(&self.aim)
    }

    /// The tune's double's share of its velocity at `step`.
    #[must_use]
    pub fn doubled_at(&self, step: f64) -> f64 {
        Self::glide(self.doubled, self.along(step))
    }

    /// An arc's filter aim at `step`.
    #[must_use]
    pub fn aim_at(&self, step: f64) -> f64 {
        Self::glide(self.aim, self.along(step))
    }

    /// How far `layer`'s note `midi` moves in the unit, in semitones: to
    /// another mode's key along the map (by the note's place from the home
    /// note `root`), then the layer's own move (a key a fifth away, an
    /// octave to stay in range; the chords' register; the tune's lift).
    #[must_use]
    pub fn moved(&self, layer: Layer, midi: f64, root: f64) -> f64 {
        let k = layer as usize;
        let mut by = f64::from(self.shift[k]);
        if self.mapped & (1 << k) != 0 {
            let pc = js::index((js::round(midi - root) % 12.0 + 12.0) % 12.0) % 12;
            by += f64::from(self.map[pc]);
        }
        by + match layer {
            Layer::Chords => f64::from(self.register),
            Layer::Melody => f64::from(self.lift),
            _ => 0.0,
        }
    }

    /// How far the home note itself moves for `layer` (a tuned kick follows
    /// the bass's).
    #[must_use]
    pub fn tonic(&self, layer: Layer) -> i16 {
        let k = layer as usize;
        self.shift[k] + if self.mapped & (1 << k) != 0 { i16::from(self.map[0]) } else { 0 }
    }

    /// How much of its velocity a kit hit keeps at `step` (1 all), by its
    /// piece.
    #[must_use]
    pub fn kit_at(&self, step: f64, piece: Piece) -> f64 {
        let t = self.along(step);
        let arrived = match piece {
            Piece::Shimmer => 1.0 - Self::glide(self.shimmer, t),
            Piece::Backbeat => 1.0 - Self::glide(self.backbeat, t),
            Piece::Other => 1.0,
        };
        (1.0 - Self::glide(self.soft, t)) * arrived
    }
}

/// What a kit hit is to an arc's energy (session 13): its shimmer, its
/// backbeat, or another piece (the kick).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    Shimmer,
    Backbeat,
    Other,
}

impl Piece {
    /// The piece a kit instrument is.
    #[must_use]
    pub fn of(inst: &str) -> Piece {
        if super::data::lookup(ENERGY.shimmer, inst).is_some() {
            Piece::Shimmer
        } else if super::data::lookup(ENERGY.backbeat, inst).is_some() {
            Piece::Backbeat
        } else {
            Piece::Other
        }
    }
}

/// A finite arc's landing (session 13): over the round's last bars the
/// loop's tonic chord and the tune on the tonic, alone, held for half of
/// them and ringing out. Every field is nothing at zero.
#[derive(Clone, Copy)]
pub struct Landing {
    /// Whether the section lands, from section step `at` (its first, where
    /// the landing began in a section before), the two notes held `hold`
    /// steps; and whether it strikes them (the landing starts in it).
    pub on: bool,
    pub at: f64,
    pub hold: f64,
    pub strikes: bool,
    /// The tonic chord, as the loop voices it where it plays one (in the
    /// voice of `chord_from`, the loop's chord it is taken from), at
    /// `chord_vel`; empty with no chords.
    pub chord: Notes,
    pub chord_from: usize,
    pub chord_vel: f64,
    /// The tune's tonic (0 none), in the voice of tune note `tune_from`, at
    /// `tune_vel`.
    pub tune: f64,
    pub tune_from: usize,
    pub tune_vel: f64,
    /// Whether the chord is rolled, a note after another (a loop that
    /// plays its chords a note at a time).
    pub rolled: bool,
}

impl core::fmt::Debug for Landing {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Landing").field("on", &self.on).field("at", &self.at).field("hold", &self.hold).field("chord", &&self.chord[..]).field("tune", &self.tune).finish_non_exhaustive()
    }
}

impl Landing {
    /// No landing.
    pub const NONE: Landing = Landing { on: false, at: 0.0, hold: 0.0, strikes: false, chord: Notes::EMPTY, chord_from: 0, chord_vel: 0.0, tune: 0.0, tune_from: 0, tune_vel: 0.0, rolled: false };
}

/// What a section's plan has heard: its units, and whether it is a
/// wander's at all (none: the pass is exactly as before).
#[derive(Clone, Copy, Debug)]
pub struct Heard {
    pub on: bool,
    units: [Unit; UNITS_MAX],
    count: usize,
    /// The loop's seed, for the tune's ranks, and its steps a bar.
    seed: u32,
    spb: f64,
    /// Each tune note's place among the tune's notes by rank, from just
    /// over 0 to 1 (`sounds`): its share of the notes ranked at or under it.
    places: [f32; MELODY_MAX],
    /// Whether it is an arc's (session 13), not a wander's.
    pub arc: bool,
    /// An arc's: the steps in which the tune sounds whole (its last phrase
    /// before the landing; none where the two are equal), and the landing.
    whole: [f64; 2],
    pub landing: Landing,
    /// The tune the section plays beside the loop's own (session 14; none:
    /// `Tune::NONE`), a phrase's steps and its first half's, and the home
    /// note.
    pub second: Tune,
    pub phrase: f64,
    pub half: f64,
    pub root: f64,
    /// Whether the section plays a close the length slider asked for
    /// (`stop_at`): the track ends ringing out, not faded.
    pub stops: bool,
}

impl Heard {
    /// Nothing heard.
    pub const NONE: Heard = Heard {
        on: false,
        units: [Unit::EMPTY; UNITS_MAX],
        count: 0,
        seed: 0,
        spb: 0.0,
        places: [0.0; MELODY_MAX],
        arc: false,
        whole: [0.0; 2],
        landing: Landing::NONE,
        second: Tune::NONE,
        phrase: 0.0,
        half: 0.0,
        root: 0.0,
        stops: false,
    };

    /// What the tune plays at section step `step` (`tune`): the unit's, by
    /// the half of the phrase the step is in.
    #[must_use]
    pub fn tune_at(&self, step: f64) -> u8 {
        let Some(u) = self.unit(step) else { return tune::OWN };
        let later = self.phrase > 0.0 && step % self.phrase >= self.half;
        u.tune[usize::from(later)]
    }

    /// Whether note `i` of the section's second tune sounds at `step`: as
    /// `sounds`, by a rank of its own.
    #[must_use]
    pub fn second_sounds(&self, i: usize, step: f64) -> bool {
        let Some(u) = self.unit(step) else { return true };
        let thin = Unit::glide(u.thin, u.along(step));
        thin <= 0.0 || rank(self.seed ^ SECOND_SALT, i) < 1.0 - thin
    }

    /// Where a stretched tune's note sounding at section step `step` is
    /// written: the phrase's first half at half speed, so the note at
    /// phrase step `x` sounds at `2x`. None where the unit is not
    /// stretched, or no note falls on the step.
    #[must_use]
    pub fn stretched_from(&self, step: f64) -> Option<f64> {
        let u = self.unit(step)?;
        if !u.stretched || !(self.phrase > 0.0) {
            return None;
        }
        let x = step % self.phrase;
        (x % 2.0 == 0.0).then(|| step - x + x / 2.0)
    }

    /// The phrase lines of a loop of `bars` bars of `spb` steps: a
    /// phrase's steps and its first half's (whole bars; the longer half
    /// first where they are odd).
    fn phrases(&mut self, cx: &Cx) {
        let bars = cx.spec.bars;
        let phrase = if bars % 4.0 == 0.0 {
            4.0
        } else if bars % 2.0 == 0.0 {
            2.0
        } else {
            bars
        };
        self.phrase = phrase * cx.spb;
        self.half = if phrase >= 2.0 { js::ceil(phrase / 2.0) * cx.spb } else { js::ceil(cx.spb / 2.0) };
        self.root = cx.spec.root;
    }

    /// The unit holding section step `step` (none: nothing heard).
    #[must_use]
    pub fn unit(&self, step: f64) -> Option<&Unit> {
        if !self.on {
            return None;
        }
        let units = self.units.get(..self.count)?;
        units.iter().find(|u| step < u.to).or(units.last())
    }

    /// The units, in order.
    #[must_use]
    pub fn units(&self) -> &[Unit] {
        self.units.get(..self.count).unwrap_or(&[])
    }

    /// Whether tune note `i` sounds at `step`, by its place among the
    /// tune's notes against how thin the tune is there: the notes ranked
    /// highest go first, at least one as soon as it thins at all. A note's
    /// rank is drawn once, from the seed and its place in the loop, so a
    /// thin tune leaves out the same notes pass after pass; a note on a bar
    /// line ranks lower, so the bar lines' notes stay longest.
    #[must_use]
    pub fn sounds(&self, i: usize, step: f64) -> bool {
        let Some(u) = self.unit(step) else { return true };
        if step >= self.whole[0] && step < self.whole[1] {
            return true;
        }
        let thin = Unit::glide(u.thin, u.along(step));
        thin <= 0.0 || f64::from(self.places.get(i).copied().unwrap_or(0.0)) < 1.0 - thin
    }

    /// Each tune note's place by rank (`places`), counted in 256 bins: a
    /// walk of the tune, no sort.
    fn place_notes(&mut self, base: &Render) {
        const BINS: usize = 256;
        let n = base.melody.len().min(MELODY_MAX);
        if n == 0 {
            return;
        }
        let bin = |i: usize, step: f64| {
            let on_bar = self.spb > 0.0 && step % self.spb == 0.0;
            let r = rank(self.seed, i) * if on_bar { 0.5 } else { 1.0 };
            ((r * BINS as f64) as usize).min(BINS - 1)
        };
        let mut count = [0u16; BINS];
        for (i, e) in base.melody.iter().enumerate().take(n) {
            count[bin(i, e.step)] += 1;
        }
        let mut under = [0u16; BINS];
        let mut sum = 0u16;
        for (u, &c) in under.iter_mut().zip(count.iter()) {
            sum += c;
            *u = sum;
        }
        for (i, e) in base.melody.iter().enumerate().take(n) {
            self.places[i] = (f64::from(under[bin(i, e.step)]) / n as f64) as f32;
        }
    }

    /// Hear section `n` of the piece `form` plans, of the loop of `cx`
    /// rendered as `base`: every unit's state, and the one before the first
    /// (where it starts from). An arc's is its energy (`hear_arc`).
    pub fn hear(&mut self, form: Option<&Form>, cx: &Cx, base: &Render, second: Option<&Melody>, n: u64) {
        *self = Heard::NONE;
        let Some(form) = form else { return };
        if form.journey == Journey::Arc {
            self.hear_arc(form, cx, base, second, n);
            return;
        }
        self.on = true;
        self.seed = cx.spec.seed;
        self.spb = cx.spb;
        self.place_notes(base);
        self.phrases(cx);
        self.second = Tune::of(form, n);
        let ways = Ways::of(cx, base, None);
        let spb = cx.spb;
        let unit_steps = form.unit_bars * spb;
        let count = (form.units_in(n) as usize).min(UNITS_MAX);
        // A glide lasts the data's seconds, over as many units as that is.
        let glide = js::max(1.0, js::ceil(LONGFORM.glide / form.unit_seconds)) as u64;
        let mut from = form.before(n, 0).map(|(b, p)| ways.glided(form, b, p, glide));
        let mut at = 0.0;
        for p in 0..count {
            let end = ways.glided(form, n, p as u64, glide);
            // Every unit starts from the end of the one before (a round's
            // first from the close before it); the piece's first as its own.
            let start = from.unwrap_or(end);
            let steps = form.at(n, p as u64).bars * spb;
            let u = &mut self.units[p];
            *u = end;
            u.from = at;
            u.to = at + if p + 1 == count { steps } else { unit_steps.min(steps) };
            for (c, s) in u.cut.iter_mut().zip(start.cut.iter()) {
                c[0] = s[1];
            }
            for (t, s) in u.tone.iter_mut().zip(start.tone.iter()) {
                t[0] = s[1];
            }
            u.thin[0] = start.thin[1];
            at = u.to;
            from = Some(end);
        }
        self.count = count;
        self.joins(form, cx, base, second, &ways, n);
    }

    /// Hear section `n` of an arc (session 13): each unit's energy, as
    /// marks and levels (`Lift::end`), gliding over the unit from where the
    /// unit before ended, so a build is a slow swell and nothing jumps; and
    /// at a finite piece's end, the tune's last phrase whole and the landing
    /// on the tonic (`land`).
    fn hear_arc(&mut self, form: &Form, cx: &Cx, base: &Render, second: Option<&Melody>, n: u64) {
        self.on = true;
        self.arc = true;
        self.seed = cx.spec.seed;
        self.spb = cx.spb;
        self.place_notes(base);
        self.phrases(cx);
        self.second = Tune::of(form, n);
        let ways = Ways::of(cx, base, second);
        let lift = Lift::of(form, cx, &ways);
        let spb = cx.spb;
        let unit_steps = form.unit_bars * spb;
        let count = (form.units_in(n) as usize).min(UNITS_MAX);
        // A second tune that rests (its theme's own loop has none: queue
        // item 85f, a theme that rests stays resting, never given a tune)
        // leaves the arc its one theme: those units play theme A, in the
        // unit's key. (A rest of the tune through a second role sat 4.4 LU
        // under its neighbors in a tune-led loop: session 14 measured.)
        let has = self.second != Tune::NONE && second.is_some_and(|m| m.iter().any(|e| e.vel != 0.0));
        let mut from = form.before(n, 0).map(|(b, p)| {
            let span = smoothed(form, b, p);
            let mut end = lift.end(&ways, &span);
            lift.themed(form, &ways, &span, has, &mut end);
            end
        });
        let mut at = 0.0;
        for p in 0..count {
            let span = smoothed(form, n, p as u64);
            let mut end = lift.end(&ways, &span);
            lift.themed(form, &ways, &span, has, &mut end);
            let start = from.unwrap_or(end);
            let steps = span.bars * spb;
            let u = &mut self.units[p];
            *u = end;
            u.from = at;
            u.to = at + if p + 1 == count { steps } else { unit_steps.min(steps) };
            for (c, s) in u.cut.iter_mut().zip(start.cut.iter()) {
                c[0] = s[1];
            }
            for (t, s) in u.tone.iter_mut().zip(start.tone.iter()) {
                t[0] = s[1];
            }
            u.thin[0] = start.thin[1];
            // A theme stated comes in whole on its line (the second theme,
            // theme A come home): it never fills in from the outline before.
            if matches!(span.role, Role::Second | Role::Return) && span.changed & super::longform::changed::ROLE != 0 && u.thin[1] < u.thin[0] {
                u.thin[0] = u.thin[1];
            }
            u.soft[0] = start.soft[1];
            u.shimmer[0] = start.shimmer[1];
            u.backbeat[0] = start.backbeat[1];
            u.doubled[0] = start.doubled[1];
            // A double arriving starts from nothing, in the octave it will
            // play.
            if u.double != 0 && start.double == 0 {
                u.doubled[0] = 0.0;
            }
            u.aim[0] = start.aim[1];
            // A return prepared: over the unit's last phrase the bass holds
            // the home key's fifth.
            if span.prepares {
                u.pedal = ways.fifth(form, cx);
                u.pedal_from = js::max(u.from, u.to - self.phrase);
            }
            at = u.to;
            // (What a line changes, a tune, a key, a voice, is this unit's
            // alone: the next starts from its levels only.)
            from = Some(end);
        }
        self.count = count;
        self.joins(form, cx, base, second, &ways, n);
        if let Some(to_end) = form.to_end(n) {
            self.land(form, cx, base, to_end * spb, at);
        }
    }

    /// The joins at the ends of section `n`'s units (session 15): each
    /// unit's `Join`, as the plan chose it, played over its last bar (at
    /// most `[longform joins] share` of the unit; a breath over its share of
    /// that, a fill over the last beat), where the loop has what it needs,
    /// else the next of its kind it has: a fill with no kit to play it is a
    /// glide, the tune alone where the tune is silent there a breath, a
    /// pivot with no chords the floor out. Nothing is made but a fill's few
    /// hits and the pivot's chord.
    fn joins(&mut self, form: &Form, cx: &Cx, base: &Render, second: Option<&Melody>, ways: &Ways, n: u64) {
        for p in 0..self.count {
            let a = form.at(n, p as u64);
            self.units[p].join = a.join;
            if matches!(a.join, Join::None | Join::Glide | Join::Prepared) {
                continue;
            }
            let (bn, bp) = form.after(n, p as u64);
            let b = form.at(bn, bp);
            self.join_unit(form, cx, base, second, ways, p, &a, &b);
        }
    }

    /// Unit `p`'s join (`joins`), unit `a` of the plan meeting `b` after it.
    #[allow(clippy::too_many_lines, clippy::too_many_arguments)] // a join's kinds, each a few lines
    fn join_unit(&mut self, form: &Form, cx: &Cx, base: &Render, second: Option<&Melody>, ways: &Ways, p: usize, a: &Span, b: &Span) {
        let spb = cx.spb;
        let beat = if cx.metre == Metre::SixEight { 6.0 } else { 4.0 };
        let bit = |l: Layer| 1u8 << l as usize;
        let all = 0b1_1111u8;
        let cycle = |layer: Layer| cx.spec.cycles.of(layer as usize).filter(|&c| c > 0.0).unwrap_or(cx.spec.bars * spb);
        // Whether a layer's notes fall in steps `from` to `to` of the section.
        let falls = |step: f64, len: f64, from: f64, to: f64| len > 0.0 && step + js::ceil((from - step) / len) * len < to;
        // The kit's backbeat instrument, else its shimmer, and its loudest.
        let mut kit: [Option<(Word, f64)>; 2] = [None; 2];
        for e in base.drums.iter().filter(|e| e.vel > 0.0) {
            let slot = match Piece::of(e.inst.get()) {
                Piece::Backbeat => 0,
                Piece::Shimmer => 1,
                Piece::Other => continue,
            };
            if kit[slot].is_none_or(|(_, v)| e.vel > v) {
                kit[slot] = Some((e.inst, e.vel));
            }
        }
        let chord = base.chords.iter().enumerate().find(|(_, e)| !e.notes.is_empty()).map(|(i, e)| (i, e.vel));
        {
            let Some(u) = self.units.get_mut(p) else { return };
            u.join = a.join;
            let len = js::max(1.0, js::floor(js::min(JOINS.bars * spb, JOINS.share * (u.to - u.from))));
            let mut kind = a.join;
            if kind == Join::Fill {
                let pick = kit[0].or(kit[1]);
                match pick {
                    Some((inst, vel)) if !u.rests(Layer::Drums) && u.cut[Layer::Drums as usize][1] < 1.0 => {
                        let span = js::min(beat, len);
                        u.join_from = u.to - span;
                        u.fill = inst;
                        u.fill_every = if beat == 6.0 { 2.0 } else { 1.0 };
                        u.fill_vel = [JOINS.fill[0] * vel, JOINS.fill[1] * vel];
                    }
                    _ => kind = Join::Glide,
                }
            }
            if kind == Join::Pivot {
                let pivot = longform::pivot(a.key, b.key);
                match (pivot, ways.chords, chord) {
                    (Some(pv), Some([lo, _]), Some((from, vel))) => {
                        let c = Layer::Chords as usize;
                        let by = u.shift[c] + u.register + if u.mapped & bit(Layer::Chords) != 0 { i16::from(u.map[0]) } else { 0 };
                        let low = lo + by;
                        // The chord's root at or a little under the chords'
                        // lowest, its notes stacked over it.
                        let root = i16::from(pv.notes[0]);
                        let mut m = low - (low - root).rem_euclid(12);
                        if m < low - 4 {
                            m += 12;
                        }
                        let mut notes = [0i16; 3];
                        let mut last = m - 1;
                        for (x, &pc) in pv.notes.iter().take(pv.count).enumerate() {
                            let q = i16::from(pc);
                            let mut note = last + 1 + (q - (last + 1)).rem_euclid(12);
                            if x == 0 {
                                note = m;
                            }
                            notes[x] = note;
                            last = note;
                        }
                        while notes[pv.count - 1] > CHORDS_RANGE[1] {
                            notes.iter_mut().take(pv.count).for_each(|x| *x -= 12);
                        }
                        while notes[0] < CHORDS_RANGE[0] {
                            notes.iter_mut().take(pv.count).for_each(|x| *x += 12);
                        }
                        u.join_from = u.to - len;
                        u.out = bit(Layer::Chords);
                        u.pivot = notes;
                        u.pivot_count = pv.count;
                        u.pivot_from = from;
                        u.pivot_vel = vel;
                        u.pivot_hold = len + JOINS.hold * beat;
                        // The bass on the chord's root, in its own register.
                        if u.pedal == 0
                            && let Some([blo, _]) = ways.ranges[Layer::Bass as usize]
                        {
                            let blo = blo + f64::from(u.shift[Layer::Bass as usize]);
                            u.pedal = (blo + ((f64::from(root) - blo) % 12.0 + 12.0) % 12.0) as i16;
                            u.pedal_from = u.join_from;
                        }
                    }
                    _ => kind = if form.playing & (bit(Layer::Drums) | bit(Layer::Bass)) != 0 && a.journey == Journey::Arc { Join::Floor } else { Join::Breath },
                }
            }
            if kind == Join::Alone {
                // The tune's notes in the last bar: the loop's own or the
                // second's, by the phrase's later half.
                let from = u.to - len;
                let notes: &[super::melody::MelodyEvent] = match u.tune[1] {
                    tune::OWN => &base.melody,
                    tune::SECOND => second.map_or(&[], |m| &m[..]),
                    _ => &[],
                };
                let c = cycle(Layer::Melody);
                if notes.iter().any(|e| e.vel > 0.0 && falls(e.step, c, from, u.to)) && !u.rests(Layer::Melody) {
                    u.join_from = from;
                    u.out = all & !bit(Layer::Melody);
                } else {
                    kind = Join::Breath;
                }
            }
            if kind == Join::Floor {
                u.join_from = u.to - len;
                u.out = bit(Layer::Drums) | bit(Layer::Bass);
            }
            if kind == Join::Breath {
                u.join_from = u.to - js::max(1.0, js::floor(len * JOINS.breath));
                // A wander's line move: only the layer that moves takes the
                // breath; else every layer.
                let c = b.changed;
                u.out = if a.journey == Journey::Wander && b.journey == Journey::Wander {
                    if c & longform::changed::VOICE != 0 {
                        b.state.voice.or(a.state.voice).map_or(all, |(l, _)| bit(l))
                    } else if c & (longform::changed::REGISTER | longform::changed::LENGTH) != 0 {
                        bit(Layer::Chords)
                    } else if c & longform::changed::THEME != 0 {
                        bit(Layer::Melody)
                    } else {
                        all
                    }
                } else {
                    all
                };
            }
            u.join = kind;
        }
    }

    /// An ending that lands (session 15; the brief's "Endings that land"):
    /// the length slider will stop the piece `stop` steps from this
    /// section's first, before its own close (or in Endless), so the
    /// phrases before the stop play the close in place of the fade: from
    /// where the piece is, thinning to the close's energy at home on theme
    /// A, the tune's last phrase whole, and the landing (the tonic chord
    /// and the tune on the tonic, alone, ringing out) in its last bars. The
    /// close lasts the data's least (`[longform arc] close`), in whole
    /// phrases. Steps before `decided` are played already: where the
    /// landing would begin before them, nothing changes (the track ends as
    /// it always did). The player calls it after `hear`, with the track's
    /// end; with no end asked it is never called, so nothing changes.
    #[allow(clippy::too_many_lines)] // the close's units, then the landing
    pub fn stop_at(&mut self, form: &Form, cx: &Cx, base: &Render, second: Option<&Melody>, n: u64, stop: f64, decided: f64) {
        if !self.on || !(stop > 0.0) || self.phrase <= 0.0 || self.count == 0 {
            return;
        }
        let spb = cx.spb;
        let steps = self.units[self.count - 1].to;
        let phrase_seconds = self.phrase / spb * form.bar_seconds();
        let close = self.phrase * js::max(1.0, js::ceil(LONGFORM.close / phrase_seconds));
        let start = stop - close;
        let land_bars = js::max(1.0, js::ceil(ENERGY.land / form.bar_seconds()));
        if start >= steps || stop - land_bars * spb < decided {
            return;
        }
        // The plan's own close is there already: its round ends inside the
        // slider's close.
        if form.to_end(n).is_some_and(|end| end * spb >= start && end * spb <= stop) {
            return;
        }
        // The close's line in a unit: split it there (a section planned
        // whole has one unit; a phrase-planned one has its lines there).
        if let Some(p) = self.units[..self.count].iter().position(|u| u.from < start && u.to > start)
            && self.count < UNITS_MAX
        {
            self.units.copy_within(p..self.count, p + 1);
            self.count += 1;
            self.units[p].to = start;
            self.units[p + 1].from = start;
            // (The join at its end was the plan's, at the section's end.)
            self.units[p].join = Join::None;
            self.units[p].out = 0;
            self.units[p].pivot_count = 0;
            self.units[p].pedal = 0;
        }
        let ways = Ways::of(cx, base, second);
        let lift = Lift::of(form, cx, &ways);
        let [_, low] = LONGFORM.energy.iter().find(|e| e.0 == Role::Close.name()).map_or([0.4, 0.15], |e| e.1);
        // Where the energy stands as the close begins.
        let first = self.units[..self.count].iter().position(|u| u.to > start).unwrap_or(self.count);
        let plan_at = |step: f64| {
            let unit = js::floor(js::max(0.0, step) / (form.unit_bars * spb)) as u64;
            form.at(n, unit.min(form.units_in(n) - 1))
        };
        let before = if start > 0.0 { plan_at(start - 1.0) } else { form.before(n, 0).map_or_else(|| plan_at(0.0), |(b, p)| form.at(b, p)) };
        let from_energy = before.energy;
        let reach = |step: f64| js::clamp((step - start) / close, 0.0, 1.0);
        let mut prev: Option<Unit> = first.checked_sub(1).map(|p| self.units[p]);
        for p in first..self.count {
            let (a, b) = (self.units[p].from, self.units[p].to);
            let mut span = plan_at(a);
            span.role = Role::Close;
            span.theme = Theme::A;
            span.theme_seed = None;
            span.variation = Variation::Outline;
            span.fragment = false;
            span.calls = false;
            span.prepares = false;
            span.key = form.home;
            span.energy = from_energy + (low - from_energy) * reach(b);
            span.closes = true;
            let mut end = if self.arc {
                let mut u = lift.end(&ways, &span);
                lift.themed(form, &ways, &span, false, &mut u);
                u
            } else {
                // A wander's: every layer down toward its close's level, the
                // tune toward its thinnest, at home.
                let t = reach(b);
                let mut u = self.units[p];
                for c in &mut u.cut {
                    c[1] = js::max(c[1], (1.0 - CLOSE_LEVEL) * t);
                }
                let [thinnest, own] = LONGFORM.density;
                u.thin[1] = js::max(u.thin[1], (1.0 - thinnest / own.max(1e-9)) * t);
                u.shift = [0; 5];
                u.map = [0; 12];
                u.mapped = 0;
                u.tune = [tune::OWN; 2];
                u
            };
            end.from = a;
            end.to = b;
            if let Some(s) = prev {
                for (c, w) in end.cut.iter_mut().zip(s.cut.iter()) {
                    c[0] = w[1];
                }
                for (t, w) in end.tone.iter_mut().zip(s.tone.iter()) {
                    t[0] = w[1];
                }
                end.thin[0] = s.thin[1];
                end.soft[0] = s.soft[1];
                end.shimmer[0] = s.shimmer[1];
                end.backbeat[0] = s.backbeat[1];
                end.doubled[0] = s.doubled[1];
                end.aim[0] = s.aim[1];
            }
            if !self.arc {
                end.cut.iter_mut().zip(self.units[p].cut.iter()).for_each(|(c, w)| c[0] = w[0]);
            }
            end.join = Join::None;
            end.out = 0;
            end.pivot_count = 0;
            end.pedal = 0;
            self.units[p] = end;
            prev = Some(end);
            // The line into the close: written as any other (a pivot home,
            // the floor out), on the unit before it.
            if p == first && p > 0 && start > decided {
                let (into, join) = form.join_between(&before, &span);
                self.units[p - 1].join = join;
                if !matches!(join, Join::None | Join::Glide | Join::Prepared) {
                    self.join_unit(form, cx, base, second, &ways, p - 1, &before, &into);
                }
            }
        }
        // The landing on the tonic, from the stop's last bars.
        self.landing = Landing::NONE;
        self.whole = [0.0; 2];
        self.land(form, cx, base, stop, steps);
        self.stops = true;
    }

    /// A finite arc's end, where it reaches section steps 0 to `steps`
    /// (`end` steps from the section's first): its last bars land (at least
    /// `[longform energy] land` seconds and a bar, over as many sections as
    /// that takes), and the tune's last phrase before them sounds whole.
    fn land(&mut self, form: &Form, cx: &Cx, base: &Render, end: f64, steps: f64) {
        let spb = cx.spb;
        let bars = cx.spec.bars;
        let land_bars = js::max(1.0, js::ceil(ENERGY.land / form.bar_seconds()));
        let phrase = if bars % 4.0 == 0.0 {
            4.0
        } else if bars % 2.0 == 0.0 {
            2.0
        } else {
            bars
        };
        // Where the landing starts, from the section's first step (before it
        // where the landing is longer than the section, so the section is
        // all landing).
        let at = end - land_bars * spb;
        let whole = [js::max(0.0, at - phrase * spb), js::clamp(at, 0.0, steps)];
        if whole[1] > whole[0] {
            self.whole = whole;
        }
        if at >= steps {
            return;
        }
        let l = &mut self.landing;
        l.on = true;
        l.at = js::max(0.0, at);
        l.hold = js::max(1.0, js::floor((end - at) / 2.0));
        // The section it starts in strikes it.
        l.strikes = at >= 0.0;
        if !l.strikes {
            return;
        }
        let root = i16::from(form.home.root);
        let pc = |n: i16| n.rem_euclid(12);
        // The tonic chord: the mode's triad on the root (its third and its
        // fifth, or their nearest where the mode has none), as the loop
        // voices it where it plays one in root position, else built at the
        // chords' lowest. (A loop whose chords are played a
        // note at a time has no chord of three: it lands on the triad,
        // rolled.)
        let steps = cx.spec.steps();
        let has = |x: f64| steps.contains(&x);
        let third: i16 = if has(4.0) { 4 } else if has(3.0) { 3 } else { steps.get(1).map_or(4, |&x| x as i16) };
        let fifth: i16 = if has(7.0) { 7 } else if has(8.0) { 8 } else { 6 };
        let triad = [root, pc(root + third), pc(root + fifth)];
        let lowest = |e: &plan::ChordEvent| e.notes.iter().copied().min();
        let chords = || base.chords.iter().enumerate().filter(|(_, e)| !e.notes.is_empty());
        let tonic = |e: &plan::ChordEvent| e.notes.len() >= 3 && triad.iter().all(|&t| e.notes.iter().any(|&n| pc(n) == t));
        // (A chord the loop leaves silent, an entry's or a rest's, gives its
        // notes and its voice but never its velocity: session 15 found
        // landings struck at 0. The chords' own loudest stands in.)
        let loudest = chords().map(|(_, e)| e.vel).fold(0.0, js::max);
        let sounding = |vel: f64| if vel > 0.0 { vel } else { loudest };
        let own = chords().filter(|(_, e)| e.vel > 0.0).find(|(_, e)| tonic(e) && lowest(e).is_some_and(|lo| pc(lo) == root)).or_else(|| chords().find(|(_, e)| tonic(e) && lowest(e).is_some_and(|lo| pc(lo) == root)));
        l.rolled = !chords().any(|(_, e)| e.notes.len() >= 2);
        if let Some((i, e)) = own {
            l.chord = e.notes;
            l.chord_from = i;
            l.chord_vel = sounding(e.vel);
        } else if let Some((i, e)) = chords().find(|(_, e)| e.vel > 0.0).or_else(|| chords().next()) {
            let lo = base.chords.iter().flat_map(|e| e.notes.iter().copied()).min().unwrap_or(48);
            let mut r = lo - pc(lo - root);
            while r < CHORDS_RANGE[0] {
                r += 12;
            }
            l.chord = Notes::new();
            for by in [0, third, fifth] {
                l.chord.push(r + by);
            }
            l.chord_from = i;
            l.chord_vel = sounding(e.vel);
        }
        // The tune on the tonic, nearest its last note that sounds, in its
        // range.
        if let Some((i, e)) = base.melody.iter().enumerate().filter(|(_, e)| e.vel > 0.0).next_back() {
            let m = js::round(e.midi) as i16;
            let below = m - pc(m - root);
            let near = if m - below <= 6 { below } else { below + 12 };
            let near = if near + 12 <= 96 && near < 48 { near + 12 } else if near > 96 { near - 12 } else { near };
            l.tune = f64::from(near);
            l.tune_from = i;
            l.tune_vel = e.vel;
        }
    }

    /// Whether the plan's landing has every layer's own notes left out at
    /// section step `step` (the landing plays alone).
    #[must_use]
    pub fn landed(&self, step: f64) -> bool {
        self.landing.on && step >= self.landing.at
    }

    /// The heard units' canonical text (the goldens' passes, a piece of the
    /// wander's), in the pass's own: an arc's (session 13) as its units,
    /// its last phrase whole and its landing.
    pub fn write<S: Sink>(&self, w: &mut Writer<S>) {
        if !self.arc {
            // (A wander that hands its tune over says whose it plays.)
            if self.second == Tune::NONE {
                self.write_units(w);
                return;
            }
            w.obj();
            w.key("second");
            w.num(f64::from(self.second.seed));
            w.key("units");
            self.write_units(w);
            w.end_obj();
            return;
        }
        w.obj();
        w.key("landing");
        let l = &self.landing;
        if l.on {
            w.obj();
            w.key("at");
            w.num(l.at);
            w.key("chord");
            w.list();
            for &n in l.chord.iter() {
                w.num(f64::from(n));
            }
            w.end_list();
            w.key("hold");
            w.num(l.hold);
            w.key("strikes");
            w.bool(l.strikes);
            w.key("tune");
            w.num(l.tune);
            w.end_obj();
        } else {
            w.null();
        }
        w.key("second");
        w.list();
        w.num(f64::from(self.second.kind));
        w.num(f64::from(self.second.seed));
        w.end_list();
        w.key("units");
        self.write_units(w);
        w.key("whole");
        w.list();
        w.num(self.whole[0]);
        w.num(self.whole[1]);
        w.end_list();
        w.end_obj();
    }

    #[allow(clippy::too_many_lines)] // a unit's marks, a key each
    fn write_units<S: Sink>(&self, w: &mut Writer<S>) {
        w.list();
        for u in self.units() {
            w.obj();
            if self.arc {
                w.key("aim");
                w.list();
                w.num(u.aim[0]);
                w.num(u.aim[1]);
                w.end_list();
            }
            w.key("cut");
            w.list();
            for c in &u.cut {
                w.num(c[0]);
                w.num(c[1]);
            }
            w.end_list();
            if self.arc {
                w.key("double");
                w.num(f64::from(u.double));
                w.key("doubled");
                w.list();
                w.num(u.doubled[0]);
                w.num(u.doubled[1]);
                w.end_list();
            }
            w.key("from");
            w.num(u.from);
            // How it meets the next (session 15), where it does more than
            // glide.
            if !matches!(u.join, Join::None | Join::Glide) {
                w.key("join");
                w.obj();
                w.key("fill");
                w.list();
                if u.join == Join::Fill {
                    w.str(u.fill.get());
                    w.num(u.fill_every);
                    w.num(u.fill_vel[0]);
                    w.num(u.fill_vel[1]);
                }
                w.end_list();
                w.key("from");
                w.num(u.join_from);
                w.key("kind");
                w.str(u.join.name());
                w.key("out");
                w.num(f64::from(u.out));
                w.key("pivot");
                w.list();
                for &n in &u.pivot[..u.pivot_count.min(3)] {
                    w.num(f64::from(n));
                }
                w.end_list();
                w.end_obj();
            }
            if self.arc {
                w.key("lift");
                w.num(f64::from(u.lift));
                w.key("map");
                w.list();
                for &m in &u.map {
                    w.num(f64::from(m));
                }
                w.end_list();
                w.key("mapped");
                w.num(f64::from(u.mapped));
                w.key("pedal");
                w.list();
                w.num(f64::from(u.pedal));
                w.num(u.pedal_from);
                w.end_list();
            }
            w.key("register");
            w.num(f64::from(u.register));
            w.key("shift");
            w.list();
            for &s in &u.shift {
                w.num(f64::from(s));
            }
            w.end_list();
            if self.arc {
                w.key("shimmer");
                w.list();
                w.num(u.shimmer[0]);
                w.num(u.shimmer[1]);
                w.num(u.backbeat[0]);
                w.num(u.backbeat[1]);
                w.end_list();
            }
            if self.arc {
                w.key("soft");
                w.list();
                w.num(u.soft[0]);
                w.num(u.soft[1]);
                w.end_list();
            }
            w.key("stretch");
            w.num(u.stretch);
            if self.arc {
                w.key("stretched");
                w.bool(u.stretched);
            }
            w.key("thin");
            w.list();
            w.num(u.thin[0]);
            w.num(u.thin[1]);
            w.end_list();
            w.key("tone");
            w.list();
            for t in &u.tone {
                w.num(t[0]);
                w.num(t[1]);
            }
            w.end_list();
            w.key("tune");
            w.list();
            w.num(f64::from(u.tune[0]));
            w.num(f64::from(u.tune[1]));
            w.end_list();
            w.key("voice");
            w.list();
            for v in &u.voice {
                w.str(v.get());
            }
            w.end_list();
            w.end_obj();
        }
        w.end_list();
    }
}

impl Default for Heard {
    fn default() -> Self {
        Self::NONE
    }
}

/// Unit `phrase` of section `n` of an arc, its energy smoothed over its
/// neighbors (a quarter each side, half its own: so a role of one unit,
/// a breath, is approached and left over three, and no unit steps far from
/// the one before), never across a round's end (a finite piece's close
/// reaches its own low, and the round after starts at its opening). Three
/// units read, by number.
fn smoothed(form: &Form, n: u64, phrase: u64) -> Span {
    let mut span = form.at(n, phrase);
    let last = phrase + 1 >= form.units_in(n);
    let before = if phrase == 0 && form.comes_round(n) { None } else { form.before(n, phrase) };
    let after = if !last {
        Some((n, phrase + 1))
    } else if n.checked_add(1).is_some_and(|m| !form.comes_round(m)) {
        Some((n + 1, 0))
    } else {
        None
    };
    let energy = |x: Option<(u64, u64)>| x.map_or(span.energy, |(b, p)| form.at(b, p).energy);
    span.energy = 0.25 * energy(before) + 0.5 * span.energy + 0.25 * energy(after);
    span
}

/// A rank in [0, 1) for tune note `i` of the loop of `seed`.
fn rank(seed: u32, i: usize) -> f64 {
    let mut x = u64::from(seed ^ THIN_SALT) << 32 | i as u64;
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^= x >> 31;
    f64::from((x >> 32) as u32) / 4_294_967_296.0
}

/// How a loop plays an arc's energy (session 13), worked out once a
/// section: the layers that may fade with it, whether its tune may be
/// doubled, whether the filter and the space may move.
#[allow(clippy::struct_excessive_bools)] // four separate facts about a loop, not a state
struct Lift {
    /// The layers that fade with the energy, a bit each (never one asked
    /// on, off or re-rolled; the chords and the tune never, and a loop of
    /// fewer than two of them keeps its bass, then its texture, then its
    /// kit), and whether the tune rests in a breath.
    fading: u8,
    tune_rests: bool,
    /// The chords and the tune, those that swell with the energy (none
    /// asked on, off or re-rolled).
    swelling: u8,
    /// Whether the tune has room an octave up (its highest at 84 or under).
    doubles: bool,
    /// Whether the arc's filter takes the energy's aim (no arc asked), and
    /// whether the space opens with it (the code left the space as it was).
    aims: bool,
    spaces: bool,
}

impl Lift {
    fn of(form: &Form, cx: &Cx, ways: &Ways) -> Lift {
        let bit = |l: Layer| 1u8 << l as usize;
        let playing = form.playing;
        let mut fading = form.fading & (bit(Layer::Drums) | bit(Layer::Bass) | bit(Layer::Texture));
        let mut kept = (playing & (bit(Layer::Chords) | bit(Layer::Melody))).count_ones();
        for l in [Layer::Bass, Layer::Texture, Layer::Drums] {
            if kept >= 2 {
                break;
            }
            if playing & bit(l) != 0 {
                fading &= !bit(l);
                kept += 1;
            }
        }
        let asked = ["arc", "arc.shape", "arc.period", "arc.from", "arc.floor"].iter().any(|p| cx.spec.pins.get(p).is_some());
        Lift {
            fading,
            tune_rests: form.fading & bit(Layer::Melody) != 0,
            swelling: form.fading & (bit(Layer::Chords) | bit(Layer::Melody)),
            doubles: ways.ranges[Layer::Melody as usize].is_some_and(|[_, hi]| hi + 12.0 <= 96.0),
            aims: !asked && !form.toned[0],
            spaces: !form.toned[1],
        }
    }

    /// What unit `span` of an arc ends as: its energy heard, each gliding
    /// thing at its end in both of its pair (`hear_arc` sets the start from
    /// the unit before).
    fn end(&self, ways: &Ways, span: &Span) -> Unit {
        let e = span.energy;
        let x = &ENERGY;
        let mut u = Unit::EMPTY;
        for &(name, [gone, low, own, floor]) in x.levels {
            let Some(layer) = LAYERS.into_iter().find(|l| l.name() == name) else { continue };
            if self.fading & (1 << layer as usize) == 0 {
                continue;
            }
            // `build.rs` holds the three in order, apart.
            let level = if e <= gone {
                0.0
            } else if e < low {
                floor * (e - gone) / (low - gone)
            } else if e < own {
                floor + (1.0 - floor) * (e - low) / (own - low)
            } else {
                1.0
            };
            u.cut[layer as usize] = [1.0 - level; 2];
        }
        let [share, low, own] = x.swell;
        let swell = share + (1.0 - share) * js::clamp((e - low) / (own - low), 0.0, 1.0);
        for layer in [Layer::Chords, Layer::Melody] {
            if self.swelling & (1 << layer as usize) != 0 {
                u.cut[layer as usize] = [1.0 - swell; 2];
            }
        }
        if span.theme == Theme::Rest && self.tune_rests {
            u.cut[Layer::Melody as usize] = [1.0; 2];
        }
        let [own, reach] = x.aim;
        let below = if own > 0.0 { js::clamp((own - e) / own, 0.0, 1.0) } else { 0.0 };
        let [silent, full] = x.kit;
        let arrived = if full > silent { js::clamp((e - silent) / (full - silent), 0.0, 1.0) } else { f64::from(u8::from(e >= full)) };
        u.shimmer = [1.0 - arrived; 2];
        u.backbeat = [(1.0 - x.beat) * (1.0 - arrived); 2];
        u.soft = [x.soft * below; 2];
        let [thin_at, thinnest] = x.thin;
        u.thin = [if e < thin_at && thin_at > 0.0 { thinnest * (thin_at - e) / thin_at } else { 0.0 }; 2];
        // A breath between arcs: the tune at its thinnest, never resting
        // (session 15).
        if span.role == Role::Breath {
            u.thin = [thinnest; 2];
        }
        u.stretch = if e < x.held { ways.stretch(1) } else { 0.0 };
        let [none, most, share] = x.double;
        if self.doubles && e > none {
            u.double = 12;
            u.doubled = [share * js::clamp((e - none) / (most - none), 0.0, 1.0); 2];
        }
        let aim = if self.aims && reach > 0.0 { js::clamp((e - own) / reach, -1.0, 1.0) } else { 0.0 };
        u.aim = [aim; 2];
        if self.spaces {
            u.tone[1] = [x.space * aim; 2];
        }
        u
    }
}

impl Lift {
    /// Unit `span` of an arc's theme, its variation and its key, on the
    /// unit `u` its energy made (session 14; the module's doc). `has`: the
    /// section plays a tune beside the loop's own.
    #[allow(clippy::unused_self)] // beside `end`, which it follows
    fn themed(&self, form: &Form, ways: &Ways, span: &Span, has: bool, u: &mut Unit) {
        // An interlude: the wander's state over the energy.
        if span.journey == Journey::Wander {
            ways.overlay(&span.state, u);
            return;
        }
        // Whose tune, by the halves of each phrase.
        let second = if has { tune::SECOND } else { tune::OWN };
        let other = matches!(span.theme, Theme::B | Theme::Recall(_)) || span.variation == Variation::Answered;
        u.tune = match (span.fragment, span.calls, other) {
            (true, true, _) => [tune::OWN, second],
            (true, false, true) => [second, tune::REST],
            (true, false, false) => [tune::OWN, tune::REST],
            // A theme answered: its first half kept, the second the answer's.
            (false, _, true) if span.theme == Theme::A => [tune::OWN, second],
            (false, _, true) => [second; 2],
            (false, _, false) => [tune::OWN; 2],
        };
        // The key: a fifth away, the loop moved whole; another mode, every
        // pitched layer along the map, an octave over where it would leave
        // its range.
        let relation = span.key.relation;
        if relation.is_fifth() {
            u.shift = ways.shift_up(relation.shift());
        } else if relation != Relation::Home {
            u.map = key_map(form.home, span.key);
            u.mapped = PITCHED;
            u.shift = ways.octaves(u.map[0]);
        }
        // The variation: a mark on the notes, by the movement's number.
        let mut variation = span.variation;
        let melody = Layer::Melody as usize;
        if variation == Variation::Voice {
            match ways.voice(Layer::Melody, span.pick) {
                Some(v) => u.voice[melody] = Word::new(v),
                // A tune with no other voice to take moves an octave.
                None => variation = Variation::Register,
            }
        }
        match variation {
            // The peak's register is its double, an octave over (session
            // 13), and so is any rising role's: a build's swells in at the
            // double's share from its first unit, the small form's lift's
            // with its energy. (The tune itself moved an octave there sat
            // under the theme before it, either way: measured.) A return
            // moves whole.
            Variation::Register if matches!(span.role, Role::Build) => {
                if self.doubles {
                    u.double = 12;
                    u.doubled = [js::max(u.doubled[1], ENERGY.double[2]); 2];
                }
            }
            Variation::Register if !matches!(span.role, Role::Peak | Role::Lift) => {
                u.lift = ways.lift(span.pick, u.shift[melody]);
                if u.lift > 0 {
                    u.double = 0;
                    u.doubled = [0.0; 2];
                } else if u.lift == 0 {
                    // A tune too wide to move is thinned to its outline.
                    u.thin = [js::max(u.thin[1], LONGFORM.outline); 2];
                }
            }
            Variation::Stretched => {
                u.stretched = true;
                u.stretch = ways.stretch(1);
            }
            Variation::Reharmonized => {
                if let Some(relative) = form.related().find(|k| k.relation == Relation::Relative) {
                    u.map = key_map(form.home, relative);
                    u.mapped = PITCHED & !(1 << melody);
                    let octaves = ways.octaves(u.map[0]);
                    for layer in [Layer::Bass, Layer::Chords, Layer::Texture] {
                        u.shift[layer as usize] = octaves[layer as usize];
                    }
                }
            }
            // (An opening and a close are already as thin as their energy
            // makes them, session 13: the outline's share is the release's,
            // whose energy alone leaves the tune whole.)
            Variation::Outline if span.role == Role::Release => u.thin = [js::max(u.thin[1], LONGFORM.outline); 2],
            _ => {}
        }
        ways.settle(u);
    }
}

/// How far each pitch class, counted from `home`'s note, moves to play in
/// the key `to` (session 14). The relative key (the same notes, another
/// home note): every note the same count of the mode's steps along, the
/// nearer way, so a tune keeps its shape and a chord becomes the chord
/// that many steps away; a note outside the mode goes with the mode's note
/// under it. The parallel (the same home note, another mode): each of the
/// mode's notes to the other mode's, step for step where they have as
/// many, else to its nearest. A key a fifth away moves nothing here (the
/// whole loop moves, `Ways::shift`).
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)] // steps and semitones, under 24
pub fn key_map(home: Key, to: Key) -> [i8; 12] {
    let mut map = [0i8; 12];
    let (Some(own), Some(other)) = (SCALES.get(home.scale), SCALES.get(to.scale)) else { return map };
    let steps = own.steps;
    let n = steps.len() as i32;
    if n == 0 {
        return map;
    }
    let at = |x: i32| steps[x.rem_euclid(n) as usize] as i32 + 12 * x.div_euclid(n);
    match to.relation {
        Relation::Relative => {
            let up = (i32::from(to.root) - i32::from(home.root)).rem_euclid(12);
            let Some(d) = steps.iter().position(|&s| s as i32 == up) else { return map };
            let d = if up > 6 { d as i32 - n } else { d as i32 };
            for pc in 0..12 {
                // The mode's note at or under this one.
                let x = steps.iter().rposition(|&s| s as i32 <= pc).unwrap_or(0) as i32;
                map[pc as usize] = (at(x + d) - at(x)) as i8;
            }
        }
        Relation::Parallel => {
            for (x, &s) in steps.iter().enumerate() {
                let near = |o: f64| ((o as i32 - s as i32 + 18).rem_euclid(12)) - 6;
                let by = if other.steps.len() == steps.len() {
                    near(other.steps[x])
                } else {
                    other.steps.iter().map(|&o| near(o)).min_by_key(|d| (d.abs(), *d)).unwrap_or(0)
                };
                map[(s as usize) % 12] = by as i8;
            }
        }
        _ => {}
    }
    map
}

/// How a loop's things move, worked out once a section from the loop: the
/// voices its pools offer, which way its chords may move, which octave each
/// layer takes in another key.
struct Ways {
    /// Each voicing layer's own voice and its pool, `LAYERS`' order.
    own: [&'static str; 5],
    pools: [Option<Names>; 5],
    /// The chords' lowest and highest notes.
    chords: Option<[i16; 2]>,
    /// Whether a chord has room to sound longer, and to sound shorter.
    longer: bool,
    shorter: bool,
    /// Each layer's lowest and highest note.
    ranges: [Option<[f64; 2]>; 5],
    /// How far the tune's centre sits over the chords' (the chords' octave
    /// rule, queue item 84: five semitones apart at least where the loop
    /// keeps it); none without both.
    gap: Option<f64>,
    /// The same for the section's second tune, where it plays one with
    /// notes.
    second_gap: Option<f64>,
}

impl Ways {
    fn of(cx: &Cx, base: &Render, second: Option<&Melody>) -> Ways {
        let first = |v: Option<Word>| v.map_or("", |w| w.get());
        let own = [
            "",
            first(base.bass.iter().map(|e| e.voice).next()),
            first(base.chords.iter().map(|e| e.voice).next()),
            first(base.melody.iter().map(|e| e.voice).next()),
            "",
        ];
        let c = &cx.c;
        let pools = [None, c.bass_voices, c.chord_voices, c.melody_voices, None];
        let mut chords: Option<[i16; 2]> = None;
        for &n in base.chords.iter().flat_map(|e| e.notes.iter()) {
            chords = Some(chords.map_or([n, n], |[lo, hi]| [lo.min(n), hi.max(n)]));
        }
        let mut shorter = false;
        for e in base.chords.iter() {
            shorter |= e.dur * LONGFORM.lengths[0] >= 1.0 && e.dur >= 2.0;
        }
        let longer = has_room(base, cx);
        let mut ranges: [Option<[f64; 2]>; 5] = [None; 5];
        let mut widen = |l: Layer, m: f64| {
            let r = &mut ranges[l as usize];
            *r = Some(r.map_or([m, m], |[lo, hi]| [js::min(lo, m), js::max(hi, m)]));
        };
        base.bass.iter().for_each(|e| widen(Layer::Bass, e.midi));
        base.chords.iter().flat_map(|e| e.notes.iter()).for_each(|&n| widen(Layer::Chords, f64::from(n)));
        base.melody.iter().for_each(|e| widen(Layer::Melody, e.midi));
        base.texture.iter().flat_map(|e| e.notes.iter()).for_each(|&n| widen(Layer::Texture, f64::from(n)));
        let mean = |sum: f64, n: usize| (n > 0).then(|| sum / n as f64);
        let tune = mean(base.melody.iter().filter(|e| e.vel != 0.0).map(|e| e.midi).sum(), base.melody.iter().filter(|e| e.vel != 0.0).count());
        let under = mean(base.chords.iter().flat_map(|e| e.notes.iter()).map(|&n| f64::from(n)).sum(), base.chords.iter().map(|e| e.notes.len()).sum());
        let other = second.and_then(|m| mean(m.iter().filter(|e| e.vel != 0.0).map(|e| e.midi).sum(), m.iter().filter(|e| e.vel != 0.0).count()));
        Ways { own, pools, chords, longer, shorter, ranges, gap: tune.zip(under).map(|(t, c)| t - c), second_gap: other.zip(under).map(|(t, c)| t - c) }
    }

    /// The voice `layer` plays for the plan's number `pick` (0 to 1 over its
    /// pool's other voices, by weight): none where the pool offers none, or
    /// the layer sings.
    fn voice(&self, layer: Layer, pick: f64) -> Option<&'static str> {
        let own = self.own[layer as usize];
        let pool = self.pools[layer as usize].as_ref()?;
        if own.is_empty() || SUNG.contains(&own) {
            return None;
        }
        let other = |name: &&str| !name.is_empty() && *name != own && !SUNG.contains(name);
        let total: f64 = pool.iter().filter(|e| other(&e.0)).map(|e| e.1).sum();
        if !(total > 0.0) {
            return None;
        }
        let mut at = pick * total;
        let mut last = None;
        for &(name, w) in pool.iter().filter(|e| other(&e.0)) {
            last = Some(name);
            if at < w {
                return Some(name);
            }
            at -= w;
        }
        last
    }

    /// The octave the chords move by for the plan's `register` (-1, 0, 1):
    /// the other way where that one leaves their range, none where neither
    /// fits.
    fn register(&self, register: i8) -> i16 {
        let Some([lo, hi]) = self.chords else { return 0 };
        let fits = |by: i16| lo + by >= CHORDS_RANGE[0] && hi + by <= CHORDS_RANGE[1];
        let want = 12 * i16::from(register);
        if want == 0 || fits(want) {
            want
        } else if fits(-want) {
            -want
        } else {
            0
        }
    }

    /// The chords' lengths for the plan's `length` (-1, 0, 1), as a share of
    /// their own: the other way where none of them can go this one.
    fn stretch(&self, length: i8) -> f64 {
        let [short, long] = LONGFORM.lengths;
        let (first, then) = match length {
            1 => ((self.longer, long), (self.shorter, short)),
            -1 => ((self.shorter, short), (self.longer, long)),
            _ => return 0.0,
        };
        [first, then].into_iter().find(|w| w.0).map_or(0.0, |w| w.1)
    }

    /// Each layer's move to the key `shift` semitones away: the nearer way,
    /// or an octave over where that leaves the layer's range.
    fn shift(&self, shift: i8) -> [i16; 5] {
        let mut out = [0; 5];
        if shift == 0 {
            return out;
        }
        for layer in LAYERS {
            let k = layer as usize;
            let s = f64::from(shift);
            let [lo, hi] = KEY_RANGE[k];
            out[k] = self.ranges[k].map_or(0, |[a, b]| {
                let fits = |by: f64| a + by >= lo && b + by <= hi;
                let by = [s, s + 12.0, s - 12.0].into_iter().find(|&by| fits(by)).unwrap_or(s);
                by as i16
            });
        }
        out
    }

    /// An arc's move to the key `shift` semitones away (session 14): as
    /// `shift`, but every layer over the bass takes the way up where it
    /// fits its range (a fifth above is seven semitones up, not a fourth
    /// down), so a second theme and a peak in the related key sit brighter
    /// than home, as their energy asks; the bass keeps the nearer way.
    fn shift_up(&self, shift: i8) -> [i16; 5] {
        let mut out = self.shift(shift);
        if shift == 0 {
            return out;
        }
        let up = f64::from(i16::from(shift).rem_euclid(12));
        for layer in [Layer::Chords, Layer::Melody, Layer::Texture] {
            let k = layer as usize;
            let [lo, hi] = KEY_RANGE[k];
            if self.ranges[k].is_some_and(|[a, b]| a + up >= lo && b + up <= hi) {
                out[k] = up as i16;
            }
        }
        out
    }

    /// Each pitched layer's octave where a key's map moves the home note
    /// `by` semitones: none, or the octave that keeps the layer in its
    /// range.
    fn octaves(&self, by: i8) -> [i16; 5] {
        let mut out = self.shift_up(by);
        if by != 0 {
            for o in &mut out[1..] {
                *o -= i16::from(by);
            }
        }
        out
    }

    /// The octave a register variation moves the tune by, for the
    /// movement's number `pick` (down under a half, up over it, the other
    /// way where that one leaves the tune's range, none where both do),
    /// the tune already moved `shift` by its key.
    fn lift(&self, pick: f64, shift: i16) -> i16 {
        let Some([lo, hi]) = self.ranges[Layer::Melody as usize] else { return 0 };
        let (lo, hi) = (lo + f64::from(shift), hi + f64::from(shift));
        // (Never the way that would bring the tune's centre within five
        // semitones of the chords', where the loop keeps them apart.)
        let clear = |by: f64| self.gap.is_none_or(|g| js::abs(g) < 5.0 || js::abs(g + by) >= 5.0);
        let up = hi + 12.0 <= TUNE_RANGE[1] && clear(12.0);
        let down = lo - 12.0 >= TUNE_RANGE[0] && clear(-12.0);
        match (pick >= 0.5, up, down) {
            (true, true, _) | (false, true, false) => 12,
            (false, _, true) | (true, false, true) => -12,
            _ if hi + 12.0 <= 96.0 && clear(12.0) => 12,
            _ => 0,
        }
    }

    /// The chords' octave rule in the unit's key (queue item 84): where the
    /// loop keeps its tune's centre five semitones or more from its
    /// chords', a unit whose moves bring them nearer moves the chords an
    /// octave away from the tune, or the tune an octave from the chords,
    /// whichever has room.
    fn settle(&self, u: &mut Unit) {
        let (Some(own), Some([lo, hi]), Some([tlo, thi])) = (self.gap, self.chords, self.ranges[Layer::Melody as usize]) else { return };
        // The tune the unit plays whole: the loop's own, or the second.
        let gap = if u.tune == [tune::SECOND; 2] { self.second_gap.unwrap_or(own) } else { own };
        let (m, c) = (Layer::Melody as usize, Layer::Chords as usize);
        let by = |u: &Unit, k: usize| f64::from(u.shift[k]) + if u.mapped & (1 << k) != 0 { f64::from(u.map[0]) } else { 0.0 };
        let now = |u: &Unit| gap + by(u, m) + f64::from(u.lift) - by(u, c) - f64::from(u.register);
        if js::abs(own) < 5.0 || js::abs(now(u)) >= 5.0 {
            return;
        }
        // The tune stays over the chords where it can: the chords down, or
        // the tune up; else the other way about.
        let chords = i16::from(u.map[0]) * i16::from(u.mapped & (1 << c) != 0) + u.shift[c] + u.register;
        let tune = by(u, m) + f64::from(u.lift);
        for (layer, octave) in [(c, -12i16), (m, 12), (c, 12), (m, -12)] {
            let fits = if layer == c {
                lo + chords + octave >= CHORDS_RANGE[0] && hi + chords + octave <= CHORDS_RANGE[1]
            } else {
                tlo + tune + f64::from(octave) >= TUNE_RANGE[0] && thi + tune + f64::from(octave) <= 96.0
            };
            let then = now(u) + f64::from(if layer == c { -octave } else { octave });
            if fits && js::abs(then) >= 5.0 {
                u.shift[layer] += octave;
                return;
            }
        }
    }

    /// The home key's fifth for the bass to hold before a return: the
    /// mode's (a perfect fifth, or its nearest), the lowest in the bass's
    /// own range.
    fn fifth(&self, form: &Form, cx: &Cx) -> i16 {
        let steps = cx.spec.steps();
        let has = |x: f64| steps.contains(&x);
        let fifth = if has(7.0) { 7.0 } else if has(8.0) { 8.0 } else { 6.0 };
        let pc = (f64::from(form.home.root) + fifth) % 12.0;
        let [lo, _] = self.ranges[Layer::Bass as usize].unwrap_or([36.0, 48.0]);
        let lo = js::max(lo, KEY_RANGE[Layer::Bass as usize][0]);
        (lo + ((pc - lo) % 12.0 + 12.0) % 12.0) as i16
    }

    /// An interlude's state on a unit an arc's energy made: the wander's
    /// things, as `end` hears them, over it.
    fn overlay(&self, s: &State, u: &mut Unit) {
        let reach = LONGFORM.reach;
        let [_, own] = LONGFORM.density;
        if let Some(layer) = s.rest {
            u.cut[layer as usize] = [1.0; 2];
        }
        u.tone[0] = [u.tone[0][1] + s.warmth * reach; 2];
        u.tone[1] = [u.tone[1][1] + s.space * reach; 2];
        let thin = if own > 0.0 { js::clamp(1.0 - s.density / own, 0.0, 1.0) } else { 0.0 };
        u.thin = [js::max(u.thin[1], thin); 2];
        if let Some((layer, pick)) = s.voice
            && let Some(v) = self.voice(layer, pick)
        {
            u.voice[layer as usize] = Word::new(v);
        }
        u.register = self.register(s.register);
        if s.length != 0 {
            u.stretch = self.stretch(s.length);
        }
    }

    /// What unit `phrase` of section `n` ends as, its glides under way: in
    /// the first `glide` units of a hold, the gliding things (the levels,
    /// the tone, the tune's density) are that share of the way from where
    /// the hold before left them; the rest (a voice, a register, a length,
    /// a key) are the hold's from its first step. A close keeps its own.
    fn glided(&self, form: &Form, n: u64, phrase: u64, glide: u64) -> Unit {
        let full = self.end(form, n, phrase);
        let span = form.at(n, phrase);
        if span.at + 1 >= glide || form.close_place(n, phrase).is_some() {
            return full;
        }
        // The unit before the hold's first: where the glide starts.
        let mut back = (n, phrase);
        for _ in 0..=span.at {
            match form.before(back.0, back.1) {
                Some(b) => back = b,
                None => return full,
            }
        }
        let was = self.end(form, back.0, back.1);
        let t = (span.at + 1) as f64 / glide as f64;
        let mut u = full;
        let mix = |a: f64, b: f64| a + (b - a) * t;
        for (c, w) in u.cut.iter_mut().zip(was.cut.iter()) {
            *c = [mix(w[1], c[1]); 2];
        }
        for (x, w) in u.tone.iter_mut().zip(was.tone.iter()) {
            *x = [mix(w[1], x[1]); 2];
        }
        u.thin = [mix(was.thin[1], u.thin[1]); 2];
        u
    }

    /// What unit `phrase` of section `n` ends as: the plan's state there,
    /// heard: each gliding thing at its end, in both of its pair (`hear`
    /// sets the start from the unit before).
    fn end(&self, form: &Form, n: u64, phrase: u64) -> Unit {
        let span: Span = form.at(n, phrase);
        let s = span.state;
        let mut u = Unit::EMPTY;
        let reach = LONGFORM.reach;
        let [thinnest, own] = LONGFORM.density;
        let close = form.close_place(n, phrase).map(|(at, of)| (at + 1) as f64 / of as f64);
        let down = close.map_or(0.0, |t| (1.0 - CLOSE_LEVEL) * t);
        for layer in LAYERS {
            let rest = if s.rest == Some(layer) { 1.0 } else { down };
            u.cut[layer as usize] = [rest, rest];
        }
        u.tone = [[s.warmth * reach; 2], [s.space * reach; 2]];
        let thin = if own > 0.0 { js::clamp(1.0 - s.density / own, 0.0, 1.0) } else { 0.0 };
        let thin = close.map_or(thin, |t| js::max(thin, (1.0 - thinnest / own.max(1e-9)) * t));
        u.thin = [thin; 2];
        if let Some((layer, pick)) = s.voice
            && let Some(v) = self.voice(layer, pick)
        {
            u.voice[layer as usize] = Word::new(v);
        }
        u.register = self.register(s.register);
        u.stretch = self.stretch(s.length);
        u.shift = self.shift(span.key.relation.shift());
        // The tune handed to the piece's second theme (session 14).
        if span.theme == Theme::B {
            u.tune = [tune::SECOND; 2];
        }
        u
    }
}

/// Chord `i`'s room: the steps from its start to the next chord's that
/// sounds after it, round the chords' `cycle` (the loop's, or the layer's
/// own).
#[must_use]
pub fn room(base: &Render, i: usize, cycle: f64) -> f64 {
    let Some(e) = base.chords.get(i) else { return 0.0 };
    let mut best = f64::INFINITY;
    for o in base.chords.iter() {
        if o.notes.is_empty() || o.step == e.step {
            continue;
        }
        let ahead = ((o.step - e.step) % cycle + cycle) % cycle;
        best = js::min(best, ahead);
    }
    if best.is_finite() { best } else { cycle }
}

/// Whether some chord of the loop has room to sound longer before the
/// next one starts: read in one walk where the chords are in order, as
/// render writes them (else over the first `ROOM_MOST`, so a pass's making
/// stays a walk of the loop).
fn has_room(base: &Render, cx: &Cx) -> bool {
    let chords = &base.chords;
    let cycle = cx.spec.cycles.of(Layer::Chords as usize).filter(|&c| c > 0.0).unwrap_or(cx.spec.bars * cx.spb);
    let sorted = chords.iter().zip(chords.iter().skip(1)).all(|(a, b)| a.step <= b.step);
    if !sorted {
        return (0..chords.len().min(ROOM_MOST)).any(|i| chords.get(i).is_some_and(|e| room(base, i, cycle) > e.dur + 0.5));
    }
    let Some(first) = chords.iter().find(|e| !e.notes.is_empty()).map(|e| e.step) else { return false };
    let mut next = first + cycle;
    let mut later = f64::INFINITY;
    for e in chords.iter().rev() {
        if e.notes.is_empty() {
            continue;
        }
        if e.step < later {
            next = if later.is_finite() { later } else { next };
            later = e.step;
        }
        if next - e.step > e.dur + 0.5 {
            return true;
        }
    }
    false
}

/// Whether the loop's layer `layer` plays one voice the plan may change: it
/// has notes, a voice of its own that is not sung, and its pool another.
#[must_use]
pub fn voiceable(cx: &Cx, base: &Render) -> u8 {
    let ways = Ways::of(cx, base, None);
    let mut mask = 0;
    for layer in [Layer::Bass, Layer::Chords, Layer::Melody] {
        let sings = cx.choir.is_some() && matches!(layer, Layer::Chords | Layer::Melody);
        if !sings && ways.voice(layer, 0.0).is_some() {
            mask |= 1 << layer as usize;
        }
    }
    mask
}
