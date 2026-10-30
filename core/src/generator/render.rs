//! `render` (queue item 80): the layers drawn over the plan, then the
//! entry schedules and their repairs, the gaps, the per-loop choices
//! (`drawChoices`, which the pedal needs), the pedal, the register work,
//! the spellings, the long chords' octave, the slurs, the strums, the
//! ornaments and the harmony line, as `js/generator.js` does them, in its
//! order. The goldens hash what comes out as `form`, each layer's events,
//! `meta` and `choices`.

use super::canon::{Sink, Writer};
use super::choices::lean_of;
use super::data::{self, CHOICES, MAX_BARS, PADSWEEP, PEDAL};
use super::js;
use super::layers::{self, Bass, BassEvent, BassOut, DrumEvent, Drums, DrumsOut, PedalOut, Texture, TextureEvent};
use super::list::List;
use super::melody::{self, Depth, Melody, MelodyOut, Over};
use super::plan::{self, CHORD_EVENTS_MAX, ChordEvent, Cx, Form, Gap, Harmony, Plan};
use super::rng::Rng;
use super::spec::Metre;
use super::{LAYERS, Layer};

/// A layer's own length in bars (`layerSpec(spec, layer).bars`).
#[must_use]
pub fn layer_bars(cx: &Cx, layer: Layer) -> f64 {
    match cx.spec.cycles.and_then(|c| c[layer as usize]) {
        Some(cyc) if cyc != 0.0 => js::max(1.0, js::round(cyc / cx.spb)),
        _ => cx.spec.bars,
    }
}

/// The chords layer's events.
pub type Chords = List<ChordEvent, CHORD_EVENTS_MAX>;

/// The most choices there can be.
const CHOICES_MAX: usize = 16;

/// A loop rendered: `render(spec)`'s return, as far as the goldens hash it.
pub struct Render {
    pub harmony: Harmony,
    pub drums: Drums,
    pub bass: Bass,
    pub chords: Chords,
    pub melody: Melody,
    pub texture: Texture,
    pub form: Option<Form>,
    pub gaps: List<Gap, 3>,
    pub quiet: List<usize, MAX_BARS>,
    /// Each choice's option, in `CHOICES`' order.
    pub choices: List<&'static str, CHOICES_MAX>,
    pub bass_out: BassOut,
    pub pedal: Option<(PedalOut, f64)>,
    pub drums_out: DrumsOut,
    pub melody_out: MelodyOut,
    pub texture_kind: &'static str,
    pub depth: Option<Depth>,
    // Working room: the pedal's notes, a developing loop's sections.
    pedal_notes: Bass,
    a_notes: Melody,
    section_notes: Melody,
}

impl Render {
    /// Nothing rendered.
    #[must_use]
    pub const fn new() -> Self {
        Render {
            harmony: Harmony::new(),
            drums: List::filled(layers::NO_DRUM),
            bass: List::filled(layers::NO_BASS),
            chords: List::filled(plan::EMPTY_EVENT),
            melody: List::filled(melody::NO_NOTE),
            texture: List::filled(layers::NO_TEXTURE),
            form: None,
            gaps: List::filled(Gap { start: 0.0, len: 0.0 }),
            quiet: List::filled(0),
            choices: List::filled(""),
            bass_out: BassOut { style: "", voice: "" },
            pedal: None,
            drums_out: DrumsOut { kit: "" },
            melody_out: MelodyOut { voice: "", motif: List::filled(melody::NO_MOTIF), cell: None, centre: None },
            texture_kind: "",
            depth: None,
            pedal_notes: List::filled(layers::NO_BASS),
            a_notes: List::filled(melody::NO_NOTE),
            section_notes: List::filled(melody::NO_NOTE),
        }
    }

    /// How many items its lists dropped (0).
    #[must_use]
    pub fn lost(&self) -> usize {
        self.harmony.lost()
            + self.drums.lost()
            + self.bass.lost()
            + self.chords.lost()
            + self.melody.lost()
            + self.texture.lost()
            + self.pedal_notes.lost()
            + self.a_notes.lost()
            + self.section_notes.lost()
            + self.choices.lost()
    }

