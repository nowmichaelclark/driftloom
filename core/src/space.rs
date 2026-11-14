//! The space recipe (queue item 65; `docs/CORE.md`, promise 9): where each
//! part sits in the stereo render, as data. Mikey's approved stereo baseline
//! (`docs/ears/batch-4.md`, round 4) is the blend of two recipes from round
//! 3, held still: the **room** (every dry part centered, the width in the
//! reverb) weighted 0.64, and the **ensemble** (the chords and the texture
//! widened, the light kit and a light texture placed) weighted 0.5, all
//! scaled by how busy and how wet the loop is, under the width ceiling
//! (`blocks::width`). The reference is the sketch's code
//! (`tools/ears/sketches/ears4-sketch.patch`, the space sketch;
//! `tools/ears/space_jobs.py`, `room3` and `ens3`; `blend_side.py`).
//!
//! Every number is a named setting with a range (`SETTINGS`), and a loop
//! draws within each range from its seed (`Space::draw`), so the variation
//! is part of the music and the code reproduces it. A host may narrow or
//! pin any of them (`Core::space_range`): a request, a slider, a game. The
//! loop tells the core what it needs to know about it at load: how busy it
//! is and how wet (`Loop`).
//!
//! What each part does in the mix, where the side is made (`mix`):
//!
//! - **Room:** the reverb's side is the reverb itself (its combs, before
//!   the room's output gain) through four delay allpasses (5-23 ms) and a
//!   highpass at 180 Hz, then through the same output gain, pre-delay and
//!   tails gain as the mid. No echo width.
//! - **Spread:** the chords, the texture and the melody each get a copy of
//!   themselves through a slowly swaying 9-15 ms delay, above 200 Hz, as side
//!   only (+ left, - right): width without a pan, which cancels in the fold.
//! - **Placement:** a balance above 150 Hz (the side is `p` times the part
//!   highpassed), so low notes stay centered by construction. The light kit
//!   (the hat voice's hats and shakers, the drums' jingles) to one side, as
//!   one, the shakers and jingles at 0.7 of the hats; a light texture (bells,
//!   chimes, drops: the whole texture layer) to the other; glass, in a loop
//!   whose texture isn't light, alone.
//! - **Never placed:** the melody, the chords and pads (widened only), the
//!   bass, the kick and the other heavy drums. The organ rides the chords'
//!   spread and the room; the wash's two noises go out as its own width,
//!   above 200 Hz and scaled by the loop's width (Mikey, 2026-10-25).
//!
//! In a mono render none of it runs.

// Ranges and draws in 64-bit; a seed carried in a float is a whole number.
#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use crate::math;
use crate::rng::{Key, unit};

/// A setting: its name and the range a loop draws it from (`lo` = `hi` for
/// a fixed number).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Setting {
    pub name: &'static str,
    pub lo: f64,
    pub hi: f64,
}

const fn s(name: &'static str, lo: f64, hi: f64) -> Setting {
    Setting { name, lo, hi }
}

