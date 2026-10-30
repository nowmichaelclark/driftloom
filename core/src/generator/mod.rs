//! The generator, in Rust (phase 2, queue items 79 to 82): a loop from its
//! seed and the generation data, with no JavaScript.
//!
//! It is `js/generator.js` ported note for note, and proven so: for every
//! case of the generation goldens (`core/golden/generation.hashes`, queue
//! item 77) it makes the same decisions, hashed over the same canonical
//! text, on every target. Until the player (item 82) is in the app, the
//! JavaScript generator plays and this one is held to it; a change to one
//! is made in both, or waits (`docs/QUEUE.md`, phase 2's rules).
//!
//! **What is here.** Item 79: the spec (`spec::new_spec`: the vibes' draw,
//! drive, the profile mix, the feel, bars, tempo and gait, key and mode) and
//! what `render` plans before any layer plays (`plan`: the development's
//! sections, the harmony and its progression, the entry schedules and the
//! gaps). Item 80: the layers (`layers`: the bass with its register and the
//! pedal, the drums and wayfare's grooves, the texture; `melody`: motifs,
//! cells, phrases and development) and the rest of `render` (`render`: the
//! repairs, the gaps, the per-loop choices the pedal needs, the register
//! work, the spellings, the long chords' octave, the slurs, strums,
//! ornaments and the harmony line). Item 81: each choice's own draws
//! (`choices`: the arc, the tuned kick, subDuck, lateFloor, the floor
//! drop's shapes, padSweep, the echo time, the bass's level), what the
//! engine decides at load (`load`: dubStab, the organ, the wash, the
//! space, and the rest), and "Let the loop wander" (`wander`: each pass's
//! `drift`, its floor drop and its kick). With these, every decision the
//! JavaScript makes before scheduling has its twin here.
//!
//! **The data** (`data/*.txt`) is compiled in by `core/build.rs`, into
//! read-only tables (`data`): the core reads nothing at run time and a host
//! hands nothing over, the tables cost no working memory, and they are the
//! same bytes everywhere. The price is that a data edit (a vibe tuned)
//! rebuilds the core; the app's `.wasm` does not carry the generator yet
//! (the `gen` feature adds its exports to a test build only), so a vibe
//! edit changes nothing the app ships until the player does.
//!
//! **How it stays exact.** Every number is a double, as in JavaScript, and
//! every operation is done in the JavaScript's order; the few functions
//! that differ between the languages are `js`'s, and the transcendental
//! ones are the core's own (`crate::math`, which `js/dlmath.js` mirrors).
//! The random streams are `js/rng.js`'s (`rng`). Nothing allocates: the
//! working memory is fixed lists (`list`), sized where they are declared.

// Comparisons are exact on purpose: the port compares doubles as the
// JavaScript does (`===`, `!(a < b)` where a NaN must fall through, `(a + b)
// / 2` in its own order), and a tolerance or a reordering would give
// another loop.
#![allow(clippy::float_cmp, clippy::neg_cmp_op_on_partial_ord, clippy::manual_midpoint)]
// The port keeps the JavaScript's short names (r, c, e, spb), so the two
// read side by side.
#![allow(clippy::many_single_char_names)]

pub mod canon;
pub mod character;
pub mod choices;
pub mod data;
pub mod js;
pub mod layers;
pub mod list;
pub mod load;
pub mod melody;
pub mod plan;
pub mod render;
pub mod rng;
pub mod spec;
pub mod theory;
pub mod wander;

/// The layers, in the order the generator draws them and the app lists
/// them (`data/voices.txt`'s `[voices layers]`, which `build.rs` checks).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layer {
    Drums = 0,
    Bass = 1,
    Chords = 2,
    Melody = 3,
    Texture = 4,
}

/// Every layer, in order.
pub const LAYERS: [Layer; 5] = [Layer::Drums, Layer::Bass, Layer::Chords, Layer::Melody, Layer::Texture];

impl Layer {
    /// The layer's name, as the JavaScript's keys spell it.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Layer::Drums => "drums",
            Layer::Bass => "bass",
            Layer::Chords => "chords",
            Layer::Melody => "melody",
            Layer::Texture => "texture",
        }
    }
}

/// The parts of a loop the generation goldens hash (`tools/gen-golden.mjs`
/// names each).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    /// The spec, as `newSpec` makes it.
    Spec,
    /// What render plans before the layers (`planOf`).
    Plan,
    /// What render decides besides the layers' notes.
    Form,
    /// A layer's events, after render.
    Layer(Layer),
    /// Render's meta: the voices, bass style, kit, melody cell, texture.
    Meta,
    /// The per-loop choices (`drawChoices`, through render).
    Choices,
    /// What the engine decides at load (`Engine.load`).
    Load,
    /// A wandered pass, from 1 (`drift`, through `Engine._wander(pass)`).
    Pass(u32),
}

/// How many wandered passes the goldens hash.
pub const PASSES: u32 = 8;

impl Part {
    /// Every part the goldens hash, in the file's order: the passes last,
    /// in their order.
    pub const ALL: [Part; 19] = [
        Part::Spec,
        Part::Plan,
        Part::Form,
        Part::Layer(Layer::Drums),
        Part::Layer(Layer::Bass),
        Part::Layer(Layer::Chords),
        Part::Layer(Layer::Melody),
        Part::Layer(Layer::Texture),
        Part::Meta,
        Part::Choices,
        Part::Load,
        Part::Pass(1),
        Part::Pass(2),
        Part::Pass(3),
        Part::Pass(4),
        Part::Pass(5),
        Part::Pass(6),
        Part::Pass(7),
        Part::Pass(8),
    ];

