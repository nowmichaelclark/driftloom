//! padWow (queue item 101b; batch 6's #3): a slow wow on the chord layer
//! alone, a choice drawn by itself (`alone`), not among the changes, on a
//! stream of its own. What it holds to: it goes only to loops that play a
//! chord, for about its share of them, within its ranges and under the
//! batch's guards; the vibes lean it as the batch's table says; a pin plays
//! it or keeps it off; and in the player it is set on the chords channel
//! alone, moving no note. That no other choice moved is the generation
//! goldens' (the PR has the count). The sound is `lib.rs`'s
//! `pad_wow_bends_the_chords_slowly` and the kit's.

#![allow(clippy::cast_precision_loss)] // counts of loops, as shares

use dlcore::block::Event;
use dlcore::blocks::{own, pad_wow};
use dlcore::generator::Generator;
use dlcore::generator::data::PADWOW;
use dlcore::generator::request::Request;
use dlcore::generator::rng::Rng;
use dlcore::player::host::Host;
use dlcore::player::{Player, State};
use dlcore::{CHORDS, EXTRA, Key, QUANTUM, Tap};

fn roomy(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new().stack_size(64 << 20).spawn(f).expect("a thread").join().expect("the test");
}

/// The seeds of the batch's corpus (corpus seed 20261009), its first `n`.
fn corpus(n: usize) -> Vec<u32> {
    let mut r = Rng::new(20_261_009);
    (0..n).map(|_| r.seed32()).collect()
}

fn plays_chords(g: &Generator) -> bool {
    g.render().chords.iter().any(|e| e.vel != 0.0 && !e.notes.is_empty())
}

/// Of `n` corpus loops as `request` asks them: how many may have the wow,
/// and how many play it.
fn share(g: &mut Generator, request: &Request, n: usize) -> (usize, usize) {
    let (mut of, mut on) = (0, 0);
    for seed in corpus(n) {
        g.load_with(seed, request);
        let plays = g.render().choice("padWow") == Some("on");
        assert_eq!(plays, g.settings().pad_wow.is_some(), "seed {seed}: the choice and its settings agree");
        if plays_chords(g) {
            of += 1;
            on += usize::from(plays);
        } else {
            assert!(!plays, "seed {seed}: no chord, no wow");
        }
    }
    (of, on)
}

#[test]
fn it_goes_to_its_share_of_the_loops_with_chords_within_its_ranges() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let (of, on) = share(&mut g, &Request::new(), 2000);
        let f = on as f64 / of as f64;
        println!("padWow: {on} of {of} loops with chords ({f:.3})");
        assert!((0.10..0.25).contains(&f), "{f:.3} of the loops with chords");
        let [d0, d1] = PADWOW.depth;
        let [r0, r1] = PADWOW.rate;
        for seed in corpus(2000) {
            g.load_with(seed, &Request::new());
            if let Some(w) = g.settings().pad_wow {
                assert!(w.depth >= d0 && w.depth <= d1 && w.rate >= r0 && w.rate <= r1, "seed {seed}: {w:?}");
                // The batch's guards hold for every draw the data allows.
                assert!(w.depth <= pad_wow::DEPTH_MAX && w.rate <= pad_wow::RATE_MAX);
            }
        }
        assert!(d1 <= pad_wow::DEPTH_MAX && r1 <= pad_wow::RATE_MAX, "the data's ranges within the guards");
    });
}

#[test]
fn the_vibes_lean_it_as_the_batch_s_table_says() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let of = |g: &mut Generator, vibe: &str| {
            let (of, on) = share(g, &Request::new().with_vibe(vibe).expect("a vibe"), 600);
            on as f64 / of.max(1) as f64
        };
        let (hazy, wistful, crisp, bright) = (of(&mut g, "hazy"), of(&mut g, "wistful"), of(&mut g, "crisp"), of(&mut g, "bright"));
        println!("padWow: hazy {hazy:.3}, wistful {wistful:.3}, crisp {crisp:.3}, bright {bright:.3}");
        assert!(hazy > 0.15 && wistful > 0.15, "toward the hazy vibes");
        assert!(crisp < 0.15 && bright < 0.15, "away from the clear ones");
        assert!(crisp > 0.0 || bright > 0.0, "a lean, never 0");
    });
}

