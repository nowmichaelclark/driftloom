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
//!   Three seams are optional parts of it (queue item 46b), each with a
//!   default, so a block that does not use one writes nothing for it:
//!   - **modulation:** a signal into a setting ([`Block::MODS`],
//!     [`Block::render_mod`]), as Web Audio connects a node into an
//!     `AudioParam`;
//!   - **stereo:** a second signal ([`Stereo`], [`Block::render_stereo`]),
//!     which the core's mono render never calls, so it pays nothing for it;
//!   - **channel events:** a damp, a release or a bend sent to the notes
//!     still held in a channel ([`Event`], [`Voice::event`]).
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

use crate::math;
use crate::rng::{Draws, Key};
use crate::wave::Waves;
use crate::{EXTRA, QUANTUM};

/// One block of samples, mono: the unit every block works in.
pub type Signal = [f32; QUANTUM];

/// One block of samples, stereo: the left side, then the right. A second
/// signal beside the first rather than a wider `Signal`, so the mono render
/// keeps its shape and pays nothing for stereo (`docs/CORE.md`, promise 9).
pub type Stereo = [Signal; 2];

/// A setting of a block that a signal can drive (`Block::MODS`).
#[derive(Clone, Copy)]
pub struct Mod {
    /// What the setting is, in plain ASCII (`"freq"`), for the kit's report.
    pub name: &'static str,
    /// How far the test kit swings it when it probes it, in the setting's
    /// own unit (Hz, cents, dB...): a swing a listener would hear, inside
    /// what the block takes.
    pub swing: f32,
}

/// The most settings one block lets a signal drive.
pub const MAX_MODS: usize = 4;

/// What drives a block's settings for one render: for each setting in its
/// `MODS`, by number, a signal or nothing. At every frame a driven setting
/// is its own value -- its automation, if it has some -- plus the signal,
/// as Web Audio sums an `AudioParam`'s value and the nodes connected into
/// it. A setting with a signal connected moves for the whole block, even
/// where the signal is zero (Chromium's rule, which an oscillator's start
/// depends on). Several signals into one setting are summed by whoever
/// connects them. Borrowed, on the stack: nothing allocates.
#[derive(Clone, Copy)]
pub struct Mods<'a>([Option<&'a Signal>; MAX_MODS]);

impl<'a> Mods<'a> {
    /// No setting driven: the block renders as `Block::render` does.
    pub const NONE: Mods<'static> = Mods([None; MAX_MODS]);

    /// These, with `signal` driving setting number `setting` (a number past
    /// `MAX_MODS` is ignored).
    #[must_use]
    pub fn with(mut self, setting: usize, signal: &'a Signal) -> Self {
        if let Some(slot) = self.0.get_mut(setting) {
            *slot = Some(signal);
        }
        self
    }

    /// The signal driving setting number `setting`, if one does.
    #[must_use]
    pub fn get(&self, setting: usize) -> Option<&'a Signal> {
        self.0.get(setting).copied().flatten()
    }
}

/// The kinds of channel event (`Event::kind`). The numbers are the host's.
pub mod event {
    /// A damp: the note lets go at the event's time, on its own short
    /// release (a new strum stopping the strings of the one before).
    pub const DAMP: u32 = 0;
    /// A release: the note lets go at the event's time, on its own
    /// release, as if its length had ended there.
    pub const RELEASE: u32 = 1;
    /// A bend: the note's pitch moves by `value` cents from the event's
    /// time.
    pub const BEND: u32 = 2;
    /// How many kinds there are.
    pub const KINDS: usize = 3;
}

