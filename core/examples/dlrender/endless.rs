//! `dlrender --endless HOURS` (queue item 88): an Endless piece rendered
//! for hours at speed, on the player, checked for drift and for growth.
//!
//! - **Drift:** the step the player decides next falls where the step grid
//!   says, the loop's first step plus whole steps at its tempo, to within a
//!   tenth of a sample (its clock adds a step's length at a time, as the
//!   page's engine does, and that addition's rounding moves it some 0.6 us a
//!   day, a fortieth of a sample at 44.1 kHz; the core's frames are whole
//!   numbers and cannot drift).
//! - **Growth:** each hour costs what the first did (the render's time
//!   within twice the first's, room for a shared machine's moods; the voice
//!   budget's units within a third, the notes sounding at the hour's end
//!   within half as many again), and sounds as loud (its RMS within 6 dB of
//!   the first hour's); no sample is not a number or past full scale, no
//!   note is late, missed or dropped, the voice budget's list never fills,
//!   and the generator's lists drop nothing.
//!
//! One line an hour, and a last line saying whether it held. CI runs 24
//! hours of one loop at 22,050 Hz, lite, mono (some thirteen minutes here:
//! 115 times real time).

// Frames, steps and hours, far below 2^53.
#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]

use std::time::Instant;

use dlcore::player::{Player, Request};
use dlcore::{Core, QUANTUM};

/// How the endless render is made.
pub struct Way {
    pub hours: f64,
    pub seed: u32,
    pub rate: f64,
    pub full: bool,
    pub layout: u32,
    pub wander: f64,
    pub request: Request,
}

/// What an hour cost and how it sounded.
struct Hour {
    wall: f64,
    busy: usize,
    units: f64,
    rms: f64,
    peak: f32,
}

/// Render `way.hours` hours, a line an hour; an error says what didn't hold.
pub fn run(way: &Way) -> Result<(), String> {
    // A core and a player are built on the stack before they are boxed.
    let stack = (std::mem::size_of::<Core>() + std::mem::size_of::<Player>()) * 4 + (16 << 20);
    crate::notelist::roomy(stack, || render(way))
}

fn render(way: &Way) -> Result<(), String> {
    let mut core = Box::new(Core::new());
    let mut player = Box::new(Player::new());
    player.start(&mut core, way.rate, way.layout, way.full).map_err(|e| format!("{e:?}"))?;
    player.set_wander(way.wander);
    player.load(way.seed, &way.request);
    player.ready(&mut *core);
    let name = player.generator().spec().name.as_str().to_string();
    let bpm = player.generator().spec().bpm;
    let sd = 60.0 / bpm / 4.0;
    // The step grid's origin: the first step's time.
    let (_, _, first) = player.position();
    let hour_frames = (3600.0 * way.rate) as usize;
    let chunk = QUANTUM * 64;
    let mut left = vec![0.0f32; chunk];
    let mut right = vec![0.0f32; chunk];
    let mut hours: Vec<Hour> = Vec::new();
    let mut worst_drift = 0.0f64;
    let whole = way.hours.ceil() as usize;
    println!("endless seed={} name={name} bpm={bpm} quality={} layout={} rate={} hours={}", way.seed, if way.full { "full" } else { "lite" }, if way.layout == 2 { "stereo" } else { "mono" }, way.rate, way.hours);
    for h in 0..whole {
        let frames = if h + 1 == whole && way.hours.fract() > 0.0 { (way.hours.fract() * 3600.0 * way.rate) as usize } else { hour_frames };
        let started = Instant::now();
        let (mut sum, mut peak, mut done) = (0.0f64, 0.0f32, 0usize);
        let mut units = 0.0;
        let mut readings = 0u32;
        while done < frames {
            let n = chunk.min(frames - done);
            if way.layout == 2 {
                let (l, r) = (&mut left[..n], &mut right[..n]);
                player.render(&mut core, &mut [l, r]).map_err(|e| format!("{e:?}"))?;
            } else {
                player.render(&mut core, &mut [&mut left[..n]]).map_err(|e| format!("{e:?}"))?;
            }
            for &x in left[..n].iter().chain(if way.layout == 2 { &right[..n] } else { &right[..0] }) {
                if !x.is_finite() || x.abs() > 1.0 {
                    return Err(format!("hour {h}: a sample of {x}"));
                }
                sum += f64::from(x) * f64::from(x);
                peak = peak.max(x.abs());
            }
            done += n;
            // The next step against the grid.
            let (_, _, next) = player.position();
            let steps = player.steps() as f64;
            worst_drift = worst_drift.max((next - (first + steps * sd)).abs());
            if let Some(u) = player.units() {
                units += u;
                readings += 1;
            }
        }
        let wall = started.elapsed().as_secs_f64();
        let count = (done * way.layout as usize) as f64;
        let hour = Hour { wall, busy: core.busy(), units: units / f64::from(readings.max(1)), rms: (sum / count).sqrt(), peak };
        let (_, section, _) = player.position();
        println!(
            "hour={} section={section} wall={:.1}s speed={:.0}x busy={} units={:.1} rms={:.2}dB peak={:.2}dB drift={:.3}us",
            h + 1,
            hour.wall,
            done as f64 / way.rate / hour.wall,
            hour.busy,
            hour.units,
            20.0 * hour.rms.log10(),
            20.0 * f64::from(hour.peak).log10(),
            worst_drift * 1e6
        );
        hours.push(hour);
    }
    let mut failed = verdict(way, &hours, worst_drift);
    if core.late() + core.dropped() != 0 {
        failed.push(format!("{} notes late, {} dropped", core.late(), core.dropped()));
    }
    if player.budget_full() != 0 {
        failed.push(format!("the voice budget's list filled {} times", player.budget_full()));
    }
    if player.generator().lost() != 0 {
        failed.push("the generator's lists dropped something".to_string());
    }
    if failed.is_empty() {
        println!("endless: held for {} hours (drift {:.3} us)", way.hours, worst_drift * 1e6);
        Ok(())
    } else {
        Err(failed.join("; "))
    }
}

/// What the hours say didn't hold: the drift past a tenth of a sample, and
/// any hour that cost or sounded unlike the first.
fn verdict(way: &Way, hours: &[Hour], worst_drift: f64) -> Vec<String> {
    let Some(first) = hours.first() else { return vec!["no hour rendered".to_string()] };
    let mut failed = Vec::new();
    if worst_drift > 0.1 / way.rate {
        failed.push(format!("the step grid drifted {:.3} us", worst_drift * 1e6));
    }
    for (i, h) in hours.iter().enumerate().skip(1) {
        if h.wall > first.wall * 2.0 + 5.0 {
            failed.push(format!("hour {} took {:.1}s, the first {:.1}s", i + 1, h.wall, first.wall));
        }
        if h.busy > first.busy.max(8) * 3 / 2 {
            failed.push(format!("hour {}: {} notes sounding at its end, the first hour {}", i + 1, h.busy, first.busy));
        }
        let u = h.units / first.units.max(1e-9);
        if !(0.67..=1.5).contains(&u) {
            failed.push(format!("hour {}: the voices held {:.1} units, the first hour {:.1}", i + 1, h.units, first.units));
        }
        let db = 20.0 * (h.rms / first.rms.max(1e-12)).log10();
        if db.abs() > 6.0 {
            failed.push(format!("hour {}: {db:+.2} dB against the first", i + 1));
        }
    }
    failed
}
