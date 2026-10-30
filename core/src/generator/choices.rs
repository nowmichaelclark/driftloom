//! The per-loop choices' own draws (queue items 63 to 76d, ported in item
//! 81): what each choice a loop drew plays, from `js/choices.js`. The arc's
//! shape, period, start and floor, and whether a glacial loop plays it
//! outside its choices; the tuned kick's note, falls and levels by ear, and
//! the kick a wandered pass swaps to; subDuck's depth and recovery;
//! lateFloor's wait, order and second arrival; the floor drop's shapes;
//! padSweep's sweep; the echo's time; and the bass layer's level. Which
//! choices a loop gets is drawn in `render` (`drawChoices`, which the pedal
//! needs), and the pedal's own draws are `layers::pedal_bass`'s.
//!
//! Each draws from the loop's seed on a stream of its own (a salt each, the
//! sketch's), so no other draw moves and a loop replays the same way, and
//! every number it draws by is `data/choices.txt`'s (`data`). The requests'
//! pins (`pins` in the JavaScript) are phase 2's item 85: the engine asks
//! for none, so none are here yet.

use super::character::Vibes;
use super::data::{self, ARC, BASS_LIFT, ECHO, FLOORDROP, KICK, LATEFLOOR, PADSWEEP, SUBDUCK, Table};
use super::js;
use super::layers::DrumEvent;
use super::rng::Rng;
use crate::math;

/// `leanOf(table, vibes)`: a choice's lean for a loop's vibes, each vibe's
/// held between the choices' bounds and raised to its strength times its
/// weight; 1 for a loop without vibes, or no table.
#[must_use]
pub fn lean_of(table: Option<Table>, v: Option<&Vibes>) -> f64 {
    let (Some(v), Some(table)) = (v, table) else { return 1.0 };
    let [lo, hi] = data::CHOICE_LEAN;
    let mut m = 1.0;
    for &(id, w) in v.list.iter() {
        let Some(x) = data::lookup(table, data::VIBES[id].id) else { continue };
        m *= math::pow(js::min(hi, js::max(lo, x)), v.strength * w);
    }
    m
}

/// Whether a loop's vibes include `id`.
fn has_vibe(v: &Vibes, id: &str) -> bool {
    v.list.iter().any(|&(i, _)| data::VIBES[i].id == id)
}

// ---------------------------------------------------------------- the arc

const ARC_SALT: u32 = 0x0a4c;

/// A loop's arc (`arcOf`): its shape ("deep" or "dip"), its swing's period
/// in seconds, where it starts on the swing (0 the darkest point), the
/// dip's floor in Hz and the share of the swing the dip takes.
#[derive(Clone, Copy, Debug)]
pub struct Arc {
    pub shape: &'static str,
    pub period: f64,
    pub from: f64,
    pub floor: f64,
    pub dip: f64,
}

/// The period range a loop's vibes pick (`arcPace`): the calm or the
/// bright list's, whichever the vibes weigh more (glacial's own for a calm
/// loop with glacial, slower still with one of `glacial_near`), else the
/// rest's.
fn arc_period(v: Option<&Vibes>) -> [f64; 2] {
    let p = &ARC.period;
    let Some(v) = v else { return p.rest };
    let mut calm = 0.0;
    let mut bright = 0.0;
    for &(id, w) in v.list.iter() {
        let name = data::VIBES[id].id;
        if ARC.calm.contains(&name) {
            calm += w;
        }
        if ARC.bright.contains(&name) {
            bright += w;
        }
    }
    if calm > bright && has_vibe(v, "glacial") {
        let near = v.list.iter().any(|&(id, _)| ARC.glacial_near.contains(&data::VIBES[id].id));
        return if near { p.glacial_near } else { p.glacial };
    }
    if calm > bright {
        p.calm
    } else if bright > calm {
        p.bright
    } else {
        p.rest
    }
}

/// The arc's draws on its stream, in their order (`arcDraws`): the period,
/// where it starts (a share, placed by the shape), the shape and the floor;
/// and the stream, left after them.
fn arc_draws(seed: u32, v: Option<&Vibes>) -> (f64, f64, &'static str, f64, Rng) {
    let mut r = Rng::new(js::salted(seed, ARC_SALT, 1));
    let [lo, hi] = arc_period(v);
    let period = r.range(lo, hi);
    let at = r.f();
    let shape = r
        .weighted_index(ARC.shapes.iter().map(|&(id, w)| w * lean_of(data::table_named(ARC.shape_vibes, id), v)))
        .map_or("dip", |i| ARC.shapes[i].0);
    let floor = r.range(ARC.floor[0], ARC.floor[1]);
    (period, at, shape, floor, r)
}

