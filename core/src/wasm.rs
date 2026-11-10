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
/// (stereo, queue item 64). 1 if it did, 0 for any other number. While the
/// core plays, the switch fades (`Core::set_layout`, queue item 66).
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

/// Let the notes of `channel` keyed (`seed`, `pass`, `step`, `ord`) go,
/// falling to nothing from `from` to `to` (queue item 85c); see
/// `Core::steal`. Returns how many it reached.
#[unsafe(no_mangle)]
pub extern "C" fn dl_steal(channel: u32, seed: u32, pass: u32, step: u32, ord: u32, from: f64, to: f64) -> u32 {
    core().steal(channel, Key { seed, pass, step, ord }, from, to)
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

/// Set the bass layer's lift in dB for the loop; see `Core::bass_lift`
/// (queue item 76c). 1 if it took.
#[unsafe(no_mangle)]
pub extern "C" fn dl_bass_lift(db: f64) -> u32 {
    u32::from(core().bass_lift(db))
}

/// One of the core's own math functions (`crate::math`) at `x` (and `y`,
/// for `pow`): 0 `pow`, 1 `exp`, 2 `log`, 3 `log2`, 4 `log10`, 5 `sin`,
/// 6 `cos`, 7 `sqrt`; NaN for any other `which`. Nothing that plays calls
/// it: it is how `test/dlmath.test.mjs` holds the generator's JavaScript
/// mirror (`js/dlmath.js`, queue item 77) to these functions bit for bit,
/// in the very module the app runs.
#[unsafe(no_mangle)]
pub extern "C" fn dl_math(which: u32, x: f64, y: f64) -> f64 {
    use crate::math;
    match which {
        0 => math::pow(x, y),
        1 => math::exp(x),
        2 => math::log(x),
        3 => math::log2(x),
        4 => math::log10(x),
        5 => math::sin(x),
        6 => math::cos(x),
        7 => math::sqrt(x),
        _ => f64::NAN,
    }
}

/// The number of output channels: the layout, 1 or 2.
#[unsafe(no_mangle)]
pub extern "C" fn dl_outputs() -> u32 {
    core().layout()
}

/// How many of `dl_out`'s runs the last block filled: the layout, except
/// while a switch into mono fades the side out (`Core::lanes`, queue item
/// 66), when it is still 2.
#[unsafe(no_mangle)]
pub extern "C" fn dl_lanes() -> u32 {
    core().lanes()
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

/// Let every note handed over go, falling from `from` to nothing at `to`
/// (seconds; `Core::fade_all`): the page's track that its length slider
/// ends inside a pass, at a phrase line (queue item 88b), before it hands
/// over the next loop's notes. Returns how many notes it reached.
#[unsafe(no_mangle)]
pub extern "C" fn dl_fade_all(from: f64, to: f64) -> u32 {
    core().fade_all(from, to)
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

// ------------------------------------------------------------ the generator

/// The generator's exports (queue item 79), in a build with the
/// `generator` feature only: not in the app's `.wasm` until the player is
/// (item 82). `tools/gen-core.sh` builds one and checks the generation
/// goldens through it in Node. A host writes the vibes it asks for at
/// `dl_gen_input` (one byte a vibe, its number), loads a loop, and reads
/// what the generator made: a part's hash, or the spec as text.
#[cfg(feature = "generator")]
mod generator {
    use core::cell::UnsafeCell;

    use crate::generator::canon::Text;
    use crate::generator::request::Request;
    use crate::generator::{Generator, Part, data};

    struct Global(UnsafeCell<Generator>);
    // Single-threaded, as the core's own global.
    unsafe impl Sync for Global {}
    static GENERATOR: Global = Global(UnsafeCell::new(Generator::empty()));

    struct Bytes<const N: usize>(UnsafeCell<[u8; N]>);
    // Single-threaded, as above.
    unsafe impl<const N: usize> Sync for Bytes<N> {}
    /// Room for the vibes' numbers, or a request's text (`dl_gen_request`).
    const INPUT_MAX: usize = 1024;
    static INPUT: Bytes<INPUT_MAX> = Bytes(UnsafeCell::new([0; INPUT_MAX]));
    /// Room for a part's text: the spec's is about 600 bytes, the longest
    /// in the goldens (a wandered pass, the whole pattern) 74 KB.
    const TEXT_MAX: usize = 1 << 20;
    static TEXT: Bytes<TEXT_MAX> = Bytes(UnsafeCell::new([0; TEXT_MAX]));

    fn generator() -> &'static mut Generator {
        // SAFETY: single-threaded, and no export keeps the reference.
        unsafe { &mut *GENERATOR.0.get() }
    }

    /// Where the host writes the vibes of the next `dl_gen_load`.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_gen_input() -> *mut u8 {
        INPUT.0.get().cast()
    }

    /// Generate the loop of `seed` with the first `vibes` bytes at
    /// `dl_gen_input` as its vibes. 1 if it did; 0 if a vibe's number is
    /// unknown (nothing is loaded then).
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_gen_load(seed: u32, vibes: u32) -> u32 {
        // SAFETY: single-threaded; the host wrote the bytes before calling.
        let input = unsafe { &*INPUT.0.get() };
        let mut picked = [0usize; data::VIBE_COUNT];
        let n = (vibes as usize).min(input.len());
        for (p, &b) in picked.iter_mut().zip(&input[..n]) {
            if usize::from(b) >= data::VIBE_COUNT {
                return 0;
            }
            *p = usize::from(b);
        }
        generator().load(seed, &picked[..n]);
        1
    }

    /// Generate the loop of `seed` as the request whose text is the first
    /// `length` bytes at `dl_gen_input` asks (`Request::parse`; the goldens'
    /// second field). 1 if it did; 0 if the request cannot be met (nothing
    /// is loaded then).
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_gen_request(seed: u32, length: u32) -> u32 {
        // SAFETY: single-threaded; the host wrote the bytes before calling.
        let input = unsafe { &*INPUT.0.get() };
        let bytes = &input[..(length as usize).min(input.len())];
        match core::str::from_utf8(bytes).map(Request::parse) {
            Ok(Ok(r)) => {
                generator().load_with(seed, &r);
                1
            }
            _ => 0,
        }
    }

    /// The hash of part `part` (`Part::ALL`'s order) of the loop loaded.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_gen_hash(part: u32) -> u64 {
        Part::ALL.get(part as usize).map_or(0, |&p| generator().hash(p))
    }

    /// How many items the generator's lists dropped (0).
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_gen_lost() -> u32 {
        u32::try_from(generator().lost()).unwrap_or(u32::MAX)
    }

    /// The loaded loop's spec as plain ASCII, its canonical text (the
    /// goldens' `spec`), at `dl_gen_text`; returns its length.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_spec() -> u32 {
        // SAFETY: single-threaded, as above.
        let buf = unsafe { &mut *TEXT.0.get() };
        let mut text = Text { buf, len: 0, lost: 0 };
        generator().write(Part::Spec, &mut text);
        u32::try_from(text.len).unwrap_or(0)
    }

    /// Part `part`'s canonical text (`Part::ALL`'s order) at
    /// `dl_gen_text`; returns its length (0 for no such part, or one too
    /// long for the buffer).
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_gen_part(part: u32) -> u32 {
        let Some(&p) = Part::ALL.get(part as usize) else { return 0 };
        // SAFETY: single-threaded, as above.
        let buf = unsafe { &mut *TEXT.0.get() };
        let mut text = Text { buf, len: 0, lost: 0 };
        generator().write(p, &mut text);
        if text.lost > 0 { 0 } else { u32::try_from(text.len).unwrap_or(0) }
    }

    /// Where `dl_spec` and `dl_gen_part` write.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_gen_text() -> *const u8 {
        TEXT.0.get().cast()
    }
}

