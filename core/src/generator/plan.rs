//! What `render` plans before any layer plays (queue item 79): whether
//! the loop develops and its sections (`developmentOf`), the harmony
//! (`genHarmony`, its long chords, `developHarmony`), the entry schedules
//! (`genForm`) and the short gaps (`genGaps`), draw for draw as
//! `js/generator.js` makes them. The goldens hash it as `plan`
//! (`planOf`).

use super::canon::{Sink, Writer};
use super::character::Char;
use super::data::{self, Degree, MAX_BARS, MAX_CHORD_SIZE, Progressions};
use super::js;
use super::list::List;
use super::rng::Rng;
use super::spec::{Choir, Metre, Spec};
use super::theory::{self, scale_pitch};
use super::{LAYERS, Layer};

/// The notes a chord holds at most: the largest chord size and its ninth.
pub const NOTES_MAX: usize = MAX_CHORD_SIZE + 1;
/// The slots a harmony holds at most: two chords a bar.
pub const SLOTS_MAX: usize = MAX_BARS * 2;
/// The chord events a harmony holds at most: the most hits a bar's rhythm
/// has (five, 5/4's offbeat), each an arpeggio of every note.
pub const CHORD_EVENTS_MAX: usize = MAX_BARS * 5 * NOTES_MAX;

/// The sung vowels (`VOWEL_KEYS`).
pub const VOWEL_KEYS: [&str; 4] = ["a", "e", "o", "u"];

/// A chord's notes (MIDI numbers, whole, as the JavaScript's are).
pub type Notes = List<i16, NOTES_MAX>;

/// A chord slot of the progression.
#[derive(Clone, Copy)]
pub struct Slot {
    pub start_step: f64,
    pub length_steps: f64,
    pub degree: f64,
    pub notes: Notes,
    pub root_midi: f64,
    pub bass_degree: f64,
    pub bass_midi: f64,
    /// A spelled chord's scale (its length is the mode's).
    pub steps: Option<List<f64, 7>>,
}

/// A chord as the chords layer plays it.
#[derive(Clone, Copy)]
pub struct ChordEvent {
    pub step: f64,
    pub dur: f64,
    pub notes: Notes,
    pub vel: f64,
    pub voice: &'static str,
    pub vowel: Option<&'static str>,
    pub detune: Option<f64>,
    /// A nylon guitar's strum ('down' or 'up'), written by render.
    pub strum: Option<&'static str>,
    /// In the chords layer: the velocity as generated (a repair puts it
    /// back), and whether the event is still the harmony's own (the same
    /// object in the JavaScript, so what render writes on it, a strum,
    /// shows in the harmony too).
    pub vel0: f64,
    pub alias: bool,
}

/// A harmony: `genHarmony`'s return, or (with `sections`) a developing
/// loop's, `developHarmony`'s.
#[derive(Clone, Copy)]
pub struct Harmony {
    pub slots: List<Slot, SLOTS_MAX>,
    pub events: List<ChordEvent, CHORD_EVENTS_MAX>,
    pub shape: List<Degree, 8>,
    pub cycle_steps: f64,
    /// The long chords' kind; `None` on a developing loop's whole harmony
    /// (which has no such key).
    pub long_chords: Option<&'static str>,
}

const EMPTY_SLOT: Slot = Slot {
    start_step: 0.0,
    length_steps: 0.0,
    degree: 0.0,
    notes: List::filled(0),
    root_midi: 0.0,
    bass_degree: 0.0,
    bass_midi: 0.0,
    steps: None,
};
/// An empty chord event.
pub const EMPTY_EVENT: ChordEvent =
    ChordEvent { step: 0.0, dur: 0.0, notes: List::filled(0), vel: 0.0, voice: "", vowel: None, detune: None, strum: None, vel0: 0.0, alias: false };

impl Harmony {
    /// An empty harmony.
    #[must_use]
    pub const fn new() -> Self {
        Harmony {
            slots: List::filled(EMPTY_SLOT),
            events: List::filled(EMPTY_EVENT),
            shape: List::filled(Degree::plain(0)),
            cycle_steps: 0.0,
            long_chords: None,
        }
    }

    /// `slotAt`: the slot sounding at `step`, wrapping on the cycle.
    #[must_use]
    pub fn slot_at(&self, step: f64) -> &Slot {
        let c = self.cycle_steps;
        let q = if c == 0.0 { step } else { ((step % c) + c) % c };
        for s in self.slots.iter().rev() {
            if q >= s.start_step {
                return s;
            }
        }
        self.slots.first().unwrap_or(&EMPTY_SLOT)
    }

    /// How many items its lists dropped (0).
    #[must_use]
    pub fn lost(&self) -> usize {
        self.slots.lost() + self.events.lost() + self.shape.lost()
    }
}

impl Default for Harmony {
    fn default() -> Self {
        Self::new()
    }
}

/// `slotAt(slots, step, cycleSteps)`.
#[must_use]
pub fn slot_at(slots: &[Slot], step: f64, cycle: f64) -> &Slot {
    let q = if cycle == 0.0 { step } else { ((step % cycle) + cycle) % cycle };
    for s in slots.iter().rev() {
        if q >= s.start_step {
            return s;
        }
    }
    slots.first().unwrap_or(&EMPTY_SLOT)
}

/// A section of a developing loop.
#[derive(Clone, Copy, Debug)]
pub struct Section {
    /// 'A', 'B', 'V' or 'R'.
    pub role: char,
    pub bars: f64,
    pub at: f64,
}

