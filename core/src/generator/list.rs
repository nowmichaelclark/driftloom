//! Fixed-capacity lists: the generator's working memory, sized at compile
//! time so nothing allocates (the core's rule, `core/AGENTS.md`).
//!
//! A `List` is an array and a length. Pushing past its capacity drops the
//! item and counts it (`List::lost`); nothing panics. Every capacity is
//! chosen above the most the generator can make (each says why where it is
//! declared), and the tests check across the goldens' corpus that nothing
//! is ever lost.

use core::ops::{Deref, DerefMut};

/// A type with an empty value that is all zero bytes (queue item 93).
///
/// A `List` fills its slots with it, and a `Maybe` holds it while empty, so a
/// list or a maybe, and everything that holds one, starts as zero bytes: the
/// module then keeps the player in `.bss` and not as bytes in the file.
/// Rust's own fillers are not zeros (a `&'static str` is never null; an
/// `Option`'s `None` can be a spare value of one of its fields), so every
/// type a `List` or a `Maybe` holds says what its empty value is.
/// `a_new_player_is_all_zeros` (`tests/zeros.rs`) fails, with the offset, if
/// one is not zero bytes where the player holds it.
pub trait Empty: Copy {
    /// The empty value.
    const EMPTY: Self;
}

macro_rules! empty_as {
    ($($t:ty => $v:expr),* $(,)?) => { $( impl Empty for $t { const EMPTY: Self = $v; } )* };
}

empty_as!(f64 => 0.0, f32 => 0.0, i16 => 0, i32 => 0, u8 => 0, u16 => 0, u32 => 0, u64 => 0, usize => 0, bool => false);

impl<A: Empty, B: Empty> Empty for (A, B) {
    const EMPTY: Self = (A::EMPTY, B::EMPTY);
}

impl<T: Empty, const N: usize> Empty for [T; N] {
    const EMPTY: Self = [T::EMPTY; N];
}

impl<T: Copy> Empty for Option<T> {
    const EMPTY: Self = None;
}

impl<T: Empty, const N: usize> Empty for List<T, N> {
    const EMPTY: Self = List::new();
}

impl<T: 'static> Empty for &'static [T] {
    const EMPTY: Self = &[];
}

/// `""`: held in a list or a maybe it is a slot nothing reads; to hold an
/// empty string where it must be zero bytes, use `Word`.
impl Empty for &'static str {
    const EMPTY: Self = "";
}

/// Up to `N` items of `T`.
///
/// Slots past the length hold `T::EMPTY`, which nothing reads.
#[derive(Clone, Copy)]
pub struct List<T: Empty, const N: usize> {
    items: [T; N],
    len: usize,
    lost: usize,
}

impl<T: Empty, const N: usize> List<T, N> {
    /// An empty list, all zero bytes where `T::EMPTY` is.
    #[must_use]
    pub const fn new() -> Self {
        List { items: [T::EMPTY; N], len: 0, lost: 0 }
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
    pub fn from_slice(items: &[T]) -> Self {
        let mut out = Self::new();
        for x in items {
            out.push(*x);
        }
        out
    }
}

impl<T: Empty, const N: usize> Default for List<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Empty, const N: usize> Deref for List<T, N> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        &self.items[..self.len]
    }
}

impl<T: Empty, const N: usize> DerefMut for List<T, N> {
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

/// A static string whose empty state is all zero bytes (queue item 93).
///
/// A bare `&'static str` can never be zero (a reference is not null), so a
/// struct holding one cannot start as zeros, and the player that holds
/// dozens of them would be stored in the module instead of left in `.bss`.
/// `Word` holds an `Option` of the reference, whose `None` is the null
/// pointer, and reads as the empty string.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Word(Option<&'static str>);

impl Word {
    /// The empty string, all zero bytes.
    pub const EMPTY: Word = Word(None);

    /// The word `s`.
    #[must_use]
    pub const fn new(s: &'static str) -> Self {
        Word(Some(s))
    }

    /// The string; `""` for the empty word.
    #[must_use]
    pub const fn get(self) -> &'static str {
        match self.0 {
            Some(s) => s,
            None => "",
        }
    }
}