// --------------------------------------------------------------- the player

/// The player's exports (queue item 82), in a build with the `player`
/// feature: `js/dlplayer.wasm`, which the app plays with `?engine=player`
/// (for testing; the app's own `.wasm` stays without it until phase 3). The
/// worklet starts the core as always (`dl_init`, `dl_quality`, `dl_layout`),
/// then the player on it (`dl_play_attach`); it asks for a loop (the vibes
/// first, `dl_play_vibe`, then `dl_play_load`) and renders each block with
/// `dl_play_process`, which decides the block's steps first. The load is
/// spread across the blocks that follow, so it never spikes one.
#[cfg(feature = "player")]
mod player {
    use core::cell::UnsafeCell;

    use super::core;
    use crate::generator::Layer;
    use crate::player::about::GRID_MAX;
    use crate::player::events::EVENTS_MAX;
    use crate::player::{Handover, Moment, Player, Request, State, Track};

    struct Global(UnsafeCell<Player>);
    // Single-threaded, as the core's own global.
    unsafe impl Sync for Global {}
    static PLAYER: Global = Global(UnsafeCell::new(Player::new()));

    struct Asked(UnsafeCell<Request>);
    // Single-threaded, as above.
    unsafe impl Sync for Asked {}
    static REQUEST: Asked = Asked(UnsafeCell::new(Request::new()));

