//! Share codes, version 2 (queue item 105; `dlcore::code`): every name a
//! code carries has its id and version (`data/codes.txt`); every part of a
//! request, every length, journey and edit round-trips, text and sound (a
//! walk of the request's whole list, as the pins' test walks them, and a few
//! thousand random requests); every generation golden's case written as a
//! code and read back generates its same hashes; codes from a newer build,
//! or naming what this build lacks, are refused plainly; and corrupt or
//! truncated codes never panic. A name travels with its code in any script
//! and with emoji, capped in bytes, and what a name refuses is refused
//! (session 23). The dummy build's test (`tests/dummy.rs`) holds the
//! version rule: what joins later moves no older code.

use dlcore::code::{self, ALPHABET, Album, BAD_NAME, BAD_PACE, Kind, Loop, MARKS_MOST, NAME_MOST, NEEDS_NEWER, NOT_A_CODE, Name, PACE_MOST, Pace};
use dlcore::fades::Fades;
use dlcore::generator::data::{self, CHARACTERS, CHOICES, Ids, MOODS, VIBES};
use dlcore::generator::request::{self, Journey, LAYER_NAMES, LENGTH_MOST, LayerAsk, Length, PROFILE_ORDER, Pins, Request, VOICE_ORDER};
use dlcore::generator::rng::{Rng, seed_name};
use dlcore::generator::theory::SCALES;
use dlcore::generator::{Generator, Part};
use dlcore::space;

/// Run `f` on a thread with room for the generator's frames.
fn roomy(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new().stack_size(64 << 20).spawn(f).expect("a thread").join().expect("the test");
}

fn write(l: &Loop) -> String {
    let mut out = [0u8; code::LOOP_MOST];
    let n = code::write_loop(l, &mut out).expect("a code");
    String::from_utf8(out[..n].to_vec()).expect("ASCII")
}

/// Every part's hash of the loop `l` generates.
fn hashes(g: &mut Generator, l: &Loop) -> Vec<u64> {
    g.load_with(l.seed, &l.request);
    Part::ALL.iter().map(|&p| g.hash(p)).collect()
}

/// `l` written, read back, written again: the same code, the same mutes and
/// salt, and (where `g` is given) the same loop generated.
fn round_trip(g: Option<&mut Generator>, l: &Loop) -> String {
    let text = write(l);
    assert!(text.bytes().all(|b| ALPHABET.contains(&b)), "{text}: the alphabet only");
    let back = code::read_loop(text.as_bytes()).unwrap_or_else(|e| panic!("{text}: {e}"));
    assert_eq!(write(&back), text, "read and written again");
    assert_eq!((back.seed, back.mutes, back.salt, back.name), (l.seed, l.mutes, l.salt, l.name), "{text}");
    assert_eq!(back.request.version(), data::CODE_NEWEST, "{text}: a code's loop is drawn at its version");
    if let Some(g) = g {
        let want = hashes(g, l);
        assert_eq!(hashes(g, &back), want, "{text}: the loop it plays");
    }
    text
}

#[allow(clippy::large_types_passed_by_value)] // the request's builders' results, as they come
fn asked(r: Result<Request, &'static str>) -> Loop {
    Loop { request: r.expect("a request"), ..Loop::of(20_261_009) }
}

// -------------------------------------------------------------- the ids

/// `names` each have one row in `list`, and `list` names nothing else.
fn covers(what: &str, list: Ids, names: &[&str]) {
    for n in names {
        assert!(list.iter().any(|e| e.0 == *n), "{what}: '{n}' has no row in data/codes.txt");
    }
    for e in list {
        assert!(names.contains(&e.0), "{what}: data/codes.txt names '{}', which is none", e.0);
    }
}

#[test]
fn every_member_has_an_id_and_a_version() {
    covers("vibes", data::CODE_VIBES, &VIBES.iter().map(|v| v.id).collect::<Vec<_>>());
    covers("profiles", data::CODE_PROFILES, &CHARACTERS.iter().map(|c| c.id).collect::<Vec<_>>());
    for p in PROFILE_ORDER {
        assert!(data::CODE_PROFILES.iter().any(|e| e.0 == p), "{p}");
    }
    covers("moods", data::CODE_MOODS, &MOODS.iter().map(|m| m.id).collect::<Vec<_>>());
    covers("modes", data::CODE_SCALES, &SCALES.iter().map(|s| s.id).collect::<Vec<_>>());
    covers("layers", data::CODE_LAYERS, &LAYER_NAMES);
    covers("journeys", data::CODE_JOURNEYS, &[Journey::Arc.name(), Journey::Wander.name()]);
    covers("tones", data::CODE_TONES, &request::TONE_NAMES);
    covers("choices", data::CODE_CHOICES, &CHOICES.iter().map(|c| c.id).collect::<Vec<_>>());
    for c in CHOICES {
        let list = data::CODE_OPTIONS.iter().find(|e| e.0 == c.id).map_or_else(|| panic!("{}: no [options]", c.id), |e| e.1);
        covers(c.id, list, &c.options.iter().map(|o| o.0).collect::<Vec<_>>());
    }
    assert_eq!(data::CODE_OPTIONS.len(), CHOICES.len(), "an [options] block for no choice");
    // The settings a pin names, and each setting's words.
    let mut settings = Vec::new();
    let mut worded = Vec::new();
    for number in request::CHOICE_PINS..request::PINS_MAX {
        let Some((prefix, name)) = Pins::name(number) else { continue };
        let full: &'static str = Box::leak(format!("{prefix}{name}").into_boxed_str());
        settings.push(full);
        if let Some(words) = Pins::words(number) {
            let list = data::CODE_WORDS.iter().find(|e| e.0 == full).map_or_else(|| panic!("{full}: no [words]"), |e| e.1);
            covers(full, list, &words.iter().map(|w| w.0).collect::<Vec<_>>());
            worded.push(full);
        }
    }
    covers("settings", data::CODE_SETTINGS, &settings);
    assert_eq!(data::CODE_WORDS.len(), worded.len(), "a [words] block for no setting of words");
    assert_eq!(settings.len(), request::SETTINGS.len() + space::n::COUNT);
    // Each layer's voices: everything a request can ask, and every name a
    // profile's pool draws from.
    let pools: [fn(&data::Character) -> Option<data::Pool>; 5] = [|c| c.kits, |c| c.bass_voices, |c| c.chord_voices, |c| c.melody_voices, |c| c.textures];
    for (layer, list) in data::CODE_VOICES.iter().enumerate() {
        for v in VOICE_ORDER[layer] {
            assert!(list.iter().any(|e| e.0 == *v), "{}: '{v}' has no row", LAYER_NAMES[layer]);
        }
        for c in &CHARACTERS {
            for (name, _) in pools[layer](c).unwrap_or(&[]) {
                assert!(list.iter().any(|e| e.0 == *name), "{}: '{name}' ({}) has no row", LAYER_NAMES[layer], c.id);
            }
        }
    }
    for c in &CHARACTERS {
        for (name, _) in c.bass_styles.unwrap_or(&[]) {
            assert!(data::CODE_BASS_STYLES.iter().any(|e| e.0 == *name), "bass style '{name}' has no row");
        }
    }
}

#[test]
fn ids_today_are_the_old_numbers() {
    // The first ids were given in today's orders, so a host that numbered
    // them by place still names the same thing (they are frozen from here).
    for (i, v) in VIBES.iter().enumerate() {
        assert_eq!(data::CODE_VIBES.iter().find(|e| e.0 == v.id).map(|e| e.1), Some(u32::try_from(i).unwrap()), "{}", v.id);
    }
    for (i, c) in CHOICES.iter().enumerate() {
        assert_eq!(data::CODE_CHOICES.iter().find(|e| e.0 == c.id).map(|e| e.1), Some(u32::try_from(i).unwrap()), "{}", c.id);
    }
}

// ------------------------------------------------------- short and plain

