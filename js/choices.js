// The per-loop choices' tables, as the page reads them (queue items 54 and
// 78): data/choices.txt's choices and settings, for the pins a request or a
// share code carries (js/pins.js: which options a choice has, which range a
// setting takes). Which changes a loop gets, and every draw, is the core's
// (`core/src/generator/choices.rs`, the same file compiled in).

import { blockOf, blocksOf, readBlock } from './data.js';

const WHERE = 'data/choices.txt';
const setting = (name, schema) => readBlock(blockOf('choices', 'setting', name), schema, WHERE);

// The data of every choice: { options, weight?, vibes?, optionVibes?,
// ownStream? } (ownStream: the option is drawn on a stream of its own, queue
// item 101).
const CHOICE_DATA = Object.fromEntries(blocksOf('choices', 'choice').map((b) => [b.name, readBlock(b, {
  options: 'pool', weight: 'num', vibes: 'table', 'optionVibes.*': 'table', ownStream: 'num',
}, WHERE)]));
const choiceData = (id) => CHOICE_DATA[id];

// The arc's settings (queue items 63 and 63b): every variable of the arc,
// named, with the range a loop draws it from, in this one place, so phase 2's
// requests can pin them (docs/CORE.md, promises 9 and 11) and a later slider
// or a game host can set them. The core draws a loop's from them and
// plays it (core/src/blocks/arc.rs). The file says what each is.
export const ARC = {
  weight: choiceData('arc').weight,
  vibes: choiceData('arc').vibes,
  ...setting('arc', {
    shapes: 'pool', 'shapeVibes.*': 'table', 'period.*': 'range', calm: 'words', bright: 'words', glacialNear: 'words',
    glacialChance: 'num', 'from.*': 'range', floor: 'range', dip: 'num',
  }),
};

// The tuned kick's settings (queue item 67): every variable of it, named,
// with its range, as ARC's are. The core draws a loop's from them, works
// out its level at load and plays it (core/src/blocks/kick.rs).
export const KICK = {
  options: choiceData('tunedKick').options,
  optionVibes: choiceData('tunedKick').optionVibes,
  ...setting('kick', {
    note: 'range', short: 'range', long: 'range', loud: 'num', tailLoud: 'num', band: 'num', swap: 'num',
  }),
};

// subDuck's settings (queue item 68): the bass channel alone dips at each
// kick and comes back, so the kick and the sub take turns; `deep` is
// deepDuck's own ranges (item 101a). The core draws a loop's and plays it
// (core/src/blocks/sub_duck.rs).
export const SUBDUCK = {
  weight: choiceData('subDuck').weight,
  vibes: choiceData('subDuck').vibes,
  ...setting('subduck', { depth: 'range', recover: 'range', beat: 'num', 'deep.*': 'range' }),
};

// lateFloor's settings (queue item 69): after a load the kit and the bass sit
// out a while, then arrive; `slow` is slowOpening's wait and fade (item
// 101a). The core draws a loop's and its player plays it
// (core/src/player/sequencer.rs).
export const LATEFLOOR = {
  weight: choiceData('lateFloor').weight,
  vibes: choiceData('lateFloor').vibes,
  ...setting('latefloor', {
    seconds: 'range', round: 'num', first: 'num', orders: 'pool', gap: 'numpool', maxGap: 'num', gapLong: 'num', longVibes: 'table',
    'slow.*': 'range',
  }),
};

// padSweep's settings (queue item 72): a lowpass on the chord layer swings
// open and shut while the rest stays as bright as it was; `wide` and `fast`
// are wideSweep's two, each the same four ranges (item 101a). The core draws
// a loop's and plays it (core/src/blocks/pad_sweep.rs).
export const PADSWEEP = {
  weight: choiceData('padSweep').weight,
  vibes: choiceData('padSweep').vibes,
  ...setting('padsweep', {
    sustained: 'words', lo: 'range', oct: 'range', period: 'range', q: 'range', 'wide.*': 'range', 'fast.*': 'range',
  }),
};

// echoTime's settings (queue item 73): the echo's time drawn per loop, in
// beats. Drawn for every loop as the sketch drew it, outside the list of
// choices and not counted among a loop's changes.
export const ECHO = setting('echo', { beats: 'numpool', 'vibes.*': 'table' });

// tuningOffset's settings (queue item 101b): how often a loop plays a little
// off A440, and how far, drawn per loop outside the list of choices, as
// echoTime is. The page reads them for the tuning pin's range.
export const TUNING = setting('tuning', { share: 'num', cents: 'range', vibes: 'table' });

// Every choice, in the file's order (the pins' order): its id and its
// options, today's first.
export const CHOICES = Object.keys(CHOICE_DATA).map((id) => ({ id, ...CHOICE_DATA[id] }));
