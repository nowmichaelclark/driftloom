//! The building blocks, and the one list of them (queue item 46).
//!
//! Every block is one file in this directory behind the interface in
//! `block` ([`Block`], and [`Voice`] for a block a host plays by number),
//! and one line in `registry!` below. The line declares the file's module
//! too, so adding a block is exactly one new file plus one line, and the
//! test kit (`kit`) checks it with no test written by hand. `core/README.md`
//! walks through it.
//!
//! From the list the macro writes everything the core needs to find a
//! voice: its numbers, the slot a sounding note is held in, and a `match`
//! for starting, rendering and letting go of each (static dispatch: the
//! same code a hand-written `match` makes, and no `dyn` anywhere).

use crate::block::{Block, Context, Note, Signal, Voice};

mod shared;

pub use shared::{midi_to_freq, slide_of};

#[cfg(test)]
mod kit;

/// A pool of one kind of voice too big for a slot: the voices, and which
/// are taken.
pub(crate) struct Pool<T, const N: usize> {
    busy: [bool; N],
    voices: [T; N],
}

impl<T, const N: usize> Pool<T, N> {
    /// Take a free voice: its place, or `None` if every one is busy.
    fn take(&mut self) -> Option<(usize, &mut T)> {
        let i = self.busy.iter().position(|b| !b)?;
        let flag = self.busy.get_mut(i)?;
        *flag = true;
        Some((i, self.voices.get_mut(i)?))
    }

    /// Let the voice at `i` go.
    fn free(&mut self, i: usize) {
        if let Some(b) = self.busy.get_mut(i) {
            *b = false;
        }
    }

    /// Whether every voice is free.
    #[cfg(test)]
    pub(crate) fn idle(&self) -> bool {
        self.busy.iter().all(|b| !b)
    }
}

/// Where a voice with a body plays into: its body's bus in its channel, or
/// the channel itself.
fn into<'a>(body: Option<usize>, out: &'a mut Signal, buses: &'a mut [Signal; body::KINDS]) -> &'a mut Signal {
    match body.and_then(|kind| buses.get_mut(kind)) {
        Some(bus) => bus,
        None => out,
    }
}

