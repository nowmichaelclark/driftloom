//! The note-list host and the golden renders (queue item 42), as far as a
//! test run on one machine can say: the parser, the hash against published
//! answers, and the committed render of the shortest golden on this
//! machine. `tools/golden.sh` renders every golden on every target.

// The host also writes WAVs, which only the example uses.
#[allow(dead_code)]
#[path = "../examples/dlrender/notelist.rs"]
mod notelist;

use notelist::{Fnv, parse, render};

#[test]
fn the_hash_is_fnv_1a_64() {
    // The published test vectors: the empty string and "a".
    assert_eq!(Fnv::new().hex(), "cbf29ce484222325");
    let mut h = Fnv::new();
    h.bytes(b"a");
    assert_eq!(h.hex(), "af63dc4c8601ec8c");
    // A sample goes in as its four bytes, least significant first: 1.0 is
    // 0x3f800000.
    let (mut a, mut b) = (Fnv::new(), Fnv::new());
    a.samples(&[1.0]);
    b.bytes(&[0x00, 0x00, 0x80, 0x3f]);
    assert_eq!(a.hex(), b.hex());
}

#[test]
fn the_noise_is_the_synths() {
    // The first samples of the synth's noise for seed 1 at 48 kHz, as
    // `Synth._makeNoise` makes them (tools/golden-export.mjs checks the
    // whole buffer against the synth's own on every export).
    let n = notelist::noise(1, 48_000.0);
    assert_eq!(n.len(), 96_000);
    assert_eq!(n[..4], [0.091_103_88, -0.301_661_58, -0.155_117_96, 0.231_918_57]);
}

#[test]
fn a_list_reads_back_what_it_says() {
    let text = "dlnotes 1\n# c\nname t\nrate 48000\nquality lite\nnoise-seed 7\nframes 256\n\
                param 17 0 0.5 0 0\ndamp 1 0.25\ndraws 2 0.5 -0\n\
                note 25 0 0.1 60 0.4 0.8 0 nan nan nan nan nan nan nan nan nan nan nan nan\n";
    let list = parse(text).expect("a valid list");
    assert_eq!((list.name.as_str(), list.frames, list.full, list.noise_seed), ("t", 256, false, 7));
    assert_eq!(list.events.len(), 3);
    for bad in ["", "dlnotes 2\n", "dlnotes 1\nrate 48000\nframes 128\nbogus 1\n", "dlnotes 1\nrate 48000\n"] {
        let err = parse(bad).err().expect("an invalid list");
        assert!(err.is_ascii(), "{err}");
    }
}

#[test]
fn an_empty_list_renders_silence_the_same_twice() {
    let list = parse("dlnotes 1\nname quiet\nrate 48000\nquality full\nnoise-seed 1\nframes 1024\n").expect("a valid list");
    let (a, b) = (render(&list, Some(0)).expect("renders"), render(&list, None).expect("renders"));
    assert_eq!(a.hashes, b.hashes);
    assert_eq!(a.kept.len(), 1024);
}

/// The shortest golden, rendered here, against the hashes kept for this
/// target (`tools/golden.sh` does all of them, on every target). A debug
/// build and a release build must agree to the bit, which this also shows.
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
#[test]
fn the_shortest_golden_matches_its_kept_hashes() {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/golden");
    let name = "14-bloom";
    let list = parse(&std::fs::read_to_string(format!("{dir}/{name}.dlnotes")).expect("the golden list")).expect("a valid list");
    let kept = std::fs::read_to_string(format!("{dir}/x86_64-unknown-linux-gnu.hashes")).expect("the kept hashes");
    let want = kept.lines().find(|l| l.starts_with(&format!("{name} "))).expect("its hash line");
    let rendered = render(&list, None).expect("renders");
    assert_eq!(notelist::line(&list, &rendered.hashes), want);
}
