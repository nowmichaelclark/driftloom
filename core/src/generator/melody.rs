//! The melody (queue item 80): `genMelody` (the motif, its cell, the
//! phrase plan, the anchors, the register) and a developing loop's
//! `developMelody` (`answerFigure`, `varyFigure`), from `js/generator.js`.

use super::data::{MAX_BARS, MAX_STEPS_PER_BAR};
use super::js;
use super::list::List;
use super::plan::{B_SALT, ChordEvent, Cx, Development, RETURN_SALT, Sections, Slot, VARY_SALT, VOWEL_KEYS, slot_at};
use super::rng::Rng;
use super::spec::Metre;
use super::theory::scale_pitch;
use super::Layer;
use crate::math;

/// The notes a figure holds at most: a cell's (eight at most) and, for a
/// fragment, as many again.
pub const FIGURE_MAX: usize = 16;
/// The melody's notes at most: a figure's every note in every bar.
pub const MELODY_MAX: usize = MAX_BARS * FIGURE_MAX;

/// A note of a motif: where in the bar, its scale degree, its length, its
/// velocity.
#[derive(Clone, Copy, Debug)]
pub struct Motif {
    pub offset: f64,
    pub degree: f64,
    pub dur: f64,
    pub vel: f64,
}

/// A figure: a motif's notes.
pub type Figure = List<Motif, FIGURE_MAX>;

/// An empty motif note.
pub const NO_MOTIF: Motif = Motif { offset: 0.0, degree: 0.0, dur: 0.0, vel: 0.0 };

/// A melody note, as the melody layer plays it.
#[derive(Clone, Copy)]
pub struct MelodyEvent {
    pub step: f64,
    pub dur: f64,
    pub midi: f64,
    pub vel: f64,
    /// The velocity as generated, before the entry schedules (a repair
    /// puts it back).
    pub vel0: f64,
    pub voice: &'static str,
    pub degree: f64,
    pub phrase: f64,
    pub vowel: &'static str,
    pub detune: Option<f64>,
    pub prev: Option<f64>,
    pub orn: Option<&'static str>,
    pub harm: Option<&'static str>,
    pub steps: Option<List<f64, 7>>,
}

/// An empty note.
pub const NO_NOTE: MelodyEvent = MelodyEvent {
    step: 0.0,
    dur: 0.0,
    midi: 0.0,
    vel: 0.0,
    vel0: 0.0,
    voice: "",
    degree: 0.0,
    phrase: 0.0,
    vowel: "a",
    detune: None,
    prev: None,
    orn: None,
    harm: None,
    steps: None,
};

/// The melody's notes.
pub type Melody = List<MelodyEvent, MELODY_MAX>;

// ------------------------------------------------------------- cells

/// A rhythmic cell: one bar of `[step, length, accent]`.
#[derive(Clone, Copy)]
pub struct Cell {
    pub id: &'static str,
    e: f64,
    l: f64,
    w: f64,
    pub pat: List<[f64; 3], 10>,
}

type Pat = &'static [[f64; 3]];

const fn c(id: &'static str, e: f64, l: f64, w: f64, pat: Pat) -> (&'static str, f64, f64, f64, Pat) {
    (id, e, l, w, pat)
}

