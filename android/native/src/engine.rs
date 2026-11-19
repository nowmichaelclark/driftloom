//! The engine: a core, the player on it, and the queues its commands come
//! by and its reports go back by (queue items 107b and 107c).
//!
//! The app's thread sends commands (`send`); the audio thread renders
//! (`fill`), taking the commands waiting at the start of each callback, and
//! publishes what it decided: the player's events (with the layers' grid of
//! a pass that begins), the answers to a ping or a probe, and a snapshot of
//! where the player is and what the core counts. The core and the player
//! are the renderer's alone: nothing else ever touches them while a stream
//! runs, so `fill` takes no lock, and it allocates nothing
//! (`fill_allocates_nothing`, below). This is the worklet's way
//! (`js/worklet.js`: messages in, events out) on a native audio thread; the
//! page's messages become these commands, and these reports its messages
//! back, in `bridge.rs`.
//!
//! The same `fill` renders offline (`render_line`), so the hash the tests
//! hold to the web's is made by the path the speaker hears.

use std::cell::UnsafeCell;
use std::fmt::Write as _;
use std::ptr;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU32, AtomicU64, Ordering};
use std::sync::{Mutex, PoisonError};
use std::time::Instant;

use dlcore::Core;
use dlcore::generator::Layer;
use dlcore::player::about::GRID_MAX;
use dlcore::player::{CROSSFADE, Fades, Handover, Jump, Kind, Moment, Player, Request, State, Track};

use crate::ring::Ring;

/// The frames the core renders in one call, at most: a callback asking for
/// more is rendered in several. The core gives the same samples whatever
/// the split (`core/README.md`, rendering ahead), so this only sizes the
/// engine's two scratch planes.
pub const CHUNK: usize = 1024;

/// The commands that may wait at once. The page sends a burst of twenty or
/// so when it builds a player (a load with the controls that hold across
/// loops); a full queue refuses (`Counts::refused`) rather than block.
const COMMANDS: usize = 128;

/// The reports that may wait for the app's thread at once: events (a step's
/// every sixteenth of a bar, posted every 50 ms), pongs and probes.
const OUT: usize = 1024;

/// The grids that may wait at once (a pass that begins brings one), and the
/// layers each has.
const GRIDS: usize = 16;
const LAYERS: usize = 5;

/// The longest name kept for the app to read (the core's are short words).
const NAME: usize = 48;

/// Room for a probe's description (`Player::describe`, some 600 bytes):
/// `dl_play_about_at`'s in the web's module.
pub const ABOUT_MAX: usize = 4096;

/// The probes that may wait for the one slot they are answered in.
const PROBES: usize = 4;

/// How busy the audio thread is (queue item 87b, `js/worklet.js`'s
/// `BUSY_WINDOW`): the render's time over each quarter-second of audio, as a
/// share of it.
const BUSY_WINDOW: f64 = 0.25;

/// What a command does to the loop that follows the track's end
/// (`js/worklet.js`'s `arm`): drops the one waiting, so the core's own loop
/// follows (`null`); leaves it (an arm whose track is no kind there is, which
/// `dl_play_change_track` refuses whole); or arms one.
#[allow(clippy::large_enum_variant, reason = "inside a fixed slot of the queue; boxing would allocate")]
#[derive(Clone, Copy)]
pub enum ArmEnd {
    Drop,
    Keep,
    Set(Arm),
}

/// The loop that follows when the track ends: a change waiting for the
/// track's end, with the track that loop plays.
#[derive(Clone, Copy)]
pub struct Arm {
    pub seed: u32,
    pub request: Request,
    /// The old loop's fall to nothing, in seconds; negative, it rings out.
    pub fade: f64,
    pub track: Option<(Track, Fades)>,
}

/// What the app asks of the player, as the worklet's messages do. A load
/// carries its whole request (about 1.7 KB), so every slot of the queue is
/// that size: boxing it would allocate, which the queue must never do.
#[allow(clippy::large_enum_variant, reason = "a fixed slot of the queue; boxing would allocate")]
#[derive(Clone, Copy)]
pub enum Command {
    /// A loop from its seed, with what it asks for (`Player::load`): made a
    /// stage a block over the next blocks, then played.
    Load { seed: u32, request: Request },
    /// Make what is left of the loop asked for, now (`Player::ready`): the
    /// offline render's, never a stream's (it would spike one callback).
    Ready,
    /// Go on after a pause (`Player::resume`).
    Play,
    /// Stop deciding steps; what was handed over rings out (`Player::stop`).
    Pause,
    /// A loop of the core's own (`Player::next`), crossfading as the app's
    /// Next does.
    Next,
    /// The listener's volume, 0 to 1 (`Player::set_volume`).
    Volume(f64),
    /// How far each pass wanders (`Player::set_wander`).
    Wander(f64),
    /// The seed the core's own loops are drawn from (`Player::set_draws`).
    Draws(u32),
    /// Forget the busiest callback so far (the debug screen's reset).
    ResetBusiest,

    // The page's (session 27): js/worklet.js's messages, one each, in its
    // order there.
    /// A player for a page (the worklet's constructor): the core's mix built
    /// afresh at `full` or lite's, rendering `layout` lanes, and the player
    /// started on it, every voice let go. The clock goes on, as a browser's
    /// context's does across its nodes.
    Init { full: bool, layout: u32 },
    /// The page's player let go (`dispose`): every voice let go and nothing
    /// decided; reports made before it are not the next page's.
    Dispose,
    /// The Mono switch (`layout`): `Core::set_layout`, faded.
    Layout(u32),
    /// Drop the notes handed over that have not started (`cancel`), or only
    /// those asked for at a time or later.
    Cancel(Option<f64>),
    /// One of the mix's parameters the page still sets (`param`).
    Param { id: u32, op: u32, value: f64, time: f64, tau: f64 },
    /// `playLoad`: a loop afresh, its first step at `at` on the core's clock
    /// (0, as soon as made), `resumed` a loop moved from another player.
    LoadAt { seed: u32, request: Request, at: f64, resumed: bool },
    /// `playChange`: change to a loop at a moment.
    Change { seed: u32, request: Request, moment: Moment, handover: Handover },
    /// `playRevise`: the loop playing made again in its place (a re-roll).
    Revise { seed: u32, request: Request },
    /// `playTrack`: how long the track plays (none for a kind there is no
    /// such number for), and the loop that follows, in one command so that
    /// no block falls between them.
    Track { track: Option<(Track, Fades)>, arm: ArmEnd },
    /// `playArm`: the loop that follows the track's end, or none.
    Arm(ArmEnd),
    /// `playPicks`: the New loops panel's request, held for every loop the
    /// core makes itself, numbered `tag` (refused parts already taken out).
    Picks { request: Request, tag: u32 },
    /// `playNext`: a loop of the core's own at a moment (none for a moment
    /// there is no such number for).
    NextAt { moment: Option<Moment>, handover: Handover },
    /// `playJump`: somewhere else in the loop playing (none: no such place).
    Jump(Option<Jump>),
    /// `playTone`: warmth, space, wobble.
    Tone([f64; 3]),
    /// `probe`: what the player says of the loop now, answered by number.
    Probe(f64),
    /// `playStop`, `playResume`.
    Stop,
    Resume(f64),
    /// `playWander`: the amount and its salt.
    WanderSalted { amount: f64, salt: u32 },
    /// `playMute`.
    Mute { layer: Layer, muted: bool },
    /// `playTempo`.
    Tempo(f64),
    /// `ping`: every command before it has been taken; answered by number.
    Ping(f64),
}

