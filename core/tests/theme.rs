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
    let mut bent = Box::new(Generator::new());
    let mut theme = Box::new(Generator::new());
    let mut draws = Rng::new(855);

    // The theme's own seed, nothing else asked: the theme's own loop, every
    // part but the spec (which keeps the theme) the same.
    for _ in 0..40 {
        let seed = draws.seed32();
        theme.load_with(seed, &Request::new());
        bent.load_with(seed, &Request::new().with_theme(seed));
        for part in Part::ALL.into_iter().filter(|&p| p != Part::Spec) {
            assert_eq!(bent.hash(part), theme.hash(part), "seed {seed}: {part:?}");
        }
        assert_eq!(bent.spec().theme, Some(seed));
    }

    // Bent by vibes, on its own seed or another.
    let mut moved = 0;
    for i in 0..200 {
        let theme_seed = draws.seed32();
        let seed = if i % 2 == 1 { theme_seed } else { draws.seed32() };
        let mut request = Request::new().with_theme(theme_seed);
        for _ in 0..=(i % 3) {
            request = request.with((draws.f() * VIBES.len() as f64) as usize);
        }
        theme.load_with(theme_seed, &Request::new());
        bent.load_with(seed, &request);
        let (own, held) = (theme.spec(), bent.spec());
        assert_eq!((held.root, held.scale, held.steps_per_bar), (own.root, own.scale, own.steps_per_bar), "theme {theme_seed} on {seed}: the key, mode and metre");
        assert_eq!(held.mix.len(), own.mix.len());
        for (x, y) in held.mix.iter().zip(own.mix.iter()) {
            assert_eq!(x, y, "theme {theme_seed} on {seed}: the mix");
        }
        assert_eq!(held.metre(&held.character()), own.metre(&own.character()), "theme {theme_seed} on {seed}: the metre");
        let (ma, mb) = (&theme.render().melody_out, &bent.render().melody_out);
        // A theme whose melody rests has no tune to hold: the loop rests too,
        // unless it is one of the rare choir loops, which always sing their
        // own.
        if ma.cell.is_none() && held.choir().is_some() {
            continue;
        }
        assert_eq!(mb.cell, ma.cell, "theme {theme_seed} on {seed}: the cell");
        assert_eq!(mb.motif.len(), ma.motif.len(), "theme {theme_seed} on {seed}: the figure");
        for (x, y) in mb.motif.iter().zip(ma.motif.iter()) {
            assert_eq!((x.offset, x.degree, x.dur, x.vel), (y.offset, y.degree, y.dur, y.vel), "theme {theme_seed} on {seed}: the figure");
        }
        if held.bpm != own.bpm {
            moved += 1;
        }
    }
    assert!(moved > 100, "the vibes still move the tempo: {moved} of 200");
}
