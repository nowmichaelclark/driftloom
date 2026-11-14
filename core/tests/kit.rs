//! Batch 6's kits (queue item 101d; `generator::kit`): tripletKit, and
//! dottedPulse below. tripletKit: a kit in
//! threes, a choice drawn by itself (`alone`), not among the changes, on a
//! stream of its own. What it holds to: it goes only to loops with a kit and
//! not in 6/8, for about its share of them, leaned by the vibes as the
//! batch's table says; a pin plays it or keeps it off; where it plays, every
//! hit sounds on a third of the beat (the kick on the beat), none on the
//! straight eighth, and nothing but the kit moves. That no other choice
//! moved is the generation goldens' (the PR has the count).

#![allow(clippy::cast_precision_loss, clippy::float_cmp)] // counts of loops, as shares; steps copied, not computed

use dlcore::generator::Generator;
use dlcore::generator::plan::Cx;
use dlcore::generator::request::Request;
use dlcore::generator::rng::Rng;
use dlcore::generator::spec::Metre;

fn roomy(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new().stack_size(64 << 20).spawn(f).expect("a thread").join().expect("the test");
}

/// The seeds of the batch's corpus (corpus seed 20261009), its first `n`.
fn corpus(n: usize) -> Vec<u32> {
    let mut r = Rng::new(20_261_009);
    (0..n).map(|_| r.seed32()).collect()
}

fn has_kit(g: &Generator) -> bool {
    g.render().drums.iter().any(|e| e.vel != 0.0)
}

fn six_eight(g: &Generator) -> bool {
    Cx::new(g.spec()).metre == Metre::SixEight
}

/// Of `n` corpus loops as `request` asks them: how many may have the kit in
/// threes, and how many play it.
fn share(g: &mut Generator, request: &Request, n: usize) -> (usize, usize) {
    let (mut of, mut on) = (0, 0);
    for seed in corpus(n) {
        g.load_with(seed, request);
        let plays = g.render().choice("tripletKit") == Some("on");
        if has_kit(g) && !six_eight(g) {
            of += 1;
            on += usize::from(plays);
        } else {
            assert!(!plays, "seed {seed}: no kit, or 6/8, and no kit in threes");
        }
    }
    (of, on)
}

#[test]
fn it_goes_to_its_share_of_the_loops_with_a_kit_but_never_in_six_eight() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let (of, on) = share(&mut g, &Request::new(), 1500);
        let f = on as f64 / of as f64;
        assert!(of > 300 && (0.11..=0.19).contains(&f), "{on} of {of} ({f:.3}), not near 15%");
    });
}

#[test]
fn the_vibes_lean_it_toward_swaying_and_away_from_still() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let ask = |t: &str| Request::parse(t).expect("a request");
        let swaying = share(&mut g, &ask("swaying"), 800);
        let held = share(&mut g, &ask("still"), 800);
        let (sway, still) = (swaying.1 as f64 / swaying.0 as f64, held.1 as f64 / held.0 as f64);
        assert!(sway > 0.2 && still < 0.1 && still > 0.0, "swaying {sway:.3}, still {still:.3}");
    });
}

/// Where each sounding hit of the loop's kit falls, in steps from the
/// loop's start, the swing an odd step takes included; with whether it is
/// the floor's.
fn places(g: &Generator) -> Vec<(f64, bool)> {
    let swing = g.spec().swing;
    g.render()
        .drums
        .iter()
        .filter(|e| e.vel != 0.0)
        .map(|e| (e.step + e.micro.unwrap_or(0.0) + if e.step % 2.0 == 1.0 { swing } else { 0.0 }, dlcore::generator::kit::is_kick(e.inst.get())))
        .collect()
}

#[test]
fn pinned_every_hit_is_on_a_third_and_nothing_but_the_kit_moves() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let on = Request::parse("tripletKit=on").expect("a request");
        let off = Request::parse("tripletKit=today").expect("a request");
        let (mut played, mut written) = (0, 0);
        for seed in corpus(120) {
            g.load_with(seed, &Request::new());
            let drew = g.render().choice("tripletKit");
            let plain = Box::new((g.render().bass.iter().map(|e| (e.step, e.midi, e.vel)).collect::<Vec<_>>(), g.render().melody.iter().map(|e| (e.step, e.midi, e.vel)).collect::<Vec<_>>(), g.render().choices.iter().map(|w| w.get()).collect::<Vec<_>>(), g.render().drums.len()));
            g.load_with(seed, &off);
            assert_eq!(g.render().choice("tripletKit"), Some("today"), "seed {seed}: pinned off");
            if drew != Some("on") {
                assert_eq!(g.render().drums.len(), plain.3, "seed {seed}: today's kit");
            }
            g.load_with(seed, &on);
            if !has_kit(&g) || six_eight(&g) {
                assert_eq!(g.render().choice("tripletKit"), Some("today"), "seed {seed}: a pin plays it only where it may go");
                continue;
            }
            assert_eq!(g.render().choice("tripletKit"), Some("on"), "seed {seed}");
            played += 1;
            for (at, kick) in places(&g) {
                let third = at * 3.0 / 4.0;
                if kick {
                    assert!((at % 4.0).abs() < 1e-9, "seed {seed}: a kick at {at}, off the beat");
                } else {
                    assert!((third - third.round()).abs() < 1e-9, "seed {seed}: a hit at {at}, not on a third");
                }
            }
            if g.render().drums.iter().any(|e| e.micro == Some(2.0 / 3.0) && e.vel != 0.0) {
                written += 1;
            }
            let r = g.render();
            assert_eq!(r.bass.iter().map(|e| (e.step, e.midi, e.vel)).collect::<Vec<_>>(), plain.0, "seed {seed}: the bass as it was");
            assert_eq!(r.melody.iter().map(|e| (e.step, e.midi, e.vel)).collect::<Vec<_>>(), plain.1, "seed {seed}: the tune as it was");
            for (i, (a, b)) in r.choices.iter().zip(plain.2.iter()).enumerate() {
                if dlcore::generator::data::CHOICES[i].id != "tripletKit" {
                    assert_eq!(a.get(), *b, "seed {seed}: no other choice moved");
                }
            }
            assert_eq!(r.lost(), 0, "seed {seed}: no list overflowed");
        }
        assert!(played > 30 && written > 20, "{played} loops played it, {written} with a third");
    });
}

