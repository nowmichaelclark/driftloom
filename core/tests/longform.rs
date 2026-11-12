//! A long piece's plan (queue item 102a; `generator::longform`,
//! `docs/LONGFORM.md`): the map's promises, over a few hundred seeds at 64,
//! 256, 1,024 and 32,768 bars and Endless, for both journeys. An arc's
//! themes return (A after B, changed, in the home key) and its energy rises
//! to one peak and falls; a finite piece's last section is a close on the
//! tonic; a wander moves at most one thing a unit and holds between its
//! moves; no key leaves the related set; a section is the same reached
//! straight or through, and costs the same at section 1 and at 10^9; the
//! plan is the same wherever the wander's slider stands; an arc lasts about
//! as long in minutes at 50 bpm as at 150; and what was asked is never
//! moved. `tools/longform.mjs` runs the same checks in Node, on the tools'
//! build, and prints the table.
//!
//! What a wander's plan sounds like is `tests/heard.rs`'s, and
//! `tools/longform.mjs` checks what is played.

// Units and sections, far below 2^53.
#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
// The plan's numbers are compared exactly on purpose: the same draw gives
// the same double.
#![allow(clippy::float_cmp)]

use std::sync::OnceLock;
use std::time::Instant;

use dlcore::block::Event;
use dlcore::generator::longform::{Form, Grain, Kind, Relation, Role, Span, State, Theme, Variation, changed};
use dlcore::generator::piece::Piece;
use dlcore::generator::request::{Journey, Request};
use dlcore::generator::rng::Rng;
use dlcore::generator::spec::new_spec;
use dlcore::generator::{Generator, Layer};
use dlcore::player::host::Host;
use dlcore::player::{Player, State as Playing};
use dlcore::{EXTRA, Key, Tap};

/// How many seeds the corpus checks walk, from the corpus seed.
const SEEDS: usize = 240;
const LENGTHS: [&str; 5] = ["bars=64", "bars=256", "bars=1024", "bars=32768", "bars=endless"];
const JOURNEYS: [Journey; 2] = [Journey::Arc, Journey::Wander];

/// The corpus: each seed with the layers its loop makes (one load a seed:
/// the loop is the same at every long length and for either journey).
fn corpus() -> &'static [(u32, u8)] {
    static CORPUS: OnceLock<Vec<(u32, u8)>> = OnceLock::new();
    CORPUS.get_or_init(|| {
        roomy(|| {
            let mut r = Rng::new(20_261_009);
            let mut generator = Box::new(Generator::new());
            (0..SEEDS)
                .map(|_| {
                    let seed = (r.f() * 4_294_967_296.0) as u32;
                    generator.load_with(seed, &Request::parse("bars=64").expect("a request"));
                    (seed, generator.layers_made())
                })
                .collect()
        })
    })
}

/// A load needs more stack than a test's thread has (`tests/length.rs`).
fn roomy<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    let stack = (std::mem::size_of::<Player>() + std::mem::size_of::<Generator>()) * 4 + (16 << 20);
    std::thread::Builder::new().stack_size(stack).spawn(f).expect("a thread").join().expect("the test")
}

/// The plan of `seed` asked `text` and the journey, with its sections a
/// round (none for Endless).
fn form(seed: u32, made: u8, text: &str, journey: Journey) -> (Form, Option<u64>) {
    let request = Request::parse(text).expect("a request").with_journey(journey);
    let spec = new_spec(seed, &request);
    let form = Form::of(&spec, request.edits(), made).expect("a long piece has a plan");
    assert_eq!(form.journey, journey, "an asked journey is the piece's");
    (form, Piece::of(&spec).and_then(|p| p.end).map(|e| e.0))
}

/// The units of sections `from..to`, in order.
fn units(form: &Form, from: u64, to: u64) -> Vec<Span> {
    (from..to).flat_map(|n| (0..form.units_in(n)).map(move |p| form.at(n, p))).collect()
}

/// Up to `most` sections' units from a round's start (a whole short piece).
fn opening(form: &Form, sections: Option<u64>, most: u64) -> Vec<Span> {
    units(form, 0, sections.map_or(most, |n| n.min(most)))
}

