//! A loop from its seed, on the player (queue item 82): `dlrender --seed`
//! and `--corpus`. No note list: the core's own generator makes the loop,
//! its own sequencer plays it, pass after pass, wandering as the app's
//! does. Each render prints one line: the loop, how it was rendered, and
//! the hashes of what reached the speakers, FNV-1a over the output's float
//! samples (the 32-bit one over their words that `tools/measure.mjs` and
//! `tools/baseline.mjs` keep, and the 64-bit one over their bytes the
//! golden lists use). `--calls` writes every call the player made to the
//! core, as `tools/player.mjs --calls` writes the page's, to diff the two.

// Frame counts and rates here are bounded by the render's own length; a
// whole number is told from the rest exactly, as JavaScript tells it.
#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss, clippy::float_cmp)]

use std::fmt::Write as _;

use dlcore::block::Event;
use dlcore::block::event::RETUNE;
use dlcore::generator::rng::Rng;
use dlcore::player::host::Host;
use dlcore::player::{Player, Request, Work};
use dlcore::{Core, EXTRA, Key, Tap};

use crate::notelist::{Fnv, wav, wav_channels};

/// How a loop is rendered.
#[derive(Clone)]
pub struct Way {
    pub seconds: f64,
    pub rate: f64,
    pub full: bool,
    pub layout: u32,
    pub wander: f64,
    pub vibes: Vec<String>,
    /// The request as text (`--request`; `Request::parse`), the vibes of
    /// `--vibes` added to it.
    pub request: Option<String>,
    /// The sketch build's levers (queue item 83, `--force`, `--set`,
    /// `--sketch`, `--pin`; `clip.rs`): choices forced, named constants
    /// moved, the sketch block's layer, settings pinned.
    pub clip: crate::clip::Clip,
    /// Add what the loop drew among its choices to the line (`choices=`).
    pub choices: bool,
}

impl Way {
    /// What it asks the player for: `--request`'s text, with `--vibes`'s.
    ///
    /// # Errors
    /// The request's line, where it cannot be met.
    pub fn request(&self) -> Result<Request, String> {
        let text = self.request.as_deref().unwrap_or("");
        let mut request = Request::parse(text).map_err(|why| format!("--request {text}: {why}"))?;
        for id in &self.vibes {
            request = request.with_vibe(id).ok_or_else(|| format!("no vibe '{id}'"))?;
        }
        Ok(request)
    }

    /// The line's description of it.
    fn describe(&self) -> String {
        let vibes = if self.vibes.is_empty() { String::new() } else { format!(" vibes={}", self.vibes.join(",")) };
        let request = self.request.as_ref().map_or(String::new(), |r| format!(" request={r}"));
        format!(
            "quality={} layout={} seconds={} rate={} wander={}{vibes}{request}",
            if self.full { "full" } else { "lite" },
            if self.layout == 2 { "stereo" } else { "mono" },
            self.seconds,
            self.rate,
            self.wander
        )
    }
}

/// A rendered loop: its name, its hash line, and its samples (interleaved
/// in stereo).
pub struct Rendered {
    pub line: String,
    pub samples: Vec<f32>,
    pub calls: Option<String>,
}

/// `tools/measure.mjs`'s fingerprint: FNV-1a over the samples' words.
fn words32(samples: impl Iterator<Item = f32>) -> String {
    let mut h: u32 = 0x811c_9dc5;
    for s in samples {
        h ^= s.to_bits();
        h = h.wrapping_mul(0x0100_0193);
    }
    format!("{h:08x}")
}

/// A number as the call lists write it (`tools/player.mjs`, `num`): a whole
/// number in decimal, NaN 'nan', anything else 'x' and its bits.
fn num(out: &mut String, x: f64) {
    if x.is_nan() {
        out.push_str("nan");
    } else if x == x.trunc() && x.abs() <= 9_007_199_254_740_992.0 {
        if x == 0.0 && x.is_sign_negative() {
            out.push_str("-0");
        } else {
            let _ = write!(out, "{}", x as i64);
        }
    } else {
        let _ = write!(out, "x{:016x}", x.to_bits());
    }
}

