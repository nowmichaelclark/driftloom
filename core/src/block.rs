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
//!   A [`Voice`] is a block whose settings are a [`Note`]; a [`Channel`]
//!   block is one a channel holds for as long as the core runs.
//!   Four seams are optional parts of it (queue items 46b and 46c), each
//!   with a default, so a block that does not use one writes nothing for it:
//!   - **modulation:** a signal into a setting ([`Block::MODS`],
//!     [`Block::render_mod`]), as Web Audio connects a node into an
//!     `AudioParam`;
//!   - **stereo:** a second signal ([`Stereo`], [`Block::render_stereo`]),
//!     which the core's mono render never calls, so it pays nothing for it;
//!   - **channel events:** a damp, a release, a bend or a retune sent to
//!     a channel ([`Event`], [`Block::HEARS`], [`Block::event`]), each kind
//!     reaching its notes once or every time ([`event::reach`]);
//!   - **settings for the loop:** numbers a host sets on a channel's own
//!     block ([`Block::SETTINGS`], [`Block::set`]).
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

use crate::blocks::osc::frame_at;
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

/// What a voice sends past its channel (`Voice::SENDS`): into the reverb,
/// then into the echo, beside what its channel's own sends carry.
pub type Sends = [Signal; 2];

/// Make `sends` ready for this block's first sender: zeros the first time a
/// block asks (`sent` says whether one already has), so a block in which
/// nothing sends leaves the mix exactly as it was.
pub fn open_sends(sends: &mut Sends, sent: &mut bool) {
    if !*sent {
        *sends = [[0.0; QUANTUM]; 2];
        *sent = true;
    }
}

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

/// The kinds of channel event (`Event::kind`), and how far each reaches.
/// The numbers are the host's.
pub mod event {
    /// A damp: the note lets go at the event's time, on its own short
    /// release (a new strum stopping the strings of the one before).
    /// Reaches each note once.
    pub const DAMP: u32 = 0;
    /// A release: the note lets go at the event's time, on its own
    /// release, as if its length had ended there. Reaches each note once.
    pub const RELEASE: u32 = 1;
    /// A bend: the note's pitch moves by `value` cents from the event's
    /// time. Reaches every held note, every time.
    pub const BEND: u32 = 2;
    /// A retune: a held instrument moves to the chord in `notes` at the
    /// event's time (queue item 46c; the organ, item 61). Notes the two
    /// chords share keep sounding, untouched; only the changed ones are let
    /// go and spoken again (`blocks::retune`). Reaches every held note,
    /// every time.
    pub const RETUNE: u32 = 3;
    /// How many kinds there are.
    pub const KINDS: usize = 4;
    /// The kinds that reach each note once are numbered first, `0..ONCE`:
    /// a note still waiting keeps one of each of these (`Core::event`).
    pub const ONCE: usize = 2;
    /// The most notes a retune carries: a chord.
    pub const NOTES: usize = 6;

    /// How far an event reaches the notes of its channel.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum Reach {
        /// Each note hears it at most once: the notes that arrived since
        /// the channel's last event of the kind (a damp reaches the strum
        /// before it and no further). Sent as it comes, to the notes
        /// sounding and waiting.
        Once,
        /// Every note held when its time comes hears it, as often as it is
        /// sent (a retune at every chord, a pitch wheel). Delivered when due.
        Every,
    }

    /// How far `kind` reaches, or `None` for a kind the core does not have.
    #[must_use]
    pub const fn reach(kind: u32) -> Option<Reach> {
        match kind {
            DAMP | RELEASE => Some(Reach::Once),
            BEND | RETUNE => Some(Reach::Every),
            _ => None,
        }
    }

    /// The kind's name, in plain ASCII: the test kit's rows say it
    /// (`nylon/strum@damp`).
    #[must_use]
    pub const fn name(kind: u32) -> &'static str {
        match kind {
            DAMP => "damp",
            RELEASE => "release",
            BEND => "bend",
            RETUNE => "retune",
            _ => "unknown",
        }
    }
}

