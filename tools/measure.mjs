// Offline audio measurement, on the player (queue item 96).
//
//   node tools/measure.mjs [--n 20] [--seed 1] [options]     the corpus report
//   node tools/measure.mjs --voice all|a,b [options]         the voice probe
//   node tools/measure.mjs --refusals [--n 100] [options]    the voice budget
//   node tools/measure.mjs --endings                         every note fades out
//
// This is the audio half of roadmap item 1; `tools/stats.mjs` is the
// generation half. It plays loops from their seeds on the core's player,
// js/dlplayer.wasm in Node, the module the app plays, and reports what came
// out: K-weighted loudness (ITU-R BS.1770) and loudness range, peak, the
// punch figures, what the master chain did, and each layer's level as it
// leaves its channel, so the balance is measured rather than read off the
// gain table. One way of playing is measured: the player decides the notes
// and hands them to the core as it does on a phone, and every target makes
// the same samples from a seed (tools/seeds.sh), so there is no browser
// here and nothing to seed: same options, same numbers, to the bit.
//
// Until item 96 this tool drove the page's Engine and Synth in headless
// Chromium (that tool retired with the JavaScript synth, queue item 97).
// For the same loops the two agree to 0.01 dB in every figure
// both report (the PR of item 96 has the table).
//
// The voice probe, the request as text and the budget's counts need the
// tools' build of the player (tools/build-core.sh --tools, made on first
// use; tools/play-lib.mjs).

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { Rng } from '../js/rng.js';
import { VIBE_IDS } from '../js/vibes.js';
import { loudnessOf, presenceOf, seedFor, hashOf, peakOf } from './meter.mjs';
import { ROOT, QUANTUM, LAYERS, player, render, aboutOf, probe, probeVoices, budgetOf } from './play-lib.mjs';

const USAGE = `
driftloom offline audio measurement, on the player

  node tools/measure.mjs [options]

  --n <count>       how many loops to play                 (default 20)
  --seed <number>   corpus seed; same seed, same loops       (default 1)
  --seconds <s>     how long each loop plays; 'pass' plays one pass and
                    3 s more, 60 s at most             (default pass)
  --wander <amount> "Let the loop wander", 0 to 2.4; 0 plays the loop as
                    composed, 1 is the app's default       (default 1)
  --rate <hz>       sample rate                          (default 44100)
  --quality <q>     'full' or 'lite'                       (default full)
  --stereo          the stereo render, each loop held against its mono
                    render: whether each side is mono's bit for bit, the
                    fold's loudness and residual against mono, and the side
                    against the mid
  --vibes <a,b>     draw every loop with these vibes
  --vibe-mix        draw each loop with 1, 3, 5 or 10 random vibes in turn
                    (queue item 38a)
  --request <text>  draw every loop with this request, as text:
                    "glacial=0.7,bpm=70-85" (vibes are a request too)
  --profile <id>    keep only loops that profile leads, drawing from the
                    same corpus until --n of them are in
  --moods <a,b>     keep only loops whose top mood is one of these
  --json <file>     also write every figure, per loop or per note, as JSON
  --voice <names>   per-voice probe instead of the corpus report; 'all'
                    probes every voice the probe knows, by layer
  --note <seconds>  how long each probed note is held        (default 1.6)
  --reps <count>    with --voice, renders of each note, each with its own
                    draws                                      (default 3)
  --stride <n>      with --voice, probe every nth note of the window
                                                               (default 1)
  --refusals        what the voice budget refused and let go, layer by
                    layer, over the corpus                (--n default 100)
  --endings         check that every note of every voice fades out
  --help            this

  The corpus report: integrated loudness (LUFS) and loudness range (LRA)
  per loop, per ITU-R BS.1770 and EBU Tech 3342 (the mono bus as one
  channel at weight 1.0), sample peak, crest (peak less integrated), the
  punch figures (the loudest momentary and short-term loudness, PSR), the
  deepest the bus compressor and the ceiling reached, and each layer's
  loudness as it leaves its channel gain (dry: sends and returns are
  shared, so a tail can't be put back on the layer that sent it). Then the
  catalog's median and each profile's against it: a profile's 'level' is
  solved so its loops land on that median (--profile <id> for more of one).

  --voice reports, for each voice in each layer that plays it, its
  K-weighted loudness note by note across the layer's two octaves at
  velocities 0.4 and 0.8, through that layer's own path in the player
  (the sequencer is handed a loop of one note), in LU against kalimba as a
  melody; and for melody voices the share of A-weighted energy in 2-5 kHz
  at velocity 0.8, the coffee-shop test: a new voice sits at or below the
  flute's.

  --refusals: a voice call is one note as a layer asks for it; it counts
  as refused if the budget answered no to any part of it. "Let go" are
  older notes of the same layer faded early to make room (queue item 85c).

  --endings plays one note of every voice at 0.1, 0.4 and 1.6 s (a melody
  voice at 30 ms too, a grace) and reads the most sudden fall in its
  level. Over 12 dB the note was cut off, and the run exits non-zero.
`;

