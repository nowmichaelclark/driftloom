//! The test kit (queue item 46): every block in the registry, played alone
//! and checked, with no test written for any one of them.
//!
//! For each of a block's probes (`Block::PROBES`) the kit renders the block
//! alone at 48 kHz (`block::solo`: a source until its span ends, a
//! processor fed one second of the core's noise), twice, and checks:
//!
//! - **level:** every sample finite, not silent, and not running away (its
//!   peak under `CEILING`); RMS and peak are reported in dBFS;
//! - **tone:** the share of its A-weighted energy between 2 and 5 kHz, as
//!   `tools/measure.mjs` reckons it (the coffee-shop test), reported;
//! - **determinism:** the second render is the first, bit for bit;
//! - **cost:** its render time as a share of the audio's own length, which
//!   in a release build must stay under `COST_LIMIT` (a debug build only
//!   reports it: unoptimized code says nothing about the real cost);
//! - **null:** its hash (FNV-1a 64 over the samples, as the goldens'),
//!   level and tone against the line kept for it in `core/golden/blocks.txt`.
//!   An equal hash proves the render identical to the stored one; any other
//!   is a sound change, reported with the stored level and tone beside the
//!   new, for an A/B (`dlrender --block NAME/PROBE --wav` renders either
//!   version, and `--compare` gives the residual).
//!
//! A block added to the registry has no line yet, and the kit says so; a
//! block whose sound changes on purpose, likewise. Either way, store the new
//! renders with
//!
//! ```sh
//! KIT=update cargo test --lib kit
//! ```
//!
//! and say in the PR what moved and why. `KIT=report` prints the whole
//! table; `cargo test --release --lib kit` checks the costs too.

// Sample counts and frequencies as floats: far below 2^52.
#![allow(clippy::cast_precision_loss)]

use std::fmt::Write as _;
use std::time::Instant;

use crate::block::{Block, Context, Note, Probe, Voice, solo};
use crate::Core;

use super::{Visit, each};

/// The rate the kit plays at, in Hz.
const RATE: f64 = 48_000.0;
/// The longest a source probe renders, in seconds: past any voice's tail.
const LONGEST: f64 = 12.0;
/// The most a probe may cost in a release build, as a share of real time:
/// one note or one block of processing taking a fiftieth of the audio's
/// length would leave room for only fifty at once.
const COST_LIMIT: f64 = 0.02;
/// The loudest a block may peak on its own, in dBFS. Not full scale: a raw
/// oscillator sits at it and the noise above it, and a voice may pass it
/// before the mix and the master's ceiling (the frame drum does, at +1 dB).
/// Past this, a block is running away.
const CEILING: f64 = 6.0;
/// Where the stored renders are kept.
const STORED: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/golden/blocks.txt");

/// One probe, rendered and measured.
struct Row {
    /// `block/probe`.
    name: String,
    hash: u64,
    /// RMS and peak, in dBFS.
    rms: f64,
    peak: f64,
    /// The 2-5 kHz share of the A-weighted energy, in percent.
    tone: f64,
    /// Render time over the audio's length.
    cost: f64,
    /// What is wrong with it, beside the null test.
    faults: Vec<String>,
}

impl Row {
    /// Its line in the stored file: what the null test keeps.
    fn line(&self) -> String {
        format!("{} {:016x} {:.2} {:.2} {:.1}", self.name, self.hash, self.rms, self.peak, self.tone)
    }
}

/// The kit as it walks the registry.
struct Kit<'a> {
    cx: Context<'a>,
    /// Only this block, if named.
    only: Option<&'static str>,
    rows: Vec<Row>,
}

impl Visit for Kit<'_> {
    fn voice<V: Voice + Default>(&mut self, name: &'static str) {
        if self.only.is_some_and(|only| only != name) {
            return;
        }
        for probe in V::PROBES {
            // As `Core::note` settles a note when it arrives.
            let mut note: Note = probe.settings;
            V::settle(&mut note);
            self.play::<V>(name, &Probe { name: probe.name, settings: note });
        }
    }

    fn part<B: Block + Default>(&mut self, name: &'static str) {
        if self.only.is_some_and(|only| only != name) {
            return;
        }
        for probe in B::PROBES {
            self.play::<B>(name, probe);
        }
    }
}

