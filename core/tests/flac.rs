//! The FLAC writer (queue item 104a): what it writes decodes to exactly the
//! samples it was given, for every shape of stream it takes, whatever the
//! host's call sizes, and the header it finishes says what the stream is.
//!
//! The decoder here is written from the format's specification (RFC 9639)
//! and shares no code with the encoder: it checks both CRCs of every frame
//! itself. `tools/flac.mjs` holds real renders to the reference decoder
//! (`flac`, or `ffmpeg`), and checks the MD5 there.

// A decoder's arithmetic: fields read as wide numbers and narrowed to what
// they hold, and the specification's one-letter names (n, k, q, u, x).
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::many_single_char_names
)]

use dlcore::flac::{BLOCK, Flac, HEADER, MAX_BYTES, Refused, quantize};

/// A stream as a decoder reads it.
struct Decoded {
    rate: u32,
    channels: usize,
    bits: u32,
    total: u64,
    smallest: u32,
    largest: u32,
    md5: [u8; 16],
    /// Interleaved.
    samples: Vec<i32>,
    frames: usize,
}

struct Reader<'a> {
    bytes: &'a [u8],
    bit: usize,
}

impl Reader<'_> {
    fn bits(&mut self, n: u32) -> u64 {
        let mut v = 0;
        for _ in 0..n {
            let byte = self.bytes[self.bit / 8];
            v = (v << 1) | u64::from((byte >> (7 - self.bit % 8)) & 1);
            self.bit += 1;
        }
        v
    }
    fn signed(&mut self, n: u32) -> i64 {
        let v = self.bits(n);
        if n > 0 && v >> (n - 1) == 1 { v as i64 - (1i64 << n) } else { v as i64 }
    }
    fn unary(&mut self) -> u64 {
        let mut q = 0;
        while self.bits(1) == 0 {
            q += 1;
        }
        q
    }
    fn align(&mut self) {
        self.bit = self.bit.div_ceil(8) * 8;
    }
    fn byte_at(&self) -> usize {
        assert_eq!(self.bit % 8, 0);
        self.bit / 8
    }
}

/// CRC-8 and CRC-16 bit by bit, as the specification states them.
fn crc(bytes: &[u8], width: u32, poly: u32) -> u32 {
    let top = 1u32 << (width - 1);
    let mask = (1u64 << width) as u32 - 1;
    let mut c = 0u32;
    for &b in bytes {
        c ^= u32::from(b) << (width - 8);
        for _ in 0..8 {
            c = if c & top != 0 { (c << 1) ^ poly } else { c << 1 } & mask;
        }
    }
    c
}

