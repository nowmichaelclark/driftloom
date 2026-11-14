// The FLAC writer's yardsticks (queue item 104a): the core's own encoder
// (core/src/flac/) against the reference decoder, on real renders.
//
//   node tools/flac.mjs                 the whole check (CI's `flac` job,
//                                       tools/gate.sh flac)
//   node tools/flac.mjs --loops N       N corpus loops (20)
//   node tools/flac.mjs --seconds S     each corpus loop's length (30)
//   node tools/flac.mjs --long HOURS    also one loop streamed for HOURS
//                                       (16-bit stereo), its peak memory
//                                       against a minute's, and its file
//                                       tested by the decoder
//
// For the golden note lists (core/golden/*.dlnotes) and the first N loops
// of corpus 20261009, mono and stereo, 16 and 24 bits, 44.1 and 48 kHz:
// `dlrender --flac` writes the file (from a seed it streams, as an export
// will) and `--wav` the float WAV of the same render. Then each file
//
//   - decodes with the reference decoder: `flac` (it checks every frame's
//     CRCs and the MD5 itself: `flac -t`), else `ffmpeg`;
//   - nulls sample for sample against the WAV: every decoded sample is the
//     WAV's float made an integer by the core's rule (`dlcore::flac::quantize`:
//     scaled by 2^(bits-1), rounded half away from zero, held to the range),
//     so within half a step of it where it is not past full scale;
//   - carries the right MD5: STREAMINFO's equals the MD5 of the decoded
//     samples, worked out here, and the length is the render's.
//
// It prints the sizes against a WAV of the same depth, a row a case, and
// the encoder's speed natively (dlrender's own timing of the encoder alone).
// Then the same bytes in WebAssembly: a few corpus loops rendered by the
// tools' build in Node (the player's wasm, `dl_flac_*`), encoded there, must
// be the native file byte for byte; their encoding is timed too. And the C
// host (core/examples/c/seed.c --flac, through include/dlcore.h alone) writes
// the same bytes as dlrender.
//
// Needs Rust (the pinned toolchain), a C compiler, and `flac` or `ffmpeg`
// (dev-only: CI's runner installs `flac`). Exits 1 on any failure.

import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import crypto from 'node:crypto';
import { spawn, spawnSync } from 'node:child_process';
import { ROOT, player, render } from './play-lib.mjs';

const args = process.argv.slice(2);
const opt = (name, fallback) => {
  const i = args.indexOf(name);
  return i >= 0 ? Number(args[i + 1]) : fallback;
};
const LOOPS = opt('--loops', 20);
const SECONDS = opt('--seconds', 30);
const LONG = opt('--long', 0);
const CORPUS = 20261009;
const RATES = [44100, 48000];
const LAYOUTS = [1, 2];
const DEPTHS = [16, 24];
const WASM_LOOPS = 3;

const CORE = path.join(ROOT, 'core');
const DLRENDER = path.join(CORE, 'target', 'release', 'examples', 'dlrender');
const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'dlflac-'));
let failures = 0;
const fail = (what) => {
  failures++;
  console.log(`FAIL ${what}`);
};

function run(cmd, argv, opts = {}) {
  const r = spawnSync(cmd, argv, { encoding: 'utf8', maxBuffer: 1 << 30, ...opts });
  if (r.status !== 0) throw new Error(`${cmd} ${argv.join(' ')} failed:\n${r.stdout}\n${r.stderr}`);
  return r;
}
const has = (cmd) => spawnSync('sh', ['-c', `command -v ${cmd}`]).status === 0;

