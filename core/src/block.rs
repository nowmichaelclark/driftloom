//! The building-block interface (queue item 46; `docs/CORE.md`, promise 5).
//!
//! A *block* is one piece of sound with its own state: an oscillator, a
//! filter, a source playing the noise, an instrument's body, a compressor,
//! or a whole voice. It is set once when it starts and then rendered a
//! block of `QUANTUM` frames at a time. Every block in the core implements
//! [`Block`], and every one is named in one list, the registry in
//! `blocks/mod.rs`, which is how the core finds the voices a host plays and
//! how the test kit finds everything it checks. Nothing else in the core
//! knows a block's insides.
//!
//! Three things to know, and only three:
//!
//! - [`Block`]: what a block takes in and puts out (one [`Signal`], in
//!   place), what sets it (its `Settings`), and when it sounds (its span).
//!   A [`Voice`] is a block whose settings are a [`Note`].
//! - [`Context`]: what a block may read besides its own state -- the sample
//!   rate, the wavetables and the noise -- and nothing else.
//! - The registry (`blocks`): one line a block. The line declares the
//!   block's file too, so a new block is one new file plus one line.
//!
//! A block holds all its state by value, in a plain struct whose
//! `const fn new()` is all zeros, so a pool of them costs nothing in the
//! `.wasm`. Nothing allocates, nothing is boxed, and nothing is reached
//! through `dyn`: the core dispatches on a `match` the registry writes, and
//! a voice calls its parts directly, by type.

use crate::rng::{Draws, Key};
use crate::wave::Waves;
use crate::{EXTRA, QUANTUM};

/// One block of samples, mono: the unit every block works in.
pub type Signal = [f32; QUANTUM];

/// What every block may read while it starts and renders, besides its own
/// state. Read-only, built once when the core starts at a rate: the same
/// for every block, so a block never reaches into the core for anything.
#[derive(Clone, Copy)]
pub struct Context<'a> {
    /// The sample rate, in Hz.
    pub rate: f64,
    /// The wavetables every oscillator reads (`wave`).
    pub waves: &'a Waves,
    /// The noise every breath, strike and drum plays (`noise`); empty at a
    /// rate too high to hold it (see [`Voice::uses_noise`]).
    pub noise: &'a [f32],
}

/// One way the test kit plays a block: a name for the report, and the
/// settings to start it with. A source is rendered until its span ends; a
/// processor is fed the core's noise for a second.
#[derive(Clone, Copy)]
pub struct Probe<S: 'static> {
    /// What the probe is, in plain ASCII (`"c4"`, `"kick"`).
    pub name: &'static str,
    /// What the block is started with.
    pub settings: S,
}

/// A building block.
///
/// It takes in and puts out one [`Signal`], in place: a *source* (an
/// oscillator, a noise grain, a voice) adds its sound into the signal; a
/// *processor* (`INPUT = true`: a filter, a body, a compressor) replaces
/// the signal with its output. It is set by its `Settings`, a plain value
/// handed over at `start`; inside a voice, a setting that moves over time
/// is a `param::Param`, as Web Audio's `AudioParam` is.
///
/// Every block also has an inherent `const fn new()`, all zeros, and
/// `Default`: the registry builds pools of blocks in a `const`, which a
/// trait cannot promise.
pub trait Block {
    /// What sets the block when it starts. A voice's is the [`Note`].
    type Settings: Copy + 'static;

    /// How the test kit plays it (`blocks::kit`): at least one probe. The
    /// kit checks each one's level, tone, cost and determinism, and nulls it
    /// against its stored render, with no test written by hand.
    const PROBES: &'static [Probe<Self::Settings>];

    /// Whether it works on the signal it is given (a processor) rather than
    /// adding its own sound to it (a source).
    const INPUT: bool = false;

    /// Start, or start again, with `settings`. Sets every value a render
    /// reads, so a block taken from a pool carries nothing over.
    fn start(&mut self, settings: &Self::Settings, cx: &Context);

    /// The frames it sounds in: the first, and the one it has certainly
    /// finished before. The core frees a voice once a block has passed its
    /// end. A processor that runs for as long as it is fed says
    /// `(0, u64::MAX)`.
    fn span(&self) -> (u64, u64);

    /// Render the `QUANTUM` frames from frame `block` (a multiple of
    /// `QUANTUM`, in order) into `signal`.
    fn render(&mut self, block: u64, cx: &Context, signal: &mut Signal);
}

