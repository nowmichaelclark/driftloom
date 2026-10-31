//! The request (queue item 85; promise 11): a test that walks its choices,
//! and the tempo it asks for measured in what the player plays (the notes'
//! onsets), not only in the spec. It grows with items 86 to 91.

// The tempos are whole bpm, compared exactly on purpose.
#![allow(clippy::float_cmp)]

use dlcore::block::Event;
use dlcore::generator::data::{CHARACTERS, VIBES};
use dlcore::generator::request::{self, Request};
use dlcore::generator::rng::Rng;
use dlcore::generator::spec::new_spec;
use dlcore::player::host::Host;
use dlcore::player::{Player, State};
use dlcore::{EXTRA, Key, QUANTUM, Tap};

const RATE: f64 = 48_000.0;

/// A host that renders nothing and writes down when each note starts: (its
/// pass, the step it was decided on, its time).
struct Onsets {
    frame: u64,
    key: Key,
    notes: Vec<(u32, u32, f64)>,
}

impl Host for Onsets {
    fn frame(&self) -> u64 {
        self.frame
    }
    fn key(&mut self, key: Key) {
        self.key = key;
    }
    fn note(&mut self, _: u32, _: u32, time: f64, _: f64, _: f64, _: f64, _: u32, _: [f64; EXTRA]) {
        if self.key.ord == 0 {
            self.notes.push((self.key.pass, self.key.step, time));
        }
    }
    fn event(&mut self, _: u32, _: Event) {}
    fn damp(&mut self, _: u32, _: f64) {}
    fn set(&mut self, _: u32, _: usize, _: usize, _: f64, _: f64) {}
    fn route(&mut self, _: u32, _: usize, _: usize, _: Option<Tap>, _: f64) {}
    fn param(&mut self, _: u32, _: u32, _: f64, _: f64, _: f64) {}
    fn bass_lift(&mut self, _: f64) {}
    fn space_range(&mut self, _: usize, _: f64, _: f64) {}
    fn steal(&mut self, _: u32, _: Key, _: f64, _: f64) {}
}

/// The tempo the player plays the loop of `seed` at, as `request` asks,
/// measured from its notes' onsets: on the steps that fall on a beat (swing
/// moves only the steps between), each step's earliest note, and the median
/// of the slopes between every pair of beats (a step's length), so a grace
/// or a strum cannot pull it. Played until twelve beats have sounded, or a
/// minute.
fn measured_bpm(player: &mut Player, seed: u32, request: &Request) -> f64 {
    let mut host = Onsets { frame: 0, key: Key::default(), notes: Vec::new() };
    player.attach(&mut host, RATE, true);
    player.load(seed, request);
    player.ready(&mut host);
    assert_eq!(player.state(), State::Playing);
    let spec = player.generator().spec();
    let spb = spec.steps_per_bar;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // a loop's steps, a few hundred
    let pass = (spec.bars * spb) as u64;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let beat = if spb == 12.0 { 3 } else { 4 } as u64;
    let mut first: Vec<(u64, f64)> = Vec::new();
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // a minute of frames
    let most = (60.0 * RATE) as u64;
    while host.frame < most && first.len() < 12 {
        player.decide(&mut host);
        host.frame += QUANTUM as u64;
        for (p, step, time) in host.notes.drain(..) {
            let at = u64::from(p) * pass + u64::from(step);
            if !at.is_multiple_of(beat) {
                continue;
            }
            match first.iter_mut().find(|e| e.0 == at) {
                Some(e) => e.1 = e.1.min(time),
                None => first.push((at, time)),
            }
        }
    }
    let mut slopes = Vec::new();
    for (i, a) in first.iter().enumerate() {
        for b in &first[i + 1..] {
            if a.0 != b.0 {
                #[allow(clippy::cast_precision_loss)] // steps, far below 2^53
                slopes.push((b.1 - a.1) / (b.0 as f64 - a.0 as f64));
            }
        }
    }
    assert!(slopes.len() >= 3, "seed {seed}: too few beats to measure ({} onsets)", first.len());
    slopes.sort_by(f64::total_cmp);
    let step = slopes[slopes.len() / 2];
    60.0 / (step * 4.0)
}

fn lead(seed: u32, request: &Request) -> usize {
    new_spec(seed, request).character().lead
}

