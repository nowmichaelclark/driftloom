//! The synth's note calls (queue item 82): what `Synth` in `js/synth.js`
//! does with each note the engine asks for, on the core: the voice budget
//! asked as it asks it, the voice's own values worked out as it works them
//! out, and the note handed to the core with the parts that got through.
//! Every voice of the app's engine is here, each with its JavaScript twin's
//! name; the JavaScript voices (the fallback a phone without the core
//! hears) are not, since the player always plays on a core.
//!
//! The order of every ask and every hold is the JavaScript's, so the player
//! refuses exactly the notes the page does; every number a note carries is
//! worked out in the JavaScript's order of operations, on the core's math.

use crate::block::Event;
use crate::block::event::{CHOKE, RELEASE};
use crate::blocks::own;
use crate::blocks::midi_to_freq;
use crate::blocks::{bass, drum, kalimba, struck, texture};
use crate::generator::data::{self, VOICE_COSTS};
use crate::generator::js;
use crate::rng::{Key, salt, unit};
use crate::{BASS_CHANNEL, CHORDS, DRUMS_CHANNEL, EXTRA, MELODY, TEXTURE_CHANNEL, Tap, WASH_CHANNEL};
use crate::{
    ACCORDION, ANALOGLEAD, ANALOGPAD, BASS, BEEP, CHOIR, DRUM, DUBSTAB, FIDDLE, FLUTE, FM, HAT, HUM, KALIMBA, KNOCK, MOOG, NYLON,
    OCARINA, PAD, PANFLUTE, SAWPLUCK, SINE, SOFTPAD, STAB, TEMPLEBELL, TEXTURE, TUBULAR, TUNEDKICK, VOWEL, WHISTLE,
};

use super::budget::{Budget, YIELD_MARGIN};
use super::formant::{self, HUM_REF, OPEN_HUM, Refs, VOWELS};
use super::host::Host;

/// Every voice's values a note leaves out (`x[i] ?? NaN`).
const NONE: [f64; EXTRA] = [f64::NAN; EXTRA];

/// A note's own values: these, then NaN.
fn extra(values: &[f64]) -> [f64; EXTRA] {
    let mut x = NONE;
    for (to, &v) in x.iter_mut().zip(values) {
        *to = v;
    }
    x
}

/// `VOICE_COST[name]`, or `default` for a voice the table leaves out.
fn cost_or(name: &str, default: f64) -> f64 {
    data::lookup(VOICE_COSTS.cost, name).unwrap_or(default)
}

/// `DEFAULT_COST`: what a voice costs that names no cost of its own.
fn default_cost() -> f64 {
    VOICE_COSTS.default
}

// The mix's parameters and their calls (`MIX` and `SET` and the rest in
// js/core.js; core/src/mix.rs and core/src/lib.rs).
const GAINS: u32 = crate::mix::GAINS;
const PUMP: u32 = crate::mix::PUMP;
const TAILS: u32 = crate::mix::TAILS;
const ECHO: u32 = crate::mix::ECHO_DELAY;
const COMB_FB: u32 = crate::mix::COMB_FB;
const COMB_FREQ: u32 = crate::mix::COMB_FREQ;
const COMB_SUM: u32 = crate::mix::COMB_SUM;
const REVERB_OUT: u32 = crate::mix::REVERB_GAIN;
const SET: u32 = crate::mix::SET;
const LINEAR: u32 = crate::mix::LINEAR;
const TARGET: u32 = crate::mix::TARGET;
const CANCEL: u32 = crate::mix::CANCEL;

/// The channels' own levels in the mix (`cfg` in `Synth._build`), in the
/// core's channel order (melody, chords, bass, texture, drums).
pub const CHANNEL_GAIN: [f64; 5] = [0.58, 0.44, 0.62, 0.5, 0.82];
/// The master gain before anyone sets the volume (`userVolume`).
pub const VOLUME: f64 = 0.85;

/// How long the budget holds each drum (`DRUM_HOLD`).
fn drum_hold(inst: &str) -> f64 {
    match inst {
        "kick" => 0.45,
        "softkick" => 0.36,
        "snare" => 0.16,
        "clap" => 0.2,
        "hat" => 0.045,
        "ohat" => 0.26,
        "shaker" => 0.07,
        "rim" => 0.1,
        "frame" => 0.32,
        "tap" => 0.12,
        "jingle" => 0.09,
        "ojingle" => 0.24,
        _ => f64::NAN,
    }
}

/// A hat's kind on the hat voice (`HAT_KINDS`), and a drum's on the drum
/// voice (`DRUM_KINDS`).
fn hat_kind(inst: &str) -> Option<u32> {
    match inst {
        "hat" => Some(struck::HAT),
        "ohat" => Some(struck::OPEN_HAT),
        "shaker" => Some(struck::SHAKER),
        _ => None,
    }
}

fn drum_kind(inst: &str) -> Option<u32> {
    match inst {
        "kick" => Some(drum::KICK),
        "softkick" => Some(drum::SOFTKICK),
        "snare" => Some(drum::SNARE),
        "clap" => Some(drum::CLAP),
        "rim" => Some(drum::RIM),
        "frame" => Some(drum::FRAME),
        "tap" => Some(drum::TAP),
        "jingle" => Some(drum::JINGLE),
        "ojingle" => Some(drum::OPEN_JINGLE),
        _ => None,
    }
}