    /// Render the loop of `cx` over `plan` (made with the same `cx`).
    #[allow(clippy::too_many_lines)] // one function, as render is: its steps in order
    pub fn run(&mut self, cx: &Cx, plan: &Plan) {
        let spec = cx.spec;
        let spb = cx.spb;
        self.harmony = plan.harmony;
        let bass_bars = layer_bars(cx, Layer::Bass);
        self.bass_out = layers::gen_bass(cx, bass_bars, &self.harmony.slots, &mut self.bass);
        self.depth = None;
        if let Some(dev) = &plan.development {
            let (out, depth) = melody::develop_melody(cx, &plan.sections, dev, &mut self.melody, &mut self.a_notes, &mut self.section_notes);
            self.melody_out = out;
            self.depth = Some(depth);
        } else {
            let over = Over { slots: &self.harmony.slots, cycle_steps: self.harmony.cycle_steps, events: &self.harmony.events };
            self.melody_out =
                melody::gen_melody(cx, layer_bars(cx, Layer::Melody), spec.layer_seed(Layer::Melody), &over, None, &mut self.melody);
        }
        self.drums_out = layers::gen_drums(cx, layer_bars(cx, Layer::Drums), &mut self.drums);
        self.texture_kind =
            layers::gen_texture(cx, layer_bars(cx, Layer::Texture), &self.harmony.slots, self.harmony.cycle_steps, &mut self.texture);
        self.chords.clear();
        for e in self.harmony.events.iter() {
            self.chords.push(ChordEvent { vel0: e.vel, alias: true, ..*e });
        }
        self.form = plan.form;
        self.gaps = plan.gaps;
        self.quiet.clear();
        if self.form.is_some() {
            if let (Some(dev), Some(form)) = (&plan.development, &mut self.form) {
                let sched = &mut form[Layer::Melody as usize];
                let n = sched.len().max(1);
                for s in dev.sections.iter().filter(|s| s.role == 'B') {
                    let mut bar = s.at;
                    while bar < s.at + s.bars {
                        if let Some(x) = sched.get_mut(js::index(bar) % n) {
                            *x = true;
                        }
                        bar += 1.0;
                    }
                }
            }
            for layer in LAYERS {
                self.apply_form(layer, spb);
            }
            let max_rest = js::max(1.0, js::round(1.0 + cx.c.airy.unwrap_or(0.25) * 2.0));
            let counts = self.limit_silence(cx, max_rest);
            for (bar, &n) in counts.iter().enumerate() {
                if n == 0.0 {
                    self.quiet.push(bar);
                }
            }
        }
        if !self.gaps.is_empty() {
            let gaps = self.gaps;
            let in_gap = |step: f64| gaps.iter().any(|g| step >= g.start && step < g.start + g.len);
            for e in self.drums.iter_mut().filter(|e| in_gap(e.step)) {
                e.vel = 0.0;
            }
            for e in self.bass.iter_mut().filter(|e| in_gap(e.step)) {
                e.vel = 0.0;
            }
            for e in self.chords.iter_mut().filter(|e| in_gap(e.step)) {
                e.vel = 0.0;
                e.alias = false;
            }
            for e in self.melody.iter_mut().filter(|e| in_gap(e.step)) {
                e.vel = 0.0;
            }
            for e in self.texture.iter_mut().filter(|e| in_gap(e.step)) {
                e.vel = 0.0;
            }
        }

        self.draw_choices(cx);

        self.pedal = None;
        if self.choice("pedalBass") == Some("on") {
            let out = layers::pedal_bass(cx, bass_bars, &self.bass, self.bass_out.voice, &mut self.pedal_notes);
            let mut on = [false; 4 * MAX_BARS];
            for e in self.bass.iter().filter(|e| e.vel != 0.0) {
                if let Some(x) = on.get_mut(js::index(js::floor(e.step / spb))) {
                    *x = true;
                }
            }
            let n0 = js::num(self.bass.len());
            let gaps = self.gaps;
            self.bass = self.pedal_notes;
            for e in self.bass.iter_mut() {
                let bar_on = on.get(js::index(js::floor(e.step / spb))).copied().unwrap_or(false);
                let in_gap = gaps.iter().any(|g| e.step >= g.start && e.step < g.start + g.len);
                if !bar_on || in_gap {
                    e.vel = 0.0;
                }
            }
            self.pedal = Some((out, n0));
        }

        self.separate_registers(cx);
        if self.harmony.slots.iter().any(|s| s.steps.is_some()) {
            self.spell_over(cx);
            self.separate_registers(cx);
        }

        let cycle = spec.cycles.and_then(|c| c[Layer::Melody as usize]).filter(|&c| c != 0.0).unwrap_or(spec.bars * spb);
        self.annotate_prev(cx, cycle);

        let beat = if cx.metre == Metre::SixEight { 6.0 } else { 4.0 };
        for (i, e) in self.chords.iter_mut().enumerate() {
            if e.voice == "nylon" {
                let strum = if (e.step % spb) % beat == 0.0 { "down" } else { "up" };
                e.strum = Some(strum);
                if e.alias
                    && let Some(h) = self.harmony.events.get_mut(i)
                {
                    h.strum = Some(strum);
                }
            }
        }

        if cx.choir.is_none() {
            self.ornament(cx);
            self.harmonize(cx);
        } else {
            for e in self.texture.iter_mut() {
                if e.vel != 0.0 {
                    e.vel *= 0.4;
                }
            }
        }
    }