    pub(super) fn player() -> &'static mut Player {
        // SAFETY: single-threaded, and no export keeps the reference.
        unsafe { &mut *PLAYER.0.get() }
    }

    pub(super) fn request() -> &'static mut Request {
        // SAFETY: single-threaded, and no export keeps the reference.
        unsafe { &mut *REQUEST.0.get() }
    }

    /// The first thing refused in the request being asked (queue item 95), so
    /// the picks can say why they can't be met: plain ASCII, the request's
    /// own line ("tempo must be 32 to 190 bpm"). None while all is met.
    struct Refused(UnsafeCell<Option<&'static str>>);
    // Single-threaded, as above.
    unsafe impl Sync for Refused {}
    static REFUSED: Refused = Refused(UnsafeCell::new(None));

    pub(super) fn refused() -> &'static mut Option<&'static str> {
        // SAFETY: single-threaded, and no export keeps the reference.
        unsafe { &mut *REFUSED.0.get() }
    }

    /// A part of the request asked: kept if it can be met (1), else the
    /// request left as it was and the refusal noted (0).
    // The request comes back from its builder by value, as each `with_` makes it.
    #[allow(clippy::large_types_passed_by_value)]
    fn take(asked: Result<Request, &'static str>) -> u32 {
        match asked {
            Ok(r) => {
                *request() = r;
                1
            }
            Err(why) => {
                refused().get_or_insert(why);
                0
            }
        }
    }

    /// The request asked so far, taken for a load, a change, a re-roll or
    /// the picks: the next one starts from nothing.
    fn taken() -> (Request, Option<&'static str>) {
        let r = *request();
        *request() = Request::new();
        (r, refused().take())
    }

    /// Start the player on the core just started (`Player::attach`), with
    /// the full mix (1) or lite's (0), as the worklet asked `dl_quality`.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_attach(full: u32) {
        let rate = core().context().rate;
        player().attach(core(), rate, full != 0);
    }

    /// Add the vibe numbered `index` to what the next `dl_play_load` asks for
    /// (`generator::data::VIBES`, `VIBE_IDS` in `js/vibes.js`). 1 if the core
    /// has it.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_vibe(index: u32) -> u32 {
        let known = (index as usize) < crate::generator::data::VIBE_COUNT;
        *request() = request().with(index as usize);
        if !known {
            refused().get_or_insert(crate::generator::request::NO_SUCH_VIBE);
        }
        u32::from(known)
    }

    /// Add the vibe numbered `index` at `weight` to what the next
    /// `dl_play_load` asks for (`Request::with_weight`). 1 if it can be met.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_weight(index: u32, weight: f64) -> u32 {
        take(request().with_weight(index as usize, weight))
    }

    /// Ask the next `dl_play_load` for a tempo, `lo` to `hi` bpm
    /// (`Request::with_tempo`). 1 if it can be met.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_ask_tempo(lo: f64, hi: f64) -> u32 {
        take(request().with_tempo(lo, hi))
    }

    /// Bend the loop of `seed` in what the next `dl_play_load` asks for
    /// (`Request::with_theme`; queue item 85d).
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_theme(seed: u32) {
        *request() = request().with_theme(seed);
    }

    /// Ask the next `dl_play_load` for a piece `bars` bars long, or Endless
    /// with `bars` infinite (`Request::with_length`; queue item 88). 1 if it
    /// can be met.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_length(bars: f64) -> u32 {
        use crate::generator::request::Length;
        let length = if bars == f64::INFINITY { Length::Endless } else { Length::Bars(bars) };
        take(request().with_length(length))
    }

    /// Ask the next `dl_play_load` for the home notes in `keys`, a bit each
    /// with C the lowest (`Request::with_keys`; queue item 89). 1 if it can
    /// be met.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_keys(keys: u32) -> u32 {
        take(request().with_keys(keys))
    }

    /// Ask the next `dl_play_load` for the modes in `scales`, a bit each by
    /// its place in the core's list (`Request::with_scales`; queue item 89).
    /// 1 if it can be met.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_scales(scales: u32) -> u32 {
        take(request().with_scales(scales))
    }

    /// Ask the next `dl_play_load` for the profiles in `styles`, a bit each
    /// by place in the frozen order (`Request::with_styles`; queue item 90).
    /// 1 if it can be met.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_styles(styles: u32) -> u32 {
        take(request().with_styles(styles))
    }

    /// Ask the next `dl_play_load` for the voices in `voices` of layer
    /// `layer` (0 to 4), a bit each by place in the layer's list
    /// (`Request::with_voices`; queue item 91). 1 if it can be met.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_voices(layer: u32, voices: u32) -> u32 {
        take(request().with_voices(layer as usize, voices))
    }

    /// Ask the next `dl_play_load` for layer `layer` (0 to 4) to be left to
    /// chance (`state` 0), always played (1) or left out (2)
    /// (`Request::with_layer`; queue item 91). 1 if it can be met.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_layer(layer: u32, state: u32) -> u32 {
        use crate::generator::request::LayerAsk;
        let ask = match state {
            0 => LayerAsk::Chance,
            1 => LayerAsk::On,
            2 => LayerAsk::Off,
            _ => return 0,
        };
        take(request().with_layer(layer as usize, ask))
    }

    /// Pin setting `number` at `value` for what the next `dl_play_load`
    /// asks for (`Request::with_pin`; queue item 85b). 1 if it can be met.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_pin(number: u32, value: f64) -> u32 {
        take(request().with_pin(number as usize, value))
    }

    /// Set layer `layer`'s seed (0 to 4, the core's order) to `seed` in what
    /// the next load, change or re-roll asks for, re-rolled (always played)
    /// with `rolled` not 0 (`Request::with_layer_seed`; queue item 94a, a
    /// code's edit). 1 if it can be met.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_seed(layer: u32, seed: u32, rolled: u32) -> u32 {
        take(request().with_layer_seed(layer as usize, seed, rolled != 0))
    }

    /// Set the tone's part `which` (0 warmth, 1 space, 2 wobble) to `value`,
    /// 0 to 1, in what the next load asks for (`Request::with_tone`; a code's
    /// edit). 1 if it can be met.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_edit_tone(which: u32, value: f64) -> u32 {
        take(request().with_tone(which as usize, value))
    }

    /// Set the tempo to `bpm` after the next load's loop is drawn
    /// (`Request::with_bpm`; a code's edit, the tempo slider's). 1 if it can
    /// be met.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_edit_bpm(bpm: f64) -> u32 {
        take(request().with_bpm(bpm))
    }

    /// The loop playing made again with what was asked since the last load,
    /// in its place (`Player::revise`; queue item 94a, a re-roll): the loop
    /// of `seed` with a layer's new seed (`dl_play_seed`).
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_revise(seed: u32) {
        player().revise(seed, &taken().0);
    }

    /// The tone from now (`Player::set_tone`): warmth, space and wobble.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_tone(warmth: f64, space: f64, wobble: f64) {
        player().set_tone(core(), [warmth, space, wobble]);
    }

    /// How long the track plays (`Player::set_track`): `kind` 0 on and on, 1
    /// `count` whole passes, 2 `count` bars (to the next phrase line). 1 if
    /// it is a kind there is.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_track(kind: u32, count: u32) -> u32 {
        let Some(track) = track_of(kind, count) else { return 0 };
        player().set_track(core(), track);
        1
    }

    /// The track `kind` and `count` say (`dl_play_track`'s numbers).
    fn track_of(kind: u32, count: u32) -> Option<Track> {
        match kind {
            0 => Some(Track::Always),
            1 => Some(Track::Passes(count.max(1))),
            2 => Some(Track::Bars(count.max(1))),
            _ => None,
        }
    }

    /// The step from the loop's load the track ends at
    /// (`Player::track_end`); -1 while it plays on.
    #[unsafe(no_mangle)]
    #[allow(clippy::cast_precision_loss)] // a step, far below 2^53
    pub extern "C" fn dl_play_track_end() -> f64 {
        player().track_end().map_or(-1.0, |s| s as f64)
    }

    struct Floats<const N: usize>(UnsafeCell<[f64; N]>);
    // Single-threaded, as above.
    unsafe impl<const N: usize> Sync for Floats<N> {}
    static EVENTS: Floats<{ 4 * EVENTS_MAX }> = Floats(UnsafeCell::new([0.0; 4 * EVENTS_MAX]));
    static GRID: Floats<GRID_MAX> = Floats(UnsafeCell::new([0.0; GRID_MAX]));

    /// Take every event waiting (`Player::event`; queue item 94a) into
    /// `dl_play_event_at`, four doubles each: its time on the core's clock,
    /// its kind (`Kind::ALL`'s order: 0 step, 1 pass, 2 section, 3 phrase, 4
    /// loop, 5 revised, 6 end, 7 own), its number and its step from the load.
    /// Returns how many. The worklet calls it after each block.
    #[unsafe(no_mangle)]
    #[allow(clippy::cast_precision_loss)] // numbers and steps, far below 2^53
    pub extern "C" fn dl_play_events() -> u32 {
        // SAFETY: single-threaded, and no export keeps the reference.
        let buf = unsafe { &mut *EVENTS.0.get() };
        let mut n = 0;
        for slot in buf.as_chunks_mut::<4>().0 {
            let Some(e) = player().event() else { break };
            slot.copy_from_slice(&[e.time, f64::from(e.kind as u8), e.number as f64, e.step as f64]);
            n += 1;
        }
        n
    }

    /// Where `dl_play_events` writes.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_event_at() -> *const f64 {
        EVENTS.0.get().cast()
    }

    /// Events lost for want of room (`Player::events_lost`).
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_events_lost() -> u32 {
        player().events_lost()
    }

    /// Layer `layer`'s grid (`Player::grid`) at `dl_play_grid_at`, a double
    /// a step; returns its steps (0 for none).
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_grid(layer: u32) -> u32 {
        let Some(&l) = crate::generator::LAYERS.get(layer as usize) else { return 0 };
        // SAFETY: single-threaded, and no export keeps the reference.
        let buf = unsafe { &mut *GRID.0.get() };
        u32::try_from(player().grid(l, buf)).unwrap_or(0)
    }

    /// Where `dl_play_grid` writes.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_grid_at() -> *const f64 {
        GRID.0.get().cast()
    }

    static NOTE: Floats<16> = Floats(UnsafeCell::new([0.0; 16]));

    /// How many notes layer `layer` has in the loop as composed
    /// (`Player::notes`; the MIDI export's).
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_notes(layer: u32) -> u32 {
        let Some(&l) = crate::generator::LAYERS.get(layer as usize) else { return 0 };
        u32::try_from(player().notes(l)).unwrap_or(0)
    }

    /// Layer `layer`'s note `index` (`Player::note`) at `dl_play_note_at`:
    /// its step, length, velocity, offset (a roll's), how many pitches, then
    /// the pitches; a drum hit's instrument at `dl_play_text`, its length
    /// returned. -1 past the last note.
    #[unsafe(no_mangle)]
    #[allow(clippy::cast_precision_loss)] // a few pitches
    pub extern "C" fn dl_play_note(layer: u32, index: u32) -> i32 {
        let Some(&l) = crate::generator::LAYERS.get(layer as usize) else { return -1 };
        let Some(n) = player().note(l, index as usize) else { return -1 };
        // SAFETY: single-threaded, and no export keeps the reference.
        let buf = unsafe { &mut *NOTE.0.get() };
        buf[..5].copy_from_slice(&[n.step, n.dur, n.vel, n.micro, n.count as f64]);
        for (slot, &p) in buf[5..].iter_mut().zip(&n.notes) {
            *slot = p;
        }
        // SAFETY: as above.
        let name = unsafe { &mut *NAME.0.get() };
        let drum = n.drum.as_bytes();
        let k = drum.len().min(name.len());
        name[..k].copy_from_slice(&drum[..k]);
        i32::try_from(k).unwrap_or(0)
    }

    /// Where `dl_play_note` writes the numbers.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_note_at() -> *const f64 {
        NOTE.0.get().cast()
    }

    /// Room for the loop's description: some 600 bytes, with room to spare.
    const ABOUT_MAX: usize = 4096;
    struct About(UnsafeCell<[u8; ABOUT_MAX]>);
    // Single-threaded, as above.
    unsafe impl Sync for About {}
    static ABOUT: About = About(UnsafeCell::new([0; ABOUT_MAX]));

    /// A writer into a fixed buffer that fails rather than cut short.
    struct Fill<'a> {
        buf: &'a mut [u8],
        len: usize,
    }

    impl core::fmt::Write for Fill<'_> {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            let end = self.len + s.len();
            let slot = self.buf.get_mut(self.len..end).ok_or(core::fmt::Error)?;
            slot.copy_from_slice(s.as_bytes());
            self.len = end;
            Ok(())
        }
    }

    /// The loop's description (`Player::describe`: plain ASCII lines,
    /// `name=value`) at `dl_play_about_at`; returns its length (0 with no
    /// loop made).
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_describe() -> u32 {
        // SAFETY: single-threaded, and no export keeps the reference.
        let buf = unsafe { &mut *ABOUT.0.get() };
        let mut w = Fill { buf, len: 0 };
        match player().describe(&mut w) {
            Ok(()) => u32::try_from(w.len).unwrap_or(0),
            Err(_) => 0,
        }
    }

    /// Where `dl_play_describe` writes.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_about_at() -> *const u8 {
        ABOUT.0.get().cast()
    }

    /// Ask for the loop of `seed` with what was asked since the last load,
    /// its first step at `at` seconds on the core's clock, or as soon as it
    /// is made if that has passed (0: as soon as made). See `Player::load`.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_load(seed: u32, at: f64) {
        player().load_at(seed, &taken().0, at);
    }

    /// `dl_play_load` for a loop resumed on a new core (`Player::load_resumed`;
    /// queue item 94b, the app's automatic quality switch): its lateFloor is
    /// not waited out a second time.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_load_resumed(seed: u32, at: f64) {
        player().load_resumed(seed, &taken().0, at);
    }

    /// Change to the loop of `seed` with what was asked since the last load,
    /// at `moment` of the loop playing (0 now, 1 the next beat, 2 bar, 3
    /// phrase, 4 pass, 5 the track's end: `Moment::ALL`), the old loop crossfading over
    /// `crossfade` seconds, or ringing out for a negative one (queue item
    /// 86, `Player::change`). 0 for a moment there is no such number for
    /// (nothing changes), else 1.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_change(seed: u32, moment: u32, crossfade: f64) -> u32 {
        let Some(m) = Moment::of(moment) else { return 0 };
        let handover = if crossfade < 0.0 { Handover::RingOut } else { Handover::Crossfade(crossfade) };
        player().change(seed, &taken().0, m, handover);
        1
    }

    /// `dl_play_change`, and how long the loop's track plays once it starts
    /// (`Player::change_with_track`; queue item 95): `kind` and `count` as
    /// `dl_play_track` takes them. The app arms the loop that follows a
    /// track's end this way. 0 for a moment or a kind there is no such number
    /// for (nothing changes), else 1.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_change_track(seed: u32, moment: u32, crossfade: f64, kind: u32, count: u32) -> u32 {
        let Some(m) = Moment::of(moment) else { return 0 };
        let Some(track) = track_of(kind, count) else { return 0 };
        let handover = if crossfade < 0.0 { Handover::RingOut } else { Handover::Crossfade(crossfade) };
        player().change_with_track(seed, &taken().0, m, handover, Some(track));
        1
    }

    /// A new loop of the core's own at `moment` (`Player::next`; queue item
    /// 95, the app's Next at the end of its history), crossfading as
    /// `dl_play_change` does: its seed drawn from `dl_play_draws`', with the
    /// picks (`dl_play_picks`). 1 if asked; 0 for no such moment, or with no
    /// seed to draw from.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_next(moment: u32, crossfade: f64) -> u32 {
        let Some(m) = Moment::of(moment) else { return 0 };
        let handover = if crossfade < 0.0 { Handover::RingOut } else { Handover::Crossfade(crossfade) };
        u32::from(player().next(m, handover))
    }

    /// Drop the change waiting for `moment` (`Player::drop_change`; queue item
    /// 95): the app drops the one armed for the track's end when nothing it
    /// holds comes next, so the core's own loop follows. 1 if there is such
    /// a moment.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_drop(moment: u32) -> u32 {
        let Some(m) = Moment::of(moment) else { return 0 };
        player().drop_change(m);
        1
    }

    /// Why the picks last held could not be met as asked, if they could not
    /// (plain ASCII, at `dl_play_refusal_at`).
    struct Why(UnsafeCell<Option<&'static str>>);
    // Single-threaded, as above.
    unsafe impl Sync for Why {}
    static PICKS_REFUSED: Why = Why(UnsafeCell::new(None));

    /// Hold what was asked since the last load as the picks, numbered `tag`
    /// (`Player::set_picks`; queue item 95, the app's New loops panel): every
    /// loop the core makes itself asks them. If any part of them was refused,
    /// only their vibes are held, as the page's own draw kept the moods of a
    /// request it could not meet, and the first refusal is kept for
    /// `dl_play_refusal`. 1 if held whole, 0 if only the vibes.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_picks(tag: u32) -> u32 {
        let (r, why) = taken();
        // SAFETY: single-threaded, and no export keeps the reference.
        unsafe { *PICKS_REFUSED.0.get() = why };
        if why.is_none() {
            player().set_picks(&r, tag);
            1
        } else {
            player().set_picks(&r.vibes_only(), tag);
            0
        }
    }

    /// The length of why the picks last held could not be met (its bytes at
    /// `dl_play_refusal_at`); 0 if they were met.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_refusal() -> u32 {
        // SAFETY: single-threaded, and no export keeps the reference.
        let why = unsafe { *PICKS_REFUSED.0.get() };
        why.map_or(0, |w| u32::try_from(w.len()).unwrap_or(0))
    }

    /// Where that line's bytes are (null if there is none).
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_refusal_at() -> *const u8 {
        // SAFETY: single-threaded, and no export keeps the reference.
        let why = unsafe { *PICKS_REFUSED.0.get() };
        why.map_or(core::ptr::null(), str::as_ptr)
    }

    /// The number of the picks the core holds (`Player::picks`).
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_picks_tag() -> u32 {
        player().picks().1
    }

    /// Give the core a seed to draw its own loops' seeds from
    /// (`Player::set_draws`; queue item 95); 0, none of its own.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_draws(seed: u32) {
        player().set_draws(seed);
    }

    /// When the next change waiting starts its loop, in seconds on the
    /// core's clock (`Player::next_change`); -1 if none is worked out yet.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_next_change() -> f64 {
        player().next_change().unwrap_or(-1.0)
    }

    /// Make what is left of the loop asked for, now (`Player::ready`): for a
    /// host with no deadline (an offline render, the checks), never on an
    /// audio thread.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_ready() {
        player().ready(core());
    }

    /// Go on with the loop stopped, its next step at `at` seconds
    /// (`Player::resume`).
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_resume(at: f64) {
        player().resume(at);
    }

    /// Stop deciding: the loop stays loaded, and what is handed over plays
    /// out (`Player::stop`).
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_stop() {
        player().stop();
    }

    /// Render the block that starts at frame `block` (`dl_process`), the
    /// player deciding its steps first (`Player::decide`).
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_process(block: f64) -> *const f32 {
        player().decide(core());
        super::dl_process(block)
    }

    /// Render the next `frames` frames (at most 4,096, or 2,048 a side in
    /// stereo) the way a host rendering ahead does (`Player::render`; queue
    /// item 87): any number a call, each call carrying on where the last
    /// stopped. Where they are, as `dl_render` says; 0 if the core would not
    /// render. Never mixed with `dl_play_process`, which owns the block clock.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_render(frames: u32) -> *const f32 {
        // SAFETY: single-threaded, and nothing else holds the scratch.
        let scratch = unsafe { &mut *super::SCRATCH.0.get() };
        let core = core();
        let stereo = core.layout() == 2;
        let frames = (frames as usize).min(if stereo { super::RENDER_MAX / 2 } else { super::RENDER_MAX });
        let (left, rest) = scratch.split_at_mut(frames);
        let result = if stereo {
            let Some(right) = rest.get_mut(..frames) else {
                return core::ptr::null();
            };
            player().render(core, &mut [left, right])
        } else {
            player().render(core, &mut [left])
        };
        match result {
            Ok(()) => scratch.as_ptr(),
            Err(_) => core::ptr::null(),
        }
    }

    /// The frames to the end of the pass playing (`Player::pass_frames`);
    /// -1 while no loop plays.
    #[unsafe(no_mangle)]
    #[allow(clippy::cast_precision_loss)] // frames, far below 2^53
    pub extern "C" fn dl_play_pass_frames() -> f64 {
        player().pass_frames(core()).map_or(-1.0, |n| n as f64)
    }

    /// The mean cost units the loop's voices hold (`Player::units`); -1 for
    /// none measured yet.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_units() -> f64 {
        player().units().unwrap_or(-1.0)
    }

    /// How long the next `frames` frames will take to render, in
    /// microseconds on the reference host (`Player::estimate`).
    #[unsafe(no_mangle)]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // a count of frames
    pub extern "C" fn dl_play_estimate(frames: f64) -> f64 {
        let n = if frames > 0.0 { frames as u64 } else { 0 };
        player().estimate(core(), n)
    }

    /// How far each pass wanders (`Player::set_wander`).
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_wander(amount: f64) {
        player().set_wander(amount);
    }

    /// The salt on the wander's stream (queue item 92,
    /// `Player::set_wander_salt`): 0, every code's own wander.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_wander_salt(salt: u32) {
        player().set_wander_salt(salt);
    }

    /// Mute layer `layer` (the generator's order: drums, bass, chords,
    /// melody, texture) or bring it back (`Player::set_mute`).
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_mute(layer: u32, muted: u32) {
        if let Some(&l) = crate::generator::LAYERS.get(layer as usize) {
            let l: Layer = l;
            player().set_mute(core(), l, muted != 0);
        }
    }

    /// The tempo, in beats a minute (`Player::set_tempo`).
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_tempo(bpm: f64) {
        player().set_tempo(core(), bpm);
    }

    /// The listener's volume, 0 to 1 (`Player::set_volume`).
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_volume(volume: f64) {
        player().set_volume(core(), volume);
    }

    /// Where the player is: 0 idle, 1 making a loop, 2 drawing its first
    /// pass, 3 playing.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_state() -> u32 {
        match player().state() {
            State::Idle => 0,
            State::Loading => 1,
            State::Wandering => 2,
            State::Playing => 3,
        }
    }

    struct Name(UnsafeCell<[u8; 64]>);
    // Single-threaded, as above.
    unsafe impl Sync for Name {}
    static NAME: Name = Name(UnsafeCell::new([0; 64]));

    /// The name of the loop made (plain ASCII), at `dl_play_text`; returns
    /// its length.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_name() -> u32 {
        // SAFETY: single-threaded, and no export keeps the reference.
        let buf = unsafe { &mut *NAME.0.get() };
        let name = player().generator().spec().name.as_str().as_bytes();
        let n = name.len().min(buf.len());
        buf[..n].copy_from_slice(&name[..n]);
        u32::try_from(n).unwrap_or(0)
    }

    /// Where `dl_play_name` writes.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_text() -> *const u8 {
        NAME.0.get().cast()
    }

    /// The step the player decides next, in its loop.
    #[unsafe(no_mangle)]
    #[allow(clippy::cast_precision_loss)] // a step, far below 2^53
    pub extern "C" fn dl_play_step() -> f64 {
        player().position().0 as f64
    }

    /// When that step falls, in seconds on the core's clock.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_play_time() -> f64 {
        player().position().2
    }
}

