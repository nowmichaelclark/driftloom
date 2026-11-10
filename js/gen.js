// The core's generator, on the page (queue item 97).
//
// What a loop is, the core decides: its name, key, tempo, mix and moods, its
// notes. The core that plays is in an AudioWorklet and exists only after a
// tap, but the page shows a loop before that (the card it opens on) and
// beside it (a loop from the Library or a code, a re-roll's new grid, the
// MIDI export). So the page keeps one more instance of the same module
// (js/dlplayer.wasm), here on its own thread, and asks it what a seed and a
// request make. It plays nothing: a loop is made at once (`dl_play_ready`),
// read, and left. Making one takes a millisecond or two.
//
// It is the same generator as the one that plays, byte for byte, so what
// the card says is what is heard. Until item 97 the page had a generator of
// its own in JavaScript (js/generator.js), held to the core's by the
// generation goldens; there is now one.
//
//   newSpec(seed, request)   the loop a seed and a request make, as the
//                            page's record of it (js/request.js)
//   render(spec)             that record's notes, layer by layer, as the
//                            grid and the MIDI export read them
//
// In Node the module is read from disk (the tests and the tools).

import { randomSeed } from './rng.js';
import { LAYERS, STEPS_PER_BAR, RequestError, readRequest, askOf, askFor } from './request.js';

const IN_NODE = typeof window === 'undefined' && typeof process !== 'undefined' && !!(process.versions && process.versions.node);
const TONE = ['warmth', 'space', 'wobble'];
// Any rate will do: nothing is rendered.
const RATE = 48000;

async function bytes() {
  if (IN_NODE) {
    const { readFile } = await import('node:fs/promises');
    return readFile(new URL('./dlplayer.wasm', import.meta.url));
  }
  const { fetchCore, coreUnsupported } = await import('./core.js');
  if (typeof WebAssembly !== 'object' || typeof WebAssembly.Module !== 'function') throw new Error(coreUnsupported() || 'no WebAssembly in this browser');
  return fetchCore();
}

// The instance, or why there is none (`unavailable`): a browser without
// WebAssembly, or a module that did not arrive. The page then shows no loop
// and says so (js/main.js).
let x = null;
let why = null;
try {
  const module = await WebAssembly.compile(await bytes());
  x = (await WebAssembly.instantiate(module, {})).exports;
  x.dl_init(RATE);
  // Lite: the least there is to start. Nothing is mixed.
  x.dl_quality(0);
  x.dl_play_attach(0);
} catch (err) {
  x = null;
  why = String((err && err.message) || err);
}

// Why the page has no generator, or null.
export const unavailable = why;

const text = (at, length) => String.fromCharCode(...new Uint8Array(x.memory.buffer, at, length));

// Hand the core what is asked of the next loop, call by call: the same
// calls, in the same order, as js/worklet.js's `ask` makes for the core that
// plays. Returns whether every part was taken.
function ask(a) {
  let ok = true;
  const take = (taken) => { if (!taken) ok = false; };
  for (const [i, w] of a.vibes || []) take(w == null ? x.dl_play_vibe(i) : x.dl_play_weight(i, w));
  if (a.tempo) take(x.dl_play_ask_tempo(a.tempo[0], a.tempo[1]));
  for (const [i, v] of a.pins || []) take(x.dl_play_pin(i, v));
  if (a.theme != null) x.dl_play_theme(a.theme >>> 0);
  if (a.length != null) take(x.dl_play_length(a.length === 'endless' ? Infinity : a.length));
  if (a.keys) take(x.dl_play_keys(a.keys));
  if (a.scales) take(x.dl_play_scales(a.scales));
  if (a.styles) take(x.dl_play_styles(a.styles));
  (a.voices || []).forEach((mask, layer) => { if (mask) take(x.dl_play_voices(layer, mask)); });
  (a.layers || []).forEach((state, layer) => { if (state) take(x.dl_play_layer(layer, state)); });
  for (const [layer, seed, rolled] of a.seeds || []) take(x.dl_play_seed(layer, seed >>> 0, rolled ? 1 : 0));
  (a.tone || []).forEach((v, which) => take(x.dl_play_edit_tone(which, v)));
  if (a.bpm != null) take(x.dl_play_edit_bpm(a.bpm));
  return ok;
}

// The loop of `seed` as `a` asks, made now. What the core refused, if it
// refused, as a RequestError with the core's own line.
function make(seed, a) {
  if (!x) throw new Error(`no generator: ${why}`);
  x.dl_play_stop();
  const ok = ask(a);
  const n = x.dl_play_refusal();
  const refusal = n ? text(x.dl_play_refusal_at(), n) : null;
  if (!ok || refusal) {
    // The request asked so far is dropped with the load below: nothing of
    // it reaches the next loop.
    x.dl_play_load(seed >>> 0, 0);
    x.dl_play_stop();
    throw new RequestError(refusal || "can't do that");
  }
  x.dl_play_load(seed >>> 0, 0);
  x.dl_play_ready();
}

// What the core says of the loop it made (`Player::describe`), by field.
function about() {
  const lines = text(x.dl_play_about_at(), x.dl_play_describe()).split('\n').filter(Boolean);
  return Object.fromEntries(lines.map((l) => [l.slice(0, l.indexOf('=')), l.slice(l.indexOf('=') + 1)]));
}

