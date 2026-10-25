//! The mixing stage: the synth's five channels, their reverb and echo
//! sends, the kick's duck, the echo, the reverb and the tails, summed into
//! the bus that feeds the master chain (queue item 29). The graph is
//! `_build()` in synth.js, node for node:
//!
//! ```text
//! channel gain ─┬─ drums ───────────────────────────────┐
//!               ├─ the rest ─ pumpBus (duck) ──────────────┤
//!               ├─ verb send ─ reverbIn ─ six combs (three on lite)
//!               │     ─ combSum ─ reverbOut ─ preDelay ─┐   │
//!               └─ echo send ─ echoIn ─ echo ─ echoTone ─ tails ─ preBus
//!                                  └─ echoFb ─┘
//! ```
//!
//! Each comb is a delay with a lowpass and a gain in its feedback; its
//! output, taken before the lowpass, is what combSum sums. The echo's
//! feedback is taken after its lowpass.
//!
//! **The loops, as Chromium renders them.** A node renders once a block,
//! and one pulled again while it is rendering hands over the block it
//! rendered last (`AudioHandler::ProcessIfNecessary`). A comb is reached
//! through its delay, the only node of the loop anything outside pulls: the
//! delay pulls its feedback gain, which pulls the lowpass, which finds the
//! delay mid-render and filters the delay's last block. The echo is reached
//! through its lowpass (tails pulls it): the lowpass pulls the delay, the
//! delay pulls the feedback gain, and the gain finds the lowpass mid-render
//! and takes its last block. So each loop goes round a block (128 frames)
//! later than its delay alone, and here each keeps its last block for the
//! next, in the same place.
//!
//! **The parameters** are `Timeline`s, Chromium's AudioParam timeline, and
//! the delays `Delay`s, Chromium's delay line; the lowpasses are the same
//! `Biquad` the voices use. Gains that never move are plain numbers, each a
//! 32-bit float as the node holds it. Where several signals meet at one node
//! Chromium does not fix the order it adds them in, so sums here can differ
//! in the last bit.

use crate::block::{Sends, Signal};
use crate::delay::{Delay, buffer_len};
use crate::blocks::filter::{Biquad, Kind};
use crate::math;
use crate::space::{CENTERED, Drawn};
use crate::timeline::Timeline;
use crate::{BASS_CHANNEL, CHANNELS, CHORDS, DRUMS_CHANNEL, MELODY, QUANTUM, TEXTURE_CHANNEL, WASH_CHANNEL};


/// The highest sample rate the mix holds its delays for (`crate::MAX_RATE`).
/// A host at a higher rate gets no mix (`Mix::ready`).
pub const MAX_RATE: f64 = crate::MAX_RATE;

// The longest delay each line is made for, in seconds (`createDelay`).
const COMB_MAX: f64 = 0.5;
const PRE_DELAY_MAX: f64 = 0.2;
const ECHO_MAX: f64 = 2.0;

/// The delay lines' memory: six combs, the pre-delay and the echo, at
/// `MAX_RATE`, a block longer each (`delay::buffer_len`). All zeros, so it
/// costs nothing in the `.wasm`.
pub const MEMORY: usize = 6 * (QUANTUM + COMB_FRAMES) + (QUANTUM + PRE_DELAY_FRAMES) + (QUANTUM + ECHO_FRAMES);

// Each line's longest delay in frames at `MAX_RATE`: half a second, a fifth
// of one, and two (`COMB_MAX`, `PRE_DELAY_MAX`, `ECHO_MAX`).
const COMB_FRAMES: usize = crate::MAX_FRAMES_A_SECOND / 2;
const PRE_DELAY_FRAMES: usize = crate::MAX_FRAMES_A_SECOND / 5;
const ECHO_FRAMES: usize = crate::MAX_FRAMES_A_SECOND * 2;

/// The channels' gains, sends and order, in the core's channel numbers
/// (`MELODY` and the rest), as `_build()` sets them; the wash's channel as
/// the sketch made it, a copy of the textures' (queue item 62).
const GAIN: [f64; CHANNELS] = [0.58, 0.44, 0.62, 0.5, 0.82, 0.5];
const VERB: [f64; CHANNELS] = [0.28, 0.35, 0.05, 0.6, 0.1, 0.6];
const ECHO: [f64; CHANNELS] = [0.12, 0.15, 0.0, 0.25, 0.05, 0.25];
/// The order `_build()` makes the channels in; sums run in it. The wash's
/// comes last, so a mix without it adds a silent channel at the end, which
/// changes no sum by a bit.
const ORDER: [u32; CHANNELS] = [DRUMS_CHANNEL, BASS_CHANNEL, CHORDS, MELODY, TEXTURE_CHANNEL, WASH_CHANNEL];
/// The channels the duck presses: all but the drums.
const PUMPED: [u32; 5] = [BASS_CHANNEL, CHORDS, MELODY, TEXTURE_CHANNEL, WASH_CHANNEL];

