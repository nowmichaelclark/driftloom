//! Note lists: everything the core needs to render a loop with no
//! JavaScript, and a host that renders one (queue item 42).
//!
//! A note list is plain ASCII text, one fact per line, because it is
//! committed beside the hashes it produces and a diff of it should be
//! readable. The page's host (`js/core.js`) makes the same few calls to the
//! core whatever it plays -- `param`, `damp`, `note`, and the sung notes'
//! draws before a note -- so a note list is those calls, in the order the
//! synth made them, and nothing else. `tools/golden-export.mjs` writes
//! them; this file reads them. Every number is written the way JavaScript
//! prints a double (the shortest text that reads back exactly) and read
//! back exactly, so a list is bit for bit what the host sent.
//!
//! ```text
//! dlnotes 1                    the format and its version, first line
//! # ...                        a comment; so are blank lines
//! name tide-1                  what the list is called
//! rate 48000                   the sample rate, Hz
//! quality full                 the mix: full (six combs) or lite (three)
//! noise-seed 12345             the noise the voices play (see `noise`)
//! frames 1536000               how many frames to render and hash
//! param ID OP VALUE TIME TAU   Core::param
//! damp CHANNEL TIME            Core::damp
//! draws N D0 D1 ...            the next note's staged draws (Core::sung_draws)
//! note VOICE CHANNEL TIME MIDI DUR VEL PARTS X0 ... X11
//!                              Core::note; "nan" for a value left out
//! ```
//!
//! Events are handed to the core in file order before the first block is
//! rendered, as the measure harness hands over a whole render. A note's
//! `TIME` is on the render's clock, in seconds.

// The casts here are of values the code has just bounded: a rate checked to
// lie between 8,000 and 384,000 Hz, a block length of at most `QUANTUM`, a
// WAV's length. Spelling each as a checked conversion would bury the format
// in error handling for numbers that cannot be out of range.
#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use std::fmt::Write as _;

use dlcore::{CHANNELS, Core, EXTRA, QUANTUM, Taken};

/// One call the host made to the core.
pub enum Event {
    /// `Core::param(id, op, value, time, tau)`.
    Param { id: u32, op: u32, value: f64, time: f64, tau: f64 },
    /// `Core::damp(channel, time)`.
    Damp { channel: u32, time: f64 },
    /// `Core::note(...)`, with the draws staged for it, if it is a sung note.
    Note {
        voice: u32,
        channel: u32,
        time: f64,
        midi: f64,
        dur: f64,
        vel: f64,
        parts: u32,
        extra: [f64; EXTRA],
        draws: Vec<f64>,
    },
}

/// A loop to render: the header and the calls, in order.
pub struct List {
    pub name: String,
    pub rate: f64,
    pub full: bool,
    pub noise_seed: u32,
    pub frames: u64,
    pub events: Vec<Event>,
}

/// The signals a render hashes, in the order the hash line gives them: what
/// the speakers get, then each channel as it went into the mix, then what
/// reached the bus compressor. The channels' order is the core's.
pub const SIGNALS: [&str; 7] = ["out", "melody", "chords", "bass", "texture", "drums", "bus"];

fn number(word: &str, what: &str, line: usize) -> Result<f64, String> {
    word.parse::<f64>()
        .map_err(|_| format!("line {line}: {what} is not a number: {word}"))
}

fn whole(word: &str, what: &str, line: usize) -> Result<u32, String> {
    word.parse::<u32>()
        .map_err(|_| format!("line {line}: {what} is not a whole number: {word}"))
}

