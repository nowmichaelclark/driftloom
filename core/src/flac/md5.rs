//! MD5 (RFC 1321), for the one place FLAC asks for it: STREAMINFO carries
//! the MD5 of the decoded samples, so a decoder (`flac -t`) can say the
//! file is exactly what was encoded. Not for anything secret.
//!
//! Plain 32-bit integer arithmetic, so the digest is the same on every
//! target; a fixed 64-byte block buffer, so nothing allocates.

// RFC 1321's own names (a, b, c, d, f, g, k, s): the algorithm reads as
// the specification writes it.
#![allow(clippy::many_single_char_names)]

/// The rotations of each step.
const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11,
    16, 23, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

/// The constants of each step (the integer part of 2^32 |sin(i + 1)|).
const K: [u32; 64] = [
    0xd76a_a478, 0xe8c7_b756, 0x2420_70db, 0xc1bd_ceee, 0xf57c_0faf, 0x4787_c62a, 0xa830_4613, 0xfd46_9501, 0x6980_98d8, 0x8b44_f7af, 0xffff_5bb1, 0x895c_d7be,
    0x6b90_1122, 0xfd98_7193, 0xa679_438e, 0x49b4_0821, 0xf61e_2562, 0xc040_b340, 0x265e_5a51, 0xe9b6_c7aa, 0xd62f_105d, 0x0244_1453, 0xd8a1_e681, 0xe7d3_fbc8,
    0x21e1_cde6, 0xc337_07d6, 0xf4d5_0d87, 0x455a_14ed, 0xa9e3_e905, 0xfcef_a3f8, 0x676f_02d9, 0x8d2a_4c8a, 0xfffa_3942, 0x8771_f681, 0x6d9d_6122, 0xfde5_380c,
    0xa4be_ea44, 0x4bde_cfa9, 0xf6bb_4b60, 0xbebf_bc70, 0x289b_7ec6, 0xeaa1_27fa, 0xd4ef_3085, 0x0488_1d05, 0xd9d4_d039, 0xe6db_99e5, 0x1fa2_7cf8, 0xc4ac_5665,
    0xf429_2244, 0x432a_ff97, 0xab94_23a7, 0xfc93_a039, 0x655b_59c3, 0x8f0c_cc92, 0xffef_f47d, 0x8584_5dd1, 0x6fa8_7e4f, 0xfe2c_e6e0, 0xa301_4314, 0x4e08_11a1,
    0xf753_7e82, 0xbd3a_f235, 0x2ad7_d2bb, 0xeb86_d391,
];

/// A digest being made: the four words, the bytes not yet a whole block,
/// and the length so far. All zeros until `reset` (the encoder's `new` is).
#[derive(Clone, Copy)]
pub struct Md5 {
    words: [u32; 4],
    block: [u8; 64],
    fill: usize,
    bytes: u64,
}

impl Md5 {
    /// All zero bytes: not a digest yet, until `reset`.
    pub const fn new() -> Self {
        Self { words: [0; 4], block: [0; 64], fill: 0, bytes: 0 }
    }

    /// Begin a new digest.
    pub fn reset(&mut self) {
        self.words = [0x6745_2301, 0xefcd_ab89, 0x98ba_dcfe, 0x1032_5476];
        (self.fill, self.bytes) = (0, 0);
    }

    /// Add `data` to it.
    pub fn update(&mut self, data: &[u8]) {
        self.bytes = self.bytes.wrapping_add(data.len() as u64);
        for &b in data {
            if let Some(slot) = self.block.get_mut(self.fill) {
                *slot = b;
            }
            self.fill += 1;
            if self.fill == 64 {
                let block = self.block;
                self.compress(&block);
                self.fill = 0;
            }
        }
    }

    /// The digest of everything added, leaving this one as it was.
    pub fn digest(&self) -> [u8; 16] {
        let mut end = *self;
        let bits = self.bytes.wrapping_mul(8);
        end.update(&[0x80]);
        while end.fill != 56 {
            end.update(&[0]);
        }
        end.update(&bits.to_le_bytes());
        let mut out = [0u8; 16];
        for (chunk, w) in out.as_chunks_mut::<4>().0.iter_mut().zip(end.words) {
            chunk.copy_from_slice(&w.to_le_bytes());
        }
        out
    }

    fn compress(&mut self, block: &[u8; 64]) {
        let mut m = [0u32; 16];
        for (w, chunk) in m.iter_mut().zip(block.as_chunks::<4>().0) {
            *w = u32::from_le_bytes(*chunk);
        }
        let [mut a, mut b, mut c, mut d] = self.words;
        for (i, (&k, &s)) in K.iter().zip(&S).enumerate() {
            let (f, g) = match i / 16 {
                0 => ((b & c) | (!b & d), i),
                1 => ((d & b) | (!d & c), (5 * i + 1) % 16),
                2 => (b ^ c ^ d, (3 * i + 5) % 16),
                _ => (c ^ (b | !d), (7 * i) % 16),
            };
            let f = f.wrapping_add(a).wrapping_add(k).wrapping_add(m[g & 15]);
            (a, d, c) = (d, c, b);
            b = b.wrapping_add(f.rotate_left(s));
        }
        for (w, x) in self.words.iter_mut().zip([a, b, c, d]) {
            *w = w.wrapping_add(x);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Md5;

    fn hex(data: &[u8]) -> String {
        let mut m = Md5::new();
        m.reset();
        m.update(data);
        m.digest().iter().fold(String::new(), |mut s, b| {
            use std::fmt::Write as _;
            let _ = write!(s, "{b:02x}");
            s
        })
    }

    // RFC 1321's test suite.
    #[test]
    fn rfc_1321_vectors() {
        assert_eq!(hex(b""), "d41d8cd98f00b204e9800998ecf8427e");
        assert_eq!(hex(b"abc"), "900150983cd24fb0d6963f7d28e17f72");
        assert_eq!(hex(b"message digest"), "f96b697d7cb7938d525a2f31aaf161d0");
        assert_eq!(
            hex(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"),
            "57edf4a22be3c955ac49da2e2107b67a"
        );
    }

    #[test]
    fn pieces_make_the_same_digest_as_the_whole() {
        let data: Vec<u8> = (0..1000u32).map(|i| (i * 7 + 3).to_le_bytes()[0]).collect();
        let mut m = Md5::new();
        m.reset();
        for chunk in data.chunks(37) {
            m.update(chunk);
        }
        let mut whole = Md5::new();
        whole.reset();
        whole.update(&data);
        assert_eq!(m.digest(), whole.digest());
    }
}
