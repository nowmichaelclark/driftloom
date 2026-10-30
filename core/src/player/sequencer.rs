//! The sequencer (queue item 82): what `Engine` in `js/engine.js` does
//! between the generator's events and the synth's notes, step by step:
//! where each layer is in its own cycle, the swing, the drums' timing
//! jitter and the ensemble's slip, the strums, the chord's spread, the
//! tune's harmony line and graces, the chapel organ's holds, glass and
//! dubStab, the tails at a silent bar, lateFloor's wait, the tuned kick and
//! the pump; and the per-loop setups at a loop's first step. Every number
//! is worked out in the JavaScript's order of operations, and every draw is
//! the page's (`js/draw.js`, the core's own generator), from where the note
//! sits in its loop.

use crate::block::event::NOTES;
use crate::generator::choices::{self, Fall};
use crate::generator::js;
use crate::generator::layers::DrumEvent;
use crate::generator::load::{Load, Stab};
use crate::generator::plan::ChordEvent;
use crate::generator::render::Render;
use crate::generator::spec::Spec;
use crate::generator::wander::{Pass, neighbour_in_scale};
use crate::generator::{LAYERS, Layer};
use crate::math;
use crate::rng::{Key, salt, unit};
use crate::{BASS_CHANNEL, CHORDS, DRUMS_CHANNEL, MELODY, TEXTURE_CHANNEL};

use super::host::Host;
use super::synth::{Fm, KickHit, Opts, StabOpts, Synth};

/// A chord's level spread across its voicing (`CHORD_SPREAD`).
const CHORD_SPREAD: f64 = 0.8;
/// Grace notes: each this long, running straight into the note, lighter
/// than it (`GRACE`, `GRACE_VEL`); the harmony line a step back
/// (`HARMONY_VEL`).
const GRACE: f64 = 0.03;
const GRACE_VEL: f64 = 0.6;
const HARMONY_VEL: f64 = 0.6;
/// A nylon strum crosses its strings in 10-20 ms, the upstroke lighter
/// (`STRUM`).
const STRUM_MIN: f64 = 0.01;
const STRUM_MAX: f64 = 0.02;
const STRUM_UP: f64 = 0.75;
/// glass (`GLASS`): how often a chord rings out, its register, its level
/// against the chord's velocity, and the bell.
const GLASS_CHANCE: f64 = 0.55;
const GLASS_LOW: i16 = 76;
const GLASS_HIGH: i16 = 91;
const GLASS_VEL: f64 = 0.62;
const GLASS_CHORD_VEL: f64 = 0.5;
/// Where glass's and dubStab's notes sit among their layer's at a step
/// (`GLASS_ORD`, `STAB_ORD`): past any place the layer's own notes take.
const GLASS_ORD: u32 = 256;
const STAB_ORD: u32 = 256;

/// The most notes a chord plays: the largest chord, its ninth, and the
/// ninth a wandering pass opens it up with.
const CHORD_NOTES: usize = 8;

/// A chord's notes as a pass plays it (`e.notes`, a ninth pushed on when it
/// opens up). Compared whole, as the engine compares `notes.join()`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Chord {
    notes: [i16; CHORD_NOTES],
    len: usize,
}

impl Chord {
    const NONE: Chord = Chord { notes: [0; CHORD_NOTES], len: 0 };

    fn of(e: &ChordEvent, opened: Option<i16>) -> Self {
        let mut c = Chord::NONE;
        for &n in e.notes.iter().chain(opened.as_ref()) {
            if c.len < CHORD_NOTES {
                c.notes[c.len] = n;
                c.len += 1;
            }
        }
        c
    }

    fn notes(&self) -> &[i16] {
        self.notes.get(..self.len).unwrap_or(&[])
    }

    fn sorted(mut self) -> Self {
        let len = self.len;
        if let Some(n) = self.notes.get_mut(..len) {
            n.sort_unstable();
        }
        self
    }
}

/// A note's pitch class, as the engine folds one (`((n % 12) + 12) % 12`).
fn pc(n: i16) -> i16 {
    ((n % 12) + 12) % 12
}

