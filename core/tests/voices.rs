//! The request's voices and layers (queue item 91; `Request::with_voices`,
//! `Request::with_layer`): a layer's voices, a set or one, and a layer on or
//! off. Read from text, refused with a plain line, and honored in what the
//! generator makes and in what the player hands the core: the voice that
//! plays is one asked, the choir is not forced by the melody's and the
//! chords', a layer on plays, a layer off is silent. test/generator.test.mjs
//! measures the same in the JavaScript's render; the generation goldens'
//! `voice` cases hold the two generators together.

// Counts and indexes, compared exactly on purpose.
#![allow(clippy::float_cmp, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use dlcore::block::Event;
use dlcore::generator::Generator;
use dlcore::generator::request::{self, LAYER_NAMES, LayerAsk, PROFILE_ORDER, Request, VOICE_ORDER};
use dlcore::generator::rng::Rng;
use dlcore::generator::spec::new_spec;
use dlcore::player::host::Host;
use dlcore::player::{Player, State};
use dlcore::{EXTRA, Key, QUANTUM, Tap};

fn roomy(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new().stack_size(64 << 20).spawn(f).expect("a thread").join().expect("the test");
}

fn character(bit: usize) -> usize {
    dlcore::generator::data::CHARACTERS.iter().position(|c| c.id == PROFILE_ORDER[bit]).expect("a profile")
}

#[test]
fn the_text_and_the_refusals() {
    let voices = |text: &str, layer: usize| Request::parse(text).map(|r| r.voices(layer));
    assert_eq!(voices("voice.melody=flute", 3), Ok(1 << 5));
    assert_eq!(voices("voice.melody=marimba|kalimba", 3), Ok((1 << 19) | (1 << 20)));
    assert_eq!(voices("voice.drums=hand", 0), Ok(1 << 2));
    assert_eq!(voices("voice.texture=waves", 4), Ok(1 << 5));
    assert_eq!(voices("cozy,bpm=80", 3), Ok(0));
    for (layer, name) in LAYER_NAMES.iter().enumerate() {
        for (bit, voice) in VOICE_ORDER[layer].iter().enumerate() {
            assert_eq!(voices(&format!("voice.{name}={voice}"), layer), Ok(1 << bit), "{name} {voice}");
        }
    }
    for text in ["voice.melody=nope", "voice.melody=", "voice.melody", "voice.melody=vowel", "voice.melody=hum", "voice.chords=choir", "voice.drums=kalimba", "voice.melody=flute|"] {
        assert_eq!(Request::parse(text).err(), Some(request::NO_SUCH_VOICE), "{text}");
    }
    assert_eq!(Request::parse("voice.nope=flute").err(), Some(request::NO_SUCH_LAYER));
    assert_eq!(Request::parse("layer.nope=on").err(), Some(request::NO_SUCH_LAYER));
    for text in ["layer.drums=maybe", "layer.drums=", "layer.drums"] {
        assert_eq!(Request::parse(text).err(), Some(request::BAD_LAYER), "{text}");
    }
    let r = Request::parse("layer.drums=off,layer.melody=on,layer.bass=off").expect("layers");
    assert_eq!((r.layer(0), r.layer(1), r.layer(2), r.layer(3), r.layer(4)), (LayerAsk::Off, LayerAsk::Off, LayerAsk::Chance, LayerAsk::On, LayerAsk::Chance));
    assert_eq!((r.layers_on(), r.layers_off()), (0b01000, 0b00011));
    let all = "layer.drums=off,layer.bass=off,layer.chords=off,layer.melody=off,layer.texture=off";
    assert_eq!(Request::parse(all).err(), Some(request::ALL_LAYERS_OFF));
    let four = Request::parse("layer.drums=off,layer.bass=off,layer.chords=off,layer.melody=off").expect("four off");
    assert_eq!(four.with_layer(4, LayerAsk::Off).err(), Some(request::ALL_LAYERS_OFF));
    assert_eq!(four.with_layer(4, LayerAsk::On).map(|r| r.layer(4)), Ok(LayerAsk::On));
    assert_eq!(four.with_layer(0, LayerAsk::Chance).map(|r| r.layers_off()), Ok(0b01110));
    assert_eq!(Request::new().with_voices(5, 1).err(), Some(request::NO_SUCH_LAYER));
    assert_eq!(Request::new().with_voices(0, 1 << 3).err(), Some(request::NO_SUCH_VOICE));
    assert_eq!(Request::new().with_layer(5, LayerAsk::On).err(), Some(request::NO_SUCH_LAYER));
    for line in [request::NO_SUCH_VOICE, request::NO_SUCH_LAYER, request::BAD_LAYER, request::ALL_LAYERS_OFF] {
        assert!(line.is_ascii() && line.len() < 40);
    }
}