/// A bass voice's kind (`BASS_KINDS`): a name not here is the original,
/// `sub`.
fn bass_kind(voice: &str) -> u32 {
    match voice {
        "round" => bass::ROUND,
        "fifths" => bass::FIFTHS,
        "pluckbass" => bass::PLUCKBASS,
        "moogbass" => bass::MOOGBASS,
        _ => bass::SUB,
    }
}

/// How long the tuned kick is said to ring at most (`TUNED_KICK_RINGS`).
const TUNED_KICK_RINGS: f64 = 2.5;
/// The longest sung note the core takes (`SUNG_LONGEST`).
const SUNG_LONGEST: f64 = 13.5;
/// What a moving vowel costs on top (`VOWEL_DRIFT_COST`).
const VOWEL_DRIFT_COST: f64 = 5.0;

/// The tuned kick a hit plays (`_coreKick`'s `kick`): its pitch, and the
/// fall this pass plays, or today's kick (a wandering pass's swap).
#[derive(Clone, Copy)]
pub struct KickHit {
    /// None: today's kick, choked into.
    pub fall: Option<crate::generator::choices::Fall>,
    /// Its pitch, in Hz.
    pub hz: f64,
}

/// An FM note's options (`fm`'s `opts`): what the voice asks, the rest at
/// `fm`'s defaults.
#[derive(Clone, Copy)]
pub struct Fm {
    /// Asks the budget's soft ceiling (a pad's or a texture's).
    pub soft: bool,
    /// The core's channel it plays into.
    pub channel: u32,
    /// The modulator's ratio to the carrier, and its depth in Hz.
    pub ratio: f64,
    /// See `ratio`.
    pub index: f64,
    /// The attack, in seconds.
    pub attack: f64,
    /// None: the note's own length.
    pub decay: Option<f64>,
    /// Cents off the note.
    pub detune: f64,
    /// None: `DEFAULT_COST`.
    pub cost: Option<f64>,
    /// Asks for room and holds nothing (glass).
    pub yields: bool,
    /// Asks nothing and holds nothing (rhodesbass: `bass` holds for it).
    pub skip_budget: bool,
}

impl Fm {
    /// `fm`'s defaults, into `channel`.
    #[must_use]
    pub const fn into(channel: u32) -> Self {
        Fm { soft: false, channel, ratio: 2.0, index: 220.0, attack: 0.006, decay: None, detune: 0.0, cost: None, yields: false, skip_budget: false }
    }
}

/// A sung note's or a chord voice's options (`voice`'s `opts`).
#[derive(Clone, Copy, Default)]
pub struct Opts {
    /// The vowel a sung voice sings ("a", "e", "o", "u"; none: its own).
    pub vowel: Option<&'static str>,
    /// Cents off the note (none: the voice's own).
    pub detune: Option<f64>,
    /// The note before, when this one is joined to it: a held voice
    /// (bowed, fiddle, accordion) slurs from it.
    pub prev: Option<f64>,
    /// A chord's string, strummed (the nylon plays it as a strum).
    pub strum: bool,
    /// Asks for room and holds nothing (a part that yields).
    pub yields: bool,
}

/// dubStab's settings for the loop (`stabOf`'s `opts`, by its names: the
/// stab's decay, its filter's opening, its saw's share, the echo's throw,
/// its tone, its reverb send and its level).
#[derive(Clone, Copy)]
#[allow(missing_docs)] // the struct's doc names them, as stabOf does
pub struct StabOpts {
    pub decay: f64,
    pub open: f64,
    pub saw: f64,
    pub throw: f64,
    pub tone: f64,
    pub verb: f64,
    pub level: f64,
}

/// The synth's side of the player: the budget, the quality, the rate's
/// references for the sung trims, and what the loop's own instruments are
/// set to (whether the organ, the wash, subDuck and padSweep are set up).
// Its flags are the synth's own, each a state of its own (an instrument set
// up for the loop), not one state spelled in bools.
#[allow(clippy::struct_excessive_bools)]
pub struct Synth {
    /// The voice budget (`_budget`, `_room`, `_releases`).
    pub budget: Budget,
    refs: Refs,
    lite: bool,
    organ: bool,
    wash: bool,
    wash_routed: bool,
    sub_duck: bool,
    pad_sweep: bool,
    /// The space recipe's settings the last loop pinned (`_spacePinned`).
    space_pinned: [bool; crate::space::n::COUNT],
    /// Each channel muted (`_asked.mutes`), in the core's channel order.
    pub mutes: [bool; 5],
    /// The listener's volume (`_gain`).
    pub volume: f64,
    /// The profile's level (`_gain`).
    pub level: f64,
}

impl Synth {
    /// A synth at rest.
    #[must_use]
    pub const fn new() -> Self {
        Synth {
            budget: Budget::new(),
            refs: Refs::new(),
            lite: false,
            organ: false,
            wash: false,
            wash_routed: false,
            sub_duck: false,
            pad_sweep: false,
            space_pinned: [false; crate::space::n::COUNT],
            mutes: [false; 5],
            volume: VOLUME,
            level: 1.0,
        }
    }

    /// A new synth on a core started at `rate`, `lite` or full (`new
    /// Synth`, then its core handed over, `_mixInCore`): the LFOs started
    /// and the master gain where it stands, at `now`.
    pub fn start<H: Host>(&mut self, host: &mut H, rate: f64, lite: bool, now: f64) {
        self.budget.reset(lite);
        self.refs.at(rate);
        self.lite = lite;
        self.organ = false;
        self.wash = false;
        self.wash_routed = false;
        self.sub_duck = false;
        self.pad_sweep = false;
        self.mutes = [false; 5];
        self.level = 1.0;
        host.param(crate::LFO_START, SET, 0.0, now, 0.0);
        host.param(crate::VOLUME, SET, self.gain(), 0.0, 0.0);
    }