/// What the chapel organ holds (`organHold`): the chord, until when (in
/// steps from the load), the chord a passing chord is held against, when
/// it last moved, and whether it may still be sounding.
#[derive(Clone, Copy)]
struct Hold {
    key: Option<Chord>,
    until: f64,
    held: Option<Chord>,
    last: f64,
    on: bool,
}

const NO_HOLD: Hold = Hold { key: None, until: 0.0, held: None, last: f64::NEG_INFINITY, on: false };

/// A JavaScript number's truth: not 0, not NaN.
fn truthy(x: f64) -> bool {
    x != 0.0 && !x.is_nan()
}

/// The loop the sequencer plays this step: its spec, the loop as rendered,
/// the pass wandering over it (none: the loop as composed), and what the
/// engine decided at load.
pub struct Loop<'a> {
    pub spec: &'a Spec,
    pub base: &'a Render,
    pub pass: Option<&'a Pass>,
    pub settings: &'a Load,
}

impl Loop<'_> {
    /// `p.totalSteps`.
    fn total(&self) -> f64 {
        self.spec.bars * self.spec.steps_per_bar
    }

    /// A layer's own cycle in steps (`p.cycles[layer]`): the loop's length
    /// unless the layer has its own.
    fn cycle(&self, layer: Layer) -> f64 {
        self.spec.cycles.and_then(|c| c[layer as usize]).filter(|&c| truthy(c)).unwrap_or_else(|| self.total())
    }

    /// Drum hit `i` as the pass plays it.
    fn drum(&self, i: usize, e: &DrumEvent) -> (f64, &'static str) {
        self.pass.map_or((e.vel, e.inst), |p| p.drum(i, e))
    }

    /// Every drum hit as the pass plays it, its own ghost hit last: the
    /// velocity, the instrument and the hit.
    fn drums(&self) -> impl Iterator<Item = (f64, &'static str, &DrumEvent)> {
        let own = self.base.drums.iter().enumerate().map(|(i, e)| {
            let (vel, inst) = self.drum(i, e);
            (vel, inst, e)
        });
        own.chain(self.pass.and_then(Pass::ghost).map(|g| (g.vel, g.inst, g)))
    }

    /// Chord `i` as the pass plays it: its velocity and its notes.
    fn chord(&self, i: usize, e: &ChordEvent) -> (f64, Chord) {
        let (vel, opened) = self.pass.map_or((e.vel, None), |p| p.chord(i, e));
        (vel, Chord::of(e, opened))
    }
}

/// The engine's state as it decides: where it is in the loop and in time,
/// the organ's hold and the tails, the setups still to send, and the
/// loop's settings it plays by.
pub struct Sequencer {
    /// When the next step falls, in seconds on the core's clock.
    pub next_step_time: f64,
    /// The step in the loop, from its first; the step from the load (it
    /// never resets as layers with their own cycles index off it); and the
    /// pass.
    pub step: u64,
    pub abs_step: u64,
    pub loop_count: u32,
    /// The tempo playing, in beats a minute (the loop's, unless a host set
    /// another).
    pub bpm: f64,
    /// Each layer muted, in the generator's layer order.
    pub mutes: [bool; 5],
    tails_ducked: bool,
    hold: Hold,
    /// The per-loop setups still to send, at the loop's first step.
    organ_change: bool,
    wash_change: bool,
    arc_change: bool,
    sub_duck_change: bool,
    pad_sweep_change: bool,
    bass_lift_change: bool,
    space_change: bool,
    /// Whether the last loop had each (a new loop without one lets it go).
    had_organ: bool,
    had_wash: bool,
    had_arc: bool,
    had_sub_duck: bool,
    had_pad_sweep: bool,
    /// The loop's: glass on, a choir's ensemble, and the tuned kick's kind
    /// this pass plays.
    glass: bool,
    choir: bool,
    kick_kind: Option<&'static str>,
}