#[test]
fn a_loop_with_nothing_asked_is_nine_characters() {
    for seed in [0, 1, 20_261_009, u32::MAX] {
        let text = round_trip(None, &Loop::of(seed));
        assert_eq!(text.len(), 9, "{text}");
        assert_eq!(code::kind(text.as_bytes()), Some(Kind::Loop));
    }
    // A loop with a few picks, as a listener makes one, still under 20.
    let picks = Request::parse("glacial,cozy,bars=64,bpm=72").expect("picks");
    let text = round_trip(None, &Loop { request: picks, ..Loop::of(4_000_000_000) });
    assert!(text.len() < 20, "{text}: {} characters", text.len());
}

#[test]
fn the_charters_lengths() {
    // Promise 11: up to 32 bars, one character; up to 1,024, two; up to
    // 32,768, three (beside the field's own character).
    for (bars, chars) in [(1.0, 1), (32.0, 1), (33.0, 2), (1024.0, 2), (1025.0, 3), (32768.0, 3)] {
        let l = asked(Request::new().with_length(Length::Bars(bars)));
        let text = write(&l);
        assert_eq!(text.len(), 9 + 1 + chars, "{bars} bars: {text}");
    }
    assert_eq!(write(&asked(Request::new().with_length(Length::Endless))).len(), 10);
}

// ------------------------------------------------------- the whole list

#[test]
fn every_part_of_a_request_round_trips() {
    roomy(walk);
}

#[allow(clippy::too_many_lines)] // the request's whole list, item by item
fn walk() {
    let mut g = Box::new(Generator::new());
    let g = &mut *g;
    let mut n = 0;
    let mut check = |g: &mut Generator, l: Loop| {
        round_trip(Some(g), &l);
        n += 1;
    };
    // Every vibe, without a weight and with one.
    for i in 0..data::VIBE_COUNT {
        check(g, asked(Ok(Request::new().with(i))));
        check(g, asked(Request::new().with_weight(i, 0.35)));
    }
    check(g, asked(Request::parse("glacial=0,cozy=2.5,misty=0.123456789")));
    // The tempo, pinned and a range, at both ends.
    for (lo, hi) in [(32.0, 32.0), (190.0, 190.0), (70.0, 85.0), (32.0, 190.0)] {
        check(g, asked(Request::new().with_tempo(lo, hi)));
    }
    // Every length kind, and the journeys.
    for bars in [1.0, 4.0, 31.0, 32.0, 33.0, 1024.0, 32_768.0, 3072.0, LENGTH_MOST] {
        check(g, asked(Request::new().with_length(Length::Bars(bars))));
    }
    check(g, asked(Request::new().with_length(Length::Endless)));
    for j in [Journey::Arc, Journey::Wander] {
        check(g, asked(Request::new().with_length(Length::Endless).map(|r| r.with_journey(j))));
        check(g, asked(Ok(Request::new().with_journey(j))));
    }
    // Every key, every mode, every style, alone and all at once.
    for k in 0..12 {
        check(g, asked(Request::new().with_keys(1 << k)));
    }
    check(g, asked(Request::new().with_keys(request::KEYS_ALL)));
    for s in 0..SCALES.len() {
        check(g, asked(Request::new().with_scales(1 << s)));
    }
    check(g, asked(Request::new().with_scales(request::SCALES_ALL)));
    for s in 0..PROFILE_ORDER.len() {
        check(g, asked(Request::new().with_styles(1 << s)));
    }
    check(g, asked(Request::new().with_styles(request::STYLES_ALL)));
    // Every layer's every voice, and each layer on and off.
    for (layer, voices) in VOICE_ORDER.iter().enumerate() {
        for v in 0..voices.len() {
            check(g, asked(Request::new().with_voices(layer, 1 << v)));
        }
        check(g, asked(Request::new().with_voices(layer, (1 << voices.len()) - 1)));
        for ask in [LayerAsk::On, LayerAsk::Off] {
            check(g, asked(Request::new().with_layer(layer, ask)));
        }
    }
    check(g, asked(Request::parse("layer.drums=on,layer.bass=off,layer.melody=on,layer.texture=off")));
    // A theme.
    check(g, asked(Ok(Request::new().with_theme(123_456_789))));
    check(g, asked(Ok(Request::new().with_theme(0))));
    // Every pin: each choice at each option, each setting at each word, or
    // at both ends of its range and between, or at each number it takes.
    for number in 0..request::PINS_MAX {
        if Pins::name(number).is_none() {
            continue;
        }
        if let Some(words) = Pins::words(number) {
            for w in 0..words.len() {
                #[allow(clippy::cast_precision_loss)] // a word's place
                check(g, asked(Request::new().with_pin(number, w as f64)));
            }
        } else if let Some(values) = Pins::values(number) {
            for &(x, _) in values {
                check(g, asked(Request::new().with_pin(number, x)));
            }
        } else if let Some(([lo, hi], whole)) = Pins::range(number) {
            let mid = if whole { lo.midpoint(hi).floor() } else { lo + (hi - lo) * 0.37 };
            for x in [lo, hi, mid] {
                check(g, asked(Request::new().with_pin(number, x)));
            }
        } else {
            panic!("pin {number} takes nothing");
        }
    }
    // The edits: every layer's seed, set and re-rolled; every part of the
    // tone; the tempo the slider set; then the listener's mutes and the
    // wander's salt, which a code carries beside the request.
    for layer in 0..5 {
        for rolled in [false, true] {
            check(g, asked(Request::new().with_layer_seed(layer, 0xdead_beef ^ u32::try_from(layer).unwrap(), rolled)));
        }
    }
    for k in 0..3 {
        for x in [0.0, 0.6, 1.0, 0.123_456_789_012_345_67] {
            check(g, asked(Request::new().with_tone(k, x)));
        }
    }
    for bpm in [32.0, 88.0, 101.25, 190.0] {
        check(g, asked(Request::new().with_bpm(bpm)));
    }
    for mutes in [1u8, 2, 4, 8, 16, 31] {
        check(g, Loop { mutes, ..Loop::of(77) });
    }
    check(g, Loop { salt: 1, ..Loop::of(77) });
    check(g, Loop { salt: u32::MAX, ..Loop::of(77) });
    // A pin of minus zero keeps its sign (as the request's text does).
    let neg = Request::parse("tuning.cents=-0").expect("a pin");
    let back = code::read_loop(write(&asked(Ok(neg))).as_bytes()).expect("a code");
    assert!(back.request.pins().get("tuning.cents").is_some_and(|x| x == 0.0 && x.is_sign_negative()));
    assert!(n > 500, "{n} requests walked");
}

#[test]
fn a_pin_to_today_frees_a_change_and_its_code_plays_it() {
    // Batch 6's draws count against a loop's changes (Mikey, 2026-11-17):
    // pinning one to today frees a change the list fills with another.
    // wen-glui (corpus 20261009 #50) with its tuning held at A440 gains the
    // arc; its code must play that, not only read back the same text.
    roomy(|| {
        let mut r = Rng::new(20_261_009);
        let seed = (0..200).map(|_| r.seed32()).find(|&s| seed_name(s).as_str() == "wen-glui").expect("wen-glui in the corpus");
        let mut g = Box::new(Generator::new());
        g.load_with(seed, &Request::new());
        let before = g.render().choice("arc");
        let l = asked(Request::parse("tuning.cents=0"));
        let l = Loop { seed, ..l };
        round_trip(Some(&mut g), &l);
        let back = code::read_loop(write(&l).as_bytes()).expect("a code");
        g.load_with(seed, &back.request);
        assert_eq!(g.render().choice("arc"), Some("on"), "the freed change");
        assert_ne!(before, Some("on"), "wen-glui has no arc unpinned");
    });
}

// --------------------------------------------------------------- the fuzz