// ------------------------------------------------------------ arguments

function fail(message) {
  console.error(`measure: ${message}`);
  process.exit(2);
}

function parseArgs(argv) {
  const opts = { n: null, seed: 1, seconds: 'pass', wander: 1, rate: 44100, quality: 'full', stereo: false, request: '', vibeMix: false, profile: null, moods: null,
    json: null, voice: null, note: 1.6, reps: 3, stride: 1, refusals: false, endings: false };
  for (let i = 0; i < argv.length; i++) {
    const value = () => (i + 1 < argv.length ? argv[++i] : fail(`${argv[i]} needs a value`));
    switch (argv[i]) {
      case '--n': opts.n = Number(value()); break;
      case '--seed': opts.seed = Number(value()); break;
      case '--seconds': { const v = value(); opts.seconds = v === 'pass' ? v : Number(v); break; }
      case '--wander': opts.wander = Number(value()); break;
      case '--rate': opts.rate = Number(value()); break;
      case '--quality': opts.quality = value(); break;
      case '--stereo': opts.stereo = true; break;
      case '--vibes': opts.request = [opts.request, value()].filter(Boolean).join(','); break;
      case '--request': opts.request = [opts.request, value()].filter(Boolean).join(','); break;
      case '--vibe-mix': opts.vibeMix = true; break;
      case '--profile': opts.profile = value(); break;
      case '--moods': opts.moods = value().split(','); break;
      case '--json': opts.json = value(); break;
      case '--voice': opts.voice = value().split(','); break;
      case '--note': opts.note = Number(value()); break;
      case '--reps': opts.reps = Number(value()); break;
      case '--stride': opts.stride = Number(value()); break;
      case '--refusals': opts.refusals = true; break;
      case '--endings': opts.endings = true; break;
      case '--help': case '-h': console.log(USAGE); process.exit(0); break;
      default: fail(`no such option: ${argv[i]} (--help lists them)`);
    }
  }
  if (!['full', 'lite'].includes(opts.quality)) fail("--quality is 'full' or 'lite'");
  if (opts.n == null) opts.n = opts.refusals ? 100 : 20;
  return opts;
}

// A line of progress, on a terminal only (a log gets none).
const progress = (text) => { if (process.stderr.isTTY) process.stderr.write(`\r  ${text.padEnd(24)}${text ? '' : '\r'}`); };
const mean = (xs) => xs.reduce((a, b) => a + b, 0) / Math.max(1, xs.length);
const median = (xs) => {
  const s = xs.filter(Number.isFinite).sort((a, b) => a - b);
  return s.length ? s[Math.floor((s.length - 1) / 2)] : NaN;
};
const db = (x) => (x > 0 ? 20 * Math.log10(x) : -Infinity);
const f1 = (x, w = 7) => (Number.isFinite(x) ? x.toFixed(1) : '-').padStart(w);
const f2 = (x, w = 8) => (Number.isFinite(x) ? x.toFixed(2) : '-').padStart(w);

// Each channel's gain into the mix, by layer: the core's own table
// (`CHANNEL_GAIN` in core/src/player/synth.rs, in the core's channel order).
function channelGains() {
  const source = fs.readFileSync(path.join(ROOT, 'core', 'src', 'player', 'synth.rs'), 'utf8');
  const m = source.match(/pub const CHANNEL_GAIN: \[f64; 5\] = \[([^\]]+)\]/);
  if (!m) fail('core/src/player/synth.rs has no CHANNEL_GAIN table');
  const g = m[1].split(',').map(Number);
  return { melody: g[0], chords: g[1], bass: g[2], texture: g[3], drums: g[4] };
}

