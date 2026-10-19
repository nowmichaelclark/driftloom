//! Driftloom's audio core: its own engine, in Rust.
//!
//! It decides nothing and knows nothing about browsers: a host owns a
//! `Core`, starts it at a sample rate (`start`), hands it notes as they are
//! decided (`note`), and asks it for samples (`render`), any number of
//! frames a call. The web app's host is an `AudioWorklet` (`js/worklet.js`,
//! through the WebAssembly exports in `wasm.rs`); a native or game host
//! wraps the same calls (`core/README.md`).
//!
//! The sound is made of building blocks (`block`, `blocks`): voices,
//! oscillators, filters, the noise, the bodies, the compressor. Each is one
//! file behind one small interface, registered in one list, and checked by
//! its own test kit. A signal can drive a block's setting (modulation), a
//! block can have a stereo render, and a host can send an event to the
//! notes of a channel (`event`); the mono render pays nothing for the
//! stereo seam (queue item 46b).
//!
//! Everything is allocated up front: a fixed pool of voices, fixed-size
//! automation, fixed output buffers. Nothing allocates after `Core::new`,
//! and nothing reachable from `process` can panic.
//!
//! The core is Driftloom's own engine, not a copy of Chromium. It was
//! ported from the JavaScript synth and proven against it to the sample,
//! which is why many blocks still say what Chromium does; the JavaScript
//! synth is now only the fallback a host without the core plays (until the
//! end of phase 3), not the reference. Changes are judged by the charter's
//! measures (`docs/CORE.md`): the goldens, the baseline, the kit, and ears.

// No standard library (queue item 43): the core needs no allocator, no
// threads and no platform math, so it builds for a console or a
// microcontroller as it does for the browser. Its own unit tests use `std`
// (threads with room for a whole core on their stack, `Vec`); every real
// build, the test host and the integration tests included, links it without.
#![cfg_attr(not(test), no_std)]

pub mod block;
pub mod blocks;
pub mod delay;
pub mod master;
pub mod math;
pub mod mix;
pub mod noise;
pub mod param;
pub mod resample;
pub mod rng;
pub mod timeline;
pub mod wave;

#[cfg(all(target_arch = "wasm32", feature = "web"))]
mod wasm;

use block::{Context, Event, Note, Signal, event};
use blocks::{Held, Own, Pools, own};
use blocks::osc::frame_at;
use master::{Master, OUT};
use mix::Mix;
use noise::Noise;
pub use blocks::voices::*;
pub use rng::Key;
use rng::Draws;
use wave::{Fft, Waves};

/// The core's internal block, Web Audio's render quantum. The core works in
/// blocks of this many frames, on frame numbers that are multiples of it,
/// as an `AudioWorklet` is called. A host with another block size does not
/// notice: `Core::render` takes any number of frames a call and buffers.
pub const QUANTUM: usize = 128;

/// The highest sample rate this build holds its delay lines and its noise
/// for, in Hz: 192,000 with the `wide` feature (the default), 96,000
/// without (the web build). Above it `Core::start` refuses the rate, and a
/// core started anyway has no mix (`Core::mixing`).
pub const MAX_RATE: f64 = if cfg!(feature = "wide") { 192_000.0 } else { 96_000.0 };
const MAX_FRAMES_A_SECOND: usize = if cfg!(feature = "wide") { 192_000 } else { 96_000 };

/// The lowest sample rate `Core::start` accepts, in Hz. The charter's range
/// is 22,050 to 192,000 Hz (`docs/CORE.md`, promise 4).
pub const MIN_RATE: f64 = 22_050.0;

/// The channels the core plays into. Each is one of the synth's per-layer
/// channels: all five of them (queue item 27). Since queue item 29 the core
/// mixes them too (`mix`), with the host's own notes for each (`inputs`),
/// through the channel gains, the sends, the duck, the echo and the reverb;
/// and since items 30 and 31 runs the mix through the whole master chain
/// (`master`): the wobble, the saturator, the tone and the highpass, the bus
/// compressor, the master and kill gains and the ceiling, out to the
/// speakers (`output`).
pub const CHANNELS: usize = 5;
/// The melody's channel.
pub const MELODY: u32 = 0;
/// The chords' channel.
pub const CHORDS: u32 = 1;
/// The bass's channel.
pub const BASS_CHANNEL: u32 = 2;
/// The textures' channel.
pub const TEXTURE_CHANNEL: u32 = 3;
/// The drums' channel.
pub const DRUMS_CHANNEL: u32 = 4;

/// Voices sounding at once. The JavaScript voice budget decides what plays,
/// and it cannot fill this: its whole ceiling is 260 units and the cheapest
/// kalimba part costs 3, so at most 86 notes are ever billed at once,
/// against 256 voices. A note that starts with every voice busy is dropped,
/// and counted (`Core::dropped`).
pub const POOL: usize = 256;

/// Notes waiting for their start time. A note takes a voice only when it
/// starts, so a host may hand notes over well ahead: the app's lookahead is
/// at most three seconds, and the measure harness hands over a whole
/// seventy-second render before it starts -- about a thousand notes on the
/// busiest kalimba loop. A note that arrives with the queue full is
/// dropped, and counted. All zeros until used, like the voices, so neither
/// costs anything in the `.wasm`.
pub const QUEUE: usize = 4096;

/// The master chain's parameters (`Core::param`), after the mix's
/// (`mix::LAST_PARAM`): the tone lowpass's frequency, the wobble's two
/// depths, the saturator's drive and the LFOs' start; the master gain and
/// the kill gain; whether the compressors are routed around; and how they
/// start.
pub const TONE_FREQ: u32 = mix::LAST_PARAM + 1;
/// The wobble's slow depth (wow).
pub const WOW_DEPTH: u32 = TONE_FREQ + 1;
/// The wobble's fast depth (flutter).
pub const FLUTTER_DEPTH: u32 = TONE_FREQ + 2;
/// The saturator's drive.
pub const SAT_DRIVE: u32 = TONE_FREQ + 3;
/// When the wobble's LFOs were built.
pub const LFO_START: u32 = TONE_FREQ + 4;
/// The master gain.
pub const VOLUME: u32 = TONE_FREQ + 5;
/// The kill gain.
pub const KILL: u32 = TONE_FREQ + 6;
/// Whether both compressors are routed around.
pub const BYPASS: u32 = TONE_FREQ + 7;
/// Whether both compressors start cold, as Chromium's do.
pub const COLD: u32 = TONE_FREQ + 8;

/// Why `Core::start` or `Core::render` would not.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
    /// The sample rate is outside `MIN_RATE` to `MAX_RATE`.
    Rate,
    /// The output layout is not one the core renders yet. Mono, one
    /// channel, is the only one; the number is how many channels were asked
    /// for.
    Layout(u32),
}

impl Error {
    /// What went wrong, in plain ASCII: a host shows it to a person.
    #[must_use]
    pub const fn message(self) -> &'static str {
        match self {
            Error::Rate => "sample rate outside what this core runs at (22050 to 192000 Hz, or 96000 in the web build)",
            Error::Layout(_) => "output layout not supported yet: the core renders mono (1 channel)",
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.message())
    }
}

/// What the core says to a note.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Taken {
    /// Queued for its start time.
    OnTime = 0,
    /// Its start time had already been rendered, so it starts now.
    Late = 1,
    /// The queue of waiting notes is full. Dropped.
    Full = 2,
    /// Not a channel or a voice the core has. Dropped.
    Unknown = 3,
    /// Its start time had been rendered more than `MISS` ago: too late to
    /// play without bunching up with the notes on time. Dropped, and
    /// counted apart (`Core::missed`).
    Missed = 4,
}

/// How late a note may arrive and still play, at once (queue item 41). A
/// late note starts at the next block, whole, so the later it is the more
/// it bunches against the notes after it -- a burst of late notes, all
/// starting together, was the Pixel 10a's glitch. A few tens of ms late is
/// not heard as late; past that the note is left out (for Mikey's ears:
/// a gap in a busy loop is quieter than a pile-up).
pub const MISS: f64 = 0.03;