// Run `jobs` (each argv for dlrender) a few at a time; their stdout lines.
async function parallel(jobs) {
  const out = new Array(jobs.length);
  let next = 0;
  const width = Math.max(1, Math.min(os.cpus().length, 8));
  async function worker() {
    while (next < jobs.length) {
      const i = next++;
      out[i] = await new Promise((resolve, reject) => {
        const p = spawn(DLRENDER, jobs[i]);
        let text = '';
        let err = '';
        p.stdout.on('data', (d) => (text += d));
        p.stderr.on('data', (d) => (err += d));
        p.on('close', (code) => (code === 0 ? resolve(text.trim()) : reject(new Error(`dlrender ${jobs[i].join(' ')}: ${err}`))));
      });
    }
  }
  await Promise.all(Array.from({ length: width }, worker));
  return out;
}

// A WAV dlrender wrote: 32-bit floats after a 44-byte header.
function readWav(file) {
  const b = fs.readFileSync(file);
  if (b.toString('latin1', 0, 4) !== 'RIFF' || b.readUInt16LE(20) !== 3) throw new Error(`${file}: not dlrender's float WAV`);
  const data = b.readUInt32LE(40);
  return { rate: b.readUInt32LE(24), channels: b.readUInt16LE(22), samples: new Float32Array(b.buffer.slice(b.byteOffset + 44, b.byteOffset + 44 + data)) };
}

// The core's rule (dlcore::flac::quantize), in doubles as the core works it.
function quantize(x, bits) {
  const full = 2 ** (bits - 1);
  if (Number.isNaN(x)) return 0;
  const v = x * full;
  const r = v < 0 ? v - 0.5 : v + 0.5;
  return Math.trunc(Math.min(Math.max(r, -full), full - 1));
}

// STREAMINFO's fields.
function streaminfo(file) {
  const b = fs.readFileSync(file, { flag: 'r' }).subarray(0, 42);
  if (b.toString('latin1', 0, 4) !== 'fLaC') throw new Error(`${file}: not FLAC`);
  const si = b.subarray(8);
  const rate = (si[10] << 12) | (si[11] << 4) | (si[12] >> 4);
  const channels = ((si[12] >> 1) & 7) + 1;
  const bits = (((si[12] & 1) << 4) | (si[13] >> 4)) + 1;
  const total = (si[13] & 0x0f) * 2 ** 32 + si.readUInt32BE(14);
  return { rate, channels, bits, total, md5: si.subarray(18, 34).toString('hex') };
}

// Decode with the reference decoder to raw little-endian signed PCM of the
// file's own width (2 or 3 bytes a sample, interleaved): the bytes FLAC's
// MD5 is of.
const DECODER = has('flac') ? 'flac' : has('ffmpeg') ? 'ffmpeg' : null;
function decode(file, bits) {
  const raw = `${file}.raw`;
  if (DECODER === 'flac') {
    run('flac', ['-s', '-t', file]);
    run('flac', ['-s', '-d', '-f', '--force-raw-format', '--endian=little', '--sign=signed', '-o', raw, file]);
  } else {
    run('ffmpeg', ['-v', 'error', '-err_detect', 'crccheck+explode', '-y', '-i', file, '-f', bits === 16 ? 's16le' : 's24le', raw]);
  }
  const bytes = fs.readFileSync(raw);
  fs.unlinkSync(raw);
  return bytes;
}

