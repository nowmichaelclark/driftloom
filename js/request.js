// A loop's record and its request, as the page keeps and sends them (queue
// item 97; this was the part of js/generator.js that decided no notes).
//
// The page keeps a loop as a spec: its seed, what the core said it made of
// it (js/gen.js: the name, the key, the tempo, the mix and the rest, for the
// card, the cover, the Library and a share code), what was asked of it (the
// request: vibes, tempo, pins, a theme, a length, keys, modes, styles,
// voices, layers) and what a listener changed since (a layer re-rolled, the
// tone, the tempo, the mutes). Here: reading a request (strictly, with the
// core's own short lines for what can't be met), writing it as the numbers
// the core's commands take, and the few sums about a piece's length the
// pickers show. Nothing here draws anything: the core's generator does.

import { randomSeed } from './rng.js';
import { NOTE_NAMES, SCALE_ORDER } from './theory.js';
import { PROFILE_ORDER, VOICE_ORDER } from './characters.js';
import { VIBES, VIBE_IDS } from './vibes.js';
import { RequestError, isPinName, pinValue, readPins, framedPins, pinItems, pinNumbers } from './pins.js';
import { blockOf, readBlock } from './data.js';

// The layers, in the order the core draws and the app lists them, and the
// label each shows: data/voices.txt (queue item 78). Share codes freeze
// their own copy of the order (js/share.js), on purpose.
const LAYER_DATA = readBlock(blockOf('voices', 'voices', 'layers'), { order: 'words', 'label.*': 'word' }, 'data/voices.txt');
export const LAYERS = LAYER_DATA.order;
export const LAYER_LABELS = LAYER_DATA.label;

// The default, for a spec that says nothing: 16 is 4/4, 12 is 6/8 (or 3/4,
// when the loop's gait reads it as a waltz), 20 is 5/4.
export const STEPS_PER_BAR = 16;

// ---------------------------------------------------------------- pieces

// A piece of a length asked (promise 11, queue item 88;
// `core/src/generator/piece.rs`). A short length is the loop's own; past the
// longest any profile draws, and for 'endless', the loop keeps the bars its
// profile drew and the piece is that loop played section after section.

// The piece of the loop of `spec`, or null for a loop with no length asked:
// { bars: a section's, sections: a round's (Infinity for 'endless'), last:
// the bars a round's last section plays }.
export function pieceOf(spec) {
  if (!spec || spec.length == null || !(spec.bars >= 1)) return null;
  const bars = spec.bars;
  if (spec.length === 'endless') return { bars, sections: Infinity, last: bars };
  // In whole numbers, so a length near 2^53 counts exactly.
  const rest = spec.length % bars;
  const sections = (spec.length - rest) / bars + (rest ? 1 : 0);
  return { bars, sections, last: rest || bars };
}

// The bars a pass of the loop of `spec` plays (queue item 88b): the length
// asked (Infinity for 'endless'), else the loop's own. What the card shows,
// and what the length slider counts a pass as.
export function loopBars(spec) {
  if (!spec) return 0;
  if (spec.length === 'endless') return Infinity;
  return typeof spec.length === 'number' ? spec.length : spec.bars;
}

// How long `bars` bars of `stepsPerBar` steps play at `bpm`, in seconds (the
// Rust's `piece::seconds`, the same operations in the same order): what the
// Bars picker shows, so the app and the core never disagree.
export function pieceSeconds(bars, stepsPerBar, bpm) {
  return bars * stepsPerBar * (60 / bpm / 4);
}

// How long `bars` bars play across a tempo range [lo, hi], shortest first
// (`piece::time`).
export function pieceTime(bars, stepsPerBar, tempo) {
  return [pieceSeconds(bars, stepsPerBar, Math.max(tempo[0], tempo[1])), pieceSeconds(bars, stepsPerBar, Math.min(tempo[0], tempo[1]))];
}

// ------------------------------------------------------------- vibes

// The listener's selection, in the frozen order: known names only, each
// once, and the order they were tapped in does not change the loop.
export function vibeSelection(selection) {
  if (!selection) return [];
  const list = Array.isArray(selection) ? selection : Object.keys(selection);
  const set = new Set(list);
  return VIBE_IDS.filter((id) => set.has(id) && VIBES[id]);
}

// ------------------------------------------------------------- the request

