//! The sketch build's levers (queue item 83): choices forced, constants
//! moved, the sketch slot. Only with `--features sketch`; the app's build
//! has none of it (`tools/build-core.sh --check` holds the bytes).
#![cfg(feature = "sketch")]
// Steps and notes are whole or exact thirds the levers place, and the loops' sizes are
// a few hundred: the casts are safe and the compares are of values copied, not computed.
#![allow(clippy::float_cmp, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]

use dlcore::generator::Generator;
use dlcore::tuning;

const SEED: u32 = 982_035_031;

/// Run `body` on a thread with room for a generator (megabytes of lists).
fn big(body: fn()) {
    std::thread::Builder::new().stack_size(64 << 20).spawn(body).expect("a thread").join().expect("the test");
}

fn choice(g: &mut Generator, id: &str) -> Option<&'static str> {
    g.load(SEED, &[]);
    g.render().choice(id)
}

#[test]
fn a_forced_choice_is_drawn_and_lifting_it_restores_the_draw() {
    big(|| {
    let mut g = Box::new(Generator::new());
    let drew = choice(&mut g, "lateFloor");
    g.force("lateFloor", Some("on")).expect("a real choice and option");
    assert_eq!(choice(&mut g, "lateFloor"), Some("on"));
    g.force("lateFloor", None).expect("a real choice");
    assert_eq!(choice(&mut g, "lateFloor"), drew, "the force is lifted");
    });
}

#[test]
fn a_force_naming_nothing_real_says_so() {
    big(|| {
    let mut g = Box::new(Generator::new());
    assert_eq!(g.force("noSuchChoice", Some("on")), Err("no such choice"));
    assert_eq!(g.force("lateFloor", Some("noSuchOption")), Err("no such option"));
    });
}

#[test]
fn a_constant_moves_only_when_named_and_finite() {
    let _held = LEVERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    assert!(!tuning::set("no.SUCH", 1.0));
    assert!(!tuning::set("bass.LIFT_DB", f64::NAN));
    assert!((tuning::get("bass.LIFT_DB", 3.0) - 3.0).abs() < f64::EPSILON, "the default until set");
    assert!(tuning::set("bass.LIFT_DB", 5.5));
    assert!((tuning::get("bass.LIFT_DB", 3.0) - 5.5).abs() < f64::EPSILON);
    // Other tests share the process: put it back.
    assert!(tuning::set("bass.LIFT_DB", 3.0));
}

// ---------------------------------------------------------------------
// The levers of queue item 99: forceable in a sketch build, off bit for
// bit today. They are process-wide like the constants, so a test that
// forces one holds this lock.

use dlcore::generator::plan::Cx;
use dlcore::levers;
use std::sync::Mutex;

static LEVERS: Mutex<()> = Mutex::new(());

/// A few loops with drums and chords, by seed.
const SEEDS: [u32; 6] = [982_035_031, 1, 7, 42, 1_234_567, 20_261_108];

/// Make the loop of `seed`, with `lever` at `option`, and hand it to `body`;
/// the lever is put back after.
fn with(lever: &str, option: Option<&str>, seed: u32, body: impl FnOnce(&Generator)) {
    let mut g = Box::new(Generator::new());
    g.force(lever, option).expect("a real lever and option");
    g.load(seed, &[]);
    body(&g);
    g.force(lever, None).expect("a real lever");
}

/// The drums of a loop as (step, micro, instrument, velocity), in order.
fn drums(g: &Generator) -> Vec<(f64, f64, String, f64)> {
    g.render().drums.iter().map(|e| (e.step, e.micro.unwrap_or(0.0), e.inst.get().to_string(), e.vel)).collect()
}

/// The harmony's slots as (root, notes).
fn slots(g: &Generator) -> Vec<(f64, Vec<i16>)> {
    g.render().harmony.slots.iter().map(|s| (s.root_midi, s.notes.iter().copied().collect())).collect()
}

