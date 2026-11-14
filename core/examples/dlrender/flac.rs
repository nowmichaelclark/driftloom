//! `dlrender --flac` (queue item 104a): a render written as FLAC by the
//! core's own encoder (`dlcore::flac`), as an export in the app will write
//! one, from a seed or a note list.
//!
//! From a seed it streams, as an export must: the player renders a block of
//! the encoder's size at a time, the block is encoded and its bytes written,
//! and nothing of the audio is kept, so a three-hour file takes the memory
//! a minute does (the line's `peak=`, the process's own high-water mark, says
//! so). At the end the finished header is written over the first, as a host
//! that seeks does. `--wav` beside it keeps the float samples too, for a WAV
//! of the same render (`tools/flac.mjs` nulls the decoded file against it);
//! only then does the memory grow with the length.
//!
//! Each file prints one line: the seed's hash line, as `--seed` prints it
//! (so a streamed render is seen to be the render), then the file, its
//! bytes against a WAV of the same depth, and the encoder's own time.

// Frame counts and rates are bounded by the render's length.
#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]

use std::fmt::Write as _;
use std::io::{Seek, SeekFrom, Write};
use std::time::{Duration, Instant};

use dlcore::Core;
use dlcore::flac::{BLOCK, Flac};
use dlcore::player::Player;

use crate::notelist::{Fnv, wav, wav_channels};
use crate::seed::Way;

/// A FLAC file being written: the file, the encoder, and the time the
/// encoder took.
struct Out {
    file: std::io::BufWriter<std::fs::File>,
    flac: Box<Flac>,
    path: String,
    bytes: u64,
    encoding: Duration,
}

impl Out {
    fn create(path: &str, rate: f64, channels: u32, bits: u32) -> Result<Out, String> {
        if rate.fract() != 0.0 {
            return Err(format!("--flac: the rate must be whole Hz, not {rate}"));
        }
        let mut flac = Box::new(Flac::new());
        let head = flac.start(rate as u32, channels, bits).map_err(|e| format!("--flac: {e}"))?.to_vec();
        let file = std::fs::File::create(path).map_err(|e| format!("{path}: {e}"))?;
        let mut out = Out { file: std::io::BufWriter::new(file), flac, path: path.to_string(), bytes: 0, encoding: Duration::ZERO };
        out.put(&head)?;
        Ok(out)
    }

    fn put(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.bytes += bytes.len() as u64;
        self.file.write_all(bytes).map_err(|e| format!("{}: {e}", self.path))
    }

    /// Encode the next frames (any number: a block of the encoder's at a
    /// time).
    fn write(&mut self, planes: &[&[f32]]) -> Result<(), String> {
        let frames = planes.first().map_or(0, |p| p.len());
        for from in (0..frames).step_by(BLOCK) {
            let to = (from + BLOCK).min(frames);
            let runs: Vec<&[f32]> = planes.iter().map(|p| &p[from..to]).collect();
            let t = Instant::now();
            let bytes = self.flac.write(&runs).map_err(|e| format!("--flac: {e}"))?.to_vec();
            self.encoding += t.elapsed();
            self.put(&bytes)?;
        }
        Ok(())
    }

    /// The last frame, then the finished header over the first; the line's
    /// words for the file.
    fn finish(mut self, frames: u64, rate: f64, channels: u32, bits: u32) -> Result<String, String> {
        let t = Instant::now();
        let last = self.flac.finish().map_err(|e| format!("--flac: {e}"))?.to_vec();
        self.encoding += t.elapsed();
        self.put(&last)?;
        let head = self.flac.header();
        self.file.seek(SeekFrom::Start(0)).map_err(|e| format!("{}: {e}", self.path))?;
        self.file.write_all(&head).map_err(|e| format!("{}: {e}", self.path))?;
        self.file.flush().map_err(|e| format!("{}: {e}", self.path))?;
        let wav_bytes = 44 + frames * u64::from(channels) * u64::from(bits / 8);
        let seconds = frames as f64 / rate;
        let encode = self.encoding.as_secs_f64();
        let md5: String = head[26..42].iter().fold(String::new(), |mut s, b| {
            let _ = write!(s, "{b:02x}");
            s
        });
        Ok(format!(
            "flac={} bits={bits} bytes={} wav={wav_bytes} ratio={:.3} md5={md5} encode={encode:.3}s speed={:.0}x",
            self.path,
            self.bytes,
            self.bytes as f64 / wav_bytes as f64,
            if encode > 0.0 { seconds / encode } else { f64::INFINITY }
        ))
    }
}

/// The process's peak memory (Linux's `VmHWM`), in KB, where it is known.
fn peak_kb() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    let line = status.lines().find(|l| l.starts_with("VmHWM:"))?;
    line.split_whitespace().nth(1)?.parse().ok()
}

