// The page's reader of the core's generator (js/gen.js, queue item 97) and
// the player's commands and events in js/dlplayer.wasm (queue item 94a),
// against the core's own account of the same loop (tools/gen-lib.mjs: the
// canonical texts the generation goldens hash, from the tools' build): the
// page's record of a loop is the core's spec, field for field; its notes are
// the core's tracks; the layers' grid of each pass as it wanders is the
// core's wandered pass; a code's edits (a layer re-rolled, the tone, the
// tempo) are made as the core makes them; and the events say where the loop
// is, through a re-roll in its place and the track's end. No browser: Node
// runs the modules. Needs core/target/dltools.wasm (tools/build-core.sh
// --tools; the gate builds it).
// Run with:  node test/player.test.mjs

import fs from 'node:fs';
import { fileURLToPath } from 'node:url';
import { Rng } from '../js/rng.js';
import { newSpec, render } from '../js/gen.js';
import { LAYERS, parseRequest, keyMask, scaleMask, styleMask, voiceMask, layerState, cloneSpec } from '../js/request.js';
import { VIBE_IDS } from '../js/vibes.js';
import { PINS, pinNumbers } from '../js/pins.js';
import { casesOf, canonical } from '../tools/gen-golden-lib.mjs';
import * as core from '../tools/gen-lib.mjs';

try {
  core.open();
} catch (err) {
  console.error(`player.test: ${err.message}`);
  process.exit(3);
}
const CASES = casesOf(fs.readFileSync(fileURLToPath(new URL('../core/golden/generation.hashes', import.meta.url)), 'utf8'));

// The page's record of `seed` with `request` (text or null), its `edit.`
// items made as the app makes them (queue item 94a): a re-roll as
// `rerollLayer` makes it (a copy, the layer's seed, `forceLayers`), a seed
// alone, the Sound panel's tone, the tempo slider's bpm.
function edited(seed, request) {
  if (typeof request !== 'string' || !request.includes('edit.')) return newSpec(seed, request);
  const items = request.split(',');
  const asked = items.filter((x) => !x.startsWith('edit.')).join(',');
  let spec = cloneSpec(newSpec(seed, asked || null));
  for (const item of items.filter((x) => x.startsWith('edit.'))) {
    const [name, value] = item.slice(5).split('=');
    const [kind, layer] = name.split('.');
    if (kind === 'roll') {
      spec = cloneSpec(spec);
      spec.layerSeeds[layer] = Number(value) >>> 0;
      spec.forceLayers = { ...(spec.forceLayers || {}), [layer]: true };
    } else if (kind === 'seed') {
      spec.layerSeeds[layer] = Number(value) >>> 0;
    } else if (kind === 'bpm') {
      spec.bpm = parseInt(value, 10);
    } else {
      spec.tone[kind] = parseFloat(value);
    }
  }
  return spec;
}

let failures = 0;
function check(label, condition, detail = '') {
  if (!condition) {
    console.log(`  FAIL  ${label} ${detail}`);
    failures++;
  }
  return condition;
}

const bytes = fs.readFileSync(fileURLToPath(new URL('../js/dlplayer.wasm', import.meta.url)));
const module = new WebAssembly.Module(bytes);
const RATE = 44100;
const QUANTUM = 128;
const TONE = ['warmth', 'space', 'wobble'];

// A player playing `seed` as `text` asks (the request and its `edit.` items,
// as the generation goldens write them), made at once, wandering as the
// app's default.
function start(seed, text = '') {
  const { exports: x } = new WebAssembly.Instance(module, {});
  x.dl_init(RATE);
  x.dl_quality(1);
  x.dl_play_wander(1);
  x.dl_play_attach(1);
  ask(x, text);
  x.dl_play_load(seed >>> 0, 0);
  x.dl_play_ready();
  return x;
}