/// What the test kit (and `dlrender --block`) is shown of each block in the
/// list: its type, and its name (its file's).
pub trait Visit {
    /// A voice: played by note.
    fn voice<V: Voice + Default>(&mut self, name: &'static str);
    /// Any other block.
    fn part<B: Block + Default>(&mut self, name: &'static str);
}

/// The list. Three kinds of line:
///
/// - `voices`: `file::Type = NAME number, ...;` -- a voice a host plays by
///   these numbers, held in a slot of the core's pool. The numbers are the
///   host's (`js/core.js` sends them), fixed for good. The first line's
///   voice is what a free slot holds, so keep it small.
/// - `pooled`: `file::Type [size] = NAME number, ...;` -- a voice too big for
///   a slot, held in a pool of its own of `size`.
/// - `parts`: `file::Type;` -- a block the voices are built from, which no
///   host plays by number. It may carry `#[cfg(...)]`.
macro_rules! registry {
    (
        voices { $( $vm:ident :: $vt:ident = $( $vn:ident $vv:literal ),+ ; )* }
        pooled { $( $pm:ident :: $pt:ident [ $ps:expr ] = $( $pn:ident $pv:literal ),+ ; )* }
        parts { $( $(#[$qa:meta])* $qm:ident :: $qt:ident $(< $qg:tt >)? ; )* }
    ) => {
        $( pub mod $vm; )*
        $( pub mod $pm; )*
        $( $(#[$qa])* pub mod $qm; )*

        /// The voices' numbers, as a host plays them (`Core::note`).
        pub mod voices {
            $($(
                #[doc = concat!("Plays `blocks::", stringify!($vm), "::", stringify!($vt), "`.")]
                pub const $vn: u32 = $vv;
            )+)*
            $($(
                #[doc = concat!("Plays `blocks::", stringify!($pm), "::", stringify!($pt), "`.")]
                pub const $pn: u32 = $pv;
            )+)*
        }
        use voices::*;

        /// The highest voice number there is.
        pub const LAST: u32 = {
            let all: &[u32] = &[$($($vv,)+)* $($($pv,)+)*];
            let (mut last, mut i) = (0, 0);
            while i < all.len() {
                if all[i] > last {
                    last = all[i];
                }
                i += 1;
            }
            last
        };

        /// A sounding voice, as a slot of the core's pool holds it: the
        /// voice itself, or a pooled voice's place in its pool.
        ///
        /// A voice lives in a slot of a fixed pool, so its size is the
        /// pool's price and boxing it would allocate; the biggest voice
        /// sets the slot's size. `repr(u8)` keeps the tag first, with the
        /// first voice's tag zero. Without it the compiler hides the other
        /// voices' tags in a spare value of a byte inside the biggest one, a
        /// free slot is no longer all zeros, and the pool is written into
        /// the `.wasm` -- 3 MB, where all zeros cost nothing.
        #[allow(clippy::large_enum_variant)]
        #[repr(u8)]
        pub(crate) enum Held {
            $( $vt($vm::$vt), )*
            $( $pt(usize), )*
        }

        impl Held {
            /// What a free slot holds: the first voice, silent.
            pub(crate) const FREE: Held = registry!(@first $( $vm::$vt )*);
        }

        /// The pools of the voices too big for a slot.
        pub(crate) struct Pools {
            $( $pm: Pool<$pm::$pt, { $ps }>, )*
        }

        impl Pools {
            /// Every pool, free and all zeros.
            // Built in a `const` for the static core, never on a stack at
            // run time.
            #[allow(clippy::large_stack_arrays)]
            pub(crate) const fn new() -> Self {
                Pools {
                    $( $pm: Pool {
                        busy: [false; $ps],
                        voices: [const { $pm::$pt::new() }; $ps],
                    }, )*
                }
            }

            /// Let every pooled voice go.
            pub(crate) fn clear(&mut self) {
                $( self.$pm.busy = [false; $ps]; )*
            }

            /// Whether no pooled voice is sounding.
            #[cfg(test)]
            pub(crate) fn idle(&self) -> bool {
                true $( && self.$pm.idle() )*
            }
        }

        /// Whether `voice` is a voice the core has.
        pub(crate) fn known(voice: u32) -> bool {
            matches!(voice, $( $($vn)|+ )|* $( | $($pn)|+ )*)
        }

        /// Settle what `note` leaves to chance (`Voice::settle`).
        pub(crate) fn settle(note: &mut Note) {
            match note.voice {
                $( $($vn)|+ => <$vm::$vt as Voice>::settle(note), )*
                $( $($pn)|+ => <$pm::$pt as Voice>::settle(note), )*
                _ => {}
            }
        }

        /// Whether `note` plays the noise (`Voice::uses_noise`).
        pub(crate) fn uses_noise(note: &Note) -> bool {
            match note.voice {
                $( $($vn)|+ => <$vm::$vt as Voice>::uses_noise(note), )*
                $( $($pn)|+ => <$pm::$pt as Voice>::uses_noise(note), )*
                _ => false,
            }
        }

        /// Start `note` in `held`, a free slot: its span, or `None` when its
        /// voice's pool is full.
        pub(crate) fn begin(held: &mut Held, pools: &mut Pools, note: &Note, cx: &Context) -> Option<(u64, u64)> {
            Some(match note.voice {
                $( $($vn)|+ => {
                    let mut v = $vm::$vt::new();
                    v.start(note, cx);
                    let span = v.span();
                    *held = Held::$vt(v);
                    span
                } )*
                $( $($pn)|+ => {
                    let (i, v) = pools.$pm.take()?;
                    v.start(note, cx);
                    *held = Held::$pt(i);
                    v.span()
                } )*
                _ => (0, 0),
            })
        }

        /// Render the block from frame `block` of the voice in `held` into
        /// its channel's signal `out`, or into its body's bus there.
        pub(crate) fn render(
            held: &mut Held,
            pools: &mut Pools,
            block: u64,
            cx: &Context,
            out: &mut Signal,
            buses: &mut [Signal; body::KINDS],
        ) {
            match held {
                $( Held::$vt(v) => v.render(block, cx, into(<$vm::$vt as Voice>::BODY, out, buses)), )*
                $( Held::$pt(i) => {
                    if let Some(v) = pools.$pm.voices.get_mut(*i) {
                        v.render(block, cx, into(<$pm::$pt as Voice>::BODY, out, buses));
                    }
                } )*
            }
        }

        /// Let the voice in `held` go: a pooled voice's place is free again.
        pub(crate) fn free(held: &Held, pools: &mut Pools) {
            match held {
                $( Held::$pt(i) => pools.$pm.free(*i), )*
                _ => {}
            }
        }

        /// Show `visit` every block in the list, in order.
        pub fn each(visit: &mut impl Visit) {
            $( visit.voice::<$vm::$vt>(stringify!($vm)); )*
            $( visit.voice::<$pm::$pt>(stringify!($pm)); )*
            $( $(#[$qa])* visit.part::<$qm::$qt $(<$qg>)?>(stringify!($qm)); )*
        }
    };
    (@first $m:ident :: $t:ident $($rest:tt)*) => {
        Held::$t($m::$t::new())
    };
}

registry! {
    voices {
        kalimba::Kalimba = KALIMBA 0;
        fiddle::Fiddle = FIDDLE 1;
        pad::PadNote = PAD 2;
        fm::Fm = FM 3;
        sine::SineNote = SINE 4;
        tubular::Tubular = TUBULAR 5;
        tone::Tone = SOFTPAD 6, ANALOGPAD 7, ANALOGLEAD 8, SAWPLUCK 9, BEEP 10;
        lead::Lead = MOOG 11, WHISTLE 12;
        nylon::Nylon = NYLON 13;
        accordion::Accordion = ACCORDION 14;
        stab::Stab = STAB 15;
        wind::Wind = OCARINA 16, FLUTE 17;
        panflute::Panflute = PANFLUTE 18;
        struck::Struck = KNOCK 19, HAT 21;
        temple_bell::TempleBell = TEMPLEBELL 20;
        bass::Bass = BASS 22;
        texture::Texture = TEXTURE 23;
        drum::Drum = DRUM 24;
    }
    pooled {
        sung::Sung [sung::POOL] = VOWEL 25, HUM 26, CHOIR 27;
    }
    parts {
        osc::Oscillator;
        filter::Biquad;
        noise_source::NoiseSource;
        body::Body;
        compressor::Compressor<1>;
        #[cfg(test)] tremolo::Tremolo;
    }
}