/// The measuring tools' exports (queue item 96; the `tools` feature, a build
/// the tools make for themselves and the app never ships): a request as
/// text, as the C host's `dl_player_ask` takes one; one note of one voice
/// through its layer's path (`player::probe`); and what the voice budget
/// was asked and refused, by layer.
#[cfg(feature = "tools")]
mod tools {
    use core::cell::UnsafeCell;

    use super::core;
    use super::player::{player, refused, request};
    use crate::generator::LAYERS;
    use crate::player::Request;
    use crate::player::probe::{Note, Room, VOICES};
    use crate::player::synth::Synth;

    struct Bytes(UnsafeCell<[u8; 1024]>);
    // Single-threaded, as the core's own global.
    unsafe impl Sync for Bytes {}
    static INPUT: Bytes = Bytes(UnsafeCell::new([0; 1024]));

    struct Probe(UnsafeCell<(Room, Synth, bool)>);
    // Single-threaded, as above.
    unsafe impl Sync for Probe {}
    static PROBE: Probe = Probe(UnsafeCell::new((Room::new(), Synth::new(), false)));

    /// Where the host writes a request's text (`dl_tools_ask`), and reads a
    /// voice's name (`dl_tools_voice`).
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_tools_input() -> *mut u8 {
        INPUT.0.get().cast()
    }