const CELLS_16: [(&str, f64, f64, f64, Pat); 18] = [
    c("quarters", -0.8, -0.6, 2.0, &[[0.0, 4.0, 1.0], [4.0, 4.0, 0.0], [8.0, 4.0, 1.0], [12.0, 4.0, 0.0]]),
    c("halves", -2.2, -0.8, 1.4, &[[0.0, 8.0, 1.0], [8.0, 8.0, 0.0]]),
    c("whole", -3.0, -1.4, 0.8, &[[0.0, 16.0, 1.0]]),
    c("openLate", -1.8, -0.3, 1.2, &[[0.0, 8.0, 1.0], [10.0, 6.0, 0.0]]),
    c("dotted32", 0.2, 0.3, 2.0, &[[0.0, 6.0, 1.0], [6.0, 6.0, 0.0], [12.0, 4.0, 0.0]]),
    c("dottedPair", 0.2, 0.4, 1.6, &[[0.0, 6.0, 1.0], [6.0, 2.0, 0.0], [8.0, 6.0, 1.0], [14.0, 2.0, 0.0]]),
    c("gallop", 1.2, 0.9, 1.9, &[[0.0, 3.0, 1.0], [3.0, 1.0, 0.0], [4.0, 3.0, 1.0], [7.0, 1.0, 0.0], [8.0, 3.0, 1.0], [11.0, 1.0, 0.0], [12.0, 4.0, 0.0]]),
    c("charleston", 1.0, 1.0, 1.8, &[[0.0, 4.0, 1.0], [6.0, 4.0, 0.0], [11.0, 5.0, 1.0]]),
    c("clave", 1.6, 1.1, 1.7, &[[0.0, 3.0, 1.0], [3.0, 3.0, 0.0], [6.0, 4.0, 0.0], [10.0, 3.0, 1.0], [13.0, 3.0, 0.0]]),
    c("offEighths", 0.9, 0.7, 1.7, &[[2.0, 2.0, 1.0], [6.0, 2.0, 0.0], [10.0, 2.0, 0.0], [13.0, 1.0, 0.0], [14.0, 2.0, 1.0]]),
    c("anticipate", 1.1, 1.0, 1.9, &[[0.0, 4.0, 1.0], [7.0, 1.0, 0.0], [8.0, 4.0, 1.0], [15.0, 1.0, 0.0]]),
    c("pushIn", 1.1, 0.9, 1.4, &[[3.0, 5.0, 1.0], [8.0, 4.0, 0.0], [15.0, 1.0, 0.0]]),
    c("shortLong", 1.3, 1.2, 1.8, &[[0.0, 1.0, 0.0], [1.0, 1.0, 0.0], [2.0, 6.0, 1.0], [8.0, 1.0, 0.0], [9.0, 1.0, 0.0], [10.0, 6.0, 1.0]]),
    c("longShort", 0.3, 0.3, 1.5, &[[0.0, 6.0, 1.0], [6.0, 1.0, 0.0], [7.0, 1.0, 0.0], [8.0, 8.0, 1.0]]),
    c("threeFour", 1.0, 0.7, 1.8, &[[0.0, 3.0, 1.0], [3.0, 3.0, 0.0], [6.0, 3.0, 0.0], [9.0, 3.0, 0.0], [12.0, 3.0, 1.0]]),
    c("burst", 2.8, 1.8, 1.4, &[[0.0, 1.0, 0.0], [1.0, 1.0, 0.0], [2.0, 1.0, 0.0], [4.0, 2.0, 1.0], [8.0, 1.0, 0.0], [9.0, 1.0, 0.0], [10.0, 1.0, 0.0], [12.0, 2.0, 1.0]]),
    c("chatter", 3.0, 1.7, 1.2, &[[0.0, 1.0, 0.0], [2.0, 1.0, 0.0], [3.0, 1.0, 0.0], [6.0, 1.0, 0.0], [8.0, 1.0, 0.0], [10.0, 1.0, 0.0], [11.0, 1.0, 0.0], [14.0, 2.0, 1.0]]),
    c("skipStep", 2.2, 1.6, 1.5, &[[0.0, 2.0, 1.0], [2.0, 2.0, 0.0], [5.0, 1.0, 0.0], [6.0, 2.0, 0.0], [9.0, 1.0, 0.0], [10.0, 2.0, 1.0], [13.0, 3.0, 0.0]]),
];

const CELLS_12: [(&str, f64, f64, f64, Pat); 12] = [
    c("six8", -0.2, 0.2, 2.1, &[[0.0, 2.0, 1.0], [2.0, 2.0, 0.0], [4.0, 2.0, 0.0], [6.0, 2.0, 1.0], [8.0, 2.0, 0.0], [10.0, 2.0, 0.0]]),
    c("dotted12", -2.2, -0.8, 1.4, &[[0.0, 6.0, 1.0], [6.0, 6.0, 0.0]]),
    c("whole12", -3.0, -1.4, 0.8, &[[0.0, 12.0, 1.0]]),
    c("airy12", -1.6, -0.3, 1.1, &[[0.0, 6.0, 1.0], [8.0, 4.0, 0.0]]),
    c("lilt12", -0.2, 0.2, 2.0, &[[0.0, 4.0, 1.0], [4.0, 2.0, 0.0], [6.0, 3.0, 1.0], [9.0, 1.0, 0.0], [10.0, 2.0, 0.0]]),
    c("hemiola12", 1.0, 0.9, 2.0, &[[0.0, 3.0, 1.0], [3.0, 3.0, 0.0], [6.0, 3.0, 1.0], [9.0, 3.0, 0.0]]),
    c("cross12", 0.3, 0.5, 1.5, &[[0.0, 4.0, 1.0], [4.0, 4.0, 0.0], [8.0, 4.0, 0.0]]),
    c("sync12", 1.3, 1.1, 1.6, &[[0.0, 2.0, 1.0], [3.0, 3.0, 0.0], [6.0, 2.0, 1.0], [9.0, 3.0, 0.0]]),
    c("antic12", 1.2, 1.0, 1.6, &[[0.0, 4.0, 1.0], [5.0, 1.0, 0.0], [6.0, 4.0, 1.0], [11.0, 1.0, 0.0]]),
    c("shortLong12", 1.3, 1.2, 1.8, &[[0.0, 1.0, 0.0], [1.0, 1.0, 0.0], [2.0, 4.0, 1.0], [6.0, 1.0, 0.0], [7.0, 1.0, 0.0], [8.0, 4.0, 1.0]]),
    c("burst12", 2.6, 1.6, 1.3, &[[0.0, 1.0, 0.0], [1.0, 1.0, 0.0], [2.0, 1.0, 0.0], [3.0, 1.0, 0.0], [6.0, 2.0, 1.0], [9.0, 3.0, 0.0]]),
    c("skip12", 2.1, 1.5, 1.4, &[[0.0, 2.0, 1.0], [3.0, 1.0, 0.0], [4.0, 2.0, 0.0], [6.0, 2.0, 1.0], [9.0, 1.0, 0.0], [10.0, 2.0, 0.0]]),
];