/// Something a host sends to the notes of a channel (`Core::event`) rather
/// than to one note: which kind (`event::DAMP` and the rest), when, and its
/// value (a bend's cents; nothing for the others). Each note still held at
/// `time` that arrived since the channel's last event of the same kind
/// hears it, once (`Voice::event`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Event {
    /// Which kind (`event::DAMP`, `event::RELEASE`, `event::BEND`).
    pub kind: u32,
    /// When, in seconds on the host's clock. A note already sounding when
    /// the event arrives hears it at this time, or at the next block if
    /// that has been rendered; a note still waiting hears it as it starts.
    pub time: f64,
    /// Its value: a bend's cents.
    pub value: f64,
}

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
/// is a `param::Param`, as Web Audio's `AudioParam` is, and a setting a
/// signal drives is one of its `MODS` (`render_mod`).
///
/// A block is mono. In a stereo signal ([`Stereo`], `render_stereo`) a mono
/// source plays centered and a mono processor hears its two sides folded
/// to one; a block with a stereo render of its own says `STEREO`.
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

    /// The settings a signal can drive, by number: `mods.get(i)` in
    /// `render_mod` drives `MODS[i]`. The kit probes each one.
    const MODS: &'static [Mod] = &[];

    /// Render as `render` does, with the signals in `mods` driving the
    /// settings in `MODS` (see [`Mods`]). A block with `MODS` writes this,
    /// and its `render` is `render_mod` with [`Mods::NONE`], which the
    /// compiler folds away; one without leaves both as they are.
    fn render_mod(&mut self, block: u64, cx: &Context, signal: &mut Signal, mods: &Mods) {
        let _ = mods;
        self.render(block, cx, signal);
    }

    /// Whether it has a stereo render of its own (`render_stereo`): a
    /// panner, a room. Its `render` is then what a mono render plays.
    const STEREO: bool = false;

    /// Render the block from frame `block` into a stereo signal, both
    /// sides. Called only by a stereo render, never by the core's mono one,
    /// so a block pays nothing for it unless it is used.
    ///
    /// By default, for a mono block: a source renders once and adds the
    /// same samples into both sides (centered: each side is its mono render,
    /// bit for bit); a processor hears the two sides folded to one,
    /// `(left + right) / 2`, and puts its output on both (fed the same on
    /// both sides, each side is its mono render, bit for bit). For a mono
    /// processor that should run on each side apart, use [`Pair`].
    fn render_stereo(&mut self, block: u64, cx: &Context, signal: &mut Stereo) {
        let [left, right] = signal;
        let mut mono = [0.0; QUANTUM];
        if Self::INPUT {
            for ((m, l), r) in mono.iter_mut().zip(left.iter()).zip(right.iter()) {
                *m = f32::midpoint(*l, *r);
            }
            self.render(block, cx, &mut mono);
            *left = mono;
            *right = mono;
        } else {
            self.render(block, cx, &mut mono);
            for ((l, r), m) in left.iter_mut().zip(right.iter_mut()).zip(mono.iter()) {
                *l += *m;
                *r += *m;
            }
        }
    }
}

/// A mono processor as dual mono: one copy of it on each side of a stereo
/// signal, each with its own state. Its mono render is the left copy's.
pub struct Pair<B>(pub B, pub B);

impl<B: Block> Block for Pair<B> {
    type Settings = B::Settings;
    const PROBES: &'static [Probe<B::Settings>] = B::PROBES;
    const INPUT: bool = B::INPUT;
    const MODS: &'static [Mod] = B::MODS;
    const STEREO: bool = true;

    fn start(&mut self, settings: &Self::Settings, cx: &Context) {
        self.0.start(settings, cx);
        self.1.start(settings, cx);
    }

    fn span(&self) -> (u64, u64) {
        self.0.span()
    }

    fn render(&mut self, block: u64, cx: &Context, signal: &mut Signal) {
        self.0.render(block, cx, signal);
    }

    fn render_mod(&mut self, block: u64, cx: &Context, signal: &mut Signal, mods: &Mods) {
        self.0.render_mod(block, cx, signal, mods);
    }

    fn render_stereo(&mut self, block: u64, cx: &Context, signal: &mut Stereo) {
        let [left, right] = signal;
        self.0.render(block, cx, left);
        self.1.render(block, cx, right);
    }
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
        }
    }
}

