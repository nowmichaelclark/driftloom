//! How a channel of a frame is coded: the cheapest of a constant (a silent
//! stretch), the samples as they are (verbatim: noise, where nothing
//! predicts), or one of FLAC's five fixed predictors with its residual in
//! partitioned Rice codes.
//!
//! Fixed predictors and Rice coding are all of FLAC a simple encoder needs
//! (queue item 104a): the predictor is chosen by the smallest sum of
//! residuals, as most encoders choose it, and then the Rice partitions and
//! their parameters exactly, by counting the bits each would take. What a
//! plan says it costs is what `write` writes, to the bit, so the encoder
//! knows a frame's size before it writes it (and the bound on
//! `super::MAX_BYTES` holds).
//!
//! Everything is integer arithmetic on fixed arrays.

use super::BLOCK;
use super::bits::Writer;

/// The finest Rice partitioning tried: 2^6 = 64 partitions, 64 samples each
/// in a whole block. Finer saves next to nothing on music and costs a table
/// twice the size.
const MAX_PARTITION_ORDER: u32 = 6;
const PARTITIONS: usize = 1 << MAX_PARTITION_ORDER;

/// Rice parameters 0 to 30: the widest FLAC has (`RICE2`'s five bits, 31
/// meaning an escape this encoder never needs: a residual here is under
/// 2^30, so parameter 30 codes any of them).
const PARAMS: usize = 31;

/// The highest Rice parameter the four-bit field holds (15 is its escape).
const RICE_MAX: usize = 14;

/// What a channel's subframe is.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    /// Every sample the same: one sample.
    Constant,
    /// The samples as they are.
    Verbatim,
    /// A fixed predictor of `order` (0 to 4) and its residual.
    Fixed,
}

/// A channel's subframe, chosen: its kind, its predictor, its Rice
/// partitioning and parameters, and its size in bits.
#[derive(Clone, Copy)]
pub struct Plan {
    pub kind: Kind,
    order: usize,
    partition_order: u32,
    /// The five-bit parameters (`RICE2`), where one partition needs more
    /// than 14.
    wide: bool,
    params: [u8; PARTITIONS],
    pub bits: u64,
}

/// The planner's scratch: one channel's residual, as Rice codes it, and
/// the bits each parameter costs in each of the finest partitions. Part of
/// the encoder's fixed state.
pub struct Scratch {
    residual: [u32; BLOCK],
    sums: [[u64; PARAMS]; PARTITIONS],
}

/// A residual as Rice codes it: 0, -1, 1, -2, ... as 0, 1, 2, 3, ...
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)] // bounded: under 2^30 (`PARAMS`)
const fn zigzag(e: i64) -> u32 {
    ((e << 1) ^ (e >> 63)) as u32
}

/// The residual of the fixed predictor of `order` at the newest sample of
/// `w`, the five samples ending at it (the oldest first).
const fn fixed(w: [i64; 5], order: usize) -> i64 {
    let [a4, a3, a2, a1, a0] = w;
    match order {
        0 => a0,
        1 => a0 - a1,
        2 => a0 - 2 * a1 + a2,
        3 => a0 - 3 * a1 + 3 * a2 - a3,
        _ => a0 - 4 * a1 + 6 * a2 - 4 * a3 + a4,
    }
}

/// The five samples ending at `x[i]`, the ones before the start as zeros
/// (only a predictor of lower order reads them, and it never does).
fn window(x: &[i32], i: usize) -> [i64; 5] {
    let at = |back: usize| i.checked_sub(back).and_then(|j| x.get(j)).map_or(0, |&s| i64::from(s));
    [at(4), at(3), at(2), at(1), at(0)]
}

/// The fixed predictors' residual of order `order` from sample `order` on,
/// into `out` (`out[i]` the residual at sample `i`).
fn residual_into(x: &[i32], order: usize, out: &mut [u32; BLOCK]) {
    for (i, slot) in out.iter_mut().enumerate().take(x.len()).skip(order) {
        *slot = zigzag(fixed(window(x, i), order));
    }
}

impl Scratch {
    /// All zero bytes, as the encoder's `new` is.
    pub const fn new() -> Self {
        Self { residual: [0; BLOCK], sums: [[0; PARAMS]; PARTITIONS] }
    }

    /// The cheapest subframe for `x`, samples of `bps` bits.
    pub fn plan(&mut self, x: &[i32], bps: u32) -> Plan {
        let n = x.len();
        let bps64 = u64::from(bps);
        let mut best = Plan { kind: Kind::Verbatim, order: 0, partition_order: 0, wide: false, params: [0; PARTITIONS], bits: 8 + n as u64 * bps64 };
        let Some(&first) = x.first() else { return best };
        if x.iter().all(|&s| s == first) {
            best.kind = Kind::Constant;
            best.bits = 8 + bps64;
            return best;
        }
        // A predictor of order 4 needs four samples before it, and a Rice
        // partition one after: a shorter block (a stream's last) goes as it is.
        if n < 5 {
            return best;
        }
        let order = Self::order(x);
        residual_into(x, order, &mut self.residual);
        let (partition_order, wide, params, rice) = self.rice(n, order);
        let bits = 8 + order as u64 * bps64 + rice;
        if bits < best.bits {
            best = Plan { kind: Kind::Fixed, order, partition_order, wide, params, bits };
        }
        best
    }