/// A random request, as a listener's picks and edits could make one.
#[allow(clippy::too_many_lines, clippy::many_single_char_names)] // one part of the request after another, each on its stream
fn random_loop(r: &mut Rng) -> Loop {
    let mut q = Request::new();
    let mut d = Rng::new(r.seed32());
    let mut u = Rng::new(r.seed32());
    let mut maybe = |q: &mut Request, p: f64, f: &mut dyn FnMut(Request) -> Result<Request, &'static str>| {
        if r.f() < p
            && let Ok(x) = f(*q)
        {
            *q = x;
        }
    };
    let mut pick = move |n: usize| d.pick_index(n);
    for _ in 0..pick(5) {
        let i = pick(data::VIBE_COUNT);
        let w = if u.f() < 0.5 { None } else { Some((u.f() * 3.0 * 100.0).round() / 100.0) };
        maybe(&mut q, 1.0, &mut |x| match w {
            None => Ok(x.with(i)),
            Some(w) => x.with_weight(i, w),
        });
    }
    let (lo, span) = (32.0 + js_floor(u.f() * 158.0), js_floor(u.f() * 30.0));
    maybe(&mut q, 0.3, &mut |x| x.with_tempo(lo, (lo + span).min(190.0)));
    let bars = [1.0, 4.0, 16.0, 64.0, 512.0, 3072.0, 32_768.0][pick(7)];
    let endless = u.f() < 0.3;
    maybe(&mut q, 0.3, &mut |x| x.with_length(if endless { Length::Endless } else { Length::Bars(bars) }));
    let journey = if u.f() < 0.5 { Journey::Arc } else { Journey::Wander };
    maybe(&mut q, 0.2, &mut |x| Ok(x.with_journey(journey)));
    let keys = (u.seed32() & 0xfff) | 1;
    maybe(&mut q, 0.2, &mut |x| x.with_keys(keys));
    let scales = (u.seed32() & request::SCALES_ALL) | 1;
    maybe(&mut q, 0.2, &mut |x| x.with_scales(scales));
    let styles = 1 << pick(PROFILE_ORDER.len());
    maybe(&mut q, 0.2, &mut |x| x.with_styles(styles));
    for (layer, voices) in VOICE_ORDER.iter().enumerate() {
        let bits = 1 << pick(voices.len());
        maybe(&mut q, 0.1, &mut |x| x.with_voices(layer, bits));
        let ask = if u.f() < 0.5 { LayerAsk::On } else { LayerAsk::Off };
        maybe(&mut q, 0.1, &mut |x| x.with_layer(layer, ask));
    }
    let theme = u.seed32();
    maybe(&mut q, 0.1, &mut |x| Ok(x.with_theme(theme)));
    for _ in 0..pick(4) {
        let number = pick(request::PINS_MAX);
        let value = if let Some(w) = Pins::words(number) {
            #[allow(clippy::cast_precision_loss)] // a word's place
            let v = pick(w.len()) as f64;
            v
        } else if let Some(v) = Pins::values(number) {
            v[pick(v.len())].0
        } else if let Some(([lo, hi], whole)) = Pins::range(number) {
            let x = lo + u.f() * (hi - lo);
            if whole { js_floor(x).max(lo) } else { x }
        } else {
            continue;
        };
        maybe(&mut q, 1.0, &mut |x| x.with_pin(number, value));
    }
    for layer in 0..5 {
        let (seed, rolled) = (u.seed32(), u.f() < 0.5);
        maybe(&mut q, 0.05, &mut |x| x.with_layer_seed(layer, seed, rolled));
    }
    for k in 0..3 {
        let t = u.f();
        maybe(&mut q, 0.1, &mut |x| x.with_tone(k, t));
    }
    let bpm = 32.0 + u.f() * 158.0;
    maybe(&mut q, 0.1, &mut |x| x.with_bpm(bpm));
    let mutes = if u.f() < 0.1 { u8::try_from(u.seed32() & 31).unwrap() } else { 0 };
    let salt = if u.f() < 0.1 { u.seed32() | 1 } else { 0 };
    let name = if u.f() < 0.1 { Name::new(NAMES[pick(NAMES.len())].as_bytes()).expect("a name") } else { Name::NONE };
    Loop { seed: u.seed32(), request: q, mutes, salt, name }
}

fn js_floor(x: f64) -> f64 {
    x.floor()
}

#[test]
fn a_few_thousand_random_requests_round_trip() {
    roomy(|| {
        let mut r = Rng::new(105);
        let mut g = Box::new(Generator::new());
        let mut longest = 0;
        for i in 0..4000 {
            let l = random_loop(&mut r);
            // Every request's text; every fourth one's sound as well.
            let text = round_trip(if i % 4 == 0 { Some(&mut g) } else { None }, &l);
            longest = longest.max(text.len());
        }
        assert!(longest < 200, "the longest random code: {longest}");
    });
}

// ------------------------------------------------------------ the goldens

#[test]
fn every_golden_case_as_a_code_makes_its_hashes() {
    roomy(|| {
        let text = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/golden/generation.hashes")).expect("the goldens");
        let mut g = Box::new(Generator::new());
        let mut cases = 0;
        for line in text.lines().filter(|l| !l.is_empty() && !l.starts_with('#')) {
            let fields: Vec<&str> = line.split(' ').collect();
            let seed: u32 = fields[1].parse().expect("a seed");
            let request = Request::parse(fields[2]).expect("a request");
            let code = write(&Loop { request, ..Loop::of(seed) });
            let back = code::read_loop(code.as_bytes()).unwrap_or_else(|e| panic!("{}: {code}: {e}", fields[0]));
            g.load_with(back.seed, &back.request);
            for part in Part::ALL {
                let want = part.in_line(line).expect("the part's hash");
                assert_eq!(format!("{:016x}", g.hash(part)), want, "{} {code} {part:?}", fields[0]);
            }
            cases += 1;
        }
        assert_eq!(cases, 5139);
    });
}

// ------------------------------------------------------------- the albums

/// Each track's code, playing on and on (no pace set).
fn on(tracks: &[String]) -> impl Iterator<Item = (&[u8], Pace)> + Clone {
    tracks.iter().map(|t| (t.as_bytes(), Pace::On))
}

fn album_tracks(album: &str) -> Vec<String> {
    let a = code::read_album(album.as_bytes()).expect("an album");
    (0..a.count)
        .map(|i| {
            let mut out = [0u8; code::LOOP_MOST];
            let n = code::album_track(album.as_bytes(), i, &mut out).expect("a track");
            String::from_utf8(out[..n].to_vec()).expect("ASCII")
        })
        .collect()
}

#[test]
fn an_album_of_loops_holds_each_as_it_was() {
    let mut r = Rng::new(9);
    let tracks: Vec<String> = (0..12).map(|_| write(&random_loop(&mut r))).collect();
    let title = name("ฝนตกวันอาทิตย์ side A 🌧️");
    let mut out = vec![0u8; 8192];
    let n = code::write_album(on(&tracks), &title, &mut out).expect("an album");
    let album = String::from_utf8(out[..n].to_vec()).expect("ASCII");
    assert_eq!(code::kind(album.as_bytes()), Some(Kind::Album));
    let a = code::read_album(album.as_bytes()).expect("an album");
    assert_eq!((a.count, a.seed, a.name), (12, None, title));
    assert_eq!(album_tracks(&album), tracks);
    // Eight plain loops: about 60 characters (promise 8).
    let plain: Vec<String> = (0..8).map(|s| write(&Loop::of(s * 1_000_003))).collect();
    let n = code::write_album(on(&plain), &Name::NONE, &mut out).expect("an album");
    assert!(n <= 64, "eight plain loops: {n} characters");
    assert_eq!(album_tracks(core::str::from_utf8(&out[..n]).unwrap()), plain);
    // A loop's code is no album's, nor an album's a loop's.
    assert_eq!(code::read_album(tracks[0].as_bytes()).err(), Some(NOT_A_CODE));
    assert_eq!(code::read_loop(album.as_bytes()).err(), Some(NOT_A_CODE));
    assert_eq!(code::write_album(core::iter::empty(), &Name::NONE, &mut out).err(), Some(code::BAD_ALBUM));
}

