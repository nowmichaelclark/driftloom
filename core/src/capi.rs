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
use crate::player::{Handover, Moment, Player, Request, State};

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

/// A refusal's line as C text (the request's `&'static str` lines,
/// NUL-terminated): plain ASCII, static, never freed.
fn line(refused: &'static str) -> *const c_char {
    use crate::generator::request::{
        ALL_LAYERS_OFF, BAD_LAYER, BAD_LENGTH, BAD_SETTING, BAD_TEMPO, BAD_THEME, BAD_WEIGHT, NO_SUCH_KEY, NO_SUCH_LAYER, NO_SUCH_SCALE,
        NO_SUCH_SETTING, NO_SUCH_STYLE, NO_SUCH_VOICE, NO_SUCH_VIBE, SETTING_OUT, TEMPO_OUT,
    };
    let c: &'static CStr = match refused {
        NO_SUCH_VIBE => c"no such vibe",
        BAD_WEIGHT => c"a bad weight",
        BAD_TEMPO => c"a bad tempo",
        TEMPO_OUT => c"tempo must be 32 to 190 bpm",
        NO_SUCH_SETTING => c"no such setting",
        BAD_SETTING => c"a bad setting",
        SETTING_OUT => c"a setting outside its range",
        BAD_THEME => c"a bad theme",
        BAD_LENGTH => c"a bad length",
        NO_SUCH_KEY => c"no such key",
        NO_SUCH_SCALE => c"no such scale",
        NO_SUCH_STYLE => c"no such style",
        NO_SUCH_LAYER => c"no such layer",
        NO_SUCH_VOICE => c"no such voice",
        BAD_LAYER => c"a bad layer",
        ALL_LAYERS_OFF => c"all layers off",
        _ => c"can't do that",
    };
    c.as_ptr()
}

/// `text` as a string, if it is one (null, or not UTF-8, is not).
///
/// # Safety
///
/// `text` is a NUL-terminated string, or null.
unsafe fn text<'a>(text: *const c_char) -> Option<&'a str> {
    if text.is_null() {
        return None;
    }
    // SAFETY: NUL-terminated (the caller's promise).
    unsafe { CStr::from_ptr(text) }.to_str().ok()
}

/// Add the vibe called `id` at `weight` (the share of the loop it steers:
/// `Request::with_weight`) to what the next `dl_player_load` asks for. Null
/// if it can be met; else the line saying why not (static, plain ASCII),
/// and the request is left as it was.
///
/// # Safety
///
/// `id` is a NUL-terminated string, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dl_player_weight(player: *mut Engine, id: *const c_char, weight: f64) -> *const c_char {
    let Some(e) = engine(player) else { return line("") };
    // SAFETY: the caller's promise, passed on.
    let Some(index) = unsafe { text(id) }.and_then(crate::generator::data::vibe_index) else {
        return line(crate::generator::request::NO_SUCH_VIBE);
    };
    match e.request.with_weight(index, weight) {
        Ok(r) => {
            e.request = r;
            ptr::null()
        }
        Err(why) => line(why),
    }
}

/// Ask the next `dl_player_load` for a tempo: `lo` to `hi` bpm (equal to
/// pin it; `Request::with_tempo`). Null if it can be met; else the line
/// saying why not, and the request is left as it was.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_ask_tempo(player: *mut Engine, lo: f64, hi: f64) -> *const c_char {
    let Some(e) = engine(player) else { return line("") };
    match e.request.with_tempo(lo, hi) {
        Ok(r) => {
            e.request = r;
            ptr::null()
        }
        Err(why) => line(why),
    }
}

/// Pin setting `number` at `value` for the next `dl_player_load` (queue
/// item 85b; `Request::with_pin`, `request::Pins`: a word by its place in
/// the pin's list). Null if it can be met; else the line saying why not,
/// and the request is left as it was.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_pin(player: *mut Engine, number: u32, value: f64) -> *const c_char {
    let Some(e) = engine(player) else { return line("") };
    match e.request.with_pin(number as usize, value) {
        Ok(r) => {
            e.request = r;
            ptr::null()
        }
        Err(why) => line(why),
    }
}

