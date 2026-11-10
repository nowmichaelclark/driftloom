// The generation goldens (queue item 77): for a fixed set of seeds and
// requests, hashes of everything the generator decides. They were taken from
// the JavaScript generator, and the Rust port (phase 2, items 79-82) was held
// to them step by step; since item 97 the core's generator is the only one,
// and the file holds it where it is: nothing that is generated moves unless
// a PR means it to and says so.
//
//   node tools/gen-golden.mjs              write the hashes of every case in
//                                          core/golden/generation.hashes again
//   node tools/gen-golden.mjs --check      fail on any case whose hashes differ from the file
//   node tools/gen-golden.mjs --show CASE [PART]
//                                          print a case's canonical texts (CASE as the
//                                          file names it, corpus:12; PART one of the
//                                          hashes' names), to see what a change moved
//   node tools/gen-golden.mjs --corpus N   print the lines of the first N corpus loops
//                                          (stdout; for counting what a change moves)
//   options:  --wasm FILE   the module to read (default core/target/dltools.wasm,
//                           which tools/build-core.sh --tools writes;
//                           tools/gen-core.sh passes the generator's own build)
//
// The file is the list of cases: a new case is a line of its first three
// fields (SET:INDEX SEED REQUEST), and a run without --check writes its
// hashes. tools/gen-core.sh checks the same file natively, on any target.
//
// The file. One case a line, after '#' comments:
//
//   SET:INDEX SEED VIBES spec=H plan=H form=H drums=H bass=H chords=H melody=H
//     texture=H meta=H choices=H load=H passes=H1,...,H8
//
// SEED is the loop's seed (unsigned), VIBES the request (queue item 85) as
// its text (`Request::parse`): the vibes asked for, in the frozen order,
// comma-separated, each with its weight or not
// ("glacial=0.7"), the tempo ("bpm=77", "bpm=70-85"), the theme
// ("theme=1234"), the pins ("organ=on", "arc.period=240"; js/pins.js), the
// length ("bars=1024", "bars=endless"), the key ("key=D", "key=D|G") and the
// mode ("scale=dorian", "scale=dorian|aeolian") and the style ("style=tide",
// "style=tide|shrine"), a layer's voices ("voice.melody=kalimba|marimba")
// and a layer on or off ("layer.drums=off", "layer.melody=on"); '-' for
// nothing.
// The loop is the one SEED and that request make. The parts' names are the
// JavaScript generator's, which the port kept (`Part::ALL`,
// core/src/generator/canon.rs). Each H is an FNV-1a 64 hash (16 hex digits)
// of a canonical text (below) of:
//   spec     the spec (and, in the edit cases, with its edits made);
//   plan     what render plans before any layer plays (`planOf`, queue item
//            79): the development's sections (null for a loop that does not
//            develop), the harmony as drawn, before the register work moves
//            it, and the entry schedules (null for none) and gaps as drawn,
//            before the repairs;
//   form     what `render` decides besides the layers' notes: form, gaps,
//            silentBars, totalSteps, stepsPerBar, cycles, harmony (the
//            progression's slots and events, its shape, longChords);
//   drums ... texture
//            each layer's events, after render (entries, rests and gaps
//            applied);
//   meta     render's meta (the voices, bass style, kit, melody cell,
//            texture);
//   choices  item 54's per-loop choices (`drawChoices`, through render);
//   load     what the engine draws and keeps at load (`Engine.load`):
//            glassScale, stab, organ, wash, arc, kick, subDuck, padSweep,
//            bassLift, late (lateFloor), space, echoBeats and pumpAmount
//            (each null where the loop's choices leave it out), and
//            `calls`, every call it makes to the synth there, in order
//            ([name, ...arguments]: the tone, the character's level, the
//            echo time, the mutes). Nothing at load may draw from
//            Math.random: the tool makes it throw.
//   passes   the first eight wandered passes (`drift`, through
//            `Engine._wander(pass)`, pass 1 to 8, at the default amount, 1):
//            the whole pattern each pass plays, less its spec, and with
//            `kick`, the tuned kick that pass plays (`kickKindOf`) where the
//            loop has one.
//
// The canonical text, which a Rust port writes the same way:
//   - a number that is an integer of at most 2^53 in size, in decimal
//     ('-0' for negative zero); any other finite number 'x' and the 16 hex
//     digits of its IEEE 754 bits (0.5 is x3fe0000000000000); NaN 'nan';
//     the infinities 'inf' and '-inf';
//   - a string in double quotes, '\' and '"' escaped with '\' (printable
//     ASCII only: anything else fails);
//   - true, false, null; undefined, in a list, 'undef';
//   - a list '[a,b,c]';
//   - an object '{k:v,k:v}', its keys sorted (by character code), unquoted,
//     and any key whose value is undefined left out.
// No spaces anywhere. FNV-1a 64 runs over the text's bytes: offset basis
// 0xcbf29ce484222325, prime 0x100000001b3.
//
// The cases, as they were drawn when the file was first written (the
// drawing code retired with the JavaScript generator; the file keeps them):
//   corpus:0-1999   the first 2,000 loops of corpus seed 1 (Rng(1).seed32(),
//                   as tools/stats.mjs draws its corpus), no vibes;
//   vibes:0-999     1,000 seeds from Rng(77), each with 1, 3 or 5 vibes in
//                   turn, drawn from the same stream (a shuffle of VIBE_IDS);
//   golden:NAME     the 21 golden lists' seeds (core/golden/codes.txt), no
//                   vibes (their codes keep older specs: the case is the seed,
//                   generated today);
//   album:N         the album loops of docs/DONE.md: "dust #7" is loop 7 of
//                   corpus seed 20261009, the ears' corpus; and noan-nou;
//   request:0-359   360 seeds from Rng(85), each with a request of the kind
//                   its index picks (gen-golden-lib.mjs, `requestCase`):
//                   tempo pinned, ranged, weighted vibes, six or seven vibes,
//                   vibes with a pin, weights past 1 with a range;
//   pins:0-239      240 seeds from Rng(8502), each with pins of the kind its
//                   index picks (gen-golden-lib.mjs, `pinCase`; queue item
//                   85b): a choice, several choices, the arc's settings, the
//                   kick's, subDuck's and lateFloor's, padSweep's, the echo's
//                   and the space's, and pins with vibes and a tempo;
//   theme:0-239     240 seeds from Rng(8504), each with a theme of the kind
//                   its index picks (gen-golden-lib.mjs, `themeCase`; queue
//                   item 85d): the loop's own seed alone, with vibes, with a
//                   tempo, with pins, and another seed with vibes;
//   rest:0-23       24 themes whose own loop rests, each on a seed whose loop
//                   draws a choir (gen-golden-lib.mjs, `restCases`; queue
//                   item 85f): the brain's case first, then from Rng(8506),
//                   alone, with vibes, weighted vibes or a tempo.
//   length:0-239    240 seeds from Rng(88), each with a length of the kind its
//                   index picks (gen-golden-lib.mjs, `lengthCase`; queue item
//                   88): 1 to 32 bars (the loop's own), 1, 2 or 3 bars, a long
//                   piece (33 to 40,000 bars, or 32,768), Endless, and short
//                   or long with vibes, a tempo, pins or a theme.
//   key:0-239       240 seeds from Rng(89), each with a key and a mode of the
//                   kind its index picks (gen-golden-lib.mjs, `keyCase`; queue
//                   item 89): one key, a set of keys, one mode, a set of
//                   modes, one of each, sets of both with vibes, one of each
//                   with a tempo and pins, and a theme with a key or a mode.
//   style:0-239     240 seeds from Rng(90), each with a style of the kind its
//                   index picks (gen-golden-lib.mjs, `styleCase`; queue item
//                   90): one profile, a set of two to five, one with vibes,
//                   with a tempo, a set with a key and a mode, one with pins,
//                   and a theme with a style asked.
//   voice:0-239     240 seeds from Rng(91), each with voices and layers of the
//                   kind its index picks (gen-golden-lib.mjs, `voiceCase`;
//                   queue item 91): one voice of a layer, a set, voices of
//                   several layers, a layer off, a layer on, voices and layers
//                   with a style and a key, with vibes and a tempo, and a theme
//                   with voices and a layer.
//   edit:0-239      240 seeds from Rng(94), each with a code's edits of the
//                   kind its index picks (gen-golden-lib.mjs, `editCase`;
//                   queue item 94a): a layer re-rolled (`edit.roll.drums=N`),
//                   two, a layer's seed alone (`edit.seed.bass=N`), the tone
//                   (`edit.warmth=0.6`, `edit.space`, `edit.wobble`), the
//                   tempo the slider set (`edit.bpm=88`), and re-rolls with
//                   vibes, a request, a length or a theme. The spec is the
//                   request's, the edits then made on it as the app makes
//                   them (`edited`).
//
// Run it after any change to the generator, the vibes, the choices or what
// is drawn at load. A change that is meant to move what is generated writes
// the file again in the same PR and says in the body which cases moved and
// why. The gate checks the file (tools/gate.sh node).

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { casesOf, lineOf, fnv1a64 } from './gen-golden-lib.mjs';
import * as gen from './gen-lib.mjs';
import { Rng } from '../js/rng.js';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const FILE = path.join(ROOT, 'core', 'golden', 'generation.hashes');
const argv = process.argv.slice(2);
const flag = (name, dflt) => {
  const i = argv.indexOf(name);
  return i >= 0 ? argv[i + 1] : dflt;
};
if (argv.includes('--help') || argv.includes('-h')) {
  console.log(fs.readFileSync(fileURLToPath(import.meta.url), 'utf8').split('\n').filter((l) => l.startsWith('//')).map((l) => l.slice(3)).join('\n'));
  process.exit(0);
}