const CELLS_34: [(&str, f64, f64, f64, Pat); 10] = [
    c("waltz34", -1.4, -0.4, 2.0, &[[0.0, 8.0, 1.0], [8.0, 4.0, 0.0]]),
    c("whole34", -3.0, -1.4, 0.8, &[[0.0, 12.0, 1.0]]),
    c("beats34", -0.6, -0.2, 1.8, &[[0.0, 4.0, 1.0], [4.0, 4.0, 0.0], [8.0, 4.0, 0.0]]),
    c("shortLong34", -0.2, 0.1, 1.3, &[[0.0, 4.0, 1.0], [4.0, 8.0, 0.0]]),
    c("pickup34", 0.2, 0.4, 2.2, &[[0.0, 8.0, 1.0], [8.0, 2.0, 0.0], [10.0, 2.0, 0.0]]),
    c("beatPick34", 0.6, 0.6, 1.6, &[[0.0, 4.0, 1.0], [4.0, 4.0, 0.0], [8.0, 2.0, 0.0], [10.0, 2.0, 0.0]]),
    c("lilt34", 0.3, 0.5, 2.0, &[[0.0, 6.0, 1.0], [6.0, 2.0, 0.0], [8.0, 4.0, 0.0]]),
    c("dotPick34", 0.8, 0.8, 1.6, &[[0.0, 6.0, 1.0], [6.0, 2.0, 0.0], [8.0, 2.0, 0.0], [10.0, 2.0, 0.0]]),
    c("turn34", 1.0, 0.9, 1.4, &[[0.0, 2.0, 1.0], [2.0, 2.0, 0.0], [4.0, 4.0, 0.0], [8.0, 4.0, 0.0]]),
    c("eighths34", 2.0, 1.4, 1.1, &[[0.0, 2.0, 1.0], [2.0, 2.0, 0.0], [4.0, 2.0, 0.0], [6.0, 2.0, 0.0], [8.0, 2.0, 0.0], [10.0, 2.0, 0.0]]),
];

type Pool = List<Cell, 18>;

fn cell(id: &'static str, e: f64, l: f64, w: f64, pat: &[[f64; 3]]) -> Cell {
    Cell { id, e, l, w, pat: List::from_slice([0.0; 3], pat) }
}