    /// The chords' centre as heard (render's `centre`), or none.
    #[must_use]
    pub fn chord_centre(&self) -> Option<f64> {
        let mut sum = 0.0;
        let mut n = 0usize;
        for e in self.chords.iter().filter(|e| e.vel != 0.0) {
            for &x in e.notes.iter() {
                sum += f64::from(x);
                n += 1;
            }
        }
        (n > 0).then(|| sum / js::num(n))
    }

    /// Move every chord, in the harmony and in the layer, by `shift`
    /// semitones.
    pub fn shift_chords(&mut self, shift: i16) {
        for e in self.harmony.events.iter_mut() {
            for n in e.notes.iter_mut() {
                *n += shift;
            }
        }
        for e in self.chords.iter_mut() {
            for n in e.notes.iter_mut() {
                *n += shift;
            }
        }
    }

    fn apply_form(&mut self, layer: Layer, spb: f64) {
        let Some(form) = &self.form else { return };
        let sched = &form[layer as usize];
        let n = sched.len().max(1);
        let off = |step: f64| !sched.get(js::index(js::floor(step / spb)) % n).copied().unwrap_or(false);
        match layer {
            Layer::Drums => self.drums.iter_mut().filter(|e| off(e.step)).for_each(|e| e.vel = 0.0),
            Layer::Bass => self.bass.iter_mut().filter(|e| off(e.step)).for_each(|e| e.vel = 0.0),
            Layer::Chords => self.chords.iter_mut().filter(|e| off(e.step)).for_each(|e| {
                e.vel = 0.0;
                e.alias = false;
            }),
            Layer::Melody => self.melody.iter_mut().filter(|e| off(e.step)).for_each(|e| e.vel = 0.0),
            Layer::Texture => self.texture.iter_mut().filter(|e| off(e.step)).for_each(|e| e.vel = 0.0),
        }
    }

    /// Each layer's (step, sounding, sounding as generated), in a callback.
    fn each(&self, layer: Layer, mut f: impl FnMut(f64, bool, bool)) {
        match layer {
            Layer::Drums => self.drums.iter().for_each(|e| f(e.step, e.vel != 0.0, e.vel0 > 0.0)),
            Layer::Bass => self.bass.iter().for_each(|e| f(e.step, e.vel != 0.0, e.vel0 > 0.0)),
            Layer::Chords => self.chords.iter().for_each(|e| f(e.step, e.vel != 0.0, e.vel0 > 0.0)),
            Layer::Melody => self.melody.iter().for_each(|e| f(e.step, e.vel != 0.0, e.vel0 > 0.0)),
            Layer::Texture => self.texture.iter().for_each(|e| f(e.step, e.vel != 0.0, e.vel0 > 0.0)),
        }
    }

    fn audible_per_bar(&self, cx: &Cx, bars: usize) -> [f64; MAX_BARS] {
        let spb = cx.spb;
        let mut counts = [0.0; MAX_BARS];
        for layer in LAYERS {
            let cb = js::index(plan::local_bars(cx.spec, layer)).clamp(1, 4 * MAX_BARS);
            let mut per = [0.0; 4 * MAX_BARS];
            self.each(layer, |step, sounding, _| {
                if sounding && let Some(x) = per.get_mut(js::index(js::floor(step / spb)) % cb) {
                    *x += 1.0;
                }
            });
            for (b, c) in counts.iter_mut().enumerate().take(bars) {
                *c += per[b % cb];
            }
        }
        counts
    }

