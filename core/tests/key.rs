//! The request's key and mode (queue item 89; `Request::with_keys`,
//! `Request::with_scales`): a note or a set of notes, a mode or a set of
//! modes, read from text, refused with a plain line, and honored in what the
//! generator makes. Measured in the render, not only in the spec: the
//! tonic chord sounds on the key, and the melody and the chords keep to the
//! mode (the JavaScript's test takes the texture too). test/generator.test.mjs measures the same in the
//! JavaScript's render; the generation goldens' `key` cases hold the two
//! together.
//!
//! What is allowed outside a mode, as the catalog's own loops are: the bass's
//! fifth and its pickups, a fixed interval from a chord's root (the catalog
//! sounds 0.85% of its bass notes outside its mode; pinned onto every mode
//! alike, 1.2%), and cinder's spelled chords with the melody over them.

// Pitches, keys and degrees are small whole numbers, compared exactly and
// turned into indexes on purpose.
#![allow(clippy::float_cmp, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use dlcore::block::Event;
use dlcore::generator::Generator;
use dlcore::generator::data::CHARACTERS;
use dlcore::generator::request::{self, Request};
use dlcore::generator::rng::Rng;
use dlcore::generator::spec::new_spec;
use dlcore::generator::theory::SCALES;
use dlcore::player::host::Host;
use dlcore::player::{Player, State};
use dlcore::{EXTRA, Key, QUANTUM, Tap};

fn roomy(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new().stack_size(64 << 20).spawn(f).expect("a thread").join().expect("the test");
}

/// A pitch class, 0 to 11.
fn pc(midi: f64) -> usize {
    (midi as i64).rem_euclid(12) as usize
}

#[test]
fn the_text_and_the_refusals() {
    let keys = |text: &str| Request::parse(text).map(|r| r.keys());
    assert_eq!(keys("key=D"), Ok(1 << 2));
    assert_eq!(keys("key=Db"), Ok(1 << 1));
    assert_eq!(keys("key=C#|Db"), Ok(1 << 1));
    assert_eq!(keys("key=G|D"), Ok((1 << 2) | (1 << 7)));
    assert_eq!(keys("cozy,bpm=80"), Ok(0));
    for (i, name) in ["C", "C#", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B"].into_iter().enumerate() {
        assert_eq!(keys(&format!("key={name}")), Ok(1 << i), "{name}");
    }
    for name in ["A#", "D#", "G#", "Gb"] {
        assert_eq!(keys(&format!("key={name}")).map(u32::count_ones), Ok(1), "{name}");
    }
    let scales = |text: &str| Request::parse(text).map(|r| r.scales());
    assert_eq!(scales("scale=dorian"), Ok(1));
    assert_eq!(scales("scale=aeolian|dorian"), Ok(0b11));
    assert_eq!(scales("scale=akebono"), Ok(1 << 18));
    assert_eq!(scales("-"), Ok(0));
    for text in ["key=H", "key=", "key", "key=D|", "key=d", "key=D,key=Q"] {
        assert_eq!(Request::parse(text).err(), Some(request::NO_SUCH_KEY), "{text}");
    }
    for text in ["scale=nope", "scale=", "scale", "scale=dorian|", "scale=Dorian"] {
        assert_eq!(Request::parse(text).err(), Some(request::NO_SUCH_SCALE), "{text}");
    }
    assert_eq!(Request::new().with_keys(1 << 12).err(), Some(request::NO_SUCH_KEY));
    assert_eq!(Request::new().with_scales(1 << 19).err(), Some(request::NO_SUCH_SCALE));
    assert_eq!(Request::new().with_keys(request::KEYS_ALL).map(|r| r.keys()), Ok(request::KEYS_ALL));
    assert_eq!(Request::new().with_scales(request::SCALES_ALL).map(|r| r.scales()), Ok(request::SCALES_ALL));
    assert_eq!(Request::new().with_keys(0).map(|r| r.keys()), Ok(0), "none asked");
    assert_eq!(SCALES.len(), 19, "a share code and the app number the modes in this order");
    for line in [request::NO_SUCH_KEY, request::NO_SUCH_SCALE] {
        assert!(line.is_ascii() && line.len() < 40);
    }
}