/// `genericCells(spb)`: the families built to fit a metre without a table
/// of its own (5/4).
fn generic_cells(spb: f64, out: &mut Pool) {
    let beat = if spb % 4.0 == 0.0 { 4.0 } else { 3.0 };
    let mut beats: List<f64, MAX_STEPS_PER_BAR> = List::filled(0.0);
    let mut i = 0.0;
    while i + beat <= spb {
        beats.push(i);
        i += beat;
    }
    let half = js::max(2.0, js::round(spb / 2.0));
    let mut dotted: List<[f64; 3], 10> = List::filled([0.0; 3]);
    let mut i = 0.0;
    while i + 3.0 <= spb {
        dotted.push([i, 3.0, if i % beat == 0.0 { 1.0 } else { 0.0 }]);
        i += 3.0;
    }
    let mut anticipated: List<[f64; 3], 10> = List::filled([0.0; 3]);
    for (k, &b) in beats.iter().enumerate() {
        if k % 2 == 1 {
            continue;
        }
        if b > 0.0 {
            anticipated.push([b - 1.0, 1.0, 0.0]);
        }
        anticipated.push([b, beat, 1.0]);
    }
    let mut pulse: List<[f64; 3], 10> = List::filled([0.0; 3]);
    for (k, &b) in beats.iter().enumerate() {
        pulse.push([b, beat, if k % 2 == 0 { 1.0 } else { 0.0 }]);
    }
    out.push(cell("pulse", -0.8, -0.6, 2.2, &pulse));
    out.push(cell("halves", -2.2, -0.8, 1.4, &[[0.0, half, 1.0], [half, spb - half, 0.0]]));
    out.push(cell("whole", -3.0, -1.4, 0.8, &[[0.0, spb, 1.0]]));
    out.push(cell("openLate", -1.8, -0.3, 1.2, &[[0.0, half, 1.0], [half + 2.0, spb - half - 2.0, 0.0]]));
    out.push(cell("dotted", 0.6, 0.6, 1.8, &dotted));
    out.push(cell("anticipate", 1.1, 1.0, 1.8, &anticipated));
    out.push(cell(
        "shortLong",
        1.3,
        1.2,
        1.8,
        &[[0.0, 1.0, 0.0], [1.0, 1.0, 0.0], [2.0, half - 2.0, 1.0], [half, 1.0, 0.0], [half + 1.0, 1.0, 0.0], [half + 2.0, spb - half - 2.0, 1.0]],
    ));
    out.push(cell("burst", 2.6, 1.7, 1.4, &[[0.0, 1.0, 0.0], [1.0, 1.0, 0.0], [2.0, 1.0, 0.0], [3.0, 1.0, 0.0], [half, 2.0, 1.0], [half + 3.0, 3.0, 0.0]]));
}

/// `cellsFor(spb, metre)`, with `usable`'s filter.
fn cells_for(spb: f64, metre: Metre) -> Pool {
    let mut pool: Pool = List::filled(cell("", 0.0, 0.0, 0.0, &[]));
    let table: &[(&str, f64, f64, f64, Pat)] = if spb == 16.0 {
        &CELLS_16
    } else if spb == 12.0 {
        if metre == Metre::ThreeFour { &CELLS_34 } else { &CELLS_12 }
    } else {
        generic_cells(spb, &mut pool);
        &[]
    };
    for &(id, e, l, w, pat) in table {
        pool.push(cell(id, e, l, w, pat));
    }
    for c in pool.iter_mut() {
        c.pat.retain(|p| p[0] >= 0.0 && p[0] < spb && p[1] >= 1.0);
    }
    pool.retain(|c| !c.pat.is_empty());
    pool
}

/// `drawCell`.
fn draw_cell(r: &mut Rng, spb: f64, metre: Metre, energy: f64, lift: f64) -> Cell {
    let pool = cells_for(spb, metre);
    let i = r.weighted_index(
        pool.iter().map(|c| c.w * math::pow(2.0, c.e * (energy - 0.5) * 2.0) * math::pow(2.0, c.l * (lift - 0.5) * 2.0)),
    );
    i.and_then(|i| pool.get(i).copied()).unwrap_or(cell("", 0.0, 0.0, 0.0, &[]))
}

// ------------------------------------------------------------ melody

/// What a developing loop's section hands its melody (`dev` in
/// `genMelody`): the figure, voice and register, and how it ends.
pub struct Handed {
    pub figure: Figure,
    pub voice: &'static str,
    pub centre: Option<f64>,
    pub open_end: bool,
    pub vel: f64,
}

/// What `genMelody` returns besides its notes.
#[derive(Clone, Copy)]
pub struct MelodyOut {
    pub voice: &'static str,
    pub motif: Figure,
    pub cell: Option<&'static str>,
    pub centre: Option<f64>,
}

/// The harmony a melody is written over (`harmony`, or a section's).
pub struct Over<'a> {
    pub slots: &'a [Slot],
    pub cycle_steps: f64,
    pub events: &'a [ChordEvent],
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Exact,
    Sequence,
    Invert,
    Retro,
    Fragment,
    Augment,
}

