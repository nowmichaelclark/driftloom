//! The player (queue item 82): a loop from a seed, played in the core, with
//! no JavaScript. Broken Friends' gate (each friend's theme, from a seed),
//! and the end of phase 2's port.
//!
//! It is what the page does between the generator and the core's notes:
//! the generator (`generator`, items 79 to 81) makes the loop and what the
//! engine decides loading it; the sequencer (`sequencer`, `Engine` in
//! `js/engine.js`) walks it step by step, pass after pass, wandering as the
//! app's does; and the synth's note calls (`synth`, `Synth` in
//! `js/synth.js`) ask the voice budget and hand each note to the core with
//! everything it carries. Proven against the page: the baseline's loops,
//! played here from their seeds, make the samples the app's own engine
//! makes in Chromium (`tools/player.mjs`, `tools/baseline.mjs`'s `player`
//! part), bit for bit.
//!
//! **How a host plays one.** It starts a core for the player (`start`),
//! asks for a loop (`load`: a seed, and a `Request`: the vibes, the tempo) and
//! renders (`render`, any number of frames a call). Before each block the
//! core renders, the player decides every step due within `LOOKAHEAD` and
//! hands its notes over (`decide`, which a host driving `Core::process`
//! itself calls before each block). Wander, mutes, tempo and volume are
//! the app's controls, as it has them (`set_wander` and the rest).
//!
//! **A change at a moment** (queue item 86, `change`): `change` asks for the
//! next loop at a beat, bar, phrase or pass of the one playing, handing over
//! by a crossfade or a ring-out, on the same core with no reset; the new loop
//! is made in the blocks before the boundary and starts on it.
//!
//! **Spread across blocks** (promise 1). A load is stages of work, each run
//! in a block of its own (`Generator::step`): the spec, the plan, the render
//! in five parts, what the engine decides at load, the tuned kick's fit in
//! slices (`choices::KICK_SLICE`), the first pass; then the loop starts. So
//! a load never spikes one block: its worst stage is about 0.14 ms natively
//! (a block's deadline is 2.9 ms at 44.1 kHz), and on every target less
//! than the loop's own worst audio block. A host with no
//! deadline (an offline render, a loading screen, `dlrender --seed`) runs
//! them all at once with `ready`, and the loop starts on the next block.
//! Each pass's wandering is drawn a pass ahead, in a block where no step is
//! decided.
//!
//! **Nothing allocates**: the generator's lists, two passes and the budget
//! are fixed, in the player, which a host keeps (a static, or boxed: it is
//! a megabyte and a half, `size_of::<Player>()`).

// The port compares doubles as the JavaScript does (`===`, `!(a > b)` where a
// NaN must fall through, `(a + b) / 2` in its own order), and keeps its short
// names and its calls' arguments, so the two read side by side.
#![allow(
    clippy::float_cmp,
    clippy::neg_cmp_op_on_partial_ord,
    clippy::manual_midpoint,
    clippy::many_single_char_names,
    clippy::too_many_arguments
)]

pub mod about;
pub mod ahead;
pub mod budget;
pub mod change;
pub mod events;
pub mod formant;
pub mod host; #[cfg(feature = "tools")] pub mod probe;
pub mod sequencer;
pub mod synth;
pub mod track;

use crate::generator::js;
use crate::generator::piece::Piece;
use crate::generator::plan::Cx;
use crate::generator::spec::Metre;
use crate::generator::wander::{AMOUNT, Pass};
pub use crate::generator::request::Length;
use crate::generator::{Generator, Layer};
use crate::{Core, Error, MAX_RATE, MIN_RATE};

use change::{Change, END_SLOT, LOAD_BLOCKS, SLOTS};
pub use change::{CROSSFADE, END_FADE, Handover, Moment};
pub use events::{Event, Kind};
use events::Events;
use crate::generator::load::Load;
pub use track::Track;
use host::Host;
use crate::generator::list::{Biased, Maybe};
use crate::generator::rng::Rng;
use sequencer::{Loop, ORGAN_FALL, Sequencer, channel_of};
use synth::Synth;

/// How far ahead the player decides, in seconds: before the block it
/// renders next, every step whose time falls before the block's start plus
/// this is decided and its notes handed over. Room for a step's earliest
/// note (its graces run 0.06 s ahead of it, a tail's restore 0.12 s) to
/// reach the core before its block renders; a change (a mute, the tempo)
/// is heard at most this late. `tools/player.mjs` hands the page's notes
/// over the same way.
pub const LOOKAHEAD: f64 = 0.25;
/// When a loop's first step falls after the block it starts on, in
/// seconds (the measure harness's 0.05).
pub const START: f64 = 0.05;

/// What a host asks for, beside the seed (promise 11's request, queue item
/// 85: the vibes with or without weights, and the tempo): the generator's
/// own, `generator::request::Request`, which says how to build one.
pub use crate::generator::request::Request;

/// Where the player is.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    /// Nothing asked for yet.
    Idle,
    /// A loop being made, a stage a block.
    Loading,
    /// The loop made; its first pass to draw before it starts.
    Wandering,
    /// A loop playing.
    Playing,
}

/// How the loop being made was asked for: by a load (the app's way, its
/// first pass drawn at the pass count the last loop reached), or by a change
/// now or at a moment (its first pass its own first; at a moment, its tone,
/// level and echo glide in at the boundary rather than when it is made), or
/// as the loop playing made again in its place (`revise`, a re-roll).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Asked {
    Load,
    Now,
    Moment,
    InPlace,
}

/// What `decide` did before a block, so a host timing its blocks (`dlrender
/// --costs`, `tools/player-wasm.mjs --costs`) can say what each cost.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Work {
    /// Nothing: no step was due.
    None,
    /// A stage of a load (the first pass drawn and the loop's start among
    /// them).
    Load,
    /// The next pass drawn, a pass ahead.
    Pass,
    /// Steps decided and their notes handed over: how many.
    Steps(usize),
}

