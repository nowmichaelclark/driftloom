//! The chapel organ (queue item 61; Ears 4, rounds 5 to 7, approved by Mikey
//! 2026-10-16 as `organVariant: 'chapelOrgan'`): an instrument the chords'
//! channel holds for the whole loop, outside the voice pool and its budget.
//!
//! Four voices, each a pipe and a celeste rank a few cents sharp, read from
//! one wavetable the loop's registration builds (its drawbars, 16' to 1').
//! A retune (`event::RETUNE`) moves it to the next chord by the shared rule
//! (`blocks::retune`): a note the two chords share sounds on, untouched; a
//! voice sounding a note that goes lets go quickly and speaks its new one,
//! with a chiff (a breath of filtered noise, air finding the pipe); a voice
//! with no note left lets go over the release. A release
//! (`event::RELEASE`) lets every voice go, over `value` seconds.
//!
//! The voices sum into a soft shaper (no drive: its gentlest curve), a
//! highpass, a dip at 800 Hz where the tunes live, and a darker lowpass;
//! then a slow level shimmer, the loop's level, and some of it into the room
//! (`Channel::SENDS`). Every number is the host's, set for the loop
//! (`SETTINGS`, `Core::set`); the host works them out from the loop's draws
//! as the sketch mixed its four organs (`ears4-sketch.patch`, `organSetup`,
//! `_organLegato`, `_organSpeak`).

// Web Audio's parameters are 32-bit floats, set from the host's 64-bit
// numbers.
#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]

use core::f64::consts::TAU;

use crate::block::{Block, Channel, Context, Event, Mods, Probe, Sends, Setting, Signal, event};
use crate::math;
use crate::param::Param;
use crate::timeline::Timeline;
use crate::QUANTUM;

use super::filter::{Biquad, Kind};
use super::noise_source::NoiseSource;
use super::shared::{Turn, midi_to_freq, retune};

/// The voices.
const VOICES: usize = 4;
/// The drawbars, and the harmonic of the voice's frequency (half the note's:
/// the 16') each one sounds: 16', 5 1/3', 8', 4', 2 2/3', 2', 1 3/5', 1 1/3',
/// 1'.
pub const BARS: usize = 9;
const HARMONIC: [f64; BARS] = [1.0, 3.0, 2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 16.0];
/// The wavetable's length: one cycle of the registration.
const TABLE: usize = 2048;
/// Where the notes are folded to: G#3 to G4 (48-67, the sketch's), and how
/// many sound.
const LOWEST: f64 = 48.0;
const HIGHEST: f64 = 67.0;

/// The settings a host sets for the loop, by number (`SETTINGS`).
pub mod set {
    /// The first drawbar (16'); the nine are `BAR..BAR + 9`, each up to 8, 3
    /// dB a step (a mixed registration's quiet stops fall under 0; 0 is off).
    pub const BAR: usize = 0;
    /// How long a pipe takes to speak, in seconds.
    pub const SPEAK: usize = 9;
    /// The chiff's level against the note's.
    pub const CHIFF: usize = 10;
    /// The celeste rank's detune, in cents, and its level.
    pub const CELESTE: usize = 11;
    /// The celeste's level.
    pub const CELESTE_LEVEL: usize = 12;
    /// How long a pipe takes to let go, in seconds.
    pub const RELEASE: usize = 13;
    /// The lowpass's corner, in Hz.
    pub const TONE: usize = 14;
    /// The highpass's corner, in Hz.
    pub const LOW: usize = 15;
    /// The dip at 800 Hz, in dB (0 for none).
    pub const CARVE: usize = 16;
    /// The level shimmer's rate, in Hz.
    pub const SHIMMER: usize = 17;
    /// The organ's level (0 for silent).
    pub const LEVEL: usize = 18;
    /// How much of it goes into the room.
    pub const VERB: usize = 19;
    /// How much of it is heard directly.
    pub const DRY: usize = 20;
    /// How many settings there are.
    pub const COUNT: usize = 21;
}

