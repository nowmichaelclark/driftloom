// The meters the measuring tools share (queue item 96; they were inside
// tools/measure.mjs, pasted into its pages): K-weighted loudness and
// loudness range, the 2-5 kHz presence share, a render's fingerprint, and
// the seed a probed note draws from.
//
//   node tools/meter.mjs --selftest   the loudness meter against reference
//                                     signals with known answers

// ITU-R BS.1770-4 integrated loudness, and EBU Tech 3342 loudness range.

// K-weighting: a high-frequency shelf, then the RLB highpass. The standard
// publishes coefficients for 48k only; these are its analogue prototypes
// through the bilinear transform, which reproduce the published 48k figures
// and hold at 44.1k, the rate everything here renders at by default.
function kWeighting(rate) {
  let K = Math.tan(Math.PI * 1681.974450955533 / rate);
  let Q = 0.7071752369554196;
  const Vh = Math.pow(10, 3.999843853973347 / 20);
  const Vb = Math.pow(Vh, 0.4996667741545416);
  let a0 = 1 + K / Q + K * K;
  const shelf = {
    b: [(Vh + Vb * K / Q + K * K) / a0, 2 * (K * K - Vh) / a0, (Vh - Vb * K / Q + K * K) / a0],
    a: [2 * (K * K - 1) / a0, (1 - K / Q + K * K) / a0],
  };
  K = Math.tan(Math.PI * 38.13547087602444 / rate);
  Q = 0.5003270373238773;
  a0 = 1 + K / Q + K * K;
  const highpass = { b: [1, -2, 1], a: [2 * (K * K - 1) / a0, (1 - K / Q + K * K) / a0] };
  return [shelf, highpass];
}

// One channel at weight 1.0: the bus is mono. Returns integrated loudness
// in LUFS (-Infinity for silence) and loudness range in LU, and the punch
// figures: the loudest momentary (400ms) and short-term (3s) loudness, as
// EBU R128's M and S, and the 95th percentile of momentary loudness,
// because a single maximum is one block and one block is fragile.
//
// All are built from K-weighted energy summed in 100ms hops, since every
// block either standard asks for is a whole number of them: 400ms blocks
// stepping 100ms (75% overlap) for integrated and momentary loudness, 3s
// short-term blocks at the same 10Hz for the range and short-term.
function loudnessOf(data, rate) {
  const [s, h] = kWeighting(rate);
  const hop = Math.round(rate / 10);
  const hops = Math.floor(data.length / hop);
  const energy = new Float64Array(hops);
  let x1 = 0, x2 = 0, y1 = 0, y2 = 0, z1 = 0, z2 = 0;
  for (let j = 0; j < hops; j++) {
    let sum = 0;
    for (let i = j * hop, end = i + hop; i < end; i++) {
      const x = data[i];
      const y = s.b[0] * x + s.b[1] * x1 + s.b[2] * x2 - s.a[0] * y1 - s.a[1] * y2;
      const z = y - 2 * y1 + y2 - h.a[0] * z1 - h.a[1] * z2;
      x2 = x1; x1 = x;
      y2 = y1; y1 = y;
      z2 = z1; z1 = z;
      sum += z * z;
    }
    energy[j] = sum;
  }
  const blocks = (len) => {
    const out = [];
    for (let j = len - 1; j < hops; j++) {
      let sum = 0;
      for (let k = j - len + 1; k <= j; k++) sum += energy[k];
      out.push(sum / (len * hop));
    }
    return out;
  };
  const lufs = (z) => -0.691 + 10 * Math.log10(z);
  const meanOf = (zs) => zs.reduce((a, b) => a + b, 0) / zs.length;
  // Absolute gate at -70 LUFS, then a relative one below the mean of what
  // survived it: -10 LU for integrated loudness, -20 LU for the range.
  const gated = (zs, relative) => {
    const loud = zs.filter((z) => lufs(z) > -70);
    if (!loud.length) return [];
    const gate = lufs(meanOf(loud)) + relative;
    return loud.filter((z) => lufs(z) > gate);
  };
  const m = blocks(4);
  const st = blocks(30);
  const momentary = gated(m, -10);
  const integrated = momentary.length ? lufs(meanOf(momentary)) : -Infinity;
  const shortTerm = gated(st, -20).map(lufs).sort((a, b) => a - b);
  const pick = (sorted, p) => sorted[Math.round((sorted.length - 1) * p)];
  const lra = shortTerm.length ? pick(shortTerm, 0.95) - pick(shortTerm, 0.1) : 0;
  // The maxima are ungated, as a meter's are. The percentile is over the
  // blocks that clear the absolute gate, so the digital silence of a
  // scheduled rest does not count as a quiet moment of the music.
  const peakOf = (zs) => (zs.length ? lufs(Math.max(...zs)) : -Infinity);
  const audible = m.map(lufs).filter((l) => l > -70).sort((a, b) => a - b);
  return {
    integrated,
    lra,
    momentaryMax: peakOf(m),
    momentaryP95: audible.length ? pick(audible, 0.95) : -Infinity,
    shortTermMax: peakOf(st),
  };
}

