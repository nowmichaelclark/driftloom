// Tests for the core's worklet (js/worklet.js) that need no browser: the real
// worklet module and the real js/dlcore.wasm, with the few globals an
// AudioWorkletGlobalScope gives it stubbed.
// Run with:  node test/worklet.test.mjs
//
// What they pin down is which block the worklet asks the core to render
// (queue item 31b): the frame count that rides out Chromium's stale
// `currentFrame`, and (item 32) never goes backward, however long the clock
// stays stale.

import fs from 'node:fs';
import { fileURLToPath } from 'node:url';

const QUANTUM = 128;
const RATE = 44100;
const wasm = fs.readFileSync(fileURLToPath(new URL('../js/dlcore.wasm', import.meta.url)));

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
    processorOptions: { bytes: wasm, noise: null, quality: 'full' },
  });
  const blocks = [];
  const real = processor.core;
  processor.core = { ...real, dl_process: (block) => { blocks.push(block); return real.dl_process(block); } };
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

console.log('\nNotes handed over ahead (queue item 39)');

// The loudest sample from `from` to `to` seconds, a kalimba note handed
// over for 0.5 s and, if `cancel`, everything waiting dropped at once.
// With `cancel` a time, only what waits to start then or later (queue item
// 40).
function loudest(cancel, from, to) {
  const { processor } = make();
  processor.receive({ type: 'note', voice: 0, channel: 0, time: 0.5, midi: 60, dur: 0.4, vel: 0.8, parts: 3, extra: [], draws: null });
  if (cancel === true) processor.receive({ type: 'cancel' });
  else if (cancel !== false) processor.receive({ type: 'cancel', from: cancel });
  let peak = 0;
  for (let b = 0; b * QUANTUM < to * RATE; b++) {
    globalThis.currentFrame = b * QUANTUM;
    const outputs = [[new Float32Array(QUANTUM)]];
    processor.process(Array.from({ length: 5 }, () => []), outputs);
    if (b * QUANTUM >= from * RATE) for (const x of outputs[0][0]) peak = Math.max(peak, Math.abs(x));
  }
  return peak;
}

{
  const kept = loudest(false, 0.5, 1);
  const cancelled = loudest(true, 0.5, 1);
  check('a note handed over ahead sounds at its time', kept > 0.01, String(kept));
  check('and cancelled before it starts, it never does', cancelled === 0, String(cancelled));
}

{
  const later = loudest(0.6, 0.5, 1);
  const from = loudest(0.5, 0.5, 1);
  check('cancelled from after its time, it sounds', later > 0.01, String(later));
  check('cancelled from its time, it never does', from === 0, String(from));
}

console.log(failures === 0 ? '\nAll good.\n' : `\n${failures} failing.\n`);
process.exit(failures === 0 ? 0 : 1);