/// Why a note was dropped (`Core::dropped_by`): the queue of waiting notes
/// was full when it arrived; every voice was busy when it started; every
/// sung voice was.
pub const QUEUE_FULL: usize = 0;
/// Every slot of the voice pool was busy when the note started.
pub const VOICES_BUSY: usize = 1;
/// Every voice of a pooled kind (the sung voices) was busy when it started.
pub const SUNG_BUSY: usize = 2;
const CAUSES: usize = 3;

/// How many extra values a note carries. Fixed, so a note is a plain value
/// and nothing allocates: the widest voice, the temple bell, settles ten
/// draws into them. What each voice reads from them is in its own file
/// (`blocks`).
pub const EXTRA: usize = 12;

/// A slot of the voice pool: a sounding note, and what the core needs to
/// know of it without asking its voice.
struct Slot {
    busy: bool,
    channel: usize,
    begin: u64,
    end: u64,
    /// When its note arrived, counted across the core, and when it ends:
    /// which channel events it hears (`Core::event`).
    seq: u64,
    until: f64,
    voice: Held,
}

impl Slot {
    const fn new() -> Self {
        Slot {
            busy: false,
            channel: 0,
            begin: 0,
            end: 0,
            seq: 0,
            until: 0.0,
            voice: Held::FREE,
        }
    }
}

/// A note waiting for its start.
#[derive(Clone, Copy)]
struct Waiting {
    note: Note,
    /// The frame it starts on.
    start: u64,
    /// When the note ends, on the time it was asked for (a late note's
    /// start moves; this does not).
    until: f64,
    /// When it arrived, counted across the core (see `Slot`).
    seq: u64,
    /// The channel events it heard while it waited, one of each kind at
    /// most: which kinds (a bit each), and each one's time and value. It
    /// hears them as it starts. Zeros for none, so the queue stays all zeros
    /// and costs nothing in the `.wasm`.
    heard: u8,
    events: [(f64, f64); event::KINDS],
}

const NOTHING: Waiting = Waiting {
    note: Note {
        voice: 0,
        channel: 0,
        time: 0.0,
        midi: 0.0,
        dur: 0.0,
        vel: 0.0,
        parts: 0,
        extra: [0.0; EXTRA],
        draws: Draws::new(Key { seed: 0, pass: 0, step: 0, ord: 0 }, 0),
    },
    start: 0,
    until: 0.0,
    seq: 0,
    heard: 0,
    events: [(0.0, 0.0); event::KINDS],
};

/// The core: the one thing a host holds. `const` and all
/// zeros when new, so a host can keep one in a static.
pub struct Core {
    rate: f64,
    /// The first frame of the next block to render.
    next: u64,
    slots: [Slot; POOL],
    queue: [Waiting; QUEUE],
    waiting: usize,
    out: [[f32; QUANTUM]; CHANNELS],
    waves: Waves,
    fft: Fft,
    /// The voices that play through a channel's own block (a body) sum
    /// here, per channel and block, then go through it (`blocks::own`).
    buses: [[Signal; own::COUNT]; CHANNELS],
    own: [Own; CHANNELS],
    /// How many notes have arrived, and for each channel and kind of event
    /// how many had when the last such event came (`Core::event`).
    arrived: u64,
    last: [[u64; event::KINDS]; CHANNELS],
    noise: Noise,
    /// The voices too big for a slot (the sung voices), each kind in a
    /// pool of its own (`blocks`).
    pools: Pools,
    /// Where the notes the host sends next sit in their loop (`key`).
    key: Key,
    /// What the host plays into each channel itself, for this block: notes
    /// it could not hand over (`inputs`).
    inputs: [[f32; QUANTUM]; CHANNELS],
    /// How many frames of the last block `render` rendered the host has yet
    /// to be given. Counted down, not up, so a new core is all zeros and the
    /// static one costs nothing in the `.wasm`.
    unserved: usize,
    mix: Mix,
    master: Master,
    /// Whether the master chain fits at this rate.
    master_ready: bool,
    late: u32,
    missed: u32,
    dropped: [u32; CAUSES],
}

impl Core {
    /// An empty core. `const` and all zeros, so a host can keep one in a
    /// static without building it on the stack.
    // Built in a `const` for a host's static core, never on a stack at run
    // time.
    #[allow(clippy::large_stack_arrays)]
    #[must_use]
    pub const fn new() -> Self {
        const FREE: Slot = Slot::new();
        Core {
            rate: 0.0,
            next: 0,
            slots: [FREE; POOL],
            queue: [NOTHING; QUEUE],
            waiting: 0,
            out: [[0.0; QUANTUM]; CHANNELS],
            waves: Waves::new(),
            fft: Fft::new(),
            buses: [[[0.0; QUANTUM]; own::COUNT]; CHANNELS],
            own: [const { Own::new() }; CHANNELS],
            arrived: 0,
            last: [[0; event::KINDS]; CHANNELS],
            noise: Noise::new(),
            pools: Pools::new(),
            key: Key { seed: 0, pass: 0, step: 0, ord: 0 },
            inputs: [[0.0; QUANTUM]; CHANNELS],
            unserved: 0,
            mix: Mix::new(),
            master: Master::new(),
            master_ready: false,
            late: 0,
            missed: 0,
            dropped: [0; CAUSES],
        }
    }

    /// Set the sample rate, build the wavetables and clear everything.
    /// Call before the first note.
    pub fn init(&mut self, rate: f64) {
        self.rate = rate;
        self.next = 0;
        self.late = 0;
        self.missed = 0;
        self.dropped = [0; CAUSES];
        self.clear();
        self.noise.make(rate);
        #[allow(clippy::cast_possible_truncation)] // the tables are 32-bit
        self.waves.build(rate as f32, &mut self.fft);
        let cx = Context { rate, waves: &self.waves, noise: self.noise.samples() };
        for own in &mut self.own {
            own.start(&cx);
        }
        self.arrived = 0;
        self.last = [[0; event::KINDS]; CHANNELS];
        self.inputs = [[0.0; QUANTUM]; CHANNELS];
        self.unserved = 0;
        self.quality(true);
    }

    /// Start the core for a host: at `rate` Hz, rendering `channels`
    /// channels, with the full mix. The first call a native or game host
    /// makes; the web host's worklet calls `init` and `quality` itself.
    ///
    /// Mono (1 channel) is the only layout there is. Asking for another
    /// returns `Error::Layout` and leaves the core as it was, so stereo
    /// can arrive later as a render of its own (`docs/CORE.md`, promise 9)
    /// without anything a host already does changing.
    ///
    /// # Errors
    ///
    /// `Error::Layout` for any layout but mono, `Error::Rate` for a rate
    /// outside `MIN_RATE` to `MAX_RATE`.
    pub fn start(&mut self, rate: f64, channels: u32) -> Result<(), Error> {
        if channels != 1 {
            return Err(Error::Layout(channels));
        }
        if !(MIN_RATE..=MAX_RATE).contains(&rate) {
            return Err(Error::Rate);
        }
        self.init(rate);
        Ok(())
    }

    /// Render the next `out[0].len()` frames, any number, into the host's
    /// buffer: one slice a channel of the layout chosen at `start`, so one,
    /// all the same length. The call after this one carries on where this
    /// one stopped, whatever the block sizes.
    ///
    /// The core renders internally in blocks of `QUANTUM` frames on frame
    /// numbers that are multiples of it, so its samples do not depend on the
    /// host's block size: asking for 1, 441 or 1,024 frames at a time gives
    /// the same bits. What a host needs to know is when a note must be
    /// handed over: before the block holding its start frame is rendered,
    /// which is `next_frame` rounded up to a block. A note handed over after
    /// that is late (`Core::late`) and starts at the next block; one handed
    /// over before it starts on its exact sample.
    ///
    /// Do not mix this with `process`, which a host that owns its own block
    /// clock (the web worklet) calls instead.
    ///
    /// # Errors
    ///
    /// `Error::Layout` if `out` does not hold exactly one channel; nothing
    /// is rendered.
    pub fn render(&mut self, out: &mut [&mut [f32]]) -> Result<(), Error> {
        let [out] = out else {
            return Err(Error::Layout(u32::try_from(out.len()).unwrap_or(u32::MAX)));
        };
        let mut done = 0;
        while done < out.len() {
            if self.unserved == 0 {
                let block = self.next;
                self.process(block);
                self.unserved = QUANTUM;
            }
            let ready = self.master.output()[0].get(QUANTUM - self.unserved..).unwrap_or(&[]);
            let take = ready.len().min(out.len() - done);
            if let (Some(to), Some(from)) = (out.get_mut(done..done + take), ready.get(..take)) {
                to.copy_from_slice(from);
            }
            self.unserved -= take;
            done += take;
        }
        Ok(())
    }

