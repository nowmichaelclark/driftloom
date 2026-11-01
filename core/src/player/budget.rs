//! The voice budget (`_budget`, `_room` and `_release` in `js/synth.js`),
//! as the player asks it (queue item 82): in cost units, not a count of
//! voices; each note that plays holds its cost until it is over, and a note
//! that would take the budget past its ceiling is refused. The softer
//! accompaniment (pads, textures, a chord voice) asks against a lower cap,
//! leaving the rest for the tune; a part that yields (glass, dubStab) asks
//! for room and holds nothing. The figures and the order of every sum are
//! the JavaScript's, so the player refuses exactly the notes the page does.
//!
//! **Taking room from old notes** (queue item 85c; `_steal` in synth.js): a
//! note that doesn't fit lets go of its own layer's oldest voice calls still
//! holding the budget, from other steps, until it fits, and is refused only
//! if that can't make room. Each call let go falls to nothing over
//! `STEAL_FADE` in the core (`Host::steal`), ending as the new note starts.
//! Every hold knows its call (`begin`: the layer and the key; the channels
//! its notes went to, `note_channel`). Lite only for now (`STEAL_LITE`).

use super::host::Host;
use crate::Key;
use crate::generator::{Layer, js};

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
/// How long a voice let go for a new note takes to fall to nothing, in
/// seconds, ending as the new note starts (`STEAL_FADE`; for Mikey's ears).
pub const STEAL_FADE: f64 = 0.05;
/// Whether the budget takes room from old notes on lite, and at full
/// quality (`STEAL_QUALITIES`): both since queue item 85e (Mikey heard
/// lite's and asked for the same at full).
pub const STEAL_LITE: bool = true;
pub const STEAL_FULL: bool = true;
/// Whether the calls already past their end go before the ones still held,
/// on lite and at full (`RELEASED_FIRST`, queue item 85e): the field's order
/// (a sampler lets a releasing voice go before a held one), oldest first
/// within each.
pub const RELEASED_FIRST_LITE: bool = true;
pub const RELEASED_FIRST_FULL: bool = true;

/// The most notes held at once. The JavaScript's list has no end; the
/// ceiling keeps it short (260 units, the cheapest note 0.3), and a hold
/// past this is refused, and counted (`Budget::full`).
pub const HOLDS: usize = 1024;

/// A voice call that holds the budget (`_group` in synth.js): its number,
/// its layer, its key, the core channels its notes went to (a bit each),
/// and whether it is never let go (`keep`: a held instrument's note).
#[derive(Clone, Copy)]
struct Group {
    id: u32,
    layer: Layer,
    key: Key,
    channels: u16,
    kept: bool,
}

const NO_KEY: Key = Key { seed: 0, pass: 0, step: 0, ord: 0 };
const NO_GROUP: Group = Group { id: 0, layer: Layer::Drums, key: NO_KEY, channels: 0, kept: false };

/// A voice call among those the budget could let go (`_steal`'s entries):
/// its earliest hold's start, its cost, and whether every note of it is past
/// its end (in its `TAIL`, releasing).
#[derive(Clone, Copy)]
struct Older {
    group: Group,
    start: f64,
    cost: f64,
    released: bool,
}

/// The notes holding the budget, earliest end first: a ring of their ends,
/// costs, starts and voice calls.
pub struct Budget {
    at: [f64; HOLDS],
    cost: [f64; HOLDS],
    start: [f64; HOLDS],
    /// Each hold's voice call, if it was made by one for a layer.
    group: [Option<Group>; HOLDS],
    head: usize,
    len: usize,
    lite: bool,
    full: u32,
    /// The voice call being made (`begin`), and how many calls there have
    /// been (each one's number).
    current: Option<Group>,
    calls: u32,
    /// The voice calls let go, by layer (`synth.stolen`).
    stolen: [u32; 5],
    /// `_steal`'s working list: the calls it could let go.
    older: [Older; HOLDS],
}

impl Budget {
    /// Nothing held, at full quality.
    // Built in a `const` for a host's static player, never on a stack at
    // run time (as `Player::new`).
    #[allow(clippy::large_stack_arrays)]
    #[must_use]
    pub const fn new() -> Self {
        Budget {
            at: [0.0; HOLDS],
            cost: [0.0; HOLDS],
            start: [0.0; HOLDS],
            group: [None; HOLDS],
            head: 0,
            len: 0,
            lite: false,
            full: 0,
            current: None,
            calls: 0,
            stolen: [0; 5],
            older: [Older { group: NO_GROUP, start: 0.0, cost: 0.0, released: false }; HOLDS],
        }
    }

