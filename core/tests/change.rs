//! Changing loops at a musical moment (queue item 86): `Player::change`.
//! The next loop takes its first step on the boundary asked for, where the
//! player said it would; the old one decides nothing past it; the latest
//! change for a boundary wins and the others wait their turn; a crossfade
//! fades the old loop's notes at the boundary and a ring-out doesn't; after
//! the crossfade the new loop sounds as it does loaded alone, but for the
//! old one's room tails; and nothing allocates. `dlrender --changes` makes
//! a thousand and measures the joins.

// Seconds and frames of renders a few seconds long.
#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use dlcore::block::Event;
use dlcore::generator::plan::Cx;
use dlcore::generator::spec::Metre;
use dlcore::player::change::unit;
use dlcore::player::host::Host;
use dlcore::player::{Handover, Moment, Player, Request, State};
use dlcore::{Core, EXTRA, Key, Tap};

struct Counting;

thread_local! {
    static ALLOCATIONS: Cell<usize> = const { Cell::new(0) };
}

fn allocations() -> usize {
    ALLOCATIONS.with(Cell::get)
}

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.with(|n| n.set(n.get() + 1));
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static COUNTING: Counting = Counting;

const RATE: f64 = 44_100.0;
const OLD: u32 = 1_063_107_040;
const NEW: u32 = 663_542_962;

fn roomy(f: impl FnOnce() + Send + 'static) {
    let stack = (std::mem::size_of::<Core>() + std::mem::size_of::<Player>()) * 4 + (16 << 20);
    std::thread::Builder::new().stack_size(stack).spawn(f).expect("a thread").join().expect("the test");
}

/// The core, with what the test watches written down: each note's time
/// and its loop's seed, and each crossfade asked for.
struct Watch<'a> {
    core: &'a mut Core,
    seed: u32,
    notes: &'a mut Vec<(u32, f64)>,
    fades: &'a mut Vec<(f64, f64)>,
}

impl Host for Watch<'_> {
    fn frame(&self) -> u64 {
        self.core.next_frame()
    }
    fn key(&mut self, key: Key) {
        self.seed = key.seed;
        self.core.key(key);
    }
    fn note(&mut self, voice: u32, channel: u32, time: f64, midi: f64, dur: f64, vel: f64, parts: u32, extra: [f64; EXTRA]) {
        self.notes.push((self.seed, time));
        self.core.note(voice, channel, time, midi, dur, vel, parts, extra);
    }
    fn event(&mut self, channel: u32, event: Event) {
        self.core.event(channel, event);
    }
    fn damp(&mut self, channel: u32, time: f64) {
        self.core.damp(channel, time);
    }
    fn set(&mut self, channel: u32, block: usize, setting: usize, value: f64, time: f64) {
        self.core.set(channel, block, setting, value, time);
    }
    fn route(&mut self, channel: u32, block: usize, setting: usize, from: Option<Tap>, time: f64) {
        self.core.route(channel, block, setting, from, time);
    }
    fn param(&mut self, id: u32, op: u32, value: f64, time: f64, tau: f64) {
        self.core.param(id, op, value, time, tau);
    }
    fn bass_lift(&mut self, db: f64) {
        self.core.bass_lift(db);
    }
    fn space_range(&mut self, which: usize, lo: f64, hi: f64) {
        self.core.space_range(which, lo, hi);
    }
    fn steal(&mut self, channel: u32, key: Key, from: f64, to: f64) {
        self.core.steal(channel, key, from, to);
    }
    fn fade_all(&mut self, from: f64, to: f64) {
        self.fades.push((from, to));
        self.core.fade_all(from, to);
    }
}

/// A player playing `seed` from the start, made at once.
fn playing(seed: u32, wander: f64) -> (Box<Core>, Box<Player>) {
    let mut core = Box::new(Core::new());
    let mut player = Box::new(Player::new());
    player.start(&mut core, RATE, 1, true).expect("a player");
    player.set_wander(wander);
    player.load(seed, &Request::new());
    player.ready(&mut *core);
    (core, player)
}

/// What a run saw: the notes (seed, time), the crossfades, and for each
/// loop that began after the first, its seed and its first step's time.
#[derive(Default)]
struct Seen {
    notes: Vec<(u32, f64)>,
    fades: Vec<(f64, f64)>,
    began: Vec<(u32, f64)>,
    said: Vec<f64>,
    /// Stop this long after a loop began (none: render it all).
    stop: Option<f64>,
}