// ------------------------------------------------------------- the corpus

// What the player says of the loop it made, as fields.
function describe(x) {
  const about = Object.fromEntries(aboutOf(x).split('\n').filter((l) => l.includes('=')).map((l) => { const i = l.indexOf('='); return [l.slice(0, i), l.slice(i + 1)]; }));
  const top = (text) => (text || '').split('|').map((p) => p.split(':')).sort((a, b) => Number(b[1]) - Number(a[1]))[0]?.[0] || '?';
  return { name: about.name, profile: top(about.mix), mood: top(about.moods), bpm: Number(about.bpm), bars: Number(about.bars), stepsPerBar: Number(about.steps_per_bar), metre: about.metre };
}

// A player with the loop of `seed` made, and what it says of the loop. The
// load is spread over the blocks that follow it, so blocks are rendered
// until the loop is there: they are kept (`head`), the render's first.
function made(seed, way) {
  const x = player(seed, way);
  const head = [];
  let frame = 0;
  while (x.dl_play_state() !== 3 && frame < 44100 * 4) { head.push(render(x, QUANTUM, { taps: true, from: frame })); frame += QUANTUM; }
  if (x.dl_play_state() !== 3) throw new Error(`seed ${seed}: the player never made its loop`);
  return { x, frame, head, about: describe(x) };
}

// --vibe-mix (queue item 38a): each loop drawn with a random selection of
// 1, 3, 5 or 10 vibes in turn, from a stream of its own, so the loops'
// seeds are the corpus's own.
function vibeMixFor(seed, i) {
  const r = new Rng(((seed ^ 0x56494245) + Math.imul(i + 1, 0x9e3779b1)) >>> 0 || 1);
  return r.shuffle(VIBE_IDS).slice(0, [1, 3, 5, 10][i % 4]);
}

// The corpus, drawn as it always was: loops from the corpus seed in order,
// and with --profile or --moods only the ones that fit, until n are in.
function* corpus(opts, way) {
  const master = new Rng(opts.seed);
  let found = 0;
  // Capped, so a profile that never leads cannot spin forever.
  for (let tries = 0; found < opts.n && tries < opts.n * 1000; tries++) {
    const seed = master.seed32();
    const request = opts.vibeMix ? [way.request, vibeMixFor(opts.seed, tries).join(',')].filter(Boolean).join(',') : way.request;
    const loop = made(seed, { ...way, request });
    if (opts.profile && loop.about.profile !== opts.profile) continue;
    if (opts.moods && !opts.moods.includes(loop.about.mood)) continue;
    found++;
    yield { seed, request, ...loop };
  }
}

// How long a loop plays: --seconds, or one pass and 3 s more, 60 s at most.
const secondsFor = (opts, about) => (opts.seconds === 'pass'
  ? Math.min(60, about.bars * about.stepsPerBar * (60 / about.bpm / 4) + 3) : opts.seconds);

const ledBy = (opts) => (opts.profile ? `, loops led by ${opts.profile}` : '') + (opts.request ? `, asked for ${opts.request}` : '') + (opts.vibeMix ? ', with 1, 3, 5 or 10 random vibes' : '') + (opts.moods ? `, top mood ${opts.moods.join(' or ')}` : '');

