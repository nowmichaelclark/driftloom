//! An arc's themes, variations and keys (queue item 102d, session 14;
//! `docs/LONGFORM.md`): the second tune's making (another seed's tune bent
//! into the loop, through the request's theme machinery; made the same
//! however a section is reached; a theme that rests stays resting; nothing
//! of the loop moved by it), each variation alone as the marks it puts on
//! a unit, the keys' maps, and the movements' kinds. `tools/longform-themes.mjs`
//! measures the same on what the player plays.

// Seeds, steps and pitches are compared exactly on purpose; sections and
// units are whole numbers far below 2^53; a generator, a section, a unit
// and a span keep their short names.
#![allow(clippy::many_single_char_names, clippy::float_cmp, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]

use dlcore::generator::data::LONGFORM;
use dlcore::generator::heard::{Tune, key_map, tune};
use dlcore::generator::piece::Piece;
use dlcore::generator::longform::{Form, Join, Key, Kind, Relation, Role, Span, Theme, Variation, changed};
use dlcore::generator::request::{Journey, Request};
use dlcore::generator::rng::Rng;
use dlcore::generator::theory::SCALES;
use dlcore::generator::{Generator, Layer, Part};
use dlcore::player::Player;

/// A load needs more stack than a test's thread has (`tests/length.rs`).
fn roomy<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    let stack = (std::mem::size_of::<Player>() + std::mem::size_of::<Generator>()) * 4 + (16 << 20);
    std::thread::Builder::new().stack_size(stack).spawn(f).expect("a thread").join().expect("the test")
}

/// How many sections a round of the piece loaded has.
fn sections_of(g: &Generator) -> u64 {
    Piece::of(g.spec()).and_then(|p| p.end).map(|e| e.0).expect("a finite piece")
}

fn seeds(n: usize) -> Vec<u32> {
    let mut r = Rng::new(20_261_009);
    (0..n).map(|_| (r.f() * 4_294_967_296.0) as u32).collect()
}

fn load(g: &mut Generator, seed: u32, text: &str) -> Form {
    g.load_with(seed, &Request::parse(text).expect("a request").with_journey(Journey::Arc));
    g.form().expect("a long piece has a plan")
}

/// A round's units of an arc piece, each with its section and its place.
fn round(form: &Form, sections: u64) -> Vec<(u64, u64, Span)> {
    (0..sections).flat_map(|n| (0..form.units_in(n)).map(move |p| (n, p, form.at(n, p)))).collect()
}

fn scale(id: &str) -> usize {
    SCALES.iter().position(|s| s.id == id).expect("a scale")
}

#[test]
fn a_key_moves_every_note_along_the_mode() {
    // C ionian to its relative, A aeolian: every note two of the mode's
    // steps down, so the tonic chord becomes the chord on the sixth, and
    // every note stays in the mode.
    let home = Key { root: 0, scale: scale("ionian"), relation: Relation::Home };
    let to = Key { root: 9, scale: scale("aeolian"), relation: Relation::Relative };
    let map = key_map(home, to);
    assert_eq!([map[0], map[4], map[7]], [-3, -4, -3], "C E G to A C E");
    let steps = SCALES[home.scale].steps;
    for &s in steps {
        let moved = (s as i32 + i32::from(map[s as usize])).rem_euclid(12);
        assert!(steps.contains(&f64::from(moved)), "{s} leaves the mode");
    }
    // A aeolian to its parallel, A ionian: the third, the sixth and the
    // seventh raised, the rest where they were.
    let home = Key { root: 9, scale: scale("aeolian"), relation: Relation::Home };
    let map = key_map(home, Key { root: 9, scale: scale("ionian"), relation: Relation::Parallel });
    assert_eq!(map, [0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0]);
    // A fifth away, and home, move nothing here (the whole loop moves).
    assert_eq!(key_map(home, Key { root: 4, scale: home.scale, relation: Relation::FifthAbove }), [0; 12]);
    assert_eq!(key_map(home, home), [0; 12]);
    // Every related key of every mode keeps every note of the mode in the
    // key it moves to.
    for (x, mode) in SCALES.iter().enumerate() {
        for (y, other) in SCALES.iter().enumerate() {
            for root in 0..12u8 {
                let home = Key { root: 0, scale: x, relation: Relation::Home };
                let notes = |k: Key, steps: &[f64]| steps.iter().fold(0u16, |m, &s| m | 1 << ((usize::from(k.root) + s as usize) % 12));
                let relative = Key { root, scale: y, relation: Relation::Relative };
                if root != 0 && notes(relative, other.steps) == notes(home, mode.steps) {
                    let map = key_map(home, relative);
                    assert_eq!((i32::from(map[0])).rem_euclid(12), i32::from(root), "{} to {} on {root}: the home note", mode.id, other.id);
                    for &s in mode.steps {
                        let moved = (s as i32 + i32::from(map[s as usize])).rem_euclid(12);
                        assert!(mode.steps.contains(&f64::from(moved)), "{} to {}: {s} leaves the notes", mode.id, other.id);
                    }
                }
            }
            let map = key_map(home_of(x), Key { root: 0, scale: y, relation: Relation::Parallel });
            for &s in mode.steps {
                let moved = (s as i32 + i32::from(map[s as usize])).rem_euclid(12);
                assert!(other.steps.contains(&f64::from(moved)), "{} to its parallel {}: {s} leaves the mode", mode.id, other.id);
            }
        }
    }
}