// Queue item 85 (promise 11): what a host asks of the next loop, beside its
// seed. One shape for the app's picks, a developer's code and a game's
// events, read the same way by the Rust generator (`core/src/generator/
// request.rs`): the vibes, with or without weights, the tempo, pinned or a
// range, the theme (queue item 85d: a seed whose loop this one bends) and
// the pins (queue item 85b, js/pins.js) and the length (queue item 88: a
// number of bars or 'endless', `pieceOf`), the key and the mode (queue item
// 89: each one, or a set to choose from) and the style (queue item 90: the
// profiles, one or a set). Whatever is not asked is drawn
// from the seed as ever, and a request of nothing is the loop the seed makes.
//
// It comes as:
//   - nothing (null), or a list of vibe names (a listener's selection: names
//     the app does not know are left out, as they always were);
//   - text, as the goldens, `dlrender --request` and the C host write it:
//     items split by commas, each a vibe ("glacial"), a vibe and its weight
//     ("glacial=0.7"), the tempo ("bpm=77", "bpm=70-85"), the theme
//     ("theme=1234"), a pin ("organ=on"), the length ("bars=1024",
//     "bars=endless"), the key ("key=D", "key=D|G"), the mode
//     ("scale=dorian", "scale=dorian|aeolian") or the style ("style=tide",
//     "style=tide|shrine"); "" or "-" for nothing;
//   - an object, { vibes, tempo, theme, pins, length, key, scale, style }: vibes a
//     list of names, { name: weight } or [name, weight] pairs (the read form,
//     below), tempo a number or [low, high], theme a seed, pins { name:
//     value }, length a whole number of bars or 'endless', key a note's name
//     or number (C is 0) or a list of them, scale a mode's name or a list,
//     style a profile's name or a list.
// Text and objects are read strictly: a name the core does not have, a bad
// weight or a tempo out of the core's range is refused (a RequestError, its
// message the short plain ASCII line "can't do that" shows), never guessed.
//
// Read, it is { vibes, tempo, theme, pins, length, key, scale, style }: vibes null
// or [id, weight] pairs in the frozen order (the weight null where none was
// given), tempo null or [low, high], whole bpm (equal when pinned), theme
// null or a seed, pins null or { name: value } in the pins' order, length
// null, a number of bars or 'endless', key null or home notes (numbers, low
// first), scale null or modes' names in their frozen order, style null or
// profile names in theirs.

// The tempos the core plays, in bpm: what the share code's tempo byte holds.
export const TEMPO_LIMIT = [32, 190];

export { RequestError };

const WEIGHT_TEXT = /^\d+(\.\d+)?$/;
const BPM_TEXT = /^\d+$/;

export function readRequest(input) {
  const out = { vibes: null, tempo: null, theme: null, pins: null, length: null, key: null, scale: null, style: null, voices: null, layers: null };
  if (input == null) return out;
  if (typeof input === 'string') return parseRequest(input);
  if (Array.isArray(input)) return { ...out, vibes: lenientVibes(input) };
  if (typeof input !== 'object') throw new RequestError('a request is text, a list or an object');
  if (!['vibes', 'tempo', 'theme', 'pins', 'length', 'key', 'scale', 'style', 'voices', 'layers'].some((k) => k in input)) return { ...out, vibes: lenientVibes(Object.keys(input)) };
  const vibes = input.vibes;
  if (Array.isArray(vibes) && vibes.some(Array.isArray)) {
    // The read form's own pairs, so a request read is read the same again.
    const asked = new Map();
    for (const [id, w] of vibes) asked.set(knownVibe(id), w == null ? null : weightOf(w));
    out.vibes = framed(asked);
  } else if (Array.isArray(vibes)) out.vibes = lenientVibes(vibes);
  else if (vibes && typeof vibes === 'object') {
    const asked = new Map();
    for (const [id, w] of Object.entries(vibes)) asked.set(knownVibe(id), w == null || w === true ? null : weightOf(w));
    out.vibes = framed(asked);
  } else if (vibes != null) throw new RequestError('vibes are a list or names with weights');
  if (input.tempo != null) {
    const t = input.tempo;
    out.tempo = Array.isArray(t) ? tempoOf(t[0], t.length > 1 ? t[1] : t[0]) : tempoOf(t, t);
  }
  // The theme (queue item 85d): a seed.
  if (input.theme != null) out.theme = themeOf(input.theme);
  // The pins (queue item 85b; js/pins.js), { name: value }.
  out.pins = readPins(input.pins);
  // The length (queue item 88): bars, or 'endless'.
  if (input.length != null) out.length = lengthOf(input.length);
  // The key and the mode (queue item 89): a name or a set of them.
  if (input.key != null) out.key = keysOf(input.key);
  if (input.scale != null) out.scale = scalesOf(input.scale);
  // The style (queue item 90): a profile or a set of them.
  if (input.style != null) out.style = stylesOf(input.style);
  // The voices (queue item 91): { layer: a voice or a set }, and the layers
  // on or off: { layer: 'on' | 'off' }.
  if (input.voices != null) out.voices = voicesOf(input.voices);
  if (input.layers != null) out.layers = layersOf(input.layers);
  return out;
}