fn each(mut check: impl FnMut(u32, &str, Journey, &Form, Option<u64>)) {
    for &(seed, made) in corpus() {
        for text in LENGTHS {
            for journey in JOURNEYS {
                let (form, sections) = form(seed, made, text, journey);
                check(seed, text, journey, &form, sections);
            }
        }
    }
}

#[test]
fn an_arc_returns_its_theme_and_rises_to_one_peak() {
    let (mut arcs, mut two, mut varied, mut recalls) = (0, 0, [0usize; 8], 0);
    each(|seed, text, journey, form, sections| {
        if journey != Journey::Arc {
            return;
        }
        // Whole arcs from the piece's start: every arc of a short piece,
        // the first few of a long one.
        let spans = opening(form, sections, 6000);
        let mut at = 0;
        while at < spans.len() {
            let arc = spans[at].part;
            let end = (at..spans.len()).find(|&x| spans[x].part != arc).unwrap_or(spans.len());
            let whole = end - at == spans[at].of as usize;
            let run = &spans[at..end];
            at = end;
            if !whole {
                continue;
            }
            let what = format!("seed {seed} {text} arc {arc}");
            // An interlude (session 14) is no arc: theme A at home, level.
            let kind = run[0].kind;
            assert!(run.iter().all(|s| s.kind == kind), "{what}: a movement of two kinds");
            if kind == Kind::Interlude {
                assert!(run.iter().all(|s| s.theme == Theme::A && s.key == form.home && s.journey == Journey::Wander && !s.closes), "{what}: an interlude");
                continue;
            }
            arcs += 1;
            // The energy: up to one peak, and down from it.
            let top = run.iter().map(|s| s.energy).fold(0.0, f64::max);
            let first = run.iter().position(|s| s.energy == top).expect("a peak");
            let last = run.iter().rposition(|s| s.energy == top).expect("a peak");
            assert!(run[..=first].windows(2).all(|w| w[0].energy <= w[1].energy), "{what}: the energy falls before its peak");
            assert!(run[last..].windows(2).all(|w| w[0].energy >= w[1].energy), "{what}: the energy rises after its peak");
            assert!(run[first..=last].iter().all(|s| matches!(s.role, Role::Peak | Role::Lift)), "{what}: the peak is not the peak's");
            assert!(run[0].energy < top && run[run.len() - 1].energy < top && (0.0..=1.0).contains(&top), "{what}: no rise and fall");
            assert!(first as f64 / run.len() as f64 > 0.3 && (last as f64) / (run.len() as f64) < 0.85, "{what}: the peak is not near the middle to two thirds");
            // The themes: A stated, then (where the arc has one) B, then A
            // again, changed, in the home key.
            let stated = run.iter().position(|s| s.role == Role::Theme).expect("theme A is stated");
            assert!(run[stated].theme == Theme::A && run[stated].variation == Variation::Plain && run[stated].key == form.home, "{what}: theme A is not stated plain at home");
            let back = run.iter().position(|s| s.role == Role::Return).expect("theme A returns");
            assert!(back > stated, "{what}: the return is before the theme");
            assert!(run[back].theme == Theme::A && run[back].variation != Variation::Plain && run[back].variation != Variation::Key, "{what}: the return is a copy");
            assert_eq!(run[back].key, form.home, "{what}: the return is not in the home key");
            varied[run[back].variation as usize] += 1;
            if form.second && !form.small && kind != Kind::Light {
                two += 1;
                let b = run.iter().position(|s| s.theme == Theme::B && s.role == Role::Second).expect("theme B is stated");
                assert!(stated < b && b < back, "{what}: B is not between A and its return");
                assert!(run[b].theme_seed.is_some() && run[b].key != form.home || form.related().next().is_none(), "{what}: B is not in a related key");
            }
            // The roles come in the form's order, each at least a unit.
            let order: Vec<Role> = run.iter().map(|s| s.role).collect::<Vec<_>>().chunk_by(|a, b| a == b).map(|c| c[0]).collect();
            let full = [Role::Opening, Role::Theme, Role::Build, Role::Second, Role::Development, Role::Peak, Role::Release, Role::Return];
            let small = [Role::Opening, Role::Theme, Role::Lift, Role::Release, Role::Return];
            let want = if form.small || kind == Kind::Light { &small[..] } else { &full[..] };
            // A finale looks back: every earlier second theme it recalls is
            // that movement's own, found by its number.
            for s in run.iter() {
                if let Theme::Recall(k) = s.theme {
                    assert!(k < arc && sections.is_some(), "{what}: a recall of movement {k}");
                    let theirs = spans.iter().find(|x| x.part == k && x.theme == Theme::B).expect("the movement recalled has a second theme");
                    assert_eq!(s.theme_seed, theirs.theme_seed, "{what}: the recall of movement {k} is not its tune");
                    recalls += 1;
                }
            }
            assert_eq!(&order[..order.len() - 1], want, "{what}: the roles");
            assert!(matches!(order[order.len() - 1], Role::Close | Role::Breath), "{what}: no close or breath");
        }
    });
    assert!(arcs > 2000 && two > 500 && recalls > 100, "{arcs} arcs, {two} with a second theme, {recalls} recalls");
    // Every return's variation is drawn somewhere.
    for v in [Variation::Register, Variation::Voice, Variation::Stretched, Variation::Reharmonized, Variation::Answered] {
        assert!(varied[v as usize] > 0, "no return is {}", v.name());
    }
}