    /// What every block reads besides its own state (`block::Context`):
    /// the rate, the wavetables and the noise, as `init` made them. A host
    /// needs it only to play a block alone, as the test kit and
    /// `dlrender --block` do.
    #[must_use]
    pub fn context(&self) -> Context<'_> {
        Context { rate: self.rate, waves: &self.waves, noise: self.noise.samples() }
    }

    /// The time of the next block to render, in seconds.
    // A frame count as a float: exact below 2^53 frames, some 5,900 years
    // at 48 kHz.
    #[allow(clippy::cast_precision_loss)]
    fn now(&self) -> f64 {
        self.next as f64 / self.rate
    }

    /// The first frame of the next block to render: every frame before it
    /// has been rendered, and a note starting before it is late.
    #[must_use]
    pub fn next_frame(&self) -> u64 {
        self.next
    }

    /// The mix as the synth builds it on lite quality (three combs, not
    /// six) or full. Call after `init`, before the first block: it builds
    /// the mix afresh.
    pub fn quality(&mut self, full: bool) {
        self.mix.init(self.rate, full);
        self.master_ready = self.master.init(self.rate, full);
    }

    /// Whether the core mixes at this sample rate (`mix::MAX_RATE`). A
    /// host whose core does not plays the mix itself.
    #[must_use]
    pub fn mixing(&self) -> bool {
        self.mix.ready() && self.master_ready
    }

    /// Move one of the mix's parameters (`mix::GAINS` and the rest) or the
    /// master chain's (`TONE_FREQ` and the rest, below) the way the
    /// `AudioParam` call `op` would (`mix::SET`, `LINEAR`, `TARGET`,
    /// `CANCEL`): to `value` at `time` seconds, with time constant `tau` for
    /// a target. A time already rendered is the next block's start, as
    /// Chromium has it. False if there is no such parameter or call, or the
    /// parameter holds as many events as it can.
    ///
    /// Four are not `AudioParam`s: `SAT_DRIVE` gives the saturator the curve
    /// `tanhCurve(value)`, from the next block, as setting a `WaveShaper`'s
    /// `curve` does; `LFO_START` says when the synth built the wobble's
    /// LFOs, `time`; `BYPASS` routes around both compressors if `value` is
    /// not 0, at once (the measure harness's `bypass`); `COLD` starts both
    /// compressors as Chromium's start, not warm, if `value` is not 0 (the
    /// harness's, to null them against Chromium's own; before the first
    /// block). All take any `op`.
    pub fn param(&mut self, id: u32, op: u32, value: f64, time: f64, tau: f64) -> bool {
        if !(value.is_finite() && time.is_finite() && tau.is_finite()) || self.rate <= 0.0 {
            return false;
        }
        let now = self.now();
        // An `AudioParam`'s value is a 32-bit float.
        #[allow(clippy::cast_possible_truncation)]
        let v = value as f32;
        match id {
            TONE_FREQ => mix::apply(self.master.tone_freq(), op, v, time, tau, now),
            WOW_DEPTH => mix::apply(self.master.wow_depth(), op, v, time, tau, now),
            FLUTTER_DEPTH => mix::apply(self.master.flutter_depth(), op, v, time, tau, now),
            SAT_DRIVE => {
                self.master.set_drive(value);
                true
            }
            LFO_START => {
                self.master.start(time);
                true
            }
            VOLUME => mix::apply(self.master.volume(), op, v, time, tau, now),
            KILL => mix::apply(self.master.kill(), op, v, time, tau, now),
            BYPASS => {
                self.master.set_bypass(value != 0.0);
                true
            }
            COLD => {
                self.master.set_cold(value != 0.0);
                true
            }
            _ => self.mix.param(id, op, v, time, tau, now),
        }
    }

    /// Where the host writes what it plays into each channel for the next
    /// block, `QUANTUM` samples a channel; `process` adds it in and clears
    /// it.
    pub fn inputs(&mut self) -> &mut [[f32; QUANTUM]; CHANNELS] {
        &mut self.inputs
    }

    /// The last block rendered, through the mix and the master chain: what
    /// the speakers get, a block an output channel (`master::OUT`).
    #[must_use]
    pub fn output(&self) -> &[[f32; QUANTUM]; OUT] {
        self.master.output()
    }

    /// The last block as it reached the bus compressor, a block an output
    /// channel: the measure harness reads it.
    #[must_use]
    pub fn bus(&self) -> &[[f32; QUANTUM]; OUT] {
        self.master.before_comp()
    }

    /// The bus compressor's `reduction` (0) or the ceiling's (1), in dB,
    /// as of the last block.
    #[must_use]
    pub fn reduction(&self, which: usize) -> f32 {
        self.master.reduction(which)
    }

    /// How hard the signal has pressed the bus compressor (0) or the
    /// ceiling (1) since this was last asked, in dB (`Compressor::deepest`).
    pub fn deepest(&mut self, which: usize) -> f32 {
        self.master.deepest(which)
    }

    /// Let every voice go at once, sounding or waiting: the host is going
    /// away.
    pub fn clear(&mut self) {
        self.waiting = 0;
        for slot in &mut self.slots {
            slot.busy = false;
        }
        self.pools.clear();
    }

    /// Drop every note still waiting for its start time, and leave the ones
    /// sounding as they are: the host handed notes over ahead of their time
    /// and the music has jumped since (Pause, Next or Previous, queue item
    /// 39).
    pub fn cancel(&mut self) {
        self.waiting = 0;
    }

    /// Drop the notes still waiting whose time is `time` (seconds) or later,
    /// and keep the rest, waiting or sounding: a change that must be heard
    /// at once while notes are handed over seconds ahead (mute, re-roll,
    /// tempo; queue item 40). The host decides again from `time` on, so the
    /// two halves meet exactly, by the time each note was asked for.
    pub fn cancel_from(&mut self, time: f64) {
        let mut i = 0;
        while i < self.waiting {
            let Some(&w) = self.queue.get(i) else { break };
            if w.note.time < time {
                i += 1;
                continue;
            }
            self.waiting -= 1;
            if let Some(&last) = self.queue.get(self.waiting)
                && let Some(gap) = self.queue.get_mut(i)
            {
                *gap = last;
            }
        }
    }

    /// Where the notes the host sends next sit in their loop: the loop's
    /// seed, the pass, the step and the note's place among its layer's
    /// notes at that step (`rng::Key`). Every draw a note makes -- how
    /// long its second reed waits, where in the noise a strike starts, a
    /// sung note's jitter -- comes from this and nothing else, so the same
    /// loop makes the same sound everywhere. It holds until the host sets
    /// another.
    pub fn key(&mut self, key: Key) {
        self.key = key;
    }

    /// Notes that arrived after their start time had been rendered, the
    /// missed ones included.
    #[must_use]
    pub fn late(&self) -> u32 {
        self.late
    }

    /// Notes that arrived more than `MISS` late, and were left out.
    #[must_use]
    pub fn missed(&self) -> u32 {
        self.missed
    }

    /// Notes dropped: the queue was full when they arrived, or every voice
    /// was busy when they started. Missed notes are not among them.
    #[must_use]
    pub fn dropped(&self) -> u32 {
        self.dropped
            .iter()
            .fold(0u32, |sum, n| sum.saturating_add(*n))
    }

    /// Notes dropped for one cause (`QUEUE_FULL`, `VOICES_BUSY`,
    /// `SUNG_BUSY`).
    #[must_use]
    pub fn dropped_by(&self, cause: usize) -> u32 {
        self.dropped.get(cause).copied().unwrap_or(0)
    }

    fn drop_for(&mut self, cause: usize) {
        if let Some(n) = self.dropped.get_mut(cause) {
            *n = n.saturating_add(1);
        }
    }

    /// Notes sounding or waiting to.
    #[must_use]
    pub fn busy(&self) -> usize {
        self.waiting + self.slots.iter().filter(|s| s.busy).count()
    }

    /// A note: `voice` (a number from the registry, `blocks::voices`), into
    /// `channel` (`MELODY` and the rest), at `time` seconds on the host's
    /// clock, with the voice's own `parts` and `extra` values (each voice's
    /// file in `blocks` says what it reads). What the note leaves to chance
    /// is settled here, from the last `key` (`Voice::settle`). A note whose start has
    /// already been rendered starts at once: the whole of it, from its
    /// first sample, at the start of the next block -- unless it is more
    /// than `MISS` late, when it is left out.
    #[allow(clippy::too_many_arguments)]
    pub fn note(
        &mut self,
        voice: u32,
        channel: u32,
        time: f64,
        midi: f64,
        dur: f64,
        vel: f64,
        parts: u32,
        extra: [f64; EXTRA],
    ) -> Taken {
        let mut note = Note {
            voice,
            channel,
            time,
            midi,
            dur,
            vel,
            parts,
            extra,
            draws: Draws::new(self.key, channel),
        };
        if !blocks::known(voice)
            || (blocks::uses_noise(&note) && !self.noise.ready())
            || channel as usize >= CHANNELS
            || !(time.is_finite() && dur.is_finite() && midi.is_finite() && vel.is_finite())
        {
            return Taken::Unknown;
        }
        let rate = self.rate;
        let until = time + dur;
        let mut taken = Taken::OnTime;
        if frame_at(time, rate) < self.next {
            let now = self.now();
            self.late = self.late.saturating_add(1);
            if now - time > MISS {
                self.missed = self.missed.saturating_add(1);
                return Taken::Missed;
            }
            note.time = now;
            taken = Taken::Late;
        }
        if self.waiting >= QUEUE {
            self.drop_for(QUEUE_FULL);
            return Taken::Full;
        }
        let Some(gap) = self.queue.get_mut(self.waiting) else {
            return Taken::Full;
        };
        // What the note leaves to chance is settled now, from where it sits
        // in its loop, and not by the host.
        blocks::settle(&mut note);
        *gap = Waiting {
            note,
            start: frame_at(note.time, rate),
            until,
            seq: self.arrived,
            heard: 0,
            events: [(0.0, 0.0); event::KINDS],
        };
        self.waiting += 1;
        self.arrived = self.arrived.wrapping_add(1);
        taken
    }

    /// A new strum into `channel` at `time` (`damp()` in synth.js): a
    /// `DAMP` event (`event`), which the strummed nylon strings of the strum
    /// before it hear. The web host's call, kept as it was.
    pub fn damp(&mut self, channel: u32, time: f64) {
        self.event(channel, Event { kind: event::DAMP, time, value: 0.0 });
    }

    /// Send `event` to the notes of `channel` (queue item 46b): every note
    /// in it still held at the event's time (its note ends after it) that
    /// arrived since the channel's last event of the same kind -- so each
    /// note hears each kind at most once, which is how a damp reaches the
    /// strum before it and no further. What a note does with it is its
    /// voice's (`Voice::event`): today only a strummed nylon string listens,
    /// to a damp.
    ///
    /// Web Audio adds such an event to a timeline that may already be
    /// rendering. Chromium renders an event added ahead of the clock exactly
    /// as one scheduled with the note, so a note already sounding takes it
    /// as it is, and a note still waiting takes it when it starts. One
    /// whose time has already been rendered is not caught up: Chromium
    /// moves an event added in the past to the start of the next block it
    /// renders (`ClampNewEventsToCurrentTime`), and a sounding note hears
    /// it there. A kind the core does not have is ignored.
    pub fn event(&mut self, channel: u32, event: Event) {
        let Some(last) = self
            .last
            .get_mut(channel as usize)
            .and_then(|kinds| kinds.get_mut(event.kind as usize))
        else {
            return;
        };
        let since = *last;
        *last = self.arrived;
        let time = event.time;
        if !time.is_finite() {
            return;
        }
        let kind = event.kind as usize;
        for w in self.queue.iter_mut().take(self.waiting) {
            if w.note.channel == channel
                && w.seq >= since
                && w.until > time
                && let Some(heard) = w.events.get_mut(kind)
            {
                *heard = (time, event.value);
                w.heard |= 1 << kind;
            }
        }
        let now = self.now();
        let at = Event { time: if time < now { now } else { time }, ..event };
        for slot in &mut self.slots {
            if slot.busy && slot.channel == channel as usize && slot.seq >= since && slot.until > time {
                blocks::event(&mut slot.voice, &mut self.pools, &at);
            }
        }
    }

    // A waiting note's start has come: give it a voice.
    fn begin_note(&mut self, w: Waiting) {
        if self.slots.iter().all(|s| s.busy) {
            self.drop_for(VOICES_BUSY);
            return;
        }
        let Some(slot) = self.slots.iter_mut().find(|s| !s.busy) else {
            return;
        };
        let cx = Context { rate: self.rate, waves: &self.waves, noise: self.noise.samples() };
        let Some((begin, end)) = blocks::begin(&mut slot.voice, &mut self.pools, &w.note, &cx) else {
            if let Some(n) = self.dropped.get_mut(SUNG_BUSY) {
                *n = n.saturating_add(1);
            }
            return;
        };
        // The channel events it heard while it waited, as they came.
        for (kind, &(time, value)) in (0u32..).zip(w.events.iter()) {
            if w.heard & (1 << kind) != 0 {
                blocks::event(&mut slot.voice, &mut self.pools, &Event { kind, time, value });
            }
        }
        slot.channel = w.note.channel as usize;
        slot.seq = w.seq;
        slot.until = w.until;
        slot.begin = begin;
        slot.end = end;
        slot.busy = end > begin;
        if !slot.busy {
            blocks::free(&slot.voice, &mut self.pools);
        }
    }

    /// Render the block of `QUANTUM` frames that starts at frame `block`
    /// (a multiple of `QUANTUM`; blocks come in order, and one that goes
    /// backward is rendered where the core is due instead): every voice into its
    /// channel, with what the host wrote into `inputs`, and the channels
    /// through the mix into the bus (`bus`). Returns the channels, one
    /// buffer each, as they went into the mix.
    pub fn process(&mut self, block: u64) -> &[[f32; QUANTUM]; CHANNELS] {
        // Time only goes forward. A block before the one due is rendered as
        // the one due: rendering a block again would start the voices that
        // began in it a second time.
        let block = block.max(self.next);
        let end = block + QUANTUM as u64;
        self.next = end;
        for ch in &mut self.out {
            ch.fill(0.0);
        }
        // Notes starting in this block take their voices. The queue is not
        // kept in order: the last waiting note fills the gap each leaves.
        let mut i = 0;
        while i < self.waiting {
            let Some(&w) = self.queue.get(i) else { break };
            if w.start >= end {
                i += 1;
                continue;
            }
            self.waiting -= 1;
            if let Some(&last) = self.queue.get(self.waiting)
                && let Some(gap) = self.queue.get_mut(i)
            {
                *gap = last;
            }
            self.begin_note(w);
        }
        for bus in self.buses.iter_mut().flatten() {
            bus.fill(0.0);
        }
        let cx = Context { rate: self.rate, waves: &self.waves, noise: self.noise.samples() };
        for slot in self.slots.iter_mut().filter(|s| s.busy) {
            if slot.begin >= end {
                continue;
            }
            let (Some(out), Some(buses)) = (
                self.out.get_mut(slot.channel),
                self.buses.get_mut(slot.channel),
            ) else {
                continue;
            };
            blocks::render(&mut slot.voice, &mut self.pools, block, &cx, out, buses);
            if slot.end <= end {
                slot.busy = false;
                blocks::free(&slot.voice, &mut self.pools);
            }
        }
        // Every note that plays through one of its channel's own blocks (a
        // body), through it.
        for ((out, buses), own) in self.out.iter_mut().zip(self.buses.iter_mut()).zip(self.own.iter_mut()) {
            own.render(block, &cx, buses, out);
        }
        // The host's own notes, then the mix. A channel's tail stops at the
        // silence floor on its way in (`math`, "Subnormal numbers").
        for (out, input) in self.out.iter_mut().zip(self.inputs.iter_mut()) {
            for (y, x) in out.iter_mut().zip(input.iter()) {
                *y = math::silence(*y + *x);
            }
            input.fill(0.0);
        }
        // The graph is mono: the mix is every lane's.
        let mixed = [*self.mix.process(block, &self.out); OUT];
        self.master.process(block, &mixed, &self.waves);
        &self.out
    }
}

