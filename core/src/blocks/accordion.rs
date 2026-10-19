//! Accordion: two soft reeds through the accordion's body
//! (`voice('accordion')`).
//!
//! The note's first extra value is the note it is joined to (NaN if none);
//! the second is drawn, how late its second reed comes in. It plays
//! through the accordion body (`own::ACCORDION_BODY`).

// Web Audio's parameters are 32-bit floats, set from the host's 64-bit
// numbers: a voice narrows every value it hands one.
#![allow(clippy::cast_possible_truncation)]

use crate::QUANTUM;
use crate::block::{Block, Context, NO_EXTRA, Note, Probe, Signal, Voice};
use crate::param::Param;
use crate::rng::salt;

use super::own;
use super::osc::{Pitch, TableOsc};
use super::shared::{SLUR_ATTACK, UNITY, midi_to_freq, speak};

/// The accordion's level, by layer (`ACCORDION_LEVEL`): a chord is several
/// notes at once through the engine's spread, so chords take their own.
const ACCORDION_MELODY: f64 = 0.0892;
const ACCORDION_CHORDS: f64 = 0.0909;
/// The two reeds' detunes, in cents either side (`REED_CENTS`), and the
/// second one's level (`SECOND_REED`).
const REED_CENTS: f32 = 1.25;
const SECOND_REED: f32 = 0.35;

/// Accordion: two soft reeds a couple of cents apart, the second quieter
/// and starting at a random point of its cycle, under one envelope, into
/// the accordion's body (`voice('accordion')`).
pub struct Accordion {
    reeds: [TableOsc; 2],
    pitch: f32,
    envelope: Param,
}

impl Accordion {
    /// Silent and all zeros, so a pool of them costs nothing in the `.wasm`;
    /// starting it sets every value a render reads.
    #[must_use]
    pub const fn new() -> Self {
        Accordion {
            reeds: [TableOsc::new(), TableOsc::new()],
            pitch: 0.0,
            envelope: Param::new(0.0),
        }
    }

    /// `chords` is whether it plays into the chords channel; `joined`,
    /// whether it follows on from the note before (`opts.prev`); `draw`, the
    /// draw in [0, 1) that sets how late the second reed comes in, under one
    /// period of the note (`Voice::settle`).
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn play(
        &mut self,
        midi: f64,
        time: f64,
        dur: f64,
        vel: f64,
        chords: bool,
        joined: bool,
        draw: f64,
        rate: f64,
    ) {
        let f = midi_to_freq(midi);
        self.pitch = f as f32;
        let level = vel
            * if chords {
                ACCORDION_CHORDS
            } else {
                ACCORDION_MELODY
            };
        let attack = if joined { SLUR_ATTACK } else { speak(dur) };
        let g = &mut self.envelope;
        g.reset_at(UNITY, rate);
        g.set_value_at_time(0.0001, time);
        g.linear_ramp_to_value_at_time(level as f32, time + attack, time);
        g.set_target_at_time(0.0001, time + dur * 0.92, 0.06);
        let stop = time + dur + 0.35;
        let [first, second] = &mut self.reeds;
        first.schedule(time, stop, rate);
        second.schedule(time + draw / f, stop, rate);
    }


    fn start_frame(&self) -> u64 {
        self.reeds[0].start_frame()
    }

    fn end_frame(&self) -> u64 {
        self.reeds[0].stop_frame()
    }
}

impl Block for Accordion {
    type Settings = Note;

    const PROBES: &'static [Probe<Note>] = &[
        Probe { name: "melody", settings: Note::probe(crate::ACCORDION, crate::MELODY, 67.0, 0, NO_EXTRA) },
        Probe { name: "chords", settings: Note::probe(crate::ACCORDION, crate::CHORDS, 55.0, 0, NO_EXTRA) },
    ];

    fn start(&mut self, note: &Note, cx: &Context) {
        let [prev, late, ..] = note.extra;
        let chords = note.channel == crate::CHORDS;
        self.play(note.midi, note.time, note.dur, note.vel, chords, prev.is_finite(), late, cx.rate);
    }

    fn span(&self) -> (u64, u64) {
        (self.start_frame(), self.end_frame())
    }

    /// Add this block into `out`, the accordion's bus for its channel.
    fn render(&mut self, block: u64, cx: &Context, out: &mut Signal) {
        let (rate, waves) = (cx.rate, cx.waves);
        let r = rate as f32;
        let [first, second] = &mut self.reeds;
        let mut low = [0.0f32; QUANTUM];
        let mut high = [0.0f32; QUANTUM];
        let (lo, hi) = first.render(
            block,
            &waves.reed,
            &Pitch::Steady {
                freq: self.pitch,
                detune: -REED_CENTS,
            },
            r,
            &mut low,
        );
        second.render(
            block,
            &waves.reed,
            &Pitch::Steady {
                freq: self.pitch,
                detune: REED_CENTS,
            },
            r,
            &mut high,
        );
        let mut envelope = [0.0f32; QUANTUM];
        self.envelope.fill(block, rate, &mut envelope);
        // The second reed through its own gain, then both into the
        // envelope's gain. The second starts no earlier than the first, so
        // the first's range covers both.
        for (((y, a), b), g) in out
            .iter_mut()
            .zip(low.iter())
            .zip(high.iter())
            .zip(envelope.iter())
            .take(hi)
            .skip(lo)
        {
            *y += (*a + *b * SECOND_REED) * *g;
        }
    }
}

impl Voice for Accordion {
    const THROUGH: Option<usize> = Some(own::ACCORDION_BODY);

    /// How late its second reed comes in.
    fn settle(note: &mut Note) {
        note.extra[1] = note.draws.unit(salt::REED_LATE, 0);
    }
}

impl Default for Accordion {
    fn default() -> Self {
        Self::new()
    }
}
