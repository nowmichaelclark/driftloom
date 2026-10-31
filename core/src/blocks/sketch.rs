//! The sketch slot (queue item 83): `core/sketch/block.rs` as a channel's
//! own block, in the `sketch` build only.
//!
//! An ears or brain session sketches a sound by dropping one block file into
//! `core/sketch/` (the kit's interface, as any block's: `Block`, and
//! `Channel` for where on the channel it works) and one line of data naming
//! where it plays (`core/sketch/where.txt`). This file is the one fixed line
//! between them and the registry: it includes the sketch's file as a module,
//! registers it as a channel's own block (the registry's `sketch::Sketch`
//! line, which carries `#[cfg(feature = "sketch")]`), and adds one setting in
//! front of the sketch's own, `on`.
//!
//! Every channel holds a copy of every channel block, so the sketch is
//! started on all six; `on` is what makes it play on one. It starts off, so
//! a core with the feature built in renders as one without, and a host turns
//! it on in the channel it is for: `dlrender --sketch` reads `where.txt` and
//! sends `set(channel, SKETCH, on, 1)` before the loop starts. Played alone
//! (`block::solo`: the kit, `dlrender --block`) it is switched on first. The
//! sketch's own settings follow `on`, shifted by one, so its `set` hears its
//! own numbers.
//!
//! The kit plays the sketch like any block (level, tone, cost, determinism,
//! modulation, stereo, events, settings) and does not compare it with a
//! stored render: a sketch changes whenever its session does, and the stored
//! renders (`golden/blocks.txt`) are the app's.

#[path = "../../sketch/block.rs"]
mod user;

use crate::block::{Block, Channel, Context, Event, Mod, Mods, Probe, Setting, Signal, Stereo};

/// The sketch's settings at rest, as the registry line starts it.
pub use user::REST;

type Inner = user::Sketch;

/// The sketch, and whether it plays in this channel.
pub struct Sketch {
    inner: Inner,
    on: bool,
}

impl Sketch {
    /// A sketch at rest, all zeros (off).
    #[must_use]
    pub const fn new() -> Self {
        Sketch { inner: Inner::new(), on: false }
    }
}

impl Default for Sketch {
    fn default() -> Self {
        Self::new()
    }
}

/// The setting number of `on`; the sketch's own follow it.
pub const ON: usize = 0;

const COUNT: usize = <Inner as Block>::SETTINGS.len() + 1;

/// `on`, then the sketch's own settings.
const SETTINGS: [Setting; COUNT] = {
    // `on` is not something a listener hears differ, so the kit leaves it
    // alone (a NaN probe).
    let mut all = [Setting { name: crate::block::SKETCH_ON, probe: f64::NAN }; COUNT];
    let mut i = 1;
    while i < COUNT {
        all[i] = <Inner as Block>::SETTINGS[i - 1];
        i += 1;
    }
    all
};

impl Block for Sketch {
    type Settings = <Inner as Block>::Settings;

    const PROBES: &'static [Probe<Self::Settings>] = <Inner as Block>::PROBES;
    const INPUT: bool = <Inner as Block>::INPUT;
    const MODS: &'static [Mod] = <Inner as Block>::MODS;
    const HEARS: &'static [u32] = <Inner as Block>::HEARS;
    const SETTINGS: &'static [Setting] = &SETTINGS;
    const STEREO: bool = <Inner as Block>::STEREO;

    fn start(&mut self, settings: &Self::Settings, cx: &Context) {
        self.on = false;
        self.inner.start(settings, cx);
    }

    fn span(&self) -> (u64, u64) {
        self.inner.span()
    }

    fn render(&mut self, block: u64, cx: &Context, signal: &mut Signal) {
        if self.on {
            self.inner.render(block, cx, signal);
        }
    }

    fn render_mod(&mut self, block: u64, cx: &Context, signal: &mut Signal, mods: &Mods) {
        if self.on {
            self.inner.render_mod(block, cx, signal, mods);
        }
    }

    fn render_stereo(&mut self, block: u64, cx: &Context, signal: &mut Stereo) {
        if self.on {
            self.inner.render_stereo(block, cx, signal);
        }
    }

    fn event(&mut self, event: &Event) {
        if self.on {
            self.inner.event(event);
        }
    }

    fn set(&mut self, setting: usize, value: f64, time: f64, cx: &Context) {
        match setting.checked_sub(1) {
            None => self.on = value != 0.0,
            Some(own) => self.inner.set(own, value, time, cx),
        }
    }
}

impl Channel for Sketch {
    const WHOLE: bool = <Inner as Channel>::WHOLE;
    const SENDS: bool = <Inner as Channel>::SENDS;

    fn render_sends(&mut self, block: u64, cx: &Context, signal: &mut Signal, mods: &Mods, sends: &mut crate::block::Sends) {
        if self.on {
            self.inner.render_sends(block, cx, signal, mods, sends);
        }
    }

    fn render_side(&mut self, block: u64, cx: &Context, signal: &mut Signal, mods: &Mods, side: &mut Signal) {
        if self.on {
            self.inner.render_side(block, cx, signal, mods, side);
        } else {
            self.render_mod(block, cx, signal, mods);
        }
    }
}