#[test]
fn the_levers_are_choices_to_force_and_off_they_change_nothing() {
    big(|| {
        let _held = LEVERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut g = Box::new(Generator::new());
        for lever in &levers::LEVERS {
            assert_eq!(g.force(lever.id, Some("noSuchOption")), Err("no such option"));
            g.load(SEEDS[0], &[]);
            assert_eq!(levers::named(lever.id), Some("today"), "{} starts off", lever.id);
        }
        for seed in SEEDS {
            let plain = Box::new({
                let mut g = Generator::new();
                g.load(seed, &[]);
                (drums(&g), slots(&g))
            });
            // Forced on and lifted again: the loop is the plain one.
            for (lever, option) in [("tripletKit", "on"), ("dottedPulse", "pulse"), ("dottedPulse", "kick332"), ("planingSevenths", "on")] {
                g.force(lever, Some(option)).expect("a lever");
                g.load(seed, &[]);
                assert_eq!(levers::named(lever), Some(option));
                g.force(lever, None).expect("a lever");
                g.load(seed, &[]);
                assert_eq!((drums(&g), slots(&g)), *plain, "{lever} lifted, seed {seed}");
            }
        }
    });
}

#[test]
fn the_triplet_kit_has_no_straight_eighth_and_the_kick_on_the_beat() {
    big(|| {
        let _held = LEVERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut moved = 0;
        for seed in SEEDS {
            let plain = {
                let mut g = Box::new(Generator::new());
                g.load(seed, &[]);
                drums(&g)
            };
            with("tripletKit", Some("on"), seed, |g| {
                let six_eight = Cx::new(g.spec()).metre == dlcore::generator::spec::Metre::SixEight;
                let beat = if six_eight { 6.0 } else { 4.0 };
                let on = drums(g);
                for (step, micro, inst, vel) in on.iter().filter(|e| e.3 != 0.0) {
                    let roll = g.render().drums.iter().any(|e| e.roll && e.step == *step && e.vel == *vel);
                    let at = (step % beat + micro) * 3.0 / beat;
                    if inst == "kick" || inst == "softkick" {
                        if !roll {
                            assert!((step % beat).abs() < 1e-9, "a kick off the beat at {step}, seed {seed}");
                        }
                    } else if !roll {
                        assert!((at - at.round()).abs() < 1e-6, "{inst} at {step}+{micro} is not on a third, seed {seed}");
                    }
                }
                let audible = |d: &[(f64, f64, String, f64)]| d.iter().filter(|e| e.3 != 0.0).count();
                assert!(audible(&on) <= audible(&plain), "re-timing adds no hit, seed {seed}");
                if on != plain {
                    moved += 1;
                }
            });
        }
        assert!(moved > 0, "some loop has a kit to re-time");
    });
}

#[test]
fn a_dotted_pulse_is_a_soft_rim_every_three_eighths_or_a_3_3_2_kick() {
    big(|| {
        let _held = LEVERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let (mut pulsed, mut kicked) = (0, 0);
        for seed in SEEDS {
            let plain = {
                let mut g = Box::new(Generator::new());
                g.load(seed, &[]);
                (drums(&g), g.spec().bars * g.spec().steps_per_bar)
            };
            with("dottedPulse", Some("pulse"), seed, |g| {
                let on = drums(g);
                assert_eq!(on[..plain.0.len()], plain.0[..], "the pulse leaves the kit as it was, seed {seed}");
                let added = &on[plain.0.len()..];
                // The kit's layer may be shorter than the loop (its own cycle).
                for (i, (step, micro, inst, vel)) in added.iter().enumerate() {
                    assert_eq!((inst.as_str(), *micro), ("rim", 0.0));
                    assert!((*step - 6.0 * i as f64).abs() < 1e-9 && (*vel - 0.2).abs() < 1e-9, "{step} {vel}");
                }
                if !added.is_empty() {
                    pulsed += 1;
                }
            });
            with("dottedPulse", Some("kick332"), seed, |g| {
                let spb = g.spec().steps_per_bar;
                let on = drums(g);
                assert_eq!(on.len(), plain.0.len(), "no hit added or taken, seed {seed}");
                let sixteen = spb / 16.0;
                for (step, _, inst, _) in on.iter().filter(|e| e.2 == "kick" || e.2 == "softkick") {
                    let at = step % spb;
                    let ok = [0.0, 6.0, 12.0].iter().any(|n| (at - n * sixteen).abs() < 1e-9);
                    // A roll's kick keeps where it was drawn.
                    let roll = g.render().drums.iter().any(|e| e.roll && e.step == *step && e.inst == inst.as_str());
                    assert!(ok || roll, "a kick at {at} of {spb}, seed {seed}");
                }
                if on != plain.0 {
                    kicked += 1;
                }
            });
        }
        assert!(pulsed > 0 && kicked > 0, "some loop has a kit to put a pulse on: {pulsed} {kicked}");
    });
}