impl Kit<'_> {
    fn play<B: Block + Default>(&mut self, block: &str, probe: &Probe<B::Settings>) {
        let render = || {
            let mut b = Box::new(B::default());
            let mut out = Vec::new();
            solo(&mut *b, &probe.settings, &self.cx, LONGEST, |s| out.extend_from_slice(s));
            out
        };
        // The cost is the quickest of three renders: the first pays for
        // cold caches, and any one can be interrupted.
        let mut took = f64::MAX;
        let mut timed = || {
            let begun = Instant::now();
            let out = render();
            took = took.min(begun.elapsed().as_secs_f64());
            out
        };
        let first = timed();
        let again = timed();
        timed();
        let mut faults = Vec::new();
        let bits = |v: &[f32]| v.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
        if bits(&first) != bits(&again) {
            faults.push("not deterministic: a second render differs".to_owned());
        }
        if first.iter().any(|x| !x.is_finite()) {
            faults.push("a sample is not finite".to_owned());
        }
        let (rms, peak) = level(&first);
        if rms < -90.0 {
            faults.push(format!("silent: {rms:.1} dBFS RMS"));
        }
        if peak >= CEILING {
            faults.push(format!("runs away: peak {peak:.2} dBFS"));
        }
        let seconds = first.len() as f64 / RATE;
        let cost = took / seconds.max(1e-9);
        if !cfg!(debug_assertions) && cost > COST_LIMIT {
            faults.push(format!("costs {:.2}% of real time, over {:.0}%", cost * 100.0, COST_LIMIT * 100.0));
        }
        self.rows.push(Row {
            name: format!("{block}/{}", probe.name),
            hash: fnv(&first),
            rms,
            peak,
            tone: tone(&first) * 100.0,
            cost,
            faults,
        });
    }
}

/// FNV-1a, 64 bits, over the samples' IEEE-754 bit patterns, four bytes
/// each, least significant first: the goldens' hash
/// (`examples/dlrender/notelist.rs`).
fn fnv(samples: &[f32]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for x in samples {
        for byte in x.to_bits().to_le_bytes() {
            h ^= u64::from(byte);
            h = h.wrapping_mul(0x0100_0000_01b3);
        }
    }
    h
}

/// RMS and peak, in dBFS (-200 for silence).
fn level(samples: &[f32]) -> (f64, f64) {
    let db = |x: f64| if x > 0.0 { 20.0 * x.log10() } else { -200.0 };
    let sum: f64 = samples.iter().map(|&x| f64::from(x) * f64::from(x)).sum();
    let peak = samples.iter().fold(0.0f64, |m, &x| m.max(f64::from(x).abs()));
    (db((sum / samples.len().max(1) as f64).sqrt()), db(peak))
}

/// The share of the A-weighted energy between 2 and 5 kHz: periodograms of
/// 4,096 frames under a Hann window, half overlapping, summed across the
/// whole render (`presenceOf` in `tools/measure.mjs`).
fn tone(samples: &[f32]) -> f64 {
    const N: usize = 4096;
    let mut power = vec![0.0f64; N / 2];
    let mut padded = samples.to_vec();
    padded.resize(samples.len().max(N), 0.0);
    let mut start = 0;
    while start + N <= padded.len() {
        let mut re: Vec<f64> = (0..N)
            .map(|i| f64::from(padded[start + i]) * (0.5 - 0.5 * (std::f64::consts::TAU * i as f64 / N as f64).cos()))
            .collect();
        let mut im = vec![0.0; N];
        fft(&mut re, &mut im);
        for (i, p) in power.iter_mut().enumerate() {
            *p += re[i] * re[i] + im[i] * im[i];
        }
        start += N / 2;
    }
    let (mut band, mut total) = (0.0, 0.0);
    for (i, p) in power.iter().enumerate().skip(1) {
        let f = i as f64 * RATE / N as f64;
        if f > 20_000.0 {
            break;
        }
        let w = a_weight(f);
        total += p * w * w;
        if (2000.0..=5000.0).contains(&f) {
            band += p * w * w;
        }
    }
    if total > 0.0 { band / total } else { 0.0 }
}

/// IEC 61672 A-weighting, as a linear magnitude factor.
fn a_weight(f: f64) -> f64 {
    let f2 = f * f;
    let num = 12194.0f64.powi(2) * f2 * f2;
    let den = (f2 + 20.6f64.powi(2)) * ((f2 + 107.7f64.powi(2)) * (f2 + 737.9f64.powi(2))).sqrt() * (f2 + 12194.0f64.powi(2));
    num / den * 10.0f64.powf(2.0 / 20.0)
}

/// Radix-2 FFT, in place.
fn fft(re: &mut [f64], im: &mut [f64]) {
    let n = re.len();
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            re.swap(i, j);
            im.swap(i, j);
        }
    }
    let mut len = 2;
    while len <= n {
        let angle = -std::f64::consts::TAU / len as f64;
        let (wr, wi) = (angle.cos(), angle.sin());
        for at in (0..n).step_by(len) {
            let (mut cr, mut ci) = (1.0, 0.0);
            for k in 0..len / 2 {
                let (a, b) = (at + k, at + k + len / 2);
                let vr = re[b] * cr - im[b] * ci;
                let vi = re[b] * ci + im[b] * cr;
                re[b] = re[a] - vr;
                im[b] = im[a] - vi;
                re[a] += vr;
                im[a] += vi;
                let next = cr * wr - ci * wi;
                ci = cr * wi + ci * wr;
                cr = next;
            }
        }
        len <<= 1;
    }
}

