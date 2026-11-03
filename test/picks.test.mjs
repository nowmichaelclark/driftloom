// Tests for the next picks (Painter session 6, design/next-picks.md): the
// words the pills show, and the request the picks make (queue item 88: any
// length, the loop's own up to 32 bars, sections past that, or Endless).
// Run with:  node test/picks.test.mjs

import { newSpec, render, pieceOf, sectionBars, pieceSeconds, pieceTime } from '../js/generator.js';
import { cleanPicks, moodsText, tempoText, lengthText, keyText, styleText, voicesText, clampBars, requestFor, pickProblem } from '../js/picks.js';

let failures = 0;
function check(label, condition, detail = '') {
  console.log(condition ? `  pass  ${label}` : `  FAIL  ${label} ${detail}`);
  if (!condition) failures++;
}

console.log('\nWords');
check('two moods', moodsText(['merry', 'tender']) === 'merry, tender');
check('three moods, no ellipsis', moodsText(['a', 'b', 'c']) === 'a, b, c');
check('four moods end in an ellipsis', moodsText(['merry', 'tender', 'dreamy', 'x']) === 'merry, tender, dreamy…');
check('tempo range', tempoText([70, 85]) === '70–85 bpm');
check('length with grouping', lengthText(4096) === '4,096 bars');
check('one bar', lengthText(1) === '1 bar');
check('endless', lengthText('endless') === 'endless');
check('clamped to 1..32,768', clampBars(0) === 1 && clampBars(99999) === 32768 && clampBars(24.4) === 24);
check('clean prefs', JSON.stringify(cleanPicks({ tempo: [85, 70], length: '16' })) === '{"tempo":null,"length":16,"key":null,"scale":null,"style":null,"voices":null,"layers":null}');
check('clean endless', cleanPicks({ length: 'endless' }).length === 'endless');
check('clean nothing', JSON.stringify(cleanPicks(undefined)) === '{"tempo":null,"length":null,"key":null,"scale":null,"style":null,"voices":null,"layers":null}');
check('the key line\'s words', keyText([2], ['dorian']) === 'D dorian' && keyText([2, 7], null) === 'D or G' && keyText(null, ['dorian', 'minorPent']) === 'dorian or minor pentatonic'
  && keyText(null, null) === '');
check('clean key and scale: sorted, once each, only what the core takes',
  JSON.stringify(cleanPicks({ key: [7, 2, 2, 12, -1, 1.5, 'D'], scale: ['aeolian', 'nope', 'dorian'] })) === '{"tempo":null,"length":null,"key":[2,7],"scale":["dorian","aeolian"],"style":null,"voices":null,"layers":null}'
  && cleanPicks({ key: [], scale: 'dorian' }).key === null && cleanPicks({ scale: 'dorian' }).scale === null);

check('the style line\'s words, and what is cleaned: frozen order, once each, only profiles', styleText(['tide']) === 'tide' && styleText(['dust', 'shrine']) === 'dust or shrine' && styleText(null) === ''
  && JSON.stringify(cleanPicks({ style: ['tide', 'nope', 'dust', 'tide'] }).style) === '["dust","tide"]' && cleanPicks({ style: [] }).style === null && cleanPicks({ style: 'tide' }).style === null);

check('the voices line\'s words: the voices, the layers left out, the layers always played',
  voicesText({ melody: ['kalimba', 'marimba'] }, null) === 'kalimba, marimba' && voicesText(null, { drums: 'off', texture: 'on' }) === 'no drums, texture always'
  && voicesText({ chords: ['marimba'], melody: ['marimba'] }, { bass: 'off' }) === 'marimba, no bass' && voicesText(null, null) === '');
check('clean voices and layers: frozen order, once each, only what the core takes, never all five off',
  JSON.stringify(cleanPicks({ voices: { melody: ['x', 'marimba', 'kalimba', 'kalimba'], drums: [], nope: ['a'] }, layers: { drums: 'off', bass: 'maybe' } }))
    === '{"tempo":null,"length":null,"key":null,"scale":null,"style":null,"voices":{"melody":["kalimba","marimba"]},"layers":{"drums":"off"}}'
  && cleanPicks({ layers: { drums: 'off', bass: 'off', chords: 'off', melody: 'off', texture: 'off' } }).layers === null && cleanPicks({ voices: 'x' }).voices === null);

