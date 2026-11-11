//! Batch 6's far ends (queue item 101a): deepDuck (subDuck's `deep`),
//! wideSweep (padSweep's `wide` and `fast`) and slowOpening (lateFloor's
//! `slow`), each an option of an approved choice, drawn on the choice's own
//! stream (`ownStream`). What they hold to: each option is drawn, within its
//! own ranges and guards (deep's recovery never past the kick spacing, fast
//! only with a kit, slow's fade never over 20 s); the vibes lean them as the
//! batch's table says; a pin is held to the range of the option the loop
//! plays; and a slow opening's kit and bass rise from nothing to exactly the
//! velocities the approved kind plays, from their arrival. That nothing else
//! moves is the generation goldens' (every case moved only in these three
//! choices, from `on` to a new option; the PR has the count).

// Velocities and settings are compared exactly on purpose: the fade is a
// factor of exactly 1 once it is over. Counts of loops, a few hundred, are
// shares as floats.
#![allow(clippy::float_cmp, clippy::cast_precision_loss)]

use dlcore::block::Event;
use dlcore::generator::Generator;
use dlcore::generator::choices::{LateFloor, kick_spacing, sweep_ranges};
use dlcore::generator::data::{LATEFLOOR, SUBDUCK};
use dlcore::generator::layers::{DrumEvent, NO_DRUM};
use dlcore::generator::request::{Request, SETTING_OUT};
use dlcore::generator::rng::Rng;
use dlcore::player::host::Host;
use dlcore::player::{Player, State};
use dlcore::{DRUMS_CHANNEL, EXTRA, Key, QUANTUM, Tap};

fn roomy(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new().stack_size(64 << 20).spawn(f).expect("a thread").join().expect("the test");
}

/// The seeds of the batch's corpus (corpus seed 20261009), its first `n`.
fn corpus(n: usize) -> Vec<u32> {
    let mut r = Rng::new(20_261_009);
    (0..n).map(|_| r.seed32()).collect()
}

fn inside(x: f64, [lo, hi]: [f64; 2]) -> bool {
    x >= lo - 1e-12 && x <= hi + 1e-12
}

#[test]
fn the_kick_spacing_is_the_middle_gap_around_the_cycle() {
    let kicks = |steps: &[f64]| -> Vec<DrumEvent> {
        let mut v: Vec<DrumEvent> = steps.iter().map(|&s| DrumEvent { step: s, inst: "kick".into(), vel: 0.9, vel0: 0.9, ..NO_DRUM }).collect();
        v.push(DrumEvent { step: 2.0, inst: "hat".into(), vel: 0.5, vel0: 0.5, ..NO_DRUM });
        v
    };
    // Four on the floor in a 16-step bar: a beat.
    assert_eq!(kick_spacing(&kicks(&[0.0, 4.0, 8.0, 12.0]), 16.0, 0.125), Some(0.5));
    // A double kick (gaps 1, 7, 8): the middle one, not the shortest.
    assert_eq!(kick_spacing(&kicks(&[0.0, 1.0, 8.0]), 16.0, 0.125), Some(0.875));
    // One kick a cycle: the cycle; a kick counted once however often it is
    // written; a hit at a step past the cycle taken where it falls.
    assert_eq!(kick_spacing(&kicks(&[3.0]), 32.0, 0.1), Some(3.2));
    assert_eq!(kick_spacing(&kicks(&[0.0, 0.0, 16.0]), 16.0, 0.125), Some(2.0));
    assert_eq!(kick_spacing(&kicks(&[]), 16.0, 0.125), None);
    let silent = vec![DrumEvent { step: 0.0, inst: "kick".into(), vel: 0.0, vel0: 0.0, ..NO_DRUM }];
    assert_eq!(kick_spacing(&silent, 16.0, 0.125), None);
}

#[test]
fn a_rise_is_one_exactly_without_a_fade() {
    let late = |fade: f64| LateFloor { order: "together", bars: 8.0, gap: 1.0, drums: 128.0, bass: 128.0, fade };
    for since in [0.0, 0.5, 3.0, 1e6] {
        assert_eq!(late(0.0).rise(since, 0.125), 1.0);
    }
    assert_eq!(late(10.0).rise(0.0, 0.125), 0.0);
    assert_eq!(late(10.0).rise(40.0, 0.125), 0.5);
    assert_eq!(late(10.0).rise(80.0, 0.125), 1.0);
    assert_eq!(late(10.0).rise(1e4, 0.125), 1.0);
}