/// Ask the next `dl_player_load` to bend the loop of `seed` (queue item
/// 85d; `Request::with_theme`): the loop made keeps that loop's profile mix,
/// key and mode, metre and tune. Any seed is a theme.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_theme(player: *mut Engine, seed: u32) {
    if let Some(e) = engine(player) {
        e.request = e.request.with_theme(seed);
    }
}

/// Ask the next `dl_player_load` for a piece `bars` bars long, a whole
/// number from 1 up, or for Endless with `bars` infinite (queue item 88;
/// `Request::with_length`, `generator::piece`). Null if it can be met; else
/// the line saying why not, and the request is left as it was.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_length(player: *mut Engine, bars: f64) -> *const c_char {
    let Some(e) = engine(player) else { return line("") };
    let length = if bars == f64::INFINITY { crate::generator::request::Length::Endless } else { crate::generator::request::Length::Bars(bars) };
    match e.request.with_length(length) {
        Ok(r) => {
            e.request = r;
            ptr::null()
        }
        Err(why) => line(why),
    }
}

/// Ask the next `dl_player_load` for the home notes in `keys`, a bit each
/// with C the lowest, or for nothing in particular with 0 (queue item 89;
/// `Request::with_keys`). Null if it can be met; else the line saying why
/// not, and the request is left as it was.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_keys(player: *mut Engine, keys: u32) -> *const c_char {
    let Some(e) = engine(player) else { return line("") };
    match e.request.with_keys(keys) {
        Ok(r) => {
            e.request = r;
            ptr::null()
        }
        Err(why) => line(why),
    }
}

/// Ask the next `dl_player_load` for the modes in `scales`, a bit each by
/// its place in the core's list of modes, or for nothing in particular with
/// 0 (queue item 89; `Request::with_scales`). Null if it can be met; else the
/// line saying why not, and the request is left as it was.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_scales(player: *mut Engine, scales: u32) -> *const c_char {
    let Some(e) = engine(player) else { return line("") };
    match e.request.with_scales(scales) {
        Ok(r) => {
            e.request = r;
            ptr::null()
        }
        Err(why) => line(why),
    }
}

/// Ask the next `dl_player_load` for the profiles in `styles`, a bit each by
/// its place in the frozen order, or for no style in particular with 0
/// (queue item 90; `Request::with_styles`). Null if it can be met; else the
/// line saying why not, and the request is left as it was.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_styles(player: *mut Engine, styles: u32) -> *const c_char {
    let Some(e) = engine(player) else { return line("") };
    match e.request.with_styles(styles) {
        Ok(r) => {
            e.request = r;
            ptr::null()
        }
        Err(why) => line(why),
    }
}

/// Ask the next `dl_player_load` for the voices in `voices` of layer `layer`
/// (0 to 4: drums, bass, chords, melody, texture), a bit each by its place in
/// the layer's list, or for none in particular with 0 (queue item 91;
/// `Request::with_voices`). Null if it can be met; else the line saying why
/// not, and the request is left as it was.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_voices(player: *mut Engine, layer: u32, voices: u32) -> *const c_char {
    let Some(e) = engine(player) else { return line("") };
    match e.request.with_voices(layer as usize, voices) {
        Ok(r) => {
            e.request = r;
            ptr::null()
        }
        Err(why) => line(why),
    }
}

/// Ask the next `dl_player_load` for layer `layer` (0 to 4) to be left to
/// chance (`state` 0), always played (1) or left out (2): queue item 91,
/// `Request::with_layer`. Null if it can be met ("all layers off" for the
/// fifth left out); else the line saying why not, and the request is left as
/// it was.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_layer(player: *mut Engine, layer: u32, state: u32) -> *const c_char {
    use crate::generator::request::{BAD_LAYER, LayerAsk};
    let Some(e) = engine(player) else { return line("") };
    let ask = match state {
        0 => LayerAsk::Chance,
        1 => LayerAsk::On,
        2 => LayerAsk::Off,
        _ => return line(BAD_LAYER),
    };
    match e.request.with_layer(layer as usize, ask) {
        Ok(r) => {
            e.request = r;
            ptr::null()
        }
        Err(why) => line(why),
    }
}

