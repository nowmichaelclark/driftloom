//! What the engine decides at load (queue item 81): `Engine.load` in
//! `js/engine.js`, for a loop just rendered. The per-loop settings the host
//! sends the core and the sequencer plays by: glass's level in a sparse
//! loop, dubStab's density and stab, the chapel organ's registration, the
//! wash, the arc, the tuned kick, subDuck, padSweep, the bass's level,
//! lateFloor's wait, the loop's space, the echo's time and the pump; and
//! the calls the engine makes to the synth there, in order (the tone, the
//! profile's level, the echo time, the mutes). The goldens hash it as
//! `load`.
//!
//! Each setting is drawn from the loop's seed on a stream of its own, or
//! worked out from what the loop plays; nothing at load draws from chance
//! (the goldens' tool makes `Math.random` throw there). The tables of the
//! sketches the engine draws dubStab, the organ and the wash from are the
//! engine's own code, not data, and are ported here as they stand.

use super::LAYERS;
use super::canon::{Sink, Writer};
use super::choices::{self, Arc, Fall, LateFloor, PadSweep, SubDuck, TunedKick};
use super::js;
use super::plan::Cx;
use super::render::Render;
use super::rng::Rng;
use crate::math;

/// dubStab (queue item 60; `stabOf`): the loop's density, the chance of a
/// stab on the offbeat after beat one and after beat three, and the stab's
/// own settings.
#[derive(Clone, Copy, Debug)]
pub struct Stab {
    pub kind: &'static str,
    pub rates: [f64; 2],
    pub decay: f64,
    pub level: f64,
    pub open: f64,
    pub saw: f64,
    pub throw: f64,
    pub tone: f64,
    pub verb: f64,
}

/// The wash (queue item 62; `washOf`): one swell's period in seconds, the
/// follower's gain, and its tint (Hz, its gain and the high one in dB, its
/// resonance).
#[derive(Clone, Copy, Debug)]
pub struct Wash {
    pub level: f64,
    pub period: f64,
    pub tint: f64,
    pub tint_gain: f64,
    pub tint_high: f64,
    pub tint_q: f64,
}

/// The loop's space (queue item 65; `spaceOf`): what the core needs to
/// place it in a stereo render. Its seed (the core draws every width from
/// it), how busy it is (notes a second, every layer, as heard), how wet,
/// and whether its texture is a light one, whose whole layer is placed.
#[derive(Clone, Copy, Debug)]
pub struct Space {
    pub seed: u32,
    pub busy: f64,
    pub wet: f64,
    pub light: bool,
}

/// The loop's tuned kick (queue item 67) and which of its kinds it plays
/// ("short" or "long").
#[derive(Clone, Copy, Debug)]
pub struct Kick {
    pub kick: TunedKick,
    pub kind: &'static str,
}

/// How many settings the organ sends: its nine drawbars, then `ORGAN_KEYS`.
pub const ORGAN_SETTINGS: usize = 21;

/// Everything the engine decides at load, each `None` where the loop's
/// choices leave it out.
#[derive(Clone, Copy, Debug)]
pub struct Load {
    /// glass's level in a sparse loop: under one chord a bar, scaled down
    /// by as much.
    pub glass_scale: f64,
    pub stab: Option<Stab>,
    /// The organ's settings, in the core's order.
    pub organ: Option<[f64; ORGAN_SETTINGS]>,
    pub wash: Option<Wash>,
    pub arc: Option<Arc>,
    pub kick: Option<Kick>,
    pub sub_duck: Option<SubDuck>,
    pub pad_sweep: Option<PadSweep>,
    /// The bass layer's level, in dB.
    pub bass_lift: f64,
    pub late: Option<LateFloor>,
    pub space: Space,
    /// The echo's time, in beats.
    pub echo_beats: f64,
    /// The profile's pump, and its level (`setCharacterLevel`).
    pub pump: f64,
    pub level: f64,
    /// The loop's tempo and tone, for the calls.
    bpm: f64,
    tone: [f64; 3],
}