/// Read a note list. The error says which line is wrong, in plain ASCII.
// One arm to a kind of line, so the function reads as the format does;
// splitting it would scatter the format over helpers.
#[allow(clippy::too_many_lines)]
pub fn parse(text: &str) -> Result<List, String> {
    let mut list = List {
        name: String::new(),
        rate: 0.0,
        full: true,
        noise_seed: 0,
        frames: 0,
        events: Vec::new(),
    };
    let mut seen_header = false;
    let mut draws: Vec<f64> = Vec::new();
    for (n, raw) in text.lines().enumerate() {
        let line = n + 1;
        let raw = raw.trim();
        if raw.is_empty() || raw.starts_with('#') {
            continue;
        }
        let words: Vec<&str> = raw.split_whitespace().collect();
        let (kind, args) = (words[0], &words[1..]);
        if !seen_header {
            if kind != "dlnotes" || args != ["1"] {
                return Err(format!("line {line}: a note list starts with: dlnotes 1"));
            }
            seen_header = true;
            continue;
        }
        let want = |count: usize| -> Result<(), String> {
            if args.len() == count {
                Ok(())
            } else {
                Err(format!("line {line}: {kind} takes {count} values, not {}", args.len()))
            }
        };
        match kind {
            "name" => {
                want(1)?;
                list.name = args[0].to_string();
            }
            "rate" => {
                want(1)?;
                list.rate = number(args[0], "the rate", line)?;
            }
            "quality" => {
                want(1)?;
                list.full = match args[0] {
                    "full" => true,
                    "lite" => false,
                    other => return Err(format!("line {line}: quality is full or lite, not {other}")),
                };
            }
            "noise-seed" => {
                want(1)?;
                list.noise_seed = whole(args[0], "the noise seed", line)?;
            }
            "frames" => {
                want(1)?;
                list.frames = u64::from(whole(args[0], "the frame count", line)?);
            }
            "param" => {
                want(5)?;
                list.events.push(Event::Param {
                    id: whole(args[0], "the parameter", line)?,
                    op: whole(args[1], "the call", line)?,
                    value: number(args[2], "the value", line)?,
                    time: number(args[3], "the time", line)?,
                    tau: number(args[4], "the time constant", line)?,
                });
            }
            "damp" => {
                want(2)?;
                list.events.push(Event::Damp {
                    channel: whole(args[0], "the channel", line)?,
                    time: number(args[1], "the time", line)?,
                });
            }
            "draws" => {
                let count = args
                    .first()
                    .ok_or_else(|| format!("line {line}: draws takes a count and the draws"))
                    .and_then(|w| whole(w, "the count", line))? as usize;
                if args.len() != count + 1 {
                    return Err(format!("line {line}: draws says {count} and gives {}", args.len() - 1));
                }
                draws = args[1..]
                    .iter()
                    .map(|w| number(w, "a draw", line))
                    .collect::<Result<_, _>>()?;
            }
            "note" => {
                want(7 + EXTRA)?;
                let mut extra = [0.0; EXTRA];
                for (slot, word) in extra.iter_mut().zip(&args[7..]) {
                    *slot = number(word, "an extra value", line)?;
                }
                list.events.push(Event::Note {
                    voice: whole(args[0], "the voice", line)?,
                    channel: whole(args[1], "the channel", line)?,
                    time: number(args[2], "the time", line)?,
                    midi: number(args[3], "the pitch", line)?,
                    dur: number(args[4], "the length", line)?,
                    vel: number(args[5], "the velocity", line)?,
                    parts: whole(args[6], "the parts", line)?,
                    extra,
                    draws: std::mem::take(&mut draws),
                });
            }
            other => return Err(format!("line {line}: no such line: {other}")),
        }
    }
    if !seen_header {
        return Err("empty: a note list starts with: dlnotes 1".to_string());
    }
    if !(list.rate >= 8000.0 && list.rate <= 384_000.0) {
        return Err("the rate is missing or not between 8000 and 384000 Hz".to_string());
    }
    if list.frames == 0 {
        return Err("the frame count is missing".to_string());
    }
    if !draws.is_empty() {
        return Err("draws at the end of the list belong to no note".to_string());
    }
    Ok(list)
}