impl Sequencer {
    /// Nothing playing.
    #[must_use]
    pub const fn new() -> Self {
        Sequencer {
            next_step_time: 0.0,
            step: 0,
            abs_step: 0,
            loop_count: 0,
            bpm: 0.0,
            mutes: [false; 5],
            tails_ducked: false,
            hold: NO_HOLD,
            organ_change: false,
            wash_change: false,
            arc_change: false,
            sub_duck_change: false,
            pad_sweep_change: false,
            bass_lift_change: false,
            space_change: false,
            had_organ: false,
            had_wash: false,
            had_arc: false,
            had_sub_duck: false,
            had_pad_sweep: false,
            glass: false,
            choir: false,
            kick_kind: None,
        }
    }

    /// Forget every loop: a new player.
    pub fn reset(&mut self) {
        *self = Sequencer::new();
    }

    /// `60 / bpm / 4`: a step's length, in seconds.
    #[must_use]
    pub fn step_dur(&self) -> f64 {
        60.0 / self.bpm / 4.0
    }

    /// `Engine.load(spec)` for a loop that starts (not a re-roll, which keeps
    /// its place, nor a loop resumed): what it decides, the calls it makes
    /// to the synth at `now`, and the setups its first step sends.
    pub fn load<H: Host>(&mut self, host: &mut H, synth: &mut Synth, lp: &Loop, now: f64) {
        let s = lp.settings;
        let on = |id: &str| lp.base.choice(id) == Some("on");
        self.organ_change = s.organ.is_some() || self.had_organ;
        self.wash_change = s.wash.is_some() || self.had_wash;
        self.arc_change = s.arc.is_some() || self.had_arc;
        self.sub_duck_change = s.sub_duck.is_some() || self.had_sub_duck;
        self.pad_sweep_change = s.pad_sweep.is_some() || self.had_pad_sweep;
        self.bass_lift_change = true;
        self.space_change = true;
        self.had_organ = s.organ.is_some();
        self.had_wash = s.wash.is_some();
        self.had_arc = s.arc.is_some();
        self.had_sub_duck = s.sub_duck.is_some();
        self.had_pad_sweep = s.pad_sweep.is_some();
        self.glass = on("glass");
        self.choir = lp.spec.choir().is_some();
        self.bpm = lp.spec.bpm;
        synth.set_tone(host, lp.spec.tone, now);
        synth.set_level(host, s.level, now);
        Synth::set_echo(host, (60.0 / self.bpm) * s.echo_beats, now);
        // spec.mutes, in its order: drums, bass, chords, melody, texture.
        for layer in LAYERS {
            synth.set_mute(host, channel_of(layer) as usize, self.mutes[layer as usize], now);
        }
        self.step = 0;
        self.abs_step = 0;
        self.loop_count = 0;
        self.hold = NO_HOLD;
        self.kick_kind = s.kick.map(|k| k.kind);
    }

    /// A new pass begins (`_advance`'s wrap, and the load's first): the
    /// tuned kick it plays, its own on a pass as composed, else the swap a
    /// wandering pass may draw (`_kickOn`, `kickKindOf`, by the pass's
    /// count).
    pub fn pass_begins(&mut self, lp: &Loop) {
        self.kick_kind = lp.settings.kick.map(|k| if lp.pass.is_some() { choices::kick_kind_of(lp.spec.seed, k.kind, self.loop_count) } else { k.kind });
    }

    /// `_advance`: the next step's time and place. True at the top of a
    /// new pass (the caller hands the sequencer the pass it plays).
    pub fn advance(&mut self, lp: &Loop) -> bool {
        self.next_step_time += self.step_dur();
        self.step += 1;
        self.abs_step += 1;
        #[allow(clippy::cast_precision_loss)] // steps, far below 2^53
        let wrapped = self.step as f64 >= lp.total();
        if wrapped {
            self.step = 0;
            self.loop_count = self.loop_count.wrapping_add(1);
        }
        wrapped
    }

    /// `_isSilentBar(absStep)`: every layer scheduled out for the bar
    /// holding the step.
    fn is_silent_bar(lp: &Loop, abs: f64) -> bool {
        let spb = lp.spec.steps_per_bar;
        let bar = js::floor((abs % lp.total()) / spb);
        if lp.base.quiet.iter().any(|&q| js::num(q) == bar) {
            return true;
        }
        let Some(form) = &lp.base.form else { return false };
        for layer in LAYERS {
            let sched = &form[layer as usize];
            let n = js::num(sched.len());
            let b = js::floor((abs % lp.cycle(layer)) / spb) % n;
            if b.is_finite() && sched.get(js::index(b)).copied().unwrap_or(false) {
                return false;
            }
        }
        true
    }