/// A loop from a seed, on a core (the module's doc says how).
// Its flags are each a state of its own (a pass drawn, a loop that played, a
// lead-in, the quality), not one state spelled in bools.
#[allow(clippy::struct_excessive_bools)]
pub struct Player {
    generator: Generator,
    /// The pass playing and the next, drawn a pass ahead (with the wander on).
    passes: [Pass; 2],
    current: usize,
    wandering: bool,
    next_drawn: bool,
    seq: Sequencer,
    synth: Synth,
    state: State,
    /// Whether a loop played before this one began loading.
    was_playing: bool,
    rate: f64,
    /// How far each pass wanders (`Engine.driftAmount`), 0 for the loop as
    /// composed.
    wander: Biased<{ AMOUNT.to_bits() }>,
    /// The host's salt on the wander's stream (queue item 92,
    /// `set_wander_salt`): 0, every code's own wander.
    salt: u32,
    /// When the loop asked for should take its first step, on the core's
    /// clock (0: as soon as it is made); and, stopped, when to go on.
    start_at: f64,
    resume_at: Option<f64>,
    /// A tempo asked while the loop is being made: it starts at that.
    tempo: Option<f64>,
    /// The changes waiting for their boundaries (queue item 86), one for
    /// each kind of moment but `Now`, and how many have been asked.
    pending: [Maybe<Change>; SLOTS],
    asked: u32,
    /// How the loop being made takes over from the one that played, and
    /// how it was asked for.
    handover: Handover,
    asked_by: Asked,
    /// The loop playing's beat, in steps (4, or 6 in 6/8).
    beat: Biased<{ 4.0_f64.to_bits() }>,
    /// The full mix or lite's, as started (`attach`).
    full: bool,
    /// The work the voices asked for, a pass at a time (queue item 87,
    /// `units`): the budget's `held` when the pass being decided began, and
    /// its first step's time; and the mean cost units the voices held over
    /// the last whole pass decided (negative: none yet).
    pass_held: f64,
    pass_from: f64,
    units: Biased<{ (-1.0_f64).to_bits() }>,
    /// Whether the loop playing began after a lead-in (`START`, a load with
    /// nothing playing): its first pass, for `pass_frames`, takes it in.
    lead_in: bool,
    /// The loop's setups before a re-roll (`revise`), to send again only
    /// those that differ, as the app's engine does.
    kept: Maybe<Load>,
    /// How long the track plays (queue item 94a, `set_track`), the step from
    /// the load it counts from, and whether its end was told.
    track: Track,
    count_from: u64,
    ended: bool,
    /// Whether the load being made is a loop resumed on a new core
    /// (`load_resumed`): it doesn't wait out its lateFloor a second time.
    resumed: bool,
    /// The host's picks (queue item 95, `set_picks`): what every loop the
    /// core makes itself asks, and the number the host gave them.
    picks: Request,
    picks_tag: u32,
    /// The seed of the core's own loops' seeds (`set_draws`; 0, none of its
    /// own), and how many it has drawn from it.
    draws: u32,
    drawn: u32,
    /// Whether the loop being made is one of the core's own, and the picks'
    /// number it took (told as `Kind::Own` when it starts).
    own: bool,
    own_tag: u32,
    /// What the player tells the host (`event`).
    events: Events,
}

impl Player {
    /// A player with nothing loaded, the wander on at the app's default.
    /// `const`, so a host can keep one in a static.
    // Built in a `const` for a host's static player, never on a stack at run
    // time (as `Core::new`).
    #[allow(clippy::large_stack_arrays)]
    #[must_use]
    pub const fn new() -> Self {
        Player {
            generator: Generator::new(),
            passes: [Pass::new(), Pass::new()],
            current: 0,
            wandering: false,
            next_drawn: false,
            seq: Sequencer::new(),
            synth: Synth::new(),
            state: State::Idle,
            was_playing: false,
            rate: 0.0,
            wander: Biased::DEFAULT,
            salt: 0,
            start_at: 0.0,
            resume_at: None,
            tempo: None,
            pending: [Maybe::NONE; SLOTS],
            asked: 0,
            handover: Handover::RingOut,
            asked_by: Asked::Load,
            beat: Biased::DEFAULT,
            full: false,
            pass_held: 0.0,
            pass_from: 0.0,
            units: Biased::DEFAULT,
            lead_in: false,
            kept: Maybe::NONE,
            track: Track::Always,
            count_from: 0,
            ended: false,
            resumed: false,
            picks: Request::new(),
            picks_tag: 0,
            draws: 0,
            drawn: 0,
            own: false,
            own_tag: 0,
            events: Events::new(),
        }
    }

    /// Start `core` for the player, as the web host starts one for the page:
    /// at `rate` Hz, with the full mix or lite's (`full`), rendering
    /// `channels` channels (1 mono, 2 stereo); then the player's own first
    /// calls (`attach`).
    ///
    /// # Errors
    ///
    /// `Error::Layout` for any layout but mono or stereo, `Error::Rate` for
    /// a rate outside `MIN_RATE` to `MAX_RATE`; the core is left as it was.
    pub fn start(&mut self, core: &mut Core, rate: f64, channels: u32, full: bool) -> Result<(), Error> {
        if channels != 1 && channels != 2 {
            return Err(Error::Layout(channels));
        }
        if !(MIN_RATE..=MAX_RATE).contains(&rate) {
            return Err(Error::Rate);
        }
        core.init(rate);
        core.quality(full);
        if channels == 2 {
            core.set_layout(2);
        }
        self.attach(core, rate, full);
        Ok(())
    }

    /// The player's start on a core just started at `rate` (a host that
    /// starts its core itself): everything let go, and the synth's first
    /// calls (its LFOs' start, the master gain). It works out the sung
    /// voices' references for the rate here, about a millisecond: call it
    /// before the audio runs.
    pub fn attach<H: Host>(&mut self, host: &mut H, rate: f64, full: bool) {
        self.rate = rate;
        self.seq.reset();
        self.state = State::Idle;
        self.was_playing = false;
        self.wandering = false;
        self.next_drawn = false;
        self.pending = [Maybe::NONE; SLOTS];
        self.handover = Handover::RingOut;
        self.asked_by = Asked::Load;
        self.full = full;
        self.units.set(-1.0);
        self.kept = Maybe::NONE;
        self.picks = Request::new();
        self.picks_tag = 0;
        self.draws = 0;
        self.drawn = 0;
        self.own = false;
        self.events.clear();
        let now = self.now(host);
        self.synth.start(host, rate, !full, now);
    }

