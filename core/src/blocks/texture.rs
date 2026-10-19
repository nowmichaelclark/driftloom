//! The textures that are not `fm()` calls (queue item 27): `swell`, `drop`,
//! `wind` and `waves`, each its JavaScript twin in `Synth.texture`, call
//! for call. (`bell` and `chime` are `fm()` calls, and the FM voice plays
//! them.) The budget is asked in JavaScript, note by note; the draws (the
//! drop's three, the wind's one and the waves' two) are the voice's own
//! (`Voice::settle`).
//!
//! The note's `parts` say which texture (`SWELL` and the rest); its extra
//! values are those draws, and for the wind its band in Hz (the host's, NaN
//! for the usual one).

// Web Audio's parameters are 32-bit floats, set from the host's 64-bit
// numbers: a voice narrows every value it hands one.
#![allow(clippy::cast_possible_truncation)]

use crate::QUANTUM;
use crate::block::{Block, Context, Mods, NO_EXTRA, Note, Probe, Signal, Voice};
use crate::param::Param;
use crate::rng::salt;

use super::filter::{self, Filter, Kind};
use super::lfo::Lfo;
use super::noise_source::NoiseSource;
use super::osc::{Pitch, TableOsc, frame_at};
use super::shared::{CUTOFF_DEFAULT, FILTER_TAIL, UNITY, midi_to_freq, noise_grain};

/// What a texture note's `parts` say.
pub const SWELL: u32 = 0;
/// A drop: a grain of noise through a ringing bandpass.
pub const DROP: u32 = 1;
/// The wind: looped noise through a band that sways.
pub const WIND: u32 = 2;
/// The waves: noise under a lowpass, swelling and falling away.
pub const WAVES: u32 = 3;

/// Whether a texture plays the noise.
#[must_use]
pub fn uses_noise(kind: u32) -> bool {
    kind != SWELL
}

/// Where the wind's band sits unless the loop names one.
const WIND_BAND: f32 = 620.0;
/// How far the wind's slow sine moves its band, in Hz.
const WIND_SWING: f32 = 280.0;
const WIND_BREATH: f64 = 0.6;
/// The waves' cutoff, and their level (`WAVES_CUTOFF`, `WAVES_LEVEL`).
const WAVES_CUTOFF: f64 = 650.0;
const WAVES_LEVEL: f64 = 0.67;
/// The waves' two lowpass filters' Q, in dB.
const WAVES_Q: f32 = -3.0;
/// The drop's bandpass Q, and the wind's.
const DROP_Q: f32 = 9.0;
const WIND_Q: f32 = 1.1;

/// A texture note: one of the four textures, by its `parts`.
pub struct Texture {
    kind: u32,
    /// The swell's sine.
    osc: TableOsc,
    freq: f32,
    /// The wind's slow sine, into its band (its filter's `FREQ`).
    sway: Lfo<0>,
    src: NoiseSource,
    filter: Filter,
    /// The waves' second lowpass.
    second: Filter,
    cutoff: Param,
    gain: Param,
    start: u64,
    end: u64,
}

