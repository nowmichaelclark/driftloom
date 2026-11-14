//! FLAC, written by the core (queue item 104a; `docs/CORE.md`, phase 7,
//! "Exports, safe first"): the lossless export, in a file every player
//! opens, at about half a WAV's size, with nothing borrowed from the
//! platform.
//!
//! **Streaming.** A host starts an encoder (`Flac::start`), hands it what
//! the core rendered a block at a time (`write`, up to `BLOCK` frames a
//! call, the channels as `Core::render` fills them), and appends the bytes
//! each call gives back to its file; `finish` gives the last frame. The
//! encoder holds one FLAC frame of samples and one frame of output, never
//! the audio, so an export takes the same memory for a minute or three
//! hours: `size_of::<Flac>()`, about 120 KB (122,576 bytes on a 64-bit
//! target), all of it fixed.
//!
//! **The header, finished at the end.** The file begins with STREAMINFO,
//! which wants the total length, the smallest and largest frames and the
//! MD5 of the samples: things known only at the end. `start` writes it with
//! those left unknown (zeros, which the format allows: a decoder reads such
//! a file, it just can't check it or show its length up front), so the
//! file is a valid FLAC at every moment, and an export cancelled or cut off
//! still plays up to where it stopped. After `finish`, `header()` is the
//! same `HEADER` bytes with everything filled in, and a host that can seek
//! writes them over the file's first bytes: a write at offset 0, which a
//! browser's file handle, Android's file descriptor and every native file
//! can do. A host that can't (a pipe, a download streamed out) leaves the
//! first header as it is and loses only the check and the length shown.
//!
//! **What it codes.** Mono or stereo, 16 or 24 bits, the core's sample
//! rates (22,050 to 192,000 Hz, whole numbers). Blocks of `BLOCK` frames;
//! each channel the cheapest of a constant (a silence), the samples as they
//! are, or one of FLAC's fixed predictors with its residual in partitioned
//! Rice codes (`plan`), and stereo the cheapest of left and right, either
//! with the side, or mid and side. No LPC: a simple encoder that is correct
//! and fast, not the smallest file (queue item 104a).
//!
//! **The same bytes everywhere.** The samples become integers by one rule
//! (`quantize`: rounded to the nearest step, halves away from zero, held to
//! full scale; no dither), exact in 64-bit floats, and everything after is
//! integer arithmetic, so the same render makes the same file on every
//! target, as the render itself is the same.
//!
//! Nothing here allocates, and nothing panics: arrays are reached by `get`
//! or by iterators, and a write past the output's end (which `MAX_BYTES`'s
//! bound rules out) is refused, never made.

#![deny(missing_docs)]

mod bits;
mod md5;
mod plan;

use bits::{Writer, crc8, crc16};
use md5::Md5;
use plan::{Plan, Scratch};

/// Frames in a FLAC frame (a block): 4,096, the usual size at 44.1 and 48
/// kHz, inside the format's streamable subset (no more than 4,608 at 48 kHz
/// and below). A `write` takes at most this many.
pub const BLOCK: usize = 4096;

/// The bytes of a file's header: `fLaC` and STREAMINFO, the only metadata
/// block written. `start` gives them with the totals unknown, `header()`
/// with them known.
pub const HEADER: usize = 42;

/// `BLOCK` as the header's fields hold it.
#[allow(clippy::cast_possible_truncation)] // 4096
const BLOCK_U32: u32 = BLOCK as u32;

/// The most bytes one call gives back: one frame at its largest, a 24-bit
/// stereo block of samples as they are (a channel never costs more than its
/// samples as they are, since that is one of the choices, and the stereo
/// pair chosen never costs more than left and right apart), with its
/// header (at most 15 bytes), the byte its subframes end on, and its CRC.
pub const MAX_BYTES: usize = 2 * BLOCK * 3 + 32;

/// Why an encoder refused a call. Every message is plain ASCII.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refused {
    /// The sample rate is not a whole number of Hz the core renders at.
    Rate,
    /// Not mono (1) or stereo (2).
    Channels,
    /// Not 16 or 24 bits.
    Bits,
    /// Not started, or already finished.
    NotOpen,
    /// Not one run of samples a channel, all the same length.
    Planes,
    /// More than `BLOCK` frames in one call.
    Frames,
    /// Past FLAC's last frame number (2^31 frames, over 50 days at 192
    /// kHz).
    TooLong,
    /// A frame did not fit its buffer: never, by `MAX_BYTES`'s bound; said
    /// rather than written short.
    Overflow,
}