#[test]
fn an_album_from_one_seed_is_one_short_code() {
    let picks = Request::parse("glacial,cozy,bars=3072").expect("picks");
    let mut out = [0u8; 256];
    let n = code::write_seed_album(424_242, 12, &picks, &Name::NONE, &mut out).expect("an album");
    let album = String::from_utf8(out[..n].to_vec()).expect("ASCII");
    // A 24-hour, 12-track album in under 20 characters (Painter, item 4).
    assert!(album.len() < 20, "{album}");
    let a = code::read_album(album.as_bytes()).expect("an album");
    assert_eq!((a.count, a.seed), (12, Some(424_242)));
    let tracks = album_tracks(&album);
    for (i, t) in tracks.iter().enumerate() {
        let l = code::read_loop(t.as_bytes()).expect("a track");
        assert_eq!(l.seed, code::track_seed(424_242, i));
        assert_eq!(write(&Loop { request: picks, ..Loop::of(l.seed) }), *t, "track {i} shares the picks");
    }
    let distinct: std::collections::BTreeSet<_> = tracks.iter().collect();
    assert_eq!(distinct.len(), 12);
    // A name, and the edits left out (an album shares picks, not edits);
    // the album's name is no track's.
    let edited = picks.with_bpm(99.0).expect("an edit");
    let title = name("Night drive 🌙");
    let n = code::write_seed_album(7, 3, &edited, &title, &mut out).expect("an album");
    let a = code::read_album(&out[..n]).expect("an album");
    assert_eq!(a, Album { count: 3, version: data::CODE_NEWEST, seed: Some(7), name: title });
    let first = code::read_loop(album_tracks(core::str::from_utf8(&out[..n]).unwrap())[0].as_bytes()).expect("a track");
    assert!(first.request.edits().bpm().is_none());
    assert!(first.name.is_none());
}

// ---------------------------------------------------- an album's pacing

/// Each track's pace, as the album's maker set it.
fn album_paces(album: &str) -> Vec<Pace> {
    let a = code::read_album(album.as_bytes()).expect("an album");
    (0..a.count).map(|i| code::album_pace(album.as_bytes(), i).expect("a pace")).collect()
}

/// The album of `tracks`, each with its pace.
fn paced_album(tracks: &[String], paces: &[Pace], title: &Name) -> String {
    let mut out = vec![0u8; 1 << 16];
    let n = code::write_album(tracks.iter().map(String::as_bytes).zip(paces.iter().copied()), title, &mut out).expect("an album");
    String::from_utf8(out[..n].to_vec()).expect("ASCII")
}

/// `paces` on `tracks`: written, read back to the same tracks and paces,
/// and written again to the same code. Its code.
fn pace_round_trip(tracks: &[String], paces: &[Pace], title: &Name) -> String {
    let album = paced_album(tracks, paces, title);
    assert!(album.bytes().all(|b| ALPHABET.contains(&b)), "{album}: the alphabet only");
    assert_eq!(album_tracks(&album), tracks, "{album}: its tracks");
    assert_eq!(album_paces(&album), paces, "{album}: its paces");
    assert_eq!(code::read_album(album.as_bytes()).expect("an album").name, *title);
    let back: Vec<String> = album_tracks(&album);
    assert_eq!(paced_album(&back, &album_paces(&album), title), album, "read and written again");
    album
}

/// A time's pace, `seconds` with the fades `f` (bit 1 the fade in, bit 2 the
/// fade out).
fn time(seconds: u32, f: u32) -> Pace {
    Pace::Time { seconds, fades: Fades { fade_in: f & 1 != 0, fade_out: f & 2 != 0 } }
}

#[test]
fn every_pace_round_trips() {
    // Every time a listener's Track length view gives, to the second (to a
    // day: session 30c), with each pair of fades, and on and on with and
    // without a fade in; on a track alone and beside tracks of another pace
    // (where it writes its own) and of the same (where the album writes it
    // once).
    let mut r = Rng::new(24);
    let tracks: Vec<String> = (0..3).map(|_| write(&random_loop(&mut r))).collect();
    let mut counts: Vec<u32> = (1..=1200).collect();
    counts.extend([3599, 3600, 5025, 7207, 43_200, PACE_MOST - 1, PACE_MOST]);
    let mut walked = 0;
    for count in counts {
        for f in 0..4 {
            let pace = time(count, f);
            assert_eq!(Pace::of(pace.kind(), pace.count()), Some(pace));
            pace_round_trip(&tracks[..1], &[pace], &Name::NONE);
            pace_round_trip(&tracks, &[pace, Pace::On, Pace::On], &Name::NONE);
            pace_round_trip(&tracks, &[pace, pace, time(3, 2)], &name("Side B"));
            walked += 1;
        }
    }
    // And on and on: "until moved on", with or without its fade in.
    assert_eq!(Pace::of(0, 0), Some(Pace::On));
    assert_eq!(Pace::of(4, 0), Some(Pace::OnFadingIn));
    pace_round_trip(&tracks, &[Pace::On, time(120, 2), time(120, 2)], &Name::NONE);
    pace_round_trip(&tracks, &[Pace::OnFadingIn, Pace::OnFadingIn, Pace::On], &Name::NONE);
    pace_round_trip(&tracks, &[Pace::On; 3], &Name::NONE);
    assert!(walked > 4000);
    // What no view gives is no pace: refused as it is written (whole passes
    // and bars among them since session 30c, and a fade out with no end).
    for (kind, count) in [(1, 2), (2, 8), (0, 1), (3, 0), (8, 0), (3, PACE_MOST + 1), (16 | 3, 5), (3, u32::MAX)] {
        assert_eq!(Pace::of(kind, count), None, "{kind} {count}");
    }
    let mut out = [0u8; 256];
    for bad in [time(0, 0), time(PACE_MOST + 1, 2)] {
        assert_eq!(code::write_album(tracks.iter().take(1).map(|t| (t.as_bytes(), bad)), &Name::NONE, &mut out).err(), Some(BAD_PACE), "{bad:?}");
    }
    // As a host writes them in text.
    let t = tracks[0].as_bytes();
    for (text, pace) in [("", Pace::On), (":i", Pace::OnFadingIn), (":t5025", time(5025, 0)), (":t1i", time(1, 1)), (":t7207o", time(7207, 2)), (":t86400io", time(86_400, 3))] {
        let whole = [t, text.as_bytes()].concat();
        assert_eq!(code::paced(&whole), Ok((t, pace)), "{text}");
    }
    for text in [":p2", ":b8", ":t0", ":t05", ":t86401", ":t", ":toi", ":t5x", ":o"] {
        let whole = [t, text.as_bytes()].concat();
        assert_eq!(code::paced(&whole).err(), Some(BAD_PACE), "{text}");
    }
}

#[test]
fn an_album_code_from_before_times_still_reads() {
    // Session 30c (Mikey, 2026-11-18: "it's okay if those codes aren't
    // exact"): a pace was whole passes or bars before, and its numbers now
    // read as times of a few seconds; every such album still reads, its
    // tracks as they were. The fixtures as they stood (session 24b), and
    // every album code in docs/ALBUMS.md.
    let old = [
        ("FADGRiSNBAGAAAAAAI_____DWSWDYD", 3),
        ("FADNNBAAAABuBCbOBMNBAAAAEmBOWAJNBAAAAGHBWHeEUmFpbgMd", 3),
        ("FACPNBAAAAF__CHAWgBONBAAAAICCEJCAAWBQD", 2),
    ];
    // (The old fixture of 72,000 passes and 2^31 - 1 bars, past any time a
    // pace carries now, is no code: no slider ever set it.)
    assert_eq!(code::read_album(b"FACGAAAAAAO_____DW-_____DW_zsExI").err(), Some(NOT_A_CODE));
    for (album, count) in old {
        let a = code::read_album(album.as_bytes()).unwrap_or_else(|e| panic!("{album}: {e}"));
        assert_eq!(a.count, count);
        let _ = album_tracks(album);
        let _ = album_paces(album);
    }
    let doc = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../docs/ALBUMS.md")).expect("the albums");
    let mut read = 0;
    for code in doc.split("#c=").skip(1).map(|r| r.split(|c: char| !(c.is_ascii_alphanumeric() || c == '-' || c == '_')).next().unwrap_or("")) {
        if code.starts_with('F') {
            assert!(code::read_album(code.as_bytes()).is_ok(), "{code}");
            let _ = album_paces(code);
            read += 1;
        }
    }
    assert!(read >= 3, "the albums in docs/ALBUMS.md");
}

