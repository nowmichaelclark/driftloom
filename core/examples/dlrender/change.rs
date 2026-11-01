//! Changes of loop at a musical moment (queue item 86): `dlrender --changes
//! N`. Each change plays a corpus loop, asks for the next one at a moment
//! (`Player::change`; the five kinds in turn, every other one bending the
//! old loop's seed as a theme, a friend's mood changing), and renders past
//! the crossfade. Then the same new loop loaded alone, its first step at
//! the same time on a core started the same way. Each change is measured:
//!
//! - **on the moment**: the new loop's first step falls on the old loop's
//!   grid for that moment (`player::change::unit`), and where the change
//!   said it would (`Player::next_change`);
//! - **no click**: the largest step between two samples from just before
//!   the boundary to just after the crossfade, against the largest either
//!   loop takes in its own playing (each alone, over the same length), and
//!   against the two summed (the most two loops sounding together step: the
//!   old one's last notes still ring as the new one's first play);
//! - **the new loop as itself**: the difference between the change's render
//!   and the loop alone, second by second from the crossfade's end, in dB
//!   under the louder loop (what is left is the old loop's room and echo
//!   tails, the held instruments' glides and the master's state);
//! - **the cost**: the worst block (the player's `decide` and the core's
//!   block) from the change asked to the crossfade's end, as a share of a
//!   block's deadline, beside the worst of the old loop's own before it.
//!
//! It prints a line a change with `--each`, and a summary; it fails if a
//! change missed its moment.

// Frame counts and rates here are bounded by the render's own length.
#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]

use std::fmt::Write as _;
use std::time::Instant;

use dlcore::Core;
use dlcore::QUANTUM;
use dlcore::generator::plan::Cx;
use dlcore::generator::rng::Rng;
use dlcore::generator::spec::Metre;
use dlcore::player::change::unit;
use dlcore::player::{CROSSFADE, Handover, Moment, Player, Request, State};

/// How the changes are made and measured.
#[derive(Clone)]
pub struct Way {
    pub changes: usize,
    pub corpus: u32,
    pub rate: f64,
    pub full: bool,
    pub wander: f64,
    pub handover: Handover,
    /// Only this kind of moment (else all five in turn).
    pub moment: Option<Moment>,
    /// Print a line for each change.
    pub each: bool,
    /// Render each change this many times, keeping each block's least time.
    pub runs: usize,
    /// Only change number `pick`, and its renders written to `wav` (a
    /// directory: `I-change.wav`, `I-alone.wav`).
    pub pick: Option<usize>,
    pub wav: Option<String>,
}

/// One change, measured.
#[derive(Clone, Default)]
struct Measured {
    line: String,
    missed: bool,
    join: f64,
    own: f64,
    /// The two loops' own steps summed: the most two loops sounding
    /// together (the old one's last notes under the new one's first) step.
    both: f64,
    residual: [f64; 3],
    worst: f64,
    worst_own: f64,
}

fn step_max(x: &[f32]) -> f64 {
    x.windows(2).map(|w| f64::from((w[1] - w[0]).abs())).fold(0.0, f64::max)
}

fn rms(x: &[f32]) -> f64 {
    if x.is_empty() {
        return 0.0;
    }
    (x.iter().map(|&s| f64::from(s) * f64::from(s)).sum::<f64>() / x.len() as f64).sqrt()
}

fn db(x: f64) -> f64 {
    if x > 0.0 { 20.0 * x.log10() } else { -300.0 }
}

/// Render into `out` a block at a time, with `before` called before each
/// (it may stop the render: false), each block's time kept at its least.
fn blocks(core: &mut Core, player: &mut Player, out: &mut [f32], times: &mut [f64], mut before: impl FnMut(&mut Player, usize) -> bool) {
    for (b, chunk) in out.chunks_mut(QUANTUM).enumerate() {
        if !before(player, b) {
            break;
        }
        let t0 = Instant::now();
        player.render(core, &mut [chunk]).expect("a render");
        let t = t0.elapsed().as_secs_f64();
        if let Some(best) = times.get_mut(b) {
            *best = best.min(t);
        }
    }
}

