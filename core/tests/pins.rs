//! The request's pins (queue item 85b; `generator::request::Pins`): their
//! numbers and refusals, as `js/pins.js` has them (test/generator.test.mjs
//! walks the same), and the space recipe's pins reaching the core through
//! the player, and let go again by a loop that doesn't pin them. What each
//! pin does to a loop is held to the JavaScript by the generation goldens'
//! `pins` cases.

// Pinned values are compared exactly on purpose: a pin is the number asked.
#![allow(clippy::float_cmp)]

use dlcore::block::Event;
use dlcore::generator::request::{BAD_SETTING, NO_SUCH_SETTING, NO_SUCH_VIBE, Pins, Request, SETTING_OUT};
use dlcore::player::host::Host;
use dlcore::player::{Player, State};
use dlcore::space;
use dlcore::{EXTRA, Key, QUANTUM, Tap};

#[test]
fn the_numbers_are_the_javascripts() {
    for (name, number) in [("glass", 0), ("tunedKick", 5), ("padSweep", 10), ("arc.shape", 16), ("kick.note", 20), ("echo.beats", 32), ("space.room-weight", 48), ("space.room", 50), ("space.wash-cutoff", 79)] {
        assert_eq!(Pins::number(name), Some(number), "{name}");
    }
    assert_eq!(Pins::number("space.nope"), None);
    assert_eq!(Pins::number("cozy"), None);
}

#[test]
fn the_refusals_are_each_a_plain_line() {
    let refused = |t: &str| Request::parse(t).err();
    for (text, line) in [
        ("organ", Some(BAD_SETTING)),
        ("organ=maybe", Some(BAD_SETTING)),
        ("arc.nope=1", Some(NO_SUCH_SETTING)),
        ("arc.period=10", Some(SETTING_OUT)),
        ("kick.note=33.5", Some(SETTING_OUT)),
        ("echo.beats=0.8", Some(SETTING_OUT)),
        ("space.room=x", Some(BAD_SETTING)),
        ("lateFloor.order=late", Some(BAD_SETTING)),
        ("lateFloor.gap=9", Some(SETTING_OUT)),
        ("space.ceiling-busy=-7", None),
        ("echo.beats=0.6666666666666666", None),
        ("arc.period=2.4e2", None),
        ("nope=on", Some(NO_SUCH_VIBE)),
    ] {
        assert_eq!(refused(text), line, "{text}");
    }
    assert_eq!(Request::new().with_pin(99, 1.0).err(), Some(NO_SUCH_SETTING));
    assert_eq!(Request::new().with_pin(5, 3.0).err(), Some(BAD_SETTING));
    assert_eq!(Request::new().with_pin(5, f64::NAN).err(), Some(BAD_SETTING));
}

#[test]
fn text_and_numbers_ask_the_same() {
    let text = Request::parse("tunedKick=long,arc.shape=dip,kick.note=33,space.room=0.8").expect("a request");
    let numbers = Request::new().with_pin(5, 2.0).and_then(|r| r.with_pin(16, 1.0)).and_then(|r| r.with_pin(20, 33.0)).and_then(|r| r.with_pin(50, 0.8)).expect("pins");
    for n in 0..dlcore::generator::request::PINS_MAX {
        assert_eq!(text.pins().at(n), numbers.pins().at(n), "pin {n}");
    }
    assert_eq!(text.pins().word("tunedKick"), Some("long"));
    assert_eq!(text.pins().word("arc.shape"), Some("dip"));
    assert_eq!(text.pins().space(space::n::ROOM), Some(0.8));
    assert!(!Request::parse("cozy,bpm=80").expect("a request").pins().any());
}

/// A host that renders nothing and writes down the space ranges it is sent.
struct Ranges {
    frame: u64,
    ranges: Vec<(usize, f64, f64)>,
}

impl Host for Ranges {
    fn frame(&self) -> u64 {
        self.frame
    }
    fn key(&mut self, _: Key) {}
    fn note(&mut self, _: u32, _: u32, _: f64, _: f64, _: f64, _: f64, _: u32, _: [f64; EXTRA]) {}
    fn event(&mut self, _: u32, _: Event) {}
    fn damp(&mut self, _: u32, _: f64) {}
    fn set(&mut self, _: u32, _: usize, _: usize, _: f64, _: f64) {}
    fn route(&mut self, _: u32, _: usize, _: usize, _: Option<Tap>, _: f64) {}
    fn param(&mut self, _: u32, _: u32, _: f64, _: f64, _: f64) {}
    fn bass_lift(&mut self, _: f64) {}
    fn space_range(&mut self, which: usize, lo: f64, hi: f64) {
        self.ranges.push((which, lo, hi));
    }
    fn steal(&mut self, _: u32, _: Key, _: f64, _: f64) {}
}

/// The space ranges the player sends playing the loop of `seed` as
/// `request` asks, for its first second.
fn ranges(player: &mut Player, host: &mut Ranges, seed: u32, request: &Request) -> Vec<(usize, f64, f64)> {
    player.load(seed, request);
    player.ready(host);
    assert_eq!(player.state(), State::Playing);
    let until = host.frame + 48_000;
    while host.frame < until {
        player.decide(host);
        host.frame += QUANTUM as u64;
    }
    std::mem::take(&mut host.ranges)
}

#[test]
fn the_space_pins_reach_the_core_and_go() {
    std::thread::Builder::new().stack_size(64 << 20).spawn(space_pins).expect("a thread").join().expect("space pins");
}

fn space_pins() {
    let mut player = Box::new(Player::new());
    let mut host = Ranges { frame: 0, ranges: Vec::new() };
    player.attach(&mut host, 48_000.0, true);
    let pinned = Request::parse("space.room=0.8,space.kit=0.1").expect("a request");
    assert_eq!(ranges(&mut player, &mut host, 7, &Request::new()), vec![], "nothing pinned, nothing sent");
    assert_eq!(ranges(&mut player, &mut host, 7, &pinned), vec![(space::n::ROOM, 0.8, 0.8), (space::n::KIT, 0.1, 0.1)]);
    let room = space::SETTINGS[space::n::ROOM];
    let kit = space::SETTINGS[space::n::KIT];
    let only_room = Request::parse("space.room=0.75").expect("a request");
    assert_eq!(ranges(&mut player, &mut host, 8, &only_room), vec![(space::n::KIT, kit.lo, kit.hi), (space::n::ROOM, 0.75, 0.75)], "the kit let go");
    assert_eq!(ranges(&mut player, &mut host, 9, &Request::new()), vec![(space::n::ROOM, room.lo, room.hi)], "the room let go");
    assert_eq!(ranges(&mut player, &mut host, 9, &Request::new()), vec![], "and nothing more");
}