    /// `_scheduleStep(step, time)`: every note, event and setting of the
    /// step at `time`, decided at `now` on the core's clock.
    #[allow(clippy::too_many_lines)] // one step, in the engine's order
    pub fn schedule_step<H: Host>(&mut self, host: &mut H, synth: &mut Synth, lp: &Loop, now: f64) {
        let time = self.next_step_time;
        let step = self.step;
        let s = lp.settings;
        #[allow(clippy::cast_precision_loss)] // steps, far below 2^53
        let abs = self.abs_step as f64;
        #[allow(clippy::cast_precision_loss)]
        let stepf = step as f64;

        // A new loop's setups, from its first step decided.
        if self.organ_change {
            Synth::organ_off(host, time, 0.15);
            synth.organ_setup(host, s.organ.as_ref(), time);
            self.organ_change = false;
            self.hold = NO_HOLD;
        }
        if self.wash_change {
            synth.wash_setup(host, s.wash.as_ref(), time);
            self.wash_change = false;
        }
        if self.arc_change {
            Synth::arc_setup(host, s.arc.as_ref(), time);
            self.arc_change = false;
        }
        if self.sub_duck_change {
            synth.sub_duck_setup(host, s.sub_duck.as_ref(), time);
            self.sub_duck_change = false;
        }
        if self.pad_sweep_change {
            synth.pad_sweep_setup(host, s.pad_sweep.as_ref(), time);
            self.pad_sweep_change = false;
        }
        if self.bass_lift_change {
            host.bass_lift(s.bass_lift);
            self.bass_lift_change = false;
        }
        if self.space_change {
            Synth::space_setup(host, &s.space, time);
            self.space_change = false;
        }
        // At each bar line, whether the tails should still be ringing.
        let spb = lp.spec.steps_per_bar;
        if abs % spb == 0.0 {
            let quiet = Self::is_silent_bar(lp, abs);
            // The organ lets go in a bar the loop leaves silent.
            if quiet && self.hold.on {
                Synth::organ_off(host, time, 0.25);
                self.hold = NO_HOLD;
            }
            if quiet && !self.tails_ducked {
                Synth::fade_tails(host, time);
                self.tails_ducked = true;
            } else if !quiet && self.tails_ducked {
                // Restored just before the bar starts, so its first note is not dry.
                Synth::restore_tails(host, js::max(now, time - 0.12));
                self.tails_ducked = false;
            }
        }
        // Where each layer is inside its own loop.
        let total = lp.total();
        let at = |layer: Layer| {
            let len = lp.cycle(layer);
            if len == total { stepf } else { ((abs % len) + len) % len }
        };
        let sd = self.step_dur();
        let swing = if step % 2 == 1 { lp.spec.swing * sd } else { 0.0 };
        let t = time + swing;
        let seed = lp.spec.seed;
        let pass = self.loop_count;
        #[allow(clippy::cast_possible_truncation)] // a step of the loop, under 2^32
        let step32 = step as u32;
        let mut placed = [0u32; 5];
        let mut key_of = |layer: Layer| {
            let ord = &mut placed[layer as usize];
            let key = Key { seed, pass, step: step32, ord: *ord };
            *ord += 1;
            key
        };
        let mutes = self.mutes;
        let late = s.late;

        if !mutes[Layer::Drums as usize] {
            let s_at = at(Layer::Drums);
            let pump = s.pump;
            let kick = s.kick.map(|k| (k, self.kick_kind));
            for (vel, inst, e) in lp.drums() {
                if e.step != s_at || !truthy(vel) {
                    continue;
                }
                // Rolls sit between the steps, and skip the jitter.
                let micro = e.micro.filter(|&m| truthy(m)).map_or(0.0, |m| m * sd);
                let key = key_of(Layer::Drums);
                // lateFloor: the kit sits out from the load (its place among
                // the notes still counted, so no draw moves).
                if late.is_some_and(|l| abs < l.drums) {
                    continue;
                }
                let jitter = if e.roll { 0.0 } else { (unit(key, DRUMS_CHANNEL, salt::HOST_JITTER, 0) - 0.5) * 0.008 };
                let at = t + micro + jitter;
                let kicked = inst == "kick" || inst == "softkick";
                let hit = kick.filter(|_| kicked).map(|(k, kind)| KickHit {
                    hz: k.kick.hz,
                    fall: match kind.unwrap_or(k.kind) {
                        "short" => Some(k.kick.short),
                        "long" => Some(k.kick.long),
                        _ => None::<Fall>,
                    },
                });
                synth.drum(host, key, inst, at, vel, hit);
                if truthy(pump) && kicked {
                    Synth::duck(host, t + micro, pump * vel);
                }
            }
        }
        if !mutes[Layer::Bass as usize] {
            let s_at = at(Layer::Bass);
            for (i, e) in lp.base.bass.iter().enumerate() {
                let (vel, midi) = lp.pass.map_or((e.vel, e.midi), |p| p.bass(i, e));
                if e.step != s_at || !truthy(vel) {
                    continue;
                }
                let key = key_of(Layer::Bass);
                if late.is_some_and(|l| abs < l.bass) {
                    continue;
                }
                synth.bass(host, key, midi, t, e.dur * sd, vel, e.glide, e.voice, e.chug);
            }
        }
        // An ensemble does not attack together: a choir's layers each slip,
        // drawn once a layer a step.
        let choir = self.choir;
        let ensemble = |channel: u32| {
            if choir { (unit(Key { seed, pass, step: step32, ord: 0 }, channel, salt::HOST_ENSEMBLE, 0) - 0.5) * 0.016 } else { 0.0 }
        };

        if !mutes[Layer::Chords as usize] {
            let s_at = at(Layer::Chords);
            let slip = ensemble(CHORDS);
            for (i, e) in lp.base.chords.iter().enumerate() {
                let (vel, chord) = lp.chord(i, e);
                if e.step != s_at || !truthy(vel) {
                    continue;
                }
                let dur = e.dur * sd;
                let opts = Opts { vowel: e.vowel, detune: e.detune, ..Opts::default() };
                if e.voice == "pad" {
                    synth.pad(host, key_of(Layer::Chords), chord.notes(), t, dur, vel * CHORD_SPREAD, CHORDS);
                    continue;
                }
                let notes = chord.notes();
                let spread = CHORD_SPREAD / math::sqrt(js::num(notes.len()));
                if let Some(strum) = e.strum.filter(|s| !s.is_empty()) {
                    let span = STRUM_MIN + unit(key_of(Layer::Chords), CHORDS, salt::HOST_STRUM, 0) * (STRUM_MAX - STRUM_MIN);
                    let down = strum == "down";
                    let weight = if down { 1.0 } else { STRUM_UP };
                    // The hand that strikes the chord stops the last one's strings.
                    host.damp(CHORDS, t + slip);
                    let n = notes.len();
                    for k in 0..n {
                        let string = if down { notes[k] } else { notes[n - 1 - k] };
                        let at = if n > 1 { span * js::num(k) / js::num(n - 1) } else { 0.0 };
                        let strummed = Opts { strum: true, ..opts };
                        synth.voice(host, key_of(Layer::Chords), e.voice, f64::from(string), t + slip + at, dur, vel * spread * weight, CHORDS, &strummed);
                    }
                    continue;
                }
                for (k, &n) in notes.iter().enumerate() {
                    // A few milliseconds apart, fingers rather than a switch.
                    let at = t + slip + js::num(k) * 0.011;
                    synth.voice(host, key_of(Layer::Chords), e.voice, f64::from(n), at, dur, vel * spread, CHORDS, &opts);
                }
            }
            if s.organ.is_some() {
                self.organ(host, synth, lp, s_at, t, sd, abs);
            }
            if self.glass {
                Self::glass(host, synth, lp, s_at, t, sd, Key { seed, pass, step: step32, ord: 0 });
            }
            if let Some(stab) = &s.stab {
                Self::dub_stab(host, synth, lp, stab, &mutes, &at, s_at, t, abs, Key { seed, pass, step: step32, ord: 0 });
            }
        }
        // Muted, the chords' organ lets go.
        if mutes[Layer::Chords as usize] && self.hold.on {
            Synth::organ_off(host, t, 0.25);
            self.hold = NO_HOLD;
        }
        if !mutes[Layer::Melody as usize] {
            let s_at = at(Layer::Melody);
            let slip = ensemble(MELODY);
            let root = lp.spec.root;
            for (i, e) in lp.base.melody.iter().enumerate() {
                let (vel, midi) = lp.pass.map_or((e.vel, e.midi), |p| p.melody(i, e));
                if e.step != s_at || !truthy(vel) {
                    continue;
                }
                let dur = e.dur * sd;
                let vowel = (!e.vowel.is_empty()).then_some(e.vowel);
                let opts = Opts { vowel, detune: e.detune, prev: e.prev, ..Opts::default() };
                synth.voice(host, key_of(Layer::Melody), e.voice, midi, t + slip, dur, vel, MELODY, &opts);
                // Asked for after the note, so where the budget is short it
                // is the decoration that yields, never the tune.
                let steps = e.steps.as_ref().map_or(lp.spec.steps(), |s| &s[..]);
                let line = Opts { vowel, detune: e.detune, ..Opts::default() };
                if let Some(under) = harmony_of(e.harm, midi, root, steps) {
                    synth.voice(host, key_of(Layer::Melody), e.voice, under, t + slip, dur, vel * HARMONY_VEL, MELODY, &line);
                }
                let (graces, n) = graces_of(e.orn, midi, root, steps);
                for (k, &g) in graces.iter().take(n).enumerate() {
                    let at = t + slip - GRACE * js::num(n - k);
                    synth.voice(host, key_of(Layer::Melody), e.voice, g, js::max(now, at), GRACE, vel * GRACE_VEL, MELODY, &line);
                }
            }
        }
        if !mutes[Layer::Texture as usize] {
            let s_at = at(Layer::Texture);
            for e in lp.base.texture.iter() {
                if e.step != s_at || !truthy(e.vel) {
                    continue;
                }
                synth.texture(host, key_of(Layer::Texture), e.kind, &e.notes, t, e.dur * sd, e.vel, e.band);
            }
        }
    }

