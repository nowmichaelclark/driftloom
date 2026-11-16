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
use crate::blocks::low_bell;
use crate::generator::data;
use crate::generator::js;
use crate::generator::list::Biased;
use crate::generator::heard::{self, tune};
use crate::generator::layers::{BassEvent, DrumEvent};
use crate::generator::melody::MelodyEvent;
use crate::generator::piece::Piece;
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
use super::synth::{Fm, KickHit, LOW_BELL_RING, Opts, StabOpts, Synth};

/// A chord's level spread across its voicing (`CHORD_SPREAD`).
const CHORD_SPREAD: f64 = 0.8;
/// How far apart a landing's rolled chord's notes start, in seconds (queue item
/// 102c: a loop that plays its chords a note at a time lands on a roll).
const LAND_ROLL: f64 = 0.12;
/// The budget's room a long piece's doubled tune needs to spare before it
/// plays (queue item 102c): its own cost and as much again, so it yields to
/// any note that comes after it.
const DOUBLE_ROOM: f64 = 24.0;
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
/// Where the low bell's strike sits among the chords' notes at a step
/// (queue item 101g): past glass's and the stab's.
const BELL_ORD: u32 = 512;
/// The low bell's register: the lowest note it strikes (D2, 73 Hz, so its
/// fundamental stays above the 60 Hz guard); it strikes up to 11 above.
pub const BELL_LOW: f64 = 38.0;
/// A full stop's damp on the low bell, in seconds to -60 dB (the sketch's).
const BELL_DAMP: f64 = 0.25;
/// dropSpotlight's damp as the floor comes back, in seconds to -60 dB (the
/// Ears 13b sketch's): the bell belongs to the drop.
const SPOT_DAMP: f64 = 1.5;

/// How long the last loop's organ takes to let go at a new loop's first
/// step, in seconds (`organOff(time, 0.15)` in the engine's setups).
pub const ORGAN_FALL: f64 = 0.15;

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

    /// Every note moved as `by` says, in semitones (a long piece's plan:
    /// the chords' register, another key).
    #[allow(clippy::cast_possible_truncation)] // whole semitones
    fn moved(mut self, by: impl Fn(f64) -> f64) -> Self {
        for n in self.notes.iter_mut().take(self.len) {
            *n += by(f64::from(*n)) as i16;
        }
        self
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
    n.rem_euclid(12)
}

/// What the chapel organ holds (`organHold`): the chord, until when (in
/// steps from the load), the chord a passing chord is held against, when
/// it last moved, and whether it may still be sounding.
#[derive(Clone, Copy)]
struct Hold {
    key: Option<Chord>,
    until: f64,
    held: Option<Chord>,
    last: Biased<{ f64::NEG_INFINITY.to_bits() }>,
    on: bool,
}

const NO_HOLD: Hold = Hold { key: None, until: 0.0, held: None, last: Biased::DEFAULT, on: false };

/// A JavaScript number's truth: not 0, not NaN.
fn truthy(x: f64) -> bool {
    x != 0.0 && !x.is_nan()
}

/// The loop the sequencer plays this step: its spec, the loop as rendered,
/// the pass wandering over it (none: the loop as composed), and what the
/// engine decided at load.
pub struct Loop<'a> {
    /// The loop's spec.
    pub spec: &'a Spec,
    /// The loop as rendered (`render`'s tracks and choices).
    pub base: &'a Render,
    /// The pass wandering over it; none, the loop as composed.
    pub pass: Option<&'a Pass>,
    /// What the engine decided at load.
    pub settings: &'a Load,
    /// The step of the section being decided (a long piece's plan reads its
    /// unit from it, queue item 102b; 0 where no step is).
    pub step: f64,
    /// The tune the section plays beside the loop's own (session 14:
    /// `Generator::second`), where it plays one.
    pub second: Option<&'a crate::generator::melody::Melody>,
}