/// Something a host sends to a channel (`Core::event`) rather than to one
/// note: which kind (`event::DAMP` and the rest), when, its value (a bend's
/// cents; nothing for the others) and, for a retune, the chord. It reaches
/// the notes of the channel as its kind says (`event::reach`, through
/// `Block::event`), and the channel's own blocks when its time comes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Event {
    /// Which kind (`event::DAMP` and the rest).
    pub kind: u32,
    /// When, in seconds on the host's clock. A note already sounding when
    /// the event arrives hears it at this time, or at the next block if
    /// that has been rendered; a note still waiting hears it as it starts.
    pub time: f64,
    /// Its value: a bend's cents.
    pub value: f64,
    /// A retune's chord, as MIDI note numbers, NaN past its last note;
    /// nothing for the other kinds.
    pub notes: [f64; event::NOTES],
}

impl Event {
    /// An event of `kind` at `time` with `value`, and no chord.
    #[must_use]
    pub const fn new(kind: u32, time: f64, value: f64) -> Self {
        Event { kind, time, value, notes: [f64::NAN; event::NOTES] }
    }

    /// A retune to `chord` at `time` (its first `event::NOTES` notes).
    #[must_use]
    pub fn retune(time: f64, chord: &[f64]) -> Self {
        let mut e = Event::new(event::RETUNE, time, 0.0);
        for (n, &m) in e.notes.iter_mut().zip(chord.iter()) {
            *n = m;
        }
        e
    }

    /// A retune's chord: its notes up to the first NaN.
    #[must_use]
    pub fn chord(&self) -> &[f64] {
        let len = self.notes.iter().position(|m| m.is_nan()).unwrap_or(event::NOTES);
        self.notes.get(..len).unwrap_or(&[])
    }
}

/// A setting a host sets on a block for the loop (`Block::SETTINGS`): a
/// channel's own block's sweep period, depth or registration, through
/// `Core::set` (queue item 46c).
#[derive(Clone, Copy)]
pub struct Setting {
    /// What the setting is, in plain ASCII (`"rate"`), for the kit's report.
    pub name: &'static str,
    /// What the test kit sets it to when it probes it: a value a listener
    /// would hear differ from the probe's own, inside what the block takes.
    pub probe: f64,
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

    /// The kinds of channel event it listens to (`event::DAMP` and the
    /// rest): the kit sends each one to every probe (`probe_events`) and
    /// fails a kind that changes nothing. A block that hears none says
    /// nothing.
    const HEARS: &'static [u32] = &[];

    /// Hear a channel event (`Core::event`), at `event.time`: a damp, a
    /// release, a bend, a retune. A voice hears it as its kind reaches
    /// (`event::reach`): a note still waiting hears a once-kind right after
    /// `start`, and an every-kind when its time comes, if it is held then.
    /// A channel's own block hears every event sent to its channel when its
    /// time comes, before the block holding that time renders. Ignoring a
    /// kind is the default.
    fn event(&mut self, event: &Event) {
        let _ = event;
    }

    /// The settings a host sets for the loop (`Core::set`), by number: the
    /// kit probes each one at its `Setting::probe`.
    const SETTINGS: &'static [Setting] = &[];