/// A developing loop's sections (`developmentOf`).
#[derive(Clone, Copy)]
pub struct Development {
    pub sections: List<Section, 4>,
}

impl Development {
    /// The section of a role.
    #[must_use]
    pub fn section(&self, role: char) -> Option<&Section> {
        self.sections.iter().find(|s| s.role == role)
    }
}

/// A layer's entry schedule: which of its bars it plays.
pub type Schedule = List<bool, MAX_BARS>;

/// A short gap: steps of nothing.
#[derive(Clone, Copy, Debug)]
pub struct Gap {
    pub start: f64,
    pub len: f64,
}

/// What the loop knows before the layers, that every part reads.
pub struct Cx<'a> {
    pub spec: &'a Spec,
    pub c: Char,
    pub choir: Option<Choir>,
    pub metre: Metre,
    pub spb: f64,
    /// Set while render works out the octave today's chord lengths would
    /// take (`longChordsOff`).
    pub long_chords_off: bool,
}

impl<'a> Cx<'a> {
    /// The context of a spec.
    #[must_use]
    pub fn new(spec: &'a Spec) -> Self {
        let c = spec.character();
        let metre = spec.metre(&c);
        Cx { spec, c, choir: spec.choir(), metre, spb: spec.steps_per_bar, long_chords_off: false }
    }
}

// ------------------------------------------------------------ harmony

const fn shape(d: [i32; 4]) -> [Degree; 4] {
    [Degree::plain(d[0]), Degree::plain(d[1]), Degree::plain(d[2]), Degree::plain(d[3])]
}

const SHAPES_7: [[Degree; 4]; 12] = [
    shape([0, 5, 3, 4]),
    shape([0, 3, 5, 4]),
    shape([0, 4, 5, 3]),
    shape([0, 6, 3, 4]),
    shape([0, 2, 5, 4]),
    shape([5, 3, 0, 4]),
    shape([0, 0, 3, 4]),
    shape([1, 4, 0, 0]),
    shape([0, 5, 1, 4]),
    shape([3, 4, 0, 5]),
    shape([0, 1, 3, 4]),
    shape([0, 3, 0, 4]),
];
const SHAPES_PENT: [[Degree; 4]; 6] =
    [shape([0, 3, 4, 2]), shape([0, 2, 4, 3]), shape([0, 4, 0, 2]), shape([0, 1, 3, 4]), shape([0, 3, 0, 4]), shape([0, 2, 3, 1])];

/// `fifthAbove`.
fn fifth_above(steps: &[f64], degree: i32) -> f64 {
    let n = steps.len();
    #[allow(clippy::cast_sign_loss)] // degrees here are not negative
    let d = degree as usize;
    (steps[(d + 4) % n] - steps[d % n] + 12.0) % 12.0
}

/// `spelledSteps`: the mode with a spelled chord's root and third moved.
fn spelled_steps(scale: &[f64], step: Degree) -> List<f64, 7> {
    let n = scale.len();
    let mut steps = List::from_slice(0.0, scale);
    #[allow(clippy::cast_sign_loss)] // a written degree, not negative
    let d = (step.d as usize) % n;
    if step.flat {
        steps[d] = (steps[d] + 11.0) % 12.0;
    }
    if step.major {
        steps[(d + 2) % n] = (steps[d] + 4.0) % 12.0;
    }
    steps
}

/// `plainTriad`.
fn plain_triad(scale: &[f64], step: Degree) -> bool {
    if !step.spelled {
        return fifth_above(scale, step.d) == 7.0;
    }
    fifth_above(&spelled_steps(scale, step), step.d) == 7.0
}

/// The degrees of a progression from `half` on (none past its end).
fn tail(d: &[Degree], half: usize) -> &[Degree] {
    d.get(half..).unwrap_or(&[])
}

/// `a.join() === b.join()` for two progressions: a spelled chord joins as
/// "[object Object]", so any two spelled chords compare equal.
fn same_shape(a: &[Degree], b: &[Degree]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.spelled == y.spelled && (x.spelled || x.d == y.d))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Rhythm {
    Pad,
    Breathe,
    TwoAndFour,
    Pushed,
    Offbeat,
    LateBloom,
    Stutter,
}

/// `CHORD_RHYTHMS`: a bar's hits, [start, length], by metre and name.
fn chord_rhythm(metre: Metre, name: Rhythm) -> &'static [[f64; 2]] {
    use Rhythm::{Breathe, LateBloom, Offbeat, Pad, Pushed, Stutter, TwoAndFour};
    match metre {
        Metre::FourFour => match name {
            Pad => &[[0.0, 16.0]],
            Breathe => &[[0.0, 12.0]],
            TwoAndFour => &[[4.0, 3.0], [12.0, 3.0]],
            Pushed => &[[0.0, 6.0], [6.0, 4.0], [12.0, 3.0]],
            Offbeat => &[[2.0, 2.0], [6.0, 2.0], [10.0, 2.0], [14.0, 2.0]],
            LateBloom => &[[6.0, 10.0]],
            Stutter => &[[0.0, 2.0], [3.0, 2.0], [8.0, 3.0]],
        },
        Metre::SixEight => match name {
            Pad | Breathe => &[[0.0, 12.0]],
            TwoAndFour => &[[4.0, 3.0]],
            Pushed => &[[0.0, 6.0], [6.0, 4.0]],
            Offbeat => &[[2.0, 2.0], [4.0, 2.0], [8.0, 2.0], [10.0, 2.0]],
            LateBloom => &[[6.0, 6.0]],
            Stutter => &[[0.0, 2.0], [3.0, 2.0], [6.0, 3.0]],
        },
        Metre::ThreeFour => match name {
            Pad | Breathe => &[[4.0, 4.0], [8.0, 4.0]],
            TwoAndFour => &[[4.0, 3.0], [8.0, 3.0]],
            Pushed => &[[4.0, 4.0], [8.0, 2.0], [10.0, 2.0]],
            Offbeat => &[[4.0, 2.0], [6.0, 2.0], [8.0, 2.0], [10.0, 2.0]],
            LateBloom => &[[4.0, 8.0]],
            Stutter => &[[4.0, 2.0], [6.0, 2.0], [8.0, 3.0]],
        },
        Metre::FiveFour => match name {
            Pad => &[[0.0, 20.0]],
            Breathe => &[[0.0, 12.0], [12.0, 8.0]],
            TwoAndFour => &[[4.0, 3.0], [16.0, 3.0]],
            Pushed => &[[0.0, 8.0], [8.0, 4.0], [12.0, 8.0]],
            Offbeat => &[[2.0, 2.0], [6.0, 2.0], [10.0, 2.0], [14.0, 2.0], [18.0, 2.0]],
            LateBloom => &[[12.0, 8.0]],
            Stutter => &[[0.0, 2.0], [3.0, 2.0], [12.0, 3.0]],
        },
    }
}

