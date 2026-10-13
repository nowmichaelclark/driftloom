// The core's draws, in JavaScript (queue item 44).
//
// A draw is a pure function of where a note sits in its loop, the channel it
// plays in, the kind of draw and which one it is (core/src/rng.rs says why).
// The core makes every draw that shapes a voice's sound itself. This file is
// the same function, for the few decisions the page still takes from one
// because it must know the answer before the core is asked:
//
//   - when a note starts: a drum's timing jitter, a layer's ensemble slip and
//     a chord's strum span move with the scheduler, which stays here until
//     phase 3;
//   - whether a held sung vowel drifts, and to where, since the voice budget
//     charges for it and the note's formant trim depends on it.
//
// The numbers are the core's (`rng::salt`), and test/draw-vectors.txt holds
// hashes both sides must make: test/draw.test.mjs checks this file against
// it, and core/tests/draws.rs checks the core's.

// The kinds of draw the page makes. Fixed for good, as the core's are.
export const HOST_JITTER = 100;
export const HOST_ENSEMBLE = 101;
export const HOST_STRUM = 102;
export const HOST_DRIFT = 103;
export const HOST_DRIFT_TO = 104;

// The core's channels, by the layer that plays into them.
export const CHANNEL = { melody: 0, chords: 1, bass: 2, texture: 3, drums: 4 };

function fold(h, word) {
  h = Math.imul(h ^ word, 0x85ebca6b);
  h ^= h >>> 13;
  h = Math.imul(h, 0xc2b2ae35);
  return (h ^ (h >>> 16)) >>> 0;
}

// The 32-bit hash of one draw. Every argument is an unsigned 32-bit whole
// number.
export function bits(seed, pass, step, channel, ord, salt, index) {
  let h = 0x9e3779b9;
  h = fold(h, seed);
  h = fold(h, pass);
  h = fold(h, step);
  h = fold(h, channel);
  h = fold(h, ord);
  h = fold(h, salt);
  return fold(h, index);
}

// A draw in [0, 1): the hash's top 24 bits over 2^24, exact in a double and
// in a 32-bit float.
export function unit(seed, pass, step, channel, ord, salt, index = 0) {
  return (bits(seed, pass, step, channel, ord, salt, index) >>> 8) / 16777216;
}

// The same, for a key `{ seed, pass, step, ord }` (what the engine gives the
// synth; see Engine._key) and the layer it is for.
export function draw(key, layer, salt, index = 0) {
  return unit(key.seed, key.pass, key.step, CHANNEL[layer], key.ord, salt, index);
}