#[test]
fn each_far_end_is_drawn_inside_its_ranges_and_guards() {
    roomy(drawn);
}

fn drawn() {
    let mut made = Box::new(Generator::new());
    let (mut deep, mut wide, mut fast, mut slow, mut approved) = (0, 0, 0, 0, [0; 3]);
    for seed in corpus(400) {
        made.load_with(seed, &Request::new());
        let (r, s, spec) = (made.render(), made.settings(), made.spec());
        let step = 60.0 / spec.bpm / 4.0;
        let bar = spec.steps_per_bar * step;
        match r.choice("subDuck") {
            Some("deep") => {
                deep += 1;
                let d = s.sub_duck.expect("deep plays a duck");
                let cycle = spec.cycles.of(0).filter(|&c| c > 0.0).unwrap_or(spec.bars * spec.steps_per_bar);
                let spacing = kick_spacing(&r.drums, cycle, step).expect("a deep duck has a kick");
                assert!(inside(d.depth, SUBDUCK.deep_depth) && d.depth <= 16.0, "seed {seed}: {d:?}");
                assert!(d.recover <= spacing && d.recover <= SUBDUCK.deep_recover[1], "seed {seed}: {d:?}, kicks every {spacing} s");
                assert!(d.recover == spacing || inside(d.recover, SUBDUCK.deep_recover), "seed {seed}: {d:?}");
            }
            Some("on") => {
                approved[0] += 1;
                let d = s.sub_duck.expect("a duck");
                assert!(inside(d.depth, SUBDUCK.depth) && d.recover <= SUBDUCK.recover[1], "seed {seed}: {d:?}");
            }
            _ => assert!(s.sub_duck.is_none()),
        }
        match r.choice("padSweep") {
            Some(kind @ ("on" | "wide" | "fast")) => {
                let p = s.pad_sweep.expect("a sweep");
                let k = sweep_ranges(kind);
                assert!(inside(p.lo, k.lo) && inside(p.oct, k.oct) && inside(p.period, k.period) && inside(p.q, k.q), "seed {seed}: {kind} {p:?}");
                match kind {
                    "wide" => wide += 1,
                    "fast" => {
                        fast += 1;
                        assert!(r.drums.iter().any(|e| e.vel != 0.0), "seed {seed}: fast with no kit");
                    }
                    _ => approved[1] += 1,
                }
            }
            _ => assert!(s.pad_sweep.is_none()),
        }
        match r.choice("lateFloor") {
            Some("slow") => {
                slow += 1;
                let l = s.late.expect("a wait");
                let wait = l.bars * bar;
                assert!(wait >= LATEFLOOR.slow_seconds[0] - 2.0 * bar && wait <= LATEFLOOR.slow_seconds[1] + 2.0 * bar, "seed {seed}: waits {wait} s");
                assert!(inside(l.fade, LATEFLOOR.slow_fade) && l.fade <= 20.0, "seed {seed}: fades over {} s", l.fade);
            }
            Some("on") => {
                approved[2] += 1;
                let l = s.late.expect("a wait");
                assert_eq!(l.fade, 0.0, "seed {seed}: the approved kind arrives at once");
                assert!(l.bars * bar <= LATEFLOOR.seconds[1] + 2.0 * bar, "seed {seed}");
            }
            _ => assert!(s.late.is_none()),
        }
    }
    println!("400 loops: deep {deep}, wide {wide}, fast {fast}, slow {slow}; the approved subDuck, padSweep, lateFloor {approved:?}");
    assert!(deep > 0 && wide > 0 && fast > 0 && slow > 0, "every far end is drawn");
    assert!(approved.iter().all(|&n| n > 0), "and so is every approved kind");
}

/// How many of `n` loops with the vibe `vibe` play `option` of `choice`,
/// out of those that play the choice at all.
fn share(g: &mut Generator, vibe: &str, choice: &str, option: &str, n: usize) -> (usize, usize) {
    let request = Request::new().with_vibe(vibe).expect("a vibe");
    let (mut hit, mut of) = (0, 0);
    for seed in corpus(n) {
        g.load_with(seed, &request);
        match g.render().choice(choice) {
            Some(o) if o == option => {
                hit += 1;
                of += 1;
            }
            Some("today") | None => {}
            Some(_) => of += 1,
        }
    }
    (hit, of)
}