/// How it is set when it starts: every setting (as `set` numbers them), and
/// a chord to play from the start and when to let it go (NaN for none):
/// the kit's probes. A host retunes and releases it with events instead.
#[derive(Clone, Copy)]
pub struct Settings {
    /// The settings, by `set` number.
    pub values: [f64; set::COUNT],
    /// A chord to speak at its first block.
    pub chord: [f64; VOICES],
    /// When to let the chord go, in seconds (NaN: never).
    pub release: f64,
}

/// At rest: silent, until a host sets its level and retunes it.
pub const REST: Settings = Settings { values: [0.0; set::COUNT], chord: [f64::NAN; VOICES], release: f64::NAN };

/// The chapel organ as the sketch mixed it (55% chapel, 25% round 5's pipe
/// on a mellow registration, 10% the distant cathedral, 10% the celeste),
/// at its level in a loop of three or more notes a second: the kit's probe.
const CHAPEL: [f64; set::COUNT] = [
    3.49, 0.0, 7.75, 5.70, 2.35, 2.92, 0.81, 1.73, 2.84, // drawbars
    0.144, 0.605, 3.68, 0.36, 0.4675, 2345.0, 72.5, -3.75, 0.5, 0.098, 0.525, 0.94,
];

/// The shaper's curve, `tanh(k x) / tanh(k)` at the sketch's gentlest `k`
/// (0.6 + 2.2 x its drive, and the approved organ has none).
const SHAPE: f64 = 0.6;
/// How deep the level shimmer goes (round 4's 0.12).
const SHIMMER_DEPTH: f64 = 0.12;

/// A phase back into 0 to 1.
fn wrap(x: f64) -> f64 {
    x - math::floor(x)
}

/// One voice: its note, its pitch and level over time, its phases, and its
/// chiff.
struct Pipe {
    midi: f64,
    on: bool,
    freq: Timeline<4>,
    gain: Timeline<8>,
    phase: f64,
    celeste: f64,
    chiff: NoiseSource,
    chiff_band: Biquad,
    chiff_gain: Param,
    chiffing: bool,
}

impl Pipe {
    /// All zeros (the core holds an organ in every channel, in its static
    /// memory, where all zeros cost nothing): `ready` makes it a voice.
    const fn new() -> Self {
        Pipe {
            midi: 0.0,
            on: false,
            freq: Timeline::new(0.0, 0.0, 0.0),
            gain: Timeline::new(0.0, 0.0, 0.0),
            phase: 0.0,
            celeste: 0.0,
            chiff: NoiseSource::unstarted(),
            chiff_band: Biquad::new(),
            chiff_gain: Param::new(0.0),
            chiffing: false,
        }
    }

    /// At rest on 110 Hz, as a fresh oscillator and a closed gain are.
    fn ready(&mut self) {
        self.freq.configure(110.0, 0.0, 24_000.0);
        self.gain.configure(0.0, -1.0, 1.0);
    }
}

/// The organ: its voices, its table, its chain, and what it was set to.
pub struct Organ {
    pipes: [Pipe; VOICES],
    table: [f32; TABLE],
    /// The registration changed since the table was built.
    stale: bool,
    values: [f64; set::COUNT],
    low: Biquad,
    carve: Biquad,
    tone: Biquad,
    shimmer: f64,
    rate: f64,
    /// The core's noise's length, which a chiff plays from.
    noise_len: usize,
    /// A chord to speak at the first block (`opening_len` notes), and when
    /// to let it go (if `closing_on`): the kit's probe (`Settings`).
    opening: [f64; VOICES],
    opening_len: usize,
    closing: f64,
    closing_on: bool,
    /// The start of the next block, in seconds: the clock an event reaches
    /// it at.
    now: f64,
}

