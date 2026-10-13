// The core at other sample rates (queue item 45, charter promise 4).
//
//   node tools/rates.mjs              check the figures below; exit 1 if any fails
//   node tools/rates.mjs --json FILE  also write every figure out
//
// The core takes any rate from 22,050 to 192,000 Hz, with every rate-
// dependent piece derived from the rate it is started at. The golden hashes
// (tools/golden.sh --rates) say the same code makes the same samples on every
// target at each rate; this says the music is the same music at each rate,
// against the 48 kHz render the app has always had:
//
//   level     integrated loudness of three golden lists (K-weighted, BS.1770,
//             ungated) at 22,050, 44,100, 96,000 and 192,000 Hz, within 0.2 dB
//             of the 48 kHz render's
//   tone      the coffee-shop test of tools/measure.mjs, voice by voice: the
//             share of a dry note's A-weighted energy in 2-5 kHz, for every
//             melody voice at three pitches, within 0.5 points of 48 kHz's at
//             96,000 and 192,000 Hz and within 1.5 points at 44,100 and
//             22,050 Hz (Mikey, 2026-10-14: a lower rate's filters and
//             interpolation lose a little more near Nyquist, and no voice is
//             trimmed for it). The whole lists' shares are printed too, for
//             the record, and not held to it.
//   aliasing  a loud pure tone through the whole chain, at five pitches from
//             C5 to C8 (the saturator and its oversampling are the likeliest
//             place to fold) and sawtooth voices at the same pitches (their
//             band-limited tables): the energy between 6 and 10.5 kHz that
//             is not the tone's own harmonics, in dB under the tone, no more
//             than 48 kHz's plus 1 dB (or under -60 dB, where neither could
//             be heard)
//
// No browser: Node runs the WebAssembly. Rates above 96 kHz need the 192 kHz
// build (tools/build-core.sh --wide), which this makes if it is missing.

import fs from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { atRate, readList, renderWasm } from './golden-lib.mjs';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const GOLDEN = path.join(ROOT, 'core', 'golden');
const argv = process.argv.slice(2);
const jsonAt = argv.indexOf('--json');
const json = jsonAt >= 0 ? argv[jsonAt + 1] : null;

const RATES = [22050, 44100, 96000, 192000];
const REFERENCE = 48000;
const LEVEL_TOLERANCE = 0.2;
const TONE_TOLERANCE = (rate) => (rate <= 44100 ? 1.5 : 0.5);
const ALIAS_TOLERANCE = 1;
const ALIAS_AUDIBLE = -60;

const lists = fs.readFileSync(path.join(GOLDEN, 'rates.txt'), 'utf8').split('\n')
  .filter((l) => l.startsWith('lists ')).flatMap((l) => l.slice(6).trim().split(/\s+/));

const WIDE = path.join(ROOT, 'core', 'target', 'dlcore-wide.wasm');
if (!fs.existsSync(WIDE)) {
  const made = spawnSync('sh', [path.join(ROOT, 'tools', 'build-core.sh'), '--wide'], { stdio: 'inherit' });
  if (made.status !== 0) process.exit(1);
}
const WEB = fs.readFileSync(path.join(ROOT, 'js', 'dlcore.wasm'));
const WIDE_BYTES = fs.readFileSync(WIDE);
const coreFor = (rate) => (rate > 96000 ? WIDE_BYTES : WEB);

// ------------------------------------------------------------ the measures

// BS.1770 K-weighting at any rate (tools/measure.mjs: the analogue
// prototypes through the bilinear transform), then the mean square, in LUFS.
function kWeighting(rate) {
  let K = Math.tan(Math.PI * 1681.974450955533 / rate);
  let Q = 0.7071752369554196;
  const Vh = 10 ** (3.999843853973347 / 20);
  const Vb = Vh ** 0.4996667741545416;
  let a0 = 1 + K / Q + K * K;
  const shelf = {
    b: [(Vh + Vb * K / Q + K * K) / a0, 2 * (K * K - Vh) / a0, (Vh - Vb * K / Q + K * K) / a0],
    a: [2 * (K * K - 1) / a0, (1 - K / Q + K * K) / a0],
  };
  K = Math.tan(Math.PI * 38.13547087602444 / rate);
  Q = 0.5003270373238773;
  a0 = 1 + K / Q + K * K;
  return [shelf, { b: [1, -2, 1], a: [2 * (K * K - 1) / a0, (1 - K / Q + K * K) / a0] }];
}