    fn gain(&self) -> f64 {
        self.volume * self.level
    }

    // ------------------------------------------------------------ the mix

    /// `setTone(tone)`: warmth, space and wobble into the tone lowpass, the
    /// saturator, the reverb and the wobble, gliding from `now`.
    pub fn set_tone<H: Host>(&mut self, host: &mut H, tone: [f64; 3], now: f64) {
        let [warmth, space, wobble] = tone;
        let fb = js::min(0.74, 0.66 + space * 0.2);
        let combs = if self.lite { 3.0 } else { 6.0 };
        host.param(crate::TONE_FREQ, TARGET, 2600.0 + (1.0 - warmth) * 9000.0, now, 0.2);
        host.param(crate::SAT_DRIVE, SET, 0.55 + warmth * 1.25, now, 0.0);
        host.param(REVERB_OUT, TARGET, 0.4 + space * 0.9, now, 0.2);
        host.param(COMB_FB, TARGET, fb, now, 0.2);
        host.param(COMB_FREQ, TARGET, 1400.0 + space * 2600.0, now, 0.2);
        host.param(COMB_SUM, TARGET, (1.0 - fb) / combs, now, 0.2);
        host.param(crate::WOW_DEPTH, TARGET, 0.0004 + wobble * 0.0038, now, 0.2);
        host.param(crate::FLUTTER_DEPTH, TARGET, 0.00004 + wobble * 0.0005, now, 0.2);
    }

    /// `setCharacterLevel(level)`, or the volume (`setVolume`): the master
    /// gain glides to the product.
    pub fn set_level<H: Host>(&mut self, host: &mut H, level: f64, now: f64) {
        self.level = level;
        host.param(crate::VOLUME, TARGET, self.gain(), now, 0.08);
    }

    /// `setVolume(v)`.
    pub fn set_volume<H: Host>(&mut self, host: &mut H, volume: f64, now: f64) {
        self.volume = volume;
        host.param(crate::VOLUME, TARGET, self.gain(), now, 0.08);
    }

    /// `setEchoTime(seconds)`.
    pub fn set_echo<H: Host>(host: &mut H, seconds: f64, now: f64) {
        host.param(ECHO, TARGET, js::min(1.9, seconds), now, 0.05);
    }

    /// `setMute(layer, muted)`, by the core's channel: its gain glides to 0
    /// or back, and the wash listens to it as the mix plays it.
    pub fn set_mute<H: Host>(&mut self, host: &mut H, channel: usize, muted: bool, now: f64) {
        let Some(&g) = CHANNEL_GAIN.get(channel) else { return };
        if let Some(m) = self.mutes.get_mut(channel) {
            *m = muted;
        }
        #[allow(clippy::cast_possible_truncation)] // a channel, under 5
        let c = channel as u32;
        host.param(GAINS + c, TARGET, if muted { 0.0 } else { g }, now, 0.03);
        if self.wash {
            host.set(WASH_CHANNEL, own::WASH, crate::blocks::wash::set::WEIGHT + channel, if muted { 0.0 } else { g }, now);
        }
    }

    /// `fadeTails(time)`: the reverb and the echo ring for a moment, then
    /// fall to nothing (a bar the loop leaves silent).
    pub fn fade_tails<H: Host>(host: &mut H, time: f64) {
        host.param(TAILS, CANCEL, 0.0, time, 0.0);
        host.param(TAILS, TARGET, 0.0001, time + 0.7, 1.1 / 3.0);
    }

    /// `restoreTails(time)`.
    pub fn restore_tails<H: Host>(host: &mut H, time: f64) {
        host.param(TAILS, CANCEL, 0.0, time, 0.0);
        host.param(TAILS, TARGET, 1.0, time, 0.08);
    }

    /// `duck(time, amount)`: the kick presses the mix down and lets it
    /// breathe back.
    pub fn duck<H: Host>(host: &mut H, time: f64, amount: f64) {
        if amount == 0.0 || amount.is_nan() {
            return;
        }
        host.param(PUMP, CANCEL, 0.0, time, 0.0);
        host.param(PUMP, SET, js::max(0.1, 1.0 - amount), time, 0.0);
        host.param(PUMP, LINEAR, 1.0, time + 0.24, 0.0);
    }

    // ---------------------------------------------- the loop's instruments

    /// `organSetup(values, time)`: the chapel organ's settings for the loop.
    pub fn organ_setup<H: Host>(&mut self, host: &mut H, values: Option<&[f64; crate::generator::load::ORGAN_SETTINGS]>, time: f64) {
        self.organ = values.is_some();
        if let Some(values) = values {
            for (n, &v) in values.iter().enumerate() {
                host.set(CHORDS, own::ORGAN, n, v, time);
            }
        }
    }

    /// `organChord(notes, time)`: the organ moves to a chord (six notes at
    /// most), a note the two share sounding on.
    pub fn organ_chord<H: Host>(&self, host: &mut H, notes: &[i16], time: f64) {
        if !self.organ {
            return;
        }
        let mut chord = [f64::NAN; crate::block::event::NOTES];
        for (c, &n) in chord.iter_mut().zip(notes) {
            *c = f64::from(n);
        }
        host.event(CHORDS, Event::retune(time, &chord));
    }

