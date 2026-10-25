// Regression tests for the parts of Driftloom that do not need a browser.
// Run with:  node test/generator.test.mjs
//
// The generator is the piece most likely to break silently. A bad chord
// voicing or an out-of-range note does not throw, it just sounds wrong,
// and often only on one seed in a thousand. So we check thousands.

import { newSpec, render, drift, rerollLayer, LAYERS, gracesOf, harmonyOf, developmentOf, DEPTH, characterOf, withTodaysChords } from '../js/generator.js';
import { Rng, randomSeed, seedName } from '../js/rng.js';
import { patternToMidi } from '../js/midi.js';
import { SCALES, SCALE_BRIGHTNESS } from '../js/theory.js';
import { MOODS, moodForCode, codeForMood, moodWord, pointWord } from '../js/moods.js';
import { encodeSong, decodeSong, encodeAlbum, decodeAlbum } from '../js/share.js';
import { VIBES, VIBE_IDS } from '../js/vibes.js';
import { CHARACTERS } from '../js/characters.js';
import { CHOICES, CHANGE_COUNTS, drawChoices, changesIn, ARC, arcOf, KICK, kickOf, kickKindOf, SUBDUCK, subDuckOf, LATEFLOOR, lateFloorOf, FLOORDROP, PEDAL, pedalOf } from '../js/choices.js';

// Node ships a real Blob, but it only hands its bytes back asynchronously.
// Override it so the export can be inspected byte for byte.
globalThis.Blob = class { constructor(parts) { this.bytes = parts[0]; } };

let failures = 0;
function check(label, condition, detail = '') {
  console.log(condition ? `  pass  ${label}` : `  FAIL  ${label} ${detail}`);
  if (!condition) failures++;
}

// ---------------------------------------------------------------------
console.log('\nGeneration across 5000 seeds');

const SEEDS = 5000;
const problems = [];
const range = { melody: [999, 0], bass: [999, 0], chords: [999, 0] };
let drumless = 0;
let sparse = 0;

for (let i = 0; i < SEEDS; i++) {
  const spec = newSpec(randomSeed());
  const p = render(spec);
  let sounding = 0;

  for (const layer of LAYERS) {
    for (const e of p.tracks[layer]) {
      if (e.vel > 0) sounding++;
      const cycle = (p.cycles && p.cycles[layer]) || p.totalSteps;
      if (e.step < 0 || e.step >= cycle) problems.push(`${layer} step ${e.step}/${cycle}`);
      if (!Number.isFinite(e.vel)) problems.push(`${layer} velocity ${e.vel}`);
      const notes = e.notes || (e.midi != null ? [e.midi] : []);
      for (const n of notes) {
        if (!Number.isInteger(n) || n < 21 || n > 108) {
          problems.push(`${layer} note ${n} in ${spec.scale}`);
        }
        if (range[layer]) {
          range[layer][0] = Math.min(range[layer][0], n);
          range[layer][1] = Math.max(range[layer][1], n);
        }
      }
    }
  }
  if (!p.tracks.drums.length) drumless++;
  if (sounding < 6) sparse++;

  // Drift has to respect the same limits, even at full intensity.
  const d = drift(p, new Rng(randomSeed()), 2.4);
  for (const e of d.tracks.melody) {
    if (e.midi < 40 || e.midi > 108) problems.push(`drifted melody ${e.midi}`);
  }
  // The bass sits in 36-57 (roundBass, queue item 56), wandering too; a
  // spelled chord may move a note a semitone after it is placed (cinder's
  // G# over E), and a drone's fifth sits a fifth over its root.
  const top = (p.meta.bassStyle === 'drone' ? 64 : 57) + 1;
  for (const e of p.tracks.bass) {
    if (e.midi < 35 || e.midi > top) problems.push(`bass ${e.midi}`);
  }
  for (const e of d.tracks.bass) {
    if (e.midi < 35 || e.midi > top) problems.push(`drifted bass ${e.midi}`);
  }
}

check('every note lands inside the playable range', problems.length === 0, problems.slice(0, 4).join('; '));
check('loops are almost never empty', sparse / SEEDS < 0.01, `${((sparse / SEEDS) * 100).toFixed(2)}% empty`);
// Six of the sixteen profiles rarely or never have drums (haven and vapor
// never; thaw, clockwork, hollow and shrine about one loop in ten), so a
// large share of loops having no percussion is the design, not a fault.
// The band is wide; the printed figure is the thing to actually look at.
check('drums appear on a reasonable share of loops', drumless / SEEDS > 0.2 && drumless / SEEDS < 0.6, `${((drumless / SEEDS) * 100).toFixed(1)}% drumless`);
console.log(`        registers: melody ${range.melody}, bass ${range.bass}, keys ${range.chords}`);

// ---------------------------------------------------------------------
console.log('\nSeeds carry the whole state');

const base = newSpec(20260903);
const a = render(base);
check('the same seed gives the same loop', JSON.stringify(a.tracks) === JSON.stringify(render(base).tracks));

const rolled = render(rerollLayer(base, 'bass'));
check('re-rolling bass changes the bass', JSON.stringify(a.tracks.bass) !== JSON.stringify(rolled.tracks.bass));
check('re-rolling bass leaves drums untouched', JSON.stringify(a.tracks.drums) === JSON.stringify(rolled.tracks.drums));
check('re-rolling bass leaves melody untouched', JSON.stringify(a.tracks.melody) === JSON.stringify(rolled.tracks.melody));

// A single re-roll can legitimately land on the same chord roots -- short
// progressions repeat, and a sparse bass may place the same few notes
// either way. The claim is that the bass *follows* the harmony, so test it
// over several re-rolls rather than demanding one differ.
let bassFollowed = false;
let drumsHeld = true;
for (let i = 0; i < 12; i++) {
  const reharmonised = render(rerollLayer(base, 'chords'));
  if (JSON.stringify(a.tracks.bass) !== JSON.stringify(reharmonised.tracks.bass)) bassFollowed = true;
  if (JSON.stringify(a.tracks.drums) !== JSON.stringify(reharmonised.tracks.drums)) drumsHeld = false;
}
check('re-rolling chords carries the bass with it', bassFollowed);
check('re-rolling chords keeps the drum groove', drumsHeld);

check('drift never edits the pattern it was given', (() => {
  const before = JSON.stringify(a.tracks);
  drift(a, new Rng(7), 2);
  return JSON.stringify(a.tracks) === before;
})());

// ---------------------------------------------------------------------
console.log('\nTide');

// Tempo and metre are drawn together, so a twelve-step tide loop at waltz
// tempo is a waltz, and one at jig tempo is a jig.
const tides = [];
for (let i = 0; tides.length < 400 && i < 20000; i++) {
  const spec = newSpec(randomSeed());
  const lead = Object.entries(spec.mix).sort((x, y) => y[1] - x[1])[0][0];
  if (lead === 'tide') tides.push({ spec, p: render(spec) });
}
check('tide loops keep tempo and metre together', tides.length === 400 && tides.every(({ spec }) => (spec.stepsPerBar === 16
  ? spec.bpm >= 112 && spec.bpm <= 136
  : (spec.bpm >= 84 && spec.bpm <= 104) || (spec.bpm >= 112 && spec.bpm <= 140))));
const waltzes = tides.filter(({ spec }) => spec.stepsPerBar === 12 && spec.bpm <= 104);
check('a tide waltz leaves the downbeat to the bass', waltzes.length > 50 && waltzes.every(({ p }) => p.harmony.events
  .filter((e) => e.notes.length > 1)
  .every((e) => [4, 6, 8, 10].includes(e.step % 12))), `${waltzes.length} waltzes`);
const drones = tides.filter(({ p }) => p.meta.bassStyle === 'drone');
// The form may still silence the bass for a section; what it silences is
// the drone, held from every downbeat through its bar.
check('a drone is struck on every downbeat and held through the bar', drones.length > 60 && drones.every(({ spec, p }) => {
  const spb = spec.stepsPerBar;
  const bass = p.tracks.bass;
  return bass.every((e) => e.step % spb === 0 && e.dur === spb - 1)
    && new Set(bass.map((e) => e.step)).size === spec.bars;
}), `${drones.length} drones`);

// The waltz tune sits on the beats and the eighths between them, and its
// breaths are whole beats of the waltz, not of 6/8.
check('a tide waltz tune falls on the eighths', waltzes.every(({ p }) => p.tracks.melody.every((e) => e.step % 2 === 0)));
check('a tide waltz breathes in waltz beats', waltzes.every(({ p }) => p.gaps.every((g) => g.start % 4 === 0 && [4, 8, 12].includes(g.len))));

// Graces and harmony are marks on the tune, played from the note as it
// stands; they only ever name a neighbour in the scale, and a real third or
// sixth below.
const heard = tides.flatMap(({ spec, p }) => p.tracks.melody.filter((e) => e.vel).map((e) => ({ spec, e })));
const graced = heard.filter(({ e }) => e.orn);
check('ornaments decorate some notes, never most', graced.length > 100 && graced.length < heard.length * 0.3, `${graced.length} of ${heard.length}`);
check('a grace is the scale neighbour of its note', graced.every(({ spec, e }) => {
  const g = gracesOf(e, spec);
  return g.length === (e.orn === 'turn' ? 2 : 1) && g[0] > e.midi && g[0] - e.midi <= 4 && (g.length === 1 || (g[1] < e.midi && e.midi - g[1] <= 4));
}));
const harmonised = tides.filter(({ p }) => p.tracks.melody.some((e) => e.harm));
check('a harmony line sits under some loops, never under every note', harmonised.length > 60 && harmonised.every(({ p }) => {
  const sounding = p.tracks.melody.filter((e) => e.vel);
  return sounding.filter((e) => e.harm).length < sounding.length;
}), `${harmonised.length} loops`);
// The hand kit is the kit's pattern on a frame drum and a tambourine: no
// kit instrument left in it, and no hand instrument anywhere else.
const HAND_INSTS = new Set(['frame', 'tap', 'jingle', 'ojingle']);
const handLoops = tides.filter(({ p }) => p.meta.kit === 'hand');
check('the hand kit plays frame, tap, zils and shaker only', handLoops.length > 20 && handLoops.every(({ p }) => p.tracks.drums
  .every((e) => HAND_INSTS.has(e.inst) || e.inst === 'shaker')), `${handLoops.length} hand-kit loops`);
check('no other kit plays a hand instrument', tides.filter(({ p }) => p.meta.kit !== 'hand')
  .every(({ p }) => p.tracks.drums.every((e) => !HAND_INSTS.has(e.inst))));