fn nums(out: &mut String, head: &str, xs: &[f64]) {
    out.push_str(head);
    for &x in xs {
        out.push(' ');
        num(out, x);
    }
    out.push('\n');
}

/// The core, with every call the player makes to it written down.
struct Tracer<'a> {
    core: &'a mut Core,
    lines: &'a mut String,
    /// Settings a clip pins: (block, setting, value), whatever the loop sets.
    pins: &'a [(usize, usize, f64)],
}

impl Host for Tracer<'_> {
    fn frame(&self) -> u64 {
        self.core.next_frame()
    }
    fn key(&mut self, key: Key) {
        nums(self.lines, "key", &[f64::from(key.seed), f64::from(key.pass), f64::from(key.step), f64::from(key.ord)]);
        self.core.key(key);
    }
    fn note(&mut self, voice: u32, channel: u32, time: f64, midi: f64, dur: f64, vel: f64, parts: u32, extra: [f64; EXTRA]) {
        let mut xs = vec![f64::from(voice), f64::from(channel), time, midi, dur, vel, f64::from(parts)];
        xs.extend_from_slice(&extra);
        nums(self.lines, "note", &xs);
        self.core.note(voice, channel, time, midi, dur, vel, parts, extra);
    }
    fn event(&mut self, channel: u32, event: Event) {
        if event.kind == RETUNE {
            let mut xs = vec![f64::from(channel), event.time];
            xs.extend_from_slice(&event.notes);
            nums(self.lines, "retune", &xs);
        } else {
            nums(self.lines, "event", &[f64::from(channel), f64::from(event.kind), event.time, event.value]);
        }
        self.core.event(channel, event);
    }
    fn damp(&mut self, channel: u32, time: f64) {
        nums(self.lines, "damp", &[f64::from(channel), time]);
        self.core.damp(channel, time);
    }
    fn set(&mut self, channel: u32, block: usize, setting: usize, value: f64, time: f64) {
        let value = self.pins.iter().find(|p| p.0 == block && p.1 == setting).map_or(value, |p| p.2);
        nums(self.lines, "set", &[f64::from(channel), block as f64, setting as f64, value, time]);
        self.core.set(channel, block, setting, value, time);
    }
    fn route(&mut self, channel: u32, block: usize, setting: usize, from: Option<Tap>, time: f64) {
        let from_channel = from.map_or(f64::from(u32::MAX), |t| f64::from(t.channel));
        nums(self.lines, "route", &[f64::from(channel), block as f64, setting as f64, from_channel, time]);
        self.core.route(channel, block, setting, from, time);
    }
    fn param(&mut self, id: u32, op: u32, value: f64, time: f64, tau: f64) {
        nums(self.lines, "param", &[f64::from(id), f64::from(op), value, time, tau]);
        self.core.param(id, op, value, time, tau);
    }
    fn bass_lift(&mut self, db: f64) {
        nums(self.lines, "lift", &[db]);
        self.core.bass_lift(db);
    }
    fn steal(&mut self, channel: u32, key: Key, from: f64, to: f64) {
        nums(self.lines, "steal", &[f64::from(channel), f64::from(key.seed), f64::from(key.pass), f64::from(key.step), f64::from(key.ord), from, to]);
        self.core.steal(channel, key, from, to);
    }
}

/// The seeds of the corpus `seed` draws (`Rng(seed).seed32()` a loop, as
/// `tools/measure.mjs` and `tools/gen-golden.mjs` draw it): its first `n`.
#[must_use]
pub fn corpus(seed: u32, n: usize) -> Vec<u32> {
    let mut r = Rng::new(seed);
    (0..n).map(|_| r.seed32()).collect()
}

