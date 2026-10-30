//! The bass, drums and texture layers (queue item 80): `genBass` with item
//! 56's register and item 71's pedal (`pedalBass`, `pedalOf`), `genDrums`
//! with wayfare's travelling grooves (`travelGroove`), and `genTexture`,
//! from `js/generator.js` and `js/choices.js`.

use super::choices::lean_of;
use super::data::{self, MAX_BARS, PEDAL};
use super::js;
use super::list::List;
use super::plan::{Cx, Notes, Slot, slot_at, SLOTS_MAX};
use super::rng::Rng;
use super::spec::Metre;
use super::theory::scale_pitch;
use super::Layer;

// --------------------------------------------------------------- bass

/// The bass's notes at most: a chug's eighths (half the loop's steps, and
/// one more a slot), above any other style's.
pub const BASS_MAX: usize = MAX_BARS * data::MAX_STEPS_PER_BAR / 2 + SLOTS_MAX;

/// A bass note.
#[derive(Clone, Copy)]
pub struct BassEvent {
    pub step: f64,
    pub dur: f64,
    pub midi: f64,
    pub vel: f64,
    /// The velocity as generated (a repair puts it back).
    pub vel0: f64,
    pub glide: bool,
    pub voice: &'static str,
    /// Marked when a chug plays it.
    pub chug: bool,
    pub steps: Option<List<f64, 7>>,
}

/// An empty bass note.
pub const NO_BASS: BassEvent =
    BassEvent { step: 0.0, dur: 0.0, midi: 0.0, vel: 0.0, vel0: 0.0, glide: false, voice: "", chug: false, steps: None };

/// The bass's notes.
pub type Bass = List<BassEvent, BASS_MAX>;

/// What `genBass` returns besides its notes.
#[derive(Clone, Copy)]
pub struct BassOut {
    pub style: &'static str,
    pub voice: &'static str,
}

const DRONE_SALT: u32 = 0x0d70_11e5;
const CHUG_SALT: u32 = 0x0c4a_6e11;
const CHUG_DUR: f64 = 0.5;

/// `chugs(spec)`: whether the loop chugs, decided once from its seed.
#[must_use]
pub fn chugs(cx: &Cx) -> bool {
    match cx.c.chug {
        Some(p) if p != 0.0 => Rng::new(js::salted(cx.spec.seed, CHUG_SALT, 1)).chance(p),
        _ => false,
    }
}

const PULSE_34: [&[f64]; 4] = [&[0.0], &[0.0, 8.0], &[0.0, 10.0], &[0.0, 6.0]];
const PULSE_12: [&[f64]; 4] = [&[0.0, 6.0], &[0.0, 3.0, 6.0, 9.0], &[0.0, 4.0, 8.0], &[0.0, 6.0, 9.0]];
const PULSE_16: [&[f64]; 4] = [&[0.0, 8.0], &[0.0, 6.0, 10.0], &[0.0, 4.0, 8.0, 12.0], &[0.0, 7.0, 10.0]];