// Reference signals with known answers. A 997Hz sine at full scale in one
// channel is -3.01 LUFS by definition; the gating cases are 13 dB quieter
// tone either side of a louder one, which the relative gate must drop, and
// silence after tone, which the absolute gate must (ungated it would read
// -30.8); the range cases are EBU Tech 3342's first four, whose answers are
// differences and so do not care that this meter has one channel where
// theirs has two. The gating cases get 0.1: the few blocks straddling an
// edge are partly tone and pass the gates, as the standard has them do.
//
// The punch figures are held to EBU Tech 3341's constancy signals: a tone
// alternating 0.18s at -20 and 0.22s at -30 dBFS has a period of exactly one
// momentary window, so every 400ms block reads -23.0 LUFS, and 1.34s against
// 1.66s does the same for the 3s short-term window. Theirs are stereo, so
// this meter's one channel takes them 3 dB hotter to read the same. A steady
// sine has every maximum equal to its integrated loudness, and a peak 3.01
// dB above it, which is what PSR has to report.
function selftest() {
  const tone = (rate, parts) => {
    const n = parts.reduce((a, [, sec]) => a + Math.round(sec * rate), 0);
    const d = new Float32Array(n);
    let i = 0;
    for (const [db, sec] of parts) {
      const amp = Math.pow(10, db / 20);
      for (const end = i + Math.round(sec * rate); i < end; i++) d[i] = amp * Math.sin(2 * Math.PI * 997 * i / rate);
    }
    return d;
  };
  const times = (k, parts) => Array.from({ length: k }, () => parts).flat();
  const cases = [
    ['full-scale 997Hz sine, 48k', () => loudnessOf(tone(48000, [[0, 10]]), 48000).integrated, -3.01, 0.05],
    ['full-scale 997Hz sine, 44.1k', () => loudnessOf(tone(44100, [[0, 10]]), 44100).integrated, -3.01, 0.05],
    ['-20 dB sine, 44.1k', () => loudnessOf(tone(44100, [[-20, 10]]), 44100).integrated, -23.01, 0.05],
    ['relative gate: -33/-20/-33 dB, 10/60/10s', () => loudnessOf(tone(44100, [[-33, 10], [-20, 60], [-33, 10]]), 44100).integrated, -23.01, 0.1],
    ['absolute gate: -20 dB then silence', () => loudnessOf(tone(44100, [[-20, 10], [-200, 50]]), 44100).integrated, -23.01, 0.1],
    ['Tech 3342 #1: -20 then -30 dB, 20s each', () => loudnessOf(tone(44100, [[-20, 20], [-30, 20]]), 44100).lra, 10, 1],
    ['Tech 3342 #2: -20 then -15 dB', () => loudnessOf(tone(44100, [[-20, 20], [-15, 20]]), 44100).lra, 5, 1],
    ['Tech 3342 #3: -40 then -20 dB', () => loudnessOf(tone(44100, [[-40, 20], [-20, 20]]), 44100).lra, 20, 1],
    ['Tech 3342 #4: -50/-35/-20/-35/-50 dB', () => loudnessOf(tone(44100, [[-50, 20], [-35, 20], [-20, 20], [-35, 20], [-50, 20]]), 44100).lra, 15, 1],
    ['-20 dB sine: max momentary', () => loudnessOf(tone(44100, [[-20, 10]]), 44100).momentaryMax, -23.01, 0.05],
    ['-20 dB sine: momentary 95th percentile', () => loudnessOf(tone(44100, [[-20, 10]]), 44100).momentaryP95, -23.01, 0.05],
    ['-20 dB sine: max short-term', () => loudnessOf(tone(44100, [[-20, 10]]), 44100).shortTermMax, -23.01, 0.05],
    ['-20 dB sine: PSR, peak less max short-term', () => -20 - loudnessOf(tone(44100, [[-20, 10]]), 44100).shortTermMax, 3.01, 0.05, 'dB'],
    ['Tech 3341 momentary: 0.18/0.22s, max M', () => loudnessOf(tone(44100, times(25, [[-17, 0.18], [-27, 0.22]])), 44100).momentaryMax, -23, 0.1],
    ['Tech 3341 momentary: 0.18/0.22s, 95th pct', () => loudnessOf(tone(44100, times(25, [[-17, 0.18], [-27, 0.22]])), 44100).momentaryP95, -23, 0.1],
    ['Tech 3341 short-term: 1.34/1.66s, max S', () => loudnessOf(tone(44100, times(5, [[-17, 1.34], [-27, 1.66]])), 44100).shortTermMax, -23, 0.1],
    ['the same, max M is the louder tone', () => loudnessOf(tone(44100, times(5, [[-17, 1.34], [-27, 1.66]])), 44100).momentaryMax, -20.01, 0.1],
  ];
  const [shelf, hp] = kWeighting(48000);
  const published = [1.53512485958697, -2.69169618940638, 1.19839281085285, -1.69065929318241, 0.73248077421585, -1.99004745483398, 0.99007225036621];
  const ours = [...shelf.b, ...shelf.a, ...hp.a];
  const coeffErr = Math.max(...ours.map((c, i) => Math.abs(c - published[i])));
  const out = [''];
  out.push('driftloom loudness meter self-test');
  let failed = 0;
  const line = (label, got, want, tol, unit) => {
    const ok = Math.abs(got - want) <= tol;
    if (!ok) failed++;
    out.push(`  ${ok ? 'pass' : 'FAIL'}  ${label.padEnd(44)} ${got.toFixed(3).padStart(8)} ${unit}  (want ${want} +/- ${tol})`);
  };
  line('K-weighting against the published 48k filter', coeffErr, 0, 1e-8, '  ');
  for (const [label, fn, want, tol, unit] of cases) line(label, fn(), want, tol, unit || (label.startsWith('Tech 3342') ? 'LU' : 'LUFS'));
  out.push('');
  out.push(failed ? `  ${failed} failed` : '  all passed');
  out.push('');
  return { text: out.join('\n'), failed };
}