fn fresh(way: &Way) -> (Box<Core>, Box<Player>) {
    let mut core = Box::new(Core::new());
    let mut player = Box::new(Player::new());
    player.start(&mut core, way.rate, 1, way.full).expect("a player");
    player.set_wander(way.wander);
    (core, player)
}

/// Change `i`: what it asks, and its measure.
fn measure(i: usize, seeds: (u32, u32), way: &Way) -> Measured {
    let (old, new) = seeds;
    let moment = way.moment.unwrap_or(Moment::ALL[i % Moment::ALL.len()]);
    let request = if i % 2 == 1 { Request::new().with_theme(old) } else { Request::new() };
    let mut r = Rng::new(0x86_0000 + i as u32);
    let ask = 2.0 + r.range(0.0, 5.0);
    let ask_block = (ask * way.rate / QUANTUM as f64) as usize;
    let fade = way.handover.fade().unwrap_or(0.0);
    let rate = way.rate;
    let deadline = QUANTUM as f64 / rate;

    // The change, rendered until its boundary is known, then past it.
    let mut boundary: Option<f64> = None;
    let mut said: Option<f64> = None;
    let mut old_start = 0.0;
    let mut old_bpm = 0.0;
    let mut grid = None;
    let mut out = Vec::new();
    let mut times = Vec::new();
    for run in 0..way.runs.max(1) {
        let (mut core, mut player) = fresh(way);
        player.load(old, &Request::new());
        player.ready(&mut *core);
        old_start = player.position().2;
        old_bpm = player.generator().spec().bpm;
        let spec = player.generator().spec();
        let beat = if Cx::new(spec).metre == Metre::SixEight { 6.0 } else { 4.0 };
        grid = unit(moment, beat, spec.steps_per_bar, spec.bars * spec.steps_per_bar);
        // Long enough for two passes' ends (a boundary nearer than the lead
        // is passed for the next), the crossfade, and three seconds.
        let pass = spec.bars * spec.steps_per_bar * 15.0 / spec.bpm;
        let total = ask + 2.0 * pass + fade + 6.0;
        let frames = (total * rate) as usize;
        if run == 0 {
            out = vec![0.0f32; frames];
            times = vec![f64::INFINITY; frames.div_ceil(QUANTUM)];
        }
        let mut done_at = out.len();
        blocks(&mut core, &mut player, &mut out[..], &mut times, |player, b| {
            if b == ask_block {
                player.change(new, &request, moment, way.handover);
            }
            if b > ask_block && said.is_none() && moment != Moment::Now {
                said = player.next_change();
            }
            if b > ask_block && boundary.is_none() && player.state() == State::Playing && player.generator().spec().seed == new {
                let t = player.position().2;
                boundary = Some(t);
                done_at = ((t + fade + 3.5) * rate) as usize;
            }
            b * QUANTUM < done_at
        });
        out.truncate(done_at.min(out.len()));
    }
    let Some(tb) = boundary else {
        return Measured { line: format!("change={i} old={old} new={new} moment={moment:?}: never changed"), missed: true, ..Measured::default() };
    };
    let frames = out.len();

    // The new loop alone, its first step at the boundary.
    let (mut core, mut player) = fresh(way);
    player.load_at(new, &request, tb);
    let mut alone = vec![0.0f32; frames];
    let mut no_times = vec![f64::INFINITY; frames.div_ceil(QUANTUM)];
    blocks(&mut core, &mut player, &mut alone, &mut no_times, |_, _| true);

    // On the moment.
    let sd = 60.0 / old_bpm / 4.0;
    let steps = (tb - old_start) / sd;
    let on_grid = (steps - steps.round()).abs() < 1e-6 && grid.is_none_or(|u| steps.round() % u == 0.0);
    let on_said = moment == Moment::Now || said.is_some_and(|s| (s - tb).abs() < 1e-6);
    let missed = !on_grid || !on_said;

    let at = |t: f64| ((t * rate) as usize).min(frames);
    let join = step_max(&out[at(tb - 0.02)..at(tb + fade + 0.02)]);
    // The old loop's own playing: the same length, with no change asked.
    let (mut core, mut player) = fresh(way);
    player.load(old, &Request::new());
    player.ready(&mut *core);
    let mut old_alone = vec![0.0f32; frames];
    blocks(&mut core, &mut player, &mut old_alone, &mut no_times, |_, _| true);
    let (own_old, own_new) = (step_max(&old_alone[at(0.1)..]), step_max(&alone[at(tb)..]));
    let (own, both) = (own_old.max(own_new), own_old + own_new);
    if let Some(dir) = &way.wav {
        write_wavs(dir, i, rate, [&out, &alone, &old_alone]);
    }
    let residual = residuals(&out, &alone, &old_alone, tb, fade, rate);
    let blk = |t: f64| ((t * rate) as usize / QUANTUM).min(times.len());
    let worst = times[blk(ask)..blk(tb + fade)].iter().copied().fold(0.0, f64::max) / deadline;
    let worst_own = times[blk(0.5)..blk(ask)].iter().copied().fold(0.0, f64::max) / deadline;
    let mut line = String::new();
    let _ = write!(
        line,
        "change={i} old={old} new={new} moment={moment:?} theme={} ask={ask:.2} at={tb:.3} steps={steps:.0}{} join={join:.4} own={own:.4} both={both:.4} after={:.1},{:.1},{:.1}dB worst={:.1}% own={:.1}%",
        i % 2 == 1,
        if missed { " MISSED" } else { "" },
        residual[0],
        residual[1],
        residual[2],
        worst * 100.0,
        worst_own * 100.0
    );
    Measured { line, missed, join, own, both, residual, worst, worst_own }
}