/// `genBass(layerSpec(spec, 'bass'), harmony)`, its notes into `out`.
#[allow(clippy::too_many_lines)] // one function, as genBass is: its draws in order
pub fn gen_bass(cx: &Cx, bars: f64, slots: &[Slot], out: &mut Bass) -> BassOut {
    let spec = cx.spec;
    let c = &cx.c;
    let seed = spec.layer_seed(Layer::Bass);
    let mut r = Rng::new(seed);
    let spb = cx.spb;
    let scale = spec.steps();
    let total = bars * spb;
    let mut style = c.bass_styles.as_ref().and_then(|p| r.weighted(p)).unwrap_or("");
    if let Some(d) = c.drone
        && d != 0.0
        && Rng::new(js::salted(seed, DRONE_SALT, 1)).chance(d)
    {
        style = "drone";
    }
    if chugs(cx) {
        style = "chug";
    }
    let voice = match &c.bass_voices {
        Some(p) => r.weighted(p).unwrap_or(""),
        None => "sub",
    };
    let glide_chance = c.glide.unwrap_or(0.0);
    r.chance(0.25);
    out.clear();
    let place = |out: &mut Bass, step: f64, dur: f64, midi: f64, vel: f64, glide: bool| {
        let mut m = midi;
        while m > 57.0 {
            m -= 12.0;
        }
        while m < 36.0 {
            m += 12.0;
        }
        out.push(BassEvent { step: step % total, dur, midi: m, vel, vel0: vel, glide, voice, ..NO_BASS });
    };

    if style == "drone" {
        let tonic = scale_pitch(spec.root, scale, 0.0, 0.0) - 24.0;
        let mut bar = 0.0;
        while bar < bars {
            place(out, bar * spb, spb - 1.0, tonic, 0.58, false);
            if voice != "fifths"
                && let Some(&root) = out.last()
            {
                out.push(BassEvent { midi: root.midi + 7.0, vel: 0.4, vel0: 0.4, ..root });
            }
            bar += 1.0;
        }
        return BassOut { style, voice };
    }

    if style == "chug" {
        let beat = if cx.metre == Metre::SixEight { 6.0 } else { 4.0 };
        for slot in slots {
            let base = slot.bass_midi - 24.0;
            let mut s = 0.0;
            while s < slot.length_steps {
                let at = slot.start_step + s;
                let vel = (if at % beat == 0.0 { 0.62 } else { 0.3 }) + r.f() * 0.06;
                place(out, at, CHUG_DUR, base, vel, false);
                if let Some(e) = out.last_mut() {
                    e.chug = true;
                }
                s += 2.0;
            }
        }
        return BassOut { style, voice };
    }

    for slot in slots {
        let base = slot.bass_midi - 24.0;
        let len = slot.length_steps;
        if r.chance(0.06) {
            continue;
        }
        let start = slot.start_step;
        match style {
            "held" => {
                let vel = 0.62 + r.f() * 0.15;
                let glide = r.chance(glide_chance);
                place(out, start, len - 1.0, base, vel, glide);
                if r.chance(0.3) {
                    place(out, start + len - 2.0, 2.0, base + 7.0, 0.4, false);
                }
            }
            "pulse" => {
                let grid = if cx.metre == Metre::ThreeFour {
                    r.pick(&PULSE_34)
                } else if spb == 12.0 {
                    r.pick(&PULSE_12)
                } else {
                    r.pick(&PULSE_16)
                }
                .unwrap_or(&[]);
                for &g in grid {
                    if g >= len {
                        continue;
                    }
                    let dur = r.pick(&[2.0, 3.0, 4.0]).unwrap_or(2.0);
                    let vel = if g == 0.0 { 0.7 } else { 0.45 + r.f() * 0.2 };
                    let glide = g > 0.0 && r.chance(glide_chance);
                    place(out, start + g, dur, base, vel, glide);
                }
            }
            "dub" => {
                let [[_, held], [answer, answer_len], [pickup, pickup_len]] = if cx.metre == Metre::SixEight {
                    [[0.0, 4.0], [8.0, 2.0], [10.0, 2.0]]
                } else {
                    [[0.0, 6.0], [10.0, 4.0], [14.0, 2.0]]
                };
                place(out, start, held, base, 0.72, false);
                if r.chance(0.7) {
                    let glide = r.chance(0.4);
                    if answer < len {
                        place(out, start + answer, answer_len, base, 0.5, glide);
                    }
                }
                if r.chance(0.35) {
                    let interval = r.pick(&[3.0, 5.0, 7.0]).unwrap_or(3.0);
                    if pickup < len {
                        place(out, start + pickup, pickup_len, base + interval, 0.4, false);
                    }
                }
            }
            "walk" => {
                let walk: &[[f64; 2]] = match cx.metre {
                    Metre::SixEight => &[[0.0, 3.0], [3.0, 3.0], [6.0, 3.0], [9.0, 3.0]],
                    Metre::ThreeFour => &[[0.0, 8.0], [8.0, 4.0]],
                    _ => &[[0.0, 3.0], [4.0, 3.0], [8.0, 3.0], [12.0, 3.0]],
                };
                let mut i = 0;
                for &[s, dur] in walk {
                    if !(s < len) {
                        continue;
                    }
                    let deg = slot.degree + if i == 0 { 0.0 } else { r.pick(&[0.0, 1.0, -1.0, 2.0, 4.0]).unwrap_or(0.0) };
                    let midi = scale_pitch(spec.root, scale, deg, -2.0);
                    let vel = if i == 0 { 0.68 } else { 0.42 + r.f() * 0.15 };
                    place(out, start + s, dur, midi, vel, false);
                    i += 1;
                }
            }
            _ => {
                if r.chance(0.75) {
                    let dur = len + (if r.chance(0.3) { len } else { 0.0 }) - 1.0;
                    place(out, start, dur, base, 0.55, false);
                }
            }
        }
    }
    BassOut { style, voice }
}

