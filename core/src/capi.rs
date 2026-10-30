//! The C face of the core (`include/dlcore.h`; `docs/CORE.md`, promise 4),
//! in a build with the `capi` feature: today the player's calls (queue item
//! 82), so a C host plays a loop from its seed with nothing but the header
//! and the library. `examples/c/seed.c` is such a host, a seed to a WAV,
//! and CI holds it to `dlrender --seed`'s samples.
//!
//! Build the library with
//! `cargo rustc --release --lib --features capi --crate-type staticlib`
//! and link `target/release/libdlcore.a`. The core never allocates: the
//! host hands each player its memory (`dl_player_size` bytes, aligned to
//! `dl_player_align`), and nothing else is kept anywhere, so a host may
//! play as many players as it gives memory for. A player is used from one
//! thread at a time.

use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::{align_of, size_of};
use core::ptr;

use crate::Core;
use crate::generator::{LAYERS, Layer};
use crate::player::{Player, Request, State};

/// What the C library does if the core panics: nothing reachable from these
/// calls can (`lib.rs`), but a `no_std` library must say. It aborts, as C
/// code that cannot go on does.
#[cfg(not(test))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    unsafe extern "C" {
        safe fn abort() -> !;
    }
    abort()
}

/// The unwinder's hook, which the prebuilt `compiler_builtins` names though
/// nothing here unwinds (the library is built with `panic = "abort"`): never
/// called, but a C linker wants it.
#[cfg(not(test))]
#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

/// A `dl_player`: a core, the player on it, and what the next load asks for.
pub struct Engine {
    core: Core,
    player: Player,
    request: Request,
    channels: u32,
}

/// A core as `Core::new` makes one, and a player as `Player::new` does,
/// built when the library is: a new `dl_player` is copied from them into
/// the host's memory, so starting one never builds megabytes on a stack.
/// The core's is all zeros (`lib.rs`), so it costs the library's file
/// nothing; the player's is a megabyte and a half of it.
struct Template<T>(T);

// Never written to: they are only ever copied from.
unsafe impl<T> Sync for Template<T> {}

static CORE: Template<Core> = Template(Core::new());
static PLAYER: Template<Player> = Template(Player::new());

/// The host's player, or `None` for a null pointer.
fn engine<'a>(player: *mut Engine) -> Option<&'a mut Engine> {
    // SAFETY: the host passes what `dl_player_start` returned, used from one
    // thread at a time (the header's promise), or null.
    unsafe { player.as_mut() }
}

/// The bytes a player takes: the memory `dl_player_start` is given.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_size() -> usize {
    size_of::<Engine>()
}

/// The alignment that memory needs.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_align() -> usize {
    align_of::<Engine>()
}

/// Start a player in `memory` (`bytes` long, at least `dl_player_size`,
/// aligned to `dl_player_align`): a core at `rate` Hz rendering `channels`
/// channels (1 mono, 2 stereo), the full mix (`full` not 0) or lite's.
/// Null if the memory is too small or misaligned, or the rate or layout is
/// one the core does not take; the memory is then left as it was.
///
/// # Safety
///
/// `memory` is `bytes` writable bytes the host keeps for the player's life.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dl_player_start(memory: *mut c_void, bytes: usize, rate: f64, channels: u32, full: c_int) -> *mut Engine {
    let p = memory.cast::<Engine>();
    if p.is_null() || bytes < size_of::<Engine>() || !p.is_aligned() {
        return ptr::null_mut();
    }
    if (channels != 1 && channels != 2) || !(crate::MIN_RATE..=crate::MAX_RATE).contains(&rate) {
        return ptr::null_mut();
    }
    // SAFETY: room and alignment checked above, and each field written
    // before the whole is used; the templates are plain data (no pointers
    // but to statics), so their copies are a core and a player of their own.
    unsafe {
        ptr::copy_nonoverlapping(&raw const CORE.0, &raw mut (*p).core, 1);
        ptr::copy_nonoverlapping(&raw const PLAYER.0, &raw mut (*p).player, 1);
        (&raw mut (*p).request).write(Request::new());
        (&raw mut (*p).channels).write(channels);
    }
    let Some(e) = engine(p) else { return ptr::null_mut() };
    match e.player.start(&mut e.core, rate, channels, full != 0) {
        Ok(()) => p,
        Err(_) => ptr::null_mut(),
    }
}

/// Add the vibe called `id` ("glacial", as `data/vibes.txt` names it) to
/// what the next `dl_player_load` asks for. 1 if the core has it, else 0.
///
/// # Safety
///
/// `id` is a NUL-terminated string, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dl_player_vibe(player: *mut Engine, id: *const c_char) -> c_int {
    let Some(e) = engine(player) else { return 0 };
    if id.is_null() {
        return 0;
    }
    // SAFETY: NUL-terminated (the caller's promise).
    let id = unsafe { CStr::from_ptr(id) };
    match id.to_str().ok().and_then(|id| e.request.with_vibe(id)) {
        Some(r) => {
            e.request = r;
            1
        }
        None => 0,
    }
}