try {
  gen.open(flag('--wasm', gen.TOOLS_WASM));
} catch (err) {
  console.error(`gen-golden: ${err.message}`);
  process.exit(3);
}
const text = fs.readFileSync(FILE, 'utf8');
const header = text.split('\n').filter((l) => l.startsWith('#')).join('\n');
const cases = casesOf(text);
// The corpus's seed (tools/stats.mjs draws its corpus the same way).
const CORPUS_SEED = 1;

// ------------------------------------------------------------- --show

if (argv.includes('--show')) {
  const want = flag('--show', '');
  const part = argv[argv.indexOf('--show') + 2];
  const c = cases.find((x) => x.name === want);
  if (!c) {
    console.error(`gen-golden: no case ${want}`);
    process.exit(2);
  }
  const texts = gen.parts(c.seed, c.request, true);
  const { hashes } = gen.hashes(c.seed, c.request);
  for (const [k, t] of Object.entries(texts)) {
    if (part && part !== k) continue;
    if (k === 'passes') t.forEach((x, i) => console.log(`passes[${i}] ${hashes.passes[i]}\n${x}\n`));
    else console.log(`${k} ${hashes[k]}\n${t}\n`);
  }
  process.exit(0);
}

// ------------------------------------------------------------- --corpus

if (argv.includes('--corpus')) {
  const n = Number(flag('--corpus', 0));
  const r = new Rng(CORPUS_SEED);
  for (let i = 0; i < n; i++) {
    const c = { name: `corpus:${i}`, seed: r.seed32(), request: null };
    console.log(lineOf(c, gen.hashes(c.seed, null).hashes));
  }
  process.exit(0);
}