/// What is left of the old loop after the crossfade, second by second for
/// three seconds: the difference between the change and the new loop alone,
/// in dB under the louder of the two loops (the new one alone from its first
/// step, the old one alone before the boundary: a loop may open quietly).
fn residuals(out: &[f32], alone: &[f32], old_alone: &[f32], tb: f64, fade: f64, rate: f64) -> [f64; 3] {
    let at = |t: f64| ((t * rate) as usize).min(out.len());
    let level = rms(&alone[at(tb)..]).max(rms(&old_alone[at(0.1)..at(tb)]));
    let mut residual = [-300.0; 3];
    for (k, res) in residual.iter_mut().enumerate() {
        let (a, b) = (at(tb + fade + k as f64), at(tb + fade + k as f64 + 1.0));
        let diff: Vec<f32> = out[a..b].iter().zip(&alone[a..b]).map(|(x, y)| x - y).collect();
        *res = db(rms(&diff)) - db(level);
    }
    residual
}

/// A change's three renders as WAVs (`--wav-to`), to listen to and to
/// compare: `I-change.wav`, `I-alone.wav` (the new loop) and `I-old.wav`.
fn write_wavs(dir: &str, i: usize, rate: f64, [change, alone, old]: [&[f32]; 3]) {
    for (name, samples) in [("change", change), ("alone", alone), ("old", old)] {
        let path = format!("{dir}/{i}-{name}.wav");
        if let Err(e) = std::fs::write(&path, crate::notelist::wav(samples, rate)) {
            eprintln!("{path}: {e}");
        }
    }
}