#[test]
fn planing_sevenths_are_major_sevenths_a_whole_step_or_a_minor_third_apart() {
    big(|| {
        let _held = LEVERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for seed in SEEDS {
            with("planingSevenths", Some("on"), seed, |g| {
                let s = slots(g);
                assert!(s.len() >= 2, "a progression, seed {seed}");
                for (root, notes) in &s {
                    let pcs: Vec<i16> = {
                        let mut v: Vec<i16> = notes.iter().map(|n| (n - *root as i16).rem_euclid(12)).collect();
                        v.sort_unstable();
                        v
                    };
                    assert_eq!(pcs, vec![0, 4, 7, 11], "a major seventh on {root}: {notes:?}, seed {seed}");
                }
                for pair in s.windows(2) {
                    let by = (pair[1].0 - pair[0].0).abs();
                    assert!(by == 2.0 || by == 3.0, "roots {} then {} (seed {seed})", pair[0].0, pair[1].0);
                }
                // The chords layer plays them.
                assert!(g.render().chords.iter().any(|e| e.vel != 0.0));
            });
        }
    });
}

// ---------------------------------------------------------------------
// The constants of queue item 99: slowOpening's wait and fade, and the
// loop's tuning.

use dlcore::Core;
use dlcore::player::{Player, Request};

/// The seconds lateFloor makes the kit and the bass wait in the loop of
/// `seed` when it is forced on, with the constants as they stand.
fn wait_of(seed: u32) -> f64 {
    let mut g = Box::new(Generator::new());
    g.force("lateFloor", Some("on")).expect("a real choice");
    g.load(seed, &[]);
    let late = g.settings().late.expect("lateFloor forced on");
    late.bars * g.spec().steps_per_bar * 60.0 / g.spec().bpm / 4.0
}

#[test]
fn a_slow_opening_can_wait_past_thirty_seconds_and_the_same_draw_stays_in_order() {
    big(|| {
        let _held = LEVERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let seeds = [SEEDS.as_slice(), &[3, 5, 11, 13, 17, 19, 23, 29]].concat();
        let today: Vec<f64> = seeds.iter().map(|&s| wait_of(s)).collect();
        // The data's wait is 8 to 30 s, to a whole round of two bars.
        let bar = |seed: u32| {
            let mut g = Box::new(Generator::new());
            g.load(seed, &[]);
            g.spec().steps_per_bar * 60.0 / g.spec().bpm / 4.0
        };
        for (&seed, &w) in seeds.iter().zip(&today) {
            assert!(w <= 30.0 + 2.0 * bar(seed), "{w} s, seed {seed}");
        }
        assert!(tuning::set("latefloor.SECONDS_MAX", 90.0));
        let slow: Vec<f64> = seeds.iter().map(|&s| wait_of(s)).collect();
        assert!(tuning::set("latefloor.SECONDS_MAX", 30.0));
        let back: Vec<f64> = seeds.iter().map(|&s| wait_of(s)).collect();
        assert_eq!(back, today, "the data's own top is the draw as it was");
        for ((&seed, &was), &now) in seeds.iter().zip(&today).zip(&slow) {
            assert!(now >= was - 1e-9, "the same draw reaches further, seed {seed}: {was} then {now}");
            assert!(now <= 90.0 + 2.0 * bar(seed), "{now} s, seed {seed}");
        }
        assert!(slow.iter().any(|&w| w > 45.0), "some loop waits past 45 s: {slow:?}");
    });
}