impl Default for Core {
    fn default() -> Self {
        Core::new()
    }
}

// Tests count with casts of values they have just bounded.
#[cfg(test)]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
mod tests {
    use super::*;
    use blocks::kalimba::{BODY, STRIKE};
    use blocks::{bass, drum, midi_to_freq, texture};

    const RATE: f64 = 44100.0;

    fn render(core: &mut Core, blocks: usize) -> Vec<[f32; QUANTUM]> {
        let mut melody = Vec::new();
        for b in 0..blocks {
            let out = core.process((b * QUANTUM) as u64);
            melody.push(out[0]);
        }
        melody
    }

    const NO: [f64; EXTRA] = [f64::NAN; EXTRA];

    // The core holds its wavetables, 1.2 MB, so it is built on a thread
    // with room for it on the stack.
    fn boxed() -> Box<Core> {
        std::thread::Builder::new()
            .stack_size(1 << 26)
            .spawn(|| {
                let mut core = Box::new(Core::new());
                core.init(RATE);
                core
            })
            .and_then(|t| t.join().map_err(|_| std::io::Error::other("panicked")))
            .expect("core")
    }

    #[test]
    fn a_kalimba_note_sounds_then_lets_its_voice_go() {
        let mut core = boxed();
        assert_eq!(
            core.note(KALIMBA, MELODY, 0.1, 69.0, 0.4, 0.8, STRIKE | BODY, NO),
            Taken::OnTime
        );
        assert_eq!(core.busy(), 1);
        assert!(
            core.slots.iter().all(|s| !s.busy),
            "a voice before its start"
        );
        let blocks = render(&mut core, (2.0 * RATE) as usize / QUANTUM);
        let peak = blocks.iter().flatten().fold(0.0f32, |a, &b| a.max(b.abs()));
        assert!(peak > 0.1 && peak < 0.3, "peak {peak}");
        // 0.4 s note, released 1.2 s after: gone by 1.71 s.
        assert_eq!(core.busy(), 0);
        let first = (0.1 * RATE).ceil() as usize;
        assert!(blocks.iter().flatten().take(first).all(|&x| x == 0.0));
    }

