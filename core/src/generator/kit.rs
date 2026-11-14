//! Batch 6's kits (queue item 101d), made once the loop's own kit is drawn
//! and its choices are (`Render::choices_part`): each rewrites the kit's
//! hits where the loop drew it, on a stream of its own, so nothing else the
//! loop draws moves.
//!
//! - **tripletKit** (batch 6's #1, approved by Mikey 2026-11-09): a kit in
//!   threes, the 12/8 shuffle of five records. Item 99's lever re-timed the
//!   sixteenths and dropped the eighth, so an eighth-note kit (most of
//!   them) lost its off-beats and gained no thirds ("What the levers did").
//!   This finishes it: the eighth moves to two thirds of the beat (the
//!   shuffle's place), a sixteenth to the third nearer it, and the triplet
//!   hats are written: the first third of every beat gets the loop's own
//!   hat (its instrument, its off-beat velocities and its density) where
//!   nothing of it sits, so a beat plays three. The kick stays on the beat.
//!   6/8 loops never draw it: their beat's thirds are the eighths they play.
//! - **dottedPulse** (batch 6's #8, approved by Mikey 2026-11-09, both
//!   versions): a part every three eighths, a lilt against the bar. `pulse`
//!   adds a soft rim (the hand kit's tap) every six steps across the kit's
//!   layer, at a level of its own (`[setting dottedpulse]`): the lever's
//!   rim at velocity 0.2 read 38 to 43 LU under the loop, too quiet to hear
//!   as a lilt. `kick332` writes the kick whole on a 3-3-2 of the bar's
//!   eighths (0, 6 and 12 of its 16 steps), in place of the loop's own: the
//!   lever only moved the kicks a loop had, so most kits never played it.
//!   Only in a bar of 16 steps (4/4), where it was heard, and never beside a
//!   kit in threes.

use super::data::{MAX_BARS, MAX_STEPS_PER_BAR};
use super::js;
use super::layers::{DrumEvent, Drums};
use super::rng::Rng;

/// tripletKit's own stream (a salt on the loop's seed).
const TRIPLET_SALT: u32 = 0x7472_6970;
/// dottedPulse's own stream.
const DOTTED_SALT: u32 = 0x646f_7473;

/// A beat's steps, in every metre that may draw a kit choice (not 6/8).
const BEAT: f64 = 4.0;

/// The kit's instruments, a bit each, for the places taken.
const KIT: [&str; 12] = ["kick", "softkick", "snare", "clap", "rim", "hat", "ohat", "shaker", "frame", "tap", "jingle", "ojingle"];

/// A beat's three thirds, every beat of the longest loop.
const THIRDS: usize = MAX_BARS * MAX_STEPS_PER_BAR / 4 * 3;

fn bit(inst: &str) -> u16 {
    KIT.iter().position(|k| *k == inst).map_or(1 << 15, |i| 1 << i)
}

/// Whether `inst` is the floor's (the kit's kick, the soft kick, the hand
/// kit's frame).
#[must_use]
pub fn is_kick(inst: &str) -> bool {
    matches!(inst, "kick" | "softkick" | "frame")
}

/// Where third `j` (0 to 2) of the beat that starts at step `beat` sounds,
/// as the sequencer places a hit: a step, and the share of a step past it
/// (`DrumEvent::micro`). The second third is the odd step after the beat,
/// which the player swings by the loop's `swing` (a share of a step, at most
/// 0.3), so its share is a third less the swing and the hit sounds on the
/// third itself; the last is the even step, two thirds past it.
#[must_use]
pub fn third(beat: f64, j: usize, swing: f64) -> (f64, Option<f64>) {
    match j {
        0 => (beat, None),
        1 => {
            let m = 1.0 / 3.0 - swing;
            (beat + 1.0, if m > 1e-9 { Some(m) } else { None })
        }
        _ => (beat + 2.0, Some(2.0 / 3.0)),
    }
}

/// The third a hit at `at` steps (its step and its share past it) goes to in
/// a kit in threes, as its beat's step and the third's number: the nearest,
/// and of two as near the later. So a sixteenth after the beat goes to the
/// first third, the eighth and the sixteenth after it to the second (the
/// shuffle's place), and a roll's hits each to the third nearest it, the
/// next beat's own included.
#[must_use]
pub fn third_of(at: f64) -> (f64, usize) {
    let q = js::floor(at * 3.0 / BEAT + 0.5);
    let beat = js::floor(q / 3.0) * BEAT;
    (beat, js::index(q - js::floor(q / 3.0) * 3.0))
}

