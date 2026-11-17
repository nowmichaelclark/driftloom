//! colorTone (queue item 101e; `generator::color`): a held 13th over the
//! major chords and an 11th over the minor ones, a choice drawn by itself
//! (`alone`), not among the changes, on a stream of its own. What it holds
//! to: it goes only to loops with a chord that can take a color, for about
//! its share of them, leaned toward the jazzy vibes; a pin plays it or keeps
//! it off; where it plays, every color is the 13 over a major chord or the 11
//! over a minor one, in the chord's scale, voiced over the chord at C6 at
//! most, held over the batch's 25 to 50% of the chord time across the
//! corpus; and nothing the loop plays but the colors moves. That no other
//! choice moved is the generation goldens' (the PR has the count).

#![allow(clippy::cast_precision_loss, clippy::float_cmp)] // counts of loops, as shares; steps copied, not computed

use dlcore::generator::Generator;
use dlcore::generator::color::{self, TOP};
use dlcore::generator::render::Render;
use dlcore::generator::request::Request;
use dlcore::generator::rng::Rng;

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

/// Of `n` corpus loops as `request` asks them: how many may have the color,
/// and how many play it.
fn share(g: &mut Generator, request: &Request, n: usize) -> (usize, usize) {
    let (mut of, mut on) = (0, 0);
    for seed in corpus(n) {
        g.load_with(seed, request);
        let r = g.render();
        let plays = r.choice("colorTone") == Some("on");
        if color::colorable(&r.harmony.slots, &r.chords, g.spec()) {
            of += 1;
            on += usize::from(plays);
        } else {
            assert!(!plays && r.colors.is_empty(), "seed {seed}: no chord can take a color, and none plays");
        }
    }
    (of, on)
}

#[test]
fn it_goes_to_its_share_of_the_loops_that_can_take_it() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let (of, on) = share(&mut g, &Request::new(), 1500);
        let f = on as f64 / of as f64;
        assert!(of > 900 && (0.11..=0.19).contains(&f), "{on} of {of} ({f:.3}), not near 15%");
    });
}

#[test]
fn the_vibes_lean_it_toward_jazzy_and_away_from_minimal() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let jazzy = share(&mut g, &ask("jazzy"), 800);
        let minimal = share(&mut g, &ask("minimal"), 800);
        let (j, m) = (jazzy.1 as f64 / jazzy.0 as f64, minimal.1 as f64 / minimal.0 as f64);
        assert!(j > 0.2 && m < 0.1 && m > 0.0, "jazzy {j:.3}, minimal {m:.3}");
    });
}

/// The chord time and the colored time of a loop's chords layer, in steps:
/// every step a chord sounds on (struck and held), and those a color holds.
fn times(r: &Render) -> (f64, f64) {
    let total = r.harmony.cycle_steps;
    let (mut chord, mut colored) = (0.0, 0.0);
    let mut s = 0.0;
    while s < total {
        let sounds = r.chords.iter().any(|e| e.vel != 0.0 && !e.notes.is_empty() && e.step <= s && s < e.step + e.dur);
        if sounds {
            chord += 1.0;
            if r.colors.iter().any(|c| c.step <= s && s < c.step + c.dur) {
                colored += 1.0;
            }
        }
        s += 1.0;
    }
    (chord, colored)
}

