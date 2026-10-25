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
pub mod space;
pub mod timeline;
pub mod wave;

#[cfg(all(target_arch = "wasm32", feature = "web"))]
mod wasm;

use block::{Context, Event, Mods, Note, Sends, Signal, event};
use block::event::Reach;
use blocks::{Held, Own, Pools, own};
use blocks::osc::frame_at;
use master::{Master, OUT};
use mix::{Mix, SideIn};
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
/// channels: all five of them (queue item 27), and since queue item 62 a
/// sixth of the core's own, the wash's, which no host note plays into. Since queue item 29 the core
/// mixes them too (`mix`), with the host's own notes for each (`inputs`),
/// through the channel gains, the sends, the duck, the echo and the reverb;
/// and since items 30 and 31 runs the mix through the whole master chain
/// (`master`): the wobble, the saturator, the tone and the highpass, the bus
/// compressor, the master and kill gains and the ceiling, out to the
/// speakers (`output`).
pub const CHANNELS: usize = 6;
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
/// The wash's channel (queue item 62): its own, set like the textures'
/// (level and sends), so its level follower can listen to the five layers
/// without hearing the wash.
pub const WASH_CHANNEL: u32 = 5;

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
/// The arc (queue item 63): where on its swing the next one starts, 0 to 1
/// (0 the darkest, 0.5 the brightest), for `ARC_PERIOD` to take.
pub const ARC_FROM: u32 = TONE_FREQ + 9;
/// The arc's swing, in seconds, from `time` (0 lets it go). It starts the
/// swing the other `ARC_` parameters set up, so it comes last.
pub const ARC_PERIOD: u32 = TONE_FREQ + 10;
/// The arc's shape (queue item 63b), for `ARC_PERIOD` to take: 0 the deep
/// swing, 1 the dip (`blocks::arc::Shape`).
pub const ARC_SHAPE: u32 = TONE_FREQ + 11;
/// The dip's floor, in Hz, for `ARC_PERIOD` to take.
pub const ARC_FLOOR: u32 = TONE_FREQ + 12;
/// The share of the swing the dip takes, 0 to 1, for `ARC_PERIOD` to take.
pub const ARC_DIP: u32 = TONE_FREQ + 13;
/// The width ceiling's settings (queue item 64; `blocks::width::SETTINGS`,
/// in that order): its target in dB against the mid, its cutoff in Hz, its
/// window, and its down and up time constants in seconds. A stereo render's
/// only; from the next block.
pub const WIDTH_TARGET: u32 = TONE_FREQ + 14;
/// The last of the width ceiling's settings (`WIDTH_TARGET` and on).
pub const WIDTH_UP: u32 = TONE_FREQ + 18;
/// The loop's space (queue item 65; `space::Loop`), for `SPACE_START` to
/// draw: its seed (a whole number), how busy it is (notes a second, every
/// layer), how wet (its reverb's space, 0 to 1), and whether its texture is
/// a light one (not 0).
pub const SPACE_SEED: u32 = TONE_FREQ + 19;
pub const SPACE_BUSY: u32 = TONE_FREQ + 20;
pub const SPACE_WET: u32 = TONE_FREQ + 21;
pub const SPACE_LIGHT: u32 = TONE_FREQ + 22;
/// Draw the loop's space and take it from `time` (a stereo render's; it
/// glides there over about 50 ms). Any value; 0 for no space (centered).
pub const SPACE_START: u32 = TONE_FREQ + 23;

/// How long a layout changed while the core plays takes (queue item 66;
/// `Core::set_layout`), in seconds: the side fades in over it, or out
/// before the render goes mono, so the switch neither clicks nor jumps.
pub const LAYOUT_FADE: f64 = 0.25;
/// Going mono, the master's two lanes agree a few blocks after the side
/// has faded (its delay and the compressors' lookahead still hold some):
/// within this, the most `|left - right| / 2` of a block (-100 dB), or at
/// the latest twice `LAYOUT_FADE` after the switch.
const LAYOUT_SETTLED: f32 = 1e-5;

/// Why `Core::start` or `Core::render` would not.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
    /// The sample rate is outside `MIN_RATE` to `MAX_RATE`.
    Rate,
    /// The output layout is not one the core renders: mono (one channel)
    /// and stereo (two) are; the number is how many channels were asked
    /// for.
    Layout(u32),
}

impl Error {
    /// What went wrong, in plain ASCII: a host shows it to a person.
    #[must_use]
    pub const fn message(self) -> &'static str {
        match self {
            Error::Rate => "sample rate outside what this core runs at (22050 to 192000 Hz, or 96000 in the web build)",
            Error::Layout(_) => "output layout not supported: the core renders mono (1 channel) or stereo (2)",
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

/// Things sent to a channel that wait for their time (`Core::event`,
/// `Core::set`, `Core::route`): a retune or a bend, a setting, a route. The
/// host sends them as it sends notes, seconds ahead at most, and a loop
/// sets a handful; one sent with every place taken is refused.
pub const PENDING: usize = 256;

/// The most signals routed from one channel into another's own blocks at
/// once (`Core::route`).
pub const ROUTES: usize = 8;

/// Where a routed signal is taken from (`Core::route`): a channel, as it
/// goes into the mix, or the output of one of its own blocks that runs on a
/// bus (`blocks::own`; a body).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Tap {
    /// The channel (`MELODY` and the rest).
    pub channel: u32,
    /// One of its own blocks, by number, or `None` for the whole channel.
    pub block: Option<usize>,
}

/// What a pending thing is (`Due::what`).
const DUE_EVENT: u8 = 0;
const DUE_SET: u8 = 1;
const DUE_ROUTE: u8 = 2;

/// Something sent to a channel, waiting for its time (`PENDING`). Flat,
/// with a number for what it is, so a core with none is all zeros.
#[derive(Clone, Copy)]
struct Due {
    /// The frame it is due on, and the order it was sent in: things due on
    /// the same frame are handed over in the order they came.
    frame: u64,
    seq: u64,
    /// When, on the host's clock, after any clamping to the clock.
    time: f64,
    channel: u32,
    what: u8,
    /// An event's.
    event: Event,
    /// A setting's or a route's: which of the channel's own blocks, which
    /// of its settings (or of its `MODS`), and the value; a route's tap (its
    /// channel, and its block, `own::COUNT` for the whole channel), or none.
    block: usize,
    setting: usize,
    value: f64,
    tap: Option<(u32, usize)>,
}

const NOT_DUE: Due = Due {
    frame: 0,
    seq: 0,
    time: 0.0,
    channel: 0,
    what: DUE_EVENT,
    event: Event { kind: 0, time: 0.0, value: 0.0, notes: [0.0; event::NOTES] },
    block: 0,
    setting: 0,
    value: 0.0,
    tap: None,
};

/// A signal routed from one channel into a setting of another's own block
/// (`Core::route`), and the block of it the setting hears next.
#[derive(Clone, Copy)]
struct Route {
    on: bool,
    /// The channel, the own block and the setting it drives.
    to: u32,
    block: usize,
    setting: usize,
    /// Where it is taken from: a channel, and one of its own blocks or
    /// `own::COUNT` for the whole channel.
    from: u32,
    tap: usize,
    /// The tap's last block: what the setting hears in the block after it
    /// (one block's delay, so the order the channels render in does not
    /// matter, and a route may run either way or both).
    drive: Signal,
}

const NO_ROUTE: Route = Route { on: false, to: 0, block: 0, setting: 0, from: 0, tap: 0, drive: [0.0; QUANTUM] };

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
    /// Where a stereo render places it apart from its channel (queue item
    /// 65; `space`): `UNPLACED`, or the light kit's (`KIT`, `KIT_SOFT`) or
    /// glass's (`GLASS`).
    place: u8,
}

/// A slot's place in a stereo render (`Slot::place`).
const UNPLACED: u8 = 0;
const KIT: u8 = 1;
const KIT_SOFT: u8 = 2;
const GLASS: u8 = 3;

