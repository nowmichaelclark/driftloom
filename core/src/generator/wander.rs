//! "Let the loop wander" (queue item 81): `drift`, the variation a pass
//! plays, drawn from the loop's seed and the pass's number
//! (`Engine._wander` in `js/engine.js`), so the same code wanders the same
//! way every time and any pass can be reached without playing the ones
//! before it. With it, the floor drop (queue item 58) and its shapes (item
//! 70, `shapeDrop`), the pedal's passing tones (item 71), and the tuned
//! kick a pass swaps to (item 67, `kickKindOf`). The goldens hash each of a
//! loop's first eight passes as `passes`.
//!
//! A pass never edits the loop: it is the loop's own events as `render`
//! made them, with what the pass changed marked on each (`Pass`): a hit,
//! a note or a chord silent this pass, a hat opened, a chord opened up or
//! played softly, the bass's and the tune's pitches; and the one ghost hit
//! it may add. So it costs a few bytes an event, not a copy of the loop.

use super::canon::{Sink, Writer};
use super::choices;
use super::data::FLOORDROP;
use super::js;
use super::layers::{self, BASS_MAX, BassEvent, DRUMS_MAX, DrumEvent};
use super::melody::{MELODY_MAX, MelodyEvent};
use super::plan::{self, CHORD_EVENTS_MAX, ChordEvent, Cx, Plan};
use super::render::{self, Render};
use super::rng::Rng;
use super::Layer;

/// Which stream of a loop's seed is its wander (what the ears' albums have
/// always used for it).
pub const DRIFT_SALT: u32 = 0x5eed_1e55;
/// How far a loop wanders, the app's default (`Engine.driftAmount`): the
/// goldens' passes are drawn at it.
pub const AMOUNT: f64 = 1.0;
/// The floor drop's chance a pass at full wander (queue item 58).
const FLOOR_DROP_RATE: f64 = 0.35;
/// The drums that are the floor.
const FLOOR_DRUMS: [&str; 3] = ["kick", "softkick", "frame"];
/// The steps a bass passing tone takes.
const PASSING: [f64; 5] = [-2.0, 3.0, 5.0, 7.0, 12.0];
/// The bass's register since roundBass (queue item 56).
const BASS_LOW: f64 = 36.0;
const BASS_HIGH: f64 = 57.0;

// What a pass changed in an event.
/// Silent this pass.
const SILENT: u8 = 1;
/// A hat (or the hand kit's jingle) opened; a chord opened up, its first
/// note's ninth above added.
const OPEN: u8 = 2;
/// A chord played at 0.4 of its velocity.
const SOFT: u8 = 4;

/// A pass's floor drop (`floorDrop`): the bars it covers from its first to
/// its last; and for a loop that draws the shapes, its shape, and for a
/// stagger the bass's wait in bars, for a stutter its drops' count and
/// length in bars.
#[derive(Clone, Copy, Debug)]
pub struct FloorDrop {
    pub bar: f64,
    pub bars: f64,
    pub kind: Option<&'static str>,
    pub late: Option<f64>,
    pub count: Option<f64>,
    pub len: Option<f64>,
}

impl FloorDrop {
    const fn single(bar: f64, bars: f64) -> Self {
        FloorDrop { bar, bars, kind: None, late: None, count: None, len: None }
    }
}

/// A wandered pass: what it changed in each of the loop's events, layer by
/// layer in their order (the texture never wanders), its ghost hit, its
/// floor drop and its kick.
pub struct Pass {
    /// The pass these marks are for, once one is drawn.
    pub number: Option<u32>,
    drums: [u8; DRUMS_MAX],
    /// The extra ghost hit, after the loop's own.
    ghost: Option<DrumEvent>,
    /// The bass's marks and pitches.
    bass: [(u8, f64); BASS_MAX],
    chords: [u8; CHORD_EVENTS_MAX],
    /// The tune's marks and pitches.
    melody: [(u8, f64); MELODY_MAX],
    pub floor_drop: Option<FloorDrop>,
    /// The tuned kick this pass plays, in a loop with one.
    pub kick: Option<&'static str>,
}

impl Pass {
    /// No pass drawn.
    #[must_use]
    pub const fn new() -> Self {
        Pass {
            number: None,
            drums: [0; DRUMS_MAX],
            ghost: None,
            bass: [(0, 0.0); BASS_MAX],
            chords: [0; CHORD_EVENTS_MAX],
            melody: [(0, 0.0); MELODY_MAX],
            floor_drop: None,
            kick: None,
        }
    }