const RHYTHMS: [(Rhythm, f64); 7] = [
    (Rhythm::Pad, 3.0),
    (Rhythm::Breathe, 2.0),
    (Rhythm::TwoAndFour, 2.5),
    (Rhythm::Pushed, 2.0),
    (Rhythm::Offbeat, 1.4),
    (Rhythm::LateBloom, 1.2),
    (Rhythm::Stutter, 0.8),
];

const LONG_SALT: u32 = 0x10c4;
const CALM_VIBES: [&str; 7] = ["floating", "meditative", "hushed", "misty", "foggy", "hypnotic", "still"];
const SUSTAINED_CHORDS: [&str; 5] = ["pad", "softpad", "moogpad", "analogpad", "choir"];

/// A loop's long chords (`longChordsOf`): the kind, the progression's
/// chord for each slot, the held chords' trim.
struct LongChords {
    kind: &'static str,
    seq: List<Degree, SLOTS_MAX>,
    trim: f64,
}

fn long_chords_of(cx: &Cx, chords_seed: u32, shape: &[Degree], chords_per_bar: f64, slot_count: f64) -> Option<LongChords> {
    if cx.long_chords_off {
        return None;
    }
    let spec = cx.spec;
    let spb = cx.spb;
    let mut lr = Rng::new(js::salted(chords_seed, LONG_SALT, 1));
    let slot_sec = (spb * 60.0 / spec.bpm / 4.0) / chords_per_bar;
    let calm = spec.feel.energy < 0.35 || CALM_VIBES.iter().any(|v| spec.vibe_over(v, 0.25));
    let w = if calm {
        1.8
    } else if spec.feel.energy > 0.65 {
        0.5
    } else {
        1.0
    };
    let kind =
        lr.weighted(&[("asIs", 56.0), ("double", 16.0), ("eight", 10.0 * w), ("long", 9.0 * w), ("whole", 9.0 * w)]).unwrap_or("asIs");
    let target = match kind {
        "asIs" => slot_sec,
        "double" => 2.0 * slot_sec,
        "eight" => 8.0,
        "long" => 16.0,
        _ => f64::INFINITY,
    };
    let mut hold = if kind == "whole" { slot_count } else { js::max(1.0, js::round(target / slot_sec)) };
    if kind != "long" && kind != "whole" {
        hold = js::min(hold, js::max(1.0, js::floor(slot_count / 2.0)));
    }
    let turnaround = kind != "whole" && hold >= 2.0 && lr.chance(0.3);
    let mut seq = List::filled(Degree::plain(0));
    let n = shape.len();
    let mut k = 0;
    while js::num(seq.len()) < slot_count && n > 0 {
        let last = k % n == n - 1;
        let len = if turnaround && last { js::max(1.0, js::floor(hold / 2.0)) } else { hold };
        let mut j = 0.0;
        while j < len && js::num(seq.len()) < slot_count {
            seq.push(shape[k % n]);
            j += 1.0;
        }
        k += 1;
    }
    let trim = match kind {
        "asIs" => 1.0,
        "double" => 0.92,
        "eight" => 0.86,
        _ => 0.8,
    };
    Some(LongChords { kind, seq, trim })
}

/// `voiceInRange`, in place.
fn voice_in_range(notes: &mut Notes, low: i16, high: i16) {
    for n in notes.iter_mut() {
        while *n < low {
            *n += 12;
        }
        while *n > high {
            *n -= 12;
        }
    }
}

fn mean(notes: &[i16]) -> f64 {
    let mut s = 0.0;
    for &n in notes {
        s += f64::from(n);
    }
    s / js::num(notes.len())
}

#[allow(clippy::cast_possible_truncation)] // MIDI notes, whole and small
fn note(x: f64) -> i16 {
    x as i16
}