/// `seconds` of the loop of `seed` through the player, lateFloor forced on,
/// mono at 22,050 Hz, and the frame the kit arrives at.
fn opening(seed: u32, seconds: f64) -> (Vec<f32>, usize) {
    const RATE: f64 = 22_050.0;
    let mut core = Box::new(Core::new());
    let mut player = Box::new(Player::new());
    player.start(&mut core, RATE, 1, true).expect("a player");
    player.force_choice("lateFloor", Some("on")).expect("a real choice");
    player.load(seed, &Request::new());
    player.ready(&mut *core);
    let g = player.generator();
    let late = g.settings().late.expect("lateFloor forced on");
    let arrives = (late.drums.min(late.bass) * 60.0 / g.spec().bpm / 4.0 * RATE) as usize;
    let mut out = vec![0.0f32; (seconds * RATE) as usize];
    for chunk in out.chunks_mut(512) {
        player.render(&mut core, &mut [chunk]).expect("a render");
    }
    (out, arrives)
}

fn rms(x: &[f32]) -> f64 {
    (x.iter().map(|v| f64::from(*v).powi(2)).sum::<f64>() / x.len().max(1) as f64).sqrt()
}

#[test]
fn the_floor_fades_up_over_the_fade_and_nothing_before_it_moves() {
    const RATE: usize = 22_050;
    let _held = LEVERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    // A wait of exactly the data's low end (8 s, to two bars), so the render
    // is short.
    assert!(tuning::set("latefloor.SECONDS_MAX", 8.0));
    let seed = SEEDS[0];
    let (plain, arrives) = std::thread::Builder::new().stack_size(256 << 20).spawn(move || opening(seed, 24.0)).expect("a thread").join().expect("a render");
    assert!(tuning::set("latefloor.FADE", 6.0));
    let (faded, same) = std::thread::Builder::new().stack_size(256 << 20).spawn(move || opening(seed, 24.0)).expect("a thread").join().expect("a render");
    assert!(tuning::set("latefloor.FADE", 0.0));
    assert!(tuning::set("latefloor.SECONDS_MAX", 30.0));
    assert_eq!(arrives, same);
    assert!(arrives > 4 * RATE && arrives < 12 * RATE, "arrives at frame {arrives}");
    // The core plays the first notes a little ahead of the step, so the
    // floor's frame is the step's, give or take the render-ahead.
    let before = arrives.saturating_sub(RATE / 2);
    assert!(plain[..before] == faded[..before], "nothing before the floor moves");
    let (a, b) = (arrives + RATE / 2, arrives + 5 * RATE / 2);
    assert!(rms(&faded[a..b]) < 0.8 * rms(&plain[a..b]), "the fade is quieter at first: {} against {}", rms(&faded[a..b]), rms(&plain[a..b]));
    // Past the fade, the floor is at its level again.
    let (c, d) = (arrives + 8 * RATE, arrives + 10 * RATE);
    assert!(d <= plain.len());
    assert!((rms(&faded[c..d]) / rms(&plain[c..d]) - 1.0).abs() < 0.15, "{} against {}", rms(&faded[c..d]), rms(&plain[c..d]));
}

#[test]
fn the_tuning_offset_moves_a_note_by_its_cents_and_zero_is_a_440() {
    use dlcore::blocks::midi_to_freq;
    let _held = LEVERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    assert!((midi_to_freq(69.0) - 440.0).abs() < 1e-12);
    assert!(tuning::set("master.CENTS", 100.0));
    assert!((midi_to_freq(69.0) - 466.163_761_518_089_9).abs() < 1e-6);
    assert!(tuning::set("master.CENTS", -40.0));
    let flat = midi_to_freq(60.0);
    assert!(tuning::set("master.CENTS", 0.0));
    let at = midi_to_freq(60.0);
    assert!((flat / at - 2f64.powf(-40.0 / 1200.0)).abs() < 1e-9, "{}", flat / at);
}

// ---------------------------------------------------------------------
// A retune at every chord (queue item 99, `chords.RETUNE`).

use dlcore::block::{Event, event};
use dlcore::player::host::Host;
use dlcore::{CHORDS, EXTRA, Key, Tap};

/// The core, with the retunes the chords channel is sent written down.
struct Retunes<'a> {
    core: &'a mut Core,
    chords: &'a mut Vec<(f64, Vec<f64>)>,
}

