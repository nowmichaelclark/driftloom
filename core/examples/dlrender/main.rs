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
//! cargo run --release --example dlrender -- --seed N[,N...] [--vibes ID,...]
//!     [--request TEXT] [--seconds 45] [--rate 44100] [--stereo] [--lite] [--wander 1]
//!     [--wav OUT.wav] [--calls OUT.calls]
//!                                           a loop from its seed on the
//!                                           player (queue item 82), no note
//!                                           list: one hash line a seed;
//!                                           --request asks as the C
//!                                           host's dl_player_ask does
//!                                           ("glacial=0.7,bpm=70-85")
//! cargo run --release --example dlrender -- --corpus SEED:N [the same]
//!     [--wav DIR] [--calls DIR]             the first N loops of corpus SEED
//!                                           (Rng(SEED).seed32() a loop), each
//!                                           to DIR/I-NAME.wav with --wav
//! cargo run --release --example dlrender -- --compare A.wav B.wav
//! cargo run --release --example dlrender -- --block NAME/PROBE [--rate HZ] [--wav OUT.wav]
//!                                           one building block alone, as the
//!                                           test kit plays it (queue item 46):
//!                                           its hash, and its samples for an
//!                                           A/B between two versions;
//!                                           NAME/PROBE~SETTING with that
//!                                           setting driven, NAME/PROBE:stereo
//!                                           a stereo block's two sides
//!                                           (queue item 46b), NAME/PROBE:wide
//!                                           a stereo processor's, fed two
//!                                           different sides (queue item 64)
//! ```
//!
//! Everything it prints is plain ASCII. It exits with 1 on any error.

mod clip;
mod notelist;
mod seed;

use std::process::ExitCode;

use dlcore::block::{Block, Context, Note, Play, Voice, solo};
use dlcore::blocks::{Visit, each};
use dlcore::Core;
use notelist::{Blocks, Fnv, line, parse, read_wav, render, render_blocks, render_flushing, signals, wav, wav_channels};

const USAGE: &str = "usage: dlrender [--twice] [--flush] [--stereo] [--rate HZ] [--blocks N,N,random] [--wav OUT.wav [--signal NAME]] LIST...\n       dlrender --seed N[,N...] | --corpus SEED:N [--vibes ID,...] [--request TEXT] [--seconds S] [--rate HZ] [--stereo] [--lite] [--wander A] [--wav OUT] [--calls OUT]\n         [--pick I] [--choices] [--from S] [--force CHOICE=OPTION] [--set NAME=VALUE] [--sketch] [--pin BLOCK.SETTING=V] (the last four: --features sketch)\n       dlrender --compare A.wav B.wav\n       dlrender --block NAME/PROBE [--rate HZ] [--wav OUT.wav]";

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
    let mut layout = 1;
    let (mut compare, mut lists, mut block) = (false, Vec::new(), None);
    let (mut rate, mut blocks): (Option<f64>, Vec<Blocks>) = (None, Vec::new());
    let mut by_seed = BySeed::default();
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        if by_seed.take(&arg, &mut args)? {
            continue;
        }
        match arg.as_str() {
            "--twice" => twice = true,
            "--stereo" => layout = 2,
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
    if !by_seed.seeds.is_empty() {
        let mut b = by_seed;
        // `--pick I` keeps the corpus's loop I and renders nothing else.
        if let Some(i) = b.pick {
            b.seeds.retain(|(at, _)| *at == Some(i));
            if b.seeds.is_empty() {
                return Err(format!("--pick {i}: the corpus has no such loop"));
            }
        }
        let way = seed::Way { seconds: b.seconds.unwrap_or(45.0), rate: rate.unwrap_or(44_100.0), full: !b.lite, layout, wander: b.wander.unwrap_or(1.0), vibes: b.vibes, request: b.request, clip: b.clip, choices: b.choices };
        if b.costs {
            return seed_costs(&b.seeds, &way);
        }
        return seed_renders(&b.seeds, &way, wav_to.as_deref(), b.calls_to.as_deref());
    }
    if lists.is_empty() || (wav_to.is_some() && lists.len() != 1) {
        return Err(USAGE.to_string());
    }
    // In stereo `out` is both sides, written as a stereo file.
    let names = signals(layout);
    let keep = match &wav_to {
        Some(_) if layout == 2 && signal == "out" => Some(0),
        Some(_) => Some(
            names
                .iter()
                .position(|s| *s == signal)
                .ok_or_else(|| format!("no such signal: {signal} (one of {})", names.join(", ")))?,
        ),
        None => None,
    };
    for path in &lists {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
        let mut list = parse(&text).map_err(|e| format!("{path}: {e}"))?;
        list.layout = layout;
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
            let want = first.hashes.get(..out.len()).unwrap_or(&[]);
            if out != want {
                return Err(format!("{path}: rendered {how:?} frames a call, the speaker output is {}, not {}", out.join(" "), want.join(" ")));
            }
        }
        if let Some(out) = &wav_to {
            let bytes = if layout == 2 && keep == Some(0) { wav_channels(&first.kept, list.rate, 2) } else { wav(&first.kept, list.rate) };
            std::fs::write(out, bytes).map_err(|e| format!("{out}: {e}"))?;
        }
    }
    Ok(())
}

