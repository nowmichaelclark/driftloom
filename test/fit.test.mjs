// Tests for fitting the words and building word albums (queue item 35).
// Run with:  node test/fit.test.mjs

import { newSpec } from '../js/generator.js';
import { encodeSong, decodeAlbum } from '../js/share.js';
import { Rng } from '../js/rng.js';
import { dialsOf, loadScale } from '../tools/dials.mjs';
import { observations, fitWords } from '../tools/fit.mjs';
import { buildAlbum, parseTarget, distance } from '../tools/wordalbum.mjs';

let failures = 0;
function check(label, condition, detail = '') {
  console.log(condition ? `  pass  ${label}` : `  FAIL  ${label} ${detail}`);
  if (!condition) failures++;
}

const scale = loadScale();

console.log('\nFitting the words (35a)');

// A synthetic export: 240 loops, five words a trial. "lofty" is tapped
// exactly when the loop is high (height above 0.6, a dial that moves with
// no other); "coin" at random; the rest at random too.
{
  const r = new Rng(35);
  const memo = new Map();
  const lines = [];
  for (let i = 0; i < 240; i++) {
    const code = encodeSong(newSpec(50000 + i));
    const d = dialsOf(code, scale).scaled;
    memo.set(code, d);
    const words = r.shuffle(['lofty', 'coin', 'gamma', 'delta', 'epsilon']);
    const taps = r.shuffle(words.filter((w) => (w === 'lofty' ? d.height > 0.6 : r.chance(w === 'coin' ? 0.5 : 0.3))));
    lines.push({ trial: i + 1, code, words, taps, pool: 3, skip: false });
  }
  const model = fitWords(observations(lines), (c) => memo.get(c), { boot: 300 });
  const lofty = model.lofty.region.map((x) => x.dial);
  check('a word tapped on high loops gets a region on height and nothing else', lofty.length === 1 && lofty[0] === 'height', JSON.stringify(lofty));
  check('its center sits high', model.lofty.region[0] && model.lofty.region[0].center > 0.6);
  check('a word tapped at random gets no region', model.coin.region.length === 0, JSON.stringify(model.coin.region.map((x) => x.dial)));
  const others = ['gamma', 'delta', 'epsilon'].filter((w) => model[w].region.length);
  check('nor do the other random words', others.length === 0, others.join());

  // Fewer than eight answers: too few. Almost always: apart.
  const few = fitWords(observations(lines.slice(0, 5)), (c) => memo.get(c), { boot: 50 });
  check('fewer than eight answers is "too few"', few.lofty.status === 'too few');
  const always = lines.slice(0, 60).map((l) => ({ ...l, taps: [...new Set(['coin', ...l.taps])] }));
  const fa = fitWords(observations(always), (c) => memo.get(c), { boot: 50 });
  check('a word tapped every time is listed apart, with no region', fa.coin.status === 'fits almost always' && !fa.coin.region.length);
  check('the bootstrap is seeded: the same fit twice', JSON.stringify(fitWords(observations(lines.slice(0, 80)), (c) => memo.get(c), { boot: 50 }))
    === JSON.stringify(fitWords(observations(lines.slice(0, 80)), (c) => memo.get(c), { boot: 50 })));
}

// Word-album answers count: fits and doesn't fit; "not sure" left out.
{
  const code = encodeSong(newSpec(7));
  const { obs, counts } = observations([
    { code, words: ['cozy'], taps: ['cozy'], answer: 'fits', album: 'X' },
    { code, words: ['cozy'], taps: [], answer: 'no', album: 'X' },
    { code, words: ['cozy'], taps: [], answer: 'unsure', album: 'X' },
  ]);
  check('album answers read as fits and doesn\'t fit, "not sure" left out',
    obs.length === 2 && obs[0].fit === 1 && obs[1].fit === 0 && counts.albums === 2 && counts.unsure === 1);
}

console.log('\nWord albums (35b)');
{
  const target = parseTarget('height>0.7,warmth<0.4');
  const album = buildAlbum({ word: 'lofty', target, n: 600, seed: 3, scale });
  const guesses = album.tracks.filter((t) => t.role === 'guess');
  const decoys = album.tracks.filter((t) => t.role === 'decoy');
  check('six guesses and two decoys', guesses.length === 6 && decoys.length === 2);
  check('the guesses are inside the target', guesses.every((t) => distance(t.d, target) === 0),
    guesses.map((t) => `${t.d.height.toFixed(2)}/${t.d.warmth.toFixed(2)}`).join(' '));
  check('the decoys are outside it', decoys.every((t) => distance(t.d, target) > 0));
  check('every track is four bars or longer, and about 40 s', album.tracks.every((t) => t.spec.bars >= 4 && t.spec.playFor >= 1));
  const back = decodeAlbum(album.code);
  check('the album code reads back, titled "Lofty?"', back.title === 'Lofty?' && back.specs.length === 8);
  check('the same seed builds the same album', buildAlbum({ word: 'lofty', target, n: 600, seed: 3, scale }).code === album.code);
}

console.log(failures === 0 ? '\nAll good.\n' : `\n${failures} failing.\n`);
process.exit(failures === 0 ? 0 : 1);
