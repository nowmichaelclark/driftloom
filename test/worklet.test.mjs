// Tests for the core's worklet (js/worklet.js) that need no browser: the real
// worklet module and the real js/dlplayer.wasm, with the few globals an
// AudioWorkletGlobalScope gives it stubbed.
// Run with:  node test/worklet.test.mjs
//
// What they pin down: which block the worklet asks the core to render
// (queue item 31b): the frame count that rides out Chromium's stale
// `currentFrame`, and (item 32) never goes backward, however long the clock
// stays stale; the Mono switch; how busy the audio thread is; and the
// player's commands and events as the app sends and reads them (94b, 95).

import fs from 'node:fs';
import { fileURLToPath } from 'node:url';
import { newSpec } from '../js/gen.js';
import { rerollLayer, vibeSelection, LAYERS } from '../js/request.js';
import { askOf, picksAsk, KIND, END_FADE, MOMENT, CHANGE_FADE } from '../js/player.js';
import { Rng } from '../js/rng.js';

const QUANTUM = 128;
const RATE = 44100;
const wasm = fs.readFileSync(fileURLToPath(new URL('../js/dlplayer.wasm', import.meta.url)));

let processors = [];
globalThis.sampleRate = RATE;
globalThis.currentFrame = 0;
globalThis.AudioWorkletProcessor = class {
  constructor() {
    this.sent = [];
    this.port = { postMessage: (m) => this.sent.push(m), onmessage: null };
  }
};
globalThis.registerProcessor = (name, cls) => { processors.push([name, cls]); };
await import('../js/worklet.js');
const [, DriftloomCore] = processors[0];

let failures = 0;
function check(label, condition, detail = '') {
  console.log(condition ? `  pass  ${label}` : `  FAIL  ${label} ${detail}`);
  if (!condition) failures++;
}

// A worklet around a core that records the blocks it is asked for.
function make() {
  const processor = new DriftloomCore({
    numberOfOutputs: 1,
    processorOptions: { bytes: wasm, quality: 'full' },
  });
  const blocks = [];
  const real = processor.core;
  processor.core = { ...real, dl_play_process: (block) => { blocks.push(block); return real.dl_play_process(block); } };
  return { processor, blocks };
}

// One process() call at the clock `frame`.
function call(processor, frame) {
  globalThis.currentFrame = frame;
  const inputs = Array.from({ length: 5 }, () => []);
  const outputs = [[new Float32Array(QUANTUM)]];
  return processor.process(inputs, outputs);
}

console.log('\nWhich block the worklet renders');

{
  const { processor, blocks } = make();
  for (let b = 0; b < 6; b++) call(processor, b * QUANTUM);
  check('a clock that keeps time renders each block as it says',
    blocks.join() === [0, 128, 256, 384, 512, 640].join(), blocks.join());
}

{
  // Seen in offline renders: 9088 again where 9216 was due, then 9344.
  const { processor, blocks } = make();
  for (const frame of [8960, 9088, 9088, 9344, 9472]) call(processor, frame);
  check('a stale frame renders the block that was due, not the last one again',
    blocks.join() === [8960, 9088, 9216, 9344, 9472].join(), blocks.join());
}

{
  const { processor, blocks } = make();
  for (const frame of [0, 128, 128, 128, 384]) call(processor, frame);
  check('a frame stale twice still renders each block once, in order',
    blocks.join() === [0, 128, 256, 384, 512].join(), blocks.join());
}

{
  const { processor, blocks } = make();
  for (const frame of [0, 128, 256, 5120, 5248]) call(processor, frame);
  check('a gap moves the core on to the clock',
    blocks.join() === [0, 128, 256, 5120, 5248].join(), blocks.join());
}

{
  // A host that called process() twice for each block of real time, or a
  // clock that stopped: the count runs ahead of it, and stays ahead -- it
  // never renders a block twice, or one before the last.
  const { processor, blocks } = make();
  for (let b = 0; b < 400; b++) {
    for (let k = 0; k < 2; k++) call(processor, b * QUANTUM);
  }
  check('a count ahead of the clock goes on, one block each call',
    blocks.every((x, i) => i === 0 || x === blocks[i - 1] + QUANTUM));
}

