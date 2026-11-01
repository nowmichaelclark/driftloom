//! A loop's spec, from its seed and its vibes: `newSpec`
//! (`js/generator.js`), draw for draw.

use super::canon::{Sink, Writer};
use super::character::{self, Char, Vibes, q8};
use super::data::{self, CHARACTERS, MOODS};
use super::js;
use super::list::{List, Name};
use super::melody::{self, Figure};
use super::request::{self, Request};
use super::rng::{Rng, seed_name};
use super::theory::{self, GLAD_FROM, SCALES};
use super::{LAYERS, Layer};
use crate::math;

/// How many profiles a mix holds at most (`howMany`'s largest).
pub const MIX_MAX: usize = 4;
/// How many moods a feel mix holds at most (`moodCount`'s largest).
pub const FEEL_MAX: usize = 3;

/// The three feeling dials.
#[derive(Clone, Copy, Debug, Default)]
pub struct Feel {
    pub lift: f64,
    pub energy: f64,
    pub warmth: f64,
}

/// A loop's spec: the JavaScript object `newSpec` returns.
#[derive(Clone, Copy)]
pub struct Spec {
    pub seed: u32,
    pub name: Name,
    /// The profiles and their weights, in the order they were drawn.
    pub mix: List<(usize, f64), MIX_MAX>,
    pub feel: Feel,
    /// The moods and their weights, in the order they were drawn.
    pub feel_mix: List<(usize, f64), FEEL_MAX>,
    pub coherence: f64,
    pub mood: f64,
    pub bpm: f64,
    pub root: f64,
    /// The mode, an index into `theory::SCALES`.
    pub scale: usize,
    pub bars: f64,
    pub steps_per_bar: f64,
    pub swing: f64,
    /// By layer, in `LAYERS`' order.
    pub layer_seeds: [u32; 5],
    /// warmth, space, wobble.
    pub tone: [f64; 3],
    pub vibes: Option<Vibes>,
    /// The tempo asked (queue item 85), [low, high] bpm: kept, so `metre`
    /// finds the gait by it and a share code carries it.
    pub tempo: Option<[f64; 2]>,
    /// The pins asked (queue item 85b), kept the same way: the choices'
    /// draw and the load read them from here.
    pub pins: request::Pins,
    /// The theme asked (queue item 85d): the seed whose loop this one bends,
    /// kept, so `metre` finds its gait again and a share code carries it.
    pub theme: Option<u32>,
    /// The theme's tune (`themeTune`, worked out once here, not written):
    /// `None` without a theme, `Some(None)` where the theme's melody rests,
    /// else its figure and rhythmic cell, which the melody plays in place of
    /// its own.
    pub tune: Option<Option<(Figure, &'static str)>>,
    /// Each layer's own cycle in steps, or `None` (null); `None` overall
    /// for a loop without cycles.
    pub cycles: Option<[Option<f64>; 5]>,
}

impl Spec {
    /// A spec of nothing, before any loop is loaded.
    pub const EMPTY: Spec = Spec {
        seed: 0,
        name: Name::new(),
        mix: List::filled((0, 0.0)),
        feel: Feel { lift: 0.0, energy: 0.0, warmth: 0.0 },
        feel_mix: List::filled((0, 0.0)),
        coherence: 0.0,
        mood: 0.0,
        bpm: 0.0,
        root: 0.0,
        scale: 0,
        bars: 0.0,
        steps_per_bar: 0.0,
        swing: 0.0,
        layer_seeds: [0; 5],
        tone: [0.0; 3],
        vibes: None,
        tempo: None,
        pins: request::Pins::NONE,
        theme: None,
        tune: None,
        cycles: None,
    };

    /// The layer's seed.
    #[must_use]
    pub fn layer_seed(&self, layer: Layer) -> u32 {
        self.layer_seeds[layer as usize]
    }

    /// The mode's steps.
    #[must_use]
    pub fn steps(&self) -> &'static [f64] {
        SCALES[self.scale].steps
    }

    /// `characterOf(spec)`.
    #[must_use]
    pub fn character(&self) -> Char {
        character::character_of(&self.mix, self.vibes.as_ref())
    }