/// `arcOf(seed, vibes)`: a loop's arc, drawn on its own stream. The period
/// and the start are drawn first, as item 63 drew them, so a loop without
/// vibes that draws the deep swing plays the swing it always did.
#[must_use]
pub fn arc_of(seed: u32, v: Option<&Vibes>) -> Arc {
    let (period, at, shape, floor, _) = arc_draws(seed, v);
    let dip = ARC.dip;
    let [a, b] = if shape == "deep" { ARC.from_deep } else { [dip / 2.0, 1.0 - dip / 2.0] };
    Arc { shape, period, from: a + at * (b - a), floor, dip }
}

/// `arcPlays(seed, choices, vibes)`: whether a loop plays the arc. When it
/// drew it among its choices (`chosen`), or, with glacial among its vibes,
/// with the glacial chance times glacial's weight, drawn on the arc's own
/// stream after `arc_of`'s draws (queue item 76b): not one of the loop's
/// changes, so nothing else moves.
#[must_use]
pub fn arc_plays(seed: u32, chosen: bool, v: Option<&Vibes>) -> bool {
    if chosen {
        return true;
    }
    let Some(w) = v.and_then(|v| v.list.iter().find(|&&(id, _)| data::VIBES[id].id == "glacial")).map(|x| x.1) else {
        return false;
    };
    arc_draws(seed, v).4.chance(ARC.glacial_chance * w)
}

// ------------------------------------------------------------ the kick

const KICK_SALT: u32 = 0x7c1d;
const KICK_SWAP_SALT: u32 = 0x6b5a;

/// A tuned kick's fall (`fits`): its rate in dB a second, and the kick's
/// and the soft kick's level by ear and tail (1: none).
#[derive(Clone, Copy, Debug)]
pub struct Fall {
    pub dbps: f64,
    pub lift: f64,
    pub tail: f64,
    pub soft_lift: f64,
    pub soft_tail: f64,
}

/// A loop's tuned kick (`kickOf`): its note (MIDI and Hz) and its two
/// falls, so a wandering loop can swap between them.
#[derive(Clone, Copy, Debug)]
pub struct TunedKick {
    pub midi: f64,
    pub hz: f64,
    pub short: Fall,
    pub long: Fall,
}

/// Today's punch, in dB a second.
const PUNCH: f64 = 213.0;

/// Today's kick, hard or soft, as `kickFit` measures it: its sweep's start
/// and end (Hz), the sweep's time and the attack (seconds).
#[derive(Clone, Copy)]
struct Today {
    f0: f64,
    end: f64,
    sweep: f64,
    att: f64,
}

const HARD: Today = Today { f0: 128.0, end: 44.0, sweep: 0.09, att: 0.004 };
const SOFT: Today = Today { f0: 96.0, end: 46.0, sweep: 0.14, att: 0.009 };

/// The weightings a kick's energy is measured through (`H2`, `SUB`, `LOW`):
/// by ear (K-weighting's 38 Hz high-pass, squared), under 60 Hz, and at
/// 60-120 Hz.
#[derive(Clone, Copy)]
enum Band {
    Ear,
    Sub,
    Low,
}

const BANDS: [Band; 3] = [Band::Ear, Band::Sub, Band::Low];

impl Band {
    fn weight(self, f: f64) -> f64 {
        match self {
            Band::Ear => math::pow(f, 4.0) / (math::pow(f * f - 38.0 * 38.0, 2.0) + math::pow(f * 38.0 / 0.5, 2.0)),
            Band::Sub => {
                if f < 60.0 {
                    1.0
                } else {
                    0.0
                }
            }
            Band::Low => {
                if (60.0..120.0).contains(&f) {
                    1.0
                } else {
                    0.0
                }
            }
        }
    }
}