function loudness(data, rate) {
  const [s, h] = kWeighting(rate);
  let x1 = 0, x2 = 0, y1 = 0, y2 = 0, z1 = 0, z2 = 0, sum = 0;
  for (let i = 0; i < data.length; i++) {
    const x = data[i];
    const y = s.b[0] * x + s.b[1] * x1 + s.b[2] * x2 - s.a[0] * y1 - s.a[1] * y2;
    const z = y - 2 * y1 + y2 - h.a[0] * z1 - h.a[1] * z2;
    x2 = x1; x1 = x; y2 = y1; y1 = y; z2 = z1; z1 = z;
    sum += z * z;
  }
  return -0.691 + 10 * Math.log10(sum / data.length);
}

function fft(re, im) {
  const n = re.length;
  for (let i = 1, j = 0; i < n; i++) {
    let bit = n >> 1;
    for (; j & bit; bit >>= 1) j ^= bit;
    j ^= bit;
    if (i < j) { [re[i], re[j]] = [re[j], re[i]]; [im[i], im[j]] = [im[j], im[i]]; }
  }
  for (let len = 2; len <= n; len <<= 1) {
    const ang = -2 * Math.PI / len;
    const wr = Math.cos(ang), wi = Math.sin(ang);
    for (let i = 0; i < n; i += len) {
      let cr = 1, ci = 0;
      for (let k = 0; k < len / 2; k++) {
        const ur = re[i + k], ui = im[i + k];
        const vr = re[i + k + len / 2] * cr - im[i + k + len / 2] * ci;
        const vi = re[i + k + len / 2] * ci + im[i + k + len / 2] * cr;
        re[i + k] = ur + vr; im[i + k] = ui + vi;
        re[i + k + len / 2] = ur - vr; im[i + k + len / 2] = ui - vi;
        const nr = cr * wr - ci * wi;
        ci = cr * wi + ci * wr; cr = nr;
      }
    }
  }
}

function aWeight(f) {
  const f2 = f * f;
  const num = 12194 ** 2 * f2 * f2;
  const den = (f2 + 20.6 ** 2) * Math.sqrt((f2 + 107.7 ** 2) * (f2 + 737.9 ** 2)) * (f2 + 12194 ** 2);
  return (num / den) * 10 ** (2 / 20);
}

// Welch periodograms of 0.1 s windows (a power of two of frames, so the
// resolution is the same Hz at every rate), summed over the render.
function spectrum(data, rate) {
  const N = 2 ** Math.round(Math.log2(rate / 10));
  const re = new Float64Array(N), im = new Float64Array(N);
  const power = new Float64Array(N / 2);
  for (let start = 0; start + N <= data.length; start += N / 2) {
    for (let i = 0; i < N; i++) { re[i] = data[start + i] * (0.5 - 0.5 * Math.cos(2 * Math.PI * i / N)); im[i] = 0; }
    fft(re, im);
    for (let i = 0; i < N / 2; i++) power[i] += re[i] * re[i] + im[i] * im[i];
  }
  return { power, df: rate / N };
}

// The A-weighted share of the energy in 2-5 kHz, in percent.
function toneShare(data, rate) {
  const { power, df } = spectrum(data, rate);
  let band = 0, total = 0;
  for (let i = 1; i < power.length; i++) {
    const f = i * df;
    if (f > 20000) break;
    const p = power[i] * aWeight(f) ** 2;
    total += p;
    if (f >= 2000 && f <= 5000) band += p;
  }
  return 100 * band / total;
}