    /// The canonical text of the spec (`tools/gen-golden.mjs`).
    pub fn write<S: Sink>(&self, w: &mut Writer<S>) {
        w.obj();
        w.key("bars");
        w.num(self.bars);
        w.key("bpm");
        w.num(self.bpm);
        w.key("coherence");
        w.num(self.coherence);
        if let Some(cycles) = &self.cycles {
            w.key("cycles");
            w.obj();
            for (key, layer) in [
                ("bass", Layer::Bass),
                ("chords", Layer::Chords),
                ("drums", Layer::Drums),
                ("melody", Layer::Melody),
                ("texture", Layer::Texture),
            ] {
                w.key(key);
                match cycles[layer as usize] {
                    Some(x) => w.num(x),
                    None => w.null(),
                }
            }
            w.end_obj();
        }
        w.key("feel");
        write_feel(w, &self.feel);
        w.key("feelMix");
        write_weights(w, &self.feel_mix, |k| MOODS[k].id);
        w.key("layerSeeds");
        w.obj();
        for (key, layer) in [
            ("bass", Layer::Bass),
            ("chords", Layer::Chords),
            ("drums", Layer::Drums),
            ("melody", Layer::Melody),
            ("texture", Layer::Texture),
        ] {
            w.key(key);
            w.num(f64::from(self.layer_seed(layer)));
        }
        w.end_obj();
        w.key("mix");
        write_weights(w, &self.mix, |k| CHARACTERS[k].id);
        w.key("mood");
        w.num(self.mood);
        w.key("mutes");
        w.obj();
        for key in ["bass", "chords", "drums", "melody", "texture"] {
            w.key(key);
            w.bool(false);
        }
        w.end_obj();
        w.key("name");
        w.str(self.name.as_str());
        if self.pins.any() {
            w.key("pins");
            self.pins.write(w);
        }
        w.key("playFor");
        w.null();
        w.key("root");
        w.num(self.root);
        w.key("scale");
        w.str(SCALES[self.scale].id);
        w.key("seed");
        w.num(f64::from(self.seed));
        w.key("stepsPerBar");
        w.num(self.steps_per_bar);
        w.key("swing");
        w.num(self.swing);
        if let Some(t) = &self.tempo {
            w.key("tempo");
            w.nums(t);
        }
        if let Some(t) = self.theme {
            w.key("theme");
            w.num(f64::from(t));
        }
        w.key("tone");
        w.obj();
        w.key("space");
        w.num(self.tone[1]);
        w.key("warmth");
        w.num(self.tone[0]);
        w.key("wobble");
        w.num(self.tone[2]);
        w.end_obj();
        if let Some(v) = &self.vibes {
            w.key("vibeStrength");
            w.num(v.strength);
            w.key("vibes");
            write_weights(w, &v.list, |k| data::VIBES[k].id);
        }
        w.end_obj();
    }
}

/// Named weights as an object, its keys sorted.
fn write_weights<S: Sink>(w: &mut Writer<S>, list: &[(usize, f64)], name: impl Fn(usize) -> &'static str) {
    let mut sorted = [(0, 0.0); 8];
    let n = list.len().min(sorted.len());
    sorted[..n].copy_from_slice(&list[..n]);
    sorted[..n].sort_unstable_by(|a, b| name(a.0).cmp(name(b.0)));
    w.obj();
    for &(k, x) in &sorted[..n] {
        w.key_any(name(k));
        w.num(x);
    }
    w.end_obj();
}

fn write_feel<S: Sink>(w: &mut Writer<S>, f: &Feel) {
    w.obj();
    w.key("energy");
    w.num(f.energy);
    w.key("lift");
    w.num(f.lift);
    w.key("warmth");
    w.num(f.warmth);
    w.end_obj();
}

const GAIT_SALT: u32 = 0x6a1c_0de5;

/// `gaitFor`: the profile's gait for this seed, if it has gaits.
#[must_use]
pub fn gait_for(seed: u32, c: &Char) -> Option<data::Gait> {
    let g = c.gaits.as_ref()?;
    let mut r = Rng::new(js::salted(seed, GAIT_SALT, 1));
    r.weighted_index(g.iter().map(|x| x.weight)).map(|i| g[i])
}