    /// `_organ`: the chapel organ at each chord the loop strikes. It holds
    /// each chord until the next with other notes, moves at most once a bar,
    /// holds through a passing chord (one that keeps its lowest note and two
    /// of its notes), and after a chord the loop rests from for more than two
    /// bars, lets go.
    #[allow(clippy::too_many_arguments)] // the step's own place and times
    fn organ<H: Host>(&mut self, host: &mut H, synth: &Synth, lp: &Loop, s: f64, t: f64, sd: f64, abs: f64) {
        let spb = lp.spec.steps_per_bar;
        let len = lp.cycle(Layer::Chords);
        for (i, e) in lp.base.chords.iter().enumerate() {
            let (vel, chord) = lp.chord(i, e);
            if e.step != s || !truthy(vel) || chord.len == 0 {
                continue;
            }
            let h = self.hold;
            if h.key == Some(chord) && abs < h.until {
                continue;
            }
            let mut next = len;
            let mut next_any = len;
            for (j, o) in lp.base.chords.iter().enumerate() {
                let (ovel, other) = lp.chord(j, o);
                if o.step <= s || !truthy(ovel) || other.len == 0 {
                    continue;
                }
                next_any = js::min(next_any, o.step);
                if other != chord {
                    next = js::min(next, o.step);
                }
            }
            if let Some(held) = h.held {
                let a = held.notes();
                let b = chord.notes();
                let shared = b.iter().filter(|&&x| a.iter().any(|&y| pc(y) == pc(x))).count();
                if abs - h.last < spb || (a.first().map(|&n| pc(n)) == b.first().map(|&n| pc(n)) && shared >= 2) {
                    continue;
                }
            }
            let mut hold = Hold { key: Some(chord), until: abs + (next - s), held: Some(chord.sorted()), last: abs, on: true };
            synth.organ_chord(host, &chord.notes()[..chord.len.min(NOTES)], t);
            if next_any - s > 2.0 * spb {
                let off = js::max(if truthy(e.dur) { e.dur } else { 0.0 }, spb);
                Synth::organ_off(host, t + off * sd, 0.6);
                hold.until = abs + off;
                // Let go, it holds nothing a passing chord could keep.
                hold.held = None;
            }
            self.hold = hold;
        }
    }

