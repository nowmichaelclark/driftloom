//! The sketch build's levers (queue item 83): choices forced, constants
//! moved, the sketch slot. Only with `--features sketch`; the app's build
//! has none of it (`tools/build-core.sh --check` holds the bytes).
#![cfg(feature = "sketch")]

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
    assert!(!tuning::set("no.SUCH", 1.0));
    assert!(!tuning::set("bass.LIFT_DB", f64::NAN));
    assert!((tuning::get("bass.LIFT_DB", 3.0) - 3.0).abs() < f64::EPSILON, "the default until set");
    assert!(tuning::set("bass.LIFT_DB", 5.5));
    assert!((tuning::get("bass.LIFT_DB", 3.0) - 5.5).abs() < f64::EPSILON);
    // Other tests share the process: put it back.
    assert!(tuning::set("bass.LIFT_DB", 3.0));
}