    /// The core's clock, in seconds: the start of the block it renders next.
    #[allow(clippy::cast_precision_loss)] // frames, far below 2^53
    fn now<H: Host>(&self, host: &H) -> f64 {
        host.frame() as f64 / self.rate
    }

    /// Ask for the loop of `seed`, with what `request` asks: it is made a
    /// stage a block from the next block on, and starts when made (or at
    /// once, with `ready`). A loop playing stops being decided now; what it
    /// has handed over plays out.
    pub fn load(&mut self, seed: u32, request: &Request) {
        self.load_at(seed, request, 0.0);
    }

    /// `load`, the loop's first step at `at` seconds on the core's clock if
    /// it is made by then (a host keeping its own clock in step with the
    /// player's: the app's page, drawing the steps); as soon as it is made
    /// if not, or for `at` 0. Any change waiting (`change`) is dropped: a
    /// load starts afresh.
    pub fn load_at(&mut self, seed: u32, request: &Request, at: f64) {
        self.pending = [Maybe::NONE; SLOTS];
        self.handover = Handover::RingOut;
        self.asked_by = Asked::Load;
        self.was_playing = self.state == State::Playing || self.was_playing;
        self.generator.begin_with(seed, request);
        self.state = State::Loading;
        self.start_at = if at.is_finite() { at } else { 0.0 };
        self.resume_at = None;
        self.tempo = None;
        self.resumed = false;
        self.own = false;
    }

    /// `load_at` for a loop that was already playing on another core (queue
    /// item 94b: the app's automatic switch to "lighter on processor", which
    /// starts a new core at the other quality and the loop again from its
    /// top, as `Engine.load(spec, { resume: true })` does): the same, but its
    /// lateFloor (the kit and the bass sitting out after a load) is not waited
    /// out a second time.
    pub fn load_resumed(&mut self, seed: u32, request: &Request, at: f64) {
        self.load_at(seed, request, at);
        self.resumed = true;
    }

    /// Change to the loop of `seed`, with what `request` asks, at a musical
    /// moment of the loop playing (queue item 86; `change`'s module says
    /// how): `Moment::Now` as soon as it is made, else on the next beat,
    /// bar line, phrase or pass it can make (`lead` away at least). The old
    /// loop hands over by `handover`. No reset, no second core.
    ///
    /// A change is an event: several may wait, one for each kind of moment,
    /// a later one for the same kind (or for the same boundary) in place of
    /// the earlier; each starts its loop at its own boundary, the ones still
    /// waiting then worked out against the loop that took over. With nothing
    /// playing it is a load; while a loop is being made, `Now` makes this
    /// one in its place (at the same moment), and the others wait for it to
    /// play.
    pub fn change(&mut self, seed: u32, request: &Request, moment: Moment, handover: Handover) {
        self.change_with_track(seed, request, moment, handover, None);
    }

    /// `change`, and the track its loop plays once it starts (`set_track`,
    /// counted from its first step), or none to keep the track as it is
    /// (queue item 95). The app arms the loop that follows a track's end
    /// this way, with that loop's own length, so a page asleep at the
    /// boundary leaves the core playing what it would have set.
    pub fn change_with_track(&mut self, seed: u32, request: &Request, moment: Moment, handover: Handover, track: Option<Track>) {
        let change = Change {
            seed,
            request: *request,
            handover,
            asked: 0,
            target: None,
            own: false,
            sets_track: track.is_some(),
            track: track.unwrap_or(Track::Always),
        };
        self.ask(change, moment);
    }

    /// Ask for `change` at `moment` (`change`'s doc says how); a change of
    /// the core's own (`next`) draws its seed and takes the picks here when
    /// it is for now, and when it starts when it waits.
    fn ask(&mut self, mut change: Change, moment: Moment) {
        let making = self.state == State::Loading || self.state == State::Wandering;
        let live = self.state == State::Playing || making;
        if change.own && (!live || moment.slot().is_none()) {
            change.seed = self.draw_seed();
            change.request = self.picks;
        }
        let (seed, handover) = (change.seed, change.handover);
        match moment.slot() {
            _ if !live => {
                self.load_at(seed, &change.request, 0.0);
                self.handover = handover;
                self.asked_by = Asked::Now;
            }
            None if making => {
                self.generator.begin_with(seed, &change.request);
                self.state = State::Loading;
                self.resumed = false;
                self.handover = handover;
                if self.asked_by == Asked::Load {
                    self.asked_by = Asked::Now;
                }
            }
            None => {
                let pending = self.pending;
                self.load_at(seed, &change.request, 0.0);
                self.pending = pending;
                self.handover = handover;
                self.asked_by = Asked::Now;
            }
            Some(slot) => {
                self.asked = self.asked.wrapping_add(1);
                change.asked = self.asked;
                if let Some(p) = self.pending.get_mut(slot) {
                    *p = Maybe::some(change);
                }
                return;
            }
        }
        // A change now: it is the loop being made.
        self.own = change.own;
        self.own_tag = self.picks_tag;
        if change.sets_track {
            self.track = change.track;
            self.ended = false;
        }
    }

    /// Change to a new loop of the core's own (queue item 95; the app's
    /// Next, at the end of its history): its seed the next drawn from the
    /// host's (`set_draws`), with the host's picks (`set_picks`), at
    /// `moment`, handing over by `handover`; it plays on (its track
    /// `Track::Always`), as a new loop does in the app until its length
    /// slider is moved. A change that waits takes its seed and the picks
    /// when it starts, so a pick made meanwhile is heard in it. The host
    /// learns its seed from its `Kind::Loop` event, after a `Kind::Own` with
    /// the picks' number. False, and nothing asked, while the host has given
    /// no seed to draw from.
    pub fn next(&mut self, moment: Moment, handover: Handover) -> bool {
        if self.draws == 0 {
            return false;
        }
        self.ask(Self::own_change(handover), moment);
        true
    }

    /// A change for a loop of the core's own, its seed and request to be
    /// drawn when it is made.
    fn own_change(handover: Handover) -> Change {
        Change {
            seed: 0,
            request: Request::new(),
            handover,
            asked: 0,
            target: None,
            own: true,
            sets_track: true,
            track: Track::Always,
        }
    }

