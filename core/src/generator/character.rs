//! A loop's profile, as its mix blends them and its vibes lean them:
//! `blendMix` (`js/characters.js`) and the vibes' hooks (`drawVibes`,
//! `vibePull`, `vibeLean`, `vibedCharacter` in `js/generator.js`).

use super::data::{self, CHARACTERS, Character, Gait, NumPool, POOL_MAX, Pool, Progressions, Table, VIBES, Vibe};
use super::js;
use super::list::List;
use super::rng::Rng;
use crate::math;

/// At most this many vibes steer one loop (`VIBE_MAX`).
pub const VIBE_MAX: usize = 5;
const VIBE_SALT: u32 = 0x5649_4245;
const VIBE_FLOOR: f64 = 0.05;
const VIBE_STRENGTH: [f64; 2] = [0.5, 0.9];

/// `q8`: onto the 1/255 grid a share code writes.
#[must_use]
pub fn q8(v: f64) -> f64 {
    js::round(clamp01(v) * 255.0) / 255.0
}

/// `clamp01`.
#[must_use]
pub fn clamp01(v: f64) -> f64 {
    js::max(0.0, js::min(1.0, v))
}

/// A loop's vibes: each vibe's number and weight, in the frozen order, and
/// the strength (`{ vibes, strength }`).
#[derive(Clone, Copy)]
pub struct Vibes {
    pub list: List<(usize, f64), VIBE_MAX>,
    pub strength: f64,
}

/// The vibes a selection asks for, in the frozen order, each once
/// (`vibeSelection`): `chosen[i]` is whether vibe `i` was picked.
#[must_use]
pub fn selection(picked: &[usize]) -> [bool; data::VIBE_COUNT] {
    let mut chosen = [false; data::VIBE_COUNT];
    for &i in picked {
        if let Some(c) = chosen.get_mut(i) {
            *c = true;
        }
    }
    chosen
}

/// `drawVibes`: the loop's vibes from its seed and the selection, or none.
#[must_use]
pub fn draw_vibes(seed: u32, chosen: &[bool; data::VIBE_COUNT]) -> Option<Vibes> {
    let mut ids = List::<usize, { data::VIBE_COUNT }>::filled(0);
    for (i, &c) in chosen.iter().enumerate() {
        if c {
            ids.push(i);
        }
    }
    if ids.is_empty() {
        return None;
    }
    let mut r = Rng::new(js::salted(seed, VIBE_SALT, 19));
    let mut picked = List::<usize, VIBE_MAX>::filled(0);
    if ids.len() <= VIBE_MAX {
        for &i in ids.iter() {
            picked.push(i);
        }
    } else {
        r.shuffle(&mut ids);
        let mut five = [usize::MAX; VIBE_MAX];
        five.copy_from_slice(&ids[..VIBE_MAX]);
        five.sort_unstable();
        for i in five {
            picked.push(i);
        }
    }
    let mut draws = [0.0; VIBE_MAX];
    for d in draws.iter_mut().take(picked.len()) {
        *d = -math::log(1.0 - r.f() * 0.999);
    }
    let mut total = 0.0;
    for d in &draws[..picked.len()] {
        total += d;
    }
    let spare = 1.0 - VIBE_FLOOR * js::num(picked.len());
    let mut list = List::filled((0, 0.0));
    for (k, &id) in picked.iter().enumerate() {
        let w = if picked.len() == 1 { 1.0 } else { q8(VIBE_FLOOR + spare * (draws[k] / total)) };
        list.push((id, w));
    }
    Some(Vibes { list, strength: q8(r.range(VIBE_STRENGTH[0], VIBE_STRENGTH[1])) })
}

/// `vibePull`: a value moved part-way toward the weighted mean of the
/// vibes that set a target for it.
#[must_use]
pub fn pull(v: Option<&Vibes>, get: impl Fn(&Vibe) -> Option<f64>, value: f64) -> f64 {
    let Some(v) = v else { return value };
    let mut weight = 0.0;
    let mut sum = 0.0;
    for &(id, w) in v.list.iter() {
        let Some(t) = get(&VIBES[id]) else { continue };
        weight += w;
        sum += w * t;
    }
    if weight == 0.0 {
        return value;
    }
    let pull = js::min(1.0, v.strength * weight);
    value + (sum / weight - value) * pull
}