/// The phrase plan's transforms (`transform` in `genMelody`).
fn transform(m: &Figure, kind: Kind, amount: f64, spb: f64, waltz: bool) -> Figure {
    let mut out: Figure = List::filled(NO_MOTIF);
    let base = m.first().map_or(0.0, |x| x.degree);
    match kind {
        Kind::Sequence => {
            for x in m.iter() {
                out.push(Motif { degree: x.degree + amount, ..*x });
            }
        }
        Kind::Invert => {
            for x in m.iter() {
                out.push(Motif { degree: base - (x.degree - base), ..*x });
            }
        }
        Kind::Retro => {
            for (i, x) in m.iter().enumerate() {
                out.push(Motif { degree: m[m.len() - 1 - i].degree, ..*x });
            }
        }
        Kind::Fragment => {
            let n = js::index(js::max(2.0, js::ceil(js::num(m.len()) / 2.0))).min(m.len());
            let half = js::floor(spb / 2.0);
            for x in &m[..n] {
                out.push(*x);
            }
            for x in &m[..n] {
                if waltz && !(x.offset + half < spb) {
                    continue;
                }
                out.push(Motif { offset: js::min(spb - 1.0, x.offset + half), degree: x.degree + amount, ..*x });
            }
        }
        Kind::Augment => {
            for (i, x) in m.iter().enumerate() {
                if i % 2 == 0 {
                    out.push(Motif { dur: x.dur * 2.0, ..*x });
                }
            }
        }
        Kind::Exact => {
            for x in m.iter() {
                out.push(*x);
            }
        }
    }
    out
}

const FORMS: [[&str; 4]; 4] = [["A", "A2", "B", "A3"], ["A", "B", "A2", "C"], ["A", "A2", "A3", "B"], ["A", "B", "B2", "A2"]];

fn kind_of(role: &str) -> Kind {
    match role {
        "A2" | "A3" => Kind::Sequence,
        "B" => Kind::Invert,
        "C" => Kind::Retro,
        "B2" => Kind::Fragment,
        _ => Kind::Exact,
    }
}

fn pc(x: f64) -> f64 {
    ((x % 12.0) + 12.0) % 12.0
}