fn home_of(scale: usize) -> Key {
    Key { root: 0, scale, relation: Relation::Home }
}

#[test]
fn the_second_tune_is_made_the_same_however_it_is_reached() {
    roomy(|| {
        let mut straight = Box::new(Generator::new());
        let mut through = Box::new(Generator::new());
        let (mut made, mut rests, mut answers) = (0, 0, 0);
        for seed in seeds(40) {
            let form = load(&mut straight, seed, "bars=256");
            load(&mut through, seed, "bars=256");
            let before: Vec<u64> = Part::ALL.into_iter().map(|p| straight.hash(p)).collect();
            let sections = sections_of(&straight);
            // Through: every section in order, a stage of the next's tune
            // at a time, as the player makes it.
            let mut walked = Vec::new();
            for n in 0..sections {
                while !through.prepare(n) {}
                let want = Tune::of(&form, n);
                walked.push(through.second(want).map(|m| m.iter().map(|e| (e.step, e.midi, e.dur, e.vel)).collect::<Vec<_>>()));
                through.section(n, 0.0);
            }
            // Straight: sections from the round's end back.
            for n in (0..sections).rev() {
                let want = Tune::of(&form, n);
                straight.section(n, 0.0);
                let notes = straight.second(want).map(|m| m.iter().map(|e| (e.step, e.midi, e.dur, e.vel)).collect::<Vec<_>>());
                assert_eq!(notes, walked[n as usize], "seed {seed} section {n}: the second tune differs by the way there");
                if let Some(notes) = notes {
                    made += 1;
                    rests += usize::from(notes.is_empty());
                    answers += usize::from(want.kind == Tune::ANSWER);
                    // In the tune's own range, and its notes on the loop's steps.
                    assert!(notes.iter().all(|n| (43.0..=96.0).contains(&n.1)), "seed {seed} section {n}: out of range");
                } else {
                    assert_eq!(want, Tune::NONE, "seed {seed} section {n}: a tune wanted and not made");
                }
            }
            // Nothing of the loop moved.
            let after: Vec<u64> = Part::ALL.into_iter().map(|p| straight.hash(p)).collect();
            assert_eq!(before, after, "seed {seed}: making a second tune moved the loop");
        }
        assert!(made > 200 && rests > 0 && rests * 4 < made && answers > 10, "{made} sections with a second tune, {rests} resting, {answers} answers");
    });
}

/// The first unit of the first `role` of the piece whose variation is
/// `variation`, over the corpus: the generator loaded, the form, and where.
fn find(g: &mut Generator, role: Role, variation: Variation, theme: Theme) -> (Form, u64, u64, Span) {
    for seed in seeds(400) {
        let form = load(g, seed, "bars=256");
        if g.render().melody.iter().filter(|e| e.vel != 0.0).count() < 6 || g.spec().choir().is_some() {
            continue;
        }
        let sections = sections_of(g);
        if let Some((n, p, s)) = round(&form, sections).into_iter().find(|(_, _, s)| s.role == role && s.variation == variation && s.theme == theme && !s.fragment) {
            return (form, n, p, s);
        }
    }
    panic!("no {} {} in the corpus", role.name(), variation.name());
}