impl Organ {
    /// Silent and all zeros.
    #[must_use]
    pub const fn new() -> Self {
        Organ {
            pipes: [Pipe::new(), Pipe::new(), Pipe::new(), Pipe::new()],
            table: [0.0; TABLE],
            stale: false,
            values: [0.0; set::COUNT],
            low: Biquad::new(),
            carve: Biquad::new(),
            tone: Biquad::new(),
            shimmer: 0.0,
            rate: 0.0,
            noise_len: 0,
            opening: [0.0; VOICES],
            opening_len: 0,
            closing: 0.0,
            closing_on: false,
            now: 0.0,
        }
    }

    fn value(&self, n: usize) -> f64 {
        self.values.get(n).copied().unwrap_or(0.0)
    }

    /// The registration's one cycle: each drawbar's harmonic at its pipe
    /// loudness (3 dB a step below 8; any value but 0 sounds), normalized to a peak of 1, as Web
    /// Audio normalizes a periodic wave.
    fn build_table(&mut self) {
        let mut amps = [0.0; BARS];
        for (a, n) in amps.iter_mut().zip(set::BAR..) {
            let b = self.value(n);
            // A mixed registration's quiet stops sit under 0 (a drawbar of
            // -0.67 is about 0.05 of the 8'): only 0 itself is off.
            *a = if b == 0.0 { 0.0 } else { math::pow(10.0, (b - 8.0) * 3.0 / 20.0) };
        }
        let mut peak = 0.0f64;
        let mut cycle = [0.0f64; TABLE];
        for (i, y) in cycle.iter_mut().enumerate() {
            let x = TAU * i as f64 / TABLE as f64;
            *y = amps.iter().zip(HARMONIC.iter()).map(|(a, h)| a * math::sin(h * x)).sum();
            peak = peak.max(y.abs());
        }
        let scale = if peak > 0.0 { 1.0 / peak } else { 0.0 };
        for (t, y) in self.table.iter_mut().zip(cycle.iter()) {
            *t = (y * scale) as f32;
        }
        self.stale = false;
    }

    /// The filters, for the settings they read.
    fn set_filters(&mut self) {
        let rate = self.rate as f32;
        self.low.set(Kind::Highpass, self.value(set::LOW).max(10.0) as f32, 0.5, 0.0, rate);
        self.carve.set(Kind::Peaking, 800.0, 0.8, self.value(set::CARVE) as f32, rate);
        self.tone.set(Kind::Lowpass, self.value(set::TONE).max(100.0) as f32, 0.5, 0.0, rate);
    }

