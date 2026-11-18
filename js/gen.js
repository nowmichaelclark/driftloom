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
//   newSpec(seed, request, edits)
//                            the loop a seed and a request make (and a
//                            code's edits, if any), as the page's record of
//                            it (js/request.js)
//   playedSpec(spec)         the record of the loop a record plays
//                            (`askOf`): what the Library and a code hear
//   loopCode, readLoopCode, albumCode, seedAlbumCode, albumTracks, codeKind,
//   nameRefusal
//                            share codes, version 2, written and read by the
//                            core (session 23; js/share.js is the page's
//                            side, core/README.md the format)
//   render(spec)             that record's notes, layer by layer, as the
//                            grid and the MIDI export read them
//   pieceMarks(spec, opts)   a long piece's marks: where its movements and
//                            its roles, themes and keys begin (session 17)
//
// In Node the module is read from disk (the tests and the tools).

import { randomSeed } from './rng.js';
import { LAYERS, STEPS_PER_BAR, RequestError, readRequest, askOf, askFor, requestOf } from './request.js';

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
const utf8 = new TextEncoder();
const fromUtf8 = new TextDecoder();

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
  if (a.journey) take(x.dl_play_journey(a.journey));
  if (a.keys) take(x.dl_play_keys(a.keys));
  if (a.scales) take(x.dl_play_scales(a.scales));
  if (a.styles) take(x.dl_play_styles(a.styles));
  (a.voices || []).forEach((mask, layer) => { if (mask) take(x.dl_play_voices(layer, mask)); });
  (a.layers || []).forEach((state, layer) => { if (state) take(x.dl_play_layer(layer, state)); });
  for (const [layer, seed, rolled] of a.seeds || []) take(x.dl_play_seed(layer, seed >>> 0, rolled ? 1 : 0));
  (a.tone || []).forEach((v, which) => { if (v != null) take(x.dl_play_edit_tone(which, v)); });
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
// page's record of it. `edits`, a code's ({ seeds: [[layer, seed, rolled]],
// tone: [warmth, space, wobble] with null for one not edited, bpm }), are
// made with it, as the core makes a code's loop. A request the core cannot
// meet throws a RequestError with the core's line for it.
export function newSpec(seed = randomSeed(), request = null, edits = null) {
  const asked = readRequest(request);
  make(seed, edits ? { ...askFor(asked), ...edits } : askFor(asked));
  return record(asked);
}

// The record of the loop a record plays: made from its seed, request and
// edits (`askOf`), as the Library, the player and a share code make it, and
// read back. Its own request is kept as the record had it.
export function playedSpec(spec) {
  make(spec.seed, askOf(spec));
  return record(readRequest(requestOf(spec)));
}

