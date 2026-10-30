//! The Rust generator against the generation goldens (queue item 79): every
//! case of `core/golden/generation.hashes` generated on this machine, each
//! part the generator makes (`Part::ALL`) equal to its hash, and nothing
//! dropped for want of room. `tools/gen-core.sh` runs the same on every
//! target; this is the native test run, and the language rules the port
//! leans on.

use dlcore::generator::{Generator, Part, canon, data, js};

#[test]
fn every_case_makes_its_hashes() {
    // A thread with room for the generator's frames in a debug build.
    std::thread::Builder::new().stack_size(64 << 20).spawn(every_case).expect("a thread").join().expect("every case");
}

fn every_case() {
    let text = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/golden/generation.hashes")).expect("the goldens");
    let mut generator = Box::new(Generator::new());
    let mut cases = 0;
    for line in text.lines().filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let fields: Vec<&str> = line.split(' ').collect();
        let seed: u32 = fields[1].parse().expect("a seed");
        let vibes: Vec<usize> =
            if fields[2] == "-" { Vec::new() } else { fields[2].split(',').map(|v| data::vibe_index(v).expect("a vibe")).collect() };
        generator.load(seed, &vibes);
        for part in Part::ALL {
            let want = fields.iter().find_map(|f| f.strip_prefix(part.name()).and_then(|f| f.strip_prefix('='))).expect("the part's hash");
            assert_eq!(format!("{:016x}", generator.hash(part)), want, "{} {}", fields[0], part.name());
        }
        assert_eq!(generator.lost(), 0, "{}: the lists ran out of room", fields[0]);
        cases += 1;
    }
    assert_eq!(cases, 3060);
}

#[test]
fn numbers_round_as_javascript_does() {
    // Math.round: halves up, and -0 from -0.5 to -0.
    assert_eq!(js::round(2.5), 3.0);
    assert_eq!(js::round(-2.5), -2.0);
    assert!(js::round(-0.5).is_sign_negative() && js::round(-0.5) == 0.0);
    assert!(js::round(-0.2).is_sign_negative());
    assert_eq!(js::round(0.499_999_999_999_999_94), 0.0);
    // Math.max and Math.min order the zeros.
    assert!(js::max(-0.0, 0.0).is_sign_positive());
    assert!(js::min(0.0, -0.0).is_sign_negative());
    assert!(js::max(f64::NAN, 1.0).is_nan());
}

#[test]
fn the_canonical_text_is_the_tools() {
    let text = |f: &dyn Fn(&mut canon::Writer<Text>)| {
        let mut t = Text(String::new());
        f(&mut canon::Writer::new(&mut t));
        t.0
    };
    assert_eq!(text(&|w| w.num(0.5)), "x3fe0000000000000");
    assert_eq!(text(&|w| w.num(-0.0)), "-0");
    assert_eq!(text(&|w| w.num(-12.0)), "-12");
    assert_eq!(text(&|w| w.num(9_007_199_254_740_992.0)), "9007199254740992");
    assert_eq!(text(&|w| w.num(f64::NAN)), "nan");
    assert_eq!(text(&|w| w.str("a\"b\\")), "\"a\\\"b\\\\\"");
    assert_eq!(
        text(&|w| {
            w.obj();
            w.key("a");
            w.nums(&[1.0, 2.0]);
            w.key("b");
            w.obj();
            w.end_obj();
            w.key("c");
            w.null();
            w.end_obj();
        }),
        "{a:[1,2],b:{},c:null}"
    );
    // FNV-1a 64's published vectors.
    let mut h = canon::Fnv::new();
    canon::Sink::bytes(&mut h, b"a");
    assert_eq!(h.0, 0xaf63_dc4c_8601_ec8c);
}

struct Text(String);

impl canon::Sink for Text {
    fn bytes(&mut self, b: &[u8]) {
        self.0.push_str(std::str::from_utf8(b).expect("ASCII"));
    }
}
