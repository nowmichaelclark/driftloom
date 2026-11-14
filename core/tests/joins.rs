//! A long piece's joins and its endings (queue item 102e, session 15;
//! `docs/LONGFORM.md`, "The joins"): where two units meet and something
//! changes, the line is written (`longform::Join`), chosen by what changes
//! and the energy either side, and played as `heard` plays it; a section
//! reached straight from its number joins as one reached by playing; and
//! where the length slider stops a piece before its close, the phrases
//! before the stop play the close and land on the tonic
//! (`heard::Heard::stop_at`). `tools/longform.mjs` measures the same on
//! what the player plays: the joins' clicks, the steps, the endings.

// Sections and units are whole numbers far below 2^53; a generator, a
// section, a unit and a span keep their short names; steps are compared
// exactly on purpose.
#![allow(clippy::many_single_char_names, clippy::float_cmp, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]

use dlcore::generator::heard::{Tune, Unit, tune};
use dlcore::generator::melody::MelodyEvent;
use dlcore::generator::wander::Pass;
use dlcore::generator::longform::{Form, Join, Key, Relation, Span, changed, pivot};
use dlcore::generator::piece::Piece;
use dlcore::generator::plan::Cx;
use dlcore::generator::request::{Journey, Request};
use dlcore::generator::rng::Rng;
use dlcore::generator::theory::SCALES;
use dlcore::generator::data::JOINS;
use dlcore::generator::{Generator, Layer};
use dlcore::player::Player;

/// A load needs more stack than a test's thread has (`tests/length.rs`).
fn roomy<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    let stack = (std::mem::size_of::<Player>() + std::mem::size_of::<Generator>()) * 4 + (16 << 20);
    std::thread::Builder::new().stack_size(stack).spawn(f).expect("a thread").join().expect("the test")
}

fn seeds(n: usize) -> Vec<u32> {
    let mut r = Rng::new(20_261_009);
    (0..n).map(|_| (r.f() * 4_294_967_296.0) as u32).collect()
}

fn load(g: &mut Generator, seed: u32, text: &str, journey: Journey) -> Form {
    g.load_with(seed, &Request::parse(text).expect("a request").with_journey(journey));
    g.form().expect("a long piece has a plan")
}

fn scale(id: &str) -> usize {
    SCALES.iter().position(|s| s.id == id).expect("a scale")
}

/// The units of sections `0..count`, each with its section, its place and
/// the unit after it.
fn units(form: &Form, count: u64) -> Vec<(u64, u64, Span, Span)> {
    let mut out = Vec::new();
    for n in 0..count {
        for p in 0..form.units_in(n) {
            let (bn, bp) = form.after(n, p);
            out.push((n, p, form.joined(n, p), form.at(bn, bp)));
        }
    }
    out
}

/// Whether a pass plays a tune note between unit `u`'s join and its line,
/// asked as the sequencer asks (`player/sequencer.rs`): whose tune the step
/// plays (`own`, the loop's, or `second`), where a stretched one's note is
/// written, and each note's velocity as the pass gives it.
fn tune_sounds(own: &[MelodyEvent], second: &[MelodyEvent], cycle: f64, pass: &Pass, u: &Unit) -> bool {
    let mut s = u.join_from;
    while s < u.to {
        let which = pass.heard.tune_at(s);
        let notes = match which {
            tune::OWN => own,
            tune::SECOND => second,
            _ => &[],
        };
        let from = pass.heard.stretched_from(s);
        for (i, e) in notes.iter().enumerate() {
            let here = if u.stretched { from == Some(e.step) } else { e.step == s % cycle };
            let vel = if which == tune::SECOND { pass.second_at(s, i, e).0 } else { pass.melody_home_at(s, i, e).0 };
            if here && vel > 0.0 {
                return true;
            }
        }
        s += 1.0;
    }
    false
}