#[test]
fn a_pin_plays_it_or_keeps_it_off() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let ask = |g: &mut Generator, seed: u32, text: &str| {
            g.load_with(seed, &Request::parse(text).expect("a request"));
            (g.render().choice("padWow"), g.settings().pad_wow.is_some())
        };
        let mut with = None;
        let mut without = None;
        for seed in corpus(400) {
            g.load_with(seed, &Request::new());
            if plays_chords(&g) && g.render().choice("padWow") == Some("today") && with.is_none() {
                with = Some(seed);
            }
            if !plays_chords(&g) && without.is_none() {
                without = Some(seed);
            }
        }
        let with = with.expect("a loop with chords and no wow");
        assert_eq!(ask(&mut g, with, "padWow=on"), (Some("on"), true), "seed {with}");
        assert_eq!(ask(&mut g, with, "padWow=today"), (Some("today"), false), "seed {with}");
        if let Some(seed) = without {
            assert_eq!(ask(&mut g, seed, "padWow=on"), (Some("today"), false), "seed {seed}: no chord to bend");
        }
    });
}

/// A host that renders nothing and writes down every note and every
/// setting of a channel's own block.
#[derive(Default)]
struct Calls {
    frame: u64,
    notes: Vec<(u32, u32, u64, u64)>,
    sets: Vec<(u32, usize, usize, f64, f64)>,
}

impl Host for Calls {
    fn frame(&self) -> u64 {
        self.frame
    }
    fn key(&mut self, _: Key) {}
    fn note(&mut self, voice: u32, channel: u32, time: f64, midi: f64, _: f64, _: f64, _: u32, _: [f64; EXTRA]) {
        self.notes.push((voice, channel, time.to_bits(), midi.to_bits()));
    }
    fn event(&mut self, _: u32, _: Event) {}
    fn damp(&mut self, _: u32, _: f64) {}
    fn set(&mut self, channel: u32, block: usize, setting: usize, value: f64, time: f64) {
        self.sets.push((channel, block, setting, value, time));
    }
    fn route(&mut self, _: u32, _: usize, _: usize, _: Option<Tap>, _: f64) {}
    fn param(&mut self, _: u32, _: u32, _: f64, _: f64, _: f64) {}
    fn bass_lift(&mut self, _: f64) {}
    fn space_range(&mut self, _: usize, _: f64, _: f64) {}
    fn steal(&mut self, _: u32, _: Key, _: f64, _: f64) {}
    fn fade_all(&mut self, _: f64, _: f64) {}
}

fn play(seed: u32, text: &str, seconds: u64) -> Calls {
    let mut player = Box::new(Player::new());
    let mut host = Calls::default();
    player.attach(&mut host, 48_000.0, true);
    player.load(seed, &Request::parse(text).expect("a request"));
    player.ready(&mut host);
    assert_eq!(player.state(), State::Playing);
    let until = seconds * 48_000;
    while host.frame < until {
        player.decide(&mut host);
        host.frame += QUANTUM as u64;
    }
    host
}

#[test]
fn the_player_sets_it_on_the_chords_alone_and_moves_no_note() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let seed = corpus(400)
            .into_iter()
            .find(|&seed| {
                g.load_with(seed, &Request::new());
                plays_chords(&g) && g.render().drums.iter().any(|e| e.vel != 0.0) && g.render().bass.iter().any(|e| e.vel != 0.0)
            })
            .expect("a loop with chords, a kit and a bass");
        g.load_with(seed, &Request::parse("padWow=on").expect("a request"));
        let w = g.settings().pad_wow.expect("its wow");
        drop(g);
        let on = play(seed, "padWow=on", 20);
        let off = play(seed, "padWow=today", 20);
        assert_eq!(on.notes, off.notes, "seed {seed}: the same notes, the kit's and the bass's among them");
        let wows: Vec<_> = on.sets.iter().filter(|s| s.1 == own::PAD_WOW).collect();
        assert!(wows.iter().all(|s| s.0 == CHORDS), "on the chords channel alone");
        assert!(wows.iter().any(|s| s.2 == pad_wow::DEPTH && s.3.to_bits() == w.depth.to_bits()), "its depth");
        assert!(wows.iter().any(|s| s.2 == pad_wow::RATE && s.3.to_bits() == w.rate.to_bits()), "its rate");
        assert!(off.sets.iter().all(|s| s.1 != own::PAD_WOW), "a loop without it sets nothing");
    });
}