// ------------------------------------------------------------- the lines

const t0 = Date.now();
const check = argv.includes('--check');
let bad = 0;
const lines = [];
for (const c of cases) {
  let made;
  try {
    made = gen.hashes(c.seed, c.request);
  } catch (err) {
    console.error(`gen-golden: ${c.name}: ${err.message}`);
    process.exit(1);
  }
  const h = made.hashes;
  lines.push(lineOf(c, h));
  if (!check) continue;
  // Which hashes moved, by name (a pass by its number); the spec read as
  // text and hashed here too, so the host's call is checked; and a list
  // that lost an item is never right.
  const moved = Object.keys(h).filter((k) => k !== 'passes' && c.want[k] !== h[k]);
  h.passes.forEach((x, i) => { if ((c.want.passes || [])[i] !== x) moved.push(`pass ${i + 1}`); });
  if (fnv1a64(gen.parts(c.seed, c.request, true).spec) !== c.want.spec && !moved.includes('spec')) moved.push('the spec as text');
  if (made.lost) moved.push('lost items');
  if (!moved.length) continue;
  bad++;
  if (bad <= 40) console.log(`  moved ${c.name}: ${moved.join(', ')}`);
}
const seconds = ((Date.now() - t0) / 1000).toFixed(1);

if (check) {
  if (bad > 40) console.log(`  ... and ${bad - 40} more`);
  console.log(bad ? `gen-golden: ${bad} of ${cases.length} cases differ from ${path.relative(ROOT, FILE)} (${seconds} s)`
    : `gen-golden: all ${cases.length} cases equal ${path.relative(ROOT, FILE)} (${seconds} s)`);
  process.exit(bad ? 1 : 0);
}

fs.writeFileSync(FILE, `${header}\n${lines.join('\n')}\n`);
console.log(`gen-golden: wrote ${lines.length} cases to ${path.relative(ROOT, FILE)} (${seconds} s)`);