/// Where a note is placed apart from its channel: the hat voice's hats
/// (the kit) and shakers (softer), the drums' jingles (softer), and an FM
/// note on the texture's channel, which only glass plays there but for the
/// light textures (bells and chimes), whose whole channel is placed anyway.
fn place_of(note: &Note) -> u8 {
    match (note.voice, note.channel) {
        (HAT, DRUMS_CHANNEL) if note.parts == blocks::struck::SHAKER => KIT_SOFT,
        (HAT, DRUMS_CHANNEL) => KIT,
        (DRUM, DRUMS_CHANNEL) if note.parts == blocks::drum::JINGLE || note.parts == blocks::drum::OPEN_JINGLE => KIT_SOFT,
        (FM, TEXTURE_CHANNEL) => GLASS,
        _ => UNPLACED,
    }
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
            place: UNPLACED,
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
    /// The channel events it heard while it waited, one of each kind that
    /// reaches a note once (`event::ONCE`) at most: which kinds (a bit
    /// each), and each one's time and value. It hears them as it starts.
    /// Zeros for none, so the queue stays all zeros and costs nothing in the
    /// `.wasm`. The kinds that reach every held note are handed over when
    /// due, to the notes sounding then, so a waiting note keeps none.
    heard: u8,
    events: [(f64, f64); event::ONCE],
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
    events: [(0.0, 0.0); event::ONCE],
};