/// `gaitIn`: the gait for a tempo asked, `t`: the seed's own if its band
/// meets the tempo; else one of those that do, drawn by weight on the
/// gait's stream; else the nearest (the first on a tie), its metre then
/// played at the tempo asked. It reads only what the spec keeps, so
/// `Spec::metre` finds the same gait again.
#[must_use]
pub fn gait_in(seed: u32, c: &Char, t: Option<[f64; 2]>) -> Option<data::Gait> {
    let own = gait_for(seed, c)?;
    let Some(t) = t else { return Some(own) };
    if request::tempo_gap(own.bpm, t) == 0.0 {
        return Some(own);
    }
    let g = c.gaits.as_ref()?;
    let mut meet: List<(data::Gait, f64), 4> = List::filled((own, 0.0));
    for &x in g.iter() {
        if request::tempo_gap(x.bpm, t) == 0.0 {
            meet.push((x, x.weight));
        }
    }
    if !meet.is_empty() {
        return Rng::new(js::salted(seed, GAIT_SALT, 1)).weighted(&meet);
    }
    let mut best = g[0];
    for &x in g.iter() {
        if request::tempo_gap(x.bpm, t) < request::tempo_gap(best.bpm, t) {
            best = x;
        }
    }
    Some(best)
}

const POLY_CYCLES: [f64; 7] = [3.0, 4.0, 5.0, 7.0, 8.0, 9.0, 11.0];
const BORROW: f64 = 1.2;
const BORROW_SALT: u32 = 0x4252_4954;

/// `drawScale`.
fn draw_scale(r: &mut Rng, seed: u32, c: &Char, mix: &[(usize, f64)], lift: f64, v: Option<&Vibes>) -> usize {
    let lean = |k: &str, w: f64| theory::mood_weighted(w, theory::brightness(k), lift) * character::lean_in(v, |x| x.modes, k);
    let mut own: character::Names = List::filled(("", 0.0));
    for &(k, w) in c.scales.as_ref().map_or(&[][..], |p| &p[..]) {
        own.push((k, lean(k, w)));
    }
    let drawn = r.weighted(&own).unwrap_or("dorian");
    let mut chosen = drawn;
    // borrowedPool
    let glad = js::max(0.0, lift - GLAD_FROM) / (1.0 - GLAD_FROM);
    let mut extra: character::Names = List::filled(("", 0.0));
    if glad != 0.0 {
        let mut total = 0.0;
        for &(_, w) in mix {
            if w > 0.0 {
                total += w;
            }
        }
        if total != 0.0 {
            for &(k, w) in mix {
                let ch = &CHARACTERS[data::resolve_key(CHARACTERS[k].id)];
                let Some(bb) = ch.borrowed_bright else { continue };
                if !(w > 0.0) {
                    continue;
                }
                for &mode in bb {
                    let add = BORROW * glad * (w / total);
                    if let Some(e) = extra.iter_mut().find(|e| e.0 == mode) {
                        e.1 += add;
                    } else {
                        extra.push((mode, 0.0 + add));
                    }
                }
            }
        }
    }
    if !extra.is_empty() {
        for e in extra.iter_mut() {
            e.1 = lean(e.0, e.1);
        }
        let mut own_total = 0.0;
        for e in own.iter() {
            own_total += e.1;
        }
        let mut extra_total = 0.0;
        for e in extra.iter() {
            extra_total += e.1;
        }
        let mut b = Rng::new(js::salted(seed, BORROW_SALT, 23));
        if b.chance(extra_total / (own_total + extra_total)) {
            chosen = b.weighted(&extra).unwrap_or(drawn);
        }
    }
    SCALES.iter().position(|s| s.id == chosen).unwrap_or(0)
}