// Waves: one slow swell after another, six to nine seconds each.
const seas = tides.filter(({ p }) => p.meta.textureKind === 'waves');
check('the sea swells in waves of six to nine seconds', seas.length > 20 && seas.every(({ spec, p }) => {
  const sd = 60 / spec.bpm / 4;
  return p.tracks.texture.length > 0 && p.tracks.texture.every((e) => e.kind === 'waves'
    && (e.dur * sd >= 5.9 || e.dur === spec.stepsPerBar) && e.dur * sd <= 9.1);
}), `${seas.length} loops`);
// A nylon guitar strums down on the beat and up off it; nothing else strums.
const strummed = tides.filter(({ p }) => p.tracks.chords.some((e) => e.voice === 'nylon'));
check('nylon strums down on the beat and up off it, and nothing else strums', strummed.length > 20 && tides.every(({ spec, p }) => {
  const beat = spec.stepsPerBar === 12 && spec.bpm >= 112 ? 6 : 4;
  return p.tracks.chords.every((e) => (e.voice === 'nylon'
    ? e.strum === ((e.step % spec.stepsPerBar) % beat === 0 ? 'down' : 'up')
    : e.strum === undefined));
}), `${strummed.length} loops with nylon`);
check('the harmony is a third or a sixth below', heard.filter(({ e }) => e.harm).every(({ spec, e }) => {
  const h = harmonyOf(e, spec);
  if (h == null) return true;
  const gap = e.midi - h;
  return e.harm === 'third' ? gap === 3 || gap === 4 : gap === 8 || gap === 9;
}));

// ---------------------------------------------------------------------
console.log('\nCinder');

const cinders = [];
for (let i = 0; cinders.length < 300 && i < 20000; i++) {
  const spec = newSpec(randomSeed());
  const lead = Object.entries(spec.mix).sort((x, y) => y[1] - x[1])[0][0];
  if (lead === 'cinder') cinders.push({ spec, p: render(spec) });
}
check('cinder loops run a fast 6/8, some 4/4', cinders.length === 300 && cinders.every(({ spec }) => (spec.stepsPerBar === 12
  || spec.stepsPerBar === 16) && spec.bpm >= 120 && spec.bpm <= 150));
const drummed = cinders.filter(({ p }) => p.tracks.drums.length).length;
check('drums play in about nine cinder loops in ten', drummed > 300 * 0.8 && drummed < 300 * 0.97, `${drummed} of 300`);
// The Andalusian cadence, in A: Am-G-F-E, the last chord major. In each
// mode it is written for, every chord of it comes out a plain triad of the
// quality the cadence wants, whatever the mode's own degrees would make.
const pcOf = (n) => ((n % 12) + 12) % 12;
// A blend can bring two-note chords, and stacked fourths that have no third
// at all; what no chord may do is carry the wrong third.
const third = (slot) => {
  const pcs = new Set(slot.notes.map(pcOf));
  const r = pcOf(slot.rootMidi);
  const [minor, major] = [pcs.has(pcOf(r + 3)), pcs.has(pcOf(r + 4))];
  return major && !minor ? 'M' : minor && !major ? 'm' : '-';
};
// A two-bar loop has room for the first two chords of it. A loop that
// develops can play it in its departure rather than from the top, so the
// four chords are read from the start of the section it is in.
// A section has only its own chords: a two-bar departure holds the first
// two of the cadence, as a two-bar loop does.
const cadenceRange = ({ spec, p }) => {
  const dev = developmentOf(spec);
  if (!dev) return [0, Infinity];
  const first = p.harmony.slots.find((s) => s.steps).startStep;
  let at = 0;
  let range = [0, Infinity];
  for (const sec of dev.sections) {
    if (first >= at * p.stepsPerBar) range = [at * p.stepsPerBar, (at + sec.bars) * p.stepsPerBar];
    at += sec.bars;
  }
  return range;
};
// Read from today's chord lengths: a loop that holds its chords longer
// (queue item 57) plays the same progression, more slowly.
const cadenceOf = (x) => {
  const [from, to] = cadenceRange(x);
  return withTodaysChords(x.spec).harmony.slots.filter((s) => s.startStep >= from && s.startStep < to).slice(0, 4);
};
const cadences = cinders.filter(({ p }) => p.harmony.slots.some((s) => s.steps));
const whole = cadences.filter((x) => cadenceOf(x).length >= 4);
const spelt = whole.map((x) => cadenceOf(x).map(third).join(''));
check('the Andalusian cadence falls a tone, a tone, a semitone, to a major chord', whole.length > 30 && whole.every((x) => {
  const four = cadenceOf(x);
  return four.slice(1).map((s, i) => pcOf(four[i].rootMidi - s.rootMidi)).join() === '2,2,1';
}) && spelt.every((q) => /^[m-][M-][M-][M-]$/.test(q)) && spelt.filter((q) => q === 'mMMM').length > whole.length * 0.8,
`${spelt.filter((q) => q === 'mMMM').length} of ${whole.length} loops spell every chord in full`);
// Over a spelled chord the tune and the bass play in its scale: the G# over
// the E major, never the mode's G against it.
const slotOf = (p, step) => p.harmony.slots.filter((s) => s.startStep <= step % p.harmony.cycleSteps).pop();
check('a line over a spelled chord never plays the tone the spelling replaced', cadences.every(({ spec, p }) => {
  const mode = SCALES[spec.scale].steps;
  return ['melody', 'bass'].every((layer) => p.tracks[layer].every((e) => {
    const slot = slotOf(p, e.step);
    if (!slot.steps) return true;
    const pc = pcOf(e.midi - spec.root);
    return e.steps === slot.steps && (slot.steps.includes(pc) || !mode.includes(pc));
  }));
}));
check('graces and harmony over a spelled chord stay in its scale', cadences.every(({ spec, p }) => p.tracks.melody
  .filter((e) => e.steps)
  .every((e) => {
    const marked = { ...e, orn: 'turn', harm: 'third' };
    const h = harmonyOf(marked, spec);
    return [...gracesOf(marked, spec), ...(h == null ? [] : [h])].every((n) => e.steps.includes(pcOf(n - spec.root)));
  })));
check('no profile but cinder spells a chord', tides.every(({ p }) => p.harmony.slots.every((s) => !s.steps)));

// ---------------------------------------------------------------------
console.log('\nWayfare');

const wayfares = [];
for (let i = 0; wayfares.length < 300 && i < 20000; i++) {
  const spec = newSpec(randomSeed());
  const lead = Object.entries(spec.mix).sort((x, y) => y[1] - x[1])[0][0];
  if (lead === 'wayfare') wayfares.push({ spec, p: render(spec) });
}
check('wayfare loops walk in 4/4, some 6/8', wayfares.length === 300 && wayfares.every(({ spec }) => (spec.stepsPerBar === 16
  || spec.stepsPerBar === 12) && spec.bpm >= 104 && spec.bpm <= 138));
const wayDrums = wayfares.filter(({ p }) => p.tracks.drums.length).length;
check('drums play in about five wayfare loops in six', wayDrums > 300 * 0.75 && wayDrums < 300 * 0.95, `${wayDrums} of 300`);
// The chug: an eighth on every eighth, on the chord's bass note, short, and
// never a gap; under brushes, a swish on every eighth as well.
const chugs = wayfares.filter(({ p }) => p.meta.bassStyle === 'chug');
check('about half of wayfare\'s loops chug', chugs.length > 300 * 0.35 && chugs.length < 300 * 0.65, `${chugs.length} of 300`);
const everyEighth = (spec, steps) => {
  const at = new Set(steps);
  for (let s = 0; s < spec.bars * spec.stepsPerBar; s += 2) if (!at.has(s)) return false;
  return steps.every((s) => s % 2 === 0);
};
check('a chug is a short bass note on every eighth, on the chord\'s bass', chugs.every(({ spec, p }) => everyEighth(spec, p.tracks.bass.map((e) => e.step))
  && p.tracks.bass.every((e) => {
    const slot = p.harmony.slots.filter((sl) => sl.startStep <= e.step).pop();
    return e.dur === 0.5 && e.chug === true && pcOf(e.midi - slot.bassMidi) === 0;
  })));
check('only a chug note is marked as one', wayfares.every(({ p }) => p.meta.bassStyle === 'chug'
  || p.tracks.bass.every((e) => e.chug === undefined)));
