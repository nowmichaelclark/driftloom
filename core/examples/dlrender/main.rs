//! `dlrender`: the core's native test host (queue item 42).
//!
//! Reads note lists (`notelist.rs`), renders each through the core on this
//! machine and prints one line of hashes per list; with `--wav` it writes
//! the samples too. No JavaScript, no dependencies: the host a game or a
//! command-line exporter would be (roadmap 17 grows from here).
//!
//! ```text
//! cargo run --release --example dlrender -- LIST...          hash lines
//! cargo run --release --example dlrender -- --twice LIST...  and render each
//!                                           twice, failing if they differ
//! cargo run --release --example dlrender -- --flush LIST...  with the
//!                                           processor flushing subnormal
//!                                           numbers (must hash the same)
//! cargo run --release --example dlrender -- --rate 96000 LIST...  at another
//!                                           sample rate (the name says which)
//! cargo run --release --example dlrender -- --blocks 1,441,random LIST...
//!                                           also render each through
//!                                           `Core::render`, that many frames
//!                                           a call, failing if the speaker
//!                                           output differs from the plain
//!                                           render's
//! cargo run --release --example dlrender -- --wav OUT.wav [--signal NAME] LIST
//! cargo run --release --example dlrender -- --compare A.wav B.wav
//! cargo run --release --example dlrender -- --block NAME/PROBE [--rate HZ] [--wav OUT.wav]
//!                                           one building block alone, as the
//!                                           test kit plays it (queue item 46):
//!                                           its hash, and its samples for an
//!                                           A/B between two versions
//! ```
//!
//! Everything it prints is plain ASCII. It exits with 1 on any error.

mod notelist;

use std::process::ExitCode;

use dlcore::block::{Block, Context, Note, Voice, solo};
use dlcore::blocks::{Visit, each};
use dlcore::Core;
use notelist::{Blocks, Fnv, SIGNALS, line, parse, read_wav, render, render_blocks, render_flushing, wav};

const USAGE: &str = "usage: dlrender [--twice] [--flush] [--rate HZ] [--blocks N,N,random] [--wav OUT.wav [--signal NAME]] LIST...\n       dlrender --compare A.wav B.wav\n       dlrender --block NAME/PROBE [--rate HZ] [--wav OUT.wav]";

fn main() -> ExitCode {
    match run(std::env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("dlrender: {message}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    let (mut twice, mut flush, mut wav_to, mut signal) = (false, false, None, "out".to_string());
    let (mut compare, mut lists, mut block) = (false, Vec::new(), None);
    let (mut rate, mut blocks): (Option<f64>, Vec<Blocks>) = (None, Vec::new());
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--twice" => twice = true,
            "--flush" => flush = true,
            "--compare" => compare = true,
            "--block" => block = Some(args.next().ok_or(USAGE)?),
            "--wav" => wav_to = Some(args.next().ok_or(USAGE)?),
            "--signal" => signal = args.next().ok_or(USAGE)?,
            "--rate" => rate = Some(args.next().ok_or(USAGE)?.parse::<f64>().map_err(|_| USAGE)?),
            "--blocks" => {
                for word in args.next().ok_or(USAGE)?.split(',') {
                    blocks.push(Blocks::parse(word)?);
                }
            }
            "--help" | "-h" => {
                println!("{USAGE}");
                return Ok(());
            }
            flag if flag.starts_with("--") => return Err(format!("no such option: {flag}\n{USAGE}")),
            _ => lists.push(arg),
        }
    }
    if compare {
        let [a, b] = <[String; 2]>::try_from(lists).map_err(|_| USAGE.to_string())?;
        return compare_wavs(&a, &b);
    }
    if let Some(name) = block {
        return solo_block(&name, rate.unwrap_or(48_000.0), wav_to.as_deref());
    }
    if lists.is_empty() || (wav_to.is_some() && lists.len() != 1) {
        return Err(USAGE.to_string());
    }
    let keep = match &wav_to {
        Some(_) => Some(
            SIGNALS
                .iter()
                .position(|s| *s == signal)
                .ok_or_else(|| format!("no such signal: {signal} (one of {})", SIGNALS.join(", ")))?,
        ),
        None => None,
    };
    for path in &lists {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
        let mut list = parse(&text).map_err(|e| format!("{path}: {e}"))?;
        if let Some(rate) = rate {
            list = list.at_rate(rate);
        }
        let first = if flush { render_flushing(&list, keep) } else { render(&list, keep) }.map_err(|e| format!("{path}: {e}"))?;
        if twice {
            let again = render(&list, None).map_err(|e| format!("{path}: {e}"))?;
            if again.hashes != first.hashes {
                return Err(format!("{path}: rendering twice gave different hashes"));
            }
        }
        println!("{}", line(&list, &first.hashes));
        for &how in &blocks {
            let out = render_blocks(&list, how).map_err(|e| format!("{path}: {e}"))?;
            if out != first.hashes[0] {
                return Err(format!("{path}: rendered {how:?} frames a call, the speaker output is {out}, not {}", first.hashes[0]));
            }
        }
        if let Some(out) = &wav_to {
            std::fs::write(out, wav(&first.kept, list.rate)).map_err(|e| format!("{out}: {e}"))?;
        }
    }
    Ok(())
}