/// Ask the next `dl_player_load` for what `request` says, as text
/// ("glacial=0.7,misty=0.3,bpm=70-85": `Request::parse`), in place of what
/// was asked since the last load. Null if it can be met; else the line
/// saying why not, and the request is left as it was.
///
/// # Safety
///
/// `request` is a NUL-terminated string, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dl_player_ask(player: *mut Engine, request: *const c_char) -> *const c_char {
    let Some(e) = engine(player) else { return line("") };
    // SAFETY: the caller's promise, passed on.
    let Some(t) = (unsafe { text(request) }) else { return line("") };
    match Request::parse(t) {
        Ok(r) => {
            e.request = r;
            ptr::null()
        }
        Err(why) => line(why),
    }
}

/// Ask for the loop of `seed`, with what was asked since the last load
/// (`Player::load`): it is made a stage a block over the blocks rendered
/// next, and starts when made. A loop playing stops being decided now.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_load(player: *mut Engine, seed: u32) {
    if let Some(e) = engine(player) {
        e.player.load(seed, &e.request);
        e.request = Request::new();
    }
}

/// Change to the loop of `seed`, with what was asked since the last load,
/// at `moment` of the loop playing (`DL_NOW`, `DL_BEAT`, `DL_BAR`,
/// `DL_PHRASE`, `DL_PASS`), the old loop crossfading over `crossfade`
/// seconds (`DL_CROSSFADE` the default), or ringing out for a negative one
/// (queue item 86, `Player::change`): no reset, no second core. Several
/// changes may wait, one a kind of moment, the latest for a boundary
/// winning. 1 if it was asked, 0 for a moment with no such number.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_change(player: *mut Engine, seed: u32, moment: u32, crossfade: f64) -> c_int {
    let Some(e) = engine(player) else { return 0 };
    let Some(m) = Moment::of(moment) else { return 0 };
    let handover = if crossfade < 0.0 { Handover::RingOut } else { Handover::Crossfade(crossfade) };
    e.player.change(seed, &e.request, m, handover);
    e.request = Request::new();
    1
}

/// When the next change waiting starts its loop, in seconds on the core's
/// clock (`Player::next_change`; a host lining up a picture with it); -1
/// if none is worked out yet (it is by the next block rendered).
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_next_change(player: *mut Engine) -> f64 {
    engine(player).and_then(|e| e.player.next_change()).unwrap_or(-1.0)
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

/// The frames from the last rendered to the end of the pass playing
/// (`Player::pass_frames`; queue item 87): the chunk a host rendering a pass
/// at a time asks for next. -1 while no loop plays.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_pass_frames(player: *mut Engine) -> i64 {
    engine(player).and_then(|e| e.player.pass_frames(&e.core)).and_then(|n| i64::try_from(n).ok()).unwrap_or(-1)
}

/// How long the next `frames` frames will take to render, in microseconds
/// on the reference host (`Player::estimate`; queue item 87): a host scales
/// it by its own speed, timed on a chunk. -1 for a null player.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_estimate(player: *mut Engine, frames: u64) -> f64 {
    engine(player).map_or(-1.0, |e| e.player.estimate(&e.core, frames))
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

/// The salt on the wander's stream, from the next pass (queue item 92,
/// `Player::set_wander_salt`): a new one each play makes a loop wander
/// differently each time; 0, the default, is every code's own wander.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_wander_salt(player: *mut Engine, salt: u32) {
    if let Some(e) = engine(player) {
        e.player.set_wander_salt(salt);
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

/// Layer `layer`'s seed set to `seed` for the next load, change or re-roll,
/// re-rolled (always played) with `rolled` not 0 (`Request::with_layer_seed`;
/// queue item 94a, a code's edit): null if it can be met, else why not.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_seed(player: *mut Engine, layer: u32, seed: u32, rolled: c_int) -> *const c_char {
    let Some(e) = engine(player) else { return ptr::null() };
    match e.request.with_layer_seed(layer as usize, seed, rolled != 0) {
        Ok(r) => {
            e.request = r;
            ptr::null()
        }
        Err(why) => line(why),
    }
}

/// The loop playing made again with what was asked since the last load, in
/// its place (`Player::revise`; a re-roll).
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_revise(player: *mut Engine, seed: u32) {
    if let Some(e) = engine(player) {
        e.player.revise(seed, &e.request);
        e.request = Request::new();
    }
}