#[test]
fn a_pinned_tempo_is_played() {
    std::thread::Builder::new().stack_size(64 << 20).spawn(pinned).expect("a thread").join().expect("pinned");
}

/// At least 200 loops, pins across the core's whole range, until every
/// profile has led three of them: each measured within 0.5 bpm of its pin.
fn pinned() {
    let mut player = Box::new(Player::new());
    let mut r = Rng::new(85);
    let mut led = [0usize; CHARACTERS.len()];
    let mut loops = 0;
    let mut worst: f64 = 0.0;
    while loops < 200 || led.iter().any(|&n| n < 3) {
        assert!(loops < 4000, "no pin led every profile: {led:?}");
        let seed = r.seed32();
        let pin = 32.0 + js_floor(r.f() * 159.0);
        let request = Request::new().with_tempo(pin, pin).expect("in range");
        let p = lead(seed, &request);
        // Past 200 loops, only those a profile still short of three leads.
        if loops >= 200 && led[p] >= 3 {
            continue;
        }
        let spec = new_spec(seed, &request);
        assert_eq!(spec.bpm, pin, "seed {seed}: the spec's tempo");
        let got = measured_bpm(&mut player, seed, &request);
        assert!((got - pin).abs() <= 0.5, "seed {seed} at {pin} bpm ({}): measured {got}", CHARACTERS[p].id);
        worst = worst.max((got - pin).abs());
        led[p] += 1;
        loops += 1;
    }
    println!("pinned: {loops} loops, every profile led at least 3, worst {worst:.4} bpm off its pin");
}

fn js_floor(x: f64) -> f64 {
    x.floor()
}

#[test]
fn a_ranged_tempo_stays_inside_and_covers_it() {
    std::thread::Builder::new().stack_size(64 << 20).spawn(ranged).expect("a thread").join().expect("ranged");
}

fn ranged() {
    let request = Request::new().with_tempo(70.0, 85.0).expect("in range");
    let mut r = Rng::new(48);
    let mut counts = [0usize; 16];
    let mut player = Box::new(Player::new());
    for i in 0..500 {
        let seed = r.seed32();
        let bpm = new_spec(seed, &request).bpm;
        assert!((70.0..=85.0).contains(&bpm) && bpm == bpm.trunc(), "seed {seed}: {bpm}");
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // 0 to 15
        let at = (bpm - 70.0) as usize;
        counts[at] += 1;
        // Every tenth measured in the render too.
        if i % 10 == 0 {
            let got = measured_bpm(&mut player, seed, &request);
            assert!((got - bpm).abs() <= 0.5, "seed {seed}: the spec says {bpm}, the player plays {got}");
        }
    }
    // 500 draws over 16 tempos, about 31 each: none under half that or over
    // twice, and as many in the low half as the high within a fifth.
    let low: usize = counts[..8].iter().sum();
    let high: usize = counts[8..].iter().sum();
    println!("ranged 70-85 over 500 seeds: {counts:?} (low half {low}, high half {high})");
    assert!(counts.iter().all(|&n| (15..=62).contains(&n)), "clumped: {counts:?}");
    assert!(low.abs_diff(high) * 5 <= 500 / 2, "lopsided: {low} against {high}");
}

#[test]
fn nothing_asked_is_the_seed_s_loop() {
    // The goldens hold every part; here the plainest form of it.
    for seed in [1, 77, 4_272_732_017] {
        let a = new_spec(seed, &Request::new());
        let b = new_spec(seed, &Request::parse("-").expect("nothing"));
        assert!(a.bpm == b.bpm && a.mix.len() == b.mix.len() && a.tempo.is_none() && b.vibes.is_none());
    }
}