// What folds back, for a steady tone: the energy between 6 and 10.5 kHz
// (a band every rate holds) that is not within 12 Hz of a harmonic of the
// tone, in dB under the energy at the tone. The spectrum is of 0.3 s from the
// middle of the note (the same time at every rate), zero-padded to a fine
// grid.
function foldedDb(data, rate, hz) {
  const W = Math.round(0.3 * rate);
  const from = Math.round(0.5 * rate);
  let N = 1;
  while (N < 8 * W) N *= 2;
  const re = new Float64Array(N), im = new Float64Array(N);
  for (let i = 0; i < W; i++) re[i] = data[from + i] * (0.5 - 0.5 * Math.cos(2 * Math.PI * i / W));
  fft(re, im);
  const df = rate / N;
  let tone = 0, folded = 0;
  for (let i = 1; i < N / 2; i++) {
    const f = i * df;
    const p = re[i] * re[i] + im[i] * im[i];
    if (Math.abs(f - hz) <= 12) tone += p;
    if (f >= 6000 && f <= 10500) {
      const k = Math.round(f / hz);
      if (!(k >= 1 && Math.abs(f - k * hz) <= 12)) folded += p;
    }
  }
  return 10 * Math.log10(folded / tone);
}

// ------------------------------------------------------------- the renders

const render = (list, keep = 'out') => renderWasm(coreFor(list.rate), list, keep).kept;

// A note for the aliasing test, played alone: the tone control wide open (so
// the master's lowpass does not hide what folded) and the tape wobble off (its
// sidebands are not harmonics, and are the same at every rate), a long loud
// note.
function toneList(rate, voice, midi) {
  return {
    name: `tone-${voice}-${midi}`, rate, quality: 'full', frames: Math.round(1.6 * rate),
    events: [
      { type: 'param', id: 12, op: 0, value: 20000, time: 0, tau: 0 },
      { type: 'param', id: 13, op: 0, value: 0, time: 0, tau: 0 },
      { type: 'param', id: 14, op: 0, value: 0, time: 0, tau: 0 },
      { type: 'key', seed: 1, pass: 0, step: 0, ord: 0 },
      { type: 'note', voice, channel: 0, time: 0.1, midi, dur: 1.4, vel: 0.9, parts: 0, extra: [] },
    ],
  };
}
const SINE = 4;
const SAW = 9; // sawpluck: a sawtooth voice with no vibrato
const MIDIS = [72, 84, 96, 100, 108];
const hzOf = (midi) => 440 * 2 ** ((midi - 69) / 12);

// ---------------------------------------------------------------- the runs

const rows = [];
let failed = 0;
const fail = (what) => { failed++; console.log(`  FAIL  ${what}`); };

console.log(`\nLevel (held) and tone (for the record), whole lists, against ${REFERENCE} Hz (${lists.join(', ')})`);
const base = {};
for (const name of lists) {
  const list = readList(fs.readFileSync(path.join(GOLDEN, `${name}.dlnotes`), 'utf8'));
  base[name] = { level: loudness(render(list), list.rate), tone: toneShare(render(list), list.rate) };
  console.log(`  ${name.padEnd(12)} ${String(REFERENCE).padStart(6)} Hz  ${base[name].level.toFixed(2)} LUFS  tone ${base[name].tone.toFixed(2)} %`);
  for (const rate of RATES) {
    const at = atRate(list, rate);
    const out = render(at);
    const level = loudness(out, rate);
    const tone = toneShare(out, rate);
    rows.push({ list: name, rate, level, tone });
    const dLevel = level - base[name].level;
    const dTone = tone - base[name].tone;
    const ok = Math.abs(dLevel) <= LEVEL_TOLERANCE;
    console.log(`  ${name.padEnd(12)} ${String(rate).padStart(6)} Hz  ${level.toFixed(2)} LUFS (${dLevel >= 0 ? '+' : ''}${dLevel.toFixed(2)})  tone ${tone.toFixed(2)} % (${dTone >= 0 ? '+' : ''}${dTone.toFixed(2)})${ok ? '' : '   <-- outside'}`);
    if (!ok) fail(`${name} at ${rate} Hz: level ${dLevel.toFixed(2)} dB`);
  }
}