    /// The part's name in the file (every pass is one of `passes`).
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Part::Spec => "spec",
            Part::Plan => "plan",
            Part::Form => "form",
            Part::Layer(l) => l.name(),
            Part::Meta => "meta",
            Part::Choices => "choices",
            Part::Load => "load",
            Part::Pass(_) => "passes",
        }
    }

    /// The part's hash in a line of the goldens (`name=H`, a pass the
    /// pass's own of `passes=H1,...,H8`), if the line has it.
    #[must_use]
    pub fn in_line(self, line: &str) -> Option<&str> {
        let field = line.split(' ').find_map(|f| f.strip_prefix(self.name()).and_then(|f| f.strip_prefix('=')))?;
        match self {
            Part::Pass(n) => field.split(',').nth(usize::try_from(n).ok()?.checked_sub(1)?),
            _ => Some(field),
        }
    }
}

/// The generator's working memory: one loop at a time, every list fixed
/// (no allocation). A host keeps one (a static, as the core is) and
/// `load`s a loop into it.
pub struct Generator {
    spec: spec::Spec,
    plan: plan::Plan,
    render: render::Render,
    settings: load::Load,
    pass: wander::Pass,
}

impl Generator {
    /// An empty generator, nothing loaded. `const`, so a host can keep one
    /// in a static.
    #[must_use]
    pub const fn empty() -> Self {
        Generator {
            spec: spec::Spec::EMPTY,
            plan: plan::Plan::new(),
            render: render::Render::new(),
            settings: load::Load::EMPTY,
            pass: wander::Pass::new(),
        }
    }

    /// An empty generator (`empty`).
    #[must_use]
    pub const fn new() -> Self {
        Self::empty()
    }

    /// Generate the loop of `seed`, with the vibes `vibes` (their numbers,
    /// `data::VIBES`; any order, as a listener's selection), and decide
    /// what the engine decides loading it (`settings`).
    pub fn load(&mut self, seed: u32, vibes: &[usize]) {
        let chosen = character::selection(vibes);
        self.spec = spec::new_spec(seed, &chosen);
        let mut cx = plan::Cx::new(&self.spec);
        // A loop that does not develop keeps the chords' octave today's
        // chord lengths would take (`withTodaysChords`): rendered once that
        // way first, for the chords' centre.
        let today = if plan::development_of(cx.spec, cx.choir).is_none() {
            cx.long_chords_off = true;
            self.plan.make(&cx);
            self.render.run(&cx, &self.plan);
            cx.long_chords_off = false;
            self.render.chord_centre()
        } else {
            None
        };
        self.plan.make(&cx);
        self.render.run(&cx, &self.plan);
        if self.render.harmony.long_chords.is_some()
            && let (Some(a), Some(b)) = (today, self.render.chord_centre())
        {
            let sh = js::round((a - b) / 12.0) * 12.0;
            if sh != 0.0 {
                #[allow(clippy::cast_possible_truncation)] // whole octaves
                self.render.shift_chords(sh as i16);
            }
        }
        self.settings = load::Load::of(&plan::Cx::new(&self.spec), &self.render);
        self.pass.number = None;
    }

    /// The loop's spec.
    #[must_use]
    pub fn spec(&self) -> &spec::Spec {
        &self.spec
    }

    /// What the engine decides loading the loop: its per-loop settings.
    #[must_use]
    pub fn settings(&self) -> &load::Load {
        &self.settings
    }

    /// The loop as pass `pass` plays it when it wanders (`drift`, at the
    /// default amount): drawn now, unless it is the pass drawn last. Any
    /// pass can be drawn without the ones before it.
    pub fn wander(&mut self, pass: u32) -> &wander::Pass {
        if self.pass.number != Some(pass) {
            let kick = self.settings.kick.map(|k| k.kind);
            self.pass.draw(&plan::Cx::new(&self.spec), &self.render, kick, pass);
        }
        &self.pass
    }

    /// Write a part's canonical text into `sink` (a pass is drawn first,
    /// if it is not the one drawn last).
    pub fn write<S: canon::Sink>(&mut self, part: Part, sink: &mut S) {
        if let Part::Pass(n) = part {
            self.wander(n);
        }
        let mut w = canon::Writer::new(sink);
        let cx = plan::Cx::new(&self.spec);
        match part {
            Part::Spec => self.spec.write(&mut w),
            Part::Plan => self.plan.write(&mut w),
            Part::Form => self.render.write_form(&mut w, &cx, &self.plan),
            Part::Layer(l) => self.render.write_layer(&mut w, l),
            Part::Meta => self.render.write_meta(&mut w, &cx, &self.plan),
            Part::Choices => self.render.write_choices(&mut w),
            Part::Load => self.settings.write(&mut w),
            Part::Pass(_) => self.pass.write(&mut w, &cx, &self.plan, &self.render),
        }
    }

    /// A part's hash: FNV-1a 64 over its canonical text.
    #[must_use]
    pub fn hash(&mut self, part: Part) -> u64 {
        let mut h = canon::Fnv::new();
        self.write(part, &mut h);
        h.0
    }

    /// How many items the working lists dropped for want of room: 0, or
    /// the capacities are wrong.
    #[must_use]
    pub fn lost(&self) -> usize {
        self.spec.mix.lost() + self.spec.feel_mix.lost() + self.plan.lost() + self.render.lost()
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}