    fn limit_silence(&mut self, cx: &Cx, max_rest: f64) -> List<f64, MAX_BARS> {
        let spb = cx.spb;
        let bars = js::index(cx.spec.bars).min(MAX_BARS);
        let mut counts = self.audible_per_bar(cx, bars);
        let mut run = 0.0;
        for bar in 0..bars {
            if counts[bar] > 0.0 {
                run = 0.0;
                continue;
            }
            run += 1.0;
            if run <= max_rest {
                continue;
            }
            let mut repaired = false;
            for layer in [Layer::Chords, Layer::Bass, Layer::Texture, Layer::Melody, Layer::Drums] {
                let cb = js::index(plan::local_bars(cx.spec, layer)).max(1);
                let local = bar % cb;
                let in_bar = |step: f64| js::index(js::floor(step / spb)) % cb == local;
                let mut any = false;
                self.each(layer, |step, _, raw| any |= raw && in_bar(step));
                if !any {
                    continue;
                }
                match layer {
                    Layer::Drums => self.drums.iter_mut().filter(|e| in_bar(e.step)).for_each(|e| e.vel = e.vel0),
                    Layer::Bass => self.bass.iter_mut().filter(|e| in_bar(e.step)).for_each(|e| e.vel = e.vel0),
                    Layer::Chords => self.chords.iter_mut().filter(|e| in_bar(e.step)).for_each(|e| {
                        e.vel = e.vel0;
                        e.alias = false;
                    }),
                    Layer::Melody => self.melody.iter_mut().filter(|e| in_bar(e.step)).for_each(|e| e.vel = e.vel0),
                    Layer::Texture => self.texture.iter_mut().filter(|e| in_bar(e.step)).for_each(|e| e.vel = e.vel0),
                }
                if let Some(form) = &mut self.form
                    && let Some(x) = form[layer as usize].get_mut(local)
                {
                    *x = true;
                }
                repaired = true;
                break;
            }
            if !repaired {
                continue;
            }
            counts = self.audible_per_bar(cx, bars);
            run = 0.0;
        }
        let mut out = List::filled(0.0);
        for &c in &counts[..bars] {
            out.push(c);
        }
        out
    }

