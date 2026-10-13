//! The seeded generator against the hashes both the core and the page must
//! make (`test/draw-vectors.txt`, written by a third implementation). The
//! page's side is `test/draw.test.mjs`.

use dlcore::rng::{Key, bits, unit};

const VECTORS: &str = include_str!("../../test/draw-vectors.txt");

#[test]
fn the_core_makes_the_hashes_the_page_does() {
    let mut checked = 0;
    for line in VECTORS.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
        let w: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(w.len(), 8, "{line}");
        let n = |i: usize| w[i].parse::<u32>().expect("a number");
        let key = Key { seed: n(0), pass: n(1), step: n(2), ord: n(4) };
        let want = u32::from_str_radix(w[7], 16).expect("a hash");
        assert_eq!(bits(key, n(3), n(5), n(6)), want, "{line}");
        // And the draw is the hash's top 24 bits over 2^24.
        assert_eq!(unit(key, n(3), n(5), n(6)), f64::from(want >> 8) / 16_777_216.0);
        checked += 1;
    }
    assert!(checked >= 80, "only {checked} vectors");
}