#[test]
fn a_pivot_is_a_chord_both_keys_share() {
    let c = Key { root: 0, scale: scale("ionian"), relation: Relation::Home };
    // A fifth above: G major is C's fifth and G's tonic.
    let g = Key { root: 7, scale: c.scale, relation: Relation::FifthAbove };
    assert_eq!(pivot(c, g).map(|p| (p.notes, p.count)), Some(([7, 11, 2], 3)));
    // Home again from F: C major is F's fifth and C's tonic.
    let f = Key { root: 5, scale: c.scale, relation: Relation::FifthBelow };
    assert_eq!(pivot(f, c).map(|p| p.notes), Some([0, 4, 7]));
    // The relative shares every note: the new key's own tonic.
    let a = Key { root: 9, scale: scale("aeolian"), relation: Relation::Relative };
    assert_eq!(pivot(c, a).map(|p| p.notes), Some([9, 0, 4]));
    // A minor to A major (the parallel) shares no triad: the root and the
    // fifth, open.
    let am = Key { root: 9, scale: scale("aeolian"), relation: Relation::Home };
    let major = Key { root: 9, scale: scale("ionian"), relation: Relation::Parallel };
    assert_eq!(pivot(am, major).map(|p| (p.notes, p.count)), Some(([9, 4, 0], 2)));
    // Every pivot of every mode's related keys is in both keys.
    for (x, mode) in SCALES.iter().enumerate() {
        if mode.steps.len() < 3 {
            continue;
        }
        for up in [5u8, 7] {
            let home = Key { root: 2, scale: x, relation: Relation::Home };
            let to = Key { root: (2 + up) % 12, scale: x, relation: Relation::FifthAbove };
            if let Some(p) = pivot(home, to) {
                let has = |k: Key, pc: u8| SCALES[k.scale].steps.iter().any(|&s| (u32::from(k.root) + s as u32) % 12 == u32::from(pc));
                assert!(p.notes.iter().take(p.count).all(|&pc| has(home, pc) && has(to, pc)), "{}: {:?}", mode.id, p);
            }
        }
    }
}