#[test]
fn a_few_thousand_random_albums_round_trip() {
    roomy(|| {
        let mut r = Rng::new(2024);
        let pool: Vec<String> = (0..64).map(|_| write(&random_loop(&mut r))).collect();
        let mut most_over = 0;
        for i in 0..3000 {
            let count = 1 + r.pick_index(if i % 50 == 0 { 300 } else { 16 });
            let tracks: Vec<String> = (0..count).map(|_| pool[r.pick_index(pool.len())].clone()).collect();
            // Paces as albums have them: most tracks one, a few another,
            // some on and on, now and then every track its own.
            let common = [Pace::On, time(120, 2), time(1 + u32::try_from(r.pick_index(3600)).unwrap(), 2), time(1 + u32::try_from(r.pick_index(600)).unwrap(), u32::try_from(r.pick_index(4)).unwrap())][r.pick_index(4)];
            let wild = r.f() < 0.1;
            let paces: Vec<Pace> = (0..count)
                .map(|_| {
                    let most = if r.f() < 0.9 { 900 } else { 86_400 };
                    let n = 1 + u32::try_from(r.pick_index(most)).unwrap();
                    match r.pick_index(if wild { 3 } else { 8 }) {
                        0 => Pace::On,
                        1 => time(n, u32::try_from(r.pick_index(4)).unwrap()),
                        2 => Pace::OnFadingIn,
                        _ => common,
                    }
                })
                .collect();
            let title = if r.f() < 0.2 { name(NAMES[r.pick_index(NAMES.len())]) } else { Name::NONE };
            let album = pace_round_trip(&tracks, &paces, &title);
            let plain = paced_album(&tracks, &vec![Pace::On; count], &title);
            // A pace costs nothing where every track plays on; else what
            // its pace and those apart from it take.
            if paces.iter().all(|&p| p == Pace::On) {
                assert_eq!(album, plain);
            }
            most_over = most_over.max(album.len() - plain.len());
        }
        assert!(most_over > 0);
    });
}

#[test]
fn an_album_without_pacing_is_its_code_as_before() {
    // The format's fixtures (written before paces) hold it to the byte; and
    // here each of them written again with every track on and on.
    let text = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/golden/share-codes.txt")).expect("the fixtures");
    let mut held = 0;
    for line in text.lines().filter(|l| l.starts_with("album ")) {
        let f: Vec<&str> = line.split(' ').collect();
        if f[2].contains(':') {
            continue;
        }
        let tracks: Vec<String> = f[2].split(',').map(String::from).collect();
        let title = if f[1] == "-" { Name::NONE } else { name(&unescape(f[1])) };
        assert_eq!(paced_album(&tracks, &vec![Pace::On; tracks.len()], &title), f[3], "{line}");
        assert_eq!(album_paces(f[3]), vec![Pace::On; tracks.len()]);
        held += 1;
    }
    assert!(held >= 3);
}

#[test]
fn twelve_tracks_of_two_minutes_cost_a_few_characters() {
    let mut r = Rng::new(12);
    let tracks: Vec<String> = (0..12).map(|_| write(&random_loop(&mut r))).collect();
    let plain = paced_album(&tracks, &[Pace::On; 12], &Name::NONE);
    let two = pace_round_trip(&tracks, &[time(120, 2); 12], &Name::NONE);
    // Written once, for the album: its tag and its number.
    assert_eq!(two.len() - plain.len(), 3, "{plain} {two}");
    // Under about 20 longer however the view was set on each (the brief's
    // yardstick), with a pace shared: twelve tracks of two minutes but one
    // of 45 seconds and one of half an hour.
    let mut mixed = [time(120, 2); 12];
    mixed[3] = time(45, 0);
    mixed[7] = time(1800, 2);
    let some = pace_round_trip(&tracks, &mixed, &Name::NONE);
    assert!(some.len() - plain.len() <= 12, "{}", some.len() - plain.len());
    // And each track its own time, to a day: still a few each.
    let own: Vec<Pace> = (1..=12).map(|i| time(i * 7200, 3)).collect();
    let each = pace_round_trip(&tracks, &own, &Name::NONE);
    assert!(each.len() - plain.len() <= 12 * 6, "{}", each.len() - plain.len());
}

#[test]
fn an_album_has_one_code_for_its_paces() {
    let tracks: Vec<String> = (0..3).map(|s| write(&Loop::of(s * 7))).collect();
    let bodies: Vec<String> = tracks.iter().map(|t| t[1..t.len() - 2].to_string()).collect();
    let head = |n: usize| format!("{}{}{}", uv(5), uv(0), uv(n as u64));
    let track = |b: &str, extra: &str| format!("{}{b}{extra}", uv((b.len() + extra.len()) as u64));
    let pace = |p: u64| format!("{}{}", uv(22), uv(p));
    // What the writer writes reads: two minutes fading out shared (480), a
    // track's own nine seconds (34).
    let good = sealed(&format!("{}{}{}{}{}", head(3), track(&bodies[0], ""), track(&bodies[1], &pace(34)), track(&bodies[2], ""), pace(480)));
    assert_eq!(good, paced_album(&tracks, &[time(120, 2), time(9, 0), time(120, 2)], &Name::NONE));
    assert_eq!(album_paces(&good), [time(120, 2), time(9, 0), time(120, 2)]);
    // Anything else saying the same is no code's: on and on written for the
    // album; a track's own pace that is the album's; the album's not the one
    // most tracks share; a pace twice, or after the album's name; a pace in
    // a loop's code, or in an album from one seed.
    let refused = [
        format!("{}{}{}{}{}", head(3), track(&bodies[0], ""), track(&bodies[1], ""), track(&bodies[2], ""), pace(0)),
        format!("{}{}{}{}", head(3), track(&bodies[0], &pace(0)), track(&bodies[1], ""), track(&bodies[2], "")),
        format!("{}{}{}{}{}", head(3), track(&bodies[0], &pace(480)), track(&bodies[1], &pace(34)), track(&bodies[2], ""), pace(480)),
        format!("{}{}{}{}{}", head(3), track(&bodies[0], &pace(34)), track(&bodies[1], &pace(34)), track(&bodies[2], ""), pace(480)),
        format!("{}{}{}{}", head(3), track(&bodies[0], &pace(480)), track(&bodies[1], &pace(480)), track(&bodies[2], "")),
        format!("{}{}{}{}{}{}", head(3), track(&bodies[0], ""), track(&bodies[1], ""), track(&bodies[2], ""), pace(480), pace(480)),
        format!("{}{}{}{}{}{}", head(3), track(&bodies[0], ""), track(&bodies[1], ""), track(&bodies[2], ""), name_field(b"x"), pace(480)),
        format!("{}{}{}{}", head(3), track(&bodies[0], &format!("{}{}", pace(480), pace(5))), track(&bodies[1], ""), track(&bodies[2], "")),
    ];
    for (i, chars) in refused.iter().enumerate() {
        assert_eq!(code::read_album(sealed(chars).as_bytes()).err(), Some(NOT_A_CODE), "{i}: {chars}");
    }
    assert_eq!(code::read_loop(sealed(&format!("{}{}", &tracks[0][..tracks[0].len() - 2], pace(480))).as_bytes()).err(), Some(NOT_A_CODE));
    let mut out = [0u8; 64];
    let n = code::write_seed_album(3, 4, &Request::new(), &Name::NONE, &mut out).expect("an album");
    let seeded = core::str::from_utf8(&out[..n]).unwrap();
    assert_eq!(album_paces(seeded), [Pace::On; 4]);
    assert_eq!(code::read_album(sealed(&format!("{}{}", &seeded[..seeded.len() - 2], pace(480))).as_bytes()).err(), Some(NOT_A_CODE));
    // Past the most a pace counts: no code.
    let far = sealed(&format!("{}{}{}{}{}", head(3), track(&bodies[0], ""), track(&bodies[1], ""), track(&bodies[2], ""), pace(2 + 4 * u64::from(PACE_MOST))));
    assert_eq!(code::read_album(far.as_bytes()).err(), Some(NOT_A_CODE));
    // A field this build lacks after the tracks is a newer build's.
    assert_eq!(code::read_album(sealed(&format!("{}{}{}{}{}", head(3), track(&bodies[0], ""), track(&bodies[1], ""), track(&bodies[2], ""), uv(25))).as_bytes()).err(), Some(NEEDS_NEWER));
    // A track read from the album is the loop it was, its pace left out.
    assert_eq!(album_tracks(&good), tracks);
}