/// `vibeLean(v, field)`: a single-number lean.
#[must_use]
pub fn lean(v: Option<&Vibes>, get: impl Fn(&Vibe) -> Option<f64>) -> f64 {
    let Some(v) = v else { return 1.0 };
    let mut m = 1.0;
    for &(id, w) in v.list.iter() {
        if let Some(x) = get(&VIBES[id])
            && x > 0.0
        {
            m *= math::pow(x, v.strength * w);
        }
    }
    m
}

/// `vibeLean(v, field, name)`: a pool lean, one entry of a vibe's table.
#[must_use]
pub fn lean_in(v: Option<&Vibes>, get: impl Fn(&Vibe) -> Option<Table>, name: &str) -> f64 {
    lean(v, |rec| get(rec).and_then(|t| data::lookup(t, name)))
}

/// A pool of names, owned (a blend's pools are built).
pub type Names = List<(&'static str, f64), POOL_MAX>;
/// A pool of numbers, owned.
pub type Nums = List<(f64, f64), POOL_MAX>;

fn names(p: Option<Pool>) -> Option<Names> {
    p.map(|p| List::from_slice(("", 0.0), p))
}

fn nums(p: Option<NumPool>) -> Option<Nums> {
    p.map(|p| List::from_slice((0.0, 0.0), p))
}

/// A loop's profile: one profile, or a blend, with the vibes' leans
/// applied (`characterOf`). The fields are the JavaScript object's; a field
/// the object lacks is `None`.
#[derive(Clone, Copy)]
pub struct Char {
    /// The leading profile (all-or-nothing traits are its).
    pub lead: usize,
    /// A single profile (the CHARACTERS entry itself in the JavaScript).
    pub single: bool,
    pub tone: [[f64; 2]; 3],
    pub feel: [[f64; 2]; 3],
    pub bpm: Option<[f64; 2]>,
    pub swing: Option<[f64; 2]>,
    pub hat_density: Option<[f64; 2]>,
    pub drums: Option<f64>,
    pub rest_bar: Option<f64>,
    pub level: Option<f64>,
    pub flat_seven: Option<f64>,
    pub quartal: Option<f64>,
    pub pointillist: Option<f64>,
    pub skip_step: Option<f64>,
    pub glide: Option<f64>,
    pub airy: Option<f64>,
    pub pump: Option<f64>,
    pub rolls: Option<f64>,
    pub four_floor: Option<f64>,
    pub dub_echo: Option<f64>,
    pub scales: Option<Names>,
    pub steps_per_bar: Option<Nums>,
    pub bars: Option<Nums>,
    pub chord_voices: Option<Names>,
    pub melody_voices: Option<Names>,
    pub bass_styles: Option<Names>,
    pub bass_voices: Option<Names>,
    pub chord_size: Option<Nums>,
    pub textures: Option<Names>,
    pub polymeter: Option<bool>,
    pub micro_loop: Option<bool>,
    pub gaits: Option<List<Gait, 4>>,
    pub kits: Option<Pool>,
    pub progressions: Option<Progressions>,
    pub drone: Option<f64>,
    pub ornament: Option<f64>,
    pub harmonize: Option<f64>,
    pub chug: Option<f64>,
    pub groove: Option<&'static str>,
}

impl Char {
    /// One profile as it stands.
    fn of(i: usize) -> Self {
        let c: &Character = &CHARACTERS[i];
        Char {
            lead: i,
            single: true,
            tone: c.tone,
            feel: c.feel,
            bpm: c.bpm,
            swing: c.swing,
            hat_density: c.hat_density,
            drums: c.drums,
            rest_bar: c.rest_bar,
            level: c.level,
            flat_seven: c.flat_seven,
            quartal: c.quartal,
            pointillist: c.pointillist,
            skip_step: c.skip_step,
            glide: c.glide,
            airy: c.airy,
            pump: c.pump,
            rolls: c.rolls,
            four_floor: c.four_floor,
            dub_echo: c.dub_echo,
            scales: names(c.scales),
            steps_per_bar: nums(c.steps_per_bar),
            bars: nums(c.bars),
            chord_voices: names(c.chord_voices),
            melody_voices: names(c.melody_voices),
            bass_styles: names(c.bass_styles),
            bass_voices: names(c.bass_voices),
            chord_size: nums(c.chord_size),
            textures: names(c.textures),
            polymeter: c.polymeter,
            micro_loop: c.micro_loop,
            gaits: c.gaits.map(|g| List::from_slice(g[0], g)),
            kits: c.kits,
            progressions: c.progressions,
            drone: c.drone,
            ornament: c.ornament,
            harmonize: c.harmonize,
            chug: c.chug,
            groove: c.groove,
        }
    }
}

/// `blendMix`: the profiles of `mix` (index and weight, in the mix's
/// order) combined.
// One function, as blendMix is; the profiles' weights and their sharpened
// weights are parallel lists, indexed together.
#[must_use]
#[allow(clippy::too_many_lines, clippy::needless_range_loop)]
pub fn blend(mix: &[(usize, f64)]) -> Char {
    // normalise
    let mut total = 0.0;
    for &(_, w) in mix {
        if w > 0.0 {
            total += w;
        }
    }
    let mut w: List<(usize, f64), 8> = List::filled((0, 0.0));
    if total == 0.0 || total.is_nan() {
        w.push((data::resolve_key("dust"), 1.0));
    } else {
        for &(k, x) in mix {
            if x > 0.0 {
                if let Some(e) = w.iter_mut().find(|e| e.0 == k) {
                    e.1 += x / total;
                } else {
                    w.push((k, 0.0 + x / total));
                }
            }
        }
    }
    if w.len() == 1 {
        return Char::of(w[0].0);
    }
    let mut sharp = [0.0; 8];
    let mut sharp_total = 0.0;
    for (i, &(_, x)) in w.iter().enumerate() {
        sharp[i] = math::pow(x, 2.2);
        sharp_total += sharp[i];
    }
    for s in sharp.iter_mut().take(w.len()) {
        *s /= sharp_total;
    }
    let ch = |i: usize| &CHARACTERS[w[i].0];
    let range = |get: fn(&Character) -> Option<[f64; 2]>| {
        let (mut lo, mut hi) = (0.0, 0.0);
        for i in 0..w.len() {
            let v = get(ch(i)).unwrap_or([0.0, 0.0]);
            lo += v[0] * sharp[i];
            hi += v[1] * sharp[i];
        }
        Some([lo, hi])
    };
    let scalar = |get: fn(&Character) -> Option<f64>| {
        let mut v = 0.0;
        for i in 0..w.len() {
            v += get(ch(i)).unwrap_or(0.0) * sharp[i];
        }
        if v == 0.0 || v.is_nan() { None } else { Some(v) }
    };
    let pool = |get: fn(&Character) -> Option<Pool>| {
        let mut acc: Names = List::filled(("", 0.0));
        for i in 0..w.len() {
            for &(name, weight) in get(ch(i)).unwrap_or(&[]) {
                if let Some(e) = acc.iter_mut().find(|e| e.0 == name) {
                    e.1 += weight * w[i].1;
                } else {
                    acc.push((name, 0.0 + weight * w[i].1));
                }
            }
        }
        acc.retain(|e| e.1 > 0.001);
        Some(acc)
    };
    let numpool = |get: fn(&Character) -> Option<NumPool>| {
        let mut acc: Nums = List::filled((0.0, 0.0));
        for i in 0..w.len() {
            for &(name, weight) in get(ch(i)).unwrap_or(&[]) {
                if let Some(e) = acc.iter_mut().find(|e| e.0 == name) {
                    e.1 += weight * w[i].1;
                } else {
                    acc.push((name, 0.0 + weight * w[i].1));
                }
            }
        }
        acc.retain(|e| e.1 > 0.001);
        Some(acc)
    };
    let mut tone = [[0.0; 2]; 3];
    let mut feel = [[0.0; 2]; 3];
    for k in 0..3 {
        for i in 0..w.len() {
            tone[k][0] += ch(i).tone[k][0] * w[i].1;
            tone[k][1] += ch(i).tone[k][1] * w[i].1;
        }
        for i in 0..w.len() {
            feel[k][0] += ch(i).feel[k][0] * sharp[i];
            feel[k][1] += ch(i).feel[k][1] * sharp[i];
        }
    }
    // The lead: the first of the heaviest.
    let mut lead = 0;
    for i in 1..w.len() {
        if !(w[lead].1 >= w[i].1) {
            lead = i;
        }
    }
    let l = ch(lead);
    Char {
        lead: w[lead].0,
        single: false,
        tone,
        feel,
        bpm: range(|c| c.bpm),
        swing: range(|c| c.swing),
        hat_density: range(|c| c.hat_density),
        drums: scalar(|c| c.drums),
        rest_bar: scalar(|c| c.rest_bar),
        level: scalar(|c| c.level),
        flat_seven: scalar(|c| c.flat_seven),
        quartal: scalar(|c| c.quartal),
        pointillist: scalar(|c| c.pointillist),
        skip_step: scalar(|c| c.skip_step),
        glide: scalar(|c| c.glide),
        airy: scalar(|c| c.airy),
        pump: scalar(|c| c.pump),
        rolls: scalar(|c| c.rolls),
        four_floor: scalar(|c| c.four_floor),
        dub_echo: scalar(|c| c.dub_echo),
        scales: pool(|c| c.scales),
        steps_per_bar: numpool(|c| c.steps_per_bar),
        bars: numpool(|c| c.bars),
        chord_voices: pool(|c| c.chord_voices),
        melody_voices: pool(|c| c.melody_voices),
        bass_styles: pool(|c| c.bass_styles),
        bass_voices: pool(|c| c.bass_voices),
        chord_size: numpool(|c| c.chord_size),
        textures: pool(|c| c.textures),
        polymeter: Some(l.polymeter.unwrap_or(false)),
        micro_loop: Some(l.micro_loop.unwrap_or(false)),
        gaits: l.gaits.map(|g| List::from_slice(g[0], g)),
        kits: l.kits,
        progressions: l.progressions,
        drone: l.drone,
        ornament: l.ornament,
        harmonize: l.harmonize,
        chug: l.chug,
        groove: l.groove,
    }
}

fn lean_pool(p: &mut Option<Names>, v: &Vibes, get: fn(&Vibe) -> Option<Table>) {
    if let Some(p) = p {
        for e in p.iter_mut() {
            e.1 *= lean_in(Some(v), get, e.0);
        }
    }
}

/// `vibedCharacter`: the blend with every render-time lean applied.
#[must_use]
pub fn vibed(mut c: Char, v: Option<&Vibes>) -> Char {
    let Some(v) = v else { return c };
    let six8 = lean(Some(v), |r| r.six8);
    let long = lean(Some(v), |r| r.long);
    let big = lean(Some(v), |r| r.big_chords);
    let drums = lean(Some(v), |r| r.drums);
    let hats = lean(Some(v), |r| r.hats);
    let rests = lean(Some(v), |r| r.rests);
    lean_pool(&mut c.melody_voices, v, |r| r.melody);
    lean_pool(&mut c.chord_voices, v, |r| r.chords);
    lean_pool(&mut c.bass_voices, v, |r| r.bass);
    lean_pool(&mut c.bass_styles, v, |r| r.bass_style);
    lean_pool(&mut c.textures, v, |r| r.texture);
    if let Some(p) = &mut c.steps_per_bar {
        for e in p.iter_mut() {
            if e.0 == 12.0 {
                e.1 *= six8;
            }
        }
    }
    if let Some(g) = &mut c.gaits {
        for e in g.iter_mut() {
            if e.steps_per_bar == 12.0 {
                e.weight *= six8;
            }
        }
    }
    if let Some(p) = &mut c.bars {
        for e in p.iter_mut() {
            if e.0 >= 8.0 {
                e.1 *= long;
            }
        }
    }
    if let Some(p) = &mut c.chord_size {
        for e in p.iter_mut() {
            if e.0 >= 4.0 {
                e.1 *= big;
            }
        }
    }
    if let Some(d) = c.drums
        && drums != 1.0
    {
        let odds = (d / js::max(1e-6, 1.0 - d)) * drums;
        c.drums = Some(if d >= 1.0 { 1.0 } else { odds / (1.0 + odds) });
    }
    if let Some(h) = c.hat_density {
        c.hat_density = Some([h[0] * hats, h[1] * hats]);
    }
    if let Some(r) = c.rest_bar {
        c.rest_bar = Some(r * rests);
    }
    c
}

/// `characterOf`: the loop's profile from its mix and its vibes.
#[must_use]
pub fn character_of(mix: &[(usize, f64)], v: Option<&Vibes>) -> Char {
    vibed(blend(mix), v)
}
