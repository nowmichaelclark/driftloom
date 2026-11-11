//! tuningOffset (queue item 101b; batch 6's #12): the whole loop a little off
//! A440, drawn per loop at load on a stream of its own (`choices::tuning_of`).
//! What it holds to: most loops play at A440 and the rest within the data's
//! cents, either way; the vibes lean how often as the batch's table says; a
//! `tuning.cents` pin plays its offset, 0 holding the loop at A440; and in
//! the player every pitched note (and every pitch a note carries, and the
//! organ's chords) moves by exactly the offset while the kit, the tuned
//! kick and the noises keep theirs. That nothing else moves is the
//! generation goldens' (only each load's `tuning` moved; the PR has the
//! count).

// Counts of loops, a few thousand, are shares as floats; pitches are
// compared to a hair on purpose.
#![allow(clippy::float_cmp, clippy::cast_precision_loss)]

use dlcore::block::Event;
use dlcore::block::event::RETUNE;
use dlcore::blocks::texture;
use dlcore::generator::Generator;
use dlcore::generator::data::TUNING;
use dlcore::generator::request::{Request, SETTING_OUT};
use dlcore::generator::rng::Rng;
use dlcore::player::host::Host;
use dlcore::player::{Player, State};
use dlcore::{ACCORDION, DRUM, DUBSTAB, EXTRA, FIDDLE, FLUTE, HAT, Key, OCARINA, QUANTUM, TEXTURE, TUNEDKICK, Tap, WHISTLE};

fn roomy(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new().stack_size(64 << 20).spawn(f).expect("a thread").join().expect("the test");
}

/// The seeds of the batch's corpus (corpus seed 20261009), its first `n`.
fn corpus(n: usize) -> Vec<u32> {
    let mut r = Rng::new(20_261_009);
    (0..n).map(|_| r.seed32()).collect()
}

/// The tuning of each of `n` corpus loops as `request` asks them.
fn tunings(g: &mut Generator, request: &Request, n: usize) -> Vec<f64> {
    corpus(n)
        .into_iter()
        .map(|seed| {
            g.load_with(seed, request);
            g.settings().tuning
        })
        .collect()
}

fn off(t: &[f64]) -> f64 {
    t.iter().filter(|&&c| c != 0.0).count() as f64 / t.len() as f64
}

#[test]
fn most_loops_play_at_a440_and_the_rest_within_the_data_s_cents() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let t = tunings(&mut g, &Request::new(), 2000);
        let [lo, hi] = TUNING.cents;
        for &c in &t {
            assert!(c == 0.0 || (c.abs() >= lo && c.abs() <= hi), "{c} cents");
        }
        let share = off(&t);
        println!("off A440: {share:.3} of 2000 (data: {})", TUNING.share);
        assert!((share - TUNING.share).abs() < 0.03, "{share:.3} of loops off A440");
        assert!(t.iter().any(|&c| c > 0.0) && t.iter().any(|&c| c < 0.0), "both ways");
        // The guard: never past 40 cents.
        assert!(hi <= 40.0);
    });
}

#[test]
fn the_vibes_lean_it_as_the_batch_s_table_says() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let of = |g: &mut Generator, vibe: &str| off(&tunings(g, &Request::new().with_vibe(vibe).expect("a vibe"), 1000));
        let (hazy, wistful, crisp, frosty) = (of(&mut g, "hazy"), of(&mut g, "wistful"), of(&mut g, "crisp"), of(&mut g, "frosty"));
        println!("off A440: hazy {hazy:.3}, wistful {wistful:.3}, crisp {crisp:.3}, frosty {frosty:.3}");
        assert!(hazy > TUNING.share && wistful > TUNING.share, "toward the hazy vibes");
        assert!(crisp < TUNING.share && frosty < TUNING.share, "away from the clear ones");
        assert!(crisp > 0.0, "a lean, never 0");
    });
}

#[test]
fn a_pin_plays_its_offset_and_0_holds_a440() {
    roomy(|| {
        for (text, line) in [
            ("tuning.cents=0", None),
            ("tuning.cents=-35", None),
            ("tuning.cents=35", None),
            ("tuning.cents=12.5", None),
            ("tuning.cents=35.5", Some(SETTING_OUT)),
            ("tuning.cents=-41", Some(SETTING_OUT)),
        ] {
            assert_eq!(Request::parse(text).err(), line, "{text}");
        }
        let mut g = Box::new(Generator::new());
        let seed = corpus(400)
            .into_iter()
            .find(|&seed| {
                g.load_with(seed, &Request::new());
                g.settings().tuning != 0.0
            })
            .expect("a loop off A440");
        let ask = |g: &mut Generator, text: &str| {
            g.load_with(seed, &Request::parse(text).expect("a request"));
            (g.settings().tuning, g.spec().bpm, g.render().choices.iter().map(|w| w.get()).collect::<Vec<_>>())
        };
        let (drawn, bpm, choices) = ask(&mut g, "");
        assert_eq!(ask(&mut g, "tuning.cents=0"), (0.0, bpm, choices.clone()), "seed {seed} (drawn {drawn}): held at A440, nothing else moved");
        assert_eq!(ask(&mut g, "tuning.cents=-20"), (-20.0, bpm, choices), "seed {seed}");
    });
}