#[test]
fn a_track_and_its_pace_in_text() {
    assert_eq!(code::paced(b"ENBAAAAEmBO_7"), Ok((&b"ENBAAAAEmBO_7"[..], Pace::On)));
    assert_eq!(code::paced(b"ENBAAAAEmBO_7:t5025o"), Ok((&b"ENBAAAAEmBO_7"[..], time(5025, 2))));
    assert_eq!(code::paced(b"ENBAAAAEmBO_7:i"), Ok((&b"ENBAAAAEmBO_7"[..], Pace::OnFadingIn)));
    assert_eq!(code::paced(format!("X:t{PACE_MOST}io").as_bytes()), Ok((&b"X"[..], time(PACE_MOST, 3))));
    for bad in ["X:", "X:t", "X:t0", "X:t02", "X:x2", "X:t2x", "X:t-1", "X:t 2", "X:t86401", "X:t99999999999", "X:p2", "X:b16", "X:t5oi", "X:io"] {
        assert_eq!(code::paced(bad.as_bytes()).err(), Some(BAD_PACE), "{bad}");
    }
}

// ------------------------------------------------------------- refusals

/// A code's characters (without its check) closed with the right check.
fn sealed(chars: &str) -> String {
    let h = chars.bytes().fold(0u32, |h, c| (h * 64 + u32::try_from(ALPHABET.iter().position(|&a| a == c).unwrap()).unwrap() + 1) % 4093);
    let mut s = chars.to_string();
    s.push(ALPHABET[(h & 63) as usize] as char);
    s.push(ALPHABET[(h >> 6) as usize] as char);
    s
}

/// A whole number as the format writes it.
fn uv(mut x: u64) -> String {
    let mut s = String::new();
    while x >= 32 {
        s.push(ALPHABET[usize::try_from(x & 31).unwrap() | 32] as char);
        x >>= 5;
    }
    s.push(ALPHABET[usize::try_from(x).unwrap()] as char);
    s
}

#[test]
fn a_newer_builds_code_is_refused_plainly() {
    let plain = write(&Loop::of(5));
    let seed = &plain[1..7];
    // A newer version, a loop's and an album's.
    let newer = u64::from(data::CODE_NEWEST) + 1;
    assert_eq!(code::read_loop(sealed(&format!("{}{seed}", uv(newer * 2))).as_bytes()).err(), Some(NEEDS_NEWER));
    assert_eq!(code::read_album(sealed(&format!("{}A{}{}", uv(newer * 2 + 1), uv(1), &plain[..7])).as_bytes()).err(), Some(NEEDS_NEWER));
    assert_eq!(code::read_loop(sealed(&format!("{}{seed}", uv(1 << 40))).as_bytes()).err(), Some(NEEDS_NEWER));
    // An id this build lacks: a vibe, a choice, a setting, a voice, a field.
    let head = &plain[..7];
    for fields in [
        format!("{}{}", uv(1), uv(9999)),
        format!("{}{}{}", uv(15), uv(999), uv(1)),
        format!("{}{}{}", uv(15), uv(0), uv(99)),
        format!("{}{}{}", uv(16), uv(999), uv(0)),
        format!("{}{}{}{}", uv(11), uv(3), uv(1), uv(200)),
        format!("{}{}", uv(7), uv(7)),
        uv(25),
        uv(31),
    ] {
        assert_eq!(code::read_loop(sealed(&format!("{head}{fields}")).as_bytes()).err(), Some(NEEDS_NEWER), "{fields}");
    }
    assert_eq!(code::kind(sealed(&format!("{}{seed}", uv(newer * 2))).as_bytes()), None);
    // The line names no app: the host says whose build it is.
    assert_eq!(NEEDS_NEWER, "needs a newer version");
}

#[test]
fn no_code_is_read_as_another() {
    let text = write(&asked(Request::parse("glacial,bpm=80,organ=on,space.room=0.8")));
    // Every character changed to every other, and every pair swapped:
    // refused, never another loop.
    let bytes = text.as_bytes();
    for i in 0..bytes.len() {
        for &c in ALPHABET {
            if c != bytes[i] {
                let mut b = bytes.to_vec();
                b[i] = c;
                assert!(code::read_loop(&b).is_err(), "{} read", String::from_utf8_lossy(&b));
            }
        }
        if i + 1 < bytes.len() && bytes[i] != bytes[i + 1] {
            let mut b = bytes.to_vec();
            b.swap(i, i + 1);
            assert!(code::read_loop(&b).is_err());
        }
    }
    for bad in ["", "DL1-ABCDE", "AAAAAAAAA", "not a code at all", "E+AAAAA//"] {
        assert_eq!(code::read_loop(bad.as_bytes()).err(), Some(NOT_A_CODE), "{bad}");
    }
    // Spaces a paste brings are let be.
    assert!(code::read_loop(format!("  {text}\n").as_bytes()).is_ok());
    // Out of order, twice, or a set empty: no code's.
    let head = &text[..7];
    for fields in [format!("{}{}", uv(3), uv(0)) + &uv(1) + &uv(1), format!("{}{}{}{}", uv(8), uv(1), uv(0), uv(8)) + &uv(1) + &uv(0), format!("{}{}", uv(8), uv(0))] {
        assert_eq!(code::read_loop(sealed(&format!("{head}{fields}")).as_bytes()).err(), Some(NOT_A_CODE), "{fields}");
    }
}

#[test]
fn corrupt_and_truncated_codes_never_panic() {
    let mut r = Rng::new(31337);
    let mut seeds = Rng::new(1);
    let codes: Vec<String> = (0..200).map(|_| write(&random_loop(&mut seeds))).collect();
    let mut out = vec![0u8; 8192];
    let n = code::write_album(on(&codes[..10]), &name("x"), &mut out).expect("an album");
    let album = String::from_utf8(out[..n].to_vec()).unwrap();
    let n = code::write_seed_album(3, 5, &Request::parse("cozy,bars=8").unwrap(), &Name::NONE, &mut out).expect("an album");
    let seeded = String::from_utf8(out[..n].to_vec()).unwrap();
    let set = [time(120, 2), time(120, 2), time(9, 0), Pace::On, time(120, 2), time(5025, 3), time(1, 1), time(120, 2), Pace::OnFadingIn, time(86_400, 2)];
    let with_paces = paced_album(&codes[10..20], &set, &Name::NONE);
    let mut all = codes.clone();
    all.push(album);
    all.push(seeded);
    all.push(with_paces);
    let mut tried = 0;
    for _ in 0..40 {
        for c in &all {
            let mut b: Vec<u8> = c.as_bytes()[..c.len() - 2].to_vec();
            // Cut short, a character changed, one added or taken out, or
            // random characters: then sealed with a right check, so the
            // reader walks what it holds, not only the check.
            match r.pick_index(5) {
                0 => b.truncate(r.pick_index(b.len() + 1)),
                1 => {
                    let i = r.pick_index(b.len());
                    b[i] = ALPHABET[r.pick_index(64)];
                }
                2 => b.insert(r.pick_index(b.len() + 1), ALPHABET[r.pick_index(64)]),
                3 => {
                    b.remove(r.pick_index(b.len()));
                }
                _ => {
                    let keep = r.pick_index(b.len() + 1);
                    b.truncate(keep);
                    for _ in 0..r.pick_index(40) {
                        b.push(ALPHABET[r.pick_index(64)]);
                    }
                }
            }
            let text = sealed(core::str::from_utf8(&b).unwrap());
            let _ = code::read_loop(text.as_bytes());
            if let Ok(a) = code::read_album(text.as_bytes()) {
                for i in 0..a.count.min(4) {
                    let mut t = [0u8; code::LOOP_MOST];
                    if let Ok(n) = code::album_track(text.as_bytes(), i, &mut t) {
                        code::read_loop(&t[..n]).expect("an album's track reads as a loop");
                    }
                    let pace = code::album_pace(text.as_bytes(), i).expect("an album's track has a pace");
                    assert_eq!(Pace::of(pace.kind(), pace.count()), Some(pace));
                }
            }
            let _ = code::kind(text.as_bytes());
            // And without sealing: the check refuses nearly all.
            let _ = code::read_loop(&b);
            tried += 1;
        }
    }
    // Text of any bytes at all.
    for _ in 0..20_000 {
        let len = r.pick_index(30);
        let b: Vec<u8> = (0..len).map(|_| u8::try_from(r.seed32() & 255).unwrap()).collect();
        let _ = code::read_loop(&b);
        let _ = code::read_album(&b);
        tried += 1;
    }
    assert!(tried > 20_000);
    // Every code still reads back as it was.
    for c in &codes {
        assert_eq!(write(&code::read_loop(c.as_bytes()).expect("a code")), *c);
    }
}