// What `text` asks, call by call, as a page that decoded a code would.
function ask(x, text) {
  const items = text ? text.split(',') : [];
  const asked = parseRequest(items.filter((i) => !i.startsWith('edit.')).join(','));
  for (const [id, w] of asked.vibes || []) {
    const i = VIBE_IDS.indexOf(id);
    if (!(w == null ? x.dl_play_vibe(i) : x.dl_play_weight(i, w))) throw new Error(`refused ${id}`);
  }
  if (asked.tempo) x.dl_play_ask_tempo(asked.tempo[0], asked.tempo[1]);
  for (const [i, v] of pinNumbers(asked.pins)) x.dl_play_pin(i, v);
  if (asked.theme != null) x.dl_play_theme(asked.theme >>> 0);
  if (asked.length != null) x.dl_play_length(asked.length === 'endless' ? Infinity : asked.length);
  if (keyMask(asked.key)) x.dl_play_keys(keyMask(asked.key));
  if (scaleMask(asked.scale)) x.dl_play_scales(scaleMask(asked.scale));
  if (styleMask(asked.style)) x.dl_play_styles(styleMask(asked.style));
  LAYERS.forEach((l, i) => {
    if (voiceMask(asked.voices, l)) x.dl_play_voices(i, voiceMask(asked.voices, l));
    if (layerState(asked.layers, l)) x.dl_play_layer(i, layerState(asked.layers, l));
  });
  for (const item of items.filter((i) => i.startsWith('edit.'))) {
    const [name, value] = item.slice(5).split('=');
    const [kind, layer] = name.split('.');
    let ok;
    if (kind === 'roll' || kind === 'seed') ok = x.dl_play_seed(LAYERS.indexOf(layer), Number(value) >>> 0, kind === 'roll' ? 1 : 0);
    else if (kind === 'bpm') ok = x.dl_play_edit_bpm(Number(value));
    else ok = x.dl_play_edit_tone(TONE.indexOf(kind), Number(value));
    if (!ok) throw new Error(`refused ${item}`);
  }
}

function about(x) {
  const n = x.dl_play_describe();
  const text = new TextDecoder().decode(new Uint8Array(x.memory.buffer, x.dl_play_about_at(), n));
  return Object.fromEntries(text.split('\n').filter(Boolean).map((l) => [l.slice(0, l.indexOf('=')), l.slice(l.indexOf('=') + 1)]));
}

// Layer `layer`'s notes from the core, as the MIDI export reads them.
function notes(x, layer) {
  const l = LAYERS.indexOf(layer);
  const out = [];
  for (let i = 0; i < x.dl_play_notes(l); i++) {
    const k = x.dl_play_note(l, i);
    const f = new Float64Array(x.memory.buffer, x.dl_play_note_at(), 16);
    const drum = new TextDecoder().decode(new Uint8Array(x.memory.buffer, x.dl_play_text(), k));
    out.push({ step: f[0], dur: f[1], vel: f[2], micro: f[3], notes: Array.from(f.slice(5, 5 + f[4])), drum });
  }
  return out;
}

// The page's, the same way.
function pageNotes(p, layer) {
  return p.tracks[layer].map((e) => ({
    step: e.step, dur: layer === 'drums' ? 0 : e.dur, vel: e.vel, micro: layer === 'drums' ? e.micro || 0 : 0,
    notes: layer === 'drums' ? [] : e.notes ? e.notes.slice() : e.midi != null ? [e.midi] : [], drum: layer === 'drums' ? e.inst : '',
  }));
}

function grid(x, layer) {
  const n = x.dl_play_grid(LAYERS.indexOf(layer));
  return Array.from(new Float64Array(x.memory.buffer, x.dl_play_grid_at(), n));
}

function events(x) {
  const n = x.dl_play_events();
  const f = new Float64Array(x.memory.buffer, x.dl_play_event_at(), n * 4);
  const out = [];
  for (let i = 0; i < n; i++) out.push({ time: f[4 * i], kind: f[4 * i + 1], number: f[4 * i + 2], step: f[4 * i + 3] });
  return out;
}

// The page's grid of a pattern: each layer's loudest note on each step of
// its cycle (`stepsForBar`, read around the whole cycle).
function pageGrid(p, layer) {
  const cycle = (p.cycles && p.cycles[layer]) || p.totalSteps;
  const out = new Array(cycle).fill(0);
  for (const e of p.tracks[layer]) {
    if (!e.vel) continue;
    const i = ((e.step % cycle) + cycle) % cycle;
    out[i] = Math.max(out[i], e.vel);
  }
  return out;
}

const num = (s) => Number(s);
const weights = (s) => Object.fromEntries((s || '').split('|').filter(Boolean).map((p) => [p.split(':')[0], num(p.split(':')[1])]));
const same = (a, b) => JSON.stringify(a) === JSON.stringify(b);

// The core's tracks, as the page's `render` hands them over: each note's
// step, length, loudness and pitches (a drum hit's name and micro-timing).
function coreNotes(tracks, layer) {
  return tracks[layer].map((e) => ({
    step: e.step, dur: layer === 'drums' ? 0 : e.dur, vel: e.vel, micro: layer === 'drums' ? e.micro || 0 : 0,
    notes: layer === 'drums' ? [] : e.notes ? e.notes.slice() : e.midi != null ? [e.midi] : [], drum: layer === 'drums' ? e.inst : '',
  }));
}