/// `genMelody(sectionSpec(spec, bars, { melody }), over, dev)`, its notes
/// into `out`.
#[allow(clippy::too_many_lines, clippy::many_single_char_names)] // one function, as genMelody is: its draws in order
pub fn gen_melody(cx: &Cx, bars: f64, seed: u32, over: &Over, dev: Option<&Handed>, out: &mut Melody) -> MelodyOut {
    let spec = cx.spec;
    let c = &cx.c;
    let mut r = Rng::new(seed);
    let spb = cx.spb;
    let scale = spec.steps();
    let total = bars * spb;
    let drawn_voice = c.melody_voices.as_ref().and_then(|p| r.weighted(p)).unwrap_or("");
    let choir = cx.choir;
    let voice = match (dev, choir) {
        (Some(d), _) => d.voice,
        (None, Some(ch)) => ch.voice,
        (None, None) => drawn_voice,
    };
    let lf = spec.feel_for(Layer::Melody);
    let mood = lf.lift;
    let silent = r.chance(0.08);
    out.clear();
    if dev.is_none() && choir.is_none() && silent {
        return MelodyOut { voice, motif: List::filled(NO_MOTIF), cell: None, centre: None };
    }
    let driven = lf.energy > 0.6;
    let pointillist = c.pointillist.unwrap_or(0.0);
    let waltz = cx.metre == Metre::ThreeFour;
    let rhythm = draw_cell(&mut r, spb, cx.metre, lf.energy, mood);

    let mut motif: Figure = List::filled(NO_MOTIF);
    let mut deg = 0.0;
    let mut last_move = 0.0;
    let up = 1.0 + mood * 2.2;
    let down = 1.0 + (1.0 - mood) * 2.2;
    let wide = if scale.len() <= 5 { 0.4 } else { 1.0 };
    for &[at, dur, accent] in rhythm.pat.iter() {
        motif.push(Motif { offset: at, degree: deg, dur, vel: 0.36 + r.f() * 0.26 + if accent == 0.0 { 0.0 } else { 0.16 } });
        let mv = if js::abs(last_move) >= 2.0 {
            let back = if last_move > 0.0 { -1.0 } else { 1.0 };
            r.weighted(&[(back, 9.0), (back * 2.0, 0.8), (0.0, 1.4)])
        } else {
            r.weighted(&[
                (0.0, 4.0),
                (1.0, 9.0 * up),
                (-1.0, 9.0 * down),
                (2.0, 0.4 * up * wide),
                (-2.0, 0.32 * down * wide),
                (3.0, 0.06 * up * wide),
                (-3.0, 0.05 * down * wide),
                (4.0, 0.02 * up * wide),
            ])
        }
        .unwrap_or(0.0);
        let before = deg;
        deg = js::max(-5.0, js::min(10.0, deg + mv));
        last_move = deg - before;
    }

    let omit = js::max(0.0, 0.2 - lf.energy * 0.15);
    let mut kept: Figure = List::filled(NO_MOTIF);
    for x in motif.iter() {
        if !r.chance(omit) {
            kept.push(*x);
        }
    }
    let figure = match dev {
        Some(d) => d.figure,
        None if kept.len() >= motif.len().min(2) => kept,
        None => motif,
    };

    let bars_per_phrase = if bars >= 8.0 {
        if r.chance(0.45) { 4.0 } else { 2.0 }
    } else {
        js::max(1.0, js::min(2.0, bars))
    };
    let phrase_count = js::max(1.0, js::round(bars / bars_per_phrase));
    let form = r.pick(&FORMS).unwrap_or(FORMS[0]);

    let mut bar_of: List<f64, MELODY_MAX> = List::filled(0.0);
    let mut centre_so_far: Option<f64> = None;
    let mut centre_count = 0.0;
    let mut centre_sum = 0.0;

    let mut chord_sum = 0.0;
    let mut chord_n = 0usize;
    for e in over.events {
        for &n in e.notes.iter() {
            chord_sum += f64::from(n);
            chord_n += 1;
        }
    }
    let unison = chord_n > 0 && choir.is_some();
    let keys_vowel = choir.and_then(|_| over.events.iter().find_map(|e| e.vowel));
    let vowel_base = keys_vowel.map_or(0, |v| (VOWEL_KEYS.iter().position(|k| *k == v).unwrap_or(0) + 2) % 4);
    if unison {
        let mut c0 = chord_sum / js::num(chord_n);
        while c0 < 55.0 {
            c0 += 12.0;
        }
        while c0 > 95.0 {
            c0 -= 12.0;
        }
        centre_so_far = Some(c0);
    }
    let rest_phrase = js::max(0.02, js::min(0.5, c.rest_bar.unwrap_or(f64::NAN) * (1.5 - lf.energy * 1.2)));

    let mut ph = 0.0;
    while ph < phrase_count {
        let role = form[js::index(ph) % 4];
        let mut kind = kind_of(role);
        let step = r.pick(&[1.0, 2.0, -1.0, 3.0, -2.0]).unwrap_or(1.0);
        if kind == Kind::Sequence && r.chance(0.25) {
            kind = Kind::Augment;
        }
        let shaped = transform(&figure, kind, step, spb, waltz);
        let len = js::num(shaped.len());
        let phrase_notes = js::max(1.0, bars_per_phrase * len);
        let breath = 0.02 + lf.energy * 0.33;
        let pos = if phrase_count > 1.0 { ph / (phrase_count - 1.0) } else { 0.0 };
        let arc = math::sin(core::f64::consts::PI * js::min(0.97, pos * 0.72 + 0.14)) * if driven { 4.5 } else { 3.2 };
        let lift = js::round(arc) + if kind == Kind::Sequence { step } else { 0.0 };

        let mut bar = 0.0;
        while bar < bars_per_phrase {
            let abs_bar = ph * bars_per_phrase + bar;
            if abs_bar >= bars {
                break;
            }
            if bar == bars_per_phrase - 1.0 && r.chance(rest_phrase) {
                bar += 1.0;
                continue;
            }
            let bar_start = abs_bar * spb;
            let slot = slot_at(over.slots, bar_start, over.cycle_steps);
            let last_bar_of_phrase = bar == bars_per_phrase - 1.0;
            let degree_of = |m: &Motif| m.degree + lift + slot.degree;
            let pitch_of = |d: f64| scale_pitch(spec.root, scale, d, 0.0) + 12.0;
            let closing = last_bar_of_phrase && ph == phrase_count - 1.0 && !dev.is_some_and(|d| d.open_end);
            let mut anchor: Option<(usize, bool)> = None;
            if closing {
                anchor = Some((shaped.len().saturating_sub(1), true));
            } else if bar == 0.0 || last_bar_of_phrase {
                anchor = Some((if bar == 0.0 { 0 } else { shaped.len().saturating_sub(1) }, false));
            }
            let mut anchor_shift = 0.0;
            if let Some((at_i, home)) = anchor
                && let Some(am) = shaped.get(at_i)
            {
                let lands = |p: f64| {
                    if home {
                        pc(p) == pc(spec.root)
                    } else {
                        slot.notes.iter().any(|&n| pc(f64::from(n)) == pc(p))
                    }
                };
                let mut sum = 0.0;
                for m in shaped.iter() {
                    sum += pitch_of(degree_of(m));
                }
                let raw_centre = sum / len;
                let aim = centre_so_far.unwrap_or(raw_centre);
                let from = degree_of(am);
                let at = pitch_of(from);
                let mut best = f64::INFINITY;
                let n = js::num(scale.len());
                let mut d = -n * 2.0;
                while d <= n * 2.0 {
                    let p = pitch_of(from + d);
                    if lands(p) {
                        let score = js::abs(raw_centre + (p - at) - aim);
                        if score < best {
                            best = score;
                            anchor_shift = d;
                        }
                    }
                    d += 1.0;
                }
            }
            for (i, m) in shaped.iter().enumerate() {
                let mut midi = pitch_of(degree_of(m) + anchor_shift);
                let pos = if phrase_notes > 1.0 { (bar * len + js::num(i)) / (phrase_notes - 1.0) } else { 0.0 };
                let arc = math::cos(pos * core::f64::consts::PI * 2.0);
                if pointillist != 0.0 && r.chance(pointillist) {
                    midi += r.pick(&[-12.0, 12.0, 12.0]).unwrap_or(0.0);
                }
                bar_of.push(abs_bar);
                centre_sum += midi;
                centre_count += 1.0;
                centre_so_far = Some(centre_sum / centre_count);
                let vowel_i = (js::index(ph) + usize::from(kind == Kind::Invert) + vowel_base) % 4;
                let vel = js::min(1.0, m.vel * (1.0 + arc * breath) * if ph == 0.0 { 1.0 } else { 0.94 } * dev.map_or(1.0, |d| d.vel));
                out.push(MelodyEvent {
                    step: ((bar_start + m.offset) % total + total) % total,
                    dur: m.dur,
                    midi,
                    vel,
                    vel0: vel,
                    voice,
                    degree: m.degree,
                    phrase: ph,
                    vowel: VOWEL_KEYS[vowel_i],
                    detune: choir.map(|ch| ch.melody_detune),
                    ..NO_NOTE
                });
            }
            bar += 1.0;
        }
        ph += 1.0;
    }

    let mut line_centre = None;
    if !out.is_empty() {
        let mut centre = if unison {
            chord_sum / js::num(chord_n)
        } else if let Some(c0) = dev.and_then(|d| d.centre) {
            c0
        } else {
            let mut s = 0.0;
            for e in out.iter() {
                s += e.midi;
            }
            s / js::num(out.len())
        };
        while centre < 55.0 {
            centre += 12.0;
        }
        while centre > 95.0 {
            centre -= 12.0;
        }
        line_centre = Some(centre);
        // One octave a bar: the bars in the order they first appear (they
        // are written in order).
        let mut i = 0;
        while i < out.len() {
            let b = bar_of[i];
            let mut j = i;
            let mut sum = 0.0;
            let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
            while j < out.len() && bar_of[j] == b {
                sum += out[j].midi;
                lo = js::min(lo, out[j].midi);
                hi = js::max(hi, out[j].midi);
                j += 1;
            }
            let mut shift = js::round((centre - sum / js::num(j - i)) / 12.0) * 12.0;
            while lo + shift < 55.0 && hi + shift + 12.0 <= 95.0 {
                shift += 12.0;
            }
            while hi + shift > 95.0 && lo + shift - 12.0 >= 55.0 {
                shift -= 12.0;
            }
            for e in &mut out[i..j] {
                let mut n = e.midi + shift;
                while n < 55.0 {
                    n += 12.0;
                }
                while n > 95.0 {
                    n -= 12.0;
                }
                e.midi = n;
            }
            i = j;
        }
    }
    MelodyOut { voice, motif: figure, cell: Some(rhythm.id), centre: line_centre }
}