/// The core's late, left-out and dropped notes, by cause.
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Tally {
    pub late: u32,
    pub missed: u32,
    pub dropped: u32,
    pub by: [u32; 3],
}

impl Tally {
    fn of(core: &Core) -> Tally {
        Tally { late: core.late(), missed: core.missed(), dropped: core.dropped(), by: [core.dropped_by(0), core.dropped_by(1), core.dropped_by(2)] }
    }

    /// The counts since `base` (the page's player began there).
    fn since(self, base: Tally) -> Tally {
        let d = |a: u32, b: u32| a.saturating_sub(b);
        Tally {
            late: d(self.late, base.late),
            missed: d(self.missed, base.missed),
            dropped: d(self.dropped, base.dropped),
            by: [d(self.by[0], base.by[0]), d(self.by[1], base.by[1]), d(self.by[2], base.by[2])],
        }
    }
}

/// Where the player is: its state (0 idle, 1 making a loop, 2 its first
/// pass, 3 playing), the step it decides next, and that step's time.
pub type Position = [f64; 3];

/// A report from the audio thread for the app's.
#[derive(Clone, Copy, Debug)]
pub enum Out {
    /// One of the player's events, as `dl_play_events` gives it, with the
    /// grid slot read with it (a pass, a section, a loop, a re-roll or a
    /// jump begins; `Engine::grid`).
    Event { epoch: u32, time: f64, kind: u8, number: u64, step: u64, grid: Option<u8> },
    /// The answer to a ping.
    Pong { epoch: u32, id: f64, tally: Tally, position: Position, busy: u32, reduction: [f32; 2], deepest: [f32; 2] },
    /// The answer to a probe: its numbers and words in the probe slot
    /// (`Engine::probe`).
    Probe { epoch: u32, id: f64 },
}

/// A probe's answer, as the worklet posts it (`probe`).
#[derive(Clone, Copy)]
pub struct ProbeAnswer {
    pub about: [u8; ABOUT_MAX],
    pub len: usize,
    pub lanes: u32,
    pub position: Position,
    pub track_end: f64,
    /// A time's track: the second it ends on, on the core's clock, or -1
    /// (session 30c; the worklet's `trackAt`).
    pub track_at: f64,
    pub next_change: f64,
    pub lost: u32,
    pub picks_tag: u32,
    pub tally: Tally,
}

/// The layers' grids of one pass: each layer's velocities and their count.
#[derive(Clone, Copy)]
struct GridSlot {
    lens: [u16; LAYERS],
    steps: [[f64; GRID_MAX]; LAYERS],
}

/// The snapshot's words (`Status`), each a double.
pub mod at {
    /// The core's clock: the frames handed to the device, in seconds.
    pub const TIME: usize = 0;
    pub const STATE: usize = 1;
    pub const STEP: usize = 2;
    pub const STEP_TIME: usize = 3;
    /// The step the track ends at (-1 while it plays on).
    pub const TRACK_END: usize = 4;
    pub const LOST: usize = 5;
    /// 1 with a place (then its five numbers), 0 with none.
    pub const PLACE: usize = 6;
    pub const BAR: usize = 7;
    pub const BARS: usize = 8;
    pub const PLAYED: usize = 9;
    pub const LEFT: usize = 10;
    pub const SECTION: usize = 11;
    pub const LANES: usize = 12;
    pub const LATE: usize = 13;
    pub const MISSED: usize = 14;
    pub const DROPPED: usize = 15;
    pub const DROPPED_BY: usize = 16;
    pub const PICKS_TAG: usize = 19;
    pub const NEXT_CHANGE: usize = 20;
    pub const EPOCH: usize = 21;
    /// What rendering has cost since the page's player began, mono then
    /// stereo: milliseconds spent, and seconds of audio made.
    pub const COST_MS: usize = 22;
    pub const COST_SECONDS: usize = 24;
    /// Frames played with anything sounding in them, since the start.
    pub const SOUNDING: usize = 26;
    pub const LAYOUT: usize = 27;
    pub const WORDS: usize = 28;
}

/// The audio thread's snapshot of where the player is and what the core
/// counts, written whole after every callback: a sequence lock over atomic
/// words, so the reader never waits and never sees half of one.
pub struct Status {
    version: AtomicU32,
    words: [AtomicU64; at::WORDS],
}

impl Status {
    fn new() -> Self {
        Status { version: AtomicU32::new(0), words: [const { AtomicU64::new(0) }; at::WORDS] }
    }

    /// The renderer's.
    fn write(&self, words: &[f64; at::WORDS]) {
        let v = self.version.load(Ordering::Relaxed);
        self.version.store(v.wrapping_add(1), Ordering::Relaxed);
        std::sync::atomic::fence(Ordering::Release);
        for (cell, w) in self.words.iter().zip(words) {
            cell.store(w.to_bits(), Ordering::Relaxed);
        }
        self.version.store(v.wrapping_add(2), Ordering::Release);
    }

    /// Anyone's: the words as last written whole.
    #[must_use]
    pub fn read(&self) -> [f64; at::WORDS] {
        loop {
            let v = self.version.load(Ordering::Acquire);
            if v % 2 == 1 {
                std::hint::spin_loop();
                continue;
            }
            let mut out = [0.0; at::WORDS];
            for (o, cell) in out.iter_mut().zip(&self.words) {
                *o = f64::from_bits(cell.load(Ordering::Relaxed));
            }
            std::sync::atomic::fence(Ordering::Acquire);
            if self.version.load(Ordering::Relaxed) == v {
                return out;
            }
        }
    }
}