/// `dlrender --changes N`: the changes, four at a time, and the summary.
pub fn run(way: &Way) -> Result<(), String> {
    let seeds = crate::seed::corpus(way.corpus, way.changes * 2);
    let pairs: Vec<(usize, (u32, u32))> = (0..way.changes).filter(|&i| way.pick.is_none_or(|p| p == i)).map(|i| (i, (seeds[2 * i], seeds[2 * i + 1]))).collect();
    let stack = (std::mem::size_of::<Core>() + std::mem::size_of::<Player>()) * 4 + (16 << 20);
    // Timed on one thread (`--runs` over 1), so the changes do not share the
    // machine with each other; measured four at a time otherwise.
    let threads = if way.runs > 1 { 1 } else { std::thread::available_parallelism().map_or(1, std::num::NonZero::get).min(8) };
    let mut results: Vec<(usize, Measured)> = Vec::new();
    std::thread::scope(|scope| -> Result<(), String> {
        let chunks: Vec<Vec<(usize, (u32, u32))>> = (0..threads).map(|t| pairs.iter().copied().skip(t).step_by(threads).collect()).collect();
        let mut handles = Vec::new();
        for chunk in chunks {
            let way = way.clone();
            let h = std::thread::Builder::new()
                .stack_size(stack)
                .spawn_scoped(scope, move || chunk.into_iter().map(|(i, s)| (i, measure(i, s, &way))).collect::<Vec<_>>())
                .map_err(|e| format!("could not start a thread: {e}"))?;
            handles.push(h);
        }
        for h in handles {
            results.extend(h.join().map_err(|_| "a change's thread panicked".to_string())?);
        }
        Ok(())
    })?;
    results.sort_by_key(|r| r.0);
    if way.each {
        for (_, m) in &results {
            println!("{}", m.line);
        }
    }
    let n = results.len();
    let missed = results.iter().filter(|r| r.1.missed).count();
    let over = results.iter().filter(|r| r.1.join > r.1.own).count();
    let ratio = results.iter().map(|r| r.1.join / r.1.own.max(1e-9)).fold(0.0, f64::max);
    let over_both = results.iter().filter(|r| r.1.join > r.1.both).count();
    let ratio_both = results.iter().map(|r| r.1.join / r.1.both.max(1e-9)).fold(0.0, f64::max);
    let res: Vec<f64> = (0..3).map(|k| results.iter().map(|r| r.1.residual[k]).fold(-300.0, f64::max)).collect();
    let median = |k: usize| {
        let mut v: Vec<f64> = results.iter().map(|r| r.1.residual[k]).collect();
        v.sort_by(f64::total_cmp);
        v.get(v.len() / 2).copied().unwrap_or(-300.0)
    };
    let worst = results.iter().map(|r| r.1.worst).fold(0.0, f64::max);
    let worst_own = results.iter().map(|r| r.1.worst_own).fold(0.0, f64::max);
    let fade = way.handover.fade().map_or("ring out".to_string(), |x| format!("crossfade {x} s"));
    println!(
        "changes={n} corpus={} rate={} quality={} wander={} {fade}: missed {missed}; join step over the larger loop's own in {over} (worst {ratio:.2}x), over the two loops' own summed in {over_both} (worst {ratio_both:.2}x); after the crossfade, against the loop alone: worst {:.1}, {:.1}, {:.1} dB, median {:.1}, {:.1}, {:.1} dB (seconds 1-3); worst block {:.1}% of its deadline while changing (the old loop's own worst {:.1}%)",
        way.corpus,
        way.rate,
        if way.full { "full" } else { "lite" },
        way.wander,
        res[0],
        res[1],
        res[2],
        median(0),
        median(1),
        median(2),
        worst * 100.0,
        worst_own * 100.0
    );
    if missed > 0 {
        return Err(format!("{missed} changes missed their moment"));
    }
    Ok(())
}

/// The handover `--crossfade S` or `--ring` asks for (the default's
/// crossfade without either).
pub fn handover(crossfade: Option<f64>, ring: bool) -> Handover {
    if ring { Handover::RingOut } else { Handover::Crossfade(crossfade.unwrap_or(CROSSFADE)) }
}