/// `genHarmony(sectionSpec(spec, bars, { chords }), vary, besides)`.
// One function, as genHarmony is, its draws in order and its short names.
#[allow(clippy::too_many_lines, clippy::many_single_char_names)]
pub fn gen_harmony(cx: &Cx, out: &mut Harmony, bars: f64, chords_seed: u32, vary: Option<&mut Rng>, besides: Option<&[Degree]>) {
    let spec = cx.spec;
    let c = &cx.c;
    let mut r = Rng::new(chords_seed);
    let spb = cx.spb;
    let scale = spec.steps();
    let n = scale.len();
    let pentatonic = n <= 5;
    let written: Option<&'static [&'static [Degree]]> = match c.progressions {
        Some(Progressions::List(l)) => Some(l),
        Some(Progressions::PerMode(m)) => m.iter().find(|(k, _)| *k == theory::SCALES[spec.scale].id).map(|x| x.1),
        None => None,
    };
    let mut fitting: List<&'static [Degree], 16> = List::filled(&[]);
    if n == 7
        && let Some(written) = written
    {
        for p in written {
            if p.iter().all(|st| plain_triad(scale, *st)) {
                fitting.push(p);
            }
        }
    }
    let own = !fitting.is_empty();
    let per_mode = own && matches!(c.progressions, Some(Progressions::PerMode(_)));
    let mut pool: List<&[Degree], 16> = List::filled(&[]);
    if pentatonic {
        for s in &SHAPES_PENT {
            pool.push(s);
        }
    } else if own {
        for p in fitting.iter() {
            pool.push(p);
        }
    } else {
        for s in &SHAPES_7 {
            pool.push(s);
        }
    }
    let mut shape: List<Degree, 8> = List::from_slice(Degree::plain(0), r.pick(&pool).unwrap_or(&[]));

    if r.chance(0.35) {
        let i = js::index(r.int(1.0, js::num(shape.len()) - 1.0));
        let by = r.pick(&[-1.0, 1.0, 2.0]).unwrap_or(0.0);
        let nn = js::num(n);
        let at = shape.get(i).map_or(f64::NAN, |d| f64::from(d.d));
        let moved = (at + by + nn) % nn;
        if !own && let Some(s) = shape.get_mut(i) {
            #[allow(clippy::cast_possible_truncation)] // a degree
            {
                *s = Degree::plain(moved as i32);
            }
        }
    }

    if let Some(fs) = c.flat_seven
        && fs != 0.0
        && !pentatonic
        && r.chance(fs)
        && !own
        && let Some(last) = shape.last_mut()
    {
        *last = Degree::plain(6);
    }

    if let Some(vary) = vary
        && !own
    {
        let half = shape.len() / 2;
        let mut ends: List<&[Degree], 16> = List::filled(&[]);
        for sh in pool.iter() {
            let differs = !same_shape(tail(sh, half), tail(&shape, half));
            let not_besides = !besides.is_some_and(|b| same_shape(tail(sh, half), tail(b, half)));
            if differs && not_besides {
                ends.push(sh);
            }
        }
        if !ends.is_empty() {
            let end = vary.pick(&ends).unwrap_or(&[]);
            shape.truncate(half);
            for d in tail(end, half) {
                shape.push(*d);
            }
        }
    }

    let chords_per_bar = if r.chance(0.18) && spb % 2.0 == 0.0 && cx.metre != Metre::ThreeFour { 2.0 } else { 1.0 };
    let slot_len = js::floor(spb / chords_per_bar);
    let slot_count = bars * chords_per_bar;
    let total = bars * spb;

    let size = c.chord_size.as_ref().and_then(|p| r.weighted(p)).unwrap_or(f64::NAN);
    let voice = c.chord_voices.as_ref().and_then(|p| r.weighted(p)).unwrap_or("");
    let quartal = match c.quartal {
        Some(q) if q != 0.0 => r.chance(q),
        _ => false,
    };
    let rhythm = r.weighted(&RHYTHMS).unwrap_or(Rhythm::Pad);
    let arpeggiate = r.chance(0.22);
    let slash_chance = 0.12;

    out.slots.clear();
    out.events.clear();
    let mut prev: Option<Notes> = None;

    let lc = long_chords_of(cx, chords_seed, &shape, chords_per_bar, slot_count);
    let choir = cx.choir;
    let mood = spec.feel_for(Layer::Chords).lift;

    let mut s = 0.0;
    while s < slot_count {
        let si = js::index(s);
        let chord = match &lc {
            Some(lc) => lc.seq.get(si).copied().unwrap_or(Degree::plain(0)),
            None => shape.get(si % shape.len().max(1)).copied().unwrap_or(Degree::plain(0)),
        };
        let degree = f64::from(chord.d);
        let spelled = chord.spelled.then(|| spelled_steps(scale, chord));
        let steps: &[f64] = spelled.as_ref().map_or(scale, |x| &x[..]);
        let mut notes: Notes = List::filled(0);
        if quartal {
            for step in [0.0, 3.0, 6.0, 9.0].iter().take(js::index(size).min(4)) {
                notes.push(note(scale_pitch(spec.root, steps, degree + step, 0.0)));
            }
        } else {
            let mut i = 0.0;
            while i < size {
                notes.push(note(scale_pitch(spec.root, steps, degree + i * 2.0, 0.0)));
                i += 1.0;
            }
        }
        if !pentatonic && r.chance(0.18 + mood * 0.3) {
            let ninth = scale_pitch(spec.root, steps, degree + 8.0, 0.0);
            if !per_mode || (ninth - scale_pitch(spec.root, steps, degree, 0.0)) % 12.0 != 3.0 {
                notes.push(note(ninth));
            }
        }
        voice_in_range(&mut notes, 55, 79);
        if let Some(p) = &prev {
            let here = mean(&notes);
            let there = mean(p);
            let shift = js::max(-1.0, js::min(1.0, js::round((there - here) / 12.0)));
            if shift != 0.0 {
                for x in notes.iter_mut() {
                    *x += note(shift * 12.0);
                }
            }
        }
        voice_in_range(&mut notes, 52, 84);
        let mut uniq: Notes = List::filled(0);
        for &x in notes.iter() {
            if !uniq.contains(&x) {
                uniq.push(x);
            }
        }
        uniq.sort_unstable();
        let notes = uniq;
        prev = Some(notes);

        let start_step = s * slot_len;
        let vowel = VOWEL_KEYS.get(js::index(js::floor(s / (1.0 + r.int(0.0, 2.0))) % 4.0)).copied().unwrap_or("a");
        let root_midi = scale_pitch(spec.root, steps, degree, 0.0);
        let pedal_degree = if r.chance(slash_chance) { degree + r.pick(&[1.0, 2.0, 4.0, 6.0]).unwrap_or(0.0) } else { degree };
        out.slots.push(Slot {
            start_step,
            length_steps: slot_len,
            degree,
            notes,
            root_midi,
            bass_degree: pedal_degree,
            bass_midi: scale_pitch(spec.root, steps, pedal_degree, 0.0),
            steps: spelled,
        });

        let mut hits: List<[f64; 2], 5> = List::filled([0.0; 2]);
        if chords_per_bar == 2.0 {
            let both = [[0.0, slot_len], [4.0, slot_len - 4.0]];
            let k = if r.chance(0.5) { 1 } else { 2 };
            for h in &both[..k] {
                hits.push(*h);
            }
        } else {
            for h in chord_rhythm(cx.metre, rhythm) {
                if h[0] < spb {
                    hits.push(*h);
                }
            }
        }
        for &[hit, dur] in hits.iter() {
            let step = start_step + hit;
            if step >= total {
                continue;
            }
            if r.chance(0.08) {
                continue;
            }
            if arpeggiate {
                let slot_end = start_step + slot_len;
                for (i, &nn) in notes.iter().enumerate() {
                    let at = step + js::num(i) * 2.0;
                    let full = js::max(2.0, dur);
                    let mut one: Notes = List::filled(0);
                    one.push(nn);
                    out.events.push(ChordEvent {
                        step: at % total,
                        dur: if at < slot_end { js::min(full, slot_end - at) } else { js::min(full, 2.0) },
                        notes: one,
                        vel: 0.45 + r.f() * 0.2,
                        voice: choir.map_or("keys", |ch| ch.voice),
                        vowel: choir.map(|_| vowel),
                        detune: choir.map(|ch| ch.keys_detune),
                        ..EMPTY_EVENT
                    });
                }
            } else {
                let drawn = if voice == "both" { if r.chance(0.5) { "keys" } else { "pad" } } else { voice };
                out.events.push(ChordEvent {
                    step,
                    dur,
                    notes,
                    vel: 0.4 + r.f() * 0.25,
                    voice: choir.map_or(drawn, |ch| ch.voice),
                    vowel: Some(vowel),
                    detune: choir.map(|ch| ch.keys_detune),
                    ..EMPTY_EVENT
                });
            }
        }
        s += 1.0;
    }

    if let Some(lc) = &lc {
        if lc.trim != 1.0 {
            for e in out.events.iter_mut() {
                e.vel *= lc.trim;
            }
        }
        // Sorted by step (stable), then a held pad tied over its repeats.
        out.events.sort_stable_by(|a, b| a.step < b.step);
        let mut tied = 0;
        let mut last = 0.0;
        for i in 0..out.events.len() {
            let e = out.events[i];
            if tied > 0 {
                let p = &mut out.events[tied - 1];
                if SUSTAINED_CHORDS.contains(&e.voice)
                    && p.voice == e.voice
                    && p.notes.len() > 1
                    && p.notes[..] == e.notes[..]
                    && e.step - last <= slot_len
                {
                    p.dur = js::max(p.dur, e.step + e.dur - p.step);
                    last = e.step;
                    continue;
                }
            }
            out.events[tied] = e;
            tied += 1;
            last = e.step;
        }
        out.events.truncate(tied);
    }
    out.shape = shape;
    out.cycle_steps = total;
    out.long_chords = Some(lc.as_ref().map_or("asIs", |l| l.kind));
}