/// What the audio thread tells the app: written by the renderer only, read
/// by anyone, never waited on.
#[derive(Default)]
pub struct Counts {
    /// The core's notes taken late, left out, and dropped, since it started
    /// (`Core::late`, `missed`, `dropped`): 0 unless something is wrong.
    pub late: AtomicU32,
    pub missed: AtomicU32,
    pub dropped: AtomicU32,
    /// Callbacks rendered, and the frames they asked for.
    pub callbacks: AtomicU64,
    pub frames: AtomicU64,
    /// The busiest callback since the start (or the last reset): its share
    /// of its block's deadline in millionths, its time in nanoseconds, its
    /// frames. A block is the frames it asked for, or the stream's burst if
    /// that is more (`Engine::set_block`): a device that tops its buffer up
    /// with a few frames between bursts gives that callback a burst's time
    /// all the same, and a share of a few frames' time would be its fixed
    /// cost, not its load (the CI emulator asks for 8 or 56 at times).
    pub busiest_ppm: AtomicU64,
    pub busiest_ns: AtomicU64,
    pub busiest_frames: AtomicU64,
    /// Every callback's time and its frames' own time, summed, for the mean
    /// share: the share of real time the audio thread spends rendering.
    pub spent_ns: AtomicU64,
    pub deadline_ns: AtomicU64,
    /// The stream's burst, in frames: the least a block's deadline is
    /// counted as (0 offline).
    pub block: AtomicU64,
    /// The player's state (0 idle, 1 loading, 2 wandering, 3 playing, as
    /// `dl_player_state`), and the seed of the loop heard last (its `Loop`
    /// event).
    pub state: AtomicU32,
    pub seed: AtomicU32,
    /// Commands refused because the queue was full (the app's side).
    pub refused: AtomicU32,
    /// Callbacks that found another renderer at work and played silence:
    /// never, unless two streams were somehow open at once.
    pub collisions: AtomicU32,
    /// Since the app's thread last took them (`Engine::take_busy`): the
    /// busiest quarter-second's share of its time (a double's bits: shares
    /// are never negative, so the bits order as the numbers), the
    /// quarter-seconds that took longer than the audio they made, and how
    /// many quarter-seconds.
    pub busy_max: AtomicU64,
    pub busy_behind: AtomicU32,
    pub busy_windows: AtomicU32,
    /// The loudest callback's level (RMS, a double's bits) since the app's
    /// thread last took it: "is anything sounding", for the checks.
    pub level: AtomicU64,
    /// Reports with no room for them in the queue back (never, unless the
    /// app's thread stopped reading).
    pub out_lost: AtomicU32,
}

/// The loop's name for the app: a sequence lock, so the reader never waits
/// on the audio thread and never sees half a name.
struct NameCell {
    version: AtomicU32,
    len: AtomicU8,
    bytes: [AtomicU8; NAME],
}

impl NameCell {
    fn new() -> Self {
        NameCell { version: AtomicU32::new(0), len: AtomicU8::new(0), bytes: [const { AtomicU8::new(0) }; NAME] }
    }

    /// The renderer's: a new name, odd version while it is written.
    fn write(&self, name: &[u8]) {
        let v = self.version.load(Ordering::Relaxed);
        self.version.store(v.wrapping_add(1), Ordering::Relaxed);
        std::sync::atomic::fence(Ordering::Release);
        let n = name.len().min(NAME);
        for (cell, b) in self.bytes.iter().zip(&name[..n]) {
            cell.store(*b, Ordering::Relaxed);
        }
        #[allow(clippy::cast_possible_truncation, reason = "n is at most NAME, 48")]
        self.len.store(n as u8, Ordering::Relaxed);
        self.version.store(v.wrapping_add(2), Ordering::Release);
    }

    /// Anyone's: the name as last written whole.
    fn read(&self) -> String {
        loop {
            let v = self.version.load(Ordering::Acquire);
            if v % 2 == 1 {
                std::hint::spin_loop();
                continue;
            }
            let n = usize::from(self.len.load(Ordering::Relaxed)).min(NAME);
            let bytes: Vec<u8> = self.bytes[..n].iter().map(|b| b.load(Ordering::Relaxed)).collect();
            std::sync::atomic::fence(Ordering::Acquire);
            if self.version.load(Ordering::Relaxed) == v {
                return String::from_utf8_lossy(&bytes).into_owned();
            }
        }
    }
}

/// A writer into a fixed buffer that fails rather than cut short (the web
/// module's `Fill`).
struct Fill<'a> {
    buf: &'a mut [u8],
    len: usize,
}

impl std::fmt::Write for Fill<'_> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let end = self.len + s.len();
        let slot = self.buf.get_mut(self.len..end).ok_or(std::fmt::Error)?;
        slot.copy_from_slice(s.as_bytes());
        self.len = end;
        Ok(())
    }
}

/// The renderer's part: the core, the player, the planes the core renders
/// into before they are interleaved, and what it keeps between callbacks.
struct Inner {
    core: Core,
    player: Player,
    planes: [[f32; CHUNK]; 2],
    /// The counts when the page's player began (`Init`): the page hears the
    /// counts since, as a new worklet's core starts from nothing.
    base: Tally,
    /// The reports' number: one more at each `Dispose`.
    epoch: u32,
    /// What rendering has cost since the page's player began (mono, stereo).
    cost_ms: [f64; 2],
    cost_seconds: [f64; 2],
    /// The quarter-second being counted: its milliseconds and frames.
    window_ms: f64,
    window_frames: f64,
    /// Frames with anything sounding, since the start.
    sounding: u64,
    /// Grids written into their slots, all told.
    grids_made: u64,
    /// Probes waiting for the probe slot.
    probes: [f64; PROBES],
    probes_waiting: usize,
}

/// A core as `Core::new` makes one and a player as `Player::new` does, built
/// when the library is: a new engine copies them into its own memory, so
/// making one never builds megabytes on a stack (`core/src/capi.rs` does the
/// same). Both are all zero bytes, so they cost the library's file nothing.
struct Template<T>(T);

// SAFETY: never written: only ever copied from.
unsafe impl<T> Sync for Template<T> {}

static CORE: Template<Core> = Template(Core::new());
static PLAYER: Template<Player> = Template(Player::new());

/// A core and its player, the queues to them and back, and what they report.
pub struct Engine {
    inner: UnsafeCell<Inner>,
    /// Set while a thread renders: `fill`'s guard that the core and player
    /// are only ever its own.
    rendering: AtomicBool,
    commands: Ring<Command, COMMANDS>,
    /// The app's side of the queue: its threads take turns writing. The
    /// audio thread never takes it.
    writer: Mutex<()>,
    /// The reports back: the audio thread writes them, one reader takes
    /// them (`reader`).
    out: Ring<Out, OUT>,
    reader: Mutex<()>,
    /// The grids' slots: written by the renderer before the event that
    /// names one is published, read by the reader after it takes that
    /// event; `grids_done` counts the slots read, so the renderer reuses
    /// only those.
    grids: UnsafeCell<[GridSlot; GRIDS]>,
    grids_done: AtomicU64,
    /// The probe's answer: the renderer writes it when `probe_full` is
    /// false and sets it; the reader reads it and clears it.
    probe: UnsafeCell<ProbeAnswer>,
    probe_full: AtomicBool,
    /// What the audio thread reports.
    pub counts: Counts,
    pub status: Status,
    name: NameCell,
    rate: f64,
    layout: usize,
}

// SAFETY: `inner` is reached only inside `fill`, behind `rendering` (one
// thread at a time), or through `&mut self`; `grids` and `probe` are handed
// between the renderer and the one reader as their comments say (a slot is
// written before a Release that the reader Acquires, and not written again
// until the reader says it is done); the rest is atomics, the lock-free
// queues (one writer and one reader each, the app's side under its mutex)
// and plain numbers set before the engine is shared.
unsafe impl Sync for Engine {}

