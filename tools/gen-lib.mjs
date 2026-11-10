// The core's generator, whole, for the tools (queue item 97): everything it
// decides about a loop, read back from its canonical texts (the generation
// goldens' parts, tools/gen-golden.mjs). The page's own reader (js/gen.js)
// asks the module that plays for a loop's record and its notes; a tool that
// measures what is generated wants the rest too (each note's voice, the
// meta, the per-loop choices, what is drawn at load, the wandered passes),
// and that is in the tools' build only:
//
//   tools/build-core.sh --tools     writes core/target/dltools.wasm
//
//   newSpec(seed, request)   the loop's spec, as the core writes it. The
//                            request is text ("cozy,bpm=77,bars=64"), a list
//                            of vibes, or null.
//   render(spec)             what the core decided for it: { spec, tracks,
//                            meta, choices, form, gaps, silentBars,
//                            totalSteps, stepsPerBar, cycles, harmony }.
//                            The spec is one `newSpec` made, or the page's
//                            record of a loop (a share code decoded,
//                            js/share.js): its request and its edits are
//                            written as the text the core reads (`askText`)
//   metreOf(spec)            the loop's metre, as the player describes it
//                            ('4/4', '6/8', '3/4', '5/4')
//   parts(seed, request)     every part by name: spec, plan, form, the five
//                            layers, meta, choices, load, passes (a list of
//                            eight)
//   hashes(seed, request)    each part's hash, as the goldens' line has them
//
// There was a JavaScript generator beside the core's until item 97, held to
// it by the goldens; this reads the one there is.

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseCanonical } from './gen-golden-lib.mjs';
import { requestOf, requestText } from '../js/request.js';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
export const TOOLS_WASM = path.join(ROOT, 'core', 'target', 'dltools.wasm');
export const LAYERS = ['drums', 'bass', 'chords', 'melody', 'texture'];
// The parts, in the order the exports number them (`Part::ALL`).
export const PARTS = ['spec', 'plan', 'form', ...LAYERS, 'meta', 'choices', 'load'];
export const PASSES = 8;

let x = null;
/** Read the module at `file` (default: the tools' build). */
export function open(file = TOOLS_WASM) {
  if (!fs.existsSync(file)) throw new Error(`${path.relative(ROOT, file)} is missing: build it with  tools/build-core.sh --tools`);
  x = new WebAssembly.Instance(new WebAssembly.Module(fs.readFileSync(file)), {}).exports;
  return x;
}

/** A request as the text the core reads: text as it is, a list of vibes joined, nothing as ''. */
export function textOf(request) {
  if (request == null || request === '-') return '';
  if (Array.isArray(request)) return request.join(',');
  if (typeof request === 'string') return request;
  throw new Error('gen-lib: a request is text, a list of vibes or null');
}

// The request each spec was made with: `render` generates the loop again.
const ASKED = new WeakMap();

function load(seed, request) {
  if (!x) open();
  const asked = new TextEncoder().encode(textOf(request));
  new Uint8Array(x.memory.buffer, x.dl_gen_input(), asked.length).set(asked);
  if (!x.dl_gen_request(seed >>> 0, asked.length)) throw new Error(`the core refused "${textOf(request)}" for seed ${seed >>> 0}`);
}

function text(i) {
  const len = x.dl_gen_part(i);
  if (!len) throw new Error(`the core wrote no text for part ${i}`);
  return new TextDecoder('ascii').decode(new Uint8Array(x.memory.buffer, x.dl_gen_text(), len));
}

const hex = (h) => BigInt.asUintN(64, h).toString(16).padStart(16, '0');

/** The loop's spec. */
export function newSpec(seed, request = null) {
  load(seed, request);
  const spec = parseCanonical(text(0));
  ASKED.set(spec, textOf(request));
  return spec;
}

/**
 * A page's record of a loop as the request text that makes it again: its
 * request (`requestOf`) and a code's edits, whole, as js/request.js's
 * `askOf` sends them to the core that plays (every layer's seed, re-rolled
 * or not, the tone, the tempo).
 */
