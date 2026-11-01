//! The request's theme (queue item 85d; `Request::with_theme`): a seed whose
//! loop the request bends. Its text and refusal, the theme's own seed with
//! nothing else asked making the theme's loop, and a theme bent by vibes,
//! on its own seed or another, keeping its key and mode, its profile mix,
//! its metre and its tune (the melody's figure and rhythmic cell, as the
//! generator plays them; a theme whose melody rests has none to hold, and a
//! choir loop then sings its own). test/generator.test.mjs measures the same in the
//! JavaScript's render; the generation goldens' `theme` cases hold the two
//! together.

// Seeds and degrees are compared exactly on purpose.
#![allow(clippy::float_cmp)]

use dlcore::generator::data::VIBES;
use dlcore::generator::request::{BAD_THEME, Request};
use dlcore::generator::rng::Rng;
use dlcore::generator::{Generator, Part};

#[test]
fn the_text_and_the_refusal() {
    for text in ["theme", "theme=x", "theme=4294967296", "theme=-1", "theme=1.5"] {
        assert_eq!(Request::parse(text).err(), Some(BAD_THEME), "{text}");
    }
    assert_eq!(Request::parse("theme=4294967295").map(|r| r.theme()).ok(), Some(Some(u32::MAX)));
    assert_eq!(Request::parse("cozy,bpm=80").map(|r| r.theme()).ok(), Some(None));
    assert_eq!(Request::new().with_theme(12).theme(), Some(12));
}

#[test]
fn a_theme_holds_its_key_mix_metre_and_tune() {
    std::thread::Builder::new().stack_size(64 << 20).spawn(held).expect("a thread").join().expect("held");
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)] // a vibe's number, under 100
fn held() {
    let mut g = Box::new(Generator::new());
    let mut theme = Box::new(Generator::new());
    let mut r = Rng::new(855);

    // The theme's own seed, nothing else asked: the theme's own loop, every
    // part but the spec (which keeps the theme) the same.
    for _ in 0..40 {
        let seed = r.seed32();
        theme.load_with(seed, &Request::new());
        g.load_with(seed, &Request::new().with_theme(seed));
        for part in Part::ALL.into_iter().filter(|&p| p != Part::Spec) {
            assert_eq!(g.hash(part), theme.hash(part), "seed {seed}: {part:?}");
        }
        assert_eq!(g.spec().theme, Some(seed));
    }

    // Bent by vibes, on its own seed or another.
    let mut moved = 0;
    for i in 0..200 {
        let t = r.seed32();
        let seed = if i % 2 == 1 { t } else { r.seed32() };
        let mut request = Request::new().with_theme(t);
        for _ in 0..=(i % 3) {
            request = request.with((r.f() * VIBES.len() as f64) as usize);
        }
        theme.load_with(t, &Request::new());
        g.load_with(seed, &request);
        let (a, b) = (theme.spec(), g.spec());
        assert_eq!((b.root, b.scale, b.steps_per_bar), (a.root, a.scale, a.steps_per_bar), "theme {t} on {seed}: the key, mode and metre");
        assert_eq!(b.mix.len(), a.mix.len());
        for (x, y) in b.mix.iter().zip(a.mix.iter()) {
            assert_eq!(x, y, "theme {t} on {seed}: the mix");
        }
        assert_eq!(b.metre(&b.character()), a.metre(&a.character()), "theme {t} on {seed}: the metre");
        let (ma, mb) = (&theme.render().melody_out, &g.render().melody_out);
        // A theme whose melody rests has no tune to hold: the loop rests too,
        // unless it is one of the rare choir loops, which always sing their
        // own.
        if ma.cell.is_none() && b.choir().is_some() {
            continue;
        }
        assert_eq!(mb.cell, ma.cell, "theme {t} on {seed}: the cell");
        assert_eq!(mb.motif.len(), ma.motif.len(), "theme {t} on {seed}: the figure");
        for (x, y) in mb.motif.iter().zip(ma.motif.iter()) {
            assert_eq!((x.offset, x.degree, x.dur, x.vel), (y.offset, y.degree, y.dur, y.vel), "theme {t} on {seed}: the figure");
        }
        if b.bpm != a.bpm {
            moved += 1;
        }
    }
    assert!(moved > 100, "the vibes still move the tempo: {moved} of 200");
}
