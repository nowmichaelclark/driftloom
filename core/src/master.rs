//! The master chain (queue items 30 and 31), from `preBus` to the
//! speakers, as `_build()` in synth.js makes it: the tape wobble, the
//! saturator, the tone lowpass and the highpass, then the bus compressor,
//! the master gain, the kill gain and the ceiling.
//!
//! - **Wobble**: a delay of 14 ms whose time two sine LFOs move, wow (0.32
//!   Hz) and flutter (6.3 Hz), each through a gain `setTone` sets. The LFOs
//!   are oscillators started when the synth is built, so their phase is
//!   counted from then (`start`); the delay time is the parameter's own
//!   value plus both, every frame, held within 0 and 0.2 s, and the delay
//!   is Chromium's (`delay`).
//! - **Saturator**: a `WaveShaperNode` with a `tanh` curve of 2048 points,
//!   read as Chromium reads it (the input mapped to an index, the two points
//!   either side blended, in 32-bit floats), with 2x oversampling on full
//!   quality (`resample`) and none on lite. `setTone` replaces the curve
//!   outright; the new one is read from the next block on.
//! - **Tone**: a lowpass at 7200 Hz, Q 0.6, that `setTone` moves; then, in
//!   loops that draw it, the arc (`blocks::arc`, queue item 63), a second
//!   lowpass breathing over minutes; then a highpass at 38 Hz. The same
//!   `Biquad` the voices use.
//! - **comp**: Chromium's `DynamicsCompressorNode` (`compressor`) at
//!   -10 dB, a 10 dB knee, 3:1, attack 6 ms, release 0.25 s.
//! - **master**: a gain at 0.85 that `setVolume` and `setCharacterLevel`
//!   glide; **kill**: a gain at 1 that `silence` ramps to 0 in 60 ms and
//!   `unsilence` back in 20 ms (`HOLD` first holds it where it is, as
//!   `setValueAtTime(g.value, when)` does).
//! - **ceiling**: a second compressor, the limiter: -3 dB, no knee, 20:1,
//!   attack 1 ms, release 80 ms.
//!
//! **Lanes.** Everything that carries the signal is kept per output
//! channel (`Lane`), and the compressors link their channels' detection as
//! Chromium's does; what moves the signal -- the LFOs, the curve, the
//! parameters -- is shared. The graph is mono, so there is one lane
//! (`OUT`); a stereo mix is a second lane and a wider mix, not a new chain.
//! Chromium runs its compressors' mono input as two identical channels,
//! which is the same thing.
//!
//! **Stereo** (queue item 64; `docs/CORE.md`, promise 9). A stereo render is
//! the same chain on two lanes (`OUT`; `set_layout`), and the mono render
//! still runs on the first alone, so it pays nothing for the second. In
//! stereo the compressors are linked the other way from Chromium's: one gain
//! for both sides, worked out from the mid, `(left + right) / 2`, so they
//! press as they do in mono (`Compressor::process_keyed`); the saturator and
//! the filters run on each side apart. After the ceiling, the width ceiling
//! (`blocks::width`) turns the side down where it gets wider than its
//! target. Fed the same on both sides (no space recipe), each side is the
//! mono render, bit for bit.

use crate::block::Signal;
use crate::math;
use crate::blocks::arc::{self, Arc};
use crate::blocks::compressor::{Compressor, Settings};
use crate::blocks::width::{self, Width};
use crate::delay::{Delay, buffer_len};
use crate::blocks::filter::{Biquad, Kind};
use crate::blocks::osc::TableOsc;
use crate::resample::{Down, Up};
use crate::timeline::Timeline;
use crate::wave::Waves;
use crate::QUANTUM;


const WOBBLE_MAX: f64 = 0.2;
const WOBBLE_TIME: f64 = 0.014;
const WOW_HZ: f64 = 0.32;
const WOW_DEPTH: f64 = 0.0016;
const FLUTTER_HZ: f64 = 6.3;
const FLUTTER_DEPTH: f64 = 0.00018;
const TONE_HZ: f64 = 7200.0;
const TONE_Q: f64 = 0.6;
const HIGHPASS_HZ: f64 = 38.0;
/// A BiquadFilterNode's Q, untouched.
const Q: f32 = 1.0;

