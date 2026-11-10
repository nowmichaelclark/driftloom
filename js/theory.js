// The notes' and the modes' names, as the page shows and writes them (the
// card, the pickers, a request, a share code). The music itself is the
// core's (`core/src/generator/theory.rs`): nothing here decides a note.

export const NOTE_NAMES = ['C', 'C#', 'D', 'Eb', 'E', 'F', 'F#', 'G', 'Ab', 'A', 'Bb', 'B'];

export const SCALES = {
  dorian: { steps: [0, 2, 3, 5, 7, 9, 10], label: 'Dorian' },
  aeolian: { steps: [0, 2, 3, 5, 7, 8, 10], label: 'Aeolian' },
  ionian: { steps: [0, 2, 4, 5, 7, 9, 11], label: 'Ionian' },
  mixolydian: { steps: [0, 2, 4, 5, 7, 9, 10], label: 'Mixolydian' },
  lydian: { steps: [0, 2, 4, 6, 7, 9, 11], label: 'Lydian' },
  phrygian: { steps: [0, 1, 3, 5, 7, 8, 10], label: 'Phrygian' },
  harmonicMinor: { steps: [0, 2, 3, 5, 7, 8, 11], label: 'Harmonic minor' },
  minorPent: { steps: [0, 3, 5, 7, 10], label: 'Minor pentatonic' },
  majorPent: { steps: [0, 2, 4, 7, 9], label: 'Major pentatonic' },
  kumoi: { steps: [0, 2, 3, 7, 9], label: 'Kumoi' },
  hirajoshi: { steps: [0, 2, 3, 7, 8], label: 'Hirajoshi' },
  insen: { steps: [0, 1, 5, 7, 10], label: 'Insen' },
  wholeTone: { steps: [0, 2, 4, 6, 8, 10], label: 'Whole tone' },

  // Kondo leans on modes that share a lowered seventh, which is what makes
  // the bVII-to-I move available and why Hyrule never sounds like it has
  // cadenced properly.
  lydianDominant: { steps: [0, 2, 4, 6, 7, 9, 10], label: 'Lydian dominant' },
  phrygianDominant: { steps: [0, 1, 4, 5, 7, 8, 10], label: 'Phrygian dominant' },
  dorianSharp4: { steps: [0, 2, 3, 6, 7, 9, 10], label: 'Dorian #4' },

  // Further Japanese pentatonics, for the Yoshimura and Hisaishi colours.
  yo: { steps: [0, 2, 5, 7, 9], label: 'Yo' },
  ritusen: { steps: [0, 2, 5, 7, 10], label: 'Ritusen' },
  akebono: { steps: [0, 2, 3, 7, 9], label: 'Akebono' },
};

// The modes in their frozen order (append only): the order a share code and
// the Rust core's `theory::SCALES` number them in, and a request's set of
// scales is written in (queue item 89).
export const SCALE_ORDER = [
  'dorian', 'aeolian', 'ionian', 'mixolydian', 'lydian', 'phrygian',
  'harmonicMinor', 'minorPent', 'majorPent', 'kumoi', 'hirajoshi', 'insen',
  'wholeTone', 'lydianDominant', 'phrygianDominant', 'dorianSharp4',
  'yo', 'ritusen', 'akebono',
];
