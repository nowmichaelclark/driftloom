//! The canonical text and its hash (queue item 77; `tools/gen-golden.mjs`,
//! "The canonical text"), written as the JavaScript writes it, so the Rust
//! generator's decisions hash to the goldens' lines.
//!
//! - a number that is an integer of at most 2^53 in size, in decimal ('-0'
//!   for negative zero); any other finite number 'x' and the 16 hex digits
//!   of its IEEE 754 bits; NaN 'nan'; the infinities 'inf' and '-inf';
//! - a string in double quotes, '\' and '"' escaped with '\';
//! - true, false, null; a list '[a,b]'; an object '{k:v}', its keys sorted
//!   by character code, unquoted; no spaces anywhere.
//!
//! The writer streams: each byte goes to a `Sink` as it is made, the hash
//! (`Fnv`) or, for a host that wants the text, a buffer. Objects are
//! written by the caller in key order; in debug builds the writer checks
//! that each key sorts after the one before it.

/// Where the text goes.
pub trait Sink {
    /// The next bytes of the text.
    fn bytes(&mut self, b: &[u8]);
}

/// FNV-1a, 64 bits, over the text's bytes.
#[derive(Clone, Copy)]
pub struct Fnv(pub u64);

impl Fnv {
    /// The offset basis.
    #[must_use]
    pub const fn new() -> Self {
        Fnv(0xcbf2_9ce4_8422_2325)
    }
}

impl Default for Fnv {
    fn default() -> Self {
        Self::new()
    }
}

impl Sink for Fnv {
    fn bytes(&mut self, b: &[u8]) {
        for &c in b {
            self.0 ^= u64::from(c);
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
}

/// A fixed buffer of text, for a host that asks for it (`dl_spec`). Text
/// past its end is dropped and counted.
pub struct Text<'a> {
    pub buf: &'a mut [u8],
    pub len: usize,
    pub lost: usize,
}

impl Sink for Text<'_> {
    fn bytes(&mut self, b: &[u8]) {
        for &c in b {
            if self.len < self.buf.len() {
                self.buf[self.len] = c;
                self.len += 1;
            } else {
                self.lost += 1;
            }
        }
    }
}

/// Both a hash and a text at once.
pub struct Tee<'a, A: Sink, B: Sink>(pub &'a mut A, pub &'a mut B);

impl<A: Sink, B: Sink> Sink for Tee<'_, A, B> {
    fn bytes(&mut self, b: &[u8]) {
        self.0.bytes(b);
        self.1.bytes(b);
    }
}

const DEPTH: usize = 16;

/// The writer: values in, canonical text out to `sink`.
pub struct Writer<'a, S: Sink> {
    sink: &'a mut S,
    // Whether the list or object open at each depth has an item yet.
    started: [bool; DEPTH],
    // A key was just written: its value takes no comma.
    after_key: bool,
    #[cfg(debug_assertions)]
    last: [Option<&'static str>; DEPTH],
    depth: usize,
}

impl<'a, S: Sink> Writer<'a, S> {
    /// A writer into `sink`.
    pub fn new(sink: &'a mut S) -> Self {
        Writer {
            sink,
            started: [false; DEPTH],
            after_key: false,
            #[cfg(debug_assertions)]
            last: [None; DEPTH],
            depth: 0,
        }
    }

    fn sep(&mut self) {
        if self.after_key {
            self.after_key = false;
            return;
        }
        if self.depth > 0 && self.depth <= DEPTH {
            let d = self.depth - 1;
            if self.started[d] {
                self.sink.bytes(b",");
            }
            self.started[d] = true;
        }
    }

    fn open(&mut self) {
        if self.depth < DEPTH {
            self.started[self.depth] = false;
            #[cfg(debug_assertions)]
            {
                self.last[self.depth] = None;
            }
        }
        self.depth += 1;
    }

    /// Open an object.
    pub fn obj(&mut self) {
        self.sep();
        self.sink.bytes(b"{");
        self.open();
    }

    /// Open a list.
    pub fn list(&mut self) {
        self.sep();
        self.sink.bytes(b"[");
        self.open();
    }

    /// Close the object open now.
    pub fn end_obj(&mut self) {
        self.depth = self.depth.saturating_sub(1);
        self.sink.bytes(b"}");
    }

    /// Close the list open now.
    pub fn end_list(&mut self) {
        self.depth = self.depth.saturating_sub(1);
        self.sink.bytes(b"]");
    }

    /// The next key of the object open now; its value follows. The
    /// generator's own keys, written in sorted order.
    pub fn key(&mut self, k: &'static str) {
        #[cfg(debug_assertions)]
        if self.depth > 0 && self.depth <= DEPTH {
            let d = self.depth - 1;
            if let Some(prev) = self.last[d] {
                debug_assert!(prev < k, "canonical keys out of order: {prev} before {k}");
            }
            self.last[d] = Some(k);
        }
        self.key_any(k);
    }

    /// A key that is data (a profile's or a mood's name): the caller sorts
    /// them.
    pub fn key_any(&mut self, k: &str) {
        self.sep();
        self.sink.bytes(k.as_bytes());
        self.sink.bytes(b":");
        self.after_key = true;
    }

    /// A number.
    pub fn num(&mut self, x: f64) {
        self.sep();
        number(self.sink, x);
    }

    /// A string (printable ASCII; the generator's own names).
    pub fn str(&mut self, s: &str) {
        self.sep();
        self.sink.bytes(b"\"");
        for &c in s.as_bytes() {
            if c == b'\\' || c == b'"' {
                self.sink.bytes(b"\\");
            }
            self.sink.bytes(&[c]);
        }
        self.sink.bytes(b"\"");
    }

    /// true or false.
    pub fn bool(&mut self, b: bool) {
        self.sep();
        self.sink.bytes(if b { b"true" } else { b"false" });
    }

    /// null.
    pub fn null(&mut self) {
        self.sep();
        self.sink.bytes(b"null");
    }

    /// A list of numbers.
    pub fn nums(&mut self, xs: &[f64]) {
        self.list();
        for &x in xs {
            self.num(x);
        }
        self.end_list();
    }
}

/// A number in the canonical form.
#[allow(clippy::cast_possible_truncation)] // an integer of at most 2^53, checked
fn number<S: Sink>(sink: &mut S, x: f64) {
    if x.is_nan() {
        sink.bytes(b"nan");
    } else if x.is_infinite() {
        sink.bytes(if x > 0.0 { b"inf" } else { b"-inf" });
    } else if crate::math::floor(x) == x && x.abs() <= 9_007_199_254_740_992.0 {
        if x == 0.0 && x.is_sign_negative() {
            sink.bytes(b"-0");
            return;
        }
        let mut n = x as i64;
        let mut digits = [0u8; 20];
        let mut i = digits.len();
        let neg = n < 0;
        if n == 0 {
            i -= 1;
            digits[i] = b'0';
        }
        while n != 0 {
            i -= 1;
            #[allow(clippy::cast_sign_loss)] // a digit, 0 to 9
            {
                digits[i] = b'0' + (n % 10).unsigned_abs() as u8;
            }
            n /= 10;
        }
        if neg {
            sink.bytes(b"-");
        }
        sink.bytes(&digits[i..]);
    } else {
        let bits = x.to_bits();
        let mut hex = [b'x'; 17];
        for k in 0..16 {
            let d = ((bits >> (60 - 4 * k)) & 0xf) as u8;
            hex[k + 1] = if d < 10 { b'0' + d } else { b'a' + d - 10 };
        }
        sink.bytes(&hex);
    }
}
