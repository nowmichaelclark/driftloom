//! Queue item 95: the picks through the core's commands. The host's picks
//! are held by the core (`Player::set_picks`) and asked by every loop it
//! makes itself: at the app's Next (`Player::next`) and at a track's end
//! with nothing waiting for it, so the page may sleep through a hand-over.
//! A pick made while a loop plays changes the next loop, never the one
//! playing; the core's own seeds are a pure function of the host's seed;
//! and none of it allocates.

// Seconds and frames of renders a few seconds long.
#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use dlcore::Core;
use dlcore::player::{Event, Handover, Kind, Moment, Player, Request, Track};

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
const SEED: u32 = 1_063_107_040;
const NEXT: u32 = 663_542_962;
const DRAWS: u32 = 20_261_108;

fn roomy(f: impl FnOnce() + Send + 'static) {
    let stack = (std::mem::size_of::<Core>() + std::mem::size_of::<Player>()) * 4 + (16 << 20);
    std::thread::Builder::new().stack_size(stack).spawn(f).expect("a thread").join().expect("the test");
}

fn start(seed: u32, request: &Request) -> (Box<Core>, Box<Player>) {
    let mut core = Box::new(Core::new());
    let mut player = Box::new(Player::new());
    player.start(&mut core, RATE, 1, true).expect("a player");
    player.load(seed, request);
    player.ready(&mut *core);
    (core, player)
}

#[derive(Default)]
struct Seen {
    allocated: usize,
    events: Vec<Event>,
    samples: Vec<f32>,
}

/// Render `seconds` a block at a time, `ask` before each block (with the
/// block's number), the events taken after each.
fn run(core: &mut Core, player: &mut Player, seconds: f64, mut ask: impl FnMut(&mut Core, &mut Player, usize)) -> Seen {
    let mut seen = Seen { events: Vec::with_capacity(1 << 16), samples: Vec::with_capacity((seconds * RATE) as usize), ..Seen::default() };
    let mut out = vec![0.0f32; dlcore::QUANTUM];
    for b in 0..(seconds * RATE) as usize / dlcore::QUANTUM {
        let before = allocations();
        ask(core, player, b);
        player.render(core, &mut [&mut out[..]]).expect("a render");
        while let Some(e) = player.event() {
            seen.events.push(e);
        }
        seen.allocated += allocations() - before;
        seen.samples.extend_from_slice(&out);
    }
    seen
}

fn about(player: &Player) -> String {
    let mut s = String::new();
    player.describe(&mut s).expect("a description");
    s
}

/// The loop `seed` makes with `request`, described, as a host loading it
/// would see it.
fn described(seed: u32, request: &Request) -> String {
    let (_core, player) = start(seed, request);
    about(&player)
}

fn picks() -> Request {
    Request::parse("glacial,bpm=77,bars=12").expect("picks")
}

fn pass_seconds(player: &Player) -> f64 {
    let s = player.generator().spec();
    s.bars * s.steps_per_bar * 60.0 / s.bpm / 4.0
}

#[test]
fn a_track_that_ends_with_nothing_waiting_hands_over_to_a_loop_of_the_cores_own() {
    roomy(|| {
        let (mut core, mut player) = start(SEED, &Request::new());
        player.set_draws(DRAWS);
        player.set_picks(&picks(), 41);
        player.set_track(&*core, Track::Passes(1));
        let seconds = pass_seconds(&player) + 4.0;
        let seen = run(&mut core, &mut player, seconds, |_, _, _| {});
        assert_eq!(seen.allocated, 0, "nothing allocates");
        let e = &seen.events;
        let end = e.iter().position(|x| x.kind == Kind::End).expect("the end told");
        let own = e.iter().position(|x| x.kind == Kind::Own).expect("the core's own loop told");
        assert!(own > end, "after the end");
        assert_eq!(e[own].number, 41, "with the picks' number");
        let lp = e[own + 1];
        assert_eq!(lp.kind, Kind::Loop, "its Loop comes next");
        assert!((lp.time - e[own].time).abs() < 1e-12 && (lp.time - e[end].time).abs() < 1e-9, "on the end");
        let taken = lp.number as u32;
        assert_ne!(taken, SEED);
        // It is the loop that seed makes with the picks, as a host loading it
        // would hear it: the picks honored (77 bpm, 12 bars, glacial).
        let spec = player.generator().spec();
        assert_eq!((spec.seed, spec.bpm, spec.bars), (taken, 77.0, 12.0));
        assert_eq!(about(&player), described(taken, &picks()));
        // It plays on, as a new loop in the app does until its slider moves.
        assert_eq!(player.track(), Track::Always);
        assert_eq!(e.iter().filter(|x| x.kind == Kind::End).count(), 1);

        // The same seed given, the same loop, and the next one another.
        let (mut core2, mut player2) = start(SEED, &Request::new());
        player2.set_draws(DRAWS);
        player2.set_picks(&picks(), 41);
        player2.set_track(&*core2, Track::Passes(1));
        let again = run(&mut core2, &mut player2, seconds, |_, _, _| {});
        assert_eq!(again.samples, seen.samples, "a pure function of what the host sends");
        assert!(player2.next(Moment::Now, Handover::Crossfade(1.0)));
        player2.ready(&mut *core2);
        assert!(!matches!(player2.generator().spec().seed, s if s == taken || s == SEED));
    });
}

