//! lowBell (queue item 101g; batch 6's #2): a struck low bell on the chord's
//! root, drawn per loop at load on a stream of its own
//! (`choices::low_bell_of`), struck by the player (`Sequencer::low_bell`),
//! sounded by its voice (`blocks::low_bell`). What it holds to: it goes only
//! to loops that play a chord, to about its share of them, half struck and
//! half swelling, each kind leaned by its own vibes; a pin plays a kind or
//! keeps it off, and nothing the generator decides but the bell moves; in the
//! player a note on each chord whose root changes (or the same root after
//! `regap` seconds), at the root's pitch class between D2 and C#3 (65 to 165
//! Hz), never in a full stop, damped there; and the voice: partials 50 to 90
//! cents off whole multiples, falling 8 to 25 dB a second, its level
//! following the chords channel, and nothing under 60 Hz.

// Counts of loops are shares as floats; steps and pitches copied, not
// computed, are compared exactly; the generator, its render and a strike are
// `g`, `r` and `t` as in the other tests.
#![allow(clippy::cast_precision_loss, clippy::float_cmp, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::many_single_char_names)]

use dlcore::block::Event;
use dlcore::block::event::CHOKE;
use dlcore::generator::Generator;
use dlcore::generator::choices::LOW_BELL_KINDS;
use dlcore::generator::data::LOWBELL;
use dlcore::generator::request::{BAD_SETTING, Request};
use dlcore::generator::rng::Rng;
use dlcore::player::host::Host;
use dlcore::player::{Player, State};
use dlcore::{CHORDS, Core, EXTRA, Key, LOWBELL as BELL, PAD, QUANTUM, Tap};

fn roomy(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new().stack_size(64 << 20).spawn(f).expect("a thread").join().expect("the test");
}

/// The seeds of the batch's corpus (corpus seed 20261009), its first `n`.
fn corpus(n: usize) -> Vec<u32> {
    let mut r = Rng::new(20_261_009);
    (0..n).map(|_| r.seed32()).collect()
}

fn ask(t: &str) -> Request {
    Request::parse(t).expect("a request")
}

/// Of `n` corpus loops as `request` asks them: how many play a chord, and
/// how many of those play each kind (struck, swelling).
fn kinds(g: &mut Generator, request: &Request, n: usize) -> (usize, [usize; 2]) {
    let (mut of, mut on) = (0, [0, 0]);
    for seed in corpus(n) {
        g.load_with(seed, request);
        let plays = g.render().chords.iter().any(|e| e.vel != 0.0 && !e.notes.is_empty());
        let bell = g.settings().low_bell;
        if plays {
            of += 1;
            if bell > 0 {
                on[usize::from(bell) - 1] += 1;
            }
        } else {
            assert_eq!(bell, 0, "seed {seed}: no chord, no bell");
        }
    }
    (of, on)
}

#[test]
fn it_goes_to_its_share_of_the_loops_that_play_a_chord_half_and_half() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let (of, [struck, swell]) = kinds(&mut g, &Request::new(), 2000);
        let f = (struck + swell) as f64 / of as f64;
        println!("lowBell on 2000 corpus loops: {struck} struck and {swell} swelling of {of} that play a chord ({f:.3})");
        assert!(of > 1500 && (f - LOWBELL.share).abs() < 0.03, "{f:.3} of the loops, not near {}", LOWBELL.share);
        assert!(struck > 0 && swell > 0 && (struck as f64 / swell as f64 - 1.0).abs() < 0.4, "{struck} struck, {swell} swelling");
    });
}

#[test]
fn each_kind_is_leaned_by_its_own_vibes_never_to_none() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let mut of = |vibe: &str| {
            let (n, [a, b]) = kinds(&mut g, &ask(vibe), 1000);
            (a as f64 / n as f64, b as f64 / n as f64)
        };
        let (mysterious, glacial, zesty) = (of("mysterious"), of("glacial"), of("zesty"));
        let half = LOWBELL.share / 2.0;
        println!("struck/swelling: mysterious {mysterious:.3?}, glacial {glacial:.3?}, zesty {zesty:.3?}");
        assert!(mysterious.0 > half * 1.15 && mysterious.0 > mysterious.1, "mysterious leans to the strike");
        assert!(glacial.1 > half * 1.15 && glacial.1 > glacial.0, "glacial leans to the swell");
        assert!(zesty.0 < half && zesty.1 < half && zesty.0 + zesty.1 > 0.0, "zesty away from both, never none");
    });
}