const PEDAL_SALT: u32 = 0x0009_eda1;

/// A pedal's draws (`pedalBass`'s, and its meta).
#[derive(Clone, Copy)]
pub struct PedalOut {
    pub rhythm: &'static str,
    pub nb: f64,
    pub share: f64,
}

/// `pedalBass`: the key's tonic in a rhythm, in place of `drawn` (genBass's
/// notes, as it drew them), into `out`.
pub fn pedal_bass(cx: &Cx, bars: f64, drawn: &Bass, voice: &'static str, out: &mut Bass) -> PedalOut {
    let spec = cx.spec;
    let spb = cx.spb;
    let scale = spec.steps();
    let v = spec.vibes.as_ref();
    // pedalOf
    let mut rng = Rng::new(js::salted(spec.layer_seed(Layer::Bass), PEDAL_SALT, 1));
    rng.f();
    let rhythm = rng.weighted(PEDAL.rhythms).unwrap_or("held");
    let mut nbs: List<(f64, f64), 8> = List::filled((0.0, 0.0));
    for &(nb, w) in PEDAL.neighbors {
        if scale.contains(&(((nb % 12.0) + 12.0) % 12.0)) {
            nbs.push((nb, w * lean_of(data::table_at(PEDAL.neighbor_vibes, nb), v)));
        }
    }
    let nb = if nbs.is_empty() { 0.0 } else { rng.weighted(&nbs).unwrap_or(0.0) };
    let share = rng.range(PEDAL.share[0], PEDAL.share[1]);

    let mid = |get: fn(&super::layers::BassEvent) -> f64| {
        let mut x: List<f64, BASS_MAX> = List::filled(0.0);
        for e in drawn.iter().filter(|e| e.vel0 != 0.0) {
            x.push(get(e));
        }
        x.sort_stable_by(|a, b| a < b);
        x.get(x.len() / 2).copied()
    };
    let orig_midi = mid(|e| e.midi);
    let orig_dur = mid(|e| e.dur);
    let grid: &[[f64; 2]] = match (cx.metre, rhythm) {
        (Metre::SixEight | Metre::ThreeFour, "held") => &[[0.0, 11.0]],
        (Metre::SixEight, "half") => &[[0.0, 5.0], [6.0, 5.0]],
        (Metre::SixEight, "quarter") => &[[0.0, 2.0], [3.0, 2.0], [6.0, 2.0], [9.0, 2.0]],
        (Metre::SixEight, _) => &[[3.0, 2.0], [9.0, 2.0]],
        (Metre::ThreeFour, "half") => &[[0.0, 7.0], [8.0, 3.0]],
        (Metre::ThreeFour, "quarter") => &[[0.0, 3.0], [4.0, 3.0], [8.0, 3.0]],
        (Metre::ThreeFour, _) => &[[2.0, 2.0], [6.0, 2.0], [10.0, 2.0]],
        (_, "held") => &[[0.0, 15.0]],
        (_, "half") => &[[0.0, 7.0], [8.0, 7.0]],
        (_, "quarter") => &[[0.0, 3.0], [4.0, 3.0], [8.0, 3.0], [12.0, 3.0]],
        _ => &[[2.0, 2.0], [6.0, 2.0], [10.0, 2.0], [14.0, 2.0]],
    };
    let (low, high) = (PEDAL.low, PEDAL.high);
    let mut t = scale_pitch(spec.root, scale, 0.0, 0.0) - 24.0;
    while t > high {
        t -= 12.0;
    }
    while t < low {
        t += 12.0;
    }
    if let Some(o) = orig_midi {
        while t - 12.0 >= low && js::abs(t - 12.0 - o) < js::abs(t - o) {
            t -= 12.0;
        }
        while t + 12.0 <= high && js::abs(t + 12.0 - o) < js::abs(t - o) {
            t += 12.0;
        }
    }
    if t + js::min(0.0, nb) < low && t + 12.0 <= high {
        t += 12.0;
    }
    out.clear();
    let mut prev_nb = false;
    let mut bar = 0.0;
    while bar < bars {
        for (i, &[st, d]) in grid.iter().enumerate() {
            let first = i == 0 && rhythm != "offbeat";
            let step = !first && !prev_nb && nb != 0.0 && rng.chance(share);
            prev_nb = step;
            let m = if step && t + nb >= low && t + nb <= high { t + nb } else { t };
            let vel = if first {
                0.66
            } else if rhythm == "offbeat" {
                0.58
            } else {
                0.48 + rng.f() * 0.1
            };
            let dur = match orig_dur {
                Some(o) if o != 0.0 && !o.is_nan() => js::min(d, js::max(1.0, o)),
                _ => d,
            };
            out.push(BassEvent { step: bar * spb + st, dur, midi: m, vel, vel0: vel, glide: false, voice, ..NO_BASS });
        }
        bar += 1.0;
    }
    PedalOut { rhythm, nb, share }
}