const COMB_TIMES_FULL: [f64; 6] = [0.0297, 0.0371, 0.0411, 0.0437, 0.0503, 0.0577];
const COMB_TIMES_LITE: [f64; 3] = [0.0297, 0.0411, 0.0503];
const COMB_FEEDBACK: f64 = 0.8;
const COMB_LOWPASS: f64 = 2400.0;
const REVERB_OUT: f64 = 0.8;
const PRE_DELAY: f64 = 0.02;
const ECHO_TIME: f64 = 0.36;
const ECHO_FEEDBACK: f64 = 0.34;
const ECHO_LOWPASS: f64 = 2000.0;
/// A BiquadFilterNode's Q, untouched.
const Q: f32 = 1.0;

/// The parameters a host can move, by number (`Core::param`). Each is one
/// AudioParam of synth.js, or, for the combs, the same one of each comb,
/// which `setTone` always moves together.
pub const GAINS: u32 = 0; // to 4: each layer's channel's gain, by channel number (the wash's has none)
pub const PUMP: u32 = 5;
pub const TAILS: u32 = 6;
pub const ECHO_DELAY: u32 = 7;
pub const COMB_FB: u32 = 8;
pub const COMB_FREQ: u32 = 9;
pub const COMB_SUM: u32 = 10;
pub const REVERB_GAIN: u32 = 11;
/// The last of the mix's parameters; the master chain's follow
/// (`master`), numbered by the core (`TONE_FREQ` and the rest in lib.rs).
pub const LAST_PARAM: u32 = REVERB_GAIN;

/// How a parameter is moved: the AudioParam call.
pub const SET: u32 = 0;
pub const LINEAR: u32 = 1;
pub const TARGET: u32 = 2;
pub const CANCEL: u32 = 3;
/// `cancelScheduledValues(time)` and then `setValueAtTime(value, time)` with
/// the `value` attribute: held where it is (the synth's `silence`).
pub const HOLD: u32 = 4;

// Events each parameter holds at once. The duck takes two for every kick,
// and the measure harness schedules a whole render ahead -- up to seventy
// seconds of kicks.
const PUMP_EVENTS: usize = 2048;
const TAIL_EVENTS: usize = 256;
const EVENTS: usize = 64;

const fn f(x: f64) -> f32 {
    x as f32
}

pub struct Mix {
    rate: f64,
    ready: bool,
    combs: usize,
    gains: [Timeline<EVENTS>; CHANNELS],
    pump: Timeline<PUMP_EVENTS>,
    tails: Timeline<TAIL_EVENTS>,
    echo_time: Timeline<EVENTS>,
    comb_fb: Timeline<EVENTS>,
    comb_freq: Timeline<EVENTS>,
    comb_sum: Timeline<EVENTS>,
    reverb_out: Timeline<EVENTS>,
    comb_delays: [Delay; 6],
    comb_times: [f32; 6],
    comb_lps: [Biquad; 6],
    /// Each comb's delay output from the block before: what its lowpass
    /// filters this block.
    comb_last: [Signal; 6],
    pre_delay: Delay,
    echo: Delay,
    echo_lp: Biquad,
    /// The echo's lowpass output from the block before: what its feedback
    /// takes this block.
    echo_last: Signal,
    memory: [f32; MEMORY],
    bus: Signal,
    /// The side, in a stereo render (queue item 65).
    side: Side,
}

/// What a stereo render's side needs besides the channels (queue item 65;
/// `space`): the notes each channel places apart from itself (the light kit
/// and glass), weighted by their places, before the channel's gain; each
/// channel's own blocks' width (the wash's), before its gain; and the loop's
/// space.
pub struct SideIn<'a> {
    pub placed: &'a [Signal; CHANNELS],
    pub own: &'a [Signal; CHANNELS],
    pub space: &'a Drawn,
}

/// The room's allpasses: delay (seconds) and gain, the sketch's
/// (`verbSmooth`, round 3).
const ALLPASSES: [(f64, f32); 4] = [(0.0051, 0.6), (0.0117, 0.55), (0.0173, 0.5), (0.0229, 0.45)];
/// The longest allpass at the highest rate, in frames.
const ALLPASS_FRAMES: usize = crate::MAX_FRAMES_A_SECOND * 23 / 1000 + 2;
/// The spread's sway line: its longest delay.
const SWAY_MAX: f64 = 0.05;
const SWAY_FRAMES: usize = crate::MAX_FRAMES_A_SECOND / 20;
const SIDE_MEMORY: usize = 3 * (QUANTUM + SWAY_FRAMES) + (QUANTUM + PRE_DELAY_FRAMES);
/// The spread channels, in `Drawn::spread`'s order.
const SPREAD: [u32; 3] = [MELODY, CHORDS, TEXTURE_CHANNEL];
/// How fast a new loop's widths are glided to, a time constant (seconds).
const GLIDE: f64 = 0.05;

/// The amounts that glide: the room, the three spreads, the kit (hats and
/// the softer ones), the texture's place, glass's and the wash's width.
const AMOUNTS: usize = 9;