/// Points on the saturator's curve (`tanhCurve`'s `n`).
pub const CURVE: usize = 2048;
/// The curve the synth starts with: `tanhCurve(1.0)`.
const DRIVE: f64 = 1.0;

/// The wobble's delay line at `mix::MAX_RATE`: a fifth of a second.
pub const MEMORY: usize = QUANTUM + crate::MAX_FRAMES_A_SECOND / 5;

const EVENTS: usize = 64;
// A volume slider moved by hand asks for a glide on every step it passes.
const VOLUME_EVENTS: usize = 256;

const MASTER: f64 = 0.85;
const COMP: Settings = Settings {
    threshold: -10.0,
    knee: 10.0,
    ratio: 3.0,
    attack: 0.006,
    release: 0.25,
};
const CEILING: Settings = Settings {
    threshold: -3.0,
    knee: 0.0,
    ratio: 20.0,
    attack: 0.001,
    release: 0.08,
};

/// The most output channels: two, a stereo render's lanes. A mono render
/// uses the first (`Master::layout`).
pub const OUT: usize = 2;

/// One output channel's share of the chain: everything with memory of the
/// signal.
struct Lane {
    wobble: Delay,
    memory: [f32; MEMORY],
    up: Up,
    down: Down,
    tone: Biquad,
    arc: Arc,
    highpass: Biquad,
}

impl Lane {
    const fn new() -> Self {
        Lane {
            wobble: Delay::new(),
            memory: [0.0; MEMORY],
            up: Up::new(),
            down: Down::new(),
            tone: Biquad::new(),
            arc: Arc::new(),
            highpass: Biquad::new(),
        }
    }

    /// Take `other`'s state, as if this lane had played what it played.
    fn copy_from(&mut self, other: &Lane) {
        self.wobble = other.wobble;
        self.memory.copy_from_slice(&other.memory);
        self.up.clone_from(&other.up);
        self.down.clone_from(&other.down);
        self.tone = other.tone;
        self.arc.clone_from(&other.arc);
        self.highpass = other.highpass;
    }
}

const fn f(x: f64) -> f32 {
    x as f32
}

/// The mid of two lanes, `(left + right) / 2`, frame by frame: what the
/// stereo compressors hear. With the same on both sides it is that signal,
/// bit for bit.
fn mid(lanes: &[Signal; OUT]) -> Signal {
    let [left, right] = lanes;
    let mut out = [0.0f32; QUANTUM];
    for ((m, &l), &r) in out.iter_mut().zip(left.iter()).zip(right.iter()) {
        *m = f32::midpoint(l, r);
    }
    out
}

pub struct Master {
    rate: f64,
    full: bool,
    /// The wow LFO steps a thirtieth of a table sample a frame, slow enough
    /// that Chromium reads it with Lagrange interpolation rather than the
    /// oscillator's usual linear read (`Wave::read_slow`), so it is run
    /// here, frame by frame: its first frame and its place in the table.
    wow_start: u64,
    wow_index: f64,
    flutter: TableOsc,
    /// The first block the LFOs have not yet been moved through.
    lfo_block: u64,
    wow_depth: Timeline<EVENTS>,
    flutter_depth: Timeline<EVENTS>,
    wobble_max: f32,
    wobble_time: f32,
    curve: [f32; CURVE],
    tone_freq: Timeline<EVENTS>,
    lanes: [Lane; OUT],
    /// How many lanes it renders: 1 (mono) or 2 (stereo).
    layout: usize,
    /// The mono render's compressors, and the stereo render's, keyed by the
    /// mid.
    comp: Compressor<1>,
    comp2: Compressor<2>,
    volume: Timeline<VOLUME_EVENTS>,
    kill: Timeline<EVENTS>,
    ceiling: Compressor<1>,
    ceiling2: Compressor<2>,
    /// The width ceiling, stereo only.
    width: Width,
    /// Both compressors routed around (the measure harness's `bypass`).
    bypass: bool,
    /// What reaches the bus compressor, and what leaves the ceiling.
    before_comp: [Signal; OUT],
    out: [Signal; OUT],
}