const brushed = chugs.filter(({ p }) => p.meta.kit === 'brush');
check('under a chug the brushes swish every eighth, none open', brushed.length > 20 && brushed.every(({ spec, p }) => {
  const hats = p.tracks.drums.filter((e) => (e.inst === 'hat' || e.inst === 'ohat') && !e.roll);
  return hats.every((e) => e.inst === 'hat') && everyEighth(spec, hats.map((e) => e.step));
}), `${brushed.length} loops`);
// Its own grooves (queue items 8 and 9): the kick anchored on the first
// and third beats (in 6/8 the first of two), with only a pickup on the
// bar's last eighth besides, never a syncopated kick; a rim, snare or tap
// on the backbeat, as a ghost a sixteenth before a beat, or in a fill on
// the last beat, never a clap; the soft kick on tape and brush, the frame
// drum by hand. And bars that differ, on several instruments, with
// dynamics: the life v53 took out.
const wayDrummed = wayfares.filter(({ p }) => p.tracks.drums.length);
const KICKS = new Set(['softkick', 'frame']);
check('wayfare\'s kick is anchored on 1 and 3, with a pickup into the bar and nothing else', wayDrummed.length > 200 && wayDrummed.every(({ spec, p }) => {
  const every = spec.stepsPerBar === 12 ? 12 : 8;
  const kicks = p.tracks.drums.filter((e) => KICKS.has(e.inst));
  const anchors = kicks.filter((e) => e.step % every === 0);
  return anchors.length === spec.bars * (spec.stepsPerBar / every)
    && kicks.every((e) => e.step % every === 0 || e.step % spec.stepsPerBar === spec.stepsPerBar - 2);
}), `${wayDrummed.length} loops`);
check('its rim, snare or tap plays the backbeat, a ghost or a fill, and never a clap', wayDrummed.every(({ spec, p }) => {
  const spb = spec.stepsPerBar;
  const beat = spb === 12 ? 6 : 4;
  return p.tracks.drums.filter((e) => ['rim', 'snare', 'tap', 'clap'].includes(e.inst)).every((e) => {
    const s = e.step % spb;
    return e.inst !== 'clap' && (s % (2 * beat) === beat || (s + 1) % beat === 0 || s > spb - beat);
  });
}));
check('the soft kick on tape and brush, the frame drum on the hand kit, and never the lo-fi kick', wayDrummed.every(({ p }) => {
  const insts = new Set(p.tracks.drums.map((e) => e.inst));
  return !insts.has('kick') && (p.meta.kit === 'hand' ? !insts.has('softkick') : !insts.has('frame'));
}));
check('the eighths stay light, and open only on the bar\'s last eighth', wayDrummed.every(({ spec, p }) => {
  const cymbal = p.tracks.drums.filter((e) => ['hat', 'ohat', 'shaker', 'jingle', 'ojingle'].includes(e.inst));
  return cymbal.every((e) => e.vel < 0.6 && (!['ohat', 'ojingle'].includes(e.inst) || e.step % spec.stepsPerBar === spec.stepsPerBar - 2));
}));
{
  const variety = wayDrummed.map(({ spec, p }) => {
    const hits = p.tracks.drums.filter((e) => e.vel > 0);
    const bars = [];
    for (let b = 0; b < spec.bars; b++) {
      bars.push(hits.filter((e) => Math.floor(e.step / spec.stepsPerBar) === b).map((e) => `${e.step % spec.stepsPerBar}:${e.inst}`).sort().join());
    }
    let pairs = 0, differ = 0;
    for (let i = 0; i < bars.length; i++) for (let j = i + 1; j < bars.length; j++) { pairs++; if (bars[i] !== bars[j]) differ++; }
    const m = hits.reduce((a, e) => a + e.vel, 0) / hits.length;
    return {
      differ: pairs ? differ / pairs : 0,
      insts: new Set(hits.map((e) => e.inst)).size,
      sd: Math.sqrt(hits.reduce((a, e) => a + (e.vel - m) ** 2, 0) / hits.length),
    };
  });
  const avg = (k) => variety.reduce((a, v) => a + v[k], 0) / variety.length;
  check('wayfare\'s bars differ, on four or five instruments, with dynamics', avg('differ') > 0.88 && avg('insts') > 4.2 && avg('sd') > 0.165,
    `differ ${avg('differ').toFixed(2)}, instruments ${avg('insts').toFixed(1)}, sd ${avg('sd').toFixed(3)}`);
}
check('a chug leans on the beat and ghosts the eighths between', chugs.every(({ spec, p }) => {
  const beat = spec.stepsPerBar === 12 ? 6 : 4;
  return p.tracks.bass.every((e) => !e.vel || (e.step % beat === 0 ? e.vel >= 0.6 : e.vel <= 0.37));
}));
check('no loop wayfare does not lead plays its soft kick', [...tides, ...cinders].every(({ p }) => p.tracks.drums.every((e) => e.inst !== 'softkick')));
check('no loop wayfare does not lead chugs', [...tides, ...cinders].every(({ p }) => p.meta.bassStyle !== 'chug'));
// Its progressions, filtered per mode: I-bVII-IV-I and I-IV-V-IV, each
// played only where every chord of it is a plain triad.
const ways = wayfares.filter(({ spec, p }) => ['mixolydian', 'ionian', 'dorian'].includes(spec.scale) && p.harmony.slots.length >= 4);
check('wayfare plays I-bVII-IV-I and I-IV-V-IV where the mode has them', ways.length > 50 && ways.every(({ spec }) => {
  const roots = withTodaysChords(spec).harmony.slots.slice(0, 4).map((s) => pcOf(s.rootMidi - spec.root)).join();
  return roots === '0,10,5,0' || roots === '0,5,7,5';
}) && ways.filter(({ spec }) => spec.scale === 'ionian').every(({ spec }) => withTodaysChords(spec).harmony.slots.slice(0, 4)
  .map((s) => pcOf(s.rootMidi - spec.root)).join() === '0,5,7,5'), `${ways.length} loops`);

// ---------------------------------------------------------------------
console.log('\nChords stay in their slot, and 5/4 has its own');

// Every chord event, arpeggio notes included, ends by the end of the chord
// it starts under. Arpeggio notes used to keep the hit's whole length from
// their own later start and ran on into the next chord.
check('every chord event ends inside the chord it starts under', (() => {
  let arps = 0;
  for (let i = 0; i < 3000; i++) {
    const p = withTodaysChords(newSpec(i * 7919 + 13));
    const { slots } = p.harmony;
    for (const e of p.tracks.chords) {
      if (!e.vel) continue;
      let slot = slots[0];
      for (const sl of slots) if (sl.startStep <= e.step) slot = sl;
      if (e.step + e.dur > slot.startStep + slot.lengthSteps) return false;
      if (e.notes.length === 1) arps++;
    }
  }
  return arps > 0;
})());

// Held chords (queue item 57): a tied pad runs on through the slots that
// hold the same chord, and ends where that chord does.
check('a held chord ends where its chord does', (() => {
  let tied = 0;
  for (let i = 0; i < 3000; i++) {
    const p = render(newSpec(i * 7919 + 13));
    const { slots } = p.harmony;
    for (const e of p.tracks.chords) {
      if (!e.vel) continue;
      let k = 0;
      for (let j = 0; j < slots.length; j++) if (slots[j].startStep <= e.step) k = j;
      const same = slots[k].notes.join();
      let end = slots[k].startStep + slots[k].lengthSteps;
      for (let j = k + 1; j < slots.length && slots[j].notes.join() === same; j++) end = slots[j].startStep + slots[j].lengthSteps;
      if (e.step + e.dur > end) return false;
      if (e.step + e.dur > slots[k].startStep + slots[k].lengthSteps) tied++;
    }
  }
  return tied > 0;
})());

// 5/4 used the 4/4 table cut at the bar line, so nothing ever sounded on
// its fifth beat. Now some chords reach it, and none rings past the bar.
{
  let loops = 0;
  let bars = 0;
  let fifth = 0;
  let past = 0;
  for (let s = 1; loops < 200 && s < 400000; s++) {
    const spec = newSpec(s);
    if ((spec.stepsPerBar || 16) !== 20) continue;
    loops++;
    // Today's lengths: a held chord rings on through the bar on purpose.
    const p = withTodaysChords(spec);
    const total = spec.bars * 20;
    const covered = new Set();
    for (const e of p.tracks.chords) {
      if (!e.vel) continue;
      if ((e.step % 20) + e.dur > 20) past++;
      for (let t = e.step; t < e.step + e.dur; t++) if (t % 20 >= 16) covered.add(Math.floor((t % total) / 20));
    }
    bars += spec.bars;
    fifth += covered.size;
  }
  check('5/4 loops found', loops >= 50, `(${loops})`);
  check('5/4 chords reach the fifth beat', fifth / bars > 0.3, `(${(fifth / bars).toFixed(2)} of bars)`);
  check('no 5/4 chord rings past its bar', past === 0, `(${past})`);
}

console.log('\nComposition depth');

// A share of the loops of 8 bars or more develop: a statement, a departure
// over its own progression, a return. Everything else is the loop it was.
{
  let long = 0;
  let developing = 0;
  let wrongly = 0;
  let departs = 0;
  let answers = 0;
  let checked = 0;
  for (let i = 0; i < 3000; i++) {
    const spec = newSpec(i * 104729 + 7);
    const dev = developmentOf(spec);
    if (dev && (spec.bars < 8 || spec.cycles)) wrongly++;
    if (spec.bars >= 8) long++;
    if (!dev) continue;
    developing++;
    const p = render(spec);
    if (!p.meta.depth) { wrongly++; continue; }
    const spb = p.stepsPerBar;
    let at = 0;
    for (const sec of dev.sections) { sec.at = at; at += sec.bars; }
    const a = dev.sections.find((x) => x.role === 'A');
    const b = dev.sections.find((x) => x.role === 'B');
    // B's chords are not A's.
    const chordsIn = (sec) => p.tracks.chords.filter((e) => e.vel && e.step >= sec.at * spb && e.step < (sec.at + sec.bars) * spb)
      .map((e) => `${(e.step - sec.at * spb)}:${e.notes.join('.')}`).join();
    if (chordsIn(a) !== chordsIn(b)) departs++;
    // B's tune is not A's.
    const melIn = (sec, len) => p.tracks.melody.filter((e) => e.vel && e.step >= sec.at * spb && e.step < (sec.at + len) * spb)
      .map((e) => `${(e.step - sec.at * spb)}:${e.midi}`).join();
    if (p.tracks.melody.some((e) => e.vel)) {
      checked++;
      if (melIn(a, b.bars) !== melIn(b, b.bars)) answers++;
    }
  }
  const share = developing / long;
  check('the depth knob is where it was set', DEPTH === 1);
  check('about a third of the loops of 8 bars or more develop', share > 0.25 && share < 0.42, `(${share.toFixed(3)})`);
  check('only loops of 8 bars or more without their own layer cycles develop', wrongly === 0, `(${wrongly})`);
  check('a departure moves to other chords', departs / developing > 0.9, `(${departs} of ${developing})`);
  check('a departure plays another tune', answers / checked > 0.95, `(${answers} of ${checked})`);
}

// Queue item 17. Sixteen bars develop in four-bar phrases -- A, A varied,
// the departure, A varied again -- and the departure moves register,
// rhythm and harmony together. In every developing loop B's tune is heard.
{
  const master = new Rng(17);
  let sixteen = 0;
  let cadences = 0;
  let generic = 0;
  let shaped = 0;
  let moves = 0;
  let varied = 0;
  let returns = 0;
  let tuned = 0;
  let heard = 0;
  let withTune = 0;
  const long = { 24: [0, 0], 32: [0, 0] };
  for (let i = 0; i < 12000; i++) {
    const spec = newSpec(master.seed32());
    if (long[spec.bars]) {
      long[spec.bars][0]++;
      if (developmentOf(spec)) long[spec.bars][1]++;
    }
    const dev = developmentOf(spec);
    if (!dev) continue;
    const p = render(spec);
    const spb = p.stepsPerBar;
    let at = 0;
    for (const sec of dev.sections) { sec.at = at; at += sec.bars; }
    const role = (k) => dev.sections.find((x) => x.role === k);
    const melIn = (sec) => p.tracks.melody.filter((e) => e.vel && e.step >= sec.at * spb && e.step < (sec.at + sec.bars) * spb)
      .map((e) => `${(e.step - sec.at * spb)}:${e.midi}`).join();
    const written = p.tracks.melody.some((e) => e.vel);
    if (written) {
      withTune++;
      if (melIn(role('B'))) heard++;
    }
    if (spec.bars !== 16) continue;
    sixteen++;
    // The progression as written: today's chord lengths (queue item 57).
    const today = withTodaysChords(spec);
    const chordsIn = (sec) => today.harmony.slots.filter((sl) => sl.startStep >= sec.at * spb && sl.startStep < (sec.at + sec.bars) * spb)
      .map((sl) => sl.notes.join('.')).join();
    const [ca, cv, cr] = ['A', 'V', 'R'].map((k) => chordsIn(role(k)));
    if (!characterOf(spec).progressions) {
      generic++;
      if (cv !== ca && cr !== ca && cr !== cv) cadences++;
    }
    if (dev.sections.map((x) => `${x.role}${x.bars}`).join() === 'A4,V4,B4,R4') shaped++;
    const d = p.meta.depth;
    if (written && melIn(role('A'))) {
      tuned++;
      if ((d.answer === 'rhythm' || d.answer === 'both') && d.shift !== 0) moves++;
      if (melIn(role('V')) !== melIn(role('A'))) varied++;
      if (melIn(role('R')) !== melIn(role('A'))) returns++;
    }
  }
  check('sixteen bars develop in four four-bar phrases', sixteen > 200 && shaped === sixteen, `(${shaped} of ${sixteen})`);
  check('the departure moves rhythm and register', moves === tuned, `(${moves} of ${tuned})`);
  // A profile's own progressions are idioms and end as written.
  check('A, its variation and the return each cadence somewhere of their own', cadences / generic > 0.97, `(${cadences} of ${generic})`);
  check('the varied statement is not the statement again', varied / tuned > 0.95, `(${varied} of ${tuned})`);
  check('nor is the return', returns / tuned > 0.95, `(${returns} of ${tuned})`);
  check('the departure\'s tune is heard', heard / withTune > 0.97, `(${heard} of ${withTune})`);
  const share24 = long[24][1] / long[24][0];
  check('about two thirds of the 24-bar loops develop', share24 > 0.6 && share24 < 0.74, `(${share24.toFixed(3)})`);
  check('32-bar loops develop more than before', long[32][1] / long[32][0] > 0.14, `(${(long[32][1] / long[32][0]).toFixed(3)})`);
}