/// A note, as a host hands it to `Core::note`: everything a voice is set by.
#[derive(Clone, Copy, Debug)]
pub struct Note {
    /// The voice's number (the registry's, as `KALIMBA`): a type that plays
    /// several voices tells them apart by it.
    pub voice: u32,
    /// The channel it plays into (`MELODY` and the rest).
    pub channel: u32,
    /// When it starts, in seconds on the host's clock.
    pub time: f64,
    /// Its pitch, as a MIDI note number (fractions are cents).
    pub midi: f64,
    /// How long it is held, in seconds; a voice rings on past it.
    pub dur: f64,
    /// How hard it is played, 0 to 1.
    pub vel: f64,
    /// The voice's own small number: which parts the voice budget let
    /// through, or which drum, bass or texture it is.
    pub parts: u32,
    /// The voice's own values: what the host decides (the note a fiddle is
    /// joined to), and, once settled, the draws the note leaves to chance.
    /// Each voice's file says what it reads where.
    pub extra: [f64; EXTRA],
    /// Where the note sits in its loop, for every draw it makes (`rng`).
    pub draws: Draws,
    /// When a later strum damps it, if one came before it started: only a
    /// strummed nylon string is damped (`Core::damp`).
    pub damp: Option<f64>,
}

impl Note {
    /// A note as the test kit's probes play one: at 0.05 s, held 0.4 s, at
    /// velocity 0.8 (`measure.mjs --voice`), with fixed draws.
    #[must_use]
    pub const fn probe(voice: u32, channel: u32, midi: f64, parts: u32, extra: [f64; EXTRA]) -> Note {
        Note {
            voice,
            channel,
            time: 0.05,
            midi,
            dur: 0.4,
            vel: 0.8,
            parts,
            extra,
            draws: Draws::new(Key { seed: 46, pass: 0, step: 0, ord: 0 }, channel),
            damp: None,
        }
    }
}

/// No extra values: what a voice that reads none is handed.
pub const NO_EXTRA: [f64; EXTRA] = [f64::NAN; EXTRA];

/// A block played by a note: a voice the host plays by number.
///
/// Its three hooks are optional; a voice that draws nothing, plays no noise
/// and has no body writes `impl Voice for MyVoice {}`.
pub trait Voice: Block<Settings = Note> {
    /// The body it plays through, one of `blocks::body`'s kinds: its notes
    /// sum into that body's bus in their channel, and the core runs the
    /// body once on the sum (as `_body()` in synth.js), not once a note.
    const BODY: Option<usize> = None;

    /// Settle what the note leaves to chance, as it arrives: overwrite the
    /// places in `note.extra` the voice reads its draws from, from
    /// `note.draws` and nothing else, so the same note makes the same
    /// draws on every device.
    fn settle(note: &mut Note) {
        let _ = note;
    }

    /// Whether the note plays the core's noise, which the core may not
    /// have (a rate too high to hold it): such a note is refused, and the
    /// host plays it itself.
    #[must_use]
    fn uses_noise(note: &Note) -> bool {
        let _ = note;
        false
    }
}

/// Play one block alone, as the test kit and `dlrender --block` do: start
/// it with `settings`, then render it from frame 0, a block at a time, and
/// hand each block to `take`. A source plays until its span ends, or for
/// `longest` seconds at most; a processor plays one second of the core's
/// noise (`feed`). Returns how many blocks it rendered.
// A few seconds of frames, a positive count well inside a u64.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn solo<B: Block>(
    block: &mut B,
    settings: &B::Settings,
    cx: &Context,
    longest: f64,
    mut take: impl FnMut(&Signal),
) -> u64 {
    block.start(settings, cx);
    let q = QUANTUM as u64;
    let seconds = |s: f64| (cx.rate * s) as u64;
    let blocks = if B::INPUT {
        seconds(1.0).div_ceil(q)
    } else {
        block.span().1.min(seconds(longest)).div_ceil(q)
    };
    for n in 0..blocks {
        let frame = n * q;
        let mut signal = [0.0; QUANTUM];
        if B::INPUT {
            feed(frame, cx, &mut signal);
        }
        block.render(frame, cx, &mut signal);
        take(&signal);
    }
    blocks
}

/// What a processor is fed when it plays alone: the core's noise from
/// `frame` on, looped, at a quarter of its level.
pub fn feed(frame: u64, cx: &Context, signal: &mut Signal) {
    let len = cx.noise.len() as u64;
    if len == 0 {
        return;
    }
    for (i, x) in (frame..).zip(signal.iter_mut()) {
        let at = usize::try_from(i % len).unwrap_or(0);
        *x = cx.noise.get(at).copied().unwrap_or(0.0) * 0.25;
    }
}
