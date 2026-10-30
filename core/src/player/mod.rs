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
//! asks for a loop (`load`: a seed, and a `Request`, the vibes for now) and
//! renders (`render`, any number of frames a call). Before each block the
//! core renders, the player decides every step due within `LOOKAHEAD` and
//! hands its notes over (`decide`, which a host driving `Core::process`
//! itself calls before each block). Wander, mutes, tempo and volume are
//! the app's controls, as it has them (`set_wander` and the rest).
//!
//! **Spread across blocks** (promise 1). A load is a few stages of work,
//! each run whole in a block of its own (`Generator::step`): the spec, the
//! plan, the render, what the engine decides at load, the first pass; then
//! the loop starts. So a load never spikes one block. A host with no
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
pub mod formant;
pub mod host;
pub mod sequencer;
pub mod synth;

use crate::generator::data::{self, VIBE_COUNT};
use crate::generator::plan::Cx;
use crate::generator::wander::{AMOUNT, Pass};
use crate::generator::{Generator, Layer};
use crate::{Core, Error, MAX_RATE, MIN_RATE};

use host::Host;
use sequencer::{Loop, Sequencer, channel_of};
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

/// What a host asks for, beside the seed (promise 11's request; queue item
/// 85 on grows it): for now, the vibes, as a listener picks them.
#[derive(Clone, Copy)]
pub struct Request {
    vibes: [bool; VIBE_COUNT],
}

impl Request {
    /// Nothing asked for: the loop the seed makes.
    #[must_use]
    pub const fn new() -> Self {
        Request { vibes: [false; VIBE_COUNT] }
    }

    /// The same, with the vibe numbered `index` (`generator::data::VIBES`).
    #[must_use]
    pub fn with(mut self, index: usize) -> Self {
        if let Some(v) = self.vibes.get_mut(index) {
            *v = true;
        }
        self
    }

    /// The same, with the vibe called `id` ("glacial", as `data/vibes.txt`
    /// names it); `None` for a vibe the core does not have.
    #[must_use]
    pub fn with_vibe(self, id: &str) -> Option<Self> {
        data::vibe_index(id).map(|i| self.with(i))
    }
}

impl Default for Request {
    fn default() -> Self {
        Self::new()
    }
}

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
        self.was_playing = self.state == State::Playing || self.was_playing;
        self.generator.begin_with(seed, &request.vibes);
        self.state = State::Loading;
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
                // (`_wander(this.loopCount)` before the load resets it).
                self.current = 0;
                self.wandering = self.wander > 0.0;
                if self.wandering {
                    let n = self.seq.loop_count;
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
        let g = &self.generator;
        let lp = Loop { spec: g.spec(), base: g.render(), pass: None, settings: g.settings() };
        self.seq.load(host, &mut self.synth, &lp, now);
        self.seq.next_step_time = if self.was_playing && at > now { at } else { now + START };
        let lp = loop_of(&self.generator, &self.passes, self.wandering.then_some(self.current));
        self.seq.pass_begins(&lp);
        self.next_drawn = false;
        self.was_playing = false;
        self.state = State::Playing;
    }

    /// Before the block the core renders next: a stage of a load, if one is
    /// under way, or the next pass drawn ahead; then every step due within
    /// `LOOKAHEAD`, its notes handed over. `render` calls it before each
    /// block; a host that drives `Core::process` itself calls it before
    /// each. Returns how many steps it decided.
    pub fn decide<H: Host>(&mut self, host: &mut H) -> usize {
        if self.state == State::Loading || self.state == State::Wandering {
            self.stage(host);
            return 0;
        }
        if self.state != State::Playing {
            return 0;
        }
        let now = self.now(host);
        let due = |seq: &Sequencer| seq.next_step_time < now + LOOKAHEAD;
        if self.wandering && !self.next_drawn && !due(&self.seq) {
            let n = self.seq.loop_count.wrapping_add(1);
            self.draw(1 - self.current, n);
            self.next_drawn = true;
            return 0;
        }
        // A step is a tenth of a second or so: this many in one block would
        // be a tempo past any the generator draws.
        let mut guard = 0;
        while due(&self.seq) && guard < 4096 {
            guard += 1;
            let lp = loop_of(&self.generator, &self.passes, self.wandering.then_some(self.current));
            self.seq.schedule_step(host, &mut self.synth, &lp, now);
            if self.seq.advance(&lp) {
                self.next_pass();
            }
        }
        guard
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
    /// played) from the next step decided.
    pub fn set_mute<H: Host>(&mut self, host: &mut H, layer: Layer, muted: bool) {
        let now = self.now(host);
        if let Some(m) = self.seq.mutes.get_mut(layer as usize) {
            *m = muted;
        }
        self.synth.set_mute(host, channel_of(layer) as usize, muted, now);
    }

    /// The tempo, in beats a minute, as the app's slider sets it: from the
    /// next step decided, the echo's time following it.
    pub fn set_tempo<H: Host>(&mut self, host: &mut H, bpm: f64) {
        if !(bpm.is_finite() && bpm > 0.0) || self.state != State::Playing {
            return;
        }
        let now = self.now(host);
        self.seq.bpm = bpm;
        Synth::set_echo(host, (60.0 / bpm) * self.generator.settings().echo_beats, now);
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