// -------------------------------------------------------- development

const DEPTH_SALT: u32 = 0x4445_5054;
/// The salt of B's own streams.
pub const B_SALT: u32 = 0x4253_4543;
/// The salt of the return's own streams.
pub const RETURN_SALT: u32 = 0x5245_5455;
/// The salt of the varied statement's own streams.
pub const VARY_SALT: u32 = 0x5641_5259;

/// `developmentOf`: whether the loop develops, and its sections.
#[must_use]
pub fn development_of(spec: &Spec, choir: Option<Choir>) -> Option<Development> {
    let share = match spec.bars {
        8.0 => 0.3,
        16.0 => 0.5,
        24.0 | 32.0 => 0.72,
        _ => return None,
    };
    if spec.cycles.is_some() || choir.is_some() {
        return None;
    }
    // leadKey: the first of the heaviest profiles in the mix.
    let mut lead = None::<(usize, f64)>;
    for &(k, w) in spec.mix.iter() {
        if lead.is_none_or(|(_, lw)| w > lw) {
            lead = Some((k, w));
        }
    }
    let lead = data::resolve_key(lead.map_or("dust", |(k, _)| data::CHARACTERS[k].id));
    let weight = if matches!(data::CHARACTERS[lead].id, "vapor" | "undertow") { 0.0 } else { 1.0 };
    let mut r = Rng::new(spec.seed ^ DEPTH_SALT);
    if !(r.f() < share * 1.0 * weight) {
        return None;
    }
    let form: &[(char, f64)] = match spec.bars {
        8.0 => &[('A', 4.0), ('B', 2.0), ('R', 2.0)],
        16.0 => &[('A', 4.0), ('V', 4.0), ('B', 4.0), ('R', 4.0)],
        24.0 => &[('A', 8.0), ('B', 8.0), ('R', 8.0)],
        _ => &[('A', 16.0), ('B', 8.0), ('R', 8.0)],
    };
    let mut sections = List::filled(Section { role: 'A', bars: 0.0, at: 0.0 });
    let mut at = 0.0;
    for &(role, bars) in form {
        sections.push(Section { role, bars, at });
        at += bars;
    }
    Some(Development { sections })
}