export function parseRequest(text) {
  const out = { vibes: null, tempo: null, theme: null, pins: null, length: null, key: null, scale: null, style: null, voices: null, layers: null };
  const t = String(text).trim();
  if (t === '' || t === '-') return out;
  const asked = new Map();
  const pins = {};
  const voices = {};
  const layers = {};
  for (const item of t.split(',')) {
    const eq = item.indexOf('=');
    const name = eq < 0 ? item : item.slice(0, eq);
    const value = eq < 0 ? null : item.slice(eq + 1);
    if (name === 'bpm') {
      if (value == null) throw new RequestError('a bad tempo');
      const dash = value.indexOf('-');
      const [lo, hi] = dash < 0 ? [value, value] : [value.slice(0, dash), value.slice(dash + 1)];
      if (!BPM_TEXT.test(lo) || !BPM_TEXT.test(hi)) throw new RequestError('a bad tempo');
      out.tempo = tempoOf(Number(lo), Number(hi));
    } else if (name === 'bars') {
      if (value !== 'endless' && (value == null || !BPM_TEXT.test(value))) throw new RequestError('a bad length');
      out.length = lengthOf(value === 'endless' ? value : Number(value));
    } else if (name === 'theme') {
      if (value == null || !BPM_TEXT.test(value)) throw new RequestError('a bad theme');
      out.theme = themeOf(Number(value));
    } else if (name === 'key') {
      out.key = keysOf(value == null ? '' : value);
    } else if (name === 'scale') {
      out.scale = scalesOf(value == null ? '' : value);
    } else if (name === 'style') {
      out.style = stylesOf(value == null ? '' : value);
    } else if (name.startsWith('voice.')) {
      voices[name.slice(6)] = value == null ? '' : value;
    } else if (name.startsWith('layer.')) {
      layers[name.slice(6)] = value == null ? '' : value;
    } else if (isPinName(name)) {
      // A pin (queue item 85b): a choice's id, or a dotted setting.
      if (value == null) throw new RequestError('a bad setting');
      pins[name] = pinValue(name, value);
    } else {
      const id = knownVibe(name);
      if (value != null && !WEIGHT_TEXT.test(value)) throw new RequestError('a bad weight');
      asked.set(id, value == null ? null : weightOf(Number(value)));
    }
  }
  out.vibes = framed(asked);
  out.pins = framedPins(pins);
  if (Object.keys(voices).length) out.voices = voicesOf(voices);
  if (Object.keys(layers).length) out.layers = layersOf(layers);
  return out;
}

// The request as text (`parseRequest` reads it back the same), or '-'.
export function requestText(request) {
  const r = readRequest(request);
  const items = (r.vibes || []).map(([id, w]) => (w == null ? id : `${id}=${w}`));
  if (r.tempo) items.push(`bpm=${r.tempo[0] === r.tempo[1] ? r.tempo[0] : `${r.tempo[0]}-${r.tempo[1]}`}`);
  if (r.length != null) items.push(`bars=${r.length}`);
  if (r.theme != null) items.push(`theme=${r.theme}`);
  if (r.key) items.push(`key=${r.key.map((k) => NOTE_NAMES[k]).join('|')}`);
  if (r.scale) items.push(`scale=${r.scale.join('|')}`);
  if (r.style) items.push(`style=${r.style.join('|')}`);
  if (r.voices) for (const layer of LAYERS) if (r.voices[layer]) items.push(`voice.${layer}=${r.voices[layer].join('|')}`);
  if (r.layers) for (const layer of LAYERS) if (r.layers[layer]) items.push(`layer.${layer}=${r.layers[layer]}`);
  items.push(...pinItems(r.pins));
  return items.length ? items.join(',') : '-';
}