impl Load {
    /// Nothing loaded.
    pub const EMPTY: Load = Load {
        glass_scale: 0.0,
        stab: None,
        organ: None,
        wash: None,
        arc: None,
        kick: None,
        sub_duck: None,
        pad_sweep: None,
        bass_lift: 0.0,
        late: None,
        space: Space { seed: 0, busy: 0.0, wet: 0.0, light: false },
        echo_beats: 0.75,
        pump: 0.0,
        level: 1.0,
        bpm: 0.0,
        tone: [0.0; 3],
    };

    /// What the engine decides loading the loop of `cx`, rendered as `r`
    /// (a load that starts the loop: not a re-roll, which keeps its place,
    /// nor a loop resumed under a rebuilt engine).
    #[must_use]
    pub fn of(cx: &Cx, r: &Render) -> Self {
        let mut load = Self::without_kick(cx, r);
        load.kick = Self::kick_kind(r).map(|kind| Kick { kick: choices::kick_of(cx.spec.seed, cx.spec.root), kind });
        load
    }

    /// The tuned kick's kind a loop rendered as `r` plays ("short" or
    /// "long"), or none.
    #[must_use]
    pub fn kick_kind(r: &Render) -> Option<&'static str> {
        r.choice("tunedKick").filter(|&kind| kind != "today")
    }

    /// `of`, all but the tuned kick's fit (`kick` none): a load spread across
    /// blocks works the fit out a slice at a time (`choices::KickFit`).
    #[must_use]
    pub fn without_kick(cx: &Cx, r: &Render) -> Self {
        let spec = cx.spec;
        let v = spec.vibes.as_ref();
        let on = |id: &str| r.choice(id) == Some("on");
        let spb = cx.spb;
        let total = spec.bars * spb;
        let busy = r.notes_per_second(cx);
        let chords = r.chords.iter().filter(|e| e.vel != 0.0).count();
        let kick = None;
        let space = spec.tone[1];
        Load {
            glass_scale: js::min(1.0, js::num(chords) / js::max(1.0, total / spb)),
            stab: on("dubStab").then(|| stab_of(spec.seed, busy)),
            organ: on("organ").then(|| organ_of(spec.seed, busy)),
            wash: on("wash").then(|| wash_of(spec.seed, spec.root)),
            arc: choices::arc_plays(spec.seed, on("arc"), v).then(|| choices::arc_of(spec.seed, v)),
            kick,
            sub_duck: on("subDuck").then(|| choices::sub_duck_of(spec.seed, spec.bpm)),
            pad_sweep: on("padSweep").then(|| choices::pad_sweep_of(spec.seed)),
            bass_lift: choices::bass_lift_of(&r.drums),
            late: on("lateFloor").then(|| choices::late_floor_of(spec.seed, spb, 60.0 / spec.bpm / 4.0, v, js::round(total / spb))),
            space: Space {
                seed: spec.seed,
                busy,
                // `spec.tone.space || 0`
                wet: if space == 0.0 || space.is_nan() { 0.0 } else { space },
                light: matches!(r.texture_kind, "bells" | "chime" | "drops"),
            },
            echo_beats: choices::echo_time_of(spec.seed, v),
            pump: cx.c.pump.unwrap_or(0.0),
            level: cx.c.level.unwrap_or(1.0),
            bpm: spec.bpm,
            tone: spec.tone,
        }
    }

    /// The echo's time in seconds, as the engine sends it.
    #[must_use]
    pub fn echo_seconds(&self) -> f64 {
        (60.0 / self.bpm) * self.echo_beats
    }

    /// The canonical text (`load` in `tools/gen-golden.mjs`): every
    /// setting, null where the loop's choices leave it out, and the calls.
    pub fn write<S: Sink>(&self, w: &mut Writer<S>) {
        w.obj();
        w.key("arc");
        or_null(w, self.arc.as_ref(), write_arc);
        w.key("bassLift");
        w.num(self.bass_lift);
        w.key("calls");
        self.write_calls(w);
        w.key("echoBeats");
        w.num(self.echo_beats);
        w.key("glassScale");
        w.num(self.glass_scale);
        w.key("kick");
        or_null(w, self.kick.as_ref(), write_kick);
        w.key("late");
        or_null(w, self.late.as_ref(), write_late);
        w.key("organ");
        or_null(w, self.organ.as_ref(), |w, o| w.nums(o));
        w.key("padSweep");
        or_null(w, self.pad_sweep.as_ref(), write_pad_sweep);
        w.key("pumpAmount");
        w.num(self.pump);
        w.key("space");
        write_space(w, &self.space);
        w.key("stab");
        or_null(w, self.stab.as_ref(), write_stab);
        w.key("subDuck");
        or_null(w, self.sub_duck.as_ref(), write_sub_duck);
        w.key("wash");
        or_null(w, self.wash.as_ref(), write_wash);
        w.end_obj();
    }

    /// The calls the engine makes to the synth at load, in order: the
    /// loop's tone, the profile's level, the echo's time, and each layer's
    /// mute (none muted: a spec from a seed mutes nothing).
    fn write_calls<S: Sink>(&self, w: &mut Writer<S>) {
        w.list();
        w.list();
        w.str("setTone");
        w.obj();
        w.key("space");
        w.num(self.tone[1]);
        w.key("warmth");
        w.num(self.tone[0]);
        w.key("wobble");
        w.num(self.tone[2]);
        w.end_obj();
        w.end_list();
        w.list();
        w.str("setCharacterLevel");
        w.num(self.level);
        w.end_list();
        w.list();
        w.str("setEchoTime");
        w.num(self.echo_seconds());
        w.end_list();
        for layer in LAYERS {
            w.list();
            w.str("setMute");
            w.str(layer.name());
            w.bool(false);
            w.end_list();
        }
        w.end_list();
    }
}

