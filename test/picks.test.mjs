// Tests for the next picks (Painter session 6, design/next-picks.md): the
// words the pills show, and how picks bend a new loop in today's generator.
// Run with:  node test/picks.test.mjs

import { newSpec, render } from '../js/generator.js';
import { applyPicks, cleanPicks, moodsText, tempoText, lengthText, clampBars } from '../js/picks.js';

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
check('clean prefs', JSON.stringify(cleanPicks({ tempo: [85, 70], length: '16' })) === '{"tempo":null,"length":16}');
check('clean endless', cleanPicks({ length: 'endless' }).length === 'endless');
check('clean nothing', JSON.stringify(cleanPicks(undefined)) === '{"tempo":null,"length":null}');

console.log('\nToday\'s generator');
{
  let ok = true, bad = '';
  for (let s = 0; s < 60; s++) {
    const spec = applyPicks(newSpec(9000 + s), { tempo: [70, 85], length: 16 });
    if (spec.bpm < 70 || spec.bpm > 85 || spec.bars !== 16 || spec.playFor !== 1) { ok = false; bad = JSON.stringify([spec.bpm, spec.bars, spec.playFor]); }
    const p = render(spec);
    if (p.totalSteps !== 16 * spec.stepsPerBar) { ok = false; bad = `steps ${p.totalSteps}`; }
  }
  check('70–85 bpm, 16 bars: tempo in range, 16 bars played once', ok, bad);
}
{
  let ok = true, bad = '';
  for (let s = 0; s < 60; s++) {
    const drawn = newSpec(9100 + s);
    const spec = applyPicks(newSpec(9100 + s), { tempo: null, length: 4096 });
    const total = spec.bars * spec.playFor;
    if (spec.bars !== drawn.bars || spec.bpm !== drawn.bpm || Math.abs(total - 4096) > spec.bars / 2) { ok = false; bad = JSON.stringify([spec.bars, spec.playFor]); }
  }
  check('4,096 bars: the loop as drawn, played about that long', ok, bad);
}
{
  const a = newSpec(9200);
  const b = applyPicks(newSpec(9200), { tempo: null, length: null });
  const c = applyPicks(newSpec(9200), { tempo: null, length: 'endless' });
  check('any and endless leave the loop as drawn', JSON.stringify(a) === JSON.stringify(b) && JSON.stringify(a) === JSON.stringify(c));
}
{
  const spec = applyPicks(newSpec(9300), { tempo: [10, 20], length: 1 });
  check('a tempo below what it can play is clamped to 32', spec.bpm === 32);
  check('one bar renders', render(spec).totalSteps === spec.stepsPerBar);
}

console.log(failures ? `\n${failures} failed` : '\nall passed');
process.exit(failures ? 1 : 0);