impl Refused {
    /// What went wrong, in plain ASCII: a host shows it to a person.
    #[must_use]
    pub const fn message(self) -> &'static str {
        match self {
            Refused::Rate => "the FLAC sample rate must be a whole number of Hz from 22050 to 192000",
            Refused::Channels => "FLAC export is mono (1 channel) or stereo (2)",
            Refused::Bits => "FLAC export is 16 or 24 bits",
            Refused::NotOpen => "the FLAC encoder is not started, or already finished",
            Refused::Planes => "FLAC samples come as one run a channel, all the same length",
            Refused::Frames => "at most 4096 frames a FLAC write",
            Refused::TooLong => "the FLAC stream is longer than the format numbers",
            Refused::Overflow => "a FLAC frame ran out of room (a bug in the encoder)",
        }
    }
}

impl core::fmt::Display for Refused {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.message())
    }
}

/// Where an encoder is: not started (all zeros), open, or finished.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum State {
    Idle,
    Open,
    Done,
}

/// A FLAC encoder: the block being filled, the scratch it is planned in,
/// one frame of output, the MD5 of the samples so far and the totals
/// STREAMINFO wants. A fixed size (`size_of::<Flac>()`), all zero bytes
/// when new, so a host keeps one in static memory at no cost to its file
/// (the web build's `.bss`).
pub struct Flac {
    state: State,
    rate: u32,
    channels: usize,
    bits: u32,
    /// Frames in the block so far.
    fill: usize,
    /// The next frame's number.
    frame: u32,
    /// Frames (samples a channel) written in all.
    total: u64,
    smallest: u32,
    largest: u32,
    /// Left and right (or mono's one), as integers.
    pcm: [[i32; BLOCK]; 2],
    /// Mid and side, worked out from them for a stereo block.
    mid_side: [[i32; BLOCK]; 2],
    scratch: Scratch,
    out: Writer,
    md5: Md5,
    digest: [u8; 16],
}

impl Default for Flac {
    fn default() -> Self {
        Self::new()
    }
}

/// A sample of the core's (a float, full scale at 1) as a `bits`-bit
/// integer, the one rule every host shares so the same render makes the
/// same file, and the WAV a host writes beside it the same samples: scaled
/// by 2^(bits - 1), rounded to the nearest whole number (halves away from
/// zero), held to the integer's range (so 1.0 is one step under full
/// scale). No dither: the same samples always make the same integers.
/// Exact on every target: a float scaled by a power of two and half added
/// are exact in 64 bits. A NaN is 0. `bits` is held to 2 to 24.
#[must_use]
pub fn quantize(sample: f32, bits: u32) -> i32 {
    let full = f64::from(1u32 << (bits.clamp(2, 24) - 1));
    let v = f64::from(sample) * full;
    let rounded = if v < 0.0 { v - 0.5 } else { v + 0.5 };
    // Held to the range first, so the conversion only drops the fraction.
    #[allow(clippy::cast_possible_truncation)]
    let whole = rounded.clamp(-full, full - 1.0) as i32;
    whole
}

impl Flac {
    /// A new encoder, all zero bytes: nothing until `start`.
    #[must_use]
    #[allow(clippy::large_stack_arrays)] // built in place: in static memory, a box or a host's
    pub const fn new() -> Self {
        Self {
            state: State::Idle,
            rate: 0,
            channels: 0,
            bits: 0,
            fill: 0,
            frame: 0,
            total: 0,
            smallest: 0,
            largest: 0,
            pcm: [[0; BLOCK]; 2],
            mid_side: [[0; BLOCK]; 2],
            scratch: Scratch::new(),
            out: Writer::new(),
            md5: Md5::new(),
            digest: [0; 16],
        }
    }