const weights = (s) => Object.fromEntries((s || '').split('|').filter(Boolean).map((p) => [p.slice(0, p.indexOf(':')), Number(p.slice(p.indexOf(':') + 1))]));
const numbers = (s) => (s || '').split('|').map(Number);

// The loop `seed` and `request` make (js/request.js's `readRequest` says
// what a request is; a list of vibe names is a listener's selection): the
// page's record of it. A request the core cannot meet throws a RequestError
// with the core's line for it.
export function newSpec(seed = randomSeed(), request = null) {
  const asked = readRequest(request);
  make(seed, askFor(asked));
  const d = about();
  const [lift, energy, warmth] = numbers(d.feel);
  const tone = numbers(d.tone);
  const seeds = numbers(d.layer_seeds);
  const spec = {
    seed: Number(d.seed),
    name: d.name,
    mix: weights(d.mix),
    feel: { lift, energy, warmth },
    feelMix: weights(d.moods),
    coherence: Number(d.coherence),
    mood: lift, // kept so older saves still read something
    bpm: Number(d.bpm),
    root: Number(d.root),
    scale: d.scale,
    bars: Number(d.bars),
    stepsPerBar: Number(d.steps_per_bar),
    swing: Number(d.swing),
    layerSeeds: Object.fromEntries(LAYERS.map((l, i) => [l, seeds[i] >>> 0])),
    mutes: Object.fromEntries(LAYERS.map((l) => [l, false])),
    // How many times round before moving on. Null means it never moves on,
    // which is the default: a loop machine should loop until you say stop.
    playFor: null,
    tone: Object.fromEntries(TONE.map((k, i) => [k, tone[i]])),
  };
  // Each layer's own cycle in steps, for a loop whose layers drift apart.
  if (d.cycles) spec.cycles = Object.fromEntries(LAYERS.map((l, i) => [l, numbers(d.cycles)[i] || null]));
  // The vibes that steered it, with the weights the core drew.
  if (d.vibes) {
    spec.vibes = weights(d.vibes);
    spec.vibeStrength = Number(d.strength);
  }
  // What was asked, kept: the loop is made again from its record, and a
  // share code carries it.
  if (asked.tempo) spec.tempo = asked.tempo.slice();
  if (asked.pins) spec.pins = { ...asked.pins };
  if (asked.length != null) spec.length = asked.length;
  if (asked.theme != null) spec.theme = asked.theme;
  if (asked.style) spec.askStyle = asked.style.slice();
  if (asked.voices) spec.askVoices = Object.fromEntries(Object.entries(asked.voices).map(([l, v]) => [l, v.slice()]));
  if (asked.layers) {
    spec.askLayers = { ...asked.layers };
    // A layer asked off is a layer muted, as the listener's mute button
    // would (and unmutes the same way).
    for (const [layer, state] of Object.entries(asked.layers)) if (state === 'off') spec.mutes[layer] = true;
  }
  if (asked.key) spec.askKey = asked.key.slice();
  if (asked.scale) spec.askScale = asked.scale.slice();
  return spec;
}

// A layer's notes in the loop as composed (`Player::note`), as the MIDI
// export and the grid read them: a drum hit { step, vel, inst, micro }, a
// bass or melody note { step, dur, midi, vel }, a chord or a texture's
// { step, dur, notes, vel }.
function track(layer) {
  const l = LAYERS.indexOf(layer);
  const out = [];
  const n = x.dl_play_notes(l);
  for (let i = 0; i < n; i++) {
    const name = x.dl_play_note(l, i);
    const f = new Float64Array(x.memory.buffer, x.dl_play_note_at(), 16);
    const [step, dur, vel, micro, count] = f;
    if (layer === 'drums') {
      const e = { step, vel, inst: text(x.dl_play_text(), name) };
      if (micro) e.micro = micro;
      out.push(e);
    } else if (layer === 'bass' || layer === 'melody') {
      out.push({ step, dur, midi: f[5], vel });
    } else {
      out.push({ step, dur, notes: Array.from(f.subarray(5, 5 + count)), vel });
    }
  }
  return out;
}

// The loop of a record, as composed: { spec, stepsPerBar, totalSteps,
// cycles, tracks, form, choir }. Made from the record's seed, request and
// edits (`askOf`), exactly as the core that plays it is asked. `form` is
// whether its layers come and go (an airy loop), `choir` whether it is one
// of the rare choir loops.
export function render(spec) {
  make(spec.seed, askOf(spec));
  const d = about();
  const stepsPerBar = Number(d.steps_per_bar) || STEPS_PER_BAR;
  const cycles = d.cycles ? Object.fromEntries(LAYERS.map((l, i) => [l, numbers(d.cycles)[i] || null])) : null;
  return {
    spec,
    stepsPerBar,
    totalSteps: Number(d.bars) * stepsPerBar,
    cycles,
    tracks: Object.fromEntries(LAYERS.map((l) => [l, track(l)])),
    form: d.airy === '1',
    choir: d.choir === '1',
  };
}