impl Master {
    /// All zeros, as `Mix::new`; `init` builds it.
    pub const fn new() -> Self {
        Master {
            rate: 0.0,
            full: false,
            wow_start: 0,
            wow_index: 0.0,
            flutter: TableOsc::new(),
            lfo_block: 0,
            wow_depth: Timeline::new(0.0, 0.0, 0.0),
            flutter_depth: Timeline::new(0.0, 0.0, 0.0),
            wobble_max: 0.0,
            wobble_time: 0.0,
            curve: [0.0; CURVE],
            tone_freq: Timeline::new(0.0, 0.0, 0.0),
            lanes: [const { Lane::new() }; OUT],
            layout: 0,
            comp: Compressor::new(),
            comp2: Compressor::new(),
            volume: Timeline::new(0.0, 0.0, 0.0),
            kill: Timeline::new(0.0, 0.0, 0.0),
            ceiling: Compressor::new(),
            ceiling2: Compressor::new(),
            width: Width::new(),
            bypass: false,
            before_comp: [[0.0; QUANTUM]; OUT],
            out: [[0.0; QUANTUM]; OUT],
        }
    }

    /// Build the chain at `rate`, oversampling the saturator on full
    /// quality. The LFOs start at frame 0 until `start` says otherwise.
    /// False if the wobble's delay does not fit (`mix::MAX_RATE`).
    pub fn init(&mut self, rate: f64, full: bool) -> bool {
        self.rate = rate;
        self.full = full;
        let any = (f32::MIN, f32::MAX);
        self.wow_depth.configure(f(WOW_DEPTH), any.0, any.1);
        self.flutter_depth.configure(f(FLUTTER_DEPTH), any.0, any.1);
        self.start(0.0);
        let r = f(rate);
        let mut fits = true;
        for lane in self.lanes.iter_mut() {
            let end = lane.wobble.place(WOBBLE_MAX, rate, 0);
            fits &= end <= MEMORY;
            self.wobble_max = lane.wobble.max;
            lane.memory.fill(0.0);
            lane.up.init();
            lane.down.init();
            lane.tone.reset();
            lane.arc.ready(rate);
            lane.highpass.reset();
            lane.highpass.set(Kind::Highpass, f(HIGHPASS_HZ), Q, 0.0, r);
        }
        self.wobble_time = f(WOBBLE_TIME).max(0.0).min(self.wobble_max);
        self.set_drive(DRIVE);
        self.tone_freq.configure(f(TONE_HZ), 0.0, r / 2.0);
        // Both compressors start warm (`Compressor::settle`): the first
        // notes after a Play, a rebuild or the core's arrival are not dipped.
        self.comp.init(rate, COMP);
        self.comp.settle();
        self.comp2.init(rate, COMP);
        self.comp2.settle();
        self.volume.configure(f(MASTER), any.0, any.1);
        self.kill.configure(1.0, any.0, any.1);
        self.ceiling.init(rate, CEILING);
        self.ceiling.settle();
        self.ceiling2.init(rate, CEILING);
        self.ceiling2.settle();
        self.width.ready(&width::REST, rate);
        // The layout is the host's (`set_layout`), kept across a rebuild.
        self.layout = self.layout.clamp(1, OUT);
        self.bypass = false;
        self.before_comp = [[0.0; QUANTUM]; OUT];
        self.out = [[0.0; QUANTUM]; OUT];
        fits
    }

    /// Render `lanes` lanes from the next block: 1 for mono, 2 for stereo
    /// (anything else is mono). A rebuild (`init`) keeps it.
    pub fn set_layout(&mut self, lanes: usize) {
        self.layout = if lanes == 2 { 2 } else { 1 };
    }

    /// Render `lanes` lanes from the next block, as `set_layout`, in the
    /// middle of a render (queue item 66): what the old layout held is
    /// carried into the new, so the sound goes on without a jump. Into
    /// stereo the second lane takes the first's state, and each compressor
    /// the mono one's; back into mono the mono compressors take the stereo
    /// ones', and the first lane goes on. The width ceiling starts afresh.
    /// What the switch does to the side is the core's (`Core::set_layout`).
    pub fn switch_layout(&mut self, lanes: usize) {
        let to = if lanes == 2 { 2 } else { 1 };
        if to == self.layout {
            return;
        }
        if to == 2 {
            let [first, second] = &mut self.lanes;
            second.copy_from(first);
            self.comp2.take_from(&self.comp);
            self.ceiling2.take_from(&self.ceiling);
            self.width.restart();
        } else {
            self.comp.take_from(&self.comp2);
            self.ceiling.take_from(&self.ceiling2);
        }
        self.layout = to;
    }