/// `newSpec(seed, request)`: the loop of `seed`, as `request` asks.
#[must_use]
#[allow(clippy::too_many_lines)] // one function, as newSpec is: its draws in order
pub fn new_spec(seed: u32, request: &Request) -> Spec {
    let mut r = Rng::new(seed);
    let vibes = character::draw_vibes(seed, request);
    let v = vibes.as_ref();
    let t = request.tempo();
    // A theme asked (queue item 85d): its own loop, nothing asked, gives what
    // it holds (the mix, the key and mode, the metre, the tune). Every draw
    // below is still made, so nothing else moves.
    let held = request.theme().map(|th| new_spec(th, &Request::new()));

    let drive = math::pow(r.f(), 0.82) * 1.8 - 0.5;
    let mut pool: List<(usize, f64), { data::CHARACTER_COUNT }> = List::filled((0, 0.0));
    for (i, c) in CHARACTERS.iter().enumerate() {
        let fe = c.feel[1];
        let mid = (fe[0] + fe[1]) / 2.0;
        let wt = c.weight.unwrap_or(f64::NAN);
        let band = c.bpm.unwrap_or([f64::NAN, f64::NAN]);
        pool.push((
            i,
            wt * js::max(0.28, 1.0 + drive * (mid - 0.45) * 2.5) * character::lean_in(v, |x| x.profiles, c.id) * request::tempo_lean(t, band),
        ));
    }
    let how_many = r.weighted(&[(1, 2.2), (2, 4.0), (3, 2.6), (4, 1.2)]).unwrap_or(1);
    let mut mix: List<(usize, f64), MIX_MAX> = List::filled((0, 0.0));
    for i in 0..how_many {
        if pool.is_empty() {
            break;
        }
        let Some(key) = r.weighted(&pool) else { break };
        if let Some(idx) = pool.iter().position(|e| e.0 == key) {
            pool.remove(idx);
        }
        let x = -math::log(1.0 - r.f() * 0.999) * if i == 0 { 1.6 } else { 1.0 };
        mix.push((key, x));
    }
    let mut total = 0.0;
    for e in mix.iter() {
        total += e.1;
    }
    for e in mix.iter_mut() {
        e.1 = q8(e.1 / total);
    }
    if let Some(h) = &held {
        mix = h.mix;
    }

    let c = character::character_of(&mix, v);

    let fit = |m: &data::Mood| {
        let mut d = 0.0;
        for (axis, x) in [m.lift, m.energy, m.warmth].into_iter().enumerate() {
            let [lo, hi] = c.feel[axis];
            if x < lo {
                d += lo - x;
            } else if x > hi {
                d += x - hi;
            }
        }
        1.0 / (1.0 + d * 3.5)
    };
    let mut moods: List<(usize, f64), { data::MOOD_COUNT }> = List::filled((0, 0.0));
    for (i, m) in MOODS.iter().enumerate() {
        moods.push((i, fit(m) * js::max(0.08, 1.0 + drive * (m.energy - 0.47) * 2.4)));
    }
    let mood_count = r.weighted(&[(1, 2.0), (2, 4.0), (3, 2.4)]).unwrap_or(1);
    let mut feel_mix: List<(usize, f64), FEEL_MAX> = List::filled((0, 0.0));
    for i in 0..mood_count {
        if moods.is_empty() {
            break;
        }
        let Some(key) = r.weighted(&moods) else { break };
        if let Some(idx) = moods.iter().position(|e| e.0 == key) {
            moods.remove(idx);
        }
        feel_mix.push((key, -math::log(1.0 - r.f() * 0.999) * if i == 0 { 1.7 } else { 1.0 }));
    }
    let mut mood_total = 0.0;
    for e in feel_mix.iter() {
        mood_total += e.1;
    }
    for e in feel_mix.iter_mut() {
        e.1 /= mood_total;
    }
    feel_mix.retain(|e| !(e.1 < 0.005));
    for e in feel_mix.iter_mut() {
        e.1 = q8(e.1);
    }

    let mut feel = [0.0; 3];
    for &(k, w) in feel_mix.iter() {
        let m = &MOODS[k];
        for (axis, x) in [m.lift, m.energy, m.warmth].into_iter().enumerate() {
            feel[axis] += x * w;
        }
    }
    for (axis, name) in ["lift", "energy", "warmth"].into_iter().enumerate() {
        let [lo, hi] = c.feel[axis];
        let pulled = character::pull(v, |rec| rec.feel.and_then(|t| data::lookup(t, name)), feel[axis]);
        feel[axis] = q8(js::max(lo, js::min(hi, pulled)));
    }
    let feel = Feel { lift: feel[0], energy: feel[1], warmth: feel[2] };
    let coherence = q8(r.range(0.42, 0.82));

    let mut steps_per_bar = c.steps_per_bar.as_ref().and_then(|p| r.weighted(p)).unwrap_or(f64::NAN);
    let bars = c.bars.as_ref().and_then(|p| r.weighted(p)).unwrap_or(f64::NAN);
    // A theme keeps its own gait and metre; a tempo asked then plays in it.
    let gait = match (request.theme(), &held) {
        (Some(th), Some(_)) => theme_gait(th, &mix),
        _ => gait_in(seed, &c, t),
    };
    if let Some(g) = gait {
        steps_per_bar = g.steps_per_bar;
    }
    if let Some(h) = &held {
        steps_per_bar = h.steps_per_bar;
    }
    let tempo_draw = character::pull(v, |rec| rec.tempo, r.f());
    let band = match gait {
        Some(g) => g.bpm,
        None => c.bpm.unwrap_or([f64::NAN, f64::NAN]),
    };
    let tempo = js::round(band[0] + tempo_draw * (band[1] - band[0]) + (feel.lift - 0.5) * 5.0 + (feel.energy - 0.5) * 10.0);
    // A tempo asked is honored, drawn across the whole of it by the same
    // draw (`newSpec` says why).
    let bpm = match (t, gait) {
        (Some(t), _) => js::min(t[1], js::floor(t[0] + tempo_draw * (t[1] - t[0] + 1.0))),
        (None, Some(g)) => js::max(g.bpm[0], js::min(g.bpm[1], tempo)),
        (None, None) => js::max(32.0, js::min(190.0, tempo)),
    };
    let root = r.int(0.0, 11.0);
    let scale = draw_scale(&mut r, seed, &c, &mix, feel.lift, v);
    // The theme's key and mode, over the draws.
    let (root, scale) = held.as_ref().map_or((root, scale), |h| (h.root, h.scale));
    let swing_band = c.swing.unwrap_or([f64::NAN, f64::NAN]);
    let swing_draw = character::pull(v, |rec| rec.swing, r.f());
    let swing = swing_band[0] + swing_draw * (swing_band[1] - swing_band[0]);
    let mut layer_seeds = [0; 5];
    for s in &mut layer_seeds {
        *s = r.seed32();
    }
    let mut tone = [
        r.range(c.tone[0][0], c.tone[0][1]) * (0.6 + feel.warmth * 0.6),
        r.range(c.tone[1][0], c.tone[1][1]),
        r.range(c.tone[2][0], c.tone[2][1]),
    ];
    if v.is_some() {
        for (k, name) in ["warmth", "space", "wobble"].into_iter().enumerate() {
            let set = character::pull(v, |rec| rec.tone.and_then(|t| data::lookup(t, name)), tone[k]);
            if set != tone[k] {
                let [lo, hi] = c.tone[k];
                tone[k] = js::max(lo, js::min(hi, set));
            }
        }
    }

    let mut cycles = None;
    if c.polymeter == Some(true) {
        let mut cy = POLY_CYCLES;
        r.shuffle(&mut cy);
        let mut out = [None; 5];
        for (i, layer) in [Layer::Bass, Layer::Chords, Layer::Melody, Layer::Texture].into_iter().enumerate() {
            out[layer as usize] = Some(cy[i] * steps_per_bar);
        }
        cycles = Some(out);
    } else if c.micro_loop == Some(true) && bars >= 8.0 {
        let short = r.pick(&[1.0, 2.0, 2.0, 4.0]).unwrap_or(1.0);
        let mut out = [None; 5];
        out[Layer::Bass as usize] = Some(short * steps_per_bar);
        out[Layer::Chords as usize] = Some(r.pick(&[2.0, 4.0]).unwrap_or(2.0) * steps_per_bar);
        out[Layer::Melody as usize] = Some(r.pick(&[1.0, 2.0, 3.0]).unwrap_or(1.0) * steps_per_bar);
        cycles = Some(out);
    }
    let _ = LAYERS;
    Spec {
        seed,
        name: seed_name(seed),
        mix,
        feel,
        feel_mix,
        coherence,
        mood: feel.lift,
        bpm,
        root,
        scale,
        bars,
        steps_per_bar,
        swing,
        layer_seeds,
        tone,
        vibes,
        tempo: t,
        pins: *request.pins(),
        theme: request.theme(),
        tune: held.as_ref().map(melody::theme_tune),
        cycles,
    }
}