/// A host that renders nothing and writes down every note (voice, channel,
/// time, midi, parts, extra) and every retune's chord.
#[derive(Default)]
struct Heard {
    frame: u64,
    notes: Vec<(u32, u32, f64, f64, u32, [f64; EXTRA])>,
    retunes: Vec<Vec<f64>>,
}

impl Host for Heard {
    fn frame(&self) -> u64 {
        self.frame
    }
    fn key(&mut self, _: Key) {}
    fn note(&mut self, voice: u32, channel: u32, time: f64, midi: f64, _: f64, _: f64, parts: u32, x: [f64; EXTRA]) {
        self.notes.push((voice, channel, time, midi, parts, x));
    }
    fn event(&mut self, _: u32, e: Event) {
        if e.kind == RETUNE {
            self.retunes.push(e.notes.iter().copied().filter(|m| !m.is_nan()).collect());
        }
    }
    fn damp(&mut self, _: u32, _: f64) {}
    fn set(&mut self, _: u32, _: usize, _: usize, _: f64, _: f64) {}
    fn route(&mut self, _: u32, _: usize, _: usize, _: Option<Tap>, _: f64) {}
    fn param(&mut self, _: u32, _: u32, _: f64, _: f64, _: f64) {}
    fn bass_lift(&mut self, _: f64) {}
    fn space_range(&mut self, _: usize, _: f64, _: f64) {}
    fn steal(&mut self, _: u32, _: Key, _: f64, _: f64) {}
    fn fade_all(&mut self, _: f64, _: f64) {}
}

fn play(seed: u32, text: &str, seconds: u64) -> Heard {
    let mut player = Box::new(Player::new());
    let mut host = Heard::default();
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

/// The extra values that carry a pitch: the note a wind or a fiddle is
/// joined to, a stab's other two notes.
fn pitches_carried(voice: u32) -> &'static [usize] {
    match voice {
        FLUTE | OCARINA | FIDDLE | ACCORDION | WHISTLE => &[0],
        DUBSTAB => &[0, 1],
        _ => &[],
    }
}

/// Whether the note's pitch is its own (the kit, the noises).
fn keeps_its_pitch(voice: u32, parts: u32) -> bool {
    matches!(voice, DRUM | HAT | TUNEDKICK) || (voice == TEXTURE && parts != texture::SWELL)
}

#[test]
fn every_pitched_note_moves_by_the_offset_and_the_kit_keeps_its_pitch() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        // Loops that between them play the tuned kick, the organ, a texture
        // and the joined winds: every way a pitch reaches a voice.
        let mut seeds = Vec::new();
        for want in ["tunedKick", "organ", "dubStab"] {
            seeds.push(
                corpus(400)
                    .into_iter()
                    .find(|&seed| {
                        g.load_with(seed, &Request::new());
                        g.render().choice(want).is_some_and(|o| o != "today")
                    })
                    .expect("a loop of the corpus"),
            );
        }
        seeds.extend(corpus(6));
        drop(g);
        let (mut moved, mut kept) = (0, 0);
        for seed in seeds {
            let a440 = play(seed, "tuning.cents=0", 40);
            let off = play(seed, "tuning.cents=-30", 40);
            assert_eq!(a440.notes.len(), off.notes.len(), "seed {seed}: the same notes");
            for (a, b) in a440.notes.iter().zip(&off.notes) {
                assert_eq!((a.0, a.1, a.2, a.4), (b.0, b.1, b.2, b.4), "seed {seed}: the same note at the same time");
                if keeps_its_pitch(a.0, a.4) {
                    let bits = |n: &(u32, u32, f64, f64, u32, [f64; EXTRA])| (n.3.to_bits(), n.5.map(f64::to_bits));
                    assert_eq!(bits(a), bits(b), "seed {seed}: voice {} keeps its pitch", a.0);
                    kept += 1;
                } else {
                    assert!((b.3 - (a.3 - 0.3)).abs() < 1e-9, "seed {seed}: voice {} at {} against {}", a.0, b.3, a.3);
                    for &i in pitches_carried(a.0) {
                        let (x, y) = (a.5[i], b.5[i]);
                        assert!((x.is_nan() && y.is_nan()) || (y - (x - 0.3)).abs() < 1e-9, "seed {seed}: voice {}'s carried pitch {x} against {y}", a.0);
                    }
                    let others = (0..EXTRA).filter(|i| !pitches_carried(a.0).contains(i));
                    for i in others {
                        assert!(a.5[i].to_bits() == b.5[i].to_bits(), "seed {seed}: voice {}'s extra {i} is not a pitch and stays", a.0);
                    }
                    moved += 1;
                }
            }
            assert_eq!(a440.retunes.len(), off.retunes.len(), "seed {seed}");
            for (a, b) in a440.retunes.iter().zip(&off.retunes) {
                assert!(a.len() == b.len() && a.iter().zip(b).all(|(x, y)| (y - (x - 0.3)).abs() < 1e-9), "seed {seed}: the organ's chord {a:?} against {b:?}");
            }
        }
        println!("{moved} pitched notes moved, {kept} kept their pitch");
        assert!(moved > 0 && kept > 0);
    });
}