#[test]
fn nothing_asked_is_the_seed_s_loop() {
    for seed in [1, 77, 4_272_732_017] {
        let a = new_spec(seed, &Request::new());
        let b = new_spec(seed, &Request::new().with_voices(3, 0).expect("none").with_layer(0, LayerAsk::Chance).expect("none"));
        assert!(a.voices == [0; 5] && b.voices == [0; 5] && a.layers_on | a.layers_off | b.layers_on | b.layers_off == 0);
    }
}

#[test]
fn the_voice_that_plays_is_one_asked() {
    roomy(voices);
}

fn voices() {
    let mut g = Box::new(Generator::new());
    let mut r = Rng::new(9101);
    let (mut loops, mut played) = (0, 0);
    for (bit, profile) in PROFILE_ORDER.iter().enumerate() {
        for layer in 0..LAYER_NAMES.len() {
            for i in 0..6usize {
                // A set of one to three of the layer's voices.
                let list = VOICE_ORDER[layer];
                let mut wanted = 0u32;
                for _ in 0..=(i % 3) {
                    wanted |= 1 << r.pick_index(list.len());
                }
                let request = Request::new().with_styles(1 << bit).expect("a style").with_voices(layer, wanted).expect("voices");
                let seed = r.seed32();
                g.load_with(seed, &request);
                let rd = g.render();
                loops += 1;
                let asked = |name: &str| list.iter().enumerate().any(|(b, &v)| v == name && wanted & (1 << b) != 0);
                let heard: Vec<&str> = match layer {
                    0 => vec![rd.drums_out.kit].into_iter().filter(|&k| k != "none").collect(),
                    1 => rd.bass.iter().filter(|e| e.vel != 0.0).map(|e| e.voice).collect(),
                    2 => rd.chords.iter().filter(|e| e.vel != 0.0).map(|e| e.voice).collect(),
                    3 => rd.melody.iter().filter(|e| e.vel != 0.0).map(|e| e.voice).collect(),
                    _ => vec![rd.texture_kind].into_iter().filter(|&k| k != "none").collect(),
                };
                played += usize::from(!heard.is_empty());
                for v in &heard {
                    assert!(asked(v), "seed {seed}, {} {}: {v} is not among those asked", profile, LAYER_NAMES[layer]);
                }
                if layer == 2 || layer == 3 {
                    assert!(g.spec().choir().is_none(), "seed {seed}: no choir for the chords' or the melody's voices");
                }
            }
        }
    }
    assert!(played * 10 > loops * 7, "the layer plays in most: {played} of {loops}");
    println!("voices: {loops} loops, the layer playing in {played}");
}

#[test]
fn a_layer_on_plays_and_keeps_the_loop_s_own() {
    roomy(on);
}

fn on() {
    let mut g = Box::new(Generator::new());
    let mut r = Rng::new(9102);
    let (mut loops, mut plays) = (0, 0);
    for i in 0..200usize {
        let seed = r.seed32();
        let layer = i % 5;
        g.load_with(seed, &Request::new().with_layer(layer, LayerAsk::On).expect("on"));
        let rd = g.render();
        loops += 1;
        let sounds = match layer {
            0 => rd.drums_out.kit != "none" && !rd.drums.is_empty(),
            1 => rd.bass.iter().any(|e| e.vel != 0.0),
            2 => rd.chords.iter().any(|e| e.vel != 0.0),
            3 => rd.melody.iter().any(|e| e.vel != 0.0),
            _ => rd.texture_kind != "none" && !rd.texture.is_empty(),
        };
        plays += usize::from(sounds);
        assert_eq!((g.spec().layers_on, g.spec().layers_off), (1 << layer, 0));
    }
    // (A melody on a one-bar cycle of undertow's micro-loops sometimes comes
    // out of its own generator with no notes, as a re-rolled melody does.)
    assert!(plays * 100 >= loops * 99, "a layer on plays: {plays} of {loops}");

    // Drums asked on keep the loop's own where it had them and no entry
    // schedule changes them.
    let (mut was, mut kept) = (0, 0);
    for seed in 1..200u32 {
        g.load_with(seed, &Request::new());
        if g.render().drums_out.kit == "none" || g.render().form.is_some() {
            continue;
        }
        let own: Vec<(u64, u64)> = g.render().drums.iter().map(|e| (e.step.to_bits(), e.vel.to_bits())).collect();
        g.load_with(seed, &Request::new().with_layer(0, LayerAsk::On).expect("on"));
        let asked: Vec<(u64, u64)> = g.render().drums.iter().map(|e| (e.step.to_bits(), e.vel.to_bits())).collect();
        was += 1;
        kept += usize::from(own == asked);
    }
    assert!(was > 50 && kept == was, "drums asked on keep the loop's own: {kept} of {was}");
}