fn amounts(d: &Drawn) -> [f64; AMOUNTS] {
    let [melody, chords, texture] = d.spread;
    [d.room, melody, chords, texture, d.kit, d.kit * d.kit_soft, d.texture, d.glass, d.wash]
}

/// The side of the mix in a stereo render: everything here is side only
/// (+ left, - right), so the mid is the mono mix untouched. See `space` for
/// what each part is; the gains it rides are the mid's, captured as the mid
/// is mixed (`gains` and the rest).
struct Side {
    rate: f64,
    /// Each channel's gain, the duck's, the comb sum's, the reverb's output
    /// and the tails', this block: the mid's own.
    gains: [Signal; CHANNELS],
    pump: Signal,
    comb_gain: Signal,
    reverb_out: Signal,
    tails: Signal,
    /// The combs' outputs summed, before any gain.
    combs: Signal,
    /// The amounts now, and the loop's.
    now: [f64; AMOUNTS],
    want: [f64; AMOUNTS],
    rates: [f64; 3],
    delay: f64,
    sway: f64,
    sway_lines: [Delay; 3],
    sway_hp: [Biquad; 3],
    /// The highpasses on the kit's, the texture's and glass's places, and
    /// the wash's own width's (none at a cutoff of 0).
    place_hp: [Biquad; 3],
    wash_hp: Biquad,
    wash_cut: bool,
    room_hp: Biquad,
    allpass: [[f32; ALLPASS_FRAMES]; 4],
    allpass_len: [usize; 4],
    allpass_at: usize,
    pre: Delay,
    memory: [f32; SIDE_MEMORY],
    bus: Signal,
}

impl Side {
    // Built in place in the core's static, never on a stack.
    #[allow(clippy::large_stack_arrays)]
    const fn new() -> Self {
        Side {
            rate: 0.0,
            gains: [[0.0; QUANTUM]; CHANNELS],
            pump: [0.0; QUANTUM],
            comb_gain: [0.0; QUANTUM],
            reverb_out: [0.0; QUANTUM],
            tails: [0.0; QUANTUM],
            combs: [0.0; QUANTUM],
            now: [0.0; AMOUNTS],
            want: [0.0; AMOUNTS],
            rates: [0.0; 3],
            delay: 0.0,
            sway: 0.0,
            sway_lines: [const { Delay::new() }; 3],
            sway_hp: [const { Biquad::new() }; 3],
            place_hp: [const { Biquad::new() }; 3],
            wash_hp: Biquad::new(),
            wash_cut: false,
            room_hp: Biquad::new(),
            allpass: [[0.0; ALLPASS_FRAMES]; 4],
            allpass_len: [0; 4],
            allpass_at: 0,
            pre: Delay::new(),
            memory: [0.0; SIDE_MEMORY],
            bus: [0.0; QUANTUM],
        }
    }

    fn init(&mut self, rate: f64) {
        self.rate = rate;
        let mut at = 0;
        for line in &mut self.sway_lines {
            at = line.place(SWAY_MAX, rate, at);
        }
        self.pre.place(PRE_DELAY_MAX, rate, at);
        self.memory.fill(0.0);
        for ring in &mut self.allpass {
            ring.fill(0.0);
        }
        for ((len, &(d, _)), _) in self.allpass_len.iter_mut().zip(ALLPASSES.iter()).zip(0..) {
            // A whole number of frames, at least one, within the ring.
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let frames = math::round(d * rate).max(1.0) as usize;
            *len = frames.clamp(1, ALLPASS_FRAMES - 1);
        }
        self.allpass_at = 0;
        self.now = [0.0; AMOUNTS];
        self.set(&CENTERED);
        self.bus = [0.0; QUANTUM];
    }

    /// The loop's space, glided to from the next block.
    fn set(&mut self, d: &Drawn) {
        self.want = amounts(d);
        self.rates = d.spread_rate;
        self.delay = d.spread_delay;
        self.sway = d.spread_sway;
        let r = f(self.rate);
        let nyquist = 0.45 * self.rate;
        for hp in &mut self.sway_hp {
            hp.set(Kind::Highpass, f(d.spread_cutoff.min(nyquist)), Q, 0.0, r);
        }
        for hp in &mut self.place_hp {
            hp.set(Kind::Highpass, f(d.place_cutoff.min(nyquist)), Q, 0.0, r);
        }
        self.room_hp.set(Kind::Highpass, f(d.room_cutoff.min(nyquist)), Q, 0.0, r);
        self.wash_cut = d.wash_cutoff > 0.0;
        if self.wash_cut {
            self.wash_hp.set(Kind::Highpass, f(d.wash_cutoff.min(nyquist)), Q, 0.0, r);
        }
    }

