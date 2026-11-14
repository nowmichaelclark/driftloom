#!/usr/bin/env node
// Each voice's loudness in the mix over a note's life (queue item 102g,
// session 16b): the numbers `[longform loudness]` in data/longform.txt holds,
// by which a long piece's join knows, from its notes alone, how much of the
// mix the layers it keeps carry (core/src/generator/heard.rs, `Heard::share`).
// One note of every voice the probe knows (tools/play-lib.mjs's `probe`,
// the voice through its layer's own path, dry), held 4 s at velocity 0.8 at
// three pitches across its layer's window (one for the kit), its mean power
// in six spans of its life (0 to 1/8 s, to 1/4, 1/2, 1, 2 and 4 s), as dB
// at velocity 1 (power goes with the velocity squared: measured, 6.0 dB from
// 0.4 to 0.8 on every tuned voice, 5.4 to 5.7 on the kit) through the
// channel's own gain (`CHANNEL_GAIN`, core/src/player/synth.rs), rounded to
// a whole dB; -99 for nothing.
//
//   node tools/voice-loudness.mjs           the section, to paste
//   node tools/voice-loudness.mjs --check   exit 1 if data/longform.txt's
//                                           section is not what this measures

import fs from 'node:fs';
import { probe, probeVoices, LAYERS, ROOT } from './play-lib.mjs';

const WINDOWS = { melody: [55, 79], chords: [43, 67], bass: [28, 52], texture: [67, 91], drums: [60, 60] };
// The channels' gains, by layer (core/src/player/synth.rs, `CHANNEL_GAIN`,
// in the core's channel order melody, chords, bass, texture, drums).
const GAIN = { melody: 0.58, chords: 0.44, bass: 0.62, texture: 0.5, drums: 0.82 };
const SPANS = [[0, 0.125], [0.125, 0.25], [0.25, 0.5], [0.5, 1], [1, 2], [2, 4]];
const RATE = 22050;
const VEL = 0.8;

const power = (d, a, b) => {
  const [i, j] = [Math.round(a * RATE), Math.min(d.length, Math.round(b * RATE))];
  let s = 0;
  for (let k = i; k < j; k++) s += d[k] * d[k];
  return s / Math.max(1, j - i);
};

const lines = [];
const voices = probeVoices();
for (const layer of LAYERS) {
  for (const voice of voices[layer]) {
    const [lo, hi] = WINDOWS[layer];
    const midis = layer === 'drums' ? [lo] : [lo + 6, Math.round((lo + hi) / 2), hi - 6];
    const acc = SPANS.map(() => 0);
    for (const midi of midis) {
      const d = probe({ layer, voice, midi, vel: VEL, seconds: 4, rate: RATE, tail: 0.5, seed: 7 });
      SPANS.forEach(([a, b], i) => (acc[i] += power(d, 0.05 + a, 0.05 + b) / midis.length));
    }
    const db = acc.map((p) => (p > 1e-10 ? Math.round(10 * Math.log10(p) - 20 * Math.log10(VEL) + 20 * Math.log10(GAIN[layer])) : -99));
    lines.push(`${layer}.${voice} = ${db.map((x) => Math.max(-99, x)).join(' ')}`);
  }
}
const section = lines.join('\n');
if (process.argv.includes('--check')) {
  const file = fs.readFileSync(`${ROOT}/data/longform.txt`, 'utf8');
  const have = file.split('[longform loudness]')[1]?.split('\n').filter((l) => /^\w+\.\w+ = /.test(l)).join('\n');
  if (have !== section) {
    console.error('voice-loudness: data/longform.txt\'s [longform loudness] is not what the voices measure; this is:');
    console.error(section);
    process.exit(1);
  }
  console.log(`voice-loudness: ${lines.length} voices, as data/longform.txt has them`);
} else {
  console.log(section);
}
