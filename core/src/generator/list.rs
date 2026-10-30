//! Fixed-capacity lists: the generator's working memory, sized at compile
//! time so nothing allocates (the core's rule, `core/AGENTS.md`).
//!
//! A `List` is an array and a length. Pushing past its capacity drops the
//! item and counts it (`List::lost`); nothing panics. Every capacity is
//! chosen above the most the generator can make (each says why where it is
//! declared), and the tests check across the goldens' corpus that nothing
//! is ever lost.

use core::ops::{Deref, DerefMut};

/// Up to `N` items of `T`.
#[derive(Clone, Copy)]
pub struct List<T: Copy, const N: usize> {
    items: [T; N],
    len: usize,
    lost: usize,
}

impl<T: Copy, const N: usize> List<T, N> {
    /// An empty list, every slot `fill`.
    #[must_use]
    pub const fn filled(fill: T) -> Self {
        List { items: [fill; N], len: 0, lost: 0 }
    }

    /// Add `x` at the end; past the capacity it is dropped and counted.
    pub fn push(&mut self, x: T) {
        if self.len < N {
            self.items[self.len] = x;
            self.len += 1;
        } else {
            self.lost += 1;
        }
    }

    /// Empty it.
    pub fn clear(&mut self) {
        self.len = 0;
        self.lost = 0;
    }

    /// Keep the first `n` items.
    pub fn truncate(&mut self, n: usize) {
        self.len = self.len.min(n);
    }

    /// Remove the item at `i`, keeping the order of the rest.
    pub fn remove(&mut self, i: usize) {
        if i < self.len {
            self.items.copy_within(i + 1..self.len, i);
            self.len -= 1;
        }
    }

    /// Keep the items `keep` says to, in order.
    pub fn retain(&mut self, mut keep: impl FnMut(&T) -> bool) {
        let mut n = 0;
        for i in 0..self.len {
            if keep(&self.items[i]) {
                self.items[n] = self.items[i];
                n += 1;
            }
        }
        self.len = n;
    }

    /// How many pushes were dropped for want of room (0, always, in a
    /// correct build).
    #[must_use]
    pub fn lost(&self) -> usize {
        self.lost
    }

    /// Sort in place, keeping equal items in their order (a stable sort,
    /// as JavaScript's `Array.prototype.sort` is): insertion, which needs
    /// no buffer, and the generator's lists are short and mostly sorted.
    pub fn sort_stable_by(&mut self, mut before: impl FnMut(&T, &T) -> bool) {
        for i in 1..self.len {
            let x = self.items[i];
            let mut j = i;
            while j > 0 && before(&x, &self.items[j - 1]) {
                self.items[j] = self.items[j - 1];
                j -= 1;
            }
            self.items[j] = x;
        }
    }

    /// The list from `items`.
    #[must_use]
    pub fn from_slice(fill: T, items: &[T]) -> Self {
        let mut out = Self::filled(fill);
        for x in items {
            out.push(*x);
        }
        out
    }
}

impl<T: Copy, const N: usize> Deref for List<T, N> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        &self.items[..self.len]
    }
}

impl<T: Copy, const N: usize> DerefMut for List<T, N> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.items[..self.len]
    }
}

/// A short ASCII name (a loop's, `seedName`), up to 16 bytes.
#[derive(Clone, Copy)]
pub struct Name {
    bytes: [u8; 16],
    len: usize,
}

impl Name {
    /// The empty name.
    #[must_use]
    pub const fn new() -> Self {
        Name { bytes: [0; 16], len: 0 }
    }

    /// Append `s`, as far as it fits.
    pub fn push_str(&mut self, s: &str) {
        for &b in s.as_bytes() {
            if self.len < self.bytes.len() {
                self.bytes[self.len] = b;
                self.len += 1;
            }
        }
    }

    /// The name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.bytes[..self.len]).unwrap_or("")
    }
}

impl Default for Name {
    fn default() -> Self {
        Self::new()
    }
}