/// The options of a loop from its seed (queue item 82): the seeds (each
/// with its place in a corpus, if from one), and how to play them.
#[derive(Default)]
struct BySeed {
    seeds: Vec<(Option<usize>, u32)>,
    calls_to: Option<String>,
    costs: bool,
    lite: bool,
    seconds: Option<f64>,
    wander: Option<f64>,
    vibes: Vec<String>,
    request: Option<String>,
    clip: clip::Clip,
    pick: Option<usize>,
    choices: bool,
}

impl BySeed {
    /// Take `arg` (and its value, from `args`) if it is one of these
    /// options: true if it was.
    fn take(&mut self, arg: &str, args: &mut impl Iterator<Item = String>) -> Result<bool, String> {
        let number = |word: Option<String>| -> Result<f64, String> { word.ok_or(USAGE)?.parse::<f64>().map_err(|_| USAGE.to_string()) };
        if self.clip.take(arg, args)? {
            return Ok(true);
        }
        match arg {
            "--seed" => {
                for word in args.next().ok_or(USAGE)?.split(',') {
                    self.seeds.push((None, word.parse::<u32>().map_err(|_| format!("not a seed: {word}"))?));
                }
            }
            "--corpus" => {
                let word = args.next().ok_or(USAGE)?;
                let (from, n) = word.split_once(':').ok_or_else(|| format!("--corpus is SEED:N, not {word}"))?;
                let (from, n) = (from.parse::<u32>().map_err(|_| USAGE)?, n.parse::<usize>().map_err(|_| USAGE)?);
                self.seeds.extend(seed::corpus(from, n).into_iter().enumerate().map(|(i, s)| (Some(i), s)));
            }
            "--pick" => self.pick = Some(args.next().ok_or(USAGE)?.parse::<usize>().map_err(|_| USAGE)?),
            "--choices" => self.choices = true,
            "--calls" => self.calls_to = Some(args.next().ok_or(USAGE)?),
            "--costs" => self.costs = true,
            "--lite" => self.lite = true,
            "--seconds" => self.seconds = Some(number(args.next())?),
            "--wander" => self.wander = Some(number(args.next())?),
            "--vibes" => self.vibes = args.next().ok_or(USAGE)?.split(',').map(str::to_string).collect(),
            "--request" => self.request = Some(args.next().ok_or(USAGE)?),
            _ => return Ok(false),
        }
        Ok(true)
    }
}

/// Loops from their seeds (`seed.rs`): a hash line each, and with `--wav`
/// and `--calls` their samples and the player's calls, to a file for one
/// seed, or into a directory for a corpus (`I-NAME.wav`,
/// `I-NAME.QUALITY.LAYOUT.calls`, as `tools/player.mjs --calls` names the
/// page's).
fn seed_renders(seeds: &[(Option<usize>, u32)], way: &seed::Way, wav_to: Option<&str>, calls_to: Option<&str>) -> Result<(), String> {
    let corpus = seeds.iter().any(|(i, _)| i.is_some());
    if !corpus && seeds.len() > 1 && (wav_to.is_some() || calls_to.is_some()) {
        return Err("--wav and --calls take one --seed, or a --corpus and a directory".to_string());
    }
    for dir in [wav_to, calls_to].into_iter().flatten().filter(|_| corpus) {
        std::fs::create_dir_all(dir).map_err(|e| format!("{dir}: {e}"))?;
    }
    for &(index, s) in seeds {
        let r = seed::render_seed(s, way, calls_to.is_some())?;
        let name = r.line.split(' ').find_map(|w| w.strip_prefix("name=")).unwrap_or("loop").to_string();
        let file = |dir: &str, ext: &str| match index {
            Some(i) => format!("{dir}/{i}-{name}{ext}"),
            None => dir.to_string(),
        };
        match index {
            Some(i) => println!("{i} {}", r.line),
            None => println!("{}", r.line),
        }
        if let Some(dir) = wav_to {
            let path = file(dir, ".wav");
            std::fs::write(&path, seed::wav_of(&r, way)).map_err(|e| format!("{path}: {e}"))?;
        }
        if let (Some(dir), Some(calls)) = (calls_to, &r.calls) {
            let kind = format!(".{}.{}.calls", if way.full { "full" } else { "lite" }, if way.layout == 2 { "stereo" } else { "mono" });
            let path = file(dir, &kind);
            std::fs::write(&path, calls).map_err(|e| format!("{path}: {e}"))?;
        }
    }
    Ok(())
}