// The page's record and notes, and what the player describes, against the
// core's spec and tracks for the same seed and request.
function describes(label, x, spec, c) {
  const d = about(x);
  const p = render(spec);
  const made = core.parts(c.seed, c.request);
  check(`${label}: the page's record is the core's spec`, canonical(spec) === canonical(made.spec), `\n    page ${canonical(spec).slice(0, 300)}\n    core ${canonical(made.spec).slice(0, 300)}`);
  check(`${label}: the page's notes are the core's tracks`, LAYERS.every((l) => same(pageNotes(p, l), coreNotes(made, l))));
  check(`${label}: the page's form and choir`, p.form === !!made.form.form && p.choir === !!made.meta.choir && p.totalSteps === made.form.totalSteps && p.stepsPerBar === made.form.stepsPerBar, `${p.form} ${p.choir}`);
  const want = {
    seed: spec.seed >>> 0, name: spec.name, root: spec.root, scale: spec.scale, bpm: spec.bpm, steps_per_bar: spec.stepsPerBar,
    bars: spec.bars, swing: spec.swing, coherence: spec.coherence,
    feel: [spec.feel.lift, spec.feel.energy, spec.feel.warmth].join('|'),
    tone: TONE.map((k) => spec.tone[k]),
    layer_seeds: LAYERS.map((l) => spec.layerSeeds[l] >>> 0),
    airy: p.form ? 1 : 0, drifting: spec.cycles ? 1 : 0, choir: p.choir ? 1 : 0,
  };
  const got = {
    seed: num(d.seed), name: d.name, root: num(d.root), scale: d.scale, bpm: num(d.bpm), steps_per_bar: num(d.steps_per_bar),
    bars: num(d.bars), swing: num(d.swing), coherence: num(d.coherence),
    feel: d.feel.split('|').map(num).join('|'),
    tone: d.tone.split('|').map(num), layer_seeds: d.layer_seeds.split('|').map(num),
    airy: num(d.airy), drifting: num(d.drifting), choir: num(d.choir),
  };
  check(`${label}: the player's notes`, LAYERS.every((l) => same(notes(x, l), pageNotes(p, l))));
  check(`${label}: the description`, same(got, want), `\n    got  ${JSON.stringify(got)}\n    want ${JSON.stringify(want)}`);
  check(`${label}: the mix`, same(weights(d.mix), spec.mix), `${d.mix} ${JSON.stringify(spec.mix)}`);
  check(`${label}: the moods`, same(weights(d.moods), spec.feelMix), `${d.moods} ${JSON.stringify(spec.feelMix)}`);
  if (spec.vibes) check(`${label}: the vibes`, same(weights(d.vibes), spec.vibes) && num(d.strength) === spec.vibeStrength, d.vibes);
  if (spec.cycles) check(`${label}: the cycles`, same(d.cycles.split('|').map(num), LAYERS.map((l) => spec.cycles[l] || 0)), d.cycles);
  const rolled = spec.forceLayers ? LAYERS.filter((l) => spec.forceLayers[l]).join('|') : undefined;
  check(`${label}: the layers re-rolled`, d.rolled === rolled, `${d.rolled} ${rolled}`);
}

// The next few passes' grids, read as each pass begins (in a piece, any
// loop with a length asked, each section, as it wanders a section at a
// time), against the core's wandered passes (the goldens' `passes`, pass 1
// on, at the app's default amount). The first pass is not among them: it
// plays the loop as composed but for what its own pass draws (the goldens
// hold it through the player's renders, tools/seeds.sh).
function grids(label, x, spec, c, passes = 3) {
  const made = core.parts(c.seed, c.request);
  let ok = LAYERS.every((l) => grid(x, l).length === ((made.form.cycles && made.form.cycles[l]) || made.form.totalSteps));
  let n = 0;
  for (let frame = 0; n < passes && frame < RATE * 600; frame += QUANTUM) {
    x.dl_play_process(frame);
    for (const e of events(x)) {
      if (e.kind === (spec.length != null ? 2 : 1) && e.number > 0) {
        n = e.number;
        ok = ok && LAYERS.every((l) => same(grid(x, l), pageGrid(made.passes[n - 1], l)));
      }
    }
  }
  check(`${label}: the grid of the next ${passes} passes`, ok && n === passes, `${ok} ${n}`);
}

console.log('\nThe page\'s record, its notes and the grid, against the core (24 corpus loops)');
{
  const corpus = CASES.filter((c) => c.set === 'corpus').slice(0, 24);
  corpus.forEach((c, i) => {
    const x = start(c.seed);
    describes(c.name, x, edited(c.seed, null), c);
    grids(c.name, x, edited(c.seed, null), c, i < 10 ? 3 : 1);
  });
}

console.log('Every kind of request (the goldens\' cases, a dozen of each set)');
{
  for (const set of ['vibes', 'request', 'pins', 'theme', 'rest', 'length', 'key', 'style', 'voice']) {
    for (const c of CASES.filter((k) => k.set === set).slice(0, 12)) {
      const spec = edited(c.seed, c.request);
      const made = core.parts(c.seed, c.request);
      const p = render(spec);
      check(`${c.name}: the page's record is the core's spec`, canonical(spec) === canonical(made.spec), `\n    page ${canonical(spec).slice(0, 400)}\n    core ${canonical(made.spec).slice(0, 400)}`);
      check(`${c.name}: the page's notes are the core's tracks`, LAYERS.every((l) => same(pageNotes(p, l), coreNotes(made, l))));
    }
  }
}