function check(c) {
  const info = streaminfo(c.flac);
  const wav = readWav(c.wav);
  const frames = wav.samples.length / wav.channels;
  const what = `${c.name} ${c.layout === 2 ? 'stereo' : 'mono'} ${c.bits}-bit ${c.rate}`;
  if (info.rate !== c.rate || info.channels !== c.layout || info.bits !== c.bits || info.total !== frames) {
    fail(`${what}: STREAMINFO says ${JSON.stringify(info)}, the render ${frames} frames`);
  }
  const raw = decode(c.flac, c.bits);
  const md5 = crypto.createHash('md5').update(raw).digest('hex');
  if (md5 !== info.md5) fail(`${what}: MD5 ${info.md5} in the header, ${md5} decoded`);
  const width = c.bits / 8;
  if (raw.length !== wav.samples.length * width) fail(`${what}: ${raw.length / width} samples decoded, ${wav.samples.length} rendered`);
  const full = 2 ** (c.bits - 1);
  let differ = 0;
  let worst = 0;
  for (let i = 0; i < wav.samples.length && i * width < raw.length; i++) {
    const got = width === 2 ? raw.readInt16LE(2 * i) : raw.readIntLE(3 * i, 3);
    const x = wav.samples[i];
    if (got !== quantize(x, c.bits)) differ++;
    if (Math.abs(x) < 1) worst = Math.max(worst, Math.abs(got / full - x) * full);
  }
  if (differ) fail(`${what}: ${differ} samples differ from the WAV made integers`);
  if (worst > 0.5) fail(`${what}: a sample ${worst.toFixed(3)} steps from the WAV`);
  const bytes = fs.statSync(c.flac).size;
  const wavBytes = 44 + frames * c.layout * width;
  fs.unlinkSync(c.wav);
  return { ...c, bytes, wavBytes, ratio: bytes / wavBytes, speed: Number(/speed=(\d+)x/.exec(c.line)?.[1] ?? 0), seconds: frames / c.rate };
}