/// A host that renders nothing and counts the notes the player hands each
/// channel.
struct Counts {
    frame: u64,
    notes: [usize; 5],
}

impl Host for Counts {
    fn frame(&self) -> u64 {
        self.frame
    }
    fn key(&mut self, _: Key) {}
    fn note(&mut self, _: u32, channel: u32, _: f64, _: f64, _: f64, vel: f64, _: u32, _: [f64; EXTRA]) {
        if vel > 0.0 && (channel as usize) < 5 {
            self.notes[channel as usize] += 1;
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
fn the_player_leaves_a_layer_off_out_and_plays_one_on() {
    roomy(played);
}

/// What the player hands the core, by the core's channel (melody 0, chords 1,
/// bass 2, texture 3, drums 4): a layer asked off is not handed a note in
/// twenty seconds, the others are (where the loop has them), and a layer
/// asked off and then not asked brings its notes back with the next load
/// only if it was asked on (a mute holds across loads until brought back).
fn played() {
    let mut player = Box::new(Player::new());
    let mut r = Rng::new(9104);
    // The core's channel of each layer, in `Layer` order.
    let channel = [dlcore::DRUMS_CHANNEL, dlcore::BASS_CHANNEL, dlcore::CHORDS, dlcore::MELODY, dlcore::TEXTURE_CHANNEL].map(|c| c as usize);
    let (mut silent, mut heard_elsewhere) = (0, 0);
    for i in 0..40usize {
        let seed = r.seed32();
        let layer = i % 4; // drums, bass, chords, melody
        let request = Request::new().with_layer(layer, LayerAsk::Off).expect("off");
        let mut host = Counts { frame: 0, notes: [0; 5] };
        player.attach(&mut host, 48_000.0, true);
        player.load(seed, &request);
        player.ready(&mut host);
        assert_eq!(player.state(), State::Playing);
        while host.frame < 20 * 48_000 {
            player.decide(&mut host);
            host.frame += QUANTUM as u64;
        }
        assert_eq!(host.notes[channel[layer]], 0, "loop {i}, seed {seed}: {} was asked off", LAYER_NAMES[layer]);
        silent += 1;
        heard_elsewhere += usize::from(host.notes.iter().enumerate().any(|(c, &n)| c != channel[layer] && n > 0));
        // The next load asks the layer on: it is unmuted and sounds (a minute
        // on: the lateFloor choice keeps a kit out for a while at the start).
        let on = Request::new().with_layer(layer, LayerAsk::On).expect("on");
        let mut host = Counts { frame: 0, notes: [0; 5] };
        player.attach(&mut host, 48_000.0, true);
        player.load(seed, &on);
        player.ready(&mut host);
        while host.frame < 60 * 48_000 {
            player.decide(&mut host);
            host.frame += QUANTUM as u64;
        }
        assert!(host.notes[channel[layer]] > 0 || layer == 3, "loop {i}, seed {seed}: {} was asked on and not heard", LAYER_NAMES[layer]);
    }
    assert!(heard_elsewhere * 10 > silent * 9, "the other layers still play: {heard_elsewhere} of {silent}");
}

#[test]
fn the_other_requests_ride_with_it() {
    roomy(riding);
}

fn riding() {
    let mut r = Rng::new(9103);
    for i in 0..60usize {
        let request = Request::parse(&format!(
            "style={},key=D,bpm=100,voice.melody=flute,voice.chords=pad,layer.bass=off,layer.texture=on",
            PROFILE_ORDER[i % PROFILE_ORDER.len()]
        ))
        .expect("a request");
        let seed = r.seed32();
        let a = new_spec(seed, &request);
        assert_eq!((a.root, a.bpm, a.character().lead), (2.0, 100.0, character(i % PROFILE_ORDER.len())), "seed {seed}");
        assert_eq!((a.layers_off, a.layers_on), (0b00010, 0b10000));
    }
}
