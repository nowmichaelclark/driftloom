//! What the player reports about the loop it plays (queue item 94a), so a
//! host shows the loop without a generator of its own: the layers' grid of
//! the pass being decided, and the loop's description, everything the
//! app's info card, its cover and a share code read from a spec.

use core::fmt::{self, Write};

use crate::generator::data::{CHARACTERS, MOODS, VIBES};
use crate::generator::plan::{Cx, NOTES_MAX};
use crate::generator::request::{LAYER_NAMES, Length};
use crate::generator::spec::Metre;
use crate::generator::theory::SCALES;
use crate::generator::{LAYERS, Layer, Stage};

use super::{Player, State, loop_of};

/// A note of the loop as composed (`Player::note`), as the app's MIDI export
/// reads `render(spec).tracks`: where it falls, how long it holds and how
/// loud, its pitches (a chord's or a texture's several), and a drum hit's
/// instrument and its offset off the step (a roll's).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Note {
    /// The step, from the loop's first.
    pub step: f64,
    /// How many steps it holds (0 for a drum hit).
    pub dur: f64,
    /// Its velocity, 0 for a note the loop leaves out (an airy form's).
    pub vel: f64,
    /// A drum hit's offset off its step, in steps (0 for others).
    pub micro: f64,
    /// Its pitches, MIDI note numbers: `count` of them.
    pub notes: [f64; NOTES_MAX],
    /// How many pitches (0 for a drum hit).
    pub count: usize,
    /// A drum hit's instrument (`kick`, `hat` and the rest; empty for
    /// others).
    pub drum: &'static str,
}

/// The longest a layer's grid is, in steps: a loop's 32 bars of 20 steps,
/// or a layer's own cycle of 11 bars, whichever is longer.
pub const GRID_MAX: usize = 32 * 20;

impl Player {
    /// Whether a loop is made and can be reported: playing, or stopped with
    /// one made.
    fn made(&self) -> bool {
        match self.state {
            State::Playing => true,
            State::Idle => self.generator.stage() == Stage::Done && self.generator.spec().bars >= 1.0,
            State::Loading | State::Wandering => false,
        }
    }

    /// Layer `layer`'s grid (the app's layers, `stepsForBar`): into `out`,
    /// one velocity a step of the layer's own cycle (the loop's pass, or its
    /// polymeter cycle), the loudest note on each step, 0 for none; of the
    /// pass the player is deciding, as it wanders. Returns the cycle's
    /// steps, or 0 with nothing made or `out` too short (`GRID_MAX` is
    /// always enough). A bar `b` of the grid is steps `b * steps_per_bar`
    /// on, around the cycle.
    ///
    /// The pass being decided is a lookahead ahead of the one heard: a host
    /// reads the grid when the player tells it a pass or a section begins
    /// (`Kind::Pass`, `Kind::Section`), in the same render, and shows it from
    /// that event's time.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // steps, far below 2^53
    pub fn grid(&self, layer: Layer, out: &mut [f64]) -> usize {
        if !self.made() {
            return 0;
        }
        let lp = loop_of(&self.generator, &self.passes, self.wandering.then_some(self.current));
        let cycle = lp.layer_cycle(layer);
        if !(cycle >= 1.0) || cycle as usize > out.len() {
            return 0;
        }
        let n = cycle as usize;
        let row = &mut out[..n];
        row.fill(0.0);
        lp.each_velocity(layer, |step, vel| {
            if vel == 0.0 || vel.is_nan() {
                return;
            }
            let i = (((step % cycle) + cycle) % cycle) as usize;
            if let Some(x) = row.get_mut(i)
                && vel > *x
            {
                *x = vel;
            }
        });
        n
    }

    /// How many notes layer `layer` has in the loop as composed (`note`); 0
    /// with nothing made.
    #[must_use]
    pub fn notes(&self, layer: Layer) -> usize {
        if !self.made() {
            return 0;
        }
        let r = self.generator.render();
        match layer {
            Layer::Drums => r.drums.len(),
            Layer::Bass => r.bass.len(),
            Layer::Chords => r.chords.len(),
            Layer::Melody => r.melody.len(),
            Layer::Texture => r.texture.len(),
        }
    }

    /// Layer `layer`'s note `i` in the loop as composed, as the app's MIDI
    /// export reads it (`patternToMidi(render(spec))`): none past the last.
    #[must_use]
    pub fn note(&self, layer: Layer, i: usize) -> Option<Note> {
        if !self.made() {
            return None;
        }
        let r = self.generator.render();
        let blank = Note { step: 0.0, dur: 0.0, vel: 0.0, micro: 0.0, notes: [0.0; NOTES_MAX], count: 0, drum: "" };
        let one = |step, dur, vel, midi| {
            let mut notes = [0.0; NOTES_MAX];
            notes[0] = midi;
            Note { step, dur, vel, notes, count: 1, ..blank }
        };
        let many = |step, dur, vel, list: &[i16]| {
            let mut notes = [0.0; NOTES_MAX];
            for (slot, &p) in notes.iter_mut().zip(list) {
                *slot = f64::from(p);
            }
            Note { step, dur, vel, notes, count: list.len().min(NOTES_MAX), ..blank }
        };
        match layer {
            Layer::Drums => r.drums.get(i).map(|e| Note { step: e.step, vel: e.vel, micro: e.micro.unwrap_or(0.0), drum: e.inst.get(), ..blank }),
            Layer::Bass => r.bass.get(i).map(|e| one(e.step, e.dur, e.vel, e.midi)),
            Layer::Melody => r.melody.get(i).map(|e| one(e.step, e.dur, e.vel, e.midi)),
            Layer::Chords => r.chords.get(i).map(|e| many(e.step, e.dur, e.vel, &e.notes)),
            Layer::Texture => r.texture.get(i).map(|e| many(e.step, e.dur, e.vel, &e.notes)),
        }
    }