impl Default for Load {
    fn default() -> Self {
        Self::EMPTY
    }
}

/// A setting's text, or null for a loop without it.
fn or_null<S: Sink, T>(w: &mut Writer<S>, x: Option<&T>, write: impl FnOnce(&mut Writer<S>, &T)) {
    match x {
        Some(x) => write(w, x),
        None => w.null(),
    }
}

fn write_arc<S: Sink>(w: &mut Writer<S>, a: &Arc) {
    w.obj();
    w.key("dip");
    w.num(a.dip);
    w.key("floor");
    w.num(a.floor);
    w.key("from");
    w.num(a.from);
    w.key("period");
    w.num(a.period);
    w.key("shape");
    w.str(a.shape);
    w.end_obj();
}

fn write_kick<S: Sink>(w: &mut Writer<S>, k: &Kick) {
    w.obj();
    w.key("kick");
    w.obj();
    w.key("hz");
    w.num(k.kick.hz);
    w.key("long");
    write_fall(w, &k.kick.long);
    w.key("midi");
    w.num(k.kick.midi);
    w.key("short");
    write_fall(w, &k.kick.short);
    w.end_obj();
    w.key("kind");
    w.str(k.kind);
    w.end_obj();
}

fn write_late<S: Sink>(w: &mut Writer<S>, l: &LateFloor) {
    w.obj();
    w.key("bars");
    w.num(l.bars);
    w.key("bass");
    w.num(l.bass);
    w.key("drums");
    w.num(l.drums);
    w.key("gap");
    w.num(l.gap);
    w.key("order");
    w.str(l.order);
    w.end_obj();
}

fn write_pad_sweep<S: Sink>(w: &mut Writer<S>, p: &PadSweep) {
    w.obj();
    w.key("lo");
    w.num(p.lo);
    w.key("oct");
    w.num(p.oct);
    w.key("period");
    w.num(p.period);
    w.key("q");
    w.num(p.q);
    w.end_obj();
}

fn write_space<S: Sink>(w: &mut Writer<S>, s: &Space) {
    w.obj();
    w.key("busy");
    w.num(s.busy);
    w.key("light");
    w.bool(s.light);
    w.key("seed");
    w.num(f64::from(s.seed));
    w.key("wet");
    w.num(s.wet);
    w.end_obj();
}

