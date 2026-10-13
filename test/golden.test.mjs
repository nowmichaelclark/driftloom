// Tests for the golden renders' JavaScript side (queue item 42): the note-list
// text and the hash, which the native host
// (core/examples/dlrender/notelist.rs) has its own copies of. Each is pinned
// to the same answers there, so the two cannot drift apart. Needs no browser.
// Run with:  node test/golden.test.mjs
//
// The renders themselves are checked by tools/golden.sh on every target.

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { Fnv, num, readList, writeList, SIGNALS } from '../tools/golden-lib.mjs';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
let failures = 0;
function check(label, condition, detail = '') {
  console.log(condition ? `  pass  ${label}` : `  FAIL  ${label} ${detail}`);
  if (!condition) failures++;
}

console.log('\nThe hash is FNV-1a, 64 bits (the same answers as the native host\'s test)');
check('the empty string', new Fnv().hex() === 'cbf29ce484222325');
{
  const h = new Fnv();
  h.bytes(Uint8Array.of(0x61));
  check('"a"', h.hex() === 'af63dc4c8601ec8c', h.hex());
}
{
  const a = new Fnv();
  a.samples(Float32Array.of(1));
  const b = new Fnv();
  b.bytes(Uint8Array.of(0x00, 0x00, 0x80, 0x3f));
  check('a sample goes in as its four bytes, least significant first', a.hex() === b.hex());
}

console.log('\nNumbers and lists');
check('nan, negative zero and shortest text', num(NaN) === 'nan' && num(-0) === '-0' && num(0.1) === '0.1' && num(1e-7) === '1e-7');
{
  const list = {
    name: 't', rate: 48000, quality: 'lite', frames: 256, comment: ['x'],
    events: [
      { type: 'param', id: 17, op: 0, value: 0.5, time: 0, tau: 0 },
      { type: 'damp', channel: 1, time: 0.25 },
      { type: 'key', seed: 4288468419, pass: 1, step: 2, ord: 3 },
      { type: 'note', voice: 25, channel: 0, time: 0.1, midi: 60.5, dur: 0.4, vel: 0.8, parts: 3, extra: [1, NaN, -0, 1 / 3] },
    ],
  };
  const text = writeList(list);
  const back = readList(text);
  check('a list reads back what was written', JSON.stringify(back.events.map((e) => ({ ...e, extra: e.extra?.map((x) => (Number.isNaN(x) ? 'nan' : Object.is(x, -0) ? '-0' : x)) })))
    === JSON.stringify([
      { type: 'param', id: 17, op: 0, value: 0.5, time: 0, tau: 0 },
      { type: 'damp', channel: 1, time: 0.25 },
      { type: 'key', seed: 4288468419, pass: 1, step: 2, ord: 3 },
      { type: 'note', voice: 25, channel: 0, time: 0.1, midi: 60.5, dur: 0.4, vel: 0.8, parts: 3, extra: [1, 'nan', '-0', 1 / 3, ...Array(8).fill('nan')] },
    ]));
  check('and its header', back.name === 't' && back.rate === 48000 && back.quality === 'lite' && back.frames === 256);
  check('it is plain ASCII', /^[\x20-\x7e\n]*$/.test(text));
}
check('a bad line is named', (() => { try { readList('dlnotes 2\nbogus 1\n'); return false; } catch (e) { return /line 2/.test(e.message); } })());

console.log('\nThe committed goldens');
{
  const dir = path.join(ROOT, 'core', 'golden');
  const lists = fs.readdirSync(dir).filter((f) => f.endsWith('.dlnotes')).sort();
  const codes = fs.readFileSync(path.join(dir, 'codes.txt'), 'utf8').split('\n').filter((l) => l.trim() && !l.startsWith('#')).map((l) => l.split(/\s+/)[0]);
  check('a list for every code, and no other', JSON.stringify(lists.map((f) => f.replace('.dlnotes', ''))) === JSON.stringify([...codes].sort()), `${lists.length} lists, ${codes.length} codes`);
  let all = true;
  for (const f of lists) {
    const text = fs.readFileSync(path.join(dir, f), 'utf8');
    try {
      const list = readList(text);
      if (writeList({ ...list, comment: [] }).split('\n').filter((l) => !l.startsWith('#')).join('\n') !== text.split('\n').filter((l) => !l.startsWith('#')).join('\n')) throw new Error('does not write back the same');
      if (!/^[\x20-\x7e\n]*$/.test(text)) throw new Error('not plain ASCII');
      if (`${list.name}.dlnotes` !== f) throw new Error('named differently from its file');
    } catch (err) {
      all = false;
      console.log(`    ${f}: ${err.message}`);
    }
  }
  check('each reads, writes back the same and is plain ASCII', all);
  for (const file of fs.readdirSync(dir).filter((f) => f.endsWith('.hashes'))) {
    const rows = fs.readFileSync(path.join(dir, file), 'utf8').split('\n').filter((l) => l.trim() && !l.startsWith('#'));
    const ok = rows.length === lists.length && rows.every((r) => {
      const [name, frames, ...hashes] = r.split(' ');
      return lists.includes(`${name}.dlnotes`) && /^frames=\d+$/.test(frames) && hashes.length === SIGNALS.length
        && hashes.every((h, i) => new RegExp(`^${SIGNALS[i]}=[0-9a-f]{16}$`).test(h));
    });
    check(`${file} has a well-formed line for every list`, ok);
  }
}

console.log(failures ? `\n${failures} failed\n` : '\nall passed\n');
process.exit(failures ? 1 : 0);
