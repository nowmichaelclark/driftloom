//! What the player asks of a core (queue item 82): the calls the web host
//! makes to it for the page's engine (`js/core.js`, `CoreHost`), as one
//! trait, so a host can watch them (`dlrender --seed --calls` writes them
//! down beside the page's own, `tools/player.mjs --calls`) at no cost to
//! one that doesn't: the player is generic over it, with no `dyn`.

use crate::block::Event;
use crate::{Core, EXTRA, Key, Tap};

/// The calls a player makes, each the `Core` method of the same name, and
/// the core's clock.
pub trait Host {
    /// `Core::next_frame`: the first frame of the block the core renders
    /// next.
    fn frame(&self) -> u64;
    /// `Core::key`: where the notes that follow sit in their loop.
    fn key(&mut self, key: Key);
    /// `Core::note`.
    #[allow(clippy::too_many_arguments)] // the note's own fields, as `Core::note` takes them
    fn note(&mut self, voice: u32, channel: u32, time: f64, midi: f64, dur: f64, vel: f64, parts: u32, extra: [f64; EXTRA]);
    /// `Core::event`: a choke, a release, a retune.
    fn event(&mut self, channel: u32, event: Event);
    /// `Core::damp`: a new strum.
    fn damp(&mut self, channel: u32, time: f64);
    /// `Core::set`: a setting of a channel's own block.
    fn set(&mut self, channel: u32, block: usize, setting: usize, value: f64, time: f64);
    /// `Core::route`: a channel into a setting of another's own block.
    fn route(&mut self, channel: u32, block: usize, setting: usize, from: Option<Tap>, time: f64);
    /// `Core::param`: the mix's and the master chain's parameters.
    fn param(&mut self, id: u32, op: u32, value: f64, time: f64, tau: f64);
    /// `Core::steal`: the notes of `channel` keyed `key` fall to nothing
    /// from `from` to `to` and end (the voice budget taking room from a
    /// layer's oldest notes, queue item 85c).
    fn steal(&mut self, channel: u32, key: Key, from: f64, to: f64);
    /// `Core::fade_all`: every note sounding or waiting falls to nothing
    /// from `from` to `to` and ends (a loop handing over by a crossfade,
    /// queue item 86).
    fn fade_all(&mut self, from: f64, to: f64);
    /// `Core::bass_lift`: the bass layer's level for the loop.
    fn bass_lift(&mut self, db: f64);
    /// `Core::space_range`: a space setting's range, from the next loop's
    /// start (a request's space pins, queue item 85b).
    fn space_range(&mut self, which: usize, lo: f64, hi: f64);
}

impl Host for Core {
    fn frame(&self) -> u64 {
        self.next_frame()
    }

    fn key(&mut self, key: Key) {
        Core::key(self, key);
    }

    fn note(&mut self, voice: u32, channel: u32, time: f64, midi: f64, dur: f64, vel: f64, parts: u32, extra: [f64; EXTRA]) {
        Core::note(self, voice, channel, time, midi, dur, vel, parts, extra);
    }

    fn event(&mut self, channel: u32, event: Event) {
        Core::event(self, channel, event);
    }

    fn damp(&mut self, channel: u32, time: f64) {
        Core::damp(self, channel, time);
    }

    fn set(&mut self, channel: u32, block: usize, setting: usize, value: f64, time: f64) {
        Core::set(self, channel, block, setting, value, time);
    }

    fn route(&mut self, channel: u32, block: usize, setting: usize, from: Option<Tap>, time: f64) {
        Core::route(self, channel, block, setting, from, time);
    }

    fn param(&mut self, id: u32, op: u32, value: f64, time: f64, tau: f64) {
        Core::param(self, id, op, value, time, tau);
    }

    fn bass_lift(&mut self, db: f64) {
        Core::bass_lift(self, db);
    }

    fn steal(&mut self, channel: u32, key: Key, from: f64, to: f64) {
        Core::steal(self, channel, key, from, to);
    }

    fn fade_all(&mut self, from: f64, to: f64) {
        Core::fade_all(self, from, to);
    }

    fn space_range(&mut self, which: usize, lo: f64, hi: f64) {
        Core::space_range(self, which, lo, hi);
    }
}