    /// Ask the next `dl_play_load` for the request whose text is the first
    /// `length` bytes at `dl_tools_input` (`Request::parse`), in place of
    /// anything asked so far. 1 if it can be met; else 0, the request left
    /// as it was and the refusal noted (`dl_play_refusal`).
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_tools_ask(length: u32) -> u32 {
        // SAFETY: single-threaded; the host wrote the bytes before calling.
        let input = unsafe { &*INPUT.0.get() };
        let bytes = &input[..(length as usize).min(input.len())];
        match core::str::from_utf8(bytes).map(Request::parse) {
            Ok(Ok(r)) => {
                *request() = r;
                1
            }
            Ok(Err(why)) => {
                refused().get_or_insert(why);
                0
            }
            Err(_) => 0,
        }
    }

    /// How many voices the probe knows in `layer` (`probe::VOICES`).
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_tools_voices(layer: u32) -> u32 {
        VOICES.get(layer as usize).map_or(0, |v| u32::try_from(v.len()).unwrap_or(0))
    }

    /// The name of voice `index` of `layer`, written at `dl_tools_input`;
    /// its length, 0 for none.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_tools_voice(layer: u32, index: u32) -> u32 {
        let Some(name) = VOICES.get(layer as usize).and_then(|v| v.get(index as usize)) else { return 0 };
        // SAFETY: single-threaded, and no export keeps the reference.
        let buf = unsafe { &mut *INPUT.0.get() };
        buf[..name.len()].copy_from_slice(name.as_bytes());
        u32::try_from(name.len()).unwrap_or(0)
    }

    /// Hand the core one note of voice `index` of `layer` at `at` seconds,
    /// through the layer's own path (`probe::Room::play`): `midi`, `vel`,
    /// held `seconds`, its draws from `seed`, a sung voice on vowel `vowel`.
    /// The synth is started on the first call. 1 if the voice exists.
    #[unsafe(no_mangle)]
    #[allow(clippy::too_many_arguments)] // the note's own fields
    pub extern "C" fn dl_tools_probe(layer: u32, index: u32, midi: f64, vel: f64, seconds: f64, seed: u32, vowel: u32, at: f64) -> u32 {
        let (Some(&at_layer), Some(&voice)) = (LAYERS.get(layer as usize), VOICES.get(layer as usize).and_then(|v| v.get(index as usize))) else {
            return 0;
        };
        // SAFETY: single-threaded, and no export keeps the reference.
        let (room, synth, started) = unsafe { &mut *PROBE.0.get() };
        let core = core();
        if !*started {
            let rate = core.context().rate;
            synth.start(core, rate, false, 0.0);
            *started = true;
        }
        room.play(core, synth, &Note { layer: at_layer, voice, midi, vel, seconds, seed, vowel: vowel as usize }, at);
        1
    }

    /// The voice calls the player's budget was asked for `layer` (0), those
    /// it refused room (1), and those it let go for newer notes (2), since
    /// the player was attached.
    #[unsafe(no_mangle)]
    pub extern "C" fn dl_tools_budget(layer: u32, which: u32) -> u32 {
        let l = layer as usize;
        if l >= 5 {
            return 0;
        }
        let (counts, stolen) = player().budget_counts();
        match which {
            0 => counts.asked[l],
            1 => counts.refused[l],
            _ => stolen[l],
        }
    }
}
