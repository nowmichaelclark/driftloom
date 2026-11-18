// A Library saved by the build before version 2 codes (v183) opens on this
// one with its loops playing as they did (session 23): test/library-v183.json
// is twenty loops as v183 kept them in localStorage, made by v183's build,
// each with the hash of the notes v183 played. Here, on this build: the
// Library's records are read as they are (nothing converts: a record is its
// seed, its request and its edits, all a version 2 code can say), each
// plays the same notes, and each shared as a version 2 code and opened
// again plays them too, its name with it. The albums travel the same way.
// A version 1 code is told apart from no code, and a newer build's.

import fs from 'node:fs';
import { createHash } from 'node:crypto';
import { render } from '../js/gen.js';
import { encodeSong, decodeSong, encodeAlbum, decodeAlbum, codeKind, fitName, carries } from '../js/share.js';
import { TRACK_TIMES, trackLength } from '../js/ui.js';
import { Rng } from '../js/rng.js';

let failures = 0;
function check(label, condition, detail = '') {
  console.log(condition ? `  pass  ${label}` : `  FAIL  ${label} ${detail}`);
  if (!condition) failures++;
}

const fixture = JSON.parse(fs.readFileSync(new URL('./library-v183.json', import.meta.url), 'utf8'));
const hash = (s) => createHash('sha256').update(JSON.stringify(render(s).tracks)).digest('hex').slice(0, 16);

console.log(`\nA ${fixture.build} Library on this build`);
const byId = new Map(fixture.saves.map((e) => [e.id, e]));
let longest = 0;
for (const e of fixture.saves) {
  const was = fixture.plays[e.id];
  check(`${e.id} (${was.why}): plays as it did`, hash(e.spec) === was.notes);
  const code = encodeSong(e.spec);
  longest = Math.max(longest, code.length);
  const back = decodeSong(code);
  // A name travels composed (NFC): the same text to the eye.
  check(`  shared and opened (${code.length} characters): the same notes, name, mutes and salt`, hash(back) === was.notes && back.name === e.spec.name.normalize('NFC')
    && JSON.stringify(back.mutes) === JSON.stringify(e.spec.mutes) && (back.wanderSalt || 0) === (e.spec.wanderSalt || 0)
    && back.length === e.spec.length && back.askJourney === e.spec.askJourney, `${code}: ${back.name}`);
  check('  the code is the 64 characters', /^[A-Za-z0-9_-]+$/.test(code) && codeKind(code) === 'song');
}
check('a plain loop is nine characters', encodeSong(byId.get('fx0').spec).length === 9);
check('two vibes picked: under 20 characters', encodeSong(byId.get('fx2').spec).length < 20);

for (const a of fixture.albums.filter((x) => x.ids.length)) {
  const specs = a.ids.map((id) => byId.get(id).spec);
  const code = encodeAlbum(a.title, specs);
  const back = decodeAlbum(code);
  check(`album ${a.title} (${specs.length} loops, ${code.length} characters): its title and every track as it plays`, back.title === a.title && codeKind(code) === 'album'
    && back.specs.length === specs.length && back.specs.every((s, i) => hash(s) === fixture.plays[a.ids[i]].notes && s.name === specs[i].name.normalize('NFC')));
}