/// `themeGait(theme, mix)`: a theme's gait (queue item 85d), the one its own
/// loop plays, from its seed and the mix it holds, whatever the vibes lean.
fn theme_gait(theme: u32, mix: &[(usize, f64)]) -> Option<data::Gait> {
    gait_for(theme, &character::character_of(mix, None))
}

/// How a bar is counted (`metreOf`): twelve steps are 6/8, or 3/4 when the
/// loop's gait reads them as a waltz.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Metre {
    FourFour,
    SixEight,
    ThreeFour,
    FiveFour,
}

/// A drawn choir (`choirOf`): the voice both layers sing, and the standing
/// detune between them.
#[derive(Clone, Copy, Debug)]
pub struct Choir {
    pub voice: &'static str,
    pub keys_detune: f64,
    pub melody_detune: f64,
}

const CHOIR_SALT: u32 = 0x7c9e_6b2d;
const CHOIR_RATE: f64 = 0.03;

const LAYER_SALT: [u32; 5] = [0x1f3b, 0x2c5d, 0x3a71, 0x4d93, 0x5e17];

impl Spec {
    /// `metreOf(spec)`, given the loop's character.
    #[must_use]
    pub fn metre(&self, c: &Char) -> Metre {
        if self.steps_per_bar == 12.0 {
            let gait = match self.theme {
                Some(th) => theme_gait(th, &self.mix),
                None => gait_in(self.seed, c, self.tempo),
            };
            match gait {
                Some(g) if g.metre == "3/4" => Metre::ThreeFour,
                _ => Metre::SixEight,
            }
        } else if self.steps_per_bar == 20.0 {
            Metre::FiveFour
        } else {
            Metre::FourFour
        }
    }