    /// Setting number `setting` becomes `value` at `time` (seconds on the
    /// host's clock). The core hands it over when its time comes, before
    /// the block holding that time renders, so a block applies it from
    /// that frame, or from the block's start. A number past `SETTINGS` is
    /// ignored, as is everything by default.
    fn set(&mut self, setting: usize, value: f64, time: f64, cx: &Context) {
        let _ = (setting, value, time, cx);
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
    const HEARS: &'static [u32] = B::HEARS;
    const SETTINGS: &'static [Setting] = B::SETTINGS;
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

    fn event(&mut self, event: &Event) {
        self.0.event(event);
        self.1.event(event);
    }

    fn set(&mut self, setting: usize, value: f64, time: f64, cx: &Context) {
        self.0.set(setting, value, time, cx);
        self.1.set(setting, value, time, cx);
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
/// Its hooks are optional; a voice that draws nothing, plays no noise,
/// plays through nothing and sends nothing of its own writes
/// `impl Voice for MyVoice {}`. It hears channel events through
/// `Block::event`, as every block may.
pub trait Voice: Block<Settings = Note> {
    /// The channel's own block it plays through (`blocks::own`, the
    /// registry's `channel` list: today the bodies): its notes sum into that
    /// block's bus in their channel, and the core runs the block once on the
    /// sum (as `_body()` in synth.js), not once a note.
    const THROUGH: Option<usize> = None;

    /// Whether it sends some of itself into the reverb and the echo directly
    /// (`render_sends`), past its channel's own sends: a stab thrown into
    /// the echo (queue item 60).
    const SENDS: bool = false;

    /// Render as `Block::render` does, and add what it sends into `sends`
    /// (`Sends`: the reverb's input, then the echo's). Called instead of
    /// `render` when `SENDS` is true; by default, `render` and nothing sent.
    fn render_sends(&mut self, block: u64, cx: &Context, out: &mut Signal, sends: &mut Sends) {
        let _ = sends;
        self.render(block, cx, out);
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

/// A channel's own block (the registry's `channel` list): a block one of
/// which every channel holds for as long as the core runs, outside the
/// voice pool and its budget -- a body its voices sum into, a processor on
/// the whole channel (a sweep, a duck), or a source that lives for the loop
/// (an organ, a wash). The host reaches it by its number (`blocks::own`):
/// its `SETTINGS` (`Core::set`), the channel's events (`Core::event`), and
/// signals from other channels into its `MODS` (`Core::route`).
pub trait Channel: Block {
    /// Whether it works on the channel's whole signal -- every voice, every
    /// bus and what the host played into it -- rather than on its own bus,
    /// which only the voices that name it in `Voice::THROUGH` sum into. A
    /// whole-channel block runs after every bus, in the registry's order.
    const WHOLE: bool = false;

    /// Whether it sends some of itself into the reverb and the echo directly
    /// (`render_sends`), past its channel's own sends: the organ's room
    /// (queue item 61).
    const SENDS: bool = false;

    /// Render as `Block::render_mod` does, and add what it sends into
    /// `sends` (the reverb's input, then the echo's). Called instead when
    /// `SENDS` is true; by default, nothing sent.
    fn render_sends(&mut self, block: u64, cx: &Context, signal: &mut Signal, mods: &Mods, sends: &mut Sends) {
        let _ = sends;
        self.render_mod(block, cx, signal, mods);
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
    /// Its mono render, hearing the events of kind `n` in `probe_events`,
    /// each before the block holding its time renders (`Block::event`).
    Heard(u32),
    /// Its mono render with setting number `n` of its `SETTINGS` set to its
    /// probe value at time 0 (`Block::set`).
    Set(usize),
}

/// The events the test kit sends a block that hears a kind (`Block::HEARS`),
/// for every probe: a damp or a release a quarter second in; a bend up a
/// semitone, down one, and back; and a retune five times, through chords
/// that share some notes and not others.
#[must_use]
pub fn probe_events(kind: u32) -> &'static [Event] {
    const NAN: f64 = f64::NAN;
    const fn chord(time: f64, n: [f64; 4]) -> Event {
        Event { kind: event::RETUNE, time, value: 0.0, notes: [n[0], n[1], n[2], n[3], NAN, NAN] }
    }
    const DAMP: [Event; 1] = [Event::new(event::DAMP, 0.25, 0.0)];
    const RELEASE: [Event; 1] = [Event::new(event::RELEASE, 0.25, 0.0)];
    const BEND: [Event; 3] = [
        Event::new(event::BEND, 0.15, 100.0),
        Event::new(event::BEND, 0.3, -100.0),
        Event::new(event::BEND, 0.45, 0.0),
    ];
    const RETUNE: [Event; 5] = [
        chord(0.2, [60.0, 65.0, 69.0, NAN]),
        chord(0.4, [62.0, 65.0, 69.0, NAN]),
        chord(0.6, [59.0, 62.0, 67.0, NAN]),
        chord(0.8, [60.0, 64.0, 67.0, 72.0]),
        chord(1.0, [57.0, 60.0, 64.0, NAN]),
    ];
    match kind {
        event::DAMP => &DAMP,
        event::RELEASE => &RELEASE,
        event::BEND => &BEND,
        event::RETUNE => &RETUNE,
        _ => &[],
    }
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
    if let Play::Set(n) = play
        && let Some(s) = B::SETTINGS.get(n)
    {
        block.set(n, s.probe, 0.0, cx);
    }
    let mut events = match play {
        Play::Heard(kind) => probe_events(kind),
        _ => &[],
    }
    .iter()
    .peekable();
    for n in 0..blocks {
        let frame = n * q;
        let mut signal = [0.0; QUANTUM];
        if B::INPUT {
            feed(frame, cx, &mut signal);
        }
        while let Some(e) = events.next_if(|e| frame_at(e.time, cx.rate) < frame + q) {
            block.event(e);
        }
        match play {
            Play::Mono | Play::Heard(_) | Play::Set(_) => {
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