{
  // Stale for good: the clock stops at block 100.
  const { processor, blocks } = make();
  for (const frame of [100, 100, 100, 100, 100, 100].map((b) => b * QUANTUM)) call(processor, frame);
  check('a clock that stops is not followed back',
    blocks.join() === [100, 101, 102, 103, 104, 105].map((b) => b * QUANTUM).join(), blocks.join());
}

{
  // And when it does move on past the count, the count follows it.
  const { processor, blocks } = make();
  for (const frame of [100, 100, 100, 100, 200, 201].map((b) => b * QUANTUM)) call(processor, frame);
  check('and a clock that jumps ahead of the count is followed',
    blocks.join() === [100, 101, 102, 103, 200, 201].map((b) => b * QUANTUM).join(), blocks.join());
}

const same = (a, b) => a.length === b.length && a.every((x, i) => Object.is(x, b[i]));

console.log('\nThe Mono switch (queue item 66)');
{
  // The app's node has two output channels whatever the layout (js/core.js,
  // CoreHost): in mono each is the mono render. A `layout` message switches
  // it as it plays; the core fades, and the output follows its lanes.
  const play = (layout, switches = []) => {
    const processor = new DriftloomCore({ numberOfOutputs: 1, processorOptions: { bytes: wasm, quality: 'full', layout } });
    processor.receive({ type: 'playWander', amount: 0, salt: 0 });
    processor.receive({ type: 'playLoad', seed: 20261009, ask: {}, at: 0 });
    const sides = [[], []];
    const lanes = [];
    for (let b = 0; b * QUANTUM < 4.1 * RATE; b++) {
      const t = (b * QUANTUM) / RATE;
      while (switches.length && switches[0][0] <= t) processor.receive({ type: 'layout', channels: switches.shift()[1] });
      globalThis.currentFrame = b * QUANTUM;
      const outputs = [[new Float32Array(QUANTUM), new Float32Array(QUANTUM)]];
      processor.process(Array.from({ length: 5 }, () => []), outputs);
      outputs[0].forEach((o, c) => sides[c].push(...o));
      lanes.push(processor.core.dl_lanes());
    }
    return { sides, lanes, sent: processor.sent };
  };
  const mono = play(1);
  const [left, right] = mono.sides;
  check('mono into two channels: the mono render in each', left.some((x) => Math.abs(x) > 1e-4) && same(left, right));
  const into = Math.round(1.3 * RATE / QUANTUM) * QUANTUM;
  const switched = play(1, [[1.3, 2], [3.0, 1]]);
  const [sl, sr] = switched.sides;
  check('until the switch, the mono render bit for bit', same(sl.slice(0, into), left.slice(0, into)) && same(sr.slice(0, into), left.slice(0, into)));
  const apart = (from, to) => Math.max(...sl.slice(from, to).map((x, i) => Math.abs(x - sr[from + i])));
  check('switched to stereo, the sides part', apart(Math.round(1.8 * RATE), Math.round(3.0 * RATE)) > 1e-4);
  const back = switched.lanes.lastIndexOf(2) + 1;
  check('back to mono, one lane again within half a second', back > 0 && back * QUANTUM < 3.5 * RATE && switched.lanes.slice(back).every((n) => n === 1), `${back}`);
  check('and the same in both channels after it', apart(back * QUANTUM, sl.length) === 0);
  const cost = switched.sent.filter((m) => m.type === 'cost').pop();
  check('the worklet reports what each layout cost', cost && cost.seconds[0] > 0.5 && cost.seconds[1] > 0.5 && cost.ms.every((x) => x >= 0), JSON.stringify(cost));
}

console.log('\nHow busy the audio thread is (queue item 87b)');
{
  // Eight seconds of an idle core, as is and with a test's slow device
  // (`burn`: microseconds of work added to every block).
  const run = (burn) => {
    const processor = new DriftloomCore({ numberOfOutputs: 1, processorOptions: { bytes: wasm, quality: 'full', burn } });
    for (let b = 0; b * QUANTUM < 8.1 * RATE; b++) {
      globalThis.currentFrame = b * QUANTUM;
      processor.process(Array.from({ length: 5 }, () => []), [[new Float32Array(QUANTUM), new Float32Array(QUANTUM)]]);
    }
    // The first report goes with the first block, before a window has closed.
    return processor.sent.filter((m) => m.type === 'cost' && m.busy && m.busy.windows).map((m) => m.busy);
  };
  const idle = run(0);
  const slow = run(1500);
  const deadline = (QUANTUM / RATE) * 1e6;
  check('each cost report says how busy, a quarter-second at a time', idle.length >= 2 && idle.every((b) => b.windows >= 7 && b.windows <= 8 && b.seconds === b.windows * 0.25 && b.max >= 0 && b.max < 1), JSON.stringify(idle));
  check('an idle core is far from the line', idle.every((b) => b.max < 0.3 && b.behind === 0), JSON.stringify(idle));
  const want = 1500 / deadline;
  // Its count settles as it runs: the last report.
  const last = slow[slow.length - 1];
  check(`a slow device reads as one (burn 1500 us: about ${Math.round(want * 100)}% of a block)`, last && last.max > want * 0.7 && last.max < want * 1.5 + 0.1, JSON.stringify(slow));
}

