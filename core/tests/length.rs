//! Length, up to 32,768 bars and Endless (queue item 88; `generator::piece`
//! says what a piece is). A length of up to 32 bars is the loop's own, in
//! every profile, honored where the tables never draw it; a longer one
//! comes round on exactly its length, a round's last section cut short at a
//! bar line where the loop's length doesn't divide it, and no note reaches
//! the core from past the cut; a section drawn straight from the seed and
//! its number is the one the player plays reaching it by playing through,
//! up to the thousandth and the last of a 32,768-bar piece; and nothing
//! allocates however long it plays. The 24-hour Endless render is
//! `dlrender --endless 24` (CI's `endless` job).
//!
//! The player is played here on a host that keeps the core's clock and
//! counts what it is handed, renders nothing: a 32,768-bar piece is hours
//! of music, and what is checked is what the player decides.

// Steps, frames and bars, far below 2^53.
#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use dlcore::block::Event;
use dlcore::generator::Generator;
use dlcore::generator::canon::Fnv;
use dlcore::generator::data::CHARACTERS;
use dlcore::generator::piece::{self, Piece, SECTION_BARS};
use dlcore::generator::request::{BAD_LENGTH, LENGTH_MOST, Length};
use dlcore::generator::rng::Rng;
use dlcore::generator::wander::AMOUNT;
use dlcore::player::host::Host;
use dlcore::player::{Player, Request, State};
use dlcore::{EXTRA, Key, Tap};

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
/// How far the host's clock moves between two of the player's decisions:
/// under its lookahead, so every step is decided before it is due.
const STRIDE: u64 = 4096;

fn roomy(f: impl FnOnce() + Send + 'static) {
    let stack = (std::mem::size_of::<Player>() + std::mem::size_of::<Generator>()) * 4 + (16 << 20);
    std::thread::Builder::new().stack_size(stack).spawn(f).expect("a thread").join().expect("the test");
}

/// A core's clock and nothing else: what the player hands over is counted,
/// the steps of each note's key kept as the highest seen in its pass.
#[derive(Default)]
struct Clock {
    frame: u64,
    notes: u64,
    /// The pass and step of the last key handed over.
    key: (u32, u32),
    /// The highest step a note was handed over at, in the pass of `key`.
    top: Option<(u32, u32)>,
    /// Each pass ended, by its number, and the highest step a note of it
    /// was handed at.
    tops: Vec<(u32, u32)>,
}

impl Host for Clock {
    fn frame(&self) -> u64 {
        self.frame
    }
    fn key(&mut self, key: Key) {
        self.key = (key.pass, key.step);
    }
    fn note(&mut self, _: u32, _: u32, _: f64, _: f64, _: f64, _: f64, _: u32, _: [f64; EXTRA]) {
        self.notes += 1;
        let (pass, step) = self.key;
        match self.top {
            Some((p, s)) if p == pass => self.top = Some((p, s.max(step))),
            Some(t) => {
                self.tops.push(t);
                self.top = Some((pass, step));
            }
            None => self.top = Some((pass, step)),
        }
    }
    fn event(&mut self, _: u32, _: Event) {}
    fn damp(&mut self, _: u32, _: f64) {}
    fn set(&mut self, _: u32, _: usize, _: usize, _: f64, _: f64) {}
    fn route(&mut self, _: u32, _: usize, _: usize, _: Option<Tap>, _: f64) {}
    fn param(&mut self, _: u32, _: u32, _: f64, _: f64, _: f64) {}
    fn steal(&mut self, _: u32, _: Key, _: f64, _: f64) {}
    fn fade_all(&mut self, _: f64, _: f64) {}
    fn bass_lift(&mut self, _: f64) {}
    fn space_range(&mut self, _: usize, _: f64, _: f64) {}
}

/// The request for a piece of `bars` bars (`f64::INFINITY`: Endless).
fn asking(bars: f64) -> Request {
    let length = if bars.is_infinite() { Length::Endless } else { Length::Bars(bars) };
    Request::new().with_length(length).expect("a length")
}

/// A player on the clock, the loop of `seed` as `request` asks made and
/// started.
fn started(seed: u32, request: &Request) -> (Clock, Box<Player>) {
    let mut clock = Clock::default();
    let mut player = Box::new(Player::new());
    player.attach(&mut clock, RATE, true);
    player.load(seed, request);
    player.ready(&mut clock);
    assert_eq!(player.state(), State::Playing);
    (clock, player)
}

/// Where each pass began, in steps from the load, as the player stands
/// after each decision: pass `n` began `starts[n]` steps in. Plays until
/// `passes` passes have begun past the first, calling `each` at the top of
/// each pass played (its number) with the player.
fn play(clock: &mut Clock, player: &mut Player, passes: u32, mut each: impl FnMut(u32, &Player)) -> Vec<u64> {
    let mut starts = vec![0u64];
    let mut seen = 0;
    each(0, player);
    let mut guard = 0u64;
    while seen < passes {
        player.decide(clock);
        clock.frame += STRIDE;
        let (step, pass, _) = player.position();
        while seen < pass {
            seen += 1;
            // The pass playing began `step` steps before the next to decide.
            let abs = player_abs(player);
            starts.push(abs - step);
            each(seen, player);
        }
        guard += 1;
        assert!(guard < 50_000_000, "the player stopped deciding");
    }
    starts
}

