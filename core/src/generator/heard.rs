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

// Steps and units are whole numbers far below 2^53.
#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]

use super::canon::{Sink, Writer};
use super::character::Names;
use super::data::{ENERGY, LONGFORM};
use super::js;
use super::list::{Empty, Word};
use super::longform::{Form, Span, Theme};
use super::melody::MELODY_MAX;
use super::plan::{self, Cx, Notes};
use super::render::Render;
use super::request::Journey;
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
    };

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
    pub fn hear(&mut self, form: Option<&Form>, cx: &Cx, base: &Render, n: u64) {
        *self = Heard::NONE;
        let Some(form) = form else { return };
        if form.journey == Journey::Arc {
            self.hear_arc(form, cx, base, n);
            return;
        }
        self.on = true;
        self.seed = cx.spec.seed;
        self.spb = cx.spb;
        self.place_notes(base);
        let ways = Ways::of(cx, base);
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
    }

    /// Hear section `n` of an arc (session 13): each unit's energy, as
    /// marks and levels (`Lift::end`), gliding over the unit from where the
    /// unit before ended, so a build is a slow swell and nothing jumps; and
    /// at a finite piece's end, the tune's last phrase whole and the landing
    /// on the tonic (`land`).
    fn hear_arc(&mut self, form: &Form, cx: &Cx, base: &Render, n: u64) {
        self.on = true;
        self.arc = true;
        self.seed = cx.spec.seed;
        self.spb = cx.spb;
        self.place_notes(base);
        let ways = Ways::of(cx, base);
        let lift = Lift::of(form, cx, &ways);
        let spb = cx.spb;
        let unit_steps = form.unit_bars * spb;
        let count = (form.units_in(n) as usize).min(UNITS_MAX);
        let mut from = form.before(n, 0).map(|(b, p)| lift.end(&ways, &smoothed(form, b, p)));
        let mut at = 0.0;
        for p in 0..count {
            let span = smoothed(form, n, p as u64);
            let end = lift.end(&ways, &span);
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
            at = u.to;
            from = Some(end);
        }
        self.count = count;
        if let Some(to_end) = form.to_end(n) {
            self.land(form, cx, base, to_end * spb, at);
        }
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
        let own = chords().find(|(_, e)| tonic(e) && lowest(e).is_some_and(|lo| pc(lo) == root));
        l.rolled = !chords().any(|(_, e)| e.notes.len() >= 2);
        if let Some((i, e)) = own {
            l.chord = e.notes;
            l.chord_from = i;
            l.chord_vel = e.vel;
        } else if let Some((i, e)) = chords().next() {
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
            l.chord_vel = e.vel;
        }
        // The tune on the tonic, nearest its last note, in its range.
        if let Some((i, e)) = base.melody.iter().enumerate().next_back() {
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
            self.write_units(w);
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
        w.key("units");
        self.write_units(w);
        w.key("whole");
        w.list();
        w.num(self.whole[0]);
        w.num(self.whole[1]);
        w.end_list();
        w.end_obj();
    }

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
}

impl Ways {
    fn of(cx: &Cx, base: &Render) -> Ways {
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
        Ways { own, pools, chords, longer, shorter, ranges }
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
    let ways = Ways::of(cx, base);
    let mut mask = 0;
    for layer in [Layer::Bass, Layer::Chords, Layer::Melody] {
        let sings = cx.choir.is_some() && matches!(layer, Layer::Chords | Layer::Melody);
        if !sings && ways.voice(layer, 0.0).is_some() {
            mask |= 1 << layer as usize;
        }
    }
    mask
}