    /// Let everything go, and ask from now on at `lite` quality or full.
    pub fn reset(&mut self, lite: bool) {
        self.head = 0;
        self.len = 0;
        self.lite = lite;
        self.full = 0;
        self.current = None;
        self.stolen = [0; 5];
    }

    /// A voice call for `layer` begins, keyed `key` (the entry points'
    /// wrapper in synth.js): the holds it makes are its own, and the budget
    /// can let them go together. `None` for a part that plays for no layer.
    pub fn begin(&mut self, layer: Option<Layer>, key: Key) {
        self.calls = self.calls.wrapping_add(1);
        self.current = layer.map(|layer| Group { id: self.calls, layer, key, channels: 0, kept: false });
    }

    /// The voice call being made is never let go for a newer note (queue
    /// item 85e): a held instrument's, the drone's (`keep` in synth.js).
    /// The organ, the wash and padSweep's bed are the channels' own blocks
    /// and hold no budget.
    pub fn keep(&mut self) {
        if let Some(g) = self.current.as_mut() {
            g.kept = true;
        }
    }

    /// The voice call is over.
    pub fn end(&mut self) {
        self.current = None;
    }

    /// The voice call handed a note to the core's `channel` (`_toCore`):
    /// it goes there too if it is let go.
    pub fn note_channel(&mut self, channel: u32) {
        let Some(g) = self.current.as_mut() else { return };
        let bit = 1u16.checked_shl(channel).unwrap_or(0);
        g.channels |= bit;
        let id = g.id;
        for k in 0..self.len {
            let i = self.slot(k);
            if let Some(h) = self.group[i].as_mut()
                && h.id == id
            {
                h.channels |= bit;
            }
        }
    }

    /// The voice calls let go so far, by layer (`LAYERS`' order).
    #[must_use]
    pub fn stolen(&self) -> [u32; 5] {
        self.stolen
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
    /// is let go first; a note that doesn't fit takes room from its own
    /// layer's older voice calls, if that can make it (`steal`).
    pub fn ask<H: Host>(&mut self, host: &mut H, time: f64, soft: bool, cost: f64) -> bool {
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
        if spent + cost > cap {
            return self.steal(host, time, cost, cap, spent);
        }
        true
    }

    /// `_steal(time, cost, cap, spent)`: the voice calls holding the budget
    /// for the asking call's layer, from other steps, started before `time`,
    /// and not kept (`keep`), the released ones first (`RELEASED_FIRST`),
    /// oldest first within each (ties as the holds stand), let go one at a
    /// time until
    /// the note fits; none if all of them would not make room. Each falls
    /// from `STEAL_FADE` before `time` (or its start, if later) to nothing
    /// at `time` in every channel its notes went to, lowest first.
    fn steal<H: Host>(&mut self, host: &mut H, time: f64, cost: f64, cap: f64, spent: f64) -> bool {
        let Some(g) = self.current else { return false };
        if !(if self.lite { STEAL_LITE } else { STEAL_FULL }) {
            return false;
        }
        let mut n = 0;
        for k in 0..self.len {
            let i = self.slot(k);
            let Some(h) = self.group[i] else { continue };
            if h.kept || h.id == g.id || h.layer != g.layer || (h.key.pass == g.key.pass && h.key.step == g.key.step) {
                continue;
            }
            let released = self.at[i] - TAIL <= time;
            if let Some(e) = self.older[..n].iter_mut().find(|e| e.group.id == h.id) {
                if self.start[i] < e.start {
                    e.start = self.start[i];
                }
                e.group.channels |= h.channels;
                e.cost += self.cost[i];
                e.released = e.released && released;
            } else {
                self.older[n] = Older { group: h, start: self.start[i], cost: 0.0 + self.cost[i], released };
                n += 1;
            }
        }
        let released_first = if self.lite { RELEASED_FIRST_LITE } else { RELEASED_FIRST_FULL };
        // Those started before the note, oldest first (the released ones
        // before the held, `RELEASED_FIRST`): an insertion sort, which keeps
        // ties as they stood (the JavaScript's stable sort).
        let mut m = 0;
        for j in 0..n {
            if self.older[j].start < time {
                self.older[m] = self.older[j];
                m += 1;
            }
        }
        for j in 1..m {
            let e = self.older[j];
            let mut k = j;
            while k > 0 && after(&self.older[k - 1], &e, released_first) {
                self.older[k] = self.older[k - 1];
                k -= 1;
            }
            self.older[k] = e;
        }
        let mut left = spent;
        let mut take = 0;
        while take < m && left + cost > cap {
            left -= self.older[take].cost;
            take += 1;
        }
        if left + cost > cap {
            return false;
        }
        for j in 0..take {
            let e = self.older[j];
            let from = js::max(e.start, time - STEAL_FADE);
            for channel in 0..16u32 {
                if e.group.channels & (1 << channel) != 0 {
                    host.steal(channel, e.group.key, from, time);
                }
            }
            if let Some(c) = self.stolen.get_mut(e.group.layer as usize) {
                *c += 1;
            }
        }
        // Their holds let go, the rest kept in order.
        let mut kept = 0;
        for k in 0..self.len {
            let i = self.slot(k);
            let gone = self.group[i].is_some_and(|h| self.older[..take].iter().any(|e| e.group.id == h.id));
            if !gone {
                let to = self.slot(kept);
                self.at[to] = self.at[i];
                self.cost[to] = self.cost[i];
                self.start[to] = self.start[i];
                self.group[to] = self.group[i];
                kept += 1;
            }
        }
        self.len = kept;
        true
    }

    /// Every hold over by `time` at the latest (queue item 86): the notes a
    /// loop handed over before a crossfade let go of the budget at the
    /// boundary, so the next loop's notes are not refused for room the old
    /// ones only fade in. The
    /// holds stay in order of their ends (each end only comes down to
    /// `time`, which keeps them sorted).
    pub fn end_by(&mut self, time: f64) {
        for k in 0..self.len {
            let i = self.slot(k);
            if self.at[i] > time {
                self.at[i] = time;
            }
        }
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
            self.start[to] = self.start[from];
            self.group[to] = self.group[from];
            i -= 1;
        }
        let to = self.slot(i);
        self.at[to] = at;
        self.cost[to] = cost;
        self.start[to] = time;
        self.group[to] = self.current;
        self.len += 1;
    }
}