    /// The loop playing (or stopped), described for a host to show and to
    /// write a share code from: plain ASCII lines of `name=value`, numbers
    /// written so they read back to the same double, lists with `|` between
    /// and a name and its weight with `:`. Nothing with no loop made.
    ///
    /// - `seed`, `name`;
    /// - `mix` (the profiles and their weights, the style), `moods` (the
    ///   feelings and theirs), `feel` (lift, energy, warmth), and with vibes
    ///   `vibes` (each and the weight it drew) and `strength`;
    /// - `root` (0 is C), `scale`, `bpm` (the tempo playing), `metre`,
    ///   `steps_per_bar`, `bars` (the loop's), `length` (the length asked:
    ///   bars, or `endless`; only if asked);
    /// - `swing`, `coherence`, `tone` (warmth, space, wobble);
    /// - `layer_seeds` (in the core's layer order), `rolled` (the layers
    ///   re-rolled, by name; only if any), `cycles` (each layer's own, in
    ///   steps, 0 for the loop's; only for a loop with its own cycles);
    /// - `airy` (1 for a loop with an airy form), `drifting` (1 with its own
    ///   cycles), `choir` (1 for a loop whose melody and chords sing),
    ///   `layers` (each layer 1 if it plays notes, 0 if empty).
    ///
    /// # Errors
    /// The writer's.
    pub fn describe<W: Write>(&self, w: &mut W) -> fmt::Result {
        if !self.made() {
            return Ok(());
        }
        let g = &self.generator;
        let spec = g.spec();
        writeln!(w, "seed={}", spec.seed)?;
        writeln!(w, "name={}", spec.name.as_str())?;
        weights(w, "mix", spec.mix.iter().map(|&(k, x)| (CHARACTERS[k].id, x)))?;
        weights(w, "moods", spec.feel_mix.iter().map(|&(k, x)| (MOODS[k].id, x)))?;
        writeln!(w, "feel={}|{}|{}", spec.feel.lift, spec.feel.energy, spec.feel.warmth)?;
        if let Some(v) = &spec.vibes {
            weights(w, "vibes", v.list.iter().map(|&(k, x)| (VIBES[k].id, x)))?;
            writeln!(w, "strength={}", v.strength)?;
        }
        writeln!(w, "root={}", spec.root)?;
        writeln!(w, "scale={}", SCALES[spec.scale].id)?;
        let bpm = if self.seq.bpm > 0.0 && self.state == State::Playing { self.seq.bpm } else { spec.bpm };
        writeln!(w, "bpm={bpm}")?;
        let metre = match Cx::new(spec).metre {
            Metre::FourFour => "4/4",
            Metre::SixEight => "6/8",
            Metre::ThreeFour => "3/4",
            Metre::FiveFour => "5/4",
        };
        writeln!(w, "metre={metre}")?;
        writeln!(w, "steps_per_bar={}", spec.steps_per_bar)?;
        writeln!(w, "bars={}", spec.bars)?;
        match spec.length {
            Some(Length::Bars(n)) => writeln!(w, "length={n}")?,
            Some(Length::Endless) => writeln!(w, "length=endless")?,
            None => {}
        }
        writeln!(w, "swing={}", spec.swing)?;
        writeln!(w, "coherence={}", spec.coherence)?;
        writeln!(w, "tone={}|{}|{}", spec.tone[0], spec.tone[1], spec.tone[2])?;
        let s = &spec.layer_seeds;
        writeln!(w, "layer_seeds={}|{}|{}|{}|{}", s[0], s[1], s[2], s[3], s[4])?;
        if spec.rolled != 0 {
            w.write_str("rolled=")?;
            let mut first = true;
            for (i, name) in LAYER_NAMES.iter().enumerate() {
                if spec.rolled & (1 << i) != 0 {
                    if !first {
                        w.write_char('|')?;
                    }
                    w.write_str(name)?;
                    first = false;
                }
            }
            w.write_char('\n')?;
        }
        if spec.cycles.set {
            let c = |l: usize| spec.cycles.layers[l].unwrap_or(0.0);
            writeln!(w, "cycles={}|{}|{}|{}|{}", c(0), c(1), c(2), c(3), c(4))?;
        }
        let r = g.render();
        writeln!(w, "airy={}", u8::from(r.form.is_some()))?;
        writeln!(w, "drifting={}", u8::from(spec.cycles.set))?;
        writeln!(w, "choir={}", u8::from(spec.choir().is_some()))?;
        let plays = |l: Layer| match l {
            Layer::Drums => !r.drums.is_empty(),
            Layer::Bass => !r.bass.is_empty(),
            Layer::Chords => !r.chords.is_empty(),
            Layer::Melody => !r.melody.is_empty(),
            Layer::Texture => !r.texture.is_empty(),
        };
        let p = LAYERS.map(|l| u8::from(plays(l)));
        writeln!(w, "layers={}|{}|{}|{}|{}", p[0], p[1], p[2], p[3], p[4])
    }
}

/// A line of names and weights: `key=a:0.5|b:0.5`.
fn weights<W: Write>(w: &mut W, key: &str, list: impl Iterator<Item = (&'static str, f64)>) -> fmt::Result {
    write!(w, "{key}=")?;
    for (i, (id, x)) in list.enumerate() {
        if i > 0 {
            w.write_char('|')?;
        }
        write!(w, "{id}:{x}")?;
    }
    w.write_char('\n')
}