#[test]
fn every_line_that_changes_a_theme_or_a_key_is_written() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let mut kinds = [0usize; Join::ALL.len()];
        let mut heard_kinds = [0usize; Join::ALL.len()];
        let mut lines = 0;
        for seed in seeds(40) {
            for (text, journey) in [("bars=64", Journey::Arc), ("bars=256", Journey::Arc), ("bars=1024", Journey::Arc), ("bars=endless", Journey::Arc), ("bars=endless", Journey::Wander), ("bars=1024", Journey::Wander)] {
                let form = load(&mut g, seed, text, journey);
                let count = Piece::of(g.spec()).and_then(|p| p.end).map_or(60, |e| e.0.min(60));
                for (n, p, a, b) in units(&form, count) {
                    let what = format!("seed {seed} {text} {journey:?} section {n}.{p} ({} to {})", a.role.name(), b.role.name());
                    kinds[a.join as usize] += 1;
                    // The plan's join is what `join_of` says of the two.
                    assert_eq!(a.join, if a.closes { Join::None } else { form.join_of(&a, &b) }, "{what}");
                    if a.closes {
                        // A close lands: no join (the round after starts from it).
                        assert_eq!(a.join, Join::None, "{what}");
                        continue;
                    }
                    // Something changed: a join is written, a glide only for
                    // what glides (a level, the tone, the energy).
                    let c = b.changed;
                    if c & (changed::THEME | changed::KEY | changed::VARIATION | changed::VOICE | changed::REGISTER | changed::LENGTH) != 0 {
                        lines += 1;
                        assert!(!matches!(a.join, Join::None | Join::Glide), "{what}: {:?} changed, no join", changed::NAMES.iter().enumerate().filter(|(x, _)| c & (1 << x) != 0).map(|x| x.1).collect::<Vec<_>>());
                    } else if c != 0 {
                        assert!(a.join != Join::None, "{what}: something changed, nothing written");
                    } else {
                        assert!(matches!(a.join, Join::None | Join::Prepared), "{what}: a join where nothing changes");
                    }
                    // A key changes: a pivot, the return prepared, or the
                    // floor out or a breath where the loop has no chords.
                    if c & changed::KEY != 0 {
                        assert!(matches!(a.join, Join::Pivot | Join::Prepared | Join::Floor | Join::Breath), "{what}: {:?}", a.join);
                        assert!(a.join != Join::Pivot || pivot(a.key, b.key).is_some(), "{what}: a pivot the keys don't share");
                    }
                }
                // As heard: each unit's join played, its marks inside the
                // unit, only before its line.
                let spec = g.spec();
                let cycle = spec.cycles.of(Layer::Melody as usize).filter(|&c| c > 0.0).unwrap_or(spec.bars * spec.steps_per_bar);
                let own: Vec<MelodyEvent> = g.render().melody.iter().copied().collect();
                for n in 0..count.min(24) {
                    // The section's second tune, made with it.
                    let _ = g.section(n, 0.0);
                    let second: Vec<MelodyEvent> = g.second(Tune::of(&form, n)).map_or(Vec::new(), |m| m.iter().copied().collect());
                    let pass = g.section(n, 0.0);
                    for (p, u) in pass.heard.units().iter().enumerate() {
                        let s = form.joined(n, p as u64);
                        heard_kinds[u.join as usize] += 1;
                        let what = format!("seed {seed} {text} section {n}.{p}");
                        // What the plan asked, or the next of its kind the
                        // loop could play (a fill with no kit is a glide).
                        // (Or, where it would be a hole, a breath or a glide:
                        // session 16b.)
                        assert!(u.join == s.join || matches!((s.join, u.join), (Join::Fill | Join::Floor | Join::Alone | Join::Breath | Join::Pivot, Join::Glide) | (Join::Alone | Join::Pivot | Join::Floor, Join::Breath) | (Join::Pivot, Join::Floor)), "{what}: planned {:?}, played {:?}", s.join, u.join);
                        // Never a hole, by its notes (session 16b).
                        if matches!(u.join, Join::Alone | Join::Floor | Join::Breath) {
                            assert!(u.under < JOINS.hole, "{what}: a {:?} {:.1} dB under the unit", u.join, u.under);
                        }
                        if !matches!(u.join, Join::None | Join::Glide | Join::Prepared) {
                            assert!(u.join_from > u.from && u.join_from < u.to, "{what}: {:?} from {} in {}..{}", u.join, u.join_from, u.from, u.to);
                        }
                        if u.join == Join::Pivot {
                            assert!(u.pivot_count >= 2 && u.out & (1 << Layer::Chords as usize) != 0, "{what}");
                            // Struck, never at a silent entry chord's 0
                            // (session 16: 1 loop in 12 had every pivot a hole).
                            assert!(u.pivot_vel > 0.0, "{what}: a pivot struck at nothing");
                        }
                        if u.join == Join::Fill {
                            assert!(u.fill_vel[0] > 0.0 && u.fill_vel[0] < u.fill_vel[1] && !u.fill.get().is_empty(), "{what}");
                        }
                        if u.join == Join::Alone {
                            assert_eq!(u.out & (1 << Layer::Melody as usize), 0, "{what}: the tune left out of its own bar");
                            // The tune sounds there as the unit plays it
                            // (session 16: a stretched return joined "alone"
                            // over a tune silent there, three seconds near
                            // silence).
                            assert!(tune_sounds(&own, &second, cycle, pass, u), "{what}: the tune alone over a silent tune");
                        }
                    }
                }
            }
        }
        // Every kind is in the corpus, planned and as played.
        for (x, k) in Join::ALL.iter().enumerate() {
            assert!(kinds[x] > 0, "no {} planned", k.name());
            assert!(heard_kinds[x] > 0, "no {} played", k.name());
        }
        assert!(lines > 500, "{lines} lines");
    });
}

/// The holes session 05 found on Endless arcs (session 16b): ta-muin's tune
/// alone over a stab's one note into sections 4 and 18, a kit and a bass
/// that are most of the loop (19 to 38 dB under it), and yi-yoar's floor out
/// of section 15, its kit and bass most of the mix (33 to 40 dB under). Each
/// falls back where its notes say it would be a hole: a breath, or nothing
/// written.
#[test]
fn a_join_that_would_be_a_hole_falls_back() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        for (seed, n, planned) in [(1_337_654_778, 3, Join::Alone), (1_337_654_778, 17, Join::Alone), (819_045_617, 15, Join::Floor)] {
            let form = load(&mut g, seed, "bars=endless", Journey::Arc);
            let last = form.units_in(n) - 1;
            assert_eq!(form.joined(n, last).join, planned, "seed {seed} section {n}: the plan's join");
            let pass = g.section(n, 0.0);
            let u = pass.heard.units().last().copied().expect("a unit");
            assert!(matches!(u.join, Join::Breath | Join::Glide), "seed {seed} section {n}: {:?} played, {:.1} dB under", u.join, u.under);
        }
    });
}