/// The steps the player has decided since the load.
fn player_abs(player: &Player) -> u64 {
    player.steps()
}


/// One seed whose loop each profile leads (the heaviest of its mix), from
/// their own stream, with `request` asked: sixteen loops.
fn by_profile(request: &Request) -> Vec<(usize, u32)> {
    let mut found: Vec<Option<u32>> = vec![None; CHARACTERS.len()];
    let mut rng = Rng::new(88);
    let mut g = Box::new(Generator::new());
    let mut tries = 0;
    while found.iter().any(Option::is_none) {
        let seed = rng.seed32();
        g.begin_with(seed, request);
        // The spec is the first stage: enough to read the mix.
        g.step();
        let lead = g.spec().mix.iter().fold((0, -1.0), |best, &(k, w)| if w > best.1 { (k, w) } else { best }).0;
        if found[lead].is_none() {
            found[lead] = Some(seed);
        }
        tries += 1;
        assert!(tries < 200_000, "a profile no loop leads");
    }
    found.into_iter().enumerate().map(|(i, s)| (i, s.expect("found"))).collect()
}

#[test]
fn a_length_reads_and_refuses() {
    assert_eq!(Request::parse("bars=1024").expect("a length").length(), Some(Length::Bars(1024.0)));
    assert_eq!(Request::parse("bars=endless,glacial").expect("a length").length(), Some(Length::Endless));
    assert_eq!(Request::parse("bars=9007199254740991").expect("the longest").length(), Some(Length::Bars(LENGTH_MOST)));
    for bad in ["bars=0", "bars=", "bars", "bars=1.5", "bars=-4", "bars=forever", "bars=9007199254740992", "bars=9007199254740993"] {
        assert_eq!(Request::parse(bad).err(), Some(BAD_LENGTH), "{bad}");
    }
    assert_eq!(Request::new().with_length(Length::Bars(f64::NAN)).err(), Some(BAD_LENGTH));
    assert_eq!(Request::new().length(), None);
}

#[test]
fn a_short_length_is_the_loops_own_in_every_profile() {
    roomy(|| {
        for bars in [1.0, 2.0, 3.0, 7.0, 24.0, 32.0] {
            let request = asking(bars);
            for (profile, seed) in by_profile(&request) {
                let (mut clock, mut player) = started(seed, &request);
                let g = player.generator();
                let name = CHARACTERS[profile].id;
                assert_eq!(g.spec().bars, bars, "{name} {seed}: the loop's bars");
                assert_eq!(g.lost(), 0, "{name} {seed}: the lists held it");
                let spb = g.spec().steps_per_bar;
                let piece = Piece::of(g.spec()).expect("a piece");
                assert_eq!((piece.end, piece.cut()), (Some((1, bars)), false));
                // Three passes, each exactly the length asked.
                let starts = play(&mut clock, &mut player, 3, |_, _| {});
                for w in starts.windows(2) {
                    assert_eq!((w[1] - w[0]) as f64, bars * spb, "{name} {seed}: a pass of {bars} bars");
                }
                // No note handed over past the pass's end.
                for &(_, top) in &clock.tops {
                    assert!(f64::from(top) < bars * spb, "{name} {seed}: a note at step {top}");
                }
                assert!(clock.notes > 0, "{name} {seed}: it plays");
            }
        }
    });
}

#[test]
fn nothing_asked_draws_as_ever() {
    roomy(|| {
        // The table's draw is made and overruled: every other draw of the
        // spec is where it was, for a length or none.
        let mut rng = Rng::new(8801);
        let mut a = Box::new(Generator::new());
        let mut b = Box::new(Generator::new());
        for _ in 0..200 {
            let seed = rng.seed32();
            a.begin_with(seed, &Request::new());
            a.step();
            for asked in [3.0, 1024.0, f64::INFINITY] {
                b.begin_with(seed, &asking(asked));
                b.step();
                let (x, y) = (a.spec(), b.spec());
                assert_eq!((x.bpm, x.root, x.scale, x.swing, x.layer_seeds, x.tone), (y.bpm, y.root, y.scale, y.swing, y.layer_seeds, y.tone), "{seed}");
                assert_eq!(x.steps_per_bar, y.steps_per_bar, "{seed}");
                // A long length keeps the loop's own bars.
                if asked > SECTION_BARS {
                    assert_eq!(x.bars, y.bars, "{seed}");
                }
            }
        }
    });
}