// One loop played: its figures and fingerprints.
function playLoop(loop, opts, gains) {
  const { x, frame, about } = loop;
  if (frame > Math.ceil(secondsFor(opts, about) * opts.rate)) throw new Error(`${about.name}: the loop took longer to make than it was asked to play`);
  const seconds = secondsFor(opts, about);
  const frames = Math.ceil(seconds * opts.rate);
  // The blocks rendered while the loop was made are the render's first.
  const r = render(x, frames - frame, { taps: true, from: frame });
  const whole = (pick) => {
    const a = new Float32Array(frames);
    loop.head.forEach((h, i) => a.set(pick(h).subarray(0, Math.min(QUANTUM, frames - i * QUANTUM)), i * QUANTUM));
    a.set(pick(r), frame);
    return a;
  };
  const out = r.out.map((_, c) => whole((part) => part.out[c]));
  if (x.dl_dropped() || x.dl_late() || x.dl_missed()) throw new Error(`${about.name}: the core dropped ${x.dl_dropped()}, took ${x.dl_late()} late and missed ${x.dl_missed()} notes`);
  const mono = out[0];
  const l = loudnessOf(mono, opts.rate);
  const peak = Math.max(...out.map(peakOf));
  const layers = {};
  const hashes = { layers: {} };
  for (const layer of LAYERS) {
    const tap = whole((part) => part.taps[layer]);
    hashes.layers[layer] = hashOf(tap);
    const level = new Float32Array(tap.length);
    for (let i = 0; i < tap.length; i++) level[i] = tap[i] * gains[layer];
    const ll = loudnessOf(level, opts.rate);
    layers[layer] = { lufs: Number.isFinite(ll.integrated) ? ll.integrated : null, momentaryMax: ll.momentaryMax };
  }
  if (out.length === 2) { hashes.left = hashOf(out[0]); hashes.right = hashOf(out[1]); } else hashes.out = hashOf(mono);
  return { name: about.name, seed: loop.seed, profile: about.profile, mood: about.mood, bpm: about.bpm, bars: about.bars, metre: about.metre, seconds, frames,
    lufs: l.integrated, lra: l.lra, peak, momentaryMax: l.momentaryMax, shortTermMax: l.shortTermMax,
    comp: x.dl_deepest(0), ceiling: x.dl_deepest(1), layers, hashes, out };
}

// A stereo loop against its mono render (queue item 64): whether each side
// is mono's bit for bit, the fold ((L + R) / 2) against mono, and the side
// ((L - R) / 2) against the mid.
function spaceOf(stereo, mono, rate) {
  const [L, R] = stereo;
  const n = L.length;
  const fold = new Float32Array(n); const resid = new Float32Array(n); const side = new Float32Array(n);
  let exact = true;
  for (let i = 0; i < n; i++) {
    fold[i] = (L[i] + R[i]) / 2;
    side[i] = (L[i] - R[i]) / 2;
    resid[i] = fold[i] - mono[i];
    if (L[i] !== mono[i] || R[i] !== mono[i]) exact = false;
  }
  const rms = (d) => { let s = 0; for (let i = 0; i < d.length; i++) s += d[i] * d[i]; return Math.sqrt(s / d.length); };
  return { exact, foldLU: loudnessOf(fold, rate).integrated - loudnessOf(mono, rate).integrated, foldResid: db(rms(resid) / rms(mono)), sideMid: db(rms(side) / rms(fold)) };
}

function measureCorpus(opts) {
  const gains = channelGains();
  const way = { rate: opts.rate, full: opts.quality === 'full', wander: opts.wander, request: opts.request };
  const rows = [];
  for (const loop of corpus(opts, { ...way, layout: opts.stereo ? 2 : 1 })) {
    const row = playLoop(loop, opts, gains);
    if (opts.stereo) {
      const mono = playLoop(made(loop.seed, { ...way, request: loop.request, layout: 1 }), opts, gains);
      row.space = spaceOf(row.out, mono.out[0], opts.rate);
      row.monoLufs = mono.lufs;
    }
    delete row.out;
    rows.push(row);
    progress(`${rows.length}/${opts.n}`);
  }
  progress('');
  return rows;
}

