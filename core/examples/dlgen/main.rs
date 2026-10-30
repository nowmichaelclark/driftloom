//! `dlgen`: the Rust generator against the generation goldens (queue items
//! 79 to 82), on whatever target it is built for.
//!
//! ```text
//! cargo run --release --example dlgen -- --check golden/generation.hashes
//!     every case of the file, generated here: fails on any part that
//!     differs from its hash (every part, the load and the eight passes too)
//! cargo run --release --example dlgen -- --show SEED [VIBE,...] [PART]
//!     a loop's canonical texts, to diff against
//!     `node tools/gen-golden.mjs --show CASE`
//! ```
//!
//! Everything it prints is plain ASCII. It exits with 1 on any difference.

use std::process::ExitCode;

use dlcore::generator::{self, Part};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("--check") => args.get(1).ok_or_else(|| "--check needs a file".to_string()).and_then(|f| check(f)),
        Some("--show") => show(&args[1..]),
        _ => Err("usage: dlgen --check FILE | --show SEED [VIBE,...] [PART]".to_string()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("dlgen: {message}");
            ExitCode::FAILURE
        }
    }
}

fn vibes(word: &str) -> Result<Vec<usize>, String> {
    if word == "-" {
        return Ok(Vec::new());
    }
    word.split(',').map(|id| generator::data::vibe_index(id).ok_or_else(|| format!("no vibe '{id}'"))).collect()
}

fn check(file: &str) -> Result<(), String> {
    let text = std::fs::read_to_string(file).map_err(|e| format!("{file}: {e}"))?;
    let mut generator = Box::new(generator::Generator::new());
    let mut cases = 0;
    let mut bad = 0;
    let mut counts = vec![0usize; Part::ALL.len()];
    let mut lost = 0;
    let mut spent = std::time::Duration::ZERO;
    let mut worst = (std::time::Duration::ZERO, String::new());
    for line in text.lines().filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let fields: Vec<&str> = line.split(' ').collect();
        let [name, seed, vibe_word, ..] = fields[..] else { return Err(format!("a bad line: {line}")) };
        let seed: u32 = seed.parse().map_err(|_| format!("a bad seed: {line}"))?;
        let picked = vibes(vibe_word)?;
        let t0 = std::time::Instant::now();
        generator.load(seed, &picked);
        let took = t0.elapsed();
        spent += took;
        if took > worst.0 {
            worst = (took, name.to_string());
        }
        cases += 1;
        lost += generator.lost();
        let mut moved = Vec::new();
        for (i, part) in Part::ALL.iter().enumerate() {
            let Some(want) = part.in_line(line) else { continue };
            let got = format!("{:016x}", generator.hash(*part));
            if got == want {
                counts[i] += 1;
            } else {
                moved.push(label(*part));
            }
        }
        if !moved.is_empty() {
            bad += 1;
            if bad <= 40 {
                println!("  differs {name}: {}", moved.join(", "));
            }
        }
    }
    if lost > 0 {
        return Err(format!("the generator's lists ran out of room ({lost} items lost)"));
    }
    let parts: Vec<String> = Part::ALL.iter().zip(&counts).map(|(p, n)| format!("{} {n}", label(*p))).collect();
    println!("dlgen: {} of {cases} cases equal {file} ({})", cases - bad, parts.join(", "));
    println!(
        "dlgen: a loop takes {:.0} us on average, {:.0} us at most ({}); the generator's working memory is {} bytes",
        spent.as_secs_f64() * 1e6 / f64::from(u32::try_from(cases.max(1)).unwrap_or(1)),
        worst.0.as_secs_f64() * 1e6,
        worst.1,
        std::mem::size_of::<generator::Generator>()
    );
    if bad > 0 { Err(format!("{bad} cases differ")) } else { Ok(()) }
}

fn show(args: &[String]) -> Result<(), String> {
    let seed: u32 = args.first().ok_or("--show needs a seed")?.parse().map_err(|_| "a bad seed")?;
    let picked = args.get(1).map_or(Ok(Vec::new()), |w| vibes(w))?;
    let only = args.get(2);
    let mut generator = Box::new(generator::Generator::new());
    generator.load(seed, &picked);
    for part in Part::ALL {
        if only.is_some_and(|o| o != part.name()) {
            continue;
        }
        let mut text = Text(String::new());
        generator.write(part, &mut text);
        let text = text.0;
        // As `node tools/gen-golden.mjs --show` names them.
        let name = match part {
            Part::Pass(n) => format!("passes[{}]", n.saturating_sub(1)),
            _ => part.name().to_string(),
        };
        println!("{name} {:016x}\n{text}\n", generator.hash(part));
    }
    Ok(())
}

/// A part as a person reads it: a pass by its number.
fn label(part: Part) -> String {
    match part {
        Part::Pass(n) => format!("pass {n}"),
        _ => part.name().to_string(),
    }
}

struct Text(String);

impl generator::canon::Sink for Text {
    fn bytes(&mut self, b: &[u8]) {
        self.0.push_str(&String::from_utf8_lossy(b));
    }
}