fn write_stab<S: Sink>(w: &mut Writer<S>, s: &Stab) {
    w.obj();
    w.key("kind");
    w.str(s.kind);
    w.key("opts");
    w.obj();
    w.key("decay");
    w.num(s.decay);
    w.key("level");
    w.num(s.level);
    w.key("open");
    w.num(s.open);
    w.key("saw");
    w.num(s.saw);
    w.key("throw");
    w.num(s.throw);
    w.key("tone");
    w.num(s.tone);
    w.key("verb");
    w.num(s.verb);
    w.end_obj();
    w.key("rates");
    w.nums(&s.rates);
    w.end_obj();
}

fn write_sub_duck<S: Sink>(w: &mut Writer<S>, d: &SubDuck) {
    w.obj();
    w.key("depth");
    w.num(d.depth);
    w.key("recover");
    w.num(d.recover);
    w.end_obj();
}

fn write_wash<S: Sink>(w: &mut Writer<S>, x: &Wash) {
    w.obj();
    w.key("level");
    w.num(x.level);
    w.key("period");
    w.num(x.period);
    w.key("tint");
    w.num(x.tint);
    w.key("tintGain");
    w.num(x.tint_gain);
    w.key("tintHigh");
    w.num(x.tint_high);
    w.key("tintQ");
    w.num(x.tint_q);
    w.end_obj();
}

/// A kick's fall's text.
fn write_fall<S: Sink>(w: &mut Writer<S>, f: &Fall) {
    w.obj();
    w.key("dbps");
    w.num(f.dbps);
    w.key("lift");
    w.num(f.lift);
    w.key("softLift");
    w.num(f.soft_lift);
    w.key("softTail");
    w.num(f.soft_tail);
    w.key("tail");
    w.num(f.tail);
    w.end_obj();
}

// ------------------------------------------------------------ dubStab

const STAB_SALT: u32 = 0x57ac;

/// `stabOf`: a loop's dubStab, drawn as the sketch drew it: its density,
/// then the options in the sketch's order (round 2's, then round 3's over
/// them, then round 4's), so a loop draws what Mikey heard; its level down
/// by up to 8 dB in a loop of under four notes a second (`busy`).
fn stab_of(seed: u32, busy: f64) -> Stab {
    let mut r = Rng::new(js::salted(seed, STAB_SALT, 1));
    let kind = r.weighted(&[("rare", 34.0), ("sparse", 36.0), ("some", 20.0), ("busy", 14.0)]).unwrap_or("rare");
    // Round 2's and 3's draws, which round 4's replace.
    r.range(0.18, 0.3);
    r.range(2400.0, 3400.0);
    let saw = r.range(0.6, 0.9);
    r.range(0.45, 0.7);
    r.range(1200.0, 1800.0);
    r.range(0.32, 0.45);
    let throw = r.range(0.3, 0.42);
    let tone = r.range(1000.0, 1300.0);
    let open = r.range(2000.0, 2800.0);
    let decay = r.range(0.24, 0.34);
    // `STAB_RATES`.
    let rates = match kind {
        "rare" => [0.22, 0.0],
        "sparse" => [0.45, 0.05],
        "some" => [0.6, 0.2],
        _ => [0.65, 0.5],
    };
    let level = 0.19 * js::min(1.0, js::max(0.4, (busy + 1.0) / 5.0));
    Stab { kind, rates, decay, level, open, saw, throw, tone, verb: 0.12 }
}

// ------------------------------------------------------------ the organ

const ORGAN_SALT: u32 = 0x0e9a;
const ORGAN_SALT_5: u32 = 0x0e9b;