/// The settings, by number, as the sketch drew them. Each is a range a loop
/// draws from, or one number.
pub mod n {
    /// How much of the room recipe the blend takes.
    pub const ROOM_WEIGHT: usize = 0;
    /// How much of the ensemble recipe.
    pub const ENSEMBLE_WEIGHT: usize = 1;
    /// The reverb's side against its mid (the sketch's `verbSmooth`).
    pub const ROOM: usize = 2;
    /// The chords' spread (round 2's; the ensemble takes `SPREAD_SCALE` of it).
    pub const SPREAD_CHORDS: usize = 3;
    /// The texture's, a heavy texture's.
    pub const SPREAD_TEXTURE: usize = 4;
    /// A light texture's.
    pub const SPREAD_LIGHT: usize = 5;
    /// The melody's.
    pub const SPREAD_MELODY: usize = 6;
    /// The ensemble's spread against round 2's.
    pub const SPREAD_SCALE: usize = 7;
    /// The most any spread is, scaled.
    pub const SPREAD_MAX: usize = 8;
    /// The spread's delay, its sway either side, in seconds, and its rate:
    /// `SPREAD_RATE` Hz plus `SPREAD_RATE_STEP` for each channel past the
    /// first (the chords 0.35 Hz, the melody 0.42, the texture 0.49).
    pub const SPREAD_DELAY: usize = 9;
    pub const SPREAD_SWAY: usize = 10;
    pub const SPREAD_RATE: usize = 11;
    pub const SPREAD_RATE_STEP: usize = 12;
    /// The light kit's place (the hats'), from the center.
    pub const KIT: usize = 13;
    /// The shakers' and jingles' place against the hats'.
    pub const KIT_SOFT: usize = 14;
    /// A light texture's place, and glass's, to the other side.
    pub const TEXTURE_PLACE: usize = 15;
    /// The highpasses: the room's side, a spread, a placement (Hz).
    pub const ROOM_CUTOFF: usize = 16;
    pub const SPREAD_CUTOFF: usize = 17;
    pub const PLACE_CUTOFF: usize = 18;
    /// How busy a loop is, as a share (0 to 1): from `BUSY_FROM` notes a
    /// second, all layers, over `BUSY_SPAN` more.
    pub const BUSY_FROM: usize = 19;
    pub const BUSY_SPAN: usize = 20;
    /// The width at the least busy, and how much a wet loop keeps (its
    /// reverb's space over `WET_FROM`).
    pub const WIDTH_LEAST: usize = 21;
    pub const WET_FROM: usize = 22;
    pub const WET_SCALE: usize = 23;
    /// The width ceiling's target (dB, side against mid): a busy loop's (over
    /// `DENSE` of busy), a sparse one's (over `SPARSE`), a very sparse one's.
    pub const CEILING_BUSY: usize = 24;
    pub const CEILING_SPARSE: usize = 25;
    pub const CEILING_VERY_SPARSE: usize = 26;
    pub const DENSE: usize = 27;
    pub const SPARSE: usize = 28;
    /// The wash's own width: how much of its two noises' difference goes out
    /// as side; how much of that follows the loop's width (0: none, the
    /// sketch's; 1: all, as every other width, the default); and what it is
    /// kept above (Hz; 0 for no highpass, the sketch's; 200 the default).
    pub const WASH: usize = 29;
    pub const WASH_FOLLOWS: usize = 30;
    pub const WASH_CUTOFF: usize = 31;
    /// widePad (queue item 101c; batch 6's #4): the loop's kind, which its
    /// choice sets (`load::Space::pins`): 0 the baseline, 1 the chords wide
    /// from the start, 2 the chords opening to it late, over `WIDE_OPEN`.
    pub const WIDE: usize = 32;
    /// The chords' spread in a widePad loop, in place of `SPREAD_CHORDS`, and
    /// the most it is, scaled, in place of `SPREAD_MAX` (the chords' alone).
    pub const WIDE_CHORDS: usize = 33;
    pub const WIDE_MAX: usize = 34;
    /// The width ceiling's targets in a widePad loop (dB), in place of
    /// `CEILING_BUSY` and the other two: the chords' spread alone stops at
    /// the baseline's.
    pub const WIDE_CEILING_BUSY: usize = 35;
    pub const WIDE_CEILING_SPARSE: usize = 36;
    pub const WIDE_CEILING_VERY_SPARSE: usize = 37;
    /// The chords' spread's highpass in a widePad loop (Hz), in place of
    /// `SPREAD_CUTOFF` (the chords' alone): it keeps the side under 120 Hz
    /// at -30 dB or lower.
    pub const WIDE_CUTOFF: usize = 38;
    /// How long the late kind takes to open, in seconds, drawn per loop.
    pub const WIDE_OPEN: usize = 39;
    /// How many there are.
    pub const COUNT: usize = 40;
}

