//! The player (queue item 82): a loop from a seed, played in the core. Its
//! samples are held to the page's elsewhere (`tools/baseline.mjs`'s
//! `player` part, `core/golden/seeds.hashes`); these hold it to itself: a
//! load spread across blocks hands over the notes one made at once does,
//! any block size renders the same samples, the kick's fit comes out the
//! same in any slices, nothing allocates while it plays, and its memory is
//! fixed.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use dlcore::block::Event;
use dlcore::generator::Generator;
use dlcore::generator::choices::{KICK_SLICE, KickFit, kick_of};
use dlcore::player::host::Host;
use dlcore::player::{Player, Request, State};
use dlcore::{Core, EXTRA, Key, Tap};

/// The allocations each thread makes, counted: the render must make none.
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

/// A thread with room for a core and a player built on its stack (a debug
/// build makes them there before boxing them).
fn roomy(f: impl FnOnce() + Send + 'static) {
    let stack = (std::mem::size_of::<Core>() + std::mem::size_of::<Player>()) * 4 + (16 << 20);
    std::thread::Builder::new().stack_size(stack).spawn(f).expect("a thread").join().expect("the test");
}

fn start(rate: f64, channels: u32) -> (Box<Core>, Box<Player>) {
    let mut core = Box::new(Core::new());
    let mut player = Box::new(Player::new());
    player.start(&mut core, rate, channels, true).expect("a player");
    (core, player)
}

/// `frames` frames of the loop of `seed`, rendered `split` frames a call.
fn render(core: &mut Core, player: &mut Player, frames: usize, split: usize) -> Vec<f32> {
    let mut out = vec![0.0f32; frames];
    for chunk in out.chunks_mut(split) {
        player.render(core, &mut [chunk]).expect("a render");
    }
    out
}

/// The core, with every note handed to it written down (its fields, as
/// text, bit for bit).
struct Notes<'a> {
    core: &'a mut Core,
    notes: &'a mut Vec<String>,
}

impl Host for Notes<'_> {
    fn frame(&self) -> u64 {
        self.core.next_frame()
    }
    fn key(&mut self, key: Key) {
        self.core.key(key);
    }
    fn note(&mut self, voice: u32, channel: u32, time: f64, midi: f64, dur: f64, vel: f64, parts: u32, extra: [f64; EXTRA]) {
        let bits: Vec<u64> = [time, midi, dur, vel].iter().chain(&extra).map(|x| x.to_bits()).collect();
        self.notes.push(format!("{voice} {channel} {parts} {bits:?}"));
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
}

/// The notes the player hands over in `frames` frames, the loop asked for
/// with its first step at `at` and made a stage a block, or at once.
fn notes_of(seed: u32, at: f64, at_once: bool, frames: usize) -> (Vec<String>, State) {
    let (mut core, mut player) = start(44_100.0, 1);
    player.load_at(seed, &Request::new(), at);
    if at_once {
        player.ready(&mut *core);
    }
    let mut notes = Vec::new();
    let mut out = vec![0.0f32; frames];
    core.render_with(&mut [&mut out], |core| {
        player.decide(&mut Notes { core, notes: &mut notes });
    })
    .expect("a render");
    (notes, player.state())
}

#[test]
fn a_load_spread_across_blocks_plays_as_one_made_at_once() {
    roomy(|| {
        // The first step a second on, so the spread load is made by then. The
        // load's own settings land when it is made, so the samples differ in
        // their last bits; the notes are the same.
        for seed in [2_693_262_067, 306_182_396] {
            let (spread, state) = notes_of(seed, 1.0, false, 441_000);
            let (at_once, _) = notes_of(seed, 1.0, true, 441_000);
            assert_eq!(state, State::Playing);
            assert!(spread.len() > 20, "seed {seed}: the loop plays");
            assert!(spread == at_once, "seed {seed}: a spread load and one made at once hand over the same notes");
        }
    });
}

#[test]
fn the_same_seed_renders_the_same_samples_again() {
    roomy(|| {
        let mut runs = Vec::new();
        for _ in 0..2 {
            let (mut core, mut player) = start(44_100.0, 1);
            player.load(3_963_855_042, &Request::new());
            runs.push(render(&mut core, &mut player, 300_000, 128));
        }
        assert!(runs[0] == runs[1], "a spread load renders alike every time");
    });
}

#[test]
fn any_block_size_renders_the_same_samples() {
    roomy(|| {
        let mut runs = Vec::new();
        for split in [128, 1, 37, 1000, 4096] {
            let (mut core, mut player) = start(48_000.0, 1);
            player.load(11_749_833, &Request::new().with_vibe("peaceful").expect("a vibe"));
            player.ready(&mut *core);
            runs.push(render(&mut core, &mut player, 240_000, split));
        }
        assert!(runs.iter().all(|r| *r == runs[0]), "every split renders alike");
    });
}

#[test]
fn rendering_allocates_nothing() {
    roomy(|| {
        let (mut core, mut player) = start(44_100.0, 2);
        let mut left = vec![0.0f32; 4096];
        let mut right = vec![0.0f32; 4096];
        let before = allocations();
        // A load spread across the blocks, the loop played, a new one asked
        // for while it plays, and the controls.
        player.load(306_182_396, &Request::new());
        for i in 0..400 {
            if i == 200 {
                player.load(3_963_855_042, &Request::new());
            }
            if i == 300 {
                player.set_mute(&mut *core, dlcore::generator::Layer::Drums, true);
                player.set_tempo(&mut *core, 90.0);
                player.set_wander(2.0);
            }
            player.render(&mut core, &mut [&mut left, &mut right]).expect("a render");
        }
        assert_eq!(allocations(), before, "the player allocated while it played");
        assert_eq!(player.state(), State::Playing);
    });
}

#[test]
fn the_kick_fits_alike_in_any_slices() {
    for (seed, root) in [(1, 0.0), (2_693_262_067, 5.0), (77, 11.0), (3_963_855_042, 3.0)] {
        let whole = kick_of(seed, root);
        for budget in [1, 7, 333, KICK_SLICE] {
            let mut fit = KickFit::new(seed, root);
            let mut slices = 0;
            while !fit.step(budget) {
                slices += 1;
                assert!(slices < 10_000_000, "the fit ends");
            }
            let k = fit.kick();
            assert_eq!(format!("{k:?}"), format!("{whole:?}"), "seed {seed}, slices of {budget}");
        }
    }
}

#[test]
fn its_memory_is_fixed() {
    // Reported, not pinned: a 32-bit target's are smaller.
    println!(
        "core {} bytes, player {} bytes (its generator {} bytes)",
        std::mem::size_of::<Core>(),
        std::mem::size_of::<Player>(),
        std::mem::size_of::<Generator>()
    );
}