/// The steps `energy`'s walk keeps a table of: the sweep's (it lasts at
/// most 0.14 s) and the punch's first 0.4 s, past the longest it falls
/// before a tail takes over (60 dB, to the quietest tail tried: 0.29 s). A
/// step past either is worked out where it falls.
const SWEEP_STEPS: usize = 300;
const PUNCH_STEPS: usize = 800;

/// A kick's walk, as `energy` takes it: the time from 0 by half a
/// millisecond (summed as the JavaScript sums it, so step `i` is the same
/// time in every walk), the weight of each band through the sweep to `f1`,
/// and the punch's amplitude. What every measurement of one kick shares is
/// worked out once, here (the JavaScript works it out at every step of
/// every measurement: the same numbers). Built a slice at a time
/// (`Walk::build`), as the player spreads a load across blocks.
#[derive(Clone, Copy)]
struct Walk {
    k: Today,
    f1: f64,
    landed: [f64; 3],
    sweep: [[f64; 3]; SWEEP_STEPS],
    punch: [f64; PUNCH_STEPS],
    /// How many steps of the tables are built, and the next one's time.
    built: usize,
    t: f64,
}

/// What a step of a walk's tables costs against a slice's budget, in the
/// units a step of `energies` costs one of (`KickFit::step`): a step of the
/// sweep works out four powers, one of the energies one.
const BUILD_COST: usize = 4;

/// An `energies` walk under way: where it is, the sums so far, and what it
/// was asked.
#[derive(Clone, Copy)]
struct Walking {
    t: f64,
    i: usize,
    e: [f64; 3],
    t1: f64,
    end: f64,
    tail: f64,
    rate: f64,
    stop: Option<(f64, [f64; 3])>,
}

impl Walk {
    const EMPTY: Walk =
        Walk { k: HARD, f1: 0.0, landed: [0.0; 3], sweep: [[0.0; 3]; SWEEP_STEPS], punch: [0.0; PUNCH_STEPS], built: 0, t: 0.0 };

    /// A walk of kick `k` landing on `f1`, its tables still to build.
    fn start(k: Today, f1: f64) -> Self {
        Walk { k, f1, landed: BANDS.map(|b| b.weight(f1)), built: 0, t: 0.0, ..Walk::EMPTY }
    }

    /// Build the tables, spending at most `left` (or what one entry costs,
    /// if `left` is less, so any slice gets on): true once built.
    fn build(&mut self, left: &mut usize) -> bool {
        while self.built < PUNCH_STEPS {
            if *left == 0 {
                return false;
            }
            *left = left.saturating_sub(BUILD_COST);
            let (i, t) = (self.built, self.t);
            if i < SWEEP_STEPS && t < self.k.sweep {
                self.sweep[i] = self.sweeping(t);
            }
            self.punch[i] = self.punching(t);
            self.t += 0.0005;
            self.built += 1;
        }
        true
    }

    /// The bands' weights at time `t` of the sweep.
    fn sweeping(&self, t: f64) -> [f64; 3] {
        let f = self.k.f0 * math::pow(self.f1 / self.k.f0, t / self.k.sweep);
        BANDS.map(|b| b.weight(f))
    }

    /// The amplitude at time `t` of the attack and the punch.
    fn punching(&self, t: f64) -> f64 {
        if t < self.k.att { t / self.k.att } else { math::pow(10.0, -PUNCH * (t - self.k.att) / 20.0) }
    }

    /// `energy(f1, tail, rate, w)` through the three bands at once, begun:
    /// the kick's energy, its sweep landing on `f1`, its punch falling to
    /// `tail` (1: none), then at `rate` dB a second; each band's sum its own,
    /// added in the JavaScript's order. With `stop` (the lift squared and
    /// each band's bound): none as soon as one band's lifted sum passes its
    /// bound, as the whole sum then would (every step adds a number of zero
    /// or more, and a sum of IEEE doubles never falls as it grows).
    fn begin(&self, tail: f64, rate: f64, stop: Option<(f64, [f64; 3])>) -> Walking {
        let k = &self.k;
        let t1 = k.att + -20.0 * math::log10(tail) / PUNCH;
        let end = t1 + (80.0 + 20.0 * math::log10(tail)) / rate;
        Walking { t: 0.0, i: 0, e: [0.0; 3], t1, end, tail, rate, stop }
    }