// ---------------------------------------------------------------------
// dottedPulse: a part every three eighths (`pulse`), or the kick on a 3-3-2
// (`kick332`), drawn alone on a stream of its own, for loops with a kit in
// bars of 16 steps, never beside a kit in threes.

fn may_dot(g: &Generator) -> bool {
    has_kit(g) && g.spec().steps_per_bar == 16.0 && !six_eight(g) && g.render().choice("tripletKit") != Some("on")
}

#[test]
fn a_dotted_pulse_goes_to_its_share_of_the_four_four_kits_and_never_beside_threes() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let (mut of, mut on, mut kicked) = (0usize, 0usize, 0usize);
        for seed in corpus(1500) {
            g.load_with(seed, &Request::new());
            let drew = g.render().choice("dottedPulse");
            if may_dot(&g) {
                of += 1;
                on += usize::from(drew != Some("today"));
                kicked += usize::from(drew == Some("kick332"));
            } else {
                assert_eq!(drew, Some("today"), "seed {seed}: no 4/4 kit, or a kit in threes, and no dotted pulse");
            }
        }
        let f = on as f64 / of as f64;
        assert!(of > 250 && (0.11..=0.19).contains(&f), "{on} of {of} ({f:.3}), not near 15%");
        assert!(kicked * 3 > on && kicked * 3 < 2 * on, "about half each: {kicked} of {on} a 3-3-2");
        let both = Request::parse("tripletKit=on,dottedPulse=pulse").expect("a request");
        for seed in corpus(60) {
            g.load_with(seed, &both);
            if g.render().choice("tripletKit") == Some("on") {
                assert_eq!(g.render().choice("dottedPulse"), Some("today"), "seed {seed}: never both");
            }
        }
    });
}

#[test]
fn pinned_a_pulse_every_six_steps_or_a_3_3_2_and_nothing_but_the_kit_moves() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let ask = |t: &str| Request::parse(t).expect("a request");
        let (mut pulsed, mut kicked) = (0, 0);
        for seed in corpus(120) {
            g.load_with(seed, &ask("tripletKit=today,dottedPulse=today"));
            if !may_dot(&g) {
                continue;
            }
            let plain = Box::new((g.render().bass.iter().map(|e| (e.step, e.midi, e.vel)).collect::<Vec<_>>(), g.render().melody.iter().map(|e| (e.step, e.midi, e.vel)).collect::<Vec<_>>(), g.render().choices.iter().map(|w| w.get()).collect::<Vec<_>>()));
            let floor = |g: &Generator| g.render().drums.iter().any(|e| e.vel != 0.0 && dlcore::generator::kit::is_kick(e.inst.get()));
            let had_kick = floor(&g);
            for (option, check) in [("pulse", 0), ("kick332", 1)] {
                g.load_with(seed, &ask(&format!("tripletKit=today,dottedPulse={option}")));
                let r = g.render();
                if option == "kick332" && !had_kick {
                    assert_eq!(r.choice("dottedPulse"), Some("today"), "seed {seed}: no kick, no 3-3-2");
                    continue;
                }
                assert_eq!(r.choice("dottedPulse"), Some(option), "seed {seed}");
                let written = &r.drums[r.drums_out.written..];
                if check == 0 {
                    assert!(written.iter().all(|e| e.step % 6.0 == 0.0 && (e.inst == "rim" || e.inst == "tap")), "seed {seed}: the pulse every six steps");
                    pulsed += usize::from(written.iter().any(|e| e.vel != 0.0));
                } else {
                    for e in r.drums.iter().filter(|e| e.vel != 0.0 && !e.roll && dlcore::generator::kit::is_kick(e.inst.get())) {
                        assert!([0.0, 6.0, 12.0].contains(&(e.step % 16.0)), "seed {seed}: a kick at {} of the bar", e.step % 16.0);
                    }
                    kicked += usize::from(floor(&g));
                }
                assert_eq!(r.bass.iter().map(|e| (e.step, e.midi, e.vel)).collect::<Vec<_>>(), plain.0, "seed {seed}: the bass as it was");
                assert_eq!(r.melody.iter().map(|e| (e.step, e.midi, e.vel)).collect::<Vec<_>>(), plain.1, "seed {seed}: the tune as it was");
                for (i, (a, b)) in r.choices.iter().zip(plain.2.iter()).enumerate() {
                    if dlcore::generator::data::CHOICES[i].id != "dottedPulse" {
                        assert_eq!(a.get(), *b, "seed {seed}: no other choice moved");
                    }
                }
                assert_eq!(r.lost(), 0, "seed {seed}: no list overflowed");
            }
        }
        assert!(pulsed > 15 && kicked > 15, "{pulsed} pulsed, {kicked} kicked");
    });
}
