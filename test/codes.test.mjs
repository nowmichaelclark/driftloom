// Share codes, version 2 (queue item 105) through the tools' WebAssembly
// build (`dl_code_*`, tools/codes.mjs): every line of core/golden/share-codes.txt
// written and read back to the same characters, as the core's native test
// holds them, so the web and the native core speak one format; a name in
// any script travels with its code, and what a name refuses is refused.
// Needs `tools/build-core.sh --tools`.

import fs from 'node:fs';
import { albumCode, albumTracks, loopCode, nameRefusal, readLoop, seedAlbumCode } from '../tools/codes.mjs';

let failures = 0;
function check(label, condition, detail = '') {
  console.log(condition ? `  pass  ${label}` : `  FAIL  ${label} ${detail}`);
  if (!condition) failures++;
}
const refusal = (fn) => {
  try {
    fn();
    return null;
  } catch (e) {
    return e.message;
  }
};

console.log('\ncore/golden/share-codes.txt, on the web build');
const lines = fs.readFileSync(new URL('../core/golden/share-codes.txt', import.meta.url), 'utf8').split('\n').filter((l) => l && !l.startsWith('#'));
for (const line of lines) {
  const f = line.split(' ');
  const req = (r) => (r === '-' ? '' : r);
  // A name: "-" for none, `%20` a space, `%25` a percent sign.
  const named = (t) => (t === '-' ? '' : t.replace(/%20/g, ' ').replace(/%25/g, '%'));
  if (f[0] === 'loop') {
    const [, seed, request, mutes, salt, name, code] = f;
    const made = loopCode(Number(seed), req(request), Number(mutes), Number(salt), named(name));
    check(`loop ${seed} ${request} ${name}`, made === code, `${made} != ${code}`);
    const back = readLoop(code);
    // What it asks, as text, makes the same code again.
    const again = loopCode(Number(seed), back.request, Number(mutes), Number(salt), named(name));
    check('  its request as text makes it again', again === code, `${back.request}: ${again}`);
    check(`  reads back`, back.seed === Number(seed) && back.mutes === Number(mutes) && back.salt === Number(salt) && back.name === named(name), JSON.stringify(back));
  } else if (f[0] === 'album') {
    const [, t, list, code] = f;
    const codes = list.split(',');
    check(`album of ${codes.length} ${t}`, albumCode(codes, named(t)) === code);
    const a = albumTracks(code);
    check('  its tracks, their paces and its name', a.paced.join(',') === list && a.name === named(t), JSON.stringify(a));
  } else if (f[0] === 'seed-album') {
    const [, seed, count, request, t, code] = f;
    check(`seed-album ${seed} x${count}`, seedAlbumCode(Number(seed), Number(count), req(request), named(t)) === code);
    const a = albumTracks(code);
    check('  its tracks', a.tracks.length === Number(count) && a.name === named(t));
  } else {
    check(`a line it knows: ${f[0]}`, false);
  }
}

console.log('\nRefusals');
{
  // A code from a newer build: version 3's head, sealed with its check.
  const A = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_';
  const chars = 'GRiSNBA';
  let h = 0;
  for (const c of chars) h = (h * 64 + A.indexOf(c) + 1) % 4093;
  const newer = chars + A[h & 63] + A[h >> 6];
  check('a newer version: "needs a newer version"', refusal(() => readLoop(newer)) === 'needs a newer version');
  check('a typo: "not a code"', refusal(() => readLoop('ERiSNBA2l')) === 'not a code');
  check('an old DL1- code: "not a code"', refusal(() => readLoop('DL1-0A1B2-C3D4E')) === 'not a code');
}

console.log('\nNames');
{
  const bad = "a name a code can't carry";
  for (const ok of ['ฝนตกที่กรุงเทพ', '👨‍👩‍👧‍👦', 'Café ☕', 'a'.repeat(60), 'ก'.repeat(20), '']) check(`carried: ${ok || '(none)'}`, nameRefusal(ok) === null);
  for (const no of [' lead', 'two  spaces', '\u202Eevil', 'zero\u200Bwidth', `a${'\u0301'.repeat(5)}`, 'a'.repeat(61), 'ก'.repeat(21), 'tab\t']) check(`refused: ${JSON.stringify(no)}`, nameRefusal(no) === bad);
  check('a loop refuses a name it can\'t carry', refusal(() => loopCode(5, '', 0, 0, ' lead')) === bad);
  check('an album too', refusal(() => albumCode([loopCode(5)], '\u202Eevil')) === bad);
}

if (failures) {
  console.log(`\n${failures} failed`);
  process.exit(1);
}
console.log('\nall passed');
