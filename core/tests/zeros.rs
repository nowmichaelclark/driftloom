//! Queue item 93: a new player is all zero bytes, so `js/dlplayer.wasm` keeps
//! it in `.bss` (no bytes in the file) and loads small. A field whose empty
//! state is not zero (a default number, an `Option` whose `None` Rust writes
//! as a spare value, a `&'static str`) fails here, with where it is.

use dlcore::player::Player;

#[test]
fn a_new_player_is_all_zeros() {
    static NEW: Player = Player::new();
    let n = core::mem::size_of::<Player>();
    // SAFETY: reads the bytes of a static (a constant: its padding is zero
    // too) as a slice of `u8`, as long as the static is.
    let bytes = unsafe { core::slice::from_raw_parts(core::ptr::addr_of!(NEW).cast::<u8>(), n) };
    let first = bytes.iter().position(|&b| b != 0);
    assert_eq!(first, None, "Player::new() has a nonzero byte at offset {first:?} of {n}");
}