/// The approved stereo baseline (Ears 4, round 4), as the sketch's code has
/// it.
pub const SETTINGS: [Setting; n::COUNT] = [
    s("room-weight", 0.64, 0.64),
    s("ensemble-weight", 0.5, 0.5),
    s("room", 0.7, 0.9),
    s("spread-chords", 0.3, 0.5),
    s("spread-texture", 0.35, 0.6),
    s("spread-light-texture", 0.15, 0.3),
    s("spread-melody", 0.0, 0.12),
    s("spread-scale", 1.6, 1.6),
    s("spread-max", 0.8, 0.8),
    s("spread-delay", 0.012, 0.012),
    s("spread-sway", 0.003, 0.003),
    s("spread-rate", 0.35, 0.35),
    s("spread-rate-step", 0.07, 0.07),
    s("kit", 0.08, 0.15),
    s("kit-soft", 0.7, 0.7),
    s("texture-place", 0.1, 0.25),
    s("room-cutoff", 180.0, 180.0),
    s("spread-cutoff", 200.0, 200.0),
    s("place-cutoff", 150.0, 150.0),
    s("busy-from", 0.5, 0.5),
    s("busy-span", 5.0, 5.0),
    s("width-least", 0.25, 0.25),
    s("wet-from", 0.7, 0.7),
    s("wet-scale", 0.75, 0.75),
    s("ceiling-busy", -7.0, -7.0),
    s("ceiling-sparse", -9.0, -9.0),
    s("ceiling-very-sparse", -12.0, -12.0),
    s("dense", 0.5, 0.5),
    s("sparse", 0.15, 0.15),
    s("wash", 1.0, 1.0),
    // Mikey, 2026-10-25: the wash's width above 200 Hz and scaled by the
    // loop's, as every other width (the sketch's was full and unfiltered,
    // which put a sparse loop's lows on the side).
    s("wash-follows-width", 1.0, 1.0),
    s("wash-cutoff", 200.0, 200.0),
    // widePad (queue item 101c; batch 6's #4, approved by Mikey 2026-11-09):
    // the Ears 13b sketch's space, the chords' alone. Appended, so no setting
    // before it moves and no draw does (`Space::draw`'s draw numbers).
    s("wide", 0.0, 0.0),
    s("wide-chords", 1.4, 1.4),
    s("wide-max", 2.0, 2.0),
    s("wide-ceiling-busy", -4.0, -4.0),
    s("wide-ceiling-sparse", -5.0, -5.0),
    s("wide-ceiling-very-sparse", -7.0, -7.0),
    s("wide-cutoff", 250.0, 250.0),
    s("wide-open", 60.0, 120.0),
];

/// What the core needs to know about a loop to place it: its seed, how
/// busy it is (notes a second, every layer, as heard), how wet (its
/// reverb's space, 0 to 1), and whether its texture is a light one (bells,
/// chimes, drops).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Loop {
    pub seed: u32,
    pub busy: f64,
    pub wet: f64,
    pub light: bool,
}

/// No loop yet: all zeros.
pub const NO_LOOP: Loop = Loop { seed: 0, busy: 0.0, wet: 0.0, light: false };