// What makes a spec's loop again: its seed with this request (the player's
// `?engine=player`, a share code's loop on the core). Its own vibes with the
// weights it drew, and the tempo it was asked for.
export function requestOf(spec) {
  const v = vibesOf(spec);
  return {
    vibes: v ? Object.fromEntries(VIBE_IDS.filter((id) => v.vibes[id] > 0).map((id) => [id, v.vibes[id]])) : null,
    tempo: spec && Array.isArray(spec.tempo) ? spec.tempo.slice() : null,
    theme: spec && spec.theme != null ? spec.theme : null,
    pins: spec && spec.pins ? { ...spec.pins } : null,
    length: spec && spec.length != null ? spec.length : null,
    key: spec && Array.isArray(spec.askKey) ? spec.askKey.slice() : null,
    scale: spec && Array.isArray(spec.askScale) ? spec.askScale.slice() : null,
    style: spec && Array.isArray(spec.askStyle) ? spec.askStyle.slice() : null,
    voices: spec && spec.askVoices ? Object.fromEntries(Object.entries(spec.askVoices).map(([l, v]) => [l, v.slice()])) : null,
    layers: spec && spec.askLayers ? { ...spec.askLayers } : null,
  };
}

// A request's keys and modes as the bits the core takes (`Request::keys`,
// `Request::scales`): a bit each, C and the first mode the lowest; 0 for none.
export function keyMask(keys) {
  return keys ? keys.reduce((m, k) => m | (1 << k), 0) : 0;
}
export function scaleMask(scales) {
  return scales ? scales.reduce((m, k) => m | (1 << SCALE_ORDER.indexOf(k)), 0) : 0;
}

// A style or a set of styles (queue item 90), by profile name ("tide"), a
// list or names written with "|" between; read in the profiles' frozen order.
function stylesOf(x) {
  const list = Array.isArray(x) ? x : typeof x === 'string' ? x.split('|') : [x];
  if (!list.length) return null;
  const asked = new Set();
  for (const k of list) {
    if (typeof k !== 'string' || !PROFILE_ORDER.includes(k)) throw new RequestError('no such style');
    asked.add(k);
  }
  return PROFILE_ORDER.filter((k) => asked.has(k));
}

// The voices asked (queue item 91), { layer: a voice, a list, or names
// written with "|" between }: each layer's names from its own list
// (`VOICE_ORDER`), read in that list's frozen order. A layer with none is
// nothing asked.
function voicesOf(x) {
  if (typeof x !== 'object' || Array.isArray(x)) throw new RequestError('no such layer');
  const out = {};
  for (const layer of LAYERS) {
    if (!Object.hasOwn(x, layer)) continue;
    const v = x[layer];
    const list = Array.isArray(v) ? v : typeof v === 'string' ? v.split('|') : [v];
    if (!list.length) continue;
    const asked = new Set();
    for (const k of list) {
      if (typeof k !== 'string' || !VOICE_ORDER[layer].includes(k)) throw new RequestError('no such voice');
      asked.add(k);
    }
    out[layer] = VOICE_ORDER[layer].filter((k) => asked.has(k));
  }
  for (const k of Object.keys(x)) if (!LAYERS.includes(k)) throw new RequestError('no such layer');
  return Object.keys(out).length ? out : null;
}

// The layers asked on or off (queue item 91), { layer: 'on' | 'off' }. All
// five off is the one request that cannot be met.
function layersOf(x) {
  if (typeof x !== 'object' || x === null || Array.isArray(x)) throw new RequestError('no such layer');
  for (const k of Object.keys(x)) if (!LAYERS.includes(k)) throw new RequestError('no such layer');
  const out = {};
  for (const layer of LAYERS) {
    if (!Object.hasOwn(x, layer) || x[layer] == null) continue;
    if (x[layer] !== 'on' && x[layer] !== 'off') throw new RequestError('a bad layer');
    out[layer] = x[layer];
  }
  if (LAYERS.every((l) => out[l] === 'off')) throw new RequestError('all layers off');
  return Object.keys(out).length ? out : null;
}

// A request's voices as the bits the core takes (`Request::voices`): a mask
// for a layer, a bit each by place in the layer's frozen list; 0 for none.
export function voiceMask(voices, layer) {
  return voices && voices[layer] ? voices[layer].reduce((m, k) => m | (1 << VOICE_ORDER[layer].indexOf(k)), 0) : 0;
}

