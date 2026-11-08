//! Queue item 94a: everything the app does, as a command or an event. The
//! events say where the loop is, in time and in order; a code's edits make
//! the spec the app makes; a re-roll keeps its place; the track ends where
//! the app's length slider ends it, and a change waiting for it starts
//! there; the tone and the description report what plays; and none of it
//! allocates or moves a sample.

// Seconds and frames of renders a few seconds long.
#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use dlcore::block::Event as CoreEvent;
use dlcore::generator::Layer;
use dlcore::generator::request::{BAD_SEED, BAD_TONE, NO_SUCH_EDIT, TEMPO_OUT};
use dlcore::player::about::GRID_MAX;
use dlcore::player::host::Host;
use dlcore::player::{Event, Handover, Kind, Moment, Player, Request, State, Track};
use dlcore::{Core, DRUMS_CHANNEL, EXTRA, Key, Tap};

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

fn roomy(f: impl FnOnce() + Send + 'static) {
    let stack = (std::mem::size_of::<Core>() + std::mem::size_of::<Player>()) * 4 + (16 << 20);
    std::thread::Builder::new().stack_size(stack).spawn(f).expect("a thread").join().expect("the test");
}

/// The core, with the drums' notes and the crossfades written down.
struct Watch<'a> {
    core: &'a mut Core,
    drums: &'a mut Vec<f64>,
    fades: &'a mut Vec<(f64, f64)>,
}