/// A loop's space, drawn: every amount the mix uses, signed where it is a
/// side (+ left).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Drawn {
    /// The width the loop takes, 0.25 to 1, and the ceiling's target (dB).
    pub width: f64,
    pub ceiling: f64,
    /// The reverb's side against its mid, blended.
    pub room: f64,
    /// The spreads, blended: melody, chords, texture (the core's order).
    pub spread: [f64; 3],
    /// The light kit's place (the hats'), signed, blended; the shakers' and
    /// jingles' is `kit_soft` of it.
    pub kit: f64,
    pub kit_soft: f64,
    /// The whole texture layer's place (a light texture), signed, blended.
    pub texture: f64,
    /// Glass's alone (a loop whose texture isn't light), signed, blended.
    pub glass: f64,
    /// The wash's own width, and what it is kept above (Hz, 0 for all).
    pub wash: f64,
    pub wash_cutoff: f64,
    /// The cutoffs, and the spread's delay, sway and rates.
    pub room_cutoff: f64,
    pub spread_cutoff: f64,
    pub place_cutoff: f64,
    pub spread_delay: f64,
    pub spread_sway: f64,
    pub spread_rate: [f64; 3],
    /// The chords' spread's highpass (Hz): `spread_cutoff` but in a widePad
    /// loop (queue item 101c).
    pub chords_cutoff: f64,
    /// widePad's late kind (queue item 101c): how long the chords take to
    /// open, in seconds from the space's start (0: as drawn from the start),
    /// and where they open from, the baseline's chords' spread and ceiling.
    /// `spread` and `ceiling` are where they open to; `at` is the way there.
    pub open: f64,
    pub from_chords: f64,
    pub from_ceiling: f64,
}

/// All zeros: what a core built in a static holds until it starts.
pub const ZERO: Drawn = Drawn {
    width: 0.0,
    ceiling: 0.0,
    room: 0.0,
    spread: [0.0; 3],
    kit: 0.0,
    kit_soft: 0.0,
    texture: 0.0,
    glass: 0.0,
    wash: 0.0,
    wash_cutoff: 0.0,
    room_cutoff: 0.0,
    spread_cutoff: 0.0,
    place_cutoff: 0.0,
    spread_delay: 0.0,
    spread_sway: 0.0,
    spread_rate: [0.0; 3],
    chords_cutoff: 0.0,
    open: 0.0,
    from_chords: 0.0,
    from_ceiling: 0.0,
};

/// Nothing placed or widened: the stereo render is mono on each side.
pub const CENTERED: Drawn = Drawn {
    width: 0.0,
    ceiling: 0.0,
    room: 0.0,
    spread: [0.0; 3],
    kit: 0.0,
    kit_soft: 0.0,
    texture: 0.0,
    glass: 0.0,
    wash: 0.0,
    wash_cutoff: 0.0,
    room_cutoff: 180.0,
    spread_cutoff: 200.0,
    place_cutoff: 150.0,
    spread_delay: 0.012,
    spread_sway: 0.0,
    spread_rate: [0.0; 3],
    chords_cutoff: 200.0,
    open: 0.0,
    from_chords: 0.0,
    from_ceiling: 0.0,
};

impl Drawn {
    /// The space `seconds` after it started (queue item 101c): itself, but
    /// for widePad's late kind, whose chords' spread and width ceiling open
    /// from the baseline's to the wide ones over `open` seconds, on a smooth
    /// curve (slow at both ends), the spread evenly in dB and the ceiling in
    /// dB. Plain arithmetic, a power a call: cheap enough for every block.
    #[must_use]
    pub fn at(&self, seconds: f64) -> Drawn {
        let p = if self.open > 0.0 { (seconds / self.open).max(0.0) } else { 1.0 };
        if p >= 1.0 {
            // Open: exactly where it was going.
            return *self;
        }
        let k = p * p * (3.0 - 2.0 * p);
        let [melody, to, texture] = self.spread;
        let from = self.from_chords;
        // Evenly in dB where both ends are a width; straight where the
        // baseline's is none (a request's pin of 0).
        let chords = if from > 0.0 && to > 0.0 { from * math::pow(to / from, k) } else { from + (to - from) * k };
        Drawn {
            spread: [melody, chords, texture],
            ceiling: self.from_ceiling + (self.ceiling - self.from_ceiling) * k,
            ..*self
        }
    }
}

/// The draws' kind (`rng::salt`): the space's own, fixed for good.
pub const SALT: u32 = 40;