impl Host for Retunes<'_> {
    fn frame(&self) -> u64 {
        self.core.next_frame()
    }
    fn key(&mut self, key: Key) {
        self.core.key(key);
    }
    fn note(&mut self, voice: u32, channel: u32, time: f64, midi: f64, dur: f64, vel: f64, parts: u32, extra: [f64; EXTRA]) {
        self.core.note(voice, channel, time, midi, dur, vel, parts, extra);
    }
    fn event(&mut self, channel: u32, e: Event) {
        if channel == CHORDS && e.kind == event::RETUNE {
            self.chords.push((e.time, e.chord().to_vec()));
        }
        self.core.event(channel, e);
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
    fn steal(&mut self, channel: u32, key: Key, from: f64, to: f64) {
        self.core.steal(channel, key, from, to);
    }
    fn fade_all(&mut self, from: f64, to: f64) {
        self.core.fade_all(from, to);
    }
    fn space_range(&mut self, which: usize, lo: f64, hi: f64) {
        self.core.space_range(which, lo, hi);
    }
}

/// The retunes of the chords channel in the first `seconds` of the loop of
/// `seed`, and whether the loop has the organ, and its slots' chords.
fn retunes(seed: u32, seconds: f64) -> (Vec<(f64, Vec<f64>)>, bool, usize) {
    let mut core = Box::new(Core::new());
    let mut player = Box::new(Player::new());
    player.start(&mut core, 22_050.0, 1, true).expect("a player");
    player.load(seed, &Request::new());
    player.ready(&mut *core);
    let organ = player.generator().settings().organ.is_some();
    let slots = player.generator().render().harmony.slots.len();
    let mut chords = Vec::new();
    let mut out = vec![0.0f32; (seconds * 22_050.0) as usize];
    core.render_with(&mut [&mut out], |core| {
        player.decide(&mut Retunes { core, chords: &mut chords });
    })
    .expect("a render");
    (chords, organ, slots)
}

#[test]
fn the_chords_hear_every_chord_with_the_constant_on_and_no_organ_and_nothing_with_it_off() {
    let _held = LEVERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let ran = std::thread::Builder::new()
        .stack_size(256 << 20)
        .spawn(|| {
            let mut checked = 0;
            for seed in &SEEDS[..4] {
                let seed = *seed;
                let (today, organ, _) = retunes(seed, 8.0);
                assert!(tuning::set("chords.RETUNE", 1.0));
                let (every, same, slots) = retunes(seed, 8.0);
                assert!(tuning::set("chords.RETUNE", 0.0));
                assert_eq!(organ, same);
                if organ {
                    assert_eq!(every, today, "the organ keeps its own retunes, seed {seed}");
                    continue;
                }
                assert!(today.is_empty(), "no retune without the organ today, seed {seed}");
                assert!(!every.is_empty() && slots > 0, "a retune a chord, seed {seed}");
                for (_, chord) in &every {
                    assert!(!chord.is_empty() && chord.len() <= event::NOTES);
                    assert!(chord.windows(2).all(|w| w[0] < w[1]), "low to high, each once: {chord:?}");
                }
                checked += 1;
            }
            checked
        })
        .expect("a thread")
        .join()
        .expect("the test");
    assert!(ran > 0, "some loop has no organ");
}

// ---------------------------------------------------------------------
// dropSpotlight's route (`dlrender --route sketch=drums`): a layer's signal
// into the sketch block's mod, as the kick's is into the bass's duck.

#[test]
fn a_layer_routes_into_the_sketch_exactly_when_the_sketch_declares_a_mod() {
    std::thread::Builder::new().stack_size(256 << 20).spawn(routes).expect("a thread").join().expect("the test");
}

fn routes() {
    use dlcore::block::Block;
    use dlcore::blocks::{own, sketch::Sketch};
    use dlcore::{BASS_CHANNEL, DRUMS_CHANNEL, MELODY};
    let mut core = Box::new(Core::new());
    let drums = Some(Tap { channel: DRUMS_CHANNEL, block: None });
    // The shape the lever calls, into a block that has a mod.
    assert!(core.route(BASS_CHANNEL, own::SUB_DUCK, 0, drums, 0.0));
    let declares = !<Sketch as Block>::MODS.is_empty();
    for channel in [MELODY, CHORDS, BASS_CHANNEL, DRUMS_CHANNEL] {
        assert_eq!(core.route(channel, own::SKETCH, 0, drums, 0.0), declares, "channel {channel}");
    }
    assert!(!core.route(MELODY, own::SKETCH, <Sketch as Block>::MODS.len(), drums, 0.0), "no mod past the last");
}