    /// `organOff(time, fall)`: the organ lets go.
    pub fn organ_off<H: Host>(host: &mut H, time: f64, fall: f64) {
        host.event(CHORDS, Event::new(RELEASE, time, fall));
    }

    /// `washSetup(o, time)`: the wash for the loop, or none (the last one
    /// let go).
    pub fn wash_setup<H: Host>(&mut self, host: &mut H, wash: Option<&crate::generator::load::Wash>, time: f64) {
        let had = self.wash;
        self.wash = wash.is_some();
        match wash {
            Some(o) => {
                // The weights and routes first: the period starts the swells.
                for (c, &g) in CHANNEL_GAIN.iter().enumerate() {
                    host.set(WASH_CHANNEL, own::WASH, crate::blocks::wash::set::WEIGHT + c, if self.mutes[c] { 0.0 } else { g }, time);
                    if !self.wash_routed {
                        #[allow(clippy::cast_possible_truncation)] // a channel, under 5
                        host.route(WASH_CHANNEL, own::WASH, c, Some(Tap { channel: c as u32, block: None }), time);
                    }
                }
                self.wash_routed = true;
                let settings = [o.period, o.level, o.tint, o.tint_gain, o.tint_high, o.tint_q];
                for n in (0..settings.len()).rev() {
                    host.set(WASH_CHANNEL, own::WASH, n, settings[n], time);
                }
            }
            None if had => host.set(WASH_CHANNEL, own::WASH, 0, 0.0, time),
            None => {}
        }
    }

    /// `subDuckSetup(o, time)`: the bass dips at each kick, keyed by the
    /// drums routed in; or none (the route undone).
    pub fn sub_duck_setup<H: Host>(&mut self, host: &mut H, duck: Option<&crate::generator::choices::SubDuck>, time: f64) {
        let had = self.sub_duck;
        self.sub_duck = duck.is_some();
        match duck {
            Some(o) => {
                host.set(BASS_CHANNEL, own::SUB_DUCK, 0, o.depth, time);
                host.set(BASS_CHANNEL, own::SUB_DUCK, 1, o.recover, time);
                host.route(BASS_CHANNEL, own::SUB_DUCK, 0, Some(Tap { channel: DRUMS_CHANNEL, block: None }), time);
            }
            None if had => {
                host.set(BASS_CHANNEL, own::SUB_DUCK, 0, 0.0, time);
                host.route(BASS_CHANNEL, own::SUB_DUCK, 0, None, time);
            }
            None => {}
        }
    }

    /// `padSweepSetup(o, time)`: the chords' lowpass swinging, `oct` last
    /// (it starts the sweep); or none (the last one glides open).
    pub fn pad_sweep_setup<H: Host>(&mut self, host: &mut H, sweep: Option<&crate::generator::choices::PadSweep>, time: f64) {
        let had = self.pad_sweep;
        self.pad_sweep = sweep.is_some();
        // PAD_SWEEP_SETTINGS: lo, oct, period, q.
        match sweep {
            Some(o) => {
                host.set(CHORDS, own::PAD_SWEEP, 0, o.lo, time);
                host.set(CHORDS, own::PAD_SWEEP, 2, o.period, time);
                host.set(CHORDS, own::PAD_SWEEP, 3, o.q, time);
                host.set(CHORDS, own::PAD_SWEEP, 1, o.oct, time);
            }
            None if had => host.set(CHORDS, own::PAD_SWEEP, 1, 0.0, time),
            None => {}
        }
    }

    /// `arcSetup(o, time)`: the master's lowpass breathing over minutes, or
    /// none.
    pub fn arc_setup<H: Host>(host: &mut H, arc: Option<&crate::generator::choices::Arc>, time: f64) {
        if let Some(o) = arc {
            host.param(crate::ARC_FROM, SET, o.from, time, 0.0);
            host.param(crate::ARC_SHAPE, SET, if o.shape == "dip" { 1.0 } else { 0.0 }, time, 0.0);
            host.param(crate::ARC_FLOOR, SET, o.floor, time, 0.0);
            host.param(crate::ARC_DIP, SET, o.dip, time, 0.0);
        }
        host.param(crate::ARC_PERIOD, SET, arc.map_or(0.0, |o| o.period), time, 0.0);
    }

    /// `spaceSetup(o, time)`: what the core needs to place the loop in a
    /// stereo render. A request's space pins (queue item 85b) narrow the
    /// recipe first (`Core::space_range`, read as the start arrives), and a
    /// setting the last loop pinned and this one doesn't goes back to its
    /// range.
    pub fn space_setup<H: Host>(&mut self, host: &mut H, space: &crate::generator::load::Space, time: f64) {
        for (i, s) in crate::space::SETTINGS.iter().enumerate() {
            if self.space_pinned[i] && space.pins[i].is_none() {
                host.space_range(i, s.lo, s.hi);
            }
        }
        for (i, v) in space.pins.iter().enumerate() {
            if let &Some(v) = v {
                host.space_range(i, v, v);
            }
            self.space_pinned[i] = v.is_some();
        }
        host.param(crate::SPACE_SEED, SET, f64::from(space.seed), time, 0.0);
        host.param(crate::SPACE_BUSY, SET, space.busy, time, 0.0);
        host.param(crate::SPACE_WET, SET, space.wet, time, 0.0);
        host.param(crate::SPACE_LIGHT, SET, if space.light { 1.0 } else { 0.0 }, time, 0.0);
        host.param(crate::SPACE_START, SET, 1.0, time, 0.0);
    }