// -------------------------------------------------------------- drums

/// The drums' hits at most: a bar's every hit (kick, snare, ghost, hats on
/// every step, the shaker, a roll and a fill), every bar.
pub const DRUMS_MAX: usize = MAX_BARS * (4 + 3 + 1 + data::MAX_STEPS_PER_BAR + 5 + 8 + 4);

/// A drum hit.
#[derive(Clone, Copy)]
pub struct DrumEvent {
    pub step: f64,
    pub inst: &'static str,
    pub vel: f64,
    /// The velocity as generated (a repair puts it back).
    pub vel0: f64,
    pub roll: bool,
    pub micro: Option<f64>,
}

/// An empty hit.
pub const NO_DRUM: DrumEvent = DrumEvent { step: 0.0, inst: "", vel: 0.0, vel0: 0.0, roll: false, micro: None };

/// The drums' hits.
pub type Drums = List<DrumEvent, DRUMS_MAX>;

/// What `genDrums` returns besides its hits.
#[derive(Clone, Copy)]
pub struct DrumsOut {
    pub kit: &'static str,
}

const KICK_PATTERNS: [&[f64]; 8] = [
    &[0.0, 10.0],
    &[0.0, 6.0, 10.0],
    &[0.0, 7.0, 10.0],
    &[0.0, 10.0, 11.0],
    &[0.0],
    &[0.0, 3.0, 10.0],
    &[0.0, 8.0],
    &[0.0, 6.0, 11.0],
];
const SNARE_PATTERNS: [&[f64]; 5] = [&[4.0, 12.0], &[12.0], &[4.0, 12.0], &[4.0, 12.0, 14.0], &[8.0]];
const KICK_12: [&[f64]; 5] = [&[0.0, 6.0], &[0.0], &[0.0, 7.0], &[0.0, 6.0, 9.0], &[0.0, 4.0]];
const SNARE_12: [&[f64]; 4] = [&[6.0], &[3.0, 9.0], &[6.0], &[6.0, 10.0]];
const SNARE_34: [&[f64]; 4] = [&[4.0, 8.0], &[8.0], &[6.0, 10.0], &[4.0, 8.0]];
const KICK_20: [&[f64]; 4] = [&[0.0, 12.0], &[0.0], &[0.0, 8.0, 12.0], &[0.0, 12.0, 16.0]];
const SNARE_20: [&[f64]; 3] = [&[8.0], &[8.0, 16.0], &[12.0]];