function reportCorpus(rows, opts) {
  const out = [''];
  out.push('driftloom offline audio measurement, on the player');
  out.push(`  ${rows.length} loops (corpus seed ${opts.seed}${ledBy(opts)}), ${opts.seconds === 'pass' ? 'one pass and 3 s each (60 s at most)' : `${opts.seconds} s each`}, wander ${opts.wander}, ${opts.quality}, ${opts.stereo ? 'stereo' : 'mono'}, ${(opts.rate / 1000).toFixed(1)}k`);
  out.push('');
  out.push(`  ${'loop'.padEnd(20)} ${'profile'.padEnd(10)} bpm    LUFS    LRA   peak  crest   maxM   maxS    PSR   comp   ceil | ${LAYERS.map((l) => l.slice(0, 6).padStart(6)).join(' ')}`);
  for (const r of rows) {
    const pk = db(r.peak);
    out.push(`  ${r.name.padEnd(20)} ${r.profile.padEnd(10)} ${String(Math.round(r.bpm)).padStart(3)} ${f2(r.lufs)} ${f1(r.lra, 6)} ${f1(pk, 6)} ${f1(pk - r.lufs, 6)} ${f1(r.momentaryMax, 6)} ${f1(r.shortTermMax, 6)} ${f1(pk - r.shortTermMax, 6)} ${f1(r.comp, 6)} ${f1(r.ceiling, 6)} | ${LAYERS.map((l) => f1(r.layers[l].lufs ?? NaN, 6)).join(' ')}`);
  }
  out.push('');
  const cat = median(rows.map((r) => r.lufs));
  out.push(`  the catalog: median ${f2(cat, 0)} LUFS, mean ${f2(mean(rows.map((r) => r.lufs)), 0)}, quietest ${f2(Math.min(...rows.map((r) => r.lufs)), 0)}, loudest ${f2(Math.max(...rows.map((r) => r.lufs)), 0)}; loudest peak ${f1(db(Math.max(...rows.map((r) => r.peak))), 0)} dBFS`);
  out.push(`  each layer's median, where it plays: ${LAYERS.map((l) => `${l} ${f1(median(rows.map((r) => r.layers[l].lufs ?? NaN)), 0)}`).join(', ')}`);
  out.push('');
  out.push('  by profile (median LUFS, and against the catalog)');
  const by = {};
  for (const r of rows) (by[r.profile] ||= []).push(r.lufs);
  for (const [p, xs] of Object.entries(by).sort((a, b) => b[1].length - a[1].length)) {
    out.push(`    ${p.padEnd(12)} ${String(xs.length).padStart(4)} loops  ${f2(median(xs))}  ${(median(xs) - cat >= 0 ? '+' : '') + (median(xs) - cat).toFixed(2)}`);
  }
  if (opts.stereo) {
    out.push('');
    out.push('  stereo against mono (queue item 64): the fold (L+R)/2 in LU against mono, its residual and the side, in dB');
    out.push(`    fold: median ${f2(median(rows.map((r) => r.space.foldLU)), 0)} LU, worst ${f2(Math.max(...rows.map((r) => Math.abs(r.space.foldLU))), 0)}; residual: worst ${f1(Math.max(...rows.map((r) => r.space.foldResid)), 0)} dB under mono; side under mid: median ${f1(median(rows.map((r) => r.space.sideMid)), 0)} dB, widest ${f1(Math.max(...rows.map((r) => r.space.sideMid)), 0)}; ${rows.filter((r) => r.space.exact).length} of ${rows.length} loops mono on each side`);
  }
  out.push('');
  return out.join('\n');
}

// ------------------------------------------------------ the voice probe
//
// The share of a note's A-weighted energy between 2 and 5 kHz: where the
// ear is most sensitive and a voice reads as harsh. A comparison between
// voices, not an absolute: read it beside a voice nobody complains about.
// Each layer is probed across the two octaves it plays in (a 3,000-loop
// corpus, 5th to 95th percentile: melody 56-71, chords 43-65, bass 28-39,
// textures 67-89).
const PROBE_WINDOWS = { melody: [55, 79], chords: [43, 67], bass: [28, 52], texture: [67, 91] };
const PROBE_VELOCITIES = [0.4, 0.8];
const TONE_VELOCITY = 0.8;
const PROBE_REFERENCE = { layer: 'melody', voice: 'kalimba' };
const FAMILY_LIMIT = 3;

function probeJobs(names) {
  const known = probeVoices();
  const all = names.length === 1 && names[0] === 'all';
  const jobs = [];
  const add = (layer, voice, tone) => {
    if (jobs.some((j) => j.layer === layer && j.voice === voice)) return;
    const [low, high] = PROBE_WINDOWS[layer];
    jobs.push({ layer, voice, low, high, tone });
  };
  for (const layer of ['melody', 'chords', 'bass', 'texture']) {
    for (const voice of known[layer]) if (all || names.includes(voice)) add(layer, voice, layer === 'melody');
  }
  for (const voice of names) if (!all && !jobs.some((j) => j.voice === voice)) fail(`the probe has no voice '${voice}'`);
  add(PROBE_REFERENCE.layer, PROBE_REFERENCE.voice, false);
  return jobs;
}