    // -------------------------------------------------------------- notes

    fn note<H: Host>(&mut self, host: &mut H, key: Key, voice: u32, channel: u32, time: f64, midi: f64, dur: f64, vel: f64, parts: u32, x: [f64; EXTRA]) {
        // The voice call notes the channel (`_toCore`, queue item 85c).
        self.budget.note_channel(channel);
        host.key(key);
        host.note(voice, channel, time, midi, dur, vel, parts, x);
    }

    /// `drum(inst, time, vel, kick)`: a hit of the kit, the tuned kick
    /// (`kick`) in a loop that plays one.
    pub fn drum<H: Host>(&mut self, host: &mut H, key: Key, inst: &str, time: f64, vel: f64, kick: Option<KickHit>) {
        let cost = cost_or(inst, 1.5);
        if !self.budget.ask(host, time, false, cost) {
            return;
        }
        let v = js::max(0.0, js::min(1.0, vel));
        if let Some(k) = kick {
            // Every kick of a loop that plays the tuned kick chokes the last.
            host.event(DRUMS_CHANNEL, Event::new(CHOKE, time, 0.0));
            if let Some(f) = k.fall {
                let soft = inst == "softkick";
                let x = extra(&[k.hz, f.dbps, if soft { f.soft_lift } else { f.lift }, if soft { f.soft_tail } else { f.tail }]);
                self.note(host, key, TUNEDKICK, DRUMS_CHANNEL, time, 0.0, TUNED_KICK_RINGS, v, drum_kind(inst).unwrap_or(0), x);
                self.budget.hold(time, drum_hold(inst), cost);
                return;
            }
        }
        if let Some(hat) = hat_kind(inst) {
            self.note(host, key, HAT, DRUMS_CHANNEL, time, 0.0, 0.0, v, hat, NONE);
        } else if let Some(kind) = drum_kind(inst) {
            self.note(host, key, DRUM, DRUMS_CHANNEL, time, 0.0, 0.0, v, kind, NONE);
        } else {
            return;
        }
        self.budget.hold(time, drum_hold(inst), cost);
    }

    /// `bass(midi, time, dur, vel, glide, voice, chug)`.
    #[allow(clippy::too_many_arguments)] // the bass note's own fields
    pub fn bass<H: Host>(&mut self, host: &mut H, key: Key, midi: f64, time: f64, dur: f64, vel: f64, glide: bool, voice: &str, chug: bool) {
        let cost = cost_or(voice, 8.0);
        if !self.budget.ask(host, time, false, cost) {
            return;
        }
        let exact = chug && voice != "fifths" && voice != "rhodesbass";
        let hold = dur + if exact { 0.4 } else { 0.8 };
        if voice == "rhodesbass" {
            // An FM note on the bass channel (its TRIM, 0.8), held here.
            let fm = Fm { ratio: 1.0, index: 130.0, decay: Some(0.4), attack: 0.006, skip_budget: true, ..Fm::into(BASS_CHANNEL) };
            self.fm(host, key, midi, time, dur, vel * 0.8, &fm);
        } else {
            let parts = bass_kind(voice) | if glide { bass::GLIDE } else { 0 } | if chug { bass::CHUG } else { 0 };
            self.note(host, key, BASS, BASS_CHANNEL, time, midi, dur, vel, parts, NONE);
        }
        self.budget.hold(time, hold, cost);
    }

    /// `fm(midi, time, dur, vel, opts)`: two-operator FM, the five options
    /// resolved.
    pub fn fm<H: Host>(&mut self, host: &mut H, key: Key, midi: f64, time: f64, dur: f64, vel: f64, o: &Fm) {
        let cost = o.cost.unwrap_or_else(default_cost);
        let holds = !o.skip_budget && !o.yields;
        if o.yields {
            if !self.budget.room(time, cost + YIELD_MARGIN) {
                return;
            }
        } else if holds && !self.budget.ask(host, time, o.soft, cost) {
            return;
        }
        let decay = o.decay.unwrap_or(dur);
        self.note(host, key, FM, o.channel, time, midi, dur, vel, 0, extra(&[o.ratio, o.index, o.attack, decay, o.detune]));
        if holds {
            self.budget.hold(time, dur + 1.2, cost);
        }
    }

    /// `pad(notes, time, dur, vel)`: each note the budget lets through, with
    /// the chord's size for the level; the first refused ends the chord.
    pub fn pad<H: Host>(&mut self, host: &mut H, key: Key, notes: &[i16], time: f64, dur: f64, vel: f64, channel: u32) {
        let cost = cost_or("pad", default_cost());
        for &midi in notes {
            if !self.budget.ask(host, time, true, cost) {
                return;
            }
            self.note(host, key, PAD, channel, time, f64::from(midi), dur, vel, 0, extra(&[js::num(notes.len()), f64::NAN, f64::NAN, f64::NAN]));
            self.budget.hold(time, dur + 1.6, cost);
        }
    }

    /// `dubStab(notes, time, vel, o)`: a soft chord stab (three notes at
    /// most) that yields.
    pub fn dub_stab<H: Host>(&mut self, host: &mut H, key: Key, notes: &[f64], time: f64, vel: f64, o: &StabOpts) {
        if !self.budget.room(time, VOICE_COSTS.dubstab + YIELD_MARGIN) {
            return;
        }
        let dur = 0.022 + o.decay * 1.8;
        let x = extra(&[
            notes.get(1).copied().unwrap_or(f64::NAN),
            notes.get(2).copied().unwrap_or(f64::NAN),
            o.decay,
            o.open,
            o.saw,
            o.throw,
            o.tone,
            o.verb,
            o.level,
        ]);
        self.note(host, key, DUBSTAB, CHORDS, time, notes.first().copied().unwrap_or(0.0), dur, vel, 0, x);
    }