    /// Move to `chord` at `time` (`_organLegato`).
    fn retune(&mut self, chord: &[f64], time: f64) {
        // Folded into G#3-G4, each note once, low to high, four at most.
        let mut ms = [f64::NAN; VOICES];
        let mut count = 0;
        let mut sorted = [f64::INFINITY; crate::block::event::NOTES];
        for (s, &n) in sorted.iter_mut().zip(chord.iter()) {
            let mut m = n;
            while m > HIGHEST {
                m -= 12.0;
            }
            while m < LOWEST {
                m += 12.0;
            }
            *s = m;
        }
        // Low to high, by insertion: six notes at most.
        for i in 1..sorted.len() {
            let mut j = i;
            while j > 0 && let (Some(&a), Some(&b)) = (sorted.get(j - 1), sorted.get(j)) && a > b {
                sorted.swap(j - 1, j);
                j -= 1;
            }
        }
        for &m in &sorted {
            if m.is_finite() && !ms.contains(&m) && let Some(slot) = ms.get_mut(count) {
                *slot = m;
                count += 1;
            }
        }
        let each = 1.0 / math::sqrt(count.max(3) as f64);
        let speak = self.value(set::SPEAK);
        let release = self.value(set::RELEASE);
        let chiff = self.value(set::CHIFF);
        let now = self.now;
        let held = self.pipes.each_ref().map(|p| if p.on { p.midi } else { f64::NAN });
        let turns = retune(&held, ms.get(..count).unwrap_or(&[]));
        let (rate, noise_len) = (self.rate, self.noise_len);
        for (pipe, turn) in self.pipes.iter_mut().zip(turns) {
            let g = &mut pipe.gain;
            match turn {
                Turn::Keep => {
                    g.cancel_scheduled_values(time, now);
                    g.set_target_at_time(each as f32, time, 0.05, now);
                }
                Turn::Speak(m) => {
                    let f = (midi_to_freq(m) / 2.0) as f32;
                    g.cancel_scheduled_values(time, now);
                    // A voice still sounding another note lets go first, quickly.
                    let at = if pipe.on {
                        g.set_target_at_time(0.0, time, 0.008, now);
                        time + 0.03
                    } else {
                        time
                    };
                    pipe.freq.set_value_at_time(f, at, now);
                    g.set_target_at_time(each as f32, at, speak / 3.0, now);
                    pipe.midi = m;
                    pipe.on = true;
                    if chiff > 0.0 {
                        // The breath of air finding the pipe, at its second
                        // harmonic.
                        let offset = (at * 3.17 + m * 0.013) % 1.5;
                        pipe.chiff.grain(at, offset, 0.15, 1.0, noise_len, rate);
                        pipe.chiff_band.reset();
                        pipe.chiff_band.set(Kind::Bandpass, (midi_to_freq(m) * 2.0).min(6000.0) as f32, 6.0, 0.0, rate as f32);
                        let c = &mut pipe.chiff_gain;
                        c.reset(0.0);
                        c.set_value_at_time(0.0, at);
                        c.linear_ramp_to_value_at_time((each * chiff) as f32, at + 0.012, at);
                        c.set_target_at_time(0.0, at + 0.012, 0.025);
                        pipe.chiffing = true;
                    }
                }
                Turn::Rest => {
                    g.cancel_scheduled_values(time, now);
                    g.set_target_at_time(0.0, time, release / 3.0, now);
                    pipe.on = false;
                    pipe.midi = f64::NAN;
                }
            }
        }
    }

    /// Let every voice go at `time`, over `fall` seconds (`organOff`).
    fn let_go(&mut self, time: f64, fall: f64) {
        let now = self.now;
        for pipe in &mut self.pipes {
            pipe.gain.cancel_scheduled_values(time, now);
            pipe.gain.set_target_at_time(0.0, time, fall / 3.0, now);
            pipe.on = false;
            pipe.midi = f64::NAN;
        }
    }

    /// Whether anything is sounding or ringing: a silent organ costs nothing.
    fn quiet(&self) -> bool {
        self.pipes.iter().all(|p| !p.on && !p.chiffing && p.gain.idle() && p.gain.value() == 0.0)
            && self.low.at_rest()
            && self.carve.at_rest()
            && self.tone.at_rest()
    }