/// Ask for the loop of `seed`, with the vibes added since the last load
/// (`Player::load`): it is made a stage a block over the blocks rendered
/// next, and starts when made. A loop playing stops being decided now.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_load(player: *mut Engine, seed: u32) {
    if let Some(e) = engine(player) {
        e.player.load(seed, &e.request);
        e.request = Request::new();
    }
}

/// Make what is left of the loop asked for, now (`Player::ready`): for a
/// host with no deadline (an offline render, a loading screen). The loop
/// starts on the next block.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_ready(player: *mut Engine) {
    if let Some(e) = engine(player) {
        e.player.ready(&mut e.core);
    }
}

/// Render the next `frames` frames into `out`: one pointer for each channel
/// of the layout, each to `frames` floats. Any number of frames a call;
/// the same samples whatever the split. 0, or -1 for a null player or
/// channel (nothing rendered).
///
/// # Safety
///
/// `out` holds a pointer for each channel, each to `frames` writable floats.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dl_player_render(player: *mut Engine, out: *const *mut f32, frames: usize) -> c_int {
    let Some(e) = engine(player) else { return -1 };
    if out.is_null() {
        return -1;
    }
    let mut lanes: [&mut [f32]; 2] = [&mut [], &mut []];
    let n = if e.channels == 2 { 2 } else { 1 };
    for (c, lane) in lanes.iter_mut().enumerate().take(n) {
        // SAFETY: a pointer for each channel (the caller's promise).
        let p = unsafe { *out.add(c) };
        if p.is_null() {
            return -1;
        }
        // SAFETY: `frames` writable floats (the caller's promise).
        *lane = unsafe { core::slice::from_raw_parts_mut(p, frames) };
    }
    match e.player.render(&mut e.core, &mut lanes[..n]) {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

/// Stop deciding (a pause): the loop stays loaded, and what was handed over
/// plays out.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_stop(player: *mut Engine) {
    if let Some(e) = engine(player) {
        e.player.stop();
    }
}

/// Go on with the loop stopped, from where it stood, on the next block.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_resume(player: *mut Engine) {
    if let Some(e) = engine(player) {
        e.player.resume(0.0);
    }
}

/// How far each pass wanders: 0 plays the loop as composed, 1 is the
/// app's default, up to 2.4 (`Player::set_wander`).
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_wander(player: *mut Engine, amount: f64) {
    if let Some(e) = engine(player) {
        e.player.set_wander(amount);
    }
}

/// Mute layer `layer` (`DL_DRUMS` to `DL_TEXTURE`) or bring it back (`muted`
/// 0). It holds, across loads, until brought back.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_mute(player: *mut Engine, layer: u32, muted: c_int) {
    let Some(e) = engine(player) else { return };
    if let Some(&l) = LAYERS.get(layer as usize) {
        let l: Layer = l;
        e.player.set_mute(&mut e.core, l, muted != 0);
    }
}

/// The tempo, in beats a minute, from the next step decided.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_tempo(player: *mut Engine, bpm: f64) {
    if let Some(e) = engine(player) {
        e.player.set_tempo(&mut e.core, bpm);
    }
}

/// The listener's volume, 0 to 1 (0.85 until set).
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_volume(player: *mut Engine, volume: f64) {
    if let Some(e) = engine(player) {
        e.player.set_volume(&mut e.core, volume);
    }
}

/// Where the player is: `DL_IDLE`, `DL_LOADING`, `DL_WANDERING` (its first
/// pass being drawn) or `DL_PLAYING`.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_state(player: *mut Engine) -> u32 {
    match engine(player).map(|e| e.player.state()) {
        None | Some(State::Idle) => 0,
        Some(State::Loading) => 1,
        Some(State::Wandering) => 2,
        Some(State::Playing) => 3,
    }
}

/// The loop's name, plain ASCII, into `out` (`size` bytes, NUL-terminated,
/// cut short if it must be). Returns the name's whole length.
///
/// # Safety
///
/// `out` is `size` writable bytes, or null with `size` 0.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dl_player_name(player: *mut Engine, out: *mut c_char, size: usize) -> usize {
    let Some(e) = engine(player) else { return 0 };
    let name = e.player.generator().spec().name.as_str().as_bytes();
    if !out.is_null() && size > 0 {
        let n = name.len().min(size - 1);
        // SAFETY: `size` writable bytes (the caller's promise), n < size.
        unsafe {
            ptr::copy_nonoverlapping(name.as_ptr().cast::<c_char>(), out, n);
            *out.add(n) = 0;
        }
    }
    name.len()
}

/// Notes the core could not play as asked since the start: dropped (no
/// voice free, or no room), taken late, or left out. 0 in every render the
/// player makes.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_trouble(player: *mut Engine) -> u32 {
    engine(player).map_or(0, |e| e.core.dropped().saturating_add(e.core.late()).saturating_add(e.core.missed()))
}