/// Every block in the registry (or only the one named), through the kit.
fn run(only: Option<&'static str>) -> Vec<Row> {
    std::thread::Builder::new()
        .stack_size(1 << 26)
        .spawn(move || {
            let mut core = Box::new(Core::new());
            core.init(RATE);
            let mut kit = Kit { cx: core.context(), only, rows: Vec::new() };
            each(&mut kit);
            kit.rows
        })
        .and_then(|t| t.join().map_err(|_| std::io::Error::other("the kit panicked")))
        .expect("the kit runs")
}

#[test]
fn every_block_passes_its_kit() {
    let rows = run(None);
    let mode = std::env::var("KIT").unwrap_or_default();
    if mode == "update" {
        let mut file = String::from(
            "# The test kit's stored renders (core/src/blocks/kit.rs): one line a probe,\n\
             # block/probe, FNV-1a 64 of its samples, RMS and peak in dBFS, 2-5 kHz share in %.\n\
             # Rewritten by `KIT=update cargo test --lib kit`; a change here is a sound change.\n",
        );
        for row in &rows {
            let _ = writeln!(file, "{}", row.line());
        }
        std::fs::write(STORED, file).expect("the stored renders are written");
    }
    let stored = std::fs::read_to_string(STORED).unwrap_or_default();
    let kept: Vec<(&str, &str)> = stored
        .lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .filter_map(|l| l.split_once(' '))
        .collect();
    let mut faults = Vec::new();
    let mut table = String::from("block/probe                    rms dB  peak dB  2-5 kHz   cost\n");
    for row in &rows {
        let _ = writeln!(
            table,
            "{:<28} {:>8.2} {:>8.2} {:>7.1}% {:>6.3}%",
            row.name,
            row.rms,
            row.peak,
            row.tone,
            row.cost * 100.0
        );
        for f in &row.faults {
            faults.push(format!("{}: {f}", row.name));
        }
        let line = row.line();
        let now = line.split_once(' ').map_or("", |(_, rest)| rest);
        match kept.iter().find(|(name, _)| *name == row.name) {
            None => faults.push(format!("{}: no stored render (a new block or probe; store it with KIT=update)", row.name)),
            Some((_, was)) if *was != now => faults.push(format!(
                "{}: a sound change, not proven identical: stored {was}, now {now} (hash, RMS, peak, 2-5 kHz); \
                 an A/B is due, and KIT=update stores it",
                row.name
            )),
            Some(_) => {}
        }
    }
    for (name, _) in &kept {
        if !rows.iter().any(|r| r.name == *name) {
            faults.push(format!("{name}: stored, but no block plays it now (KIT=update forgets it)"));
        }
    }
    if mode == "report" || !faults.is_empty() {
        println!("{table}");
    }
    assert!(faults.is_empty(), "the kit:\n  {}", faults.join("\n  "));
    assert!(rows.len() > 60, "every block's probes: {} rows", rows.len());
}

/// A new block is one new file plus one line in the registry (the charter's
/// measure, promise 5). The demonstration block, the tremolo, is named in
/// exactly those two places in the whole core -- its own file, and one line
/// of `blocks/mod.rs` -- and the kit plays it all the same.
#[test]
fn the_demonstration_block_is_one_file_and_one_line() {
    let src = concat!(env!("CARGO_MANIFEST_DIR"), "/src");
    let mut mentions = Vec::new();
    let mut dirs = vec![std::path::PathBuf::from(src)];
    while let Some(dir) = dirs.pop() {
        for entry in std::fs::read_dir(&dir).expect("the source") {
            let path = entry.expect("an entry").path();
            if path.is_dir() {
                dirs.push(path);
                continue;
            }
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name == "tremolo.rs" || name == "kit.rs" {
                continue;
            }
            let text = std::fs::read_to_string(&path).unwrap_or_default();
            for line in text.lines().filter(|l| l.to_lowercase().contains("tremolo")) {
                mentions.push(format!("{name}: {}", line.trim()));
            }
        }
    }
    assert_eq!(mentions, vec!["mod.rs: #[cfg(test)] tremolo::Tremolo;".to_owned()], "where the tremolo is named");
    let rows = run(Some("tremolo"));
    for probe in ["tremolo/slow", "tremolo/deep"] {
        let row = rows.iter().find(|r| r.name == probe).expect("the kit plays the tremolo");
        assert!(row.faults.is_empty(), "{probe}: {:?}", row.faults);
    }
}