#[test]
fn every_color_is_the_thirteen_over_major_or_the_eleven_over_minor_above_the_chord() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let on = ask("colorTone=on");
        let (mut loops, mut colors, mut major, mut minor) = (0, 0, 0, 0);
        let (mut chord_time, mut colored_time) = (0.0, 0.0);
        for seed in corpus(600) {
            g.load_with(seed, &on);
            let r = g.render();
            if r.colors.is_empty() {
                continue;
            }
            loops += 1;
            let spec = g.spec();
            for c in r.colors.iter() {
                colors += 1;
                let slot = r.harmony.slot_at(c.step);
                #[allow(clippy::cast_possible_truncation)] // a MIDI note
                let root = slot.root_midi.round() as i32;
                let iv = (i32::from(c.midi) - root).rem_euclid(12);
                let rel = |n: i16| (i32::from(n) - root).rem_euclid(12);
                let major3 = slot.notes.iter().any(|&n| rel(n) == 4);
                let minor3 = slot.notes.iter().any(|&n| rel(n) == 3);
                assert!(major3 || minor3, "seed {seed}: a color over a chord with no third");
                if major3 {
                    assert_eq!(iv, 9, "seed {seed}: the 13 over a major chord");
                    major += 1;
                } else {
                    assert_eq!(iv, 5, "seed {seed}: the 11 over a minor chord");
                    minor += 1;
                }
                assert!(!slot.notes.iter().any(|&n| rel(n) == iv), "seed {seed}: the color already in the chord");
                assert!(slot.notes.iter().any(|&n| rel(n) == 7) || !slot.notes.iter().any(|&n| matches!(rel(n), 6 | 8)), "seed {seed}: a chord whose fifth is not perfect");
                let scale = slot.steps.as_ref().map_or(spec.steps(), |s| &s[..]);
                #[allow(clippy::cast_possible_truncation)] // a MIDI note
                let at = (i32::from(c.midi) - spec.root.round() as i32).rem_euclid(12);
                #[allow(clippy::cast_possible_truncation)] // a scale's step
                let in_scale = scale.iter().any(|&s| s.round() as i32 == at);
                assert!(in_scale, "seed {seed}: a color its scale lacks");
                assert!(c.midi <= TOP, "seed {seed}: above C6");
                let e = &r.chords[usize::from(c.anchor)];
                assert!(e.vel != 0.0 && e.step < c.step + c.dur && e.step + e.dur > c.step, "seed {seed}: its chord sounds with it");
                let mut played = slot.notes;
                dlcore::generator::render::lower(&mut played, &slot.notes, r.chord_shift);
                let top = played.iter().copied().max().unwrap_or(0);
                if c.midi <= top {
                    assert!(c.midi + 12 > TOP, "seed {seed}: under the top note only where C6 holds it");
                }
            }
            let (ch, co) = times(r);
            chord_time += ch;
            colored_time += co;
        }
        let held = colored_time / chord_time;
        println!("colorTone on 600 corpus loops: {loops} loops with a color, {colors} colors ({major} 13s, {minor} 11s), held over {:.1}% of the chord time", held * 100.0);
        assert!(loops > 300 && major > 0 && minor > 0, "{loops} loops, {major} 13s, {minor} 11s");
        assert!((0.25..=0.5).contains(&held), "held over {:.1}% of the chord time", held * 100.0);
    });
}

#[test]
fn nothing_but_the_colors_moves() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let mut a = Box::new(Generator::new());
        for seed in corpus(200) {
            g.load_with(seed, &ask("colorTone=on"));
            a.load_with(seed, &ask("colorTone=today"));
            let (on, off) = (g.render(), a.render());
            assert!(off.colors.is_empty(), "seed {seed}: today's plays no color");
            assert_eq!(on.chords.len(), off.chords.len(), "seed {seed}");
            for (x, y) in on.chords.iter().zip(off.chords.iter()) {
                assert!(x.step == y.step && x.dur == y.dur && x.vel == y.vel && x.notes[..] == y.notes[..], "seed {seed}: a chord moved");
            }
            for (x, y) in on.melody.iter().zip(off.melody.iter()) {
                assert!(x.step == y.step && x.midi == y.midi && x.vel == y.vel, "seed {seed}: the tune moved");
            }
            for (x, y) in on.bass.iter().zip(off.bass.iter()) {
                assert!(x.step == y.step && x.midi == y.midi && x.vel == y.vel, "seed {seed}: the bass moved");
            }
            for (x, y) in on.drums.iter().zip(off.drums.iter()) {
                assert!(x.step == y.step && x.vel == y.vel && x.inst == y.inst, "seed {seed}: the kit moved");
            }
            assert_eq!(on.chord_shift, off.chord_shift, "seed {seed}: the chords' octave");
        }
    });
}