impl Engine {
    /// A new engine: a core at `rate` Hz rendering `layout` channels (1 mono,
    /// 2 stereo), the full mix or lite's (`full`), and its player, idle.
    ///
    /// # Errors
    ///
    /// The core's refusal of the rate or layout, in plain words.
    pub fn new(rate: f64, layout: u32, full: bool) -> Result<Box<Engine>, &'static str> {
        let mut memory = Box::<Engine>::new_uninit();
        let p = memory.as_mut_ptr();
        // SAFETY: each field is written once before the whole is assumed
        // made; the templates are plain data (no pointers but to statics),
        // so their copies are a core and a player of their own. `UnsafeCell`
        // has its contents' layout; the grids and the probe's answer are
        // numbers and bytes, for which all zero bytes are a value.
        let mut engine = unsafe {
            let inner = (&raw mut (*p).inner).cast::<Inner>();
            ptr::copy_nonoverlapping(&raw const CORE.0, &raw mut (*inner).core, 1);
            ptr::copy_nonoverlapping(&raw const PLAYER.0, &raw mut (*inner).player, 1);
            (&raw mut (*inner).planes).write([[0.0; CHUNK]; 2]);
            (&raw mut (*inner).base).write(Tally::default());
            (&raw mut (*inner).epoch).write(0);
            (&raw mut (*inner).cost_ms).write([0.0; 2]);
            (&raw mut (*inner).cost_seconds).write([0.0; 2]);
            (&raw mut (*inner).window_ms).write(0.0);
            (&raw mut (*inner).window_frames).write(0.0);
            (&raw mut (*inner).sounding).write(0);
            (&raw mut (*inner).grids_made).write(0);
            (&raw mut (*inner).probes).write([0.0; PROBES]);
            (&raw mut (*inner).probes_waiting).write(0);
            (&raw mut (*p).rendering).write(AtomicBool::new(false));
            (&raw mut (*p).commands).write(Ring::new());
            (&raw mut (*p).writer).write(Mutex::new(()));
            (&raw mut (*p).out).write(Ring::new());
            (&raw mut (*p).reader).write(Mutex::new(()));
            ptr::write_bytes((&raw mut (*p).grids).cast::<u8>(), 0, size_of::<[GridSlot; GRIDS]>());
            (&raw mut (*p).grids_done).write(AtomicU64::new(0));
            ptr::write_bytes((&raw mut (*p).probe).cast::<u8>(), 0, size_of::<ProbeAnswer>());
            (&raw mut (*p).probe_full).write(AtomicBool::new(false));
            (&raw mut (*p).counts).write(Counts::default());
            (&raw mut (*p).status).write(Status::new());
            (&raw mut (*p).name).write(NameCell::new());
            (&raw mut (*p).rate).write(rate);
            (&raw mut (*p).layout).write(if layout == 2 { 2 } else { 1 });
            memory.assume_init()
        };
        let Inner { core, player, .. } = engine.inner.get_mut();
        player.start(core, rate, layout, full).map_err(|_| "the core can't play at that rate or layout")?;
        Ok(engine)
    }

    /// The rate the core renders at.
    pub fn rate(&self) -> f64 {
        self.rate
    }

    /// Queue a command for the audio thread's next callback. False if the
    /// queue is full (counted in `Counts::refused`): the command is dropped.
    pub fn send(&self, command: &Command) -> bool {
        let _turn = self.writer.lock().unwrap_or_else(PoisonError::into_inner);
        // SAFETY: the one writer: every push is made holding `writer`.
        let sent = unsafe { self.commands.push(*command) };
        if !sent {
            self.counts.refused.fetch_add(1, Ordering::Relaxed);
        }
        sent
    }

    /// Queue several commands at once, in order, so that the audio thread
    /// takes them in one callback (unless one comes between the pushes: a
    /// command never splits, and every command here is whole on its own).
    /// False if any was refused.
    #[cfg(test)]
    pub fn send_all(&self, commands: &[Command]) -> bool {
        let _turn = self.writer.lock().unwrap_or_else(PoisonError::into_inner);
        let mut all = true;
        for c in commands {
            // SAFETY: the one writer, as in `send`.
            if !unsafe { self.commands.push(*c) } {
                self.counts.refused.fetch_add(1, Ordering::Relaxed);
                all = false;
            }
        }
        all
    }

    /// The name of the loop heard last.
    pub fn name(&self) -> String {
        self.name.read()
    }

    /// Take the reports waiting, oldest first, with each grid and probe
    /// answer they name: the app's thread's call (the one reader).
    pub fn take(&self, mut each: impl FnMut(Out, Option<&GridView>, Option<&ProbeAnswer>)) {
        let _turn = self.reader.lock().unwrap_or_else(PoisonError::into_inner);
        // SAFETY: the one reader: every pop is made holding `reader`.
        while let Some(out) = unsafe { self.out.pop() } {
            match out {
                Out::Event { grid: Some(slot), .. } => {
                    // SAFETY: the renderer wrote this slot before it published
                    // the event (Release; the pop Acquired it) and leaves it
                    // until `grids_done` passes it, below.
                    let g = unsafe { &(*self.grids.get())[usize::from(slot) % GRIDS] };
                    each(out, Some(&GridView(g)), None);
                    self.grids_done.fetch_add(1, Ordering::Release);
                }
                Out::Probe { .. } => {
                    if self.probe_full.load(Ordering::Acquire) {
                        // SAFETY: the renderer wrote it before setting
                        // `probe_full` (Release, Acquired above) and leaves it
                        // until that is cleared.
                        let a = unsafe { &*self.probe.get() };
                        each(out, None, Some(a));
                        self.probe_full.store(false, Ordering::Release);
                    }
                }
                _ => each(out, None, None),
            }
        }
    }

    /// The busy reports since the last call (the app's thread's): the
    /// busiest quarter-second's share, those behind, and how many.
    pub fn take_busy(&self) -> (f64, u32, u32) {
        let c = &self.counts;
        (f64::from_bits(c.busy_max.swap(0, Ordering::Relaxed)), c.busy_behind.swap(0, Ordering::Relaxed), c.busy_windows.swap(0, Ordering::Relaxed))
    }

    /// The loudest callback's level since the last call.
    pub fn take_level(&self) -> f64 {
        f64::from_bits(self.counts.level.swap(0, Ordering::Relaxed))
    }

    /// Fill `out`, interleaved, `channels` samples a frame, with the next
    /// frames: the audio thread's call, once a callback. It takes the
    /// commands waiting, renders, reads the player's events and publishes
    /// the counts. No lock, no allocation, nothing that can wait.
    pub fn fill(&self, out: &mut [f32], channels: usize) {
        if channels == 0 {
            return;
        }
        if self.rendering.swap(true, Ordering::Acquire) {
            out.fill(0.0);
            self.counts.collisions.fetch_add(1, Ordering::Relaxed);
            return;
        }
        let began = Instant::now();
        // SAFETY: `rendering` was false and is now ours: no other thread is
        // inside `fill`, and nothing else reaches `inner` through `&self`.
        let inner = unsafe { &mut *self.inner.get() };

        // SAFETY: the one reader: every pop is made inside `fill`, behind
        // `rendering`.
        while let Some(command) = unsafe { self.commands.pop() } {
            self.apply(inner, &command);
        }
        self.answer_probe(inner);

        let frames = out.len() / channels;
        let mut done = 0;
        while done < frames {
            let n = (frames - done).min(CHUNK);
            let layout = if inner.core.layout() == 2 { 2 } else { 1 };
            let Inner { core, player, planes, .. } = &mut *inner;
            let [left, right] = planes;
            let rendered = {
                let mut both: [&mut [f32]; 2] = [&mut left[..n], &mut right[..n]];
                player.render(core, &mut both[..layout]).is_ok()
            };
            let into = &mut out[done * channels..(done + n) * channels];
            if rendered {
                interleave(into, channels, &left[..n], &right[..n], layout);
            } else {
                into.fill(0.0);
            }
            self.take_events(inner);
            done += n;
        }
        // The rest of a buffer not a whole number of frames, if a host gave one.
        out[frames * channels..].fill(0.0);

        // How loud it was: the callback's RMS, and whether anything sounded.
        let mut sum = 0.0f64;
        let mut peak = 0.0f32;
        for &x in &*out {
            sum += f64::from(x) * f64::from(x);
            peak = peak.max(x.abs());
        }
        #[allow(clippy::cast_precision_loss, reason = "a callback's samples, far under 2^53")]
        let rms = if out.is_empty() { 0.0 } else { (sum / out.len() as f64).sqrt() };
        self.counts.level.fetch_max(rms.to_bits(), Ordering::Relaxed);
        if peak > 1e-5 {
            inner.sounding += frames as u64;
        }

        let c = &self.counts;
        c.late.store(inner.core.late(), Ordering::Relaxed);
        c.missed.store(inner.core.missed(), Ordering::Relaxed);
        c.dropped.store(inner.core.dropped(), Ordering::Relaxed);
        c.state.store(state_number(inner.player.state()), Ordering::Relaxed);
        c.callbacks.fetch_add(1, Ordering::Relaxed);
        c.frames.fetch_add(frames as u64, Ordering::Relaxed);

        // The callback's time against its deadline: the frames it was asked
        // for, at the rate, are how long the device takes to play them (the
        // mean's); a block is at least a burst (the busiest's, `Counts`).
        let spent = u64::try_from(began.elapsed().as_nanos()).unwrap_or(u64::MAX);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss, reason = "a callback's deadline in ns, far inside u64")]
        let time = |frames: u64| ((frames as f64) * 1e9 / self.rate) as u64;
        let deadline = time(frames as u64);
        let block = time((frames as u64).max(c.block.load(Ordering::Relaxed)));
        c.spent_ns.fetch_add(spent, Ordering::Relaxed);
        c.deadline_ns.fetch_add(deadline, Ordering::Relaxed);
        if let Some(ppm) = spent.saturating_mul(1_000_000).checked_div(block)
            && ppm > c.busiest_ppm.load(Ordering::Relaxed)
        {
            c.busiest_ppm.store(ppm, Ordering::Relaxed);
            c.busiest_ns.store(spent, Ordering::Relaxed);
            c.busiest_frames.store(frames as u64, Ordering::Relaxed);
        }

        // The worklet's cost and busy reports (`costReport`): this
        // callback's time, counted with the lanes it had.
        #[allow(clippy::cast_precision_loss, reason = "a callback's ns and frames, far under 2^53")]
        {
            let ms = spent as f64 / 1e6;
            let lanes = usize::from(inner.core.lanes() == 2);
            inner.cost_ms[lanes] += ms;
            inner.cost_seconds[lanes] += frames as f64 / self.rate;
            inner.window_ms += ms;
            inner.window_frames += frames as f64;
            if inner.window_frames >= BUSY_WINDOW * self.rate {
                let share = inner.window_ms / (inner.window_frames / self.rate * 1000.0);
                c.busy_max.fetch_max(share.max(0.0).to_bits(), Ordering::Relaxed);
                if share >= 1.0 {
                    c.busy_behind.fetch_add(1, Ordering::Relaxed);
                }
                c.busy_windows.fetch_add(1, Ordering::Relaxed);
                inner.window_ms = 0.0;
                inner.window_frames = 0.0;
            }
        }
        self.publish(inner);

        self.rendering.store(false, Ordering::Release);
    }

    /// The player's events from the frames just rendered, onto the queue
    /// back; with one that begins a pass (a pass, a section, a new loop, a
    /// re-roll, a jump), the layers' grid of that pass, read now
    /// (`Player::grid` is the pass being decided), as the worklet reads it.
    fn take_events(&self, inner: &mut Inner) {
        while let Some(event) = inner.player.event() {
            if event.kind == Kind::Loop {
                #[allow(clippy::cast_possible_truncation, reason = "a Loop event's number is its seed, a u32")]
                self.counts.seed.store(event.number as u32, Ordering::Relaxed);
                self.name.write(inner.player.generator().spec().name.as_str().as_bytes());
            }
            let begins = matches!(event.kind, Kind::Pass | Kind::Section | Kind::Loop | Kind::Revised | Kind::Jump);
            let mut grid = None;
            if begins && inner.grids_made.wrapping_sub(self.grids_done.load(Ordering::Acquire)) < GRIDS as u64 {
                #[allow(clippy::cast_possible_truncation, reason = "a slot under GRIDS, 16")]
                let slot = (inner.grids_made % GRIDS as u64) as usize;
                // SAFETY: the reader is done with this slot (`grids_done`
                // has passed its last use) and will not look at it again until
                // the event naming it is published, below.
                let g = unsafe { &mut (*self.grids.get())[slot] };
                for (l, layer) in dlcore::generator::LAYERS.iter().enumerate() {
                    let n = inner.player.grid(*layer, &mut g.steps[l]);
                    g.lens[l] = u16::try_from(n).unwrap_or(0);
                }
                inner.grids_made += 1;
                grid = u8::try_from(slot).ok();
            }
            let out = Out::Event { epoch: inner.epoch, time: event.time, kind: event.kind as u8, number: event.number, step: event.step, grid };
            // SAFETY: the one writer of the queue back is the renderer.
            if !unsafe { self.out.push(out) } {
                self.counts.out_lost.fetch_add(1, Ordering::Relaxed);
                if grid.is_some() {
                    // Never read: the slot is free again.
                    self.grids_done.fetch_add(1, Ordering::Release);
                }
            }
        }
    }

    /// A probe waiting, answered if the slot is free.
    fn answer_probe(&self, inner: &mut Inner) {
        if inner.probes_waiting == 0 || self.probe_full.load(Ordering::Acquire) {
            return;
        }
        let id = inner.probes[0];
        inner.probes.copy_within(1.., 0);
        inner.probes_waiting -= 1;
        // SAFETY: `probe_full` is false: the reader is done with the slot,
        // and won't read it until it is set again, below.
        let a = unsafe { &mut *self.probe.get() };
        let Inner { core, player, .. } = &mut *inner;
        let mut w = Fill { buf: &mut a.about, len: 0 };
        let described = player.describe(&mut w).is_ok();
        a.len = if described { w.len } else { 0 };
        a.lanes = core.lanes();
        a.position = position(player);
        #[allow(clippy::cast_precision_loss, reason = "a step, far below 2^53")]
        {
            a.track_end = player.track_end().map_or(-1.0, |s| s as f64);
        }
        a.track_at = player.track_at().unwrap_or(-1.0);
        a.next_change = player.next_change().unwrap_or(-1.0);
        a.lost = player.events_lost();
        a.picks_tag = player.picks().1;
        a.tally = Tally::of(core).since(inner.base);
        self.probe_full.store(true, Ordering::Release);
        // SAFETY: the one writer of the queue back.
        if !unsafe { self.out.push(Out::Probe { epoch: inner.epoch, id }) } {
            self.counts.out_lost.fetch_add(1, Ordering::Relaxed);
            self.probe_full.store(false, Ordering::Release);
        }
    }

    /// The snapshot, after every callback.
    #[allow(clippy::cast_precision_loss, reason = "frames, steps and counts, far below 2^53")]
    fn publish(&self, inner: &Inner) {
        let Inner { core, player, .. } = inner;
        let mut w = [0.0; at::WORDS];
        w[at::TIME] = core.served() as f64 / self.rate;
        let [state, step, step_time] = position(player);
        w[at::STATE] = state;
        w[at::STEP] = step;
        w[at::STEP_TIME] = step_time;
        w[at::TRACK_END] = player.track_end().map_or(-1.0, |s| s as f64);
        w[at::LOST] = f64::from(player.events_lost());
        if let Some(p) = player.place(core) {
            w[at::PLACE] = 1.0;
            w[at::BAR] = p.bar;
            w[at::BARS] = p.bars.unwrap_or(-1.0);
            w[at::PLAYED] = p.played;
            w[at::LEFT] = p.left.unwrap_or(-1.0);
            w[at::SECTION] = p.section as f64;
        }
        w[at::LANES] = f64::from(core.lanes());
        let t = Tally::of(core).since(inner.base);
        w[at::LATE] = f64::from(t.late);
        w[at::MISSED] = f64::from(t.missed);
        w[at::DROPPED] = f64::from(t.dropped);
        for (i, by) in t.by.iter().enumerate() {
            w[at::DROPPED_BY + i] = f64::from(*by);
        }
        w[at::PICKS_TAG] = f64::from(player.picks().1);
        w[at::NEXT_CHANGE] = player.next_change().unwrap_or(-1.0);
        w[at::EPOCH] = f64::from(inner.epoch);
        w[at::COST_MS] = inner.cost_ms[0];
        w[at::COST_MS + 1] = inner.cost_ms[1];
        w[at::COST_SECONDS] = inner.cost_seconds[0];
        w[at::COST_SECONDS + 1] = inner.cost_seconds[1];
        w[at::SOUNDING] = inner.sounding as f64;
        w[at::LAYOUT] = f64::from(core.layout());
        self.status.write(&w);
    }

    /// One command, on the audio thread.
    #[allow(clippy::too_many_lines, reason = "one arm a command, as the worklet's `receive`")]
    fn apply(&self, inner: &mut Inner, command: &Command) {
        let Inner { core, player, base, epoch, cost_ms, cost_seconds, window_ms, window_frames, probes, probes_waiting, .. } = &mut *inner;
        match *command {
            Command::Load { seed, ref request } => player.load(seed, request),
            Command::Ready => player.ready(core),
            Command::Play => {
                if player.state() == State::Idle {
                    player.resume(0.0);
                }
            }
            Command::Pause | Command::Stop => player.stop(),
            Command::Next => {
                player.next(Moment::Now, Handover::Crossfade(CROSSFADE));
            }
            Command::Volume(v) => player.set_volume(core, v),
            Command::Wander(a) => player.set_wander(a),
            Command::Draws(seed) => player.set_draws(seed),
            Command::ResetBusiest => {
                self.counts.busiest_ppm.store(0, Ordering::Relaxed);
                self.counts.busiest_ns.store(0, Ordering::Relaxed);
                self.counts.busiest_frames.store(0, Ordering::Relaxed);
                self.counts.spent_ns.store(0, Ordering::Relaxed);
                self.counts.deadline_ns.store(0, Ordering::Relaxed);
            }
            Command::Init { full, layout } => {
                // A new page's player, as a new worklet's core: every voice
                // let go, the mix built afresh at its quality, the player
                // started on it. About a millisecond, once (the sung
                // voices' references), with nothing sounding.
                core.clear();
                core.quality(full);
                core.set_layout(if layout == 2 { 2 } else { 1 });
                player.attach(core, self.rate, full);
                *base = Tally::of(core);
                *cost_ms = [0.0; 2];
                *cost_seconds = [0.0; 2];
                *window_ms = 0.0;
                *window_frames = 0.0;
                *probes_waiting = 0;
            }
            Command::Dispose => {
                player.stop();
                core.clear();
                *epoch = epoch.wrapping_add(1);
                *probes_waiting = 0;
            }
            Command::Layout(channels) => {
                core.set_layout(channels);
            }
            Command::Cancel(from) => match from {
                None => core.cancel(),
                Some(t) => core.cancel_from(t),
            },
            Command::Param { id, op, value, time, tau } => {
                core.param(id, op, value, time, tau);
            }
            Command::LoadAt { seed, ref request, at, resumed } => {
                if resumed {
                    player.load_resumed(seed, request, at);
                } else {
                    player.load_at(seed, request, at);
                }
            }
            Command::Change { seed, ref request, moment, handover } => player.change(seed, request, moment, handover),
            Command::Revise { seed, ref request } => player.revise(seed, request),
            Command::Track { track, ref arm } => {
                if let Some((t, fades)) = track {
                    player.set_track_with(core, t, fades);
                }
                arm_end(player, arm);
            }
            Command::Arm(ref arm) => arm_end(player, arm),
            Command::Picks { ref request, tag } => player.set_picks(request, tag),
            Command::NextAt { moment, handover } => {
                if let Some(m) = moment {
                    player.next(m, handover);
                }
            }
            Command::Jump(to) => {
                if let Some(j) = to {
                    player.jump(j);
                }
            }
            Command::Tone(tone) => player.set_tone(core, tone),
            Command::Probe(id) => {
                if *probes_waiting < PROBES {
                    probes[*probes_waiting] = id;
                    *probes_waiting += 1;
                }
            }
            Command::Resume(at) => player.resume(at),
            Command::WanderSalted { amount, salt } => {
                player.set_wander(amount);
                player.set_wander_salt(salt);
            }
            Command::Mute { layer, muted } => player.set_mute(core, layer, muted),
            Command::Tempo(bpm) => player.set_tempo(core, bpm),
            Command::Ping(id) => {
                let reduction = [core.reduction(0), core.reduction(1)];
                let deepest = [core.deepest(0), core.deepest(1)];
                let out = Out::Pong {
                    epoch: *epoch,
                    id,
                    tally: Tally::of(core).since(*base),
                    position: position(player),
                    busy: u32::try_from(core.busy()).unwrap_or(u32::MAX),
                    reduction,
                    deepest,
                };
                // SAFETY: the one writer of the queue back.
                if !unsafe { self.out.push(out) } {
                    self.counts.out_lost.fetch_add(1, Ordering::Relaxed);
                }
            }
        }
    }

    /// The name of the loop the player holds, read straight from it: for
    /// the offline render, which owns its engine.
    fn name_now(&mut self) -> String {
        self.inner.get_mut().player.generator().spec().name.as_str().to_owned()
    }

    /// The notes the core could not play as asked (`dl_player_trouble`).
    fn trouble(&mut self) -> u32 {
        let core = &self.inner.get_mut().core;
        core.late().saturating_add(core.missed()).saturating_add(core.dropped())
    }
}

