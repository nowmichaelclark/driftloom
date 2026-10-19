//! The folk voices' bodies: a processor block.
//!
//! A body is an instrument's fixed resonances (`BODIES` in synth.js): one
//! chain of filters per kind and channel, shared by every note that voice
//! plays into the channel, as `_body()` builds them. Notes sum into their
//! body's bus; the body runs on the sum (`Voice::THROUGH`), as one of a channel's own blocks (`blocks::own`, the registry's `channel` list). The fiddle, the
//! nylon guitar and the accordion play through one.

use crate::block::{Block, Channel, Context, Probe, Signal};

use super::filter::{Biquad, Kind};

/// The kinds of body, by number (a body's settings).
pub const FIDDLE: usize = 0;
/// The guitar's.
pub const NYLON: usize = 1;
/// The accordion's.
pub const ACCORDION: usize = 2;
/// How many kinds there are.
pub const KINDS: usize = 3;

/// The most filters a body has.
const MOST: usize = 3;

/// `BODIES` in synth.js: [type, Hz, Q, dB]. A lowpass's Q is in dB, so -3
/// is the plain, unresonant slope.
const BODIES: [&[(Kind, f32, f32, f32)]; KINDS] = [
    // The violin's wood around 300 Hz, a bridge hill at 2.7 kHz, the fizz
    // above rolled away.
    &[
        (Kind::Peaking, 300.0, 1.2, 5.0),
        (Kind::Peaking, 2700.0, 0.9, 2.0),
        (Kind::Lowpass, 4500.0, -3.0, 0.0),
    ],
    // The guitar's air and top-plate resonances.
    &[
        (Kind::Peaking, 110.0, 1.0, 3.0),
        (Kind::Peaking, 230.0, 1.2, 2.0),
        (Kind::Lowpass, 3500.0, -3.0, 0.0),
    ],
    // The reed's milder formant, under a lowpass near 3 kHz.
    &[
        (Kind::Peaking, 1250.0, 0.9, 3.0),
        (Kind::Lowpass, 2800.0, -3.0, 0.0),
    ],
];

/// One body: its filters, in order.
pub struct Body {
    filters: [Biquad; MOST],
    len: usize,
}

impl Body {
    /// A body with no filters, all zeros.
    #[must_use]
    pub const fn new() -> Self {
        Body {
            filters: [Biquad::new(); MOST],
            len: 0,
        }
    }

    /// At rest, as the kind `kind` at `rate`.
    pub fn set(&mut self, kind: usize, rate: f32) {
        let spec = BODIES.get(kind).copied().unwrap_or(&[]);
        self.len = spec.len().min(MOST);
        for (f, &(k, freq, q, gain)) in self.filters.iter_mut().zip(spec.iter()) {
            f.reset();
            f.set(k, freq, q, gain, rate);
        }
    }
}

impl Block for Body {
    /// Which body (`FIDDLE`, `NYLON`, `ACCORDION`).
    type Settings = usize;

    const PROBES: &'static [Probe<usize>] = &[
        Probe { name: "fiddle", settings: FIDDLE },
        Probe { name: "nylon", settings: NYLON },
        Probe { name: "accordion", settings: ACCORDION },
    ];

    const INPUT: bool = true;

    /// At rest, as the kind `kind`.
    // Web Audio's filter parameters are 32-bit floats.
    #[allow(clippy::cast_possible_truncation)]
    fn start(&mut self, kind: &usize, cx: &Context) {
        self.set(*kind, cx.rate as f32);
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    /// The bus, through the body. A body with nothing in it and nothing
    /// ringing is skipped: zeros through filters at rest are zeros, so this
    /// changes no sample, and a channel's three bodies cost nothing while
    /// their voices are not playing.
    fn render(&mut self, _: u64, _: &Context, bus: &mut Signal) {
        let filters = self.filters.get_mut(..self.len).unwrap_or(&mut []);
        if filters.iter().all(Biquad::at_rest) && bus.iter().all(|&x| x == 0.0) {
            return;
        }
        for x in bus.iter_mut() {
            let mut v = *x;
            for f in filters.iter_mut() {
                v = f.step(v);
            }
            *x = v;
        }
        for f in filters.iter_mut() {
            f.flush();
        }
    }
}

// It runs on its bus: the notes of its kind in its channel.
impl Channel for Body {}

impl Default for Body {
    fn default() -> Self {
        Self::new()
    }
}