impl From<&'static str> for Word {
    fn from(s: &'static str) -> Self {
        Word::new(s)
    }
}

impl Deref for Word {
    type Target = str;
    fn deref(&self) -> &str {
        self.get()
    }
}

impl PartialEq<&str> for Word {
    fn eq(&self, other: &&str) -> bool {
        self.get() == *other
    }
}

/// An `Option` whose `None` is all zero bytes (queue item 93).
///
/// Rust writes the `None` of an `Option<T>` into a spare value of one of
/// `T`'s fields (a flag's 2, say), so a struct holding one is not all zeros
/// even when empty, and a player made of them cannot start in `.bss`. A
/// `Maybe` keeps its own flag, and holds `T::EMPTY` while it is empty.
#[derive(Clone, Copy)]
pub struct Maybe<T: Empty> {
    some: bool,
    value: T,
}

impl<T: Empty> Maybe<T> {
    /// Nothing, all zero bytes where `T::EMPTY` is.
    pub const NONE: Maybe<T> = Maybe { some: false, value: T::EMPTY };

    /// `x`.
    #[must_use]
    pub const fn some(x: T) -> Self {
        Maybe { some: true, value: x }
    }

    /// The `Option`.
    #[must_use]
    pub fn get(&self) -> Option<T> {
        self.some.then_some(self.value)
    }

    /// A reference to the value, if any.
    #[must_use]
    pub fn as_ref(&self) -> Option<&T> {
        self.some.then_some(&self.value)
    }

    /// A mutable reference to the value, if any.
    pub fn as_mut(&mut self) -> Option<&mut T> {
        if self.some { Some(&mut self.value) } else { None }
    }

    /// Whether there is a value.
    #[must_use]
    pub const fn is_some(&self) -> bool {
        self.some
    }

    /// Whether there is none.
    #[must_use]
    pub const fn is_none(&self) -> bool {
        !self.some
    }

    /// Put `x` in its place (`Some`), or nothing (`None`).
    pub fn set(&mut self, x: Option<T>) {
        *self = x.map_or(Self::NONE, Self::some);
    }

    /// The value, made by `make` first if there is none (`Option::get_or_insert_with`).
    pub fn get_or_insert_with(&mut self, make: impl FnOnce() -> T) -> &mut T {
        if !self.some {
            *self = Self::some(make());
        }
        &mut self.value
    }

    /// Take the value, leaving none.
    pub fn take(&mut self) -> Option<T> {
        let x = self.get();
        *self = Self::NONE;
        x
    }
}

impl<T: Empty> Default for Maybe<T> {
    fn default() -> Self {
        Self::NONE
    }
}

impl<T: Empty> From<Option<T>> for Maybe<T> {
    fn from(x: Option<T>) -> Self {
        x.map_or(Self::NONE, Self::some)
    }
}

impl Empty for Word {
    const EMPTY: Self = Word::EMPTY;
}

/// A number whose stored bits are its bits XOR `BITS`: all zero bytes while it
/// holds the default `BITS` is the bits of (queue item 93).
///
/// A state that starts at 0.75 or 1.0 is not zeros, and the player that holds
/// dozens would be stored in the module instead of left in `.bss`. Stored
/// this way a default is zero, and every value reads back exactly as it was
/// set: no arithmetic, only a flip of bits.
#[derive(Clone, Copy, Debug)]
pub struct Biased<const BITS: u64>(u64);

impl<const BITS: u64> Biased<BITS> {
    /// The default (the number whose bits are `BITS`), all zero bytes.
    pub const DEFAULT: Self = Biased(0);

    /// `x`.
    #[must_use]
    pub const fn of(x: f64) -> Self {
        Biased(x.to_bits() ^ BITS)
    }

    /// The number.
    #[must_use]
    pub const fn get(self) -> f64 {
        f64::from_bits(self.0 ^ BITS)
    }

    /// Replace it with `x`.
    pub fn set(&mut self, x: f64) {
        *self = Self::of(x);
    }
}

impl<const BITS: u64> Default for Biased<BITS> {
    fn default() -> Self {
        Self::DEFAULT
    }
}