/// The synth's noise (`Synth._makeNoise`), made from a seed: two seconds of
/// white noise from `mulberry32` (`js/rng.js`), lightly pinked. The synth
/// draws it from `Math.random`, so the page cannot hand it over twice the
/// same; the exporter seeds `Math.random` with `mulberry32(noise-seed)`
/// before the synth is built, so its noise is exactly this (and checks it).
/// Item 44 moves the noise into the core, and this goes.
pub fn noise(seed: u32, rate: f64) -> Vec<f32> {
    let mut state = seed;
    let mut next = move || {
        state = state.wrapping_add(0x6d2b_79f5);
        let mut t = (state ^ (state >> 15)).wrapping_mul(1 | state);
        t = t.wrapping_add((t ^ (t >> 7)).wrapping_mul(0x3d | t)) ^ t;
        f64::from(t ^ (t >> 14)) / 4_294_967_296.0
    };
    let len = (rate * 2.0).floor() as usize;
    let (mut b0, mut b1, mut b2) = (0.0f64, 0.0f64, 0.0f64);
    (0..len)
        .map(|_| {
            let white = next() * 2.0 - 1.0;
            b0 = 0.99765 * b0 + white * 0.099;
            b1 = 0.963 * b1 + white * 0.293;
            b2 = 0.57 * b2 + white * 1.0526;
            ((b0 + b1 + b2 + white * 0.1848) * 0.22) as f32
        })
        .collect()
}

/// FNV-1a, 64 bits, over bytes: the hash a render is kept by. It is not a
/// secure hash and need not be; it has to be short, the same on every
/// machine and written in a few lines in any language (the page's tools
/// have their own, `tools/golden-lib.mjs`, and a test pins both to the same
/// published answers). Samples go in as their IEEE-754 bit patterns, four
/// bytes each, least significant first, whatever the machine's byte order.
pub struct Fnv(u64);

impl Fnv {
    pub const fn new() -> Self {
        Fnv(0xcbf2_9ce4_8422_2325)
    }
    pub fn bytes(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0 ^ u64::from(b)).wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    pub fn samples(&mut self, samples: &[f32]) {
        for s in samples {
            self.bytes(&s.to_bits().to_le_bytes());
        }
    }
    pub fn hex(&self) -> String {
        format!("{:016x}", self.0)
    }
}

/// What a render gives back.
pub struct Rendered {
    /// One hash per signal, in `SIGNALS` order.
    pub hashes: Vec<String>,
    /// The signal asked to be kept, `frames` samples of it.
    pub kept: Vec<f32>,
}

/// Render a list through a fresh core and hash every signal. `keep` is the
/// index in `SIGNALS` of one signal whose samples to return as well.
pub fn render(list: &List, keep: Option<usize>) -> Result<Rendered, String> {
    // The core is megabytes of zeros: it is built on a thread whose stack
    // can hold it (a debug build copies it a few times on the way), and
    // lives on the heap.
    let stack = std::mem::size_of::<Core>() * 4 + (16 << 20);
    std::thread::scope(|scope| {
        std::thread::Builder::new()
            .stack_size(stack)
            .spawn_scoped(scope, || {
                let mut core = Box::new(Core::new());
                play(&mut core, list, keep)
            })
            .map_err(|e| format!("could not start the render thread: {e}"))?
            .join()
            .map_err(|_| "the render thread panicked".to_string())?
    })
}