    /// The side of the block from `block`, from the channels after their
    /// gains (`outs`) and the rest (`side`, and what `capture` kept), into
    /// `bus`.
    // A frame count as a float, exact for millennia.
    #[allow(clippy::cast_precision_loss)]
    fn mix(&mut self, block: u64, outs: &[Signal; CHANNELS], side: &SideIn<'_>) {
        let rate = self.rate;
        let r32 = f(rate);
        let step = 1.0 - math::exp(-(QUANTUM as f64 / rate) / GLIDE);
        for (now, &want) in self.now.iter_mut().zip(self.want.iter()) {
            *now = math::flush(*now + (want - *now) * step);
        }
        let [room, melody, chords, tex_spread, kit, _, texture, glass, wash] = self.now.map(f);
        let at = |x: &[Signal; CHANNELS], ch: u32| x.get(ch as usize).copied().unwrap_or([0.0; QUANTUM]);
        let mut pumped = [0.0f32; QUANTUM];
        // The spreads: each channel through its swaying line, above the
        // cutoff.
        let (t0, t1) = (block as f64 / rate, (block + QUANTUM as u64) as f64 / rate);
        for (((&ch, line), hp), (&rate_hz, &amount)) in SPREAD
            .iter()
            .zip(self.sway_lines.iter_mut())
            .zip(self.sway_hp.iter_mut())
            .zip(self.rates.iter().zip([melody, chords, tex_spread].iter()))
        {
            if amount == 0.0 {
                continue;
            }
            let wave = |time: f64| self.delay + self.sway * math::sin(core::f64::consts::TAU * rate_hz * time);
            let (a, b) = (wave(t0), wave(t1));
            let mut times = [0.0f32; QUANTUM];
            for (i, x) in times.iter_mut().enumerate() {
                *x = f(a + (b - a) * (i as f64 / QUANTUM as f64)).clamp(0.0, f(SWAY_MAX));
            }
            let mut copy = [0.0f32; QUANTUM];
            line.process(&mut self.memory, &at(outs, ch), &times, r32, &mut copy);
            for (y, &x) in pumped.iter_mut().zip(copy.iter()) {
                *y += hp.step(x) * amount;
            }
            hp.flush();
        }
        // The light texture's whole channel, glass alone, the wash's own.
        let [kit_hp, texture_hp, glass_hp] = &mut self.place_hp;
        if texture != 0.0 {
            for (y, &x) in pumped.iter_mut().zip(at(outs, TEXTURE_CHANNEL).iter()) {
                *y += texture_hp.step(x) * texture;
            }
            texture_hp.flush();
        }
        let gain = |ch: u32| at(&self.gains, ch);
        let (tex_gain, wash_gain) = (gain(TEXTURE_CHANNEL), gain(WASH_CHANNEL));
        if glass != 0.0 {
            for ((y, &x), &g) in pumped.iter_mut().zip(at(side.placed, TEXTURE_CHANNEL).iter()).zip(tex_gain.iter()) {
                *y += glass_hp.step(x) * g * glass;
            }
            glass_hp.flush();
        }
        if wash != 0.0 {
            let cut = self.wash_cut;
            for ((y, &x), &g) in pumped.iter_mut().zip(at(side.own, WASH_CHANNEL).iter()).zip(wash_gain.iter()) {
                let x = if cut { self.wash_hp.step(x) } else { x };
                *y += x * g * wash;
            }
            self.wash_hp.flush();
        }
        for (y, &k) in pumped.iter_mut().zip(self.pump.iter()) {
            *y *= k;
        }
        // The light kit, placed in the drums before their gain.
        let mut drums = [0.0f32; QUANTUM];
        if kit != 0.0 {
            let drum_gain = gain(DRUMS_CHANNEL);
            for ((y, &x), &g) in drums.iter_mut().zip(at(side.placed, DRUMS_CHANNEL).iter()).zip(drum_gain.iter()) {
                *y = kit_hp.step(x) * g;
            }
            kit_hp.flush();
        }
        // The room: the combs through the allpasses, above the cutoff, then
        // the reverb's output gain and the pre-delay, as the mid.
        let mut verb = [0.0f32; QUANTUM];
        if room != 0.0 {
            let n = self.allpass_at;
            for (i, y) in verb.iter_mut().enumerate() {
                let mut x = self.combs[i] * self.comb_gain[i] * room;
                let at = (n + i) % ALLPASS_FRAMES;
                for ((ring, &len), &(_, g)) in self.allpass.iter_mut().zip(self.allpass_len.iter()).zip(ALLPASSES.iter()) {
                    let back = (at + ALLPASS_FRAMES - len) % ALLPASS_FRAMES;
                    let delayed = ring.get(back).copied().unwrap_or(0.0);
                    let w = math::silence(x + g * delayed);
                    if let Some(slot) = ring.get_mut(at) {
                        *slot = w;
                    }
                    x = -g * w + delayed;
                }
                *y = self.room_hp.step(x) * self.reverb_out[i];
            }
            self.room_hp.flush();
        }
        self.allpass_at = (self.allpass_at + QUANTUM) % ALLPASS_FRAMES;
        let mut pre = [0.0f32; QUANTUM];
        let pre_times = [f(PRE_DELAY); QUANTUM];
        self.pre.process(&mut self.memory, &verb, &pre_times, r32, &mut pre);
        for (i, y) in self.bus.iter_mut().enumerate() {
            *y = math::silence(pre[i] * self.tails[i] + pumped[i] + drums[i]);
        }
    }
}