fn decode(file: &[u8]) -> Decoded {
    assert_eq!(&file[..4], b"fLaC");
    let mut r = Reader { bytes: file, bit: 32 };
    assert_eq!(r.bits(1), 1, "STREAMINFO is the last metadata block");
    assert_eq!(r.bits(7), 0, "STREAMINFO");
    assert_eq!(r.bits(24), 34);
    assert_eq!(r.bits(16), BLOCK as u64);
    assert_eq!(r.bits(16), BLOCK as u64);
    let smallest = r.bits(24) as u32;
    let largest = r.bits(24) as u32;
    let rate = r.bits(20) as u32;
    let channels = r.bits(3) as usize + 1;
    let bits = r.bits(5) as u32 + 1;
    let total = r.bits(36);
    let mut md5 = [0u8; 16];
    for b in &mut md5 {
        *b = r.bits(8) as u8;
    }
    assert_eq!(r.byte_at(), HEADER);
    let mut samples = Vec::new();
    let mut frames = 0;
    while r.byte_at() < file.len() {
        let start = r.byte_at();
        assert_eq!(r.bits(15), 0x7FFC, "sync, fixed block size");
        assert_eq!(r.bits(1), 0);
        let size_code = r.bits(4);
        let rate_code = r.bits(4);
        let assignment = r.bits(4);
        let size_bits = r.bits(3);
        assert_eq!(r.bits(1), 0);
        assert_eq!(size_bits, if bits == 16 { 4 } else { 6 });
        // The frame number, UTF-8 coded.
        let lead = r.bits(8);
        let ones = (lead as u8).leading_ones();
        let mut number = if ones == 0 { lead } else { lead & ((1 << (7 - ones)) - 1) };
        for _ in 1..ones {
            assert_eq!(r.bits(2), 0b10);
            number = (number << 6) | r.bits(6);
        }
        assert_eq!(number, frames as u64, "frame numbers count up");
        let n = match size_code {
            0b1100 => BLOCK,
            0b0110 => r.bits(8) as usize + 1,
            0b0111 => r.bits(16) as usize + 1,
            other => panic!("block size code {other}"),
        };
        let frame_rate = match rate_code {
            0 => rate,
            1 => 88_200,
            2 => 176_400,
            3 => 192_000,
            6 => 22_050,
            7 => 24_000,
            8 => 32_000,
            9 => 44_100,
            10 => 48_000,
            11 => 96_000,
            12 => r.bits(8) as u32 * 1000,
            13 => r.bits(16) as u32,
            14 => r.bits(16) as u32 * 10,
            other => panic!("rate code {other}"),
        };
        assert_eq!(frame_rate, rate);
        let header_end = r.byte_at();
        assert_eq!(r.bits(8) as u32, crc(&file[start..header_end], 8, 0x07), "the header's CRC-8");
        let widths: Vec<u32> = match (channels, assignment) {
            (1, 0) => vec![bits],
            (2, 1) => vec![bits, bits],
            (2, 8 | 10) => vec![bits, bits + 1],
            (2, 9) => vec![bits + 1, bits],
            other => panic!("channel assignment {other:?}"),
        };
        let subs: Vec<Vec<i64>> = widths.iter().map(|&w| subframe(&mut r, n, w)).collect();
        r.align();
        let end = r.byte_at();
        assert_eq!(r.bits(16) as u32, crc(&file[start..end], 16, 0x8005), "the frame's CRC-16");
        for i in 0..n {
            let (a, b) = (subs[0][i], subs.get(1).map_or(0, |s| s[i]));
            let pair = match assignment {
                0 => [a, 0],
                1 => [a, b],
                8 => [a, a - b],
                9 => [a + b, b],
                _ => {
                    let mid = (a << 1) | (b & 1);
                    [(mid + b) >> 1, (mid - b) >> 1]
                }
            };
            samples.extend(pair.iter().take(channels).map(|&s| i32::try_from(s).unwrap()));
        }
        frames += 1;
    }
    Decoded { rate, channels, bits, total, smallest, largest, md5, samples, frames }
}

/// The fixed predictors' coefficients, newest sample first.
const COEFFS: [&[i64]; 5] = [&[], &[1], &[2, -1], &[3, -3, 1], &[4, -6, 4, -1]];

fn subframe(r: &mut Reader, n: usize, bits: u32) -> Vec<i64> {
    assert_eq!(r.bits(1), 0);
    let kind = r.bits(6);
    assert_eq!(r.bits(1), 0, "no wasted bits");
    match kind {
        0 => vec![r.signed(bits); n],
        1 => (0..n).map(|_| r.signed(bits)).collect(),
        8..=12 => {
            let order = (kind - 8) as usize;
            let mut x: Vec<i64> = (0..order).map(|_| r.signed(bits)).collect();
            let method = r.bits(2);
            assert!(method <= 1);
            let width = if method == 0 { 4 } else { 5 };
            let partition_order = r.bits(4);
            let parts = 1usize << partition_order;
            assert_eq!(n % parts, 0);
            let mut residual = Vec::with_capacity(n);
            for p in 0..parts {
                let k = r.bits(width) as u32;
                assert_ne!(k, (1 << width) - 1, "no escapes");
                let count = n / parts - if p == 0 { order } else { 0 };
                for _ in 0..count {
                    let q = r.unary();
                    let u = (q << k) | r.bits(k);
                    residual.push(((u >> 1) as i64) ^ -((u & 1) as i64));
                }
            }
            for e in residual {
                let i = x.len();
                let predicted: i64 = COEFFS[order].iter().enumerate().map(|(j, c)| c * x[i - 1 - j]).sum();
                x.push(predicted + e);
            }
            x
        }
        other => panic!("subframe type {other}"),
    }
}