// What the core said of the loop just made, and `asked`, as the page's
// record.
function record(asked) {
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
  // The layers a code's edits re-rolled, played whatever their chances draw.
  if (d.rolled) spec.forceLayers = Object.fromEntries(d.rolled.split('|').map((l) => [l, true]));
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
  if (asked.journey) spec.askJourney = asked.journey;
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

// A long piece's plan, by the numbers the core gives (core/src/generator/
// longform.rs, in its enums' order): a unit's role, its theme, its key's
// relation to home, and its movement's kind.
export const ROLES = ['opening', 'theme', 'build', 'second', 'development', 'peak', 'release', 'return', 'close', 'breath', 'lift', 'holding', 'turning'];
export const THEMES = ['rest', 'A', 'B', 'recall'];
export const RELATIONS = ['home', 'fifthAbove', 'fifthBelow', 'relative', 'parallel'];
export const MOVEMENTS = ['full', 'gentle', 'light', 'interlude'];
const MARK = 8;
// The most marks one call gives (`dl_play_marks`): ask again from the
// section after the last for more.
export const MARKS_MOST = 2048;

// A long piece's marks for the page (session 17, queue item 106a): the
// units of `count` sections from section `from` where something a listener
// would see marked begins, read straight from the piece's plan, nothing
// before them walked: the first unit, then each where its role, theme, key
// or movement changes ({ bar, section, role, theme, root, relation,
// energy, movement }, `bar` from the piece's start, `root` 0 for C to 11,
// `energy` 0 to 1). With `coarse`, a movement's start alone: a 32,768-bar
// piece's whole run of movements costs a few milliseconds, so the page can
// draw the whole piece's movements and ask a window of roles around the
// playhead; Endless is asked a window at a time (`count` sections, 64
// unless asked; a finite piece's whole round unless asked). Null for a loop
// that is no long piece. At most MARKS_MOST a call.
export function pieceMarks(spec, { from = 0, count = null, coarse = false } = {}) {
  make(spec.seed, askOf(spec));
  // A piece's sections, once round (the core gives a round at most); for
  // Endless, a window: 64 sections unless asked.
  const sections = Number.isFinite(spec.length) && spec.bars > 0 ? Math.ceil(spec.length / spec.bars) : 64;
  const n = x.dl_play_marks(from, count == null ? sections : count, coarse ? 1 : 0);
  if (!n) return null;
  const f = new Float64Array(x.memory.buffer, x.dl_play_marks_at(), n * MARK);
  const marks = [];
  for (let i = 0; i < n; i++) {
    const [bar, section, role, theme, root, relation, energy, movement] = f.subarray(i * MARK, (i + 1) * MARK);
    marks.push({ bar, section, role: ROLES[role], theme: THEMES[theme], root, relation: RELATIONS[relation], energy, movement: MOVEMENTS[movement] });
  }
  return marks;
}

// ------------------------------------------------------------ share codes

// Share codes, version 2 (session 23; core/src/code.rs, core/README.md's
// "Share codes"): the core writes and reads them, the page hands it a
// loop's seed, what was asked and changed, the mutes, the wander's salt and
// a name of its own, and reads the same back. Names are UTF-8 both ways.
// A refusal is a CodeError whose message is the core's plain line ("not a
// code", "needs a newer version", "a name a code can't carry").

export class CodeError extends Error {}

function input(bytes) {
  new Uint8Array(x.memory.buffer, x.dl_code_input(), bytes.length).set(bytes);
  return bytes.length;
}

const codeText = (n) => fromUtf8.decode(new Uint8Array(x.memory.buffer, x.dl_code_text(), n));

function refused() {
  const n = x.dl_code_refusal();
  return new CodeError(n ? codeText(n) : 'not a code');
}

// The request asked so far dropped, so nothing of a code reaches the next
// loop made.
function dropAsked(seed) {
  x.dl_play_load(seed >>> 0, 0);
  x.dl_play_stop();
}

function need() {
  if (!x) throw new Error(`no generator: ${why}`);
}

// A loop's code: `seed`, `a` (the core's numbers: js/request.js's askFor,
// with a code's edits), the layers muted (a bit each, in LAYERS' order), the
// wander's salt (0 for none) and its own name ('' for none).
export function loopCode(seed, a, { mutes = 0, salt = 0, name = '' } = {}) {
  need();
  x.dl_play_stop();
  const ok = ask(a);
  const n = ok ? x.dl_code_write(seed >>> 0, mutes, salt >>> 0, input(utf8.encode(name))) : 0;
  const err = !ok ? new CodeError(text(x.dl_play_refusal_at(), x.dl_play_refusal()) || "can't do that") : n ? null : refused();
  const code = n ? codeText(n) : null;
  dropAsked(seed);
  if (err) throw err;
  return code;
}

// A loop's code read: { seed, mutes, salt, name, request }, the request as
// the text the core's `Request::parse` (and js/request.js's readRequest, for
// its asks) reads, its edits ("edit.seed.drums=1234") and its version
// ("version=2") among it.
export function readLoopCode(code) {
  need();
  if (!x.dl_code_read(input(utf8.encode(String(code))))) throw refused();
  const out = { seed: x.dl_code_seed() >>> 0, mutes: x.dl_code_mutes(), salt: x.dl_code_salt() >>> 0 };
  out.name = codeText(x.dl_code_name());
  out.request = codeText(x.dl_code_request());
  dropAsked(out.seed);
  return out;
}

// An album's code: the loops' codes in their order, each with how long it
// plays before the next after it where it has one (`CODE:p2` two whole
// passes, `CODE:b8` eight bars: core/src/code.rs's `paced`), named `name`
// ('' for none).
export function albumCode(codes, name = '') {
  need();
  const head = utf8.encode(name);
  const body = utf8.encode(codes.join('.'));
  const all = new Uint8Array(head.length + body.length);
  all.set(head);
  all.set(body, head.length);
  const n = x.dl_code_album_write(input(all), head.length);
  if (!n) throw refused();
  return codeText(n);
}

// An album of `count` tracks drawn from `seed`, each asked what `a` asks (the
// core's numbers: js/request.js's askFor; edits have no place in it), named
// `name` ('' for none): its code, a few characters whatever the count.
export function seedAlbumCode(seed, count, a, name = '') {
  need();
  x.dl_play_stop();
  const ok = ask(a);
  const n = ok ? x.dl_code_album_seed(seed >>> 0, count, input(utf8.encode(name))) : 0;
  const err = !ok ? new CodeError(text(x.dl_play_refusal_at(), x.dl_play_refusal()) || "can't do that") : n ? null : refused();
  const code = n ? codeText(n) : null;
  dropAsked(seed);
  if (err) throw err;
  return code;
}

// An album's { name, tracks, paces }: each track a loop's code, and how long
// it plays before the next as [kind, count], as the player takes it
// (js/player.js's trackOf: 0 on and on, 1 whole passes, 2 bars).
export function albumTracks(code) {
  need();
  const len = input(utf8.encode(String(code)));
  const count = x.dl_code_album(len);
  if (!count) throw refused();
  const mem = new Uint8Array(x.memory.buffer, x.dl_code_text(), 256);
  const name = fromUtf8.decode(mem.slice(0, mem.indexOf(0)));
  const tracks = [];
  const paces = [];
  for (let i = 0; i < count; i++) {
    const n = x.dl_code_track(len, i);
    if (!n) throw refused();
    tracks.push(codeText(n));
    const kind = x.dl_code_pace(len, i);
    if (!kind) throw refused();
    paces.push([kind - 1, x.dl_code_pace_count() >>> 0]);
  }
  return { name, tracks, paces };
}

// What `text` is, read whole: 'loop', 'album', 'newer' (a newer build's),
// 'instrument' (needs one this device lacks) or null (no code).
export function codeKind(text) {
  need();
  const len = input(utf8.encode(String(text || '')));
  if (x.dl_code_kind(len) === 2) {
    if (x.dl_code_album(len)) return 'album';
  } else if (x.dl_code_read(len)) {
    dropAsked(x.dl_code_seed());
    return 'loop';
  }
  // A newer build's code says so from its head, loop or album; a code that
  // names an instrument this device does not have (the core says so in
  // words with "instrument" in them, once it can) its own line.
  const why = refused().message;
  return why === NEWER ? 'newer' : /instrument/i.test(why) ? 'instrument' : null;
}

const NEWER = 'needs a newer version';

// Why a code can't carry `name` (the core's line), or null if it can.
export function nameRefusal(name) {
  need();
  return x.dl_code_name_ok(input(utf8.encode(String(name)))) ? null : refused().message;
}