    /// `texture(kind, notes, time, dur, vel, opts)`: the air layer.
    pub fn texture<H: Host>(&mut self, host: &mut H, key: Key, kind: &str, notes: &[i16], time: f64, dur: f64, vel: f64, band: Option<f64>) {
        let ch = TEXTURE_CHANNEL;
        match kind {
            "swell" => {
                for &midi in notes {
                    if !self.budget.ask(host, time, true, default_cost()) {
                        return;
                    }
                    self.note(host, key, TEXTURE, ch, time, f64::from(midi), dur, vel, texture::SWELL, NONE);
                    self.budget.hold(time, dur + 0.2, default_cost());
                }
            }
            "bell" | "chime" => {
                let first = notes.first().map_or(f64::NAN, |&n| f64::from(n));
                let fm = if kind == "bell" {
                    Fm { ratio: 5.1, index: 300.0, decay: Some(1.1), cost: Some(cost_or("bell", default_cost())), ..Fm::into(ch) }
                } else {
                    Fm { ratio: 2.76, index: 230.0, decay: Some(1.6), attack: 0.004, cost: Some(cost_or("chime", default_cost())), ..Fm::into(ch) }
                };
                self.fm(host, key, first, time, dur, vel, &fm);
            }
            "drop" => {
                if !self.budget.ask(host, time, true, default_cost()) {
                    return;
                }
                self.note(host, key, TEXTURE, ch, time, 0.0, dur, vel, texture::DROP, NONE);
                self.budget.hold(time, 0.15, default_cost());
            }
            "wind" => {
                if !self.budget.ask(host, time, true, default_cost()) {
                    return;
                }
                let band = band.filter(|&b| b != 0.0 && !b.is_nan()).unwrap_or(f64::NAN);
                self.note(host, key, TEXTURE, ch, time, 0.0, dur, vel, texture::WIND, extra(&[f64::NAN, band]));
                self.budget.hold(time, dur + 1.0, default_cost());
            }
            "waves" => {
                let cost = cost_or("waves", default_cost());
                if !self.budget.ask(host, time, true, cost) {
                    return;
                }
                self.note(host, key, TEXTURE, ch, time, 0.0, dur, vel, texture::WAVES, NONE);
                self.budget.hold(time, dur + 0.1, cost);
            }
            _ => {}
        }
    }

    /// One note of a voice the budget asks once for (`_coreNote` after
    /// `_budget`): `cost`, soft or not, the note, its hold.
    #[allow(clippy::too_many_arguments)] // the note's own fields
    fn held<H: Host>(&mut self, host: &mut H, key: Key, voice: u32, channel: u32, time: f64, midi: f64, dur: f64, vel: f64, soft: bool, cost: f64, tail: f64, x: [f64; EXTRA]) {
        if !self.budget.ask(host, time, soft, cost) {
            return;
        }
        self.note(host, key, voice, channel, time, midi, dur, vel, 0, x);
        self.budget.hold(time, tail, cost);
    }