// A layer's state as the core takes it: 0 chance, 1 on, 2 off.
export function layerState(layers, layer) {
  return layers && layers[layer] === 'on' ? 1 : layers && layers[layer] === 'off' ? 2 : 0;
}

// A request's styles as the bits the core takes (`Request::styles`): a bit
// each by place in the profiles' frozen order; 0 for none.
export function styleMask(styles) {
  return styles ? styles.reduce((m, k) => m | (1 << PROFILE_ORDER.indexOf(k)), 0) : 0;
}

// Home notes by their names, either spelling of a black key.
const KEY_NAMES = {
  C: 0, 'C#': 1, Db: 1, D: 2, 'D#': 3, Eb: 3, E: 4, F: 5, 'F#': 6, Gb: 6, G: 7, 'G#': 8, Ab: 8, A: 9, 'A#': 10, Bb: 10, B: 11,
};

// A key or a set of keys (queue item 89): a note's name ("D", "Eb"), its
// number from 0 (C) to 11, a list of them, or names written with "|" between
// ("D|G"). Read as the numbers, once each, low first; an empty list is
// nothing asked.
function keysOf(x) {
  const list = Array.isArray(x) ? x : typeof x === 'string' ? x.split('|') : [x];
  if (!list.length) return null;
  const keys = new Set();
  for (const k of list) {
    const n = typeof k === 'number' && Number.isInteger(k) && k >= 0 && k <= 11 ? k
      : typeof k === 'string' && Object.hasOwn(KEY_NAMES, k) ? KEY_NAMES[k] : -1;
    if (n < 0) throw new RequestError('no such key');
    keys.add(n);
  }
  return [...keys].sort((a, b) => a - b);
}

// A mode or a set of modes (queue item 89), by name ("dorian"), a list or
// names written with "|" between; read in the modes' frozen order.
function scalesOf(x) {
  const list = Array.isArray(x) ? x : typeof x === 'string' ? x.split('|') : [x];
  if (!list.length) return null;
  const asked = new Set();
  for (const k of list) {
    if (typeof k !== 'string' || !SCALE_ORDER.includes(k)) throw new RequestError('no such scale');
    asked.add(k);
  }
  return SCALE_ORDER.filter((k) => asked.has(k));
}

function lenientVibes(list) {
  const chosen = vibeSelection(list);
  return chosen.length ? chosen.map((id) => [id, null]) : null;
}

function knownVibe(id) {
  if (!VIBES[id] || VIBE_IDS.indexOf(id) < 0) throw new RequestError('no such vibe');
  return id;
}

function weightOf(w) {
  if (typeof w !== 'number' || !Number.isFinite(w) || w < 0) throw new RequestError('a bad weight');
  return w;
}

// Asked vibes in the frozen order; a vibe asked twice keeps the last.
function framed(asked) {
  const list = VIBE_IDS.filter((id) => asked.has(id)).map((id) => [id, asked.get(id)]);
  return list.length ? list : null;
}

// The longest length a request holds, in bars (`LENGTH_MOST`): 2^53 - 1,
// so a length written longer never rounds into range. No cap on the music.
export const LENGTH_MOST = Number.MAX_SAFE_INTEGER;

// A length: a whole number of bars from 1 to LENGTH_MOST, or 'endless'.
function lengthOf(x) {
  if (x === 'endless') return x;
  if (typeof x !== 'number' || !Number.isInteger(x) || x < 1 || x > LENGTH_MOST) throw new RequestError('a bad length');
  return x;
}

// A theme: a seed, a whole number from 0 to 2^32 - 1, or refused.
function themeOf(x) {
  if (typeof x !== 'number' || !Number.isInteger(x) || x < 0 || x > 0xffffffff) throw new RequestError('a bad theme');
  return x;
}

// A tempo, whole bpm, low first, inside the core's range or refused.
function tempoOf(a, b) {
  if (typeof a !== 'number' || typeof b !== 'number' || !Number.isFinite(a) || !Number.isFinite(b)) {
    throw new RequestError('a bad tempo');
  }
  let lo = Math.round(a);
  let hi = Math.round(b);
  if (lo > hi) [lo, hi] = [hi, lo];
  if (lo < TEMPO_LIMIT[0] || hi > TEMPO_LIMIT[1]) throw new RequestError('tempo must be 32 to 190 bpm');
  return [lo, hi];
}