impl Host for Watch<'_> {
    fn frame(&self) -> u64 {
        self.core.next_frame()
    }
    fn key(&mut self, key: Key) {
        self.core.key(key);
    }
    fn note(&mut self, voice: u32, channel: u32, time: f64, midi: f64, dur: f64, vel: f64, parts: u32, extra: [f64; EXTRA]) {
        if channel == DRUMS_CHANNEL {
            self.drums.push(time);
        }
        self.core.note(voice, channel, time, midi, dur, vel, parts, extra);
    }
    fn event(&mut self, channel: u32, event: CoreEvent) {
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

#[derive(Default)]
struct Seen {
    /// Allocations inside the renders (the player's, and the core's).
    allocated: usize,
    events: Vec<Event>,
    drums: Vec<f64>,
    fades: Vec<(f64, f64)>,
    samples: Vec<f32>,
}

fn start(seed: u32, request: &Request) -> (Box<Core>, Box<Player>) {
    let mut core = Box::new(Core::new());
    let mut player = Box::new(Player::new());
    player.start(&mut core, RATE, 1, true).expect("a player");
    player.load(seed, request);
    player.ready(&mut *core);
    (core, player)
}

/// Render `seconds` a block at a time, `ask` before each block, the events
/// taken after each (or not, with `take` false).
fn run(core: &mut Core, player: &mut Player, seconds: f64, take: bool, mut ask: impl FnMut(&mut Core, &mut Player, usize)) -> Seen {
    let mut seen = Seen::default();
    let frames = (seconds * RATE) as usize;
    seen.drums.reserve(1 << 16);
    seen.fades.reserve(64);
    let mut out = vec![0.0f32; dlcore::QUANTUM];
    for b in 0..frames / dlcore::QUANTUM {
        let before = allocations();
        ask(core, player, b);
        core.render_with(&mut [&mut out[..]], |core| {
            let mut w = Watch { core, drums: &mut seen.drums, fades: &mut seen.fades };
            player.decide(&mut w);
        })
        .expect("a render");
        seen.allocated += allocations() - before;
        seen.samples.extend_from_slice(&out);
        while take && let Some(e) = player.event() {
            seen.events.push(e);
        }
    }
    seen
}

fn block_at(seconds: f64) -> usize {
    (seconds * RATE / dlcore::QUANTUM as f64) as usize
}

#[test]
fn the_events_say_where_the_loop_is() {
    roomy(|| {
        let (mut core, mut player) = start(SEED, &Request::new());
        player.set_wander(1.0);
        let seen = run(&mut core, &mut player, 30.0, true, |_, _, _| {});
        assert_eq!(seen.allocated, 0, "nothing allocates");
        let spec = player.generator().spec();
        let total = (spec.bars * spec.steps_per_bar) as u64;
        let sd = 60.0 / spec.bpm / 4.0;
        let e = &seen.events;
        assert_eq!(e[0].kind, Kind::Loop);
        assert_eq!(e[0].number, u64::from(SEED));
        assert!(e.windows(2).all(|w| w[0].time <= w[1].time), "in time order");
        let steps: Vec<&Event> = e.iter().filter(|x| x.kind == Kind::Step).collect();
        assert!(steps.len() as f64 > 25.0 / sd, "a step event a step");
        for (i, s) in steps.iter().enumerate() {
            assert_eq!(s.step, i as u64, "every step, once, in order");
            assert_eq!(s.number, s.step % total, "the step in its pass");
            assert!((s.time - (steps[0].time + i as f64 * sd)).abs() < 1e-6, "on the grid");
        }
        let passes: Vec<&Event> = e.iter().filter(|x| x.kind == Kind::Pass).collect();
        assert!(passes.len() >= 2);
        for (n, p) in passes.iter().enumerate() {
            assert_eq!((p.number, p.step), (n as u64, n as u64 * total), "a pass at each pass's top");
        }
        assert!(e.iter().filter(|x| x.kind == Kind::Phrase).all(|p| p.number % (4.0 * spec.steps_per_bar).min(total as f64) as u64 == 0));
        assert!(e.iter().any(|x| x.kind == Kind::Phrase));
        assert!(!e.iter().any(|x| matches!(x.kind, Kind::Section | Kind::End | Kind::Revised)));
        assert_eq!(player.events_lost(), 0);
        // Taken or not, the samples are the same.
        let (mut core2, mut player2) = start(SEED, &Request::new());
        player2.set_wander(1.0);
        let other = run(&mut core2, &mut player2, 30.0, false, |_, _, _| {});
        assert_eq!(seen.samples, other.samples);
        assert!(player2.events_lost() > 0, "a host that never takes them loses the newest");
    });
}

#[test]
fn a_piece_tells_its_sections_and_its_rounds() {
    roomy(|| {
        let request = Request::new().with_length(dlcore::player::Length::Bars(100.0)).expect("a length");
        let (mut core, mut player) = start(SEED, &request);
        let seen = run(&mut core, &mut player, 40.0, true, |_, _, _| {});
        let p = player.piece().expect("a piece");
        let sections: Vec<u64> = seen.events.iter().filter(|x| x.kind == Kind::Section).map(|x| x.number).collect();
        assert!(sections.len() >= 2 && sections.iter().enumerate().all(|(i, &n)| n == i as u64));
        let rounds: Vec<u64> = seen.events.iter().filter(|x| x.kind == Kind::Pass).map(|x| x.number).collect();
        assert_eq!(rounds[0], 0, "the first round");
        assert!(p.end.is_some());
    });
}

#[test]
fn edits_read_from_text_and_refused() {
    let r = Request::parse("glacial,edit.roll.drums=1234,edit.seed.bass=7,edit.warmth=0.25,edit.space=1,edit.wobble=0,edit.bpm=88").expect("a request");
    let e = r.edits();
    assert_eq!((e.seed(0), e.seed(1), e.seed(2)), (Some(1234), Some(7), None));
    assert_eq!(e.rolled(), 1);
    assert_eq!((e.tone(0), e.tone(1), e.tone(2), e.bpm()), (Some(0.25), Some(1.0), Some(0.0), Some(88.0)));
    assert!(r.vibe(dlcore::generator::data::vibe_index("glacial").expect("a vibe")).is_some());
    assert!(!r.asked().edits().any());
    assert_eq!(Request::parse("edit.warmth=1.5").err(), Some(BAD_TONE));
    assert_eq!(Request::parse("edit.seed.drums=-1").err(), Some(BAD_SEED));
    assert_eq!(Request::parse("edit.seed.drums=4294967296").err(), Some(BAD_SEED));
    assert_eq!(Request::parse("edit.bpm=200").err(), Some(TEMPO_OUT));
    assert_eq!(Request::parse("edit.tempo=80").err(), Some(NO_SUCH_EDIT));
    // The spec is the seed's, with the edits over it, nothing else moved.
    roomy(edits_over_the_seed);
}

fn edits_over_the_seed() {
    let mut g = Box::new(dlcore::generator::Generator::new());
    g.load_with(SEED, &Request::new());
    let own = *g.spec();
    g.load_with(SEED, &Request::parse("edit.roll.melody=99,edit.bpm=101").expect("edits"));
    let s = g.spec();
    assert_eq!((s.layer_seed(Layer::Melody), s.layer_seed(Layer::Drums), s.bpm), (99, own.layer_seed(Layer::Drums), 101.0));
    assert!(s.forced(Layer::Melody) && !s.forced(Layer::Drums));
    assert_eq!((s.root, s.scale, s.bars, s.tone), (own.root, own.scale, own.bars, own.tone));
}

#[test]
fn a_reroll_keeps_its_place() {
    roomy(|| {
        let (mut core, mut player) = start(SEED, &Request::new());
        player.set_wander(1.0);
        player.set_tempo(&mut *core, 90.0);
        let at = block_at(6.0);
        let roll = Request::new().with_layer_seed(Layer::Drums as usize, 424_242, true).expect("a seed");
        let seen = run(&mut core, &mut player, 14.0, true, |_, p, b| {
            if b == at {
                p.revise(SEED, &roll);
            }
        });
        assert_eq!(seen.allocated, 0, "nothing allocates");
        let e = &seen.events;
        let revised = e.iter().find(|x| x.kind == Kind::Revised).expect("the loop made again");
        assert_eq!(revised.number, u64::from(SEED));
        assert_eq!(e.iter().filter(|x| x.kind == Kind::Loop).count(), 1, "not a new loop");
        // The steps go on, one after another, on the same grid, through it.
        let steps: Vec<&Event> = e.iter().filter(|x| x.kind == Kind::Step).collect();
        let sd = 60.0 / 90.0 / 4.0;
        for (i, s) in steps.iter().enumerate() {
            assert_eq!(s.step, i as u64);
            assert!((s.time - (steps[0].time + i as f64 * sd)).abs() < 1e-6, "on the grid, at the tempo playing");
        }
        assert!(revised.step > 0 && steps.iter().any(|s| s.step == revised.step && (s.time - revised.time).abs() < 1e-12));
        let spec = player.generator().spec();
        assert!(spec.forced(Layer::Drums) && spec.layer_seed(Layer::Drums) == 424_242);
        assert!(seen.drums.iter().any(|&t| t >= revised.time), "the re-rolled drums play");
        assert!(seen.fades.is_empty(), "no crossfade: the loop goes on");
        assert_eq!(player.state(), State::Playing);
    });
}

#[test]
fn the_track_ends_where_the_slider_says() {
    roomy(|| {
        // Two passes, then the next loop: it starts at the end, ringing out.
        let (mut core, mut player) = start(SEED, &Request::new());
        let spec = *player.generator().spec();
        let total = (spec.bars * spec.steps_per_bar) as u64;
        let first = player.position().2;
        let sd = 60.0 / spec.bpm / 4.0;
        player.set_track(&*core, Track::Passes(2));
        assert_eq!(player.track_end(), Some(2 * total));
        player.change(NEXT, &Request::new(), Moment::End, Handover::Crossfade(1.0));
        let seconds = (2 * total) as f64 * sd + 4.0;
        let seen = run(&mut core, &mut player, seconds, true, |_, _, _| {});
        let end = seen.events.iter().find(|x| x.kind == Kind::End).expect("the end told");
        assert_eq!(end.step, 2 * total);
        assert!((end.time - (first + (2 * total) as f64 * sd)).abs() < 1e-6);
        let next = seen.events.iter().find(|x| x.kind == Kind::Loop && x.number == u64::from(NEXT)).expect("the next loop");
        assert!((next.time - end.time).abs() < 1e-9, "on the end");
        assert!(seen.fades.is_empty(), "a pass's end rings out");
        assert_eq!(player.generator().spec().seed, NEXT);

        // A bar of an eight-bar loop or longer ends on a phrase line inside
        // the pass, fading.
        let request = Request::new().with_length(dlcore::player::Length::Bars(8.0)).expect("eight bars");
        let (mut core, mut player) = start(SEED, &request);
        player.set_track(&*core, Track::Bars(1));
        let end = player.track_end().expect("an end");
        player.change(NEXT, &Request::new(), Moment::End, Handover::Crossfade(1.0));
        let seen = run(&mut core, &mut player, 20.0, true, |_, _, _| {});
        let told = seen.events.iter().find(|x| x.kind == Kind::End).expect("the end told");
        assert!(told.step >= end && told.step % (8 * 16) != 0);
        assert_eq!(seen.fades, vec![(told.time, told.time + 1.0)], "inside a pass, the crossfade asked");

        // With no change waiting it is told, and the loop plays on.
        let (mut core, mut player) = start(SEED, &Request::new());
        player.set_track(&*core, Track::Passes(1));
        let seen = run(&mut core, &mut player, (total as f64) * sd + 3.0, true, |_, _, _| {});
        assert_eq!(seen.events.iter().filter(|x| x.kind == Kind::End).count(), 1);
        assert_eq!(player.generator().spec().seed, SEED);
        assert_eq!(player.track(), Track::Passes(1), "it holds across loads until set again");
    });
}

#[test]
fn the_grid_and_the_description_report_what_plays() {
    roomy(|| {
        let (mut core, mut player) = start(SEED, &Request::new());
        let mut out = [0.0; GRID_MAX];
        let mut grids = Vec::new();
        let seen = run(&mut core, &mut player, 1.0, true, |_, p, _| {
            let n = p.grid(Layer::Chords, &mut out);
            grids.push(out[..n].to_vec());
        });
        assert_ne!(seen.events.len(), 0);
        // The notes, as the MIDI export reads them.
        let r = player.generator().render();
        assert_eq!(player.notes(Layer::Bass), r.bass.len());
        if let (Some(n), Some(e)) = (player.note(Layer::Bass, 0), r.bass.first()) {
            assert_eq!((n.step, n.dur, n.vel, n.notes[0], n.count), (e.step, e.dur, e.vel, e.midi, 1));
        }
        if let (Some(n), Some(e)) = (player.note(Layer::Chords, 0), r.chords.first()) {
            assert_eq!(n.count, e.notes.len());
            assert!(n.notes[..n.count].iter().zip(e.notes.iter()).all(|(&a, &b)| a.to_bits() == f64::from(b).to_bits()));
        }
        if let (Some(n), Some(e)) = (player.note(Layer::Drums, 0), r.drums.first()) {
            assert_eq!((n.drum, n.count), (e.inst.get(), 0));
        }
        assert!(player.note(Layer::Melody, r.melody.len()).is_none());
        let spec = player.generator().spec();
        let total = (spec.bars * spec.steps_per_bar) as usize;
        let chords = player.grid(Layer::Chords, &mut out);
        assert_eq!(chords, spec.cycles.layers[Layer::Chords as usize].map_or(total, |c| c as usize));
        assert!(out[..chords].iter().any(|&v| v > 0.0) && out[..chords].iter().all(|&v| (0.0..=1.5).contains(&v)));
        assert_eq!(player.grid(Layer::Chords, &mut [0.0; 4]), 0, "too short");
        let mut text = String::new();
        player.describe(&mut text).expect("a description");
        let field = |k: &str| text.lines().find_map(|l| l.strip_prefix(&format!("{k}="))).map(str::to_owned);
        assert_eq!(field("seed"), Some(SEED.to_string()));
        assert_eq!(field("name").as_deref(), Some(spec.name.as_str()));
        assert_eq!(field("bars").and_then(|x| x.parse::<f64>().ok()), Some(spec.bars));
        assert_eq!(field("swing").and_then(|x| x.parse::<f64>().ok()), Some(spec.swing), "numbers read back exactly");
        assert!(text.is_ascii());
        let seeds: Vec<u32> = field("layer_seeds").expect("seeds").split('|').map(|x| x.parse().expect("a seed")).collect();
        assert_eq!(seeds, spec.layer_seeds.to_vec());
        // The tone set is the tone told.
        player.set_tone(&mut *core, [0.1, 0.2, 0.3]);
        let mut text = String::new();
        player.describe(&mut text).expect("a description");
        assert!(text.lines().any(|l| l == "tone=0.1|0.2|0.3"));
        // Nothing made, nothing told.
        let mut core = Box::new(Core::new());
        let mut idle = Box::new(Player::new());
        idle.start(&mut core, RATE, 1, true).expect("a player");
        let mut text = String::new();
        idle.describe(&mut text).expect("nothing");
        assert!(text.is_empty() && idle.grid(Layer::Drums, &mut out) == 0);
    });
}