#[test]
fn a_request_asks_its_version_in_text() {
    let r = Request::parse("version=2,glacial").expect("a request");
    assert_eq!(r.version(), 2);
    assert_eq!(Request::parse("version=1").err(), Some(request::BAD_VERSION));
    assert_eq!(Request::parse(&format!("version={}", data::CODE_NEWEST + 1)).err(), Some(NEEDS_NEWER));
    // No room: refused, not cut short.
    let mut small = [0u8; 8];
    assert_eq!(code::write_loop(&Loop::of(1), &mut small).err(), Some(code::NO_ROOM));
}

#[test]
fn the_formats_fixtures_are_its_bytes() {
    // core/golden/share-codes.txt: the format held to its bytes, the same lines
    // test/codes.test.mjs holds the web build to. From 1.0 on none changes.
    let text = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/golden/share-codes.txt")).expect("the fixtures");
    let req = |r: &str| if r == "-" { Request::new() } else { Request::parse(r).expect("a request") };
    let title = |t: &str| if t == "-" { Name::NONE } else { name(&unescape(t)) };
    let mut out = vec![0u8; 8192];
    let mut lines = 0;
    for line in text.lines().filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let f: Vec<&str> = line.split(' ').collect();
        match f[0] {
            "loop" => {
                let l = Loop { seed: f[1].parse().unwrap(), request: req(f[2]), mutes: f[3].parse().unwrap(), salt: f[4].parse().unwrap(), name: title(f[5]) };
                assert_eq!(write(&l), f[6], "{line}");
                assert_eq!(write(&code::read_loop(f[6].as_bytes()).expect("reads")), f[6]);
            }
            "album" => {
                // Each track its code, and its pace after it where it has one.
                let paced: Vec<(&[u8], Pace)> = f[2].split(',').map(|t| code::paced(t.as_bytes()).expect("a track")).collect();
                let n = code::write_album(paced.iter().copied(), &title(f[1]), &mut out).expect("an album");
                assert_eq!(core::str::from_utf8(&out[..n]).unwrap(), f[3], "{line}");
                let codes: Vec<&[u8]> = paced.iter().map(|p| p.0).collect();
                assert_eq!(album_tracks(f[3]).iter().map(String::as_bytes).collect::<Vec<_>>(), codes);
                assert_eq!(album_paces(f[3]), paced.iter().map(|p| p.1).collect::<Vec<_>>(), "{line}");
            }
            "seed-album" => {
                let n = code::write_seed_album(f[1].parse().unwrap(), f[2].parse().unwrap(), &req(f[3]), &title(f[4]), &mut out).expect("an album");
                assert_eq!(core::str::from_utf8(&out[..n]).unwrap(), f[5], "{line}");
            }
            other => panic!("a line of no kind: {other}"),
        }
        lines += 1;
    }
    assert!(lines >= 19);
}

// ---------------------------------------------------------------- names

/// Names in many scripts, with emoji: each a name a code carries.
const NAMES: [&str; 18] = [
    "Rainy Sunday",
    "x",
    "ฝนตกที่กรุงเทพ",
    "ที่นี่",
    "เพลงรักริมทะเล",
    "日本語の夜",
    "비 오는 날",
    "Café ☕",
    "🌧️🎶",
    "👩🏽‍💻 late",
    "👨‍👩‍👧‍👦",
    "🏳️‍🌈",
    "1️⃣ 2️⃣",
    "🏴\u{E0067}\u{E0062}\u{E0073}\u{E0063}\u{E0074}\u{E007F} glen",
    "می‌خواهم",
    "שָׁלוֹם",
    "Tiếng Việt",
    "Привет, мир!",
];

fn name(s: &str) -> Name {
    Name::new(s.as_bytes()).unwrap_or_else(|e| panic!("{s:?}: {e}"))
}

/// The fixtures' names: `%` and two hex digits for a byte (a space is `%20`).
fn unescape(t: &str) -> String {
    let b = t.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' {
            out.push(u8::from_str_radix(&t[i + 1..i + 3], 16).expect("hex"));
            i += 3;
        } else {
            out.push(b[i]);
            i += 1;
        }
    }
    String::from_utf8(out).expect("UTF-8")
}

/// A name's field as the format writes it: its tag, its length, its bytes
/// eight bits each, packed six to a character.
fn name_field(bytes: &[u8]) -> String {
    let mut s = uv(30) + &uv(bytes.len() as u64);
    let (mut acc, mut bits) = (0u32, 0);
    for &b in bytes {
        acc = (acc << 8) | u32::from(b);
        bits += 8;
        while bits >= 6 {
            bits -= 6;
            s.push(ALPHABET[((acc >> bits) & 63) as usize] as char);
        }
        acc &= (1 << bits) - 1;
    }
    if bits > 0 {
        s.push(ALPHABET[((acc << (6 - bits)) & 63) as usize] as char);
    }
    s
}

#[test]
fn a_name_travels_with_its_code_in_any_script() {
    roomy(|| {
        let mut g = Generator::new();
        let plain = write(&Loop::of(77));
        for s in NAMES {
            let l = Loop { name: name(s), ..Loop::of(77) };
            let text = round_trip(Some(&mut g), &l);
            assert_eq!(code::read_loop(text.as_bytes()).expect("reads").name.text(), s.as_bytes(), "{s}");
            // The name is its own field, last: the code without it is the
            // plain loop's.
            assert_eq!(text.len(), plain.len() + name_field(s.as_bytes()).len(), "{s}");
            // The name changes nothing the loop plays.
            assert_eq!(hashes(&mut g, &l), hashes(&mut g, &Loop::of(77)), "{s}");
        }
    });
    // None costs nothing: nine characters.
    assert_eq!(write(&Loop::of(77)).len(), 9);
    assert_eq!(write(&Loop { name: Name::NONE, ..Loop::of(77) }), write(&Loop::of(77)));
    assert!(Name::new(b"").expect("none").is_none());
    // About 1.3 characters an English letter, 4 a Thai one, 5 or 6 an emoji.
    let cost = |s: &str| write(&Loop { name: name(s), ..Loop::of(77) }).len() - 9 - 2;
    assert_eq!(cost("Rainy Sunday"), 16);
    assert_eq!(cost("ฝนตก"), 16);
    assert_eq!(cost("🌧"), 6);
    // An album's track keeps its own name; the album has its own.
    let tracks: Vec<String> = NAMES.iter().take(4).enumerate().map(|(i, s)| write(&Loop { name: name(s), ..Loop::of(u32::try_from(i).unwrap()) })).collect();
    let mut out = vec![0u8; 4096];
    let n = code::write_album(on(&tracks), &name("อัลบั้ม 🎧"), &mut out).expect("an album");
    let album = core::str::from_utf8(&out[..n]).unwrap();
    assert_eq!(album_tracks(album), tracks);
    assert_eq!(code::read_album(album.as_bytes()).expect("an album").name, name("อัลบั้ม 🎧"));
}