    /// How far apart the last block's two lanes were, the most of
    /// `|left - right| / 2`: 0 in mono.
    #[must_use]
    pub fn side_peak(&self) -> f32 {
        if self.layout != 2 {
            return 0.0;
        }
        let [left, right] = &self.out;
        left.iter().zip(right.iter()).map(|(l, r)| (l - r).abs() * 0.5).fold(0.0, f32::max)
    }

    /// How many lanes it renders.
    #[must_use]
    pub fn layout(&self) -> usize {
        self.layout
    }

    /// Setting number `setting` of the width ceiling (`width::SETTINGS`)
    /// becomes `value`, from the next block.
    pub fn set_width(&mut self, setting: usize, value: f64) {
        self.width.set(setting, value);
    }

    /// The LFOs' start, in seconds on the host's clock: when the synth
    /// built them (`start()` with no time starts an oscillator on the next
    /// block rendered).
    pub fn start(&mut self, time: f64) {
        let rate = self.rate;
        self.wow_start = crate::blocks::osc::frame_at(time, rate);
        self.wow_index = 0.0;
        self.flutter.schedule(time, f64::INFINITY, rate);
        // Nothing to move through before the block they start in.
        self.lfo_block = self.wow_start - self.wow_start % QUANTUM as u64;
    }

    /// `tanhCurve(drive)`, the curve `setTone` gives the saturator.
    pub fn set_drive(&mut self, drive: f64) {
        if !(drive.is_finite() && drive != 0.0) {
            return;
        }
        let n = CURVE as f64;
        for (i, y) in self.curve.iter_mut().enumerate() {
            let x = (i as f64 / (n - 1.0)) * 2.0 - 1.0;
            *y = (math::tanh(x * drive) / drive) as f32;
        }
    }

    pub fn wow_depth(&mut self) -> &mut Timeline<EVENTS> {
        &mut self.wow_depth
    }

    pub fn flutter_depth(&mut self) -> &mut Timeline<EVENTS> {
        &mut self.flutter_depth
    }

    /// The arc (queue items 63 and 63b): the swing `s` describes, from
    /// `time`; a period of 0 lets the last one go.
    pub fn arc(&mut self, s: &arc::Settings, time: f64) {
        for lane in &mut self.lanes {
            lane.arc.begin(s, time);
        }
    }

    /// What the arc lifts the echo's feedback by over the block from
    /// `block`, frame by frame; `None` with no arc.
    pub fn arc_lift(&mut self, block: u64) -> Option<Signal> {
        self.lanes.first_mut().and_then(|lane| lane.arc.lift(block))
    }

    pub fn tone_freq(&mut self) -> &mut Timeline<EVENTS> {
        &mut self.tone_freq
    }

    /// The master gain (`setVolume`, `setCharacterLevel`).
    pub fn volume(&mut self) -> &mut Timeline<VOLUME_EVENTS> {
        &mut self.volume
    }

    /// The kill gain (`silence`, `unsilence`).
    pub fn kill(&mut self) -> &mut Timeline<EVENTS> {
        &mut self.kill
    }

    /// Route around both compressors, or not.
    pub fn set_bypass(&mut self, bypass: bool) {
        self.bypass = bypass;
    }

    /// Start both compressors as Chromium's start (`cold`) or warm, the
    /// default (`Compressor::settle`). Before the first block; the measure
    /// harness's, to null the compressors against Chromium's own.
    pub fn set_cold(&mut self, cold: bool) {
        self.comp.init(self.rate, COMP);
        self.ceiling.init(self.rate, CEILING);
        if !cold {
            self.comp.settle();
            self.ceiling.settle();
        }
    }

    /// The compressors' `reduction`: the bus compressor's (0) or the
    /// ceiling's (1), in dB.
    pub fn reduction(&self, which: usize) -> f32 {
        match (which, self.layout) {
            (0, 2) => self.comp2.reduction(),
            (0, _) => self.comp.reduction(),
            (_, 2) => self.ceiling2.reduction(),
            _ => self.ceiling.reduction(),
        }
    }

