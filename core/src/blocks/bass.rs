//! The bass voices (queue item 27): `sub`, `round`, `fifths`, `pluckbass`
//! and `moogbass`, each its JavaScript twin in `Synth.bass`, call for call.
//! (`rhodesbass` is an `fm()` call, and the FM voice plays it.)
//!
//! They are one shape with different parts: an oscillator (a sawtooth or a
//! triangle) through a resonant lowpass whose cutoff falls, under an
//! envelope; for `sub` a sine of weight underneath it; for the plucked one
//! a click of bandpassed noise instead; and for `fifths` no filter at all,
//! just two sines. The only draws are the click's two, which the voice
//! settles from the note's key (`Voice::settle`).
//!
//! Every one adds its soft harmonics (`harmonics`, queue item 56's
//! roundBass): the 2nd, 3rd and 4th partials under a 900 Hz lowpass, so the
//! note reads on a phone.
//!
//! The note's `parts` are the voice (`SUB` and the rest) with the flags
//! `GLIDE` and `CHUG`; its first two extra values, the pluck's click's
//! rate and offset draws; its third, the loop's bass lift in dB
//! (`LIFT_SLOT`), which the core stamps from `Core::bass_lift`.

// Web Audio's parameters are 32-bit floats, set from the host's 64-bit
// numbers: a voice narrows every value it hands one.
#![allow(clippy::cast_possible_truncation)]
// The names follow synth.js's (f, g, c...), so the port reads against it.
#![allow(clippy::many_single_char_names)]
// A note's setup is one JavaScript function, ported call for call: split,
// it would no longer read against its twin.
#![allow(clippy::too_many_lines)]

use crate::QUANTUM;
use crate::math;
use crate::block::{Block, Context, Mods, NO_EXTRA, Note, Probe, Signal, Voice};
use crate::param::Param;
use crate::rng::salt;
use crate::wave::Waves;

use super::filter::{self, Biquad, Filter, Kind};
use super::harmonics::{self, Harmonics};
use super::noise_source::NoiseSource;
use super::osc::{Part, TableOsc, frame_at};
use super::shared::{
    CUTOFF_DEFAULT, FILTER_TAIL, Q_DEFAULT, UNITY, midi_to_freq, noise_grain, pitch_of,
    starts_in,
};

/// What a bass note's `parts` say: the voice in the low three bits, and
/// whether it glides in and whether it is a chug in two flags.
pub const SUB: u32 = 0;
/// `round`: a triangle, rounder.
pub const ROUND: u32 = 1;
/// `fifths`: two sines, a fifth apart, unfiltered.
pub const FIFTHS: u32 = 2;
/// `pluckbass`: with a click of noise.
pub const PLUCKBASS: u32 = 3;
/// `moogbass`.
pub const MOOGBASS: u32 = 4;
/// The bits of `parts` that say the voice.
pub const KIND: u32 = 7;
/// The flag for a note that glides in.
pub const GLIDE: u32 = 8;
/// The flag for a chug.
pub const CHUG: u32 = 16;

/// The note's extra value that holds the loop's bass lift, in dB (item 76c).
/// The core stamps it on each bass note as the note arrives (`Core::note`),
/// from the setting the host sent for the loop; a note without one (the kit's
/// probe, a golden list) reads it as 0 dB, today's level.
pub const LIFT_SLOT: usize = 2;

/// The trims, each voice's measured level (`TRIM` in `bass()`).
fn trim(kind: u32) -> f64 {
    match kind {
        SUB | FIFTHS => 0.55,
        ROUND => 0.75,
        PLUCKBASS => 0.72,
        _ => 1.0,
    }
}

/// roundBass's trims (queue item 56), in dB, each voice's: the harmonics
/// add 3 to 7 dB by ear (K-weighted, across the register), so the whole
/// note, harmonics and all, comes down by that much and the bass layer
/// keeps today's level; `REGISTER_DB` takes back what the higher register
/// adds on top, and the loop's lift (`LIFT_SLOT`) lifts them all. The harmonics keep the
/// prototype's balance against the note, so the timbre is what Mikey heard;
/// the prototype played the bass about 6 dB louder by ear than today. Mikey
/// heard the two side by side (Session 26's album) and chose +3 dB in loops
/// with a kick, today's level in loops without (item 76c); the host decides
/// which, per loop.
fn round_trim(kind: u32, lift_db: f64) -> f64 {
    let db = match kind {
        SUB => -5.5,
        ROUND => -4.8,
        FIFTHS => -3.6,
        PLUCKBASS => -3.2,
        _ => -6.7,
    } + REGISTER_DB
        + lift_db;
    math::pow(10.0, db / 20.0)
}