    /// `_glass`: on a chord's first beat, 55% of the time, two or three of
    /// its notes ring out in E5-G6 as a soft FM bell, a dotted eighth
    /// apart, into the texture's channel; every decision a draw from where
    /// the chord sits.
    #[allow(clippy::too_many_arguments)] // the step's own place and times
    fn glass<H: Host>(host: &mut H, synth: &mut Synth, lp: &Loop, s: f64, t: f64, sd: f64, key: Key) {
        let mut ord = 0;
        for (i, e) in lp.base.chords.iter().enumerate() {
            let (vel, chord) = lp.chord(i, e);
            if e.step != s || !truthy(vel) || chord.len == 0 {
                continue;
            }
            let at = Key { ord, ..key };
            ord += 1;
            // The chord's notes, each once, low to high.
            let mut pcs = [0i16; CHORD_NOTES];
            let mut n = 0;
            for &m in chord.notes() {
                if !pcs[..n].contains(&m) {
                    pcs[n] = m;
                    n += 1;
                }
            }
            pcs[..n].sort_unstable();
            if n < 2 {
                continue;
            }
            if unit(at, TEXTURE_CHANNEL, salt::HOST_GLASS, 0) >= GLASS_CHANCE {
                continue;
            }
            let count = if unit(at, TEXTURE_CHANNEL, salt::HOST_GLASS, 1) < 0.5 { 3 } else { 2 };
            let first = js::index(js::floor(unit(at, TEXTURE_CHANNEL, salt::HOST_GLASS, 2) * js::num(n)));
            for k in 0..count {
                let mut m = pcs[(first + k) % n];
                while m < GLASS_LOW {
                    m += 12;
                }
                while m > GLASS_HIGH {
                    m -= 12;
                }
                let kf = js::num(k);
                let when = t + kf * 3.0 * sd;
                let v = GLASS_VEL * (vel / GLASS_CHORD_VEL) * lp.settings.glass_scale * (1.0 - kf * 0.18);
                #[allow(clippy::cast_possible_truncation)] // a note's place, a few
                let place = Key { ord: GLASS_ORD + at.ord * 4 + k as u32, ..key };
                let bell = Fm { ratio: 3.5, index: 150.0, decay: Some(1.8), attack: 0.003, detune: 3.0, cost: Some(6.0), yields: true, ..Fm::into(TEXTURE_CHANNEL) };
                synth.fm(host, place, f64::from(m), when, js::max(0.6, 2.5 * sd), v, &bell);
            }
        }
    }

