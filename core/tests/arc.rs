//! An arc heard (queue item 102c; `generator::heard`, `docs/LONGFORM.md`,
//! "How an arc is heard"): an arc's sections carry its energy as marks and
//! levels, and a finite arc lands on its tonic. `tools/longform.mjs` measures
//! what is played (its loudness and brightness, its last chord, its
//! ring-out); these check the marks.
//!
//! - each unit's levels, kit, tune, filter aim and double follow its energy:
//!   the lower the energy, the lower each fading layer, the darker the aim,
//!   the thinner the tune; the chords and the tune never rest but in a
//!   breath;
//! - a unit starts where the one before it ended, so nothing jumps, and the
//!   round after a close starts from it, at its opening;
//! - what was asked holds: a layer asked on, off or re-rolled is its own
//!   throughout, a tone edited is not moved, an arc asked keeps its filter;
//! - a finite arc's last section lands: the tonic's triad, every layer's own
//!   notes left out from its first step, the tune's last phrase whole before
//!   it; Endless never lands;
//! - the player draws an arc's sections as passes at 0, 1 and 2.4, and the
//!   marks are the same at each.

// Units and steps, far below 2^53.
#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
// The plan's numbers are compared exactly on purpose: the same draw gives
// the same double.
#![allow(clippy::float_cmp)]

use dlcore::generator::heard::{Heard, Unit};
use dlcore::generator::longform::{Kind, Role};
use dlcore::generator::request::{Journey, Request};
use dlcore::generator::rng::Rng;
use dlcore::generator::theory::SCALES;
use dlcore::generator::{Generator, Layer};
use dlcore::player::host::Host;
use dlcore::player::{Player, State as Playing};
use dlcore::{EXTRA, Key, Tap};

/// A load needs more stack than a test's thread has (`tests/length.rs`).
fn roomy<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    let stack = (std::mem::size_of::<Player>() + std::mem::size_of::<Generator>()) * 4 + (16 << 20);
    std::thread::Builder::new().stack_size(stack).spawn(f).expect("a thread").join().expect("the test")
}

/// The corpus's first seeds.
fn seeds(n: usize) -> Vec<u32> {
    let mut r = Rng::new(20_261_009);
    (0..n).map(|_| (r.f() * 4_294_967_296.0) as u32).collect()
}

/// What a unit ends as, the gliding things: each layer's cut, the tone, the
/// tune's thinning, the kit's softness and shimmer, the double, the aim.
fn ends(u: &Unit) -> Vec<f64> {
    let mut v: Vec<f64> = u.cut.iter().map(|c| c[1]).collect();
    v.extend([u.tone[0][1], u.tone[1][1], u.thin[1], u.soft[1], u.shimmer[1], u.backbeat[1], u.doubled[1], u.aim[1]]);
    v
}

/// And what it starts as.
fn starts(u: &Unit) -> Vec<f64> {
    let mut v: Vec<f64> = u.cut.iter().map(|c| c[0]).collect();
    v.extend([u.tone[0][0], u.tone[1][0], u.thin[0], u.soft[0], u.shimmer[0], u.backbeat[0], u.doubled[0], u.aim[0]]);
    v
}

#[test]
fn an_arcs_units_follow_its_energy_and_glide() {
    roomy(|| {
        let mut generator = Box::new(Generator::new());
        let (mut down, mut dark, mut thin, mut doubled) = (0, 0, 0, 0);
        for seed in seeds(40) {
            for text in ["bars=64,journey=arc", "bars=256,journey=arc", "bars=endless,journey=arc"] {
                generator.load_with(seed, &Request::parse(text).expect("a request"));
                let form = generator.form().expect("a plan");
                let what = format!("seed {seed} {text}");
                let mut last: Option<Vec<f64>> = None;
                // (energy, the bass's level, the aim) for every unit seen.
                let mut seen: Vec<(f64, f64, f64)> = Vec::new();
                for n in 0..120u64.min(form.to_end(0).map_or(120, |b| (b / generator.spec().bars).ceil() as u64)) {
                    let heard: Heard = generator.section(n, 1.0).heard;
                    assert!(heard.on && heard.arc, "{what}: section {n} hears no arc");
                    for (p, u) in heard.units().iter().enumerate() {
                        let span = form.at(n, p as u64);
                        if let Some(l) = &mut last {
                            // (A theme stated comes in whole on its line,
                            // session 14: the tune never fills in from the
                            // outline before it.)
                            if matches!(span.role, Role::Second | Role::Return) && u.thin[0] == u.thin[1] {
                                l[7] = u.thin[0];
                            }
                            assert_eq!(&starts(u), l, "{what}: section {n} unit {p} jumps from the unit before");
                        }
                        last = Some(ends(u));
                        for layer in [Layer::Chords, Layer::Melody] {
                            // (But in an interlude, the wander's way: session 14.)
                            assert!(!u.rests(layer) || (layer == Layer::Melody && span.role == Role::Breath) || span.kind == Kind::Interlude, "{what}: section {n} rests {layer:?}");
                        }
                        let bass = 1.0 - u.cut[Layer::Bass as usize][1];
                        seen.push((span.energy, bass, u.aim[1]));
                        down += usize::from(u.cut[Layer::Drums as usize][1] > 0.0);
                        dark += usize::from(u.aim[1] < 0.0);
                        thin += usize::from(u.thin[1] > 0.0);
                        doubled += usize::from(u.double != 0 && u.doubled[1] > 0.0);
                    }
                }
                // The smoothed energy follows the plan's: the bass's level
                // and the aim never fall as the plan's energy rises past
                // what the smoothing moves (a tenth).
                for a in &seen {
                    for b in &seen {
                        if b.0 > a.0 + 0.2 {
                            assert!(b.1 >= a.1 - 1e-9 && b.2 >= a.2 - 1e-9, "{what}: energy {} plays above {}: {a:?} {b:?}", b.0, a.0);
                        }
                    }
                }
            }
        }
        assert!(down > 0 && dark > 0 && thin > 0 && doubled > 0, "{down} {dark} {thin} {doubled}");
    });
}

