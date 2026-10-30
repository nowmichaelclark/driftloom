// The generation data (queue item 78): the text format's reader, and every
// entry of data/*.txt: that it parses and that every name it uses exists.
// Run with:  node test/data.test.mjs

import fs from 'node:fs';
import { parseData, number, flag, readBlock, DATA, FILES } from '../js/data.js';
import { CHARACTERS } from '../js/characters.js';
import { VIBES, VIBE_IDS } from '../js/vibes.js';
import { MOODS } from '../js/moods.js';
import { LAYERS, LAYER_LABELS } from '../js/generator.js';
import { SCALES } from '../js/theory.js';
import { CHOICES, CHANGE_COUNTS, ARC, KICK, SUBDUCK, LATEFLOOR, FLOORDROP, PEDAL, PADSWEEP, ECHO, BASS_LIFT } from '../js/choices.js';

let failures = 0;
function check(label, condition, detail = '') {
  console.log(condition ? `  pass  ${label}` : `  FAIL  ${label} ${detail}`);
  if (!condition) failures++;
}
const throws = (fn, part) => { try { fn(); return false; } catch (e) { return String(e.message).includes(part); } };

console.log('\nThe format');
{
  const text = `# a comment
[thing one]
a = 1  # trailing comment
b = x 2, y 3,
    z 4,  # a continuation
    # a comment between continuations
    w 5
c = 1/3

[thing two]
d =
`;
  const blocks = parseData(text, 't');
  check('two blocks, in order', blocks.length === 2 && blocks[0].name === 'one' && blocks[1].name === 'two');
  check('a number and its comment', JSON.stringify(blocks[0].entries[0]) === '["a",[["1"]]]');
  check('rows, commas and continuations', JSON.stringify(blocks[0].entries[1][1]) === '[["x","2"],["y","3"],["z","4"],["w","5"]]');
  check('an empty value has no rows', blocks[1].entries[0][1].length === 0);
  check('a ratio is one division', number('1/3') === 1 / 3 && number('2/3') === 2 / 3);
  check('a number is the nearest double', number('0.1') === 0.1 && number('1.05') === 1.05 && number('-0') === 0 && Object.is(number('-0'), -0) && number('1e-3') === 0.001);
  check('a flag', flag('true') === true && flag('false') === false);
  check('not a number', throws(() => number('abc'), 'not a number') && throws(() => number('1/'), 'not a number') && throws(() => number('Infinity'), 'not a number'));
  check('non-ASCII fails', throws(() => parseData('[a b]\nk = café\n', 't'), 'ASCII'));
  check('a repeated key fails', throws(() => parseData('[a b]\nk = 1\nk = 2\n', 't'), 'twice'));
  check('an entry before a block fails', throws(() => parseData('k = 1\n', 't'), 'before any'));
  check('a stray line fails', throws(() => parseData('[a b]\nnonsense\n', 't'), 'not a block'));
  check('an unknown key fails, naming it', throws(() => readBlock(parseData('[a b]\nk = 1\n', 't')[0], { j: 'num' }, 't'), "no key 'k'"));
  check('a part reads into an object', JSON.stringify(readBlock(parseData('[a b]\nx.p = 1 2\nx.q = 3 4\n', 't')[0], { 'x.*': 'range' })) === '{"x":{"p":[1,2],"q":[3,4]}}');
}

console.log('\nThe files');
for (const name of FILES) {
  const raw = fs.readFileSync(new URL(`../data/${name}.txt`, import.meta.url), 'utf8');
  check(`data/${name}.txt is plain ASCII, with unix lines`, /^[\x20-\x7e\n]*$/.test(raw) && raw.endsWith('\n'));
  check(`data/${name}.txt has blocks`, DATA[name].length > 0);
}

const unit = (x) => typeof x === 'number' && x >= 0 && x <= 1;
const lean = (x) => typeof x === 'number' && x > 0 && Number.isFinite(x);
const vibeSet = new Set(VIBE_IDS);
const leansOk = (table) => Object.entries(table).every(([id, x]) => vibeSet.has(id) && lean(x));