/// The recipe: each setting's range, as the core has it (`SETTINGS`, unless
/// a host has narrowed or pinned one). Plain numbers, so the core's static
/// state stays all zeros until it starts (`ZERO`).
#[derive(Clone, Copy)]
pub struct Space {
    ranges: [[f64; 2]; n::COUNT],
}

impl Space {
    /// All zeros; `new` is the recipe.
    pub const ZERO: Space = Space { ranges: [[0.0; 2]; n::COUNT] };

    /// The approved baseline.
    #[must_use]
    pub fn new() -> Self {
        Space { ranges: SETTINGS.map(|s| [s.lo, s.hi]) }
    }

    /// Set setting `which` to draw from `lo` to `hi` (the same for a pin).
    /// False for no such setting or a range that isn't one.
    pub fn range(&mut self, which: usize, lo: f64, hi: f64) -> bool {
        match self.ranges.get_mut(which) {
            Some(r) if lo.is_finite() && hi.is_finite() && lo <= hi => {
                *r = [lo, hi];
                true
            }
            _ => false,
        }
    }

    /// Setting `which`'s range now.
    #[must_use]
    pub fn get(&self, which: usize) -> Setting {
        let name = SETTINGS.get(which).map_or("", |s| s.name);
        let [lo, hi] = self.ranges.get(which).copied().unwrap_or([0.0; 2]);
        s(name, lo, hi)
    }

    /// The fixed value of setting `which` (its range's low end).
    fn fixed(&self, which: usize) -> f64 {
        self.get(which).lo
    }

    /// A loop's space: each range drawn from the loop's seed, in a fixed
    /// order (draw number `i` is the same for a setting whatever the others
    /// are), scaled by how busy and wet the loop is and blended.
    #[must_use]
    pub fn draw(&self, l: &Loop) -> Drawn {
        let key = Key { seed: l.seed, pass: 0, step: 0, ord: 0 };
        let u = |i: u32| unit(key, 0, SALT, i);
        let pick = |which: usize| {
            let s = self.get(which);
            s.lo + (s.hi - s.lo) * u(100 + which as u32)
        };
        // How busy: 0 under BUSY_FROM notes a second, 1 at BUSY_FROM +
        // BUSY_SPAN; the width from WIDTH_LEAST up to 1, and less for a wet
        // loop.
        let span = self.fixed(n::BUSY_SPAN).max(1e-9);
        let busy = ((l.busy - self.fixed(n::BUSY_FROM)) / span).clamp(0.0, 1.0);
        let least = self.fixed(n::WIDTH_LEAST);
        let mut width = least + (1.0 - least) * busy;
        if l.wet > self.fixed(n::WET_FROM) {
            width *= self.fixed(n::WET_SCALE);
        }
        // The ceiling by how busy the loop is: busy, sparse, very sparse.
        let class = |of: [usize; 3]| {
            if busy > self.fixed(n::DENSE) {
                self.fixed(of[0])
            } else if busy > self.fixed(n::SPARSE) {
                self.fixed(of[1])
            } else {
                self.fixed(of[2])
            }
        };
        let ceiling = class([n::CEILING_BUSY, n::CEILING_SPARSE, n::CEILING_VERY_SPARSE]);
        // Which side the light kit goes to; a light texture and glass go to
        // the other.
        let side = if u(0) < 0.5 { -1.0 } else { 1.0 };
        let room_w = self.fixed(n::ROOM_WEIGHT);
        let ens_w = self.fixed(n::ENSEMBLE_WEIGHT);
        let scale = self.fixed(n::SPREAD_SCALE);
        let most = self.fixed(n::SPREAD_MAX);
        let spread = |which: usize| (width * pick(which) * scale).min(most) * ens_w;
        let texture_spread = if l.light { spread(n::SPREAD_LIGHT) } else { spread(n::SPREAD_TEXTURE) };
        let place = -side * width * pick(n::TEXTURE_PLACE) * ens_w;
        let rate = self.fixed(n::SPREAD_RATE);
        let step = self.fixed(n::SPREAD_RATE_STEP);
        // widePad (queue item 101c): the chords' spread past the baseline's
        // range and the most, the ceiling raised and the chords' side kept
        // above 250 Hz, all scaled by the loop's width as the baseline's are,
        // so a sparse loop stays at the narrow end. The late kind starts at
        // the baseline's and opens to it (`Drawn::at`).
        let chords = spread(n::SPREAD_CHORDS);
        let kind = self.fixed(n::WIDE);
        let wide = kind > 0.0;
        let (wide_chords, wide_ceiling) = if wide {
            let most = self.fixed(n::WIDE_MAX);
            (
                (width * self.fixed(n::WIDE_CHORDS) * scale).min(most) * ens_w,
                class([n::WIDE_CEILING_BUSY, n::WIDE_CEILING_SPARSE, n::WIDE_CEILING_VERY_SPARSE]),
            )
        } else {
            (chords, ceiling)
        };
        let late = kind > 1.0;
        Drawn {
            width,
            ceiling: wide_ceiling,
            room: width * pick(n::ROOM) * room_w,
            spread: [spread(n::SPREAD_MELODY), wide_chords, texture_spread],
            kit: side * width * pick(n::KIT) * ens_w,
            kit_soft: self.fixed(n::KIT_SOFT),
            texture: if l.light { place } else { 0.0 },
            glass: if l.light { 0.0 } else { place },
            wash: self.fixed(n::WASH) * (1.0 - self.fixed(n::WASH_FOLLOWS) * (1.0 - width)),
            wash_cutoff: self.fixed(n::WASH_CUTOFF),
            room_cutoff: self.fixed(n::ROOM_CUTOFF),
            spread_cutoff: self.fixed(n::SPREAD_CUTOFF),
            place_cutoff: self.fixed(n::PLACE_CUTOFF),
            spread_delay: self.fixed(n::SPREAD_DELAY),
            spread_sway: self.fixed(n::SPREAD_SWAY),
            // Melody, chords, texture: the sketch's rates by its channel
            // order (chords, melody, texture).
            spread_rate: [rate + step, rate, rate + 2.0 * step],
            chords_cutoff: if wide { self.fixed(n::WIDE_CUTOFF) } else { self.fixed(n::SPREAD_CUTOFF) },
            open: if late { pick(n::WIDE_OPEN).max(0.0) } else { 0.0 },
            from_chords: chords,
            from_ceiling: ceiling,
        }
    }
}