/// How two renders differ: the largest difference between samples, where it
/// is, where the first difference is, and the difference as a level under
/// the first render's own.
fn compare_wavs(first_path: &str, second_path: &str) -> Result<(), String> {
    let load = |path: &str| -> Result<(Vec<f32>, u32), String> {
        read_wav(&std::fs::read(path).map_err(|e| format!("{path}: {e}"))?).map_err(|e| format!("{path}: {e}"))
    };
    let ((was, rate), (is, other_rate)) = (load(first_path)?, load(second_path)?);
    if was.len() != is.len() || rate != other_rate {
        return Err(format!(
            "not the same render: {} samples at {rate} Hz against {} at {other_rate}",
            was.len(),
            is.len()
        ));
    }
    let (mut first, mut worst, mut largest) = (None, 0usize, 0.0f64);
    let (mut energy_diff, mut energy, mut count) = (0.0f64, 0.0f64, 0usize);
    for (at, (before, after)) in was.iter().zip(&is).enumerate() {
        let before64 = f64::from(*before);
        energy += before64 * before64;
        if before.to_bits() == after.to_bits() {
            continue;
        }
        count += 1;
        first.get_or_insert(at);
        let gap = (before64 - f64::from(*after)).abs();
        energy_diff += gap * gap;
        if gap > largest {
            (largest, worst) = (gap, at);
        }
    }
    // A frame number in a message for a person: never near 2^52.
    #[allow(clippy::cast_precision_loss)]
    let place = |at: usize| format!("frame {at} ({:.3} s)", at as f64 / f64::from(rate));
    match first {
        None => println!("identical: {} samples", was.len()),
        Some(first) => {
            let level = if energy_diff > 0.0 && energy > 0.0 { 10.0 * (energy_diff / energy).log10() } else { f64::NEG_INFINITY };
            println!(
                "differ: {count} of {} samples; first at {}; largest {largest:.3e} at {}; difference {level:.1} dB under the signal",
                was.len(),
                place(first),
                place(worst)
            );
        }
    }
    Ok(())
}

/// One block alone, as the test kit plays it (`core/src/blocks/kit.rs`):
/// `NAME/PROBE` is a line of `core/golden/blocks.txt`. Prints the same line's
/// name and hash (at 48 kHz, the kit's), and writes the samples with
/// `--wav`, so two versions of a block can be heard side by side and
/// compared (`--compare`).
fn solo_block(name: &str, rate: f64, wav_to: Option<&str>) -> Result<(), String> {
    struct Find<'a> {
        name: &'a str,
        cx: Context<'a>,
        out: Option<Vec<f32>>,
    }
    impl Find<'_> {
        fn play<B: Block + Default>(&mut self, block: &str, probe: &str, settings: &B::Settings) {
            if self.name == format!("{block}/{probe}") {
                let mut b = Box::new(B::default());
                let mut out = Vec::new();
                solo(&mut *b, settings, &self.cx, 12.0, |s| out.extend_from_slice(s));
                self.out = Some(out);
            }
        }
    }
    impl Visit for Find<'_> {
        fn voice<V: Voice + Default>(&mut self, block: &'static str) {
            for probe in V::PROBES {
                let mut note: Note = probe.settings;
                V::settle(&mut note);
                self.play::<V>(block, probe.name, &note);
            }
        }
        fn part<B: Block + Default>(&mut self, block: &'static str) {
            for probe in B::PROBES {
                self.play::<B>(block, probe.name, &probe.settings);
            }
        }
    }
    let stack = std::mem::size_of::<Core>() * 4 + (16 << 20);
    let samples = std::thread::scope(|scope| {
        std::thread::Builder::new()
            .stack_size(stack)
            .spawn_scoped(scope, || {
                let mut core = Box::new(Core::new());
                core.start(rate, 1).map_err(|e| e.message().to_string())?;
                let mut find = Find { name, cx: core.context(), out: None };
                each(&mut find);
                find.out.ok_or_else(|| format!("no block and probe called {name} (see core/golden/blocks.txt)"))
            })
            .map_err(|e| format!("could not start the render thread: {e}"))?
            .join()
            .map_err(|_| "the render thread panicked".to_string())?
    })?;
    let mut hash = Fnv::new();
    hash.samples(&samples);
    println!("{name} {}", hash.hex());
    if let Some(out) = wav_to {
        std::fs::write(out, wav(&samples, rate)).map_err(|e| format!("{out}: {e}"))?;
    }
    Ok(())
}