console.log('\nMoods steer the mode');
{
  // Queue item 16. The mood picks the mode a loop is made in: loops whose
  // largest share is a bright mood come out mostly in bright modes, the
  // reflective ones mostly in the middle of the axis, and the middle moods
  // as they were. Before, every mood drew bright modes 38-60% of the time.
  const master = new Rng(16);
  const tally = {};
  for (let i = 0; i < 4000; i++) {
    const spec = newSpec(master.seed32());
    const [mood] = Object.entries(spec.feelMix).sort((a, b) => b[1] - a[1])[0];
    const b = SCALE_BRIGHTNESS[spec.scale];
    const t = tally[mood] ||= { n: 0, bright: 0, darkest: 0 };
    t.n++;
    if (b >= 0.6) t.bright++;
    if (b < 0.24) t.darkest++;
  }
  const share = (moods, key) => {
    const n = moods.reduce((a, m) => a + tally[m].n, 0);
    return moods.reduce((a, m) => a + tally[m][key], 0) / n;
  };
  const glad = share(['joyful', 'happy', 'enthusiastic'], 'bright');
  const middle = share(['soothing', 'peaceful', 'comforting'], 'bright');
  const inward = share(['reflective'], 'bright');
  check('the bright moods are mostly in bright modes', glad > 0.7, `(${glad.toFixed(3)})`);
  check('the middle moods are left where they were', middle > 0.4 && middle < 0.58, `(${middle.toFixed(3)})`);
  check('reflective leans inward', inward < 0.25, `(${inward.toFixed(3)})`);
  check('reflective is rarely in the darkest modes', share(['reflective'], 'darkest') < 0.06,
    `(${share(['reflective'], 'darkest').toFixed(3)})`);

  // The vocabulary is data: every mood has its own share-code number, the
  // numbers are the ones codes have always used, and a code still reads.
  const codes = Object.values(MOODS).map((m) => m.code);
  check('every mood has its own code', new Set(codes).size === codes.length);
  const WRITTEN = ['joyful', 'happy', 'enthusiastic', 'refreshing', 'soothing', 'peaceful', 'comforting', 'reflective'];
  check('mood codes are the ones already written down', WRITTEN.every((k, i) => codeForMood(k) === i && moodForCode(i) === k));
  check('an unknown mood code reads as peaceful', moodForCode(200) === 'peaceful');
  check('every mood has a word', Object.keys(MOODS).every((k) => typeof moodWord(k) === 'string' && moodWord(k).length));
  const spec = newSpec(12345);
  check('a share code keeps its moods', JSON.stringify(decodeSong(encodeSong(spec)).feelMix) === JSON.stringify(spec.feelMix));
  const old = (lift, energy) => {
    const high = lift > 0.66;
    const mid = lift > 0.38;
    if (energy > 0.68) return high ? 'enthusiastic' : mid ? 'refreshing' : 'restless';
    if (energy > 0.36) return high ? 'happy' : mid ? 'comforting' : 'reflective';
    return high ? 'joyful' : mid ? 'peaceful' : 'reflective';
  };
  let same = true;
  for (let l = 0; l <= 1.0001; l += 0.02) for (let e = 0; e <= 1.0001; e += 0.02) if (pointWord({ lift: l, energy: e }) !== old(l, e)) same = false;
  check('an old save shows the word it always did', same);
}

console.log('\nNames');

const names = new Set();
for (let i = 0; i < 3000; i++) names.add(seedName(randomSeed()));
check('names are reasonably distinct', names.size > 2200, `${names.size} of 3000 unique`);
check('names stay short enough for one line', [...names].every((n) => n.length <= 14));
check('the same seed always gets the same name', seedName(42) === seedName(42));

// ---------------------------------------------------------------------
console.log('\nMIDI export');

function parseMidi(bytes) {
  const dv = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  if (String.fromCharCode(...bytes.slice(0, 4)) !== 'MThd') throw new Error('missing header');
  if (dv.getUint32(4) !== 6) throw new Error('bad header length');
  const trackCount = dv.getUint16(10);
  let pos = 14;
  const open = new Map();

  for (let t = 0; t < trackCount; t++) {
    if (String.fromCharCode(...bytes.slice(pos, pos + 4)) !== 'MTrk') throw new Error(`track ${t} is not MTrk`);
    const end = pos + 8 + dv.getUint32(pos + 4);
    if (end > bytes.length) throw new Error('track runs past the end of the file');
    let q = pos + 8;
    let running = null;
    let ended = false;
    const varlen = () => {
      let v = 0;
      for (let g = 0; g < 5; g++) {
        const byte = bytes[q++];
        v = (v << 7) | (byte & 0x7f);
        if (!(byte & 0x80)) return v;
      }
      throw new Error('runaway variable-length quantity');
    };

    while (q < end) {
      varlen();
      let status = bytes[q];
      if (status & 0x80) { q++; running = status; } else { status = running; }
      if (status == null) throw new Error('running status with no status byte before it');
      if (status === 0xff) {
        const type = bytes[q++];
        // Read the length into a variable first. `q += varlen()` would
        // capture q before varlen advances it, throwing the read pointer
        // back to where the length field started.
        const dataLength = varlen();
        q += dataLength;
        if (type === 0x2f) ended = true;
      } else {
        const kind = status & 0xf0;
        if (kind === 0x90 || kind === 0x80) {
          const note = bytes[q];
          const vel = bytes[q + 1];
          q += 2;
          if (note > 127 || vel > 127) throw new Error('data byte above 127');
          const key = `${status & 0x0f}:${note}`;
          const n = open.get(key) || 0;
          open.set(key, kind === 0x90 && vel > 0 ? n + 1 : Math.max(0, n - 1));
        } else if (kind === 0xc0 || kind === 0xd0) q += 1;
        else q += 2;
      }
    }
    if (q !== end) throw new Error('track length does not match its contents');
    if (!ended) throw new Error('track has no end marker');
    pos = end;
  }
  if (pos !== bytes.length) throw new Error('bytes left over after the last track');
  const stuck = [...open].filter(([, v]) => v !== 0);
  if (stuck.length) throw new Error(`notes never released: ${JSON.stringify(stuck.slice(0, 2))}`);
}

let midiFailures = 0;
let lastMidiError = '';
for (let i = 0; i < 500; i++) {
  try {
    parseMidi(patternToMidi(render(newSpec(randomSeed())), { repeats: 2 }).bytes);
  } catch (err) {
    midiFailures++;
    lastMidiError = err.message;
  }
}
check('every exported file parses as valid MIDI', midiFailures === 0, lastMidiError);

// A layer on its own cycle has to repeat on that cycle in the export too,
// or the file does not match what was playing.
let polySpec = null;
for (let s = 1; s < 80000 && !polySpec; s++) {
  const cand = newSpec(s);
  if (cand.cycles && cand.cycles.melody && cand.cycles.melody < cand.bars * cand.stepsPerBar) {
    if (render(cand).tracks.melody.some((e) => e.vel > 0)) polySpec = cand;
  }
}
check('a short-cycle layer repeats across the exported file', (() => {
  if (!polySpec) return false;
  const pat = render(polySpec);
  const bytes = patternToMidi(pat, { repeats: 1 }).bytes;
  parseMidi(bytes);
  const cycleSteps = polySpec.cycles.melody;
  const perCycle = pat.tracks.melody.filter((e) => e.vel > 0).length;
  const repeatsInLoop = Math.floor((polySpec.bars * polySpec.stepsPerBar) / cycleSteps);
  // Count melody note-ons in the exported file (track 4, channel 2).
  let ons = 0;
  for (let i = 0; i < bytes.length - 2; i++) {
    if (bytes[i] === 0x92 && bytes[i + 2] > 0) ons++;
  }
  return repeatsInLoop > 1 && ons >= perCycle * repeatsInLoop * 0.8;
})(), polySpec ? '' : 'no polymetric example found');

// Roll bursts must not collapse onto the same tick and same pitch.
check('stutter rolls keep their sub-step timing', (() => {
  for (let s = 1; s < 120000; s++) {
    const cand = newSpec(s);
    const pat = render(cand);
    const rolls = pat.tracks.drums.filter((e) => e.roll && e.vel > 0);
    if (rolls.length < 4) continue;
    const bytes = patternToMidi(pat, { repeats: 1 }).bytes;
    parseMidi(bytes); // must still be structurally valid
    const micros = new Set(rolls.map((e) => e.micro));
    return micros.size > 1; // the burst is spread, not stacked
  }
  return false;
})());


// ---------------------------------------------------------------------
console.log('\nVibes (queue item 38a)');