/// The tone from now (`Player::set_tone`).
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_tone(player: *mut Engine, warmth: f64, space: f64, wobble: f64) {
    if let Some(e) = engine(player) {
        e.player.set_tone(&mut e.core, [warmth, space, wobble]);
    }
}

/// How long the track plays (`Player::set_track`): `kind` 0 on and on, 1
/// `count` passes, 2 `count` bars. 1 if it is a kind there is.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_track(player: *mut Engine, kind: u32, count: u32) -> c_int {
    let Some(e) = engine(player) else { return 0 };
    let track = match kind {
        0 => crate::player::Track::Always,
        1 => crate::player::Track::Passes(count.max(1)),
        2 => crate::player::Track::Bars(count.max(1)),
        _ => return 0,
    };
    e.player.set_track(&e.core, track);
    1
}

/// The step from the loop's load the track ends at (`Player::track_end`);
/// -1 while it plays on.
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_track_end(player: *mut Engine) -> i64 {
    engine(player).and_then(|e| e.player.track_end()).and_then(|s| i64::try_from(s).ok()).unwrap_or(-1)
}

/// An event as C sees it (`dl_event`).
#[repr(C)]
pub struct CEvent {
    time: f64,
    kind: u32,
    number: u64,
    step: u64,
}

/// Take the oldest event waiting into `out` (`Player::event`): 1 if one was
/// taken, 0 if none waits.
///
/// # Safety
///
/// `out` is a writable `dl_event`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dl_player_event(player: *mut Engine, out: *mut CEvent) -> c_int {
    let Some(e) = engine(player) else { return 0 };
    if out.is_null() {
        return 0;
    }
    let Some(ev) = e.player.event() else { return 0 };
    // SAFETY: a writable `dl_event` (the caller's promise), checked not null.
    unsafe { out.write(CEvent { time: ev.time, kind: u32::from(ev.kind as u8), number: ev.number, step: ev.step }) };
    1
}

/// Events lost for want of room (`Player::events_lost`).
#[unsafe(no_mangle)]
pub extern "C" fn dl_player_events_lost(player: *mut Engine) -> u32 {
    engine(player).map_or(0, |e| e.player.events_lost())
}

/// Layer `layer`'s grid into `out` (`size` doubles; `Player::grid`).
///
/// # Safety
///
/// `out` is `size` writable doubles, or null with `size` 0.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dl_player_grid(player: *mut Engine, layer: u32, out: *mut f64, size: usize) -> usize {
    let Some(e) = engine(player) else { return 0 };
    let Some(&l) = crate::generator::LAYERS.get(layer as usize) else { return 0 };
    if out.is_null() || size == 0 {
        return 0;
    }
    // SAFETY: `size` writable doubles (the caller's promise), not null.
    let out = unsafe { core::slice::from_raw_parts_mut(out, size) };
    e.player.grid(l, out)
}

/// Counts what a description would write, and copies what fits.
struct Text<'a> {
    out: &'a mut [u8],
    len: usize,
}

impl core::fmt::Write for Text<'_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for &b in s.as_bytes() {
            if let Some(x) = self.out.get_mut(self.len) {
                *x = b;
            }
            self.len += 1;
        }
        Ok(())
    }
}

/// The loop described into `out` (`size` bytes, NUL-terminated, cut short if
/// it must be; `Player::describe`); returns its whole length.
///
/// # Safety
///
/// `out` is `size` writable bytes, or null with `size` 0.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dl_player_describe(player: *mut Engine, out: *mut c_char, size: usize) -> usize {
    let Some(e) = engine(player) else { return 0 };
    let buf: &mut [u8] = if out.is_null() || size == 0 {
        &mut []
    } else {
        // SAFETY: `size` writable bytes (the caller's promise), not null.
        unsafe { core::slice::from_raw_parts_mut(out.cast::<u8>(), size - 1) }
    };
    let mut t = Text { out: buf, len: 0 };
    let _ = e.player.describe(&mut t);
    if !out.is_null() && size > 0 {
        // SAFETY: as above; the last byte is the NUL's.
        unsafe { *out.add(t.len.min(size - 1)) = 0 };
    }
    t.len
}