function measureVoices(opts) {
  const rows = [];
  const jobs = probeJobs(opts.voice);
  for (const job of jobs) {
    const row = { layer: job.layer, voice: job.voice, low: job.low, high: job.high, notes: {}, tone: null };
    for (const vel of PROBE_VELOCITIES) {
      const notes = []; const tone = [];
      for (let midi = job.low; midi <= job.high; midi += opts.stride) {
        // Repeats are averaged as energy, then read back as loudness.
        let energy = 0; let band = 0; let total = 0;
        for (let rep = 0; rep < opts.reps; rep++) {
          const d = probe({ layer: job.layer, voice: job.voice, midi, vel, seconds: opts.note, rate: opts.rate, vowel: (midi - job.low) % 4,
            seed: seedFor([job.layer, job.voice, midi, vel.toFixed(2), rep].join('|')) });
          const lufs = loudnessOf(d, opts.rate).integrated;
          energy += Number.isFinite(lufs) ? Math.pow(10, lufs / 10) : 0;
          if (job.tone && vel === TONE_VELOCITY) { const p = presenceOf(d, opts.rate); band += p.band; total += p.total; }
        }
        notes.push({ midi, lufs: energy > 0 ? 10 * Math.log10(energy / opts.reps) : -Infinity });
        tone.push({ midi, share: total > 0 ? band / total : 0 });
      }
      row.notes[vel] = notes;
      if (job.tone && vel === TONE_VELOCITY) row.tone = tone;
    }
    rows.push(row);
    progress(`${rows.length}/${jobs.length} voices`);
  }
  progress('');
  return { rows, gains: channelGains() };
}

function reportVoices(data, opts) {
  const out = [''];
  const ref = data.rows.find((r) => r.layer === PROBE_REFERENCE.layer && r.voice === PROBE_REFERENCE.voice);
  const level = (r, vel) => median(r.notes[vel].map((n) => n.lufs));
  out.push('driftloom per-voice probe, on the player');
  out.push(`  ${opts.note} s notes, every ${opts.stride === 1 ? '' : `${opts.stride}rd `}note of each layer's two octaves, ${opts.reps} render(s) each at ${(opts.rate / 1000).toFixed(1)}k, dry, through the layer's own path`);
  out.push('  level: K-weighted, the median across the window, in LU against kalimba as a melody at the same velocity');
  out.push('  tone: the A-weighted share of energy in 2-5 kHz at velocity 0.8 (melody voices), mean and worst note');
  out.push('');
  for (const layer of ['melody', 'chords', 'bass', 'texture']) {
    const rows = data.rows.filter((r) => r.layer === layer);
    if (!rows.length) continue;
    const med = median(rows.map((r) => level(r, 0.4)));
    out.push(`  ${layer} (the layer's median at 0.4: ${f2(med, 0)} LUFS; the channel's gain ${data.gains[layer]})`);
    out.push(`    ${'voice'.padEnd(12)}  LUFS@0.4   vs kal  vs layer  LUFS@0.8   vs kal   tone   worst`);
    for (const r of rows) {
      const lo = level(r, 0.4); const hi = level(r, 0.8);
      const shares = r.tone ? r.tone.map((t) => t.share) : null;
      const far = Math.abs(lo - med) > FAMILY_LIMIT ? ' !' : '';
      out.push(`    ${r.voice.padEnd(12)} ${f2(lo, 9)} ${f2(lo - level(ref, 0.4))} ${f2(lo - med, 9)}${far.padEnd(2)}${f2(hi, 9)} ${f2(hi - level(ref, 0.8))} ${shares ? `${(100 * mean(shares)).toFixed(1)}%`.padStart(6) : '     -'} ${shares ? `${(100 * Math.max(...shares)).toFixed(1)}%`.padStart(7) : '      -'}`);
    }
    out.push('');
  }
  out.push(`  '!' marks a voice more than ${FAMILY_LIMIT} LU from its layer's median. A new voice's level is calibrated to that median at 0.4 s notes,`);
  out.push("  and a melody voice's tone share sits at or below the flute's (the coffee-shop test).");
  out.push('');
  return out.join('\n');
}

// ------------------------------------------------------ budget refusals