/// The core: the one thing a host holds. `const` and all
/// zeros when new, so a host can keep one in a static.
// Its flags are separate states a host sets apart (the layout, a space, one
// waiting), not one state spelled in bools.
#[allow(clippy::struct_excessive_bools)]
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
    /// that reaches a note once how many had when the last such event came
    /// (`Core::event`).
    arrived: u64,
    last: [[u64; event::ONCE]; CHANNELS],
    /// What waits for its time: events for the channels' own blocks and for
    /// the notes every event of its kind reaches, settings and routes; how
    /// many; and how many have been sent.
    pending: [Due; PENDING],
    waiting_due: usize,
    sent: u64,
    /// The signals routed between channels (`Core::route`).
    routes: [Route; ROUTES],
    /// What the voices that send (`Voice::SENDS`) sent into the reverb and
    /// the echo this block, past their channels' sends.
    sends: Sends,
    /// The next arc, as the `ARC_` parameters set it up, for `ARC_PERIOD`
    /// to start.
    arc_next: blocks::arc::Settings,
    /// The mix going into the master, a block a lane (the second only in
    /// stereo).
    mixed: [Signal; OUT],
    /// The space recipe (queue item 65) and the loop it last drew for, the
    /// draw, and whether one has been set: with none, a stereo render is
    /// mono on each side.
    space: space::Space,
    space_loop: space::Loop,
    drawn: space::Drawn,
    spaced: bool,
    /// A space waiting for its frame (`SPACE_START`): whether one is, from
    /// which frame, the draw, and whether it is a space (false: centered).
    /// Plain fields, all zeros at rest (an `Option` of them would not be).
    space_waits: bool,
    space_at: u64,
    space_drawn: space::Drawn,
    space_on: bool,
    /// In a stereo render with a space, this block: each channel's notes
    /// placed apart from it (`Slot::place`), and its own blocks' width.
    placed: [Signal; CHANNELS],
    own_side: [Signal; CHANNELS],
    /// The layout the host asked for (`set_layout`), and how much of the
    /// side the render keeps (0 to 1): changed while playing, the master
    /// goes stereo at once and the side fades in, or the side fades out and
    /// then the master goes mono.
    layout: u32,
    fade: f32,
    /// The frame the host asked for mono while the master was stereo.
    layout_at: u64,
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
            last: [[0; event::ONCE]; CHANNELS],
            pending: [NOT_DUE; PENDING],
            waiting_due: 0,
            sent: 0,
            routes: [NO_ROUTE; ROUTES],
            sends: [[0.0; QUANTUM]; 2],
            arc_next: blocks::arc::ZERO,
            mixed: [[0.0; QUANTUM]; OUT],
            space: space::Space::ZERO,
            space_loop: space::NO_LOOP,
            drawn: space::ZERO,
            spaced: false,
            space_waits: false,
            space_at: 0,
            space_drawn: space::ZERO,
            space_on: false,
            placed: [[0.0; QUANTUM]; CHANNELS],
            own_side: [[0.0; QUANTUM]; CHANNELS],
            layout: 0,
            fade: 0.0,
            layout_at: 0,
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
        self.arc_next = blocks::arc::REST;
        self.clear();
        self.noise.make(rate);
        #[allow(clippy::cast_possible_truncation)] // the tables are 32-bit
        self.waves.build(rate as f32, &mut self.fft);
        let cx = Context { rate, waves: &self.waves, noise: self.noise.samples() };
        for own in &mut self.own {
            own.start(&cx);
        }
        self.arrived = 0;
        self.last = [[0; event::ONCE]; CHANNELS];
        self.sent = 0;
        self.inputs = [[0.0; QUANTUM]; CHANNELS];
        self.unserved = 0;
        self.master.set_layout(1);
        self.layout = 1;
        self.fade = 1.0;
        self.space = space::Space::new();
        self.space_loop = space::NO_LOOP;
        self.drawn = space::CENTERED;
        self.spaced = false;
        self.space_waits = false;
        self.quality(true);
    }

    /// Start the core for a host: at `rate` Hz, rendering `channels`
    /// channels, with the full mix. The first call a native or game host
    /// makes; the web host's worklet calls `init`, `quality` and
    /// `set_layout` itself.
    ///
    /// Two layouts (queue item 64; `docs/CORE.md`, promise 9): mono, one
    /// channel, and stereo, two, each a render of its own. The mono render
    /// is the one the core has always made and pays nothing for stereo; the
    /// stereo render's mid is the mono mix. Asking for another returns
    /// `Error::Layout` and leaves the core as it was.
    ///
    /// # Errors
    ///
    /// `Error::Layout` for any layout but mono or stereo, `Error::Rate` for
    /// a rate outside `MIN_RATE` to `MAX_RATE`.
    pub fn start(&mut self, rate: f64, channels: u32) -> Result<(), Error> {
        if channels != 1 && channels != 2 {
            return Err(Error::Layout(channels));
        }
        if !(MIN_RATE..=MAX_RATE).contains(&rate) {
            return Err(Error::Rate);
        }
        self.init(rate);
        self.set_layout(channels);
        Ok(())
    }

    /// Render `channels` channels from the next block: 1 (mono) or 2
    /// (stereo). False, and nothing changes, for any other. `init` starts
    /// the core mono.
    ///
    /// Before the first block it is the render from the start. After it
    /// (the listener's Mono switch, queue item 66) the sound goes on: into
    /// stereo the master takes two lanes at once (`Master::switch_layout`)
    /// and the side fades in from silence over `LAYOUT_FADE`; into mono the
    /// side fades out over it, and then the master takes one lane. Until
    /// then a mono host is given the left lane (`render`), which is the
    /// mono render by the fade's end.
    pub fn set_layout(&mut self, channels: u32) -> bool {
        if channels != 1 && channels != 2 {
            return false;
        }
        if channels == 1 && self.layout == 2 {
            self.layout_at = self.next;
        }
        self.layout = channels;
        if self.next == 0 {
            self.master.set_layout(channels as usize);
            self.fade = 1.0;
        } else if channels == 2 && self.master.layout() == 1 {
            self.master.switch_layout(2);
            self.mix.restart_side(&self.drawn);
            self.fade = 0.0;
        }
        true
    }

    /// How many channels it renders, as the host last asked: 1 or 2.
    #[must_use]
    pub fn layout(&self) -> u32 {
        self.layout
    }

    /// How many lanes the master rendered the last block in: the layout,
    /// except while a switch into mono fades the side out (2 until then).
    #[must_use]
    pub fn lanes(&self) -> u32 {
        // 1 or 2.
        #[allow(clippy::cast_possible_truncation)]
        let n = self.master.layout() as u32;
        n
    }

    /// Render the next `out[0].len()` frames, any number, into the host's
    /// buffer: one slice a channel of the layout chosen at `start`, one or
    /// two, all the same length. The call after this one carries on where this
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
    /// `Error::Layout` if `out` does not hold one slice for each channel of
    /// the layout, all the same length; nothing is rendered.
    pub fn render(&mut self, out: &mut [&mut [f32]]) -> Result<(), Error> {
        let n = self.layout as usize;
        let frames = out.first().map_or(0, |o| o.len());
        if out.len() != n || out.iter().any(|o| o.len() != frames) {
            return Err(Error::Layout(u32::try_from(out.len()).unwrap_or(u32::MAX)));
        }
        let mut done = 0;
        while done < frames {
            if self.unserved == 0 {
                let block = self.next;
                self.process(block);
                self.unserved = QUANTUM;
            }
            let from = QUANTUM - self.unserved;
            let take = self.unserved.min(frames - done);
            for (to, lane) in out.iter_mut().zip(self.master.output().iter()) {
                if let (Some(to), Some(from)) = (to.get_mut(done..done + take), lane.get(from..from + take)) {
                    to.copy_from_slice(from);
                }
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
        // The loop's space survives a rebuild.
        self.mix.set_space(&self.drawn);
        if self.spaced {
            self.master.set_width(0, self.drawn.ceiling);
        }
    }

    /// Set the space recipe's setting `which` (`space::n`, `space::SETTINGS`)
    /// to draw from `lo` to `hi` (the same number pins it), from the next
    /// loop's `SPACE_START`: a request, a slider or a game narrowing the
    /// approved baseline (queue item 65; `docs/CORE.md`, promise 9). False,
    /// and nothing changes, for no such setting or a range that isn't one.
    pub fn space_range(&mut self, which: usize, lo: f64, hi: f64) -> bool {
        self.space.range(which, lo, hi)
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
    /// block). All take any `op`. The `ARC_` ones aren't either: they set
    /// up the next arc (`blocks::arc`), and `ARC_PERIOD` starts it at
    /// `time`.
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
            ARC_FROM => {
                self.arc_next.from = value;
                true
            }
            ARC_SHAPE => {
                self.arc_next.shape = blocks::arc::Shape::from_number(value);
                true
            }
            ARC_FLOOR => {
                self.arc_next.floor = value;
                true
            }
            ARC_DIP => {
                self.arc_next.dip = value;
                true
            }
            ARC_PERIOD => {
                self.master.arc(&blocks::arc::Settings { period: value, ..self.arc_next }, time);
                true
            }
            WIDTH_TARGET..=WIDTH_UP => {
                self.master.set_width((id - WIDTH_TARGET) as usize, value);
                true
            }
            SPACE_SEED => {
                // A seed, carried in a float: a whole number under 2^32.
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let seed = value.clamp(0.0, f64::from(u32::MAX)) as u32;
                self.space_loop.seed = seed;
                true
            }
            SPACE_BUSY => {
                self.space_loop.busy = value;
                true
            }
            SPACE_WET => {
                self.space_loop.wet = value;
                true
            }
            SPACE_LIGHT => {
                self.space_loop.light = value != 0.0;
                true
            }
            SPACE_START => {
                let drawn = if value == 0.0 { space::CENTERED } else { self.space.draw(&self.space_loop) };
                self.space_waits = true;
                self.space_at = frame_at(time, self.rate);
                self.space_drawn = drawn;
                self.space_on = value != 0.0;
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
    ///
    /// Every event, setting and route still waiting for its time goes too,
    /// and every route is undone.
    pub fn clear(&mut self) {
        self.waiting = 0;
        for slot in &mut self.slots {
            slot.busy = false;
        }
        self.pools.clear();
        self.waiting_due = 0;
        self.routes = [NO_ROUTE; ROUTES];
    }

    /// Drop every note still waiting for its start time, and leave the ones
    /// sounding as they are: the host handed notes over ahead of their time
    /// and the music has jumped since (Pause, Next or Previous, queue item
    /// 39). The events still waiting for their time go with them (they
    /// belong to the music that jumped); settings and routes stay, as the
    /// loop's.
    pub fn cancel(&mut self) {
        self.waiting = 0;
        self.cancel_events(f64::NEG_INFINITY);
    }

    /// Drop the events waiting for their time whose time is `time` or later.
    fn cancel_events(&mut self, time: f64) {
        let mut i = 0;
        while i < self.waiting_due {
            let Some(&d) = self.pending.get(i) else { break };
            if d.what != DUE_EVENT || d.time < time {
                i += 1;
                continue;
            }
            self.waiting_due -= 1;
            if let Some(&last) = self.pending.get(self.waiting_due)
                && let Some(gap) = self.pending.get_mut(i)
            {
                *gap = last;
            }
        }
    }

    /// Drop the notes still waiting whose time is `time` (seconds) or later,
    /// and keep the rest, waiting or sounding: a change that must be heard
    /// at once while notes are handed over seconds ahead (mute, re-roll,
    /// tempo; queue item 40). The host decides again from `time` on, so the
    /// two halves meet exactly, by the time each note was asked for. The
    /// events waiting from `time` on go the same way.
    pub fn cancel_from(&mut self, time: f64) {
        self.cancel_events(time);
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
            events: [(0.0, 0.0); event::ONCE],
        };
        self.waiting += 1;
        self.arrived = self.arrived.wrapping_add(1);
        taken
    }

    /// A new strum into `channel` at `time` (`damp()` in synth.js): a
    /// `DAMP` event (`event`), which the strummed nylon strings of the strum
    /// before it hear. The web host's call, kept as it was.
    pub fn damp(&mut self, channel: u32, time: f64) {
        self.event(channel, Event::new(event::DAMP, time, 0.0));
    }

    /// Send `event` to `channel` (queue items 46b and 46c). Its kind says
    /// how far it reaches the channel's notes (`event::reach`):
    ///
    /// - **once** (a damp, a release): every note in the channel still held
    ///   at the event's time (its note ends after it) that arrived since the
    ///   channel's last event of the same kind -- so each note hears each
    ///   such kind at most once, which is how a damp reaches the strum
    ///   before it and no further. Handed over as it comes: Web Audio adds
    ///   such an event to a timeline that may already be rendering, and
    ///   Chromium renders an event added ahead of the clock exactly as one
    ///   scheduled with the note, so a note already sounding takes it as it
    ///   is, and a note still waiting takes it when it starts. One whose
    ///   time has already been rendered is not caught up: Chromium moves an
    ///   event added in the past to the start of the next block it renders
    ///   (`ClampNewEventsToCurrentTime`), and a sounding note hears it there.
    /// - **every time** (a bend, a retune): every note of the channel held
    ///   when its time comes -- started by then, and its note not yet ended
    ///   -- as often as such events are sent. Handed over when due, before
    ///   the block holding its time renders.
    ///
    /// The channel's own blocks hear every event when its time comes. What
    /// a note or a block does with one is its own (`Block::event`): today
    /// only a strummed nylon string listens, to a damp. False for a kind or
    /// channel the core does not have, a time that is not a number, or an
    /// event with every pending place taken (`PENDING`), which the channel's
    /// own blocks then miss.
    pub fn event(&mut self, channel: u32, event: Event) -> bool {
        let Some(reach) = event::reach(event.kind) else {
            return false;
        };
        if reach == Reach::Once {
            let Some(last) = self
                .last
                .get_mut(channel as usize)
                .and_then(|kinds| kinds.get_mut(event.kind as usize))
            else {
                return false;
            };
            let since = *last;
            *last = self.arrived;
            let time = event.time;
            if !time.is_finite() {
                return false;
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
        if channel as usize >= CHANNELS || !event.time.is_finite() {
            return false;
        }
        self.post(Due { channel, what: DUE_EVENT, event, time: event.time, ..NOT_DUE })
    }

    /// Set setting number `setting` (`Block::SETTINGS`) of `channel`'s own
    /// block number `block` (`blocks::own`) to `value` at `time` seconds
    /// (queue item 46c): a loop's sweep period, duck depth or registration,
    /// as `param` sets the mix's. The block hears it when its time comes,
    /// before the block holding that time renders; a time already rendered
    /// is the next block's start. False for no such channel, block or
    /// setting, a number that is not one, or every pending place taken.
    pub fn set(&mut self, channel: u32, block: usize, setting: usize, value: f64, time: f64) -> bool {
        if channel as usize >= CHANNELS || setting >= blocks::own_settings(block) || !(value.is_finite() && time.is_finite()) {
            return false;
        }
        self.post(Due { channel, what: DUE_SET, time, block, setting, value, ..NOT_DUE })
    }

    /// Route a signal into setting number `setting` (`Block::MODS`) of
    /// `channel`'s own block number `block`, from `time` seconds on (queue
    /// item 46c): the kick's channel into the bass's duck, the other layers
    /// into the wash's follower. `from` is where it is taken (`Tap`): a
    /// channel as it goes into the mix, or one of its own blocks' bus; `None`
    /// undoes the route. One signal a setting: a second route into it takes
    /// the first's place.
    ///
    /// **The order:** a routed signal is the tap's previous block, so a
    /// setting hears its source one block (`QUANTUM` frames) late. The
    /// channels may then render in any order and a route may run either
    /// way between two channels, or both; and since the core's blocks are
    /// its own, never the host's, it is the same on every target, rate and
    /// host block size. A route starts and stops at the start of the block
    /// holding `time`; the first block it is on, its setting hears silence.
    ///
    /// False for no such channel, block, setting or tap, a time that is not
    /// a number, or every pending place taken. More than `ROUTES` at once
    /// are left out.
    pub fn route(&mut self, channel: u32, block: usize, setting: usize, from: Option<Tap>, time: f64) -> bool {
        if channel as usize >= CHANNELS || setting >= blocks::own_mods(block) || !time.is_finite() {
            return false;
        }
        let tap = match from {
            None => None,
            Some(Tap { channel: c, block: b }) => {
                let b = b.unwrap_or(own::COUNT);
                if c as usize >= CHANNELS || b > own::COUNT {
                    return false;
                }
                Some((c, b))
            }
        };
        self.post(Due { channel, what: DUE_ROUTE, time, block, setting, tap, ..NOT_DUE })
    }

    /// Queue `due` for its time: at the frame its time falls on, or the next
    /// block's start for a time already rendered.
    fn post(&mut self, mut due: Due) -> bool {
        let now = self.now();
        if due.time < now {
            due.time = now;
            due.event.time = now;
        }
        due.frame = frame_at(due.time, self.rate).max(self.next);
        due.seq = self.sent;
        let Some(gap) = self.pending.get_mut(self.waiting_due) else {
            return false;
        };
        *gap = due;
        self.waiting_due += 1;
        self.sent = self.sent.wrapping_add(1);
        true
    }

    /// Hand over everything due before frame `end`, in order: by frame, then
    /// as sent.
    fn deliver(&mut self, end: u64) {
        while self.waiting_due > 0 {
            let mut first: Option<(usize, u64, u64)> = None;
            for (i, d) in self.pending.iter().take(self.waiting_due).enumerate() {
                if d.frame < end && first.is_none_or(|(_, f, q)| (d.frame, d.seq) < (f, q)) {
                    first = Some((i, d.frame, d.seq));
                }
            }
            let Some((i, _, _)) = first else { break };
            let Some(&d) = self.pending.get(i) else { break };
            self.waiting_due -= 1;
            if let Some(&last) = self.pending.get(self.waiting_due)
                && let Some(gap) = self.pending.get_mut(i)
            {
                *gap = last;
            }
            self.hand_over(&d);
        }
    }

    /// One thing come due.
    fn hand_over(&mut self, d: &Due) {
        let channel = d.channel as usize;
        let cx = Context { rate: self.rate, waves: &self.waves, noise: self.noise.samples() };
        let Some(own) = self.own.get_mut(channel) else { return };
        match d.what {
            DUE_EVENT => {
                own.event(&d.event);
                if event::reach(d.event.kind) == Some(Reach::Every) {
                    for slot in &mut self.slots {
                        if hears_every(slot, channel, d.frame, d.time) {
                            blocks::event(&mut slot.voice, &mut self.pools, &d.event);
                        }
                    }
                }
            }
            DUE_SET => {
                own.set(d.block, d.setting, d.value, d.time, &cx);
            }
            _ => {
                let same = |r: &Route| r.on && r.to == d.channel && r.block == d.block && r.setting == d.setting;
                let at = self.routes.iter().position(same).or_else(|| self.routes.iter().position(|r| !r.on));
                if let Some(r) = at.and_then(|i| self.routes.get_mut(i)) {
                    *r = match d.tap {
                        Some((from, tap)) => Route { on: true, to: d.channel, block: d.block, setting: d.setting, from, tap, drive: [0.0; QUANTUM] },
                        None => NO_ROUTE,
                    };
                }
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
                blocks::event(&mut slot.voice, &mut self.pools, &Event::new(kind, time, value));
            }
        }
        slot.channel = w.note.channel as usize;
        slot.place = place_of(&w.note);
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
    // One block, in the order it happens: split, it would scatter the
    // order a block renders in across helpers.
    #[allow(clippy::too_many_lines)]
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
        // Then the events, settings and routes due in this block.
        if self.waiting_due > 0 {
            self.deliver(end);
        }
        for bus in self.buses.iter_mut().flatten() {
            bus.fill(0.0);
        }
        let cx = Context { rate: self.rate, waves: &self.waves, noise: self.noise.samples() };
        let mut sent = false;
        // A stereo render with a space: the notes placed apart from their
        // channel (`Slot::place`) and the channel blocks' own width, each
        // block afresh.
        if self.space_waits && self.space_at < end {
            let (drawn, on) = (self.space_drawn, self.space_on);
            self.drawn = drawn;
            self.spaced = on;
            self.space_waits = false;
            self.mix.set_space(&drawn);
            if on {
                self.master.set_width(0, drawn.ceiling);
            }
        }
        // Into mono once the side has faded out (`set_layout`), or at once
        // with no side to fade, and the two lanes agree.
        if self.layout == 1 && self.master.layout() == 2 && (self.fade <= 0.0 || !self.spaced) {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let late = self.layout_at + (2.0 * LAYOUT_FADE * self.rate) as u64;
            if self.master.side_peak() < LAYOUT_SETTLED || block >= late {
                self.master.switch_layout(1);
            }
        }
        let spaced = self.master.layout() == 2 && self.spaced;
        if spaced {
            self.placed = [[0.0; QUANTUM]; CHANNELS];
            self.own_side = [[0.0; QUANTUM]; CHANNELS];
        }
        // A place is a gain, a 32-bit float as every gain the mix holds.
        #[allow(clippy::cast_possible_truncation)]
        let places = [0.0, self.drawn.kit, self.drawn.kit * self.drawn.kit_soft, self.drawn.glass].map(|x| x as f32);
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
            let place = if spaced { places.get(usize::from(slot.place)).copied().unwrap_or(0.0) } else { 0.0 };
            // What a placed note added to its channel, taken as the
            // difference, so the channel (the mid) is the mono render's.
            let before = if place == 0.0 { None } else { Some(*out) };
            blocks::render(&mut slot.voice, &mut self.pools, block, &cx, out, buses, &mut self.sends, &mut sent);
            if let (Some(before), Some(placed)) = (before, self.placed.get_mut(slot.channel)) {
                // A note's place is the sign and size of the kit's or glass's;
                // the weight is relative to the hats' (`space`), so it is
                // carried here and the mix applies none.
                for ((p, &now), &was) in placed.iter_mut().zip(out.iter()).zip(before.iter()) {
                    *p += (now - was) * place;
                }
            }
            if slot.end <= end {
                slot.busy = false;
                blocks::free(&slot.voice, &mut self.pools);
            }
        }
        // Each channel: every note that plays through one of its own blocks
        // (a body) through it, and the blocks that add their own sound; the
        // host's own notes; then the blocks on the whole channel. Its tail
        // stops at the silence floor on its way into the mix (`math`,
        // "Subnormal numbers"). The signals routed in are the taps' last
        // block (`Core::route`).
        let routes = &self.routes;
        for (n, (((out, buses), own), input)) in
            self.out.iter_mut().zip(self.buses.iter_mut()).zip(self.own.iter_mut()).zip(self.inputs.iter_mut()).enumerate()
        {
            let mut mods = [Mods::NONE; own::COUNT];
            for r in routes.iter().filter(|r| r.on && r.to as usize == n) {
                if let Some(m) = mods.get_mut(r.block) {
                    *m = m.with(r.setting, &r.drive);
                }
            }
            let side = if spaced { self.own_side.get_mut(n) } else { None };
            own.buses(block, &cx, buses, out, &mods, &mut self.sends, &mut sent, side);
            if own::ANY_WHOLE {
                for (y, x) in out.iter_mut().zip(input.iter()) {
                    *y += *x;
                }
                own.whole(block, &cx, out, &mods);
                for y in out.iter_mut() {
                    *y = math::silence(*y);
                }
            } else {
                for (y, x) in out.iter_mut().zip(input.iter()) {
                    *y = math::silence(*y + *x);
                }
            }
            input.fill(0.0);
        }
        // What each route's setting hears in the next block.
        for r in self.routes.iter_mut().filter(|r| r.on) {
            let tap = match self.buses.get(r.from as usize).and_then(|b| b.get(r.tap)) {
                Some(bus) => bus,
                None => match self.out.get(r.from as usize) {
                    Some(out) => out,
                    None => continue,
                },
            };
            r.drive = *tap;
        }
        // The mix: the mono mix, and in a stereo render with a space its
        // side (queue item 65), left = mid + side and right = mid - side.
        // With no space both lanes are the mid.
        let lift = self.master.arc_lift(block);
        let side_in = SideIn { placed: &self.placed, own: &self.own_side, space: &self.drawn };
        let [first, second] = &mut self.mixed;
        *first = *self.mix.process(block, &self.out, if sent { Some(&self.sends) } else { None }, lift.as_ref(), spaced.then_some(&side_in));
        let want = if self.layout == 2 { 1.0 } else { 0.0 };
        if spaced && self.layout == 2 && self.fade >= 1.0 {
            for ((l, r), &s) in first.iter_mut().zip(second.iter_mut()).zip(self.mix.side().iter()) {
                let m = *l;
                *l = m + s;
                *r = m - s;
            }
        } else if spaced {
            // A layout changing (`set_layout`): the side faded in or out,
            // frame by frame.
            #[allow(clippy::cast_possible_truncation)]
            let step = (1.0 / (LAYOUT_FADE * self.rate)) as f32;
            let mut fade = self.fade;
            for ((l, r), &s) in first.iter_mut().zip(second.iter_mut()).zip(self.mix.side().iter()) {
                fade = if want > fade { (fade + step).min(want) } else { (fade - step).max(want) };
                let m = *l;
                *l = m + s * fade;
                *r = m - s * fade;
            }
            self.fade = fade;
        } else if self.master.layout() == 2 {
            *second = *first;
        }
        self.master.process(block, &self.mixed, &self.waves);
        &self.out
    }
}

/// Whether the note in `slot` hears an event that reaches every held note
/// (`event::Reach::Every`) sent to `channel`, due on `frame` at `time`: it
/// is sounding in that channel, it has started by then, and its note has not
/// ended.
fn hears_every(slot: &Slot, channel: usize, frame: u64, time: f64) -> bool {
    slot.busy && slot.channel == channel && slot.begin <= frame && slot.until > time
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
            core.note(KALIMBA, CHANNELS as u32, 0.1, 60.0, 0.5, 0.5, STRIKE, NO),
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
            (48_000.0, 3, Error::Layout(3)),
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
        assert_eq!(core.start(48_000.0, 3), Err(Error::Layout(3)));
        assert!(core.mixing());
        assert_eq!(core.layout(), 1);
        // Stereo (queue item 64).
        assert_eq!(core.start(48_000.0, 2), Ok(()));
        assert_eq!(core.layout(), 2);
        assert!(!core.set_layout(3));
        assert_eq!(core.layout(), 2);
    }

    /// With nothing placed or widened, each side of the stereo render is the
    /// mono render, bit for bit (queue item 64): the mix's mid is the mono
    /// mix, the saturator and filters run on each side alike, and the
    /// compressors, keyed by the mid, press as mono's do. A loud loop, so
    /// both compressors are pressing, with the arc on, at full and lite
    /// quality and two rates.
    #[test]
    fn stereo_with_nothing_placed_is_mono_on_each_side_bit_for_bit() {
        let play = |channels: u32, rate: f64, full: bool| -> Vec<Vec<u32>> {
            let mut core = boxed();
            core.start(rate, channels).expect("a layout");
            core.quality(full);
            core.param(ARC_FROM, mix::SET, 0.45, 0.0, 0.0);
            core.param(ARC_PERIOD, mix::SET, 2.0, 0.0, 0.0);
            for i in 0..40 {
                let t = f64::from(i) * 0.05;
                core.key(key(3, 0, i, 0));
                core.note(KALIMBA, MELODY, t + 0.001, 60.0 + f64::from(i % 12), 0.4, 1.0, STRIKE | BODY, NO);
                core.key(key(3, 0, i, 1));
                core.note(DRUM, DRUMS_CHANNEL, t, 0.0, 0.3, 1.0, 0, NO);
            }
            let frames = (2.5 * rate) as usize;
            let mut sides = vec![vec![0.0f32; frames]; channels as usize];
            let mut refs: Vec<&mut [f32]> = sides.iter_mut().map(|s| &mut s[..]).collect();
            core.render(&mut refs).expect("render");
            assert!(core.reduction(0) < -0.5, "the bus compressor presses: {}", core.reduction(0));
            sides.iter().map(|s| s.iter().map(|x| x.to_bits()).collect()).collect()
        };
        for (rate, full) in [(48_000.0, true), (48_000.0, false), (44_100.0, true)] {
            let mono = play(1, rate, full);
            let stereo = play(2, rate, full);
            assert!(mono[0].iter().any(|&b| f32::from_bits(b) != 0.0));
            assert!(stereo[0] == mono[0], "left, {rate} Hz, full {full}");
            assert!(stereo[1] == mono[0], "right, {rate} Hz, full {full}");
        }
    }

    /// padSweep (queue item 72): the chords' channel through a lowpass that
    /// starts closed and is open half a period later; a new sweep glides from
    /// where the cutoff was (no jump); with its octaves set to 0 it glides
    /// open and then passes the channel as it is.
    #[test]
    fn pad_sweep_swings_the_chords_open_and_shut() {
        use blocks::{own, pad_sweep};
        let play = |sweep: Option<[f64; 4]>, off_at: Option<f64>| -> Vec<f32> {
            let mut core = boxed();
            core.key(key(6, 0, 0, 0));
            // A held tone at C6 (1047 Hz): past the sweep's closed cutoff,
            // under its open one.
            core.note(SINE, CHORDS, 0.0, 84.0, 6.0, 1.0, 0, NO);
            if let Some(settings) = sweep {
                for (n, v) in [(pad_sweep::LO, settings[0]), (pad_sweep::PERIOD, settings[2]), (pad_sweep::Q, settings[3]), (pad_sweep::OCT, settings[1])] {
                    assert!(core.set(CHORDS, own::PAD_SWEEP, n, v, 0.5));
                }
            }
            if let Some(t) = off_at {
                assert!(core.set(CHORDS, own::PAD_SWEEP, pad_sweep::OCT, 0.0, t));
            }
            (0..(5.0 * RATE) as usize / QUANTUM).flat_map(|b| core.process((b * QUANTUM) as u64)[CHORDS as usize]).collect()
        };
        let rms = |x: &[f32], a: f64, b: f64| {
            let s = &x[(a * RATE) as usize..(b * RATE) as usize];
            (s.iter().map(|v| f64::from(*v).powi(2)).sum::<f64>() / s.len() as f64).sqrt()
        };
        let plain = play(None, None);
        let swept = play(Some([450.0, 3.0, 2.0, 1.0]), None);
        // A setting takes hold from the start of the block holding its time.
        let before = (0.5 * RATE) as usize / QUANTUM * QUANTUM;
        assert!(plain[..before] == swept[..before], "untouched before it starts");
        let closed = 20.0 * (rms(&swept, 0.55, 0.65) / rms(&plain, 0.55, 0.65)).log10();
        let open = 20.0 * (rms(&swept, 1.45, 1.55) / rms(&plain, 1.45, 1.55)).log10();
        assert!(closed < -6.0, "closed at its start: {closed:.1} dB");
        assert!(open > -1.0, "open half a period later: {open:.1} dB");
        // Off at 2.5 s: open within half a second, then the channel as it is.
        let off = play(Some([450.0, 3.0, 2.0, 1.0]), Some(2.5));
        let a = (3.2 * RATE) as usize;
        let diff = off[a..].iter().zip(plain[a..].iter()).fold(0.0f32, |m, (x, y)| m.max((x - y).abs()));
        assert!(diff < 1e-3, "off, the channel passes as it is: {diff}");
        // No jump anywhere: frame to frame it moves no more than the tone does.
        let step = |x: &[f32]| x.windows(2).fold(0.0f32, |m, w| m.max((w[1] - w[0]).abs()));
        assert!(step(&off) <= step(&plain) * 1.05, "no click: {} against {}", step(&off), step(&plain));
        // A new loop's sweep at 1.5 s, the old one wide open: it glides there,
        // and is closed again a period after it started.
        let mut core = boxed();
        core.key(key(6, 0, 0, 0));
        core.note(SINE, CHORDS, 0.0, 84.0, 6.0, 1.0, 0, NO);
        for (t, lo) in [(0.5, 450.0), (1.5, 700.0)] {
            for (n, v) in [(pad_sweep::LO, lo), (pad_sweep::PERIOD, 2.0), (pad_sweep::Q, 1.0), (pad_sweep::OCT, 3.0)] {
                assert!(core.set(CHORDS, own::PAD_SWEEP, n, v, t));
            }
        }
        let next: Vec<f32> = (0..(4.0 * RATE) as usize / QUANTUM).flat_map(|b| core.process((b * QUANTUM) as u64)[CHORDS as usize]).collect();
        assert!(step(&next) <= step(&plain) * 1.05, "a new loop's sweep glides in: {} against {}", step(&next), step(&plain));
        let then = 20.0 * (rms(&next, 3.45, 3.55) / rms(&plain, 3.45, 3.55)).log10();
        assert!(then < -3.0, "and closed a period on: {then:.1} dB");
    }

    /// subDuck (queue item 68): the bass channel dips at each kick of the
    /// drums' channel routed into it, by its depth (scaled by the kick's
    /// velocity, full at 0.8), within 10-50 ms of the kick, and is back
    /// within 1 dB by a quarter second; the drums are untouched, and with no
    /// depth set nothing moves.
    #[test]
    fn sub_duck_dips_the_bass_at_each_kick() {
        use blocks::{own, sub_duck};
        let play = |depth: f64, tuned: bool, vel: f64| -> (Vec<f32>, Vec<f32>) {
            let mut core = boxed();
            core.key(key(5, 0, 0, 0));
            core.note(SINE, BASS_CHANNEL, 0.0, 45.0, 4.0, 1.0, 0, NO);
            if depth > 0.0 {
                assert!(core.set(BASS_CHANNEL, own::SUB_DUCK, sub_duck::DEPTH, depth, 0.0));
                assert!(core.set(BASS_CHANNEL, own::SUB_DUCK, sub_duck::RECOVER, 0.2, 0.0));
                assert!(core.route(BASS_CHANNEL, own::SUB_DUCK, sub_duck::KEY, Some(Tap { channel: DRUMS_CHANNEL, block: None }), 0.0));
            }
            for (i, t) in [1.0, 1.5, 2.0, 2.5].into_iter().enumerate() {
                core.key(key(5, 0, 1 + i as u32, 0));
                if tuned {
                    core.note(TUNEDKICK, DRUMS_CHANNEL, t, 0.0, 2.5, vel, drum::KICK, blocks::kick::extra(58.27, 62.0, 1.0, 0.25));
                } else {
                    core.note(DRUM, DRUMS_CHANNEL, t, 0.0, 0.3, vel, drum::KICK, NO);
                }
            }
            let (mut bass, mut drums) = (Vec::new(), Vec::new());
            for b in 0..(3.2 * RATE) as usize / QUANTUM {
                let out = core.process((b * QUANTUM) as u64);
                bass.extend(out[BASS_CHANNEL as usize]);
                drums.extend(out[DRUMS_CHANNEL as usize]);
            }
            (bass, drums)
        };
        let rms = |x: &[f32], from: f64, to: f64| {
            let s = &x[(from * RATE) as usize..(to * RATE) as usize];
            (s.iter().map(|v| f64::from(*v).powi(2)).sum::<f64>() / s.len() as f64).sqrt()
        };
        let db = |a: f64, b: f64| 20.0 * (a / b).log10();
        let (plain, plain_drums) = play(0.0, false, 0.8);
        for tuned in [false, true] {
            let (ducked, drums) = play(7.0, tuned, 0.8);
            let (_, kept) = play(0.0, tuned, 0.8);
            assert!(drums == kept, "the drums are untouched");
            for t in [1.0, 1.5, 2.0, 2.5] {
                // The deepest 5 ms between 10 and 50 ms after the kick.
                let dip = (0..8)
                    .map(|k| {
                        let a = t + 0.01 + 0.005 * f64::from(k);
                        db(rms(&ducked, a, a + 0.005), rms(&plain, a, a + 0.005))
                    })
                    .fold(0.0, f64::min);
                let back = db(rms(&ducked, t + 0.25, t + 0.3), rms(&plain, t + 0.25, t + 0.3));
                assert!((-9.0..=-5.0).contains(&dip), "tuned {tuned}: {dip:.2} dB 10-50 ms after the kick at {t}");
                assert!(back > -1.0, "tuned {tuned}: back within 1 dB by 0.25 s: {back:.2}");
            }
            let before = db(rms(&ducked, 0.2, 0.95), rms(&plain, 0.2, 0.95));
            assert!(before.abs() < 1e-6, "nothing moves before the first kick: {before}");
        }
        // A softer kick dips less; with no depth, nothing at all.
        let (soft, _) = play(7.0, false, 0.4);
        let dip = (0..8)
            .map(|k| {
                let a = 1.01 + 0.005 * f64::from(k);
                db(rms(&soft, a, a + 0.005), rms(&plain, a, a + 0.005))
            })
            .fold(0.0, f64::min);
        assert!(dip > -5.5 && dip < -2.0, "a kick at half velocity, about half the depth: {dip:.2}");
        let (off, off_drums) = play(0.0, false, 0.8);
        assert!(off == plain && off_drums == plain_drums);
    }

    /// A tuned kick's next hit chokes it (queue item 67): a `CHOKE` sent to
    /// the drums' channel reaches a boom still ringing, which falls away in
    /// a few ms, and never the hit it was sent with.
    #[test]
    fn a_tuned_kick_is_choked_by_the_next() {
        let play = |first: bool, choke: bool, second: bool| -> Vec<f32> {
            let mut core = boxed();
            let boom = blocks::kick::extra(58.27, 50.0, 1.0, 0.3);
            if first {
                core.key(key(4, 0, 0, 0));
                core.note(TUNEDKICK, DRUMS_CHANNEL, 0.1, 0.0, 2.5, 0.8, drum::KICK, boom);
            }
            if choke {
                assert!(core.event(DRUMS_CHANNEL, Event::new(event::CHOKE, 0.6, 0.0)));
            }
            if second {
                core.key(key(4, 0, 1, 0));
                core.note(TUNEDKICK, DRUMS_CHANNEL, 0.6, 0.0, 2.5, 0.8, drum::KICK, boom);
            }
            (0..(2.0 * RATE) as usize / QUANTUM)
                .flat_map(|b| core.process((b * QUANTUM) as u64)[DRUMS_CHANNEL as usize])
                .collect()
        };
        let peak = |x: &[f32]| x.iter().fold(0.0f32, |m, v| m.max(v.abs()));
        let at = |t: f64| (t * RATE) as usize;
        let (rang, choked) = (play(true, false, false), play(true, true, false));
        // Up to the choke the same curve, but for how the ramp kept up to it
        // is stepped (`cancelAndHoldAtTime`'s).
        let before = rang[..at(0.6)].iter().zip(choked[..at(0.6)].iter()).fold(0.0f32, |m, (a, b)| m.max((a - b).abs()));
        assert!(before < 1e-6, "the same up to the choke: {before}");
        assert!(peak(&rang[at(0.7)..]) > 0.005, "a boom rings on: {}", peak(&rang[at(0.7)..]));
        // Down to -80 dB within 30 ms (the sketch's target), and done after.
        assert!(peak(&choked[at(0.63)..]) < 1.5e-4, "choked: {}", peak(&choked[at(0.63)..]));
        assert!(peak(&choked[at(0.7)..]) == 0.0, "choked, it is gone: {}", peak(&choked[at(0.7)..]));
        // The second hit, sent with its choke, plays whole.
        let (both, alone) = (play(true, true, true), play(false, false, true));
        let diff = both[at(0.7)..].iter().zip(alone[at(0.7)..].iter()).fold(0.0f32, |m, (a, b)| m.max((a - b).abs()));
        assert!(diff < 1e-6 && peak(&both[at(0.7)..]) > 0.005, "the new hit is untouched: {diff}");
    }

    /// The listener's Mono switch (queue item 66), flipped while a chord
    /// plays: until then the render is the mono one bit for bit; into stereo
    /// the side starts from silence and fades in; back into mono it fades
    /// out, then the master goes to one lane. Neither way clicks: a step
    /// would show in the side's second difference, which stays near what
    /// the stereo render's own does over the same frames.
    #[test]
    fn the_layout_switches_while_playing_without_a_click() {
        let rate = 48_000.0;
        let play = |switch: &[(f64, u32)]| -> (Vec<f32>, Vec<f32>, Vec<u32>) {
            let mut core = boxed();
            core.start(rate, 1).expect("mono");
            for (id, value) in [(SPACE_SEED, 7.0), (SPACE_BUSY, 4.0), (SPACE_WET, 0.6), (SPACE_LIGHT, 0.0), (SPACE_START, 1.0)] {
                core.param(id, mix::SET, value, 0.0, 0.0);
            }
            for (i, midi) in [60.0, 64.0, 67.0, 72.0].into_iter().enumerate() {
                core.key(key(5, 0, 0, i as u32));
                core.note(SOFTPAD, CHORDS, 0.05, midi, 3.9, 0.8, 0, NO);
            }
            let (mut left, mut right, mut lanes) = (Vec::new(), Vec::new(), Vec::new());
            let mut switches = switch.iter().peekable();
            for b in 0..(4.0 * rate) as usize / QUANTUM {
                let t = (b * QUANTUM) as f64 / rate;
                while let Some(&&(at, n)) = switches.peek()
                    && at <= t
                {
                    assert!(core.set_layout(n));
                    switches.next();
                }
                // As the worklet plays it: both lanes while there are two.
                core.process((b * QUANTUM) as u64);
                let [l, r] = core.output();
                left.extend_from_slice(l);
                right.extend_from_slice(if core.lanes() == 2 { r } else { l });
                lanes.push(core.lanes());
            }
            (left, right, lanes)
        };
        let (mono, _, _) = play(&[]);
        let (stereo, stereo_right, _) = play(&[(0.0, 2)]);
        let (left, right, lanes) = play(&[(1.0, 2), (2.5, 1)]);
        let into = (1.0 * rate) as usize;
        let out = (2.5 * rate) as usize;
        let fade = (LAYOUT_FADE * rate) as usize;
        // The fade out starts with the first block after the switch.
        let fading = out.div_ceil(QUANTUM) * QUANTUM;
        let bits = |x: &[f32]| x.iter().map(|v| v.to_bits()).collect::<Vec<_>>();
        assert!(bits(&left[..into]) == bits(&mono[..into]), "mono until the switch");
        // The stereo render from the start has a side; the switched one's
        // starts at nothing and comes in over the fade.
        let side = |l: &[f32], r: &[f32]| l.iter().zip(r).map(|(a, b)| (a - b).abs() * 0.5).fold(0.0f32, f32::max);
        let full = side(&stereo[into..out], &stereo_right[into..out]);
        assert!(full > 1e-3, "a side to fade: {full}");
        assert!(side(&left[into..into + QUANTUM], &right[into..into + QUANTUM]) < 0.02 * full);
        assert!(side(&left[into + fade..out], &right[into + fade..out]) > 0.3 * full);
        // Two lanes from the switch in until the side has faded out and the
        // lanes agree, then one: within twice the fade.
        let block = |frame: usize| frame / QUANTUM;
        assert_eq!(lanes[block(into) - 1], 1);
        assert_eq!(lanes[block(into)], 2);
        let mono_at = lanes.iter().enumerate().skip(block(out)).find(|&(_, &n)| n == 1).map(|(b, _)| b * QUANTUM).expect("mono again");
        assert!(mono_at >= fading + fade && mono_at <= fading + 2 * fade, "mono at frame {mono_at}");
        assert!(lanes[block(mono_at)..].iter().all(|&n| n == 1));
        assert!(side(&left[mono_at - QUANTUM..mono_at], &right[mono_at - QUANTUM..mono_at]) < 1e-3 * full);
        // No click: the side's second difference, over each switch, within
        // one and a half times the stereo render's over the same frames
        // (0.7 and 0.85 times; switched at once, without the fade, 2.3 and
        // 2.7 times).
        let bend = |l: &[f32], r: &[f32], from: usize, to: usize| {
            (from..to)
                .map(|n| {
                    let s = |k: usize| (l[k] - r[k]) * 0.5;
                    (s(n + 1) - 2.0 * s(n) + s(n - 1)).abs()
                })
                .fold(0.0f32, f32::max)
        };
        for (from, to) in [(into - QUANTUM, into + fade), (fading - QUANTUM, mono_at + QUANTUM)] {
            let (switched, own) = (bend(&left, &right, from, to), bend(&stereo, &stereo_right, from, to));
            assert!(switched <= 1.5 * own, "a click in frames {from} to {to}: {switched} against {own}");
        }
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
            sent.event(channel, Event::new(event::RELEASE, 0.6, 0.0));
            sent.event(channel, Event::new(event::BEND, 0.7, 100.0));
            sent.event(channel, Event::new(99, 0.8, 0.0));
        }
        assert_eq!(all(&mut plain, 3.0), all(&mut sent, 3.0));
    }

    /// Every channel's output through `Core::render`, `sizes` frames a call
    /// in turn, for `frames` frames, after `setup`.
    fn rendered(setup: impl Fn(&mut Core), sizes: &[usize], frames: usize) -> Vec<u32> {
        let mut core = boxed();
        setup(&mut core);
        let mut all = Vec::new();
        let mut at = 0;
        while all.len() < frames {
            let want = sizes[at % sizes.len()].min(frames - all.len());
            at += 1;
            let mut buffer = vec![0.0f32; want];
            core.render(&mut [&mut buffer]).expect("mono");
            all.extend(buffer.iter().map(|x| x.to_bits()));
        }
        all
    }

    /// A channel's samples, block by block, for `seconds`.
    fn channel(core: &mut Core, ch: usize, seconds: f64) -> Vec<f32> {
        let mut out = Vec::new();
        for b in 0..(seconds * RATE) as usize / QUANTUM {
            out.extend(core.process((b * QUANTUM) as u64)[ch].iter());
        }
        out
    }

    /// The first sample at which two renders differ.
    fn first_difference(a: &[f32], b: &[f32]) -> Option<usize> {
        a.iter().zip(b).position(|(x, y)| x.to_bits() != y.to_bits())
    }

    // A setting reaches a channel's own block when its time comes, not
    // before: the swell (a test-only demonstration, `own::SWELL`) on the melody,
    // set to swing from 0.3 s, leaves every block before the one holding
    // 0.3 s as it was. The same whatever size of call the host renders in.
    #[test]
    fn a_setting_reaches_a_channels_own_block_when_its_time_comes() {
        // The swell's settings, by number: its rate, then its depth.
        const SWING: usize = 0;
        const DEPTH: usize = 1;
        let play = |core: &mut Core| {
            core.note(KALIMBA, MELODY, 0.05, 69.0, 1.5, 0.8, STRIKE | BODY, NO);
        };
        let (mut plain, mut set) = (boxed(), boxed());
        play(&mut plain);
        play(&mut set);
        assert!(set.set(MELODY, own::SWELL, SWING, 3.0, 0.3));
        assert!(set.set(MELODY, own::SWELL, DEPTH, 0.5, 0.3));
        let (a, b) = (channel(&mut plain, 0, 1.0), channel(&mut set, 0, 1.0));
        let at = frame_at(0.3, RATE) as usize;
        let first = first_difference(&a, &b).expect("the swell is heard");
        assert!(first >= at / QUANTUM * QUANTUM, "heard at {first}, before {at}");
        // Not a block, setting or channel the core has.
        assert!(!set.set(MELODY, own::SWELL, 2, 1.0, 0.3));
        assert!(!set.set(MELODY, own::COUNT, 0, 1.0, 0.3));
        assert!(!set.set(CHANNELS as u32, own::SWELL, DEPTH, 1.0, 0.3));
        assert!(!set.set(MELODY, own::SWELL, DEPTH, f64::NAN, 0.3));
        // A body takes no settings.
        assert!(!set.set(MELODY, own::NYLON_BODY, 0, 1.0, 0.3));
        let host = |core: &mut Core| {
            play(core);
            core.set(MELODY, own::SWELL, DEPTH, 0.5, 0.3);
        };
        let whole = rendered(host, &[40_000], 40_000);
        for sizes in [&[1][..], &[441], &[127, 1024, 3]] {
            assert!(rendered(host, sizes, 40_000) == whole, "calls of {sizes:?} frames");
        }
    }

    // A retune reaches a held instrument every time it is sent: the
    // chorale (a test-only demonstration, `own::CHORALE`) in the chords hears all
    // five, and a note the chords share sounds on untouched -- a retune to
    // the chord it holds changes no sample.
    #[test]
    fn a_held_instrument_hears_every_retune() {
        let chords: [&[f64]; 5] = [&[60.0, 64.0, 67.0], &[60.0, 65.0, 69.0], &[62.0, 65.0, 69.0], &[59.0, 62.0, 67.0], &[60.0, 64.0, 67.0]];
        let retuned = |n: usize, channel: u32| {
            let mut core = boxed();
            for (i, chord) in chords.iter().take(n).enumerate() {
                assert!(core.event(channel, Event::retune(0.1 + 0.3 * i as f64, chord)));
            }
            channel_all(&mut core, 2.0)
        };
        let (four, five) = (retuned(4, CHORDS), retuned(5, CHORDS));
        assert!(four.iter().any(|&x| x != 0.0), "the chorale sounds");
        let first = first_difference(&four, &five).expect("the fifth retune is heard");
        assert!(first >= frame_at(1.3, RATE) as usize / QUANTUM * QUANTUM * CHANNELS);
        assert!(retuned(5, MELODY) != five, "a retune into the melody reached the chords");
        // The same chord again: nothing changes.
        let mut again = boxed();
        let mut once = boxed();
        for core in [&mut again, &mut once] {
            core.event(CHORDS, Event::retune(0.1, &[60.0, 64.0, 67.0]));
        }
        again.event(CHORDS, Event::retune(0.5, &[67.0, 60.0, 64.0]));
        assert_eq!(channel_all(&mut again, 1.5), channel_all(&mut once, 1.5));
        let host = |core: &mut Core| {
            for (i, chord) in chords.iter().enumerate() {
                core.event(CHORDS, Event::retune(0.1 + 0.15 * i as f64, chord));
            }
        };
        let whole = rendered(host, &[60_000], 60_000);
        assert!(whole.iter().any(|&b| f32::from_bits(b) != 0.0));
        for sizes in [&[1][..], &[441], &[127, 1024, 3]] {
            assert!(rendered(host, sizes, 60_000) == whole, "calls of {sizes:?} frames");
        }
    }

    /// Every channel's samples, block by block, for `seconds`.
    fn channel_all(core: &mut Core, seconds: f64) -> Vec<f32> {
        all(core, seconds)
    }

    // The retune rule: a note two chords share stays on its voice; the new
    // notes go to the voices left, in order; a voice with none rests.
    #[test]
    fn the_retune_rule_keeps_shared_notes_where_they_are() {
        use blocks::{Turn, retune};
        let nan = f64::NAN;
        assert_eq!(
            retune(&[60.0, 64.0, 67.0, nan], &[60.0, 65.0, 69.0]),
            [Turn::Keep, Turn::Speak(65.0), Turn::Speak(69.0), Turn::Rest]
        );
        assert_eq!(
            retune(&[60.0, 64.0, 67.0, nan], &[67.0, 64.0, 60.0, 72.0]),
            [Turn::Keep, Turn::Keep, Turn::Keep, Turn::Speak(72.0)]
        );
        // More notes than voices, a note twice, and a NaN.
        assert_eq!(
            retune(&[nan, nan], &[60.0, 60.0, nan, 64.0, 67.0]),
            [Turn::Speak(60.0), Turn::Speak(64.0)]
        );
        assert_eq!(retune(&[60.0, 64.0], &[]), [Turn::Rest, Turn::Rest]);
    }

    // A route carries one channel's signal into another's own block one
    // block late: the duck (a test-only demonstration, `own::DUCK`) on the
    // melody, keyed by the drums, leaves the melody as it was up to the
    // block after the drums first sound, presses it down after, and lets it
    // be again from the block holding the time the route is undone.
    #[test]
    fn a_route_carries_one_channels_signal_into_anothers_block_a_block_late() {
        // The duck's one setting a signal drives: its key.
        const KEY: usize = 0;
        let play = |core: &mut Core| {
            core.note(SINE, MELODY, 0.0, 69.0, 2.0, 0.8, 0, NO);
            for i in 0..4 {
                core.note(DRUM, DRUMS_CHANNEL, 0.2 + 0.25 * f64::from(i), 0.0, 0.1, 0.9, drum::KICK, NO);
            }
        };
        let (mut plain, mut ducked) = (boxed(), boxed());
        play(&mut plain);
        play(&mut ducked);
        let drums = Some(Tap { channel: DRUMS_CHANNEL, block: None });
        assert!(ducked.route(MELODY, own::DUCK, KEY, drums, 0.0));
        assert!(ducked.route(MELODY, own::DUCK, KEY, None, 1.2));
        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut kick = None;
        for blk in 0..(2.0 * RATE) as usize / QUANTUM {
            let x = plain.process((blk * QUANTUM) as u64);
            a.extend(x[0]);
            kick = kick.or(x[DRUMS_CHANNEL as usize].iter().position(|&v| v != 0.0).map(|i| blk * QUANTUM + i));
            b.extend(ducked.process((blk * QUANTUM) as u64)[0]);
        }
        let kick = kick.expect("the kick sounds");
        let first = first_difference(&a, &b).expect("the duck is heard");
        assert!(first >= (kick / QUANTUM + 1) * QUANTUM, "heard at {first}, the kick at {kick}");
        let energy = |v: &[f32]| v.iter().map(|x| x * x).sum::<f32>();
        let (from, to) = ((0.2 * RATE) as usize, (1.2 * RATE) as usize);
        assert!(energy(&b[from..to]) < energy(&a[from..to]) * 0.9, "pressed down");
        let undone = frame_at(1.2, RATE) as usize / QUANTUM * QUANTUM;
        assert_eq!(first_difference(&a[undone..], &b[undone..]), None, "let be once undone");
        // Not a setting, block or tap the core has.
        assert!(!ducked.route(MELODY, own::DUCK, 1, drums, 0.0));
        assert!(!ducked.route(MELODY, own::SWELL, 0, drums, 0.0));
        assert!(!ducked.route(MELODY, own::DUCK, KEY, Some(Tap { channel: 9, block: None }), 0.0));
        assert!(ducked.route(MELODY, own::DUCK, KEY, Some(Tap { channel: CHORDS, block: Some(own::NYLON_BODY) }), 3.0));
        let host = |core: &mut Core| {
            play(core);
            core.route(MELODY, own::DUCK, KEY, drums, 0.0);
        };
        let whole = rendered(host, &[60_000], 60_000);
        for sizes in [&[1][..], &[441], &[127, 1024, 3]] {
            assert!(rendered(host, sizes, 60_000) == whole, "calls of {sizes:?} frames");
        }
    }

    // How far each kind reaches: a damp and a release once, a bend and a
    // retune every time; a note hears an every-time event when it is held
    // then -- started, and not yet ended.
    #[test]
    fn each_kind_says_how_far_it_reaches() {
        use event::{BEND, CHOKE, DAMP, KINDS, ONCE, RELEASE, RETUNE, reach};
        assert_eq!(reach(DAMP), Some(Reach::Once));
        assert_eq!(reach(RELEASE), Some(Reach::Once));
        assert_eq!(reach(BEND), Some(Reach::Every));
        assert_eq!(reach(RETUNE), Some(Reach::Every));
        assert_eq!(reach(CHOKE), Some(Reach::Every));
        assert_eq!(reach(KINDS as u32), None);
        for kind in 0..KINDS as u32 {
            assert_eq!(reach(kind) == Some(Reach::Once), (kind as usize) < ONCE, "the once-kinds come first");
            assert!(event::name(kind).is_ascii());
        }
        let mut slot = Slot::new();
        (slot.busy, slot.channel, slot.begin, slot.until) = (true, 1, 1000, 2.0);
        assert!(hears_every(&slot, 1, 1000, 1.0));
        assert!(!hears_every(&slot, 1, 999, 0.02), "not started yet");
        assert!(!hears_every(&slot, 1, 100_000, 2.0), "ended");
        assert!(!hears_every(&slot, 2, 1000, 1.0), "another channel");
        slot.busy = false;
        assert!(!hears_every(&slot, 1, 1000, 1.0), "not sounding");
    }

    // Cancelling drops the events still waiting for their time, from its
    // time on, and keeps the settings: they are the loop's.
    #[test]
    fn cancel_drops_waiting_events_and_keeps_settings() {
        // The swell's depth.
        const DEPTH: usize = 1;
        let mut core = boxed();
        core.event(CHORDS, Event::retune(0.5, &[60.0]));
        core.event(CHORDS, Event::retune(1.5, &[64.0]));
        core.set(MELODY, own::SWELL, DEPTH, 0.5, 1.5);
        core.cancel_from(1.0);
        assert_eq!(core.waiting_due, 2);
        core.cancel();
        assert_eq!(core.waiting_due, 1);
        core.clear();
        assert_eq!(core.waiting_due, 0);
        // A place for every pending thing, and no more.
        for i in 0..PENDING {
            assert!(core.event(CHORDS, Event::new(event::BEND, 1.0 + i as f64 * 0.001, 0.0)));
        }
        assert!(!core.event(CHORDS, Event::new(event::BEND, 9.0, 0.0)));
    }
}