/// Render `seconds` a block at a time, calling `ask` before each block
/// with the block's number.
fn run(core: &mut Core, player: &mut Player, seconds: f64, seen: &mut Seen, mut ask: impl FnMut(&mut Player, usize)) -> Vec<f32> {
    let frames = (seconds * RATE) as usize;
    let mut out = vec![0.0f32; frames];
    let mut playing = player.generator().spec().seed;
    for (b, chunk) in out.chunks_mut(dlcore::QUANTUM).enumerate() {
        ask(player, b);
        core.render_with(&mut [chunk], |core| {
            let mut w = Watch { core, seed: 0, notes: &mut seen.notes, fades: &mut seen.fades };
            if let Some(t) = player.next_change()
                && seen.said.last() != Some(&t)
            {
                seen.said.push(t);
            }
            player.decide(&mut w);
        })
        .expect("a render");
        let now = player.generator().spec().seed;
        if player.state() == State::Playing && now != playing {
            seen.began.push((now, player.position().2));
            playing = now;
        }
        if let (Some(after), Some(&(_, at))) = (seen.stop, seen.began.last())
            && (b * dlcore::QUANTUM) as f64 / RATE > at + after
        {
            break;
        }
    }
    out
}

fn block_at(seconds: f64) -> usize {
    (seconds * RATE / dlcore::QUANTUM as f64) as usize
}

#[test]
fn a_change_lands_on_its_moment() {
    roomy(|| {
        for moment in Moment::ALL {
            let (mut core, mut player) = playing(OLD, 1.0);
            let first = player.position().2;
            let spec = player.generator().spec();
            let sd = 60.0 / spec.bpm / 4.0;
            let beat = if Cx::new(spec).metre == Metre::SixEight { 6.0 } else { 4.0 };
            let grid = unit(moment, beat, spec.steps_per_bar, spec.bars * spec.steps_per_bar);
            let mut seen = Seen { stop: Some(1.5), ..Seen::default() };
            let asked = block_at(2.3);
            run(&mut core, &mut player, 40.0, &mut seen, |p, b| {
                if b == asked {
                    p.change(NEW, &Request::new(), moment, Handover::default());
                }
            });
            let &[(began, at)] = seen.began.as_slice() else { panic!("{moment:?}: one change, not {:?}", seen.began) };
            assert_eq!(began, NEW);
            let steps = (at - first) / sd;
            assert!((steps - steps.round()).abs() < 1e-6, "{moment:?}: on a step");
            if let Some(u) = grid {
                assert_eq!(steps.round() % u, 0.0, "{moment:?}: on its grid");
                assert!(at - 2.3 >= player.lead() - sd, "{moment:?}: at least the lead away");
                assert!(seen.said.iter().any(|&t| (t - at).abs() < 1e-6), "{moment:?}: where it said (to a microsecond: it multiplies, the steps add)");
            }
            // The old loop decides nothing past the boundary (its last step's
            // swing and rolls aside), and every note after is the new one's.
            assert!(seen.notes.iter().filter(|n| n.0 == OLD).all(|n| n.1 < at), "{moment:?}: the old loop stops at the boundary");
            assert!(seen.notes.iter().any(|n| n.0 == NEW && n.1 >= at), "{moment:?}: the new loop plays");
            assert_eq!(seen.fades, vec![(at, at + 1.0)], "{moment:?}: one crossfade, from the boundary");
        }
    });
}

#[test]
fn a_ring_out_fades_nothing() {
    roomy(|| {
        let (mut core, mut player) = playing(OLD, 1.0);
        let mut seen = Seen::default();
        run(&mut core, &mut player, 8.0, &mut seen, |p, b| {
            if b == block_at(2.0) {
                p.change(NEW, &Request::new(), Moment::Bar, Handover::RingOut);
            }
        });
        assert_eq!(seen.began.len(), 1);
        assert_eq!(seen.fades, Vec::<(f64, f64)>::new(), "a ring-out asks for no crossfade");
    });
}

#[test]
fn the_latest_for_a_boundary_wins_and_the_others_wait() {
    roomy(|| {
        let (a, b, c) = (11u32, 22u32, NEW);
        let (mut core, mut player) = playing(OLD, 1.0);
        let mut seen = Seen::default();
        run(&mut core, &mut player, 70.0, &mut seen, |p, blk| {
            if blk == block_at(2.0) {
                p.change(a, &Request::new(), Moment::Bar, Handover::default());
                p.change(c, &Request::new(), Moment::Pass, Handover::default());
                // The same kind again: in place of the first.
                p.change(b, &Request::new(), Moment::Bar, Handover::default());
            }
        });
        let seeds: Vec<u32> = seen.began.iter().map(|x| x.0).collect();
        assert_eq!(seeds, vec![b, c], "the bar's latest first, then the pass's, at the next loop's pass end");
    });
}

