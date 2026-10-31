//! Named constants a clip can move (queue item 83), in the `sketch` build
//! only.
//!
//! The brain's and the ears' A/B clips change one number and nothing else:
//! the bass layer's lift, say, at 3 dB and at 5.5. In the app that number is
//! a `const`, and moving it meant editing the source and rebuilding. Here
//! the places that matter read it through [`get`], with the `const` as the
//! default, and `dlrender --set bass.LIFT_DB=3` sets it ([`set`]). With the
//! feature off none of this exists, so the app's build is the same bytes.
//!
//! To make a constant movable: add its name to [`NAMES`], and where the
//! core reads it write `#[cfg(feature = "sketch")] let x = tuning::get(NAME,
//! x);` before the first use. A name is `block.CONSTANT`, ASCII.
//!
//! The values are process-wide: a render sets them before it starts and
//! leaves them alone, so two renders at once with different values would
//! trample each other. Each clip is its own process.

use core::sync::atomic::{AtomicU64, Ordering};

/// The constants that can move: their names, as `--set` spells them. A
/// constant is in this list or it cannot be set.
pub const NAMES: [&str; 1] = ["bass.LIFT_DB"];

/// The bits of NaN, which means "not set" (every real value is a finite
/// number, which `set` insists on).
const UNSET: u64 = 0x7ff8_0000_0000_0000;

static VALUES: [AtomicU64; NAMES.len()] = [const { AtomicU64::new(UNSET) }; NAMES.len()];

/// Move the constant `name` to `value` for the rest of the process. False,
/// and nothing moves, for a name that is not in [`NAMES`] or a value that is
/// not a finite number.
pub fn set(name: &str, value: f64) -> bool {
    let Some(i) = NAMES.iter().position(|n| *n == name) else { return false };
    if !value.is_finite() {
        return false;
    }
    VALUES[i].store(value.to_bits(), Ordering::Relaxed);
    true
}

/// The constant `name`: the value `set` gave it, or `default` if none was
/// (or the name is not in [`NAMES`]).
#[must_use]
pub fn get(name: &str, default: f64) -> f64 {
    let Some(i) = NAMES.iter().position(|n| *n == name) else { return default };
    let bits = VALUES[i].load(Ordering::Relaxed);
    if bits == UNSET { default } else { f64::from_bits(bits) }
}