impl Texture {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`;
    /// starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        Texture {
            kind: SWELL,
            osc: TableOsc::new(),
            freq: 0.0,
            sway: Lfo::new(),
            src: NoiseSource::new(),
            filter: Filter::new(),
            second: Filter::new(),
            cutoff: Param::new(0.0),
            gain: Param::new(0.0),
            start: 0,
            end: 0,
        }
    }

    /// `draws` are the voice's: the drop's rate, offset and band; the
    /// wind's slow sine's rate (and the loop's band in Hz, NaN if it names
    /// none); the waves' rate and offset.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn play(
        &mut self,
        kind: u32,
        midi: f64,
        time: f64,
        dur: f64,
        vel: f64,
        draws: [f64; 3],
        noise: usize,
        rate: f64,
    ) {
        self.kind = kind;
        let r = rate as f32;
        // A cutoff that is all envelope goes into the filter's frequency.
        let swept = |kind, q| filter::Settings { kind, freq: 0.0, q, gain: 0.0 };
        let g = &mut self.gain;
        match kind {
            SWELL => {
                // One sine, up and down in a line.
                self.freq = midi_to_freq(midi) as f32;
                g.reset(UNITY);
                g.set_value_at_time(0.0001, time);
                g.linear_ramp_to_value_at_time((vel * 0.3) as f32, time + dur * 0.45, time);
                g.linear_ramp_to_value_at_time(0.0001, time + dur, time);
                self.osc.schedule(time, time + dur + 0.2, rate);
                self.start = self.osc.start_frame();
                self.end = self.osc.stop_frame();
            }
            DROP => {
                noise_grain(&mut self.src, time, 0.12, (draws[0], draws[1]), noise, rate);
                let c = &mut self.cutoff;
                c.reset(CUTOFF_DEFAULT);
                c.set_value_at_time((2400.0 + draws[2] * 3000.0) as f32, time);
                self.filter.set(swept(Kind::Bandpass, DROP_Q), r);
                // Its gain holds its first value ahead of the event (item 26b).
                g.reset_at(0.0001, rate);
                g.set_value_at_time(0.0001, time);
                g.exponential_ramp_to_value_at_time((vel * 0.3) as f32, time + 0.004, time);
                g.exponential_ramp_to_value_at_time(0.0001, time + 0.12, time);
                self.start = self.src.start_frame();
                self.end = self.src.end_frame() + frame_at(FILTER_TAIL, rate);
            }
            WIND => {
                let stop = time + dur + 1.0;
                self.src
                    .looped(time, stop, WIND_BREATH, noise, rate);
                let band = if draws[1].is_finite() {
                    draws[1] as f32
                } else {
                    WIND_BAND
                };
                let settings = filter::Settings { kind: Kind::Bandpass, freq: band, q: WIND_Q, gain: 0.0 };
                self.filter.set(settings, r);
                self.sway.set((0.07 + draws[0] * 0.06) as f32, WIND_SWING, time, stop, rate);
                g.reset_at(0.0001, rate);
                g.set_value_at_time(0.0001, time);
                g.linear_ramp_to_value_at_time(
                    (vel * 0.32) as f32,
                    time + (dur * 0.4).max(0.8),
                    time,
                );
                g.set_target_at_time(0.0001, time + dur * 0.75, (dur * 0.2).max(0.4));
                self.start = self.src.start_frame().min(self.sway.start_frame());
                self.end = self.src.end_frame() + frame_at(FILTER_TAIL, rate);
            }
            _ => {
                self.src.looped_at(
                    time,
                    draws[1] * 1.5,
                    time + dur + 0.1,
                    0.5 + draws[0] * 0.1,
                    noise,
                    rate,
                );
                let crest = time + dur * 0.4;
                let c = &mut self.cutoff;
                c.reset(CUTOFF_DEFAULT);
                c.set_value_at_time((WAVES_CUTOFF * 0.7) as f32, time);
                c.linear_ramp_to_value_at_time(WAVES_CUTOFF as f32, crest, time);
                c.linear_ramp_to_value_at_time((WAVES_CUTOFF * 0.7) as f32, time + dur, time);
                self.filter.set(swept(Kind::Lowpass, WAVES_Q), r);
                self.second.set(swept(Kind::Lowpass, WAVES_Q), r);
                g.reset_at(0.0001, rate);
                g.set_value_at_time(0.0001, time);
                g.linear_ramp_to_value_at_time((vel * WAVES_LEVEL) as f32, crest, time);
                g.linear_ramp_to_value_at_time(0.0001, time + dur, time);
                self.start = self.src.start_frame();
                self.end = self.src.end_frame() + frame_at(FILTER_TAIL, rate);
            }
        }
    }

    fn start_frame(&self) -> u64 {
        self.start
    }

    fn end_frame(&self) -> u64 {
        self.end
    }
}

impl Block for Texture {
    type Settings = Note;

    const PROBES: &'static [Probe<Note>] = &[
        Probe { name: "swell", settings: Note::probe(crate::TEXTURE, crate::TEXTURE_CHANNEL, 79.0, SWELL, NO_EXTRA) },
        Probe { name: "drop", settings: Note::probe(crate::TEXTURE, crate::TEXTURE_CHANNEL, 79.0, DROP, NO_EXTRA) },
        Probe { name: "wind", settings: Note::probe(crate::TEXTURE, crate::TEXTURE_CHANNEL, 79.0, WIND, NO_EXTRA) },
        Probe { name: "waves", settings: Note::probe(crate::TEXTURE, crate::TEXTURE_CHANNEL, 79.0, WAVES, NO_EXTRA) },
    ];

    fn start(&mut self, note: &Note, cx: &Context) {
        let [a, b, c, ..] = note.extra;
        let (noise, rate) = (cx.noise.len(), cx.rate);
        self.play(note.parts, note.midi, note.time, note.dur, note.vel, [a, b, c], noise, rate);
    }

    fn span(&self) -> (u64, u64) {
        (self.start_frame(), self.end_frame())
    }

    fn render(&mut self, block: u64, cx: &Context, out: &mut Signal) {
        let (rate, waves, noise) = (cx.rate, cx.waves, cx.noise);
        let r = rate as f32;
        let mut gain = [0.0f32; QUANTUM];
        self.gain.fill(block, rate, &mut gain);
        match self.kind {
            SWELL => {
                let steady = Pitch::Steady {
                    freq: self.freq,
                    detune: 0.0,
                };
                let mut tone = [0.0f32; QUANTUM];
                let (lo, hi) = self.osc.render(block, &waves.sine, &steady, r, &mut tone);
                for ((y, x), g) in out
                    .iter_mut()
                    .zip(tone.iter())
                    .zip(gain.iter())
                    .take(hi)
                    .skip(lo)
                {
                    *y += *x * *g;
                }
            }
            DROP => {
                let mut x = [0.0f32; QUANTUM];
                self.src.fill(block, noise, &mut x);
                let mut cutoff = [0.0f32; QUANTUM];
                self.cutoff.fill(block, rate, &mut cutoff);
                self.filter.render_mod(block, cx, &mut x, &Mods::NONE.with(filter::FREQ, &cutoff));
                add(out, &x, &gain);
            }
            WIND => {
                let mut x = [0.0f32; QUANTUM];
                self.src.fill(block, noise, &mut x);
                // The band's own value and the slow sine through its depth,
                // summed at every frame as an AudioParam sums its input.
                let mut sway = [0.0f32; QUANTUM];
                self.sway.render(block, cx, &mut sway);
                self.filter.render_mod(block, cx, &mut x, &Mods::NONE.with(filter::FREQ, &sway));
                add(out, &x, &gain);
            }
            _ => {
                let mut x = [0.0f32; QUANTUM];
                self.src.fill(block, noise, &mut x);
                let mut cutoff = [0.0f32; QUANTUM];
                self.cutoff.fill(block, rate, &mut cutoff);
                let swept = Mods::NONE.with(filter::FREQ, &cutoff);
                self.filter.render_mod(block, cx, &mut x, &swept);
                self.second.render_mod(block, cx, &mut x, &swept);
                add(out, &x, &gain);
            }
        }
    }
}

/// `x` under `gain`, added into `out`.
fn add(out: &mut Signal, x: &Signal, gain: &Signal) {
    for ((y, v), g) in out.iter_mut().zip(x.iter()).zip(gain.iter()) {
        *y += *v * *g;
    }
}

impl Voice for Texture {
    /// The drop's rate, offset and band; the wind's slow sine's rate (its
    /// band is the host's); the waves' rate and offset.
    fn settle(note: &mut Note) {
        let d = note.draws;
        let x = &mut note.extra;
        match note.parts {
            DROP => {
                x[0] = d.unit(salt::GRAIN, 0);
                x[1] = d.unit(salt::GRAIN, 1);
                x[2] = d.unit(salt::GRAIN, 2);
            }
            WIND => x[0] = d.unit(salt::TEXTURE_RATE, 0),
            WAVES => {
                x[0] = d.unit(salt::GRAIN, 0);
                x[1] = d.unit(salt::GRAIN, 1);
            }
            _ => {}
        }
    }

    fn uses_noise(note: &Note) -> bool {
        uses_noise(note.parts)
    }
}

impl Default for Texture {
    fn default() -> Self {
        Self::new()
    }
}
