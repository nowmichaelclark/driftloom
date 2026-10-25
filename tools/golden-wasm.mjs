// The golden renders on the WebAssembly target (queue item 42): every note
// list in core/golden/ rendered through js/dlcore.wasm in Node, with the
// same exports the worklet calls, and hashed.
//
//   node tools/golden-wasm.mjs                  print the hash lines
//   node tools/golden-wasm.mjs --twice          render each twice, fail if they differ
//   node tools/golden-wasm.mjs --wav DIR        also write each list's output to DIR/NAME.wav
//   node tools/golden-wasm.mjs LIST.dlnotes...  only these lists
//   node tools/golden-wasm.mjs --rate HZ        every list at another sample rate (NAME@HZ)
//   node tools/golden-wasm.mjs --blocks 1,441,random
//                                               also render each list that many frames a
//                                               call (dl_render), failing if the speaker
//                                               output is not the plain render's
//   node tools/golden-wasm.mjs --wasm FILE      render through another build (the 192 kHz
//                                               one, tools/build-core.sh --wide)
//   node tools/golden-wasm.mjs --stereo         the stereo render (queue item 64): both sides
//                                               hashed, as the native host's --stereo
//
// tools/golden.sh compares the lines with core/golden/wasm32-unknown-unknown.hashes.
// No browser: Node runs WebAssembly itself.

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { atRate, readList, renderWasm, renderWasmBlocks, hashLine, wavBytes } from './golden-lib.mjs';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const GOLDEN = path.join(ROOT, 'core', 'golden');
const argv = process.argv.slice(2);
if (argv.includes('--help') || argv.includes('-h')) {
  console.log(fs.readFileSync(fileURLToPath(import.meta.url), 'utf8').split('\n').filter((l) => l.startsWith('//')).map((l) => l.slice(3)).join('\n'));
  process.exit(0);
}
let twice = false;
let wavDir = null;
let rate = null;
let blocks = [];
let wasm = path.join(ROOT, 'js', 'dlcore.wasm');
let layout = 1;
const lists = [];
for (let i = 0; i < argv.length; i++) {
  if (argv[i] === '--twice') twice = true;
  else if (argv[i] === '--wav') wavDir = argv[++i];
  else if (argv[i] === '--rate') rate = Number(argv[++i]);
  else if (argv[i] === '--blocks') blocks = argv[++i].split(',').map((w) => (w === 'random' ? w : Number(w)));
  else if (argv[i] === '--wasm') wasm = path.resolve(argv[++i]);
  else if (argv[i] === '--stereo') layout = 2;
  else if (argv[i].startsWith('--')) { console.error(`golden-wasm: no such option: ${argv[i]}`); process.exit(2); }
  else lists.push(argv[i]);
}
const files = lists.length ? lists : fs.readdirSync(GOLDEN).filter((f) => f.endsWith('.dlnotes')).sort().map((f) => path.join(GOLDEN, f));
if (wavDir) fs.mkdirSync(wavDir, { recursive: true });

const bytes = fs.readFileSync(wasm);
for (const file of files) {
  try {
    let list = { ...readList(fs.readFileSync(file, 'utf8')), layout };
    if (rate) list = atRate(list, rate);
    const first = renderWasm(bytes, list, wavDir ? (layout === 2 ? 'left' : 'out') : null);
    if (twice && renderWasm(bytes, list).hashes.join() !== first.hashes.join()) throw new Error('rendering twice gave different hashes');
    console.log(hashLine(list, first.hashes));
    for (const size of blocks) {
      const out = renderWasmBlocks(bytes, list, size).join(' ');
      const want = first.hashes.slice(0, layout).join(' ');
      if (out !== want) throw new Error(`${size} frames a call: the speaker output is ${out}, not ${want}`);
    }
    if (wavDir) fs.writeFileSync(path.join(wavDir, `${list.name}.wav`), wavBytes(first.kept, list.rate));
  } catch (err) {
    console.error(`golden-wasm: ${path.relative(ROOT, file)}: ${err.message}`);
    process.exit(1);
  }
}
