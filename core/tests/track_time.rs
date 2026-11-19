//! Session 30c (queue item 111): a track set to a time ends on its second.
//! A track of 1:23:45 is done at 1:23:45 on the core's clock, to the sample,
//! at full and at lite, on a loop, a long piece and Endless, with a tempo
//! change halfway, with each fade on and off: nothing of the loop is decided
//! from that second on, the loop that follows starts on it (or, with none,
//! the end is told there and the loop stops), and the fades are planned on
//! the track's gain to land on their seconds (a fade in up from silence at
//! the start, a fade out at silence on the last second, the notes ended
//! there). The decisions alone, on a clock: the fades as heard are measured
//! on renders (`node tools/track-fades.mjs`).

// Seconds and frames; and the gain's exact values, as the player writes them.
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::float_cmp)]

use dlcore::block::Event;
use dlcore::player::host::Host;
use dlcore::player::track::{FADE_IN, FADE_OUT};
use dlcore::player::{Fades, Handover, Kind, Moment, Player, Request, State, Track};
use dlcore::{EXTRA, Key, Tap, TRACK_GAIN};

const RATE: f64 = 44_100.0;
/// 1:23:45.
const SECONDS: u32 = 5025;
/// The loop that follows.
const NEXT: u32 = 663_542_962;
const SEED: u32 = 1_063_107_040;

fn roomy(f: impl FnOnce() + Send + 'static) {
    let stack = std::mem::size_of::<Player>() * 4 + (16 << 20);
    std::thread::Builder::new().stack_size(stack).spawn(f).expect("a thread").join().expect("the test");
}

/// A host with no audio: the clock, the notes' times, the fades of every
/// note, and the track gain's calls.
#[derive(Default)]
struct Clock {
    frame: u64,
    notes: Vec<f64>,
    fades: Vec<(f64, f64)>,
    gain: Vec<(u32, f64, f64)>,
}

impl Host for Clock {
    fn frame(&self) -> u64 {
        self.frame
    }
    fn key(&mut self, _: Key) {}
    fn note(&mut self, _: u32, _: u32, time: f64, _: f64, _: f64, _: f64, _: u32, _: [f64; EXTRA]) {
        self.notes.push(time);
    }
    fn event(&mut self, _: u32, _: Event) {}
    fn damp(&mut self, _: u32, _: f64) {}
    fn set(&mut self, _: u32, _: usize, _: usize, _: f64, _: f64) {}
    fn route(&mut self, _: u32, _: usize, _: usize, _: Option<Tap>, _: f64) {}
    fn param(&mut self, id: u32, op: u32, value: f64, time: f64, _: f64) {
        if id == TRACK_GAIN {
            self.gain.push((op, value, time));
        }
    }
    fn steal(&mut self, _: u32, _: Key, _: f64, _: f64) {}
    fn fade_all(&mut self, from: f64, to: f64) {
        self.fades.push((from, to));
    }
    fn bass_lift(&mut self, _: f64) {}
    fn space_range(&mut self, _: usize, _: f64, _: f64) {}
}

/// What one track did: its loop's first step, the end told, the next loop's
/// start (if one followed), the last note of the track's own loop, and the
/// host's record.
struct Played {
    start: f64,
    end: f64,
    next: Option<f64>,
    last_note: f64,
    clock: Clock,
}

/// Play `request` as a track of `SECONDS` with `fades`, its tempo moved by
/// a third halfway, the loop `NEXT` armed for its end when `follow`.
fn play(request: &str, full: bool, fades: Fades, follow: bool) -> Played {
    let mut h = Clock::default();
    let mut p = Box::new(Player::new());
    p.attach(&mut h, RATE, full);
    p.load(SEED, &Request::parse(request).expect("a request"));
    // Set as the loop is made: it counts from the loop's first step.
    p.set_track_with(&h, Track::Seconds(SECONDS), fades);
    p.ready(&mut h);
    if follow {
        p.change_with(NEXT, &Request::new(), Moment::End, Handover::Crossfade(1.0), Some((Track::Always, Fades::NONE)));
    }
    let (mut start, mut end, mut next) = (None, None, None);
    let mut moved = false;
    let mut last_note = f64::NEG_INFINITY;
    let mut guard = 0u64;
    loop {
        p.decide(&mut h);
        // Blocks of 4096 frames for speed, and the host's own 128 from a few
        // seconds before the end (a load is a stage a block, timed for them).
        let near = start.is_some_and(|s| h.frame as f64 / RATE > s + f64::from(SECONDS) - 3.0);
        h.frame += if near { 128 } else { 4096 };
        guard += 1;
        assert!(guard < 50_000_000, "{request}: the player never ended");
        while let Some(e) = p.event() {
            match e.kind {
                Kind::Loop if e.number == u64::from(SEED) && start.is_none() => start = Some(e.time),
                Kind::Loop if e.number == u64::from(NEXT) => next = Some(e.time),
                Kind::End => end = Some(e.time),
                _ => {}
            }
        }
        let now = h.frame as f64 / RATE;
        if let Some(s) = start {
            // Halfway, the tempo a third faster: the second doesn't move.
            if !moved && now > s + f64::from(SECONDS) / 2.0 {
                moved = true;
                let bpm = p.generator().spec().bpm;
                p.set_tempo(&mut h, bpm * 4.0 / 3.0);
            }
        }
        if next.is_none() && end.is_none() {
            last_note = h.notes.iter().copied().fold(last_note, f64::max);
            h.notes.clear();
        }
        let done = if follow { next.is_some() } else { end.is_some() && p.state() == State::Idle };
        if done || end.is_some_and(|e| now > e + 30.0) {
            break;
        }
    }
    let start = start.expect("the loop started");
    let end = end.expect("the end told");
    // What the old loop decided after its end was told (none in the app:
    // nothing comes after the step on its second).
    if !follow {
        last_note = h.notes.iter().copied().fold(last_note, f64::max);
    }
    Played { start, end, next, last_note, clock: h }
}

