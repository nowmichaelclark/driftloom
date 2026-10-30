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
//! gaps).
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

pub mod canon;
pub mod character;
pub mod data;
pub mod js;
pub mod list;
pub mod plan;
pub mod rng;
pub mod spec;
pub mod theory;

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

/// The parts of a loop the generation goldens hash, as far as the Rust
/// generator makes them (`tools/gen-golden.mjs` names each).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    /// The spec, as `newSpec` makes it.
    Spec,
    /// What render plans before the layers (`planOf`).
    Plan,
}

impl Part {
    /// Every part the generator makes, in the file's order.
    pub const ALL: [Part; 2] = [Part::Spec, Part::Plan];

    /// The part's name in the file.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Part::Spec => "spec",
            Part::Plan => "plan",
        }
    }
}

/// The generator's working memory: one loop at a time, every list fixed
/// (no allocation). A host keeps one (a static, as the core is) and
/// `load`s a loop into it.
pub struct Generator {
    spec: spec::Spec,
    plan: plan::Plan,
}

impl Generator {
    /// An empty generator, nothing loaded. `const`, so a host can keep one
    /// in a static.
    #[must_use]
    pub const fn empty() -> Self {
        Generator { spec: spec::Spec::EMPTY, plan: plan::Plan::new() }
    }

    /// An empty generator (`empty`).
    #[must_use]
    pub const fn new() -> Self {
        Self::empty()
    }

    /// Generate the loop of `seed`, with the vibes `vibes` (their numbers,
    /// `data::VIBES`; any order, as a listener's selection).
    pub fn load(&mut self, seed: u32, vibes: &[usize]) {
        let chosen = character::selection(vibes);
        self.spec = spec::new_spec(seed, &chosen);
        let cx = plan::Cx::new(&self.spec);
        self.plan.make(&cx);
    }

    /// The loop's spec.
    #[must_use]
    pub fn spec(&self) -> &spec::Spec {
        &self.spec
    }

    /// Write a part's canonical text into `sink`.
    pub fn write<S: canon::Sink>(&self, part: Part, sink: &mut S) {
        let mut w = canon::Writer::new(sink);
        match part {
            Part::Spec => self.spec.write(&mut w),
            Part::Plan => self.plan.write(&mut w),
        }
    }

    /// A part's hash: FNV-1a 64 over its canonical text.
    #[must_use]
    pub fn hash(&self, part: Part) -> u64 {
        let mut h = canon::Fnv::new();
        self.write(part, &mut h);
        h.0
    }

    /// How many items the working lists dropped for want of room: 0, or
    /// the capacities are wrong.
    #[must_use]
    pub fn lost(&self) -> usize {
        self.spec.mix.lost() + self.spec.feel_mix.lost() + self.plan.lost()
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}
