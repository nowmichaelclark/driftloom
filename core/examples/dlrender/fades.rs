//! The voice budget's let-go fades, heard (queue item 85e): `dlrender --seed
//! N --fades` or `--corpus SEED:N --fades`. Each loop plays on the player as
//! `--seed` plays it, every note the budget lets go for a newer one written
//! down (`Host::steal`: its fade, from and to); then the largest step between
//! two samples inside each fade (from 5 ms before it to 5 ms after) is held
//! against the largest the loop takes anywhere outside the fades. A fade
//! that steps further than the loop's own playing would be a click.

// Frame counts and rates here are bounded by the render's own length.
#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]

use dlcore::block::Event;
use dlcore::player::host::Host;
use dlcore::player::{Player, Request};
use dlcore::{Core, EXTRA, Key, Tap};

use crate::seed::Way;

/// A loop's fades, measured: how many, the largest step inside one, the
/// largest outside them, and the voice calls let go.
pub struct Fades {
    pub name: String,
    pub fades: usize,
    pub inside: f64,
    pub outside: f64,
}

/// The core, with every let-go fade written down.
struct Watch<'a> {
    core: &'a mut Core,
    fades: &'a mut Vec<(f64, f64)>,
}

impl Host for Watch<'_> {
    fn frame(&self) -> u64 {
        self.core.next_frame()
    }
    fn key(&mut self, key: Key) {
        self.core.key(key);
    }
    fn note(&mut self, voice: u32, channel: u32, time: f64, midi: f64, dur: f64, vel: f64, parts: u32, extra: [f64; EXTRA]) {
        self.core.note(voice, channel, time, midi, dur, vel, parts, extra);
    }
    fn event(&mut self, channel: u32, event: Event) {
        self.core.event(channel, event);
    }
    fn damp(&mut self, channel: u32, time: f64) {
        self.core.damp(channel, time);
    }
    fn set(&mut self, channel: u32, block: usize, setting: usize, value: f64, time: f64) {
        self.core.set(channel, block, setting, value, time);
    }
    fn route(&mut self, channel: u32, block: usize, setting: usize, from: Option<Tap>, time: f64) {
        self.core.route(channel, block, setting, from, time);
    }
    fn param(&mut self, id: u32, op: u32, value: f64, time: f64, tau: f64) {
        self.core.param(id, op, value, time, tau);
    }
    fn bass_lift(&mut self, db: f64) {
        self.core.bass_lift(db);
    }
    fn space_range(&mut self, which: usize, lo: f64, hi: f64) {
        self.core.space_range(which, lo, hi);
    }
    fn steal(&mut self, channel: u32, key: Key, from: f64, to: f64) {
        self.fades.push((from, to));
        self.core.steal(channel, key, from, to);
    }
    fn fade_all(&mut self, from: f64, to: f64) {
        self.core.fade_all(from, to);
    }
}

/// Play the loop of `seed` as `way` says (mono), its fades measured.
pub fn measure(seed: u32, way: &Way) -> Result<Fades, String> {
    let request: Request = way.request()?;
    let stack = (std::mem::size_of::<Core>() + std::mem::size_of::<Player>()) * 4 + (16 << 20);
    std::thread::scope(|scope| {
        std::thread::Builder::new()
            .stack_size(stack)
            .spawn_scoped(scope, || {
                let mut core = Box::new(Core::new());
                let mut player = Box::new(Player::new());
                player.start(&mut core, way.rate, 1, way.full).map_err(|e| e.message().to_string())?;
                player.set_wander(way.wander);
                player.load(seed, &request);
                player.ready(&mut *core);
                let frames = (way.seconds * way.rate).ceil() as usize;
                let mut out = vec![0.0f32; frames];
                let mut fades = Vec::new();
                core.render_with(&mut [&mut out], |core| {
                    player.decide(&mut Watch { core, fades: &mut fades });
                })
                .map_err(|e| e.message().to_string())?;
                let at = |t: f64| ((t * way.rate).max(0.0) as usize).min(frames);
                let mut inside_mask = vec![false; frames];
                for &(from, to) in &fades {
                    for m in &mut inside_mask[at(from - 0.005)..at(to + 0.005)] {
                        *m = true;
                    }
                }
                let (mut inside, mut outside) = (0.0f64, 0.0f64);
                for i in 1..frames {
                    let step = f64::from((out[i] - out[i - 1]).abs());
                    if inside_mask[i] {
                        inside = inside.max(step);
                    } else {
                        outside = outside.max(step);
                    }
                }
                let name = player.generator().spec().name.as_str().to_string();
                Ok(Fades { name, fades: fades.len(), inside, outside })
            })
            .map_err(|e| format!("could not start the render thread: {e}"))?
            .join()
            .map_err(|_| "the render thread panicked".to_string())?
    })
}

/// `--fades`: a line a loop, and how many loops' fades step past their own
/// playing.
pub fn run(seeds: &[(Option<usize>, u32)], way: &Way) -> Result<(), String> {
    let (mut loops, mut with, mut over, mut worst, mut all) = (0, 0, 0, 0.0f64, 0);
    for &(index, s) in seeds {
        let f = measure(s, way)?;
        loops += 1;
        all += f.fades;
        let ratio = if f.outside > 0.0 { f.inside / f.outside } else { 0.0 };
        if f.fades > 0 {
            with += 1;
            worst = worst.max(ratio);
            if f.inside > f.outside {
                over += 1;
            }
        }
        let i = index.map_or(String::new(), |i| format!("{i} "));
        println!("{i}seed={s} name={} fades={} inside={:.4} outside={:.4} ratio={ratio:.2}", f.name, f.fades, f.inside, f.outside);
    }
    println!(
        "loops={loops} quality={} seconds={}: {all} fades in {with} loops; a fade's step over the loop's own in {over} (worst {worst:.2}x)",
        if way.full { "full" } else { "lite" },
        way.seconds
    );
    Ok(())
}