/// tripletKit on `drums`, a kit layer of `total` steps of a loop with
/// `swing` and `seed`; `sounds(step)` says whether the kit plays at a step
/// (its entry's bars and the loop's gaps silence it), so a hat written
/// where the kit is silent is silent too, as the loop's own are.
///
/// A kick off the beat goes to the beat nearer it, or the other where that one
/// has a kick, or out where both have (so the kit keeps its kicks, and its
/// low end, as far as it can); any other hit off the beat goes to its third
/// (`third_of`), or out where the same instrument already sits there; a
/// roll's hits too, so a roll of four or eight over a beat plays its thirds.
/// A hit put out stays in its place, silent (velocity 0 as generated too, so
/// no repair brings it back), and the kit's list keeps its order: the
/// wander's draws for it are the same draws. Then the hats are written,
/// after the loop's own hits.
pub fn triplet_kit(drums: &mut Drums, total: f64, swing: f64, seed: u32, sounds: impl Fn(f64) -> bool) {
    let hat = LoopHat::of(drums, total);
    let before = Energy::of(drums);
    let mut taken = [0u16; THIRDS];
    let slot = |beat: f64, j: usize| js::index(beat / BEAT) * 3 + j;
    let at = |e: &DrumEvent| e.step + e.micro.unwrap_or(0.0);
    for e in drums.iter().filter(|e| at(e) % BEAT == 0.0) {
        if let Some(t) = taken.get_mut(slot(at(e), 0)) {
            *t |= bit(e.inst.get());
        }
    }
    for e in drums.iter_mut() {
        let p = at(e);
        if p % BEAT == 0.0 {
            continue;
        }
        let b = bit(e.inst.get());
        // Where it may go, in order: a kick to the beat nearer it (the one
        // before on the eighth), then the other; anything else to its third.
        let (beat, j) = third_of(p);
        let own = p - p % BEAT;
        let (near, far) = if p % BEAT > 2.0 { (own + BEAT, own) } else { (own, own + BEAT) };
        let places = if is_kick(e.inst.get()) { [(near, 0), (far, 0)] } else { [(beat, j), (beat, j)] };
        let free = places.into_iter().find(|&(at, j)| at < total && taken.get(slot(at, j)).is_some_and(|t| t & b == 0));
        if let Some((at, j)) = free {
            if let Some(t) = taken.get_mut(slot(at, j)) {
                *t |= b;
            }
            (e.step, e.micro) = third(at, j, swing);
        } else {
            e.vel = 0.0;
            e.vel0 = 0.0;
        }
    }
    let Some(hat) = hat else { return };
    let mut r = Rng::new(js::salted(seed, TRIPLET_SALT, 1));
    let mut beat = 0.0;
    while beat < total {
        for j in 1..3 {
            let plays = r.chance(hat.density);
            let vel = hat.vel * (0.85 + 0.3 * r.f());
            let free = taken.get(slot(beat, j)).is_some_and(|t| t & hat.family == 0);
            let (step, micro) = third(beat, j, swing);
            if plays && free && step < total {
                drums.push(DrumEvent { step, inst: hat.inst.into(), vel: if sounds(step) { vel } else { 0.0 }, vel0: vel, roll: false, micro });
            }
        }
        beat += BEAT;
    }
    // No brighter than the loop's own kit: the shimmer off the beat (the
    // hats moved and written) scaled so the shimmer's energy against the
    // kick's (which carries the kit's energy; the rest's in a kit without
    // one) is no more than it was (`Energy`).
    let after = Energy::of(drums);
    let (was, is) = if before.floor > 0.0 { (before.floor, after.floor) } else { (before.rest, after.rest) };
    let most = before.shimmer * js::min(1.0, if was > 0.0 { is / was } else { 1.0 });
    let off = after.shimmer - after.on_beat;
    if after.shimmer > most && off > 0.0 {
        let k = crate::math::sqrt(js::max(0.0, most - after.on_beat) / off);
        for e in drums.iter_mut().filter(|e| !e.roll && is_shimmer(e.inst.get()) && e.micro.is_some()) {
            e.vel *= k;
            e.vel0 *= k;
        }
    }
}

/// Whether `inst` is the kit's shimmer: the hats, the shakers, the hand
/// kit's jingles (what sounds above 2 kHz).
fn is_shimmer(inst: &str) -> bool {
    matches!(inst, "hat" | "ohat" | "shaker" | "jingle" | "ojingle")
}