fn play(core: &mut Core, list: &List, keep: Option<usize>) -> Result<Rendered, String> {
    core.init(list.rate);
    core.quality(list.full);
    if !core.mixing() {
        return Err(format!("the core does not mix at {} Hz", list.rate));
    }
    let noise = noise(list.noise_seed, list.rate);
    let space = core.noise(noise.len());
    if space.len() != noise.len() {
        return Err(format!("the core cannot hold {} samples of noise", noise.len()));
    }
    space.copy_from_slice(&noise);
    for (n, event) in list.events.iter().enumerate() {
        match event {
            Event::Param { id, op, value, time, tau } => {
                if !core.param(*id, *op, *value, *time, *tau) {
                    return Err(format!("event {}: the core refused parameter {id}", n + 1));
                }
            }
            Event::Damp { channel, time } => core.damp(*channel, *time),
            Event::Note { voice, channel, time, midi, dur, vel, parts, extra, draws } => {
                if !draws.is_empty() {
                    let room = core.sung_draws(draws.len());
                    if room.len() != draws.len() {
                        return Err(format!("event {}: no room for {} draws", n + 1, draws.len()));
                    }
                    room.copy_from_slice(draws);
                }
                let taken = core.note(*voice, *channel, *time, *midi, *dur, *vel, *parts, *extra);
                if taken != Taken::OnTime {
                    return Err(format!("event {}: the core took the note as {taken:?}, not on time", n + 1));
                }
            }
        }
    }
    let mut hashes: Vec<Fnv> = SIGNALS.iter().map(|_| Fnv::new()).collect();
    let mut kept = Vec::new();
    let mut frame = 0u64;
    while frame < list.frames {
        let taps = *core.process(frame);
        let mut blocks: [&[f32]; 7] = [&[]; 7];
        blocks[0] = &core.output()[0];
        for (slot, tap) in blocks[1..=CHANNELS].iter_mut().zip(taps.iter()) {
            *slot = &tap[..];
        }
        blocks[CHANNELS + 1] = &core.bus()[0];
        // The last block may run past the end; the end is the list's.
        let used = (list.frames - frame).min(QUANTUM as u64) as usize;
        for (i, block) in blocks.iter().enumerate() {
            let block = block.get(..used).unwrap_or(&[]);
            hashes[i].samples(block);
            if keep == Some(i) {
                kept.extend_from_slice(block);
            }
        }
        frame += QUANTUM as u64;
    }
    // A render with notes dropped or late is not a golden: it would not
    // be the same on a machine that is faster or slower.
    if core.dropped() > 0 || core.late() > 0 || core.missed() > 0 {
        return Err(format!(
            "the core dropped {}, took {} late and missed {} notes",
            core.dropped(),
            core.late(),
            core.missed()
        ));
    }
    Ok(Rendered { hashes: hashes.iter().map(Fnv::hex).collect(), kept })
}

/// The hash line for a render: the list's name, its length and each
/// signal's hash.
pub fn line(list: &List, hashes: &[String]) -> String {
    let mut line = format!("{} frames={}", list.name, list.frames);
    for (signal, hash) in SIGNALS.iter().zip(hashes) {
        let _ = write!(line, " {signal}={hash}");
    }
    line
}

/// A mono 32-bit float WAV: the format every tool reads, and exact (a
/// 16-bit file would round the samples a comparison is after).
pub fn wav(samples: &[f32], rate: f64) -> Vec<u8> {
    let data = samples.len() * 4;
    let rate = rate as u32;
    let mut out = Vec::with_capacity(44 + data);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data as u32).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&3u16.to_le_bytes()); // IEEE float
    out.extend_from_slice(&1u16.to_le_bytes()); // mono
    out.extend_from_slice(&rate.to_le_bytes());
    out.extend_from_slice(&(rate * 4).to_le_bytes());
    out.extend_from_slice(&4u16.to_le_bytes());
    out.extend_from_slice(&32u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&(data as u32).to_le_bytes());
    for s in samples {
        out.extend_from_slice(&s.to_bits().to_le_bytes());
    }
    out
}

/// The samples of a WAV that `wav` wrote (and the rate), or why it is not
/// one.
pub fn read_wav(bytes: &[u8]) -> Result<(Vec<f32>, u32), String> {
    let at = |from: usize, len: usize| bytes.get(from..from + len).ok_or_else(|| "a truncated WAV".to_string());
    if at(0, 4)? != b"RIFF" || at(8, 8)? != b"WAVEfmt " || at(36, 4)? != b"data" {
        return Err("not a WAV written by dlrender".to_string());
    }
    let word = |from: usize| -> Result<u32, String> {
        Ok(u32::from_le_bytes(at(from, 4)?.try_into().map_err(|_| "a truncated WAV".to_string())?))
    };
    let (rate, data) = (word(24)?, word(40)? as usize);
    let samples = at(44, data)?
        .as_chunks::<4>()
        .0
        .iter()
        .map(|bytes| f32::from_bits(u32::from_le_bytes(*bytes)))
        .collect();
    Ok((samples, rate))
}