impl Mix {
    /// An empty mix, all zeros (so a core in a static costs nothing in
    /// the `.wasm`); `init` builds it.
    pub const fn new() -> Self {
        Mix {
            rate: 0.0,
            ready: false,
            combs: 0,
            gains: [const { Timeline::new(0.0, 0.0, 0.0) }; CHANNELS],
            pump: Timeline::new(0.0, 0.0, 0.0),
            tails: Timeline::new(0.0, 0.0, 0.0),
            echo_time: Timeline::new(0.0, 0.0, 0.0),
            comb_fb: Timeline::new(0.0, 0.0, 0.0),
            comb_freq: Timeline::new(0.0, 0.0, 0.0),
            comb_sum: Timeline::new(0.0, 0.0, 0.0),
            reverb_out: Timeline::new(0.0, 0.0, 0.0),
            comb_delays: [const { Delay::new() }; 6],
            comb_times: [0.0; 6],
            comb_lps: [const { Biquad::new() }; 6],
            comb_last: [[0.0; QUANTUM]; 6],
            pre_delay: Delay::new(),
            echo: Delay::new(),
            echo_lp: Biquad::new(),
            echo_last: [0.0; QUANTUM],
            memory: [0.0; MEMORY],
            bus: [0.0; QUANTUM],
            side: Side::new(),
        }
    }

    /// Build the graph as `_build()` does at `rate`: six combs on full
    /// quality, three on lite. Everything starts silent.
    pub fn init(&mut self, rate: f64, full: bool) {
        self.rate = rate;
        self.ready = rate > 0.0 && rate <= MAX_RATE;
        let nyquist = f(rate / 2.0);
        let any = (f32::MIN, f32::MAX);
        for (g, &base) in self.gains.iter_mut().zip(GAIN.iter()) {
            g.configure(f(base), any.0, any.1);
        }
        self.pump.configure(1.0, any.0, any.1);
        self.tails.configure(1.0, any.0, any.1);
        self.echo_time.configure(f(ECHO_TIME), 0.0, f(ECHO_MAX));
        self.comb_fb.configure(f(COMB_FEEDBACK), any.0, any.1);
        self.comb_freq.configure(f(COMB_LOWPASS), 0.0, nyquist);
        let times: &[f64] = if full {
            &COMB_TIMES_FULL
        } else {
            &COMB_TIMES_LITE
        };
        self.combs = times.len();
        self.comb_sum
            .configure(f(0.2 / times.len() as f64), any.0, any.1);
        self.reverb_out.configure(f(REVERB_OUT), any.0, any.1);
        let mut at = 0;
        for ((d, t), &time) in self
            .comb_delays
            .iter_mut()
            .zip(self.comb_times.iter_mut())
            .zip(times.iter())
        {
            at = d.place(COMB_MAX, rate, at);
            *t = f(time).max(0.0).min(d.max);
        }
        at = self.pre_delay.place(PRE_DELAY_MAX, rate, at);
        at = self.echo.place(ECHO_MAX, rate, at);
        if at > MEMORY {
            self.ready = false;
        }
        for lp in self.comb_lps.iter_mut() {
            lp.reset();
        }
        self.echo_lp.reset();
        self.echo_lp
            .set(Kind::Lowpass, f(ECHO_LOWPASS), Q, 0.0, f(rate));
        self.comb_last = [[0.0; QUANTUM]; 6];
        self.echo_last = [0.0; QUANTUM];
        self.memory.fill(0.0);
        self.bus = [0.0; QUANTUM];
        self.side.init(rate);
    }

    /// A stereo render's side of the last block (queue item 65): zeros
    /// until a block is mixed with a side.
    #[must_use]
    pub fn side(&self) -> &Signal {
        &self.side.bus
    }

    /// The loop's space (`space::Space::draw`), glided to from the next
    /// block mixed with a side.
    pub fn set_space(&mut self, drawn: &Drawn) {
        self.side.set(drawn);
    }

    /// The last block's bus.
    pub fn bus(&self) -> &Signal {
        &self.bus
    }

    /// Whether the mix can play at this rate (`MAX_RATE`).
    pub fn ready(&self) -> bool {
        self.ready
    }

    /// Move a parameter (`GAINS` and the rest) by `op` (`SET`, `LINEAR`,
    /// `TARGET`, `CANCEL`), as the AudioParam call of the same name: to
    /// `value`, at `time`, with time constant `tau` for a target. `now` is
    /// the host's current time. False if there is no such parameter or it
    /// is full.
    pub fn param(&mut self, id: u32, op: u32, value: f32, time: f64, tau: f64, now: f64) -> bool {
        match id {
            GAINS..=4 => match self.gains.get_mut(id as usize) {
                Some(t) => apply(t, op, value, time, tau, now),
                None => false,
            },
            PUMP => apply(&mut self.pump, op, value, time, tau, now),
            TAILS => apply(&mut self.tails, op, value, time, tau, now),
            ECHO_DELAY => apply(&mut self.echo_time, op, value, time, tau, now),
            COMB_FB => apply(&mut self.comb_fb, op, value, time, tau, now),
            COMB_FREQ => apply(&mut self.comb_freq, op, value, time, tau, now),
            COMB_SUM => apply(&mut self.comb_sum, op, value, time, tau, now),
            REVERB_GAIN => apply(&mut self.reverb_out, op, value, time, tau, now),
            _ => false,
        }
    }