/// Render the loop of `seed` as `way` says: started as the web host starts
/// a core, the loop made at once (`Player::ready`), then rendered, its
/// steps decided before each block.
pub fn render_seed(seed: u32, way: &Way, calls: bool) -> Result<Rendered, String> {
    let request = way.request()?;
    let frames = (way.seconds * way.rate).ceil() as usize;
    // A core and a player are megabytes each: built in a thread with room
    // for them on its stack, as the test host builds a core.
    let stack = (std::mem::size_of::<Core>() + std::mem::size_of::<Player>()) * 4 + (16 << 20);
    std::thread::scope(|scope| {
        std::thread::Builder::new()
            .stack_size(stack)
            .spawn_scoped(scope, || render_on(seed, way, &request, frames, calls))
            .map_err(|e| format!("could not start the render thread: {e}"))?
            .join()
            .map_err(|_| "the render thread panicked".to_string())?
    })
}

fn render_on(seed: u32, way: &Way, request: &Request, frames: usize, calls: bool) -> Result<Rendered, String> {
    let mut core = Box::new(Core::new());
    let mut player = Box::new(Player::new());
    let mut lines = String::new();
    core.init(way.rate);
    core.quality(way.full);
    if way.layout == 2 {
        core.set_layout(2);
    }
    player.set_wander(way.wander);
    way.clip.apply(&mut core, &mut player)?;
    {
        let mut host = Tracer { core: &mut core, lines: &mut lines, pins: &way.clip.pins };
        player.attach(&mut host, way.rate, way.full);
        player.load(seed, request);
        player.ready(&mut host);
    }
    let channels = way.layout as usize;
    let mut left = vec![0.0f32; frames];
    let mut right = vec![0.0f32; if channels == 2 { frames } else { 0 }];
    {
        let mut out: Vec<&mut [f32]> = if channels == 2 { vec![&mut left, &mut right] } else { vec![&mut left] };
        let mut block = String::new();
        core.render_with(&mut out, |core| {
            let frame = core.next_frame();
            block.clear();
            let decided = {
                let mut host = Tracer { core, lines: &mut block, pins: &way.clip.pins };
                player.decide(&mut host)
            };
            if matches!(decided, Work::Steps(_)) {
                let _ = writeln!(lines, "at {frame}");
            }
            lines.push_str(&block);
        })
        .map_err(|e| e.message().to_string())?;
    }
    if player.budget_full() > 0 {
        return Err(format!("the voice budget's list ran out of room ({} holds refused)", player.budget_full()));
    }
    let name = player.generator().spec().name.as_str().to_string();
    let mut line = format!("seed={seed} name={name} {} frames={frames}", way.describe());
    let samples: Vec<f32> = if channels == 2 {
        let _ = write!(line, " left={} right={}", words32(left.iter().copied()), words32(right.iter().copied()));
        left.iter().zip(&right).flat_map(|(&l, &r)| [l, r]).collect()
    } else {
        let _ = write!(line, " out={}", words32(left.iter().copied()));
        left
    };
    let mut h = Fnv::new();
    h.samples(&samples);
    let _ = write!(line, " hash={}", h.hex());
    if way.choices {
        // The options the loop drew other than today's, as the brain's
        // Session 26 index listed them: `lateFloor`, or `kick:soft`.
        let drawn: Vec<String> = dlcore::generator::data::CHOICES
            .iter()
            .filter_map(|c| {
                let option = player.generator().render().choice(c.id)?;
                (Some(option) != c.options.first().map(|o| o.0)).then(|| if option == "on" { c.id.to_string() } else { format!("{}:{option}", c.id) })
            })
            .collect();
        let _ = write!(line, " choices={}", if drawn.is_empty() { "-".to_string() } else { drawn.join(",") });
    }
    Ok(Rendered { line, samples, calls: calls.then_some(lines) })
}

/// The WAV of a render.
#[must_use]
pub fn wav_of(r: &Rendered, way: &Way) -> Vec<u8> {
    // `--from` trims the head of the clip: rendered from the start, written
    // from there (as Session 26's renderer did), so a loop's state is the
    // one it has by then.
    let skip = ((way.clip.from * way.rate) as usize * way.layout as usize).min(r.samples.len());
    let samples = &r.samples[skip..];
    if way.layout == 2 { wav_channels(samples, way.rate, 2) } else { wav(samples, way.rate) }
}