#[test]
fn each_variation_alone() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let m = Layer::Melody as usize;
        let own = |g: &Generator| g.render().melody.iter().map(|e| e.midi).collect::<Vec<_>>();

        // Register: the tune an octave away, every note of it, nothing else
        // moved (or, a tune too wide to move, thinned to its outline).
        let (_, n, p, _) = find(&mut g, Role::Return, Variation::Register, Theme::A);
        let tune = own(&g);
        let pass = g.section(n, 0.0);
        let u = pass.heard.units()[p as usize];
        assert!(u.lift.abs() == 12 || u.thin[1] >= LONGFORM.outline, "register: {u:?}");
        let moved = pass.moved(u.from, Layer::Melody, tune[0]);
        assert_eq!(moved, f64::from(u.lift + u.shift[m]), "register: the tune's move");
        assert_eq!(u.tune, [tune::OWN; 2]);

        // Voice: another voice of the tune's pool, the notes its own.
        let (_, n, p, _) = find(&mut g, Role::Return, Variation::Voice, Theme::A);
        let voice = g.render().melody_out.voice;
        let u = g.section(n, 0.0).heard.units()[p as usize];
        assert!((!u.voice[m].is_empty() && u.voice[m] != voice) || u.lift != 0 || u.thin[1] >= LONGFORM.outline, "voice: {u:?}");
        assert_eq!((u.tune, u.mapped), ([tune::OWN; 2], 0));

        // Stretched: each phrase's first half at half speed, the chords held.
        let (_, n, p, _) = find(&mut g, Role::Return, Variation::Stretched, Theme::A);
        let pass = g.section(n, 0.0);
        let u = pass.heard.units()[p as usize];
        assert!(u.stretched, "stretched: {u:?}");
        let phrase = pass.heard.phrase;
        assert_eq!(pass.heard.stretched_from(u.from + 6.0), Some(u.from + 3.0), "the note three steps in sounds at six");
        assert_eq!(pass.heard.stretched_from(u.from + 5.0), None, "nothing between");
        assert_eq!(pass.heard.stretched_from(u.from + phrase - 2.0), Some(u.from + phrase / 2.0 - 1.0), "the half's last note at the phrase's end");

        // Reharmonized: the chords, the bass and the texture to the relative
        // key, along the mode; the tune where it was.
        let (form, n, p, _) = find(&mut g, Role::Return, Variation::Reharmonized, Theme::A);
        let pass = g.section(n, 0.0);
        let u = pass.heard.units()[p as usize];
        assert_eq!(u.mapped & (1 << m), 0, "reharmonized: the tune moved");
        assert!(u.mapped & (1 << Layer::Chords as usize) != 0 && u.mapped & (1 << Layer::Bass as usize) != 0 && u.map[0] != 0, "reharmonized: {u:?}");
        assert_eq!(pass.moved(u.from, Layer::Melody, 60.0), 0.0);
        let relative = form.related().find(|k| k.relation == Relation::Relative).expect("a relative key");
        let root = f64::from(form.home.root);
        let by = pass.moved(u.from, Layer::Bass, 36.0 + root);
        assert_eq!((36.0 + root + by).rem_euclid(12.0), f64::from(relative.root), "reharmonized: the bass's home note to the relative's");

        // Answered: each phrase's first half the loop's own, its second the
        // answer's, a tune made from the loop's own figure.
        let (_, n, p, _) = find(&mut g, Role::Return, Variation::Answered, Theme::A);
        let pass = g.section(n, 0.0);
        let (second, u, half) = (pass.heard.second, pass.heard.units()[p as usize], pass.heard.half);
        assert_eq!(u.tune, [tune::OWN, tune::SECOND], "answered");
        assert_eq!((pass.heard.tune_at(u.from), pass.heard.tune_at(u.from + half)), (tune::OWN, tune::SECOND));
        assert!(second != Tune::NONE && g.second(second).is_some(), "answered: no answer made");

        // Outline: the tune thinned, by the data's share at least.
        let (_, n, p, _) = find(&mut g, Role::Release, Variation::Outline, Theme::A);
        let u = g.section(n, 0.0).heard.units()[p as usize];
        assert!(u.thin[1] >= LONGFORM.outline, "outline: {u:?}");

        // Key: theme A in the related key, the whole loop there.
        let (_, n, p, s) = find(&mut g, Role::Second, Variation::Key, Theme::A);
        let u = g.section(n, 0.0).heard.units()[p as usize];
        assert!(s.key.relation != Relation::Home && (u.shift[m] != 0 || u.mapped & (1 << m) != 0), "key: {u:?}");
        assert_eq!(u.tune, [tune::OWN; 2]);
    });
}

