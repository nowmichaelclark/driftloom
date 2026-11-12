//! The wander heard (queue item 102b; `generator::heard`, `docs/LONGFORM.md`,
//! "The wander"): a wander's sections carry its plan as marks and levels,
//! the player plays them at any slider setting, and a loop's passes carry
//! nothing (an arc's are `tests/arc.rs`'s, session 13). `tools/longform.mjs` checks what is played,
//! note by note, from the player's calls; these check the marks.
//!
//! - every unit of a section is its plan's state: a layer rests where the
//!   plan rests it (faded out over the glide, its notes left out once it is
//!   down), a voice, the chords' octave and length, the tune's density and a
//!   key a fifth away where the plan has them, and the loop's own elsewhere;
//! - a unit starts where the one before it ended, so nothing jumps;
//! - a loop of up to 32 bars, or with no length, hears nothing;
//! - the player draws a wander's sections as passes at 0, 1 and 2.4, and the
//!   marks are the same at each (the slider is only the breathing);
//! - the plan's level comes back under a mute lifted, and a voice is only
//!   ever one of the layer's pool.

// Units and steps, far below 2^53.
#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
// The plan's numbers are compared exactly on purpose: the same draw gives
// the same double.
#![allow(clippy::float_cmp)]

use dlcore::block::Event;
use dlcore::generator::data::LONGFORM;
use dlcore::generator::heard::Heard;
use dlcore::generator::longform::Relation;
use dlcore::generator::request::{Journey, Request};
use dlcore::generator::rng::Rng;
use dlcore::generator::{Generator, LAYERS, Layer};
use dlcore::player::host::Host;
use dlcore::player::synth::{CHANNEL_GAIN, Synth};
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

#[test]
fn every_unit_of_a_wander_is_its_plans_state() {
    roomy(|| {
        let mut generator = Box::new(Generator::new());
        let (mut rests, mut voices, mut registers, mut lengths, mut thin, mut keys) = (0, 0, 0, 0, 0, 0);
        for seed in seeds(60) {
            for text in ["bars=endless,journey=wander", "bars=1024,journey=wander"] {
                generator.load_with(seed, &Request::parse(text).expect("a request"));
                let form = generator.form().expect("a plan");
                let what = format!("seed {seed} {text}");
                let mut last: Option<[[f64; 2]; 5]> = None;
                for n in 0..400u64 {
                    let heard: Heard = generator.section(n, 1.0).heard;
                    assert!(heard.on, "{what}: section {n} hears nothing");
                    assert_eq!(heard.units().len() as u64, form.units_in(n), "{what}: section {n}'s units");
                    for (p, u) in heard.units().iter().enumerate() {
                        let span = form.at(n, p as u64);
                        let closing = span.closes;
                        // A unit starts where the one before ended.
                        if let Some(was) = last {
                            for (l, (c, w)) in u.cut.iter().zip(was.iter()).enumerate() {
                                assert_eq!(c[0], w[1], "{what}: section {n} unit {p}: layer {l}'s level jumps");
                            }
                        }
                        last = Some(u.cut);
                        for layer in LAYERS {
                            let rests_here = span.state.rest == Some(layer);
                            let down = u.cut[layer as usize][1];
                            if rests_here {
                                assert!(down > 0.0, "{what}: section {n}: the plan rests {layer:?} and it plays on");
                                if u.rests(layer) {
                                    rests += 1;
                                }
                            } else if !closing && span.at > 1 + (LONGFORM.glide / form.unit_seconds) as u64 {
                                assert_eq!(down, 0.0, "{what}: section {n}: {layer:?} down with no rest in the plan");
                            }
                        }
                        let other = u.voice.iter().filter(|v| !v.get().is_empty()).count();
                        assert!(other <= 1, "{what}: two voices changed");
                        assert_eq!(other == 1, span.state.voice.is_some(), "{what}: section {n}: a voice not the plan's");
                        voices += other;
                        if span.state.register == 0 {
                            assert_eq!(u.register, 0, "{what}: section {n}: the chords moved with no register in the plan");
                        } else if u.register != 0 {
                            assert_eq!(u.register.abs(), 12, "{what}: the chords moved by {}", u.register);
                            registers += 1;
                        }
                        if span.state.length == 0 {
                            assert_eq!(u.stretch, 0.0, "{what}: section {n}: the chords' lengths moved with none in the plan");
                        } else if u.stretch != 0.0 {
                            assert!(LONGFORM.lengths.contains(&u.stretch), "{what}: a length of {}", u.stretch);
                            lengths += 1;
                        }
                        if span.state.density < 1.0 && !closing {
                            thin += usize::from(u.thin[1] > 0.0);
                        }
                        let shifted = u.shift.iter().any(|&s| s != 0);
                        if span.key.relation == Relation::Home {
                            assert!(!shifted, "{what}: section {n}: a key moved at home");
                        } else {
                            assert!(span.key.relation.is_fifth(), "{what}: a wander visits {:?}", span.key.relation);
                            for (k, &s) in u.shift.iter().enumerate() {
                                assert!(s == 0 || (s - i16::from(span.key.relation.shift())) % 12 == 0, "{what}: layer {k} moved {s}");
                            }
                            keys += usize::from(shifted);
                        }
                    }
                }
            }
        }
        assert!(rests > 0 && voices > 0 && registers > 0 && lengths > 0 && thin > 0 && keys > 0, "{rests} {voices} {registers} {lengths} {thin} {keys}");
    });
}