#[test]
fn a_finite_piece_closes_on_the_tonic_and_endless_never_does() {
    each(|seed, text, _, form, sections| {
        let what = format!("seed {seed} {text} {}", form.journey.name());
        match sections {
            Some(n) => {
                let last = n - 1;
                let end = form.at(last, form.units_in(last) - 1);
                assert!(end.role == Role::Close && end.closes, "{what}: the last unit is {}", end.role.name());
                assert!(end.key == form.home && end.theme == Theme::A, "{what}: the close is not theme A on the tonic");
                // The round after begins at the opening again.
                let again = form.at(n, 0);
                let first = form.at(0, 0);
                assert_eq!(Span { changed: 0, ..again }, Span { changed: 0, ..first }, "{what}: the second round's first");
                // Nothing before the close closes, and the close is whole.
                let spans = units(form, n.saturating_sub(400), n);
                let from = spans.iter().position(|s| s.closes).expect("a close");
                assert!(spans[from..].iter().all(|s| s.closes && s.role == Role::Close), "{what}: the close is broken");
                let seconds: f64 = spans[from..].iter().map(|s| s.bars).sum::<f64>() / form.unit_bars * form.unit_seconds;
                assert!(seconds >= 9.99, "{what}: a close of {seconds:.1} s");
            }
            None => {
                for from in [0, 5000, 1_000_000_000, 1 << 50] {
                    assert!(units(form, from, from + 300).iter().all(|s| !s.closes && s.role != Role::Close), "{what}: Endless closes near {from}");
                }
            }
        }
    });
}

/// The wander's things, as bits of `changed`.
fn things(s: &Span) -> u16 {
    s.changed & changed::THINGS
}