// The coffee-shop probe at every rate, voice by voice: dry notes on the
// melody channel, 0.4 s, velocity 0.8, three pitches.
const FM_KEYS = [2, 260, 0.006, 0.4, 0];
const VOICES = {
  kalimba: [0, 3, []], fiddle: [1, 0, []], fm: [3, 0, FM_KEYS], sine: [4, 0, []], tubular: [5, 0, []],
  softpad: [6, 0, []], analogpad: [7, 0, []], analoglead: [8, 0, []], sawpluck: [9, 0, []], beep: [10, 0, []],
  moog: [11, 0, []], whistle: [12, 0, []], nylon: [13, 0, []], accordion: [14, 0, []], stab: [15, 0, []],
  ocarina: [16, 0, []], flute: [17, 0, []], panflute: [18, 0, []], templebell: [20, 0, []],
};
function noteList(rate, voice, parts, extra, midi) {
  return {
    name: 'probe', rate, quality: 'full', frames: Math.round(2.6 * rate),
    events: [
      { type: 'key', seed: midi, pass: 0, step: 0, ord: 0 },
      { type: 'note', voice, channel: 0, time: 0.05, midi, dur: 0.4, vel: 0.8, parts, extra },
    ],
  };
}
function probe(rate, voice, parts, extra) {
  let sum = 0;
  for (const midi of [60, 72, 84]) sum += toneShare(render(noteList(rate, voice, parts, extra, midi), 'melody'), rate);
  return sum / 3;
}
console.log(`\nTone, voice by voice (the 2-5 kHz share of a dry 0.4 s note, three pitches; against ${REFERENCE} Hz)`);
for (const [name, [voice, parts, extra]] of Object.entries(VOICES)) {
  const at48 = probe(REFERENCE, voice, parts, extra);
  const cells = [];
  for (const rate of RATES) {
    const share = probe(rate, voice, parts, extra);
    const d = share - at48;
    rows.push({ voice: name, rate, share, at48 });
    const ok = Math.abs(d) <= TONE_TOLERANCE(rate);
    cells.push(`${rate}: ${d >= 0 ? '+' : ''}${d.toFixed(2)}${ok ? '' : ' <--'}`);
    if (!ok) fail(`${name} at ${rate} Hz: tone share ${share.toFixed(2)} % against ${at48.toFixed(2)} % at ${REFERENCE} Hz`);
  }
  console.log(`  ${name.padEnd(11)} ${at48.toFixed(2).padStart(6)} %   ${cells.join('   ')}`);
}

console.log(`\nWhat folds back, against ${REFERENCE} Hz (dB under the tone, 6-10.5 kHz, not its harmonics; lower is cleaner)`);
for (const [label, voice] of [['sine, through the saturator', SINE], ['sawtooth', SAW]]) {
  for (const midi of MIDIS) {
    const hz = hzOf(midi);
    const at48 = foldedDb(render(toneList(REFERENCE, voice, midi), voice === SINE ? 'out' : 'melody'), REFERENCE, hz);
    const cells = [];
    for (const rate of RATES) {
      const folded = foldedDb(render(toneList(rate, voice, midi), voice === SINE ? 'out' : 'melody'), rate, hz);
      const ok = folded <= Math.max(at48, ALIAS_AUDIBLE) + ALIAS_TOLERANCE;
      rows.push({ alias: label, midi, rate, folded, at48 });
      cells.push(`${rate}: ${folded.toFixed(1)}${ok ? '' : ' <--'}`);
      if (!ok) fail(`${label}, note ${midi} (${hz.toFixed(0)} Hz) at ${rate} Hz folds to ${folded.toFixed(1)} dB, against ${at48.toFixed(1)} at ${REFERENCE} Hz`);
    }
    console.log(`  ${label.padEnd(28)} note ${String(midi).padStart(3)} ${hz.toFixed(0).padStart(5)} Hz   ${REFERENCE}: ${at48.toFixed(1)}   ${cells.join('   ')}`);
  }
}

if (json) fs.writeFileSync(json, `${JSON.stringify(rows, null, 1)}\n`);
console.log(failed ? `\n${failed} outside\n` : '\nAll within.\n');
process.exit(failed ? 1 : 0);