/// What each loop costs to play (`--costs`, `seed::cost_seed`), a line each
/// in microseconds and against a block's deadline, and the worst of all.
fn seed_costs(seeds: &[(Option<usize>, u32)], way: &seed::Way) -> Result<(), String> {
    let deadline = f64::from(u32::try_from(dlcore::QUANTUM).unwrap_or(128)) / way.rate;
    let us = |s: f64| s * 1e6;
    let pct = |s: f64| 100.0 * s / deadline;
    let (mut worst, mut worst_gen, mut worst_audio, mut gen_share, mut longest): (f64, f64, f64, f64, usize) = (0.0, 0.0, 0.0, 0.0, 0);
    println!("# the deadline: a block of {} frames at {} Hz, {:.0} us; times are each block's least over 3 runs", dlcore::QUANTUM, way.rate, us(deadline));
    for &(index, s) in seeds {
        let c = seed::cost_seed(s, way, 3)?;
        let all = c.generating + c.deciding + c.audio;
        println!(
            "{}seed={s} load={} blocks; worst block {:.0} us ({:.1}%): load stage {:.0} ({}), pass {:.0}, steps {:.0}, audio {:.0}; over {:.0} s: generating {:.2}%, deciding {:.2}%, audio {:.1}% of the time it played",
            index.map_or(String::new(), |i| format!("{i} ")),
            c.load_blocks,
            us(c.worst_block),
            pct(c.worst_block),
            us(c.worst_load),
            c.worst_stage,
            us(c.worst_pass),
            us(c.worst_steps),
            us(c.worst_audio),
            c.seconds,
            100.0 * c.generating / c.seconds,
            100.0 * c.deciding / c.seconds,
            100.0 * c.audio / c.seconds,
        );
        worst = worst.max(c.worst_block);
        worst_gen = worst_gen.max(c.worst_load.max(c.worst_pass));
        worst_audio = worst_audio.max(c.worst_audio);
        gen_share = gen_share.max(c.generating / all);
        longest = longest.max(c.load_blocks);
    }
    println!(
        "worst block {:.0} us ({:.1}% of the deadline); worst generation in a block {:.0} us; worst audio block {:.0} us; generation's largest share of a render {:.2}%; longest load {longest} blocks",
        us(worst),
        pct(worst),
        us(worst_gen),
        us(worst_audio),
        100.0 * gen_share
    );
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
        /// The samples (a stereo row's interleaved) and how many channels.
        out: Option<(Vec<f32>, u16)>,
    }
    impl Find<'_> {
        fn play<B: Block + Default>(&mut self, block: &str, probe: &str, settings: &B::Settings) {
            let row = format!("{block}/{probe}");
            let mut ways = vec![(row.clone(), Play::Mono)];
            ways.extend(B::MODS.iter().enumerate().map(|(n, m)| (format!("{row}~{}", m.name), Play::Driven(n))));
            ways.extend(B::HEARS.iter().map(|&kind| (format!("{row}@{}", dlcore::block::event::name(kind)), Play::Heard(kind))));
            ways.extend(B::SETTINGS.iter().enumerate().filter(|(_, s)| s.probe.is_finite()).map(|(n, s)| (format!("{row}={}", s.name), Play::Set(n))));
            if B::STEREO {
                ways.push((format!("{row}:stereo"), Play::Stereo));
                if B::INPUT {
                    ways.push((format!("{row}:wide"), Play::Wide));
                }
            }
            for (name, play) in ways {
                if self.name == name {
                    let mut b = Box::new(B::default());
                    let mut out = Vec::new();
                    solo(&mut *b, settings, &self.cx, 12.0, play, |s| out.extend_from_slice(s));
                    self.out = Some((out, if matches!(play, Play::Stereo | Play::Wide) { 2 } else { 1 }));
                }
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
    let (samples, channels) = std::thread::scope(|scope| {
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
        let bytes = if channels == 1 { wav(&samples, rate) } else { wav_channels(&samples, rate, channels) };
        std::fs::write(out, bytes).map_err(|e| format!("{out}: {e}"))?;
    }
    Ok(())
}