#[test]
fn a_wander_moves_one_thing_at_a_time_and_holds() {
    let (mut moves, mut seconds, mut holds, mut rests) = (0usize, 0.0, 0usize, 0usize);
    let mut by_thing = [0usize; 11];
    each(|seed, text, journey, form, sections| {
        if journey != Journey::Wander {
            return;
        }
        let what = format!("seed {seed} {text}");
        for from in [0, 40_000, 1_000_000_000] {
            if from > 0 && sections.is_some_and(|n| from + 2500 > n) {
                continue;
            }
            let spans = units(form, from, sections.map_or(from + 2500, |n| n.min(from + 2500)));
            let mut last: Option<u16> = None;
            for (x, s) in spans.iter().enumerate() {
                // (A piece's first unit, or a round's, is a start.)
                if from == 0 && x == 0 {
                    assert_eq!(s.changed, 0, "{what}: the first unit changed");
                    continue;
                }
                assert!(things(s).count_ones() <= 1, "{what}: {} things move at once", things(s).count_ones());
                assert!(s.state.rest.is_none() || s.state.rest != s.state.voice.map(|v| v.0), "{what}: a layer rests while its voice is changed");
                if s.closes {
                    continue;
                }
                // A hold is held: a thing moves only at its first unit.
                if s.at > 0 {
                    assert_eq!(things(s), 0, "{what}: a thing moves inside a hold");
                    assert_eq!(s.role, Role::Holding, "{what}: a hold turns");
                } else if x > 0 && !spans[x - 1].closes {
                    assert!(things(s) != 0 && s.role == Role::Turning, "{what}: a hold ends and nothing moves");
                    // Never the thing that moved last, but a layer back
                    // from its rest.
                    if form.turns() >= 4 {
                        assert!(last != Some(things(s)) || things(s) == changed::LAYERS, "{what}: the same thing twice running");
                    }
                    last = Some(things(s));
                    moves += 1;
                    by_thing[things(s).trailing_zeros() as usize] += 1;
                    // A layer's rest is a hold or two.
                    if let Some(l) = s.state.rest.filter(|_| things(s) == changed::LAYERS) {
                        let rested = spans[x..].iter().take_while(|u| u.state.rest == Some(l) && !u.closes).count();
                        if x + rested < spans.len() && !spans[x + rested].closes {
                            let two = s.of + spans[x + s.of as usize].of;
                            assert!(rested as u64 == s.of || rested as u64 == two, "{what}: a layer rests {rested} units, not a hold ({}) or two ({two})", s.of);
                            rests += 1;
                        }
                    }
                    if x + s.of as usize <= spans.len() {
                        holds += 1;
                        seconds += s.of as f64 * form.unit_seconds;
                        let run = &spans[x..x + s.of as usize];
                        assert!(run.iter().enumerate().all(|(k, u)| u.at == k as u64 && u.of == s.of && u.part == s.part), "{what}: a hold of the wrong length");
                    }
                }
            }
        }
    });
    // Holds as long as drawn: between one and five minutes on average, and
    // every thing takes its turns.
    let mean = seconds / holds as f64;
    assert!(moves > 20_000 && rests > 1000 && (90.0..=270.0).contains(&mean), "{moves} moves, {rests} rests, holds of {mean:.0} s");
    // (The theme's turn waits for theme B's tune: `data/longform.txt`.)
    let off = if dlcore::generator::data::LONGFORM.theme > 0.0 { 0 } else { changed::THEME };
    for (bit, name) in changed::NAMES.iter().enumerate().filter(|(bit, _)| (changed::THINGS & !off) & (1 << bit) != 0) {
        assert!(by_thing[bit] > 0, "nothing ever moves the {name}");
    }
}

#[test]
fn no_key_leaves_the_related_set() {
    each(|seed, text, _, form, sections| {
        let related: Vec<_> = form.related().collect();
        assert!(related.iter().all(|k| k.relation != Relation::Home && *k != form.home));
        for from in [0, 1_000_000_000] {
            if from > 0 && sections.is_some() {
                continue;
            }
            for s in units(form, from, sections.map_or(from + 1500, |n| n.min(1500))) {
                assert!(s.key == form.home || related.contains(&s.key), "seed {seed} {text}: a key outside the related set");
                assert_eq!(s.key.relation == Relation::Home, s.key == form.home);
                // The tempo is the loop's own, and the energy is one.
                assert!((0.0..=1.0).contains(&s.energy) && s.bpm == form.at(0, 0).bpm);
            }
        }
    });
}

