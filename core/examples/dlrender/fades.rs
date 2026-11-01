//! The voice budget's let-go fades, heard (queue item 85e): `dlrender --seed
//! N --fades` or `--corpus SEED:N --fades`. Each loop plays on the player as
//! `--seed` plays it, every note the budget lets go for a newer one written
//! down (`Host::steal`: its fade, from and to); then the largest step between
//! two samples inside each fade (from 5 ms before it to its end, where the
//! note that asked for the room starts) is held against the largest its own
//! channel takes anywhere outside the fades (each channel's output before the
//! mix, so another layer's hit landing in a fade isn't counted as it). A fade that steps further than the
//! loop's own playing would be a click. The 5 ms after each fade, the new
//! note's attack among the old note's last, are counted apart (`at`).

// Frame counts and rates here are bounded by the render's own length.
#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]

use dlcore::block::Event;
use dlcore::player::host::Host;
use dlcore::player::{Player, Request};
use dlcore::{CHANNELS, Core, EXTRA, Key, QUANTUM, Tap};

use crate::seed::Way;

/// A loop's fades, measured: how many, the largest step inside one, in the
/// 5 ms after one, and outside them.
pub struct Fades {
    pub name: String,
    pub fades: usize,
    /// The largest step inside a fade over its channel's own outside them,
    /// in the channel where that is furthest; and the same for the 5 ms
    /// after a fade.
    pub ratio: f64,
    pub at_ratio: f64,
    pub channel: usize,
    /// Whether that largest step is in a note's attack (a note of the
    /// channel starting within 15 ms before it): a chord's other tones, strummed in
    /// before the one that asked for the room, start inside its fade.
    pub attack: bool,
}

/// The core, with every let-go fade written down.
struct Watch<'a> {
    core: &'a mut Core,
    fades: &'a mut Vec<(u32, f64, f64)>,
    starts: &'a mut Vec<(u32, f64)>,
}

impl Host for Watch<'_> {
    fn frame(&self) -> u64 {
        self.core.next_frame()
    }
    fn key(&mut self, key: Key) {
        self.core.key(key);
    }
    fn note(&mut self, voice: u32, channel: u32, time: f64, midi: f64, dur: f64, vel: f64, parts: u32, extra: [f64; EXTRA]) {
        self.starts.push((channel, time));
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
        self.fades.push((channel, from, to));
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
                let blocks = (way.seconds * way.rate / QUANTUM as f64).ceil() as usize;
                let frames = blocks * QUANTUM;
                // Each channel's own output, before the mix: a fade is held
                // against its own channel's playing, not another layer's hit.
                let mut chans = vec![vec![0.0f32; frames]; CHANNELS];
                let mut fades = Vec::new();
                let mut starts = Vec::new();
                for b in 0..blocks {
                    player.decide(&mut Watch { core: &mut core, fades: &mut fades, starts: &mut starts });
                    let frame = core.next_frame();
                    let out = core.process(frame);
                    for (c, lane) in chans.iter_mut().enumerate() {
                        lane[b * QUANTUM..(b + 1) * QUANTUM].copy_from_slice(&out[c]);
                    }
                }
                let at = |t: f64| ((t * way.rate).max(0.0) as usize).min(frames);
                let (mut ratio, mut at_ratio, mut channel, mut attack) = (0.0f64, 0.0f64, 0, false);
                for (c, lane) in chans.iter().enumerate() {
                    // 1 inside a fade, 2 just after one (the new note's attack).
                    let mut mask = vec![0u8; frames];
                    for &(_, _, to) in fades.iter().filter(|f| f.0 as usize == c) {
                        for m in &mut mask[at(to)..at(to + 0.005)] {
                            *m = (*m).max(2);
                        }
                    }
                    for &(_, from, to) in fades.iter().filter(|f| f.0 as usize == c) {
                        for m in &mut mask[at(from - 0.005)..at(to)] {
                            *m = 1;
                        }
                    }
                    let (mut i_c, mut a_c, mut o_c, mut i_at) = (0.0f64, 0.0f64, 0.0f64, 0);
                    for i in 1..frames {
                        let step = f64::from((lane[i] - lane[i - 1]).abs());
                        match mask[i] {
                            1 if step > i_c => {
                                i_c = step;
                                i_at = i;
                            }
                            1 => {}
                            2 => a_c = a_c.max(step),
                            _ => o_c = o_c.max(step),
                        }
                    }
                    if o_c > 0.0 {
                        if i_c / o_c > ratio {
                            ratio = i_c / o_c;
                            channel = c;
                            // A note of the channel starting within 15 ms
                            // before it: the step is in that note's attack
                            // (a pluck's steepest samples come a few cycles
                            // in).
                            let t = i_at as f64 / way.rate;
                            attack = starts.iter().any(|&(ch, s)| ch as usize == c && s <= t + 0.0005 && s >= t - 0.015);
                            if std::env::var("FADES_DEBUG").is_ok() {
                                let near = starts.iter().filter(|&&(ch, s)| ch as usize == c && s <= t + 0.0005).map(|&(_, s)| t - s).fold(f64::INFINITY, f64::min);
                                let fade = fades.iter().filter(|f| f.0 as usize == c && f.1 - 0.005 <= t && t < f.2).map(|f| (f.1, f.2)).next();
                                eprintln!("channel {c}: largest step in a fade {i_c:.4} at {t:.4} s (fade {fade:?}); the last note started {:.1} ms before", near * 1000.0);
                            }
                        }
                        at_ratio = at_ratio.max(a_c / o_c);
                    }
                }
                let name = player.generator().spec().name.as_str().to_string();
                Ok(Fades { name, fades: fades.len(), ratio, at_ratio, channel, attack })
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
    let (mut over_at, mut worst_at, mut unexplained) = (0, 0.0f64, 0);
    for &(index, s) in seeds {
        let f = measure(s, way)?;
        loops += 1;
        all += f.fades;
        if f.fades > 0 {
            with += 1;
            worst = worst.max(f.ratio);
            worst_at = worst_at.max(f.at_ratio);
            over += usize::from(f.ratio > 1.0);
            over_at += usize::from(f.at_ratio > 1.0);
        }
        let i = index.map_or(String::new(), |i| format!("{i} "));
        if f.ratio > 1.0 && !f.attack {
            unexplained += 1;
        }
        println!("{i}seed={s} name={} fades={} ratio={:.2} channel={} attack={} at_ratio={:.2}", f.name, f.fades, f.ratio, f.channel, f.attack, f.at_ratio);
    }
    println!(
        "loops={loops} quality={} seconds={}: {all} fades in {with} loops; a fade's step over its channel's own in {over} (worst {worst:.2}x; {unexplained} of them not a note's attack); the 5 ms after one, as the new note starts, in {over_at} (worst {worst_at:.2}x)",
        if way.full { "full" } else { "lite" },
        way.seconds
    );
    Ok(())
}
