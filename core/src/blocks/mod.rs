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

use crate::block::{Block, Channel, Context, Event, Mods, Note, Sends, Signal, Voice};

mod shared;

pub use shared::{Turn, midi_to_freq, retune, slide_of};

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

/// Where a voice plays into: the bus of the channel's own block it plays
/// through (`Voice::THROUGH`), or the channel itself.
fn into<'a>(through: Option<usize>, out: &'a mut Signal, buses: &'a mut [Signal; own::COUNT]) -> &'a mut Signal {
    match through.and_then(|n| buses.get_mut(n)) {
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
    /// A channel's own block: by default, as any other.
    fn channel<C: Channel + Default>(&mut self, name: &'static str) {
        self.part::<C>(name);
    }
}

/// The list. Four kinds of line:
///
/// - `voices`: `file::Type = NAME number, ...;` -- a voice a host plays by
///   these numbers, held in a slot of the core's pool. The numbers are the
///   host's (`js/core.js` sends them), fixed for good. The first line's
///   voice is what a free slot holds, so keep it small.
/// - `pooled`: `file::Type [size] = NAME number, ...;` -- a voice too big for
///   a slot, held in a pool of its own of `size`.
/// - `channel`: `file::Type = NAME (settings), ...;` -- a channel's own
///   block (queue items 46b and 46c, `block::Channel`): one of each name in
///   every channel, started with its settings, outside the voice pool. It
///   runs on a bus its voices sum into (the bodies; a voice plays through
///   one by naming it, `Voice::THROUGH = Some(own::NAME)`), or on the whole
///   channel (`Channel::WHOLE`: a sweep, a duck), or adds its own sound (a
///   source: an organ, a wash). A host reaches it by its number: settings,
///   events, and signals routed from other channels. A shared resonance,
///   space or held instrument is one file and one line here. It may carry
///   `#[cfg(...)]`.
/// - `parts`: `file::Type;` -- a block the voices are built from, which no
///   host plays by number. It may carry `#[cfg(...)]`.
macro_rules! registry {
    // The list as it is written: each `channel` line is given an attribute
    // it may lack (`cfg(all())`, always true), one at a time, so the arm
    // that builds everything holds exactly one a line and can repeat it at
    // every name the line declares.
    (
        voices $v:tt
        pooled $p:tt
        channel { $($c:tt)* }
        parts $q:tt
    ) => {
        registry!(@channel [$v $p $q] [] $($c)*);
    };
    (@channel $lists:tt [$($done:tt)*] #[$a:meta] $m:ident :: $t:ident = $( $n:ident ( $s:expr ) ),+ ; $($rest:tt)*) => {
        registry!(@channel $lists [$($done)* #[$a] $m::$t = $( $n ($s) ),+ ;] $($rest)*);
    };
    (@channel $lists:tt [$($done:tt)*] $m:ident :: $t:ident = $( $n:ident ( $s:expr ) ),+ ; $($rest:tt)*) => {
        registry!(@channel $lists [$($done)* #[cfg(all())] $m::$t = $( $n ($s) ),+ ;] $($rest)*);
    };
    (@channel [$v:tt $p:tt $q:tt] [$($done:tt)*]) => {
        registry!(@build voices $v pooled $p channel { $($done)* } parts $q);
    };
    (@build
        voices { $( $vm:ident :: $vt:ident = $( $vn:ident $vv:literal ),+ ; )* }
        pooled { $( $pm:ident :: $pt:ident [ $ps:expr ] = $( $pn:ident $pv:literal ),+ ; )* }
        channel { $( #[$ca:meta] $cm:ident :: $ct:ident = $( $cn:ident ( $cs:expr ) ),+ ; )* }
        parts { $( $(#[$qa:meta])* $qm:ident :: $qt:ident $(< $qg:tt >)? ; )* }
    ) => {
        $( pub mod $vm; )*
        $( pub mod $pm; )*
        $( #[$ca] pub mod $cm; )*
        $( $(#[$qa])* pub mod $qm; )*

        /// A channel's own blocks, by number: how a host names one
        /// (`Core::set`, `Core::route`), and the bus each runs on, which a
        /// voice names in `Voice::THROUGH`.
        pub mod own {
            #[allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]
            #[repr(usize)]
            enum Number { $($( #[$ca] $cn, )+)* Count }
            $($(
                #[$ca]
                #[doc = concat!("The channel's own `blocks::", stringify!($cm), "::", stringify!($ct), "`, set by `", stringify!($cs), "`.")]
                pub const $cn: usize = Number::$cn as usize;
            )+)*
            /// How many a channel has.
            pub const COUNT: usize = Number::Count as usize;
            /// Whether any works on a whole channel (`Channel::WHOLE`): with
            /// none, the core adds the host's notes and stops the channel's
            /// tail at the silence floor in one pass.
            pub const ANY_WHOLE: bool = {
                #[allow(unused_mut)]
                let mut any = false;
                $(
                    #[$ca]
                    {
                        any = any || <super::$cm::$ct as super::Channel>::WHOLE;
                    }
                )*
                any
            };
        }

        /// One channel's own blocks.
        #[allow(non_snake_case)]
        pub(crate) struct Own {
            $($( #[$ca] $cn: $cm::$ct, )+)*
        }

        impl Own {
            /// Every one at rest, all zeros.
            pub(crate) const fn new() -> Self {
                Own { $($( #[$ca] $cn: $cm::$ct::new(), )+)* }
            }

            /// Start each with its settings.
            pub(crate) fn start(&mut self, cx: &Context) {
                $($( #[$ca] self.$cn.start(&$cs, cx); )+)*
            }

            /// Run each that works on a bus (not `Channel::WHOLE`) on its
            /// bus, in order, with the signals in `mods` driving its
            /// settings, and add it into the channel's signal `out`.
            pub(crate) fn buses(
                &mut self,
                block: u64,
                cx: &Context,
                buses: &mut [Signal; own::COUNT],
                out: &mut Signal,
                mods: &[Mods; own::COUNT],
            ) {
                $($(
                    #[$ca]
                    if !<$cm::$ct as Channel>::WHOLE
                        && let (Some(bus), Some(m)) = (buses.get_mut(own::$cn), mods.get(own::$cn))
                    {
                        self.$cn.render_mod(block, cx, bus, m);
                        for (y, x) in out.iter_mut().zip(bus.iter()) {
                            *y += *x;
                        }
                    }
                )+)*
            }

            /// Run each that works on the whole channel (`Channel::WHOLE`)
            /// on `out`, in order, with the signals in `mods` driving its
            /// settings.
            #[allow(unused_variables)]
            pub(crate) fn whole(&mut self, block: u64, cx: &Context, out: &mut Signal, mods: &[Mods; own::COUNT]) {
                $($(
                    #[$ca]
                    if <$cm::$ct as Channel>::WHOLE
                        && let Some(m) = mods.get(own::$cn)
                    {
                        self.$cn.render_mod(block, cx, out, m);
                    }
                )+)*
            }

            /// Hand each a channel event (`Block::event`).
            pub(crate) fn event(&mut self, event: &Event) {
                $($( #[$ca] <$cm::$ct as Block>::event(&mut self.$cn, event); )+)*
            }

            /// Set setting number `setting` of block number `n`
            /// (`Block::set`): false if there is no such block or setting.
            pub(crate) fn set(&mut self, n: usize, setting: usize, value: f64, time: f64, cx: &Context) -> bool {
                $($(
                    #[$ca]
                    if n == own::$cn {
                        if setting >= <$cm::$ct as Block>::SETTINGS.len() {
                            return false;
                        }
                        <$cm::$ct as Block>::set(&mut self.$cn, setting, value, time, cx);
                        return true;
                    }
                )+)*
                let _ = (setting, value, time, cx);
                false
            }
        }

        /// How many settings a host sets on block number `n` of a channel's
        /// own (`Block::SETTINGS`): zero for no such block.
        pub(crate) fn own_settings(n: usize) -> usize {
            $($(
                #[$ca]
                if n == own::$cn {
                    return <$cm::$ct as Block>::SETTINGS.len();
                }
            )+)*
            0
        }

        /// How many settings a signal can drive on block number `n` of a
        /// channel's own (`Block::MODS`): zero for no such block.
        pub(crate) fn own_mods(n: usize) -> usize {
            $($(
                #[$ca]
                if n == own::$cn {
                    return <$cm::$ct as Block>::MODS.len();
                }
            )+)*
            0
        }

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
        /// its channel's signal `out`, or into the bus there of the channel's
        /// own block it plays through; a voice that sends (`Voice::SENDS`)
        /// adds into `sends` too, and says so in `sent`.
        #[allow(clippy::too_many_arguments)]
        pub(crate) fn render(
            held: &mut Held,
            pools: &mut Pools,
            block: u64,
            cx: &Context,
            out: &mut Signal,
            buses: &mut [Signal; own::COUNT],
            sends: &mut Sends,
            sent: &mut bool,
        ) {
            match held {
                $( Held::$vt(v) => {
                    let to = into(<$vm::$vt as Voice>::THROUGH, out, buses);
                    if <$vm::$vt as Voice>::SENDS {
                        v.render_sends(block, cx, to, sends);
                        *sent = true;
                    } else {
                        v.render(block, cx, to);
                    }
                } )*
                $( Held::$pt(i) => {
                    if let Some(v) = pools.$pm.voices.get_mut(*i) {
                        v.render(block, cx, into(<$pm::$pt as Voice>::THROUGH, out, buses));
                    }
                } )*
            }
        }

        /// Whether the voice in `held` sends some of itself past its channel
        /// (`Voice::SENDS`).
        pub(crate) fn sends(held: &Held) -> bool {
            match held {
                $( Held::$vt(_) => <$vm::$vt as Voice>::SENDS, )*
                $( Held::$pt(_) => <$pm::$pt as Voice>::SENDS, )*
            }
        }

        /// Hand the voice in `held` a channel event (`Block::event`).
        pub(crate) fn event(held: &mut Held, pools: &mut Pools, event: &Event) {
            match held {
                $( Held::$vt(v) => <$vm::$vt as Block>::event(v, event), )*
                $( Held::$pt(i) => {
                    if let Some(v) = pools.$pm.voices.get_mut(*i) {
                        <$pm::$pt as Block>::event(v, event);
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
            $( #[$ca] visit.channel::<$cm::$ct>(stringify!($cm)); )*
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
        dub_stab::DubStab = DUBSTAB 28;
    }
    pooled {
        sung::Sung [sung::POOL] = VOWEL 25, HUM 26, CHOIR 27;
    }
    channel {
        body::Body = FIDDLE_BODY (body::FIDDLE), NYLON_BODY (body::NYLON), ACCORDION_BODY (body::ACCORDION);
        #[cfg(test)] swell::Swell = SWELL (swell::REST);
        #[cfg(test)] chorale::Chorale = CHORALE (chorale::REST);
        #[cfg(test)] duck::Duck = DUCK (duck::REST);
    }
    parts {
        osc::Oscillator;
        filter::Filter;
        noise_source::NoiseSource;
        compressor::Compressor<1>;
        lfo::Lfo;
        harmonics::Harmonics;
        #[cfg(test)] tremolo::Tremolo;
        #[cfg(test)] amp::Amp;
        #[cfg(test)] pan::Pan;
    }
}
