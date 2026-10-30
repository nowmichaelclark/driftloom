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
/// every measurement: the same numbers).
struct Walk<'a> {
    k: &'a Today,
    f1: f64,
    landed: [f64; 3],
    sweep: [[f64; 3]; SWEEP_STEPS],
    punch: [f64; PUNCH_STEPS],
}

impl<'a> Walk<'a> {
    fn new(k: &'a Today, f1: f64) -> Self {
        let mut walk = Walk { k, f1, landed: BANDS.map(|b| b.weight(f1)), sweep: [[0.0; 3]; SWEEP_STEPS], punch: [0.0; PUNCH_STEPS] };
        let mut t = 0.0;
        for i in 0..PUNCH_STEPS {
            if i < SWEEP_STEPS && t < k.sweep {
                walk.sweep[i] = walk.sweeping(t);
            }
            walk.punch[i] = walk.punching(t);
            t += 0.0005;
        }
        walk
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

    /// `energy(f1, tail, rate, w)` through the three bands at once: the
    /// kick's energy, its sweep landing on `f1`, its punch falling to `tail`
    /// (1: none), then at `rate` dB a second; each band's sum its own, added
    /// in the JavaScript's order. With `stop` (the lift squared and each
    /// band's bound): none as soon as one band's lifted sum passes its
    /// bound, as the whole sum then would (every step adds a number of zero
    /// or more, and a sum of IEEE doubles never falls as it grows).
    fn energies(&self, tail: f64, rate: f64, stop: Option<(f64, [f64; 3])>) -> Option<[f64; 3]> {
        let k = self.k;
        let t1 = k.att + -20.0 * math::log10(tail) / PUNCH;
        let end = t1 + (80.0 + 20.0 * math::log10(tail)) / rate;
        let mut e = [0.0; 3];
        let mut t = 0.0;
        let mut i = 0;
        while t < end {
            let a = if t < k.att || t < t1 {
                self.punch.get(i).copied().unwrap_or_else(|| self.punching(t))
            } else {
                tail * math::pow(10.0, -rate * (t - t1) / 20.0)
            };
            let w = if t < k.sweep { self.sweep.get(i).copied().unwrap_or_else(|| self.sweeping(t)) } else { self.landed };
            for (x, w) in e.iter_mut().zip(w) {
                *x += a * a * w;
            }
            if let Some((lift2, bound)) = stop
                && e.iter().zip(bound).any(|(x, b)| lift2 * x > b)
            {
                return None;
            }
            t += 0.0005;
            i += 1;
        }
        Some(e)
    }
}

/// Today's kick's three energies (`ref`), the same for every loop.
fn today(k: &Today) -> [f64; 3] {
    Walk::new(k, k.end).energies(1.0, PUNCH, None).unwrap_or([0.0; 3])
}

/// `kickFit`: the lift that keeps a kick (for a long boom, its punch) at
/// today's loudness by ear and its own energy under 60 Hz and at 60-120 Hz
/// within the band's room of today's (`refs`, today's three); then, for a
/// long boom, its tail as loud as fits within the tail's room. Each by 24
/// halvings, as the sketch did. A lift's test measures the kick with no
/// tail, the same for every lift tried, so its energies are measured once.
fn kick_fit(walk: &Walk, long: bool, dbps: f64, refs: &[f64; 3]) -> (f64, f64) {
    // `ok`'s test, band by band: `lift * lift * e <= room * ref * 1.0001`.
    let bounds = |room: f64| [room * refs[0] * 1.0001, KICK.band * refs[1] * 1.0001, KICK.band * refs[2] * 1.0001];
    let fits = |lift: f64, e: &[f64; 3], bound: &[f64; 3]| e.iter().zip(bound).all(|(&x, &b)| lift * lift * x <= b);
    let punch = walk.energies(1.0, if long { PUNCH } else { dbps }, None).unwrap_or([f64::NAN; 3]);
    let bound = bounds(KICK.loud);
    let (mut lo, mut hi) = (0.0, 1.5);
    for _ in 0..24 {
        let m = (lo + hi) / 2.0;
        if fits(m, &punch, &bound) {
            lo = m;
        } else {
            hi = m;
        }
    }
    let lift = lo;
    if !long {
        return (lift, 1.0);
    }
    let bound = bounds(KICK.tail_loud);
    let (mut tlo, mut thi) = (0.001, 1.0);
    for _ in 0..24 {
        let m = math::sqrt(tlo * thi);
        if walk.energies(m, dbps, Some((lift * lift, bound))).is_some_and(|e| fits(lift, &e, &bound)) {
            tlo = m;
        } else {
            thi = m;
        }
    }
    (lift, tlo)
}

/// `kickOf(seed, root)`: a loop's tuned kick, drawn on its own stream: both
/// falls' rates, its note (the key's, between G1 and D2; D# to F# take
/// their fifth), and each fall's level by ear, worked out as the sketch did
/// at load.
#[must_use]
pub fn kick_of(seed: u32, root: f64) -> TunedKick {
    let mut r = Rng::new(js::salted(seed, KICK_SALT, 1));
    let short_rate = r.range(KICK.short[0], KICK.short[1]);
    let long_rate = r.range(KICK.long[0], KICK.long[1]);
    let pc = ((root % 12.0) + 12.0) % 12.0;
    let key = if (3.0..=6.0).contains(&pc) { (pc + 7.0) % 12.0 } else { pc };
    let midi = 24.0 + key + if key < 7.0 { 12.0 } else { 0.0 };
    let hz = 440.0 * math::pow(2.0, (midi - 69.0) / 12.0);
    let (hard_today, soft_today) = (today(&HARD), today(&SOFT));
    let (hard, soft) = (Walk::new(&HARD, hz), Walk::new(&SOFT, hz));
    let fall = |long: bool, dbps: f64| {
        let (lift, tail) = kick_fit(&hard, long, dbps, &hard_today);
        let (soft_lift, soft_tail) = kick_fit(&soft, long, dbps, &soft_today);
        Fall { dbps, lift, tail, soft_lift, soft_tail }
    };
    TunedKick { midi, hz, short: fall(false, short_rate), long: fall(true, long_rate) }
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
