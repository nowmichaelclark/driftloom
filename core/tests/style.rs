//! The request's style (queue item 90; `Request::with_styles`): a profile or
//! a set of them, read from text, refused with a plain line, and honored in
//! what the generator and the player make. A pinned style is that profile
//! whole; a set draws only from the set, every member reached; a mix already
//! inside the set stands; the other requests (a tempo, a key and a mode, a
//! theme) ride with it. test/generator.test.mjs measures the same in the
//! JavaScript's render, and holds a pinned profile's figures to its whole
//! loops on chance; the generation goldens' `style` cases hold the two
//! generators together.

// Weights and degrees are compared exactly on purpose.
#![allow(clippy::float_cmp)]

use dlcore::generator::Generator;
use dlcore::generator::data::CHARACTERS;
use dlcore::generator::request::{self, PROFILE_ORDER, Request};
use dlcore::generator::rng::Rng;
use dlcore::generator::spec::new_spec;
use dlcore::player::{Player, State};

fn roomy(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new().stack_size(64 << 20).spawn(f).expect("a thread").join().expect("the test");
}

/// The place in `CHARACTERS` of the profile at `bit` in the frozen order.
fn character(bit: usize) -> usize {
    CHARACTERS.iter().position(|c| c.id == PROFILE_ORDER[bit]).expect("a profile")
}

#[test]
fn the_text_and_the_refusals() {
    let styles = |text: &str| Request::parse(text).map(|r| r.styles());
    assert_eq!(styles("style=dust"), Ok(1));
    assert_eq!(styles("style=wayfare"), Ok(1 << 15));
    assert_eq!(styles("style=shrine|tide"), Ok((1 << 12) | (1 << 13)));
    assert_eq!(styles("cozy,bpm=80"), Ok(0));
    for (bit, id) in PROFILE_ORDER.iter().enumerate() {
        assert_eq!(styles(&format!("style={id}")), Ok(1 << bit), "{id}");
    }
    for text in ["style=nope", "style=", "style", "style=tide|", "style=Tide", "style=tape"] {
        assert_eq!(Request::parse(text).err(), Some(request::NO_SUCH_STYLE), "{text}");
    }
    assert_eq!(Request::new().with_styles(1 << 16).err(), Some(request::NO_SUCH_STYLE));
    assert_eq!(Request::new().with_styles(request::STYLES_ALL).map(|r| r.styles()), Ok(request::STYLES_ALL));
    assert_eq!(Request::new().with_styles(0).map(|r| r.styles()), Ok(0), "none asked");
    assert!(request::NO_SUCH_STYLE.is_ascii() && request::NO_SUCH_STYLE.len() < 40);
    assert_eq!(PROFILE_ORDER.len(), CHARACTERS.len());
    for id in PROFILE_ORDER {
        assert!(CHARACTERS.iter().any(|c| c.id == id), "{id} is a profile");
    }
}

#[test]
fn nothing_asked_is_the_seed_s_loop() {
    for seed in [1, 77, 4_272_732_017] {
        let a = new_spec(seed, &Request::new());
        let b = new_spec(seed, &Request::new().with_styles(0).expect("none"));
        assert!(a.styles == 0 && b.styles == 0 && a.mix[..] == b.mix[..]);
    }
}

#[test]
fn a_pinned_style_is_that_profile_whole() {
    roomy(pinned);
}

fn pinned() {
    let mut g = Box::new(Generator::new());
    let mut r = Rng::new(9001);
    let mut loops = 0;
    for (bit, id) in PROFILE_ORDER.iter().enumerate() {
        for _ in 0..40 {
            let seed = r.seed32();
            g.load_with(seed, &Request::new().with_styles(1 << bit).expect("a style"));
            let spec = g.spec();
            assert_eq!(spec.mix.len(), 1, "seed {seed}: {id}");
            assert_eq!((spec.mix[0].0, spec.mix[0].1, spec.styles), (character(bit), 1.0, 1 << bit), "seed {seed}");
            assert_eq!(spec.character().lead, character(bit));
            assert_eq!(g.lost(), 0, "seed {seed}: the lists ran out of room");
            let rd = g.render();
            assert!(!rd.chords.is_empty() || !rd.melody.is_empty() || !rd.bass.is_empty(), "seed {seed}: nothing");
            loops += 1;
        }
    }
    println!("pinned: {loops} loops, each its profile whole");
}