/// `placeHarmony`: the part of `h` in its first `bars` bars, moved to bar
/// `at`, appended to `out`.
fn place_harmony(h: &Harmony, bars: f64, at: f64, spb: f64, out: &mut Harmony) {
    let end = bars * spb;
    let shift = at * spb;
    for sl in h.slots.iter().filter(|sl| sl.start_step < end) {
        let mut x = *sl;
        x.start_step = sl.start_step + shift;
        x.length_steps = js::min(sl.length_steps, end - sl.start_step);
        out.slots.push(x);
    }
    for e in h.events.iter().filter(|e| e.step < end) {
        let mut x = *e;
        x.step = e.step + shift;
        x.dur = js::min(e.dur, end - e.step);
        out.events.push(x);
    }
}

/// A developing loop's section harmonies (`sectionHarmony`): A, B, and
/// the varied statement (V) and the return (R) where the form has them
/// (`has_v`, `has_r`; null otherwise). Each is generated in place, so no
/// harmony is ever built on the stack.
pub struct Sections {
    pub a: Harmony,
    pub b: Harmony,
    pub v: Harmony,
    pub r: Harmony,
    pub has_v: bool,
    pub has_r: bool,
}

impl Sections {
    /// Empty.
    #[must_use]
    pub const fn new() -> Self {
        Sections { a: Harmony::new(), b: Harmony::new(), v: Harmony::new(), r: Harmony::new(), has_v: false, has_r: false }
    }

    /// V, if the form has it.
    #[must_use]
    pub fn v(&self) -> Option<&Harmony> {
        self.has_v.then_some(&self.v)
    }

    /// R, if the form has it.
    #[must_use]
    pub fn r(&self) -> Option<&Harmony> {
        self.has_r.then_some(&self.r)
    }
}

impl Default for Sections {
    fn default() -> Self {
        Self::new()
    }
}