/// Whether `a` goes after `b` among the calls to let go: held after
/// released (with `released_first`), then later after earlier.
fn after(a: &Older, b: &Older, released_first: bool) -> bool {
    if released_first && a.released != b.released {
        return b.released;
    }
    a.start - b.start > 0.0
}

impl Default for Budget {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::Event;
    use crate::{EXTRA, Tap};

    /// A host that only writes down what the budget lets go.
    #[derive(Default)]
    struct Steals(Vec<(u32, Key, f64, f64)>);

    impl Host for Steals {
        fn frame(&self) -> u64 {
            0
        }
        fn key(&mut self, _: Key) {}
        fn note(&mut self, _: u32, _: u32, _: f64, _: f64, _: f64, _: f64, _: u32, _: [f64; EXTRA]) {}
        fn event(&mut self, _: u32, _: Event) {}
        fn damp(&mut self, _: u32, _: f64) {}
        fn set(&mut self, _: u32, _: usize, _: usize, _: f64, _: f64) {}
        fn route(&mut self, _: u32, _: usize, _: usize, _: Option<Tap>, _: f64) {}
        fn param(&mut self, _: u32, _: u32, _: f64, _: f64, _: f64) {}
        fn bass_lift(&mut self, _: f64) {}
        fn space_range(&mut self, _: usize, _: f64, _: f64) {}
        fn steal(&mut self, channel: u32, key: Key, from: f64, to: f64) {
            self.0.push((channel, key, from, to));
        }
        fn fade_all(&mut self, _: f64, _: f64) {}
    }

    #[test]
    fn holds_and_lets_go_as_the_javascript_does() {
        let mut core = Steals::default();
        let mut b = Budget::new();
        b.reset(false);
        assert!(b.ask(&mut core, 0.0, false, 200.0));
        b.hold(0.0, 1.0, 200.0);
        // 200 held until 1.12 s: 60 more fit under 260, 61 do not.
        assert!(b.ask(&mut core, 0.5, false, 60.0));
        assert!(!b.ask(&mut core, 0.5, false, 61.0));
        // Under the soft cap (170), nothing fits now.
        assert!(!b.ask(&mut core, 0.5, true, 1.0));
        // Past its end it is let go.
        assert!(b.ask(&mut core, 1.2, true, 170.0));
        // Lite: a ceiling of 140 and a soft cap of 81.
        b.reset(true);
        assert!(b.ask(&mut core, 0.0, true, 81.0) && !b.ask(&mut core, 0.0, true, 81.5));
    }

    #[test]
    fn keeps_its_holds_in_order_of_their_ends() {
        let mut core = Steals::default();
        let mut b = Budget::new();
        b.reset(false);
        b.hold(0.0, 3.0, 1.0);
        b.hold(0.0, 1.0, 2.0);
        b.hold(0.0, 2.0, 4.0);
        // At 1.5 the 1.12 s hold is over, the others are not.
        assert!(b.room(1.5, 255.0) && !b.room(1.5, 256.0));
        assert!(b.ask(&mut core, 2.5, false, 259.0) && !b.ask(&mut core, 2.5, false, 259.5));
    }