    /// Begin a stream of `channels` channels (1 or 2) of `bits`-bit samples
    /// (16 or 24) at `rate` Hz, forgetting any before. Gives the file's
    /// first `HEADER` bytes, its totals unknown (the module's doc says why,
    /// and how `header()` finishes them).
    ///
    /// # Errors
    /// A rate, a layout or a bit depth the export does not write.
    pub fn start(&mut self, rate: u32, channels: u32, bits: u32) -> Result<&[u8], Refused> {
        if f64::from(rate) < crate::MIN_RATE || f64::from(rate) > 192_000.0 {
            return Err(Refused::Rate);
        }
        if channels != 1 && channels != 2 {
            return Err(Refused::Channels);
        }
        if bits != 16 && bits != 24 {
            return Err(Refused::Bits);
        }
        self.state = State::Open;
        (self.rate, self.channels, self.bits) = (rate, channels as usize, bits);
        (self.fill, self.frame, self.total, self.smallest, self.largest) = (0, 0, 0, 0, 0);
        self.digest = [0; 16];
        self.md5.reset();
        self.out.clear();
        let head = self.header();
        self.out.put_bytes(&head);
        Ok(self.out.bytes())
    }

    /// Take the next frames, `planes` one run of samples a channel (as
    /// `Core::render` fills them), each the same length and at most `BLOCK`
    /// long. Gives the bytes to append to the file: a whole FLAC frame when
    /// the block filled, else none. Valid until the next call.
    ///
    /// # Errors
    /// Not started; the runs not one a channel of one length; more than
    /// `BLOCK` frames; a stream past FLAC's last frame number.
    pub fn write(&mut self, planes: &[&[f32]]) -> Result<&[u8], Refused> {
        if self.state != State::Open {
            return Err(Refused::NotOpen);
        }
        let frames = planes.first().map_or(0, |p| p.len());
        if planes.len() != self.channels || planes.iter().any(|p| p.len() != frames) {
            return Err(Refused::Planes);
        }
        if frames > BLOCK {
            return Err(Refused::Frames);
        }
        self.out.clear();
        let mut at = 0;
        while at < frames {
            let take = (frames - at).min(BLOCK - self.fill);
            for (row, plane) in self.pcm.iter_mut().zip(planes) {
                for (dst, &src) in row.iter_mut().skip(self.fill).zip(plane.iter().skip(at)).take(take) {
                    *dst = quantize(src, self.bits);
                }
            }
            (self.fill, at) = (self.fill + take, at + take);
            if self.fill == BLOCK {
                // At most one a call: `frames` is at most a block.
                self.frame()?;
            }
        }
        Ok(self.out.bytes())
    }

    /// End the stream: gives the last frame's bytes (none if the block was
    /// empty). Then `header()` is the finished header.
    ///
    /// # Errors
    /// Not started, or already finished; a stream past FLAC's last frame
    /// number.
    pub fn finish(&mut self) -> Result<&[u8], Refused> {
        if self.state != State::Open {
            return Err(Refused::NotOpen);
        }
        self.out.clear();
        if self.fill > 0 {
            self.frame()?;
        }
        self.digest = self.md5.digest();
        self.state = State::Done;
        Ok(self.out.bytes())
    }