/// `developHarmony`: A's progression for every A and the return, B's own
/// for B, placed across the whole form into `out`; the sections' own
/// harmonies into `parts`.
pub fn develop_harmony(cx: &Cx, dev: &Development, out: &mut Harmony, parts: &mut Sections) {
    let spec = cx.spec;
    let spb = cx.spb;
    let chords = spec.layer_seed(Layer::Chords);
    let a_bars = dev.section('A').map_or(0.0, |s| s.bars);
    let b_bars = dev.section('B').map_or(0.0, |s| s.bars);
    gen_harmony(cx, &mut parts.a, a_bars, chords, None, None);
    parts.has_v = dev.section('V').is_some();
    if parts.has_v {
        gen_harmony(cx, &mut parts.v, a_bars, chords, Some(&mut Rng::new(chords ^ VARY_SALT)), None);
    }
    for k in 0..4u32 {
        gen_harmony(cx, &mut parts.b, b_bars, chords ^ B_SALT.wrapping_add(k), None, None);
        let same = same_shape(&parts.b.shape, &parts.a.shape);
        if !same && parts.b.slots.first().map(|s| s.degree) != parts.a.slots.first().map(|s| s.degree) {
            break;
        }
    }
    // The same instrument all the way through: A's commonest voice (the
    // first of the commonest).
    let mut best: Option<(&'static str, usize)> = None;
    for e in parts.a.events.iter() {
        let n = parts.a.events.iter().filter(|x| x.voice == e.voice).count();
        if best.is_none_or(|(_, bn)| n > bn) {
            best = Some((e.voice, n));
        }
    }
    if let Some((voice, _)) = best {
        for e in parts.b.events.iter_mut() {
            e.voice = voice;
        }
    }
    parts.has_r = parts.has_v;
    if parts.has_r {
        let besides = parts.v.shape;
        gen_harmony(cx, &mut parts.r, a_bars, chords, Some(&mut Rng::new(chords ^ RETURN_SALT)), Some(&besides));
    }
    out.slots.clear();
    out.events.clear();
    for s in dev.sections.iter() {
        let h = match s.role {
            'B' => &parts.b,
            'V' => parts.v().unwrap_or(&parts.a),
            'R' => parts.r().unwrap_or(&parts.a),
            _ => &parts.a,
        };
        place_harmony(h, s.bars, s.at, spb, out);
    }
    out.shape = parts.a.shape;
    out.cycle_steps = spec.bars * spb;
    out.long_chords = None;
}

// --------------------------------------------------------------- form

/// `FORM_RUNS`: each layer's in-run and out-run lengths, in bars.
const FORM_RUNS: [[[f64; 2]; 2]; 5] =
    [[[4.0, 12.0], [2.0, 6.0]], [[4.0, 10.0], [2.0, 6.0]], [[4.0, 12.0], [2.0, 5.0]], [[2.0, 6.0], [3.0, 9.0]], [[2.0, 6.0], [3.0, 10.0]]];

fn run_schedule(r: &mut Rng, bars: f64, inr: [f64; 2], outr: [f64; 2], start_in: bool) -> Schedule {
    let mut out: Schedule = List::filled(false);
    let mut i = 0.0;
    while i < bars {
        out.push(false);
        i += 1.0;
    }
    let mut i = 0;
    let mut on = start_in;
    let n = out.len();
    while i < n {
        let raw = if on { r.int(inr[0], inr[1]) } else { r.int(outr[0], outr[1]) };
        let len = if on { js::max(2.0, js::round(raw / 2.0) * 2.0) } else { js::max(1.0, raw) };
        let mut k = 0.0;
        while k < len && i < n {
            out[i] = on;
            k += 1.0;
            i += 1;
        }
        on = !on;
    }
    if !out.iter().any(|&x| x) {
        for x in out.iter_mut().take(n.min(4)) {
            *x = true;
        }
    }
    out
}

/// A loop's entry schedules (`genForm`), by layer; `None` for a loop
/// without them.
pub type Form = [Schedule; 5];

/// `localBars`: a layer's own cycle, in bars.
#[must_use]
pub fn local_bars(spec: &Spec, layer: Layer) -> f64 {
    let cycle = spec.cycles.and_then(|c| c[layer as usize]).unwrap_or(spec.bars * spec.steps_per_bar);
    js::max(1.0, js::round(cycle / spec.steps_per_bar))
}

/// `genForm`.
#[must_use]
pub fn gen_form(cx: &Cx) -> Option<Form> {
    let spec = cx.spec;
    if spec.bars < 8.0 {
        return None;
    }
    let mut r = Rng::new(js::salted(spec.seed, 0x5f37_59df, 7));
    if !r.chance(cx.c.airy.unwrap_or(0.25)) {
        return None;
    }
    let mut form: Form = [List::filled(false); 5];
    for layer in LAYERS {
        let bars = local_bars(spec, layer);
        let [inr, outr] = FORM_RUNS[layer as usize];
        let start = r.chance(0.7);
        form[layer as usize] = run_schedule(&mut r, bars, inr, outr, start);
    }
    if silent_bars(&form, spec.bars).is_empty() {
        let at = js::min(spec.bars - 2.0, js::max(4.0, js::round(spec.bars / 2.0 / 4.0) * 4.0));
        let span = if r.chance(0.5) { 2 } else { 1 };
        for sched in &mut form {
            let len = sched.len();
            for k in 0..span {
                let idx = (js::index(at) + k) % len.max(1);
                if idx >= len {
                    continue;
                }
                sched[idx] = false;
            }
        }
    }
    Some(form)
}

/// `silentBars`: the bars with every layer scheduled out.
#[must_use]
pub fn silent_bars(form: &Form, bars: f64) -> List<usize, MAX_BARS> {
    let mut out = List::filled(0);
    let mut bar = 0;
    while js::num(bar) < bars {
        if form.iter().all(|s| !s.get(bar % s.len().max(1)).copied().unwrap_or(false)) {
            out.push(bar);
        }
        bar += 1;
    }
    out
}

/// `genGaps`: a beat or two of nothing, now and then.
#[must_use]
pub fn gen_gaps(cx: &Cx) -> List<Gap, 3> {
    let spec = cx.spec;
    let spb = cx.spb;
    let total = spec.bars * spb;
    let mut r = Rng::new(js::salted(spec.seed, 0x3c6e_f372, 13));
    let mut gaps = List::filled(Gap { start: 0.0, len: 0.0 });
    if !r.chance(0.18 + cx.c.airy.unwrap_or(0.2) * 0.5) {
        return gaps;
    }
    let count = r.weighted(&[(1, 4.0), (2, 3.0), (3, 1.5)]).unwrap_or(1);
    let beat = if cx.metre == Metre::ThreeFour { 4.0 } else { spb / 4.0 };
    for _ in 0..count {
        let len = r.pick(&[beat, beat, beat * 2.0, beat * 2.0, spb]).unwrap_or(beat);
        let beats = js::max(1.0, js::floor(total / beat));
        let start = r.int(1.0, beats - 1.0) * beat;
        if start + len > total {
            continue;
        }
        gaps.push(Gap { start: js::round(start), len: js::round(len) });
    }
    gaps
}

// --------------------------------------------------------------- plan

/// What render plans before the layers (`planOf`).
pub struct Plan {
    pub development: Option<Development>,
    pub harmony: Harmony,
    pub sections: Sections,
    pub form: Option<Form>,
    pub gaps: List<Gap, 3>,
}

impl Plan {
    /// An empty plan.
    #[must_use]
    pub const fn new() -> Self {
        Plan {
            development: None,
            harmony: Harmony::new(),
            sections: Sections::new(),
            form: None,
            gaps: List::filled(Gap { start: 0.0, len: 0.0 }),
        }
    }

    /// Plan the loop of `cx`.
    pub fn make(&mut self, cx: &Cx) {
        self.development = development_of(cx.spec, cx.choir);
        if let Some(dev) = &self.development {
            develop_harmony(cx, dev, &mut self.harmony, &mut self.sections);
        } else {
            let bars =
                cx.spec.cycles.and_then(|c| c[Layer::Chords as usize]).map_or(cx.spec.bars, |cyc| js::max(1.0, js::round(cyc / cx.spb)));
            gen_harmony(cx, &mut self.harmony, bars, cx.spec.layer_seed(Layer::Chords), None, None);
        }
        self.form = gen_form(cx);
        self.gaps = gen_gaps(cx);
    }

    /// How many items its lists dropped (0).
    #[must_use]
    pub fn lost(&self) -> usize {
        let s = &self.sections;
        self.harmony.lost()
            + s.a.lost()
            + s.b.lost()
            + s.v.lost()
            + s.r.lost()
            + self.form.as_ref().map_or(0, |f| f.iter().map(List::lost).sum())
            + self.gaps.lost()
    }

    /// The canonical text of the plan.
    pub fn write<S: Sink>(&self, w: &mut Writer<S>) {
        w.obj();
        w.key("development");
        match &self.development {
            None => w.null(),
            Some(d) => {
                w.obj();
                w.key("sections");
                w.list();
                for s in d.sections.iter() {
                    w.obj();
                    w.key("at");
                    w.num(s.at);
                    w.key("bars");
                    w.num(s.bars);
                    w.key("role");
                    let mut b = [0u8; 4];
                    w.str(s.role.encode_utf8(&mut b));
                    w.end_obj();
                }
                w.end_list();
                w.end_obj();
            }
        }
        w.key("form");
        write_form(w, self.form.as_ref());
        w.key("gaps");
        write_gaps(w, &self.gaps);
        w.key("harmony");
        write_harmony(w, &self.harmony, self.development.is_some().then_some(&self.sections));
        w.end_obj();
    }
}

impl Default for Plan {
    fn default() -> Self {
        Self::new()
    }
}

/// A form's canonical text.
pub fn write_form<S: Sink>(w: &mut Writer<S>, form: Option<&Form>) {
    match form {
        None => w.null(),
        Some(f) => {
            w.obj();
            for (key, layer) in [
                ("bass", Layer::Bass),
                ("chords", Layer::Chords),
                ("drums", Layer::Drums),
                ("melody", Layer::Melody),
                ("texture", Layer::Texture),
            ] {
                w.key(key);
                w.list();
                for &b in f[layer as usize].iter() {
                    w.bool(b);
                }
                w.end_list();
            }
            w.end_obj();
        }
    }
}

/// The gaps' canonical text.
pub fn write_gaps<S: Sink>(w: &mut Writer<S>, gaps: &[Gap]) {
    w.list();
    for g in gaps {
        w.obj();
        w.key("len");
        w.num(g.len);
        w.key("start");
        w.num(g.start);
        w.end_obj();
    }
    w.end_list();
}

fn write_notes<S: Sink>(w: &mut Writer<S>, notes: &[i16]) {
    w.list();
    for &n in notes {
        w.num(f64::from(n));
    }
    w.end_list();
}

/// A chord event's canonical text.
pub fn write_chord<S: Sink>(w: &mut Writer<S>, e: &ChordEvent) {
    w.obj();
    if let Some(d) = e.detune {
        w.key("detune");
        w.num(d);
    }
    w.key("dur");
    w.num(e.dur);
    w.key("notes");
    write_notes(w, &e.notes);
    w.key("step");
    w.num(e.step);
    if let Some(s) = e.strum {
        w.key("strum");
        w.str(s);
    }
    w.key("vel");
    w.num(e.vel);
    w.key("voice");
    w.str(e.voice);
    if let Some(v) = e.vowel {
        w.key("vowel");
        w.str(v);
    }
    w.end_obj();
}

fn write_degree<S: Sink>(w: &mut Writer<S>, d: Degree) {
    if !d.spelled {
        w.num(f64::from(d.d));
        return;
    }
    w.obj();
    w.key("d");
    w.num(f64::from(d.d));
    if d.flat {
        w.key("flat");
        w.bool(true);
    }
    if d.major {
        w.key("major");
        w.bool(true);
    }
    w.end_obj();
}

/// A harmony's canonical text; `sections` for a developing loop's.
pub fn write_harmony<S: Sink>(w: &mut Writer<S>, h: &Harmony, sections: Option<&Sections>) {
    w.obj();
    w.key("cycleSteps");
    w.num(h.cycle_steps);
    w.key("events");
    w.list();
    for e in h.events.iter() {
        write_chord(w, e);
    }
    w.end_list();
    if let Some(l) = h.long_chords {
        w.key("longChords");
        w.str(l);
    }
    if let Some(s) = sections {
        w.key("sectionHarmony");
        w.obj();
        w.key("A");
        write_harmony(w, &s.a, None);
        w.key("B");
        write_harmony(w, &s.b, None);
        w.key("R");
        match s.r() {
            Some(h) => write_harmony(w, h, None),
            None => w.null(),
        }
        w.key("V");
        match s.v() {
            Some(h) => write_harmony(w, h, None),
            None => w.null(),
        }
        w.end_obj();
    }
    w.key("shape");
    w.list();
    for d in h.shape.iter() {
        write_degree(w, *d);
    }
    w.end_list();
    w.key("slots");
    w.list();
    for s in h.slots.iter() {
        w.obj();
        w.key("bassDegree");
        w.num(s.bass_degree);
        w.key("bassMidi");
        w.num(s.bass_midi);
        w.key("degree");
        w.num(s.degree);
        w.key("lengthSteps");
        w.num(s.length_steps);
        w.key("notes");
        write_notes(w, &s.notes);
        w.key("rootMidi");
        w.num(s.root_midi);
        w.key("startStep");
        w.num(s.start_step);
        if let Some(st) = &s.steps {
            w.key("steps");
            w.nums(st);
        }
        w.end_obj();
    }
    w.end_list();
    w.end_obj();
}