impl Default for Space {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_setting_is_named_once_in_plain_ascii() {
        for (i, a) in SETTINGS.iter().enumerate() {
            assert!(!a.name.is_empty() && a.name.is_ascii() && a.lo <= a.hi, "{}", a.name);
            assert!(SETTINGS.iter().skip(i + 1).all(|b| b.name != a.name), "{}", a.name);
        }
    }

    #[test]
    fn a_loop_draws_inside_its_ranges_and_the_same_way_every_time() {
        let space = Space::new();
        for seed in 1..400u32 {
            let l = Loop { seed, busy: f64::from(seed % 9), wet: f64::from(seed % 10) / 10.0, light: seed % 3 == 0 };
            let d = space.draw(&l);
            assert_eq!(d, space.draw(&l));
            assert!((0.18..=1.0).contains(&d.width), "{}", d.width);
            assert!([-7.0, -9.0, -12.0].contains(&d.ceiling));
            assert!(d.room >= 0.64 * 0.7 * d.width - 1e-12 && d.room <= 0.64 * 0.9 * d.width + 1e-12);
            assert!(d.spread.iter().all(|&x| (0.0..=0.4).contains(&x)));
            assert!(d.kit.abs() >= 0.5 * 0.08 * d.width - 1e-12 && d.kit.abs() <= 0.5 * 0.15 * d.width + 1e-12);
            // A light texture and glass to the kit's other side, one at a time.
            let other = if l.light { d.texture } else { d.glass };
            assert!(other * d.kit < 0.0 && (if l.light { d.glass } else { d.texture }) == 0.0);
        }
        // Busy loops wide, sparse ones narrow; wet ones narrower.
        let at = |busy, wet| space.draw(&Loop { seed: 5, busy, wet, light: false }).width;
        assert!((at(0.0, 0.0) - 0.25).abs() < 1e-12 && (at(9.0, 0.0) - 1.0).abs() < 1e-12);
        assert!((at(9.0, 0.8) - 0.75).abs() < 1e-12);
    }

