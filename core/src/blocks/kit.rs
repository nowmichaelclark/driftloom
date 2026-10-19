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
//! And, from what a block declares (queue item 46b):
//!
//! - **modulation:** for each setting in its `MODS`, each probe again with
//!   that setting driven by a 3 Hz sine of the setting's `swing`
//!   (`block::drive`), checked as above and nulled as `block/probe~setting`;
//!   a setting that changes nothing on any probe is not wired, and fails;
//! - **stereo:** a mono block in a stereo signal (`Block::render_stereo`'s
//!   defaults: a source centered, a processor fed the same on both sides,
//!   and a processor as a `Pair`) must put its mono render on each side,
//!   bit for bit, so stereo costs mono nothing in sound; a block with a
//!   stereo render of its own (`STEREO`) is checked as above on both sides,
//!   nulled as `block/probe:stereo`, and must put different sides out on
//!   at least one probe, or it is not stereo.
//!
//! A block added to the registry has no line yet, and the kit says so; a
//! block whose sound changes on purpose, likewise. Either way, store the new
//! renders (lines already stored keep their place; new ones go at the end)
//! with
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

use crate::block::{Block, Context, Note, Pair, Play, Probe, Voice, solo};
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
    /// A stereo row whose two sides differ.
    apart: bool,
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
        // As `Core::note` settles a note when it arrives.
        let probes: Vec<Probe<Note>> = V::PROBES
            .iter()
            .map(|probe| {
                let mut note: Note = probe.settings;
                V::settle(&mut note);
                Probe { name: probe.name, settings: note }
            })
            .collect();
        self.check::<V>(name, &probes);
    }

    fn part<B: Block + Default>(&mut self, name: &'static str) {
        if self.only.is_some_and(|only| only != name) {
            return;
        }
        self.check::<B>(name, B::PROBES);
    }
}

/// One probe's render: its samples (a stereo one's interleaved, left
/// first), and the quickest of the renders timed, in seconds.
struct Render {
    samples: Vec<f32>,
    took: f64,
}

/// Play `settings` through a fresh `B` the way `play` says, `times` times
/// (at least once), and keep the first render with the quickest time: the
/// first pays for cold caches, and any one can be interrupted. Faults if
/// two renders differ.
fn render<B: Block>(fresh: impl Fn() -> Box<B>, settings: &B::Settings, cx: &Context, play: Play, times: usize, faults: &mut Vec<String>) -> Render {
    let mut first: Option<Vec<f32>> = None;
    let mut took = f64::MAX;
    for _ in 0..times.max(1) {
        let mut b = fresh();
        let mut out = Vec::new();
        let begun = Instant::now();
        solo(&mut *b, settings, cx, LONGEST, play, |s| out.extend_from_slice(s));
        took = took.min(begun.elapsed().as_secs_f64());
        match &first {
            None => first = Some(out),
            Some(was) if bits(was) != bits(&out) => faults.push("not deterministic: a second render differs".to_owned()),
            Some(_) => {}
        }
    }
    Render { samples: first.unwrap_or_default(), took }
}

/// Samples as their bit patterns, so NaN equals itself.
fn bits(v: &[f32]) -> Vec<u32> {
    v.iter().map(|x| x.to_bits()).collect()
}

/// A stereo render's two sides, from its interleaved samples.
fn sides(v: &[f32]) -> (Vec<f32>, Vec<f32>) {
    (v.iter().step_by(2).copied().collect(), v.iter().skip(1).step_by(2).copied().collect())
}