// The recipes are data the brain edits by hand, so every field and every
// name is checked here: a typo would otherwise lean nothing, silently.
{
  const known = { profiles: new Set(Object.keys(CHARACTERS)), modes: new Set(Object.keys(SCALES)) };
  const poolOf = { melody: 'melodyVoices', chords: 'chordVoices', bass: 'bassVoices', bassStyle: 'bassStyles', texture: 'textures' };
  for (const [field, pool] of Object.entries(poolOf)) {
    known[field] = new Set();
    for (const c of Object.values(CHARACTERS)) for (const [n] of c[pool] || []) known[field].add(n);
  }
  const TARGETS = { feel: ['lift', 'energy', 'warmth'], tone: ['warmth', 'space', 'wobble'] };
  const SCALARS = ['tempo', 'swing'];
  const NUMBER_LEANS = ['six8', 'long', 'drums', 'hats', 'rests', 'bigChords'];
  const bad = [];
  const unit = (x) => typeof x === 'number' && x >= 0 && x <= 1;
  const lean = (x) => typeof x === 'number' && x > 0 && Number.isFinite(x);
  for (const [id, rec] of Object.entries(VIBES)) {
    if (!rec || typeof rec !== 'object') { bad.push(`${id}: not a recipe`); continue; }
    for (const [field, value] of Object.entries(rec)) {
      if (TARGETS[field]) {
        for (const [k, t] of Object.entries(value)) {
          if (!TARGETS[field].includes(k)) bad.push(`${id}.${field}.${k}: unknown`);
          else if (!unit(t)) bad.push(`${id}.${field}.${k}: ${t} not in 0..1`);
        }
      } else if (SCALARS.includes(field)) {
        if (!unit(value)) bad.push(`${id}.${field}: ${value} not in 0..1`);
      } else if (NUMBER_LEANS.includes(field)) {
        if (!lean(value)) bad.push(`${id}.${field}: ${value} not a positive number`);
      } else if (known[field]) {
        for (const [name, m] of Object.entries(value)) {
          if (!known[field].has(name)) bad.push(`${id}.${field}.${name}: unknown name`);
          else if (!lean(m)) bad.push(`${id}.${field}.${name}: ${m} not a positive number`);
        }
      } else {
        bad.push(`${id}.${field}: unknown field`);
      }
    }
  }
  check('every vibe recipe names only known fields, profiles, modes, voices, textures and bass styles', bad.length === 0, bad.slice(0, 8).join('; '));
  const ids = new Set(VIBE_IDS);
  check('VIBE_IDS holds every vibe once, and nothing else',
    ids.size === VIBE_IDS.length && VIBE_IDS.every((id) => VIBES[id]) && Object.keys(VIBES).every((id) => ids.has(id)),
    `${VIBE_IDS.length} ids, ${Object.keys(VIBES).length} recipes`);
  check('71 vibes, every pool-3 word', VIBE_IDS.length === 71);
}

// No vibes is today's loop, draw for draw: the spec, the render and the code.
{
  const master = new Rng(0x56494245);
  let differ = 0;
  let first = '';
  for (let i = 0; i < 1000; i++) {
    const seed = master.seed32();
    const plain = newSpec(seed);
    const a = JSON.stringify(plain);
    for (const sel of [[], null, undefined, ['not-a-vibe']]) {
      if (JSON.stringify(newSpec(seed, sel)) !== a) { differ++; if (!first) first = `${seed} ${JSON.stringify(sel)}`; }
    }
    if (i < 200 && JSON.stringify(render(newSpec(seed, []))) !== JSON.stringify(render(plain))) differ++;
    if (plain.vibes || plain.vibeStrength != null) differ++;
  }
  check('with no vibes the spec is exactly today\'s (1,000 seeds)', differ === 0, first);
  // /rank's first loop of Mikey's 181 (v85), so every ranking still names its loop.
  check('a loop without vibes keeps its code string (format 3)',
    encodeSong(newSpec(19000)) === 'DL1-0C07C-1R60G-60789-DFZDM-QV19C-NRNCG-FKYW6-B9P0G-ENS6B-KR001-53G0G-8YC3G-R0G3Y-W10HC-XZDMW-6WC00-00000-04CK0');
}

// The draw: up to five, weights sharing 100% with each at least 5%, a
// strength in 0.5..0.9; from its own stream, deterministic, tap order free.
{
  const problems2 = [];
  const fifthWeights = new Set();
  const pickedSets = new Set();
  for (let s = 1; s <= 400; s++) {
    for (const n of [1, 3, 5, 10]) {
      const sel = VIBE_IDS.slice(s % 50, (s % 50) + n);
      const spec = newSpec(s, sel);
      const ws = Object.values(spec.vibes);
      const sum = ws.reduce((a, b) => a + b, 0);
      if (ws.length !== Math.min(5, n)) problems2.push(`${s}/${n}: ${ws.length} vibes`);
      if (n === 1 && ws[0] !== 1) problems2.push(`${s}: lone weight ${ws[0]}`);
      if (Math.abs(sum - 1) > 0.01) problems2.push(`${s}/${n}: sum ${sum}`);
      if (ws.some((w) => w < 0.05)) problems2.push(`${s}/${n}: below 5%`);
      if (!(spec.vibeStrength >= 0.5 && spec.vibeStrength <= 0.9)) problems2.push(`${s}/${n}: strength ${spec.vibeStrength}`);
      if (Object.keys(spec.vibes).some((k) => !sel.includes(k))) problems2.push(`${s}/${n}: a vibe not selected`);
      if (ws.some((w) => Math.round(w * 255) !== w * 255)) problems2.push(`${s}/${n}: off the q8 grid`);
      if (n === 5) fifthWeights.add(spec.vibes[sel[4]]);
      if (n === 10) pickedSets.add(Object.keys(spec.vibes).join());
      if (n === 3 && JSON.stringify(newSpec(s, sel.slice().reverse())) !== JSON.stringify(spec)) problems2.push(`${s}: tap order matters`);
    }
  }
  check('vibe draws: up to five, weights share 100% (each at least 5%), strength 0.5-0.9', problems2.length === 0, problems2.slice(0, 5).join('; '));
  check('with more than five selected, each loop picks a new five', pickedSets.size > 300, `${pickedSets.size} distinct of 400`);
  check('a vibe\'s weight moves from loop to loop', Math.min(...fifthWeights) <= 0.1 && Math.max(...fifthWeights) >= 0.5,
    `${Math.min(...fifthWeights)}..${Math.max(...fifthWeights)}`);
  const sel = ['cozy', 'joyful', 'misty'];
  check('the same seed and selection give the same loop', JSON.stringify(newSpec(77, sel)) === JSON.stringify(newSpec(77, sel)));
  // The loop's own stream does not move: the layer seeds are its last draws.
  check('vibes leave the loop\'s own draws tied to its seed',
    JSON.stringify(newSpec(77, sel).layerSeeds) === JSON.stringify(newSpec(77).layerSeeds));
}

// Share codes: a vibe loop round-trips to an identical render; old formats
// read with no vibes; an album carries them.
{
  let mismatched = 0;
  let first = '';
  for (let s = 1; s <= 150; s++) {
    const sel = VIBE_IDS.filter((_, i) => (i * 7 + s) % 11 === 0);
    const spec = newSpec(s * 101, sel);
    const code = encodeSong(spec);
    const back = decodeSong(code);
    // Every note, as a plain loop's code already does (swing and tone go
    // through the code's 1/255 grid, which the notes never see); and the
    // code is a fixed point, so it renders the same wherever it is opened.
    if (JSON.stringify(back.vibes) !== JSON.stringify(spec.vibes) || back.vibeStrength !== spec.vibeStrength
      || JSON.stringify(render(back).tracks) !== JSON.stringify(render(spec).tracks)
      || encodeSong(back) !== code) {
      mismatched++;
      if (!first) first = code;
    }
  }
  check('a vibe loop\'s code round-trips to an identical render (150 loops)', mismatched === 0, first);
  const vibeCode = encodeSong(newSpec(5, ['cozy']));
  check('a vibe loop is written as format 4, a plain one as format 3',
    vibeCode.slice(4, 6) === '0G' && encodeSong(newSpec(5)).slice(4, 6) === '0C');
  check('a format-3 code reads with no vibes', decodeSong(encodeSong(newSpec(5))).vibes === undefined);
  const album = decodeAlbum(encodeAlbum('Mixed', [newSpec(8), newSpec(9, ['spicy', 'merry'])]));
  check('an album carries its tracks\' vibes as they are',
    album.specs[0].vibes === undefined && JSON.stringify(album.specs[1].vibes) === JSON.stringify(newSpec(9, ['spicy', 'merry']).vibes));
}

// The leans and targets reach the loop, each the way its recipe says.
{
  const N = 600;
  const corpus = (sel) => Array.from({ length: N }, (_, i) => newSpec(1000 + i, sel));
  const share = (specs, f) => specs.filter(f).length / specs.length;
  const mean = (specs, f) => specs.reduce((a, s) => a + f(s), 0) / specs.length;
  const plain = corpus(null);
  const energetic = corpus(['energetic']);
  const still = corpus(['still']);
  check('energetic loops are quicker, still loops slower', mean(energetic, (s) => s.bpm) > mean(plain, (s) => s.bpm) + 5
    && mean(still, (s) => s.bpm) < mean(plain, (s) => s.bpm) - 5,
    `${mean(still, (s) => s.bpm).toFixed(1)} / ${mean(plain, (s) => s.bpm).toFixed(1)} / ${mean(energetic, (s) => s.bpm).toFixed(1)}`);
  check('skipping loops are 6/8 more often', share(corpus(['skipping']), (s) => s.stepsPerBar === 12) > share(plain, (s) => s.stepsPerBar === 12) + 0.1);
  check('bright loops sit in brighter modes', mean(corpus(['bright']), (s) => SCALE_BRIGHTNESS[s.scale]) > mean(plain, (s) => SCALE_BRIGHTNESS[s.scale]) + 0.05);
  check('a vibe leans the render-time pools through characterOf', (() => {
    const sp = newSpec(42, ['chiming']);
    const c = characterOf(sp);
    const bare = characterOf({ ...sp, vibes: undefined });
    const w = (pool, k) => (pool.find(([n]) => n === k) || [0, 0])[1];
    return w(bare.textures, 'chime') > 0 && w(c.textures, 'chime') > w(bare.textures, 'chime')
      && w(c.melodyVoices, 'bell') >= w(bare.melodyVoices, 'bell');
  })());
  check('characterOf without vibes is the plain blend, untouched', (() => {
    const sp = newSpec(42);
    return JSON.stringify(characterOf(sp)) === JSON.stringify(characterOf({ ...sp, vibes: {} }));
  })());
  const drumsOn = (specs) => share(specs.slice(0, 300), (s) => render(s).tracks.drums.some((e) => e.vel > 0));
  check('hushed loops have drums less often than plain ones', drumsOn(corpus(['hushed'])) < drumsOn(plain) - 0.1);
}


// ---------------------------------------------------------------------
console.log('\nBorrowed bright modes (queue item 36)');
{
  const BORROWED = { cinder: ['ionian', 'mixolydian'], shatter: ['lydian', 'mixolydian'] };
  let notGlad = 0;
  let wrong = 0;
  const glad = { cinder: 0, shatter: 0 };
  const borrowed = { cinder: 0, shatter: 0 };
  let cinderIonian = null;
  let shatterLydian = null;
  for (let s = 1; s < 60000; s++) {
    const spec = newSpec(s);
    for (const p of ['cinder', 'shatter']) {
      if ((spec.mix[p] || 0) < 0.999) continue;
      const isBorrowed = BORROWED[p].includes(spec.scale);
      if (spec.feel.lift <= 0.66) {
        notGlad++;
        if (isBorrowed) wrong++;
      } else {
        glad[p]++;
        if (isBorrowed) borrowed[p]++;
      }
      if (p === 'cinder' && spec.scale === 'ionian' && !cinderIonian) cinderIonian = spec;
      if (p === 'shatter' && spec.scale === 'lydian' && !shatterLydian) shatterLydian = spec;
    }
  }
  check('a cinder or shatter loop that is not glad never borrows a bright mode', notGlad > 200 && wrong === 0, `${wrong} of ${notGlad}`);
  check('a glad cinder loop sometimes does', borrowed.cinder > 0, `${borrowed.cinder} of ${glad.cinder}`);
  check('a glad shatter loop sometimes does', borrowed.shatter > 0, `${borrowed.shatter} of ${glad.shatter}`);
  for (const [label, spec] of [['a cinder loop in ionian', cinderIonian], ['a shatter loop in lydian', shatterLydian]]) {
    const back = spec && decodeSong(encodeSong(spec));
    check(`${label} encodes and decodes to the same notes`, !!spec && back.scale === spec.scale
      && JSON.stringify(render(back).tracks) === JSON.stringify(render(spec).tracks));
  }
}