#[test]
fn a_piece_plays_its_plan_theme_by_theme() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let (mut seconds, mut calls, mut prepared, mut fragments, mut rests) = (0, 0, 0, 0, 0);
        for seed in seeds(60) {
            let form = load(&mut g, seed, "bars=256");
            let sections = sections_of(&g);
                        for n in 0..sections {
                let want = Tune::of(&form, n);
                g.section(n, 0.0);
                // (A second tune that rests leaves the arc theme A.)
                let sings = g.second(want).is_some_and(|m| m.iter().any(|e| e.vel != 0.0));
                rests += usize::from(want.kind == Tune::SECOND && !sings);
                let pass = g.section(n, 0.0);
                let has = pass.heard.second != Tune::NONE && sings;
                for (p, u) in pass.heard.units().iter().enumerate() {
                    let s = form.at(n, p as u64);
                    let what = format!("seed {seed} section {n}.{p} ({})", s.role.name());
                    if s.journey == Journey::Wander {
                        continue;
                    }
                    // One voice: a half plays one tune or rests.
                    assert!(u.tune.iter().all(|&t| t <= tune::REST), "{what}");
                    let other = matches!(s.theme, Theme::B | Theme::Recall(_)) || s.variation == Variation::Answered;
                    if other && sings {
                        assert!(u.tune.contains(&tune::SECOND), "{what}: the plan's other tune is not played");
                        seconds += 1;
                    } else if !s.calls || !sings {
                        assert!(!u.tune.contains(&tune::SECOND), "{what}: a second tune where the plan has theme A");
                    }
                    if s.fragment {
                        fragments += 1;
                        assert!(if s.calls { u.tune == [tune::OWN, if has { tune::SECOND } else { tune::OWN }] } else { u.tune[1] == tune::REST }, "{what}: a fragment is a phrase's first half");
                    }
                    calls += usize::from(s.calls);
                    // The key: home is the loop where it was written.
                    if s.key.relation == Relation::Home && s.variation != Variation::Reharmonized {
                        assert!(u.mapped == 0 && u.shift.iter().all(|&x| x % 12 == 0), "{what}: moved at home");
                    }
                    // A return prepared: the bass on the home key's fifth for
                    // the unit's last phrase, and the next unit theme A at home.
                    if s.prepares {
                        prepared += 1;
                        let fifth = (i16::from(form.home.root) + 7).rem_euclid(12);
                        assert!(u.pedal > 0 && (u.pedal.rem_euclid(12) - fifth).abs() <= 1 && u.pedal_from >= u.from && u.pedal_from < u.to, "{what}: {u:?}");
                    } else if u.join == Join::Pivot {
                        // (A pivot holds the bass, where the loop has one, on
                        // its chord's root for its bar: session 15.)
                        assert!(u.pedal == 0 || u.pedal_from == u.join_from, "{what}: {u:?}");
                    } else {
                        assert_eq!(u.pedal, 0, "{what}");
                    }
                    // The landing is the home key's.
                    if s.closes {
                        assert!(s.key == form.home && u.mapped == 0, "{what}: the close has left home");
                    }
                }
            }
        }
        assert!(seconds > 300 && calls > 20 && prepared > 20 && fragments > 100 && rests > 0, "{seconds} units of a second tune, {calls} calls, {prepared} returns prepared, {fragments} fragments, {rests} sections whose second tune rests");
    });
}

