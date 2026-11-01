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

pub mod budget;
pub mod change;
pub mod formant;
pub mod host;
pub mod sequencer;
pub mod synth;

use crate::generator::js;
use crate::generator::plan::Cx;
use crate::generator::spec::Metre;
use crate::generator::wander::{AMOUNT, Pass};
use crate::generator::{Generator, Layer};
use crate::{Core, Error, MAX_RATE, MIN_RATE};

use change::{Change, LOAD_BLOCKS};
pub use change::{CROSSFADE, Handover, Moment};
use host::Host;
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
/// level and echo glide in at the boundary rather than when it is made).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Asked {
    Load,
    Now,
    Moment,
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
    wander: f64,
    /// When the loop asked for should take its first step, on the core's
    /// clock (0: as soon as it is made); and, stopped, when to go on.
    start_at: f64,
    resume_at: Option<f64>,
    /// A tempo asked while the loop is being made: it starts at that.
    tempo: Option<f64>,
    /// The changes waiting for their boundaries (queue item 86), one for
    /// each kind of moment but `Now`, and how many have been asked.
    pending: [Option<Change>; 4],
    asked: u32,
    /// How the loop being made takes over from the one that played, and
    /// how it was asked for.
    handover: Handover,
    asked_by: Asked,
    /// The loop playing's beat, in steps (4, or 6 in 6/8).
    beat: f64,
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
            wander: AMOUNT,
            start_at: 0.0,
            resume_at: None,
            tempo: None,
            pending: [None; 4],
            asked: 0,
            handover: Handover::RingOut,
            asked_by: Asked::Load,
            beat: 4.0,
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
        self.pending = [None; 4];
        self.handover = Handover::RingOut;
        self.asked_by = Asked::Load;
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
        self.pending = [None; 4];
        self.handover = Handover::RingOut;
        self.asked_by = Asked::Load;
        self.was_playing = self.state == State::Playing || self.was_playing;
        self.generator.begin_with(seed, request);
        self.state = State::Loading;
        self.start_at = if at.is_finite() { at } else { 0.0 };
        self.resume_at = None;
        self.tempo = None;
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
        let making = self.state == State::Loading || self.state == State::Wandering;
        let live = self.state == State::Playing || making;
        match moment.slot() {
            _ if !live => {
                self.load_at(seed, request, 0.0);
                self.handover = handover;
                self.asked_by = Asked::Now;
            }
            None if making => {
                self.generator.begin_with(seed, request);
                self.state = State::Loading;
                self.handover = handover;
                if self.asked_by == Asked::Load {
                    self.asked_by = Asked::Now;
                }
            }
            None => {
                let pending = self.pending;
                self.load_at(seed, request, 0.0);
                self.pending = pending;
                self.handover = handover;
                self.asked_by = Asked::Now;
            }
            Some(slot) => {
                self.asked = self.asked.wrapping_add(1);
                if let Some(p) = self.pending.get_mut(slot) {
                    *p = Some(Change { seed, request: *request, handover, asked: self.asked, target: None });
                }
            }
        }
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
        self.pending = [None; 4];
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
                // alone (queue item 86).
                self.current = 0;
                self.wandering = self.wander > 0.0;
                if self.wandering {
                    let n = if self.asked_by == Asked::Load { self.seq.loop_count } else { 0 };
                    self.draw(0, n);
                }
                self.begin(host);
            }
            State::Idle | State::Playing => {}
        }
    }

    /// Draw pass `number` into `passes[slot]`, at the wander's amount.
    fn draw(&mut self, slot: usize, number: u32) {
        let g = &self.generator;
        let kick = g.settings().kick.map(|k| k.kind);
        if let Some(p) = self.passes.get_mut(slot) {
            p.draw_by(&Cx::new(g.spec()), g.render(), kick, number, self.wander);
        }
    }


    /// The loop made starts: what the engine does loading it, at the core's
    /// clock; its first step `START` on (or, after a loop that was playing,
    /// where that one's next step would have fallen).
    fn begin<H: Host>(&mut self, host: &mut H) {
        let now = self.now(host);
        let at = self.seq.next_step_time;
        let start = if self.start_at > now {
            self.start_at
        } else if self.was_playing && at > now {
            at
        } else {
            now + START
        };
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
        self.beat = if Cx::new(g.spec()).metre == Metre::SixEight { 6.0 } else { 4.0 };
        if let Some(bpm) = self.tempo.take() {
            self.seq.bpm = bpm;
            let when = if at_moment { start } else { now };
            Synth::set_echo(host, (60.0 / bpm) * self.generator.settings().echo_beats, when);
        }
        // The old loop's notes hold the voice budget no longer: the new one
        // asks as it would alone (the fading notes' cost is the change's,
        // measured in `dlrender --changes`).
        if let Some(x) = fade {
            host.fade_all(start, start + x);
            self.synth.budget.end_by(start);
        }
        self.seq.next_step_time = start;
        let lp = loop_of(&self.generator, &self.passes, self.wandering.then_some(self.current));
        self.seq.pass_begins(&lp);
        self.next_drawn = false;
        self.was_playing = false;
        self.asked_by = Asked::Load;
        self.handover = Handover::RingOut;
        // The changes still waiting are worked out against this loop.
        for c in self.pending.iter_mut().flatten() {
            c.target = None;
        }
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
            let Some(c) = c else { continue };
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
        let spb = g.spec().steps_per_bar;
        let total = g.spec().bars * spb;
        let u = change::unit(moment, self.beat, spb, total)? as u64;
        let total = total as u64;
        if total == 0 {
            return None;
        }
        let lead = self.lead();
        let abs = self.seq.abs_step;
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
        for slot in 0..self.pending.len() {
            if self.pending[slot].is_some_and(|c| c.target.is_none()) {
                let t = self.resolve(slot, now);
                match t {
                    Some(t) => {
                        if let Some(c) = self.pending[slot].as_mut() {
                            c.target = Some(t);
                        }
                    }
                    None => self.pending[slot] = None,
                }
            }
        }
        let (slot, target) = self.soonest()?;
        if self.time_of(target) - now > self.lead() {
            return None;
        }
        let change = self.pending[slot]?;
        let mut n = 0;
        while self.seq.abs_step < target && n < 4096 {
            n += 1;
            self.step_once(host, now, target);
        }
        for c in &mut self.pending {
            if c.is_some_and(|c| c.target == Some(target)) {
                *c = None;
            }
        }
        let start = self.seq.next_step_time;
        let pending = self.pending;
        self.load_at(change.seed, &change.request, start);
        self.pending = pending;
        self.handover = change.handover;
        self.asked_by = Asked::Moment;
        Some(n)
    }

    /// Decide the next step and hand its notes over; at the top of a pass,
    /// the next pass (unless the step it reaches is `last`, where the loop
    /// playing hands over).
    fn step_once<H: Host>(&mut self, host: &mut H, now: f64, last: u64) {
        let lp = loop_of(&self.generator, &self.passes, self.wandering.then_some(self.current));
        self.seq.schedule_step(host, &mut self.synth, &lp, now);
        if self.seq.advance(&lp) && self.seq.abs_step != last {
            self.next_pass();
        }
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
        if self.wander > 0.0 {
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
        if amount != self.wander {
            self.wander = amount;
            self.next_drawn = false;
        }
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
            Synth::set_echo(host, (60.0 / bpm) * self.generator.settings().echo_beats, now);
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