/// No extra values: what a voice that reads none is handed.
pub const NO_EXTRA: [f64; EXTRA] = [f64::NAN; EXTRA];

/// A block played by a note: a voice the host plays by number.
///
/// Its four hooks are optional; a voice that draws nothing, plays no noise,
/// plays through nothing and hears no channel event writes
/// `impl Voice for MyVoice {}`.
pub trait Voice: Block<Settings = Note> {
    /// The channel's own block it plays through (`blocks::own`, the
    /// registry's `channel` list: today the bodies): its notes sum into that
    /// block's bus in their channel, and the core runs the block once on the
    /// sum (as `_body()` in synth.js), not once a note.
    const THROUGH: Option<usize> = None;

    /// Hear a channel event (`Core::event`), at `event.time`: a damp, a
    /// release or a bend. A voice that does nothing with a kind ignores it,
    /// which is the default. Called once the note has started; a note that
    /// heard the event while it waited hears it right after `start`.
    fn event(&mut self, event: &Event) {
        let _ = event;
    }

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

/// How `solo` plays a block alone (the test kit, `dlrender --block`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Play {
    /// Its mono render, `Block::render`.
    Mono,
    /// Its mono render with setting number `n` of its `MODS` driven by
    /// `drive`: `Block::render_mod`.
    Driven(usize),
    /// Its stereo render, `Block::render_stereo`, a processor fed the same
    /// on both sides; each block handed over is the two sides interleaved,
    /// left first.
    Stereo,
}

/// Play one block alone, as the test kit and `dlrender --block` do: start
/// it with `settings`, then render it from frame 0, a block at a time, the
/// way `play` says, and hand each block to `take`. A source plays until its
/// span ends, or for `longest` seconds at most; a processor plays one second
/// of the core's noise (`feed`). Returns how many blocks it rendered.
// A few seconds of frames, a positive count well inside a u64.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn solo<B: Block>(
    block: &mut B,
    settings: &B::Settings,
    cx: &Context,
    longest: f64,
    play: Play,
    mut take: impl FnMut(&[f32]),
) -> u64 {
    block.start(settings, cx);
    let q = QUANTUM as u64;
    let seconds = |s: f64| (cx.rate * s) as u64;
    let blocks = if B::INPUT {
        seconds(1.0).div_ceil(q)
    } else {
        block.span().1.min(seconds(longest)).div_ceil(q)
    };
    let swing = |n: usize| B::MODS.get(n).map_or(0.0, |m| m.swing);
    for n in 0..blocks {
        let frame = n * q;
        let mut signal = [0.0; QUANTUM];
        if B::INPUT {
            feed(frame, cx, &mut signal);
        }
        match play {
            Play::Mono => {
                block.render(frame, cx, &mut signal);
                take(&signal);
            }
            Play::Driven(setting) => {
                let mut m = [0.0; QUANTUM];
                drive(frame, swing(setting), cx.rate, &mut m);
                block.render_mod(frame, cx, &mut signal, &Mods::NONE.with(setting, &m));
                take(&signal);
            }
            Play::Stereo => {
                let mut both = [signal, signal];
                block.render_stereo(frame, cx, &mut both);
                let [l, r] = both;
                let mut out = [0.0; 2 * QUANTUM];
                for ((pair, a), b) in out.as_chunks_mut::<2>().0.iter_mut().zip(l.iter()).zip(r.iter()) {
                    *pair = [*a, *b];
                }
                take(&out);
            }
        }
    }
    blocks
}

/// What the test kit drives a setting with when it probes it: a 3 Hz sine
/// of `swing` (the setting's `Mod::swing`), from frame `frame` on.
// A frame count as a float: exact far past any probe's length.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
pub fn drive(frame: u64, swing: f32, rate: f64, signal: &mut Signal) {
    for (i, x) in (frame..).zip(signal.iter_mut()) {
        let t = i as f64 / rate;
        *x = (f64::from(swing) * math::sin(core::f64::consts::TAU * 3.0 * t)) as f32;
    }
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