#[test]
fn movements_differ_in_kind() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let (mut cycled, mut every) = (0, 0);
        for seed in seeds(60) {
            for text in ["bars=1024", "bars=3072", "bars=32768"] {
                let form = load(&mut g, seed, text);
                let what = format!("seed {seed} {text}");
                let sections = sections_of(&g);
                let mut kinds: Vec<(u64, Kind)> = Vec::new();
                for (_, _, s) in round(&form, sections.min(40_000)) {
                    if kinds.last().map(|k| k.0) != Some(s.part) {
                        assert_eq!(s.part, kinds.len() as u64, "{what}: the movements in order");
                        kinds.push((s.part, s.kind));
                    }
                }
                let whole = sections <= 40_000;
                if form.arcs() <= 2 {
                    assert!(kinds.iter().all(|k| k.1 == Kind::Full), "{what}: one or two arcs are as they were");
                    continue;
                }
                if form.cycle().len() == 1 {
                    continue;
                }
                cycled += 1;
                assert_eq!(kinds[0].1, Kind::Full, "{what}: the first movement");
                if whole {
                    assert_eq!(kinds.len() as u64, form.arcs(), "{what}: the movements counted");
                    assert_eq!(kinds.last().map(|k| k.1), Some(Kind::Full), "{what}: the last movement");
                }
                for pair in kinds.windows(2) {
                    assert!(pair[0].1 != pair[1].1 || pair[0].1 == Kind::Full, "{what}: two {} movements side by side", pair[0].1.name());
                }
                if form.arcs() >= 4 {
                    every += 1;
                    for kind in [Kind::Full, Kind::Gentle, Kind::Light, Kind::Interlude] {
                        assert!(kinds.iter().any(|k| k.1 == kind), "{what}: no {} movement", kind.name());
                    }
                } else {
                    assert!(kinds.iter().all(|k| k.1 != Kind::Interlude), "{what}: an interlude in three movements");
                }
            }
            // Endless takes the same cycle, round and round, found by a
            // division at any number.
            let form = load(&mut g, seed, "bars=endless");
            assert_eq!(form.cycle().len(), 4, "seed {seed}: Endless's cycle");
            for n in [0u64, 1_000_000, 1_000_000_000, 1 << 50] {
                let s = form.at(n, 0);
                assert_eq!(s.kind, form.cycle()[(s.part % 4) as usize], "seed {seed} section {n}");
                assert!(!s.closes, "seed {seed}: Endless closes");
            }
        }
        assert!(cycled > 60 && every > 40, "{cycled} pieces in movements of kinds, {every} with every kind");
    });
}

#[test]
fn an_interlude_moves_one_thing_at_a_time_and_comes_home() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let (mut interludes, mut moves) = (0, 0);
        for seed in seeds(40) {
            let form = load(&mut g, seed, "bars=3072");
            if !form.cycle().contains(&Kind::Interlude) {
                continue;
            }
            let sections = sections_of(&g);
            let spans = round(&form, sections);
            for (x, (_, _, s)) in spans.iter().enumerate() {
                if s.kind != Kind::Interlude {
                    continue;
                }
                let what = format!("seed {seed} unit {x}");
                let things = s.changed & changed::STATE;
                assert!(things.count_ones() <= 1, "{what}: {} things move at once", things.count_ones());
                moves += things.count_ones();
                assert!(s.key == form.home && s.theme == Theme::A, "{what}: an interlude leaves the key or the theme");
                // Its first and its last are the loop's own.
                let first = spans[x - 1].2.kind != Kind::Interlude;
                let last = spans.get(x + 1).is_none_or(|n| n.2.kind != Kind::Interlude);
                if first || last {
                    assert_eq!(s.state, dlcore::generator::longform::State::OWN, "{what}: not home at its end");
                }
                interludes += usize::from(first);
            }
        }
        assert!(interludes > 40 && moves as usize >= interludes, "{interludes} interludes, {moves} moves");
    });
}
