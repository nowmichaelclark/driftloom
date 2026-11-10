// "Lighter on processor" chosen by the app on slow devices (queue item 87b,
// js/quality.js): the measurements kept only while the loop plays, the rule
// (full to lite past OVER, or at once for a render that falls behind; lite to
// full once, only from the first guess), the listener's choice never
// overridden, no back and forth, the boundaries a switch waits for, and the
// old quality's hand-off.
// Run with:  node test/quality.test.mjs

import * as q from '../js/quality.js';

let failures = 0;
function check(label, condition, detail = '') {
  console.log(condition ? `  pass  ${label}` : `  FAIL  ${label} ${detail}`);
  if (!condition) failures++;
}

// A run of reports, two seconds of audio each, the busiest window `max`.
function played(seconds, max, { behind = 0, busy = new q.Busy(), playing = true } = {}) {
  for (let t = 0; t < seconds; t += 2) busy.add({ max, behind, windows: 8, seconds: 2 }, playing, t);
  return busy;
}

console.log('the measurements');
{
  const b = played(q.WARM - 1, 0.9);
  check('the first WARM seconds are left out', b.busiest(2) === null);
  played(q.OVER_FOR, 0.3, { busy: b });
  check('then the busiest over a span', b.busiest(q.OVER_FOR) === 0.3, `${b.busiest(q.OVER_FOR)}`);
  check('none over a span not yet played', b.busiest(q.UNDER_FOR) === null);
  b.add({ max: 0.2, behind: 0, windows: 8, seconds: 2 }, false);
  check('a pause ends the run', b.busiest(2) === null);
  const c = played(q.WARM + 100, 0.4);
  check('it keeps no more than it reads', c.runs.reduce((a, r) => a + r.seconds, 0) <= q.UNDER_FOR + 32);
}

console.log('the rule');
{
  const over = played(q.WARM + q.OVER_FOR + 2, q.OVER + 0.05);
  const under = played(q.WARM + q.OVER_FOR + 2, q.OVER - 0.05);
  const d = q.decide({ quality: 'full', yours: undefined, auto: null, guess: false, busy: over });
  check('full past the line: lite, at the next pass', d && d.lite === true && d.at === 'pass', JSON.stringify(d));
  check('it says why', d && /busiest 55% at full/.test(d.why), d && d.why);
  check('full under the line: stays', q.decide({ quality: 'full', yours: undefined, auto: null, guess: false, busy: under }) === null);
  const short = played(q.WARM + q.OVER_FOR - 4, 0.95);
  check('not before OVER_FOR seconds of playing', q.decide({ quality: 'full', yours: undefined, auto: null, guess: false, busy: short }) === null);
  const behind = played(q.WARM + 4, 0.3, { behind: 1 });
  const e = q.decide({ quality: 'full', yours: undefined, auto: null, guess: false, busy: behind });
  check('a render behind for its own cost: lite at the next bar', e && e.lite && e.at === 'bar', JSON.stringify(e));
  // The listener's choice, either way, is never changed.
  check('yours (on) is kept', q.decide({ quality: 'full', yours: true, auto: null, guess: false, busy: over }) === null);
  check('yours (off) is kept', q.decide({ quality: 'full', yours: false, auto: null, guess: false, busy: over }) === null);
  // Once moved, never again.
  const moved = { lite: true, why: 'busiest 60% at full' };
  check('once moved, it holds', q.decide({ quality: 'full', yours: undefined, auto: moved, guess: false, busy: over }) === null);
  const quiet = played(q.WARM + q.UNDER_FOR + 2, q.UNDER - 0.05);
  check('lite from the guess, quiet a minute: full', (() => { const x = q.decide({ quality: 'lite', yours: undefined, auto: null, guess: true, busy: quiet }); return x && x.lite === false; })());
  check('lite not from the guess: stays', q.decide({ quality: 'lite', yours: undefined, auto: null, guess: false, busy: quiet }) === null);
  check('lite moved there by the app: stays', q.decide({ quality: 'lite', yours: undefined, auto: moved, guess: true, busy: quiet }) === null);
  const busyLite = played(q.WARM + q.UNDER_FOR + 2, q.UNDER + 0.05);
  check('lite from the guess, not quiet enough: stays', q.decide({ quality: 'lite', yours: undefined, auto: null, guess: true, busy: busyLite }) === null);
  // No flapping: near the line for an hour, at most one move.
  let moves = 0;
  let quality = 'full';
  let auto = null;
  const near = new q.Busy();
  for (let t = 0; t < 3600; t += 2) {
    near.add({ max: q.OVER + 0.04 * Math.sin(t / 7), behind: 0, windows: 8, seconds: 2 }, true, t);
    const x = q.decide({ quality, yours: undefined, auto, guess: false, busy: near });
    if (x) {
      moves++;
      quality = x.lite ? 'lite' : 'full';
      auto = x;
      near.reset();
    }
  }
  check('near the line for an hour: one move at most', moves === 1, `${moves}`);
}

console.log('where it comes from');
check('yours', q.source({ yours: false, auto: { why: 'x' }, cores: 8 }) === 'yours');
check('the app\'s', q.source({ yours: undefined, auto: { why: 'busiest 63% at full' }, cores: 8 }) === 'auto (busiest 63% at full)');
check('the guess', q.source({ yours: undefined, auto: null, cores: 4 }) === 'guess (4 cores)');

console.log('the boundary');
{
  // 120 bpm: a step is 0.125 s; a pass of 32 steps, 4 s.
  const engine = { stepDur: 0.125, step: 10, nextStepTime: 11.25 };
  // Step 10 at 11.25: the pass began at 10.0, the next at 14.0.
  check('the next pass', q.boundary(engine, 32, 12) === 14, `${q.boundary(engine, 32, 12)}`);
  check('the one after, if that is too soon', q.boundary(engine, 32, 14.5) === 18);
  // Steps decided ahead past `after`: the first boundary after it, even if
  // already decided (the old quality drops what it handed over from there).
  check('one already decided', q.boundary(engine, 32, 9.5) === 10, `${q.boundary(engine, 32, 9.5)}`);
  check('the next bar', q.boundary(engine, 16, 12) === 12, `${q.boundary(engine, 16, 12)}`);
  check('no unit, no boundary', q.boundary(engine, 0, 12) === null);
}

console.log('the hand-off');
{
  const calls = [];
  const old = {
    engine: { until: Infinity },
    output: {
      ctx: { currentTime: 3 },
      cancelFrom: (t) => calls.push(['cancelFrom', t]),
      fadeOut: (a, b) => calls.push(['fadeOut', a, b]),
    },
  };
  q.handOff(old, 5, 0.5);
  check('decides nothing from the boundary', old.engine.until === 5);
  check('drops what it handed over from there', calls.some((c) => c[0] === 'cancelFrom' && c[1] === 5));
  check('fades out to it', calls.some((c) => c[0] === 'fadeOut' && c[1] === 4.5 && c[2] === 5));
}

if (failures) {
  console.log(`\n${failures} failed`);
  process.exit(1);
}
console.log('\nall passed');