console.log('\nThe core\'s player (queue item 94b)');
{
  // The app's engine on the player: js/player.js's `askOf` through the
  // worklet's `ask` to js/dlplayer.wasm, as the app sends it. A loop's
  // request with its edits sent whole (every layer's seed, the tone, the
  // tempo) plays the very loop its seed and request make; an edited spec
  // plays its edits; the events reach the page, with the grids of each
  // pass; the loop armed for the track's end starts there; a resumed load
  // doesn't wait out its lateFloor again.
  const make = () => new DriftloomCore({ numberOfOutputs: 1, processorOptions: { bytes: wasm, quality: 'full', layout: 2 } });
  // `seconds` of blocks from the processor's next, the output's left lane.
  const render = (p, seconds) => {
    const out = [];
    const from = p.next || 0;
    for (let b = 0; b * QUANTUM < seconds * RATE; b++) {
      globalThis.currentFrame = from + b * QUANTUM;
      const o = [[new Float32Array(QUANTUM), new Float32Array(QUANTUM)]];
      p.process(Array.from({ length: 5 }, () => []), o);
      out.push(...o[0][0]);
    }
    return out;
  };
  const probe = (p) => {
    p.receive({ type: 'probe', id: 1 });
    const m = p.sent.filter((x) => x.type === 'probe').pop();
    return Object.fromEntries(m.about.split('\n').filter(Boolean).map((l) => [l.slice(0, l.indexOf('=')), l.slice(l.indexOf('=') + 1)]));
  };
  const r = new Rng(94);
  let whole = true;
  let detail = '';
  for (let i = 0; i < 8; i++) {
    const seed = r.seed32();
    const spec = newSpec(seed, i % 2 ? vibeSelection(['cozy', 'rainy'].slice(0, 1 + (i % 2))) : null);
    const full = askOf(spec);
    const bare = { ...full, seeds: [], tone: null, bpm: null };
    const a = make();
    a.receive({ type: 'playWander', amount: 1, salt: 0 });
    a.receive({ type: 'playLoad', seed, ask: full, at: 0 });
    const b = make();
    b.receive({ type: 'playWander', amount: 1, salt: 0 });
    b.receive({ type: 'playLoad', seed, ask: bare, at: 0 });
    const ra = render(a, 6);
    const rb = render(b, 6);
    if (!same(ra, rb) || a.refused || !ra.some((v) => v !== 0)) {
      whole = false;
      detail += ` ${seed}${a.refused ? ` (refused ${a.refused})` : ''}`;
    }
  }
  check('a loop\'s edits sent whole play the loop its seed and request make, sample for sample (8 loops)', whole, detail);
  // An edited spec: a layer re-rolled, the tone, the tempo.
  const seed = r.seed32();
  let spec = newSpec(seed);
  spec = rerollLayer(spec, 'bass');
  spec.tone = { warmth: 0.2, space: 0.75, wobble: 0.05 };
  spec.bpm = 101;
  const p = make();
  p.receive({ type: 'playLoad', seed, ask: askOf(spec), at: 0 });
  render(p, 1);
  const d = probe(p);
  check('an edited spec plays its edits: the layer re-rolled, the tone, the tempo', d.layer_seeds === LAYERS.map((l) => spec.layerSeeds[l] >>> 0).join('|')
    && d.rolled === 'bass' && d.tone === '0.2|0.75|0.05' && Number(d.bpm) === 101 && p.refused === 0, JSON.stringify(d));
  // The events, posted every 50 ms of audio (or at once), with the grids.
  const posted = p.sent.filter((m) => m.type === 'events');
  const ev = posted.flatMap((m) => m.events);
  const kinds = [];
  for (let i = 0; i < ev.length; i += 4) kinds.push(ev[i + 1]);
  const steps = [];
  for (let i = 0; i < ev.length; i += 4) if (ev[i + 1] === KIND.step) steps.push(ev[i + 3]);
  check('the events reach the page: the loop, its first pass, every step in order', kinds[0] === KIND.loop && kinds.includes(KIND.pass) && steps.length > 4 && steps.every((s, i) => s === i));
  const grids = posted.flatMap((m) => m.grids);
  check('with the layers\' grids of the pass, read as it begins', grids.length >= 2 && grids.every((g) => g.grid.length === 5) && grids[0].grid.some((row) => row.some((v) => v > 0)));
  check('posted a few times a second, not every block', posted.length > 5 && posted.length < 40, `${posted.length}`);
  // The track's end: one pass, the loop armed for it starts there.
  const next = newSpec(r.seed32());
  p.receive({ type: 'playTrack', kind: 1, count: 1, arm: { seed: next.seed, ask: askOf(next), fade: END_FADE } });
  const total = (spec.bars * (spec.stepsPerBar || 16) * 60) / spec.bpm / 4;
  render(p, total + 2);
  const later = p.sent.filter((m) => m.type === 'events').flatMap((m) => m.events);
  let ended = false;
  let took = null;
  for (let i = 0; i < later.length; i += 4) {
    if (later[i + 1] === KIND.end) ended = true;
    if (ended && later[i + 1] === KIND.loop) took = later[i + 2];
  }
  check('the track ends where the slider set it, and the loop armed for it starts there', ended && took === next.seed >>> 0, `${ended} ${took}`);
  // A tempo the core refuses is counted, for Diagnostics.
  const q = make();
  q.receive({ type: 'playLoad', seed, ask: { ...askOf(spec), bpm: 400 }, at: 0 });
  check('a refused edit is counted', q.refused === 1);
  // A resumed load (the quality switch) doesn't wait out its lateFloor.
  const late = newSpec(seed, 'lateFloor=on');
  const first = (resumed) => {
    const x = make();
    x.receive({ type: 'playLoad', seed, ask: askOf(late), at: 0, resumed });
    return render(x, 3);
  };
  check('a resumed load plays its lateFloor loop otherwise than a load does', !same(first(true), first(false)));
}

