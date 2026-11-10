// The player in Node, for the measuring tools (queue item 96): the core's
// player module (js/dlplayer.wasm, what the app plays) and the tools' build
// of it (core/target/dltools.wasm, `tools/build-core.sh --tools`: the same
// player with a request as text, the voice probe and the budget's counts).
// No browser: Node runs WebAssembly itself, and every target renders the
// same samples from a seed (tools/seeds.sh), so what is measured here is what
// a phone plays.

import fs from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const QUANTUM = 128;
// The generator's layer order (the player's commands), and the core's
// channels (its taps).
const LAYERS = ['drums', 'bass', 'chords', 'melody', 'texture'];
const CHANNELS = ['melody', 'chords', 'bass', 'texture', 'drums'];
const PLAYER = path.join(ROOT, 'js', 'dlplayer.wasm');
const TOOLS = path.join(ROOT, 'core', 'target', 'dltools.wasm');

const modules = new Map();
// A compiled module, by file: compiled once, instantiated a render.
function moduleOf(file) {
  if (!modules.has(file)) modules.set(file, new WebAssembly.Module(fs.readFileSync(file)));
  return modules.get(file);
}

// The tools' module, built if it is not there or is older than the core's
// source (cargo does the comparing: a build with nothing to do takes a
// moment).
let built = false;
function toolsModule() {
  if (!built) {
    const run = spawnSync(path.join(ROOT, 'tools', 'build-core.sh'), ['--tools'], { cwd: ROOT, encoding: 'utf8' });
    if (run.status !== 0) throw new Error(`tools/build-core.sh --tools failed:\n${run.stdout}\n${run.stderr}`);
    built = true;
  }
  return moduleOf(TOOLS);
}

const text = (x, at, length) => new TextDecoder().decode(new Uint8Array(x.memory.buffer, at, length));

// A core started as the worklet starts one: `rate`, full or lite, mono or
// stereo. `tools` asks for the tools' module.
function core({ rate = 44100, full = true, layout = 1, tools = false } = {}) {
  const { exports: x } = new WebAssembly.Instance(tools ? toolsModule() : moduleOf(PLAYER), {});
  x.dl_init(rate);
  x.dl_quality(full ? 1 : 0);
  if (layout === 2) x.dl_layout(2);
  return x;
}

// A player on a core, the loop of `seed` asked for: `request` as text
// ("glacial=0.7,bpm=70-85"; the tools' module reads it), wandering by
// `wander` (the app's default is 1).
function player(seed, { rate = 44100, full = true, layout = 1, wander = 1, request = '', tools = false } = {}) {
  const x = core({ rate, full, layout, tools: tools || !!request });
  x.dl_play_wander(wander);
  x.dl_play_attach(full ? 1 : 0);
  if (request) {
    const bytes = new TextEncoder().encode(request);
    new Uint8Array(x.memory.buffer, x.dl_tools_input(), bytes.length).set(bytes);
    if (!x.dl_tools_ask(bytes.length)) {
      const n = x.dl_play_refusal();
      throw new Error(`the core refused '${request}': ${n ? text(x, x.dl_play_refusal_at(), n) : 'not a request'}`);
    }
  }
  x.dl_play_load(seed >>> 0, 0);
  x.dl_play_ready();
  return x;
}

// The name of the loop a player made.
const nameOf = (x) => text(x, x.dl_play_text(), x.dl_play_name());

// What a player says of its loop (`dl_play_describe`), as its text.
const aboutOf = (x) => text(x, x.dl_play_about_at(), x.dl_play_describe());

// Render `frames` of a started core or player, a block at a time as the
// worklet does: the output (a lane a side) and, with `taps`, each channel
// as it went into the mix. `step` is called before each block with its
// first frame (a tool that changes something as it plays).
function render(x, frames, { playing = true, taps = false, from = 0, step = null } = {}) {
  const lanes = x.dl_outputs();
  const out = Array.from({ length: lanes }, () => new Float32Array(frames));
  const tap = taps ? Object.fromEntries(CHANNELS.map((c) => [c, new Float32Array(frames)])) : null;
  for (let done = 0; done < frames; done += QUANTUM) {
    if (step) step(from + done);
    const at = playing ? x.dl_play_process(from + done) : x.dl_process(from + done);
    const used = Math.min(QUANTUM, frames - done);
    for (let c = 0; c < lanes; c++) out[c].set(new Float32Array(x.memory.buffer, x.dl_out() + c * QUANTUM * 4, used), done);
    if (tap) CHANNELS.forEach((c, i) => tap[c].set(new Float32Array(x.memory.buffer, at + i * QUANTUM * 4, used), done));
  }
  return { out, taps: tap };
}

// The voices the probe knows, by layer (`probe::VOICES` in the core).
function probeVoices() {
  const x = core({ tools: true });
  const out = {};
  LAYERS.forEach((layer, l) => {
    out[layer] = [];
    for (let i = 0; i < x.dl_tools_voices(l); i++) out[layer].push(text(x, x.dl_tools_input(), x.dl_tools_voice(l, i)));
  });
  return out;
}

// One note of `voice` through `layer`'s own path, dry: the layer's channel
// as it goes into the mix (no channel gain, sends, bus or master chain).
// `seconds` held, then `tail` to ring; its draws from `seed`.
function probe({ layer, voice, midi, vel, seconds, seed, vowel = 0, rate = 44100, tail = 2.2, at = 0.05 }) {
  const voices = probe.voices || (probe.voices = probeVoices());
  const index = voices[layer].indexOf(voice);
  if (index < 0) throw new Error(`the probe has no ${layer} voice '${voice}'`);
  const x = core({ rate, tools: true });
  if (!x.dl_tools_probe(LAYERS.indexOf(layer), index, midi, vel, seconds, seed >>> 0, vowel, at)) throw new Error(`the core refused ${layer}/${voice}`);
  const frames = Math.ceil((at - 0.05 + seconds + tail) * rate);
  const r = render(x, frames, { playing: false, taps: true });
  if (x.dl_dropped() || x.dl_late() || x.dl_missed()) throw new Error(`${layer}/${voice}: the core dropped ${x.dl_dropped()}, took ${x.dl_late()} late and missed ${x.dl_missed()} notes`);
  return r.taps[layer];
}

// What the player's voice budget was asked, refused and let go, by layer
// (the tools' module).
function budgetOf(x) {
  const row = (which) => Object.fromEntries(LAYERS.map((l, i) => [l, x.dl_tools_budget(i, which)]));
  return { asked: row(0), refused: row(1), stolen: row(2) };
}

export { ROOT, QUANTUM, LAYERS, CHANNELS, core, player, render, nameOf, aboutOf, probe, probeVoices, budgetOf, toolsModule };