console.log('\nAn album carries how long each track plays (session 24b)');
{
  const pace = (s) => (s.playFor ? `p${s.playFor}` : s.playBars ? `b${s.playBars}` : '-');
  const specs = fixture.saves.map((e) => JSON.parse(JSON.stringify(e.spec)));
  // Every stop of the length slider, "until moved on" (its start) among
  // them, on every saved loop (short loops, long pieces, Endless; slow and
  // fast): set on a track beside a plain one, shared and opened again.
  let walked = 0;
  let wrong = [];
  for (const spec of specs) {
    for (const secs of TRACK_TIMES) {
      const s = JSON.parse(JSON.stringify(spec));
      trackLength(s, secs);
      const back = decodeAlbum(encodeAlbum('Paced', [s, specs[0], s]));
      if (pace(back.specs[0]) !== pace(s) || pace(back.specs[2]) !== pace(s) || pace(back.specs[1]) !== pace(specs[0])) wrong.push(`${s.name} ${secs}s: ${pace(s)} came back ${pace(back.specs[0])}`);
      walked++;
    }
  }
  check(`every slider stop on every saved loop round-trips (${walked})`, !wrong.length, wrong.slice(0, 4).join('; '));
  // A few hundred albums of saved loops, each track at a stop of its own
  // (most sharing one): the same paces, the same notes, back.
  const r = new Rng(105);
  wrong = [];
  for (let i = 0; i < 300; i++) {
    const n = 1 + Math.floor(r.next() * 14);
    const shared = TRACK_TIMES[Math.floor(r.next() * TRACK_TIMES.length)];
    const tracks = Array.from({ length: n }, () => {
      const s = JSON.parse(JSON.stringify(specs[Math.floor(r.next() * specs.length)]));
      trackLength(s, r.next() < 0.7 ? shared : TRACK_TIMES[Math.floor(r.next() * TRACK_TIMES.length)]);
      return s;
    });
    const back = decodeAlbum(encodeAlbum(`Album ${i}`, tracks));
    if (back.specs.length !== n || back.specs.some((s, k) => pace(s) !== pace(tracks[k]))) wrong.push(`album ${i}: ${tracks.map(pace)} came back ${back.specs.map(pace)}`);
  }
  check('300 random albums: every track\'s pace back', !wrong.length, wrong.slice(0, 2).join('; '));
  // No pace set: the code as before (the tracks alone); twelve tracks of
  // two passes, two characters more.
  const plain = specs.slice(0, 12).map((s) => ({ ...s, playFor: null }));
  const before = encodeAlbum('Twelve', plain);
  const two = plain.map((s) => ({ ...s, playFor: 2 }));
  const paced = encodeAlbum('Twelve', two);
  check(`twelve tracks of two passes: ${paced.length - before.length} characters more (under 20)`, paced.length - before.length <= 20 && paced.length > before.length);
  const opened = decodeAlbum(paced);
  check('  and each opens to play two passes, then the next', opened.specs.every((s) => s.playFor === 2 && !s.playBars) && opened.specs.every((s, k) => hash(s) === hash(plain[k])));
  check('  with none set, every track plays on until moved on', decodeAlbum(before).specs.every((s) => !s.playFor && !s.playBars));
}

console.log('\nWhat is no code');
check('a version 1 loop code: from an older build', codeKind('DL1-0C088-1GE08-60511') === 'old' && codeKind('  dl1-abcde') === 'old');
check('a version 1 album code: from an older build', codeKind('DLA1-0C7N0-T31ED') === 'old');
{
  const A = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_';
  let h = 0;
  for (const c of 'GRiSNBA') h = (h * 64 + A.indexOf(c) + 1) % 4093;
  check('a newer build\'s code: says so', codeKind(`GRiSNBA${A[h & 63]}${A[h >> 6]}`) === 'newer');
}
for (const t of ['', 'hello', 'hellohello', 'ERiSNBA2l', 'https://example.com']) check(`no code: ${JSON.stringify(t)}`, codeKind(t) === null);

console.log('\nNames, as the rename box keeps them');
check('Thai and emoji are kept', fitName('ฝนตก 🌧️') === 'ฝนตก 🌧️' && carries('ฝนตก 🌧️'));
check('spaces tidied', fitName('  Rainy   Sunday ') === 'Rainy Sunday');
check('composed (NFC)', fitName('á') === 'á');
check('at most 24 characters, never half of one', fitName('🌧'.repeat(30), 24) === '🌧'.repeat(15) && fitName('ก'.repeat(30), 24) === 'ก'.repeat(20));
check('an invisible or a direction character: not a name a code carries', !carries('a‮b') && !carries('zero​width') && fitName('a‮b') === '');

if (failures) {
  console.log(`\n${failures} failed`);
  process.exit(1);
}
console.log(`\nall passed (the longest loop's code ${longest} characters)`);