/// `HAND_KIT`: the hand kit's instrument for each kit instrument (any
/// other plays as itself).
#[must_use]
pub fn hand(inst: &'static str) -> &'static str {
    match inst {
        "kick" => "frame",
        "snare" | "clap" | "rim" => "tap",
        "hat" => "jingle",
        "ohat" => "ojingle",
        other => other,
    }
}

fn hit(step: f64, inst: &'static str, vel: f64) -> DrumEvent {
    DrumEvent { step, inst, vel, vel0: vel, ..NO_DRUM }
}

/// `genDrums(layerSpec(spec, 'drums'))`, its hits into `out`.
#[allow(clippy::too_many_lines)] // one function, as genDrums is: its draws in order
pub fn gen_drums(cx: &Cx, bars: f64, out: &mut Drums) -> DrumsOut {
    let spec = cx.spec;
    let c = &cx.c;
    let mut r = Rng::new(spec.layer_seed(Layer::Drums));
    let spb = cx.spb;
    let beat = if cx.metre == Metre::SixEight { 6.0 } else { 4.0 };
    let total = bars * spb;
    out.clear();
    if !r.chance(c.drums.unwrap_or(f64::NAN)) {
        return DrumsOut { kit: "none" };
    }
    let kit = match c.kits {
        Some(p) => r.weighted(p),
        None => r.weighted(&[("tape", 4.0), ("brush", 2.0), ("machine", 3.0)]),
    }
    .unwrap_or("tape");
    if c.groove == Some("travel") {
        travel_groove(cx, bars, &mut r, kit, out);
        return DrumsOut { kit };
    }
    let four_floor = match c.four_floor {
        Some(p) if p != 0.0 => spb == 16.0 && r.chance(p),
        _ => false,
    };
    let kick: &[f64] = if four_floor {
        &[0.0, 4.0, 8.0, 12.0]
    } else if spb == 12.0 {
        r.pick(&KICK_12).unwrap_or(&[])
    } else if spb == 20.0 {
        r.pick(&KICK_20).unwrap_or(&[])
    } else {
        r.pick(&KICK_PATTERNS).unwrap_or(&[])
    };
    let snare: &[f64] = if spb == 12.0 {
        if cx.metre == Metre::ThreeFour { r.pick(&SNARE_34) } else { r.pick(&SNARE_12) }.unwrap_or(&[])
    } else if spb == 20.0 {
        r.pick(&SNARE_20).unwrap_or(&[])
    } else {
        r.pick(&SNARE_PATTERNS).unwrap_or(&[])
    };
    let snare_voice = r.weighted(&[("snare", 3.0), ("rim", 2.0), ("clap", 1.5)]).unwrap_or("snare");
    let feel = spec.feel_for(Layer::Drums);
    let hd = c.hat_density.unwrap_or([f64::NAN, f64::NAN]);
    let hat_density = js::min(1.0, r.range(hd[0], hd[1]) * (0.55 + feel.energy * 0.9));
    let hat_grid = if r.chance(0.55) { 2.0 } else { 1.0 };
    let use_shaker = r.chance(0.4);
    let rolls = c.rolls.unwrap_or(0.0);

    let mut bar = 0.0;
    while bar < bars {
        let b = bar * spb;
        let last_bar = bar == bars - 1.0;
        for &s in kick {
            if bar > 0.0 && r.chance(0.1) {
                continue;
            }
            let vel = if s == 0.0 { 0.84 } else { 0.62 + r.f() * 0.18 };
            out.push(hit(b + s, "kick", vel));
        }
        for &s in snare {
            let vel = 0.58 + r.f() * 0.18;
            out.push(hit(b + s, snare_voice, vel));
        }
        if r.chance(0.4) {
            let at = r.int(1.0, spb - 1.0);
            out.push(hit(b + at, "rim", 0.22));
        }
        let mut s = 0.0;
        while s < spb {
            let plays = if four_floor && s % 4.0 != 2.0 { r.chance(hat_density * 0.25) } else { r.chance(hat_density) };
            if plays {
                let open = r.chance(0.08);
                let vel = (if s % beat == 0.0 { 0.5 } else { 0.3 }) + r.f() * 0.2;
                out.push(hit(b + s, if open { "ohat" } else { "hat" }, vel));
            }
            s += hat_grid;
        }
        if use_shaker {
            let mut s = 2.0;
            let mut i = 0;
            while s < spb {
                if r.chance(0.6) {
                    let vel = 0.18 + r.f() * 0.15;
                    if cx.metre == Metre::SixEight {
                        let pairs: [&[f64]; 3] = [&[2.0, 4.0], &[8.0, 10.0], &[]];
                        for &at in pairs.get(i).copied().unwrap_or(&[]) {
                            out.push(hit(b + at, "shaker", vel));
                        }
                    } else {
                        out.push(hit(b + s, "shaker", vel));
                    }
                }
                s += 4.0;
                i += 1;
            }
        }
        if rolls != 0.0 && r.chance(rolls) {
            let from = r.int(0.0, spb - 4.0);
            let count = r.pick(&[3.0, 4.0, 6.0, 8.0]).unwrap_or(3.0);
            let inst = r.pick(&["snare", "rim", "hat", "kick"]).unwrap_or("snare");
            let mut k = 0.0;
            while k < count {
                let step = b + from + js::floor((k * 4.0) / count);
                if step >= b + spb {
                    break;
                }
                let vel = (0.55 - k * 0.045) * (0.8 + r.f() * 0.4);
                out.push(DrumEvent { roll: true, micro: Some(k / count), ..hit(step, inst, vel) });
                k += 1.0;
            }
        }
        if last_bar && r.chance(0.35) {
            let from = spb - r.int(2.0, 4.0);
            let mut s = from;
            while s < spb {
                let inst = r.pick(&["rim", "snare", "shaker"]).unwrap_or("rim");
                let vel = 0.3 + r.f() * 0.3;
                out.push(hit(b + s, inst, vel));
                s += 1.0;
            }
        }
        bar += 1.0;
    }
    out.retain(|e| e.step < total);
    if kit == "hand" {
        for e in out.iter_mut() {
            e.inst = hand(e.inst);
        }
    }
    DrumsOut { kit }
}

const TRAVEL_SWELL: [f64; 4] = [0.78, 0.92, 1.08, 1.22];
const EIGHTH_LEAN: [f64; 3] = [1.2, 1.0, 0.78];
const TRAVEL_FORMS: [[u8; 4]; 5] = [[0, 0, 0, 1], [0, 1, 0, 1], [0, 0, 1, 1], [0, 1, 0, 0], [0, 1, 1, 0]];

/// `travelGroove`: wayfare's grooves, its hits into `out`.
#[allow(clippy::too_many_lines)] // one function, as travelGroove is: its draws in order
fn travel_groove(cx: &Cx, bars: f64, r: &mut Rng, kit: &'static str, out: &mut Drums) {
    let spb = cx.spb;
    let beat = if cx.metre == Metre::SixEight { 6.0 } else { 4.0 };
    let beats = spb / beat;
    let total = bars * spb;
    let back = r.weighted(&[("rim", 3.0), ("snare", 2.0)]).unwrap_or("rim");
    let shaker = r.chance(0.4);
    let fills = r.chance(0.7);
    let chug = kit == "brush" && chugs(cx);
    let colour = r.chance(0.8);
    let ghosts = r.chance(0.7);
    let open = !chug && r.chance(0.75);
    let pickup = r.chance(0.7);
    let form = r.pick(&TRAVEL_FORMS).unwrap_or([0; 4]);
    let thin = if chug { 0.0 } else { r.range(0.0, 0.25) };
    let eighth = if shaker && !chug { "shaker" } else { "hat" };
    let other = if eighth == "hat" { "shaker" } else { "hat" };
    let ghost = if back == "rim" { "snare" } else { "rim" };
    let mut offbeats: List<f64, 16> = List::filled(0.0);
    let mut s = 2.0;
    while s < spb {
        if s % beat != 0.0 {
            offbeats.push(s);
        }
        s += 2.0;
    }
    let mut ghost_at: List<f64, 8> = List::filled(0.0);
    let mut k = 1.0;
    while k <= beats {
        if k % 2.0 == 1.0 || k == beats {
            ghost_at.push(k * beat - 1.0);
        }
        k += 1.0;
    }
    let mut bar = 0.0;
    while bar < bars {
        let b = bar * spb;
        let pos = bar % 4.0;
        let swell = TRAVEL_SWELL[js::index(js::min(3.0, pos + js::max(0.0, 4.0 - bars))).min(3)];
        let answer = form[js::index(pos).min(3)] == 1;
        let fill = fills && (pos == 3.0 || bar == bars - 1.0);
        let last_beat = spb - beat;
        let mut k = 0.0;
        while k < beats {
            let at = b + k * beat;
            if k % 2.0 == 0.0 {
                let vel = (if k == 0.0 { 0.64 } else { 0.64 * 0.85 }) + r.f() * 0.05;
                out.push(hit(at, "kick", vel));
            } else {
                let vel = (0.42 + r.f() * 0.08) * (if k == beats - 1.0 { 1.1 } else { 1.0 }) * swell;
                out.push(hit(at, back, vel));
            }
            k += 1.0;
        }
        if answer && pickup && !fill {
            let vel = 0.64 * 0.62 + r.f() * 0.05;
            out.push(hit(b + spb - 2.0, "kick", vel));
        }
        let (on, off) = if chug { (0.28, 0.1) } else { (0.34, 0.15) };
        let mut s = 0.0;
        while s < spb {
            if fill && !chug && s > last_beat {
                s += 2.0;
                continue;
            }
            if s % beat != 0.0 && r.chance(thin) {
                s += 2.0;
                continue;
            }
            let opens = open && answer && !fill && s == spb - 2.0;
            let inst = if opens {
                if eighth == "hat" { "ohat" } else { "shaker" }
            } else {
                eighth
            };
            let lean = if (s / beat) % 2.0 != 0.0 {
                EIGHTH_LEAN[2]
            } else if s != 0.0 {
                EIGHTH_LEAN[1]
            } else {
                EIGHTH_LEAN[0]
            };
            let base = if s % beat == 0.0 { on * lean } else { off };
            let vel = (base + (if opens { 0.1 } else { 0.0 }) + r.f() * 0.05) * swell;
            out.push(hit(b + s, inst, vel));
            s += 2.0;
        }
        if colour && r.chance(0.5) {
            let at = r.pick(&offbeats).unwrap_or(f64::NAN);
            let vel = 0.17 + r.f() * 0.08;
            out.push(hit(b + at, other, vel));
        }
        if ghosts && r.chance(0.7) {
            let at = r.pick(&ghost_at).unwrap_or(f64::NAN);
            if !(fill && at > last_beat) {
                let vel = 0.08 + r.f() * 0.06;
                out.push(hit(b + at, ghost, vel));
            }
        }
        if fill {
            match r.pick(&["run", "roll", "pickup"]).unwrap_or("run") {
                "run" => {
                    let mut s = spb - 3.0;
                    let mut k = 0.0;
                    while s < spb {
                        let vel = 0.22 + k * 0.08 + r.f() * 0.05;
                        out.push(hit(b + s, back, vel));
                        s += 1.0;
                        k += 1.0;
                    }
                }
                "roll" => {
                    let mut k = 0.0;
                    while k < 4.0 {
                        let vel = 0.16 + k * 0.07 + r.f() * 0.04;
                        out.push(DrumEvent { roll: true, micro: Some((k % 2.0) / 2.0), ..hit(b + spb - 2.0 + js::floor(k / 2.0), back, vel) });
                        k += 1.0;
                    }
                }
                _ => {
                    let mut s = last_beat + 2.0;
                    while s < spb {
                        let vel = 0.26 + r.f() * 0.08;
                        out.push(hit(b + s, back, vel));
                        s += 2.0;
                    }
                }
            }
        }
        bar += 1.0;
    }
    out.retain(|e| e.step < total);
    for e in out.iter_mut() {
        e.inst = if kit == "hand" {
            hand(e.inst)
        } else if e.inst == "kick" {
            "softkick"
        } else {
            e.inst
        };
    }
}

// ------------------------------------------------------------ texture

/// The texture's events at most: a wave or a wind segment a bar.
pub const TEXTURE_MAX: usize = MAX_BARS + 8;

/// A texture event.
#[derive(Clone, Copy)]
pub struct TextureEvent {
    pub step: f64,
    pub dur: f64,
    pub notes: Notes,
    pub vel: f64,
    /// The velocity as generated (a repair puts it back).
    pub vel0: f64,
    pub kind: &'static str,
    pub band: Option<f64>,
}

/// An empty texture event.
pub const NO_TEXTURE: TextureEvent = TextureEvent { step: 0.0, dur: 0.0, notes: List::filled(0), vel: 0.0, vel0: 0.0, kind: "", band: None };

/// The texture's events.
pub type Texture = List<TextureEvent, TEXTURE_MAX>;

fn air(step: f64, dur: f64, notes: &[i16], vel: f64, kind: &'static str) -> TextureEvent {
    TextureEvent { step, dur, notes: List::from_slice(0, notes), vel, vel0: vel, kind, band: None }
}

/// `genTexture(layerSpec(spec, 'texture'), harmony)`, into `out`; returns
/// the kind.
pub fn gen_texture(cx: &Cx, bars: f64, slots: &[Slot], cycle_steps: f64, out: &mut Texture) -> &'static str {
    let spec = cx.spec;
    let mut r = Rng::new(spec.layer_seed(Layer::Texture));
    let spb = cx.spb;
    let total = bars * spb;
    let kind = match &cx.c.textures {
        Some(p) => r.weighted(p),
        None => r.weighted(&[("bells", 3.0), ("swell", 3.0), ("chime", 2.0), ("drops", 2.0), ("wind", 1.0), ("none", 1.5)]),
    }
    .unwrap_or("none");
    out.clear();
    let note = |slot: &Slot, r: &mut Rng, up: i16| {
        let n = slot.notes.len();
        let i = js::index(r.int(0.0, js::num(n) - 1.0));
        slot.notes.get(i).map_or(0, |&x| x + up)
    };
    match kind {
        "none" => {}
        "swell" => {
            let mut bar = 0.0;
            while bar < bars {
                let slot = slot_at(slots, bar * spb, cycle_steps);
                let notes: List<i16, 2> = slot.notes.iter().take(2).map(|n| n + 12).fold(List::filled(0), |mut l, n| {
                    l.push(n);
                    l
                });
                let vel = 0.16 + r.f() * 0.1;
                out.push(air(bar * spb, 32.0, &notes, vel, "swell"));
                bar += 2.0;
            }
        }
        "bells" | "chime" => {
            let (count, dur, up, lo, span, name) = if kind == "bells" {
                (r.int(2.0, 5.0), 8.0, 24, 0.12, 0.12, "bell")
            } else {
                (r.int(2.0, 4.0), 16.0, 12, 0.1, 0.1, "chime")
            };
            let mut i = 0.0;
            while i < count {
                let step = r.int(0.0, total - 1.0);
                let slot = slot_at(slots, step, cycle_steps);
                let n = note(slot, &mut r, up);
                let vel = lo + r.f() * span;
                out.push(air(step, dur, &[n], vel, name));
                i += 1.0;
            }
        }
        "drops" => {
            let mut i = 0.0;
            while i < r.int(3.0, 7.0) {
                let step = r.int(0.0, total - 1.0);
                let vel = 0.2 + r.f() * 0.2;
                out.push(air(step, 2.0, &[], vel, "drop"));
                i += 1.0;
            }
        }
        "waves" => {
            let sd = 60.0 / spec.bpm / 4.0;
            let mut st = 0.0;
            while st < total {
                let len = js::max(spb, js::round((6.0 + r.f() * 3.0) / sd));
                let vel = 0.14 + r.f() * 0.08;
                out.push(air(st, len, &[], vel, "waves"));
                st += len + if r.chance(0.3) { js::round((1.0 + r.f() * 2.0) / sd) } else { 0.0 };
            }
        }
        _ => {
            let seg = spb * 2.0;
            let mut st = 0.0;
            while st < total {
                if !(st > 0.0 && r.chance(0.35)) {
                    let vel = 0.07 + r.f() * 0.08;
                    let band = 420.0 + r.f() * 520.0;
                    out.push(TextureEvent { band: Some(band), ..air(st, seg, &[], vel, "wind") });
                }
                st += seg;
            }
        }
    }
    kind
}
