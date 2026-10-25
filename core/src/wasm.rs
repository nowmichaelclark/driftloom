//! The WebAssembly face of the core, for `js/worklet.js`. One core per
//! instance: the worklet compiles one instance per AudioWorkletNode.
//!
//! Plain numbers in and out, nothing else -- no imports, no strings, no
//! allocator. Samples move through linear memory: before each block the
//! host writes its own notes for each channel at the address `dl_inputs`
//! returns, `CHANNELS` runs of `QUANTUM` floats; after it, what goes to the
//! speakers is at `dl_out`, a run an output channel (`dl_outputs`: one, or
//! two in stereo), what reached the bus compressor at `dl_bus`, as many
//! (`master::OUT` runs apart, the first the mono one), and the channels as
//! they went into the mix at the address `dl_process` returns, `CHANNELS`
//! runs.

use core::cell::UnsafeCell;

use crate::block::Event;
use crate::{Core, Key, Tap, Taken};

/// What the module does if the core panics: nothing reachable from the
/// exports can (`lib.rs`), but a `no_std` module must say. It traps, which
/// the worklet reports as a processor error and the page answers by
/// falling back to the JavaScript synth (queue item 31b).
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

struct Global(UnsafeCell<Core>);

// WebAssembly here runs on one thread: the audio thread that owns the
// worklet. Nothing else can reach the core.
unsafe impl Sync for Global {}

static CORE: Global = Global(UnsafeCell::new(Core::new()));

fn core() -> &'static mut Core {
    // SAFETY: single-threaded (above), and no export holds the reference
    // past its own return.
    unsafe { &mut *CORE.0.get() }
}

/// Where `dl_render` puts the frames it renders.
const RENDER_MAX: usize = 4096;
struct Scratch(UnsafeCell<[f32; RENDER_MAX]>);

// Single-threaded, as above.
unsafe impl Sync for Scratch {}

static SCRATCH: Scratch = Scratch(UnsafeCell::new([0.0; RENDER_MAX]));

/// Set the sample rate and clear everything. The core renders mono until
/// `dl_layout` says otherwise.
#[unsafe(no_mangle)]
pub extern "C" fn dl_init(rate: f64) {
    core().init(rate);
}

/// Render `channels` output channels from the next block: 1 (mono) or 2
/// (stereo, queue item 64). 1 if it did, 0 for any other number.
#[unsafe(no_mangle)]
pub extern "C" fn dl_layout(channels: u32) -> u32 {
    u32::from(core().set_layout(channels))
}

/// A note; see `Core::note`. `a` to `l` are its `EXTRA` values. Returns a
/// `Taken`, as a number.
#[unsafe(no_mangle)]
#[allow(clippy::too_many_arguments)]
pub extern "C" fn dl_note(
    voice: u32,
    channel: u32,
    time: f64,
    midi: f64,
    dur: f64,
    vel: f64,
    parts: u32,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
    g: f64,
    h: f64,
    i: f64,
    j: f64,
    k: f64,
    l: f64,
) -> u32 {
    core().note(
        voice,
        channel,
        time,
        midi,
        dur,
        vel,
        parts,
        [a, b, c, d, e, f, g, h, i, j, k, l],
    ) as u32
}

/// Where the notes sent next sit in their loop; see `Core::key`.
#[unsafe(no_mangle)]
pub extern "C" fn dl_key(seed: u32, pass: u32, step: u32, ord: u32) {
    core().key(Key { seed, pass, step, ord });
}

/// A new strum into `channel` at `time`; see `Core::damp`.
#[unsafe(no_mangle)]
pub extern "C" fn dl_damp(channel: u32, time: f64) {
    core().damp(channel, time);
}

/// An event of `kind` into `channel` at `time`, with `value`; see
/// `Core::event`. Returns 1 if it took.
#[unsafe(no_mangle)]
pub extern "C" fn dl_event(channel: u32, kind: u32, time: f64, value: f64) -> u32 {
    u32::from(core().event(channel, Event::new(kind, time, value)))
}

/// A retune of `channel` to a chord of up to six notes at `time` (NaN past
/// the last); see `Core::event` and `event::RETUNE`. Returns 1 if it took.
#[unsafe(no_mangle)]
pub extern "C" fn dl_retune(channel: u32, time: f64, n1: f64, n2: f64, n3: f64, n4: f64, n5: f64, n6: f64) -> u32 {
    u32::from(core().event(channel, Event::retune(time, &[n1, n2, n3, n4, n5, n6])))
}

/// Set a setting of one of `channel`'s own blocks at `time`; see
/// `Core::set`. Returns 1 if it took.
#[unsafe(no_mangle)]
pub extern "C" fn dl_set(channel: u32, block: u32, setting: u32, value: f64, time: f64) -> u32 {
    u32::from(core().set(channel, block as usize, setting as usize, value, time))
}

/// Route `from_channel` (or one of its own blocks, `from_block`; 4294967295
/// for the whole channel) into a setting of one of `channel`'s own blocks
/// from `time`, or undo the route with `from_channel` 4294967295; see
/// `Core::route`. Returns 1 if it took.
#[unsafe(no_mangle)]
pub extern "C" fn dl_route(channel: u32, block: u32, setting: u32, from_channel: u32, from_block: u32, time: f64) -> u32 {
    let from = (from_channel != u32::MAX).then_some(Tap {
        channel: from_channel,
        block: (from_block != u32::MAX).then_some(from_block as usize),
    });
    u32::from(core().route(channel, block as usize, setting as usize, from, time))
}