console.log('\nProfiles');
{
  const bad = [];
  const poolNames = (c, key) => (c[key] || []).map(([n]) => n);
  const VOICES = {};
  for (const [id, c] of Object.entries(CHARACTERS)) {
    if (!c.label || !(c.weight > 0) || !c.bpm || !c.scales || !c.tone || !c.feel) bad.push(`${id}: missing a required key`);
    for (const n of poolNames(c, 'scales')) if (!SCALES[n]) bad.push(`${id}.scales.${n}: not a scale`);
    for (const n of c.borrowedBright || []) if (!SCALES[n]) bad.push(`${id}.borrowedBright.${n}: not a scale`);
    for (const mode of Object.keys(Array.isArray(c.progressions) ? {} : c.progressions || {})) if (!SCALES[mode]) bad.push(`${id}.progressions.${mode}: not a scale`);
    for (const [lo, hi] of [c.bpm, c.swing, c.hatDensity].filter(Boolean)) if (!(lo <= hi)) bad.push(`${id}: a range runs backward`);
    for (const axis of ['warmth', 'space', 'wobble']) if (!c.tone[axis] || !unit(c.tone[axis][0]) || !unit(c.tone[axis][1])) bad.push(`${id}.tone.${axis}`);
    for (const axis of ['lift', 'energy', 'warmth']) if (!c.feel[axis] || !unit(c.feel[axis][0]) || !unit(c.feel[axis][1])) bad.push(`${id}.feel.${axis}`);
    for (const [, w] of [...(c.scales || []), ...(c.chordVoices || []), ...(c.melodyVoices || []), ...(c.textures || [])]) if (!(w > 0)) bad.push(`${id}: a weight is not positive`);
    for (const g of c.gaits || []) if (!(g.weight > 0) || ![12, 16].includes(g.stepsPerBar) || !(g.bpm[0] <= g.bpm[1])) bad.push(`${id}: a bad gait`);
    for (const k of ['chordVoices', 'melodyVoices', 'bassVoices', 'bassStyles', 'textures']) {
      if (!poolNames(c, k).length) bad.push(`${id}.${k}: empty`);
      (VOICES[k] ??= new Set());
      for (const n of poolNames(c, k)) VOICES[k].add(n);
    }
  }
  check('every profile is complete, and every mode it names exists', bad.length === 0, bad.slice(0, 5).join('; '));
}

console.log('\nVibes');
{
  const bad = [];
  for (const id of Object.keys(VIBES)) if (!vibeSet.has(id)) bad.push(`${id}: a vibe not in the order`);
  for (const id of VIBE_IDS) if (!VIBES[id]) bad.push(`${id}: in the order, no recipe`);
  check('every vibe has a place in the share code order, and the order has no other', bad.length === 0, bad.join('; '));
  check('the order has no repeats', vibeSet.size === VIBE_IDS.length);
  const profiles = Object.values(VIBES).flatMap((v) => Object.keys(v.profiles || {}));
  check('every profile a vibe names exists', profiles.every((p) => CHARACTERS[p]));
  const modes = Object.values(VIBES).flatMap((v) => Object.keys(v.modes || {}));
  check('every mode a vibe names exists', modes.every((m) => SCALES[m]));
}

console.log('\nMoods');
{
  const codes = Object.values(MOODS).map((m) => m.code);
  check('codes are whole, distinct numbers', codes.every(Number.isInteger) && new Set(codes).size === codes.length);
  check('every mood has a word and three points in 0..1', Object.values(MOODS).every((m) => /^[a-z]+$/.test(m.word) && unit(m.lift) && unit(m.energy) && unit(m.warmth)));
}