    /// Draw pass `pass` of the loop of `cx`, rendered as `base` (`_wander`):
    /// `drift` at the default amount on the pass's own stream, and, in a
    /// loop with a tuned kick of the kind `kick`, the kick it plays.
    pub fn draw(&mut self, cx: &Cx, base: &Render, kick: Option<&'static str>, pass: u32) {
        self.draw_by(cx, base, kick, pass, AMOUNT);
    }

    /// The same, wandering `amount` as far (the app's slider, 0 to 2.4;
    /// `Engine.driftAmount`): what the player draws each pass by.
    pub fn draw_by(&mut self, cx: &Cx, base: &Render, kick: Option<&'static str>, pass: u32, amount: f64) {
        let seed = cx.spec.seed;
        let mut rng = Rng::new((seed ^ DRIFT_SALT).wrapping_add(pass.wrapping_mul(0x9e37_79b1)));
        self.drift(cx, base, &mut rng, amount);
        self.kick = kick.map(|k| choices::kick_kind_of(seed, k, pass));
        self.number = Some(pass);
    }

    /// `drift(pattern, rng, amount)`: hats flicker, a ghost hit, a kick
    /// sits one out, the tune steps to a neighbor or jumps an octave, the
    /// bass finds a passing tone, a chord opens up, now and then the floor
    /// drops out and a layer takes a breath; draw for draw as the
    /// JavaScript does, in its order.
    #[allow(clippy::too_many_lines)] // one function, as drift is: its draws in order
    fn drift(&mut self, cx: &Cx, base: &Render, rng: &mut Rng, a: f64) {
        let spec = cx.spec;
        let spb = cx.spb;
        let total = spec.bars * spb;
        let scale = spec.steps();

        // Hats flicker in and out, and so do the hand kit's zils.
        for (m, e) in self.drums.iter_mut().zip(base.drums.iter()) {
            *m = 0;
            let hat = e.inst == "hat" || e.inst == "jingle";
            if (hat || e.inst == "shaker") && rng.chance(0.1 * a) {
                *m |= SILENT;
            }
            if hat && rng.chance(0.05 * a) {
                *m |= OPEN;
            }
        }
        // An extra ghost hit, on whichever kit the loop is playing.
        self.ghost = None;
        if rng.chance(0.4 * a) && !base.drums.is_empty() {
            let step = rng.int(0.0, total - 1.0);
            let inst = rng.pick(&["rim", "shaker", "hat"]).unwrap_or("rim");
            let vel = 0.16 + rng.f() * 0.14;
            let inst = if base.drums_out.kit == "hand" { layers::hand(inst) } else { inst };
            self.ghost = Some(DrumEvent { step, inst, vel, vel0: vel, roll: false, micro: None });
        }
        // Occasionally the kick sits one out.
        if rng.chance(0.18 * a) {
            let kick = |e: &&DrumEvent| (e.inst == "kick" || e.inst == "softkick") && e.step > 0.0;
            let n = base.drums.iter().filter(kick).count();
            if n > 0 {
                let k = js::index(rng.int(0.0, js::num(n) - 1.0));
                let hit = base.drums.iter().enumerate().filter(|(_, e)| kick(e)).nth(k).map(|x| x.0);
                if let Some(m) = hit.and_then(|i| self.drums.get_mut(i)) {
                    *m |= SILENT;
                }
            }
        }
        // A melody note steps to a neighbor, or jumps an octave, inside the
        // melody's own window.
        for (m, e) in self.melody.iter_mut().zip(base.melody.iter()) {
            let mut midi = e.midi;
            let mut marks = 0;
            if rng.chance(0.09 * a) {
                let steps = e.steps.as_ref().map_or(scale, |s| &s[..]);
                let dir = if rng.chance(0.5) { 1.0 } else { -1.0 };
                midi = neighbour_in_scale(midi, spec.root, steps, dir);
            }
            if rng.chance(0.04 * a) && midi + 12.0 <= 95.0 {
                midi += 12.0;
            }
            if rng.chance(0.07 * a) {
                marks |= SILENT;
            }
            *m = (marks, midi);
        }
        // The bass finds a passing tone, in its own register. A pedal bass
        // spends the draws its loop's own bass would have made, so nothing
        // drawn after moves, and finds its own at the very end.
        for (m, e) in self.bass.iter_mut().zip(base.bass.iter()) {
            *m = (0, e.midi);
        }
        let pedal = base.pedal.map(|(_, n0)| n0);
        if let Some(n0) = pedal {
            let mut k = 0.0;
            while k < n0 {
                if rng.chance(0.07 * a) {
                    rng.pick(&PASSING);
                }
                k += 1.0;
            }
        } else {
            self.passing_tones(base, rng, a);
        }
        // A chord opens up, or steps back.
        for (m, e) in self.chords.iter_mut().zip(base.chords.iter()) {
            *m = 0;
            if rng.chance(0.06 * a) && !e.notes.is_empty() {
                *m |= OPEN;
            }
            if rng.chance(0.05 * a) {
                *m |= SOFT;
            }
        }
        // And now and then the floor drops out: the kick and the bass out
        // for a bar or two while everything else plays on. A loop that
        // draws the shapes shapes its drop at the very end of these draws.
        let shapes = base.choice("floorDrop") == Some("shapes");
        let mut drop_at = None;
        self.floor_drop = None;
        if rng.chance(FLOOR_DROP_RATE * a) {
            let bars = js::max(1.0, js::floor(total / spb));
            if bars >= 4.0 {
                let span = if rng.chance(0.35) { 2.0 } else { 1.0 };
                let latest = js::max(1.0, bars - span);
                // On an even bar (`& ~1`).
                #[allow(clippy::cast_possible_truncation)] // a bar number, far inside i32
                let even = f64::from((rng.int(1.0, latest) as i32) & !1);
                let start = js::min(latest, js::max(2.0, even));
                if shapes {
                    drop_at = Some((start, span, bars));
                } else {
                    self.floor_out(base, start * spb, (start + span) * spb, true);
                    self.floor_drop = Some(FloorDrop::single(start, span));
                }
            }
        }
        // Now and then a single layer takes a breath.
        if rng.chance(0.1 * a) {
            let layer = rng.pick(&[Layer::Melody, Layer::Chords, Layer::Drums]).unwrap_or(Layer::Melody);
            let half = if rng.chance(0.5) { 0.0 } else { total / 2.0 };
            let inside = |step: f64| step >= half && step < half + total / 2.0;
            match layer {
                Layer::Melody => {
                    for (m, e) in self.melody.iter_mut().zip(base.melody.iter()) {
                        if inside(e.step) {
                            m.0 |= SILENT;
                        }
                    }
                }
                Layer::Chords => {
                    for (m, e) in self.chords.iter_mut().zip(base.chords.iter()) {
                        if inside(e.step) {
                            *m |= SILENT;
                        }
                    }
                }
                _ => {
                    for (m, e) in self.drums.iter_mut().zip(base.drums.iter()) {
                        if inside(e.step) {
                            *m |= SILENT;
                        }
                    }
                    if let Some(g) = &mut self.ghost
                        && inside(g.step)
                    {
                        g.vel = 0.0;
                    }
                }
            }
        }
        if let Some((start, span, bars)) = drop_at {
            self.floor_drop = Some(self.shape_drop(cx, base, rng, start, span, bars));
        }
        // A pedal bass's passing tones, drawn last of all, so even a drop's
        // shape is the one its loop's own bass gets.
        if pedal.is_some() {
            self.passing_tones(base, rng, a);
        }
    }