/// A grid slot, for the reader: each layer's velocities.
pub struct GridView<'a>(&'a GridSlot);

impl GridView<'_> {
    /// Layer `l`'s velocities, a step each.
    #[must_use]
    pub fn layer(&self, l: usize) -> &[f64] {
        let n = usize::from(self.0.lens.get(l).copied().unwrap_or(0)).min(GRID_MAX);
        self.0.steps.get(l).map_or(&[], |s| &s[..n])
    }
}

/// The loop that follows the track's end (`js/worklet.js`'s `arm`), or none:
/// the one waiting dropped, so the core's own loop follows.
fn arm_end(player: &mut Player, arm: &ArmEnd) {
    match arm {
        ArmEnd::Drop => player.drop_change(Moment::End),
        ArmEnd::Keep => {}
        ArmEnd::Set(a) => {
            let handover = if a.fade < 0.0 { Handover::RingOut } else { Handover::Crossfade(a.fade) };
            match a.track {
                Some(t) => player.change_with(a.seed, &a.request, Moment::End, handover, Some(t)),
                None => player.change(a.seed, &a.request, Moment::End, handover),
            }
        }
    }
}

/// Where the player is, as the worklet's `player` field says it.
#[allow(clippy::cast_precision_loss, reason = "a step, far below 2^53")]
fn position(player: &Player) -> Position {
    let (step, _, time) = player.position();
    [f64::from(state_number(player.state())), step as f64, time]
}