    /// Walk on, spending at most `left` (a unit a step): `None` while there
    /// is more to walk, then the energies (`None` within: stopped).
    #[allow(clippy::option_option)] // done or not, and then the energies or a stop
    fn walk(&self, w: &mut Walking, left: &mut usize) -> Option<Option<[f64; 3]>> {
        let k = &self.k;
        while w.t < w.end {
            if *left == 0 {
                return None;
            }
            *left -= 1;
            let a = if w.t < k.att || w.t < w.t1 {
                self.punch.get(w.i).copied().unwrap_or_else(|| self.punching(w.t))
            } else {
                w.tail * math::pow(10.0, -w.rate * (w.t - w.t1) / 20.0)
            };
            let weights = if w.t < k.sweep { self.sweep.get(w.i).copied().unwrap_or_else(|| self.sweeping(w.t)) } else { self.landed };
            for (x, wt) in w.e.iter_mut().zip(weights) {
                *x += a * a * wt;
            }
            if let Some((lift2, bound)) = w.stop
                && w.e.iter().zip(bound).any(|(x, b)| lift2 * x > b)
            {
                return Some(None);
            }
            w.t += 0.0005;
            w.i += 1;
        }
        Some(Some(w.e))
    }
}

/// `kickFit`: the lift that keeps a kick (for a long boom, its punch) at
/// today's loudness by ear and its own energy under 60 Hz and at 60-120 Hz
/// within the band's room of today's (`refs`, today's three); then, for a
/// long boom, its tail as loud as fits within the tail's room. Each by 24
/// halvings, as the sketch did. A lift's test measures the kick with no
/// tail, the same for every lift tried, so its energies are measured once.
/// Resumable: each `step` walks on as far as its budget lets it.
#[derive(Clone, Copy)]
struct Fitting {
    long: bool,
    dbps: f64,
    refs: [f64; 3],
    /// The punch's lift, once measured; the tail's halvings so far.
    lift: Option<f64>,
    k: u32,
    tlo: f64,
    thi: f64,
    walking: Option<Walking>,
}

impl Fitting {
    fn new(long: bool, dbps: f64, refs: [f64; 3]) -> Self {
        Fitting { long, dbps, refs, lift: None, k: 0, tlo: 0.001, thi: 1.0, walking: None }
    }

    /// `ok`'s test, band by band: `lift * lift * e <= room * ref * 1.0001`.
    fn bounds(&self, room: f64) -> [f64; 3] {
        let refs = &self.refs;
        [room * refs[0] * 1.0001, KICK.band * refs[1] * 1.0001, KICK.band * refs[2] * 1.0001]
    }

    fn fits(lift: f64, e: &[f64; 3], bound: &[f64; 3]) -> bool {
        e.iter().zip(bound).all(|(&x, &b)| lift * lift * x <= b)
    }

    /// Fit on along `walk`, spending at most `left`: the lift and the tail
    /// once fitted.
    fn step(&mut self, walk: &Walk, budget: &mut usize) -> Option<(f64, f64)> {
        let (long, dbps) = (self.long, self.dbps);
        if self.lift.is_none() {
            let w = self.walking.get_or_insert_with(|| walk.begin(1.0, if long { PUNCH } else { dbps }, None));
            let punch = walk.walk(w, budget)?.unwrap_or([f64::NAN; 3]);
            self.walking = None;
            let bound = self.bounds(KICK.loud);
            let (mut lo, mut hi) = (0.0, 1.5);
            for _ in 0..24 {
                let m = (lo + hi) / 2.0;
                if Self::fits(m, &punch, &bound) {
                    lo = m;
                } else {
                    hi = m;
                }
            }
            self.lift = Some(lo);
        }
        let punch = self.lift.unwrap_or(0.0);
        if !long {
            return Some((punch, 1.0));
        }
        let bound = self.bounds(KICK.tail_loud);
        while self.k < 24 {
            let m = math::sqrt(self.tlo * self.thi);
            let w = self.walking.get_or_insert_with(|| walk.begin(m, dbps, Some((punch * punch, bound))));
            let e = walk.walk(w, budget)?;
            self.walking = None;
            if e.is_some_and(|e| Self::fits(punch, &e, &bound)) {
                self.tlo = m;
            } else {
                self.thi = m;
            }
            self.k += 1;
        }
        Some((punch, self.tlo))
    }
}