console.log('\nThe request (queue items 85 and 88)');
{
  let ok = true, bad = '';
  for (let s = 0; s < 60; s++) {
    const state = { vibes: ['cozy'], picks: { tempo: [70, 85], length: 16 } };
    const spec = newSpec(9000 + s, requestFor(state));
    if (spec.bpm < 70 || spec.bpm > 85 || spec.bars !== 16 || spec.length !== 16 || !spec.vibes.cozy) { ok = false; bad = JSON.stringify([spec.bpm, spec.bars, spec.length]); }
    const p = render(spec);
    if (p.totalSteps !== 16 * spec.stepsPerBar) { ok = false; bad = `steps ${p.totalSteps}`; }
    if (JSON.stringify(pieceOf(spec)) !== JSON.stringify({ bars: 16, sections: 1, last: 16 })) { ok = false; bad = 'piece'; }
  }
  check('70–85 bpm, 16 bars: tempo in range, the loop 16 bars, a piece of one section', ok, bad);
}
{
  let ok = true, bad = '';
  for (let s = 0; s < 60; s++) {
    const drawn = newSpec(9100 + s);
    const spec = newSpec(9100 + s, requestFor({ vibes: [], picks: { tempo: null, length: 4096 } }));
    const piece = pieceOf(spec);
    const { length, ...rest } = spec;
    const total = (piece.sections - 1) * piece.bars + piece.last;
    if (JSON.stringify(rest) !== JSON.stringify(drawn) || length !== 4096 || total !== 4096 || piece.bars !== drawn.bars) { ok = false; bad = JSON.stringify([spec.bars, piece]); }
  }
  check('4,096 bars: the loop as drawn, played in sections, exactly 4,096 bars', ok, bad);
}
{
  // A length the loop's doesn't divide: a round's last section cut short.
  const spec = newSpec(9150, 'bars=100');
  const piece = pieceOf(spec);
  check('100 bars of a 16-bar loop: six sections and four bars', spec.bars !== 16 || (piece.sections === 7 && piece.last === 4 && sectionBars(piece, 6) === 4 && sectionBars(piece, 5) === 16),
    JSON.stringify(piece));
  let cuts = 0;
  for (let s = 0; s < 40; s++) {
    const p = pieceOf(newSpec(9160 + s, 'bars=1000'));
    const total = (p.sections - 1) * p.bars + p.last;
    if (total !== 1000) cuts = -1000;
    else if (p.last < p.bars) cuts++;
  }
  check('1,000 bars: every piece exactly 1,000, some cut', cuts > 0, String(cuts));
}
{
  const a = newSpec(9200);
  const c = newSpec(9200, requestFor({ vibes: [], picks: { tempo: null, length: 'endless' } }));
  const { length, ...rest } = c;
  check('endless leaves the loop as drawn, and says so', JSON.stringify(rest) === JSON.stringify(a) && length === 'endless' && pieceOf(c).sections === Infinity);
  check('nothing picked asks for nothing', JSON.stringify(newSpec(9200, requestFor({ vibes: [], picks: { tempo: null, length: null } }))) === JSON.stringify(a));
  check('nothing picked is no piece', pieceOf(a) === null);
}
{
  const state = { vibes: ['cozy'], picks: { tempo: [10, 20], length: 1 } };
  check('a tempo the core cannot play says so', pickProblem(state) === 'tempo must be 32 to 190 bpm');
  check('a request it can meet says nothing', pickProblem({ vibes: ['cozy'], picks: { tempo: [70, 85], length: 32768 } }) === null);
  for (const bars of [1, 2, 3, 5, 7, 13, 24, 31, 32]) {
    let ok = true, bad = '';
    for (let s = 0; s < 40; s++) {
      const spec = newSpec(9300 + s * 7, requestFor({ vibes: [], picks: { tempo: null, length: bars } }));
      const p = render(spec);
      if (spec.bars !== bars || p.totalSteps !== bars * spec.stepsPerBar) { ok = false; bad = `${spec.bars}`; }
    }
    check(`${bars} bar${bars === 1 ? '' : 's'}: the loop's own length`, ok, bad);
  }
}
{
  // The play time: one function, the core's (`piece::time`).
  const t = pieceTime(16, 16, [60, 120]);
  check('16 bars at 60–120 bpm: 32 to 64 s', t[0] === 32 && t[1] === 64 && pieceSeconds(16, 16, 120) === 32);
}