    /// The loop's option for the choice `id` (`render(spec).choices[id]`).
    #[must_use]
    pub fn choice(&self, id: &str) -> Option<&'static str> {
        CHOICES.iter().position(|c| c.id == id).and_then(|i| self.choices.get(i).copied())
    }

    /// `drawChoices(spec.seed, ctx, vibes)`.
    fn draw_choices(&mut self, cx: &Cx) {
        let spec = cx.spec;
        let v = spec.vibes.as_ref();
        self.choices.clear();
        for c in CHOICES {
            self.choices.push(c.options.first().map_or("", |o| o.0));
        }
        if CHOICES.is_empty() {
            return;
        }
        let mut r = Rng::new(js::salted(spec.seed, 0x4348_4f49, 1));
        let count = r.weighted(data::CHANGE_COUNTS).unwrap_or(0.0);
        let mut pool: List<(usize, f64), CHOICES_MAX> = List::filled((0, 0.0));
        for (i, c) in CHOICES.iter().enumerate() {
            if self.eligible(cx, c.id) {
                pool.push((i, c.weight.unwrap_or(1.0) * lean_of(c.vibes, v)));
            }
        }
        let mut k = 0.0;
        while k < count && !pool.is_empty() {
            let Some(i) = r.weighted(&pool) else { break };
            pool.retain(|x| x.0 != i);
            let c = &CHOICES[i];
            let others = c.options.get(1..).unwrap_or(&[]);
            let pick = if others.len() == 1 {
                others[0].0
            } else {
                let mut leaned: List<(&'static str, f64), 8> = List::filled(("", 0.0));
                for &(o, w) in others {
                    leaned.push((o, w * lean_of(data::table_named(c.option_vibes, o), v)));
                }
                r.weighted(&leaned).unwrap_or("")
            };
            if let Some(x) = self.choices.get_mut(i) {
                *x = pick;
            }
            k += 1.0;
        }
    }

    /// A choice's eligibility test (`ELIGIBLE` in `js/choices.js`).
    fn eligible(&self, cx: &Cx, id: &str) -> bool {
        let plays_chords = self.chords.iter().any(|e| e.vel != 0.0 && !e.notes.is_empty());
        let has_kick = self.drums.iter().any(|e| e.vel != 0.0 && (e.inst == "kick" || e.inst == "softkick"));
        let has_bass = self.bass.iter().any(|e| e.vel != 0.0);
        let has_drums = self.drums.iter().any(|e| e.vel != 0.0);
        match id {
            "glass" => self.chords.iter().any(|e| {
                e.vel != 0.0 && {
                    let mut distinct = 0;
                    for (i, n) in e.notes.iter().enumerate() {
                        if !e.notes[..i].contains(n) {
                            distinct += 1;
                        }
                    }
                    distinct >= 2
                }
            }),
            "dubStab" => plays_chords && has_drums && self.notes_per_second(cx) >= 7.0,
            "organ" => plays_chords,
            "wash" | "arc" => true,
            "tunedKick" => has_kick,
            "subDuck" => has_kick && has_bass,
            "lateFloor" => {
                let step = 60.0 / cx.spec.bpm / 4.0;
                let first = |s: f64| s * step < data::LATEFLOOR.first;
                (has_drums || has_bass)
                    && (self.chords.iter().any(|e| e.vel != 0.0 && first(e.step))
                        || self.melody.iter().any(|e| e.vel != 0.0 && first(e.step))
                        || self.texture.iter().any(|e| e.vel != 0.0 && first(e.step)))
            }
            "floorDrop" => cx.spec.bars >= 4.0 && (has_drums || has_bass),
            "pedalBass" => PEDAL.styles.contains(&self.bass_out.style) && has_bass,
            "padSweep" => self.chords.iter().any(|e| e.vel != 0.0 && PADSWEEP.sustained.contains(&e.voice)),
            _ => false,
        }
    }

    /// `notesPerSecond(spec, tracks)`: how many notes a second the loop
    /// plays, every layer, as heard.
    #[must_use]
    pub fn notes_per_second(&self, cx: &Cx) -> f64 {
        let seconds = cx.spec.bars * cx.spb * (60.0 / cx.spec.bpm / 4.0);
        let mut n = 0.0;
        for layer in LAYERS {
            self.each(layer, |_, sounding, _| {
                if sounding {
                    n += 1.0;
                }
            });
        }
        n / seconds
    }

    /// `separateRegisters(harmony, tracks, choir)`.
    fn separate_registers(&mut self, cx: &Cx) {
        let mut sum = 0.0;
        let mut n = 0usize;
        let (mut lowest, mut highest) = (f64::INFINITY, f64::NEG_INFINITY);
        for e in self.chords.iter().filter(|e| e.vel != 0.0 && !e.notes.is_empty()) {
            for &x in e.notes.iter() {
                let x = f64::from(x);
                sum += x;
                n += 1;
                lowest = js::min(lowest, x);
                highest = js::max(highest, x);
            }
        }
        let mut msum = 0.0;
        let mut mn = 0usize;
        for e in self.melody.iter().filter(|e| e.vel != 0.0) {
            msum += e.midi;
            mn += 1;
        }
        if n == 0 || mn == 0 {
            return;
        }
        let melody_centre = msum / js::num(mn);
        let chord_centre = sum / js::num(n);
        let room = |sh: f64| lowest + sh >= 33.0 && highest + sh <= 96.0;
        let gap = |sh: f64| js::abs(melody_centre - (chord_centre + sh));
        let mut shift = 0.0;
        if cx.choir.is_some() {
            for sh in [-12.0, 12.0, -24.0] {
                if room(sh) && gap(sh) < gap(shift) {
                    shift = sh;
                }
            }
        } else if gap(0.0) < 5.0 {
            let down: List<f64, 2> = [-12.0, -24.0].into_iter().filter(|&sh| room(sh)).fold(List::filled(0.0), |mut l, x| {
                l.push(x);
                l
            });
            shift = if let Some(&sh) = down.iter().find(|&&sh| gap(sh) >= 5.0) {
                sh
            } else if down.len() == 2 && gap(down[1]) > gap(down[0]) {
                down[1]
            } else {
                down.first().copied().unwrap_or(0.0)
            };
        }
        if shift != 0.0 {
            #[allow(clippy::cast_possible_truncation)] // whole octaves
            self.shift_chords(shift as i16);
        }
    }

    /// `spellOver`, on the melody and then the bass.
    fn spell_over(&mut self, cx: &Cx) {
        let spec = cx.spec;
        let scale = spec.steps();
        let h = &self.harmony;
        let respell = |midi: &mut f64, steps: &mut Option<List<f64, 7>>, step: f64| {
            let slot = h.slot_at(step);
            let Some(st) = &slot.steps else { return };
            let p = (((*midi - spec.root) % 12.0) + 12.0) % 12.0;
            if let Some(i) = scale.iter().position(|&x| x == p) {
                *midi += ((st[i] - scale[i] + 18.0) % 12.0) - 6.0;
            }
            *steps = Some(*st);
        };
        for e in self.melody.iter_mut() {
            respell(&mut e.midi, &mut e.steps, e.step);
        }
        for e in self.bass.iter_mut() {
            respell(&mut e.midi, &mut e.steps, e.step);
        }
    }

    /// `annotatePrev`: the pitch each melody note slides from.
    fn annotate_prev(&mut self, cx: &Cx, cycle: f64) {
        let sd = 60.0 / cx.spec.bpm / 4.0;
        let mut order: List<usize, { melody::MELODY_MAX }> = List::filled(0);
        for (i, e) in self.melody.iter().enumerate() {
            if e.vel != 0.0 {
                order.push(i);
            }
        }
        let m = &mut self.melody;
        order.sort_stable_by(|&a, &b| m[a].step < m[b].step);
        if order.len() < 2 {
            return;
        }
        let joined = |from_end: f64, to_start: f64| (to_start - from_end) * sd <= 0.12;
        for k in 1..order.len() {
            let (p, c) = (m[order[k - 1]], order[k]);
            if joined(p.step + p.dur, m[c].step) {
                m[c].prev = Some(p.midi);
            }
        }
        let (first, last) = (order[0], m[order[order.len() - 1]]);
        if cycle != 0.0 && joined(last.step + last.dur - cycle, m[first].step) {
            m[first].prev = Some(last.midi);
        }
    }

    /// `ornament`: cuts and turns.
    fn ornament(&mut self, cx: &Cx) {
        let Some(p) = cx.c.ornament.filter(|&p| p != 0.0) else { return };
        let spb = cx.spb;
        let beat = if cx.metre == Metre::SixEight { 6.0 } else { 4.0 };
        let sd = 60.0 / cx.spec.bpm / 4.0;
        let mut r = Rng::new(js::salted(cx.spec.layer_seed(Layer::Melody), 0x0a11_e7ed, 1));
        for e in self.melody.iter_mut() {
            if e.vel == 0.0 || (e.step % spb) % beat != 0.0 {
                continue;
            }
            if e.dur < 2.0 || e.dur * sd < 0.15 {
                continue;
            }
            if !r.chance(p) {
                continue;
            }
            e.orn = Some(if e.dur >= 4.0 && r.chance(0.25) { "turn" } else { "cut" });
        }
    }

    /// `harmonize`: a line in thirds or sixths on the held notes.
    fn harmonize(&mut self, cx: &Cx) {
        let Some(p) = cx.c.harmonize.filter(|&p| p != 0.0) else { return };
        let mut r = Rng::new(js::salted(cx.spec.layer_seed(Layer::Melody), 0x7417_d5e5, 1));
        if !r.chance(p) {
            return;
        }
        let kind = r.weighted(&[("third", 2.0), ("sixth", 1.0)]).unwrap_or("third");
        let m = &mut self.melody;
        // The last note of each phrase (the first of the latest, by step).
        let mut last: List<(f64, usize), { melody::MELODY_MAX }> = List::filled((0.0, 0));
        let mut sounding = 0;
        let mut held = 0;
        for (i, e) in m.iter().enumerate().filter(|(_, e)| e.vel != 0.0) {
            sounding += 1;
            if e.dur >= 6.0 {
                held += 1;
            }
            match last.iter_mut().find(|x| x.0 == e.phrase) {
                Some(x) => {
                    if e.step > m[x.1].step {
                        x.1 = i;
                    }
                }
                None => last.push((e.phrase, i)),
            }
        }
        let only_ends = held * 2 > sounding;
        let chosen = |i: usize, e: &melody::MelodyEvent| {
            e.vel != 0.0 && (last.iter().any(|x| x.1 == i) || (!only_ends && e.dur >= 6.0))
        };
        let size = m.iter().enumerate().filter(|(i, e)| chosen(*i, e)).count();
        if size >= sounding {
            return;
        }
        let marks: List<usize, { melody::MELODY_MAX }> =
            m.iter().enumerate().filter(|(i, e)| chosen(*i, e)).map(|(i, _)| i).fold(List::filled(0), |mut l, i| {
                l.push(i);
                l
            });
        for &i in marks.iter() {
            m[i].harm = Some(kind);
        }
    }

    // ------------------------------------------------- canonical texts

    /// `form`'s text: what render decides besides the layers' notes.
    pub fn write_form<S: Sink>(&self, w: &mut Writer<S>, cx: &Cx, plan: &Plan) {
        w.obj();
        w.key("cycles");
        write_cycles(w, cx);
        w.key("form");
        plan::write_form(w, self.form.as_ref());
        w.key("gaps");
        plan::write_gaps(w, &self.gaps);
        w.key("harmony");
        self.write_harmony(w, plan);
        w.key("silentBars");
        self.write_silent_bars(w);
        w.key("stepsPerBar");
        w.num(cx.spb);
        w.key("totalSteps");
        w.num(cx.spec.bars * cx.spb);
        w.end_obj();
    }

    /// The harmony's text (with a developing loop's sections).
    pub fn write_harmony<S: Sink>(&self, w: &mut Writer<S>, plan: &Plan) {
        plan::write_harmony(w, &self.harmony, plan.development.is_some().then_some(&plan.sections));
    }

    /// The bars where nothing plays (`silentBars`).
    pub fn write_silent_bars<S: Sink>(&self, w: &mut Writer<S>) {
        w.list();
        for &b in self.quiet.iter() {
            w.num(js::num(b));
        }
        w.end_list();
    }

    /// A layer's events' text.
    pub fn write_layer<S: Sink>(&self, w: &mut Writer<S>, layer: Layer) {
        w.list();
        match layer {
            Layer::Drums => self.drums.iter().for_each(|e| write_drum(w, e)),
            Layer::Bass => self.bass.iter().for_each(|e| write_bass(w, e)),
            Layer::Chords => self.chords.iter().for_each(|e| plan::write_chord(w, e, None)),
            Layer::Melody => self.melody.iter().for_each(|e| write_melody(w, e)),
            Layer::Texture => self.texture.iter().for_each(|e| write_texture(w, e)),
        }
        w.end_list();
    }

    /// `meta`'s text.
    pub fn write_meta<S: Sink>(&self, w: &mut Writer<S>, cx: &Cx, plan: &Plan) {
        w.obj();
        w.key("bassStyle");
        w.str(if self.pedal.is_some() { "pedal" } else { self.bass_out.style });
        w.key("bassVoice");
        w.str(self.bass_out.voice);
        if let Some(ch) = cx.choir {
            w.key("choir");
            w.obj();
            w.key("keysDetune");
            w.num(ch.keys_detune);
            w.key("melodyDetune");
            w.num(ch.melody_detune);
            w.key("voice");
            w.str(ch.voice);
            w.end_obj();
        }
        if let (Some(dev), Some(depth)) = (&plan.development, &self.depth) {
            w.key("depth");
            w.obj();
            w.key("answer");
            match depth.answer {
                Some(a) => w.str(a),
                None => w.null(),
            }
            w.key("form");
            let mut roles = [0u8; 4];
            let n = dev.sections.len().min(4);
            for (r, s) in roles.iter_mut().zip(dev.sections.iter()) {
                *r = u8::try_from(u32::from(s.role)).unwrap_or(b'?');
            }
            w.str(core::str::from_utf8(&roles[..n]).unwrap_or(""));
            w.key("shift");
            match depth.shift {
                Some(s) => w.num(s),
                None => w.null(),
            }
            w.end_obj();
        }
        w.key("kit");
        w.str(self.drums_out.kit);
        w.key("melodyCell");
        match self.melody_out.cell {
            Some(c) => w.str(c),
            None => w.null(),
        }
        w.key("melodyVoice");
        w.str(self.melody_out.voice);
        if let Some((p, n0)) = &self.pedal {
            w.key("pedal");
            w.obj();
            w.key("n0");
            w.num(*n0);
            w.key("nb");
            w.num(p.nb);
            w.key("rhythm");
            w.str(p.rhythm);
            w.key("share");
            w.num(fixed3(p.share));
            w.key("was");
            w.str(self.bass_out.style);
            w.end_obj();
        }
        w.key("textureKind");
        w.str(self.texture_kind);
        w.end_obj();
    }

    /// `choices`' text.
    pub fn write_choices<S: Sink>(&self, w: &mut Writer<S>) {
        let mut order: List<usize, CHOICES_MAX> = List::filled(0);
        for i in 0..self.choices.len() {
            order.push(i);
        }
        order.sort_stable_by(|&a, &b| CHOICES[a].id < CHOICES[b].id);
        w.obj();
        for &i in order.iter() {
            w.key_any(CHOICES[i].id);
            w.str(self.choices[i]);
        }
        w.end_obj();
    }
}