/// A kit's energy by its velocities as generated (each hit's square): its
/// shimmer, the shimmer on the beat, the floor (`is_kick`) and the rest. A
/// kit in threes holds the first against the floor (or the rest) to what
/// the loop's own kit had, so the kit's share above 2 kHz is no higher than
/// the loop's (batch 6's guard).
struct Energy {
    shimmer: f64,
    on_beat: f64,
    floor: f64,
    rest: f64,
}

impl Energy {
    fn of(drums: &Drums) -> Self {
        let mut e = Energy { shimmer: 0.0, on_beat: 0.0, floor: 0.0, rest: 0.0 };
        for h in drums.iter().filter(|h| !h.roll && h.vel0 > 0.0) {
            let x = h.vel0 * h.vel0;
            if is_kick(h.inst.get()) {
                e.floor += x;
            } else if !is_shimmer(h.inst.get()) {
                e.rest += x;
            } else if h.micro.is_none() && h.step % BEAT == 0.0 {
                e.shimmer += x;
                e.on_beat += x;
            } else {
                e.shimmer += x;
            }
        }
        e
    }
}

/// The loop's own hat, as its kit plays it: the instrument it plays most of
/// the hat, the jingle (the hand kit's) and the shaker, that family's bits
/// (closed and open), its mean velocity off the beat, and its density: how
/// many of a beat's two halves have one, over the kit's layer.
struct LoopHat {
    inst: &'static str,
    family: u16,
    vel: f64,
    density: f64,
}