    /// The bass's passing tones: a step from `PASSING`, folded into the
    /// bass's register.
    fn passing_tones(&mut self, base: &Render, rng: &mut Rng, a: f64) {
        for m in self.bass.iter_mut().take(base.bass.len()) {
            if rng.chance(0.07 * a) {
                let mut midi = m.1 + rng.pick(&PASSING).unwrap_or(0.0);
                while midi > BASS_HIGH {
                    midi -= 12.0;
                }
                while midi < BASS_LOW {
                    midi += 12.0;
                }
                m.1 = midi;
            }
        }
    }

    /// `floorOut`: the kick, the soft kick, the frame (with `kit`) and the
    /// bass, silent from step `from` to step `to`.
    fn floor_out(&mut self, base: &Render, from: f64, to: f64, kit: bool) {
        let inside = |step: f64| step >= from && step < to;
        if kit {
            for (m, e) in self.drums.iter_mut().zip(base.drums.iter()) {
                if FLOOR_DRUMS.contains(&e.inst) && inside(e.step) {
                    *m |= SILENT;
                }
            }
        }
        for (m, e) in self.bass.iter_mut().zip(base.bass.iter()) {
            if inside(e.step) {
                m.0 |= SILENT;
            }
        }
    }

    /// `shapeDrop`: a floor drop in one of the shapes (queue item 70), on
    /// the bars item 58's drop drew (`start` and `span`, in a loop of
    /// `bars` bars), its shape drawn with `rng`. A shape follows the bar's
    /// length, as the sketch's did; a stutter with no room for two drops is
    /// a single one.
    fn shape_drop(&mut self, cx: &Cx, base: &Render, rng: &mut Rng, start: f64, span: f64, bars: f64) -> FloorDrop {
        let d = &FLOORDROP;
        let spb = cx.spb;
        let has_bass = self.bass.iter().zip(base.bass.iter()).any(|(m, e)| e.vel != 0.0 && (m.0 & SILENT) == 0);
        let mut kind = choices::drop_shape_of(rng, has_bass, cx.spec.vibes.as_ref());
        let bar_sec = spb * 60.0 / cx.spec.bpm / 4.0;
        let out = |this: &mut Self, b0: f64, b1: f64, kit: bool| {
            this.floor_out(base, js::max(0.0, b0) * spb, js::min(bars, b1) * spb, kit);
        };
        if kind == "stagger" {
            let late = if bar_sec < d.stagger_short { 2.0 } else { 1.0 };
            out(self, start, start + span, true);
            out(self, start + span, start + span + late, false);
            return FloorDrop { kind: Some(kind), late: Some(late), ..FloorDrop::single(start, span + late) };
        }
        if kind == "stutter" {
            let k = if bar_sec < d.stutter_short { 2.0 } else { 1.0 };
            let n = if rng.chance(0.5) { d.stutter[0] } else { d.stutter[1] };
            let fit = js::min(n, js::floor((bars + k) / (2.0 * k)));
            if fit >= 2.0 {
                let at = js::max(0.0, js::min(start, bars - (2.0 * k * (fit - 1.0) + k)));
                let mut i = 0.0;
                while i < fit {
                    out(self, at + 2.0 * k * i, at + 2.0 * k * i + k, true);
                    i += 1.0;
                }
                return FloorDrop { kind: Some(kind), count: Some(fit), len: Some(k), ..FloorDrop::single(at, 2.0 * k * (fit - 1.0) + k) };
            }
            kind = "single";
        }
        if kind == "long" {
            let len = js::max(d.long_bars[0], js::min(d.long_bars[1], js::round(d.long_seconds / bar_sec)));
            let whole = bars <= d.whole_under;
            let at = if whole { 0.0 } else { start };
            let n = if whole { bars } else { js::min(len, bars - at) };
            out(self, at, at + n, true);
            return FloorDrop { kind: Some(kind), ..FloorDrop::single(at, n) };
        }
        out(self, start, start + span, true);
        FloorDrop { kind: Some("single"), ..FloorDrop::single(start, span) }
    }