    /// `voice(name, midi, time, dur, vel, out, opts)`: one entry for every
    /// tuned sound, into `channel` (the chords' channel asks under the soft
    /// cap: accompaniment yields before the tune).
    #[allow(clippy::too_many_arguments, clippy::too_many_lines)] // one arm a voice, as `voice` is
    pub fn voice<H: Host>(&mut self, host: &mut H, key: Key, name: &str, midi: f64, time: f64, dur: f64, vel: f64, channel: u32, opts: &Opts) {
        let soft = channel == CHORDS;
        let prev = opts.prev.unwrap_or(f64::NAN);
        let fm = |o: Fm| Fm { soft, ..o };
        match name {
            "harp" => {
                self.fm(host, key, midi, time, dur * 0.8, vel, &fm(Fm { ratio: 3.0, index: 200.0, decay: Some(0.35), cost: Some(13.0), ..Fm::into(channel) }));
                let second = Fm { ratio: 3.0, index: 140.0, decay: Some(0.3), detune: 7.0, cost: Some(7.0), ..Fm::into(channel) };
                self.fm(host, key, midi, time + 0.006, dur * 0.6, vel * 0.4, &fm(second));
            }
            "ocarina" | "flute" => {
                let voice = if name == "flute" { FLUTE } else { OCARINA };
                let cost = cost_or(name, f64::NAN);
                self.held(host, key, voice, channel, time, midi, dur, vel, soft, cost, dur + 0.4, extra(&[prev]));
            }
            "fiddle" => {
                self.held(host, key, FIDDLE, channel, time, midi, dur, vel, soft, cost_or("fiddle", f64::NAN), dur + 0.4, extra(&[prev]));
            }
            "accordion" => {
                let cost = cost_or("accordion", f64::NAN);
                self.held(host, key, ACCORDION, channel, time, midi, dur, vel, soft, cost, dur + 0.35, extra(&[prev]));
            }
            "nylon" => {
                let x = extra(&[if opts.strum { 1.0 } else { 0.0 }]);
                self.held(host, key, NYLON, channel, time, midi, dur, vel, soft, cost_or("nylon", f64::NAN), dur + 0.4, x);
            }
            "panflute" => {
                self.held(host, key, PANFLUTE, channel, time, midi, dur, vel, soft, cost_or("panflute", f64::NAN), dur + 0.4, NONE);
            }
            "piano" => {
                let first = Fm { ratio: 1.0, index: 340.0, decay: Some(0.16), attack: 0.002, cost: Some(19.0), ..Fm::into(channel) };
                self.fm(host, key, midi, time, dur, vel * 0.9, &fm(first));
                let second = Fm { ratio: 1.0, index: 120.0, decay: Some(0.1), detune: 4.0, cost: Some(7.0), ..Fm::into(channel) };
                self.fm(host, key, midi + 12.0, time, dur * 0.5, vel * 0.16, &fm(second));
            }
            "analogpad" => {
                let cost = cost_or("analogpad", f64::NAN);
                self.held(host, key, ANALOGPAD, channel, time, midi, dur, vel, true, cost, dur + 1.6, NONE);
            }
            "analoglead" => {
                let cost = cost_or("analoglead", f64::NAN);
                self.held(host, key, ANALOGLEAD, channel, time, midi, dur, vel, soft, cost, dur + 0.8, NONE);
            }
            "prepared" => {
                let strike = Fm { ratio: 1.0, index: 200.0, decay: Some(0.1), attack: 0.002, cost: Some(15.0), ..Fm::into(channel) };
                self.fm(host, key, midi, time, dur * 0.7, vel * 0.85, &fm(strike));
                if !self.budget.ask(host, time, soft, 3.0) {
                    return;
                }
                self.note(host, key, KNOCK, channel, time, midi, 0.05, vel, 0, NONE);
                let detuned = Fm { ratio: 1.0, index: 90.0, decay: Some(0.08), detune: 9.0, cost: Some(7.0), ..Fm::into(channel) };
                self.fm(host, key, midi, time + 0.004, dur * 0.45, vel * 0.2, &fm(detuned));
                self.budget.hold(time, dur + 0.4, 3.0);
            }
            "celeste" => {
                let cost = cost_or("celeste", default_cost());
                self.fm(host, key, midi, time, dur, vel * 0.9, &fm(Fm { ratio: 4.0, index: 200.0, decay: Some(0.5), cost: Some(cost), ..Fm::into(channel) }));
            }
            "stab" => {
                let holds = !opts.yields;
                if holds && !self.budget.ask(host, time, soft, default_cost()) {
                    return;
                }
                let len = js::min(dur, 0.22);
                self.note(host, key, STAB, channel, time, midi, dur, vel, 0, NONE);
                if holds {
                    self.budget.hold(time, len + 0.2, default_cost());
                }
            }
            "kalimba" => {
                let mut parts = 0;
                if self.budget.ask(host, time, soft, 8.0) {
                    parts |= kalimba::STRIKE;
                    self.budget.hold(time, js::min(dur, 1.1) + 1.2, 8.0);
                }
                if self.budget.ask(host, time, soft, 3.0) {
                    parts |= kalimba::BODY;
                    self.budget.hold(time, 0.25, 3.0);
                }
                if parts != 0 {
                    self.note(host, key, KALIMBA, channel, time, midi, dur, vel, parts, NONE);
                }
            }
            "marimba" => {
                let o = Fm { ratio: 4.0, index: 190.0, decay: Some(0.16), attack: 0.003, cost: Some(9.0), ..Fm::into(channel) };
                self.fm(host, key, midi, time, js::min(dur, 1.6), vel, &fm(o));
            }
            "vowel" | "hum" | "choir" => self.sung(host, key, name, midi, time, dur, vel, channel, opts),
            "templebell" => {
                let hold = js::max(dur, 6.5);
                let cost = cost_or("templebell", f64::NAN);
                self.held(host, key, TEMPLEBELL, channel, time, midi, dur, vel, soft, cost, hold + 1.4, NONE);
            }
            "tubular" => {
                let hold = js::max(dur, 5.0);
                let cost = cost_or("tubular", f64::NAN);
                self.held(host, key, TUBULAR, channel, time, midi, dur, vel, soft, cost, hold + 1.2, NONE);
            }
            "musicbox" => {
                let cost = cost_or("musicbox", default_cost());
                self.fm(host, key, midi, time, dur * 0.9, vel, &fm(Fm { ratio: 5.1, index: 420.0, decay: Some(0.45), cost: Some(cost), ..Fm::into(channel) }));
            }
            "rhodes" => {
                let cost = cost_or("rhodes", default_cost());
                let o = Fm { ratio: 2.0, index: 190.0, decay: Some(0.5), attack: 0.004, cost: Some(cost), ..Fm::into(channel) };
                self.fm(host, key, midi, time, dur, vel, &fm(o));
            }
            "moog" | "whistle" => {
                let whistle = name == "whistle";
                let x = if whistle { extra(&[prev]) } else { NONE };
                let voice = if whistle { WHISTLE } else { MOOG };
                self.held(host, key, voice, channel, time, midi, dur, vel, soft, cost_or(name, f64::NAN), dur + 0.5, x);
            }
            "softpad" => {
                let cost = cost_or("pad", default_cost());
                self.held(host, key, SOFTPAD, channel, time, midi, dur, vel, true, cost, dur + 1.8, NONE);
            }
            "sine" => {
                self.held(host, key, SINE, channel, time, midi, dur, vel, soft, default_cost(), dur + 1.0, NONE);
            }
            #[allow(clippy::cast_possible_truncation)] // a MIDI note, whole
            "moogpad" => self.pad(host, key, &[midi as i16], time, dur, vel, channel),
            _ => self.pluck(host, key, name, midi, time, dur, vel, soft, channel),
        }
    }