/// The player's state as the C header numbers it.
fn state_number(state: State) -> u32 {
    match state {
        State::Idle => 0,
        State::Loading => 1,
        State::Wandering => 2,
        State::Playing => 3,
    }
}

/// The core's planes into the device's interleaved frames. The same
/// channels, frame by frame; a stereo render on a mono device as its mid
/// (the mono render, to rounding); a mono render on every channel; past
/// two channels, silence.
fn interleave(out: &mut [f32], channels: usize, left: &[f32], right: &[f32], layout: usize) {
    for (i, frame) in out.chunks_exact_mut(channels).enumerate() {
        match (layout, channels) {
            (2, 1) => frame[0] = f32::midpoint(left[i], right[i]),
            (2, _) => {
                frame[0] = left[i];
                frame[1] = right[i];
                frame[2..].fill(0.0);
            }
            _ => frame.fill(left[i]),
        }
    }
}

/// How a loop is rendered offline, as `dlrender --seed` and
/// `examples/c/seed.c` take it.
#[derive(Clone, Debug)]
pub struct Render {
    pub seed: u32,
    pub request: String,
    pub seconds: f64,
    pub rate: f64,
    pub layout: u32,
    pub full: bool,
    pub wander: f64,
    /// The frames a "callback" asks for: any number, the same samples.
    pub chunk: usize,
}