// Radix-2 FFT, in place.
function fft(re, im) {
  const n = re.length;
  for (let i = 1, j = 0; i < n; i++) {
    let bit = n >> 1;
    for (; j & bit; bit >>= 1) j ^= bit;
    j ^= bit;
    if (i < j) { const tr = re[i]; re[i] = re[j]; re[j] = tr; const ti = im[i]; im[i] = im[j]; im[j] = ti; }
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

// IEC 61672 A-weighting, as a linear magnitude factor.
function aWeight(f) {
  if (f <= 0) return 0;
  const f2 = f * f;
  const num = 12194 * 12194 * f2 * f2;
  const den = (f2 + 20.6 * 20.6)
    * Math.sqrt((f2 + 107.7 * 107.7) * (f2 + 737.9 * 737.9))
    * (f2 + 12194 * 12194);
  return (num / den) * Math.pow(10, 2.0 / 20);
}

// A-weighted energy in 2-5kHz and in total, for one rendered note.
//
// Summed periodograms across the whole note, so the figure is over the
// energy that actually leaves the voice -- attack included, which is where
// a struck voice does its brightest work.
function presenceOf(d, rate) {
  const N = 4096;
  const re = new Float64Array(N);
  const im = new Float64Array(N);
  const power = new Float64Array(N / 2);
  const hop = N / 2;
  for (let start = 0; start + N <= d.length; start += hop) {
    for (let i = 0; i < N; i++) {
      re[i] = d[start + i] * (0.5 - 0.5 * Math.cos(2 * Math.PI * i / N));
      im[i] = 0;
    }
    fft(re, im);
    for (let i = 0; i < N / 2; i++) power[i] += re[i] * re[i] + im[i] * im[i];
  }
  const df = rate / N;
  let band = 0;
  let total = 0;
  for (let i = 1; i < N / 2; i++) {
    const f = i * df;
    if (f > 20000) break;
    const w = aWeight(f);
    const p = power[i] * w * w;
    total += p;
    if (f >= 2000 && f <= 5000) band += p;
  }
  return { band, total };
}


// FNV-1a over a string: a note's seed comes from what it is (layer, voice,
// pitch, velocity, repeat) and not from where it falls in a run.
function seedFor(key) {
  let h = 0x811c9dc5;
  for (let i = 0; i < key.length; i++) { h ^= key.charCodeAt(i); h = Math.imul(h, 0x01000193); }
  return h >>> 0;
}

// A render's fingerprint: FNV-1a over the words of its float samples, from
// `from` to `to`. The core's renders are deterministic, so the same
// fingerprint says the same bits.
function hashOf(d, from = 0, to = d.length) {
  const words = new Uint32Array(d.buffer, d.byteOffset, d.length);
  let h = 0x811c9dc5;
  for (let i = from; i < to; i++) { h ^= words[i]; h = Math.imul(h, 0x01000193); }
  return (h >>> 0).toString(16).padStart(8, '0');
}

// The largest sample, as a linear figure.
function peakOf(d) {
  let p = 0;
  for (let i = 0; i < d.length; i++) { const a = Math.abs(d[i]); if (a > p) p = a; }
  return p;
}

export { kWeighting, loudnessOf, selftest, fft, aWeight, presenceOf, seedFor, hashOf, peakOf };

if (process.argv[1] && process.argv[1].endsWith('meter.mjs') && process.argv.includes('--selftest')) {
  const { text, failed } = selftest();
  console.log(text);
  process.exit(failed ? 1 : 0);
}