/// A piece of `bars` bars on the loop of `seed`, played one round and into
/// the next: the round is exactly `bars` bars, its sections the loop's
/// length but a cut last, the next round begins where it ends, and no note
/// is handed over from past a section's end. Returns the player at the top
/// of the next round, with what the round's last section hashed as played.
fn round(seed: u32, bars: f64) -> (Box<Player>, u64) {
    let request = asking(bars);
    let (mut clock, mut player) = started(seed, &request);
    let g = player.generator();
    let spb = g.spec().steps_per_bar;
    let piece = Piece::of(g.spec()).expect("a piece");
    let (sections, last) = piece.end.expect("a length");
    let mut last_hash = 0;
    let passes = u32::try_from(sections).expect("sections");
    let starts = play(&mut clock, &mut player, passes + 1, |n, p| {
        if n + 1 == passes {
            last_hash = pass_hash(p);
        }
    });
    let n = starts.len() - 1;
    assert_eq!(starts[passes as usize] as f64, bars * spb, "{seed}: a round of {bars} bars");
    for (i, w) in starts.windows(2).enumerate().take(passes as usize) {
        let want = if i + 1 == passes as usize { last } else { piece.bars };
        assert_eq!((w[1] - w[0]) as f64, want * spb, "{seed}: section {i} of {sections}");
    }
    assert!(n > passes as usize);
    for &(pass, top) in &clock.tops {
        let len = piece.bars_of(u64::from(pass)) * spb;
        assert!(f64::from(top) < len, "{seed}: section {pass}: a note at step {top} of {len}");
    }
    (player, last_hash)
}

/// The pass the player plays now, hashed as the goldens hash a pass (0 for
/// the loop as composed).
fn pass_hash(player: &Player) -> u64 {
    let mut h = Fnv::new();
    match player.pass() {
        Some(p) => player.generator().write_pass(p, &mut h),
        None => return 0,
    }
    h.0
}

/// Section `n` of the loop of `seed` as `request` asks it, drawn straight:
/// a generator of its own, made from the seed, and the number.
fn straight(seed: u32, request: &Request, n: u64) -> u64 {
    let mut g = Box::new(Generator::new());
    g.load_with(seed, request);
    g.section_hash(n, AMOUNT)
}

#[test]
fn a_long_piece_comes_round_on_its_length() {
    roomy(|| {
        // Every profile at 1,024 bars and at a length its loop doesn't
        // divide (100: a section of 16 or 24 bars leaves 4).
        for bars in [1024.0, 100.0, 33.0] {
            for (profile, seed) in by_profile(&asking(bars)) {
                let (player, _) = round(seed, bars);
                let p = player.piece().expect("a piece");
                assert_eq!(p.length(), Some(bars), "{} {seed}", CHARACTERS[profile].id);
            }
        }
    });
}

#[test]
fn the_last_section_of_32768_bars_is_reached_straight() {
    roomy(|| {
        // Three loops: a loop of 2 bars (16,384 sections), of 8 and of 24
        // (1,366 sections, the last 8 bars).
        for seed in [1_027_928_796, 4_272_732_017, 2_428_270_433] {
            let request = asking(32_768.0);
            let before = allocations();
            let (player, last) = round(seed, 32_768.0);
            let (sections, _) = player.piece().and_then(|p| p.end).expect("a length");
            assert_eq!(last, straight(seed, &request, sections - 1), "{seed}: the last section");
            assert_ne!(last, 0, "{seed}: it wanders");
            // Nothing allocated while it played (the test's own lists aside:
            // a vector of the passes' starts and tops, a few dozen growths).
            assert!(allocations() - before < 200, "{seed}: {} allocations", allocations() - before);
        }
    });
}

#[test]
fn a_section_is_the_same_reached_straight_or_through() {
    roomy(|| {
        for (seed, bars) in [(1_063_107_040, 4096.0), (663_542_962, f64::INFINITY)] {
            let request = asking(bars);
            let (mut clock, mut player) = started(seed, &request);
            let mut through = Vec::new();
            play(&mut clock, &mut player, 1000, |_, p| through.push(pass_hash(p)));
            let mut g = Box::new(Generator::new());
            g.load_with(seed, &request);
            for (n, &h) in through.iter().enumerate() {
                assert_eq!(h, g.section_hash(n as u64, AMOUNT), "{seed}: section {n}");
                assert_ne!(h, 0);
            }
            // And the generator's own wandered pass is the same draw.
            let mut w = Fnv::new();
            g.write(dlcore::generator::Part::Pass(7), &mut w);
            assert_eq!(w.0, through[7], "{seed}: pass 7");
        }
    });
}

#[test]
fn endless_and_long_pieces_allocate_nothing() {
    roomy(|| {
        let seed = 1_027_928_796;
        for bars in [4.0, 32_768.0, f64::INFINITY] {
            let (mut clock, mut player) = started(seed, &asking(bars));
            // Warm the test's own vectors first.
            clock.tops.reserve(1 << 16);
            let before = allocations();
            for _ in 0..200_000 {
                player.decide(&mut clock);
                clock.frame += STRIDE;
            }
            assert_eq!(allocations(), before, "{bars} bars");
            assert_eq!(player.budget_full(), 0);
        }
        assert_eq!(piece::pass_number(u64::from(u32::MAX) + 3), 2);
    });
}