#[test]
fn nothing_asked_is_the_seed_s_loop() {
    // The goldens hold every part; here the plainest form of it.
    for seed in [1, 77, 4_272_732_017] {
        let a = new_spec(seed, &Request::new());
        let b = new_spec(seed, &Request::new().with_keys(0).expect("none").with_scales(0).expect("none"));
        assert!((a.root, a.scale, a.keys, a.scales) == (b.root, b.scale, 0, 0) && b.bpm == a.bpm);
    }
}

#[test]
fn a_pinned_key_and_mode_are_played() {
    roomy(pinned);
}

/// At least 200 loops, a key and a mode pinned on each, until every profile
/// has led a dozen: the spec is the pin's, the tonic chord sounds on the
/// key, the melody, chords and texture stay in the mode.
fn pinned() {
    let mut g = Box::new(Generator::new());
    let mut r = Rng::new(8901);
    let mut led = [0usize; CHARACTERS.len()];
    let (mut loops, mut sounded, mut outside, mut bass_all, mut bass_out) = (0, 0, 0, 0, 0);
    let mut out_by = [0usize; CHARACTERS.len()];
    let mut i = 0;
    while loops < 200 || led.iter().any(|&n| n < 12) {
        i += 1;
        assert!(i < 6000, "no pin led every profile: {led:?}");
        let seed = r.seed32();
        let key = r.pick_index(12);
        let scale = r.pick_index(SCALES.len());
        let request = Request::new().with_keys(1 << key).expect("a key").with_scales(1 << scale).expect("a mode");
        let p = new_spec(seed, &request).character().lead;
        if loops >= 200 && led[p] >= 12 {
            continue;
        }
        led[p] += 1;
        loops += 1;
        g.load_with(seed, &request);
        let spec = g.spec();
        assert_eq!((spec.root as usize, spec.scale), (key, scale), "seed {seed}: the spec");
        assert_eq!((spec.keys, spec.scales), (1 << key, 1 << scale), "seed {seed}: kept, for a share code");
        let rd = g.render();
        let in_mode = |midi: f64| SCALES[scale].steps.iter().any(|&s| (key + s as usize) % 12 == pc(midi));
        // The first chord built on the tonic is on the key.
        if let Some(slot) = rd.harmony.slots.iter().find(|s| s.degree == 0.0) {
            assert_eq!(pc(slot.root_midi), key, "seed {seed}: the tonic chord");
        }
        for e in rd.melody.iter().filter(|e| e.vel != 0.0) {
            sounded += 1;
            if !in_mode(e.midi) {
                outside += 1;
                out_by[p] += 1;
            }
        }
        for e in rd.chords.iter().filter(|e| e.vel != 0.0) {
            for &m in e.notes.iter() {
                sounded += 1;
                if !in_mode(f64::from(m)) {
                    outside += 1;
                    out_by[p] += 1;
                }
            }
        }
        for e in rd.bass.iter().filter(|e| e.vel != 0.0) {
            bass_all += 1;
            bass_out += usize::from(!in_mode(e.midi));
        }
    }
    for (p, &n) in out_by.iter().enumerate() {
        assert!(n == 0 || CHARACTERS[p].id == "cinder", "{}: {n} notes outside the mode", CHARACTERS[p].id);
    }
    assert!(outside * 2000 < sounded, "{outside} of {sounded} notes outside the mode");
    assert!(bass_out * 50 < bass_all, "the bass: {bass_out} of {bass_all} outside the mode");
    println!("pinned: {loops} loops, every profile led at least 12, {outside} of {sounded} notes outside (cinder's spelled chords), the bass {bass_out} of {bass_all}");
}

#[test]
fn a_set_draws_only_from_the_set_and_keeps_a_draw_inside_it() {
    roomy(sets);
}