    /// Drum hit `i` of the loop, `e`, as this pass plays it: its velocity
    /// (0, silent this pass) and its instrument (a hat or a jingle opened).
    /// What the sequencer plays (`player`), and what `write` writes, so the
    /// goldens prove it.
    #[must_use]
    pub fn drum(&self, i: usize, e: &DrumEvent) -> (f64, &'static str) {
        let m = self.drums.get(i).copied().unwrap_or(0);
        let inst = if (m & OPEN) != 0 { if e.inst == "hat" { "ohat" } else { "ojingle" } } else { e.inst };
        (if (m & SILENT) != 0 { 0.0 } else { e.vel }, inst)
    }

    /// The ghost hit this pass adds after the loop's own, if it adds one.
    #[must_use]
    pub fn ghost(&self) -> Option<&DrumEvent> {
        self.ghost.as_ref()
    }

    /// Bass note `i` of the loop, `e`, as this pass plays it: its velocity
    /// and its pitch (a passing tone).
    #[must_use]
    pub fn bass(&self, i: usize, e: &BassEvent) -> (f64, f64) {
        let (m, midi) = self.bass.get(i).copied().unwrap_or((0, e.midi));
        (if (m & SILENT) != 0 { 0.0 } else { e.vel }, midi)
    }

    /// Chord `i` of the loop, `e`, as this pass plays it: its velocity (0.4
    /// of it played softly) and the note it adds when it opens up, its
    /// first note's ninth above.
    #[must_use]
    pub fn chord(&self, i: usize, e: &ChordEvent) -> (f64, Option<i16>) {
        let m = self.chords.get(i).copied().unwrap_or(0);
        let mut vel = e.vel;
        if (m & SOFT) != 0 {
            vel *= 0.4;
        }
        if (m & SILENT) != 0 {
            vel = 0.0;
        }
        (vel, if (m & OPEN) != 0 { e.notes.first().map(|&n| n + 14) } else { None })
    }