#[test]
fn a_section_is_the_same_reached_straight_or_through() {
    let (seed, made) = corpus()[3];
    for text in LENGTHS {
        for journey in JOURNEYS {
            let (walked, sections) = form(seed, made, text, journey);
            let last = sections.map_or(3000, |n| n.min(3000));
            let through = units(&walked, 0, last);
            // Straight: a plan made afresh for each unit asked, backwards.
            let mut x = through.len();
            for n in (0..last).rev().step_by(7) {
                let (fresh, _) = form(seed, made, text, journey);
                assert_eq!(fresh, walked, "the plan made again is the plan");
                x -= (n + 1..(n + 7).min(last)).map(|m| walked.units_in(m) as usize).sum::<usize>();
                x -= walked.units_in(n) as usize;
                assert_eq!(fresh.at(n, 0), through[x], "seed {seed} {text} {}: section {n}", journey.name());
            }
        }
    }
}

/// A host that keeps the core's clock and renders nothing (as
/// `tests/length.rs`'s).
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
    fn event(&mut self, _: u32, _: Event) {}
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
fn the_plan_never_reads_the_wander_slider() {
    // The player, with the slider at 0, 1 and 2.4, played through twenty
    // sections: the plan its generator gives is the same plan, unit for
    // unit.
    let run = || {
        let mut maps: Vec<Vec<Span>> = Vec::new();
        for amount in [0.0, 1.0, 2.4] {
            let mut clock = Clock::default();
            let mut player = Box::new(Player::new());
            player.attach(&mut clock, 44_100.0, true);
            player.set_wander(amount);
            player.load(77, &Request::parse("bars=256,journey=arc").expect("a request"));
            player.ready(&mut clock);
            assert_eq!(player.state(), Playing::Playing);
            let mut guard = 0u64;
            while player.position().1 < 20 {
                player.decide(&mut clock);
                clock.frame += 4096;
                guard += 1;
                assert!(guard < 5_000_000, "the player stopped deciding");
            }
            let form = player.generator().form().expect("a plan");
            maps.push(units(&form, 0, 64));
        }
        assert!(maps[0] == maps[1] && maps[1] == maps[2], "the plan moved with the wander's amount");
    };
    roomy(run);
}

#[test]
fn a_section_costs_the_same_at_one_and_at_a_billion() {
    const UNITS: u64 = 4000;
    let (seed, made) = corpus()[5];
    for journey in JOURNEYS {
        let (form, _) = form(seed, made, "bars=endless", journey);
        // The least of many short runs, the places taken in turn, so a busy
        // machine slows them all alike and can't fail it.
        let places = [1, 100_000, 1_000_000_000, 1 << 52];
        let mut least = [f64::INFINITY; 4];
        for _ in 0..60 {
            for (best, from) in least.iter_mut().zip(places) {
                let t0 = Instant::now();
                let mut sum = 0.0;
                for n in from..from + UNITS {
                    sum += std::hint::black_box(form.at(n, 0)).energy;
                }
                std::hint::black_box(sum);
                *best = best.min(t0.elapsed().as_secs_f64());
            }
        }
        // (A wander's first epoch has no epoch before it to read, so the
        // first sections may cost less than any later one: the cost is flat
        // from the second epoch on, and never more than twice the first's.)
        let [near, on, far, farther] = least;
        let us = |t: f64| t / UNITS as f64 * 1e6;
        println!("a section's plan, {}: {:.2} us at 1, {:.2} at 10^5, {:.2} at 10^9, {:.2} at 2^52", journey.name(), us(near), us(on), us(far), us(farther));
        let flat = |t: f64| t < on * 1.5 + 2e-4 && t < near * 2.5 + 2e-4;
        assert!(flat(far) && flat(farther), "{}: {near:.4} s at 1, {on:.4} s at 10^5, {far:.4} s at 10^9, {farther:.4} s at 2^52", journey.name());
    }
}

#[test]
fn an_arc_lasts_as_long_at_50_bpm_as_at_150() {
    roomy(arc_minutes);
}