    /// How hard the signal has pressed the bus compressor (0) or the
    /// ceiling (1) since this was last asked, in dB (`Compressor::deepest`).
    pub fn deepest(&mut self, which: usize) -> f32 {
        match (which, self.layout) {
            (0, 2) => self.comp2.deepest(),
            (0, _) => self.comp.deepest(),
            (_, 2) => self.ceiling2.deepest(),
            _ => self.ceiling.deepest(),
        }
    }

    /// What the last block sent to the speakers, a block a lane (the second
    /// only in stereo).
    pub fn output(&self) -> &[Signal; OUT] {
        &self.out
    }

    /// What the last block brought to the bus compressor, a block a lane.
    pub fn before_comp(&self) -> &[Signal; OUT] {
        &self.before_comp
    }

    /// One block of `preBus` in, a block a lane; the speakers' out. A mono
    /// render reads and writes the first lane only.
    pub fn process(&mut self, block: u64, input: &[Signal; OUT], waves: &Waves) -> &[Signal; OUT] {
        let n = self.layout.clamp(1, OUT);
        let rate = self.rate;
        let r32 = f(rate);
        let wave = &waves.sine;
        let (wow_hz, flutter_hz) = (f(WOW_HZ), f(FLUTTER_HZ));

        // The LFOs run from when the synth built them, rendered or not: a
        // host that starts later has them where they would be.
        self.catch_up(block, wave);
        let mut wow = [0.0f32; QUANTUM];
        let mut flutter = [0.0f32; QUANTUM];
        let steady = |hz| crate::blocks::osc::Pitch::Steady {
            freq: hz,
            detune: 0.0,
        };
        self.wow(block, wave, wow_hz, r32, Some(&mut wow));
        self.flutter
            .render(block, wave, &steady(flutter_hz), r32, &mut flutter);
        self.lfo_block = block + QUANTUM as u64;
        let mut depth = [0.0f32; QUANTUM];
        self.wow_depth.fill(block, rate, &mut depth);
        for (x, &d) in wow.iter_mut().zip(depth.iter()) {
            *x *= d;
        }
        self.flutter_depth.fill(block, rate, &mut depth);
        for (x, &d) in flutter.iter_mut().zip(depth.iter()) {
            *x *= d;
        }
        // The delay's time: its own value, then each LFO summed in, then
        // held within the parameter's range.
        let max = self.wobble_max;
        let mut times = [0.0f32; QUANTUM];
        for ((t, &w), &fl) in times.iter_mut().zip(wow.iter()).zip(flutter.iter()) {
            let mut x = self.wobble_time;
            x += w;
            x += fl;
            *t = if x.is_nan() { max } else { x.max(0.0).min(max) };
        }
        let mut shaped = [[0.0f32; QUANTUM]; OUT];
        for (lane, (input, shaped)) in self.lanes.iter_mut().zip(input.iter().zip(shaped.iter_mut())).take(n) {
            let mut wobbled = [0.0f32; QUANTUM];
            lane.wobble.process(&mut lane.memory, input, &times, r32, &mut wobbled);

            // The saturator.
            if self.full {
                let mut high = [0.0f32; 2 * QUANTUM];
                lane.up.process(&wobbled, &mut high);
                shape(&self.curve, &mut high);
                lane.down.process(&high, shaped);
            } else {
                *shaped = wobbled;
                shape(&self.curve, shaped);
            }
        }

        // Tone, then the highpass. The first lane works out the tone's
        // coefficients, frame by frame, and the others take them.
        let mut freq = [0.0f32; QUANTUM];
        self.tone_freq.fill(block, rate, &mut freq);
        let q = f(TONE_Q);
        for (i, &fr) in freq.iter().enumerate() {
            let (first, rest) = self.lanes.split_at_mut(1);
            let Some(lead) = first.first_mut() else { break };
            lead.tone.set(Kind::Lowpass, fr, q, 0.0, r32);
            for lane in rest.iter_mut().take(n - 1) {
                lane.tone.follow(&lead.tone);
            }
            for ((lane, x), y) in self.lanes.iter_mut().zip(shaped.iter()).zip(self.before_comp.iter_mut()).take(n) {
                if let (Some(&x), Some(y)) = (x.get(i), y.get_mut(i)) {
                    *y = lane.tone.step(x);
                }
            }
        }
        for (lane, y) in self.lanes.iter_mut().zip(self.before_comp.iter_mut()).take(n) {
            lane.arc.process(block, y);
            for x in y.iter_mut() {
                *x = lane.highpass.step(*x);
            }
            lane.tone.flush();
            lane.highpass.flush();
        }

        // The bus compressor, the master and kill gains, the ceiling.
        let mut pressed = [[0.0f32; QUANTUM]; OUT];
        let stereo = n == 2;
        if self.bypass {
            pressed = self.before_comp;
        } else if stereo {
            self.comp2.process_keyed(&self.before_comp, &mid(&self.before_comp), &mut pressed);
        } else {
            let [input, _] = &self.before_comp;
            let [out, _] = &mut pressed;
            self.comp.process(core::array::from_ref(input), core::array::from_mut(out));
        }
        let mut g = [0.0f32; QUANTUM];
        self.volume.fill(block, rate, &mut g);
        for lane in pressed.iter_mut().take(n) {
            for (y, &k) in lane.iter_mut().zip(g.iter()) {
                *y *= k;
            }
        }
        self.kill.fill(block, rate, &mut g);
        for lane in pressed.iter_mut().take(n) {
            for (y, &k) in lane.iter_mut().zip(g.iter()) {
                *y *= k;
            }
        }
        if self.bypass {
            self.out = pressed;
        } else if stereo {
            self.ceiling2.process_keyed(&pressed, &mid(&pressed), &mut self.out);
        } else {
            let [input, _] = &pressed;
            let [out, _] = &mut self.out;
            self.ceiling.process(core::array::from_ref(input), core::array::from_mut(out));
        }
        if stereo {
            self.width.process(&mut self.out);
        }
        &self.out
    }
}

