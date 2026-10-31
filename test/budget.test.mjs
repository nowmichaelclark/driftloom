// The voice budget's taking room from old notes (queue item 85c): the same
// steps as core/src/player/budget.rs's `takes_room_from_its_own_layers_oldest_calls`,
// so the page and the player let the same notes go.
// Run with:  node test/budget.test.mjs

import { Synth } from '../js/synth.js';

let failures = 0;
function check(label, condition, detail = '') {
  console.log(condition ? `  pass  ${label}` : `  FAIL  ${label} ${detail}`);
  if (!condition) failures++;
}

// The budget's methods on a bare synth: a core that writes down what it is
// told to let go, and nothing else.
function budget(quality) {
  const steals = [];
  const s = {
    quality, engine: 'rust', core: { steal: (...a) => steals.push(a) },
    ctx: { currentTime: 0 }, _releases: [], _group: null, _building: null,
    stolen: { drums: 0, bass: 0, chords: 0, melody: 0, texture: 0 },
  };
  for (const name of ['_budget', '_steal', '_release', '_room']) s[name] = Synth.prototype[name];
  return { s, steals };
}

const at = (step, ord) => ({ seed: 1, pass: 0, step, ord });
const call = (s, layer, key, channels, fn) => {
  s._group = { key, layer, channels };
  try { return fn(); } finally { s._group = null; }
};

console.log('\nTaking room from old notes (lite)');
{
  const { s, steals } = budget('lite');
  // Two melody notes' worth and a chord, each its own call, 60 each on
  // lite's 140: the first melody call on channels 5 and 3 (a knock).
  call(s, 'melody', at(0, 0), [5, 3], () => s._release(0.0, 4.0, 60));
  call(s, 'chords', at(0, 0), [2], () => s._release(0.1, 4.0, 60));
  const ok = call(s, 'melody', at(4, 0), [], () => s._budget(1.0, false, 30));
  check('a note that does not fit takes the oldest call of its own layer', ok);
  check('faded over the 0.05 s before it, in each of its channels, lowest first',
    JSON.stringify(steals) === JSON.stringify([[3, at(0, 0), 0.95, 1.0], [5, at(0, 0), 0.95, 1.0]]), JSON.stringify(steals));
  check('counted, and the chord kept', s.stolen.melody === 1 && s._room(1.0, 80) && !s._room(1.0, 81));
  steals.length = 0;
  check('nothing of its own layer from another step: refused, nothing let go',
    !call(s, 'melody', at(8, 0), [], () => s._budget(1.5, false, 90)) && steals.length === 0);
  check('a call never takes from its own step (a chord\'s own tones)',
    !call(s, 'chords', at(0, 1), [], () => s._budget(1.5, false, 90)) && steals.length === 0);
}
{
  const { s, steals } = budget('full');
  call(s, 'melody', at(0, 0), [3], () => s._release(0.0, 4.0, 250));
  check('at full quality it refuses as before', !call(s, 'melody', at(4, 0), [], () => s._budget(1.0, false, 30)) && steals.length === 0);
}
{
  const { s, steals } = budget('lite');
  s.engine = 'js';
  s.core = null;
  call(s, 'melody', at(0, 0), [], () => s._release(0.0, 4.0, 130));
  check('the JavaScript synth (no core) refuses as before', !call(s, 'melody', at(4, 0), [], () => s._budget(1.0, false, 30)) && steals.length === 0);
}

console.log(failures ? `\n${failures} failed` : '\nall passed');
process.exit(failures ? 1 : 0);