    /// One block: the channels, as the voices and the host left them, in;
    /// the bus out.
    ///
    /// `sends`, if any, is what voices sent past their channels this block
    /// (`block::Sends`: into the reverb, then the echo), added into
    /// reverbIn and echoIn beside the channels' sends, as the sketch's stab
    /// connects its throw straight into echoIn (queue item 60).
    ///
    /// `lift`, if any, is what the arc adds to the echo's feedback, frame by
    /// frame (queue item 63).
    ///
    /// `side`, in a stereo render, makes the side as well (`Mix::side`,
    /// queue item 65); the bus, the mid, is the same either way.
    pub fn process(
        &mut self,
        block: u64,
        channels: &[Signal; CHANNELS],
        sends: Option<&Sends>,
        lift: Option<&Signal>,
        side: Option<&SideIn<'_>>,
    ) -> &Signal {
        if !self.ready {
            self.bus.fill(0.0);
            return &self.bus;
        }
        let rate = self.rate;
        let r32 = f(rate);
        let mut g = [0.0f32; QUANTUM];

        // Each channel through its gain.
        let mut outs = [[0.0f32; QUANTUM]; CHANNELS];
        let stereo = side.is_some();
        for (((out, input), gain), kept) in outs.iter_mut().zip(channels.iter()).zip(self.gains.iter_mut()).zip(self.side.gains.iter_mut()) {
            gain.fill(block, rate, &mut g);
            for ((y, &x), &k) in out.iter_mut().zip(input.iter()).zip(g.iter()) {
                *y = x * k;
            }
            if stereo {
                *kept = g;
            }
        }
        let at = |c: u32| outs.get(c as usize).copied().unwrap_or([0.0; QUANTUM]);

        // The sends, each a gain, summed into reverbIn and echoIn; the
        // pump bus, the four channels the duck presses.
        let mut verb_in = [0.0f32; QUANTUM];
        let mut echo_in = [0.0f32; QUANTUM];
        for &c in ORDER.iter() {
            let ch = at(c);
            let send = |sends: &[f64; CHANNELS]| f(sends.get(c as usize).copied().unwrap_or(0.0));
            let (verb, echo) = (send(&VERB), send(&ECHO));
            for i in 0..QUANTUM {
                verb_in[i] += ch[i] * verb;
                echo_in[i] += ch[i] * echo;
            }
        }
        if let Some([verb, echo]) = sends {
            for (y, x) in verb_in.iter_mut().zip(verb.iter()) {
                *y += *x;
            }
            for (y, x) in echo_in.iter_mut().zip(echo.iter()) {
                *y += *x;
            }
        }
        let mut pumped = [0.0f32; QUANTUM];
        for &c in PUMPED.iter() {
            let ch = at(c);
            for (y, &x) in pumped.iter_mut().zip(ch.iter()) {
                *y += x;
            }
        }
        self.pump.fill(block, rate, &mut g);
        for (y, &k) in pumped.iter_mut().zip(g.iter()) {
            *y *= k;
        }
        if stereo {
            self.side.pump = g;
        }

        // The combs. The lowpass filters the delay's last block, its gain
        // feeds that back beside reverbIn, and the delay renders.
        let n = self.combs;
        let mut fb = [0.0f32; QUANTUM];
        let mut freq = [0.0f32; QUANTUM];
        self.comb_fb.fill(block, rate, &mut fb);
        self.comb_freq.fill(block, rate, &mut freq);
        let mut into = [[0.0f32; QUANTUM]; 6];
        for i in 0..QUANTUM {
            let (first, rest) = self.comb_lps.split_at_mut(1);
            let Some(lead) = first.first_mut() else { break };
            lead.set(Kind::Lowpass, freq[i], Q, 0.0, r32);
            let (verb, gain) = (verb_in[i], fb[i]);
            if let (Some(last), Some(y)) = (self.comb_last.first(), into.first_mut()) {
                y[i] = verb + lead.step(last[i]) * gain;
            }
            for ((lp, last), y) in rest
                .iter_mut()
                .zip(self.comb_last.iter().skip(1))
                .zip(into.iter_mut().skip(1))
                .take(n.saturating_sub(1))
            {
                lp.follow(lead);
                y[i] = verb + lp.step(last[i]) * gain;
            }
        }
        let mut comb_sum = [0.0f32; QUANTUM];
        for ((((lp, delay), &time), last), input) in self
            .comb_lps
            .iter_mut()
            .zip(self.comb_delays.iter_mut())
            .zip(self.comb_times.iter())
            .zip(self.comb_last.iter_mut())
            .zip(into.iter())
            .take(n)
        {
            lp.flush();
            let times = [time; QUANTUM];
            delay.process(&mut self.memory, input, &times, r32, last);
            for (y, &x) in comb_sum.iter_mut().zip(last.iter()) {
                *y += x;
            }
        }
        if stereo {
            self.side.combs = comb_sum;
        }
        self.comb_sum.fill(block, rate, &mut g);
        for (y, &k) in comb_sum.iter_mut().zip(g.iter()) {
            *y *= k;
        }
        if stereo {
            self.side.comb_gain = g;
        }
        self.reverb_out.fill(block, rate, &mut g);
        for (y, &k) in comb_sum.iter_mut().zip(g.iter()) {
            *y *= k;
        }
        if stereo {
            self.side.reverb_out = g;
        }
        let mut pre = [0.0f32; QUANTUM];
        let pre_times = [f(PRE_DELAY); QUANTUM];
        self.pre_delay
            .process(&mut self.memory, &comb_sum, &pre_times, r32, &mut pre);

        // The echo. Its feedback takes the lowpass's last block, beside
        // echoIn; the delay renders, and the lowpass after it.
        let feedback = f(ECHO_FEEDBACK);
        let mut into_echo = [0.0f32; QUANTUM];
        if let Some(lift) = lift {
            for (((y, &x), &last), &up) in into_echo.iter_mut().zip(echo_in.iter()).zip(self.echo_last.iter()).zip(lift.iter()) {
                *y = x + last * (feedback + up);
            }
        } else {
            for ((y, &x), &last) in into_echo.iter_mut().zip(echo_in.iter()).zip(self.echo_last.iter()) {
                *y = x + last * feedback;
            }
        }
        self.echo_time.fill(block, rate, &mut g);
        let mut echoed = [0.0f32; QUANTUM];
        self.echo
            .process(&mut self.memory, &into_echo, &g, r32, &mut echoed);
        for (y, &x) in self.echo_last.iter_mut().zip(echoed.iter()) {
            *y = self.echo_lp.step(x);
        }
        self.echo_lp.flush();

        // Both tails through one gain, and onto the bus with the rest.
        self.tails.fill(block, rate, &mut g);
        let drums = at(DRUMS_CHANNEL);
        for i in 0..QUANTUM {
            let tails = (pre[i] + self.echo_last[i]) * g[i];
            self.bus[i] = tails + pumped[i] + drums[i];
        }
        if let Some(side) = side {
            self.side.tails = g;
            self.side.mix(block, &outs, side);
        }
        &self.bus
    }
}

