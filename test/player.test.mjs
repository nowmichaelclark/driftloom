// The player's commands and events in js/dlplayer.wasm (queue item 94a)
// against what the page's own generator and engine make of the same loop:
// what the core describes (the info card's, the cover's and a share code's
// fields), the notes as the MIDI export reads them, the layers' grid of each pass as it wanders (`stepsForBar`'s
// velocities), the events that say where the loop is, a code's edits (a
// layer re-rolled as `rerollLayer` makes it, the tone, the tempo), a re-roll
// in its place and the track's end. No browser: Node runs the module.
// Run with:  node test/player.test.mjs

import fs from 'node:fs';
import { fileURLToPath } from 'node:url';
import { Rng } from '../js/rng.js';
import { render, choirOf, LAYERS, parseRequest, keyMask, scaleMask, styleMask, voiceMask, layerState } from '../js/generator.js';
import { Engine } from '../js/engine.js';
import { VIBE_IDS } from '../js/vibes.js';
import { pinNumbers } from '../js/pins.js';
import { cases, edited } from '../tools/gen-golden-lib.mjs';

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

// The page's engine on a loop, with a synth that does nothing.
function pageEngine(spec) {
  const synth = new Proxy({ queuesAhead: true, channels: Object.fromEntries(LAYERS.map((l) => [l, {}])) }, {
    get: (t, k) => (k in t ? t[k] : () => {}),
  });
  const engine = new Engine({ currentTime: 0, state: 'running', sampleRate: 48000 }, synth);
  engine.driftOn = true;
  engine.driftAmount = 1;
  engine.load(spec);
  return engine;
}

const num = (s) => Number(s);
const weights = (s) => Object.fromEntries((s || '').split('|').filter(Boolean).map((p) => [p.split(':')[0], num(p.split(':')[1])]));
const same = (a, b) => JSON.stringify(a) === JSON.stringify(b);

// The description against the page's spec.
function describes(label, x, spec) {
  const d = about(x);
  const p = render(spec);
  const want = {
    seed: spec.seed >>> 0, name: spec.name, root: spec.root, scale: spec.scale, bpm: spec.bpm, steps_per_bar: spec.stepsPerBar,
    bars: spec.bars, swing: spec.swing, coherence: spec.coherence,
    feel: [spec.feel.lift, spec.feel.energy, spec.feel.warmth].join('|'),
    tone: TONE.map((k) => spec.tone[k]),
    layer_seeds: LAYERS.map((l) => spec.layerSeeds[l] >>> 0),
    airy: p.form ? 1 : 0, drifting: spec.cycles ? 1 : 0, choir: choirOf(spec) ? 1 : 0,
  };
  const got = {
    seed: num(d.seed), name: d.name, root: num(d.root), scale: d.scale, bpm: num(d.bpm), steps_per_bar: num(d.steps_per_bar),
    bars: num(d.bars), swing: num(d.swing), coherence: num(d.coherence),
    feel: d.feel.split('|').map(num).join('|'),
    tone: d.tone.split('|').map(num), layer_seeds: d.layer_seeds.split('|').map(num),
    airy: num(d.airy), drifting: num(d.drifting), choir: num(d.choir),
  };
  check(`${label}: the notes`, LAYERS.every((l) => same(notes(x, l), pageNotes(p, l))));
  check(`${label}: the description`, same(got, want), `\n    got  ${JSON.stringify(got)}\n    want ${JSON.stringify(want)}`);
  check(`${label}: the mix`, same(weights(d.mix), spec.mix), `${d.mix} ${JSON.stringify(spec.mix)}`);
  check(`${label}: the moods`, same(weights(d.moods), spec.feelMix), `${d.moods} ${JSON.stringify(spec.feelMix)}`);
  if (spec.vibes) check(`${label}: the vibes`, same(weights(d.vibes), spec.vibes) && num(d.strength) === spec.vibeStrength, d.vibes);
  if (spec.cycles) check(`${label}: the cycles`, same(d.cycles.split('|').map(num), LAYERS.map((l) => spec.cycles[l] || 0)), d.cycles);
  const rolled = spec.forceLayers ? LAYERS.filter((l) => spec.forceLayers[l]).join('|') : undefined;
  check(`${label}: the layers re-rolled`, d.rolled === rolled, `${d.rolled} ${rolled}`);
}

// Pass 0 and the next few passes' grids, read as each pass begins (in a
// piece, any loop with a length asked: `pieceOf`, each section, as it
// wanders a section at a time), against the
// page's engine's wandered passes.
function grids(label, x, spec, passes = 3) {
  const engine = pageEngine(spec);
  const wanted = (n) => engine._wander(n);
  let ok = LAYERS.every((l) => same(grid(x, l), pageGrid(wanted(0), l)));
  let n = 0;
  for (let frame = 0; n < passes && frame < RATE * 600; frame += QUANTUM) {
    x.dl_play_process(frame);
    for (const e of events(x)) {
      if (e.kind === (spec.length != null ? 2 : 1) && e.number > 0) {
        n = e.number;
        ok = ok && LAYERS.every((l) => same(grid(x, l), pageGrid(wanted(n), l)));
      }
    }
  }
  check(`${label}: the grid of ${passes + 1} passes`, ok && n === passes);
  engine.clock.dispose();
}

console.log('\nThe description and the grid, against the page (24 corpus loops)');
{
  const r = new Rng(1);
  for (let i = 0; i < 24; i++) {
    const seed = r.seed32();
    const x = start(seed);
    describes(`corpus:${i}`, x, edited(seed, null));
    grids(`corpus:${i}`, x, edited(seed, null), i < 10 ? 3 : 1);
  }
}

console.log('A code\'s edits, against the page\'s (the generation goldens\' edit cases)');
{
  const edits = cases([]).filter((c) => c.set === 'edit').slice(0, 48);
  for (const c of edits) {
    const x = start(c.seed, c.request);
    const spec = edited(c.seed, c.request);
    describes(`edit:${c.index}`, x, spec);
    grids(`edit:${c.index}`, x, spec, 1);
  }
}

console.log('The events, a re-roll in place and the track\'s end');
{
  const seed = 1063107040;
  const x = start(seed);
  const spec = edited(seed, null);
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