#[test]
fn a_pin_plays_its_kind_or_keeps_it_off_and_nothing_else_moves() {
    roomy(|| {
        assert_eq!(Request::parse("lowBell.kind=loud").err(), Some(BAD_SETTING));
        let mut g = Box::new(Generator::new());
        let mut a = Box::new(Generator::new());
        for seed in corpus(200) {
            for (i, &(kind, _)) in LOW_BELL_KINDS.iter().enumerate() {
                g.load_with(seed, &ask(&format!("lowBell.kind={kind}")));
                a.load_with(seed, &ask("lowBell.kind=today"));
                let (on, off) = (g.render(), a.render());
                let plays = on.chords.iter().any(|e| e.vel != 0.0 && !e.notes.is_empty());
                assert_eq!(usize::from(g.settings().low_bell), if plays { i } else { 0 }, "seed {seed}: {kind}");
                assert_eq!(a.settings().low_bell, 0, "seed {seed}: today's plays none");
                assert_eq!(on.choices.iter().map(|w| w.get()).collect::<Vec<_>>(), off.choices.iter().map(|w| w.get()).collect::<Vec<_>>(), "seed {seed}");
                assert!(on.chords.iter().zip(off.chords.iter()).all(|(x, y)| x.step == y.step && x.vel == y.vel && x.notes[..] == y.notes[..]), "seed {seed}: a chord moved");
                assert!(on.melody.iter().zip(off.melody.iter()).all(|(x, y)| x.step == y.step && x.midi == y.midi), "seed {seed}: the tune moved");
                assert!(on.bass.iter().zip(off.bass.iter()).all(|(x, y)| x.step == y.step && x.midi == y.midi), "seed {seed}: the bass moved");
                assert_eq!(g.settings().tuning, a.settings().tuning, "seed {seed}");
            }
        }
    });
}

/// A host that renders nothing and writes down every low bell (time, midi,
/// parts) and every choke to the chords channel.
#[derive(Default)]
struct Heard {
    frame: u64,
    bells: Vec<(f64, f64, u32)>,
    chokes: Vec<f64>,
}