#[test]
fn a_set_draws_only_from_the_set() {
    roomy(sets);
}

fn sets() {
    let sets: [u32; 3] = [(1 << 12) | (1 << 13), 1 | 2 | (1 << 14), (1 << 15) | (1 << 8) | (1 << 11) | (1 << 5) | (1 << 9)];
    let mut r = Rng::new(9002);
    let mut reached = [0u32; 3];
    let mut led = [0u32; 3];
    let (mut inside_before, mut stands, mut blends) = (0, 0, [0; 3]);
    for _ in 0..500 {
        let seed = r.seed32();
        let plain = new_spec(seed, &Request::new());
        for (k, &set) in sets.iter().enumerate() {
            let spec = new_spec(seed, &Request::new().with_styles(set).expect("a set"));
            let member = |c: usize| (0..PROFILE_ORDER.len()).find(|&b| character(b) == c).filter(|&b| set & (1 << b) != 0);
            for &(c, _) in spec.mix.iter() {
                let b = member(c).unwrap_or_else(|| panic!("seed {seed}: {} is outside the set", CHARACTERS[c].id));
                reached[k] |= 1 << b;
            }
            led[k] |= 1 << member(spec.character().lead).expect("a member");
            blends[k] += usize::from(spec.mix.len() > 1);
            if plain.mix.iter().all(|e| member(e.0).is_some()) {
                inside_before += 1;
                stands += usize::from(plain.mix[..] == spec.mix[..]);
            }
        }
    }
    assert_eq!((reached, led), (sets, sets), "every member reached and leading");
    assert!(inside_before > 0 && stands == inside_before, "a mix already inside the set stands: {stands} of {inside_before}");
    assert!(blends[1] > 10 && blends[2] > 10, "a set of several still blends: {blends:?}");
}

#[test]
fn the_other_requests_ride_with_it() {
    roomy(riding);
}

fn riding() {
    let mut r = Rng::new(9005);
    for i in 0..80usize {
        let (seed, theme) = (r.seed32(), r.seed32());
        let bit = i % PROFILE_ORDER.len();
        let bpm = 32.0 + f64::from(u32::try_from(r.pick_index(159)).expect("small"));
        let request = Request::parse(&format!("style={},bpm={bpm},key=D,scale=dorian", PROFILE_ORDER[bit])).expect("a request");
        let a = new_spec(seed, &request);
        assert_eq!((a.bpm, a.root, a.scale, a.character().lead), (bpm, 2.0, 0, character(bit)), "seed {seed}");
        // A style overrules a theme's mix; its key, mode and tune hold.
        let own = new_spec(theme, &Request::new());
        let bent = new_spec(seed, &Request::new().with_theme(theme).with_styles(1 << bit).expect("a style"));
        assert_eq!((bent.mix.len(), bent.mix[0].0), (1, character(bit)), "theme {theme} on {seed}");
        assert_eq!((bent.root, bent.scale), (own.root, own.scale), "theme {theme} on {seed}: the key and mode hold");
    }
}

#[test]
fn the_player_plays_the_style() {
    roomy(played);
}

/// What the player decides, not only the generator: the loop it plays for a
/// style asked is that profile's.
fn played() {
    let mut player = Box::new(Player::new());
    let mut host = dlcore::Core::new();
    let mut r = Rng::new(9006);
    for bit in [3usize, 12, 13, 14, 15] {
        let seed = r.seed32();
        player.attach(&mut host, 48_000.0, true);
        player.load(seed, &Request::new().with_styles(1 << bit).expect("a style"));
        player.ready(&mut host);
        assert_eq!(player.state(), State::Playing);
        assert_eq!(player.generator().spec().character().lead, character(bit), "seed {seed}");
    }
}
