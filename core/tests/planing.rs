//! planingSevenths (queue item 101f; batch 6's #10): an option of the
//! progression's per-loop draw (`generator::planing`). What it holds to: about
//! the data's share of loops plane, leaned by the vibes as the batch's table
//! says; a planed loop's every chord is a major seventh, the first on the
//! tonic, each root a whole step or a minor third from the one before and
//! never more than a fourth from the tonic; every note of its tune and its
//! bass is in the scale of the chord it starts over (the chord's Lydian),
//! most on the chord's own tones; the tune keeps its rhythm, so it is the
//! same tune following the chords; a `progression.kind` pin holds a loop to
//! either kind; and the spelling moves a note a semitone at most. That a loop
//! that does not plane plays exactly as before is the generation goldens'
//! (the PR has the count of the cases that moved).

// Counts of loops are shares as floats; pitches are whole and compared
// exactly on purpose.
#![allow(clippy::float_cmp, clippy::cast_precision_loss, clippy::cast_possible_truncation)]

use dlcore::generator::Generator;
use dlcore::generator::data::PROGRESSION;
use dlcore::generator::plan::{Cx, Slot};
use dlcore::generator::planing::{self, CHORD};
use dlcore::generator::request::{BAD_SETTING, Request};
use dlcore::generator::rng::Rng;

fn roomy(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new().stack_size(64 << 20).spawn(f).expect("a thread").join().expect("the test");
}

/// The seeds of the batch's corpus (corpus seed 20261009), its first `n`.
fn corpus(n: usize) -> Vec<u32> {
    let mut r = Rng::new(20_261_009);
    (0..n).map(|_| r.seed32()).collect()
}

fn pc(x: f64) -> f64 {
    ((x % 12.0) + 12.0) % 12.0
}

/// Whether the loop loaded in `g` planes, as its context draws it.
fn planes(g: &Generator) -> bool {
    Cx::new(g.spec()).planing
}

/// The share of `n` corpus loops that plane as `request` asks them.
fn share(g: &mut Generator, request: &Request, n: usize) -> f64 {
    let seeds = corpus(n);
    seeds.iter().filter(|&&seed| {
        g.load_with(seed, request);
        planes(g)
    }).count() as f64 / n as f64
}

/// The slot sounding at `step` (the harmony's, wrapping on its cycle).
fn slot_at(g: &Generator, step: f64) -> Slot {
    *g.render().harmony.slot_at(step)
}

#[test]
fn about_the_data_s_share_of_loops_plane() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let s = share(&mut g, &Request::new(), 2000);
        println!("planing: {s:.3} of 2000 (data: {})", PROGRESSION.planing);
        assert!((s - PROGRESSION.planing).abs() < 0.03, "{s:.3} of loops plane");
    });
}

#[test]
fn the_vibes_lean_it_as_the_batch_s_table_says() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let of = |g: &mut Generator, vibe: &str| share(g, &Request::new().with_vibe(vibe).expect("a vibe"), 1000);
        let (dreamy, floating, earthy, minimal) = (of(&mut g, "dreamy"), of(&mut g, "floating"), of(&mut g, "earthy"), of(&mut g, "minimal"));
        println!("planing: dreamy {dreamy:.3}, floating {floating:.3}, earthy {earthy:.3}, minimal {minimal:.3}");
        assert!(dreamy > PROGRESSION.planing && floating > PROGRESSION.planing, "toward the dreamy vibes");
        assert!(earthy < PROGRESSION.planing && minimal < PROGRESSION.planing, "away from the earthy ones");
        assert!(earthy > 0.0 && minimal > 0.0, "a lean, never 0");
    });
}