fn arc_minutes() {
    let mut generator = Box::new(Generator::new());
    let mut r = Rng::new(20_261_009);
    let (mut pieces, mut phrase) = (0, 0);
    for _ in 0..60 {
        let seed = (r.f() * 4_294_967_296.0) as u32;
        let minutes = |generator: &mut Generator, text: &str| {
            let request = Request::parse(text).expect("a request");
            generator.load_with(seed, &request);
            let form = generator.form().expect("a plan");
            // Each role's minutes in the first arc.
            let spans = units(&form, 0, form.arc_units());
            let mut roles = [0.0; 13];
            for s in spans.iter().take(form.arc_units() as usize) {
                roles[s.role as usize] += form.unit_seconds / 60.0;
            }
            (form, roles)
        };
        let (slow, slow_roles) = minutes(&mut generator, "bars=endless,journey=arc,bpm=50");
        let (fast, fast_roles) = minutes(&mut generator, "bars=endless,journey=arc,bpm=150");
        pieces += 1;
        phrase += usize::from(slow.grain == Grain::Phrase);
        let (a, b) = (slow.arc_units() as f64 * slow.unit_seconds / 60.0, fast.arc_units() as f64 * fast.unit_seconds / 60.0);
        let unit = slow.unit_seconds.max(fast.unit_seconds) / 60.0;
        assert!((3.5..=16.0).contains(&a) && (3.5..=16.0).contains(&b), "seed {seed}: arcs of {a:.1} and {b:.1} minutes");
        assert!((a - b).abs() <= unit + 0.05, "seed {seed}: an arc of {a:.1} minutes at 50 bpm, {b:.1} at 150");
        for (x, (s, f)) in slow_roles.iter().zip(&fast_roles).enumerate() {
            assert!((s - f).abs() <= 2.0 * unit + 0.15 * s.max(*f), "seed {seed}: role {x} lasts {s:.2} minutes at 50 bpm, {f:.2} at 150");
        }
    }
    assert!(pieces == 60 && phrase > 0, "no slow loop is planned in phrases");
}

#[test]
fn what_was_asked_is_never_moved() {
    let made = 0b1_1111;
    let all = |text: &str, journey: Journey, seed: u32| {
        let request = Request::parse(text).expect("a request").with_journey(journey);
        let spec = new_spec(seed, &request);
        let form = Form::of(&spec, request.edits(), made).expect("a plan");
        let spans = units(&form, 0, 4000);
        (spec, form, spans)
    };
    for seed in 1..=40u32 {
        // A key asked is the home key: the wander never leaves it, the arc
        // comes home.
        for journey in JOURNEYS {
            let (spec, form, spans) = all("bars=endless,key=D", journey, seed);
            assert_eq!((spec.root, form.home.root), (2.0, 2));
            if journey == Journey::Wander {
                assert!(spans.iter().all(|s| s.key == form.home), "seed {seed}: a wander left the key asked");
            } else {
                assert!(spans.iter().filter(|s| matches!(s.role, Role::Opening | Role::Theme | Role::Return | Role::Breath)).all(|s| s.key == form.home));
            }
            // A scale asked rules out the parallel mode.
            let (_, form, spans) = all("bars=endless,scale=aeolian", journey, seed);
            assert!(form.related().all(|k| k.relation != Relation::Parallel) && spans.iter().all(|s| s.key.relation != Relation::Parallel), "seed {seed}: the parallel of a scale asked");
            // A voice asked is never handed on.
            let (_, _, spans) = all("bars=endless,voice.melody=flute,voice.bass=sub", journey, seed);
            assert!(spans.iter().all(|s| s.variation != Variation::Voice), "seed {seed}: the tune's voice asked, handed on");
            assert!(spans.iter().all(|s| !matches!(s.state.voice, Some((Layer::Melody | Layer::Bass, _)))), "seed {seed}: a voice asked, changed");
            // A layer asked on or off, or re-rolled, never rests.
            let (_, _, spans) = all("bars=endless,layer.drums=on,layer.texture=off,edit.roll.bass=5", journey, seed);
            assert!(spans.iter().all(|s| !matches!(s.state.rest, Some(Layer::Drums | Layer::Texture | Layer::Bass))), "seed {seed}: a layer asked, rested");
            // A tone edited is never moved; the tempo asked is the piece's.
            let (spec, _, spans) = all("bars=endless,edit.warmth=0.6,edit.space=0.2,bpm=77", journey, seed);
            assert!(spans.iter().all(|s| s.state.warmth == 0.0 && s.state.space == 0.0 && s.bpm == 77.0 && spec.bpm == 77.0), "seed {seed}: the tone or the tempo moved");
            // A theme asked stays the tune.
            let (_, _, spans) = all("bars=endless,theme=1234", journey, seed);
            assert!(spans.iter().all(|s| matches!(s.theme, Theme::A | Theme::Rest)), "seed {seed}: a theme asked, handed over");
        }
        // An arc moves nothing of the wander's.
        let (_, _, spans) = all("bars=1024", Journey::Arc, seed);
        assert!(spans.iter().filter(|s| s.kind != Kind::Interlude).all(|s| s.state == State::OWN && s.changed & (changed::THINGS & !changed::THEME & !changed::KEY) == 0));
    }
}