pub(crate) fn apply<const N: usize>(
    t: &mut Timeline<N>,
    op: u32,
    value: f32,
    time: f64,
    tau: f64,
    now: f64,
) -> bool {
    match op {
        SET => t.set_value_at_time(value, time, now),
        LINEAR => t.linear_ramp_to_value_at_time(value, time, now),
        TARGET => t.set_target_at_time(value, time, tau, now),
        CANCEL => {
            t.cancel_scheduled_values(time, now);
            true
        }
        HOLD => {
            t.cancel_scheduled_values(time, now);
            t.set_value_at_time(t.value(), time, now)
        }
        _ => false,
    }
}

impl Default for Mix {
    fn default() -> Self {
        Self::new()
    }
}

/// The memory the mix needs at `rate`, for a host checking ahead.
pub fn memory_for(rate: f64, full: bool) -> usize {
    let combs = if full { 6 } else { 3 };
    combs * buffer_len(COMB_MAX, rate) + buffer_len(PRE_DELAY_MAX, rate) + buffer_len(ECHO_MAX, rate)
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATE: f64 = 44100.0;

    fn boxed(full: bool) -> Box<Mix> {
        std::thread::Builder::new()
            .stack_size(1 << 25)
            .spawn(move || {
                let mut m = Box::new(Mix::new());
                m.init(RATE, full);
                m
            })
            .and_then(|t| t.join().map_err(|_| std::io::Error::other("panicked")))
            .expect("mix")
    }

    /// A stereo render's side (queue item 65): the mid is the mono mix bit
    /// for bit whatever the side does, a centered space makes no side, and
    /// the side keeps out of the low end (every part of it is highpassed).
    #[test]
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    fn the_side_leaves_the_mid_alone_and_the_low_end_centered() {
        let space = crate::space::Space::new();
        let drawn = space.draw(&crate::space::Loop { seed: 11, busy: 9.0, wet: 0.2, light: true });
        let noise = |b: u64, ch: usize| -> Signal {
            core::array::from_fn(|i| {
                let n = (b * QUANTUM as u64 + i as u64).wrapping_mul(2_654_435_761).wrapping_add(ch as u64 * 977);
                (((n >> 7) % 2000) as f32 / 1000.0 - 1.0) * 0.2
            })
        };
        let low = |b: u64| -> Signal {
            core::array::from_fn(|i| (core::f64::consts::TAU * 60.0 * (b * QUANTUM as u64 + i as u64) as f64 / RATE).sin() as f32 * 0.3)
        };
        let (mut mono, mut stereo, mut centered) = (boxed(true), boxed(true), boxed(true));
        stereo.set_space(&drawn);
        let placed = [[0.0; QUANTUM]; CHANNELS];
        let own = [[0.0; QUANTUM]; CHANNELS];
        let (mut side_lo, mut side_hi) = (0.0f64, 0.0f64);
        for b in 0..2000u64 {
            let mut ch: [Signal; CHANNELS] = core::array::from_fn(|c| noise(b, c));
            // A low tone in the chords: the spread must not carry it.
            for (y, x) in ch[CHORDS as usize].iter_mut().zip(low(b).iter()) {
                *y += *x;
            }
            let a = *mono.process(b * QUANTUM as u64, &ch, None, None, None);
            let input = SideIn { placed: &placed, own: &own, space: &drawn };
            let m = *stereo.process(b * QUANTUM as u64, &ch, None, None, Some(&input));
            let flat = SideIn { placed: &placed, own: &own, space: &CENTERED };
            let _ = centered.process(b * QUANTUM as u64, &ch, None, None, Some(&flat));
            assert!(a.iter().zip(m.iter()).all(|(x, y)| x.to_bits() == y.to_bits()), "block {b}: the mid moved");
            assert!(centered.side().iter().all(|&x| x == 0.0), "block {b}: a centered space made a side");
            if b > 200 {
                for &x in stereo.side() {
                    side_hi += f64::from(x) * f64::from(x);
                }
            }
        }
        assert!(side_hi > 1e-3, "the space made a side: {side_hi}");
        // The low tone alone through the side: a tenth of a percent at most.
        let mut quiet = boxed(true);
        quiet.set_space(&drawn);
        for b in 0..2000u64 {
            let mut ch = [[0.0; QUANTUM]; CHANNELS];
            ch[CHORDS as usize] = low(b);
            let input = SideIn { placed: &placed, own: &own, space: &drawn };
            let _ = quiet.process(b * QUANTUM as u64, &ch, None, None, Some(&input));
            if b > 200 {
                for &x in quiet.side() {
                    side_lo += f64::from(x) * f64::from(x);
                }
            }
        }
        let tone: f64 = (0.3f64 * 0.3 / 2.0) * 1800.0 * QUANTUM as f64;
        assert!(side_lo / tone < 1e-3, "60 Hz in the side: {}", side_lo / tone);
    }

    #[test]
    fn the_memory_holds_every_line_at_the_highest_rate() {
        assert_eq!(memory_for(MAX_RATE, true), MEMORY);
        assert!(memory_for(44100.0, true) < MEMORY);
    }

    #[test]
    fn a_drum_impulse_reaches_the_bus_at_its_gain_then_the_tails_follow() {
        let mut m = boxed(true);
        let mut ch = [[0.0f32; QUANTUM]; CHANNELS];
        ch[DRUMS_CHANNEL as usize][10] = 1.0;
        let first = *m.process(0, &ch, None, None, None);
        assert_eq!(first[10], f(0.82));
        assert!(first[..10].iter().all(|&x| x == 0.0));
        let silent = [[0.0f32; QUANTUM]; CHANNELS];
        // The reverb arrives after its shortest comb and the pre-delay, about
        // 0.05 s; the echo after 0.36 s and a block.
        let mut energy = 0.0f32;
        for b in 1..200 {
            energy += m.process(b * QUANTUM as u64, &silent, None, None, None).iter().map(|x| x * x).sum::<f32>();
        }
        assert!(energy > 1e-4, "{energy}");
    }

    #[test]
    fn a_muted_channel_falls_silent() {
        let mut m = boxed(false);
        assert!(m.param(GAINS + MELODY, TARGET, 0.0, 0.0, 0.03, 0.0));
        let ch = [[1.0f32; QUANTUM]; CHANNELS];
        let mut melody_only = [[0.0f32; QUANTUM]; CHANNELS];
        melody_only[MELODY as usize] = ch[0];
        let mut last = 1.0;
        for b in 0..200 {
            last = m.process(b * QUANTUM as u64, &melody_only, None, None, None)[QUANTUM - 1];
        }
        // Only the tails of what came before are left, and fading.
        assert!(last.abs() < 0.05, "{last}");
    }

    #[test]
    fn unknown_parameters_and_calls_are_refused() {
        let mut m = boxed(true);
        assert!(!m.param(LAST_PARAM + 1, SET, 1.0, 0.0, 0.0, 0.0));
        assert!(!m.param(PUMP, 9, 1.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn too_high_a_rate_has_no_mix() {
        let mut m = boxed(true);
        m.init(2.0 * MAX_RATE, true);
        assert!(!m.ready());
        let ch = [[1.0f32; QUANTUM]; CHANNELS];
        assert!(m.process(0, &ch, None, None, None).iter().all(|&x| x == 0.0));
    }
}