function measureRefusals(opts) {
  const way = { rate: opts.rate, full: opts.quality === 'full', wander: opts.wander, request: opts.request, tools: true };
  const rows = [];
  for (const loop of corpus(opts, way)) {
    const frames = Math.ceil(secondsFor(opts, loop.about) * opts.rate);
    render(loop.x, frames - loop.frame, { from: loop.frame });
    rows.push({ name: loop.about.name, seed: loop.seed, profile: loop.about.profile, ...budgetOf(loop.x) });
    progress(`${rows.length}/${opts.n}`);
  }
  progress('');
  return rows;
}

function reportRefusals(rows, opts) {
  const out = [''];
  out.push('driftloom voice budget refusals, on the player');
  out.push(`  ${rows.length} loops (corpus seed ${opts.seed}${ledBy(opts)}), ${opts.quality}, wander ${opts.wander}: voice calls asked, refused room, and older notes let go for newer ones`);
  out.push('');
  out.push(`    ${'layer'.padEnd(8)} ${'asked'.padStart(8)} ${'refused'.padStart(8)} ${'share'.padStart(7)} ${'let go'.padStart(8)}   loops refusing any`);
  for (const l of LAYERS) {
    const asked = rows.reduce((a, r) => a + r.asked[l], 0);
    const refused = rows.reduce((a, r) => a + r.refused[l], 0);
    const stolen = rows.reduce((a, r) => a + r.stolen[l], 0);
    out.push(`    ${l.padEnd(8)} ${String(asked).padStart(8)} ${String(refused).padStart(8)} ${(asked ? `${(100 * refused / asked).toFixed(2)}%` : '-').padStart(7)} ${String(stolen).padStart(8)}   ${rows.filter((r) => r.refused[l] > 0).length}`);
  }
  out.push('');
  const melody = rows.filter((r) => r.asked.melody).map((r) => r.refused.melody / r.asked.melody);
  out.push(`  melody, loop by loop: mean refusal ${(100 * mean(melody)).toFixed(2)}%, worst ${(100 * Math.max(0, ...melody)).toFixed(1)}%, ${melody.filter((x) => x > 0.001).length} of ${melody.length} loops losing any of the tune`);
  const worst = rows.map((r) => ({ r, share: r.asked.melody ? r.refused.melody / r.asked.melody : 0 })).filter((w) => w.share > 0).sort((a, b) => b.share - a.share).slice(0, 5);
  for (const w of worst) out.push(`    ${w.r.name} (${w.r.profile}, seed ${w.r.seed}): ${(100 * w.share).toFixed(1)}%`);
  out.push('');
  return out.join('\n');
}

// --------------------------------------------------------------- endings
//
// Whether a note was cut off: the most sudden fall in its level anywhere.
// At every point the level just before is set against the level just
// after, in windows of two periods of the note, so a low note's own
// waveform does not read as a fall. A cut is a fall that happens at once
// and then stops; a decay keeps on falling, and a bell's beating dips and
// comes straight back. So a fall scores what is left of it after the fall
// in the next window and anything it climbs back within 30 ms. Only points
// within 30 dB of the peak count.
function endingOf(d, rate, midi) {
  const hz = 440 * Math.pow(2, (midi - 69) / 12);
  const w = Math.max(Math.round(0.002 * rate), Math.round((2 * rate) / hz));
  const back = Math.round(0.03 * rate);
  const sq = new Float64Array(d.length + 1);
  for (let i = 0; i < d.length; i++) sq[i + 1] = sq[i] + d[i] * d[i];
  const rms = (a, b) => Math.sqrt(Math.max(0, sq[b] - sq[a]) / (b - a));
  let peak = 0;
  for (let i = 0; i + w <= d.length; i += w) peak = Math.max(peak, rms(i, i + w));
  if (!peak) return { silent: true };
  const floor = peak * 1e-6;
  const dbOf = (x, y) => 20 * Math.log10(Math.max(x, floor) / Math.max(y, floor));
  let sudden = 0;
  let at = 0;
  const hop = Math.max(1, w >> 2);
  for (let n = w; n + 2 * w + back <= d.length; n += hop) {
    const before = rms(n - w, n);
    if (before < peak * 0.0316) continue;
    const after = rms(n, n + w);
    const fall = dbOf(before, after);
    if (fall <= 0) continue;
    const onward = Math.max(0, dbOf(after, rms(n + w, n + 2 * w)));
    let high = 0;
    for (let k = n + w; k + w <= n + back; k += hop) high = Math.max(high, rms(k, k + w));
    const climb = Math.max(0, dbOf(high, after));
    const score = fall - onward - climb;
    if (score > sudden) { sudden = score; at = n / rate; }
  }
  let last = d.length - 1;
  while (last > 0 && Math.abs(d[last]) < peak * 1e-3) last--;
  return { sudden, at, ringing: last >= d.length - 2 };
}