#[test]
fn with_no_seed_to_draw_from_the_core_makes_no_loop_of_its_own() {
    roomy(|| {
        let (mut core, mut player) = start(SEED, &Request::new());
        player.set_picks(&picks(), 1);
        assert!(!player.next(Moment::Now, Handover::Crossfade(1.0)), "nothing asked");
        player.set_track(&*core, Track::Passes(1));
        let seconds = pass_seconds(&player) + 3.0;
        let seen = run(&mut core, &mut player, seconds, |_, _, _| {});
        assert!(!seen.events.iter().any(|x| x.kind == Kind::Own));
        assert_eq!(player.generator().spec().seed, SEED, "the loop plays on");
        // And 0 takes them away again.
        player.set_draws(DRAWS);
        player.set_draws(0);
        assert!(!player.next(Moment::Now, Handover::Crossfade(1.0)));
    });
}

#[test]
fn a_pick_made_while_a_loop_plays_changes_the_next_loop_never_the_current_one() {
    roomy(|| {
        // The loop playing, with and without picks set halfway through: the
        // same samples, the same loop.
        let quiet = {
            let (mut core, mut player) = start(SEED, &Request::new());
            player.set_draws(DRAWS);
            run(&mut core, &mut player, 12.0, |_, _, _| {}).samples
        };
        let (mut core, mut player) = start(SEED, &Request::new());
        player.set_draws(DRAWS);
        let before = about(&player);
        let seen = run(&mut core, &mut player, 12.0, |_, p, b| {
            if b == 2000 {
                p.set_picks(&Request::parse("bpm=150,style=tide").expect("picks"), 7);
            }
        });
        assert_eq!(seen.samples, quiet, "the loop playing never changes for a pick");
        assert_eq!(about(&player), before);
        assert_eq!(player.picks().1, 7);
        // The next loop takes them.
        assert!(player.next(Moment::Now, Handover::Crossfade(1.0)));
        let seen = run(&mut core, &mut player, 3.0, |_, _, _| {});
        assert_eq!(seen.allocated, 0, "nothing allocates");
        let own = seen.events.iter().position(|x| x.kind == Kind::Own).expect("told");
        assert_eq!(seen.events[own].number, 7);
        assert_eq!(seen.events[own + 1].kind, Kind::Loop);
        let taken = seen.events[own + 1].number as u32;
        assert_eq!(player.generator().spec().bpm, 150.0);
        assert_eq!(about(&player), described(taken, &Request::parse("bpm=150,style=tide").expect("picks")));
        // A re-roll keeps the request its loop was made with, whatever the
        // picks say now.
        player.set_picks(&Request::parse("bpm=60").expect("picks"), 8);
        let re = picks().with_layer_seed(0, 12_345, true).expect("a seed");
        player.revise(taken, &re);
        run(&mut core, &mut player, 2.0, |_, _, _| {});
        assert_eq!(player.generator().spec().bpm, 77.0, "the re-roll's own request");
        // And they hold across loads until set again.
        player.load(NEXT, &Request::new());
        player.ready(&mut *core);
        assert_eq!(player.picks().1, 8);
    });
}

#[test]
fn a_change_waiting_for_the_end_takes_it_and_the_core_fills_it_when_dropped() {
    roomy(|| {
        // The host's armed loop wins over the core's own, and plays the track
        // it was armed with.
        let (mut core, mut player) = start(SEED, &Request::new());
        player.set_draws(DRAWS);
        player.set_track(&*core, Track::Passes(1));
        player.change_with_track(NEXT, &Request::new(), Moment::End, Handover::Crossfade(1.0), Some(Track::Passes(2)));
        let seconds = pass_seconds(&player) + 3.0;
        let seen = run(&mut core, &mut player, seconds, |_, _, _| {});
        assert!(!seen.events.iter().any(|x| x.kind == Kind::Own));
        assert_eq!(player.generator().spec().seed, NEXT);
        assert_eq!(player.track(), Track::Passes(2), "the armed loop's own track");

        // Dropped, the core's own loop follows instead.
        let (mut core, mut player) = start(SEED, &Request::new());
        player.set_draws(DRAWS);
        player.set_track(&*core, Track::Passes(1));
        player.change(NEXT, &Request::new(), Moment::End, Handover::Crossfade(1.0));
        player.drop_change(Moment::End);
        let seconds = pass_seconds(&player) + 3.0;
        let seen = run(&mut core, &mut player, seconds, |_, _, _| {});
        assert!(seen.events.iter().any(|x| x.kind == Kind::Own));
        assert_ne!(player.generator().spec().seed, NEXT);
        assert_ne!(player.generator().spec().seed, SEED);

        // At a phrase line inside a pass it fades over END_FADE.
        let long = Request::parse("bars=16").expect("sixteen bars");
        let (mut core, mut player) = start(SEED, &long);
        player.set_draws(DRAWS);
        player.set_track(&*core, Track::Bars(1));
        let seen = run(&mut core, &mut player, 20.0, |_, _, _| {});
        let own = seen.events.iter().find(|x| x.kind == Kind::Own).expect("the core's own loop");
        let end = seen.events.iter().find(|x| x.kind == Kind::End).expect("the end");
        assert!((own.time - end.time).abs() < 1e-9 && end.step % (16 * 16) != 0);
    });
}