/// What a loop costs to play from its seed, block by block (`--costs`): the
/// load spread across blocks as a real-time host plays it (`Player::load`,
/// no `ready`), each block's work timed (the player's `decide`, then the
/// core's block), each block's least time over `runs` renders kept (the
/// work is the same every run; the machine's own interruptions are not).
pub struct Costs {
    /// The blocks the load took, and when its first step fell.
    pub load_blocks: usize,
    /// The worst block's whole time, and its parts' worst: a load's stage,
    /// a pass drawn, steps decided, the core's own block.
    pub worst_block: f64,
    pub worst_load: f64,
    pub worst_pass: f64,
    pub worst_steps: f64,
    pub worst_audio: f64,
    /// The load's stage that cost the most.
    pub worst_stage: String,
    /// The time in all: generating (the load and the passes), deciding the
    /// steps, the core's blocks; and the audio's length.
    pub generating: f64,
    pub deciding: f64,
    pub audio: f64,
    pub seconds: f64,
}

/// Time playing the loop of `seed` as `way` says, `runs` times.
pub fn cost_seed(seed: u32, way: &Way, runs: usize) -> Result<Costs, String> {
    let request = way.request()?;
    let blocks = (way.seconds * way.rate / dlcore::QUANTUM as f64).ceil() as usize;
    let stack = (std::mem::size_of::<Core>() + std::mem::size_of::<Player>()) * 4 + (16 << 20);
    std::thread::scope(|scope| {
        std::thread::Builder::new()
            .stack_size(stack)
            .spawn_scoped(scope, || {
                let mut best: Vec<(f64, f64, Work)> = vec![(f64::INFINITY, f64::INFINITY, Work::None); blocks];
                let mut stages: Vec<Option<dlcore::generator::Stage>> = vec![None; blocks];
                let mut core = Box::new(Core::new());
                let mut player = Box::new(Player::new());
                for _ in 0..runs.max(1) {
                    core.init(way.rate);
                    core.quality(way.full);
                    if way.layout == 2 {
                        core.set_layout(2);
                    }
                    player.set_wander(way.wander);
                    player.attach(&mut *core, way.rate, way.full);
                    player.load(seed, &request);
                    for (b, st) in best.iter_mut().zip(stages.iter_mut()) {
                        *st = (player.state() == dlcore::player::State::Loading).then(|| player.generator().stage());
                        let t0 = std::time::Instant::now();
                        let work = player.decide(&mut *core);
                        let t1 = std::time::Instant::now();
                        let frame = core.next_frame();
                        core.process(frame);
                        let t2 = std::time::Instant::now();
                        let (d, p) = ((t1 - t0).as_secs_f64(), (t2 - t1).as_secs_f64());
                        *b = (b.0.min(d), b.1.min(p), work);
                    }
                }
                let mut c = Costs {
                    load_blocks: best.iter().take_while(|b| matches!(b.2, Work::Load | Work::None)).count(),
                    worst_block: 0.0,
                    worst_load: 0.0,
                    worst_pass: 0.0,
                    worst_steps: 0.0,
                    worst_audio: 0.0,
                    worst_stage: String::new(),
                    generating: 0.0,
                    deciding: 0.0,
                    audio: 0.0,
                    seconds: blocks as f64 * dlcore::QUANTUM as f64 / way.rate,
                };
                for (&(d, p, work), st) in best.iter().zip(&stages) {
                    if work == Work::Load && d > c.worst_load {
                        c.worst_stage = st.map_or_else(|| "first pass".to_string(), |s| format!("{s:?}"));
                    }
                    c.worst_block = c.worst_block.max(d + p);
                    c.worst_audio = c.worst_audio.max(p);
                    c.audio += p;
                    match work {
                        Work::Load => {
                            c.worst_load = c.worst_load.max(d);
                            c.generating += d;
                        }
                        Work::Pass => {
                            c.worst_pass = c.worst_pass.max(d);
                            c.generating += d;
                        }
                        Work::Steps(_) => {
                            c.worst_steps = c.worst_steps.max(d);
                            c.deciding += d;
                        }
                        Work::None => c.deciding += d,
                    }
                }
                Ok(c)
            })
            .map_err(|e| format!("could not start the render thread: {e}"))?
            .join()
            .map_err(|_| "the render thread panicked".to_string())?
    })
}