impl Default for Render {
    fn default() -> Self {
        Self::new()
    }
}

/// Each layer's own cycle in steps (`cycles`).
pub fn write_cycles<S: Sink>(w: &mut Writer<S>, cx: &Cx) {
    let spec = cx.spec;
    let full = spec.bars * cx.spb;
    w.obj();
    for (key, layer) in [("bass", Layer::Bass), ("chords", Layer::Chords), ("drums", Layer::Drums), ("melody", Layer::Melody), ("texture", Layer::Texture)] {
        w.key(key);
        w.num(spec.cycles.and_then(|c| c[layer as usize]).filter(|&c| c != 0.0).unwrap_or(full));
    }
    w.end_obj();
}

/// A drum hit's text.
pub fn write_drum<S: Sink>(w: &mut Writer<S>, e: &DrumEvent) {
    w.obj();
    w.key("inst");
    w.str(e.inst);
    if let Some(m) = e.micro {
        w.key("micro");
        w.num(m);
    }
    if e.roll {
        w.key("roll");
        w.bool(true);
    }
    w.key("step");
    w.num(e.step);
    w.key("vel");
    w.num(e.vel);
    w.end_obj();
}

/// A bass note's text.
pub fn write_bass<S: Sink>(w: &mut Writer<S>, e: &BassEvent) {
    w.obj();
    if e.chug {
        w.key("chug");
        w.bool(true);
    }
    w.key("dur");
    w.num(e.dur);
    w.key("glide");
    w.bool(e.glide);
    w.key("midi");
    w.num(e.midi);
    w.key("step");
    w.num(e.step);
    if let Some(s) = &e.steps {
        w.key("steps");
        w.nums(s);
    }
    w.key("vel");
    w.num(e.vel);
    w.key("voice");
    w.str(e.voice);
    w.end_obj();
}