    /// Melody note `i` of the loop, `e`, as this pass plays it: its
    /// velocity and its pitch (a neighbor, or an octave up).
    #[must_use]
    pub fn melody(&self, i: usize, e: &MelodyEvent) -> (f64, f64) {
        let (m, midi) = self.melody.get(i).copied().unwrap_or((0, e.midi));
        (if (m & SILENT) != 0 { 0.0 } else { e.vel }, midi)
    }

    /// The pass's canonical text: the whole pattern it plays, less its spec
    /// (`render`'s, with the pass's tracks and floor drop), and the kick it
    /// plays in a loop with a tuned kick.
    pub fn write<S: Sink>(&self, w: &mut Writer<S>, cx: &Cx, plan: &Plan, base: &Render) {
        w.obj();
        w.key("choices");
        base.write_choices(w);
        w.key("cycles");
        render::write_cycles(w, cx);
        if let Some(d) = &self.floor_drop {
            w.key("floorDrop");
            write_drop(w, d);
        }
        w.key("form");
        plan::write_form(w, base.form.as_ref());
        w.key("gaps");
        plan::write_gaps(w, &base.gaps);
        w.key("harmony");
        base.write_harmony(w, plan);
        if let Some(k) = self.kick {
            w.key("kick");
            w.str(k);
        }
        w.key("meta");
        base.write_meta(w, cx, plan);
        w.key("silentBars");
        base.write_silent_bars(w);
        w.key("stepsPerBar");
        w.num(cx.spb);
        w.key("totalSteps");
        w.num(cx.spec.bars * cx.spb);
        w.key("tracks");
        w.obj();
        w.key("bass");
        w.list();
        for (i, e) in base.bass.iter().enumerate() {
            let (vel, midi) = self.bass(i, e);
            render::write_bass(w, &BassEvent { vel, midi, ..*e });
        }
        w.end_list();
        w.key("chords");
        w.list();
        for (i, e) in base.chords.iter().enumerate() {
            let (vel, opened) = self.chord(i, e);
            plan::write_chord(w, &ChordEvent { vel, ..*e }, opened);
        }
        w.end_list();
        w.key("drums");
        w.list();
        for (i, e) in base.drums.iter().enumerate() {
            let (vel, inst) = self.drum(i, e);
            render::write_drum(w, &DrumEvent { vel, inst, ..*e });
        }
        if let Some(g) = self.ghost() {
            render::write_drum(w, g);
        }
        w.end_list();
        w.key("melody");
        w.list();
        for (i, e) in base.melody.iter().enumerate() {
            let (vel, midi) = self.melody(i, e);
            render::write_melody(w, &MelodyEvent { vel, midi, ..*e });
        }
        w.end_list();
        w.key("texture");
        base.write_layer(w, Layer::Texture);
        w.end_obj();
        w.end_obj();
    }
}

impl Default for Pass {
    fn default() -> Self {
        Self::new()
    }
}

/// A floor drop's text.
fn write_drop<S: Sink>(w: &mut Writer<S>, d: &FloorDrop) {
    w.obj();
    w.key("bar");
    w.num(d.bar);
    w.key("bars");
    w.num(d.bars);
    if let Some(c) = d.count {
        w.key("count");
        w.num(c);
    }
    if let Some(k) = d.kind {
        w.key("kind");
        w.str(k);
    }
    if let Some(l) = d.late {
        w.key("late");
        w.num(l);
    }
    if let Some(l) = d.len {
        w.key("len");
        w.num(l);
    }
    w.end_obj();
}

/// `neighbourInScale`: walk up or down from `midi` to the nearest note of
/// the mode (`steps`, from `root`), so a wandering tune never leaves the
/// mode it was written in; the note itself if none is within four
/// semitones.
pub(crate) fn neighbour_in_scale(midi: f64, root: f64, steps: &[f64], dir: f64) -> f64 {
    for i in 1..=4 {
        let cand = midi + dir * f64::from(i);
        let pc = (((cand - root) % 12.0) + 12.0) % 12.0;
        if steps.contains(&pc) {
            return cand;
        }
    }
    midi
}