    // A block before the one the core is due to render (the host's frame
    // count went stale for good, say) is not rendered again: a voice that
    // started in it would start a second time, and the pads' and leads'
    // filters, set going again, spiked to thirty times their level. The
    // core goes on from its own count.
    #[test]
    fn a_block_that_goes_backward_is_ridden_out() {
        let mut a = boxed();
        let mut b = boxed();
        // The voices that play from a bare note (the fiddle, the pads and
        // leads, the stab, a bass and a texture) all start in the blocks
        // that would repeat.
        for core in [&mut a, &mut b] {
            for voice in [1, 2, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 22, 23] {
                core.note(voice, MELODY, 0.2905, 69.0, 1.0, 0.7, 0, NO);
            }
        }
        let steady = render(&mut a, 160);
        let mut rewound = Vec::new();
        let mut asked = (0..103usize).collect::<Vec<_>>();
        asked.extend(99..156usize);
        for blk in asked {
            rewound.push(b.process((blk * QUANTUM) as u64)[0]);
        }
        assert_eq!(rewound.len(), steady.len());
        // Bit for bit (some of these voices are given nothing to play, and
        // say NaN, which is not equal to itself).
        let bits = |blocks: &[[f32; QUANTUM]]| blocks.iter().flatten().map(|v| v.to_bits()).collect::<Vec<_>>();
        let (x, y) = (bits(&rewound), bits(&steady));
        let first = x.iter().zip(&y).position(|(p, q)| p != q);
        assert_eq!(first.map(|i| i / QUANTUM), None, "the first block that differs");
        let peak = steady.iter().flatten().fold(0.0f32, |m, &v| m.max(v.abs()));
        assert!(peak > 0.05, "the voices play: {peak}");
    }

    #[test]
    fn the_same_notes_render_the_same_samples() {
        let mut a = boxed();
        let mut b = boxed();
        for core in [&mut a, &mut b] {
            core.note(KALIMBA, MELODY, 0.0123, 72.0, 0.3, 0.6, STRIKE | BODY, NO);
            core.note(KALIMBA, MELODY, 0.2001, 64.0, 1.5, 0.5, STRIKE, NO);
        }
        assert_eq!(render(&mut a, 400), render(&mut b, 400));
    }

    #[test]
    fn parts_the_budget_refused_stay_silent() {
        let mut both = boxed();
        let mut body = boxed();
        both.note(KALIMBA, CHORDS, 0.05, 60.0, 0.5, 0.7, STRIKE | BODY, NO);
        body.note(KALIMBA, CHORDS, 0.05, 60.0, 0.5, 0.7, BODY, NO);
        let chords = |core: &mut Core| -> f32 {
            let mut e = 0.0;
            for b in 0..200 {
                e += core.process((b * QUANTUM) as u64)[1]
                    .iter()
                    .map(|x| x * x)
                    .sum::<f32>();
            }
            e
        };
        assert!(chords(&mut both) > chords(&mut body) * 2.0);
        assert_eq!(body.busy(), 0);
    }

    #[test]
    fn a_late_note_starts_at_once_and_is_counted() {
        let mut core = boxed();
        render(&mut core, 10);
        assert_eq!(
            core.note(KALIMBA, MELODY, 0.001, 69.0, 0.2, 0.8, STRIKE, NO),
            Taken::Late
        );
        assert_eq!(core.late(), 1);
        let out = core.process(10 * QUANTUM as u64);
        // sin(0) at the first frame, then sounding.
        assert_eq!(out[0][0], 0.0);
        assert!(out[0][1] != 0.0);
    }

    #[test]
    fn a_note_more_than_a_miss_late_is_left_out_and_counted_apart() {
        let mut core = boxed();
        render(&mut core, 20);
        // 20 blocks are 58 ms: a note due at 1 ms is 57 ms late.
        assert_eq!(
            core.note(KALIMBA, MELODY, 0.001, 69.0, 0.2, 0.8, STRIKE, NO),
            Taken::Missed
        );
        // One a block late still plays.
        let just = (20 * QUANTUM - QUANTUM) as f64 / RATE;
        assert_eq!(
            core.note(KALIMBA, MELODY, just, 69.0, 0.2, 0.8, STRIKE, NO),
            Taken::Late
        );
        assert_eq!(core.late(), 2);
        assert_eq!(core.missed(), 1);
        assert_eq!(core.dropped(), 0);
        assert_eq!(core.busy(), 1);
    }

    #[test]
    fn notes_wait_without_taking_voices() {
        let mut core = boxed();
        // Far more than there are voices, a second apart: each gets one.
        for i in 0..POOL * 2 {
            assert_eq!(
                core.note(
                    KALIMBA,
                    MELODY,
                    0.01 + i as f64 * 0.01,
                    60.0,
                    0.1,
                    0.5,
                    BODY,
                    NO
                ),
                Taken::OnTime
            );
        }
        render(&mut core, (6.0 * RATE) as usize / QUANTUM);
        assert_eq!(core.dropped(), 0);
        assert_eq!(core.busy(), 0);
    }

    #[test]
    fn a_full_queue_drops_the_note_and_counts_it() {
        let mut core = boxed();
        for i in 0..QUEUE {
            assert_eq!(
                core.note(
                    KALIMBA,
                    MELODY,
                    1.0 + i as f64 * 0.001,
                    60.0,
                    0.5,
                    0.5,
                    STRIKE,
                    NO
                ),
                Taken::OnTime
            );
        }
        assert_eq!(
            core.note(KALIMBA, MELODY, 9.0, 60.0, 0.5, 0.5, STRIKE, NO),
            Taken::Full
        );
        assert_eq!(core.dropped(), 1);
        assert_eq!(core.dropped_by(QUEUE_FULL), 1);
    }