/// How much of `kickOf`'s work a slice does at most (`KickFit::step`), in
/// steps of a walk: about 40 microseconds on a laptop's core.
pub const KICK_SLICE: usize = 1500;

/// `kickOf(seed, root)`'s work, a slice at a time (`step`): its draws, then
/// today's kick's energies, hard and soft, then the walks at the loop's
/// note, then the four fits (each fall, hard and soft). The same numbers
/// `kick_of` works out at once: it is this, run to the end.
#[derive(Clone, Copy)]
pub struct KickFit {
    short_rate: f64,
    long_rate: f64,
    midi: f64,
    hz: f64,
    /// Where the work is (`step`'s stages), its two walks and what they
    /// measured: today's kick's energies, each fit's lift and tail.
    stage: u8,
    walks: [Walk; 2],
    walking: Option<Walking>,
    todays: [[f64; 3]; 2],
    fitting: Option<Fitting>,
    fits: [(f64, f64); 4],
}

impl KickFit {
    /// Nothing to fit. Built in a `const` (the generator's static), never on
    /// a stack at run time.
    #[allow(clippy::large_stack_arrays)]
    pub const EMPTY: KickFit = KickFit {
        short_rate: 0.0,
        long_rate: 0.0,
        midi: 0.0,
        hz: 0.0,
        stage: 0,
        walks: [Walk::EMPTY; 2],
        walking: None,
        todays: [[0.0; 3]; 2],
        fitting: None,
        fits: [(0.0, 0.0); 4],
    };

    /// The tuned kick of a loop of `seed` in `root`: its draws, its note
    /// (the key's, between G1 and D2; D# to F# take their fifth), its fits
    /// still to work out.
    #[must_use]
    pub fn new(seed: u32, root: f64) -> Self {
        let mut r = Rng::new(js::salted(seed, KICK_SALT, 1));
        let short_rate = r.range(KICK.short[0], KICK.short[1]);
        let long_rate = r.range(KICK.long[0], KICK.long[1]);
        let pc = ((root % 12.0) + 12.0) % 12.0;
        let key = if (3.0..=6.0).contains(&pc) { (pc + 7.0) % 12.0 } else { pc };
        let midi = 24.0 + key + if key < 7.0 { 12.0 } else { 0.0 };
        let hz = 440.0 * math::pow(2.0, (midi - 69.0) / 12.0);
        KickFit { short_rate, long_rate, midi, hz, ..KickFit::EMPTY }
    }

    /// Work on, at most `budget` steps of a walk (`KICK_SLICE`); true once
    /// the kick is worked out (`kick`).
    pub fn step(&mut self, budget: usize) -> bool {
        let mut left = budget;
        loop {
            match self.stage {
                // Today's kick, hard then soft: its walk, then its energies.
                0 | 3 => {
                    let k = if self.stage == 0 { HARD } else { SOFT };
                    self.walks[0] = Walk::start(k, k.end);
                    self.stage += 1;
                }
                1 | 4 | 7 | 9 => {
                    let slot = usize::from(self.stage == 9);
                    if !self.walks[slot].build(&mut left) {
                        return false;
                    }
                    self.stage += 1;
                }
                2 | 5 => {
                    let w = self.walking.get_or_insert_with(|| self.walks[0].begin(1.0, PUNCH, None));
                    let Some(e) = self.walks[0].walk(w, &mut left) else { return false };
                    self.walking = None;
                    self.todays[usize::from(self.stage == 5)] = e.unwrap_or([0.0; 3]);
                    self.stage += 1;
                }
                // The walks at the loop's note, hard then soft.
                6 => {
                    self.walks[0] = Walk::start(HARD, self.hz);
                    self.stage = 7;
                }
                8 => {
                    self.walks[1] = Walk::start(SOFT, self.hz);
                    self.stage = 9;
                }
                // The fits: the short fall hard and soft, the long fall hard
                // and soft.
                10..=13 => {
                    let n = usize::from(self.stage - 10);
                    let (slot, long) = (n % 2, n >= 2);
                    let (dbps, refs) = (if long { self.long_rate } else { self.short_rate }, self.todays[slot]);
                    let f = self.fitting.get_or_insert_with(|| Fitting::new(long, dbps, refs));
                    let Some(fit) = f.step(&self.walks[slot], &mut left) else { return false };
                    self.fitting = None;
                    self.fits[n] = fit;
                    self.stage += 1;
                }
                _ => return true,
            }
        }
    }