async function main() {
  if (!DECODER) throw new Error('no reference decoder: install flac (or ffmpeg)');
  console.log(`flac: building dlrender; the decoder is ${DECODER}`);
  run('cargo', ['build', '-q', '--release', '--locked', '--example', 'dlrender'], { cwd: CORE });

  // The cases: every golden list and corpus loop, in every way.
  const lists = fs.readdirSync(path.join(CORE, 'golden')).filter((f) => f.endsWith('.dlnotes')).sort();
  const cases = [];
  for (const rate of RATES) {
    for (const layout of LAYOUTS) {
      for (const bits of DEPTHS) {
        const tag = `${rate}-${layout}-${bits}`;
        for (const list of lists) {
          const base = path.join(tmp, `${list}-${tag}`);
          cases.push({ kind: 'golden', name: list.replace('.dlnotes', ''), rate, layout, bits, flac: `${base}.flac`, wav: `${base}.wav`,
            argv: [path.join(CORE, 'golden', list), '--rate', String(rate), ...(layout === 2 ? ['--stereo'] : []), '--wav', `${base}.wav`, '--flac', `${base}.flac`, '--bits', String(bits)] });
        }
        for (let i = 0; i < LOOPS; i++) {
          const base = path.join(tmp, `corpus-${i}-${tag}`);
          cases.push({ kind: 'corpus', name: `corpus ${i}`, index: i, rate, layout, bits, flac: `${base}.flac`, wav: `${base}.wav`,
            argv: ['--corpus', `${CORPUS}:${LOOPS}`, '--pick', String(i), '--seconds', String(SECONDS), '--rate', String(rate), ...(layout === 2 ? ['--stereo'] : []), '--flac', `${base}.dir`, '--wav', `${base}.wdir`, '--bits', String(bits)] });
        }
      }
    }
  }
  console.log(`flac: ${cases.length} renders (${lists.length} golden lists and ${LOOPS} corpus loops of ${SECONDS} s; mono and stereo, 16 and 24 bits, 44.1 and 48 kHz)`);
  const lines = await parallel(cases.map((c) => c.argv));
  const results = [];
  for (const [i, c] of cases.entries()) {
    c.line = lines[i];
    if (c.kind === 'corpus') {
      // A corpus writes into directories, I-NAME.flac and I-NAME.wav.
      const dir = `${c.flac.replace(/\.flac$/, '')}.dir`;
      const wdir = `${c.wav.replace(/\.wav$/, '')}.wdir`;
      const file = fs.readdirSync(dir)[0];
      c.name = file.replace(/\.flac$/, '');
      fs.renameSync(path.join(dir, file), c.flac);
      fs.renameSync(path.join(wdir, file.replace(/\.flac$/, '.wav')), c.wav);
      fs.rmSync(dir, { recursive: true });
      fs.rmSync(wdir, { recursive: true });
    }
    results.push(check(c));
  }

  // The sizes, a row a way, and the cases behind them.
  console.log('\n| way | files | FLAC / WAV, all | smallest | largest | encoder, x real time (median) |');
  console.log('|---|---|---|---|---|---|');
  const median = (xs) => [...xs].sort((a, b) => a - b)[Math.floor(xs.length / 2)];
  for (const rate of RATES) {
    for (const layout of LAYOUTS) {
      for (const bits of DEPTHS) {
        const rows = results.filter((r) => r.rate === rate && r.layout === layout && r.bits === bits);
        const all = rows.reduce((s, r) => s + r.bytes, 0) / rows.reduce((s, r) => s + r.wavBytes, 0);
        const ratios = rows.map((r) => r.ratio);
        console.log(`| ${rate / 1000} kHz ${layout === 2 ? 'stereo' : 'mono'} ${bits}-bit | ${rows.length} | ${(100 * all).toFixed(1)}% | ${(100 * Math.min(...ratios)).toFixed(1)}% | ${(100 * Math.max(...ratios)).toFixed(1)}% | ${median(rows.map((r) => r.speed))} |`);
      }
    }
  }
  const all = results.reduce((s, r) => s + r.bytes, 0) / results.reduce((s, r) => s + r.wavBytes, 0);
  console.log(`\nflac: all ${results.length} files decode with ${DECODER} and null against their WAVs; together ${(100 * all).toFixed(1)}% of the WAVs' size`);
  const lookup = (kind, rate, layout, bits) => results.filter((r) => r.kind === kind && r.rate === rate && r.layout === layout && r.bits === bits);
  console.log('\ncorpus loops at 44.1 kHz stereo, FLAC against WAV:');
  for (const bits of DEPTHS) console.log(`  ${bits}-bit: ${lookup('corpus', 44100, 2, bits).map((r) => `${r.name} ${(100 * r.ratio).toFixed(0)}%`).join(', ')}`);

  // The same bytes in WebAssembly: rendered and encoded by the tools' build.
  console.log(`\nflac: the tools' build in Node (the player's wasm with dl_flac_*), ${WASM_LOOPS} corpus loops, every way at 44.1 kHz`);
  let wasmFrames = 0;
  let wasmTime = 0;
  for (const r of results.filter((x) => x.kind === 'corpus' && x.index < WASM_LOOPS && x.rate === 44100)) {
    const seed = Number(/seed=(\d+)/.exec(r.line)[1]);
    const x = player(seed, { rate: r.rate, layout: r.layout, tools: true });
    const frames = Math.ceil(SECONDS * r.rate);
    const { out } = render(x, frames);
    const t0 = process.hrtime.bigint();
    const parts = [];
    const take = (n) => {
      if (n < 0) throw new Error('dl_flac refused');
      parts.push(Buffer.from(new Uint8Array(x.memory.buffer, x.dl_flac_out(), n)));
    };
    take(x.dl_flac_start(r.rate, r.layout, r.bits));
    for (let at = 0; at < frames; at += 4096) {
      const n = Math.min(4096, frames - at);
      for (let c = 0; c < r.layout; c++) new Float32Array(x.memory.buffer, x.dl_flac_input() + c * 4096 * 4, n).set(out[c].subarray(at, at + n));
      take(x.dl_flac_write(n));
    }
    take(x.dl_flac_finish());
    const file = Buffer.concat(parts);
    Buffer.from(new Uint8Array(x.memory.buffer, x.dl_flac_header(), 42)).copy(file, 0);
    wasmTime += Number(process.hrtime.bigint() - t0) / 1e9;
    wasmFrames += frames;
    const native = fs.readFileSync(r.flac);
    const same = native.equals(file);
    if (!same) fail(`${r.name} ${r.layout === 2 ? 'stereo' : 'mono'} ${r.bits}-bit: the wasm encoder's file differs from the native one`);
    console.log(`  ${r.name} ${r.layout === 2 ? 'stereo' : 'mono'} ${r.bits}-bit: ${same ? 'the same bytes' : 'DIFFERENT'} (${file.length} bytes)`);
    if (r === results.find((y) => y.kind === 'corpus')) console.log(`  the encoder's state: ${x.dl_flac_size()} bytes (dl_flac_size)`);
  }
  console.log(`  the wasm encoder: ${(wasmFrames / 44100 / wasmTime).toFixed(0)}x real time (all ways together, the encoding alone)`);

  // The C host, through the header alone.
  console.log('\nflac: the C host (examples/c/seed.c --flac)');
  run('cargo', ['rustc', '-q', '--release', '--locked', '--lib', '--features', 'capi', '--crate-type', 'staticlib'], { cwd: CORE });
  const cc = process.env.CC || 'cc';
  const host = path.join(tmp, 'seed');
  run(cc, ['-O2', '-std=c11', '-Wall', '-Wextra', '-Werror', '-pedantic', '-Iinclude', 'examples/c/seed.c', 'target/release/libdlcore.a', '-lm', '-o', host], { cwd: CORE });
  for (const r of results.filter((x) => x.kind === 'corpus' && x.index === 0 && x.rate === 48000)) {
    const seed = Number(/seed=(\d+)/.exec(r.line)[1]);
    const file = path.join(tmp, 'c.flac');
    run(host, [String(seed), '--seconds', String(SECONDS), '--rate', String(r.rate), ...(r.layout === 2 ? ['--stereo'] : []), '--flac', file, '--bits', String(r.bits)]);
    const same = fs.readFileSync(file).equals(fs.readFileSync(r.flac));
    if (!same) fail(`the C host's ${r.layout === 2 ? 'stereo' : 'mono'} ${r.bits}-bit file differs from dlrender's`);
    console.log(`  ${r.name} ${r.layout === 2 ? 'stereo' : 'mono'} ${r.bits}-bit at 48 kHz: ${same ? 'the same bytes as dlrender' : 'DIFFERENT'}`);
  }

  if (LONG > 0) {
    console.log(`\nflac: ${LONG} hours streamed, 16-bit stereo at 44.1 kHz`);
    const peak = (seconds) => {
      const file = path.join(tmp, `long-${seconds}.flac`);
      const line = run(DLRENDER, ['--seed', '1234', '--seconds', String(seconds), '--stereo', '--flac', file, '--bits', '16']).stdout.trim();
      if (seconds > 60) run(DECODER === 'flac' ? 'flac' : 'ffmpeg', DECODER === 'flac' ? ['-s', '-t', file] : ['-v', 'error', '-err_detect', 'crccheck+explode', '-i', file, '-f', 'null', '-']);
      const bytes = fs.statSync(file).size;
      fs.unlinkSync(file);
      return { kb: Number(/peak=(\d+)KB/.exec(line)?.[1]), bytes, line };
    };
    const minute = peak(60);
    const long = peak(LONG * 3600);
    console.log(`  a minute: peak ${minute.kb} KB; ${LONG} hours: peak ${long.kb} KB, a ${(long.bytes / 1e6).toFixed(0)} MB file that decodes`);
    if (!(long.kb <= minute.kb * 1.05)) fail(`the long render's memory grew: ${minute.kb} KB to ${long.kb} KB`);
  }

  fs.rmSync(tmp, { recursive: true, force: true });
  if (failures) {
    console.log(`\nflac: ${failures} failures`);
    process.exit(1);
  }
  console.log('\nflac: every check passed');
}

main().catch((e) => {
  console.error(e.message);
  fs.rmSync(tmp, { recursive: true, force: true });
  process.exit(1);
});