/// A deterministic signal of `frames` frames: a few tones, a little noise,
/// silences, and a stretch past full scale.
fn signal(frames: usize, channels: usize, seed: u32) -> Vec<Vec<f32>> {
    let mut s = seed;
    let mut noise = move || {
        s = s.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        f64::from(s >> 8) / f64::from(1u32 << 24) - 0.5
    };
    (0..channels)
        .map(|c| {
            (0..frames)
                .map(|i| {
                    let t = i as f64 / 48_000.0;
                    let x = match (i / 3000) % 4 {
                        0 => 0.0,
                        1 => 0.4 * (t * 440.0 * (1.0 + c as f64) * core::f64::consts::TAU).sin() + 0.01 * noise(),
                        2 => 1.3 * (t * 97.0 * core::f64::consts::TAU).sin(),
                        _ => 0.3 * noise() + 0.2 * (t * 3.0).sin(),
                    };
                    x as f32
                })
                .collect()
        })
        .collect()
}

/// Encode `planes` in calls of the sizes `chunk` gives, then patch the
/// header as a host that seeks does.
fn encode(planes: &[Vec<f32>], rate: u32, bits: u32, mut chunk: impl FnMut() -> usize) -> Vec<u8> {
    let mut flac = Box::new(Flac::new());
    let mut file = flac.start(rate, planes.len() as u32, bits).unwrap().to_vec();
    assert_eq!(file.len(), HEADER);
    let frames = planes[0].len();
    let mut at = 0;
    while at < frames {
        let n = chunk().clamp(1, BLOCK).min(frames - at);
        let runs: Vec<&[f32]> = planes.iter().map(|p| &p[at..at + n]).collect();
        let bytes = flac.write(&runs).unwrap();
        assert!(bytes.len() <= MAX_BYTES);
        file.extend_from_slice(bytes);
        at += n;
    }
    file.extend_from_slice(flac.finish().unwrap());
    assert_eq!(flac.frames(), frames as u64);
    file[..HEADER].copy_from_slice(&flac.header());
    file
}

fn check(planes: &[Vec<f32>], rate: u32, bits: u32) -> Vec<u8> {
    let file = encode(planes, rate, bits, || BLOCK);
    let d = decode(&file);
    let frames = planes[0].len();
    assert_eq!((d.rate, d.channels, d.bits, d.total), (rate, planes.len(), bits, frames as u64));
    assert_eq!(d.frames, frames.div_ceil(BLOCK));
    let want: Vec<i32> = (0..frames).flat_map(|i| planes.iter().map(move |p| quantize(p[i], bits))).collect();
    assert!(d.samples == want, "{} channels, {bits} bits at {rate} Hz, {frames} frames: decodes to other samples", planes.len());
    if frames > 0 {
        assert!(d.smallest > 0 && d.smallest <= d.largest && d.largest as usize <= MAX_BYTES);
        assert_ne!(d.md5, [0; 16]);
    }
    file
}

#[test]
fn every_shape_decodes_to_its_samples() {
    for channels in [1, 2] {
        for bits in [16, 24] {
            for frames in [0, 1, 4, 5, 255, 256, 257, 4095, 4096, 4097, 3 * BLOCK + 1000] {
                check(&signal(frames, channels, 7), 48_000, bits);
            }
        }
    }
}

#[test]
fn every_rate_code_decodes() {
    for rate in [22_050, 24_000, 32_000, 37_800, 44_100, 48_000, 64_000, 88_200, 96_000, 100_000, 176_400, 192_000, 100_001] {
        check(&signal(5000, 2, 3), rate, 16);
    }
}

#[test]
fn the_bytes_are_the_same_whatever_the_calls() {
    let planes = signal(5 * BLOCK + 77, 2, 11);
    let whole = encode(&planes, 44_100, 24, || BLOCK);
    let mut s = 5u32;
    let odd = encode(&planes, 44_100, 24, || {
        s = s.wrapping_mul(69_069).wrapping_add(1);
        (s >> 20) as usize % 700 + 1
    });
    assert_eq!(whole, odd);
    assert_eq!(encode(&planes, 44_100, 24, || 128), whole);
}