/// How B's figure answers A's (`answerFigure`'s `how`).
pub fn answer_figure(cx: &Cx, figure: &Figure, r: &mut Rng, moves: bool) -> (Figure, &'static str) {
    let spb = cx.spb;
    let lf = cx.spec.feel_for(Layer::Melody);
    let how = if moves {
        r.weighted(&[("rhythm", 3.0), ("both", 2.0)])
    } else {
        r.weighted(&[("rhythm", 3.0), ("invert", 2.0), ("both", 2.0), ("slow", 1.0)])
    }
    .unwrap_or("rhythm");
    let mut out = *figure;
    if how == "invert" || how == "both" {
        let base = out.first().map_or(0.0, |x| x.degree);
        for x in out.iter_mut() {
            x.degree = base - (x.degree - base);
        }
    }
    if how == "rhythm" || how == "both" {
        let cell = draw_cell(r, spb, cx.metre, lf.energy, lf.lift);
        let prev = out;
        out.clear();
        for (i, &[at, dur, accent]) in cell.pat.iter().enumerate() {
            let degree = prev.get(i % prev.len().max(1)).map_or(f64::NAN, |x| x.degree);
            out.push(Motif { offset: at, dur, degree, vel: 0.36 + r.f() * 0.26 + if accent == 0.0 { 0.0 } else { 0.16 } });
        }
    }
    if how == "slow" && out.len() >= 2 {
        let prev = out;
        out.clear();
        for (i, x) in prev.iter().enumerate() {
            if i % 2 == 0 {
                out.push(Motif { dur: js::min(x.dur * 2.0, spb - x.offset), ..*x });
            }
        }
    }
    (out, how)
}