    /// widePad (queue item 101c): the chords alone wider, by the loop's
    /// width (a sparse loop at the narrow end), the ceiling raised and the
    /// chords' side above 250 Hz; every other amount as the baseline draws
    /// it. The late kind opens from the baseline's to the same over 60-120 s,
    /// smoothly, and holds there.
    #[test]
    fn wide_pad_widens_the_chords_alone_and_the_late_kind_opens_to_it() {
        let base = Space::new();
        let mut on = Space::new();
        assert!(on.range(n::WIDE, 1.0, 1.0));
        let mut late = Space::new();
        assert!(late.range(n::WIDE, 2.0, 2.0));
        for seed in 1..300u32 {
            let l = Loop { seed, busy: f64::from(seed % 9), wet: f64::from(seed % 10) / 10.0, light: seed % 3 == 0 };
            let (b, w, s) = (base.draw(&l), on.draw(&l), late.draw(&l));
            assert_eq!((b.open, b.chords_cutoff, b.from_chords, b.from_ceiling), (0.0, 200.0, b.spread[1], b.ceiling));
            // The chords: 1.4 of the width's, scaled, against 0.3-0.5.
            assert!((w.spread[1] - (w.width * 1.4 * 1.6).min(2.0) * 0.5).abs() < 1e-12, "{}", w.spread[1]);
            assert!(w.spread[1] >= b.spread[1] * 2.5 - 1e-12 && w.spread[1] <= 1.0);
            let raised = match b.ceiling as i32 { -7 => -4.0, -9 => -5.0, _ => -7.0 };
            assert!((w.ceiling - raised).abs() < 1e-12 && (w.chords_cutoff - 250.0).abs() < 1e-12 && w.open == 0.0);
            // Nothing else moves.
            let same = |d: &Drawn| Drawn { spread: [d.spread[0], 0.0, d.spread[2]], ceiling: 0.0, chords_cutoff: 0.0, open: 0.0, ..*d };
            assert_eq!(same(&w), same(&b));
            assert_eq!(w.at(1e9), w);
            // Late: the wide one's destination, from the baseline's.
            assert!((60.0..=120.0).contains(&s.open), "{}", s.open);
            assert_eq!(Drawn { open: 0.0, ..s }, w);
            let start = s.at(0.0);
            assert!((start.spread[1] - b.spread[1]).abs() < 1e-12 && (start.ceiling - b.ceiling).abs() < 1e-12);
            assert_eq!(s.at(s.open), Drawn { ..s });
            let mut last = start.spread[1];
            for k in 1..=100 {
                let now = s.at(s.open * f64::from(k) / 100.0);
                assert!(now.spread[1] >= last - 1e-12 && now.ceiling >= start.ceiling - 1e-12);
                last = now.spread[1];
            }
            assert!((last - w.spread[1]).abs() < 1e-12);
        }
    }

    #[test]
    fn a_pinned_setting_draws_its_pin() {
        let mut space = Space::new();
        assert!(space.range(n::KIT, 0.1, 0.1));
        assert!(!space.range(n::COUNT, 0.0, 1.0));
        assert!(!space.range(n::KIT, 1.0, 0.0));
        let d = space.draw(&Loop { seed: 9, busy: 9.0, wet: 0.0, light: false });
        assert!((d.kit.abs() - 0.05).abs() < 1e-12, "{}", d.kit);
    }
}
