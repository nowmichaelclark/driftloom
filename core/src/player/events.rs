//! What the player tells a host about where the loop is (queue item 94a):
//! events, each at the time on the core's clock it is heard, so a host can
//! draw the cursor, the pass, the phrase lines and a piece's sections as the
//! music reaches them, with nothing of its own deciding when.
//!
//! **Decided ahead, heard later.** An event is made when the player decides
//! its step, up to `LOOKAHEAD` before it sounds (a change's boundary, up to
//! `Player::lead`), and carries the time it is heard. A host takes them as
//! they come (`Player::event`, after each render) and shows each at its time:
//! the app's page queues them against the audio clock, as it queues the
//! steps it decides today. They never change what plays.
//!
//! **Fixed room.** The events wait in a ring of `EVENTS_MAX`, taken oldest
//! first; a host that leaves them longer than that loses the newest
//! (`Player::events_lost` counts them), and nothing else. A step is a tenth of
//! a second or so, so the ring holds some twenty seconds of a busy loop.

use crate::generator::list::Empty;

/// How many events wait for the host at most.
pub const EVENTS_MAX: usize = 256;

/// What an event says. The numbers hosts know them by are their places here
/// (0 to 7).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Kind {
    /// A step: `number` is the step in its pass (in a piece, in its
    /// section). First, so that an empty event is all zero bytes.
    Step = 0,
    /// A pass of the loop begins (a loop's own; in a piece, a round of all
    /// its sections): `number` is the pass, from the loop's first (0).
    Pass = 1,
    /// A section of a piece begins (queue item 88): `number` is the
    /// section, from the piece's first (0). A loop with no length asked has
    /// none.
    Section = 2,
    /// A phrase line (`Moment::Phrase`'s grid; the length slider's lines
    /// inside a pass): `number` is the step in its pass.
    Phrase = 3,
    /// A new loop takes its first step: `number` is its seed.
    Loop = 4,
    /// The loop playing, made again in its place (`Player::revise`, a layer
    /// re-rolled): `number` is its seed; the steps go on from where they
    /// were.
    Revised = 5,
    /// The track ends (`Player::set_track`): the length the host set has
    /// run. `number` is 0. If a change waits for it (`Moment::End`), its
    /// loop starts here; if not, the loop plays on.
    End = 6,
    /// The loop whose `Loop` comes next, at the same time, is one the core
    /// made itself (queue item 95: `Player::next`, or a track's end with
    /// nothing waiting for it): its seed drawn from the host's
    /// (`Player::set_draws`), with the picks the host numbered `number`
    /// (`Player::set_picks`), so the host knows which loop it is.
    Own = 7,
}

impl Kind {
    /// Every kind, in the hosts' numbering.
    pub const ALL: [Kind; 8] = [Kind::Step, Kind::Pass, Kind::Section, Kind::Phrase, Kind::Loop, Kind::Revised, Kind::End, Kind::Own];
}

/// An event: what it says, when it is heard, and where the loop is then.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Event {
    /// When it is heard, in seconds on the core's clock.
    pub time: f64,
    /// What it says.
    pub kind: Kind,
    /// Its number (`Kind` says what it counts).
    pub number: u64,
    /// The step it falls on, from the loop's load (it never resets in a
    /// loop: a re-roll keeps it).
    pub step: u64,
}

impl Empty for Event {
    const EMPTY: Self = Event { time: 0.0, kind: Kind::Step, number: 0, step: 0 };
}

/// The events waiting for the host: a ring, oldest first.
pub struct Events {
    ring: [Event; EVENTS_MAX],
    head: usize,
    len: usize,
    lost: u32,
}

impl Events {
    /// None waiting. All zero bytes, as the player's start must be.
    #[must_use]
    pub const fn new() -> Self {
        Events { ring: [Event::EMPTY; EVENTS_MAX], head: 0, len: 0, lost: 0 }
    }

    /// Add one; with the ring full it is lost and counted.
    pub fn push(&mut self, event: Event) {
        if self.len >= EVENTS_MAX {
            self.lost = self.lost.saturating_add(1);
            return;
        }
        let at = (self.head + self.len) % EVENTS_MAX;
        if let Some(slot) = self.ring.get_mut(at) {
            *slot = event;
            self.len += 1;
        }
    }

    /// Take the oldest.
    pub fn pop(&mut self) -> Option<Event> {
        if self.len == 0 {
            return None;
        }
        let e = self.ring.get(self.head).copied();
        self.head = (self.head + 1) % EVENTS_MAX;
        self.len -= 1;
        e
    }

    /// How many wait.
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether none wait.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// How many were lost for want of room since the player started.
    #[must_use]
    pub fn lost(&self) -> u32 {
        self.lost
    }

    /// Forget every event waiting (a player started again).
    pub fn clear(&mut self) {
        self.head = 0;
        self.len = 0;
    }
}

impl Default for Events {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(t: f64) -> Event {
        Event { time: t, kind: Kind::Step, number: 0, step: 0 }
    }

    #[test]
    fn the_ring_keeps_its_order_and_counts_what_it_loses() {
        let mut e = Events::new();
        assert!(e.pop().is_none());
        for i in 0..EVENTS_MAX + 3 {
            e.push(at(f64::from(u32::try_from(i).unwrap_or(0))));
        }
        assert_eq!((e.len(), e.lost()), (EVENTS_MAX, 3));
        assert_eq!(e.pop().map(|x| x.time), Some(0.0));
        e.push(at(1000.0));
        let mut last = None;
        while let Some(x) = e.pop() {
            last = Some(x.time);
        }
        assert_eq!(last, Some(1000.0));
        assert!(e.is_empty());
        assert_eq!(Kind::ALL[6], Kind::End);
    }
}