// How far a band [lo, hi] sits from a tempo asked [a, b], in bpm: 0 where

// What a spec says it was steered by, or null. Unknown names (a recipe
// renamed since) are left out rather than guessed at.
export function vibesOf(spec) {
  if (!spec || !spec.vibes) return null;
  const vibes = {};
  for (const [id, w] of Object.entries(spec.vibes)) if (VIBES[id] && w > 0) vibes[id] = w;
  if (!Object.keys(vibes).length) return null;
  return { vibes, strength: spec.vibeStrength ?? 0.7 };
}

// A target: the draw moves part-way toward the weighted mean of the vibes

// ------------------------------------------------------------- edits

// A layer drawn again: a new seed for it, and marked re-rolled, so the core
// plays it whatever its own chances draw (queue item 94a).
export function rerollLayer(spec, layer) {
  const next = cloneSpec(spec);
  next.layerSeeds[layer] = randomSeed();
  next.forceLayers = { ...(next.forceLayers || {}), [layer]: true };
  return next;
}

export function cloneSpec(spec) {
  return JSON.parse(JSON.stringify(spec));
}

// ------------------------------------------------- what the core is asked

// What the core is asked for a spec's loop (`CoreHost.playLoad` and the
// rest): its request (`requestOf`: the vibes with the weights it drew, so the
// loop a selection of five or fewer made, the tempo, pins, theme, length,
// keys, modes, styles, voices and layers), and a code's edits (queue item
// 94a): every layer's seed, with whether it was re-rolled, the tone and the
// tempo. The edits are sent whole, whether or not they differ from what the
// seed draws: an edit to the value drawn makes the same loop
// (test/player.test.mjs holds it), and the page need not draw the loop to
// tell.
export function askOf(spec) {
  const asked = requestOf(spec);
  const vibes = Object.entries(asked.vibes || {}).map(([id, w]) => [VIBE_IDS.indexOf(id), w]).filter(([i]) => i >= 0);
  const seeds = spec.layerSeeds
    ? LAYERS.map((l, i) => [i, spec.layerSeeds[l] >>> 0, !!(spec.forceLayers && spec.forceLayers[l])]).filter(([, s]) => Number.isFinite(s))
    : [];
  const tone = spec.tone ? [spec.tone.warmth, spec.tone.space, spec.tone.wobble].map((v) => (Number.isFinite(v) ? v : null)) : null;
  return {
    vibes,
    tempo: asked.tempo,
    pins: pinNumbers(asked.pins),
    theme: asked.theme,
    length: asked.length,
    keys: keyMask(asked.key),
    scales: scaleMask(asked.scale),
    styles: styleMask(asked.style),
    voices: LAYERS.map((l) => voiceMask(asked.voices, l)),
    layers: LAYERS.map((l) => layerState(asked.layers, l)),
    seeds,
    tone: tone && tone.every((v) => v != null) ? tone : null,
    bpm: Number.isFinite(spec.bpm) ? spec.bpm : null,
  };
}

// A request read (`readRequest`), in the core's numbers: the vibes by their
// place in the frozen list, each with its weight or none, and the keys, the
// modes, the styles, the voices and the layers as the bits the core takes.
export function askFor(asked) {
  return {
    vibes: (asked.vibes || []).map(([id, w]) => [VIBE_IDS.indexOf(id), w]).filter(([i]) => i >= 0),
    tempo: asked.tempo,
    pins: pinNumbers(asked.pins),
    theme: asked.theme,
    length: asked.length,
    keys: keyMask(asked.key),
    scales: scaleMask(asked.scale),
    styles: styleMask(asked.style),
    voices: LAYERS.map((l) => voiceMask(asked.voices, l)),
    layers: LAYERS.map((l) => layerState(asked.layers, l)),
  };
}

// The New loops panel's request as the core takes it (queue item 95,
// `Player::set_picks`): the request read and put in the core's numbers, the
// vibes a listener's selection, each with no weight. A request that can't be
// read is sent as its vibes alone, the moods kept: { ask, vibesOnly, why }.
export function picksAsk(request) {
  try {
    return { ask: askFor(readRequest(request)), vibesOnly: false, why: null };
  } catch (e) {
    if (!(e instanceof RequestError)) throw e;
    const vibes = request && Array.isArray(request.vibes) ? request.vibes : [];
    return { ask: askFor(readRequest(vibes)), vibesOnly: true, why: e.message };
  }
}