impl Master {
    // Move the LFOs through every block before `block` that was not
    // rendered, to where they would be. Only the block they start in can be
    // partial; the whole blocks after it move each LFO on by one step a
    // block, so they are done in one multiplication, not stepped through: a
    // phone that loads the core thirty seconds in has a million frames to
    // cover, and not in one block of audio. The wow LFO lands bit for bit
    // where stepping would; the flutter, whose start can fall between two
    // frames, to within the rounding the stepped sum collects (about 7e-12
    // of a table sample after a thousand blocks, 5.4e-9 after 1.24 million;
    // `stepped`, in the tests).
    fn catch_up(&mut self, block: u64, wave: &crate::wave::Wave) {
        if self.lfo_block >= block {
            return;
        }
        let (wow_hz, flutter_hz, rate) = (f(WOW_HZ), f(FLUTTER_HZ), f(self.rate));
        // Blocks before the one they start in move nothing.
        let start_block = self.wow_start - self.wow_start % QUANTUM as u64;
        self.lfo_block = self.lfo_block.max(start_block.min(block));
        // The block they start in.
        if self.lfo_block == start_block && self.lfo_block < block {
            let b = self.lfo_block;
            self.wow(b, wave, wow_hz, rate, None);
            self.flutter.skip(b, wave, flutter_hz, rate);
            self.lfo_block += QUANTUM as u64;
        }
        let whole = block.saturating_sub(self.lfo_block) / QUANTUM as u64;
        if whole > 0 {
            let freq = wow_hz.max(-rate / 2.0).min(rate / 2.0);
            let incr = f64::from(freq * wave.rate_scale());
            let size = wave.size() as f64;
            let moved = self.wow_index + (whole * QUANTUM as u64) as f64 * incr;
            self.wow_index = moved - math::floor(moved / size) * size;
            self.flutter.skip_whole(whole, wave, flutter_hz, rate);
        }
        self.lfo_block = block;
    }

    // The wow LFO's block from frame `block` (`ProcessKRate`'s slow path):
    // each frame read at its place in the table, which then moves on by the
    // step, wrapped, in 64 bits. With no `out`, only its place moves.
    fn wow(&mut self, block: u64, wave: &crate::wave::Wave, hz: f32, rate: f32, mut out: Option<&mut Signal>) {
        let freq = hz.max(-rate / 2.0).min(rate / 2.0);
        let incr = freq * wave.rate_scale();
        let pick = wave.pick(freq);
        let size = wave.size() as f64;
        let inverse = 1.0 / size;
        for i in 0..QUANTUM {
            if block + (i as u64) < self.wow_start {
                continue;
            }
            if let Some(y) = out.as_deref_mut().and_then(|o| o.get_mut(i)) {
                *y = wave.read_slow(self.wow_index, incr, pick);
            }
            self.wow_index += incr as f64;
            self.wow_index -= math::floor(self.wow_index * inverse) * size;
        }
    }
}

