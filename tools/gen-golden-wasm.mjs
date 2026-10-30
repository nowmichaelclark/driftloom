// The Rust generator against the generation goldens on the WebAssembly target
// (queue item 79): every case of core/golden/generation.hashes generated
// through a build of the core with the generator's exports (the `generator`
// feature; tools/gen-core.sh builds it), in Node, and each part the Rust
// generator makes compared with its hash. The spec is also read back as text
// (`dl_spec`) and hashed here, so the host call is checked too.
//
//   node tools/gen-golden-wasm.mjs --wasm FILE [core/golden/generation.hashes]
//
// The parts, in the order the exports number them (`Part::ALL`). Prints one
// line per case that differs, and a summary; exits with 1 on any difference.

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { fnv1a64 } from './gen-golden-lib.mjs';
import { VIBE_IDS } from '../js/vibes.js';

const PARTS = ['spec', 'plan', 'form', 'drums', 'bass', 'chords', 'melody', 'texture', 'meta', 'choices'];

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const argv = process.argv.slice(2);
const at = argv.indexOf('--wasm');
if (at < 0) {
  console.error('usage: node tools/gen-golden-wasm.mjs --wasm FILE [HASHES]');
  process.exit(2);
}
const wasm = argv[at + 1];
const file = argv.filter((a, i) => i !== at && i !== at + 1)[0] || path.join(ROOT, 'core', 'golden', 'generation.hashes');
const { instance } = await WebAssembly.instantiate(fs.readFileSync(wasm), {});
const x = instance.exports;
const hex = (h) => (BigInt.asUintN(64, h)).toString(16).padStart(16, '0');

let cases = 0;
let bad = 0;
const counts = PARTS.map(() => 0);
for (const line of fs.readFileSync(file, 'utf8').split('\n')) {
  if (!line || line.startsWith('#')) continue;
  const [name, seed, vibes, ...fields] = line.split(' ');
  const want = Object.fromEntries(fields.map((f) => f.split('=')));
  const ids = vibes === '-' ? [] : vibes.split(',').map((v) => VIBE_IDS.indexOf(v));
  new Uint8Array(x.memory.buffer, x.dl_gen_input(), ids.length).set(ids);
  if (!x.dl_gen_load(Number(seed) >>> 0, ids.length)) throw new Error(`${name}: the core refused its vibes`);
  cases++;
  const moved = [];
  PARTS.forEach((part, i) => {
    if (hex(x.dl_gen_hash(i)) === want[part]) counts[i]++;
    else moved.push(part);
  });
  const len = x.dl_spec();
  const text = new TextDecoder('ascii').decode(new Uint8Array(x.memory.buffer, x.dl_gen_text(), len));
  if (fnv1a64(text) !== want.spec) moved.push('dl_spec');
  if (x.dl_gen_lost()) moved.push('lost items');
  if (moved.length && ++bad <= 40) console.log(`  differs ${name}: ${moved.join(', ')}`);
}
console.log(`gen-golden-wasm: ${cases - bad} of ${cases} cases equal ${path.relative(ROOT, file)} (${PARTS.map((p, i) => `${p} ${counts[i]}`).join(', ')})`);
process.exit(bad ? 1 : 0);