console.log('\nKey and mode in the request (queue item 89)');
{
  let ok = true, bad = '';
  for (let s = 0; s < 60; s++) {
    const state = { vibes: ['cozy'], picks: { tempo: [70, 85], length: 16, key: [2, 7], scale: ['dorian', 'aeolian'] } };
    const spec = newSpec(9300 + s, requestFor(state));
    if (![2, 7].includes(spec.root) || !['dorian', 'aeolian'].includes(spec.scale) || spec.bpm < 70 || spec.bpm > 85 || spec.bars !== 16) { ok = false; bad = JSON.stringify([spec.root, spec.scale, spec.bpm]); }
  }
  check('D or G, dorian or aeolian, with a tempo and a length: all honored', ok, bad);
  const a = newSpec(9400);
  check('nothing picked for the key asks for nothing', JSON.stringify(newSpec(9400, requestFor({ vibes: [], picks: { tempo: null, length: null, key: null, scale: null } }))) === JSON.stringify(a)
    && JSON.stringify(newSpec(9400, requestFor({ vibes: [], picks: { tempo: null, length: null } }))) === JSON.stringify(a));
  check('a key and a mode the core can play say nothing', pickProblem({ vibes: [], picks: { tempo: null, length: null, key: [0], scale: ['akebono'] } }) === null);
  check('and one it cannot says so', pickProblem({ vibes: [], picks: { tempo: null, length: null, key: [12] } }) === 'no such key'
    && pickProblem({ vibes: [], picks: { tempo: null, length: null, scale: ['nope'] } }) === 'no such scale');
}

console.log('\nStyle in the request (queue item 90)');
{
  let ok = true, bad = '';
  for (let s = 0; s < 60; s++) {
    const state = { vibes: ['cozy'], picks: { tempo: [70, 85], length: 16, key: [2], scale: null, style: ['tide', 'shrine'] } };
    const spec = newSpec(9500 + s, requestFor(state));
    if (!Object.keys(spec.mix).every((m) => ['tide', 'shrine'].includes(m)) || spec.bpm < 70 || spec.bpm > 85 || spec.root !== 2) { ok = false; bad = JSON.stringify([spec.mix, spec.bpm, spec.root]); }
  }
  check('tide or shrine, with a tempo, a length and a key: all honored', ok, bad);
  check('nothing picked for the style asks for nothing', JSON.stringify(newSpec(9600, requestFor({ vibes: [], picks: { tempo: null, length: null, style: null } }))) === JSON.stringify(newSpec(9600)));
  check('a style the core cannot play says so', pickProblem({ vibes: [], picks: { tempo: null, length: null, style: ['nope'] } }) === 'no such style'
    && pickProblem({ vibes: [], picks: { tempo: null, length: null, style: ['dust'] } }) === null);
}

console.log('\nVoices and layers in the request (queue item 91)');
{
  let ok = true, bad = '';
  for (let s = 0; s < 40; s++) {
    const state = { vibes: ['cozy'], picks: { tempo: [70, 85], length: 16, voices: { melody: ['kalimba', 'marimba'], bass: ['sub'] }, layers: { drums: 'off', texture: 'on' } } };
    const spec = newSpec(9700 + s, requestFor(state));
    const rb = render(spec);
    const mel = rb.tracks.melody.filter((e) => e.vel);
    if (spec.bpm < 70 || spec.bpm > 85 || !spec.mutes.drums || spec.mutes.melody || rb.tracks.bass.some((e) => e.voice !== 'sub')
      || mel.some((e) => !['kalimba', 'marimba'].includes(e.voice))) { ok = false; bad = `${s} ${spec.bpm} ${JSON.stringify(spec.mutes)}`; }
  }
  check('kalimba or marimba melody, a sub bass, no drums, texture always, with a tempo and a length: all honored', ok, bad);
  check('nothing picked for the voices asks for nothing', JSON.stringify(newSpec(9800, requestFor({ vibes: [], picks: { tempo: null, length: null, voices: null, layers: null } }))) === JSON.stringify(newSpec(9800)));
  check('a voice or a layer the core cannot play says so', pickProblem({ vibes: [], picks: { voices: { melody: ['nope'] } } }) === 'no such voice'
    && pickProblem({ vibes: [], picks: { layers: { drums: 'off', bass: 'off', chords: 'off', melody: 'off', texture: 'off' } } }) === 'all layers off'
    && pickProblem({ vibes: [], picks: { voices: { melody: ['flute'] }, layers: { drums: 'off' } } }) === null);
}

console.log(failures ? `\n${failures} failed` : '\nall passed');
process.exit(failures ? 1 : 0);