impl Loop<'_> {
    /// `p.totalSteps`.
    fn total(&self) -> f64 {
        self.spec.bars * self.spec.steps_per_bar
    }

    /// A layer's own cycle in steps (`p.cycles[layer]`): the loop's length
    /// unless the layer has its own.
    fn cycle(&self, layer: Layer) -> f64 {
        self.spec.cycles.of(layer as usize).filter(|&c| truthy(c)).unwrap_or_else(|| self.total())
    }

    /// Drum hit `i` as the pass plays it.
    fn drum(&self, i: usize, e: &DrumEvent) -> (f64, &'static str) {
        self.pass.map_or((e.vel, e.inst.get()), |p| p.drum_at(self.step, i, e))
    }

    /// Bass note `i` as the pass plays it: its velocity and its pitch.
    fn bass(&self, i: usize, e: &BassEvent) -> (f64, f64) {
        self.pass.map_or((e.vel, e.midi), |p| p.bass_at(self.step, i, e))
    }

    /// Melody note `i` as the pass plays it: its velocity and its pitch.
    fn melody(&self, i: usize, e: &MelodyEvent) -> (f64, f64) {
        self.pass.map_or((e.vel, e.midi), |p| p.melody_at(self.step, i, e))
    }

    /// The voice `layer` plays this step: the plan's, or the note's `own`.
    fn voice(&self, layer: Layer, own: &'static str) -> &'static str {
        self.pass.and_then(|p| p.voice(self.step, layer)).unwrap_or(own)
    }

    /// Melody note `i` as the pass plays it, its pitch as written in the
    /// home key (`moved` takes it to the plan's).
    fn melody_home(&self, i: usize, e: &MelodyEvent) -> (f64, f64) {
        self.pass.map_or((e.vel, e.midi), |p| p.melody_home_at(self.step, i, e))
    }

    /// How far the plan moves `layer`'s note `midi` this step, in
    /// semitones.
    fn moved(&self, layer: Layer, midi: f64) -> f64 {
        self.pass.map_or(0.0, |p| p.moved(self.step, layer, midi))
    }

    /// Whether the plan rests `layer` this step.
    fn rests(&self, layer: Layer) -> bool {
        self.pass.is_some_and(|p| p.rests(self.step, layer))
    }

    /// Chord `i`'s length this step, in steps: its own, or the plan's
    /// share of it, never running into the next chord when longer and
    /// never under a step when shorter.
    fn chord_dur(&self, i: usize, e: &ChordEvent) -> f64 {
        let stretch = self.pass.map_or(0.0, |p| p.stretch(self.step));
        if stretch == 0.0 {
            return e.dur;
        }
        if stretch > 1.0 {
            js::min(e.dur * stretch, js::max(e.dur, heard::room(self.base, i, self.cycle(Layer::Chords))))
        } else {
            js::max(1.0, e.dur * stretch)
        }
    }

    /// Every drum hit as the pass plays it, its own ghost hit last: the
    /// velocity, the instrument and the hit.
    fn drums(&self) -> impl Iterator<Item = (f64, &'static str, &DrumEvent)> {
        let own = self.base.drums.iter().enumerate().map(|(i, e)| {
            let (vel, inst) = self.drum(i, e);
            (vel, inst, e)
        });
        let rests = self.pass.is_some_and(|p| p.out(self.step, Layer::Drums, 0.0));
        own.chain(self.pass.and_then(Pass::ghost).map(move |g| (if rests { 0.0 } else { g.vel }, g.inst.get(), g)))
    }

    /// Every note of `layer` as the pass plays it: its step and velocity
    /// (`p.tracks[layer]`, the drums' ghost hit among them), for the layers'
    /// grid (queue item 94a).
    pub fn each_velocity(&self, layer: Layer, mut f: impl FnMut(f64, f64)) {
        match layer {
            // Each hit at its own step, as the bass and the tune are below
            // (a long piece's plan moves the kit's velocities over a unit).
            Layer::Drums => {
                for (i, e) in self.base.drums.iter().enumerate() {
                    f(e.step, self.pass.map_or(e.vel, |p| p.drum(i, e).0));
                }
                if let Some(g) = self.pass.and_then(Pass::ghost) {
                    f(g.step, if self.pass.is_some_and(|p| p.out(g.step, Layer::Drums, 0.0)) { 0.0 } else { g.vel });
                }
            }
            Layer::Bass => {
                for (i, e) in self.base.bass.iter().enumerate() {
                    f(e.step, self.pass.map_or(e.vel, |p| p.bass(i, e).0));
                }
            }
            Layer::Chords => {
                for (i, e) in self.base.chords.iter().enumerate() {
                    f(e.step, self.pass.map_or(e.vel, |p| p.chord(i, e).0));
                }
            }
            Layer::Melody => {
                for (i, e) in self.base.melody.iter().enumerate() {
                    f(e.step, self.pass.map_or(e.vel, |p| p.melody(i, e).0));
                }
            }
            Layer::Texture => self.base.texture.iter().for_each(|e| f(e.step, e.vel)),
        }
    }

    /// A layer's own cycle in steps (`p.cycles[layer] || p.totalSteps`).
    #[must_use]
    pub fn layer_cycle(&self, layer: Layer) -> f64 {
        self.cycle(layer)
    }

    /// Chord `i` as the pass plays it: its velocity and its notes.
    fn chord(&self, i: usize, e: &ChordEvent) -> (f64, Chord) {
        let (vel, opened) = self.pass.map_or((e.vel, None), |p| p.chord_at(self.step, i, e));
        (vel, Chord::of(e, opened).moved(|n| self.moved(Layer::Chords, n)))
    }
}