impl Host for Heard {
    fn frame(&self) -> u64 {
        self.frame
    }
    fn key(&mut self, _: Key) {}
    fn note(&mut self, voice: u32, channel: u32, time: f64, midi: f64, _: f64, _: f64, parts: u32, _: [f64; EXTRA]) {
        if voice == BELL {
            assert_eq!(channel, CHORDS);
            self.bells.push((time, midi, parts));
        }
    }
    fn event(&mut self, channel: u32, e: Event) {
        if channel == CHORDS && e.kind == CHOKE {
            self.chokes.push(e.time);
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

fn play(seed: u32, text: &str, wander: f64, seconds: u64) -> Heard {
    let mut player = Box::new(Player::new());
    let mut host = Heard::default();
    player.attach(&mut host, 48_000.0, true);
    player.set_wander(wander);
    player.load(seed, &ask(text));
    player.ready(&mut host);
    assert_eq!(player.state(), State::Playing);
    while host.frame < seconds * 48_000 {
        player.decide(&mut host);
        host.frame += QUANTUM as u64;
    }
    host
}

fn pc(x: f64) -> i32 {
    (x.round() as i32).rem_euclid(12)
}

#[test]
fn a_note_on_each_chord_whose_root_changes_at_the_root_s_pitch_class() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let (mut loops, mut strikes, mut changes) = (0, 0, 0);
        for (n, seed) in corpus(120).into_iter().enumerate() {
            let text = if n % 2 == 0 { "lowBell.kind=struck,tuning.cents=0" } else { "lowBell.kind=swell,tuning.cents=0" };
            g.load_with(seed, &ask(text));
            if g.settings().low_bell == 0 || g.render().form.is_some() {
                continue;
            }
            let (spec, r) = (g.spec(), g.render());
            let sd = 15.0 / spec.bpm;
            let cycle = spec.cycles.of(2).filter(|&c| c > 0.0).unwrap_or(spec.bars * spec.steps_per_bar);
            // As composed (no wander): every slot whose chords play.
            let heard = play(seed, text, 0.0, 30);
            let Some(&(t0, _, _)) = heard.bells.first() else { panic!("seed {seed}: no bell") };
            let first = r.harmony.slots.iter().find(|s| r.chords.iter().any(|e| e.vel != 0.0 && e.step >= s.start_step && e.step < s.start_step + s.length_steps)).map_or(0.0, |s| s.start_step);
            let mut want = Vec::new();
            let (mut last, mut at) = (-1, f64::NEG_INFINITY);
            let mut k = first;
            while (k - first) * sd < 29.0 - 6.0 * sd {
                let s = k % cycle;
                if let Some(slot) = r.harmony.slots.iter().find(|x| x.start_step == s)
                    && r.chords.iter().any(|e| e.vel != 0.0 && !e.notes.is_empty() && e.step >= s && e.step < s + slot.length_steps)
                {
                    let t = (k - first) * sd + if (k as u64) % 2 == 1 { spec.swing * sd } else { 0.0 };
                    let p = pc(slot.root_midi);
                    if p != last || t - at >= LOWBELL.regap - 0.001 {
                        want.push((t, p));
                        last = p;
                        at = t;
                    }
                }
                k += 1.0;
            }
            // Every strike is one of these; a strike the budget had no room
            // for is left out (it yields, as glass does), and the next
            // follows as if it had been struck.
            let got: Vec<(f64, i32)> = heard.bells.iter().map(|&(t, m, _)| (t - t0, pc(m))).filter(|g| g.0 <= want.last().map_or(0.0, |w| w.0) + 0.01).collect();
            for g in &got {
                assert!(want.iter().any(|w| (g.0 - w.0).abs() < 0.02 && g.1 == w.1), "seed {seed}: struck {g:?}, at no chord's root\n{want:?}");
            }
            strikes += got.len();
            changes += want.len();
            for &(_, m, parts) in &heard.bells {
                let hz = 440.0 * 2f64.powf((m - 69.0) / 12.0);
                assert!((38.0..50.0).contains(&m) && (65.0..=165.0).contains(&hz), "seed {seed}: struck at {m} ({hz:.1} Hz)");
                assert_eq!(parts, u32::from(n % 2 == 1), "seed {seed}: its kind");
            }
            loops += 1;
        }
        println!("{loops} loops, {strikes} strikes of {changes} roots, each on its chord's root");
        assert!(loops > 30 && strikes > 200, "{loops} loops, {strikes} strikes");
        assert!(strikes as f64 > 0.95 * changes as f64, "the budget left out {} of {changes}", changes - strikes);
    });
}

#[test]
fn never_in_a_full_stop_and_damped_there() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let mut stops = 0;
        for seed in corpus(400) {
            g.load_with(seed, &ask("lowBell.kind=struck"));
            let r = g.render();
            if g.settings().low_bell == 0 || r.quiet.is_empty() {
                continue;
            }
            let spec = g.spec();
            let bar = spec.steps_per_bar * 15.0 / spec.bpm;
            let total = spec.bars * bar;
            let quiet: Vec<f64> = r.quiet.iter().map(|&b| b as f64 * bar).collect();
            let heard = play(seed, "lowBell.kind=struck", 1.0, 40);
            // The loop's first step, the player's `START` on.
            let start = 0.05;
            let into = |t: f64| ((t - start) % total + total) % total;
            for &(t, _, _) in &heard.bells {
                assert!(!quiet.iter().any(|&q| into(t) >= q - 1e-6 && into(t) < q + bar - 1e-6), "seed {seed}: a bell at {t:.2} s in a full stop");
            }
            // Each full stop that comes while a ring may sound damps it.
            for &q in &quiet {
                let mut at = start + q;
                while at < 40.0 - bar {
                    let last = heard.bells.iter().map(|b| b.0).filter(|&t| t < at).fold(f64::NEG_INFINITY, f64::max);
                    if last + 5.5 > at {
                        assert!(heard.chokes.iter().any(|&c| c > last && c < at + 0.05), "seed {seed}: no damp by the full stop at {at:.2} s");
                        stops += 1;
                    }
                    at += total;
                }
            }
        }
        println!("{stops} full stops damped a ring");
        assert!(stops > 5, "{stops} full stops");
    });
}