#[test]
fn a_loop_hears_nothing() {
    roomy(|| {
        let mut generator = Box::new(Generator::new());
        for seed in seeds(30) {
            for text in ["bars=24", "bars=32", "-"] {
                generator.load_with(seed, &Request::parse(text).expect("a request"));
                for n in [0u64, 3, 40] {
                    assert!(!generator.section(n, 1.0).heard.on, "seed {seed} {text}: section {n} hears a plan");
                }
                assert!(!generator.wander(5).heard.on, "seed {seed} {text}: a pass hears a plan");
            }
        }
    });
}

/// A host keeping the time and nothing else.
#[derive(Default)]
struct Clock {
    frame: u64,
    gains: Vec<(u32, f64)>,
}

impl Host for Clock {
    fn frame(&self) -> u64 {
        self.frame
    }
    fn key(&mut self, _: Key) {}
    fn note(&mut self, _: u32, _: u32, _: f64, _: f64, _: f64, _: f64, _: u32, _: [f64; EXTRA]) {}
    fn event(&mut self, _: u32, _: Event) {}
    fn damp(&mut self, _: u32, _: f64) {}
    fn set(&mut self, _: u32, _: usize, _: usize, _: f64, _: f64) {}
    fn route(&mut self, _: u32, _: usize, _: usize, _: Option<Tap>, _: f64) {}
    fn param(&mut self, id: u32, _: u32, value: f64, _: f64, _: f64) {
        if id < 5 {
            self.gains.push((id, value));
        }
    }
    fn steal(&mut self, _: u32, _: Key, _: f64, _: f64) {}
    fn fade_all(&mut self, _: f64, _: f64) {}
    fn bass_lift(&mut self, _: f64) {}
    fn space_range(&mut self, _: usize, _: f64, _: f64) {}
}

#[test]
fn a_wander_is_heard_at_any_slider_setting() {
    roomy(|| {
        // Seed 7's wander: its first moves within twenty minutes.
        let request = Request::parse("bars=endless").expect("a request").with_journey(Journey::Wander);
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
            while player.position().1 < 260 {
                player.decide(&mut clock);
                clock.frame += 4096;
                guard += 1;
                assert!(guard < 50_000_000, "the player stopped deciding");
                let (_, section, _) = player.position();
                if section != at {
                    at = section;
                    let pass = player.pass().expect("a wander's section is a pass at any setting");
                    assert!(pass.heard.on, "section {section} at {amount} hears nothing");
                    assert_eq!(pass.breathes, amount > 0.0, "the slider is the breathing");
                    seen.push(format!("{:?}", pass.heard.units().iter().map(|u| (u.cut, u.tone, u.thin, u.register, u.stretch, u.shift)).collect::<Vec<_>>()));
                }
            }
            // The plan's levels reached the mix: a layer rested.
            assert!(clock.gains.iter().any(|&(_, v)| v == 0.0), "no layer rested in the mix at {amount}");
            marks.push(seen);
        }
        assert!(marks[0] == marks[1] && marks[1] == marks[2], "the marks moved with the wander's amount");
    });
}

#[test]
fn the_plans_level_comes_back_under_a_mute() {
    let mut clock = Clock::default();
    let mut synth = Box::new(Synth::new());
    let c = Layer::Drums as usize;
    let channel = dlcore::DRUMS_CHANNEL as usize;
    synth.set_plan_level(&mut clock, channel, 0.25, 1.0, 0.1);
    assert_eq!(clock.gains.last().copied(), Some((channel as u32, CHANNEL_GAIN[channel] * 0.25)), "the plan's level, layer {c}");
    // Sent once: the same level again is not.
    let sent = clock.gains.len();
    synth.set_plan_level(&mut clock, channel, 0.25, 2.0, 0.1);
    assert_eq!(clock.gains.len(), sent, "the same level sent twice");
    // Muted, then brought back: the plan's level, not the channel's own.
    synth.set_mute(&mut clock, channel, true, 3.0);
    assert_eq!(clock.gains.last().copied(), Some((channel as u32, 0.0)));
    synth.set_plan_level(&mut clock, channel, 0.5, 4.0, 0.1);
    assert_eq!(clock.gains.last().copied(), Some((channel as u32, 0.0)), "the plan lifted a mute");
    synth.set_mute(&mut clock, channel, false, 5.0);
    assert_eq!(clock.gains.last().copied(), Some((channel as u32, CHANNEL_GAIN[channel] * 0.5)), "a mute lifted forgot the plan");
    // A new loop: the channel's own.
    synth.clear_plan();
    synth.set_mute(&mut clock, channel, false, 6.0);
    assert_eq!(clock.gains.last().copied(), Some((channel as u32, CHANNEL_GAIN[channel])));
}