    /// The bytes the last call gave (`start`'s, `write`'s or `finish`'s),
    /// for a host that reads them apart from the call (the C and web
    /// faces).
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        self.out.bytes()
    }

    /// The channels the stream was started with (0 before `start`): how
    /// many runs a `write` takes, for a host that keeps them itself.
    #[must_use]
    pub const fn channels(&self) -> usize {
        self.channels
    }

    /// Frames (samples a channel) written so far, in finished frames.
    #[must_use]
    pub const fn frames(&self) -> u64 {
        self.total
    }

    /// The file's first `HEADER` bytes: after `finish`, with the total
    /// length, the smallest and largest frame and the MD5 of the samples;
    /// before it, with them unknown (as `start` wrote them). A host that can
    /// seek writes the finished one over the file's start.
    #[must_use]
    pub fn header(&self) -> [u8; HEADER] {
        let done = self.state == State::Done;
        let mut w = Writer::new();
        w.put_bytes(b"fLaC");
        // The last metadata block, STREAMINFO, 34 bytes.
        w.put(0x80, 8);
        w.put(34, 24);
        w.put(BLOCK_U32, 16);
        w.put(BLOCK_U32, 16);
        w.put(if done { self.smallest } else { 0 }, 24);
        w.put(if done { self.largest } else { 0 }, 24);
        w.put(self.rate, 20);
        w.put(u32::try_from(self.channels.max(1) - 1).unwrap_or(0), 3);
        w.put(self.bits.max(1) - 1, 5);
        // 36 bits of total; 0 (unknown) for a stream too long to say.
        let total = if done && self.total < 1 << 36 { self.total } else { 0 };
        w.put(u32::try_from(total >> 32).unwrap_or(0), 4);
        w.put(u32::try_from(total & 0xFFFF_FFFF).unwrap_or(0), 32);
        w.put_bytes(if done { &self.digest } else { &[0; 16] });
        let mut head = [0u8; HEADER];
        for (dst, &src) in head.iter_mut().zip(w.bytes()) {
            *dst = src;
        }
        head
    }

    /// Code the block (`fill` frames) as one FLAC frame into the output,
    /// and count it.
    fn frame(&mut self) -> Result<(), Refused> {
        if self.frame >= 1 << 31 {
            return Err(Refused::TooLong);
        }
        let (n, number, rate) = (self.fill, self.frame, self.rate);
        let Self { pcm, mid_side, scratch, out, md5, channels, bits, .. } = self;
        let (bps, channels) = (*bits, *channels);
        let stereo = channels == 2;
        let [left, right] = pcm;
        let (left, right) = (left.get(..n).unwrap_or(&[]), right.get(..n).unwrap_or(&[]));
        if stereo {
            let [mid, side] = mid_side;
            for (((m, s), &l), &r) in mid.iter_mut().zip(side.iter_mut()).zip(left).zip(right) {
                *m = (l + r) >> 1;
                *s = l - r;
            }
        }
        let [mid, side] = mid_side;
        let (mid, side) = (mid.get(..n).unwrap_or(&[]), side.get(..n).unwrap_or(&[]));
        // The channel assignment (FLAC's numbers) and its subframes: which
        // signal each is, its bits and its plan.
        let (assignment, coded): (u32, [Coded; 2]) = if stereo {
            let plan_l = scratch.plan(left, bps);
            let plan_r = scratch.plan(right, bps);
            let plan_m = scratch.plan(mid, bps);
            let plan_s = scratch.plan(side, bps + 1);
            // The cheapest pair; the earlier of equals.
            let pairs = [
                (1, [(left, bps, plan_l), (right, bps, plan_r)]),
                (8, [(left, bps, plan_l), (side, bps + 1, plan_s)]),
                (9, [(side, bps + 1, plan_s), (right, bps, plan_r)]),
                (10, [(mid, bps, plan_m), (side, bps + 1, plan_s)]),
            ];
            let cost = |pair: &(u32, [Coded; 2])| pair.1[0].2.bits + pair.1[1].2.bits;
            let mut best = pairs[0];
            for pair in pairs {
                if cost(&pair) < cost(&best) {
                    best = pair;
                }
            }
            best
        } else {
            let plan = scratch.plan(left, bps);
            (0, [(left, bps, plan), (left, bps, plan)])
        };
        // The frame's header.
        let start = out.len();
        out.put(0xFFF8, 16);
        let size_code = if n == BLOCK { 0b1100 } else if n <= 256 { 0b0110 } else { 0b0111 };
        out.put(size_code, 4);
        let (rate_code, rate_extra) = rate_code(rate);
        out.put(rate_code, 4);
        out.put(assignment, 4);
        out.put(if bps == 16 { 0b100 } else { 0b110 }, 3);
        out.put(0, 1);
        utf8(out, number);
        let less = u32::try_from(n - 1).unwrap_or(0);
        match size_code {
            0b0110 => out.put(less, 8),
            0b0111 => out.put(less, 16),
            _ => {}
        }
        if let Some((value, width)) = rate_extra {
            out.put(value, width);
        }
        let crc = crc8(out.bytes().get(start..).unwrap_or(&[]));
        out.put(u32::from(crc), 8);
        // The subframes, a byte boundary, the CRC.
        for (x, sbits, plan) in coded.iter().take(channels) {
            scratch.write(x, *sbits, plan, out);
        }
        out.align();
        let crc = crc16(out.bytes().get(start..).unwrap_or(&[]));
        out.put(u32::from(crc), 16);
        if out.full() {
            return Err(Refused::Overflow);
        }
        // The MD5 is of the samples interleaved, little-endian, in whole
        // bytes (2 a sample at 16 bits, 3 at 24), a few hundred at a time.
        let width = (bps / 8) as usize;
        let mut chunk = [0u8; 384];
        let per = (chunk.len() / (width * channels).max(1)).max(1);
        let rows = [left, right];
        for from in (0..n).step_by(per) {
            let mut len = 0;
            for i in from..(from + per).min(n) {
                for row in rows.iter().take(channels) {
                    let bytes = row.get(i).copied().unwrap_or(0).to_le_bytes();
                    for (dst, &b) in chunk.iter_mut().skip(len).zip(bytes.iter().take(width)) {
                        *dst = b;
                    }
                    len += width;
                }
            }
            md5.update(chunk.get(..len).unwrap_or(&[]));
        }
        let bytes = u32::try_from(out.len() - start).unwrap_or(u32::MAX);
        self.smallest = if number == 0 { bytes } else { self.smallest.min(bytes) };
        self.largest = self.largest.max(bytes);
        self.total += n as u64;
        self.frame += 1;
        self.fill = 0;
        Ok(())
    }
}