    /// Drop the change waiting for `moment` (queue item 95; nothing for
    /// `Moment::Now`, which never waits). The app drops the one armed for
    /// the track's end when nothing it holds comes next, so the core's own
    /// loop follows instead (`set_draws`).
    pub fn drop_change(&mut self, moment: Moment) {
        if let Some(p) = moment.slot().and_then(|slot| self.pending.get_mut(slot)) {
            *p = Maybe::NONE;
        }
    }

    /// Hold `request` as the host's picks (queue item 95; the app's New
    /// loops panel), numbered `tag` by the host: every loop the core makes
    /// itself from now on asks it (`next`, and a track's end with nothing
    /// waiting for it). The loop playing never changes for it, nor does a
    /// change asked already with its own request; a re-roll of the loop
    /// playing keeps the request that loop was made with, and each pass
    /// wanders inside it. They hold until set again, across loads.
    pub fn set_picks(&mut self, request: &Request, tag: u32) {
        self.picks = *request;
        self.picks_tag = tag;
    }

    /// The host's picks and their number (`set_picks`).
    #[must_use]
    pub fn picks(&self) -> (&Request, u32) {
        (&self.picks, self.picks_tag)
    }

    /// Give the core a seed to draw its own loops' seeds from (queue item 95;
    /// the app draws one at random each time it starts a core): the loops
    /// it makes itself are then a pure function of this seed and the picks
    /// (promise 3). 0 takes the core's own loops away: a track's end with
    /// nothing waiting for it plays on, as before.
    pub fn set_draws(&mut self, seed: u32) {
        self.draws = seed;
        self.drawn = 0;
    }

    /// The next seed of the core's own, from the host's (`set_draws`): the
    /// generator's stream, keyed by the seed and the count, so each is its
    /// own however the ones before were spent.
    fn draw_seed(&mut self) -> u32 {
        let mut r = Rng::new(self.draws ^ self.drawn.wrapping_mul(0x9e37_79b9));
        self.drawn = self.drawn.wrapping_add(1);
        r.seed32()
    }

    /// The loop playing, made again with `request` and kept in its place
    /// (queue item 94a; the app's re-roll, `loadSpec(next, { keepPosition:
    /// true })`): the seed's loop with a layer's new seed, say
    /// (`Request::with_layer_seed`). It is made a stage a block as a load is
    /// and takes over at the next step the old loop would have decided,
    /// going on from that step and pass; the setups the loop holds (the
    /// organ, the wash, the arc and the rest) are sent again only where they
    /// differ, and the lateFloor wait it was in stays. The tempo stays the
    /// one playing, unless the request edits it. The changes waiting stay.
    /// With nothing playing it is a change now.
    pub fn revise(&mut self, seed: u32, request: &Request) {
        if self.state != State::Playing {
            self.change(seed, request, Moment::Now, Handover::RingOut);
            return;
        }
        self.kept = Maybe::some(*self.generator.settings());
        let pending = self.pending;
        let tempo = self.seq.bpm;
        self.load_at(seed, request, 0.0);
        self.pending = pending;
        self.asked_by = Asked::InPlace;
        if request.edits().bpm().is_none() {
            self.tempo = Some(tempo);
        }
    }

    /// How long the track plays (queue item 94a; the app's length slider):
    /// whole passes or bars, counted from the step heard now (the pass
    /// playing as the first), or from a new loop's first step. It holds
    /// across loads until set again. When it has run, the player tells the
    /// host (`Kind::End`) and starts the change waiting for it
    /// (`Moment::End`), if one does.
    pub fn set_track<H: Host>(&mut self, host: &H, track: Track) {
        self.track = track;
        self.ended = false;
        let now = self.now(host);
        let sd = self.seq.step_dur();
        let ahead = if self.state == State::Playing && sd.is_finite() && sd > 0.0 {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // a few steps
            let n = js::round((self.seq.next_step_time - now) / sd).max(0.0) as u64;
            n
        } else {
            0
        };
        self.count_from = self.seq.abs_step.saturating_sub(ahead);
        if let Some(c) = self.pending.get_mut(END_SLOT).and_then(Maybe::as_mut) {
            c.target = None;
        }
    }

    /// The track's length (`set_track`).
    #[must_use]
    pub fn track(&self) -> Track {
        self.track
    }

    /// The step from the loop's load the track ends at, as it stands; none
    /// while it plays on (or nothing plays).
    #[must_use]
    pub fn track_end(&self) -> Option<u64> {
        if self.state != State::Playing {
            return None;
        }
        self.track.end_step(self.generator.spec(), self.count_from)
    }

    /// The tone (warmth, space and wobble, 0 to 1 each), as the Sound panel
    /// sets it: it glides there from now, and the loop reports it
    /// (`describe`). A new loop plays its own.
    pub fn set_tone<H: Host>(&mut self, host: &mut H, tone: [f64; 3]) {
        if !tone.iter().all(|t| t.is_finite()) {
            return;
        }
        let tone = tone.map(|t| t.clamp(0.0, 1.0));
        let now = self.now(host);
        self.synth.set_tone(host, tone, now);
        if self.state == State::Playing || (self.state == State::Idle && self.generator.stage() == crate::generator::Stage::Done) {
            self.generator.set_tone(tone);
        }
    }

    /// The oldest event waiting for the host (`events`' module says what
    /// they are), taken.
    pub fn event(&mut self) -> Option<Event> {
        self.events.pop()
    }

    /// Events lost for want of room since the player started (0 for a host
    /// that takes them after each render).
    #[must_use]
    pub fn events_lost(&self) -> u32 {
        self.events.lost()
    }

    /// How long before its boundary a change begins making its loop, in
    /// seconds: the lookahead, and `LOAD_BLOCKS` blocks at the core's rate.
    #[must_use]
    #[allow(clippy::cast_precision_loss)] // a few thousand frames
    pub fn lead(&self) -> f64 {
        LOOKAHEAD + (LOAD_BLOCKS * crate::QUANTUM) as f64 / self.rate
    }

    /// When the next change waiting starts its loop, in seconds on the
    /// core's clock, as the loop playing stands (its tempo may still move
    /// it); none if no change waits or it is not yet worked out (it is,
    /// before the next block).
    #[must_use]
    pub fn next_change(&self) -> Option<f64> {
        self.soonest().map(|(_, t)| self.time_of(t))
    }