fn check(request: &str, full: bool, fades: Fades, follow: bool) {
    let what = format!("{request} full={full} {fades:?} follow={follow}");
    let r = play(request, full, fades, follow);
    let want = r.start + f64::from(SECONDS);
    // To the sample: the second on the core's clock, a tempo change midway.
    assert!((r.end - want).abs() < 0.5 / RATE, "{what}: ended at {} for {}", r.end - r.start, SECONDS);
    if follow {
        let next = r.next.expect("the next loop");
        assert!((next - want).abs() < 0.5 / RATE, "{what}: the next loop starts at {}", next - r.start);
    }
    assert!(r.last_note < want, "{what}: a note at {} after the end", r.last_note - r.start);
    assert!(r.last_note > want - 5.0, "{what}: the loop stopped playing at {}", r.last_note - r.start);
    let gain = &r.clock.gain;
    // The fade in: from silence on the first step, up over FADE_IN.
    let rises = gain.iter().any(|&(op, v, t)| op == 1 && v == 1.0 && (t - (r.start + FADE_IN)).abs() < 1e-6);
    let silent_start = gain.iter().any(|&(_, v, t)| v == 0.0 && (t - r.start).abs() < 0.03);
    assert_eq!(rises && silent_start, fades.fade_in, "{what}: the fade in");
    // The fade out: the gain at silence on the second, from FADE_OUT before,
    // and every note ended there.
    let falls = gain.iter().any(|&(op, v, t)| op == 1 && v == 0.0 && (t - want).abs() < 1e-6);
    let from_full = gain.iter().any(|&(_, v, t)| v == 1.0 && (t - (want - FADE_OUT)).abs() < 1e-6);
    assert_eq!(falls && from_full, fades.fade_out, "{what}: the fade out");
    let ended = r.clock.fades.iter().any(|&(a, b)| (b - want).abs() < 1e-6 && (a - (want - FADE_OUT)).abs() < 1e-6);
    assert_eq!(ended, fades.fade_out, "{what}: the notes faded to the second");
    if !fades.fade_in && !fades.fade_out {
        assert!(gain.is_empty(), "{what}: a track without fades never moves the gain");
    }
}

const ALL: [Fades; 4] = [
    Fades { fade_in: false, fade_out: false },
    Fades { fade_in: true, fade_out: false },
    Fades { fade_in: false, fade_out: true },
    Fades { fade_in: true, fade_out: true },
];

#[test]
fn a_loop_ends_on_its_second() {
    roomy(|| {
        for full in [true, false] {
            for (i, fades) in ALL.into_iter().enumerate() {
                check("", full, fades, i % 2 == 0);
            }
        }
    });
}

#[test]
fn a_long_piece_ends_on_its_second() {
    roomy(|| {
        for full in [true, false] {
            for (i, fades) in [ALL[0], ALL[3]].into_iter().enumerate() {
                check("bars=32768", full, fades, i == 0);
            }
        }
    });
}

#[test]
fn endless_ends_on_its_second() {
    roomy(|| {
        for full in [true, false] {
            for (i, fades) in [ALL[0], ALL[3]].into_iter().enumerate() {
                check("bars=endless", full, fades, i == 1);
            }
        }
    });
}

#[test]
fn a_time_set_as_it_plays_counts_from_then() {
    roomy(|| {
        let mut h = Clock::default();
        let mut p = Box::new(Player::new());
        p.attach(&mut h, RATE, true);
        p.load(SEED, &Request::new());
        p.ready(&mut h);
        for _ in 0..200 {
            p.decide(&mut h);
            h.frame += 128;
        }
        let set = h.frame as f64 / RATE;
        p.set_track_with(&h, Track::Seconds(30), Fades { fade_in: true, fade_out: true });
        assert!(p.track_at().is_some_and(|t| (t - (set + 30.0)).abs() < 1e-9), "from the block it is set before");
        assert!(p.track_end().is_some(), "a step it falls on");
        // Its fade in has nothing to bring in: the loop has started. Set
        // again, it counts from then.
        p.decide(&mut h);
        assert!(!h.gain.iter().any(|&(_, v, _)| v == 0.0 && h.gain.iter().all(|&(_, _, t)| t < set + 29.0)));
        h.frame += 128 * 50;
        let again = h.frame as f64 / RATE;
        p.set_track_with(&h, Track::Seconds(90), Fades::NONE);
        assert!(p.track_at().is_some_and(|t| (t - (again + 90.0)).abs() < 1e-9));
        // A pause waits: the second moves with the music.
        p.decide(&mut h);
        let before = p.track_at().expect("a time");
        p.stop();
        h.frame += (RATE * 10.0) as u64;
        p.resume(h.frame as f64 / RATE + 0.1);
        p.decide(&mut h);
        let after = p.track_at().expect("a time");
        // By the music's own shift: the next step, a lookahead ahead when it
        // stopped, plays at the resume.
        assert!(after - before > 9.5 && after - before < 10.5, "the pause moved it {}", after - before);
    });
}


