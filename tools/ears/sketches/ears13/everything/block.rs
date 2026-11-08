//! Ears 13 sampler: padWow, lowBell and colorTone together on the chords
//! channel, each on or off by a constant below (`use.sh` writes the three
//! variants the sampler's loops take). The three sketches' own files sit
//! beside this one in `core/sketch/` while it is built (`lowbell.rs`,
//! `colortone.rs`, `padwow.rs`: the candidates' `block.rs`, unchanged).
//! The wow works on the chords first; the bell and the tone then follow the
//! wowed chords' level and add their own.

use crate::block::{Block, Channel, Context, Event, Probe, Signal, event};

#[path = "lowbell.rs"]
mod bell;
#[path = "colortone.rs"]
mod color;
#[path = "padwow.rs"]
mod wow;

const BELL: bool = true;
const COLOR: bool = true;
const WOW: bool = true;

/// Nothing to set: each part starts at its own rest.
#[derive(Clone, Copy)]
pub struct Settings {
    /// A chord root heard at the start (MIDI; NaN: none).
    pub start: f64,
}

/// At rest.
pub const REST: Settings = Settings { start: f64::NAN };

/// The three.
pub struct Sketch {
    bell: bell::Sketch,
    color: color::Sketch,
    wow: wow::Sketch,
}

impl Sketch {
    /// All zeros.
    #[must_use]
    pub const fn new() -> Self {
        Sketch { bell: bell::Sketch::new(), color: color::Sketch::new(), wow: wow::Sketch::new() }
    }
}

impl Default for Sketch {
    fn default() -> Self {
        Self::new()
    }
}

impl Block for Sketch {
    type Settings = Settings;

    const PROBES: &'static [Probe<Settings>] = &[Probe { name: "all", settings: Settings { start: 48.0 } }];

    const HEARS: &'static [u32] = &[event::RETUNE, event::RELEASE];

    fn start(&mut self, s: &Settings, cx: &Context) {
        self.bell.start(&bell::Settings { start: s.start, ..bell::REST }, cx);
        self.color.start(&color::Settings { start: s.start, ..color::REST }, cx);
        self.wow.start(&wow::REST, cx);
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    fn event(&mut self, e: &Event) {
        if BELL {
            self.bell.event(e);
        }
        if COLOR {
            self.color.event(e);
        }
    }

    fn render(&mut self, block: u64, cx: &Context, signal: &mut Signal) {
        if WOW {
            self.wow.render(block, cx, signal);
        }
        let dry = *signal;
        if BELL {
            self.bell.render(block, cx, signal);
        }
        if COLOR {
            let mut own = dry;
            self.color.render(block, cx, &mut own);
            for ((x, o), d) in signal.iter_mut().zip(own.iter()).zip(dry.iter()) {
                *x += *o - *d;
            }
        }
    }
}

// It works on, and adds to, its channel's whole signal.
impl Channel for Sketch {
    const WHOLE: bool = true;
}