    #[test]
    fn a_note_starting_with_every_voice_busy_is_dropped_and_counted() {
        let mut core = boxed();
        for _ in 0..=POOL {
            core.note(KALIMBA, MELODY, 0.0, 60.0, 0.5, 0.5, STRIKE, NO);
        }
        core.process(0);
        assert_eq!(core.dropped(), 1);
        assert_eq!(core.dropped_by(VOICES_BUSY), 1);
        assert_eq!(core.busy(), POOL);
    }

    #[test]
    fn unknown_voices_and_channels_are_refused() {
        let mut core = boxed();
        assert_eq!(
            core.note(99, MELODY, 0.1, 60.0, 0.5, 0.5, STRIKE, NO),
            Taken::Unknown
        );
        assert_eq!(
            core.note(KALIMBA, 5, 0.1, 60.0, 0.5, 0.5, STRIKE, NO),
            Taken::Unknown
        );
        assert_eq!(
            core.note(KALIMBA, MELODY, f64::NAN, 60.0, 0.5, 0.5, STRIKE, NO),
            Taken::Unknown
        );
        assert_eq!(core.busy(), 0);
    }

    // The options keys passes to `fm()`: ratio, index, attack, decay, detune.
    const KEYS: [f64; EXTRA] = [
        2.0,
        260.0,
        0.006,
        0.4,
        0.0,
        f64::NAN,
        f64::NAN,
        f64::NAN,
        f64::NAN,
        f64::NAN,
        f64::NAN,
        f64::NAN,
    ];

    fn energy(core: &mut Core, channel: usize, seconds: f64) -> (f32, f32) {
        let (mut peak, mut sum) = (0.0f32, 0.0f32);
        for b in 0..(seconds * RATE) as usize / QUANTUM {
            for &x in &core.process((b * QUANTUM) as u64)[channel] {
                peak = peak.max(x.abs());
                sum += x * x;
            }
        }
        (peak, sum)
    }

    #[test]
    fn an_fm_note_sounds_then_lets_its_voice_go() {
        let mut core = boxed();
        assert_eq!(
            core.note(FM, CHORDS, 0.1, 60.0, 0.5, 0.8, 0, KEYS),
            Taken::OnTime
        );
        // In the chords channel, and nothing in the melody one.
        let mut melody = 0.0f32;
        let mut chords = 0.0f32;
        for b in 0..(2.5 * RATE) as usize / QUANTUM {
            let out = core.process((b * QUANTUM) as u64);
            melody = out[0].iter().fold(melody, |a, &x| a.max(x.abs()));
            chords = out[1].iter().fold(chords, |a, &x| a.max(x.abs()));
        }
        assert_eq!(melody, 0.0);
        assert!(chords > 0.05 && chords < 0.3, "peak {chords}");
        // 0.5 s note, released 1.2 s after: gone by 1.81 s.
        assert_eq!(core.busy(), 0);
    }

    #[test]
    fn fm_options_change_the_note() {
        let mut keys = boxed();
        let mut bell = boxed();
        keys.note(FM, MELODY, 0.05, 72.0, 0.4, 0.7, 0, KEYS);
        bell.note(
            FM,
            MELODY,
            0.05,
            72.0,
            0.4,
            0.7,
            0,
            [
                3.51,
                420.0,
                0.006,
                0.5,
                7.0,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
            ],
        );
        let a = render(&mut keys, 300);
        let b = render(&mut bell, 300);
        assert_ne!(a, b);
    }

    #[test]
    fn a_sine_note_sounds_then_lets_its_voice_go() {
        let mut core = boxed();
        core.note(SINE, MELODY, 0.1, 69.0, 0.6, 0.8, 0, NO);
        let (peak, _) = energy(&mut core, 0, 3.0);
        // vel * 0.24 at the top of the swell.
        assert!(peak > 0.15 && peak < 0.2, "peak {peak}");
        assert_eq!(core.busy(), 0);
    }

    #[test]
    fn a_tubular_note_rings_for_at_least_five_seconds() {
        let mut core = boxed();
        core.note(TUBULAR, CHORDS, 0.1, 60.0, 0.4, 0.8, 0, NO);
        let (peak, _) = energy(&mut core, 1, 4.0);
        assert!(peak > 0.05 && peak < 0.4, "peak {peak}");
        assert_eq!(core.busy(), 1, "gone before five seconds");
        let mut b = (4.0 * RATE) as usize / QUANTUM;
        while core.busy() > 0 && b < 20 * RATE as usize / QUANTUM {
            core.process((b * QUANTUM) as u64);
            b += 1;
        }
        assert_eq!(core.busy(), 0);
    }

    #[test]
    fn each_wave_table_voice_sounds_then_lets_its_voice_go() {
        // (voice, level range its peak should fall in), at a note of 0.5 s.
        let voices = [
            (SOFTPAD, 0.02, 0.5),
            (ANALOGPAD, 0.02, 0.5),
            (ANALOGLEAD, 0.01, 0.5),
            (SAWPLUCK, 0.01, 0.5),
            (BEEP, 0.01, 0.5),
            (MOOG, 0.01, 0.5),
            (WHISTLE, 0.01, 0.5),
        ];
        for (voice, low, high) in voices {
            let mut core = boxed();
            assert_eq!(
                core.note(voice, MELODY, 0.05, 64.0, 0.5, 0.8, 0, NO),
                Taken::OnTime
            );
            let (peak, _) = energy(&mut core, 0, 4.0);
            assert!(peak > low && peak < high, "voice {voice}: peak {peak}");
            assert_eq!(core.busy(), 0, "voice {voice} still sounding");
        }
    }

    // The first note, from the key's ord up, that the chance sends sliding
    // (a draw under 0.22) and the first it does not.
    fn ords_that_slide_and_do_not() -> (u32, u32) {
        let slides = |ord: u32| rng::unit(key(3, 0, 0, ord), MELODY, rng::salt::SLIDE, 0) < 0.22;
        let slid = (0..1000).find(|&o| slides(o)).expect("some ord slides");
        let plain = (0..1000).find(|&o| !slides(o)).expect("some ord does not");
        (slid, plain)
    }

    #[test]
    fn a_whistle_slides_when_its_draw_says_so() {
        let (slid, plain) = ords_that_slide_and_do_not();
        let (mut a, mut b, mut c) = (boxed(), boxed(), boxed());
        for (core, ord) in [(&mut a, plain), (&mut b, slid), (&mut c, slid)] {
            core.key(key(3, 0, 0, ord));
            core.note(WHISTLE, MELODY, 0.05, 72.0, 0.4, 0.7, 0, NO);
        }
        let (a, b, c) = (render(&mut a, 100), render(&mut b, 100), render(&mut c, 100));
        assert_ne!(a, b);
        assert_eq!(b, c);
    }

    #[test]
    fn the_built_in_waves_have_chromiums_partials() {
        use wave::{sawtooth, square, triangle};
        let pi = core::f32::consts::PI;
        assert!((sawtooth(1) - 2.0 / pi).abs() < 1e-6);
        assert!((sawtooth(2) + 1.0 / pi).abs() < 1e-6);
        assert_eq!(square(2), 0.0);
        assert!((square(3) - 4.0 / (3.0 * pi)).abs() < 1e-6);
        assert!((triangle(1) - 8.0 / (pi * pi)).abs() < 1e-6);
    }

    #[test]
    fn clear_lets_every_voice_go() {
        let mut core = boxed();
        core.note(KALIMBA, MELODY, 0.0, 60.0, 0.5, 0.5, STRIKE | BODY, NO);
        core.note(KALIMBA, CHORDS, 5.0, 60.0, 0.5, 0.5, STRIKE | BODY, NO);
        core.clear();
        assert_eq!(core.busy(), 0);
        let out = core.process(0);
        assert!(out.iter().flatten().all(|&x| x == 0.0));
    }

