//! The bits of a FLAC frame: a writer that packs fields of any width into a
//! fixed buffer, most significant bit first as FLAC wants them, and the two
//! checksums a frame carries (CRC-8 over its header, CRC-16 over all of it).
//!
//! The buffer is the encoder's own and fixed (`MAX_BYTES`): one frame and
//! no more, so the writer never grows. A write past its end is dropped and
//! remembered (`Writer::full`) rather than panicking; the encoder sizes the
//! buffer so that cannot happen (`super::MAX_BYTES` says why).

use super::MAX_BYTES;

/// A frame being written: its bytes so far, and up to seven bits waiting
/// for the rest of their byte.
pub struct Writer {
    bytes: [u8; MAX_BYTES],
    len: usize,
    /// The bits not yet a whole byte, in the low `pending` bits.
    acc: u64,
    pending: u32,
    /// A write found no room (never, by `MAX_BYTES`'s bound).
    full: bool,
}

impl Writer {
    /// An empty writer, all zero bytes (the encoder's `new` is).
    #[allow(clippy::large_stack_arrays)] // built in place: in static memory, or a host's (`Flac`)
    pub const fn new() -> Self {
        Self { bytes: [0; MAX_BYTES], len: 0, acc: 0, pending: 0, full: false }
    }

    /// Forget what was written: the next call's bytes start here.
    pub fn clear(&mut self) {
        (self.len, self.acc, self.pending, self.full) = (0, 0, 0, false);
    }

    /// The whole bytes written so far.
    pub fn bytes(&self) -> &[u8] {
        self.bytes.get(..self.len).unwrap_or(&[])
    }

    /// How many whole bytes are written.
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Whether a write ever found no room since the last `clear`.
    pub const fn full(&self) -> bool {
        self.full
    }

    fn byte(&mut self, b: u8) {
        if let Some(slot) = self.bytes.get_mut(self.len) {
            *slot = b;
            self.len += 1;
        } else {
            self.full = true;
        }
    }

    /// Write the low `n` bits of `value` (`n` up to 32), the highest first.
    pub fn put(&mut self, value: u32, n: u32) {
        if n == 0 {
            return;
        }
        let n = n.min(32);
        self.acc = (self.acc << n) | (u64::from(value) & ((1u64 << n) - 1));
        self.pending += n;
        while self.pending >= 8 {
            self.pending -= 8;
            self.byte(low_byte(self.acc >> self.pending));
        }
        self.acc &= (1u64 << self.pending) - 1;
    }

    /// Write `n` zero bits: a Rice code's quotient, in unary.
    pub fn zeros(&mut self, mut n: u64) {
        while n >= 32 {
            self.put(0, 32);
            n -= 32;
        }
        // Under 32 now.
        self.put(0, u32::try_from(n).unwrap_or(0));
    }

    /// Fill the byte with zero bits: a frame's subframes end on a byte.
    pub fn align(&mut self) {
        if self.pending > 0 {
            self.put(0, 8 - self.pending);
        }
    }

    /// Write whole bytes (the writer is on a byte boundary).
    pub fn put_bytes(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.put(u32::from(b), 8);
        }
    }
}

/// The low eight bits of `x`.
#[allow(clippy::cast_possible_truncation)] // the low byte is what is wanted
const fn low_byte(x: u64) -> u8 {
    x as u8
}

/// CRC-8 with FLAC's polynomial (x^8 + x^2 + x + 1), a byte at a time.
const CRC8: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        let mut c = low_byte(i as u64);
        let mut bit = 0;
        while bit < 8 {
            c = if c & 0x80 != 0 { (c << 1) ^ 0x07 } else { c << 1 };
            bit += 1;
        }
        table[i] = c;
        i += 1;
    }
    table
};

/// CRC-16 with FLAC's polynomial (x^16 + x^15 + x^2 + 1), a byte at a time.
const CRC16: [u16; 256] = {
    let mut table = [0u16; 256];
    let mut i = 0;
    while i < 256 {
        #[allow(clippy::cast_possible_truncation)] // i is under 256
        let mut c = (i as u16) << 8;
        let mut bit = 0;
        while bit < 8 {
            c = if c & 0x8000 != 0 { (c << 1) ^ 0x8005 } else { c << 1 };
            bit += 1;
        }
        table[i] = c;
        i += 1;
    }
    table
};

/// The CRC-8 a frame's header ends with, over its bytes.
pub fn crc8(bytes: &[u8]) -> u8 {
    bytes.iter().fold(0u8, |c, &b| CRC8[usize::from(c ^ b)])
}

/// The CRC-16 a frame ends with, over all its bytes before it.
pub fn crc16(bytes: &[u8]) -> u16 {
    bytes.iter().fold(0u16, |c, &b| {
        let [hi, _] = c.to_be_bytes();
        (c << 8) ^ CRC16[usize::from(hi ^ b)]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fields_pack_highest_bit_first() {
        let mut w = Writer::new();
        w.put(0b1, 1);
        w.put(0b011, 3);
        w.put(0xA, 4);
        w.put(0x1234_5678, 32);
        w.zeros(5);
        w.put(1, 1);
        w.align();
        assert_eq!(w.bytes(), &[0b1011_1010, 0x12, 0x34, 0x56, 0x78, 0b0000_0100]);
        assert!(!w.full());
    }

    // The check values of the two CRCs FLAC uses (CRC-8/SMBUS and
    // CRC-16/UMTS over "123456789").
    #[test]
    fn crcs_have_their_check_values() {
        assert_eq!(crc8(b"123456789"), 0xF4);
        assert_eq!(crc16(b"123456789"), 0xFEE8);
    }
}