#[test]
fn the_vibes_lean_them_as_the_batch_s_table_says() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        for (choice, option, toward, away) in
            [("subDuck", "deep", "pulsing", "hushed"), ("padSweep", "wide", "hypnotic", "still"), ("lateFloor", "slow", "meditative", "lively")]
        {
            let (a, an) = share(&mut g, toward, choice, option, 300);
            let (b, bn) = share(&mut g, away, choice, option, 300);
            let (fa, fb) = (a as f64 / an.max(1) as f64, b as f64 / bn.max(1) as f64);
            println!("{choice}={option}: {toward} {a}/{an}, {away} {b}/{bn}");
            assert!(an > 0 && fa > fb, "{choice}={option}: {toward} {fa:.2} against {away} {fb:.2}");
        }
    });
}

/// The first of the corpus's loops (up to 400) where `test` holds.
fn find(g: &mut Generator, test: impl Fn(&Generator) -> bool) -> u32 {
    corpus(400)
        .into_iter()
        .find(|&seed| {
            g.load_with(seed, &Request::new());
            test(g)
        })
        .expect("a loop of the corpus")
}

#[test]
fn a_pin_is_held_to_the_range_of_the_option_played() {
    roomy(|| {
        for (text, line) in [
            ("subDuck.depth=16.5", Some(SETTING_OUT)),
            ("subDuck.depth=14", None),
            ("lateFloor.fade=21", Some(SETTING_OUT)),
            ("lateFloor.fade=3", Some(SETTING_OUT)),
            ("lateFloor.seconds=91", Some(SETTING_OUT)),
            ("padSweep.oct=4.6", Some(SETTING_OUT)),
            ("padSweep.period=4", Some(SETTING_OUT)),
            ("subDuck=deep,padSweep=fast,lateFloor=slow,lateFloor.fade=12", None),
        ] {
            assert_eq!(Request::parse(text).err(), line, "{text}");
        }
        let mut g = Box::new(Generator::new());
        let ask = |g: &mut Generator, seed: u32, text: &str| {
            g.load_with(seed, &Request::parse(text).expect("a request"));
            *g.settings()
        };
        // A loop with a kick, a bass and a sustained chord voice.
        let seed = find(&mut g, |g| g.render().choice("subDuck").is_some_and(|o| o != "today") && g.render().choice("padSweep").is_some_and(|o| o != "today"));
        let d = ask(&mut g, seed, "subDuck=deep,subDuck.depth=14,subDuck.recover=0.6").sub_duck.expect("deep");
        assert_eq!(d.depth, 14.0);
        assert!(d.recover <= 0.6);
        let d = ask(&mut g, seed, "subDuck=on,subDuck.depth=14,subDuck.recover=0.6").sub_duck.expect("the approved duck");
        assert_eq!((d.depth, d.recover), (SUBDUCK.depth[1], SUBDUCK.recover[1]), "held to the approved kind's range");
        let d = ask(&mut g, seed, "subDuck=on,subDuck.depth=7").sub_duck.expect("the approved duck");
        assert_eq!(d.depth, 7.0, "a pin inside the approved range is as it always was");
        let p = ask(&mut g, seed, "padSweep=wide,padSweep.oct=4.2,padSweep.lo=380").pad_sweep.expect("wide");
        assert_eq!((p.oct, p.lo), (4.2, 380.0));
        let p = ask(&mut g, seed, "padSweep=on,padSweep.oct=4.2").pad_sweep.expect("the approved sweep");
        assert_eq!(p.oct, 3.5);
        // A loop with sustained chords and no kit: fast is never played there.
        let seed = find(&mut g, |g| g.render().choice("padSweep") == Some("on") && g.render().drums.iter().all(|e| e.vel == 0.0));
        assert!(ask(&mut g, seed, "padSweep=fast").pad_sweep.is_none(), "seed {seed}: fast with no kit");
        assert!(ask(&mut g, seed, "padSweep=wide").pad_sweep.is_some());
        // lateFloor: the fade shapes the slow kind only.
        let seed = find(&mut g, |g| g.render().choice("lateFloor").is_some_and(|o| o != "today"));
        let l = ask(&mut g, seed, "lateFloor=slow,lateFloor.fade=12,lateFloor.seconds=60").late.expect("slow");
        assert_eq!(l.fade, 12.0);
        let l = ask(&mut g, seed, "lateFloor=on,lateFloor.fade=12,lateFloor.seconds=60").late.expect("the approved wait");
        assert_eq!(l.fade, 0.0);
        let spb = g.spec().steps_per_bar * 60.0 / g.spec().bpm / 4.0;
        assert!(l.bars * spb <= LATEFLOOR.seconds[1] + 2.0 * spb, "held to the approved kind's 30 s");
    });
}