/// A melody note's text.
pub fn write_melody<S: Sink>(w: &mut Writer<S>, e: &melody::MelodyEvent) {
    w.obj();
    w.key("degree");
    w.num(e.degree);
    if let Some(d) = e.detune {
        w.key("detune");
        w.num(d);
    }
    w.key("dur");
    w.num(e.dur);
    if let Some(h) = e.harm {
        w.key("harm");
        w.str(h);
    }
    w.key("midi");
    w.num(e.midi);
    if let Some(o) = e.orn {
        w.key("orn");
        w.str(o);
    }
    w.key("phrase");
    w.num(e.phrase);
    if let Some(p) = e.prev {
        w.key("prev");
        w.num(p);
    }
    w.key("step");
    w.num(e.step);
    if let Some(s) = &e.steps {
        w.key("steps");
        w.nums(s);
    }
    w.key("vel");
    w.num(e.vel);
    w.key("voice");
    w.str(e.voice);
    w.key("vowel");
    w.str(e.vowel);
    w.end_obj();
}

/// A texture event's text.
pub fn write_texture<S: Sink>(w: &mut Writer<S>, e: &TextureEvent) {
    w.obj();
    if let Some(b) = e.band {
        w.key("band");
        w.num(b);
    }
    w.key("dur");
    w.num(e.dur);
    w.key("kind");
    w.str(e.kind);
    w.key("notes");
    w.list();
    for &n in e.notes.iter() {
        w.num(f64::from(n));
    }
    w.end_list();
    w.key("step");
    w.num(e.step);
    w.key("vel");
    w.num(e.vel);
    w.end_obj();
}