// ---------------------------------------------------------------------
console.log('\nThe per-loop choices (queue item 54)');
{
  // Every loop carries its choices; while none are listed, every one is today.
  let empty = true;
  for (let i = 0; i < 200; i++) {
    const p = render(newSpec(9000 + i));
    if (!p.choices || Object.keys(p.choices).length !== CHOICES.length || changesIn(p.choices) !== 0) empty = false;
  }
  check('every loop carries its choices, all today while the list is empty', CHOICES.length > 0 || empty);
  // "Let the loop wander" never redraws them: a pass keeps the loop's own.
  const loop = render(newSpec(31337));
  check('wandering keeps the loop\'s choices', drift(loop, new Rng(3), 2.4).choices === loop.choices);

  // A list of five, every one eligible, to check the draw itself.
  const always = () => true;
  const list = ['a', 'b', 'c', 'd', 'e'].map((id, i) => ({
    id, options: [['today', 1], ['one', 2], ['two', 1]], weight: 1 + i * 0.25, eligible: always,
    vibes: id === 'a' ? { hypnotic: 3, bouncy: 0.2 } : null,
  }));
  list.push({ id: 'never', options: [['today', 1], ['on', 1]], eligible: () => false });
  const N = 10000;
  const counts = new Map();
  const reached = new Set();
  let never = 0;
  for (let i = 0; i < N; i++) {
    const c = drawChoices(i * 7919 + 13, {}, null, list);
    const k = changesIn(c, list);
    counts.set(k, (counts.get(k) || 0) + 1);
    for (const x of list) if (c[x.id] !== 'today') reached.add(`${x.id}:${c[x.id]}`);
    if (c.never !== 'today') never++;
  }
  const total = CHANGE_COUNTS.reduce((a, [, w]) => a + w, 0);
  const off = CHANGE_COUNTS.map(([k, w]) => Math.abs((counts.get(k) || 0) / N - w / total) * 100);
  check('loops get none, one, two or three changes within two points of the weights', off.every((d) => d <= 2),
    off.map((d) => d.toFixed(2)).join(' '));
  check('every choice and option is reachable', list.filter((x) => x.id !== 'never')
    .every((x) => x.options.slice(1).every(([o]) => reached.has(`${x.id}:${o}`))), [...reached].join(' '));
  check('a choice a loop is not eligible for is never drawn', never === 0);
  // A loop eligible for fewer changes than it drew gets all it can.
  const two = list.slice(0, 2);
  let most = 0;
  for (let i = 0; i < 2000; i++) most = Math.max(most, changesIn(drawChoices(i, {}, null, two), two));
  check('a loop eligible for two gets two at most', most === 2);
  // The same seed, the same choices; the vibes lean them, never to zero.
  check('the same seed draws the same choices', JSON.stringify(drawChoices(77, {}, null, list)) === JSON.stringify(drawChoices(77, {}, null, list)));
  const share = (vibes) => {
    let a = 0;
    for (let i = 0; i < 4000; i++) if (drawChoices(i, {}, vibes, list).a !== 'today') a++;
    return a / 4000;
  };
  const plain = share(null);
  const toward = share({ vibes: { hypnotic: 1 }, strength: 0.9 });
  const away = share({ vibes: { bouncy: 1 }, strength: 0.9 });
  check('a vibe the choice leans toward draws it more, and one it leans away from less, never to none',
    toward > plain && away < plain && away > 0, `${away.toFixed(3)} ${plain.toFixed(3)} ${toward.toFixed(3)}`);
}

// ---------------------------------------------------------------------
console.log('\nThe arc\'s two shapes (queue item 63b)');
{
  // Item 63's draw, for a loop with no vibes: the period, then a bright start.
  const was = (seed) => {
    const r = new Rng(((seed ^ 0x0a4c) >>> 0) || 1);
    return { period: r.range(120, 300), from: r.range(0.4, 0.6) };
  };
  const N = 6000;
  let deep = 0, same = true, inRange = true;
  for (let i = 0; i < N; i++) {
    const seed = i * 7919 + 5;
    const a = arcOf(seed);
    const w = was(seed);
    if (a.shape === 'deep') {
      deep++;
      if (a.period !== w.period || a.from !== w.from) same = false;
    } else if (!(a.from >= a.dip / 2 && a.from <= 1 - a.dip / 2 && a.floor >= 800 && a.floor <= 1500)) inRange = false;
    if (!(a.period >= 120 && a.period <= 300)) inRange = false;
  }
  check('about one loop in three draws the deep swing', Math.abs(deep / N - 1 / 3) < 0.02, (deep / N).toFixed(3));
  check('a deep loop with no vibes keeps item 63\'s period and start, to the bit', same);
  check('a dip starts in the open part, its floor 800 Hz to 1.5 kHz, periods 2-5 minutes', inRange);
  // The vibes: calm lean deep and long; bright lean dip and short.
  const shareOf = (vibes) => {
    let d = 0, lo = Infinity, hi = 0;
    for (let i = 0; i < 3000; i++) {
      const a = arcOf(i * 104729 + 1, vibes);
      if (a.shape === 'deep') d++;
      lo = Math.min(lo, a.period);
      hi = Math.max(hi, a.period);
    }
    return { deep: d / 3000, lo, hi };
  };
  const calm = shareOf({ vibes: { foggy: 1 }, strength: 0.7 });
  const bright = shareOf({ vibes: { sunny: 0.6, peaceful: 0.4 }, strength: 0.7 });
  const neither = shareOf({ vibes: { peaceful: 1 }, strength: 0.7 });
  check('calm vibes lean deep, over 3-5 minutes', calm.deep > 0.4 && calm.lo >= 180 && calm.hi <= 300, JSON.stringify(calm));
  check('bright vibes lean dip, over 1.5-2.5 minutes', bright.deep < 0.3 && bright.deep > 0 && bright.lo >= 90 && bright.hi <= 150, JSON.stringify(bright));
  check('other vibes keep the shares and 2-5 minutes', Math.abs(neither.deep - 1 / 3) < 0.03 && neither.lo >= 120 && neither.hi <= 300, JSON.stringify(neither));
  // The arc itself, on against today: calm vibes draw it more, lively less.
  const arcShare = (vibes) => {
    let n = 0;
    for (let i = 0; i < 4000; i++) if (drawChoices(i, {}, vibes).arc === 'on') n++;
    return n / 4000;
  };
  const plain = arcShare(null);
  const still = arcShare({ vibes: { still: 1 }, strength: 0.9 });
  const zany = arcShare({ vibes: { zany: 1 }, strength: 0.9 });
  check('calm vibes draw the arc more, lively ones less, never to none', still > plain && zany < plain && zany > 0,
    `${zany.toFixed(3)} ${plain.toFixed(3)} ${still.toFixed(3)}`);
  // A pin fixes its own setting and moves nothing else.
  const free = arcOf(4242);
  const pinned = arcOf(4242, null, { shape: free.shape === 'deep' ? 'dip' : 'deep', period: 200 });
  check('a pin moves nothing else', pinned.period === 200 && pinned.floor === free.floor && pinned.shape !== free.shape);
  check('every setting is named, with its range', ['weight', 'shapes', 'period', 'from', 'floor', 'dip'].every((k) => k in ARC));
}

// ---------------------------------------------------------------------
console.log('\ntunedKick (queue item 67)');
{
  // The key's note between G1 and D2, D# to F# on their fifth; the falls in
  // their ranges; today's loudness by ear; a long boom's tail under its punch.
  let notes = true, falls = true, lifts = true, tails = true;
  for (let root = 36; root < 48; root++) {
    for (let i = 0; i < 40; i++) {
      const k = kickOf(i * 7919 + 3, root);
      const pc = root % 12;
      const use = pc >= 3 && pc <= 6 ? (pc + 7) % 12 : pc;
      if (!(k.midi >= 31 && k.midi <= 38 && k.midi % 12 === use)) notes = false;
      if (!(k.short.dbps >= 180 && k.short.dbps <= 260 && k.long.dbps >= 45 && k.long.dbps <= 80)) falls = false;
      for (const f of [k.short.lift, k.short.softLift, k.long.lift, k.long.softLift]) if (!(f > 0.9 && f < 1.1)) lifts = false;
      const db = 20 * Math.log10(k.long.tail);
      if (!(k.short.tail === 1 && db < -5 && db > -20 && k.long.softTail < 1)) tails = false;
    }
  }
  check('on the key\'s note, G1 to D2, D# to F# on their fifth', notes);
  check('short falls 180-260 dB a second, long 45-80', falls);
  check('each at today\'s loudness by ear (lift 0.9 to 1.1)', lifts);
  check('a long boom\'s tail under its punch, none on a short kick', tails);
  check('the same seed, the same kick', JSON.stringify(kickOf(99, 41)) === JSON.stringify(kickOf(99, 41)));
  const pinned = kickOf(99, 41, { long: 60 });
  check('a pin moves nothing else', pinned.long.dbps === 60 && pinned.short.dbps === kickOf(99, 41).short.dbps && pinned.midi === kickOf(99, 41).midi);
  // A wandering loop's swaps: never on the first pass, about one pass in
  // four after it, to either of the other two, the same every time.
  let swaps = 0, first = true, steady = true;
  const to = { today: 0, short: 0, long: 0 };
  for (let i = 0; i < 4000; i++) {
    const seed = i * 104729 + 11;
    if (kickKindOf(seed, 'long', 0) !== 'long') first = false;
    const k = kickKindOf(seed, 'long', 1 + (i % 50));
    if (k !== kickKindOf(seed, 'long', 1 + (i % 50))) steady = false;
    if (k !== 'long') { swaps++; to[k]++; }
  }
  check('a swap is never on the first pass, and the same pass swaps the same way', first && steady);
  check('about one pass in four swaps, half to each of the others', Math.abs(swaps / 4000 - KICK.swap) < 0.03 && to.long === 0
    && Math.abs(to.today / swaps - 0.5) < 0.05, `${swaps} ${JSON.stringify(to)}`);
  // The choice: only on a loop with a kick; booming leans long.
  const list = CHOICES.filter((c) => c.id === 'tunedKick');
  const kicks = { tracks: { drums: [{ inst: 'kick', vel: 0.8, step: 0 }] } };
  const none = { tracks: { drums: [{ inst: 'hat', vel: 0.5, step: 0 }] } };
  check('only on a loop with a kick', list[0].eligible(kicks) && !list[0].eligible(none));
  const longShare = (vibes) => {
    let l = 0, n = 0;
    for (let i = 0; i < 4000; i++) {
      const c = drawChoices(i, kicks, vibes, list).tunedKick;
      if (c !== 'today') { n++; if (c === 'long') l++; }
    }
    return l / n;
  };
  const plainLong = longShare(null);
  const booming = longShare({ vibes: { booming: 1 }, strength: 0.9 });
  const hushed = longShare({ vibes: { hushed: 1 }, strength: 0.9 });
  check('short and long half and half; booming leans long, hushed away', Math.abs(plainLong - 0.5) < 0.03 && booming > 0.6 && hushed < 0.4 && hushed > 0,
    `${hushed.toFixed(3)} ${plainLong.toFixed(3)} ${booming.toFixed(3)}`);
  check('every setting is named, with its range', ['options', 'optionVibes', 'note', 'short', 'long', 'loud', 'tailLoud', 'band', 'swap'].every((k) => k in KICK));
}