const ENDING_MIDI = { melody: 67, chords: 55, bass: 40, texture: 79 };
const ENDING_LENGTHS = [0.03, 0.1, 0.4, 1.6];
const ENDING_TAIL = 6;
const ENDING_LIMIT = 12;
const endsBadly = (r) => !r.silent && (r.ringing || r.sudden > ENDING_LIMIT);

function measureEndings(opts) {
  const known = probeVoices();
  const rows = [];
  for (const layer of ['melody', 'chords', 'bass']) {
    for (const voice of known[layer]) {
      for (const dur of layer === 'melody' ? ENDING_LENGTHS : ENDING_LENGTHS.slice(1)) {
        const midi = ENDING_MIDI[layer];
        const d = probe({ layer, voice, midi, vel: 0.7, seconds: dur, rate: opts.rate, tail: ENDING_TAIL, seed: seedFor([layer, voice, midi, dur].join('|')) });
        rows.push({ layer, voice, dur, ...endingOf(d, opts.rate, midi) });
      }
    }
  }
  return rows;
}

function reportEndings(rows) {
  const out = [''];
  out.push('driftloom note endings, on the player');
  out.push(`  one note per voice and layer at ${ENDING_LENGTHS.map((d) => `${d}s`).join(', ')} (the grace, melody only), dry, velocity 0.7;`);
  out.push("  the most sudden fall in each note's level: dB from one window of two periods to the next,");
  out.push(`  beyond the falls either side, within 30 dB of its peak. Over ${ENDING_LIMIT} dB, the note was cut off.`);
  out.push('');
  out.push(`    ${'layer'.padEnd(8)} ${'voice'.padEnd(12)} ${ENDING_LENGTHS.map((d) => `${d}s`.padStart(9)).join('')}`);
  for (const key of [...new Set(rows.map((r) => `${r.layer}|${r.voice}`))]) {
    const [layer, voice] = key.split('|');
    const cells = ENDING_LENGTHS.map((d) => {
      const r = rows.find((x) => x.layer === layer && x.voice === voice && x.dur === d);
      if (!r) return '-  '.padStart(9);
      const text = r.silent ? 'silent' : r.ringing ? 'ringing' : `${r.sudden.toFixed(1)}`;
      return `${text}${endsBadly(r) ? ' !' : '  '}`.padStart(9);
    });
    out.push(`    ${layer.padEnd(8)} ${voice.padEnd(12)} ${cells.join('')}`);
  }
  out.push('');
  const bad = rows.filter(endsBadly);
  out.push(bad.length ? `  ${bad.length} note(s) cut off: ${bad.map((r) => `${r.voice} (${r.layer}) at ${r.dur}s`).join(', ')}` : '  every note of every voice fades out');
  out.push('');
  return out.join('\n');
}

// ----------------------------------------------------------------- main

export { measureCorpus, measureVoices, measureRefusals, measureEndings, channelGains, made, playLoop, spaceOf, PROBE_WINDOWS };

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const opts = parseArgs(process.argv.slice(2));
  let data;
  let failed = false;
  if (opts.endings) {
    data = measureEndings(opts);
    console.log(reportEndings(data));
    failed = data.some(endsBadly);
  } else if (opts.voice) {
    data = measureVoices(opts);
    console.log(reportVoices(data, opts));
  } else if (opts.refusals) {
    data = measureRefusals(opts);
    console.log(reportRefusals(data, opts));
  } else {
    data = { rows: measureCorpus(opts) };
    console.log(reportCorpus(data.rows, opts));
  }
  if (opts.json) fs.writeFileSync(opts.json, `${JSON.stringify(data, null, 1)}\n`);
  process.exit(failed ? 1 : 0);
}