/// What the higher register (36-57, from 28-52) adds by ear on the layer,
/// taken back, in dB.
const REGISTER_DB: f64 = -1.0;

/// A sine, its frequency set once, under its own gain.
struct Sine {
    osc: TableOsc,
    pitch: Param,
    gain: Param,
}

impl Sine {
    const fn new() -> Self {
        Sine {
            osc: TableOsc::new(),
            pitch: Param::new(0.0),
            gain: Param::new(0.0),
        }
    }

    fn render(&mut self, block: u64, rate: f64, waves: &Waves, out: &mut Signal) {
        let mut freq = [0.0f32; QUANTUM];
        let first = starts_in(self.osc.start_frame(), block);
        let pitch = pitch_of(
            &mut self.pitch,
            block,
            rate,
            first,
            &mut freq,
            Part::Steady(0.0),
        );
        let mut tone = [0.0f32; QUANTUM];
        let (lo, hi) = self
            .osc
            .render(block, &waves.sine, &pitch, rate as f32, &mut tone);
        let mut gain = [0.0f32; QUANTUM];
        self.gain.fill(block, rate, &mut gain);
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
}

/// A bass note: one of the five bass voices, by its `parts`.
pub struct Bass {
    /// The voice, as `parts` names it.
    kind: u32,
    /// The main oscillator, filter and envelope: every voice but `fifths`.
    main: bool,
    triangle: bool,
    osc: TableOsc,
    pitch: Param,
    cutoff: Param,
    q: f32,
    filter: Filter,
    gain: Param,
    /// The sine underneath (`sub`, `pluckbass`), or both of `fifths`'.
    sines: [Sine; 2],
    sine_count: usize,
    /// The pluck's click.
    click: bool,
    src: NoiseSource,
    click_filter: Biquad,
    click_gain: Param,
    /// The soft harmonics every bass note adds (roundBass).
    harmonics: Harmonics,
    start: u64,
    end: u64,
}

impl Bass {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`;
    /// starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        Bass {
            kind: SUB,
            main: false,
            triangle: false,
            osc: TableOsc::new(),
            pitch: Param::new(0.0),
            cutoff: Param::new(0.0),
            q: 0.0,
            filter: Filter::new(),
            gain: Param::new(0.0),
            sines: [const { Sine::new() }; 2],
            sine_count: 0,
            click: false,
            src: NoiseSource::new(),
            click_filter: Biquad::new(),
            click_gain: Param::new(0.0),
            harmonics: Harmonics::new(),
            start: 0,
            end: 0,
        }
    }

    /// `draws` are the click's `_noiseSource` rate and offset (pluckbass
    /// only); `noise` is the length of the host's noise.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn play(
        &mut self,
        parts: u32,
        midi: f64,
        time: f64,
        dur: f64,
        vel: f64,
        draws: (f64, f64),
        noise: usize,
        lift_db: f64,
        rate: f64,
    ) {
        let kind = parts & KIND;
        let glide = parts & GLIDE != 0;
        let chug = parts & CHUG != 0;
        self.kind = kind;
        let f = midi_to_freq(midi);
        let vel = vel * trim(kind) * round_trim(kind, lift_db);
        let stop = time + dur + 0.4;
        // A chug note lets go in under half the time.
        let let_go = |release: f64| if chug { release * 0.45 } else { release };

        self.main = kind != FIFTHS;
        self.click = kind == PLUCKBASS;
        self.sine_count = 0;
        let mut ends = 0;
        let mut starts = u64::MAX;

        if self.main {
            self.triangle = kind == ROUND || kind == PLUCKBASS;
            // The pitch, gliding in from two thirds if asked. It rests at
            // its first frequency, not Web Audio's 440 Hz (queue item 56):
            // in the block a note starts in its moving pitch is read from
            // the block's start (Chromium's rule, see `osc`), so the note
            // began with a blip of 440 Hz as long as it happened to fall
            // into its block, under an envelope still near silence. That
            // left the note's phase to chance, and with it how its
            // overtones meet the harmonics', which moved a note's level by
            // a dB or two with where it landed (and so with the rate).
            let p = &mut self.pitch;
            p.reset(if glide { (f * 0.66) as f32 } else { f as f32 });
            if glide {
                p.set_value_at_time((f * 0.66) as f32, time);
                p.exponential_ramp_to_value_at_time(f as f32, time + 0.08, time);
            } else {
                p.set_value_at_time(f as f32, time);
            }
            // The filter, and the envelope's peak, sustain and release.
            let c = &mut self.cutoff;
            c.reset(CUTOFF_DEFAULT);
            let (peak, sustain, release) = match kind {
                PLUCKBASS => {
                    self.q = 2.0;
                    c.set_value_at_time((f * 9.0).min(2600.0) as f32, time);
                    c.exponential_ramp_to_value_at_time(
                        (f * 2.0).max(130.0) as f32,
                        time + 0.22,
                        time,
                    );
                    (vel * 0.42, None, let_go(0.1))
                }
                ROUND => {
                    self.q = 0.9;
                    c.set_value_at_time((f * 6.0).min(2200.0) as f32, time);
                    c.exponential_ramp_to_value_at_time(
                        (f * 2.4).max(140.0) as f32,
                        time + dur.min(0.5),
                        time,
                    );
                    (vel * 0.34, Some(vel * 0.22), let_go(0.08))
                }
                MOOGBASS => {
                    self.q = 9.0;
                    c.set_value_at_time((f * 11.0).min(3600.0) as f32, time);
                    c.exponential_ramp_to_value_at_time(
                        (f * 2.2).max(150.0) as f32,
                        time + dur.min(0.35),
                        time,
                    );
                    (vel * 0.26, Some(vel * 0.15), let_go(0.07))
                }
                _ => {
                    self.q = 3.0;
                    c.set_value_at_time((f * 10.0).min(4200.0) as f32, time);
                    c.exponential_ramp_to_value_at_time(
                        (f * 2.2).max(120.0) as f32,
                        time + dur.min(0.4),
                        time,
                    );
                    (vel * 0.17, Some(vel * 0.11), let_go(0.06))
                }
            };
            // `env()`: strike, settle to the sustain, let go at the end. The
            // pluck has its own: a faster strike, and it lets go early.
            let g = &mut self.gain;
            g.reset_at(UNITY, rate);
            g.set_value_at_time(0.0001, time);
            if kind == PLUCKBASS {
                g.exponential_ramp_to_value_at_time(peak as f32, time + 0.006, time);
                g.set_target_at_time(0.0001, time + dur.min(0.3), release);
            } else {
                g.exponential_ramp_to_value_at_time(peak.max(0.001) as f32, time + 0.014, time);
                if let Some(level) = sustain {
                    g.set_target_at_time(level as f32, time + 0.05, 0.25);
                }
                g.set_target_at_time(0.0001, time + dur, release);
            }
            // The cutoff is all envelope, into the filter's frequency.
            self.filter.set(filter::Settings { kind: Kind::Lowpass, freq: 0.0, q: self.q, gain: 0.0 }, rate as f32);
            self.osc.schedule(time, stop, rate);
            starts = starts.min(self.osc.start_frame());
            ends = ends.max(self.osc.stop_frame() + frame_at(FILTER_TAIL, rate));

            // The sine under it: the weight of the note.
            if kind == SUB {
                let s = &mut self.sines[0];
                s.pitch.reset(f as f32);
                s.pitch.set_value_at_time(f as f32, time);
                let g = &mut s.gain;
                g.reset_at(UNITY, rate);
                g.set_value_at_time(0.0001, time);
                g.exponential_ramp_to_value_at_time((vel * 0.13) as f32, time + 0.015, time);
                g.set_target_at_time(0.0001, time + dur, let_go(0.08));
                s.osc.schedule(time, stop, rate);
                self.sine_count = 1;
                ends = ends.max(s.osc.stop_frame());
            }
        } else {
            // Root and fifth, sines only.
            for (s, (mult, level)) in self.sines.iter_mut().zip([(1.0, 0.3), (1.5, 0.14)]) {
                s.pitch.reset((f * mult) as f32);
                s.pitch.set_value_at_time((f * mult) as f32, time);
                let g = &mut s.gain;
                g.reset_at(UNITY, rate);
                g.set_value_at_time(0.0001, time);
                g.linear_ramp_to_value_at_time((vel * level) as f32, time + (dur * 0.3).min(0.4), time);
                g.set_target_at_time(0.0001, time + dur * 0.8, 0.2);
                s.osc.schedule(time, stop + 0.4, rate);
                starts = starts.min(s.osc.start_frame());
                ends = ends.max(s.osc.stop_frame());
            }
            self.sine_count = 2;
        }

        if self.click {
            // A little noise for the finger: a grain through a bandpass.
            noise_grain(&mut self.src, time, 0.02, draws, noise, rate);
            self.click_filter.reset();
            self.click_filter
                .set(Kind::Bandpass, 900.0, Q_DEFAULT, 0.0, rate as f32);
            // Its gain is already at its first value when the noise begins:
            // a source can start a frame ahead of the event (item 26b).
            let g = &mut self.click_gain;
            g.reset_at((vel * 0.1) as f32, rate);
            g.set_value_at_time((vel * 0.1) as f32, time);
            g.exponential_ramp_to_value_at_time(0.0001, time + 0.025, time);
            starts = starts.min(self.src.start_frame());
            ends = ends.max(self.src.end_frame() + frame_at(FILTER_TAIL, rate));
        }
        // The harmonics, softly, under every voice (queue item 56).
        self.harmonics.set(
            &harmonics::Settings { midi, time, dur, vel, release: let_go(0.09), stop },
            rate,
        );
        starts = starts.min(self.harmonics.span().0);
        ends = ends.max(self.harmonics.span().1);
        self.start = starts;
        self.end = ends;
    }


    fn start_frame(&self) -> u64 {
        self.start
    }

    fn end_frame(&self) -> u64 {
        self.end
    }
}

impl Block for Bass {
    type Settings = Note;

    const PROBES: &'static [Probe<Note>] = &[
        Probe { name: "sub", settings: Note::probe(crate::BASS, crate::BASS_CHANNEL, 40.0, SUB, NO_EXTRA) },
        Probe { name: "round", settings: Note::probe(crate::BASS, crate::BASS_CHANNEL, 40.0, ROUND, NO_EXTRA) },
        Probe { name: "fifths", settings: Note::probe(crate::BASS, crate::BASS_CHANNEL, 40.0, FIFTHS, NO_EXTRA) },
        Probe { name: "pluck", settings: Note::probe(crate::BASS, crate::BASS_CHANNEL, 40.0, PLUCKBASS, NO_EXTRA) },
        Probe { name: "moog", settings: Note::probe(crate::BASS, crate::BASS_CHANNEL, 40.0, MOOGBASS, NO_EXTRA) },
    ];

    fn start(&mut self, note: &Note, cx: &Context) {
        let [a, b, lift, ..] = note.extra;
        let (noise, rate) = (cx.noise.len(), cx.rate);
        // No lift stamped (the kit's probe) is today's level.
        let lift = if lift.is_finite() { lift } else { 0.0 };
        self.play(note.parts, note.midi, note.time, note.dur, note.vel, (a, b), noise, lift, rate);
    }

    fn span(&self) -> (u64, u64) {
        (self.start_frame(), self.end_frame())
    }

    fn render(&mut self, block: u64, cx: &Context, out: &mut Signal) {
        let (rate, waves, noise) = (cx.rate, cx.waves, cx.noise);
        let r = rate as f32;
        if self.main {
            let mut freq = [0.0f32; QUANTUM];
            let first = starts_in(self.osc.start_frame(), block);
            let pitch = pitch_of(
                &mut self.pitch,
                block,
                rate,
                first,
                &mut freq,
                Part::Steady(0.0),
            );
            let wave = if self.triangle {
                &waves.triangle
            } else {
                &waves.saw
            };
            let mut tone = [0.0f32; QUANTUM];
            self.osc.render(block, wave, &pitch, r, &mut tone);
            // The filter runs every frame of the block, as a node does, its
            // cutoff swept.
            let mut cutoff = [0.0f32; QUANTUM];
            self.cutoff.fill(block, rate, &mut cutoff);
            self.filter.render_mod(block, cx, &mut tone, &Mods::NONE.with(filter::FREQ, &cutoff));
            let mut gain = [0.0f32; QUANTUM];
            self.gain.fill(block, rate, &mut gain);
            for ((y, x), g) in out.iter_mut().zip(tone.iter()).zip(gain.iter()) {
                *y += *x * *g;
            }
        }
        for s in self.sines.iter_mut().take(self.sine_count) {
            s.render(block, rate, waves, out);
        }
        self.harmonics.render(block, cx, out);
        if self.click {
            let mut x = [0.0f32; QUANTUM];
            self.src.fill(block, noise, &mut x);
            let mut gain = [0.0f32; QUANTUM];
            self.click_gain.fill(block, rate, &mut gain);
            for ((y, v), g) in out.iter_mut().zip(x.iter()).zip(gain.iter()) {
                *y += self.click_filter.step(*v) * *g;
            }
            self.click_filter.flush();
        }
    }
}

impl Voice for Bass {
    /// The pluck's click: its rate and offset.
    fn settle(note: &mut Note) {
        if note.parts & KIND == PLUCKBASS {
            note.extra[0] = note.draws.unit(salt::GRAIN, 0);
            note.extra[1] = note.draws.unit(salt::GRAIN, 1);
        }
    }

    fn uses_noise(note: &Note) -> bool {
        note.parts & KIND == PLUCKBASS
    }
}

impl Default for Bass {
    fn default() -> Self {
        Self::new()
    }
}