// ---------------------------------------------------------------------
console.log('\nsubDuck (queue item 68)');
{
  // The sketch's draws: whether, then the depth and the recovery, on its
  // stream; the depth 5-9 dB, the recovery 0.15-0.25 s and never over 0.8 of
  // a beat.
  let same = true, inRange = true;
  for (let i = 0; i < 2000; i++) {
    const seed = i * 7919 + 1;
    const bpm = 70 + (i % 120);
    const r = new Rng(((seed ^ 0x5dc4) >>> 0) || 1);
    r.chance(0.5);
    const db = r.range(5, 9);
    const rec = Math.min(r.range(0.15, 0.25), 0.8 * 60 / bpm);
    const d = subDuckOf(seed, bpm);
    if (d.depth !== db || d.recover !== rec) same = false;
    if (!(d.depth >= 5 && d.depth <= 9 && d.recover <= 0.25 && d.recover <= 0.8 * 60 / bpm + 1e-12)) inRange = false;
  }
  check('the depth and recovery the sketch drew', same);
  check('5-9 dB, back in 0.15-0.25 s, never over 0.8 of a beat', inRange);
  check('a pin moves nothing else', subDuckOf(5, 120, { depth: 6 }).recover === subDuckOf(5, 120).recover && subDuckOf(5, 120, { depth: 6 }).depth === 6);
  const list = CHOICES.filter((c) => c.id === 'subDuck');
  const loop = (kick, bass) => ({ tracks: { drums: [{ inst: kick ? 'kick' : 'hat', vel: 0.8, step: 0 }], bass: bass ? [{ midi: 40, vel: 0.6, step: 0 }] : [] } });
  check('only with a kick and a bass line', list[0].eligible(loop(true, true)) && !list[0].eligible(loop(false, true)) && !list[0].eligible(loop(true, false)));
  const share = (vibes) => {
    let n = 0;
    // Beside a neutral choice, so a lean shows in which is picked.
    const two = [...list, { id: 'other', options: [['today', 1], ['on', 1]], eligible: () => true }];
    for (let i = 0; i < 4000; i++) if (drawChoices(i, loop(true, true), vibes, two).subDuck === 'on') n++;
    return n / 4000;
  };
  const plain = share(null);
  const bouncy = share({ vibes: { bouncy: 1 }, strength: 0.9 });
  const serene = share({ vibes: { serene: 1 }, strength: 0.9 });
  check('bouncy leans toward it, serene away, never to none', bouncy > plain && serene < plain && serene > 0, `${serene.toFixed(3)} ${plain.toFixed(3)} ${bouncy.toFixed(3)}`);
  check('every setting is named, with its range', ['weight', 'vibes', 'depth', 'recover', 'beat'].every((k) => k in SUBDUCK));
}

// ---------------------------------------------------------------------
console.log('\nlateFloor (queue item 69)');
{
  // The sketch's draws: whether, how long, then the order, on its stream;
  // the wait rounded to two bars and at least two; the second part two bars
  // after the first.
  let same = true, shape = true;
  const seen = { together: 0, kitFirst: 0, bassFirst: 0 };
  for (let i = 0; i < 3000; i++) {
    const seed = i * 7919 + 3;
    const spb = i % 3 ? 16 : 12;
    const stepDur = 60 / (70 + (i % 100)) / 4;
    const r = new Rng(((seed ^ 0x1a7e) >>> 0) || 1);
    r.chance(0.3);
    const secs = r.range(8, 30);
    const bars = Math.max(2, Math.round(secs / (spb * stepDur) / 2) * 2);
    const order = r.weighted([['together', 40], ['kitFirst', 35], ['bassFirst', 25]]);
    const l = lateFloorOf(seed, spb, stepDur);
    seen[l.order]++;
    if (l.bars !== bars || l.order !== order) same = false;
    const first = Math.min(l.drums, l.bass);
    const second = Math.max(l.drums, l.bass);
    if (first !== bars * spb || second - first !== (l.order === 'together' ? 0 : 2 * spb) || l.bars % 2) shape = false;
    if ((l.order === 'kitFirst' && l.drums > l.bass) || (l.order === 'bassFirst' && l.bass > l.drums)) shape = false;
  }
  check('the wait and the order the sketch drew', same);
  check('two bars at the least, in twos; the second part two bars after the first', shape);
  check('together 40, the kit first 35, the bass first 25', Math.abs(seen.together / 3000 - 0.4) < 0.03 && Math.abs(seen.kitFirst / 3000 - 0.35) < 0.03, JSON.stringify(seen));
  check('a pin moves nothing else', lateFloorOf(9, 16, 0.125, { order: 'together' }).bars === lateFloorOf(9, 16, 0.125).bars);
  const list = CHOICES.filter((c) => c.id === 'lateFloor');
  const loop = (drums, bass, other, at = 0) => ({ spec: { bpm: 120 }, tracks: {
    drums: drums ? [{ inst: 'kick', vel: 0.8, step: 0 }] : [], bass: bass ? [{ midi: 40, vel: 0.6, step: 0 }] : [],
    chords: other ? [{ notes: [60, 64], vel: 0.5, step: at }] : [], melody: [], texture: [] } });
  check('only with drums or a bass, and something else that plays', list[0].eligible(loop(true, false, true)) && list[0].eligible(loop(false, true, true))
    && !list[0].eligible(loop(true, true, false)) && !list[0].eligible(loop(false, false, true)));
  // At 120 bpm a step is 0.125 s: a chord on step 7 is inside the first
  // second, on step 8 it is not.
  check('and only if that plays within its first second', list[0].eligible(loop(true, true, true, 7)) && !list[0].eligible(loop(true, true, true, 8)));
  const share = (vibes) => {
    const two = [...list, { id: 'other', options: [['today', 1], ['on', 1]], eligible: () => true }];
    let n = 0;
    for (let i = 0; i < 4000; i++) if (drawChoices(i, loop(true, true, true), vibes, two).lateFloor === 'on') n++;
    return n / 4000;
  };
  const plain = share(null);
  const hypnotic = share({ vibes: { hypnotic: 1 }, strength: 0.9 });
  const energetic = share({ vibes: { energetic: 1 }, strength: 0.9 });
  check('hypnotic leans toward it, energetic away, never to none', hypnotic > plain && energetic < plain && energetic > 0,
    `${energetic.toFixed(3)} ${plain.toFixed(3)} ${hypnotic.toFixed(3)}`);
  check('every setting is named, with its range', ['weight', 'vibes', 'seconds', 'round', 'first', 'orders', 'lag'].every((k) => k in LATEFLOOR));
}

// ---------------------------------------------------------------------
console.log('\nlongChords (queue item 57)');
{
  // The Ears 4 corpus's first 200 loops (seed 20261009): every loop whose
  // chords hold longer than today's keeps the octave today's chords take
  // (round 4: 11 of 87 held loops had jumped an octave before the fix).
  const master = new Rng(20261009);
  const centre = (list) => {
    const ns = list.filter((e) => e.vel && e.notes && e.notes.length).flatMap((e) => e.notes);
    return ns.length ? ns.reduce((a, b) => a + b, 0) / ns.length : null;
  };
  let held = 0, flips = 0;
  const kinds = {};
  for (let i = 0; i < 200; i++) {
    const spec = newSpec(master.seed32());
    const p = render(spec);
    const kind = p.harmony.longChords ?? 'developing';
    kinds[kind] = (kinds[kind] || 0) + 1;
    const a = centre(withTodaysChords(spec).tracks.chords);
    const b = centre(p.tracks.chords);
    if (a == null || b == null) continue;
    if (kind !== 'asIs') held++;
    if (Math.round((a - b) / 12) !== 0) flips++;
  }
  check('held chords keep today\'s octave (200 loops, 0 flips)', flips === 0, `${flips} of ${held} held loops flipped`);
  check('every length is drawn', ['asIs', 'double', 'eight', 'long', 'whole'].every((k) => kinds[k] > 0), JSON.stringify(kinds));
  // Over 2000 loops the spread is the approved one: today's lengths most.
  const m = new Rng(7);
  const n = {};
  for (let i = 0; i < 2000; i++) {
    const k = render(newSpec(m.seed32())).harmony.longChords;
    if (k) n[k] = (n[k] || 0) + 1;
  }
  const total = Object.values(n).reduce((x, y) => x + y, 0);
  check('today\'s lengths stay the most common', n.asIs / total > 0.45 && n.asIs / total < 0.62, JSON.stringify(n));
}

// ---------------------------------------------------------------------
// Whether a floor drop (`drift`'s floorDrop) takes `layer` out in bar `bar`:
// every bar of a single or a long drop; a stagger's last bars for the bass
// alone; a stutter's drops, one every two of their lengths (queue item 70).
function dropOut(drop, layer, bar) {
  const i = bar - drop.bar;
  if (i < 0 || i >= drop.bars) return false;
  if (drop.kind === 'stagger') return layer === 'bass' || i < drop.bars - drop.late;
  if (drop.kind === 'stutter') return i % (2 * drop.len) < drop.len;
  return true;
}

