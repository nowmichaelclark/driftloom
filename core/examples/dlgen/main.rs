//! `dlgen`: the Rust generator against the generation goldens (queue items
//! 79 to 82), on whatever target it is built for.
//!
//! ```text
//! cargo run --release --example dlgen -- --check golden/generation.hashes
//!     every case of the file, generated here: fails on any part that
//!     differs from its hash (every part, the load and the eight passes too)
//! cargo run --release --example dlgen -- --show SEED [REQUEST] [PART]
//!     a loop's canonical texts, to diff against
//!     `node tools/gen-golden.mjs --show CASE`; REQUEST as the file's
//!     second field writes it (`Request::parse`: "glacial,misty",
//!     "glacial=0.7,bpm=70-85", or - for none)
//! cargo run --release --example dlgen -- --map SEED [REQUEST] [--from N] [--count N]
//!     a long piece's map (queue item 102; `generator::longform`): one
//!     line a section (a line a phrase where the plan's unit is the
//!     phrase), from section N (0) for COUNT sections (a finite piece's
//!     whole round up to 200, else 60): its role, theme, key, tempo and
//!     energy, and what changed from the line before. Any section of any
//!     length is printed straight from its number
//! cargo run --release --example dlgen -- --section-cost SEED [REQUEST]
//!     what a section costs to make as the player makes it (its pass drawn
//!     and its plan heard, queue item 102b), timed at sections 1, 1,000 and
//!     10^9: the least of many makings each, in microseconds
//! ```
//!
//! Everything it prints is plain ASCII. It exits with 1 on any difference.

use std::fmt::Write as _;
use std::process::ExitCode;

use dlcore::generator::longform::{self, Join, Theme, Variation, changed};
use dlcore::generator::piece::{self, Piece};
use dlcore::generator::request::{Journey, Request};
use dlcore::generator::theory::SCALES;
use dlcore::generator::{self, Part};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("--check") => args.get(1).ok_or_else(|| "--check needs a file".to_string()).and_then(|f| check(f)),
        Some("--show") => show(&args[1..]),
        Some("--map") => map(&args[1..]),
        Some("--theme-cost") => theme_cost(&args[1..]),
        Some("--section-cost") => section_cost(&args[1..]),
        _ => Err("usage: dlgen --check FILE | --show SEED [REQUEST] [PART] | --map SEED [REQUEST] [--from N] [--count N] | --section-cost SEED [REQUEST] | --theme-cost SEED [REQUEST] [FROM]".to_string()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("dlgen: {message}");
            ExitCode::FAILURE
        }
    }
}

/// `--section-cost`: section 1, 1,000 and 10^9 made again and again
/// (`Generator::section`, the pass and its plan heard, at the app's amount),
/// the least time of each kept.
fn section_cost(args: &[String]) -> Result<(), String> {
    let seed: u32 = args.first().and_then(|s| s.parse().ok()).ok_or("--section-cost needs a seed")?;
    let asked = request(args.get(1).map_or("-", String::as_str))?;
    let mut generator = Box::new(generator::Generator::new());
    generator.load_with(seed, &asked);
    let mut line = format!("seed={seed} request={}", args.get(1).map_or("-", String::as_str));
    for n in [1u64, 1_000, 1_000_000_000] {
        let mut best = f64::INFINITY;
        for _ in 0..50 {
            let t0 = std::time::Instant::now();
            for k in 0..20 {
                std::hint::black_box(generator.section(n + k % 2, generator::wander::AMOUNT));
            }
            best = best.min(t0.elapsed().as_secs_f64() / 20.0);
        }
        let _ = write!(line, " n{n}={:.1}us", best * 1e6);
    }
    println!("{line}");
    Ok(())
}

/// `--theme-cost`: what a section of an arc costs to make by whose tune it
/// plays (session 14), from section `from` on: a theme-A section and a
/// theme-B one made again and again (the second tune already made, as the
/// player has it a section ahead), a theme-B section whose tune is made
/// from nothing each time (two movements' in turn), and the loop's own
/// load beside them. The least time of each kept.
fn theme_cost(args: &[String]) -> Result<(), String> {
    let seed: u32 = args.first().and_then(|s| s.parse().ok()).ok_or("--theme-cost needs a seed")?;
    let asked = request(args.get(1).map_or("-", String::as_str))?;
    let from: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    let mut generator = Box::new(generator::Generator::new());
    let mut load = f64::INFINITY;
    for _ in 0..5 {
        let t0 = std::time::Instant::now();
        generator.load_with(seed, &asked);
        load = load.min(t0.elapsed().as_secs_f64());
    }
    let form = generator.form().ok_or("no long piece")?;
    let first = |role: longform::Role, theme: Theme, after: u64| (after..after + 20_000).find(|&n| {
        let s = form.at(n, 0);
        s.role == role && s.theme == theme && form.units_in(n) == 1
    });
    let a = first(longform::Role::Theme, Theme::A, from).ok_or("no theme-A section (a piece planned in phrases?)")?;
    let b = first(longform::Role::Second, Theme::B, from).ok_or("no theme-B section: a piece of one theme")?;
    let other = first(longform::Role::Second, Theme::B, b + form.movement_units(b, 0)).filter(|&n| form.at(n, 0).theme_seed != form.at(b, 0).theme_seed).ok_or("no second movement with a theme B")?;
    let mut time = |sections: &[u64]| {
        let mut best = f64::INFINITY;
        for _ in 0..40 {
            let t0 = std::time::Instant::now();
            for k in 0..20 {
                std::hint::black_box(generator.section(sections[k % sections.len()], generator::wander::AMOUNT));
            }
            best = best.min(t0.elapsed().as_secs_f64() / 20.0);
        }
        best * 1e6
    };
    let (warm_a, warm_b, cold_b) = (time(&[a, a + 1]), time(&[b, b + 1]), time(&[b, other]));
    println!("seed={seed} request={} from={from} a={a} b={b} themeA={warm_a:.1}us themeB={warm_b:.1}us themeBmade={cold_b:.1}us load={:.1}us", args.get(1).map_or("-", String::as_str), load * 1e6);
    Ok(())
}