    /// One block of the organ, through its chain to just before its level:
    /// added into `out` at its level and directly, and into the room.
    // A phase in 0 to 1 times the table's length, and a few blocks' frames:
    // positive and small.
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn play(&mut self, block: u64, cx: &Context, out: &mut Signal, room: Option<&mut Signal>) {
        let start = block as f64 / cx.rate;
        self.now = start;
        if self.opening_len > 0 {
            let chord = self.opening;
            let len = core::mem::take(&mut self.opening_len);
            self.retune(chord.get(..len).unwrap_or(&[]), start);
        }
        let end = (block + QUANTUM as u64) as f64 / cx.rate;
        if self.closing_on && self.closing < end {
            self.closing_on = false;
            self.let_go(self.closing, self.value(set::RELEASE));
        }
        self.now = end;
        if self.quiet() {
            return;
        }
        if self.stale {
            self.build_table();
        }
        let detune = math::pow(2.0, self.value(set::CELESTE) / 1200.0);
        let celeste_level = self.value(set::CELESTE_LEVEL) as f32;
        let mut bus = [0.0f32; QUANTUM];
        let mut freq = [0.0f32; QUANTUM];
        let mut gain = [0.0f32; QUANTUM];
        let table = &self.table;
        let read = |phase: f64| -> f32 {
            let x = phase * TABLE as f64;
            let i = x as usize;
            let frac = (x - i as f64) as f32;
            let a = table.get(i % TABLE).copied().unwrap_or(0.0);
            let b = table.get((i + 1) % TABLE).copied().unwrap_or(0.0);
            a + (b - a) * frac
        };
        for pipe in &mut self.pipes {
            pipe.freq.fill(block, cx.rate, &mut freq);
            pipe.gain.fill(block, cx.rate, &mut gain);
            for ((y, &f), &g) in bus.iter_mut().zip(freq.iter()).zip(gain.iter()) {
                if g != 0.0 {
                    *y += (read(pipe.phase) + read(pipe.celeste) * celeste_level) * g;
                }
                let step = f64::from(f) / cx.rate;
                pipe.phase = wrap(pipe.phase + step);
                pipe.celeste = wrap(pipe.celeste + step * detune);
            }
            if pipe.chiffing {
                let mut noise = [0.0f32; QUANTUM];
                pipe.chiff.fill(block, cx.noise, &mut noise);
                let mut c = [0.0f32; QUANTUM];
                pipe.chiff_gain.fill(block, cx.rate, &mut c);
                for ((y, &x), &k) in bus.iter_mut().zip(noise.iter()).zip(c.iter()) {
                    *y += pipe.chiff_band.step(x) * k;
                }
                pipe.chiff_band.flush();
                if block > pipe.chiff.end_frame() + (cx.rate * 0.3) as u64 {
                    pipe.chiffing = false;
                }
            }
        }
        // The shaper at its gentlest (no drive), with the sketch's headroom
        // and make-up: in at 0.4, out at 1.5 tanh(k) / k.
        let post = 1.5 * math::tanh(SHAPE) / SHAPE;
        let tk = math::tanh(SHAPE);
        let level = self.value(set::LEVEL) as f32;
        let dry = self.value(set::DRY) as f32;
        let verb = self.value(set::VERB) as f32;
        // The slow level shimmer (round 4's, no pitch-bending chorus): a
        // sine under 1 Hz, worked out at the block's two ends and drawn
        // straight between them (2.7 ms at 48 kHz: a millionth off the
        // curve).
        let shimmer = |frame: u64| 1.0 + SHIMMER_DEPTH * math::sin(TAU * self.shimmer * (frame as f64 / cx.rate));
        let (from, to) = (shimmer(block), shimmer(block + QUANTUM as u64));
        let mut room = room;
        for (i, x) in bus.iter().enumerate() {
            let s = (f64::from(*x) * 0.4).clamp(-1.0, 1.0);
            let shaped = (math::tanh(SHAPE * s) / tk * post) as f32;
            let v = self.tone.step(self.carve.step(self.low.step(shaped)));
            let trem = (from + (to - from) * (i as f64 / QUANTUM as f64)) as f32;
            let leveled = v * trem * level;
            if let Some(y) = out.get_mut(i) {
                *y += leveled * dry;
            }
            if let Some(r) = room.as_deref_mut().and_then(|r| r.get_mut(i)) {
                *r += leveled * verb;
            }
        }
        self.low.flush();
        self.carve.flush();
        self.tone.flush();
        for pipe in &mut self.pipes {
            pipe.phase = math::flush(pipe.phase);
        }
    }
}

impl Default for Organ {
    fn default() -> Self {
        Self::new()
    }
}

/// The kit's probe value of every setting: a step a listener hears.
const fn probe_of(n: usize) -> f64 {
    match n {
        0..=8 => 8.0,
        set::SPEAK | set::DRY => 0.4,
        set::CHIFF => 1.4,
        set::CELESTE => 7.0,
        set::CELESTE_LEVEL => 0.9,
        set::RELEASE => 0.8,
        set::TONE => 1200.0,
        set::LOW => 200.0,
        set::CARVE => -12.0,
        set::SHIMMER => 3.0,
        set::LEVEL => 0.3,
        set::VERB => 1.3,
        _ => f64::NAN,
    }
}