/// The engine's state as it decides: where it is in the loop and in time,
/// the organ's hold and the tails, the setups still to send, and the
/// loop's settings it plays by.
// Its flags are the engine's own, each a state of its own (a setup still to
// send, a loop that had one), not one state spelled in bools.
#[allow(clippy::struct_excessive_bools)]
pub struct Sequencer {
    /// When the next step falls, in seconds on the core's clock.
    pub next_step_time: f64,
    /// The step in the loop, from its first.
    pub step: u64,
    /// The step from the load: it never resets, as layers with their own
    /// cycles index off it.
    pub abs_step: u64,
    /// The pass, from the load's first (0): in a piece (queue item 88), the
    /// section playing.
    pub loop_count: u32,
    /// The steps the pass playing takes: the loop's, or fewer in a piece's
    /// section cut short (queue item 88).
    pub pass_steps: f64,
    /// The step from the load the piece's round playing began on (0 in a
    /// loop with no length, and in a piece no section of which is cut): the
    /// loop's bars are counted from it, as a cut section moves them off the
    /// load's grid. Layers with their own cycles go on counting from the
    /// load, drifting as ever.
    pub origin: u64,
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
    pad_wow_change: bool,
    bass_lift_change: bool,
    space_change: bool,
    /// Whether the last loop had each (a new loop without one lets it go).
    had_organ: bool,
    had_wash: bool,
    had_arc: bool,
    had_sub_duck: bool,
    had_pad_sweep: bool,
    had_pad_wow: bool,
    /// colorTone (queue item 101e): its setup still to send, whether the
    /// loop plays colors, and until when (seconds on the core's clock) the
    /// last color sent may sound.
    color_change: bool,
    had_color: bool,
    color_until: f64,
    /// lowBell (queue item 101g): the pitch class of the last root struck
    /// and one (0: none yet), when it was struck, and until when (seconds
    /// on the core's clock) a ring may sound.
    bell_pc: u8,
    bell_at: f64,
    bell_until: f64,
    /// dropSpotlight (queue item 101g): the step from the load (`abs_step`)
    /// at which the floor drop the bell strikes in ends (0: none).
    bell_drop: f64,
    /// Whether the floor has played since the load: a loop that opens
    /// without its kit and bass has not dropped them.
    bell_floor: bool,
    /// Whether the floor was out in the bar before (a drop starts where it
    /// played there, in the pass before at a pass's first bar).
    bell_was_out: bool,
    /// How long the last loop's organ takes to let go at this one's first
    /// step, in seconds (the engine's 0.15; a crossfade's length, queue item
    /// 86).
    organ_fall: Biased<{ ORGAN_FALL.to_bits() }>,
    /// The loop's: glass on, a choir's ensemble, and the tuned kick's kind
    /// this pass plays.
    glass: bool,
    choir: bool,
    kick_kind: Option<&'static str>,
    /// Whether the listener set the tone while the loop plays (the Sound
    /// panel): a long piece's plan no longer moves it.
    pub tone_held: bool,
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
            pass_steps: 0.0,
            origin: 0,
            bpm: 0.0,
            mutes: [false; 5],
            tails_ducked: false,
            hold: NO_HOLD,
            organ_change: false,
            wash_change: false,
            arc_change: false,
            sub_duck_change: false,
            pad_sweep_change: false,
            pad_wow_change: false,
            bass_lift_change: false,
            space_change: false,
            had_organ: false,
            had_wash: false,
            had_arc: false,
            had_sub_duck: false,
            had_pad_sweep: false,
            had_pad_wow: false,
            color_change: false,
            had_color: false,
            color_until: 0.0,
            bell_pc: 0,
            bell_at: 0.0,
            bell_until: 0.0,
            bell_drop: 0.0,
            bell_floor: false,
            bell_was_out: false,
            organ_fall: Biased::DEFAULT,
            glass: false,
            choir: false,
            kick_kind: None,
            tone_held: false,
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
        self.load_at(host, synth, lp, now, now, ORGAN_FALL);
    }

    /// `load`, for a loop that starts at a moment (queue item 86): its tone,
    /// level and echo glide in at `at` (its first step) rather than `now`,
    /// the mutes are sent again at `now` (as they stand), and the last
    /// loop's organ lets go over `organ_fall` seconds at the first step.
    pub fn load_at<H: Host>(&mut self, host: &mut H, synth: &mut Synth, lp: &Loop, now: f64, at: f64, organ_fall: f64) {
        self.organ_fall.set(organ_fall);
        let s = lp.settings;
        let on = |id: &str| lp.base.choice(id) == Some("on");
        self.organ_change = s.organ.is_some() || self.had_organ;
        self.wash_change = s.wash.is_some() || self.had_wash;
        self.arc_change = s.arc.is_some() || self.had_arc;
        self.sub_duck_change = s.sub_duck.is_some() || self.had_sub_duck;
        self.pad_sweep_change = s.pad_sweep.is_some() || self.had_pad_sweep;
        self.pad_wow_change = s.pad_wow.is_some() || self.had_pad_wow;
        self.bass_lift_change = true;
        self.space_change = true;
        self.had_organ = s.organ.is_some();
        self.had_wash = s.wash.is_some();
        self.had_arc = s.arc.is_some();
        self.had_sub_duck = s.sub_duck.is_some();
        self.had_pad_sweep = s.pad_sweep.is_some();
        self.had_pad_wow = s.pad_wow.is_some();
        let colors = !lp.base.colors.is_empty();
        self.color_change = colors || self.had_color;
        self.had_color = colors;
        // A new loop's bell strikes its first chord, and its floor has yet to
        // play (queue item 101g).
        self.bell_pc = 0;
        self.bell_drop = 0.0;
        self.bell_floor = false;
        self.glass = on("glass");
        self.choir = lp.spec.choir().is_some();
        self.bpm = lp.spec.bpm;
        // A new loop has no plan on its levels or its tone (queue item 102b).
        synth.clear_plan();
        synth.let_go_aim(host, at);
        self.tone_held = false;
        synth.set_tone(host, lp.spec.tone, at);
        synth.set_level(host, s.level.get(), at);
        Synth::set_echo(host, (60.0 / self.bpm) * s.echo_beats.get(), at);
        // A layer the request asked off is muted, as the listener's mute
        // would (queue item 91), one asked on is not; the others stay as the
        // host left them (a mute holds across loads until brought back).
        for layer in LAYERS {
            let bit = 1u8 << layer as usize;
            if lp.spec.layers_off & bit != 0 {
                self.mutes[layer as usize] = true;
            } else if lp.spec.layers_on & bit != 0 {
                self.mutes[layer as usize] = false;
            }
        }
        // spec.mutes, in its order: drums, bass, chords, melody, texture.
        for layer in LAYERS {
            synth.set_mute(host, channel_of(layer) as usize, self.mutes[layer as usize], now);
        }
        self.step = 0;
        self.abs_step = 0;
        self.loop_count = 0;
        self.origin = 0;
        self.pass_steps = Piece::of(lp.spec).map_or_else(|| lp.total(), |p| p.bars_of(0) * lp.spec.steps_per_bar);
        self.hold = NO_HOLD;
        self.kick_kind = s.kick.map(|k| k.kind);
    }

    /// `Engine.load(spec, { keepPosition: true })`, a re-roll (queue item
    /// 94a, `Player::revise`): the loop made again keeps its place (the step,
    /// the pass, the round, the organ's hold) and the tempo it plays at
    /// (`bpm`, which the caller sets first); each per-loop setup is sent
    /// again at its first step only where it differs from `was`, the loop's
    /// before, as the engine compares them; the tone, the level, the echo and
    /// the mutes at `now`. The lateFloor the loop was in stays (the caller
    /// keeps it in the settings).
    pub fn revise<H: Host>(&mut self, host: &mut H, synth: &mut Synth, lp: &Loop, now: f64, was: &Load) {
        self.organ_fall.set(ORGAN_FALL);
        let s = lp.settings;
        let on = |id: &str| lp.base.choice(id) == Some("on");
        self.organ_change = s.organ != was.organ;
        self.wash_change = s.wash != was.wash;
        self.arc_change = s.arc != was.arc;
        self.sub_duck_change = s.sub_duck != was.sub_duck;
        self.pad_sweep_change = s.pad_sweep != was.pad_sweep;
        self.pad_wow_change = s.pad_wow != was.pad_wow;
        self.bass_lift_change = s.bass_lift != was.bass_lift;
        self.space_change = s.space != was.space;
        self.had_organ = s.organ.is_some();
        self.had_wash = s.wash.is_some();
        self.had_arc = s.arc.is_some();
        self.had_sub_duck = s.sub_duck.is_some();
        self.had_pad_sweep = s.pad_sweep.is_some();
        self.had_pad_wow = s.pad_wow.is_some();
        let colors = !lp.base.colors.is_empty();
        self.color_change = colors != self.had_color;
        self.had_color = colors;
        self.glass = on("glass");
        self.choir = lp.spec.choir().is_some();
        synth.set_tone(host, lp.spec.tone, now);
        synth.set_level(host, s.level.get(), now);
        Synth::set_echo(host, (60.0 / self.bpm) * s.echo_beats.get(), now);
        for layer in LAYERS {
            let bit = 1u8 << layer as usize;
            if lp.spec.layers_off & bit != 0 {
                self.mutes[layer as usize] = true;
            } else if lp.spec.layers_on & bit != 0 {
                self.mutes[layer as usize] = false;
            }
        }
        for layer in LAYERS {
            synth.set_mute(host, channel_of(layer) as usize, self.mutes[layer as usize], now);
        }
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // whole steps
        let total = lp.total() as u64;
        if total > 0 && Piece::of(lp.spec).is_none() {
            self.step %= total;
        }
        self.kick_kind = s.kick.map(|k| k.kind);
    }

    /// What a long piece's plan has heard at section step `step` (queue item
    /// 102b, `heard::Unit`), sent at `time`: each layer's level and the
    /// tone, on the unit's first step and, where something glides in it, on
    /// each beat, gliding to where the unit will be a beat on. Nothing is
    /// sent where nothing moved (`Synth::set_plan_level`, `glide_tone`).
    #[allow(clippy::too_many_arguments)] // the step's own place and times
    fn hear<H: Host>(&mut self, host: &mut H, synth: &mut Synth, lp: &Loop, u: &heard::Unit, step: f64, time: f64, sd: f64) {
        const BEAT: f64 = 4.0;
        let glides = u.glides();
        if step != u.from && !(glides && (step - u.from) % BEAT == 0.0) {
            return;
        }
        let (ahead, tau) = if glides { (js::min(u.to, step + BEAT), BEAT * sd / 3.0) } else { (step, 0.05) };
        for layer in LAYERS {
            synth.set_plan_level(host, channel_of(layer) as usize, u.level(layer, ahead), time, tau);
        }
        if !self.tone_held {
            let moved = u.tone_at(ahead);
            let own = lp.spec.tone;
            synth.glide_tone(host, [own[0] + moved[0], own[1] + moved[1]], moved, time, tau);
        }
        // An arc's energy aims the loop's own slow filter (queue item 102c).
        if lp.pass.is_some_and(|p| p.heard.arc) {
            let e = &crate::generator::data::ENERGY;
            synth.glide_aim(host, u.aim_at(ahead), e.dark, e.open, time, tau);
        }
    }

    /// A finite arc's landing (queue item 102c, `heard::Landing`), on its
    /// first step: the loop's tonic chord, in the chords' voice as the plan
    /// has it, and the tune on the tonic, held for `hold` steps and ringing
    /// out; nothing else sounds after it (the pass leaves every layer's
    /// notes out). A muted layer stays silent.
    #[allow(clippy::too_many_arguments)] // the step's own place and times
    fn land<H: Host>(host: &mut H, synth: &mut Synth, lp: &Loop, l: &heard::Landing, mutes: [bool; 5], t: f64, sd: f64, key: &mut impl FnMut(Layer) -> Key) {
        let dur = l.hold * sd;
        synth.budget.set_firm(true);
        if !mutes[Layer::Chords as usize] && !l.chord.is_empty() {
            let e = lp.base.chords.get(l.chord_from);
            let voice = lp.voice(Layer::Chords, e.map_or("", |e| e.voice.get()));
            let opts = Opts { vowel: e.and_then(|e| e.vowel), detune: e.and_then(|e| e.detune), ..Opts::default() };
            if voice == "pad" {
                let k = key(Layer::Chords);
                synth.budget.begin(Some(Layer::Chords), k);
                synth.pad(host, k, &l.chord, t, dur, l.chord_vel * CHORD_SPREAD, CHORDS);
                synth.budget.end();
            } else if !voice.is_empty() {
                let spread = CHORD_SPREAD / math::sqrt(js::num(l.chord.len()));
                // Fingers, or a roll where the loop plays a note at a time.
                let apart = if l.rolled { LAND_ROLL } else { 0.011 };
                for (i, &n) in l.chord.iter().enumerate() {
                    let k = key(Layer::Chords);
                    synth.budget.begin(Some(Layer::Chords), k);
                    synth.voice(host, k, voice, f64::from(n), t + js::num(i) * apart, dur, l.chord_vel * spread, CHORDS, &opts);
                    synth.budget.end();
                }
            }
        }
        if !mutes[Layer::Melody as usize] && l.tune > 0.0
            && let Some(e) = lp.base.melody.get(l.tune_from)
        {
            let vowel = (!e.vowel.is_empty()).then_some(e.vowel.get());
            let opts = Opts { vowel, detune: e.detune, ..Opts::default() };
            let k = key(Layer::Melody);
            synth.budget.begin(Some(Layer::Melody), k);
            synth.voice(host, k, lp.voice(Layer::Melody, e.voice.get()), l.tune, t, dur, l.tune_vel, MELODY, &opts);
            synth.budget.end();
        }
        synth.budget.set_firm(false);
    }

    /// A pivot (session 15, `heard::Unit::pivot`): the chord both keys
    /// share, in the chords' voice as the plan has it there, held across
    /// the line. A muted layer stays silent.
    #[allow(clippy::too_many_arguments)] // the step's own place and times
    fn pivot<H: Host>(host: &mut H, synth: &mut Synth, lp: &Loop, u: &heard::Unit, mutes: [bool; 5], t: f64, sd: f64, key: &mut impl FnMut(Layer) -> Key) {
        if mutes[Layer::Chords as usize] {
            return;
        }
        let notes = &u.pivot[..u.pivot_count.min(3)];
        let dur = u.pivot_hold * sd;
        let e = lp.base.chords.get(u.pivot_from);
        let voice = lp.voice(Layer::Chords, e.map_or("", |e| e.voice.get()));
        let opts = Opts { vowel: e.and_then(|e| e.vowel), detune: e.and_then(|e| e.detune), ..Opts::default() };
        if voice == "pad" {
            let mut chord = crate::generator::plan::Notes::new();
            for &n in notes {
                chord.push(n);
            }
            let k = key(Layer::Chords);
            synth.budget.begin(Some(Layer::Chords), k);
            synth.pad(host, k, &chord, t, dur, u.pivot_vel * CHORD_SPREAD, CHORDS);
            synth.budget.end();
        } else if !voice.is_empty() {
            let spread = CHORD_SPREAD / math::sqrt(js::num(notes.len()));
            for (i, &n) in notes.iter().enumerate() {
                let k = key(Layer::Chords);
                synth.budget.begin(Some(Layer::Chords), k);
                synth.voice(host, k, voice, f64::from(n), t + js::num(i) * 0.011, dur, u.pivot_vel * spread, CHORDS, &opts);
                synth.budget.end();
            }
        }
    }

    /// A new pass begins (`_advance`'s wrap, and the load's first): the
    /// tuned kick it plays, its own on a pass as composed, else the swap a
    /// wandering pass may draw (`_kickOn`, `kickKindOf`, by the pass's
    /// count).
    pub fn pass_begins(&mut self, lp: &Loop) {
        self.kick_kind = lp.settings.kick.map(|k| if lp.pass.is_some_and(|p| p.breathes) { choices::kick_kind_of(lp.spec.seed, k.kind, self.loop_count, lp.spec.pins.get("tunedKick").is_some()) } else { k.kind });
    }

    /// `_advance`: the next step's time and place. True at the top of a
    /// new pass (the caller hands the sequencer the pass it plays). In a
    /// piece (queue item 88) a pass is a section: a round's last may be cut
    /// short, and the round after it counts its bars from where it begins.
    pub fn advance(&mut self, lp: &Loop) -> bool {
        self.next_step_time += self.step_dur();
        self.step += 1;
        self.abs_step += 1;
        #[allow(clippy::cast_precision_loss)] // steps, far below 2^53
        let wrapped = self.step as f64 >= self.pass_steps;
        if wrapped {
            self.step = 0;
            self.loop_count = self.loop_count.wrapping_add(1);
            self.pass_steps = lp.total();
            if let Some(p) = Piece::of(lp.spec) {
                let n = u64::from(self.loop_count);
                self.pass_steps = p.bars_of(n) * lp.spec.steps_per_bar;
                if p.cut() && p.begins_round(n) {
                    self.origin = self.abs_step;
                }
            }
        }
        wrapped
    }

    /// `_isSilentBar(absStep, rel)`: every layer scheduled out for the bar
    /// holding the step, `abs` from the load and `rel` from the round's
    /// start (the same, but in a piece with a section cut short).
    fn is_silent_bar(lp: &Loop, abs: f64, rel: f64) -> bool {
        let spb = lp.spec.steps_per_bar;
        let total = lp.total();
        let bar = js::floor((rel % total) / spb);
        if lp.base.quiet.iter().any(|&q| js::num(q) == bar) {
            return true;
        }
        let Some(form) = lp.base.form.as_ref() else { return false };
        for layer in LAYERS {
            let sched = &form[layer as usize];
            // A layer re-rolled plays throughout (queue item 94a).
            if sched.is_empty() {
                return false;
            }
            let n = js::num(sched.len());
            let cycle = lp.cycle(layer);
            let b = js::floor(((if cycle == total { rel } else { abs }) % cycle) / spb) % n;
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
        #[allow(clippy::cast_precision_loss)] // the same
        let rel = (self.abs_step - self.origin) as f64;
        #[allow(clippy::cast_precision_loss)]
        let stepf = step as f64;

        // The loop's tuning, on every note this step sends (queue item 101b).
        synth.set_tuning(s.tuning);
        // A long piece's plan at this step: its levels and its tone (queue
        // item 102b).
        if let Some(u) = lp.pass.and_then(|p| p.heard.unit(stepf)) {
            self.hear(host, synth, lp, u, stepf, time, self.step_dur());
        }
        // A new loop's setups, from its first step decided.
        if self.organ_change {
            Synth::organ_off(host, time, self.organ_fall.get());
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
        if self.pad_wow_change {
            synth.pad_wow_setup(host, s.pad_wow.as_ref(), time);
            self.pad_wow_change = false;
        }
        // The last loop's color lets go at a new loop's first step, and the
        // channel's color is on only where this one plays colors.
        if self.color_change {
            self.let_color_go(host, time);
            Synth::color_setup(host, self.had_color, time);
            self.color_change = false;
        }
        if self.bass_lift_change {
            host.bass_lift(s.bass_lift);
            self.bass_lift_change = false;
        }
        if self.space_change {
            synth.space_setup(host, &s.space, time);
            self.space_change = false;
        }
        // At each bar line, whether the tails should still be ringing.
        let spb = lp.spec.steps_per_bar;
        if abs % spb == 0.0 {
            let quiet = Self::is_silent_bar(lp, abs, rel);
            // The organ lets go in a bar the loop leaves silent.
            if quiet && self.hold.on {
                Synth::organ_off(host, time, 0.25);
                self.hold = NO_HOLD;
            }
            // And so does the color, and the low bell is damped.
            if quiet {
                self.let_color_go(host, time);
                self.damp_bell(host, time);
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
            // A tuned kick moves with the bass to the plan's key.
            let by = lp.pass.map_or(0, |p| p.tonic(stepf, Layer::Bass));
            let key_ratio = if by == 0 { 1.0 } else { math::pow(2.0, f64::from(by) / 12.0) };
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
                // slowOpening (queue item 101a): the kit rises to its level
                // over the fade from its arrival (none: exactly as it was),
                // and a hit faded to nothing is not played.
                let vel = vel * late.map_or(1.0, |l| l.rise(abs - l.drums, sd)); #[cfg(feature = "sketch")] let vel = vel * late.map_or(1.0, |l| crate::levers::late_fade(abs - l.drums, sd));
                if vel == 0.0 {
                    continue;
                }
                let jitter = if e.roll { 0.0 } else { (unit(key, DRUMS_CHANNEL, salt::HOST_JITTER, 0) - 0.5) * 0.008 };
                let at = t + micro + jitter;
                let kicked = inst == "kick" || inst == "softkick";
                let hit = kick.filter(|_| kicked).map(|(k, kind)| KickHit {
                    hz: k.kick.hz * key_ratio,
                    fall: match kind.unwrap_or(k.kind) {
                        "short" => Some(k.kick.short),
                        "long" => Some(k.kick.long),
                        _ => None::<Fall>,
                    },
                });
                synth.budget.begin(Some(Layer::Drums), key);
                synth.drum(host, key, inst, at, vel, hit);
                synth.budget.end();
                if truthy(pump) && kicked {
                    Synth::duck(host, t + micro, pump * vel);
                }
            }
            // A fill into a long piece's line (session 15): the kit's own
            // instrument on the steps of the last beat where the loop has
            // no hit of it, rising; where the kit sits out, so does it.
            if let Some((inst, vel)) = lp.pass.and_then(|p| p.fill_at(stepf))
                && !lp.base.drums.iter().any(|e| e.step == s_at && e.inst == inst && truthy(e.vel))
                && !late.is_some_and(|l| abs < l.drums)
            {
                let vel = vel * late.map_or(1.0, |l| l.rise(abs - l.drums, sd));
                if vel > 0.0 {
                    let key = key_of(Layer::Drums);
                    synth.budget.begin(Some(Layer::Drums), key);
                    synth.drum(host, key, inst, t, vel, None);
                    synth.budget.end();
                }
            }
        }
        if !mutes[Layer::Bass as usize] {
            let s_at = at(Layer::Bass);
            // A drone's notes are a held instrument's: never let go for a
            // newer note (queue item 85e; `meta.bassStyle`, the pedal's
            // when it has one).
            let drone = lp.base.bass_out.style == "drone" && lp.base.pedal.is_none();
            for (i, e) in lp.base.bass.iter().enumerate() {
                let (vel, midi) = lp.bass(i, e);
                if e.step != s_at || !truthy(vel) {
                    continue;
                }
                let key = key_of(Layer::Bass);
                if late.is_some_and(|l| abs < l.bass) {
                    continue;
                }
                // The bass rises as the kit does.
                let vel = vel * late.map_or(1.0, |l| l.rise(abs - l.bass, sd)); #[cfg(feature = "sketch")] let vel = vel * late.map_or(1.0, |l| crate::levers::late_fade(abs - l.bass, sd));
                if vel == 0.0 {
                    continue;
                }
                synth.budget.begin(Some(Layer::Bass), key);
                if drone {
                    synth.budget.keep();
                }
                synth.bass(host, key, midi, t, e.dur * sd, vel, e.glide, lp.voice(Layer::Bass, e.voice.get()), e.chug);
                synth.budget.end();
            }
        }
        // An ensemble does not attack together: a choir's layers each slip,
        // drawn once a layer a step.
        let choir = self.choir;
        let ensemble = |channel: u32| {
            if choir { (unit(Key { seed, pass, step: step32, ord: 0 }, channel, salt::HOST_ENSEMBLE, 0) - 0.5) * 0.016 } else { 0.0 }
        };

        // dropSpotlight (queue item 101g): in a loop that strikes the bell in
        // its floor drops alone, at each bar line whether one starts (and so
        // this step is its first) or the floor comes back (the bell damped).
        let mut spot_first = false;
        if s.spotlight && abs % spb == 0.0 {
            let more = Self::floor_out_from(lp, stepf, abs);
            self.bell_floor |= more == 0.0;
            if self.bell_drop > 0.0 && abs >= self.bell_drop {
                if more > 0.0 {
                    // Still out across a pass's end: the drop goes on.
                    self.bell_drop = abs + more;
                } else {
                    if self.bell_until > t {
                        Synth::low_bell_off(host, t, SPOT_DAMP);
                    }
                    self.bell_until = 0.0;
                    self.bell_pc = 0;
                    self.bell_drop = 0.0;
                }
            } else if self.bell_drop == 0.0
                && self.bell_floor
                && !self.bell_was_out
                && more * sd >= data::LOWBELL.spotlight_least - 0.001
                && !late.is_some_and(|l| abs < js::max(l.drums, l.bass))
            {
                self.bell_drop = abs + more;
                spot_first = true;
            }
            self.bell_was_out = more > 0.0;
        }
        let in_drop = self.bell_drop > 0.0 && abs < self.bell_drop;
        if !mutes[Layer::Chords as usize] {
            let s_at = at(Layer::Chords);
            let slip = ensemble(CHORDS);
            for (i, e) in lp.base.chords.iter().enumerate() {
                let (vel, chord) = lp.chord(i, e);
                if e.step != s_at || !truthy(vel) {
                    continue;
                }
                let dur = lp.chord_dur(i, e) * sd;
                let opts = Opts { vowel: e.vowel, detune: e.detune, ..Opts::default() };
                let voice = lp.voice(Layer::Chords, e.voice.get());
                if voice == "pad" {
                    let k = key_of(Layer::Chords);
                    synth.budget.begin(Some(Layer::Chords), k);
                    synth.pad(host, k, chord.notes(), t, dur, vel * CHORD_SPREAD, CHORDS);
                    synth.budget.end();
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
                        let k = key_of(Layer::Chords);
                        synth.budget.begin(Some(Layer::Chords), k);
                        synth.voice(host, k, voice, f64::from(string), t + slip + at, dur, vel * spread * weight, CHORDS, &strummed);
                        synth.budget.end();
                    }
                    continue;
                }
                for (k, &n) in notes.iter().enumerate() {
                    // A few milliseconds apart, fingers rather than a switch.
                    let at = t + slip + js::num(k) * 0.011;
                    let k = key_of(Layer::Chords);
                    synth.budget.begin(Some(Layer::Chords), k);
                    synth.voice(host, k, voice, f64::from(n), at, dur, vel * spread, CHORDS, &opts);
                    synth.budget.end();
                }
            }
            if self.had_color {
                self.colors(host, synth, lp, s_at, t, sd);
            }
            if s.low_bell != 0 && (!s.spotlight || in_drop) && !lp.rests(Layer::Chords) && !Self::is_silent_bar(lp, abs, rel) {
                let kind = Bell { swell: s.low_bell == 2, spot: s.spotlight, first: spot_first };
                self.low_bell(host, synth, lp, s_at, t, kind, Key { seed, pass, step: step32, ord: BELL_ORD });
            }
            if s.organ.is_some() {
                self.organ(host, synth, lp, s_at, t, sd, abs);
            } #[cfg(feature = "sketch")] if s.organ.is_none() { Self::retune_chords(host, lp, s_at, t); }
            if self.glass {
                Self::glass(host, synth, lp, s_at, t, sd, Key { seed, pass, step: step32, ord: 0 });
            }
            if let Some(stab) = &s.stab {
                Self::dub_stab(host, synth, lp, stab, mutes, &at, s_at, t, [abs, rel], Key { seed, pass, step: step32, ord: 0 });
            }
        }
        // Muted, or resting in a long piece's plan, the chords' organ lets
        // go.
        if (mutes[Layer::Chords as usize] || lp.rests(Layer::Chords)) && self.hold.on {
            Synth::organ_off(host, t, 0.25);
            self.hold = NO_HOLD;
        }
        if mutes[Layer::Chords as usize] || lp.rests(Layer::Chords) {
            self.let_color_go(host, t);
            self.damp_bell(host, t);
        }
        if !mutes[Layer::Melody as usize] {
            let s_at = at(Layer::Melody);
            let slip = ensemble(MELODY);
            // Every pitch is made in the home key and moved to the plan's
            // (queue items 102b and 102d): a fifth away the two are the same
            // thing, and in another mode a grace or a third under the note
            // stays in the key.
            let root = lp.spec.root;
            let mv = |x: f64| x + lp.moved(Layer::Melody, x);
            // Whose tune this step plays (session 14): the loop's own, the
            // section's second, or a rest; one voice, never two tunes.
            let which = lp.pass.map_or(tune::OWN, |p| p.heard.tune_at(stepf));
            let second = which == tune::SECOND;
            let notes: &[MelodyEvent] = match which {
                tune::OWN => &lp.base.melody,
                tune::SECOND => lp.second.map_or(&[], |m| &m[..]),
                _ => &[],
            };
            // A stretched tune plays each phrase's first half at half speed:
            // the note written half as far into the phrase, twice as long.
            let stretched = lp.pass.is_some_and(|p| p.heard.unit(stepf).is_some_and(|u| u.stretched));
            let from = lp.pass.and_then(|p| p.heard.stretched_from(stepf));
            for (i, e) in notes.iter().enumerate() {
                if if stretched { from != Some(e.step) } else { e.step != s_at } {
                    continue;
                }
                let (vel, midi) = if second { lp.pass.map_or((e.vel, e.midi), |p| p.second_at(stepf, i, e)) } else { lp.melody_home(i, e) };
                if !truthy(vel) {
                    continue;
                }
                let dur = e.dur * sd * if stretched { 2.0 } else { 1.0 };
                let vowel = (!e.vowel.is_empty()).then_some(e.vowel.get());
                let opts = Opts { vowel, detune: e.detune, prev: e.prev.map(mv), ..Opts::default() };
                let k = key_of(Layer::Melody);
                synth.budget.begin(Some(Layer::Melody), k);
                let voice = lp.voice(Layer::Melody, e.voice.get());
                synth.voice(host, k, voice, mv(midi), t + slip, dur, vel, MELODY, &opts);
                synth.budget.end();
                // Asked for after the note, so where the budget is short it
                // is the decoration that yields, never the tune.
                let mode = e.steps.as_ref().map_or(lp.spec.steps(), |s| &s[..]);
                let line = Opts { vowel, detune: e.detune, ..Opts::default() };
                // Toward an arc's peak: the tune doubled an octave up,
                // softly, swelling in (queue item 102c).
                // It yields to everything: played only where the budget has
                // room to spare (as a dub stab), so it never keeps another
                // layer's note out.
                let (double, share) = lp.pass.map_or((0, 0.0), |p| p.double(stepf));
                if double != 0 && share > 0.0 && synth.budget.room(t + slip, DOUBLE_ROOM) {
                    let k = key_of(Layer::Melody);
                    synth.budget.begin(Some(Layer::Melody), k);
                    synth.voice(host, k, voice, mv(midi) + f64::from(double), t + slip, dur, vel * share, MELODY, &line);
                    synth.budget.end();
                }
                if let Some(under) = harmony_of(e.harm, midi, root, mode) {
                    let k = key_of(Layer::Melody);
                    synth.budget.begin(Some(Layer::Melody), k);
                    synth.voice(host, k, voice, mv(under), t + slip, dur, vel * HARMONY_VEL, MELODY, &line);
                    synth.budget.end();
                }
                let (graces, n) = graces_of(e.orn, midi, root, mode);
                for (k, &g) in graces.iter().take(n).enumerate() {
                    let at = t + slip - GRACE * js::num(n - k);
                    let k = key_of(Layer::Melody);
                    synth.budget.begin(Some(Layer::Melody), k);
                    synth.voice(host, k, voice, mv(g), js::max(now, at), GRACE, vel * GRACE_VEL, MELODY, &line);
                    synth.budget.end();
                }
            }
        }
        // A finite arc lands, on its landing's first step (queue item 102c).
        if let Some(l) = lp.pass.map(|p| &p.heard.landing).filter(|l| l.on && l.strikes && l.at == stepf) {
            Self::land(host, synth, lp, l, mutes, t, sd, &mut key_of);
        }
        // A long piece's key changes across a line (session 15): a chord
        // both keys share, held across it.
        if let Some(u) = lp.pass.and_then(|p| p.pivot_at(stepf)) {
            Self::pivot(host, synth, lp, u, mutes, t, sd, &mut key_of);
        }
        if !mutes[Layer::Texture as usize] {
            let s_at = at(Layer::Texture);
            let rests = lp.rests(Layer::Texture);
            for e in lp.base.texture.iter() {
                if e.step != s_at || !truthy(e.vel) {
                    continue;
                }
                let k = key_of(Layer::Texture);
                // A long piece's plan: resting (its place still counted),
                // left out for a join, or in another key.
                if rests || lp.pass.is_some_and(|p| p.out(stepf, Layer::Texture, e.dur)) {
                    continue;
                }
                let mut notes = e.notes;
                #[allow(clippy::cast_possible_truncation)] // whole semitones
                notes.iter_mut().for_each(|n| *n += lp.moved(Layer::Texture, f64::from(*n)) as i16);
                synth.budget.begin(Some(Layer::Texture), k);
                synth.texture(host, k, e.kind.get(), &notes, t, e.dur * sd, e.vel, e.band);
                synth.budget.end();
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
                if abs - h.last.get() < spb || (a.first().map(|&n| pc(n)) == b.first().map(|&n| pc(n)) && shared >= 2) {
                    continue;
                }
            }
            let mut hold = Hold { key: Some(chord), until: abs + (next - s), held: Some(chord.sorted()), last: Biased::of(abs), on: true };
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

    /// colorTone's colors that start at chords step `s` (queue item 101e):
    /// each where the chord it sounds with is played this pass (not in a
    /// gap, a rest or a breath), in the plan's key, held its length.
    fn colors<H: Host>(&mut self, host: &mut H, synth: &Synth, lp: &Loop, s: f64, t: f64, sd: f64) {
        for c in lp.base.colors.iter().filter(|c| c.step == s) {
            let i = usize::from(c.anchor);
            let Some(e) = lp.base.chords.get(i) else { continue };
            if !truthy(lp.chord(i, e).0) {
                continue;
            }
            let midi = f64::from(c.midi);
            let seconds = c.dur * sd;
            synth.color(host, midi + lp.moved(Layer::Chords, midi), t, seconds);
            self.color_until = js::max(self.color_until, t + seconds);
        }
    }

    /// lowBell (queue item 101g): at the first step `s` of a chord of the
    /// progression (a slot of the harmony) that plays this pass (an event of
    /// the slot sounds: not all of it in a gap, a rest or a breath), the low
    /// bell strikes the slot's root, in the plan's key, as its pitch class
    /// between D2 and C#3, unless that is the root it struck last and less
    /// than `regap` seconds ago (`[setting lowbell]`). The generator knows
    /// its chords, so an arpeggio's chord is struck as a whole one is.
    ///
    /// dropSpotlight's (`kind.spot`, in a floor drop): the same, and on the
    /// drop's first step (`kind.first`) the root of the chord of the
    /// progression there, whatever it struck before and whether or not the
    /// chords play it (`low_bell::ALONE` where they don't).
    #[allow(clippy::too_many_arguments)] // the step's own place and times
    fn low_bell<H: Host>(&mut self, host: &mut H, synth: &mut Synth, lp: &Loop, s: f64, t: f64, kind: Bell, key: Key) {
        let h = &lp.base.harmony;
        let slot = if kind.first { Some(h.slot_at(s)) } else { h.slots.iter().find(|x| x.start_step == s) };
        let Some(slot) = slot else { return };
        let end = slot.start_step + slot.length_steps;
        let plays = lp.base.chords.iter().enumerate().any(|(i, e)| e.step >= slot.start_step && e.step < end && truthy(lp.chord(i, e).0));
        // A drop's first strike is the bell alone in it, whatever the chords
        // do there (as the Ears 13b sketch's rang over a chord gone quiet).
        if !plays && !kind.first {
            return;
        }
        let root = slot.root_midi + lp.moved(Layer::Chords, slot.root_midi);
        let pc = (((root - BELL_LOW) % 12.0) + 12.0) % 12.0;
        if !pc.is_finite() {
            return;
        }
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // 0 to 11
        let mark = js::round(pc) as u8 % 12 + 1;
        // A millisecond's grace, so a chord exactly `regap` on is struck
        // however the times round.
        if !kind.first && mark == self.bell_pc && t - self.bell_at < data::LOWBELL.regap - 0.001 {
            return;
        }
        self.bell_pc = mark;
        self.bell_at = t;
        let parts = (if kind.swell { low_bell::SWELL } else { 0 }) | (if kind.spot { low_bell::SPOT } else { 0 }) | (if plays { 0 } else { low_bell::ALONE });
        synth.low_bell(host, key, BELL_LOW + pc, t, parts);
        self.bell_until = js::max(self.bell_until, t + LOW_BELL_RING);
    }

    /// dropSpotlight (queue item 101g): how many steps from loop step `from`
    /// (a bar line, `abs` from the load) to the end of this pass the floor
    /// stays out, bar by bar: none of the kit's kicks, soft kicks or frame
    /// hits and no bass note sounding in the bar, as this pass plays them
    /// (the wander's floor drop and its shapes, a form's bars without the
    /// floor, a breath), each layer where its own cycle puts it, as
    /// `schedule_step` places it. The generator's own marks, so it is known
    /// on the drop's first beat.
    fn floor_out_from(lp: &Loop, from: f64, abs: f64) -> f64 {
        let spb = lp.spec.steps_per_bar;
        let total = lp.total();
        // Whether a note at step `step` of a layer whose cycle is `len`,
        // lasting `dur` steps, sounds in loop steps `a` to `b` (`abs` from
        // the load at `a`): a layer on the loop's cycle by the loop's step,
        // one on a cycle of its own by the step from the load.
        let sounds = |step: f64, dur: f64, len: f64, a: f64, b: f64| {
            let (start, end, len) = if len == total || len <= 0.0 {
                (a, b, total)
            } else {
                let at = abs + (a - from);
                (at, at + (b - a), len)
            };
            let mut x = start - (((start - step) % len) + len) % len;
            while x < end {
                if x + dur > start {
                    return true;
                }
                x += len;
            }
            false
        };
        let (drums, bass) = (lp.cycle(Layer::Drums), lp.cycle(Layer::Bass));
        let plays = |a: f64, b: f64| {
            lp.drums().any(|(vel, inst, e)| truthy(vel) && matches!(inst, "kick" | "softkick" | "frame") && sounds(e.step, 1.0, drums, a, b))
                || lp.base.bass.iter().enumerate().any(|(i, e)| truthy(lp.bass(i, e).0) && sounds(e.step, js::max(1.0, e.dur), bass, a, b))
        };
        let mut at = from;
        while at < total && !plays(at, at + spb) {
            at += spb;
        }
        at - from
    }

    /// The low bell's ring damped at `time` (a full stop, the chords muted
    /// or resting), if one may still sound; the next chord strikes afresh.
    fn damp_bell<H: Host>(&mut self, host: &mut H, time: f64) {
        if self.bell_until > time {
            Synth::low_bell_off(host, time, BELL_DAMP);
        }
        self.bell_until = 0.0;
        self.bell_pc = 0;
    }

    /// The color lets go at `time`, if one may still sound.
    fn let_color_go<H: Host>(&mut self, host: &mut H, time: f64) {
        if self.color_until > time {
            Synth::color_off(host, time);
        }
        self.color_until = 0.0;
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
    fn dub_stab<H: Host>(host: &mut H, synth: &mut Synth, lp: &Loop, stab: &Stab, mutes: [bool; 5], at: &impl Fn(Layer) -> f64, s: f64, t: f64, [abs, rel]: [f64; 2], key: Key) {
        let spb = lp.spec.steps_per_bar;
        let pos = s % spb;
        let which = if pos == 2.0 {
            0
        } else if pos == js::floor(spb / 2.0) + 2.0 {
            1
        } else {
            return;
        };
        if !(stab.rates[which] > 0.0) || Self::is_silent_bar(lp, abs, rel) {
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
                    lp.base.bass.iter().enumerate().filter(|(i, e)| inside(e.step, lp.bass(*i, e).0)).count()
                }
                Layer::Melody => {
                    lp.base.melody.iter().enumerate().filter(|(i, e)| inside(e.step, lp.melody(*i, e).0)).count()
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

/// How the low bell is struck at a step (queue item 101g): its swelling kind,
/// in a floor drop (dropSpotlight), and on a drop's first step.
#[derive(Clone, Copy)]
struct Bell {
    swell: bool,
    spot: bool,
    first: bool,
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

// The sketch levers' hooks (queue item 99), after the code the app runs so
// that the lines of that code do not move (`tools/build-core.sh --check`
// holds the player's module to its panic locations).
#[cfg(feature = "sketch")]
impl Sequencer {
    /// At the first step of a chord of the progression (a slot of the
    /// harmony), send the chords channel a retune to the whole chord (the
    /// `chords.RETUNE` constant, `tuning`; the sketch build only): the notes
    /// of every chord event in the slot, so an arpeggio's single notes
    /// arrive as the chord they make. Today only the organ's loops retune
    /// (`Synth::organ_chord`), so a chord sketch had to be played with a
    /// silent organ. Held voices that hear a retune (the choir's) follow it
    /// too. A loop with the organ keeps the organ's own.
    fn retune_chords<H: Host>(host: &mut H, lp: &Loop, s: f64, t: f64) {
        use crate::block::event::NOTES;
        if crate::tuning::get("chords.RETUNE", 0.0) == 0.0 {
            return;
        }
        let Some(slot) = lp.base.harmony.slots.iter().find(|x| x.start_step == s) else { return };
        let end = slot.start_step + slot.length_steps;
        let mut chord = [i16::MIN; NOTES];
        let mut n = 0;
        for (i, e) in lp.base.chords.iter().enumerate() {
            let (vel, played) = lp.chord(i, e);
            if e.step < slot.start_step || e.step >= end || !truthy(vel) {
                continue;
            }
            for &m in played.notes() {
                if n < NOTES && !chord[..n].contains(&m) {
                    chord[n] = m;
                    n += 1;
                }
            }
        }
        if n == 0 {
            return;
        }
        chord[..n].sort_unstable();
        let mut notes = [0.0; NOTES];
        for (x, &m) in notes.iter_mut().zip(&chord[..n]) {
            *x = f64::from(m) + lp.settings.tuning / 100.0;
        }
        host.event(CHORDS, crate::block::Event::retune(t, &notes[..n]));
    }
}