fn sets() {
    let mut r = Rng::new(8902);
    let keys: u32 = (1 << 2) | (1 << 7);
    let scales: u32 = 1 | 2 | (1 << 9); // dorian, aeolian, kumoi
    let request = Request::new().with_keys(keys).expect("keys").with_scales(scales).expect("modes");
    let (mut reached_k, mut reached_s) = (0u32, 0u32);
    let (mut stands, mut inside_k, mut kept, mut inside_s) = (0, 0, 0, 0);
    for _ in 0..500 {
        let seed = r.seed32();
        let plain = new_spec(seed, &Request::new());
        let asked = new_spec(seed, &request);
        let (k, s) = (asked.root as usize, asked.scale);
        assert!(keys & (1 << k) != 0 && scales & (1 << s) != 0, "seed {seed}: {k} {s} outside the sets");
        reached_k |= 1 << k;
        reached_s |= 1 << s;
        if keys & (1 << plain.root as usize) != 0 {
            inside_k += 1;
            stands += usize::from(asked.root == plain.root);
        }
        if scales & (1 << plain.scale) != 0 {
            inside_s += 1;
            kept += usize::from(asked.scale == plain.scale);
        }
    }
    assert_eq!((reached_k, reached_s), (keys, scales), "every member reached");
    assert!(inside_k > 0 && inside_s > 0);
    assert_eq!((stands, kept), (inside_k, inside_s), "a draw already inside the set stands");
}

#[test]
fn a_mode_asked_of_every_profile_makes_a_loop() {
    roomy(every_pair);
}

/// No pair is forbidden: every mode on every profile (the profile led by a
/// seed found for it) renders chords, and the mode is the one asked.
fn every_pair() {
    let mut g = Box::new(Generator::new());
    let mut r = Rng::new(8903);
    let mut loops = 0;
    for (scale, mode) in SCALES.iter().enumerate() {
        let mut want: Vec<usize> = (0..CHARACTERS.len()).collect();
        let mut tries = 0;
        while !want.is_empty() {
            tries += 1;
            assert!(tries < 20_000, "{scale}: profiles left {want:?}");
            let seed = r.seed32();
            let request = Request::new().with_scales(1 << scale).expect("a mode");
            let p = new_spec(seed, &request).character().lead;
            let Some(at) = want.iter().position(|&w| w == p) else { continue };
            want.swap_remove(at);
            g.load_with(seed, &request);
            assert_eq!(g.spec().scale, scale, "seed {seed}");
            let rd = g.render();
            // (A layer may rest the whole loop, as it always could.)
            assert!(!rd.chords.is_empty() || !rd.melody.is_empty() || !rd.bass.is_empty(), "seed {seed}, {} on {}: nothing", mode.id, CHARACTERS[p].id);
            assert_eq!(g.lost(), 0, "seed {seed}: the lists ran out of room");
            loops += 1;
        }
    }
    println!("every pair: {loops} loops");
}

#[test]
fn a_key_and_a_mode_overrule_a_theme_s() {
    roomy(themed);
}

fn themed() {
    let mut r = Rng::new(8904);
    for i in 0..60usize {
        let (seed, theme) = (r.seed32(), r.seed32());
        let own = new_spec(theme, &Request::new());
        let key = (own.root as usize + 1 + i % 11) % 12;
        let scale = (own.scale + 1 + i % 18) % SCALES.len();
        let bent = new_spec(seed, &Request::new().with_theme(theme).with_keys(1 << key).expect("key").with_scales(1 << scale).expect("mode"));
        assert_eq!((bent.root as usize, bent.scale), (key, scale), "theme {theme} on {seed}");
        assert_eq!(&bent.mix[..], &own.mix[..], "theme {theme} on {seed}: the mix stays");
        let kept = new_spec(seed, &Request::new().with_theme(theme).with_keys(1 << own.root as usize).expect("key"));
        assert_eq!((kept.root, kept.scale), (own.root, own.scale), "a key the theme has stands");
    }
}

/// A host that renders nothing and writes down the pitch of each note the
/// player hands the melody's and the chords' channels.
struct Pitches {
    frame: u64,
    melody: Vec<f64>,
    chords: Vec<f64>,
}