/// A subframe to write: its samples, their bits and its plan.
type Coded<'a> = (&'a [i32], u32, Plan);

/// A frame header's sample-rate code, and the field after the frame number
/// that some codes need (its value and width).
fn rate_code(rate: u32) -> (u32, Option<(u32, u32)>) {
    match rate {
        88_200 => (1, None),
        176_400 => (2, None),
        192_000 => (3, None),
        22_050 => (6, None),
        24_000 => (7, None),
        32_000 => (8, None),
        44_100 => (9, None),
        48_000 => (10, None),
        96_000 => (11, None),
        r if r % 1000 == 0 && r / 1000 <= 255 => (12, Some((r / 1000, 8))),
        r if r <= 65_535 => (13, Some((r, 16))),
        r if r % 10 == 0 && r / 10 <= 65_535 => (14, Some((r / 10, 16))),
        // From STREAMINFO.
        _ => (0, None),
    }
}

/// A frame number as FLAC writes it: UTF-8's way of coding a number, up to
/// six bytes for 31 bits.
fn utf8(w: &mut Writer, n: u32) {
    if n < 0x80 {
        w.put(n, 8);
        return;
    }
    let tail = match n {
        0..0x800 => 1,
        0x800..0x1_0000 => 2,
        0x1_0000..0x20_0000 => 3,
        0x20_0000..0x400_0000 => 4,
        _ => 5,
    };
    // The lead byte: as many ones as bytes, a zero, and the top bits.
    let lead = (0xFF00u32 >> (tail + 1)) & 0xFF;
    w.put(lead | (n >> (6 * tail)), 8);
    for i in (0..tail).rev() {
        w.put(0x80 | ((n >> (6 * i)) & 0x3F), 8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantize_rounds_halves_away_and_holds_full_scale() {
        assert_eq!(quantize(0.0, 16), 0);
        assert_eq!(quantize(1.0, 16), 32767);
        assert_eq!(quantize(-1.0, 16), -32768);
        assert_eq!(quantize(2.0, 24), 8_388_607);
        assert_eq!(quantize(-3.0, 24), -8_388_608);
        assert_eq!(quantize(2.5 / 32768.0, 16), 3);
        assert_eq!(quantize(-2.5 / 32768.0, 16), -3);
        assert_eq!(quantize(2.49 / 32768.0, 16), 2);
        assert_eq!(quantize(f32::NAN, 16), 0);
        assert_eq!(quantize(f32::INFINITY, 16), 32767);
    }

    #[test]
    fn frame_numbers_code_as_utf8() {
        for (n, want) in [
            (0u32, vec![0x00]),
            (0x7F, vec![0x7F]),
            (0x80, vec![0xC2, 0x80]),
            (0x7FF, vec![0xDF, 0xBF]),
            (0x800, vec![0xE0, 0xA0, 0x80]),
            (0x1_0000, vec![0xF0, 0x90, 0x80, 0x80]),
            (0x7FFF_FFFF, vec![0xFD, 0xBF, 0xBF, 0xBF, 0xBF, 0xBF]),
        ] {
            let mut w = Writer::new();
            utf8(&mut w, n);
            assert_eq!(w.bytes(), &want[..], "{n:#x}");
        }
    }
}
