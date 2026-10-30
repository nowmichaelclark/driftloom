//! The voice budget (`_budget`, `_room` and `_release` in `js/synth.js`),
//! as the player asks it (queue item 82): in cost units, not a count of
//! voices; each note that plays holds its cost until it is over, and a note
//! that would take the budget past its ceiling is refused. The softer
//! accompaniment (pads, textures, a chord voice) asks against a lower cap,
//! leaving the rest for the tune; a part that yields (glass, dubStab) asks
//! for room and holds nothing. The figures and the order of every sum are
//! the JavaScript's, so the player refuses exactly the notes the page does.

use crate::generator::js;

/// The whole ceiling, in cost units (`MAX_BUDGET`).
pub const MAX_BUDGET: f64 = 260.0;
/// The soft cap at full quality (`SOFT_BUDGET`): what the accompaniment
/// may hold, the rest kept for the tune.
pub const SOFT_BUDGET: f64 = 170.0;
/// The ceiling on lite quality (`LITE_BUDGET`).
pub const LITE_BUDGET: f64 = 140.0;
/// What the soft cap keeps back at any ceiling (`LATE_LAYER_RESERVE`), but
/// never more than this share of it (`RESERVE_SHARE`).
const LATE_LAYER_RESERVE: f64 = MAX_BUDGET - SOFT_BUDGET;
const RESERVE_SHARE: f64 = 0.42;
/// What a yielding part asks room for above its own cost
/// (`YIELD_MARGIN`).
pub const YIELD_MARGIN: f64 = 20.0;
/// How long past its own end a note holds its cost (`_release`'s 0.12 s).
const TAIL: f64 = 0.12;

/// The most notes held at once. The JavaScript's list has no end; the
/// ceiling keeps it short (260 units, the cheapest note 0.3), and a hold
/// past this is refused, and counted (`Budget::full`).
pub const HOLDS: usize = 1024;

/// The notes holding the budget, earliest end first: a ring of their ends
/// and costs.
pub struct Budget {
    at: [f64; HOLDS],
    cost: [f64; HOLDS],
    head: usize,
    len: usize,
    lite: bool,
    full: u32,
}

impl Budget {
    /// Nothing held, at full quality.
    #[must_use]
    pub const fn new() -> Self {
        Budget { at: [0.0; HOLDS], cost: [0.0; HOLDS], head: 0, len: 0, lite: false, full: 0 }
    }

    /// Let everything go, and ask from now on at `lite` quality or full.
    pub fn reset(&mut self, lite: bool) {
        self.head = 0;
        self.len = 0;
        self.lite = lite;
        self.full = 0;
    }

    /// Holds refused for want of room in the list (0, or `HOLDS` is short).
    #[must_use]
    pub fn full(&self) -> u32 {
        self.full
    }

    fn slot(&self, k: usize) -> usize {
        (self.head + k) % HOLDS
    }

    fn ceiling(&self) -> f64 {
        if self.lite { LITE_BUDGET } else { MAX_BUDGET }
    }

    /// `_budget(time, soft, cost)`: whether a note of `cost` starting at
    /// `time` fits, under the soft cap if `soft`. Every hold over by `time`
    /// is let go first.
    pub fn ask(&mut self, time: f64, soft: bool, cost: f64) -> bool {
        while self.len > 0 && self.at[self.head] <= time {
            self.head = (self.head + 1) % HOLDS;
            self.len -= 1;
        }
        let mut spent = 0.0;
        for k in 0..self.len {
            spent += self.cost[self.slot(k)];
        }
        let ceiling = self.ceiling();
        let reserve = js::min(LATE_LAYER_RESERVE, js::round(ceiling * RESERVE_SHARE));
        let cap = if soft { js::max(40.0, ceiling - reserve) } else { ceiling };
        !(spent + cost > cap)
    }

    /// `_room(time, cost)`: whether `cost` fits under the full ceiling at
    /// `time`, for a part that yields. It only reads: it lets nothing go.
    #[must_use]
    pub fn room(&self, time: f64, cost: f64) -> bool {
        let mut spent = 0.0;
        for k in 0..self.len {
            let i = self.slot(k);
            if self.at[i] > time {
                spent += self.cost[i];
            }
        }
        spent + cost <= self.ceiling()
    }

    /// `_release(time, dur, cost)`: a note that played holds `cost` until
    /// `dur` after `time`, and a little longer.
    pub fn hold(&mut self, time: f64, dur: f64, cost: f64) {
        let at = time + dur + TAIL;
        if self.len >= HOLDS {
            self.full = self.full.saturating_add(1);
            return;
        }
        let mut i = self.len;
        while i > 0 && self.at[self.slot(i - 1)] > at {
            let (from, to) = (self.slot(i - 1), self.slot(i));
            self.at[to] = self.at[from];
            self.cost[to] = self.cost[from];
            i -= 1;
        }
        let to = self.slot(i);
        self.at[to] = at;
        self.cost[to] = cost;
        self.len += 1;
    }
}

impl Default for Budget {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn holds_and_lets_go_as_the_javascript_does() {
        let mut b = Budget::new();
        b.reset(false);
        assert!(b.ask(0.0, false, 200.0));
        b.hold(0.0, 1.0, 200.0);
        // 200 held until 1.12 s: 60 more fit under 260, 61 do not.
        assert!(b.ask(0.5, false, 60.0));
        assert!(!b.ask(0.5, false, 61.0));
        // Under the soft cap (170), nothing fits now.
        assert!(!b.ask(0.5, true, 1.0));
        // Past its end it is let go.
        assert!(b.ask(1.2, true, 170.0));
        // Lite: a ceiling of 140 and a soft cap of 81.
        b.reset(true);
        assert!(b.ask(0.0, true, 81.0) && !b.ask(0.0, true, 81.5));
    }

    #[test]
    fn keeps_its_holds_in_order_of_their_ends() {
        let mut b = Budget::new();
        b.reset(false);
        b.hold(0.0, 3.0, 1.0);
        b.hold(0.0, 1.0, 2.0);
        b.hold(0.0, 2.0, 4.0);
        // At 1.5 the 1.12 s hold is over, the others are not.
        assert!(b.room(1.5, 255.0) && !b.room(1.5, 256.0));
        assert!(b.ask(2.5, false, 259.0) && !b.ask(2.5, false, 259.5));
    }
}