    /// `pluck(midi, time, dur, vel, voice, soft, out)`: the voices `voice`
    /// leaves to it.
    #[allow(clippy::too_many_arguments)] // the note's own fields
    fn pluck<H: Host>(&mut self, host: &mut H, key: Key, name: &str, midi: f64, time: f64, dur: f64, vel: f64, soft: bool, channel: u32) {
        match name {
            "bell" => {
                let cost = cost_or("bell", default_cost());
                let o = Fm { soft, ratio: 3.51, index: 420.0, decay: Some(0.5), cost: Some(cost), ..Fm::into(channel) };
                self.fm(host, key, midi, time, dur * 0.9, vel, &o);
            }
            "keys" => {
                let cost = cost_or("keys", default_cost());
                let o = Fm { soft, ratio: 2.0, index: 260.0, decay: Some(0.4), cost: Some(cost), ..Fm::into(channel) };
                self.fm(host, key, midi, time, dur, vel, &o);
            }
            "saw" => {
                let cost = cost_or("saw", f64::NAN);
                self.held(host, key, SAWPLUCK, channel, time, midi, dur, vel, soft, cost, dur + 0.6, NONE);
            }
            _ => {
                let cost = cost_or("pluck", f64::NAN);
                self.held(host, key, BEEP, channel, time, midi, dur, vel, soft, cost, dur + 0.5, NONE);
            }
        }
    }

    /// The sung voices (`vowel`, `hum`, `choir`): whether a held vowel
    /// moves, and to where (the page's draws, from the note's key), what
    /// that costs, and the note's trim.
    #[allow(clippy::too_many_arguments)] // the note's own fields
    fn sung<H: Host>(&mut self, host: &mut H, key: Key, name: &str, midi: f64, time: f64, dur: f64, vel: f64, channel: u32, opts: &Opts) {
        let humming = name == "hum";
        let choral = name == "choir";
        // `opts.vowel || 'a'`, and its number (`SUNG_VOWELS`).
        let vowel_key = opts.vowel.filter(|v| !v.is_empty()).unwrap_or("a");
        let vowel_index = match vowel_key {
            "a" => Some(0),
            "e" => Some(1),
            "o" => Some(2),
            "u" => Some(3),
            _ => None,
        };
        let formants = if humming { formant::HUM } else { VOWELS[vowel_index.unwrap_or(0)] };
        let held = js::min(1.0, js::max(0.0, (dur - 0.5) / 1.5));
        // `_hostDraw`: from the note's key, on the chords' or the melody's.
        let layer = if channel == CHORDS { CHORDS } else { MELODY };
        let drifts = held > 0.0 && unit(key, layer, salt::HOST_DRIFT, 0) < held;
        let depth = 0.34 + held * 0.66;
        let mut target = None;
        let mut target_id = f64::NAN;
        if drifts {
            if humming {
                target = Some(OPEN_HUM);
                // A hum opening: the sung voice's target 4 (`SUNG_OPEN_HUM`).
                target_id = 4.0;
            } else {
                // DRIFT_TO: a -> o or e, e -> a, o -> u or a, u -> o.
                let to: &[usize] = match vowel_index {
                    Some(1) => &[0],
                    Some(2) => &[3, 0],
                    Some(3) => &[2],
                    _ => &[2, 1],
                };
                let pick = js::floor(unit(key, layer, salt::HOST_DRIFT_TO, 0) * js::num(to.len()));
                let k = to.get(js::index(pick)).copied().unwrap_or(0);
                target = Some(VOWELS[k]);
                target_id = js::num(k);
            }
        }
        let cost = if choral { 34.0 } else if humming { 16.0 } else { 22.0 } + if target.is_some() { VOWEL_DRIFT_COST } else { 0.0 };
        if !self.budget.ask(host, time, soft_of(channel), cost) {
            return;
        }
        let corner = if humming { 1700.0 } else { 2600.0 };
        let which = if humming { HUM_REF } else { vowel_index.unwrap_or(0) };
        let mut trim = self.refs.trim(which, &formants, corner, humming, midi);
        if let Some(target) = &target {
            trim = self.refs.moving(trim, midi_to_freq(midi), &formants, target, depth, corner, humming);
        }
        if dur > SUNG_LONGEST {
            // Longer than the core holds the jitter for: the page plays it in
            // JavaScript; the player holds the budget as the page does.
            self.budget.hold(time, dur + 0.9, cost);
            return;
        }
        let vowel = if humming { 0.0 } else { js::num(vowel_index.unwrap_or(0)) };
        let detune = opts.detune.filter(|&d| d != 0.0 && !d.is_nan()).unwrap_or(0.0);
        let voice = if humming { HUM } else if choral { CHOIR } else { VOWEL };
        self.note(host, key, voice, channel, time, midi, dur, vel, 0, extra(&[vowel, target_id, trim, detune]));
        self.budget.hold(time, dur + 0.9, cost);
    }
}

/// Whether a voice on `channel` asks under the soft cap (`voice`'s `soft`).
fn soft_of(channel: u32) -> bool {
    channel == CHORDS
}

impl Default for Synth {
    fn default() -> Self {
        Self::new()
    }
}