impl Host for Pitches {
    fn frame(&self) -> u64 {
        self.frame
    }
    fn key(&mut self, _: Key) {}
    fn note(&mut self, _: u32, channel: u32, _: f64, midi: f64, _: f64, vel: f64, _: u32, _: [f64; EXTRA]) {
        if vel > 0.0 {
            match channel {
                dlcore::CHORDS => self.chords.push(midi),
                dlcore::MELODY => self.melody.push(midi),
                _ => {}
            }
        }
    }
    fn event(&mut self, _: u32, _: Event) {}
    fn damp(&mut self, _: u32, _: f64) {}
    fn set(&mut self, _: u32, _: usize, _: usize, _: f64, _: f64) {}
    fn route(&mut self, _: u32, _: usize, _: usize, _: Option<Tap>, _: f64) {}
    fn param(&mut self, _: u32, _: u32, _: f64, _: f64, _: f64) {}
    fn bass_lift(&mut self, _: f64) {}
    fn space_range(&mut self, _: usize, _: f64, _: f64) {}
    fn steal(&mut self, _: u32, _: Key, _: f64, _: f64) {}
    fn fade_all(&mut self, _: f64, _: f64) {}
}

#[test]
fn the_player_plays_the_key_and_the_mode() {
    roomy(played);
}

/// What the player hands the core, not only what the generator decides: 60
/// loops with a key and a mode pinned, played for twenty seconds. The melody
/// stays inside the mode (cinder's too: the generator's own render, above,
/// has it outside only over the added F). The chords channel also carries
/// what the engine adds to them at load (the organ's, the stab's and the
/// wash's fixed intervals), so it is held to 97% in the mode, and its rate
/// is printed beside the same seeds' on their own modes.
fn played() {
    let mut player = Box::new(Player::new());
    let mut r = Rng::new(8905);
    let (mut sounded, mut chords, mut chords_out, mut melody_out) = (0, 0, 0, 0);
    for i in 0..60usize {
        let seed = r.seed32();
        let (key, scale) = (r.pick_index(12), r.pick_index(SCALES.len()));
        let request = Request::new().with_keys(1 << key).expect("a key").with_scales(1 << scale).expect("a mode");
        let mut host = Pitches { frame: 0, melody: Vec::new(), chords: Vec::new() };
        player.attach(&mut host, 48_000.0, true);
        player.load(seed, &request);
        player.ready(&mut host);
        assert_eq!(player.state(), State::Playing);
        assert_eq!((player.generator().spec().root as usize, player.generator().spec().scale), (key, scale), "loop {i}, seed {seed}");
        let cinder = CHARACTERS[player.generator().spec().character().lead].id == "cinder";
        // The note a pitch plays, the loop's tuning (item 101b) taken out.
        let tune = player.generator().settings().tuning / 100.0;
        while host.frame < 20 * 48_000 {
            player.decide(&mut host);
            host.frame += QUANTUM as u64;
        }
        let in_mode = |m: f64| SCALES[scale].steps.iter().any(|&s| (key + s as usize) % 12 == pc(m - tune));
        for &m in &host.melody {
            sounded += 1;
            if !in_mode(m) {
                assert!(cinder, "loop {i}, seed {seed}: melody {m} is outside {} in {key}", SCALES[scale].id);
                melody_out += 1;
            }
        }
        for &m in &host.chords {
            sounded += 1;
            chords += 1;
            chords_out += usize::from(!in_mode(m));
        }
    }
    // The same seeds on their own key and mode, for the catalog's rate.
    let (mut own, mut own_out) = (0, 0);
    let mut r = Rng::new(8905);
    for _ in 0..60 {
        let seed = r.seed32();
        let _ = (r.pick_index(12), r.pick_index(SCALES.len()));
        let mut host = Pitches { frame: 0, melody: Vec::new(), chords: Vec::new() };
        player.attach(&mut host, 48_000.0, true);
        player.load(seed, &Request::new());
        player.ready(&mut host);
        let (key, scale) = (player.generator().spec().root as usize, player.generator().spec().scale);
        let tune = player.generator().settings().tuning / 100.0;
        while host.frame < 20 * 48_000 {
            player.decide(&mut host);
            host.frame += QUANTUM as u64;
        }
        for &m in &host.chords {
            own += 1;
            own_out += usize::from(!SCALES[scale].steps.iter().any(|&s| (key + s as usize) % 12 == pc(m - tune)));
        }
    }
    println!("played: 60 loops, the melody {melody_out} notes outside (cinder), the chords channel {chords_out} of {chords} (the same seeds on their own modes: {own_out} of {own})");
    assert!(chords_out * 100 < chords * 3, "the chords channel: {chords_out} of {chords} outside the mode");
    assert!(sounded > 1000, "{sounded} notes");
}
