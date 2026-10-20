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
fn a_list_reads_back_what_it_says() {
    let text = "dlnotes 2\n# c\nname t\nrate 48000\nquality lite\nframes 256\n\
                param 17 0 0.5 0 0\ndamp 1 0.25\nkey 7 1 2 3\n\
                set 1 3 18 0.098 0.05\nevent 1 1 0.5 0.6\nretune 1 0.05 60 64 67 nan nan nan\n\
                note 25 0 0.1 60 0.4 0.8 0 nan nan nan nan nan nan nan nan nan nan nan nan\n";
    let list = parse(text).expect("a valid list");
    assert_eq!((list.name.as_str(), list.frames, list.full), ("t", 256, false));
    assert_eq!(list.events.len(), 7);
    for bad in ["", "dlnotes 1\n", "dlnotes 2\nrate 48000\nframes 128\nbogus 1\n", "dlnotes 2\nrate 48000\n"] {
        let err = parse(bad).err().expect("an invalid list");
        assert!(err.is_ascii(), "{err}");
    }
}

#[test]
fn an_empty_list_renders_silence_the_same_twice() {
    let list = parse("dlnotes 2\nname quiet\nrate 48000\nquality full\nframes 1024\n").expect("a valid list");
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

/// A golden whose drums' tails once went subnormal (queue item 43): with the
/// processor flushing subnormal numbers to zero, as Chromium's audio
/// threads do, it must render to the same bits as without, because the core
/// never makes one (`dlcore::math`, "Subnormal numbers"). `tools/golden.sh
/// --flush` does every golden.
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[test]
fn flushing_subnormals_changes_nothing() {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/golden");
    let list = parse(&std::fs::read_to_string(format!("{dir}/16-grove.dlnotes")).expect("the golden list")).expect("a valid list");
    let plain = render(&list, None).expect("renders");
    let flushing = notelist::render_flushing(&list, None).expect("renders");
    assert_eq!(plain.hashes, flushing.hashes);
}