impl Kit<'_> {
    /// Every check the kit runs on one block, from what it declares.
    fn check<B: Block + Default>(&mut self, block: &str, probes: &[Probe<B::Settings>]) {
        let fresh = || Box::new(B::default());
        let mut plain = Vec::new();
        for probe in probes {
            let row = self.row(block, probe.name, "", &probe.settings, Play::Mono, fresh);
            plain.push(row);
        }
        // Each setting a signal can drive, driven on every probe.
        for (n, setting) in B::MODS.iter().enumerate() {
            let mut heard = false;
            let mut first = None;
            for (probe, &was) in probes.iter().zip(plain.iter()) {
                let suffix = format!("~{}", setting.name);
                let row = self.row(block, probe.name, &suffix, &probe.settings, Play::Driven(n), fresh);
                heard |= self.rows.get(row).is_some_and(|r| r.hash != self.rows.get(was).map_or(0, |w| w.hash));
                first.get_or_insert(row);
            }
            if !heard && let Some(r) = first.and_then(|i| self.rows.get_mut(i)) {
                r.faults.push(format!("its setting {} changes nothing on any probe: not wired", setting.name));
            }
        }
        // Each kind of event it hears, and each setting a host sets, on
        // every probe.
        let ways = B::HEARS
            .iter()
            .map(|&kind| (format!("@{}", crate::block::event::name(kind)), Play::Heard(kind), format!("it hears {}s", crate::block::event::name(kind))))
            .chain(B::SETTINGS.iter().enumerate().map(|(n, s)| (format!("={}", s.name), Play::Set(n), format!("its setting {}", s.name))));
        for (suffix, play, what) in ways {
            let mut heard = false;
            let mut first = None;
            for (probe, &was) in probes.iter().zip(plain.iter()) {
                let row = self.row(block, probe.name, &suffix, &probe.settings, play, fresh);
                heard |= self.rows.get(row).is_some_and(|r| r.hash != self.rows.get(was).map_or(0, |w| w.hash));
                first.get_or_insert(row);
            }
            if !heard && let Some(r) = first.and_then(|i| self.rows.get_mut(i)) {
                r.faults.push(format!("{what}, but that changes nothing on any probe: not wired"));
            }
        }
        if B::STEREO {
            // A stereo block: its stereo render, checked and stored.
            let mut apart = false;
            let mut first = None;
            for probe in probes {
                let row = self.row(block, probe.name, ":stereo", &probe.settings, Play::Stereo, fresh);
                apart |= self.rows.get(row).is_some_and(|r| r.apart);
                first.get_or_insert(row);
            }
            if !apart && let Some(r) = first.and_then(|i| self.rows.get_mut(i)) {
                r.faults.push("both sides are the same on every probe: not stereo".to_owned());
            }
        } else if let (Some(probe), Some(&at)) = (probes.first(), plain.first()) {
            // A mono block in a stereo signal puts its mono render on each
            // side, bit for bit: as it is, and a processor as a pair too.
            let mut faults = Vec::new();
            let mono = render(fresh, &probe.settings, &self.cx, Play::Mono, 1, &mut faults).samples;
            let guard = |what: &str, stereo: Vec<f32>, faults: &mut Vec<String>| {
                let (l, r) = sides(&stereo);
                if bits(&l) != bits(&mono) || bits(&r) != bits(&mono) {
                    faults.push(format!("in a stereo signal{what}, a side is not its mono render"));
                }
            };
            let stereo = render(fresh, &probe.settings, &self.cx, Play::Stereo, 1, &mut faults).samples;
            guard("", stereo, &mut faults);
            if B::INPUT {
                let pair = || Box::new(Pair(B::default(), B::default()));
                let stereo = render(pair, &probe.settings, &self.cx, Play::Stereo, 1, &mut faults).samples;
                guard(" as a pair", stereo, &mut faults);
            }
            if let Some(r) = self.rows.get_mut(at) {
                r.faults.extend(faults);
            }
        }
    }

    /// Render one probe as `play` says, measure it, and keep its row: its
    /// place among the rows.
    fn row<B: Block>(
        &mut self,
        block: &str,
        probe: &str,
        suffix: &str,
        settings: &B::Settings,
        play: Play,
        fresh: impl Fn() -> Box<B>,
    ) -> usize {
        let mut faults = Vec::new();
        // The plain render is timed three times for its cost; the others
        // are rendered twice, for determinism.
        let times = if play == Play::Mono { 3 } else { 2 };
        let Render { samples, took } = render(fresh, settings, &self.cx, play, times, &mut faults);
        let (mid, apart) = if play == Play::Stereo {
            let (l, r) = sides(&samples);
            let mid = l.iter().zip(r.iter()).map(|(a, b)| (a + b) * 0.5).collect();
            (mid, bits(&l) != bits(&r))
        } else {
            (samples.clone(), false)
        };
        if samples.iter().any(|x| !x.is_finite()) {
            faults.push("a sample is not finite".to_owned());
        }
        let (rms, peak) = level(&samples);
        if rms < -90.0 {
            faults.push(format!("silent: {rms:.1} dBFS RMS"));
        }
        if peak >= CEILING {
            faults.push(format!("runs away: peak {peak:.2} dBFS"));
        }
        let seconds = mid.len() as f64 / RATE;
        let cost = took / seconds.max(1e-9);
        if play == Play::Mono && !cfg!(debug_assertions) && cost > COST_LIMIT {
            faults.push(format!("costs {:.2}% of real time, over {:.0}%", cost * 100.0, COST_LIMIT * 100.0));
        }
        self.rows.push(Row {
            name: format!("{block}/{probe}{suffix}"),
            hash: fnv(&samples),
            rms,
            peak,
            tone: tone(&mid) * 100.0,
            cost,
            apart,
            faults,
        });
        self.rows.len() - 1
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
        // Lines already stored keep their place; new ones go at the end.
        let was = std::fs::read_to_string(STORED).unwrap_or_default();
        let mut file = String::from(
            "# The test kit's stored renders (core/src/blocks/kit.rs): one line a probe,\n\
             # block/probe, FNV-1a 64 of its samples, RMS and peak in dBFS, 2-5 kHz share in %.\n\
             # Rewritten by `KIT=update cargo test --lib kit`; a change here is a sound change.\n",
        );
        let kept: Vec<&str> = was
            .lines()
            .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            .filter_map(|l| l.split_once(' ').map(|(name, _)| name))
            .collect();
        for name in &kept {
            if let Some(row) = rows.iter().find(|r| r.name == *name) {
                let _ = writeln!(file, "{}", row.line());
            }
        }
        for row in rows.iter().filter(|r| !kept.contains(&r.name.as_str())) {
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
    let mut table = String::from("block/probe                          rms dB  peak dB  2-5 kHz   cost\n");
    for row in &rows {
        let _ = writeln!(
            table,
            "{:<34} {:>8.2} {:>8.2} {:>7.1}% {:>6.3}%",
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

/// Where a block's module is named in the core's source, outside its own
/// file and the kit: every line with `NAME::`, `NAME.rs` or `mod NAME` at a
/// word's start.
fn named(name: &str) -> Vec<String> {
    let src = concat!(env!("CARGO_MANIFEST_DIR"), "/src");
    let own = format!("{name}.rs");
    let mut mentions = Vec::new();
    let mut dirs = vec![std::path::PathBuf::from(src)];
    while let Some(dir) = dirs.pop() {
        for entry in std::fs::read_dir(&dir).expect("the source") {
            let path = entry.expect("an entry").path();
            if path.is_dir() {
                dirs.push(path);
                continue;
            }
            let file = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if file == own || file == "kit.rs" {
                continue;
            }
            let text = std::fs::read_to_string(&path).unwrap_or_default();
            for line in text.lines() {
                let found = [format!("{name}::"), own.clone(), format!("mod {name}")].iter().any(|token| {
                    line.match_indices(token.as_str()).any(|(at, _)| {
                        !line[..at].chars().next_back().is_some_and(|c| c.is_alphanumeric() || c == '_')
                    })
                });
                if found {
                    mentions.push(format!("{file}: {}", line.trim()));
                }
            }
        }
    }
    mentions
}

/// A new block is one new file plus one line in the registry (the charter's
/// measure, promise 5). Each demonstration block -- the tremolo (item 46),
/// the amplifier whose gain a signal drives and the stereo panner (item
/// 46b), and a channel's own swell whose rate a host sets for the loop, held
/// chorale retuned chord by chord and duck keyed by another channel (item
/// 46c) -- is named in exactly those two places in the whole core, its own
/// file and one line of `blocks/mod.rs`, and the kit plays it all the same:
/// the amplifier's modulation probe, the panner's stereo render, the swell's
/// settings, the chorale's retunes and the duck's key too. (The core's own
/// tests reach the channel's ones by their numbers, `own::SWELL` and the
/// rest, which the registry line declares.)
#[test]
fn each_demonstration_block_is_one_file_and_one_line() {
    for (name, rest, probes) in [
        ("tremolo", "", &["tremolo/slow", "tremolo/deep"][..]),
        ("amp", "", &["amp/half", "amp/half~gain"][..]),
        ("pan", "", &["pan/left", "pan/right", "pan/left:stereo", "pan/right:stereo"][..]),
        ("swell", " = SWELL (swell::REST)", &["swell/slow", "swell/slow=rate", "swell/slow=depth"][..]),
        ("chorale", " = CHORALE (chorale::REST)", &["chorale/triad", "chorale/triad@retune"][..]),
        ("duck", " = DUCK (duck::REST)", &["duck/half", "duck/half~key"][..]),
    ] {
        let line = format!("mod.rs: #[cfg(test)] {name}::{}{}{rest};", name[..1].to_uppercase(), &name[1..]);
        assert_eq!(named(name), vec![line], "where the {name} is named");
        let rows = run(Some(name));
        for probe in probes {
            let row = rows.iter().find(|r| r.name == *probe).unwrap_or_else(|| panic!("the kit plays {probe}"));
            assert!(row.faults.is_empty(), "{probe}: {:?}", row.faults);
        }
    }
}