fn request(word: &str) -> Result<Request, String> {
    Request::parse(word).map_err(|why| format!("'{word}': {why}"))
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
        let asked = request(vibe_word)?;
        let t0 = std::time::Instant::now();
        generator.load_with(seed, &asked);
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
    let asked = args.get(1).map_or(Ok(Request::new()), |w| request(w))?;
    let only = args.get(2);
    let mut generator = Box::new(generator::Generator::new());
    generator.load_with(seed, &asked);
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

/// A key as a person reads it: "D dorian".
fn key_name(k: longform::Key) -> String {
    format!("{} {}", longform::NOTE_NAMES[usize::from(k.root) % 12], SCALES.get(k.scale).map_or("?", |s| s.id))
}

/// Seconds as a person reads them: "1:02:03".
fn clock(seconds: f64) -> String {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // whole seconds of a piece
    let s = seconds as u64;
    if s >= 3600 { format!("{}:{:02}:{:02}", s / 3600, s / 60 % 60, s % 60) } else { format!("{}:{:02}", s / 60, s % 60) }
}

#[allow(clippy::cast_precision_loss, clippy::too_many_lines)] // sections and bars, far below 2^53; one table, column by column
fn map(args: &[String]) -> Result<(), String> {
    let mut plain = Vec::new();
    let (mut from, mut count) = (0u64, None);
    let mut movements = false;
    let mut rest = args.iter();
    while let Some(a) = rest.next() {
        match a.as_str() {
            "--from" => from = rest.next().and_then(|x| x.parse().ok()).ok_or("--from needs a section's number")?,
            "--count" => count = Some(rest.next().and_then(|x| x.parse::<u64>().ok()).ok_or("--count needs a number")?),
            "--movements" => movements = true,
            _ => plain.push(a),
        }
    }
    let seed: u32 = plain.first().ok_or("--map needs a seed")?.parse().map_err(|_| "a bad seed")?;
    let word = plain.get(1).map_or("-", |w| w.as_str());
    let asked = request(word)?;
    let mut generator = Box::new(generator::Generator::new());
    generator.load_with(seed, &asked);
    let spec = generator.spec();
    let Some(form) = generator.form() else {
        return Err(format!("seed {seed} \"{word}\" is no long piece: ask a length past 32 bars (bars=256, bars=endless)"));
    };
    let piece = Piece::of(spec).ok_or("no piece")?;
    let bar_seconds = piece::seconds(1.0, spec.steps_per_bar, spec.bpm);
    println!(
        "# seed {seed} \"{word}\": {}, {} bpm, {} bars a section ({:.1} s), {}",
        spec.name.as_str(),
        spec.bpm,
        spec.bars,
        spec.bars * bar_seconds,
        key_name(form.home)
    );
    let sections = piece.end.map(|e| e.0);
    let length = match (sections, piece.length()) {
        (Some(n), Some(bars)) => format!("{bars} bars, {n} sections, {}", clock(bars * bar_seconds)),
        _ => "Endless".to_string(),
    };
    let how = if form.asked { "asked" } else { "drawn" };
    let journey = match form.journey {
        Journey::Arc if form.arcs() == 0 => format!("arcs of {} units ({}) in turn", form.arc_units(), clock(form.arc_units() as f64 * form.unit_seconds)),
        Journey::Arc => format!(
            "{} arc{} of about {} units ({}), the {} form, {}",
            form.arcs(),
            if form.arcs() == 1 { "" } else { "s" },
            form.arc_units(),
            clock(form.arc_units() as f64 * form.unit_seconds),
            if form.small { "small" } else { "full" },
            if form.second { "two themes" } else { "one theme" }
        ),
        Journey::Wander => format!("{} things in turn, an epoch {} units ({})", form.turns(), form.epoch_units, clock(form.epoch_units as f64 * form.unit_seconds)),
    };
    println!("# {length}; the {} ({how}): {journey}; the unit is the {} ({} bars, {:.1} s)", form.journey.name(), form.grain.name(), form.unit_bars, form.unit_seconds);
    let related: Vec<String> = form.related().map(|k| format!("{} ({})", key_name(k), k.relation.name())).collect();
    println!("# related keys: {}", if related.is_empty() { "none".to_string() } else { related.join(", ") });
    let kinds: Vec<&str> = form.cycle().iter().map(|k| k.name()).collect();
    if form.journey == Journey::Arc && kinds.len() > 1 {
        println!("# the movements' cycle: {}{}", kinds.join(", "), if sections.is_some() { ", then a last full arc" } else { ", round and round" });
    }
    if movements {
        // One line a movement: where it starts, its kind, how long it is,
        // its second theme's seed and how its theme returns.
        println!("# {:>9} {:>9} {:>8}  {:<10} {:>6} {:>8}  second theme, the return", "movement", "section", "time", "kind", "units", "lasts");
        let count = count.unwrap_or_else(|| sections.unwrap_or(2000));
        let mut last = None;
        for n in from..from.saturating_add(count) {
            let m = sections.map_or(n, |all| n % all);
            for p in 0..form.units_in(n) {
                let s = form.joined(n, p);
                if s.journey == Journey::Arc && s.role == longform::Role::Return && s.at > 0 && last == Some((s.part, false)) {
                    println!("{:>64}the return: A {}{}", "", s.variation.name(), if form.at(n, p).changed & changed::KEY != 0 { ", the key home" } else { "" });
                    last = Some((s.part, true));
                }
                if last.map(|l| l.0) == Some(s.part) {
                    continue;
                }
                last = Some((s.part, false));
                println!(
                    "{:>11} {:>9} {:>8}  {:<10} {:>6} {:>8}  {}",
                    s.part,
                    if form.units_in(n) > 1 { format!("{n}.{p}") } else { n.to_string() },
                    clock((m as f64 * spec.bars + p as f64 * form.unit_bars) * bar_seconds),
                    s.kind.name(),
                    form.movement_units(n, p),
                    clock(form.movement_units(n, p) as f64 * form.unit_seconds),
                    form.second_of(s.part).map_or("one theme".to_string(), |seed| format!("B of seed {seed}")),
                );
            }
        }
        return Ok(());
    }
    println!("# {:>9} {:>8} {:>4}  {:<11} {:>12}  {:<16} {:<18} {:>5} {:>6}  changed", "section", "time", "bars", "role", "part", "theme", "key", "bpm", "energy");
    let count = count.unwrap_or_else(|| sections.map_or(60, |n| n.min(200)));
    for n in from..from.saturating_add(count) {
        let (round, m) = sections.map_or((0, n), |all| (n / all, n % all));
        let before = round as f64 * piece.length().unwrap_or(0.0) + m as f64 * spec.bars;
        let units = form.units_in(n);
        for p in 0..units {
            let s = form.joined(n, p);
            let label = if units > 1 { format!("{n}.{p}") } else { n.to_string() };
            let theme = match s.theme {
                Theme::Rest => "-".to_string(),
                Theme::A => "A".to_string(),
                Theme::B => "B".to_string(),
                Theme::Recall(k) => format!("B of arc {k}"),
            };
            let theme = match (s.fragment, s.variation) {
                (true, Variation::Plain) => format!("{theme} fragment"),
                (true, v) => format!("{theme} fragment, {}", v.name()),
                (false, Variation::Plain) => theme,
                (false, v) => format!("{theme} {}", v.name()),
            };
            let mut state = Vec::new();
            if let Some(l) = s.state.rest {
                state.push(format!("{} rests", l.name()));
            }
            if let Some((l, pick)) = s.state.voice {
                state.push(format!("{} voice {pick:.2}", l.name()));
            }
            if s.calls {
                state.push("a call and response".to_string());
            }
            if s.prepares {
                state.push("the bass holds the home fifth".to_string());
            }
            // How it meets the unit after it (session 15).
            if !matches!(s.join, Join::None | Join::Glide | Join::Prepared) {
                state.push(format!("then {}", s.join.name()));
            }
            if s.journey == Journey::Arc && form.cycle().len() > 1 {
                state.push(s.kind.name().to_string());
            }
            if s.journey == Journey::Wander {
                state.push(format!("warmth {:+.2} space {:+.2} register {:+} length {:+} density {:.2}", s.state.warmth, s.state.space, s.state.register, s.state.length, s.state.density));
            }
            let moved: Vec<&str> = changed::NAMES.iter().enumerate().filter(|(x, _)| s.changed & (1 << x) != 0).map(|(_, name)| *name).collect();
            println!(
                "{:>11} {:>8} {:>4}  {:<11} {:>12}  {:<16} {:<18} {:>5} {:>6.2}  {}{}{}",
                label,
                clock((before + p as f64 * form.unit_bars) * bar_seconds),
                s.bars,
                s.role.name(),
                format!("{} {}/{}", s.part, s.at + 1, s.of),
                theme,
                key_name(s.key),
                s.bpm,
                s.energy,
                if moved.is_empty() { "-".to_string() } else { moved.join(", ") },
                if s.closes { "; closes on the tonic" } else { "" },
                if state.is_empty() { String::new() } else { format!("  [{}]", state.join(", ")) }
            );
        }
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