#[test]
fn now_while_a_loop_is_made_takes_its_place_at_the_same_moment() {
    roomy(|| {
        let (mut core, mut player) = playing(OLD, 1.0);
        let mut seen = Seen::default();
        let mut wait = None;
        run(&mut core, &mut player, 12.0, &mut seen, |p, b| {
            if b == block_at(2.0) {
                p.change(11, &Request::new(), Moment::Bar, Handover::default());
            }
            if wait.is_none() && p.state() == State::Loading && b > block_at(2.0) {
                wait = Some(b);
                p.change(NEW, &Request::new(), Moment::Now, Handover::default());
            }
        });
        assert!(wait.is_some(), "the bar's loop was being made");
        let &[(began, at)] = seen.began.as_slice() else { panic!("one change, not {:?}", seen.began) };
        assert_eq!(began, NEW);
        assert_eq!(seen.fades, vec![(at, at + 1.0)]);
    });
}

#[test]
fn a_load_or_a_stop_drops_the_changes_waiting() {
    roomy(|| {
        let (mut core, mut player) = playing(OLD, 1.0);
        let mut seen = Seen::default();
        run(&mut core, &mut player, 6.0, &mut seen, |p, b| {
            if b == 10 {
                p.change(11, &Request::new(), Moment::Pass, Handover::default());
            }
            if b == 20 {
                p.load(NEW, &Request::new());
            }
        });
        assert_eq!(seen.began.iter().map(|x| x.0).collect::<Vec<_>>(), vec![NEW]);
        assert!(player.next_change().is_none() && seen.fades.is_empty());
        player.change(11, &Request::new(), Moment::Pass, Handover::default());
        player.stop();
        assert!(player.next_change().is_none());
    });
}

fn rms(x: &[f32]) -> f64 {
    (x.iter().map(|&s| f64::from(s).powi(2)).sum::<f64>() / x.len() as f64).sqrt()
}

#[test]
fn after_the_crossfade_the_new_loop_sounds_as_it_does_alone() {
    roomy(|| {
        // The wander off: a new loop's first pass wanders as the pass the
        // old one had reached (the app's engine's way), so with it on the
        // two would play different first passes. These two loops have no
        // held instrument in common.
        let (mut core, mut player) = playing(OLD, 0.0);
        let mut seen = Seen::default();
        let request = Request::new().with_theme(OLD);
        let changed = run(&mut core, &mut player, 9.0, &mut seen, |p, b| {
            if b == block_at(2.8) {
                p.change(NEW, &request, Moment::Bar, Handover::default());
            }
        });
        let &[(_, at)] = seen.began.as_slice() else { panic!("one change") };
        let mut core = Box::new(Core::new());
        let mut player = Box::new(Player::new());
        player.start(&mut core, RATE, 1, true).expect("a player");
        player.set_wander(0.0);
        player.load_at(NEW, &request, at);
        let alone = run(&mut core, &mut player, 9.0, &mut Seen::default(), |_, _| {});
        let second = |k: f64| ((at + 1.0 + k) * RATE) as usize..((at + 2.0 + k) * RATE) as usize;
        let residual = |k: f64| {
            let d: Vec<f32> = changed[second(k)].iter().zip(&alone[second(k)]).map(|(x, y)| x - y).collect();
            20.0 * (rms(&d) / rms(&alone[second(k)])).log10()
        };
        // The old loop's room and echo tails, falling (-53, -71, -85 dB
        // when this test was written).
        assert!(residual(0.0) < -40.0, "the first second after: {}", residual(0.0));
        assert!(residual(2.0) < -70.0, "the third: {}", residual(2.0));
    });
}

#[test]
fn nothing_allocates_while_changing() {
    roomy(|| {
        let (mut core, mut player) = playing(OLD, 1.0);
        let mut out = vec![0.0f32; 512];
        let before = allocations();
        for b in 0..2000 {
            if b % 400 == 50 {
                let moment = Moment::ALL[(b / 400) % 5];
                player.change(if b % 800 == 50 { NEW } else { OLD }, &Request::new(), moment, Handover::default());
            }
            player.render(&mut core, &mut [&mut out]).expect("a render");
        }
        assert_eq!(allocations(), before, "the player allocated while changing");
    });
}