/// A host that renders nothing and writes down the drums' and the bass's
/// notes: (channel, time, velocity).
struct Notes {
    frame: u64,
    notes: Vec<(u32, f64, f64)>,
}

impl Host for Notes {
    fn frame(&self) -> u64 {
        self.frame
    }
    fn key(&mut self, _: Key) {}
    fn note(&mut self, _: u32, channel: u32, time: f64, _: f64, _: f64, vel: f64, _: u32, _: [f64; EXTRA]) {
        self.notes.push((channel, time, vel));
    }
    fn event(&mut self, _: u32, _: Event) {}
    fn damp(&mut self, _: u32, _: f64) {}
    fn set(&mut self, _: u32, _: usize, _: usize, _: f64, _: f64) {}
    fn route(&mut self, _: u32, _: usize, _: usize, _: Option<Tap>, _: f64) {}
    fn param(&mut self, _: u32, _: u32, _: f64, _: f64, _: f64) {}
    fn bass_lift(&mut self, _: f64) {}
    fn space_range(&mut self, _: usize, _: f64, _: f64) {}
    fn steal(&mut self, _: u32, _: Key, _: f64, _: f64) {}
    fn fade_all(&mut self, _: f64, _: f64) {}
}

/// Every note of `seconds` of the loop of `seed` as `text` asks, and the
/// loop's lateFloor.
fn play(seed: u32, text: &str, seconds: u64) -> (Vec<(u32, f64, f64)>, LateFloor, f64) {
    let mut player = Box::new(Player::new());
    let mut host = Notes { frame: 0, notes: Vec::new() };
    player.attach(&mut host, 48_000.0, true);
    player.load(seed, &Request::parse(text).expect("a request"));
    player.ready(&mut host);
    assert_eq!(player.state(), State::Playing);
    let until = seconds * 48_000;
    while host.frame < until {
        player.decide(&mut host);
        host.frame += QUANTUM as u64;
    }
    let g = player.generator();
    (host.notes, g.settings().late.expect("a lateFloor"), 60.0 / g.spec().bpm / 4.0)
}

#[test]
fn a_slow_opening_rises_to_the_approved_velocities() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let seed = find(&mut g, |g| g.render().choice("lateFloor").is_some_and(|o| o != "today") && g.render().drums.iter().any(|e| e.vel != 0.0));
        drop(g);
        let (slow, late, step) = play(seed, "lateFloor=slow,lateFloor.order=together,lateFloor.seconds=40,lateFloor.fade=8", 70);
        let (on, _, _) = play(seed, "lateFloor=on,lateFloor.order=together,lateFloor.seconds=8", 70);
        let arrives = late.drums * step;
        assert_eq!(late.fade, 8.0);
        let drums = |notes: &[(u32, f64, f64)], from: f64, to: f64| -> Vec<(f64, f64)> {
            notes.iter().filter(|n| n.0 == DRUMS_CHANNEL && n.1 >= from && n.1 < to).map(|n| (n.1, n.2)).collect()
        };
        assert!(slow.iter().any(|n| n.1 < 1.0), "seed {seed}: something plays from the first second");
        assert!(drums(&slow, 0.0, arrives - 0.05).is_empty(), "seed {seed}: no kit before {arrives} s");
        let rising = drums(&slow, arrives - 0.05, arrives + 8.0);
        assert!(!rising.is_empty(), "seed {seed}: the kit arrives");
        // In the fade, each hit is the approved loop's at the same time, at
        // the share of the fade gone by (the jitter moves a hit a few ms).
        let full = drums(&on, arrives - 0.05, arrives + 70.0);
        for &(t, v) in &rising {
            let (_, w) = *full.iter().min_by(|a, b| (a.0 - t).abs().total_cmp(&(b.0 - t).abs())).expect("the approved loop's hit");
            assert!(v < w + 1e-12 && v <= w * ((t - arrives + 0.05) / 8.0).min(1.0) + 1e-9, "seed {seed}: at {t:.3} s, {v} against {w}");
        }
        // Past the fade, the same hits at the same velocities, exactly.
        let after = |notes: &[(u32, f64, f64)]| drums(notes, arrives + 8.5, 70.0).into_iter().map(|x| x.1).collect::<Vec<f64>>();
        assert_ne!(after(&slow), Vec::<f64>::new(), "seed {seed}: the kit plays past the fade");
        assert_eq!(after(&slow), after(&on), "seed {seed}: past the fade, the approved velocities");
    });
}