    /// The kick worked out (once `step` says so).
    #[must_use]
    pub fn kick(&self) -> TunedKick {
        let fall = |dbps: f64, hard: (f64, f64), soft: (f64, f64)| Fall { dbps, lift: hard.0, tail: hard.1, soft_lift: soft.0, soft_tail: soft.1 };
        TunedKick {
            midi: self.midi,
            hz: self.hz,
            short: fall(self.short_rate, self.fits[0], self.fits[1]),
            long: fall(self.long_rate, self.fits[2], self.fits[3]),
        }
    }
}

/// `kickOf(seed, root)`: a loop's tuned kick, drawn on its own stream: both
/// falls' rates, its note, and each fall's level by ear, worked out as the
/// sketch did at load (`KickFit`, run to the end at once).
#[must_use]
pub fn kick_of(seed: u32, root: f64) -> TunedKick {
    let mut fit = KickFit::new(seed, root);
    while !fit.step(usize::MAX) {}
    fit.kick()
}

/// `kickKindOf(seed, kind, pass)`: the kick a wandering loop plays on pass
/// `pass`. Its own (`kind`) on the first pass and most others; now and
/// then one of the other two, half and half, drawn from the pass alone
/// (queue item 67), so any pass can be reached without the ones before it.
#[must_use]
pub fn kick_kind_of(seed: u32, kind: &'static str, pass: u32) -> &'static str {
    if pass == 0 {
        return kind;
    }
    let mut r = Rng::new((seed ^ KICK_SWAP_SALT).wrapping_add(pass.wrapping_mul(0x9e37_79b1)));
    if !r.chance(KICK.swap) {
        return kind;
    }
    let mut others = ["today", "short", "long"].into_iter().filter(|&k| k != kind);
    let (first, second) = (others.next(), others.next());
    (if r.chance(0.5) { first } else { second }).unwrap_or(kind)
}

// ------------------------------------------------- subDuck and lateFloor

const SUBDUCK_SALT: u32 = 0x5dc4;

/// A loop's subDuck (`subDuckOf`): the dip at a kick in dB, and the
/// recovery in seconds.
#[derive(Clone, Copy, Debug)]
pub struct SubDuck {
    pub depth: f64,
    pub recover: f64,
}

/// `subDuckOf(seed, bpm)`: drawn on its own stream, after the draw the
/// sketch spent on whether the loop ducked; the recovery never longer
/// than the setting's share of a beat.
#[must_use]
pub fn sub_duck_of(seed: u32, bpm: f64) -> SubDuck {
    let mut r = Rng::new(js::salted(seed, SUBDUCK_SALT, 1));
    r.f();
    let depth = r.range(SUBDUCK.depth[0], SUBDUCK.depth[1]);
    let recover = js::min(r.range(SUBDUCK.recover[0], SUBDUCK.recover[1]), SUBDUCK.beat * 60.0 / bpm);
    SubDuck { depth, recover }
}

const LATEFLOOR_SALT: u32 = 0x1a7e;

/// A loop's lateFloor (`lateFloorOf`): in what order the kit and the bass
/// arrive, the wait in bars, the gap between the two arrivals in bars, and
/// the steps from the load each sits out.
#[derive(Clone, Copy, Debug)]
pub struct LateFloor {
    pub order: &'static str,
    pub bars: f64,
    pub gap: f64,
    pub drums: f64,
    pub bass: f64,
}

/// `lateFloorOf(seed, spb, stepDur, {}, vibes, loopBars)`: drawn on its own
/// stream, after the draw the sketch spent on whether the loop waited. The
/// vibes bend the wait's one draw toward the long end (`u^(1/lean)`) and
/// lean the gap's long end; the gap is drawn last (queue item 76d), so
/// nothing before it moves, and the second arrival stays inside the loop's
/// first pass when the first is (`loop_bars` its length in bars), else
/// within one loop's length of the first.
#[must_use]
pub fn late_floor_of(seed: u32, spb: f64, step_dur: f64, v: Option<&Vibes>, loop_bars: f64) -> LateFloor {
    let mut r = Rng::new(js::salted(seed, LATEFLOOR_SALT, 1));
    r.f();
    let lean = lean_of(Some(LATEFLOOR.long_vibes), v);
    let u = r.f();
    let [lo, hi] = LATEFLOOR.seconds;
    let secs = lo + math::pow(u, 1.0 / lean) * (hi - lo);
    let order = r.weighted(LATEFLOOR.orders).unwrap_or("together");
    let k = LATEFLOOR.round;
    let bars = js::max(k, js::round(secs / (spb * step_dur) / k) * k);
    let drawn = r
        .weighted_index(LATEFLOOR.gap.iter().map(|&(g, w)| if g >= LATEFLOOR.gap_long { w * lean } else { w }))
        .map_or(1.0, |i| LATEFLOOR.gap[i].0);
    let room = if bars < loop_bars { loop_bars - bars } else { loop_bars };
    let gap = js::max(1.0, js::min(js::min(drawn, LATEFLOOR.max_gap), room));
    LateFloor {
        order,
        bars,
        gap,
        drums: (if order == "bassFirst" { bars + gap } else { bars }) * spb,
        bass: (if order == "kitFirst" { bars + gap } else { bars }) * spb,
    }
}

/// `dropShapeOf(rng, hasBass, vibes)`: one floor drop's shape, drawn with
/// `rng` (drift's, at the end of its draws), leaned by the vibes; a loop
/// without a bass line never draws the shapes that need one.
pub fn drop_shape_of(rng: &mut Rng, has_bass: bool, v: Option<&Vibes>) -> &'static str {
    let shapes = FLOORDROP.shapes.iter().filter(|&&(id, _)| has_bass || !FLOORDROP.need_bass.contains(&id));
    rng.weighted_index(shapes.clone().map(|&(id, w)| w * lean_of(data::table_named(FLOORDROP.shape_vibes, id), v)))
        .and_then(|i| shapes.clone().nth(i))
        .map_or("single", |x| x.0)
}