/// The mix as the synth builds it on full quality (1) or lite (0); see
/// `Core::quality`. Returns whether the core mixes at its rate.
#[unsafe(no_mangle)]
pub extern "C" fn dl_quality(full: u32) -> u32 {
    core().quality(full != 0);
    core().mixing() as u32
}

/// Move one of the mix's parameters; see `Core::param`. Returns 1 if it
/// took.
#[unsafe(no_mangle)]
pub extern "C" fn dl_param(id: u32, op: u32, value: f64, time: f64, tau: f64) -> u32 {
    core().param(id, op, value, time, tau) as u32
}

/// Render `frames` (at most 4,096) the way a host with its own block size
/// does, `Core::render`, and return where they are: a run of that many
/// floats. The worklet never calls this (it owns the block clock and calls
/// `dl_process`); the tools that check the sound does not depend on the
/// host's block size do. In stereo the right side follows the left: two
/// runs of `frames`, at most half the scratch each. 0 if the core would not
/// render.
#[unsafe(no_mangle)]
pub extern "C" fn dl_render(frames: u32) -> *const f32 {
    // SAFETY: single-threaded (above), and nothing else holds the scratch.
    let scratch = unsafe { &mut *SCRATCH.0.get() };
    let core = core();
    let stereo = core.layout() == 2;
    let frames = (frames as usize).min(if stereo { RENDER_MAX / 2 } else { RENDER_MAX });
    let (left, rest) = scratch.split_at_mut(frames);
    let result = if stereo {
        let Some(right) = rest.get_mut(..frames) else {
            return core::ptr::null();
        };
        core.render(&mut [left, right])
    } else {
        core.render(&mut [left])
    };
    match result {
        Ok(()) => scratch.as_ptr(),
        Err(_) => core::ptr::null(),
    }
}

/// Where the host writes its own notes for the next block.
#[unsafe(no_mangle)]
pub extern "C" fn dl_inputs() -> *mut f32 {
    core().inputs().as_mut_ptr().cast()
}

/// Where the last block's output is, a run an output channel.
#[unsafe(no_mangle)]
pub extern "C" fn dl_out() -> *const f32 {
    core().output().as_ptr().cast()
}

/// Set the space recipe's setting `which` to draw from `lo` to `hi`
/// (`Core::space_range`; queue item 65). 1 if it did.
#[unsafe(no_mangle)]
pub extern "C" fn dl_space_range(which: u32, lo: f64, hi: f64) -> u32 {
    u32::from(core().space_range(which as usize, lo, hi))
}

/// The number of output channels: the layout, 1 or 2.
#[unsafe(no_mangle)]
pub extern "C" fn dl_outputs() -> u32 {
    core().layout()
}

/// Where the last block was as it reached the bus compressor.
#[unsafe(no_mangle)]
pub extern "C" fn dl_bus() -> *const f32 {
    core().bus().as_ptr().cast()
}

/// The bus compressor's `reduction` (0) or the ceiling's (1), in dB.
#[unsafe(no_mangle)]
pub extern "C" fn dl_reduction(which: u32) -> f32 {
    core().reduction(which as usize)
}

/// How hard the signal has pressed the bus compressor (0) or the ceiling
/// (1) since this was last asked, in dB (`Compressor::deepest`).
#[unsafe(no_mangle)]
pub extern "C" fn dl_deepest(which: u32) -> f32 {
    core().deepest(which as usize)
}

/// Render the block that starts at frame `block` and return where its
/// channels are (the output is at `dl_out`).
#[unsafe(no_mangle)]
pub extern "C" fn dl_process(block: f64) -> *const f32 {
    let frame = if block > 0.0 { block as u64 } else { 0 };
    core().process(frame).as_ptr().cast()
}

/// Let every voice go.
#[unsafe(no_mangle)]
pub extern "C" fn dl_clear() {
    core().clear();
}

/// Drop every note still waiting to start (queue item 39).
#[unsafe(no_mangle)]
pub extern "C" fn dl_cancel() {
    core().cancel();
}

/// Drop the notes waiting to start at `time` or later (queue item 40).
#[unsafe(no_mangle)]
pub extern "C" fn dl_cancel_from(time: f64) {
    core().cancel_from(time);
}

#[unsafe(no_mangle)]
pub extern "C" fn dl_late() -> u32 {
    core().late()
}

/// Notes that arrived more than `MISS` late and were left out (queue item
/// 41).
#[unsafe(no_mangle)]
pub extern "C" fn dl_missed() -> u32 {
    core().missed()
}

#[unsafe(no_mangle)]
pub extern "C" fn dl_dropped() -> u32 {
    core().dropped()
}

/// Notes dropped for one cause; see `Core::dropped_by`.
#[unsafe(no_mangle)]
pub extern "C" fn dl_dropped_by(cause: u32) -> u32 {
    core().dropped_by(cause as usize)
}

#[unsafe(no_mangle)]
pub extern "C" fn dl_busy() -> u32 {
    core().busy() as u32
}

// Kept in step with `Taken`; the worklet reads these numbers.
const _: () = assert!(
    Taken::Late as u32 == 1 && Taken::Full as u32 == 2 && Taken::Missed as u32 == 4
);