/// One low bell alone through the core (`parts`, `heard`: the core
/// stamps what it heard on the chords channel, so a pad is played there
/// first when `under`), and the bell's samples from its strike at 1 s.
fn bell_alone(midi: f64, under: bool) -> Vec<f32> {
    let run = |bell: bool| {
        let mut core = Box::new(Core::new());
        core.start(48_000.0, 1).expect("a core");
        if under {
            for (k, n) in [55.0, 59.0, 62.0].into_iter().enumerate() {
                core.note(PAD, CHORDS, 0.0 + 0.01 * k as f64, n, 6.0, 0.8, 0, [3.0, f64::NAN, f64::NAN, f64::NAN, f64::NAN, f64::NAN, f64::NAN, f64::NAN, f64::NAN, f64::NAN, f64::NAN, f64::NAN]);
            }
        }
        if bell {
            core.note(BELL, CHORDS, 1.0, midi, 5.5, 1.0, 0, [f64::NAN; EXTRA]);
        }
        let mut out = Vec::new();
        let mut block = vec![0.0f32; QUANTUM];
        for _ in 0..(5 * 48_000 / QUANTUM) {
            core.render(&mut [&mut block]).expect("a block");
            out.extend_from_slice(&block);
        }
        out
    };
    let (with, without) = (run(true), run(false));
    with.iter().zip(&without).skip(48_000).map(|(a, b)| a - b).collect()
}

fn db(x: &[f32]) -> f64 {
    let p = x.iter().map(|&v| f64::from(v) * f64::from(v)).sum::<f64>() / x.len() as f64;
    10.0 * p.log10()
}

#[test]
fn the_voice_falls_8_to_25_db_a_second_with_its_partials_off_whole_multiples() {
    roomy(|| {
        // The partials (the voice's, the sketch's): 60 to 68 cents off.
        for ratio in [2.08f64, 3.11, 5.2] {
            let cents = 1200.0 * (ratio / ratio.round()).log2();
            assert!((50.0..=90.0).contains(&cents.abs()), "{ratio}: {cents:.1} cents off");
        }
        let x = bell_alone(45.0, false);
        let at = |a: f64, b: f64| db(&x[(a * 48_000.0) as usize..(b * 48_000.0) as usize]);
        let fall = (at(0.3, 0.5) - at(2.3, 2.5)) / 2.0;
        println!("the low bell falls {fall:.1} dB a second");
        assert!((8.0..=25.0).contains(&fall), "{fall:.1} dB a second");
        let peak = x.iter().fold(0.0f32, |m, &v| m.max(v.abs()));
        assert!(20.0 * f64::from(peak).log10() < -1.0, "its peak alone");
    });
}

#[test]
fn its_level_follows_the_chords_channel() {
    roomy(|| {
        let alone = db(&bell_alone(45.0, false)[..48_000]);
        let over = db(&bell_alone(45.0, true)[..48_000]);
        println!("the bell alone {alone:.1} dB, over a held chord {over:.1} dB");
        assert!(over > alone + 1.0, "louder over a louder chord layer");
        // Never past the most: 0.12 against the floor's 0.045.
        assert!(over - alone < 20.0 * (0.12f64 / 0.045).log10() + 0.1);
    });
}
