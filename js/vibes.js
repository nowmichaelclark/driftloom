// Vibes: the recipes (queue items 38a and 78).
//
// The recipes are data now: data/vibes.txt, a [vibe name] block each, in the
// text format of js/data.js. The brain edits that file (docs/DECISIONS.md,
// "The words and vibes"); docs/VIBES.md keeps the reasoning, the history and
// "How a recipe works" and "How to edit a vibe". The core's generator reads
// the same file, compiled in (`core/build.rs`); the page reads it for the
// vibes' names and their order (the picker, a request, a share code).
// test/data.test.mjs checks every key and every name in them and fails on an
// unknown one.

import { blockOf, blocksOf, READ, readBlock } from './data.js';

const SCHEMA = { feel: 'table', tone: 'table', profiles: 'table', modes: 'table', melody: 'table', chords: 'table',
  bass: 'table', bassStyle: 'table', texture: 'table' };
for (const k of ['tempo', 'swing', 'drums', 'hats', 'rests', 'long', 'bigChords', 'six8']) SCHEMA[k] = 'num';

export const VIBES = Object.fromEntries(blocksOf('vibes', 'vibe').map((b) => [b.name, readBlock(b, SCHEMA, 'data/vibes.txt')]));

// The order share codes write a vibe in (its index here), from the file's
// [order vibes] block: append only, never reorder, or codes already written
// down stop meaning what they meant.
export const VIBE_IDS = READ.words(blockOf('vibes', 'order', 'vibes').entries.find(([k]) => k === 'ids')[1]);