    /// `_dubStab`: on the offbeat after beat one and after beat three, by the
    /// loop's density, a soft stab of the chord sounding now, folded into
    /// G3-C5; never in a silent bar, and only over a groove.
    #[allow(clippy::too_many_arguments)] // the step's own place and times
    fn dub_stab<H: Host>(host: &mut H, synth: &mut Synth, lp: &Loop, stab: &Stab, mutes: &[bool; 5], at: &impl Fn(Layer) -> f64, s: f64, t: f64, abs: f64, key: Key) {
        let spb = lp.spec.steps_per_bar;
        let pos = s % spb;
        let which = if pos == 2.0 {
            0
        } else if pos == js::floor(spb / 2.0) + 2.0 {
            1
        } else {
            return;
        };
        if !(stab.rates[which] > 0.0) || Self::is_silent_bar(lp, abs) {
            return;
        }
        let mut n = 0;
        let mut floor = 0;
        for layer in [Layer::Drums, Layer::Bass, Layer::Melody, Layer::Texture] {
            if mutes[layer as usize] {
                continue;
            }
            let here = at(layer);
            let from = here - (here % spb);
            let inside = |step: f64, vel: f64| truthy(vel) && step >= from && step < from + spb;
            let k = match layer {
                Layer::Drums => lp.drums().filter(|(vel, _, e)| inside(e.step, *vel)).count(),
                Layer::Bass => {
                    lp.base.bass.iter().enumerate().filter(|(i, e)| inside(e.step, lp.pass.map_or(e.vel, |p| p.bass(*i, e).0))).count()
                }
                Layer::Melody => {
                    lp.base.melody.iter().enumerate().filter(|(i, e)| inside(e.step, lp.pass.map_or(e.vel, |p| p.melody(*i, e).0))).count()
                }
                _ => lp.base.texture.iter().filter(|e| inside(e.step, e.vel)).count(),
            };
            n += k;
            if matches!(layer, Layer::Drums | Layer::Bass) && k > 0 {
                floor += 1;
            }
        }
        if floor == 0 || n < 4 {
            return;
        }
        // The chord sounding now: the last one struck, within the last bar.
        let mut cur: Option<(f64, Chord)> = None;
        for (i, e) in lp.base.chords.iter().enumerate() {
            let (vel, chord) = lp.chord(i, e);
            if truthy(vel) && chord.len > 0 && e.step <= s && cur.is_none_or(|(step, _)| e.step >= step) {
                cur = Some((e.step, chord));
            }
        }
        let Some((step, chord)) = cur else { return };
        if s - step >= spb {
            return;
        }
        if unit(key, CHORDS, salt::HOST_STAB, 0) >= stab.rates[which] {
            return;
        }
        let mut notes = [0.0; 3];
        let mut len = 0;
        for &m in chord.notes().iter().take(3) {
            let mut m = m;
            while m > 72 {
                m -= 12;
            }
            while m < 55 {
                m += 12;
            }
            notes[len] = f64::from(m);
            len += 1;
        }
        let o = StabOpts { decay: stab.decay, open: stab.open, saw: stab.saw, throw: stab.throw, tone: stab.tone, verb: stab.verb, level: stab.level };
        synth.dub_stab(host, Key { ord: STAB_ORD, ..key }, &notes[..len], t, 0.8, &o);
    }
}