/// The nine registrations a loop draws from (`ORGAN_REG`): the drawbars
/// (16' 5 1/3' 8' 4' 2 2/3' 2' 1 3/5' 1 1/3' 1', 0-8) and the weight.
const ORGAN_REG: [(&str, [f64; 9], f64); 9] = [
    ("flute", [0.0, 0.0, 8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 14.0),
    ("stopped", [8.0, 0.0, 8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 14.0),
    ("mellow", [8.0, 3.0, 8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 14.0),
    ("choral", [6.0, 8.0, 8.0, 6.0, 0.0, 0.0, 0.0, 0.0, 0.0], 12.0),
    ("full", [8.0, 8.0, 8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 12.0),
    ("hollow", [0.0, 0.0, 8.0, 0.0, 6.0, 0.0, 4.0, 0.0, 0.0], 9.0),
    ("airy", [0.0, 0.0, 8.0, 4.0, 0.0, 0.0, 0.0, 0.0, 3.0], 9.0),
    ("round", [6.0, 0.0, 8.0, 5.0, 0.0, 3.0, 0.0, 0.0, 0.0], 10.0),
    ("bright", [8.0, 6.0, 8.0, 6.0, 4.0, 4.0, 0.0, 0.0, 2.0], 6.0),
];
/// The soft registration each loud one falls back to (`ORGAN_SOFT`).
const ORGAN_SOFT: [(&str, &str); 3] = [("choral", "mellow"), ("full", "stopped"), ("bright", "round")];

/// One organ of the mix: its drawbars and settings.
#[derive(Clone, Copy)]
struct Pipes {
    bars: [f64; 9],
    speak: f64,
    chiff: f64,
    verb: f64,
    dry: f64,
    release: f64,
    tone: f64,
    celeste: f64,
    celeste_level: f64,
    low: f64,
    carve: f64,
    level_mul: f64,
}

/// The variations (`ORGAN_VARIANTS`), with the defaults they take (`dry`
/// 1, `low` 75, `carve` -5).
const CHAPEL: Pipes = Pipes {
    bars: [0.0, 0.0, 8.0, 6.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    speak: 0.12,
    chiff: 0.6,
    verb: 0.45,
    dry: 1.0,
    release: 0.4,
    tone: 2400.0,
    celeste: 3.0,
    celeste_level: 0.25,
    low: 75.0,
    carve: -5.0,
    level_mul: 1.19,
};
const DISTANT: Pipes = Pipes {
    bars: [6.0, 0.0, 8.0, 6.0, 0.0, 5.0, 0.0, 3.0, 3.0],
    speak: 0.25,
    chiff: 0.0,
    verb: 1.3,
    dry: 0.4,
    release: 0.8,
    tone: 3200.0,
    celeste: 2.0,
    celeste_level: 0.2,
    low: 50.0,
    carve: -5.0,
    level_mul: 1.6,
};
const CELESTE: Pipes = Pipes {
    bars: [0.0, 0.0, 8.0, 6.0, 4.0, 4.0, 3.0, 2.0, 2.0],
    speak: 0.3,
    chiff: 0.0,
    verb: 0.6,
    dry: 1.0,
    release: 0.8,
    tone: 2200.0,
    celeste: 7.0,
    celeste_level: 0.9,
    low: 75.0,
    carve: -5.0,
    level_mul: 0.9,
};

/// `organOf`: a loop's organ, drawn as the sketch drew it (its rounds'
/// draws in their order, so a loop gets the organ Mikey heard): the
/// registration and tone of round 5's pipe from the loop's own draws, mixed
/// 55% chapel, 25% round 5's pipe, 10% the distant cathedral and 10%
/// celeste (`ORGAN_MIX`, in that order), stops as pipe loudness and the
/// rest as weighted means; its level down by up to 11 dB in a loop of under
/// three notes a second (`busy`). The settings, in the core's order.
fn organ_of(seed: u32, busy: f64) -> [f64; ORGAN_SETTINGS] {
    let mut r = Rng::new(js::salted(seed, ORGAN_SALT, 1));
    let (name, drawn, _) = ORGAN_REG[r.weighted_index(ORGAN_REG.iter().map(|x| x.2)).unwrap_or(0)];
    let rate = r.range(0.55, 1.15);
    r.range(0.6, 1.0);
    r.range(0.08, 0.5);
    let tone = 1400.0 + (r.range(2400.0, 4200.0) - 2400.0) * 0.6;
    let soft = ORGAN_SOFT.iter().find(|x| x.0 == name).map_or(name, |x| x.1);
    let bars = ORGAN_REG.iter().find(|x| x.0 == soft).map_or(drawn, |x| x.1);
    let mut r5 = Rng::new(js::salted(seed, ORGAN_SALT_5, 1));
    r5.chance(0.5);
    r5.chance(0.6);
    r5.range(0.08, 0.16);
    let speak = r5.range(0.06, 0.12);
    let chiff = r5.range(0.8, 1.4);
    let celeste = r5.range(3.0, 6.0);
    let pipe5 = Pipes {
        bars,
        speak,
        chiff,
        verb: 0.35,
        dry: 1.0,
        release: 0.35,
        tone,
        celeste,
        celeste_level: 0.45,
        low: 75.0,
        carve: 0.0,
        level_mul: 1.0,
    };
    let mix = [(CHAPEL, 0.55), (pipe5, 0.25), (DISTANT, 0.1), (CELESTE, 0.1)];
    let total = mix.iter().fold(0.0, |a, x| a + x.1);
    let mean = |get: fn(&Pipes) -> f64| mix.iter().fold(0.0, |a, (p, w)| a + w * get(p)) / total;
    let amp = |b: f64| if b == 0.0 { 0.0 } else { math::pow(10.0, (b - 8.0) * 3.0 / 20.0) };
    let mut out = [0.0; ORGAN_SETTINGS];
    for (i, o) in out.iter_mut().take(9).enumerate() {
        let a = mix.iter().fold(0.0, |x, (p, w)| x + w * amp(p.bars[i])) / total;
        *o = if a > 0.01 { 8.0 + 20.0 * math::log10(a) / 3.0 } else { 0.0 };
    }
    // `ORGAN_KEYS`: speak, chiff, celeste, celesteLevel, release, tone,
    // low, carve, shimmer, level, verb, dry.
    out[9..].copy_from_slice(&[
        mean(|p| p.speak),
        mean(|p| p.chiff),
        mean(|p| p.celeste),
        mean(|p| p.celeste_level),
        mean(|p| p.release),
        mean(|p| p.tone),
        mean(|p| p.low),
        mean(|p| p.carve),
        0.3 + rate * 0.25,
        0.085 * mean(|p| p.level_mul) * js::min(1.0, js::max(0.28, (busy + 0.5) / 3.5)),
        mean(|p| p.verb),
        mean(|p| p.dry),
    ]);
    out
}

// ------------------------------------------------------------- the wash

const WASH_SALT: u32 = 0x3a54;
const WASH_SALT_5: u32 = 0x3a55;
/// The sketch's calibration: its level against the loop (dB) into the
/// follower's gain.
const WASH_K: f64 = 25.0;

/// `washOf`: a loop's wash, drawn as the sketch drew it: its level against
/// the loop, then its period; and on a stream of its own its tint, on the
/// key's root (60%) or its fifth, folded into 600-1200 Hz.
fn wash_of(seed: u32, root: f64) -> Wash {
    let mut r = Rng::new(js::salted(seed, WASH_SALT, 1));
    let rel = r.range(-31.0, -27.0);
    let period = r.range(17.0, 26.0);
    let mut r5 = Rng::new(js::salted(seed, WASH_SALT_5, 1));
    let pc = (root + if r5.chance(0.6) { 0.0 } else { 7.0 }) % 12.0;
    let mut hz = 440.0 * math::pow(2.0, (pc - 9.0) / 12.0);
    while hz < 600.0 {
        hz *= 2.0;
    }
    while hz > 1200.0 {
        hz /= 2.0;
    }
    let tint_gain = r5.range(4.0, 7.0);
    let tint_high = r5.range(2.0, 4.0);
    let tint_q = r5.range(3.0, 5.0);
    Wash { level: WASH_K * math::pow(10.0, rel / 20.0), period, tint: hz, tint_gain, tint_high, tint_q }
}