impl Block for Organ {
    type Settings = Settings;

    const PROBES: &'static [Probe<Settings>] = &[Probe {
        name: "chapel",
        settings: Settings { values: CHAPEL, chord: [60.0, 64.0, 67.0, f64::NAN], release: 1.2 },
    }];

    const HEARS: &'static [u32] = &[event::RETUNE, event::RELEASE];

    const SETTINGS: &'static [Setting] = &[
        Setting { name: "16'", probe: probe_of(0) },
        Setting { name: "5-1/3'", probe: probe_of(1) },
        Setting { name: "8'", probe: 4.0 },
        Setting { name: "4'", probe: probe_of(3) },
        Setting { name: "2-2/3'", probe: probe_of(4) },
        Setting { name: "2'", probe: probe_of(5) },
        Setting { name: "1-3/5'", probe: probe_of(6) },
        Setting { name: "1-1/3'", probe: probe_of(7) },
        Setting { name: "1'", probe: probe_of(8) },
        Setting { name: "speak", probe: probe_of(set::SPEAK) },
        Setting { name: "chiff", probe: probe_of(set::CHIFF) },
        Setting { name: "celeste", probe: probe_of(set::CELESTE) },
        Setting { name: "celeste-level", probe: probe_of(set::CELESTE_LEVEL) },
        Setting { name: "release", probe: probe_of(set::RELEASE) },
        Setting { name: "tone", probe: probe_of(set::TONE) },
        Setting { name: "low", probe: probe_of(set::LOW) },
        Setting { name: "carve", probe: probe_of(set::CARVE) },
        Setting { name: "shimmer", probe: probe_of(set::SHIMMER) },
        Setting { name: "level", probe: probe_of(set::LEVEL) },
        // Only into the room, which a probe does not hear.
        Setting { name: "verb", probe: f64::NAN },
        Setting { name: "dry", probe: probe_of(set::DRY) },
    ];

    fn start(&mut self, s: &Settings, cx: &Context) {
        *self = Organ::new();
        self.rate = cx.rate;
        self.noise_len = cx.noise.len();
        self.values = s.values;
        self.shimmer = self.value(set::SHIMMER);
        self.stale = true;
        self.set_filters();
        for pipe in &mut self.pipes {
            pipe.ready();
        }
        self.opening = s.chord;
        self.opening_len = s.chord.iter().position(|m| m.is_nan()).unwrap_or(VOICES);
        self.closing = s.release;
        self.closing_on = s.release.is_finite();
    }

    fn span(&self) -> (u64, u64) {
        (0, u64::MAX)
    }

    fn set(&mut self, setting: usize, value: f64, _: f64, _: &Context) {
        if let Some(v) = self.values.get_mut(setting) {
            *v = value;
        }
        if setting < set::BAR + BARS {
            self.stale = true;
        }
        match setting {
            set::LOW | set::CARVE | set::TONE => self.set_filters(),
            set::SHIMMER => self.shimmer = value,
            _ => {}
        }
    }

    fn event(&mut self, e: &Event) {
        match e.kind {
            event::RETUNE => self.retune(e.chord(), e.time),
            event::RELEASE => self.let_go(e.time, if e.value > 0.0 { e.value } else { 0.25 }),
            _ => {}
        }
    }

    fn render(&mut self, block: u64, cx: &Context, signal: &mut Signal) {
        self.play(block, cx, signal, None);
    }
}

// A source on its own bus in the chords' channel, with its room.
impl Channel for Organ {
    const SENDS: bool = true;

    fn render_sends(&mut self, block: u64, cx: &Context, signal: &mut Signal, _: &Mods, sends: &mut Sends) {
        let [verb, _] = sends;
        self.play(block, cx, signal, Some(verb));
    }
}