/// A loop rendered offline through the engine's own path (the commands
/// through the queue, `fill` a chunk at a time, interleaved as a device
/// takes them), and its line as `dlrender --seed`, `seed.c` and
/// `tools/player-wasm.mjs` print it: the loop's name, how it played, FNV-1a
/// 32 over each channel's float words and FNV-1a 64 over their bytes,
/// interleaved. Notes the core could not play as asked are added at the end
/// (` trouble=N`), so a line with any never matches the web's.
///
/// # Errors
///
/// The request's refusal, or the core's of the rate or layout, in plain
/// words.
pub fn render_line(r: &Render) -> Result<String, &'static str> {
    let request = Request::parse(&r.request)?;
    let mut engine = Engine::new(r.rate, r.layout, r.full)?;
    let layout = engine.layout;
    // As seed.c: the wander, the load, the loop made at once.
    for command in [Command::Wander(r.wander), Command::Load { seed: r.seed, request }, Command::Ready] {
        if !engine.send(&command) {
            return Err("the command queue is full");
        }
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "a render's frames, far inside usize")]
    let total = (r.seconds * r.rate).ceil() as usize;
    let chunk = r.chunk.max(1);
    let mut buffer = vec![0.0f32; chunk * layout];
    let mut words = [0x811c_9dc5u32; 2];
    let mut bytes = 0xcbf2_9ce4_8422_2325u64;
    let mut done = 0;
    while done < total {
        let n = (total - done).min(chunk);
        let out = &mut buffer[..n * layout];
        engine.fill(out, layout);
        for frame in out.chunks_exact(layout) {
            for (c, x) in frame.iter().enumerate() {
                let w = x.to_bits();
                words[c] = (words[c] ^ w).wrapping_mul(0x0100_0193);
                for i in 0..4 {
                    bytes = (bytes ^ u64::from((w >> (8 * i)) & 255)).wrapping_mul(0x0000_0100_0000_01b3);
                }
            }
        }
        done += n;
    }
    let mut line = format!(
        "seed={} name={} quality={} layout={} seconds={} rate={} wander={}",
        r.seed,
        engine.name_now(),
        if r.full { "full" } else { "lite" },
        if layout == 2 { "stereo" } else { "mono" },
        r.seconds,
        r.rate,
        r.wander
    );
    if !r.request.is_empty() {
        let _ = write!(line, " request={}", r.request);
    }
    if layout == 2 {
        let _ = write!(line, " frames={total} left={:08x} right={:08x}", words[0], words[1]);
    } else {
        let _ = write!(line, " frames={total} out={:08x}", words[0]);
    }
    let _ = write!(line, " hash={bytes:016x}");
    let trouble = engine.trouble();
    if trouble > 0 {
        let _ = write!(line, " trouble={trouble}");
    }
    Ok(line)
}

/// A line as `render_line` prints it, read back into how it was rendered:
/// the tests take a golden line and render it again.
#[must_use]
pub fn render_of(line: &str, chunk: usize) -> Option<Render> {
    let field = |name: &str| line.split_whitespace().find_map(|w| w.strip_prefix(name)?.strip_prefix('='));
    Some(Render {
        seed: field("seed")?.parse().ok()?,
        request: field("request").unwrap_or("").to_owned(),
        seconds: field("seconds")?.parse().ok()?,
        rate: field("rate")?.parse().ok()?,
        layout: if field("layout")? == "stereo" { 2 } else { 1 },
        full: field("quality")? == "full",
        wander: field("wander")?.parse().ok()?,
        chunk,
    })
}