    #[test]
    fn cancel_from_drops_only_the_waiting_notes_from_its_time() {
        let mut core = boxed();
        core.note(KALIMBA, MELODY, 0.0, 60.0, 0.5, 0.5, STRIKE | BODY, NO);
        core.note(KALIMBA, CHORDS, 1.0, 60.0, 0.5, 0.5, STRIKE | BODY, NO);
        core.note(KALIMBA, CHORDS, 2.0, 64.0, 0.5, 0.5, STRIKE | BODY, NO);
        core.note(KALIMBA, MELODY, 3.0, 67.0, 0.5, 0.5, STRIKE | BODY, NO);
        render(&mut core, 2);
        assert_eq!(core.busy(), 4);
        // From 2 s: the notes at 2 and 3 go, the one at 1 waits on, and the
        // one sounding sounds on.
        core.cancel_from(2.0);
        assert_eq!(core.busy(), 2);
        render(&mut core, (5.0 * RATE) as usize / QUANTUM);
        assert_eq!(core.busy(), 0);
        assert_eq!(core.dropped(), 0);
    }

    #[test]
    fn cancel_drops_the_waiting_notes_and_keeps_the_sounding_ones() {
        let mut core = boxed();
        core.note(KALIMBA, MELODY, 0.0, 60.0, 0.5, 0.5, STRIKE | BODY, NO);
        core.note(KALIMBA, CHORDS, 1.0, 60.0, 0.5, 0.5, STRIKE | BODY, NO);
        render(&mut core, 2);
        assert_eq!(core.busy(), 2);
        core.cancel();
        assert_eq!(core.busy(), 1);
        assert!(core.process(256).iter().flatten().any(|&x| x != 0.0));
        // Past the second note's start: it never sounds, and nothing is
        // counted dropped.
        render(&mut core, (2.0 * RATE) as usize / QUANTUM);
        assert_eq!(core.busy(), 0);
        assert_eq!(core.dropped(), 0);
        // The queue takes notes again.
        assert_eq!(
            core.note(KALIMBA, MELODY, 3.0, 60.0, 0.5, 0.5, STRIKE, NO),
            Taken::OnTime
        );
    }

    #[test]
    fn a_sung_note_sings_then_lets_its_voice_go() {
        let mut core = boxed();
        let mut extra = NO;
        extra[..4].copy_from_slice(&[0.0, f64::NAN, 1.0, 0.0]);
        assert_eq!(core.note(CHOIR, MELODY, 0.1, 60.0, 0.4, 0.8, 0, extra), Taken::OnTime);
        let blocks = render(&mut core, (2.0 * RATE) as usize / QUANTUM);
        let peak = blocks.iter().flatten().fold(0.0f32, |a, &b| a.max(b.abs()));
        assert!(peak > 0.01 && peak < 0.5, "peak {peak}");
        assert_eq!(core.busy(), 0);
        assert!(core.pools.idle());
    }

    #[test]
    fn a_host_starts_the_core_at_a_rate_and_a_layout_or_is_told_plainly_why_not() {
        let mut core = boxed();
        assert_eq!(core.start(48_000.0, 1), Ok(()));
        for (rate, channels, want) in [
            (48_000.0, 2, Error::Layout(2)),
            (48_000.0, 0, Error::Layout(0)),
            (48_000.0, 6, Error::Layout(6)),
            (8_000.0, 1, Error::Rate),
            (22_049.0, 1, Error::Rate),
            (MAX_RATE + 1.0, 1, Error::Rate),
            (f64::NAN, 1, Error::Rate),
        ] {
            assert_eq!(core.start(rate, channels), Err(want), "{rate} Hz, {channels} channels");
            assert!(want.message().is_ascii());
        }
        // The edges of the range are in it.
        assert_eq!(core.start(22_050.0, 1), Ok(()));
        assert_eq!(core.start(MAX_RATE, 1), Ok(()));
        // A refused start leaves the core as it was.
        assert_eq!(core.start(48_000.0, 2), Err(Error::Layout(2)));
        assert!(core.mixing());
    }

    #[test]
    fn render_refuses_a_layout_it_does_not_have_and_renders_nothing() {
        let mut core = boxed();
        let (mut a, mut b) = ([1.0f32; 64], [1.0f32; 64]);
        assert_eq!(core.render(&mut [&mut a, &mut b]), Err(Error::Layout(2)));
        assert_eq!(core.render(&mut []), Err(Error::Layout(0)));
        assert!(a.iter().all(|x| x.to_bits() == 1.0f32.to_bits()));
        assert_eq!(core.next_frame(), 0);
    }

    /// The same notes, taken a call at a time of any size, are the same
    /// samples bit for bit: the core renders on its own blocks and the
    /// host's size is no business of the sound's.
    #[test]
    fn the_host_may_ask_for_any_number_of_frames_a_call() {
        let render_in = |sizes: &[usize]| -> Vec<u32> {
            let mut core = boxed();
            core.key(key(3, 0, 0, 0));
            core.note(KALIMBA, MELODY, 0.0123, 72.0, 0.3, 0.6, STRIKE | BODY, NO);
            core.note(KALIMBA, MELODY, 0.2001, 64.0, 1.5, 0.5, STRIKE, NO);
            core.note(HAT, DRUMS_CHANNEL, 0.3, 0.0, 0.1, 0.7, 0, NO);
            let mut all = Vec::new();
            let mut at = 0;
            let total = 40_000;
            while all.len() < total {
                let want = sizes[at % sizes.len()].min(total - all.len());
                at += 1;
                let mut buffer = vec![0.0f32; want];
                core.render(&mut [&mut buffer]).expect("mono");
                all.extend(buffer.iter().map(|x| x.to_bits()));
            }
            assert_eq!((core.late(), core.missed(), core.dropped()), (0, 0, 0));
            all
        };
        let whole = render_in(&[40_000]);
        assert!(whole.iter().any(|&b| f32::from_bits(b) != 0.0), "the notes sound");
        for sizes in [&[1][..], &[64], &[127], &[128], &[129], &[441], &[1024], &[1, 2, 3, 500, 1], &[3000]] {
            assert!(render_in(sizes) == whole, "calls of {sizes:?} frames");
        }
    }

    #[test]
    fn a_note_handed_over_ahead_starts_on_its_exact_sample_whatever_the_calls() {
        // First sounding frame of a hat at 0.0123 s, calls of 1 and 777.
        let first = |size: usize| -> usize {
            let mut core = boxed();
            core.note(KALIMBA, MELODY, 0.0123, 72.0, 0.3, 0.6, STRIKE | BODY, NO);
            let mut seen = 0;
            loop {
                let mut buffer = vec![0.0f32; size];
                core.render(&mut [&mut buffer]).expect("mono");
                if let Some(i) = buffer.iter().position(|&x| x != 0.0) {
                    return seen + i;
                }
                seen += size;
            }
        };
        assert_eq!(first(1), first(777));
        // And the sample is the one the time falls on, or just after (the
        // first sample of a sine is zero): the melody before the master
        // chain, which delays what it carries.
        let mut core = boxed();
        core.note(KALIMBA, MELODY, 0.0123, 72.0, 0.3, 0.6, STRIKE | BODY, NO);
        let at = usize::try_from(frame_at(0.0123, RATE)).expect("a small frame");
        let mut start = None;
        for b in 0..8 {
            let melody = core.process((b * QUANTUM) as u64)[0];
            start = start.or(melody.iter().position(|&x| x != 0.0).map(|i| b * QUANTUM + i));
        }
        assert!((at..=at + 2).contains(&start.expect("it sounds")), "{start:?} against {at}");
    }

    fn key(seed: u32, pass: u32, step: u32, ord: u32) -> Key {
        Key { seed, pass, step, ord }
    }