impl Default for Master {
    fn default() -> Self {
        Self::new()
    }
}

// `WaveShaperCurveValues`, the x86 build: the input to an index into the
// curve, the two points either side, blended -- every step a 32-bit float.
fn shape(curve: &[f32; CURVE], samples: &mut [f32]) {
    let last = (CURVE - 1) as f32;
    let max = CURVE as i32 - 1;
    let scale = 0.5 * last;
    for x in samples.iter_mut() {
        let v = ((*x + 1.0) * scale).max(0.0).min(last);
        let i1 = v as i32;
        let index = i1 as f32;
        let at = |i: i32| curve.get(i.max(0).min(max) as usize).copied().unwrap_or(0.0);
        let (v1, v2) = (at(i1), at(i1 + 1));
        let frac = v - index;
        *x = frac * (v2 - v1) + v1;
    }
}

/// The memory the wobble's delay needs at `rate`, a lane.
pub fn memory_for(rate: f64) -> usize {
    buffer_len(WOBBLE_MAX, rate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wave::Fft;

    const RATE: f64 = 44100.0;

    fn built(full: bool) -> (Box<Master>, Box<Waves>) {
        std::thread::Builder::new()
            .stack_size(1 << 26)
            .spawn(move || {
                let mut m = Box::new(Master::new());
                assert!(m.init(RATE, full));
                let mut waves = Box::new(Waves::new());
                waves.build(RATE as f32, &mut Fft::new());
                (m, waves)
            })
            .and_then(|t| t.join().map_err(|_| std::io::Error::other("panicked")))
            .expect("master")
    }

    // The catch-up as it was: block by block, from `lfo_block` to `block`.
    fn stepped(m: &mut Master, block: u64, waves: &Waves) {
        let (wow_hz, flutter_hz, r32) = (f(WOW_HZ), f(FLUTTER_HZ), f(m.rate));
        while m.lfo_block < block {
            let b = m.lfo_block;
            m.wow(b, &waves.sine, wow_hz, r32, None);
            m.flutter.skip(b, &waves.sine, flutter_hz, r32);
            m.lfo_block += QUANTUM as u64;
        }
    }

    // How far apart two places in the table are, around its circle.
    fn apart(a: f64, b: f64, size: usize) -> f64 {
        let d = (a - b).abs();
        d.min(size as f64 - d)
    }

    #[test]
    fn the_lfos_catch_up_as_if_stepped_through() {
        let (mut direct, waves) = built(true);
        let (mut reference, _) = built(true);
        // Starts in the first block, mid-block, a hair past a frame, and
        // well in; the blocks asked for run from the start itself to an
        // hour on.
        for start in [0.0, 0.00123, 0.5 / RATE, 1.0, 12.3456789, 3600.0] {
            for blocks in [0u64, 1, 2, 3, 17, 1000, 10_337, 1_238_000] {
                direct.start(start);
                reference.start(start);
                let first = direct.lfo_block;
                // Possibly mid-way, as after some blocks have been rendered.
                let at = first + blocks * QUANTUM as u64;
                direct.catch_up(at, &waves.sine);
                stepped(&mut reference, at, &waves);
                assert_eq!(direct.lfo_block, reference.lfo_block, "start {start} blocks {blocks}");
                assert_eq!(direct.wow_index.to_bits(), reference.wow_index.to_bits(), "wow, start {start} blocks {blocks}");
                let off = apart(direct.flutter.index(), reference.flutter.index(), waves.sine.size());
                assert!(off < 1e-7, "flutter {off}, start {start} blocks {blocks}");
            }
        }
    }

    #[test]
    fn the_lfos_catch_up_across_gaps_between_rendered_blocks() {
        let (mut direct, waves) = built(true);
        let (mut reference, _) = built(true);
        direct.start(0.7);
        reference.start(0.7);
        let signal: Signal = core::array::from_fn(|i| ((i as f32) * 0.07).sin() * 0.3);
        let (mut worst, mut peak) = (0.0f32, 0.0f32);
        // Blocks rendered before the LFOs start, a gap that holds their
        // start, blocks rendered after it, and another gap: the real thing
        // `process` does, against stepping every block.
        for (from, count) in [(0u64, 40u64), (400, 10), (9000, 10), (9100, 3)] {
            for k in 0..count {
                let at = (from + k) * QUANTUM as u64;
                stepped(&mut reference, at, &waves);
                let a = *direct.process(at, &[signal, signal], &waves);
                let b = *reference.process(at, &[signal, signal], &waves);
                assert_eq!(direct.wow_index.to_bits(), reference.wow_index.to_bits(), "block {}", from + k);
                assert!(apart(direct.flutter.index(), reference.flutter.index(), waves.sine.size()) < 1e-9);
                for (x, y) in a[0].iter().zip(b[0].iter()) {
                    worst = worst.max((x - y).abs());
                    peak = peak.max(x.abs());
                }
            }
        }
        // The wobble's delay times differ by far less than a thousandth of
        // a frame.
        assert!(peak > 0.0 && worst < 1e-6, "{worst} against {peak}");
    }

    #[test]
    fn the_compressors_start_warm_unless_asked_to_start_cold() {
        // A loud steady tone from the first block: cold, the ceiling dips
        // the first blocks; warm, it does not.
        let dip = |cold: bool| {
            let (mut m, waves) = built(true);
            m.set_cold(cold);
            let mut first = 0.0f32;
            let mut last = 0.0f32;
            for b in 0..300u64 {
                let input: Signal = core::array::from_fn(|i| {
                    let n = (b as usize * QUANTUM + i) as f64;
                    (0.5 * (2.0 * core::f64::consts::PI * 440.0 * n / RATE).sin()) as f32
                });
                let out = m.process(b * QUANTUM as u64, &[input, input], &waves)[0];
                let peak = out.iter().fold(0.0f32, |a, &x| a.max(x.abs()));
                if b == 12 {
                    first = peak;
                }
                last = peak;
            }
            20.0 * (first / last).log10()
        };
        assert!(dip(true) < -3.0, "cold {}", dip(true));
        assert!(dip(false) > -1.0, "warm {}", dip(false));
    }

    #[test]
    fn the_wobble_fits_at_the_highest_rate() {
        assert_eq!(memory_for(crate::mix::MAX_RATE), MEMORY);
    }

    #[test]
    fn the_curve_is_tanh_over_the_drive() {
        let mut m = Box::new(Master::new());
        m.set_drive(2.0);
        assert_eq!(m.curve[0], ((-2.0f64).tanh() / 2.0) as f32);
        assert_eq!(m.curve[CURVE - 1], (2.0f64.tanh() / 2.0) as f32);
    }

    #[test]
    fn a_quiet_sine_comes_through_about_level_and_late() {
        for full in [true, false] {
            let (mut m, waves) = built(full);
            let hz = 1000.0;
            let mut peak = 0.0f32;
            for b in 0..200u64 {
                let input: Signal = core::array::from_fn(|i| {
                    let n = (b as usize * QUANTUM + i) as f64;
                    (0.1 * (2.0 * core::f64::consts::PI * hz * n / RATE).sin()) as f32
                });
                m.process(b * QUANTUM as u64, &[input, input], &waves);
                if b > 100 {
                    peak = m.before_comp()[0].iter().fold(peak, |a, &x| a.max(x.abs()));
                }
            }
            // tanh is nearly straight at 0.1; the filters pass 1 kHz.
            assert!((peak - 0.1).abs() < 0.01, "full {full}: {peak}");
        }
    }

    #[test]
    fn shaping_reads_the_curve_at_its_ends() {
        let mut m = Box::new(Master::new());
        m.set_drive(1.0);
        let mut x = [-2.0f32, 2.0, 0.0];
        shape(&m.curve, &mut x);
        assert_eq!(x[0], m.curve[0]);
        assert_eq!(x[1], m.curve[CURVE - 1]);
        assert!(x[2].abs() < 1e-3);
    }
}


