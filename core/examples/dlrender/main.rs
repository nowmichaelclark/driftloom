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
//! cargo run --release --example dlrender -- --wav OUT.wav [--signal NAME] LIST
//! cargo run --release --example dlrender -- --compare A.wav B.wav
//! ```
//!
//! Everything it prints is plain ASCII. It exits with 1 on any error.

mod notelist;

use std::process::ExitCode;

use notelist::{SIGNALS, line, parse, read_wav, render, render_flushing, wav};

const USAGE: &str = "usage: dlrender [--twice] [--flush] [--wav OUT.wav [--signal NAME]] LIST...\n       dlrender --compare A.wav B.wav";

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
    let (mut compare, mut lists) = (false, Vec::new());
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--twice" => twice = true,
            "--flush" => flush = true,
            "--compare" => compare = true,
            "--wav" => wav_to = Some(args.next().ok_or(USAGE)?),
            "--signal" => signal = args.next().ok_or(USAGE)?,
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
        let list = parse(&text).map_err(|e| format!("{path}: {e}"))?;
        let first = if flush { render_flushing(&list, keep) } else { render(&list, keep) }.map_err(|e| format!("{path}: {e}"))?;
        if twice {
            let again = render(&list, None).map_err(|e| format!("{path}: {e}"))?;
            if again.hashes != first.hashes {
                return Err(format!("{path}: rendering twice gave different hashes"));
            }
        }
        println!("{}", line(&list, &first.hashes));
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