console.log('\nThe voice table');
// The textures take the default (wind's 12 is that figure); 'none' is no voice.
const KNOWN_DEFAULT = new Set(['none', 'bells', 'drops', 'swell', 'wind']);
{
  check('the five layers, in the generator\'s order', JSON.stringify(LAYERS) === '["drums","bass","chords","melody","texture"]');
  check('every layer has a label', LAYERS.every((l) => /^[A-Za-z]+$/.test(LAYER_LABELS[l])) && Object.keys(LAYER_LABELS).length === LAYERS.length);
  const src = DATA.voices;
  const costs = src.find((b) => b.name === 'costs');
  const rows = costs.entries.find(([k]) => k === 'cost')[1];
  check('every voice has one cost, positive', rows.every((r) => r.length === 2 && number(r[1]) > 0) && new Set(rows.map((r) => r[0])).size === rows.length);
  // A voice a profile draws is priced, or priced at the default on purpose:
  // the ones that are not listed are named here, so a new voice is noticed.
  const listed = new Set(rows.map((r) => r[0]));
  const drawn = new Set();
  for (const c of Object.values(CHARACTERS)) for (const k of ['chordVoices', 'melodyVoices', 'bassVoices', 'textures']) for (const [n] of c[k]) drawn.add(n);
  const unpriced = [...drawn].filter((n) => !listed.has(n)).sort();
  check('the voices that fall to the default cost are the known ones', JSON.stringify(unpriced) === JSON.stringify(unpriced.filter((n) => KNOWN_DEFAULT.has(n))), unpriced.join(','));
}

console.log('\nThe choice tables');
{
  const ids = CHOICES.map((c) => c.id);
  check('the choices are the eleven, in the draw\'s order',
    JSON.stringify(ids) === '["glass","dubStab","organ","wash","arc","tunedKick","subDuck","lateFloor","floorDrop","pedalBass","padSweep"]', ids.join());
  check('each has an eligibility test and today first', CHOICES.every((c) => typeof c.eligible === 'function' && c.options[0][0] === 'today' && c.options.every(([, w]) => w > 0)));
  check('every vibe a choice leans exists', CHOICES.every((c) => leansOk(c.vibes || {}) && Object.values(c.optionVibes || {}).every(leansOk)));
  check('every option a vibe leans is an option of the choice', CHOICES.every((c) => Object.keys(c.optionVibes || {}).every((o) => c.options.some(([x]) => x === o))));
  check('the change counts are four, weighted', CHANGE_COUNTS.length === 4 && CHANGE_COUNTS.every(([n, w]) => Number.isInteger(n) && w > 0));
  check('the arc\'s vibe lists are vibes', [ARC.calm, ARC.bright, ARC.glacialNear].every((l) => l.every((v) => vibeSet.has(v))));
  check('the arc\'s shape tables name its shapes', Object.keys(ARC.shapeVibes).every((s) => ARC.shapes.some(([x]) => x === s)) && Object.values(ARC.shapeVibes).every(leansOk));
  check('the arc\'s periods are ranges, in seconds, low to high', Object.values(ARC.period).every(([a, b]) => a > 0 && a <= b));
  check('the kick, subDuck and padSweep ranges run low to high', [KICK.note, KICK.short, KICK.long, SUBDUCK.depth, SUBDUCK.recover, PADSWEEP.lo, PADSWEEP.oct, PADSWEEP.period, PADSWEEP.q, PEDAL.share, LATEFLOOR.seconds].every(([a, b]) => a <= b));
  check('lateFloor\'s vibes and orders', leansOk(LATEFLOOR.longVibes) && LATEFLOOR.orders.map(([o]) => o).join() === 'together,kitFirst,bassFirst');
  check('floorDrop\'s shapes and the ones that need a bass', FLOORDROP.needBass.every((s) => FLOORDROP.shapes.some(([x]) => x === s)) && Object.values(FLOORDROP.shapeVibes).every(leansOk));
  check('pedalBass\'s styles, rhythms and neighbors', PEDAL.styles.length === 4 && PEDAL.rhythms.length === 4 && PEDAL.neighbors.every(([n]) => Number.isInteger(n)) && Object.values(PEDAL.neighborVibes).every(leansOk));
  check('padSweep\'s sustained voices are named once each', new Set(PADSWEEP.sustained).size === PADSWEEP.sustained.length && PADSWEEP.sustained.includes('pad'));
  check('the echo\'s times: today\'s dotted eighth first, two thirds a quotient', ECHO.beats[0][0] === 0.75 && ECHO.beats[3][0] === 2 / 3 && Object.values(ECHO.vibes).every(leansOk));
  check('the bass lift has its two values', BASS_LIFT.kick === 3 && BASS_LIFT.noKick === 0);
}

console.log(failures ? `\n${failures} FAILED` : '\nAll good.');
process.exit(failures ? 1 : 0);