/// `+x.toFixed(3)`: `x` at three decimals, as JavaScript writes it (the
/// decimal nearest the double's exact value, the larger of two equally
/// near), read back.
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss, clippy::cast_possible_wrap)]
pub fn fixed3(x: f64) -> f64 {
    if !x.is_finite() || x.abs() >= 1e15 {
        return x;
    }
    let neg = x < 0.0;
    let a = x.abs();
    let bits = a.to_bits();
    let exp = ((bits >> 52) & 0x7ff) as i32;
    let frac = bits & ((1u64 << 52) - 1);
    let (m, e) = if exp == 0 { (frac, -1074) } else { (frac | (1u64 << 52), exp - 1075) };
    // a = m * 2^e. n/1000 nearest a: compare |n * 2^k - 1000 m| with k = -e.
    let n = if e >= 0 {
        (a * 1000.0) as u128
    } else {
        let k = -e;
        if k > 100 {
            0
        } else {
            let scaled = u128::from(m) * 1000;
            let guess = scaled >> k;
            let dist = |n: u128| (n << k).abs_diff(scaled);
            if dist(guess + 1) <= dist(guess) { guess + 1 } else { guess }
        }
    };
    let v = n as f64 / 1000.0;
    if neg { -v } else { v }
}

#[cfg(test)]
mod tests {
    use super::fixed3;

    #[test]
    fn to_fixed_rounds_as_javascript_does() {
        // Each checked against Node's `x.toFixed(3)`: 0.1235 is just under
        // the half as a double, 0.0005 and 0.2225 just over.
        for (x, want) in [(0.1234, 0.123), (0.1235, 0.123), (1.0005, 1.0), (0.25, 0.25), (0.0005, 0.001), (0.1625, 0.163), (0.2225, 0.223)] {
            assert_eq!(fixed3(x), want, "{x}");
        }
    }
}