    /// Every voice that draws, rendered for a key: the notes it makes are a
    /// function of that key and nothing else.
    fn drawing_voices(core: &mut Core) {
        let t = 0.05;
        let mut x = NO;
        for (voice, channel, parts, dur) in [
            (FIDDLE, MELODY, 0, 1.0),
            (OCARINA, MELODY, 0, 1.0),
            (FLUTE, MELODY, 0, 1.0),
            (WHISTLE, MELODY, 0, 1.0),
            (PANFLUTE, MELODY, 0, 1.0),
            (ACCORDION, MELODY, 0, 1.0),
            (TUBULAR, CHORDS, 0, 1.0),
            (TEMPLEBELL, CHORDS, 0, 1.0),
            (KNOCK, MELODY, 0, 0.5),
            (HAT, DRUMS_CHANNEL, 0, 0.1),
            (BASS, BASS_CHANNEL, bass::PLUCKBASS, 0.5),
            (TEXTURE, TEXTURE_CHANNEL, texture::DROP, 0.5),
            (TEXTURE, TEXTURE_CHANNEL, texture::WIND, 2.0),
            (TEXTURE, TEXTURE_CHANNEL, texture::WAVES, 2.0),
            (DRUM, DRUMS_CHANNEL, drum::JINGLE, 0.1),
            (DRUM, DRUMS_CHANNEL, drum::SNARE, 0.1),
            (VOWEL, MELODY, 0, 3.0),
            (HUM, MELODY, 0, 3.0),
            (CHOIR, MELODY, 0, 3.0),
        ] {
            x[0] = if voice == VOWEL || voice == HUM || voice == CHOIR { 0.0 } else { 62.0 };
            if matches!(voice, VOWEL | HUM | CHOIR) {
                x[1] = 1.0;
                x[2] = 1.0;
                x[3] = 0.0;
            }
            core.note(voice, channel, t, 64.0, dur, 0.8, parts, x);
        }
    }

    #[test]
    fn the_same_key_makes_the_same_draws_and_another_makes_others() {
        let mut a = boxed();
        let mut b = boxed();
        let mut c = boxed();
        let mut d = boxed();
        for (core, k) in [(&mut a, key(9, 0, 4, 2)), (&mut b, key(9, 0, 4, 2)), (&mut c, key(9, 1, 4, 2)), (&mut d, key(10, 0, 4, 2))] {
            core.key(k);
            drawing_voices(core);
        }
        let all = |core: &mut Core| -> Vec<u32> {
            let mut bits = Vec::new();
            for blk in 0..(5.0 * RATE) as usize / QUANTUM {
                bits.extend(core.process((blk * QUANTUM) as u64).iter().flatten().map(|v| v.to_bits()));
            }
            bits
        };
        let (a, b, c, d) = (all(&mut a), all(&mut b), all(&mut c), all(&mut d));
        assert_eq!(a, b, "the same key");
        assert_ne!(a, c, "another pass");
        assert_ne!(a, d, "another seed");
    }

    #[test]
    fn the_core_makes_its_own_noise() {
        let core = boxed();
        assert_eq!(core.noise.samples().len(), 2 * RATE as usize);
        assert!(core.noise.ready());
        // Not silence, not clipping.
        let peak = core.noise.samples().iter().fold(0.0f32, |m, x| m.max(x.abs()));
        assert!(peak > 0.3 && peak < 2.0, "peak {peak}");
    }

    #[test]
    fn a_slide_comes_from_the_note_before_or_from_just_under() {
        use blocks::slide_of;
        // Never for a short note, and never when the draw is past the chance.
        assert_eq!(slide_of(69.0, 0.1, f64::NAN, 0.0), None);
        assert_eq!(slide_of(69.0, 1.0, f64::NAN, 0.5), None);
        // Joined by a step within a fourth: from the note before.
        let (from, reach) = slide_of(69.0, 1.0, 67.0, 0.1).expect("slides");
        assert!((from - midi_to_freq(67.0)).abs() < 1e-9);
        assert!((reach - 0.09).abs() < 1e-12);
        // A repeat, a leap or no note before: from just under.
        for prev in [69.0, 60.0, f64::NAN] {
            let (from, _) = slide_of(69.0, 0.4, prev, 0.1).expect("slides");
            assert!(from < 440.0 && from > 400.0, "{from}");
        }
        // The slide's reach follows the note, between 30 and 90 ms.
        assert!((slide_of(69.0, 0.2, 67.0, 0.1).expect("slides").1 - 0.044).abs() < 1e-12);
        assert!((slide_of(69.0, 0.18, 67.0, 0.1).expect("slides").1 - 0.0396).abs() < 1e-12);
    }

    /// A nylon note's extra values: a string of a strum.
    fn strummed() -> [f64; EXTRA] {
        let mut x = NO;
        x[0] = 1.0;
        x
    }

    /// Every channel's samples, block by block, for `seconds`.
    fn all(core: &mut Core, seconds: f64) -> Vec<f32> {
        let mut out = Vec::new();
        for b in 0..(seconds * RATE) as usize / QUANTUM {
            out.extend(core.process((b * QUANTUM) as u64).iter().flatten());
        }
        out
    }

    // A damp reaches the strings that arrived since the channel's last damp,
    // and each of them once: a second damp with no new strings changes
    // nothing, and a damp into another channel reaches no one here.
    #[test]
    fn each_note_hears_each_kind_of_event_once() {
        let (mut once, mut twice, mut never, mut elsewhere) = (boxed(), boxed(), boxed(), boxed());
        for core in [&mut once, &mut twice, &mut never, &mut elsewhere] {
            core.note(NYLON, CHORDS, 0.1, 55.0, 2.5, 0.8, 0, strummed());
        }
        once.damp(CHORDS, 0.5);
        twice.damp(CHORDS, 0.5);
        twice.damp(CHORDS, 1.0);
        elsewhere.damp(MELODY, 0.5);
        let (a, b, c, d) = (all(&mut once, 3.0), all(&mut twice, 3.0), all(&mut never, 3.0), all(&mut elsewhere, 3.0));
        assert_eq!(a, b, "a second damp reached the strum it had already damped");
        assert_ne!(a, c, "the damp was not heard");
        assert_eq!(c, d, "a damp into another channel was heard");
    }

    // A note still waiting hears an event as it starts, as one already
    // sounding hears it: the same samples either way (Chromium renders an
    // event added ahead of the clock as one scheduled with the note).
    #[test]
    fn a_waiting_note_hears_an_event_as_it_starts() {
        let (mut ahead, mut sounding) = (boxed(), boxed());
        for core in [&mut ahead, &mut sounding] {
            core.note(NYLON, CHORDS, 0.2, 55.0, 2.0, 0.8, 0, strummed());
        }
        ahead.damp(CHORDS, 0.9);
        let mut a = all(&mut ahead, 0.5);
        let mut b = all(&mut sounding, 0.5);
        assert_eq!(sounding.busy(), 1);
        sounding.damp(CHORDS, 0.9);
        a.extend(all_from(&mut ahead, 0.5, 3.0));
        b.extend(all_from(&mut sounding, 0.5, 3.0));
        assert_eq!(a, b);
    }

    /// Every channel's samples from `from` to `to` seconds.
    fn all_from(core: &mut Core, from: f64, to: f64) -> Vec<f32> {
        let mut out = Vec::new();
        for b in (from * RATE) as usize / QUANTUM..(to * RATE) as usize / QUANTUM {
            out.extend(core.process((b * QUANTUM) as u64).iter().flatten());
        }
        out
    }

    // Only a strummed nylon string listens to a damp, and no voice listens
    // yet to a release or a bend, or to a kind the core does not have: they
    // change nothing until a voice opts in (`Voice::event`).
    #[test]
    fn a_voice_hears_only_what_it_listens_to() {
        let (mut plain, mut sent) = (boxed(), boxed());
        for core in [&mut plain, &mut sent] {
            core.note(NYLON, CHORDS, 0.1, 55.0, 2.0, 0.8, 0, NO);
            core.note(FIDDLE, MELODY, 0.1, 67.0, 2.0, 0.8, 0, NO);
            core.note(KALIMBA, MELODY, 0.1, 72.0, 2.0, 0.8, STRIKE | BODY, NO);
        }
        for channel in [MELODY, CHORDS] {
            sent.damp(channel, 0.5);
            sent.event(channel, Event { kind: event::RELEASE, time: 0.6, value: 0.0 });
            sent.event(channel, Event { kind: event::BEND, time: 0.7, value: 100.0 });
            sent.event(channel, Event { kind: 99, time: 0.8, value: 0.0 });
        }
        assert_eq!(all(&mut plain, 3.0), all(&mut sent, 3.0));
    }
}
