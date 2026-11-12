//! One note of one voice, through its layer's own path (queue item 96): the
//! measuring tools' voice probe, on the player. A loop of one step holding
//! one note is handed to the sequencer, which plays it as it plays any
//! loop's, so what is measured (the level a voice sits at in its layer, its
//! tone) is what a listener gets, the layer's strum, spread and channel
//! included. The tools build only (`--features tools`): the app's modules
//! never carry it.

use crate::generator::Layer;
use crate::generator::layers::{BassEvent, DrumEvent, TextureEvent};
use crate::generator::list::List;
use crate::generator::load::Load;
use crate::generator::melody::{MelodyEvent, NO_NOTE};
use crate::generator::plan::ChordEvent;
use crate::generator::render::Render;
use crate::generator::spec::Spec;

use super::host::Host;
use super::sequencer::{Loop, Sequencer};
use super::synth::Synth;

/// The voices a probe can ask for in each layer, in `Layer`'s order: the
/// drums by hit, the bass's, the chords' and the melody's as the request
/// names them (`request::VOICE_ORDER`) with the sung ones, the texture's as
/// the synth names its kinds.
pub const VOICES: [&[&str]; 5] = [
    &["kick", "softkick", "snare", "clap", "rim", "hat", "ohat", "shaker", "frame", "tap", "jingle", "ojingle"],
    &["sub", "round", "pluckbass", "moogbass", "fifths", "rhodesbass"],
    &[
        "keys", "pad", "harp", "accordion", "piano", "rhodes", "moogpad", "softpad", "analogpad", "prepared", "musicbox", "kalimba", "marimba",
        "vowel", "hum", "choir", "templebell", "tubular", "stab", "nylon",
    ],
    &[
        "pluck", "bell", "keys", "saw", "ocarina", "flute", "harp", "musicbox", "fiddle", "accordion", "piano", "sine", "rhodes", "moog", "whistle",
        "softpad", "analoglead", "prepared", "celeste", "kalimba", "marimba", "vowel", "hum", "choir", "templebell", "tubular", "stab", "panflute",
    ],
    &["bell", "swell", "drop", "chime", "wind", "waves"],
];

/// The vowels a sung note takes in turn up the probe's window.
const VOWELS: [&str; 4] = ["a", "e", "o", "u"];

/// The probe's tempo: a step of 0.1 s.
const BPM: f64 = 150.0;

/// A note to probe.
#[derive(Clone, Copy)]
pub struct Note {
    /// The layer whose path plays it.
    pub layer: Layer,
    /// The voice, one of `VOICES[layer]`.
    pub voice: &'static str,
    /// Its pitch (the drums take none).
    pub midi: f64,
    /// Its velocity.
    pub vel: f64,
    /// How long it is held, in seconds.
    pub seconds: f64,
    /// The seed its draws are made from (the loop's, in a loop).
    pub seed: u32,
    /// Which vowel a sung voice takes (`midi` less the window's lowest).
    pub vowel: usize,
}

/// The room a probe needs, kept by the host in a static: a loop's worth of
/// lists is too large for a stack.
pub struct Room {
    spec: Spec,
    render: Render,
    seq: Sequencer,
}

impl Room {
    /// An empty room.
    // Built in a `const` for a host's static, never on a stack at run time.
    #[allow(clippy::large_stack_arrays)]
    #[must_use]
    pub const fn new() -> Self {
        Room { spec: Spec::EMPTY, render: Render::new(), seq: Sequencer::new() }
    }

    /// Hand `note` to `synth` at `at` seconds, as the sequencer plays a
    /// loop's step: nothing else of a loop (no setups, no organ, no late
    /// floor), the synth as it started.
    pub fn play<H: Host>(&mut self, host: &mut H, synth: &mut Synth, note: &Note, at: f64) {
        let steps = note.seconds / (60.0 / BPM / 4.0);
        self.spec = Spec::EMPTY;
        self.spec.bpm = BPM;
        self.spec.steps_per_bar = 16.0;
        self.spec.bars = 1.0 / 16.0;
        self.spec.seed = note.seed;
        let r = &mut self.render;
        r.drums.clear();
        r.bass.clear();
        r.chords.clear();
        r.melody.clear();
        r.texture.clear();
        let voice = note.voice.into();
        let vowel = VOWELS[note.vowel % 4];
        match note.layer {
            // A roll takes no humanizing jitter, so the hit lands where it is put.
            Layer::Drums => r.drums.push(DrumEvent { step: 0.0, inst: voice, vel: note.vel, vel0: note.vel, roll: true, micro: None }),
            Layer::Bass => {
                r.bass.push(BassEvent { step: 0.0, dur: steps, midi: note.midi, vel: note.vel, vel0: note.vel, glide: false, voice, chug: false, steps: None });
            }
            Layer::Chords => {
                #[allow(clippy::cast_possible_truncation)] // a midi note
                let notes = List::from_slice(&[note.midi as i16]);
                r.chords.push(ChordEvent { step: 0.0, dur: steps, notes, vel: note.vel, voice, vowel: Some(vowel), detune: None, strum: None, vel0: note.vel, alias: false });
            }
            Layer::Melody => {
                r.melody.push(MelodyEvent { dur: steps, midi: note.midi, vel: note.vel, vel0: note.vel, voice, vowel: vowel.into(), ..NO_NOTE });
            }
            Layer::Texture => {
                #[allow(clippy::cast_possible_truncation)] // a midi note
                let notes = List::from_slice(&[note.midi as i16]);
                r.texture.push(TextureEvent { step: 0.0, dur: steps, notes, vel: note.vel, vel0: note.vel, kind: voice, band: None });
            }
        }
        self.seq = Sequencer::new();
        self.seq.bpm = BPM;
        self.seq.next_step_time = at;
        let lp = Loop { spec: &self.spec, base: &self.render, pass: None, settings: &Load::EMPTY, step: 0.0, second: None };
        self.seq.schedule_step(host, synth, &lp, 0.0);
    }
}

impl Default for Room {
    fn default() -> Self {
        Room::new()
    }
}