#[test]
fn a_name_is_capped_in_bytes() {
    assert_eq!(NAME_MOST, 60);
    let cases: [(&str, usize); 4] = [("a", 60), ("ก", 20), ("🌧", 15), ("é", 30)];
    for (c, most) in cases {
        let at = c.repeat(most);
        assert_eq!(at.len(), NAME_MOST, "{c}");
        let l = Loop { name: name(&at), ..Loop::of(5) };
        let text = round_trip(None, &l);
        // The longest name: 80 characters of bytes, its tag and its length.
        assert_eq!(text.len(), 9 + 1 + 2 + 80, "{c}");
        assert_eq!(Name::new(c.repeat(most + 1).as_bytes()).err(), Some(BAD_NAME), "{c}");
    }
    // A code whose name says more bytes than the cap is no code's.
    let head = &write(&Loop::of(5))[..7];
    let mut field = name_field("a".repeat(61).as_bytes());
    assert_eq!(code::read_loop(sealed(&format!("{head}{field}")).as_bytes()).err(), Some(NOT_A_CODE));
    // Nor one of no bytes, nor one cut short, nor one with spare bits set.
    field = uv(30) + &uv(0);
    assert_eq!(code::read_loop(sealed(&format!("{head}{field}")).as_bytes()).err(), Some(NOT_A_CODE));
    field = name_field(b"abc");
    field.pop();
    assert_eq!(code::read_loop(sealed(&format!("{head}{field}")).as_bytes()).err(), Some(NOT_A_CODE));
    field = name_field(b"ab");
    let last = field.pop().unwrap();
    field.push(ALPHABET[ALPHABET.iter().position(|&a| a == last as u8).unwrap() | 1] as char);
    assert_eq!(code::read_loop(sealed(&format!("{head}{field}")).as_bytes()).err(), Some(NOT_A_CODE));
}

#[test]
fn what_a_name_refuses() {
    let marks = |n: usize| format!("a{}", "\u{301}".repeat(n));
    assert!(Name::new(marks(MARKS_MOST).as_bytes()).is_ok());
    let piled = marks(MARKS_MOST + 1);
    let zalgo = "z\u{30d}\u{352}\u{31a}\u{344}\u{35b}\u{300}\u{346}a\u{335}\u{317}lgo";
    let refused: [&str; 40] = [
        " lead",
        "trail ",
        "two  spaces",
        " ",
        "tab\there",
        "line\nbreak",
        "bell\u{7}",
        "del\u{7f}",
        "c1\u{85}",
        "\u{200B}zero width",
        "zero\u{200B}width",
        "\u{202E}evil",
        "a\u{2066}b\u{2069}",
        "mark\u{200F}",
        "bom\u{FEFF}",
        "soft\u{AD}hyphen",
        "\u{3164}",
        "\u{115F}x",
        "\u{2800}",
        "pua\u{E000}",
        "non\u{FFFF}",
        "non\u{FDD0}",
        "a\u{200D}",
        "\u{200D}a",
        "a\u{200D}\u{200D}b",
        "a \u{200D}b",
        "\u{301}a",
        "a \u{301}",
        &piled,
        zalgo,
        "\u{E0067}",
        "a\u{E0067}\u{E007F}",
        "🏴\u{E0067}\u{E0062}",
        "🏴\u{E007F}",
        "🏴\u{E0067}\u{E0062}\u{E0067}\u{E0062}\u{E0067}\u{E0062}\u{E0067}\u{E0062}\u{E0067}\u{E007F}",
        "\u{3000}ideographic",
        "no\u{A0}break",
        "sep\u{2028}",
        "cgj\u{34F}",
        "mvs\u{180E}",
    ];
    for bad in refused {
        assert_eq!(Name::new(bad.as_bytes()).err(), Some(BAD_NAME), "{bad:?}");
    }
    // Bytes that are no UTF-8: cut, overlong, a surrogate, past U+10FFFF.
    for bad in [&b"\xff"[..], b"\xc0\x80", b"\xed\xa0\x80", b"\xf4\x90\x80\x80", b"ab\xe0\xb8"] {
        assert_eq!(Name::new(bad).err(), Some(BAD_NAME), "{bad:?}");
    }
    // A code carrying one, built by hand, is no code's.
    let head = &write(&Loop::of(5))[..7];
    for bad in [" lead", "\u{202E}evil", piled.as_str(), "a\u{200D}"] {
        let field = name_field(bad.as_bytes());
        assert_eq!(code::read_loop(sealed(&format!("{head}{field}")).as_bytes()).err(), Some(NOT_A_CODE), "{bad:?}");
    }
    assert_eq!(code::read_loop(sealed(&format!("{head}{}", name_field(b"\xff"))).as_bytes()).err(), Some(NOT_A_CODE));
}

#[test]
fn names_fuzzed() {
    // Random text from the scripts and the characters a name refuses: each
    // is carried exactly or refused, never changed, and never panics.
    let pools: [(u32, u32); 12] = [
        (0x20, 0x7e),
        (0x0, 0x1f),
        (0x300, 0x36f),
        (0xe00, 0xe5b),
        (0x5d0, 0x5ea),
        (0x600, 0x6ff),
        (0x200b, 0x206f),
        (0x3040, 0x30ff),
        (0xfe00, 0xfe0f),
        (0x1f300, 0x1f6ff),
        (0xe0000, 0xe007f),
        (0x0, 0x0010_ffff),
    ];
    let mut r = Rng::new(2026);
    let (mut carried, mut refused) = (0, 0);
    for _ in 0..30_000 {
        let mut s = String::new();
        for _ in 0..r.pick_index(24) {
            let (lo, hi) = pools[r.pick_index(pools.len())];
            if let Some(c) = char::from_u32(lo + r.seed32() % (hi - lo + 1)) {
                s.push(c);
            }
        }
        match Name::new(s.as_bytes()) {
            Ok(n) => {
                let l = Loop { name: n, ..Loop::of(r.seed32()) };
                let text = write(&l);
                let back = code::read_loop(text.as_bytes()).expect("a code");
                assert_eq!(back.name.text(), s.as_bytes(), "{s:?}");
                carried += 1;
            }
            Err(why) => {
                assert_eq!(why, BAD_NAME);
                refused += 1;
            }
        }
    }
    assert!(carried > 1000 && refused > 1000, "{carried} carried, {refused} refused");
    // Random bytes: the same.
    for _ in 0..30_000 {
        let b: Vec<u8> = (0..r.pick_index(NAME_MOST + 4)).map(|_| u8::try_from(r.seed32() & 255).unwrap()).collect();
        if let Ok(n) = Name::new(&b) {
            let text = write(&Loop { name: n, ..Loop::of(1) });
            assert_eq!(code::read_loop(text.as_bytes()).expect("a code").name.text(), &b[..]);
        }
    }
    // A name field of random characters, sealed: read or refused, never a
    // panic, and what reads writes back the same.
    let head = &write(&Loop::of(5))[..7];
    for _ in 0..30_000 {
        let mut field = uv(30) + &uv(r.pick_index(NAME_MOST + 2) as u64);
        for _ in 0..r.pick_index(84) {
            field.push(ALPHABET[r.pick_index(64)] as char);
        }
        let text = sealed(&format!("{head}{field}"));
        if let Ok(l) = code::read_loop(text.as_bytes()) {
            assert_eq!(write(&l), text);
        }
    }
}

#[test]
fn a_request_reads_back_from_its_text() {
    // What a code asks, written as text (`code::request_text`), reads back
    // to the same request: the same code, every part and every edit.
    let mut r = Rng::new(424);
    for i in 0..3000 {
        let l = if i == 0 { Loop::of(1) } else { random_loop(&mut r) };
        let mut text = String::new();
        code::request_text(&l.request, &mut text).expect("text");
        let back = Request::parse(&text).unwrap_or_else(|e| panic!("{text}: {e}"));
        assert_eq!(write(&Loop { request: back, ..l }), write(&l), "{text}");
    }
    let mut text = String::new();
    let asked = Request::parse("glacial=0.7,misty,bpm=70-85,bars=3072,journey=arc,key=C#|G,scale=dorian,style=tide,voice.melody=kalimba|marimba,layer.drums=off,theme=1234,organ=on,tuning.cents=-12.5,edit.roll.bass=7,edit.warmth=0.6,edit.bpm=88.5").expect("a request");
    code::request_text(&asked, &mut text).expect("text");
    assert_eq!(text, "glacial=0.7,misty,bpm=70-85,bars=3072,journey=arc,key=C#|G,scale=dorian,style=tide,voice.melody=kalimba|marimba,layer.drums=off,theme=1234,organ=on,tuning.cents=-12.5,edit.roll.bass=7,edit.warmth=0.6,edit.bpm=88.5");
    // A code's loop says its version.
    let read = code::read_loop(write(&Loop::of(5)).as_bytes()).expect("a code");
    text.clear();
    code::request_text(&read.request, &mut text).expect("text");
    assert_eq!(text, format!("version={}", data::CODE_NEWEST));
}