export function askText(spec) {
  const asked = requestText(requestOf(spec));
  const items = asked === '-' ? [] : [asked];
  for (const layer of LAYERS) {
    const seed = spec.layerSeeds && spec.layerSeeds[layer];
    if (Number.isFinite(seed)) items.push(`edit.${spec.forceLayers && spec.forceLayers[layer] ? 'roll' : 'seed'}.${layer}=${seed >>> 0}`);
  }
  for (const k of ['warmth', 'space', 'wobble']) if (spec.tone && Number.isFinite(spec.tone[k])) items.push(`edit.${k}=${spec.tone[k]}`);
  if (Number.isFinite(spec.bpm)) items.push(`edit.bpm=${spec.bpm}`);
  return items.join(',');
}

const textFor = (spec) => (ASKED.has(spec) ? ASKED.get(spec) : askText(spec));

/** What the core decided for a spec: one `newSpec` made, or a page's record. */
export function render(spec) {
  load(spec.seed, textFor(spec));
  const form = parseCanonical(text(2));
  const tracks = {};
  LAYERS.forEach((layer, i) => { tracks[layer] = parseCanonical(text(3 + i)); });
  return { spec, tracks, meta: parseCanonical(text(8)), choices: parseCanonical(text(9)), ...form };
}

/** The loop's metre, as the player describes it. */
export function metreOf(spec) {
  if (!x) open();
  if (!metreOf.ready) {
    x.dl_init(48000);
    x.dl_quality(0);
    x.dl_play_attach(0);
    metreOf.ready = true;
  }
  x.dl_play_stop();
  const asked = new TextEncoder().encode(textFor(spec));
  new Uint8Array(x.memory.buffer, x.dl_tools_input(), asked.length).set(asked);
  if (asked.length && !x.dl_tools_ask(asked.length)) throw new Error(`the core refused "${textFor(spec)}"`);
  x.dl_play_load(spec.seed >>> 0, 0);
  x.dl_play_ready();
  const about = new TextDecoder('ascii').decode(new Uint8Array(x.memory.buffer, x.dl_play_about_at(), x.dl_play_describe()));
  return /^metre=(.*)$/m.exec(about)[1];
}

/** Every part of the loop, by name; `canon` for the texts themselves. */
export function parts(seed, request = null, canon = false) {
  load(seed, request);
  const read = canon ? (t) => t : parseCanonical;
  const out = Object.fromEntries(PARTS.map((name, i) => [name, read(text(i))]));
  out.passes = Array.from({ length: PASSES }, (_, i) => read(text(PARTS.length + i)));
  return out;
}

/** Each part's hash: { spec, plan, ..., load, passes: [8] }, and whether any list lost an item. */
export function hashes(seed, request = null) {
  load(seed, request);
  const out = Object.fromEntries(PARTS.map((name, i) => [name, hex(x.dl_gen_hash(i))]));
  out.passes = Array.from({ length: PASSES }, (_, i) => hex(x.dl_gen_hash(PARTS.length + i)));
  return { hashes: out, lost: x.dl_gen_lost() };
}

// How bright each mode reads, 0 (inward) to 1 (glad): the measuring tools'
// copy of the table the core steers its scale draw with
// (`core/src/generator/theory.rs`, each scale's `bright`), for the mode bands the
// reports print and the dials read.
export const SCALE_BRIGHTNESS = {
  lydian: 1.0,
  ionian: 0.92,
  majorPent: 0.9,
  yo: 0.82,
  lydianDominant: 0.78,
  mixolydian: 0.75,
  ritusen: 0.68,
  kumoi: 0.52,
  akebono: 0.48,
  dorian: 0.46,
  wholeTone: 0.44,
  dorianSharp4: 0.4,
  minorPent: 0.34,
  insen: 0.3,
  hirajoshi: 0.28,
  aeolian: 0.24,
  phrygianDominant: 0.2,
  harmonicMinor: 0.14,
  phrygian: 0.08,
};