#[test]
fn the_journey_is_asked_or_drawn_and_short_loops_have_none() {
    assert_eq!(Request::parse("journey=arc").expect("arc").journey(), Some(Journey::Arc));
    assert_eq!(Request::parse("bars=64,journey=wander").expect("wander").journey(), Some(Journey::Wander));
    assert_eq!(Request::parse("journey=up").err(), Some(dlcore::generator::request::NO_SUCH_JOURNEY));
    assert_eq!(Request::parse("journey").err(), Some(dlcore::generator::request::NO_SUCH_JOURNEY));
    // A loop with no length, or one of up to 32 bars, has no plan, asked a
    // journey or not; and asking one moves nothing of the loop.
    for text in ["-", "bars=32", "bars=8,journey=arc", "journey=wander"] {
        let request = Request::parse(text).expect("a request");
        assert!(Form::of(&new_spec(9, &request), request.edits(), 0b1_1111).is_none(), "{text} has a plan");
    }
    let hashes = roomy(|| {
        let mut generator = Box::new(Generator::new());
        let mut hashes = Vec::new();
        for text in ["bars=256", "bars=256,journey=arc", "bars=256,journey=wander"] {
            generator.load_with(31, &Request::parse(text).expect("a request"));
            let drawn = generator.form().expect("a plan").journey;
            // The loop's own parts; its passes carry a wander's plan heard
            // (session 12), so only an arc's are the loop's wandered passes.
            let own: Vec<u64> = dlcore::generator::Part::ALL.iter().filter(|p| !matches!(p, dlcore::generator::Part::Pass(_))).map(|&p| generator.hash(p)).collect();
            let passes: Vec<u64> = dlcore::generator::Part::ALL.iter().filter(|p| matches!(p, dlcore::generator::Part::Pass(_))).map(|&p| generator.hash(p)).collect();
            hashes.push((own, passes, drawn));
        }
        hashes
    });
    assert!(hashes[0].0 == hashes[1].0 && hashes[1].0 == hashes[2].0, "a journey asked moved the loop");
    let drawn = if hashes[0].2 == Journey::Arc { 1 } else { 2 };
    assert!(hashes[0].1 == hashes[drawn].1, "asking the journey a piece draws moved its sections");
    assert!(hashes[1].1 != hashes[2].1, "a wander's sections are an arc's");
    // Drawn: both journeys, the vibes leaning, Endless toward the wander.
    let share = |text: &str| {
        let request = Request::parse(text).expect("a request");
        let arcs = (0..2000u32).filter(|&seed| Form::of(&new_spec(seed, &request), request.edits(), 0b1_1111).expect("a plan").journey == Journey::Arc).count();
        arcs as f64 / 2000.0
    };
    let (plain, endless, still, lively) = (share("bars=256"), share("bars=endless"), share("bars=256,still,meditative"), share("bars=256,adventurous,lively"));
    assert!((0.44..=0.56).contains(&plain), "arcs {plain}");
    assert!((0.3..=0.45).contains(&endless), "Endless arcs {endless}");
    assert!(still < 0.4 && still > 0.02 && lively > 0.6 && lively < 0.98, "still {still}, lively {lively}");
}