#[test]
fn a_section_joins_the_same_reached_straight_or_by_playing() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        for seed in seeds(6) {
            for (text, journey) in [("bars=1024", Journey::Arc), ("bars=endless", Journey::Arc), ("bars=endless", Journey::Wander)] {
                load(&mut g, seed, text, journey);
                // Played in order, then each straight from its number, the
                // second tunes made afresh (a far section too: promise 11).
                let along: Vec<u64> = (0..12).map(|n| g.section_hash(n, 0.0)).collect();
                for n in (0..12).rev() {
                    load(&mut g, seed, text, journey);
                    assert_eq!(g.section_hash(n, 0.0), along[n as usize], "seed {seed} {text}: section {n}");
                }
                if text == "bars=endless" {
                    let far = 1_000_000_000;
                    let once = g.section_hash(far, 0.0);
                    load(&mut g, seed, text, journey);
                    let _ = g.section_hash(far - 1, 0.0);
                    assert_eq!(g.section_hash(far, 0.0), once, "seed {seed} {text}: section 10^9");
                }
            }
        }
    });
}

#[test]
fn a_close_the_slider_asks_for_lands_on_the_tonic() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let mut landed = 0;
        for seed in seeds(20) {
            for (text, journey) in [("bars=endless", Journey::Arc), ("bars=1024", Journey::Arc), ("bars=endless", Journey::Wander)] {
                let form = load(&mut g, seed, text, journey);
                let spb = g.spec().steps_per_bar;
                let bars = g.spec().bars;
                // A stop part way into section 5: the next phrase line (as
                // `track::phrase_line` finds it) past three bars in.
                let phrase = if bars % 4.0 == 0.0 { 4.0 } else if bars % 2.0 == 0.0 { 2.0 } else { bars };
                let into = (3.0 / phrase).ceil().min(bars / phrase) * phrase;
                let stop_bars = 5.0 * bars + into;
                let (mut struck, mut tune_whole) = (false, false);
                for n in 0..=5u64 {
                    let start = n as f64 * bars;
                    let stop = (stop_bars - start) * spb;
                    let mut heard = g.section(n, 0.0).heard;
                    let plain = heard;
                    let cx = Cx::new(g.spec());
                    let second = g.second(Tune::of(&form, n));
                    heard.stop_at(&form, &cx, g.render(), second, n, stop, 0.0);
                    let what = format!("seed {seed} {text} {journey:?} section {n}");
                    if !heard.stops {
                        // Nowhere near the stop: nothing changes.
                        assert_eq!(format!("{:?}", heard.units()), format!("{:?}", plain.units()), "{what}");
                        continue;
                    }
                    // The phrase before the stop is the close's: at home,
                    // no join after it.
                    let close = heard.units().iter().filter(|u| u.to > stop - phrase * spb && u.from < stop);
                    assert!(close.clone().all(|u| u.join == Join::None && u.mapped == 0 && u.shift.iter().all(|&x| x % 12 == 0)), "{what}: {:?}", heard.units());
                    if heard.landing.on && heard.landing.strikes {
                        struck = true;
                        let l = &heard.landing;
                        assert!(l.at < stop && l.at + spb <= stop, "{what}: lands at {} for a stop at {stop}", l.at);
                        let root = i16::from(form.home.root);
                        assert!(l.chord.is_empty() || l.chord.iter().min().is_some_and(|&lo| lo.rem_euclid(12) == root), "{what}: {l:?}");
                        assert!(l.chord.is_empty() || l.chord_vel > 0.0, "{what}: a landing struck at nothing");
                        assert!(l.tune == 0.0 || (l.tune as i16).rem_euclid(12) == root, "{what}: the tune off the tonic");
                        // Everything after the landing is left out: it rings alone.
                        assert!(heard.landed(stop - 1.0), "{what}");
                    }
                    tune_whole |= heard.units().iter().any(|u| u.from >= stop - 4.0 * phrase * spb);
                }
                assert!(struck, "seed {seed} {text} {journey:?}: no landing before the stop at bar {stop_bars}");
                assert!(tune_whole, "seed {seed} {text}");
                landed += 1;
            }
        }
        assert_eq!(landed, 60);
    });
}