#[test]
fn what_was_asked_holds_in_an_arc() {
    roomy(|| {
        let mut generator = Box::new(Generator::new());
        for seed in seeds(20) {
            for (ask, check) in [
                ("layer.drums=on,layer.bass=on,edit.roll.texture=4", 0),
                ("edit.warmth=0.6,edit.space=0.3", 1),
                ("arc.period=200", 2),
            ] {
                let text = format!("bars=256,journey=arc,{ask}");
                generator.load_with(seed, &Request::parse(&text).expect("a request"));
                for n in 0..40u64 {
                    let heard = generator.section(n, 1.0).heard;
                    for u in heard.units() {
                        match check {
                            0 => {
                                for layer in [Layer::Drums, Layer::Bass, Layer::Texture] {
                                    assert_eq!(u.cut[layer as usize], [0.0; 2], "seed {seed} {text}: {layer:?} moved");
                                }
                            }
                            1 => assert!(u.tone[1] == [0.0; 2] && u.aim == [0.0; 2], "seed {seed} {text}: the tone moved"),
                            _ => assert_eq!(u.aim, [0.0; 2], "seed {seed} {text}: an arc asked was aimed"),
                        }
                    }
                }
            }
        }
    });
}

#[test]
fn a_finite_arc_lands_on_its_tonic_and_endless_never() {
    roomy(|| {
        let mut generator = Box::new(Generator::new());
        let mut landed = 0;
        for seed in seeds(40) {
            // 100 bars: a last section cut short (most loops' bars don't
            // divide it).
            for text in ["bars=64,journey=arc", "bars=100,journey=arc", "bars=256,journey=arc", "bars=1024,journey=arc"] {
                generator.load_with(seed, &Request::parse(text).expect("a request"));
                let form = generator.form().expect("a plan");
                let what = format!("seed {seed} {text}");
                let spb = generator.spec().steps_per_bar;
                let sections = (form.to_end(0).expect("a finite piece") / generator.spec().bars).ceil() as u64;
                let last = sections - 1;
                // The landing: the round's last bars, its seconds and a bar
                // at least, struck once, in the section it starts in, and
                // every section from there all landing.
                let bar = form.bar_seconds();
                let land = (dlcore::generator::data::ENERGY.land / bar).ceil().max(1.0);
                let mut struck = Vec::new();
                for n in 0..sections {
                    let to = form.to_end(n).expect("a finite piece");
                    // The section's own bars: to the next one's start.
                    let own = to - if n + 1 < sections { form.to_end(n + 1).expect("a finite piece") } else { 0.0 };
                    let heard = generator.section(n, 1.0).heard;
                    assert_eq!(heard.landing.on, to - land < own, "{what}: section {n} lands {} with {to} bars left", heard.landing.on);
                    if heard.landing.strikes {
                        struck.push(n);
                        assert_eq!(heard.landing.at, (to - land) * spb, "{what}: section {n} strikes off its line");
                    }
                }
                assert_eq!(struck.len(), 1, "{what}: struck in {struck:?}");
                let steps = form.to_end(last).expect("a finite piece") * spb;
                let closing: Heard = {
                    let pass = generator.section(last, 1.0);
                    let l = pass.heard.landing;
                    assert!(l.on && l.hold >= 1.0, "{what}: the last section does not land");
                    // Every layer's own notes are left out from its first step.
                    for layer in dlcore::generator::LAYERS {
                        assert!(pass.rests(l.at, layer) && pass.rests(steps - 1.0, layer), "{what}: {layer:?} plays in the landing");
                    }
                    pass.heard
                };
                let l = generator.section(struck[0], 1.0).heard.landing;
                // The tonic's triad, the root lowest.
                if !l.chord.is_empty() {
                    let spec = generator.spec();
                    let root = (spec.root.round() as i16).rem_euclid(12);
                    let steps = SCALES[spec.scale].steps;
                    let third: i16 = if steps.contains(&4.0) { 4 } else if steps.contains(&3.0) { 3 } else { steps[1] as i16 };
                    let fifth: i16 = if steps.contains(&7.0) { 7 } else if steps.contains(&8.0) { 8 } else { 6 };
                    let pcs: Vec<i16> = l.chord.iter().map(|n| n.rem_euclid(12)).collect();
                    for t in [0, third, fifth] {
                        assert!(pcs.contains(&(root + t).rem_euclid(12)), "{what}: the landing {:?} has no {t} over {root}", &l.chord[..]);
                    }
                    assert_eq!(l.chord.iter().min().map(|n| n.rem_euclid(12)), Some(root), "{what}: the landing is not in root position");
                    landed += 1;
                }
                if l.tune > 0.0 {
                    assert_eq!((l.tune.round() as i16 - generator.spec().root.round() as i16).rem_euclid(12), 0, "{what}: the tune lands off the tonic");
                }
                // The round after starts at the opening, from the close.
                let again = generator.section(sections, 1.0).heard;
                let first = generator.section(0, 1.0).heard;
                assert!(!again.landing.on, "{what}: the round after lands");
                assert_eq!(again.units().iter().map(ends).collect::<Vec<_>>(), first.units().iter().map(ends).collect::<Vec<_>>(), "{what}: the round after is not its opening");
                assert_eq!(starts(&again.units()[0]), ends(closing.units().last().expect("a unit")), "{what}: the round after jumps from the close");
            }
            generator.load_with(seed, &Request::parse("bars=endless,journey=arc").expect("a request"));
            for n in [0u64, 7, 300, 1_000_000_007] {
                assert!(!generator.section(n, 1.0).heard.landing.on, "seed {seed}: Endless lands at {n}");
            }
        }
        assert!(landed > 100, "{landed}");
    });
}