    /// Stop deciding (the app's Pause): the loop stays loaded, a load under
    /// way is dropped, and what was handed over plays out (a host that wants
    /// silence at once cancels it, `Core::cancel`). The changes waiting are
    /// dropped.
    pub fn stop(&mut self) {
        self.pending = [Maybe::NONE; SLOTS];
        if self.state == State::Loading || self.state == State::Wandering {
            self.state = State::Idle;
            self.was_playing = false;
        } else if self.state == State::Playing {
            self.state = State::Idle;
        }
        self.resume_at = None;
    }

    /// Go on with the loop stopped (the app's Play after Pause), from where
    /// it stood, its next step at `at` seconds on the core's clock (or as
    /// soon as the next block, if that has passed).
    pub fn resume(&mut self, at: f64) {
        if self.state == State::Idle && self.generator.stage() == crate::generator::Stage::Done && self.seq.bpm > 0.0 {
            self.resume_at = Some(if at.is_finite() { at } else { 0.0 });
        }
    }

    /// Make what is left of the loop asked for, now, and start it (its first
    /// step `START` after the block the core renders next): for a host with
    /// no deadline to keep.
    pub fn ready<H: Host>(&mut self, host: &mut H) {
        while self.state == State::Loading || self.state == State::Wandering {
            self.stage(host);
        }
    }

    /// Where the player is.
    #[must_use]
    pub fn state(&self) -> State {
        self.state
    }

    /// The generator: the loop playing (its spec, its render, what the
    /// engine decided at load).
    #[must_use]
    pub fn generator(&self) -> &Generator {
        &self.generator
    }

