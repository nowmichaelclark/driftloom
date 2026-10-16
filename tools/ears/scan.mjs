// A quick look at the corpus without rendering audio: per loop its lead
// profile, tempo, length, how busy each layer is, the chord and texture
// voices, the reverb space and the vibes, so loops can be picked for
// clips (busy and sparse, with and without drums) before anything renders.
//
// Usage: node scan.mjs REPO [SEED] [N] > scan.json
// The corpus is the one render2.mjs uses: master Rng(SEED), N specs.
import path from 'node:path';
import { pathToFileURL } from 'node:url';

const [REPO, SEED = '20261009', N = '80'] = process.argv.slice(2);
const at = (f) => pathToFileURL(path.resolve(REPO, f)).href;
const { newSpec, render } = await import(at('js/generator.js'));
const { Rng } = await import(at('js/rng.js'));

const master = new Rng(Number(SEED));
const out = [];
for (let i = 0; i < Number(N); i++) {
  const spec = newSpec(master.seed32());
  const r = render(spec);
  const lead = Object.entries(spec.mix || {}).sort((a, b) => b[1] - a[1])[0]?.[0] || '?';
  const sec = r.totalSteps * 60 / spec.bpm / 4;
  const bars = r.totalSteps / (r.stepsPerBar || 16);
  const count = (layer) => (r.tracks[layer] || []).filter((e) => e.vel).length;
  const chordVoices = [...new Set((r.tracks.chords || []).filter((e) => e.vel).map((e) => e.voice))];
  let changes = 0, prev = null;
  for (const e of (r.tracks.chords || []).filter((x) => x.vel && x.notes && x.notes.length)) {
    const k = e.notes.slice().sort((a, b) => a - b).join(',');
    if (prev !== null && k !== prev) changes++;
    prev = k;
  }
  const n = { drums: count('drums'), bass: count('bass'), chords: count('chords'), melody: count('melody'), texture: count('texture') };
  out.push({
    pick: i, seed: spec.seed, lead, bpm: spec.bpm, bars, sec: +sec.toFixed(1),
    perSec: Object.fromEntries(Object.entries(n).map(([k, v]) => [k, +(v / sec).toFixed(2)])),
    drums: n.drums > 0 && !(spec.mutes || {}).drums,
    chordVoices, melodyVoice: r.meta.melodyVoice, bassVoice: r.meta.bassVoice, texture: r.meta.textureKind,
    space: +(spec.tone?.space ?? 0).toFixed(2), chordsPerMin: +(changes / sec * 60).toFixed(1),
    vibes: spec.vibes || null, mutes: spec.mutes || {},
  });
}
console.log(JSON.stringify(out, null, 1));