console.log('A code\'s edits, against the core\'s (the generation goldens\' edit cases)');
{
  const edits = CASES.filter((c) => c.set === 'edit').slice(0, 48);
  for (const c of edits) {
    const x = start(c.seed, c.request);
    const spec = edited(c.seed, c.request);
    describes(c.name, x, spec, c);
    grids(c.name, x, spec, c, 1);
  }
}

console.log('Every pin js/pins.js has, at its ends and past them, as the core takes it (queue item 101a)');
{
  // The page's table and the core's (request.rs, `Pins`) are two copies: a
  // pin the page offers the core must take, at both ends of its range (or
  // every word and value), and refuse past them; a number the page has no
  // pin for the core refuses too.
  const { exports: x } = new WebAssembly.Instance(module, {});
  const takes = (n, v) => x.dl_play_pin(n, v) === 1;
  const numbers = new Set(PINS.map((p) => p.number));
  for (const p of PINS) {
    if (p.words) {
      check(`${p.name}: every word`, p.words.every((_, i) => takes(p.number, i)) && !takes(p.number, p.words.length));
    } else if (p.values) {
      check(`${p.name}: every value`, p.values.every((v) => takes(p.number, v)) && !takes(p.number, p.values[0] + 0.01));
    } else {
      const [lo, hi] = p.range;
      const past = lo === hi ? 1e-6 : (hi - lo) * 1e-6;
      check(`${p.name}: ${lo} to ${hi}`, takes(p.number, lo) && takes(p.number, hi) && !takes(p.number, lo - past) && !takes(p.number, hi + past));
      if (p.whole) check(`${p.name}: whole numbers`, !takes(p.number, lo + 0.5));
    }
  }
  for (let n = 0; n < 128; n++) if (!numbers.has(n)) check(`pin ${n} is no pin, to the core too`, !takes(n, 0) && !takes(n, 1));
  check('the far ends pin (subDuck=deep, padSweep=wide and fast, lateFloor=slow and its fade)',
    ['subDuck', 'padSweep', 'lateFloor'].every((id) => PINS.some((p) => p.name === id)) && PINS.some((p) => p.name === 'lateFloor.fade')
    && PINS.find((p) => p.name === 'subDuck').words.includes('deep') && PINS.find((p) => p.name === 'lateFloor').words.includes('slow')
    && ['wide', 'fast'].every((w) => PINS.find((p) => p.name === 'padSweep').words.includes(w)));
}

console.log('The events, a re-roll in place and the track\'s end');
{
  const seed = 1063107040;
  const x = start(seed);
  const spec = newSpec(seed);
  const total = spec.bars * spec.stepsPerBar;
  const sd = 60 / spec.bpm / 4;
  x.dl_play_track(1, 2);
  check('the track ends two passes on', x.dl_play_track_end() === 2 * total);
  x.dl_play_change(663542962, 5, 1);
  const seen = [];
  let frame = 0;
  for (; frame < (2 * total * sd + 3) * RATE; frame += QUANTUM) {
    if (frame === QUANTUM * 2000) {
      x.dl_play_seed(0, 4242, 1);
      x.dl_play_revise(seed);
    }
    x.dl_play_process(frame);
    seen.push(...events(x));
  }
  const steps = seen.filter((e) => e.kind === 0);
  const ends = seen.filter((e) => e.kind === 6);
  const revised = seen.find((e) => e.kind === 5);
  check('a step event for every step, in order, through the re-roll', steps.every((s, i) => s.step === i || i >= 2 * total), String(steps.length));
  check('the re-roll kept its place', !!revised && revised.step > 0 && steps.some((s) => s.step === revised.step && s.time === revised.time));
  check('the end told once, on the step', ends.length === 1 && ends[0].step === 2 * total, JSON.stringify(ends));
  const next = seen.find((e) => e.kind === 4 && e.number === 663542962);
  check('the next loop starts on the end', !!next && Math.abs(next.time - ends[0].time) < 1e-9);
  check('none lost', x.dl_play_events_lost() === 0);
  check('no note dropped, late or missed', x.dl_dropped() + x.dl_late() + x.dl_missed() === 0);
  x.dl_play_tone(0.1, 0.2, 0.3);
  check('the tone set is the tone told', about(x).tone === '0.1|0.2|0.3', about(x).tone);
}

console.log(failures ? `\n${failures} failed` : '\nall passed');
process.exit(failures ? 1 : 0);