    #[test]
    fn takes_room_from_its_own_layers_oldest_calls() {
        let mut host = Steals::default();
        let mut b = Budget::new();
        b.reset(true);
        let at = |step, ord| Key { seed: 1, pass: 0, step, ord };
        // Two melody notes and a chord, each its own call, each 60 on lite's
        // 140: the first melody note on channels 3 and 5 (a knock).
        b.begin(Some(Layer::Melody), at(0, 0));
        b.note_channel(3);
        b.hold(0.0, 4.0, 60.0);
        b.note_channel(5);
        b.end();
        b.begin(Some(Layer::Chords), at(0, 0));
        b.note_channel(2);
        b.hold(0.1, 4.0, 60.0);
        b.end();
        // A melody note of 30 at 1 s: 120 + 30 > 140, so it takes the oldest
        // melody call, faded over the last 0.05 s before it, in both its
        // channels, lowest first; the chord stays.
        b.begin(Some(Layer::Melody), at(4, 0));
        assert!(b.ask(&mut host, 1.0, false, 30.0));
        b.end();
        assert_eq!(host.0, vec![(3, at(0, 0), 0.95, 1.0), (5, at(0, 0), 0.95, 1.0)]);
        assert_eq!(b.stolen()[Layer::Melody as usize], 1);
        assert!(b.room(1.0, 80.0) && !b.room(1.0, 81.0), "the chord's 60 still held");
        // Nothing of its own layer from another step: refused, nothing let go.
        host.0.clear();
        b.begin(Some(Layer::Melody), at(8, 0));
        assert!(!b.ask(&mut host, 1.5, false, 90.0));
        b.end();
        assert_eq!(host.0, Vec::new());
        // A call never takes from its own step (a chord's own tones).
        b.begin(Some(Layer::Chords), at(0, 1));
        assert!(!b.ask(&mut host, 1.5, false, 90.0));
        b.end();
        assert_eq!(host.0, Vec::new());
        // At full quality it takes room too (queue item 85e).
        let mut full = Budget::new();
        full.reset(false);
        full.begin(Some(Layer::Melody), at(0, 0));
        full.note_channel(3);
        full.hold(0.0, 4.0, 250.0);
        full.end();
        full.begin(Some(Layer::Melody), at(4, 0));
        assert!(full.ask(&mut host, 1.0, false, 30.0));
        assert_eq!(host.0, vec![(3, at(0, 0), 0.95, 1.0)]);
    }

    #[test]
    fn lets_released_calls_go_before_held_ones_and_never_a_kept_one() {
        let mut host = Steals::default();
        let at = |step, ord| Key { seed: 1, pass: 0, step, ord };
        let mut b = Budget::new();
        b.reset(true);
        // The oldest call still held at 1 s (its note to 4 s), then a call
        // whose note ended at 0.95 s (in its tail, releasing), then a kept
        // call (a drone's note), each 60 of lite's 140.
        b.begin(Some(Layer::Bass), at(0, 0));
        b.note_channel(2);
        b.hold(0.0, 4.0, 60.0);
        b.end();
        b.begin(Some(Layer::Bass), at(1, 0));
        b.note_channel(2);
        b.hold(0.5, 0.45, 60.0);
        b.end();
        // A note of 30 at 1 s needs one let go: the released one, though it
        // is the younger.
        b.begin(Some(Layer::Bass), at(4, 0));
        assert!(b.ask(&mut host, 1.0, false, 30.0));
        b.end();
        assert_eq!(host.0, vec![(2, at(1, 0), 0.95, 1.0)]);
        // A kept call is never let go: with only it and the held one to take
        // from, the held one goes.
        host.0.clear();
        let mut k = Budget::new();
        k.reset(true);
        k.begin(Some(Layer::Bass), at(0, 0));
        k.keep();
        k.note_channel(2);
        k.hold(0.0, 4.0, 60.0);
        k.end();
        k.begin(Some(Layer::Bass), at(1, 0));
        k.note_channel(2);
        k.hold(0.5, 4.0, 60.0);
        k.end();
        k.begin(Some(Layer::Bass), at(4, 0));
        assert!(k.ask(&mut host, 1.0, false, 30.0));
        k.end();
        assert_eq!(host.0, vec![(2, at(1, 0), 0.95, 1.0)]);
        // And with only the kept one, the note is refused.
        host.0.clear();
        k.begin(Some(Layer::Bass), at(8, 0));
        assert!(!k.ask(&mut host, 1.5, false, 90.0));
        k.end();
        assert_eq!(host.0, Vec::new());
    }
}