console.log('\nThe picks through the core (queue item 95)');
{
  // The New loops panel's request as js/player.js's `picksAsk` puts it,
  // held by the core (`playPicks`); the core's own loops take it, at a
  // track's end with nothing armed and at Next (`playNext`), and say which
  // picks they took (an Own event, then the Loop with the seed). The page's
  // generator makes the same loop from that seed and request.
  const make = () => new DriftloomCore({ numberOfOutputs: 1, processorOptions: { bytes: wasm, quality: 'full', layout: 2 } });
  const render = (p, seconds) => {
    const from = p.next || 0;
    for (let b = 0; b * QUANTUM < seconds * RATE; b++) {
      globalThis.currentFrame = from + b * QUANTUM;
      p.process(Array.from({ length: 5 }, () => []), [[new Float32Array(QUANTUM), new Float32Array(QUANTUM)]]);
    }
    p.next = from + Math.ceil((seconds * RATE) / QUANTUM) * QUANTUM;
  };
  const about = (p) => {
    p.receive({ type: 'probe', id: 1 });
    const m = p.sent.filter((x) => x.type === 'probe').pop();
    return { text: m.about, tag: m.picksTag };
  };
  const events = (p) => p.sent.filter((m) => m.type === 'events').flatMap((m) => m.events);
  // The Own event and the seed of the Loop after it, from `from` on.
  const own = (ev, from = 0) => {
    for (let i = from; i < ev.length; i += 4) {
      if (ev[i + 1] === KIND.own && ev[i + 5] === KIND.loop) return { tag: ev[i + 2], seed: ev[i + 6], time: ev[i], at: i };
    }
    return null;
  };
  const request = { vibes: ['cozy', 'rainy'], tempo: [77, 77], length: 12 };
  const sent = picksAsk(request);
  const p = make();
  p.receive({ type: 'playDraws', seed: 9501 });
  p.receive({ type: 'playPicks', ask: sent.ask, tag: 3 });
  const reply = p.sent.find((m) => m.type === 'picks');
  check('the core holds the picks and says so', reply && reply.tag === 3 && reply.met === true && reply.why === null && about(p).tag === 3, JSON.stringify(reply));
  const first = newSpec(424242);
  p.receive({ type: 'playLoad', seed: first.seed, ask: askOf(first), at: 0 });
  p.receive({ type: 'playTrack', kind: 1, count: 1, arm: null });
  const pass = (first.bars * (first.stepsPerBar || 16) * 60) / first.bpm / 4;
  render(p, pass + 2);
  let ev = events(p);
  const o = own(ev);
  const ended = ev.some((v, i) => i % 4 === 1 && v === KIND.end);
  check('a track that ends with nothing armed hands over to a loop of the core\'s own, with the picks\' number', ended && o && o.tag === 3 && o.seed !== first.seed, JSON.stringify(o));
  const page = newSpec(o.seed, request);
  const q = make();
  q.receive({ type: 'playLoad', seed: page.seed, ask: askOf(page), at: 0 });
  render(q, 0.5);
  check('it is the loop the page\'s generator makes from that seed and the picks', about(p).text === about(q).text && page.bpm === 77 && page.bars === 12, `${about(p).text.slice(0, 80)} | ${about(q).text.slice(0, 80)}`);
  // A pick made while it plays: the loop playing stays, the next takes it.
  const before = about(p).text;
  const later = { vibes: ['glacial'], tempo: [140, 140] };
  p.receive({ type: 'playPicks', ask: picksAsk(later).ask, tag: 4 });
  render(p, 1);
  check('a pick made while a loop plays leaves it as it is', about(p).text === before);
  const mark = events(p).length;
  p.receive({ type: 'playNext', moment: MOMENT.now, fade: CHANGE_FADE });
  render(p, 1.5);
  ev = events(p);
  const n = own(ev, mark - (mark % 4));
  const nextPage = n && newSpec(n.seed, later);
  check('Next makes the next of the core\'s own, with the new picks', n && n.tag === 4 && nextPage.bpm === 140 && /bpm=140/.test(about(p).text), JSON.stringify(n));
  // A loop armed by the page wins; dropped (an arm of null), the core's own
  // loop follows instead.
  const armed = newSpec(777001);
  const r2 = make();
  r2.receive({ type: 'playDraws', seed: 9502 });
  r2.receive({ type: 'playPicks', ask: sent.ask, tag: 5 });
  r2.receive({ type: 'playLoad', seed: first.seed, ask: askOf(first), at: 0 });
  r2.receive({ type: 'playTrack', kind: 1, count: 1, arm: { seed: armed.seed, ask: askOf(armed), fade: END_FADE, track: [0, 0] } });
  r2.receive({ type: 'playArm', arm: null });
  render(r2, pass + 2);
  const o2 = own(events(r2));
  check('an arm of null drops the page\'s loop, and the core\'s own follows', o2 && o2.tag === 5 && o2.seed !== armed.seed >>> 0, JSON.stringify(o2));
  // The same seed to draw from, the same loops.
  const r3 = make();
  r3.receive({ type: 'playDraws', seed: 9501 });
  r3.receive({ type: 'playPicks', ask: sent.ask, tag: 3 });
  r3.receive({ type: 'playLoad', seed: first.seed, ask: askOf(first), at: 0 });
  r3.receive({ type: 'playTrack', kind: 1, count: 1, arm: null });
  render(r3, pass + 2);
  check('the core\'s own seeds are a function of the seed the page gave', own(events(r3)).seed === o.seed);
  // Picks the core can't meet: their vibes alone, and why, in its words.
  const bad = make();
  bad.receive({ type: 'playPicks', ask: { ...sent.ask, tempo: [20, 30] }, tag: 6 });
  const no = bad.sent.find((m) => m.type === 'picks');
  check('picks the core can\'t meet are held as their vibes alone, and it says why', no && no.met === false && no.why === 'tempo must be 32 to 190 bpm' && bad.refused === 0, JSON.stringify(no));
  const unread = picksAsk({ vibes: ['cozy'], tempo: [20, 30] });
  check('picks the reader refuses are sent as their vibes alone', unread.vibesOnly && unread.why === 'tempo must be 32 to 190 bpm' && !unread.ask.tempo && unread.ask.vibes.length === 1);
}

console.log(failures === 0 ? '\nAll good.\n' : `\n${failures} failing.\n`);
process.exit(failures === 0 ? 0 : 1);