/// `--seed` or `--corpus` with `--flac`: each loop streamed to FLAC (a
/// file for one seed; `DIR/I-NAME.flac` for a corpus, the WAVs of `--wav`
/// beside them as `--seed` names them).
pub fn seeds(seeds: &[(Option<usize>, u32)], way: &Way, flac_to: &str, bits: u32, wav_to: Option<&str>) -> Result<(), String> {
    let corpus = seeds.iter().any(|(i, _)| i.is_some());
    if !corpus && seeds.len() > 1 {
        return Err("--flac takes one --seed, or a --corpus and a directory".to_string());
    }
    if corpus {
        for dir in std::iter::once(flac_to).chain(wav_to) {
            std::fs::create_dir_all(dir).map_err(|e| format!("{dir}: {e}"))?;
        }
    }
    for &(index, seed) in seeds {
        let stack = (std::mem::size_of::<Core>() + std::mem::size_of::<Player>()) * 4 + (16 << 20);
        let line = crate::notelist::roomy(stack, || stream(seed, index, way, flac_to, bits, wav_to))?;
        println!("{line}");
    }
    Ok(())
}

fn stream(seed: u32, index: Option<usize>, way: &Way, flac_to: &str, bits: u32, wav_to: Option<&str>) -> Result<String, String> {
    let request = way.request()?;
    let frames = (way.seconds * way.rate).ceil() as usize;
    let mut core = Box::new(Core::new());
    let mut player = Box::new(Player::new());
    core.init(way.rate);
    core.quality(way.full);
    if way.layout == 2 {
        core.set_layout(2);
    }
    player.set_wander(way.wander);
    player.set_wander_salt(way.salt);
    player.attach(&mut *core, way.rate, way.full);
    if let Some(track) = way.track {
        player.set_track(&*core, track);
        player.set_draws(1);
    }
    player.load(seed, &request);
    player.ready(&mut *core);
    let name = player.generator().spec().name.as_str().to_string();
    let path = |dir: &str, ext: &str| match index {
        Some(i) => format!("{dir}/{i}-{name}{ext}"),
        None => dir.to_string(),
    };
    let channels = way.layout;
    let mut out = Out::create(&path(flac_to, ".flac"), way.rate, channels, bits)?;
    let keep = wav_to.is_some();
    let mut kept = Vec::new();
    // The hashes `--seed` prints, made as the blocks go.
    let mut words = [0x811c_9dc5u32; 2];
    let mut hash = Fnv::new();
    let mut left = vec![0.0f32; BLOCK];
    let mut right = vec![0.0f32; BLOCK];
    let mut done = 0;
    while done < frames {
        let n = (frames - done).min(BLOCK);
        let (l, r) = (&mut left[..n], &mut right[..n]);
        let mut planes: Vec<&mut [f32]> = if channels == 2 { vec![l, r] } else { vec![l] };
        core.render_with(&mut planes, |core| {
            player.decide(core);
        })
        .map_err(|e| e.message().to_string())?;
        let planes: Vec<&[f32]> = if channels == 2 { vec![&left[..n], &right[..n]] } else { vec![&left[..n]] };
        for i in 0..n {
            for (c, p) in planes.iter().enumerate() {
                words[c] = (words[c] ^ p[i].to_bits()).wrapping_mul(0x0100_0193);
                hash.samples(&[p[i]]);
                if keep {
                    kept.push(p[i]);
                }
            }
        }
        out.write(&planes)?;
        done += n;
    }
    if player.budget_full() > 0 {
        return Err(format!("the voice budget's list ran out of room ({} holds refused)", player.budget_full()));
    }
    let mut line = format!("seed={seed} name={name} {} frames={frames}", way.describe());
    if channels == 2 {
        let _ = write!(line, " left={:08x} right={:08x}", words[0], words[1]);
    } else {
        let _ = write!(line, " out={:08x}", words[0]);
    }
    let _ = write!(line, " hash={} ", hash.hex());
    line.push_str(&out.finish(frames as u64, way.rate, channels, bits)?);
    if let Some(dir) = wav_to {
        let file = path(dir, ".wav");
        let bytes = if channels == 2 { wav_channels(&kept, way.rate, 2) } else { wav(&kept, way.rate) };
        std::fs::write(&file, bytes).map_err(|e| format!("{file}: {e}"))?;
    }
    if let Some(kb) = peak_kb() {
        let _ = write!(line, " peak={kb}KB");
    }
    Ok(line)
}

/// A note list's render (`samples`, interleaved in stereo) as FLAC.
pub fn samples(path: &str, samples: &[f32], rate: f64, channels: u32, bits: u32) -> Result<String, String> {
    let frames = samples.len() / channels as usize;
    let mut out = Out::create(path, rate, channels, bits)?;
    let lanes: Vec<Vec<f32>> = (0..channels as usize).map(|c| samples.iter().skip(c).step_by(channels as usize).copied().collect()).collect();
    let planes: Vec<&[f32]> = lanes.iter().map(Vec::as_slice).collect();
    out.write(&planes)?;
    out.finish(frames as u64, rate, channels, bits)
}