#[test]
fn a_planed_loop_slides_major_sevenths_and_its_tune_and_bass_follow() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let planing = Request::parse("progression.kind=planing").expect("a request");
        let (mut tune, mut on_chord, mut bass) = (0usize, 0usize, 0usize);
        for seed in corpus(150) {
            g.load_with(seed, &planing);
            assert!(planes(&g), "pinned, seed {seed}");
            let tonic = g.spec().root;
            let slots = &g.render().harmony.slots;
            assert!(!slots.is_empty(), "seed {seed}");
            let offset = |s: &Slot| (s.root_midi - tonic + 6.0).rem_euclid(12.0) - 6.0;
            assert_eq!(offset(&slots[0]), 0.0, "the first chord the tonic's, seed {seed}");
            for s in slots.iter() {
                let mut pcs: Vec<f64> = s.notes.iter().map(|&n| pc(f64::from(n) - s.root_midi)).collect();
                pcs.sort_by(f64::total_cmp);
                pcs.dedup();
                assert_eq!(pcs, CHORD.to_vec(), "a major seventh, seed {seed}");
                assert!(offset(s).abs() <= 5.0, "within a fourth of the tonic, seed {seed}");
                // Its own scale (the Lydian, or its pentatonic in a mode of
                // five), note for note against the mode.
                let mut scale: Vec<f64> = planing::chord_scale(g.spec().steps().len()).iter().map(|&x| pc(s.root_midi - tonic + x)).collect();
                scale.sort_by(f64::total_cmp);
                let mut st: Vec<f64> = s.steps.as_ref().expect("a planed slot's scale").to_vec();
                assert_eq!(st.len(), g.spec().steps().len(), "as many as the mode's, seed {seed}");
                st.sort_by(f64::total_cmp);
                assert_eq!(st, scale, "its own scale, seed {seed}");
                for (&a, &b) in g.spec().steps().iter().zip(s.steps.as_ref().expect("a scale").iter()) {
                    let by = (b - a + 6.0).rem_euclid(12.0) - 6.0;
                    assert!(by.abs() <= if scale.len() == 7 { 1.0 } else { 2.0 }, "{a} to {b}, seed {seed}");
                }
            }
            // Each chord after the first moves a whole step or a minor third
            // (a section's or a cycle's first chord turns back to the tonic,
            // and a long chord's repeat is the same chord).
            for w in slots.windows(2) {
                let by = (w[1].root_midi - w[0].root_midi).abs();
                assert!(by == 0.0 || by == 2.0 || by == 3.0 || offset(&w[1]) == 0.0, "roots {} then {}, seed {seed}", w[0].root_midi, w[1].root_midi);
            }
            for e in g.render().melody.iter().filter(|e| e.vel != 0.0) {
                let s = slot_at(&g, e.step);
                let p = pc(e.midi - tonic);
                assert!(s.steps.as_ref().is_some_and(|st| st.contains(&p)), "the tune's {} over {}, seed {seed}", e.midi, s.root_midi);
                tune += 1;
                on_chord += usize::from(CHORD.iter().any(|&x| pc(s.root_midi - tonic + x) == p));
            }
            for e in g.render().bass.iter().filter(|e| e.vel != 0.0) {
                let s = slot_at(&g, e.step);
                assert!(s.steps.as_ref().is_some_and(|st| st.contains(&pc(e.midi - tonic))), "the bass's {} over {}, seed {seed}", e.midi, s.root_midi);
                bass += 1;
            }
        }
        let share = on_chord as f64 / tune as f64;
        println!("{tune} tune notes, {share:.3} on a chord tone; {bass} bass notes");
        assert!(tune > 1000 && bass > 1000);
        assert!(share > 0.6, "most of the tune on the chord's own tones: {share:.3}");
    });
}

#[test]
fn the_tune_keeps_its_rhythm_and_the_kit_is_untouched() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let ask = |g: &mut Generator, seed: u32, kind: &str| {
            g.load_with(seed, &Request::parse(&format!("progression.kind={kind}")).expect("a request"));
            let r = g.render();
            let tune: Vec<(f64, f64, f64)> = r.melody.iter().map(|e| (e.step, e.dur, e.vel)).collect();
            let kit: Vec<(f64, f64, &str)> = r.drums.iter().map(|e| (e.step, e.vel, e.inst.get())).collect();
            (tune, kit, g.spec().bpm, g.spec().root, g.spec().scale)
        };
        for seed in corpus(60) {
            assert_eq!(ask(&mut g, seed, "today"), ask(&mut g, seed, "planing"), "seed {seed}");
        }
    });
}

#[test]
fn a_pin_holds_either_kind() {
    roomy(|| {
        assert_eq!(Request::parse("progression.kind=planing").err(), None);
        assert_eq!(Request::parse("progression.kind=today").err(), None);
        assert_eq!(Request::parse("progression.kind=on").err(), Some(BAD_SETTING));
        let mut g = Box::new(Generator::new());
        let seeds = corpus(200);
        let drawn = seeds.iter().copied().find(|&s| {
            g.load(s, &[]);
            planes(&g)
        });
        let plain = seeds.iter().copied().find(|&s| {
            g.load(s, &[]);
            !planes(&g)
        });
        let (Some(drawn), Some(plain)) = (drawn, plain) else { panic!("loops of both kinds") };
        g.load_with(drawn, &Request::parse("progression.kind=today").expect("a request"));
        assert!(!planes(&g), "held to today's, seed {drawn}");
        g.load_with(plain, &Request::parse("progression.kind=planing").expect("a request"));
        assert!(planes(&g), "planed, seed {plain}");
    });
}

#[test]
fn a_note_moves_a_semitone_at_most_onto_the_chord() {
    roomy(|| {
        let mut g = Box::new(Generator::new());
        let mut sizes = [false; 8];
        for seed in corpus(40) {
            g.load_with(seed, &Request::parse("progression.kind=planing").expect("a request"));
            let tonic = g.spec().root;
            sizes[g.spec().steps().len()] = true;
            for s in g.render().harmony.slots.iter() {
                let root = pc(s.root_midi - tonic);
                let chord = |q: f64| CHORD.iter().any(|&x| pc(root + x) == q);
                let st = s.steps.as_ref().expect("a planed slot's scale");
                for midi in 48..84 {
                    let m = f64::from(midi);
                    let to = planing::snap(m, tonic, s);
                    let p = pc(m - tonic);
                    assert!(st.contains(&pc(to - tonic)), "{m} to {to}");
                    if st.contains(&p) {
                        assert_eq!(to, m, "a note of the scale stays");
                        continue;
                    }
                    assert_eq!((to - m).abs(), 1.0, "{m} to {to}, seed {seed}");
                    let (down, up) = (pc(p - 1.0), pc(p + 1.0));
                    if st.contains(&down) && st.contains(&up) && chord(down) != chord(up) {
                        assert!(chord(pc(to - tonic)), "{m} onto the chord, not {to}, seed {seed}");
                    }
                }
            }
        }
        assert!(sizes[5] && sizes[7], "modes of five and of seven");
    });
}