    /// `choirOf(spec)`.
    #[must_use]
    pub fn choir(&self) -> Option<Choir> {
        let mut r = Rng::new(js::salted(self.seed, CHOIR_SALT, 17));
        if !r.chance(CHOIR_RATE) {
            return None;
        }
        let voice = r.weighted(&[("vowel", 3.0), ("hum", 2.0)]).unwrap_or("vowel");
        let cents = 4.0 + r.f() * 7.0;
        Some(Choir { voice, keys_detune: -cents, melody_detune: cents })
    }

    /// `feelForLayer(spec, layer)`: the layer's own feel, strayed from the
    /// loop's toward one of its moods.
    #[must_use]
    pub fn feel_for(&self, layer: Layer) -> Feel {
        let base = self.feel;
        if self.feel_mix.is_empty() {
            return base;
        }
        let mut r = Rng::new(js::salted(self.seed, LAYER_SALT[layer as usize], 3));
        let Some(key) = r.weighted(&self.feel_mix) else { return base };
        let m = &MOODS[key];
        let stray = 1.0 - self.coherence;
        Feel {
            lift: character::clamp01(base.lift + (m.lift - base.lift) * stray),
            energy: character::clamp01(base.energy + (m.energy - base.energy) * stray),
            warmth: character::clamp01(base.warmth + (m.warmth - base.warmth) * stray),
        }
    }

    /// Whether a vibe of the loop's carries more than `share` of it
    /// (`spec.vibes && spec.vibes[id] > share`).
    #[must_use]
    pub fn vibe_over(&self, id: &str, share: f64) -> bool {
        self.vibes.as_ref().is_some_and(|v| v.list.iter().any(|&(i, w)| data::VIBES[i].id == id && w > share))
    }
}