console.log('\nfloorDrop (queue item 58)');
{
  // Drift's full stop became a floor drop: the kick and the bass out for a
  // bar or two while the rest plays on, at 0.35 a pass at full wander, in
  // loops of 4 bars or more; no bars marked silent.
  const FLOOR = ['kick', 'softkick', 'frame'];
  let passes = 0, drops = 0, wrong = 0, stops = 0;
  for (let i = 0; i < 400; i++) {
    const p = render(newSpec(i * 104729 + 7));
    if (p.totalSteps / (p.stepsPerBar || 16) < 4) continue;
    for (let pass = 0; pass < 5; pass++) {
      const d = drift(p, new Rng(i * 31 + pass + 1), 1);
      passes++;
      if (d.driftSilentBars) stops++;
      if (!d.floorDrop) continue;
      drops++;
      const spb = p.stepsPerBar || 16;
      for (const layer of LAYERS) {
        for (const e of d.tracks[layer]) {
          const floor = layer === 'bass' || (layer === 'drums' && FLOOR.includes(e.inst));
          if (floor && e.vel && dropOut(d.floorDrop, layer, Math.floor(e.step / spb))) wrong++;
        }
      }
    }
  }
  check('a floor drop takes out the kick and the bass', drops > 0 && wrong === 0, `${wrong} wrong in ${drops} drops`);
  check('never a full stop', stops === 0, `${stops} passes`);
  check('about 0.35 a pass at full wander', drops / passes > 0.28 && drops / passes < 0.42, `${(drops / passes).toFixed(3)} of ${passes}`);
}

console.log('\nfloorDrop shapes (queue item 70)');
{
  // The same loops with the shapes and without: the drops fall on the same
  // passes and start on the same bars; nothing else the wander draws moves;
  // each shape is what it says (the bar's length deciding its bars), and
  // stagger and stutter only with a bass line.
  const FLOOR = ['kick', 'softkick', 'frame'];
  let passes = 0, same = true, rest = true, wrong = 0, noBass = 0, stops = 0;
  const kinds = { single: 0, stagger: 0, stutter: 0, long: 0 };
  let staggerOk = true, stutterOk = true, longOk = true;
  for (let i = 0; i < 400; i++) {
    const p = render(newSpec(i * 104729 + 7));
    if (p.totalSteps / (p.stepsPerBar || 16) < 4) continue;
    const shaped = { ...p, choices: { ...p.choices, floorDrop: 'shapes' } };
    const plain = { ...p, choices: { ...p.choices, floorDrop: 'today' } };
    const spb = p.stepsPerBar || 16;
    const barSec = spb * 60 / p.spec.bpm / 4;
    const hasBass = p.tracks.bass.some((e) => e.vel);
    for (let pass = 0; pass < 5; pass++) {
      const a = drift(plain, new Rng(i * 31 + pass + 1), 1);
      const b = drift(shaped, new Rng(i * 31 + pass + 1), 1);
      passes++;
      if (b.driftSilentBars) stops++;
      if (!!a.floorDrop !== !!b.floorDrop) { same = false; continue; }
      for (const layer of LAYERS) {
        b.tracks[layer].forEach((e, k) => {
          const f = a.tracks[layer][k];
          const floor = layer === 'bass' || (layer === 'drums' && FLOOR.includes(e.inst));
          if (!floor || !b.floorDrop) {
            if (JSON.stringify(e) !== JSON.stringify(f)) rest = false;
            return;
          }
          const out = dropOut(b.floorDrop, layer, Math.floor(e.step / spb));
          if (out && e.vel) wrong++;
          if (!out && e.vel !== a.tracks[layer][k].vel && !dropOut(a.floorDrop, layer, Math.floor(e.step / spb))) rest = false;
        });
      }
      if (!b.floorDrop) continue;
      const d = b.floorDrop;
      kinds[d.kind]++;
      if (!hasBass && (d.kind === 'stagger' || d.kind === 'stutter')) noBass++;
      if (d.kind === 'stagger' && !(d.late === (barSec < 2.5 ? 2 : 1) && d.bar === a.floorDrop.bar)) staggerOk = false;
      if (d.kind === 'stutter' && !(d.count >= 2 && d.count <= 4 && d.len === (barSec < 2.2 ? 2 : 1))) stutterOk = false;
      const bars = p.totalSteps / spb;
      if (d.kind === 'long' && !(bars <= 8 ? d.bars === bars && d.bar === 0 : (d.bars >= 4 || d.bar + d.bars === bars) && d.bars <= 8)) longOk = false;
    }
  }
  const n = Object.values(kinds).reduce((x, y) => x + y, 0);
  check('the drops fall on the same passes as without the shapes', same);
  check('nothing else the wander draws moves', rest);
  check('each shape takes out the kick and the bass where it says', wrong === 0, `${wrong} wrong`);
  check('never a full stop', stops === 0);
  check('stagger and stutter only with a bass line', noBass === 0);
  check('a stagger brings the bass back one bar later, two in a short bar', staggerOk);
  check('a stutter is two to four drops of one bar, two in a short bar', stutterOk);
  check('a long drop is 4-8 bars (cut at the loop\'s end), or the whole pass of a loop of 8 bars or fewer', longOk);
  check('every shape is drawn, single the most', Object.values(kinds).every((k) => k > 0) && kinds.single > kinds.stagger && kinds.stagger > kinds.long,
    JSON.stringify(kinds) + ` of ${n}`);
  check('every setting is named, with its range', ['weight', 'shapes', 'needBass', 'shapeVibes', 'staggerShort', 'stutterShort', 'stutter', 'longSeconds', 'longBars', 'wholeUnder'].every((k) => k in FLOORDROP));
}

console.log('\npedalBass (queue item 71)');
{
  // The sketch's draws on its stream; then, over loops that draw the pedal:
  // the tonic most of the time and one neighbor the rest, in the bass's
  // register, at 0.4-2 notes a second; the flat second only where the mode
  // has it; and drift spends the loop's own bass's draws, so no other layer's
  // variation moves.
  let same = true;
  for (let i = 0; i < 2000; i++) {
    const seed = i * 7919 + 9;
    const steps = [[0, 2, 4, 5, 7, 9, 11], [0, 1, 3, 5, 7, 8, 10], [0, 3, 5, 7, 10]][i % 3];
    const r = new Rng(((seed ^ 0x9eda1) >>> 0) || 1);
    r.chance(0.35);
    const rhythm = r.weighted([['held', 25], ['half', 30], ['quarter', 25], ['offbeat', 20]]);
    const has = (st) => steps.includes(((st % 12) + 12) % 12);
    const nbs = [];
    if (has(2)) nbs.push([2, 3]);
    if (has(10)) nbs.push([-2, 2]);
    if (has(1)) nbs.push([1, 3]);
    if (has(7)) nbs.push([7, 2]);
    const nb = nbs.length ? r.weighted(nbs) : 0;
    const share = r.range(0.1, 0.25);
    const o = pedalOf(seed, steps);
    if (o.rhythm !== rhythm || o.nb !== nb || o.share !== share) same = false;
  }
  check('the rhythm, neighbor and share the sketch drew', same);
  let loops = 0, tonic = 0, neighbor = 0, other = 0, downbeat = true, register = true, rate = [], flat = true, styles = true, replay = true, passes = 0;
  for (let i = 0; i < 3000 && loops < 150; i++) {
    const spec = newSpec(i * 104729 + 13);
    const p = render(spec);
    if (!p.meta.pedal) continue;
    loops++;
    if (!PEDAL.styles.includes(p.meta.pedal.was) || p.meta.bassStyle !== 'pedal') styles = false;
    const scale = SCALES[spec.scale].steps;
    const notes = p.tracks.bass.filter((e) => e.vel);
    const pcs = notes.map((e) => (((e.midi - spec.root) % 12) + 12) % 12);
    // The downbeat on the tonic, the neighbor only between, never twice in a
    // row (the sketch's, kept by Mikey, 2026-10-25), before drift's passing
    // tones.
    const spb = p.stepsPerBar || 16;
    const all = p.tracks.bass;
    all.forEach((e, k) => {
      const pc = (((e.midi - spec.root) % 12) + 12) % 12;
      if (pc !== 0 && (e.step % spb === 0 && p.meta.pedal.rhythm !== 'offbeat')) downbeat = false;
      if (pc !== 0 && k > 0 && (((all[k - 1].midi - spec.root) % 12) + 12) % 12 !== 0) downbeat = false;
    });
    for (const pc of pcs) {
      if (pc === 0) tonic++;
      else if (pc === ((p.meta.pedal.nb % 12) + 12) % 12) neighbor++;
      else other++;
    }
    if (notes.some((e) => e.midi < 36 || e.midi > 57)) register = false;
    if (p.meta.pedal.nb === 1 && !scale.includes(1)) flat = false;
    const secs = p.totalSteps * 60 / spec.bpm / 4;
    rate.push(notes.length / secs);
    // The same loop with a bass of the loop's own length in its place: drift
    // makes the same draws, so every other layer wanders the same way.
    const own = { ...p, meta: { ...p.meta, pedal: undefined }, tracks: { ...p.tracks, bass: Array.from({ length: p.meta.pedal.n0 }, (_, k) => ({ step: k % p.totalSteps, dur: 1, midi: 40, vel: 0.5 })) } };
    for (let pass = 0; pass < 4; pass++) {
      passes++;
      const a = drift(p, new Rng(i * 17 + pass + 1), 1);
      const b = drift(own, new Rng(i * 17 + pass + 1), 1);
      for (const layer of ['drums', 'chords', 'melody', 'texture']) if (JSON.stringify(a.tracks[layer]) !== JSON.stringify(b.tracks[layer])) replay = false;
      if (JSON.stringify(a.floorDrop) !== JSON.stringify(b.floorDrop)) replay = false;
    }
  }
  const n = tonic + neighbor + other;
  rate.sort((x, y) => x - y);
  check('only over a held, pulse, dub or walking bass', loops > 50 && styles, `${loops} loops`);
  check('the downbeat on the tonic, the neighbor only between and never twice in a row', downbeat);
  check('the tonic and one neighbor, nothing else; the tonic most (the sketch\'s: about 90%)', other === 0 && tonic / n > 0.85 && neighbor / n > 0.05,
    `tonic ${(tonic / n).toFixed(3)}, neighbor ${(neighbor / n).toFixed(3)}, other ${other}`);
  check('in the bass\'s register, 36-57', register);
  check('0.4-2 notes a second (median)', rate[rate.length >> 1] >= 0.4 && rate[rate.length >> 1] <= 2, `${rate[0].toFixed(2)}-${rate[rate.length - 1].toFixed(2)}, median ${rate[rate.length >> 1].toFixed(2)}`);
  check('the flat second only in a mode that has it', flat);
  check('drift spends the loop\'s own bass\'s draws: no other layer\'s variation moves', replay, `${passes} passes`);
  check('every setting is named, with its range', ['weight', 'vibes', 'styles', 'rhythms', 'neighbors', 'neighborVibes', 'share', 'low', 'high'].every((k) => k in PEDAL));
}

console.log(failures === 0 ? '\nAll good.\n' : `\n${failures} failing.\n`);
process.exit(failures === 0 ? 0 : 1);