impl LoopHat {
    fn of(drums: &Drums, total: f64) -> Option<Self> {
        let families: [(&'static str, &str); 3] = [("hat", "ohat"), ("jingle", "ojingle"), ("shaker", "shaker")];
        let count = |f: &(&str, &str)| drums.iter().filter(|e| !e.roll && e.vel0 > 0.0 && (e.inst == f.0 || e.inst == f.1)).count();
        let (inst, open) = families.into_iter().max_by_key(count)?;
        let hits = || drums.iter().filter(|e| !e.roll && e.vel0 > 0.0 && (e.inst == inst || e.inst == open));
        let n = hits().count();
        if n == 0 || total <= 0.0 {
            return None;
        }
        let off = hits().filter(|e| e.step % BEAT != 0.0);
        let (sum, k) = off.fold((0.0, 0usize), |(s, k), e| (s + e.vel0, k + 1));
        let vel = if k > 0 { sum / js::num(k) } else { 0.7 * hits().map(|e| e.vel0).sum::<f64>() / js::num(n) };
        let density = js::min(1.0, js::num(n) / (2.0 * total / BEAT));
        Some(LoopHat { inst, family: bit(inst) | bit(open), vel, density })
    }
}

/// dottedPulse's two versions.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Dotted {
    /// A soft part every three eighths.
    Pulse,
    /// The kick on a 3-3-2.
    Kick332,
}

/// The pulse's step: three eighths, six sixteenths.
const PULSE_EVERY: f64 = 6.0;
/// A 3-3-2 of a 16-step bar's eighths, in steps.
const KICK_332: [f64; 3] = [0.0, 6.0, 12.0];
/// The longest loop's bars.
const BARS: usize = MAX_BARS;

/// dottedPulse on `drums`, a kit layer of `total` steps in bars of 16, the
/// version `which`, the loop's `seed`; `hand` for the hand kit, `vel` the
/// pulse's velocity and `low` the 3-3-2's room in dB over the loop's own
/// kick (`[setting dottedpulse]`); `sounds` as `triplet_kit`'s. Its hits are written after
/// the loop's own, which keep their order: the loop's kicks a 3-3-2 takes the
/// place of stay in place, silent, so the wander's draws are the same draws.
///
/// The pulse is struck every six steps from the layer's first, unaware of
/// the bars (the lilt is against them), where the kit has no rim of its own
/// at the step. The 3-3-2 strikes the bar's first eighth at the loop's own
/// downbeat kick and the other two at its other kicks' velocity (each a
/// little apart, on its own stream), with the loop's own kick instrument; a
/// roll of the kick stays.
#[allow(clippy::too_many_arguments)] // the kit, the version, and what of the loop it reads
pub fn dotted_pulse(drums: &mut Drums, which: Dotted, total: f64, seed: u32, hand: bool, (vel, low): (f64, f64), sounds: impl Fn(f64) -> bool, bass_in: impl Fn(f64, f64) -> bool) {
    const BAR: f64 = 16.0;
    let mut r = Rng::new(js::salted(seed, DOTTED_SALT, 1));
    match which {
        Dotted::Pulse => {
            let inst = if hand { "tap" } else { "rim" };
            let mut s = 0.0;
            while s < total {
                let own = drums.iter().any(|e| e.step == s && e.micro.is_none() && e.inst == inst && e.vel != 0.0);
                if !own {
                    drums.push(DrumEvent { step: s, inst: inst.into(), vel: if sounds(s) { vel } else { 0.0 }, vel0: vel, roll: false, micro: None });
                }
                s += PULSE_EVERY;
            }
        }
        Dotted::Kick332 => {
            let floor = |e: &DrumEvent| !e.roll && is_kick(e.inst.get()) && e.vel0 > 0.0;
            let mean = |on: bool| {
                let (sum, n) = drums.iter().filter(|e| floor(e) && ((e.step % BAR == 0.0) == on)).fold((0.0, 0usize), |(s, n), e| (s + e.vel0, n + 1));
                (n > 0).then(|| sum / js::num(n))
            };
            let Some(inst) = KIT.iter().copied().filter(|k| is_kick(k)).max_by_key(|k| drums.iter().filter(|e| floor(e) && e.inst == *k).count()) else { return };
            if !drums.iter().any(floor) {
                return;
            }
            let down = mean(true).or(mean(false)).unwrap_or(0.84);
            let other = mean(false).unwrap_or(down * 0.85);
            // Each bar's own kicks' energy (their velocities as generated,
            // squared), and the half bars where its floor sounds (a kick, or
            // the bass), before its kicks are put out.
            let mut own = [0.0f64; BARS];
            let mut halves = [false; 2 * BARS];
            for e in drums.iter().filter(|e| floor(e)) {
                if let Some(x) = own.get_mut(js::index(js::floor(e.step / BAR))) {
                    *x += e.vel0 * e.vel0;
                }
                if let Some(h) = halves.get_mut(js::index(js::floor(e.step / (BAR / 2.0)))).filter(|_| e.vel != 0.0) {
                    *h = true;
                }
            }
            for (i, h) in halves.iter_mut().enumerate() {
                let from = js::num(i) * BAR / 2.0;
                *h = *h || (from < total && bass_in(from, from + BAR / 2.0));
            }
            for e in drums.iter_mut().filter(|e| floor(e)) {
                e.vel = 0.0;
                e.vel0 = 0.0;
            }
            let room = crate::math::pow(10.0, low / 10.0);
            let mut bar = 0.0;
            while bar < total {
                let was = own.get(js::index(bar / BAR)).copied().unwrap_or(0.0);
                let mut v = [down, js::min(1.0, other * (0.9 + 0.2 * r.f())), js::min(1.0, other * (0.9 + 0.2 * r.f()))];
                // Within the low guard: a bar's kicks no more than `low` dB
                // over the loop's own there, the downbeat kept where it fits.
                let most = was * room;
                let rest = most - v[0] * v[0];
                let after = v[1] * v[1] + v[2] * v[2];
                if rest <= 0.0 {
                    let k = crate::math::sqrt(most / (v[0] * v[0] + after));
                    for x in &mut v {
                        *x *= k;
                    }
                } else if after > rest {
                    let k = crate::math::sqrt(rest / after);
                    v[1] *= k;
                    v[2] *= k;
                }
                for (&at, &vel) in KICK_332.iter().zip(v.iter()) {
                    let step = bar + at;
                    let floored = halves.get(js::index(js::floor(step / (BAR / 2.0)))).copied().unwrap_or(false);
                    if step < total && was > 0.0 {
                        drums.push(DrumEvent { step, inst: inst.into(), vel: if sounds(step) && floored { vel } else { 0.0 }, vel0: vel, roll: false, micro: None });
                    }
                }
                bar += BAR;
            }
        }
    }
}

/// The wander's ghost hit (`wander::Pass::drift`) in a kit in threes: at
/// its third, as the loop's own are, from the same draws.
#[must_use]
pub fn ghost_in_threes(e: DrumEvent, swing: f64) -> DrumEvent {
    if e.step % BEAT == 0.0 {
        return e;
    }
    let (beat, j) = third_of(e.step);
    let (step, micro) = third(beat, j, swing);
    DrumEvent { step, micro, ..e }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::layers::NO_DRUM;

    fn hit(step: f64, inst: &'static str, vel: f64) -> DrumEvent {
        DrumEvent { step, inst: inst.into(), vel, vel0: vel, ..NO_DRUM }
    }

    fn at(d: &Drums, inst: &str, swing: f64) -> Vec<f64> {
        // Where each sounds, in steps, with the swing an odd step takes.
        let mut v: Vec<f64> = d
            .iter()
            .filter(|e| e.vel != 0.0 && e.inst == inst)
            .map(|e| e.step + e.micro.unwrap_or(0.0) + if e.step % 2.0 == 1.0 { swing } else { 0.0 })
            .collect();
        v.sort_by(f64::total_cmp);
        v
    }

    #[test]
    fn an_eighth_note_kit_plays_three_a_beat_on_the_thirds() {
        let swing = 0.2;
        let mut d = Drums::new();
        for s in 0..8 {
            d.push(hit(f64::from(s) * 2.0, "hat", 0.4));
        }
        d.push(hit(0.0, "kick", 0.84));
        d.push(hit(10.0, "kick", 0.7));
        d.push(hit(11.0, "kick", 0.7));
        d.push(hit(3.0, "kick", 0.7));
        d.push(hit(4.0, "snare", 0.6));
        triplet_kit(&mut d, 16.0, swing, 7, |_| true);
        let hats = at(&d, "hat", swing);
        // The eighth at two thirds of every beat, the beat's own kept.
        for b in [0.0, 4.0, 8.0, 12.0] {
            assert!(hats.iter().any(|&x| (x - b).abs() < 1e-9), "{hats:?}");
            assert!(hats.iter().any(|&x| (x - b - 8.0 / 3.0).abs() < 1e-9), "{hats:?}");
        }
        // A third of every beat at the loop's density (all its eighths: 1).
        for b in [0.0, 4.0, 8.0, 12.0] {
            assert!(hats.iter().any(|&x| (x - b - 4.0 / 3.0).abs() < 1e-9), "{hats:?}");
        }
        assert_eq!(hats.len(), 12);
        // No hat on a straight eighth or a sixteenth.
        assert!(hats.iter().all(|&x| ((x % 4.0) * 3.0 / 4.0 - ((x % 4.0) * 3.0 / 4.0).round()).abs() < 1e-9));
        // Each kick to the beat nearer it (the eighth's to the one before), the
        // other where that one has a kick: 10 to 8, 11 to 12, 3 to 4.
        assert_eq!(at(&d, "kick", swing), vec![0.0, 4.0, 8.0, 12.0], "the kick on the beat, none lost");
        assert_eq!(at(&d, "snare", swing), vec![4.0]);
    }

    #[test]
    fn a_sixteenth_kit_keeps_one_hat_a_third_and_a_roll_plays_thirds() {
        let mut d = Drums::new();
        for s in 0..4 {
            d.push(hit(f64::from(s), "hat", 0.35));
        }
        // A roll of eight from step 4: two a step, each half a step apart.
        for k in 0..8 {
            let k = f64::from(k);
            d.push(DrumEvent { roll: true, micro: Some(k / 8.0), ..hit(4.0 + (k * 4.0 / 8.0).floor(), "snare", 0.3) });
        }
        triplet_kit(&mut d, 12.0, 0.0, 1, |_| true);
        let hats = at(&d, "hat", 0.0);
        assert_eq!(hats.iter().filter(|&&x| x < 4.0).count(), 3, "one a third: the sixteenth after the eighth goes out");
        assert!(hats.iter().all(|&x| x < 4.0 || x % 4.0 != 0.0), "the hats written are on the thirds, not the beat");
        let snares = at(&d, "snare", 0.0);
        assert!(snares.iter().all(|&x| ((x * 3.0 / 4.0) - (x * 3.0 / 4.0).round()).abs() < 1e-9), "{snares:?}");
        assert_eq!(snares.len(), 4, "the beat, its thirds and the next beat: {snares:?}");
        assert!(d.iter().filter(|e| e.inst == "snare").all(|e| e.roll), "a roll stays a roll");
    }

    #[test]
    fn a_hat_written_where_the_kit_is_silent_is_silent() {
        let mut d = Drums::new();
        for s in 0..8 {
            d.push(hit(f64::from(s) * 2.0, "hat", 0.4));
        }
        triplet_kit(&mut d, 16.0, 0.0, 3, |step| step < 8.0);
        assert!(d.iter().filter(|e| e.step >= 8.0 && e.micro == Some(1.0 / 3.0)).all(|e| e.vel == 0.0 && e.vel0 > 0.0));
        assert!(d.iter().any(|e| e.step < 8.0 && e.micro == Some(1.0 / 3.0) && e.vel > 0.0));
    }

    #[test]
    fn a_kit_without_hats_gets_none_written() {
        let mut d = Drums::new();
        d.push(hit(0.0, "kick", 0.84));
        d.push(hit(6.0, "rim", 0.22));
        triplet_kit(&mut d, 16.0, 0.0, 3, |_| true);
        assert_eq!(d.len(), 2);
        assert!(matches!(at(&d, "rim", 0.0)[..], [x] if (x - (4.0 + 8.0 / 3.0)).abs() < 1e-9), "the ghost rim at two thirds");
    }

    #[test]
    fn a_pulse_strikes_every_six_steps_where_the_kit_has_no_rim() {
        let mut d = Drums::new();
        d.push(hit(0.0, "kick", 0.84));
        d.push(hit(12.0, "rim", 0.22));
        dotted_pulse(&mut d, Dotted::Pulse, 32.0, 1, false, (0.5, 1.0), |s| s < 24.0, |_, _| true);
        let rims: Vec<(f64, f64)> = d.iter().filter(|e| e.inst == "rim").map(|e| (e.step, e.vel)).collect();
        assert_eq!(rims, vec![(12.0, 0.22), (0.0, 0.5), (6.0, 0.5), (18.0, 0.5), (24.0, 0.0), (30.0, 0.0)], "the loop's own rim at 12, none doubled; silent where the kit is");
        let mut d = Drums::new();
        d.push(hit(0.0, "frame", 0.84));
        dotted_pulse(&mut d, Dotted::Pulse, 16.0, 1, true, (0.5, 1.0), |_| true, |_, _| true);
        assert_eq!(d.iter().filter(|e| e.inst == "tap").count(), 3, "the hand kit's tap");
    }

    #[test]
    fn a_3_3_2_is_written_whole_where_the_loop_plays_a_kick_within_the_low_guard() {
        let kicks = |d: &Drums| d.iter().filter(|e| e.vel != 0.0 && is_kick(e.inst.get())).map(|e| (e.step, e.vel)).collect::<Vec<_>>();
        let energy = |v: &[(f64, f64)], bar: f64| v.iter().filter(|e| e.0 >= bar && e.0 < bar + 16.0).map(|e| e.1 * e.1).sum::<f64>();
        // Four on the floor: three kicks a bar, each bar under its own four.
        let mut d = Drums::new();
        for s in [0.0, 4.0, 8.0, 12.0, 16.0, 20.0, 24.0, 28.0] {
            d.push(hit(s, "kick", if s % 16.0 == 0.0 { 0.84 } else { 0.7 }));
        }
        d.push(hit(4.0, "snare", 0.6));
        dotted_pulse(&mut d, Dotted::Kick332, 32.0, 9, false, (0.5, 1.0), |_| true, |_, _| false);
        let k = kicks(&d);
        assert_eq!(k.iter().map(|e| e.0).collect::<Vec<_>>(), vec![0.0, 6.0, 12.0, 16.0, 22.0, 28.0]);
        assert!(k.iter().all(|e| e.0 % 16.0 != 0.0 || e.1 == 0.84), "the downbeat at the loop's own");
        // One kick a bar: the downbeat kept, the other two soft enough that
        // the bar is within 1 dB of its own; a bar without a kick gets none.
        let mut d = Drums::new();
        d.push(hit(0.0, "kick", 0.84));
        d.push(hit(4.0, "snare", 0.6));
        d.push(hit(20.0, "snare", 0.6));
        dotted_pulse(&mut d, Dotted::Kick332, 32.0, 9, false, (0.5, 1.0), |_| true, |_, _| true);
        let k = kicks(&d);
        assert_eq!(k.len(), 3, "{k:?}");
        assert!(energy(&k, 0.0) <= 0.84 * 0.84 * 1.26 + 1e-9 && k[0].1 == 0.84, "{k:?}");
        // The second half of a bar without the loop's kick or bass there.
        let mut d = Drums::new();
        d.push(hit(0.0, "kick", 0.84));
        d.push(hit(6.0, "kick", 0.7));
        dotted_pulse(&mut d, Dotted::Kick332, 16.0, 9, false, (0.5, 1.0), |_| true, |_, _| false);
        assert_eq!(kicks(&d).iter().map(|e| e.0).collect::<Vec<_>>(), vec![0.0, 6.0], "no kick written where the loop's floor rests");
    }
}