/// A host keeping the time and nothing else.
#[derive(Default)]
struct Clock {
    frame: u64,
}

impl Host for Clock {
    fn frame(&self) -> u64 {
        self.frame
    }
    fn key(&mut self, _: Key) {}
    fn note(&mut self, _: u32, _: u32, _: f64, _: f64, _: f64, _: f64, _: u32, _: [f64; EXTRA]) {}
    fn event(&mut self, _: u32, _: dlcore::block::Event) {}
    fn damp(&mut self, _: u32, _: f64) {}
    fn set(&mut self, _: u32, _: usize, _: usize, _: f64, _: f64) {}
    fn route(&mut self, _: u32, _: usize, _: usize, _: Option<Tap>, _: f64) {}
    fn param(&mut self, _: u32, _: u32, _: f64, _: f64, _: f64) {}
    fn steal(&mut self, _: u32, _: Key, _: f64, _: f64) {}
    fn fade_all(&mut self, _: f64, _: f64) {}
    fn bass_lift(&mut self, _: f64) {}
    fn space_range(&mut self, _: usize, _: f64, _: f64) {}
}

#[test]
fn an_arc_is_heard_at_any_slider_setting() {
    roomy(|| {
        let request = Request::parse("bars=endless").expect("a request").with_journey(Journey::Arc);
        let mut marks: Vec<Vec<String>> = Vec::new();
        for amount in [0.0, 1.0, 2.4] {
            let mut clock = Clock::default();
            let mut player = Box::new(Player::new());
            player.attach(&mut clock, 44_100.0, true);
            player.set_wander(amount);
            player.load(7, &request);
            player.ready(&mut clock);
            assert_eq!(player.state(), Playing::Playing);
            let mut seen = Vec::new();
            let mut at = u32::MAX;
            let mut guard = 0u64;
            while player.position().1 < 80 {
                player.decide(&mut clock);
                clock.frame += 4096;
                guard += 1;
                assert!(guard < 50_000_000, "the player stopped deciding");
                let (_, section, _) = player.position();
                if section != at {
                    at = section;
                    let pass = player.pass().expect("an arc's section is a pass at any setting");
                    assert!(pass.heard.on && pass.heard.arc, "section {section} at {amount} hears no arc");
                    assert_eq!(pass.breathes, amount > 0.0, "the slider is the breathing");
                    seen.push(format!("{:?}", pass.heard.units().iter().map(|u| (ends(u), starts(u), u.stretch, u.double)).collect::<Vec<_>>()));
                }
            }
            marks.push(seen);
        }
        assert!(marks[0] == marks[1] && marks[1] == marks[2], "the marks moved with the wander's amount");
    });
}