    /// The fixed predictor with the smallest sum of residuals, over the
    /// samples every order predicts (from the fifth on).
    fn order(x: &[i32]) -> usize {
        let mut sums = [0u64; 5];
        for w in x.windows(5) {
            let &[oldest, older, old, last, now] = w else { continue };
            let w = [i64::from(oldest), i64::from(older), i64::from(old), i64::from(last), i64::from(now)];
            for (order, sum) in sums.iter_mut().enumerate() {
                *sum += fixed(w, order).unsigned_abs();
            }
        }
        let mut order = 0;
        for (o, &s) in sums.iter().enumerate() {
            if s < sums[order] {
                order = o;
            }
        }
        order
    }

    /// The Rice partitioning of the residual (`n` samples, the first
    /// `order` warm-up): its order, whether its parameters are five bits,
    /// the parameters, and its size in bits, the cheapest of every
    /// partitioning to `MAX_PARTITION_ORDER`, counted exactly.
    fn rice(&mut self, n: usize, order: usize) -> (u32, bool, [u8; PARTITIONS], u64) {
        // A partitioning splits the block evenly, and its first partition
        // holds at least one sample after the warm-up.
        let mut finest = MAX_PARTITION_ORDER.min(n.trailing_zeros());
        while finest > 0 && (n >> finest) <= order {
            finest -= 1;
        }
        let residual = self.residual.get(order..n).unwrap_or(&[]);
        let top = residual.iter().copied().max().unwrap_or(0);
        // No parameter past the widest residual's bit length pays: every
        // quotient is 0 there, and each step up costs a bit a sample.
        let params = ((32 - top.leading_zeros()) as usize).min(PARAMS - 1) + 1;
        // The bits each parameter costs in each finest partition, less the
        // stop bit and the remainder's: the quotients' sum, which adds up
        // partition by partition, so every coarser partitioning is counted
        // from these.
        // At least one sample (`n` is 5 or more, and `finest` keeps more than
        // `order` in a partition); said so the division can't fail.
        let size = (n >> finest).max(1);
        for sums in self.sums.iter_mut().take(1 << finest) {
            sums.fill(0);
        }
        for (i, &u) in self.residual.iter().enumerate().take(n).skip(order) {
            if let Some(sums) = self.sums.get_mut(i / size) {
                for (k, sum) in sums.iter_mut().enumerate().take(params) {
                    *sum += u64::from(u >> k);
                }
            }
        }
        let mut best = (0, false, [0u8; PARTITIONS], u64::MAX);
        for p in 0..=finest {
            let parts = 1usize << p;
            let merge = 1usize << (finest - p);
            let mut chosen = [0u8; PARTITIONS];
            let mut bits = 0u64;
            let mut widest = 0;
            for (j, slot) in chosen.iter_mut().enumerate().take(parts) {
                let len = ((n >> p) - if j == 0 { order } else { 0 }) as u64;
                let mut least = (u64::MAX, 0);
                for k in 0..params {
                    let quotients: u64 = self.sums.iter().skip(j * merge).take(merge).map(|s| s.get(k).copied().unwrap_or(0)).sum();
                    let cost = len * (k as u64 + 1) + quotients;
                    if cost < least.0 {
                        least = (cost, k);
                    }
                }
                bits += least.0;
                widest = widest.max(least.1);
                *slot = u8::try_from(least.1).unwrap_or(0);
            }
            let wide = widest > RICE_MAX;
            // The method (2 bits), the order (4), and each parameter.
            bits += 6 + parts as u64 * if wide { 5 } else { 4 };
            if bits < best.3 {
                best = (p, wide, chosen, bits);
            }
        }
        best
    }

    /// Write `plan`'s subframe for `x`, samples of `bps` bits: exactly
    /// `plan.bits` bits.
    pub fn write(&mut self, x: &[i32], bps: u32, plan: &Plan, w: &mut Writer) {
        // Two's complement in `bps` bits.
        #[allow(clippy::cast_sign_loss)] // the bits, as they are
        let sample = |s: i32| s as u32;
        match plan.kind {
            Kind::Constant => {
                w.put(0, 8);
                w.put(sample(x.first().copied().unwrap_or(0)), bps);
            }
            Kind::Verbatim => {
                w.put(0b0000_0010, 8);
                for &s in x {
                    w.put(sample(s), bps);
                }
            }
            Kind::Fixed => {
                let order = plan.order.min(4);
                // 0, 001 and the order, no wasted bits.
                w.put(u32::from(0b0001_0000u8 | low_order(order)), 8);
                for &s in x.iter().take(order) {
                    w.put(sample(s), bps);
                }
                residual_into(x, order, &mut self.residual);
                w.put(u32::from(plan.wide), 2);
                w.put(plan.partition_order, 4);
                let size = x.len() >> plan.partition_order;
                let width = if plan.wide { 5 } else { 4 };
                for (j, &k) in plan.params.iter().enumerate().take(1 << plan.partition_order) {
                    w.put(u32::from(k), width);
                    let from = if j == 0 { order } else { 0 };
                    let k = u32::from(k);
                    for &u in self.residual.iter().skip(j * size).take(size).skip(from) {
                        let q = u >> k;
                        let code = (1u32 << k) | (u & ((1u32 << k) - 1));
                        if q + k < 32 {
                            // The quotient's zeros, its stop bit and the
                            // remainder in one field.
                            w.put(code, q + k + 1);
                        } else {
                            w.zeros(u64::from(q));
                            w.put(code, k + 1);
                        }
                    }
                }
            }
        }
    }
}

/// A predictor's order (0 to 4) shifted into a subframe header's type bits.
fn low_order(order: usize) -> u8 {
    u8::try_from(order).unwrap_or(0) << 1
}