#[test]
fn silence_is_tiny_and_noise_stays_under_its_bound() {
    let silence = vec![vec![0.0f32; 10 * BLOCK]; 2];
    let file = check(&silence, 48_000, 16);
    // A few bytes a block: two constant subframes.
    assert!(file.len() < HEADER + 10 * 20, "{} bytes", file.len());
    // Full-scale noise predicts nothing: the samples go as they are, and
    // the largest frame stays inside the buffer's bound.
    let mut s = 1u32;
    let noise: Vec<Vec<f32>> = (0..2)
        .map(|_| {
            (0..3 * BLOCK)
                .map(|_| {
                    s = s.wrapping_mul(1_103_515_245).wrapping_add(12_345);
                    (f64::from(s) / f64::from(u32::MAX) * 2.0 - 1.0) as f32
                })
                .collect()
        })
        .collect();
    for bits in [16, 24] {
        let file = check(&noise, 48_000, bits);
        let d = decode(&file);
        assert!(d.largest as usize <= MAX_BYTES);
        assert!(file.len() <= HEADER + 3 * (2 * BLOCK * bits as usize / 8 + 20));
    }
}

#[test]
fn the_header_is_unknown_until_finished() {
    let mut flac = Box::new(Flac::new());
    let head = flac.start(48_000, 2, 16).unwrap().to_vec();
    assert_eq!(head, flac.header());
    // Min and max frame size, the total and the MD5 are zeros until the end.
    assert!(head[12..18].iter().all(|&b| b == 0));
    assert_eq!(head[21] & 0x0F, 0);
    assert!(head[22..42].iter().all(|&b| b == 0));
    let planes = signal(BLOCK + 10, 2, 5);
    let runs: Vec<&[f32]> = planes.iter().map(|p| &p[..BLOCK]).collect();
    assert_ne!(flac.write(&runs).unwrap().len(), 0);
    let runs: Vec<&[f32]> = planes.iter().map(|p| &p[BLOCK..]).collect();
    assert_eq!(flac.write(&runs).unwrap().len(), 0);
    assert_eq!(flac.header(), &head[..], "still unknown while open");
    assert_ne!(flac.finish().unwrap().len(), 0);
    assert_ne!(flac.header(), &head[..]);
    // A file whose first header was never patched still decodes.
    let mut file = head.clone();
    let mut f = Box::new(Flac::new());
    f.start(48_000, 2, 16).unwrap();
    let runs: Vec<&[f32]> = planes.iter().map(|p| &p[..BLOCK]).collect();
    file.extend_from_slice(f.write(&runs).unwrap());
    let runs: Vec<&[f32]> = planes.iter().map(|p| &p[BLOCK..]).collect();
    file.extend_from_slice(f.write(&runs).unwrap());
    file.extend_from_slice(f.finish().unwrap());
    let d = decode(&file);
    assert_eq!((d.total, d.md5, d.samples.len()), (0, [0; 16], 2 * (BLOCK + 10)));
}

#[test]
fn refusals_say_why() {
    let mut flac = Box::new(Flac::new());
    let x = [0.0f32; 8];
    assert_eq!(flac.write(&[&x]).unwrap_err(), Refused::NotOpen);
    assert_eq!(flac.finish().unwrap_err(), Refused::NotOpen);
    assert_eq!(flac.start(8_000, 1, 16).unwrap_err(), Refused::Rate);
    assert_eq!(flac.start(384_000, 1, 16).unwrap_err(), Refused::Rate);
    assert_eq!(flac.start(48_000, 3, 16).unwrap_err(), Refused::Channels);
    assert_eq!(flac.start(48_000, 1, 8).unwrap_err(), Refused::Bits);
    flac.start(48_000, 2, 16).unwrap();
    assert_eq!(flac.write(&[&x]).unwrap_err(), Refused::Planes);
    assert_eq!(flac.write(&[&x, &x[..4]]).unwrap_err(), Refused::Planes);
    let long = vec![0.0f32; BLOCK + 1];
    assert_eq!(flac.write(&[&long, &long]).unwrap_err(), Refused::Frames);
    flac.finish().unwrap();
    assert_eq!(flac.write(&[&x, &x]).unwrap_err(), Refused::NotOpen);
    for why in [Refused::Rate, Refused::Channels, Refused::Bits, Refused::NotOpen, Refused::Planes, Refused::Frames, Refused::TooLong, Refused::Overflow] {
        assert!(why.message().is_ascii() && !why.message().is_empty());
    }
}

#[test]
fn the_encoder_is_a_fixed_size() {
    // Its whole state, for a minute or three hours: the PR and the module's
    // doc give the figure.
    let size = size_of::<Flac>();
    assert!(size < 128 * 1024, "{size} bytes");
}