    /// Force a choice in every loop made from now on (queue item 83's
    /// clips): `Generator::force`. The sketch build only.
    ///
    /// # Errors
    /// The choice or the option is not in the data.
    #[cfg(feature = "sketch")]
    pub fn force_choice(&mut self, id: &str, option: Option<&str>) -> Result<(), &'static str> {
        self.generator.force(id, option)
    }

    /// Where the sequencer is: the loop's step, its pass, and when the next
    /// step falls.
    #[must_use]
    pub fn position(&self) -> (u64, u32, f64) {
        (self.seq.step, self.seq.loop_count, self.seq.next_step_time)
    }

    /// The piece of the loop playing (queue item 88: a length asked), or none
    /// for a loop with no length; `position`'s pass is its section.
    #[must_use]
    pub fn piece(&self) -> Option<Piece> {
        Piece::of(self.generator.spec())
    }

    /// The pass playing as it wanders (a piece's section), or none for the
    /// loop as composed: what `Generator::section` draws for its number.
    #[must_use]
    pub fn pass(&self) -> Option<&Pass> {
        if self.state != State::Playing || !self.wandering {
            return None;
        }
        self.passes.get(self.current)
    }

    /// The steps decided since the loop playing was loaded (`absStep`).
    #[must_use]
    pub fn steps(&self) -> u64 {
        self.seq.abs_step
    }

    /// Notes the budget could not hold for want of room in its list (0).
    #[must_use]
    pub fn budget_full(&self) -> u32 {
        self.synth.budget.full()
    }

    /// One stage of a load: the generator's next, then the first pass, then
    /// the loop's start.
    fn stage<H: Host>(&mut self, host: &mut H) {
        match self.state {
            State::Loading => {
                if self.generator.step() {
                    self.state = State::Wandering;
                }
            }
            State::Wandering => {
                // The first pass, at the pass count the engine still holds
                // (`_wander(this.loopCount)` before the load resets it); a
                // change's loop at its own first, so it plays as it does
                // alone (queue item 86); a piece's at its first section, so
                // a section is the same however it is reached (queue item
                // 88).
                self.current = 0;
                self.wandering = self.wander.get() > 0.0;
                if self.wandering {
                    let piece = self.generator.spec().length.is_some();
                    let n = if (self.asked_by == Asked::Load && !piece) || self.asked_by == Asked::InPlace { self.seq.loop_count } else { 0 };
                    self.draw(0, n);
                }
                self.begin(host);
            }
            State::Idle | State::Playing => {}
        }
    }

    /// Draw pass `number` into `passes[slot]`, at the wander's amount, on
    /// its stream salted as the host asked.
    fn draw(&mut self, slot: usize, number: u32) {
        let g = &self.generator;
        let kick = g.settings().kick.map(|k| k.kind);
        if let Some(p) = self.passes.get_mut(slot) {
            p.draw_salted(&Cx::new(g.spec()), g.render(), kick, number, self.wander.get(), self.salt);
        }
    }


    /// The loop made starts: what the engine does loading it, at the core's
    /// clock; its first step `START` on (or, after a loop that was playing,
    /// where that one's next step would have fallen).
    fn begin<H: Host>(&mut self, host: &mut H) {
        if self.asked_by == Asked::InPlace
            && let Some(was) = self.kept.get()
        {
            self.begin_in_place(host, &was);
            return;
        }
        self.kept = Maybe::NONE;
        // A loop resumed on a new core has waited out its lateFloor already.
        if self.resumed {
            self.generator.keep_late(None);
            self.resumed = false;
        }
        let now = self.now(host);
        let at = self.seq.next_step_time;
        let sd = self.seq.step_dur();
        let start = if self.start_at > now {
            self.start_at
        } else if self.was_playing && at > now {
            at
        } else if self.was_playing && at > 0.0 && sd.is_finite() && sd > 0.0 {
            // The loop took longer to make than the old one had decided
            // ahead (a tuned kick's fit can take 90-odd blocks, past the
            // lookahead), so its next step has passed: the first of its
            // steps still to come, so the new loop starts on its grid all
            // the same (queue item 86b).
            at + (js::floor((now - at) / sd) + 1.0) * sd
        } else {
            now + START
        };
        self.lead_in = start == now + START;
        // A loop handing over by a crossfade (queue item 86): its organ lets
        // go over the fade, and everything it handed over falls from the new
        // loop's first step, before the new loop hands over a note.
        let fade = if self.was_playing { self.handover.fade() } else { None };
        let g = &self.generator;
        let lp = Loop { spec: g.spec(), base: g.render(), pass: None, settings: g.settings() };
        let at_moment = self.asked_by == Asked::Moment;
        if at_moment || fade.is_some() {
            let when = if at_moment { start } else { now };
            let fall = fade.map_or(ORGAN_FALL, |x| js::max(ORGAN_FALL, x));
            self.seq.load_at(host, &mut self.synth, &lp, now, when, fall);
        } else {
            self.seq.load(host, &mut self.synth, &lp, now);
        }
        self.beat.set(if Cx::new(g.spec()).metre == Metre::SixEight { 6.0 } else { 4.0 });
        if let Some(bpm) = self.tempo.take() {
            self.seq.bpm = bpm;
            let when = if at_moment { start } else { now };
            Synth::set_echo(host, (60.0 / bpm) * self.generator.settings().echo_beats.get(), when);
        }
        // The old loop's notes hold the voice budget no longer: the new one
        // asks as it would alone (the fading notes' cost is the change's,
        // measured in `dlrender --changes`).
        if let Some(x) = fade {
            host.fade_all(start, start + x);
            self.synth.budget.end_by(start);
        }
        self.seq.next_step_time = start;
        // A new loop's work is its own: nothing measured yet.
        self.pass_held = self.synth.budget.held();
        self.pass_from = start;
        self.units.set(-1.0);
        let lp = loop_of(&self.generator, &self.passes, self.wandering.then_some(self.current));
        self.seq.pass_begins(&lp);
        self.next_drawn = false;
        self.was_playing = false;
        self.asked_by = Asked::Load;
        self.handover = Handover::RingOut;
        // The changes still waiting are worked out against this loop.
        for c in self.pending.iter_mut().filter_map(Maybe::as_mut) {
            c.target = None;
        }
        // The track counts from the new loop's first step.
        self.count_from = 0;
        self.ended = false;
        if self.own {
            self.own = false;
            self.events.push(Event { time: start, kind: Kind::Own, number: u64::from(self.own_tag), step: 0 });
        }
        self.events.push(Event { time: start, kind: Kind::Loop, number: u64::from(self.generator.spec().seed), step: 0 });
        self.state = State::Playing;
    }

    /// `begin` for the loop playing made again in its place (`revise`): it
    /// takes the next step the old one would have decided, at that step's
    /// time (or the first of its steps still to come, the steps passed while
    /// it was made counted on), its setups sent again only where they differ
    /// from `was`.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // a few steps
    fn begin_in_place<H: Host>(&mut self, host: &mut H, was: &Load) {
        let now = self.now(host);
        if let Some(bpm) = self.tempo.take() {
            self.seq.bpm = bpm;
        } else {
            self.seq.bpm = self.generator.spec().bpm;
        }
        let sd = self.seq.step_dur();
        let mut start = self.seq.next_step_time;
        let mut skip = 0;
        if !(start > now) && sd.is_finite() && sd > 0.0 {
            let k = js::floor((now - start) / sd) + 1.0;
            skip = k.clamp(0.0, 4096.0) as u32;
            start += f64::from(skip) * sd;
        }
        self.generator.keep_late(was.late);
        let g = &self.generator;
        let lp = Loop { spec: g.spec(), base: g.render(), pass: None, settings: g.settings() };
        self.seq.revise(host, &mut self.synth, &lp, now, was);
        let count = self.seq.loop_count;
        for _ in 0..skip {
            self.seq.advance(&lp);
        }
        if self.wandering && self.seq.loop_count != count {
            self.draw(self.current, self.seq.loop_count);
        }
        self.seq.next_step_time = start;
        self.lead_in = false;
        self.pass_held = self.synth.budget.held();
        self.pass_from = start;
        self.units.set(-1.0);
        let lp = loop_of(&self.generator, &self.passes, self.wandering.then_some(self.current));
        self.seq.pass_begins(&lp);
        self.next_drawn = false;
        self.was_playing = false;
        self.asked_by = Asked::Load;
        self.handover = Handover::RingOut;
        self.kept = Maybe::NONE;
        for c in self.pending.iter_mut().filter_map(Maybe::as_mut) {
            c.target = None;
        }
        self.events.push(Event { time: start, kind: Kind::Revised, number: u64::from(self.generator.spec().seed), step: self.seq.abs_step });
        self.state = State::Playing;
    }

    /// When step `target` (from the load) of the loop playing falls, at its
    /// tempo, on the core's clock.
    #[allow(clippy::cast_precision_loss)] // steps, far below 2^53
    fn time_of(&self, target: u64) -> f64 {
        self.seq.next_step_time + target.saturating_sub(self.seq.abs_step) as f64 * self.seq.step_dur()
    }

    /// The change waiting that starts soonest, the latest asked among those
    /// for the same boundary: its slot and its step.
    fn soonest(&self) -> Option<(usize, u64)> {
        let mut best: Option<(usize, u64, u32)> = None;
        for (i, c) in self.pending.iter().enumerate() {
            let Some(c) = c.as_ref() else { continue };
            let Some(t) = c.target else { continue };
            let better = match best {
                None => true,
                Some((_, bt, ba)) => t < bt || (t == bt && c.asked.wrapping_sub(ba) < u32::MAX / 2),
            };
            if better {
                best = Some((i, t, c.asked));
            }
        }
        best.map(|(i, t, _)| (i, t))
    }

    /// The step (from the load) the change waiting in `slot` starts on: the
    /// first of its moment's boundaries, from the next step to decide, that
    /// falls `lead` from `now` or later.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // a grid of a few hundred steps
    fn resolve(&self, slot: usize, now: f64) -> Option<u64> {
        let moment = Moment::ALL.get(slot + 1).copied()?;
        let g = &self.generator;
        if moment == Moment::End {
            // The first step the track ends on that is the lead away.
            let sd = self.seq.step_dur();
            if !(sd.is_finite() && sd > 0.0) {
                return None;
            }
            let need = js::ceil((now + self.lead() - self.seq.next_step_time) / sd).max(0.0);
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // steps, far below 2^53
            let first = self.seq.abs_step + need as u64;
            return self.track.next_end(g.spec(), self.count_from, first);
        }
        let spb = g.spec().steps_per_bar;
        let total = g.spec().bars * spb;
        let u = change::unit(moment, self.beat.get(), spb, total)? as u64;
        let total = total as u64;
        if total == 0 {
            return None;
        }
        let lead = self.lead();
        let abs = self.seq.abs_step;
        // A piece with a section cut short (queue item 88): its boundaries
        // walked step by step, section by section, from the next step.
        if let Some(p) = Piece::of(g.spec()).filter(Piece::cut) {
            let (mut s, mut n, mut t) = (self.seq.step, u64::from(self.seq.loop_count), abs);
            let steps = |n: u64| (p.bars_of(n) * spb) as u64;
            for _ in 0..1 << 16 {
                if s % u == 0 && self.time_of(t) - now >= lead {
                    return Some(t);
                }
                t += 1;
                s += 1;
                if s >= steps(n) {
                    s = 0;
                    n += 1;
                }
            }
            return Some(t);
        }
        let into = |t: u64| (self.seq.step + (t - abs)) % total;
        let mut t = abs + (u - into(abs) % u) % u;
        let mut guard = 0;
        while self.time_of(t) - now < lead && guard < 4096 {
            t += 1;
            t += (u - into(t) % u) % u;
            guard += 1;
        }
        Some(t)
    }

    /// Before a block, with a loop playing: the changes waiting worked out,
    /// and the soonest begun if its boundary is `lead` away or nearer. The
    /// loop playing decides every step before the boundary (the old loop's
    /// last), and the next loop is made from the next block on, to start
    /// there. Returns the steps decided, if it began one.
    fn arm<H: Host>(&mut self, host: &mut H, now: f64) -> Option<usize> {
        // A track that ends with nothing waiting for it hands over to a loop
        // of the core's own, where the host gave a seed to draw from (queue
        // item 95), so the page may sleep through it.
        if self.draws != 0
            && self.track != Track::Always
            && self.pending.get(END_SLOT).is_some_and(|p| p.get().is_none())
        {
            self.asked = self.asked.wrapping_add(1);
            let mut c = Self::own_change(Handover::Crossfade(END_FADE));
            c.asked = self.asked;
            if let Some(p) = self.pending.get_mut(END_SLOT) {
                *p = Maybe::some(c);
            }
        }
        for slot in 0..self.pending.len() {
            if self.pending[slot].get().is_some_and(|c| c.target.is_none()) {
                let t = self.resolve(slot, now);
                match t {
                    Some(t) => {
                        if let Some(c) = self.pending[slot].as_mut() {
                            c.target = Some(t);
                        }
                    }
                    // A change for the track's end waits while it has none.
                    None if slot == END_SLOT => {}
                    None => self.pending[slot] = Maybe::NONE,
                }
            }
        }
        let (slot, target) = self.soonest()?;
        if self.time_of(target) - now > self.lead() {
            return None;
        }
        let change = self.pending[slot].get()?;
        let mut n = 0;
        while self.seq.abs_step < target && n < 4096 {
            n += 1;
            self.step_once(host, now, target);
        }
        for c in &mut self.pending {
            if c.get().is_some_and(|c| c.target == Some(target)) {
                *c = Maybe::NONE;
            }
        }
        let start = self.seq.next_step_time;
        // At the track's end the old loop hands over as the app's does: its
        // notes ring out at a pass's end, and fall over the crossfade asked
        // at a phrase line inside one.
        let mut handover = change.handover;
        if slot == END_SLOT {
            if !self.ended {
                self.ended = true;
                self.events.push(Event { time: start, kind: Kind::End, number: 0, step: target });
            }
            if track::round_steps(self.generator.spec()).is_some_and(|r| target.is_multiple_of(r)) {
                handover = Handover::RingOut;
            }
        }
        let pending = self.pending;
        // A loop of the core's own takes its seed and the picks now.
        let (seed, request) = if change.own { (self.draw_seed(), self.picks) } else { (change.seed, change.request) };
        self.load_at(seed, &request, start);
        self.pending = pending;
        self.handover = handover;
        self.asked_by = Asked::Moment;
        self.own = change.own;
        self.own_tag = self.picks_tag;
        if change.sets_track {
            self.track = change.track;
        }
        Some(n)
    }

    /// Decide the next step and hand its notes over; at the top of a pass,
    /// the next pass (unless the step it reaches is `last`, where the loop
    /// playing hands over).
    fn step_once<H: Host>(&mut self, host: &mut H, now: f64, last: u64) {
        self.tell();
        let lp = loop_of(&self.generator, &self.passes, self.wandering.then_some(self.current));
        self.seq.schedule_step(host, &mut self.synth, &lp, now);
        if self.seq.advance(&lp) {
            self.pass_measured();
            if self.seq.abs_step != last {
                self.next_pass();
            }
        }
    }

    /// The events of the step about to be decided, at its time: a pass, a
    /// piece's section, a phrase line, the step, and the track's end.
    fn tell(&mut self) {
        let spec = self.generator.spec();
        let (time, step, abs) = (self.seq.next_step_time, self.seq.step, self.seq.abs_step);
        let ev = |kind, number| Event { time, kind, number, step: abs };
        let n = u64::from(self.seq.loop_count);
        if step == 0 {
            match Piece::of(spec) {
                Some(p) => {
                    if p.end.map_or(n == 0, |(sections, _)| n.is_multiple_of(sections)) {
                        let rounds = p.end.map_or(0, |(sections, _)| n / sections.max(1));
                        self.events.push(ev(Kind::Pass, rounds));
                    }
                    self.events.push(ev(Kind::Section, n));
                }
                None => self.events.push(ev(Kind::Pass, n)),
            }
        }
        if track::phrase_line(spec, abs) == abs {
            self.events.push(ev(Kind::Phrase, step));
        }
        self.events.push(ev(Kind::Step, step));
        if !self.ended && self.track.ends_at(spec, self.count_from, abs) {
            self.ended = true;
            self.events.push(ev(Kind::End, 0));
        }
    }

    /// A pass decided to its end: the mean cost units its voices held over
    /// it, for `units`, and the next pass measured from here.
    fn pass_measured(&mut self) {
        let held = self.synth.budget.held();
        let span = self.seq.next_step_time - self.pass_from;
        if span > 0.0 {
            self.units.set((held - self.pass_held) / span);
        }
        self.pass_held = held;
        self.pass_from = self.seq.next_step_time;
    }

    /// Before the block the core renders next: a stage of a load, if one is
    /// under way, or the next pass drawn ahead; then every step due within
    /// `LOOKAHEAD`, its notes handed over. `render` calls it before each
    /// block; a host that drives `Core::process` itself calls it before
    /// each. Returns what it did.
    pub fn decide<H: Host>(&mut self, host: &mut H) -> Work {
        if self.state == State::Loading || self.state == State::Wandering {
            self.stage(host);
            return Work::Load;
        }
        let now = self.now(host);
        if let Some(at) = self.resume_at.take() {
            self.seq.next_step_time = if at > now { at } else { now + START };
            self.state = State::Playing;
        }
        if self.state != State::Playing {
            return Work::None;
        }
        if let Some(n) = self.arm(host, now) {
            return Work::Steps(n);
        }
        // Never past a change's boundary: the step it starts on is the next
        // loop's.
        let last = self.soonest().map_or(u64::MAX, |(_, t)| t);
        let due = |seq: &Sequencer| seq.next_step_time < now + LOOKAHEAD && seq.abs_step < last;
        if self.wandering && !self.next_drawn && !due(&self.seq) {
            let n = self.seq.loop_count.wrapping_add(1);
            self.draw(1 - self.current, n);
            self.next_drawn = true;
            return Work::Pass;
        }
        // A step is a tenth of a second or so: this many in one block would
        // be a tempo past any the generator draws.
        let mut guard = 0;
        while due(&self.seq) && guard < 4096 {
            guard += 1;
            self.step_once(host, now, u64::MAX);
        }
        if guard == 0 { Work::None } else { Work::Steps(guard) }
    }

    /// The top of a new pass: the next one drawn (now, if it is not yet),
    /// and played.
    fn next_pass(&mut self) {
        if self.wander.get() > 0.0 {
            let next = 1 - self.current;
            if !self.wandering || !self.next_drawn {
                let n = self.seq.loop_count;
                self.draw(next, n);
            }
            self.current = next;
            self.wandering = true;
        } else {
            self.wandering = false;
        }
        self.next_drawn = false;
        let lp = loop_of(&self.generator, &self.passes, self.wandering.then_some(self.current));
        self.seq.pass_begins(&lp);
    }

    /// Render the next `out[0].len()` frames into `out` (one slice a channel
    /// of the layout), deciding before each block (`Core::render_with`).
    ///
    /// # Errors
    ///
    /// `Error::Layout` if `out` is not one slice for each channel of the
    /// core's layout, all the same length; nothing is rendered.
    pub fn render(&mut self, core: &mut Core, out: &mut [&mut [f32]]) -> Result<(), Error> {
        core.render_with(out, |core| {
            self.decide(core);
        })
    }

    /// How far each pass wanders from now on ("Let the loop wander", 0 to
    /// 2.4 in the app; 0 plays the loop as composed). Heard from the next
    /// pass.
    pub fn set_wander(&mut self, amount: f64) {
        let amount = if amount.is_finite() { amount.max(0.0) } else { 0.0 };
        if amount != self.wander.get() {
            self.wander.set(amount);
            self.next_drawn = false;
        }
    }

    /// The salt on the wander's stream from now on (queue item 92), heard
    /// from the next pass: a host that wants a loop to wander differently
    /// each time it is played (the app, across the wander slider's upper
    /// range: `UNREPEATABLE_FROM` in `js/engine.js`) sends a new one each
    /// play; 0, the default, is the wander every code has always had. The
    /// draws stay a function of what the host sends: the same seed, request
    /// and salt make the same samples.
    pub fn set_wander_salt(&mut self, salt: u32) {
        if salt != self.salt {
            self.salt = salt;
            self.next_drawn = false;
        }
    }

    /// The wander's salt (`set_wander_salt`).
    #[must_use]
    pub fn wander_salt(&self) -> u32 {
        self.salt
    }

    /// Mute a layer or bring it back, as the app's layer buttons do: its
    /// channel glides out or in at once, and its notes are left out (or
    /// played) from the next step decided. It holds, across loads, until
    /// brought back.
    pub fn set_mute<H: Host>(&mut self, host: &mut H, layer: Layer, muted: bool) {
        let now = self.now(host);
        if let Some(m) = self.seq.mutes.get_mut(layer as usize) {
            *m = muted;
        }
        self.synth.set_mute(host, channel_of(layer) as usize, muted, now);
    }

    /// The tempo, in beats a minute, as the app's slider sets it: from the
    /// next step decided, the echo's time following it. Asked while a loop
    /// is being made, the loop starts at it (a load starts at the loop's
    /// own).
    pub fn set_tempo<H: Host>(&mut self, host: &mut H, bpm: f64) {
        if !(bpm.is_finite() && bpm > 0.0) {
            return;
        }
        if self.state == State::Loading || self.state == State::Wandering {
            self.tempo = Some(bpm);
        } else if self.seq.bpm > 0.0 {
            let now = self.now(host);
            self.seq.bpm = bpm;
            Synth::set_echo(host, (60.0 / bpm) * self.generator.settings().echo_beats.get(), now);
        }
    }

    /// The listener's volume, 0 to 1 (the app's 0.85 until set), as the
    /// app's slider sets it.
    pub fn set_volume<H: Host>(&mut self, host: &mut H, volume: f64) {
        if !volume.is_finite() {
            return;
        }
        let now = self.now(host);
        self.synth.set_volume(host, volume.clamp(0.0, 1.0), now);
    }
}

/// The loop as the sequencer plays it: the generator's, with the pass in
/// `passes[slot]` wandering over it (none: the loop as composed).
fn loop_of<'a>(g: &'a Generator, passes: &'a [Pass; 2], slot: Option<usize>) -> Loop<'a> {
    Loop { spec: g.spec(), base: g.render(), pass: slot.and_then(|i| passes.get(i)), settings: g.settings() }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "tools")]
impl Player {
    /// The voice calls the budget was asked and refused, by layer, and
    /// those it let go (`Budget::counts`, `Budget::stolen`), for the
    /// measuring tools (queue item 96; the tools build only).
    #[must_use]
    pub fn budget_counts(&self) -> (budget::Counts, [u32; 5]) {
        (self.synth.budget.counts(), self.synth.budget.stolen())
    }
}