#[test]
fn the_request_walks_its_choices() {
    // Each choice it has, asked and read back: the vibes without weights,
    // with weights, and the tempo pinned and ranged.
    let glacial = dlcore::generator::data::vibe_index("glacial").expect("glacial");
    let misty = dlcore::generator::data::vibe_index("misty").expect("misty");
    let r = Request::parse("glacial,misty=0.3,bpm=85-70").expect("a request");
    assert_eq!(r.vibe(glacial), Some(None));
    assert_eq!(r.vibe(misty), Some(Some(0.3)));
    assert_eq!(r.tempo(), Some([70.0, 85.0]));
    assert!(r.weighted());
    let pinned = Request::new().with_tempo(76.6, 76.6).expect("in range");
    assert_eq!(pinned.tempo(), Some([77.0, 77.0]));

    // The refusals, each its plain line.
    assert_eq!(Request::parse("nope").err(), Some(request::NO_SUCH_VIBE));
    assert_eq!(Request::parse("glacial=x").err(), Some(request::BAD_WEIGHT));
    assert_eq!(Request::parse("glacial=-1").err(), Some(request::BAD_WEIGHT));
    assert_eq!(Request::parse("bpm=7a").err(), Some(request::BAD_TEMPO));
    assert_eq!(Request::parse("bpm=").err(), Some(request::BAD_TEMPO));
    assert_eq!(Request::parse("bpm=300").err(), Some(request::TEMPO_OUT));
    assert_eq!(Request::parse("bpm=20-40").err(), Some(request::TEMPO_OUT));
    assert_eq!(Request::new().with_tempo(f64::NAN, 80.0).err(), Some(request::BAD_TEMPO));
    assert_eq!(Request::new().with_weight(glacial, f64::INFINITY).err(), Some(request::BAD_WEIGHT));
    assert_eq!(Request::new().with_weight(VIBES.len(), 0.5).err(), Some(request::NO_SUCH_VIBE));
    for line in [request::NO_SUCH_VIBE, request::BAD_WEIGHT, request::BAD_TEMPO, request::TEMPO_OUT] {
        assert!(line.is_ascii() && line.len() < 40);
    }

    // Weights: the shares as given, scaled down past 1, a vibe without one
    // counting 1, 0 left out, the five heaviest of more.
    let w = |text: &str| new_spec(9, &Request::parse(text).expect("weights")).vibes.expect("vibes");
    let share = |v: &dlcore::generator::character::Vibes, id: &str| {
        v.list.iter().find(|e| VIBES[e.0].id == id).map(|e| e.1)
    };
    let v = w("glacial=0.7,misty=0.3");
    assert_eq!((share(&v, "glacial"), share(&v, "misty")), (Some(179.0 / 255.0), Some(77.0 / 255.0)));
    let v = w("glacial=0.2");
    assert_eq!(share(&v, "glacial"), Some(51.0 / 255.0), "a lighter touch");
    let v = w("glacial=2,misty=2");
    assert_eq!((share(&v, "glacial"), share(&v, "misty")), (Some(128.0 / 255.0), Some(128.0 / 255.0)));
    let v = w("glacial,misty=1");
    assert_eq!(share(&v, "glacial"), share(&v, "misty"));
    let v = w("glacial=0,misty=0.5");
    assert_eq!((share(&v, "glacial"), v.list.len()), (None, 1));
    assert!(new_spec(9, &Request::parse("glacial=0").expect("zero")).vibes.is_none());
    let names = ["glacial", "misty", "peaceful", "zany", "hushed", "restful", "lively"];
    let text: Vec<String> = names.iter().zip(1..).map(|(n, i)| format!("{n}={}", 0.1 * f64::from(i))).collect();
    let v = w(&text.join(","));
    assert_eq!(v.list.len(), 5);
    assert!(share(&v, "glacial").is_none() && share(&v, "misty").is_none(), "the two lightest left out");
}

#[test]
fn a_loop_s_own_weights_make_it_again() {
    // What `requestOf` sends the player for the app's loops: the weights a
    // selection drew, sent back, give the same vibes and strength.
    let mut r = Rng::new(11);
    for i in 0..200 {
        let seed = r.seed32();
        let mut asked = Request::new();
        for k in 0..=(i % 5) {
            asked = asked.with((i * 7 + k * 13) % VIBES.len());
        }
        let a = new_spec(seed, &asked).vibes.expect("vibes");
        let mut back = Request::new();
        for &(id, w) in a.list.iter() {
            back = back.with_weight(id, w).expect("a weight");
        }
        let b = new_spec(seed, &back).vibes.expect("vibes");
        assert_eq!(a.strength, b.strength, "seed {seed}");
        assert_eq!(&a.list[..], &b.list[..], "seed {seed}");
    }
}