impl Default for Sequencer {
    fn default() -> Self {
        Self::new()
    }
}

/// The core's channel a layer plays into.
#[must_use]
pub const fn channel_of(layer: Layer) -> u32 {
    match layer {
        Layer::Drums => DRUMS_CHANNEL,
        Layer::Bass => BASS_CHANNEL,
        Layer::Chords => CHORDS,
        Layer::Melody => MELODY,
        Layer::Texture => TEXTURE_CHANNEL,
    }
}

/// `harmonyOf(e, spec)`: the harmony under a marked note, two scale tones
/// down for a third, five for a sixth; only ever a real third or sixth.
fn harmony_of(harm: Option<&str>, midi: f64, root: f64, steps: &[f64]) -> Option<f64> {
    let harm = harm.filter(|h| !h.is_empty())?;
    let (count, lo, hi) = if harm == "sixth" { (5, 8.0, 9.0) } else { (2, 3.0, 4.0) };
    let mut m = midi;
    for _ in 0..count {
        m = neighbour_in_scale(m, root, steps, -1.0);
    }
    let gap = midi - m;
    (gap >= lo && gap <= hi).then_some(m)
}

/// `gracesOf(e, spec)`: a cut (the note above) or a turn (above, then
/// below); and how many.
fn graces_of(orn: Option<&str>, midi: f64, root: f64, steps: &[f64]) -> ([f64; 2], usize) {
    let Some(orn) = orn.filter(|o| !o.is_empty()) else { return ([0.0; 2], 0) };
    let above = neighbour_in_scale(midi, root, steps, 1.0);
    if orn == "turn" { ([above, neighbour_in_scale(midi, root, steps, -1.0)], 2) } else { ([above, 0.0], 1) }
}