#[cfg(test)]
mod tests {
    use super::{Command, Engine, Handover, Moment, render_line, render_of};
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::cell::Cell;

    /// Counts this thread's allocations, so a test can say `fill` makes none.
    struct Counting;

    thread_local! {
        static ALLOCATIONS: Cell<u64> = const { Cell::new(0) };
    }

    // SAFETY: every call is passed to the system's allocator unchanged.
    unsafe impl GlobalAlloc for Counting {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            ALLOCATIONS.with(|n| n.set(n.get() + 1));
            // SAFETY: the caller's layout, as given.
            unsafe { System.alloc(layout) }
        }
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            ALLOCATIONS.with(|n| n.set(n.get() + 1));
            // SAFETY: as given.
            unsafe { System.dealloc(ptr, layout) }
        }
        unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
            ALLOCATIONS.with(|n| n.set(n.get() + 1));
            // SAFETY: as given.
            unsafe { System.alloc_zeroed(layout) }
        }
        unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
            ALLOCATIONS.with(|n| n.set(n.get() + 1));
            // SAFETY: as given.
            unsafe { System.realloc(ptr, layout, size) }
        }
    }

    #[global_allocator]
    static COUNTING: Counting = Counting;

    fn allocations() -> u64 {
        ALLOCATIONS.with(Cell::get)
    }

    fn golden(n: usize) -> String {
        let text = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../core/golden/seeds.hashes")).unwrap();
        let line = text.lines().filter(|l| !l.starts_with('#')).nth(n).unwrap();
        line.split_once(' ').unwrap().1.to_owned()
    }

    /// The engine's path makes the golden samples: the corpus's loops as
    /// every target plays them (`core/golden/seeds.hashes`, which
    /// `tools/seeds.sh` holds to `js/dlplayer.wasm` too), at a callback's
    /// size, an odd one, and one past the engine's chunk.
    #[test]
    fn the_engines_path_makes_the_golden_samples() {
        for (n, chunk) in [(0, 192), (1, 333), (2, 2500)] {
            let want = golden(n);
            let render = render_of(&want, chunk).unwrap();
            assert_eq!(render_line(&render).unwrap(), want, "golden line {n}, {chunk} frames a callback");
        }
    }

    /// A line the web's module printed (`DL_WASM_LINE`: CI's build job sets
    /// it from `node tools/player-wasm.mjs`, stereo at a device's 48 kHz),
    /// rendered again through the engine's path at a callback's size.
    #[test]
    fn the_webs_line_if_given() {
        let Ok(want) = std::env::var("DL_WASM_LINE") else { return };
        let render = render_of(&want, 192).expect("DL_WASM_LINE is a hash line");
        assert_eq!(render_line(&render).unwrap(), want);
    }

    /// Promise 2 on the audio thread: half a minute of callbacks of a
    /// device's size, a load, a Next, a pause and a play among them, and not
    /// one allocation inside `fill`.
    #[test]
    fn fill_allocates_nothing() {
        let rate = 48_000.0;
        let engine = Engine::new(rate, 2, true).unwrap();
        let request = dlcore::player::Request::parse("peaceful").unwrap();
        assert!(engine.send(&Command::Draws(20_261_117)));
        assert!(engine.send(&Command::Volume(0.8)));
        assert!(engine.send(&Command::Load { seed: 2_693_262_067, request }));
        // The page's commands too (session 27): a probe, a ping, the track
        // with the loop armed for its end, the picks, a jump, the controls.
        let arm = super::ArmEnd::Set(super::Arm { seed: 99, request, fade: 1.0, track: Some((dlcore::player::Track::Bars(8), dlcore::player::Fades::NONE)) });
        let page = vec![
            Command::Probe(1.0),
            Command::Ping(2.0),
            Command::Track { track: Some((dlcore::player::Track::Bars(16), dlcore::player::Fades::NONE)), arm },
            Command::Picks { request, tag: 3 },
            Command::Jump(Some(dlcore::player::Jump::Bar(4.0))),
            Command::Tone([0.2, 0.7, 0.3]),
            Command::Mute { layer: dlcore::generator::Layer::Drums, muted: true },
            Command::Layout(1),
            Command::Param { id: 18, op: 4, value: 0.0, time: 1.0, tau: 0.0 },
            Command::Cancel(Some(1.0)),
            Command::WanderSalted { amount: 2.2, salt: 9 },
            Command::Tempo(90.0),
            Command::NextAt { moment: Some(Moment::Bar), handover: Handover::Crossfade(1.0) },
        ];
        let burst = 192;
        let mut out = vec![0.0f32; burst * 2];
        let mut inside = 0;
        let callbacks = 30 * 48_000 / burst;
        for i in 0..callbacks {
            match i {
                2_500 => assert!(engine.send(&Command::Next)),
                3_000 => assert!(engine.send_all(&page)),
                5_000 => assert!(engine.send(&Command::Pause)),
                5_500 => assert!(engine.send(&Command::Play)),
                6_000 => assert!(engine.send_all(&[Command::Dispose, Command::Init { full: false, layout: 1 }, Command::LoadAt { seed: 7, request, at: 0.0, resumed: true }])),
                _ => {}
            }
            let before = allocations();
            engine.fill(&mut out, 2);
            inside += allocations() - before;
            // The app's thread, reading the reports back (outside the count).
            if i % 6 == 0 {
                engine.take(|_, _, _| {});
            }
        }
        assert_eq!(inside, 0, "allocations inside fill");
        assert_eq!(engine.counts.out_lost.load(std::sync::atomic::Ordering::Relaxed), 0);
        let c = &engine.counts;
        assert_eq!(c.callbacks.load(std::sync::atomic::Ordering::Relaxed), callbacks as u64);
        assert_eq!(c.late.load(std::sync::atomic::Ordering::Relaxed), 0);
        assert_eq!(c.dropped.load(std::sync::atomic::Ordering::Relaxed), 0);
        assert_eq!(c.state.load(std::sync::atomic::Ordering::Relaxed), 3, "playing again after the pause");
        assert_ne!(engine.name(), "");
    }

    /// A full queue refuses and says so; the audio thread empties it.
    #[test]
    fn a_full_queue_refuses() {
        let engine = Engine::new(48_000.0, 2, true).unwrap();
        let mut sent = 0;
        while engine.send(&Command::Volume(0.5)) {
            sent += 1;
        }
        assert_eq!(sent, super::COMMANDS);
        assert_eq!(engine.counts.refused.load(std::sync::atomic::Ordering::Relaxed), 1);
        let mut out = [0.0f32; 256];
        engine.fill(&mut out, 2);
        assert!(engine.send(&Command::Volume(0.5)));
    }
}