/// `varyFigure`: A's figure with its end turned.
fn vary_figure(figure: &Figure, r: &mut Rng) -> Figure {
    let mut out = *figure;
    let by = r.pick(&[1.0, -1.0, 2.0, -2.0]).unwrap_or(1.0);
    let from = js::index(js::max(1.0, js::ceil(js::num(out.len()) / 2.0)));
    for x in out.iter_mut().skip(from) {
        x.degree += by;
    }
    out
}

/// What `developMelody` decided: the answer's kind and B's shift (null
/// where A is silent and nothing was drawn).
#[derive(Clone, Copy)]
pub struct Depth {
    pub answer: Option<&'static str>,
    pub shift: Option<f64>,
}

/// `developMelody`: the melody of a developing loop, section by section,
/// into `out`; `a_events` and `scratch` are working room for A's notes and
/// another section's.
pub fn develop_melody(
    cx: &Cx,
    harmony: &Sections,
    dev: &Development,
    out: &mut Melody,
    a_events: &mut Melody,
    scratch: &mut Melody,
) -> (MelodyOut, Depth) {
    let spec = cx.spec;
    let spb = cx.spb;
    let melody_seed = spec.layer_seed(Layer::Melody);
    let a_bars = dev.section('A').map_or(0.0, |s| s.bars);
    let over_a = Over { slots: &harmony.a.slots, cycle_steps: harmony.a.cycle_steps, events: &harmony.a.events };
    let a = gen_melody(cx, a_bars, melody_seed, &over_a, None, a_events);
    out.clear();
    if a_events.is_empty() {
        return (a, Depth { answer: None, shift: None });
    }
    let mut r = Rng::new(melody_seed ^ B_SALT);
    let four = dev.section('V').is_some();
    let (answer, how) = answer_figure(cx, &a.motif, &mut r, four);
    let shift = if four {
        r.weighted(&[(3.0, 2.0), (5.0, 3.0), (7.0, 2.0), (-5.0, 1.0), (9.0, 1.0)])
    } else {
        r.weighted(&[(0.0, 2.0), (3.0, 2.0), (5.0, 3.0), (7.0, 2.0), (-5.0, 1.0), (9.0, 1.0)])
    }
    .unwrap_or(0.0);
    out.clear();
    let mut phrases = 0.0;
    for sec in dev.sections.iter() {
        let end = sec.bars * spb;
        let part: &Melody = match sec.role {
            'A' => &*a_events,
            role => {
                let (salt, h, handed) = match role {
                    'B' => (B_SALT, &harmony.b, Handed { figure: answer, voice: a.voice, centre: a.centre.map(|c| c + shift), open_end: true, vel: 1.06 }),
                    'V' => (VARY_SALT, &harmony.v, Handed { figure: vary_figure(&a.motif, &mut Rng::new(melody_seed ^ VARY_SALT)), voice: a.voice, centre: a.centre, open_end: true, vel: 1.0 }),
                    _ => {
                        let figure = if four { vary_figure(&a.motif, &mut Rng::new(melody_seed ^ RETURN_SALT)) } else { a.motif };
                        (RETURN_SALT, if harmony.has_r { &harmony.r } else { &harmony.a }, Handed { figure, voice: a.voice, centre: a.centre, open_end: false, vel: 1.0 })
                    }
                };
                // B plays over its own harmony as drawn; the others over
                // theirs cut to the section.
                let n = if role == 'B' { h.slots.len() } else { h.slots.iter().filter(|s| s.start_step < end).count() };
                let slots: &[Slot] = if role == 'B' { &h.slots } else { &h.slots[..n] };
                let cycle = if role == 'B' { h.cycle_steps } else { end };
                let over = Over { slots, cycle_steps: cycle, events: &h.events };
                gen_melody(cx, sec.bars, melody_seed ^ salt, &over, Some(&handed), scratch);
                &*scratch
            }
        };
        let mut top = 0.0;
        for e in part.iter() {
            if e.step >= end {
                continue;
            }
            out.push(MelodyEvent { step: e.step + sec.at * spb, phrase: e.phrase + phrases, ..*e });
            top = js::max(top, e.phrase + 1.0);
        }
        phrases += top;
    }
    (MelodyOut { voice: a.voice, motif: a.motif, cell: a.cell, centre: a.centre }, Depth { answer: Some(how), shift: Some(shift) })
}