// ------------------------------------------- padSweep, the echo, the bass

const PADSWEEP_SALT: u32 = 0x5e3e;

/// A loop's padSweep (`padSweepOf`): where the lowpass closes (Hz), how
/// far it opens (octaves), one swing (seconds) and its resonance.
#[derive(Clone, Copy, Debug)]
pub struct PadSweep {
    pub lo: f64,
    pub oct: f64,
    pub period: f64,
    pub q: f64,
}

/// `padSweepOf(seed)`: drawn on its own stream in the sketch's order,
/// after the draw it spent on whether the loop swept.
#[must_use]
pub fn pad_sweep_of(seed: u32) -> PadSweep {
    let mut r = Rng::new(js::salted(seed, PADSWEEP_SALT, 1));
    r.f();
    let lo = r.range(PADSWEEP.lo[0], PADSWEEP.lo[1]);
    let oct = r.range(PADSWEEP.oct[0], PADSWEEP.oct[1]);
    let period = r.range(PADSWEEP.period[0], PADSWEEP.period[1]);
    let q = r.range(PADSWEEP.q[0], PADSWEEP.q[1]);
    PadSweep { lo, oct, period, q }
}

const ECHO_SALT: u32 = 0xec40;

/// `echoTimeOf(seed, vibes)`: a loop's echo time in beats, the sketch's one
/// draw on its own stream, leaned by the vibes (queue item 73). Every loop
/// draws it, outside the choices.
#[must_use]
pub fn echo_time_of(seed: u32, v: Option<&Vibes>) -> f64 {
    let mut r = Rng::new(js::salted(seed, ECHO_SALT, 1));
    r.weighted_index(ECHO.beats.iter().map(|&(b, w)| w * lean_of(data::table_at(ECHO.vibes, b), v))).map_or(0.75, |i| ECHO.beats[i].0)
}

/// `bassLiftOf(tracks)`: the bass layer's level in dB (queue item 76c), the
/// kick's for a loop that plays a kick, a soft kick or the hand kit's frame
/// drum anywhere in it, else the other.
#[must_use]
pub fn bass_lift_of(drums: &[DrumEvent]) -> f64 {
    let kick = drums.iter().any(|e| e.vel != 0.0 && matches!(e.inst, "kick" | "softkick" | "frame"));
    if kick { BASS_LIFT.kick } else { BASS_LIFT.no_kick }
}
