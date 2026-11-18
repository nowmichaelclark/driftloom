// The Ogg writer for Opus (js/ogg.js, queue item 104c): the CRC, the pages,
// the headers, the granule positions and the pre-skip, read back by a reader
// written here from the specs; and, where ffmpeg is installed, real Opus
// packets (from libopus) written into a file by the writer, which ffprobe
// and ffmpeg must read to the sample.
// (The reader is tools/ogg-read.mjs.)
//
//   node test/ogg.test.mjs

import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { crc32, packetSamples, page, OggOpus, GRANULE_RATE } from '../js/ogg.js';
import { pages } from '../tools/ogg-read.mjs';

let failures = 0;
let checks = 0;
const check = (ok, what) => {
  checks++;
  if (!ok) {
    failures++;
    console.log(`  FAIL ${what}`);
  }
};
const same = (a, b) => a.length === b.length && a.every((x, i) => x === b[i]);

const join = (parts) => {
  const out = new Uint8Array(parts.reduce((n, p) => n + p.length, 0));
  let at = 0;
  for (const p of parts) {
    out.set(p, at);
    at += p.length;
  }
  return out;
};

// ------------------------------------------------------------------- CRC

console.log('ogg: the checksum');
// Ogg's CRC: polynomial 0x04C11DB7, not reflected, no initial or final xor;
// here against the bit-by-bit definition.
const slow = (bytes) => {
  let r = 0;
  for (const b of bytes) {
    r ^= b << 24;
    for (let k = 0; k < 8; k++) r = r & 0x80000000 ? (r << 1) ^ 0x04c11db7 : r << 1;
  }
  return r >>> 0;
};
{
  let s = 12345;
  const noise = Uint8Array.from({ length: 5000 }, () => (s = (Math.imul(s, 1103515245) + 12345) >>> 0) >>> 24);
  check(crc32(noise) === slow(noise) && crc32(noise.subarray(0, 77)) === slow(noise.subarray(0, 77)), 'the table CRC is the bit-by-bit one');
  check(crc32(noise.subarray(100), crc32(noise.subarray(0, 100))) === crc32(noise), 'and can be run in parts');
}
check(crc32(new Uint8Array(0)) === 0, 'the CRC of nothing is 0');

// ------------------------------------------------------------ packet sizes

console.log('ogg: what a packet holds');
check(packetSamples(Uint8Array.of(0xf8, 1, 2)) === 960, 'a CELT 20 ms frame is 960 samples');
check(packetSamples(Uint8Array.of(0xe0)) === 120, 'a CELT 2.5 ms frame is 120');
check(packetSamples(Uint8Array.of(0xf9, 1)) === 1920, 'two frames of 20 ms (code 1) are 1920');
check(packetSamples(Uint8Array.of(0xfb, 3)) === 2880, 'three frames (code 3, count 3) are 2880');
check(packetSamples(Uint8Array.of(0xfb, 0)) === null, 'code 3 with no frames is not a packet');
check(packetSamples(Uint8Array.of(0xfb, 7)) === null, 'more than 120 ms is not a packet');
check(packetSamples(Uint8Array.of(0x00)) === 480, 'a SILK 10 ms frame is 480');
check(packetSamples(Uint8Array.of(0x78)) === 960, 'a hybrid 20 ms frame is 960');
check(packetSamples(new Uint8Array(0)) === null, 'nothing is not a packet');

// -------------------------------------------------------------- the pages

console.log('ogg: pages, headers and granule positions');
{
  // A packet of `n` bytes that reads as a 20 ms CELT frame.
  const packet = (n, fill = 1) => {
    const p = new Uint8Array(n).fill(fill);
    p[0] = 0xf8;
    return p;
  };
  const preSkip = 312;
  const w = new OggOpus({ channels: 2, preSkip, rate: 44100, serial: 0xdeadbeef, comments: ['TITLE=Ballad Falls', 'x=y'] });
  const parts = [w.header()];
  const given = [];
  // 7 seconds of packets of every awkward length: 1, 254, 255, 256, 510, 511.
  const sizes = [1, 254, 255, 256, 510, 511, 90, 160, 1275];
  const count = 350;
  for (let i = 0; i < count; i++) {
    const p = packet(sizes[i % sizes.length], i & 0xff);
    p[0] = 0xf8;
    given.push(p);
    for (const pg of w.add(p)) parts.push(pg);
  }
  const total = count * 960 - 500; // the audio ends mid-packet
  for (const pg of w.finish(total)) parts.push(pg);
  const ps = pages(join(parts));
  check(ps[0].flags === 2 && ps[0].granule === 0 && ps[0].sequence === 0, 'the first page is the beginning of the stream, granule 0');
  check(ps[1].flags === 0 && ps[1].granule === 0 && ps[1].sequence === 1, 'the second page is the tags');
  const h = ps[0].packets[0];
  check(ps[0].packets.length === 1 && String.fromCharCode(...h.subarray(0, 8)) === 'OpusHead', 'OpusHead alone on its page');
  const hv = new DataView(h.buffer, h.byteOffset);
  check(h[8] === 1 && h[9] === 2 && hv.getUint16(10, true) === preSkip && hv.getUint32(12, true) === 44100 && hv.getInt16(16, true) === 0 && h[18] === 0 && h.length === 19,
    'OpusHead: version 1, two channels, the pre-skip, the input rate, no gain, family 0');
  const t = ps[1].packets[0];
  check(ps[1].packets.length === 1 && String.fromCharCode(...t.subarray(0, 8)) === 'OpusTags', 'OpusTags alone on its page');
  const tv = new DataView(t.buffer, t.byteOffset);
  const vendorLength = tv.getUint32(8, true);
  const comments = tv.getUint32(12 + vendorLength, true);
  const first = tv.getUint32(16 + vendorLength, true);
  check(comments === 2 && new TextDecoder().decode(t.subarray(20 + vendorLength, 20 + vendorLength + first)) === 'TITLE=Ballad Falls', 'OpusTags: the vendor and the comments');
  check(ps.every((p, i) => p.serial === 0xdeadbeef && p.sequence === i && p.version === 0), 'one serial, sequence numbers 0, 1, 2 ...');
  const audio = ps.slice(2);
  check(audio.length >= 7 && audio.length <= 9, `audio pages of about a second (${audio.length} for 7 s)`);
  check(audio.every((p) => p.segments <= 255), 'no page past 255 segments');
  check(audio.slice(0, -1).every((p) => (p.flags & 4) === 0) && (audio[audio.length - 1].flags & 4) === 4, 'only the last page ends the stream');
  check(audio.every((p, i) => i === 0 || p.granule >= audio[i - 1].granule), 'granule positions never go back');
  check(audio[audio.length - 1].granule === preSkip + total, `the last granule is the pre-skip and the samples the audio holds (${audio[audio.length - 1].granule} = ${preSkip} + ${total})`);
  check(audio.every((p) => p.granule <= preSkip + total), 'no granule past the audio\'s end');
  check(audio.slice(0, -1).every((p, i, all) => i === 0 ? p.granule === p.packets.length * 960 : p.granule === all[i - 1].granule + p.packets.length * 960), 'each page\'s granule counts the packets so far');
  const back = audio.flatMap((p) => p.packets);
  check(back.length === count && back.every((p, i) => same(p, given[i])), 'every packet comes back, whole and in order, whatever its length');
}

console.log('ogg: a long packet, a page that fills by segments, and the end cases');
{
  const packet = (n) => {
    const p = new Uint8Array(n);
    p[0] = 0xf8;
    return p;
  };
  // 200 tiny packets of 2.5 ms (120 samples): 255 segments fill a page long
  // before a second does.
  const w = new OggOpus({ channels: 1, preSkip: 0 });
  const parts = [w.header()];
  for (let i = 0; i < 600; i++) {
    const p = packet(10);
    p[0] = 0xe0;
    for (const pg of w.add(p)) parts.push(pg);
  }
  for (const pg of w.finish(600 * 120)) parts.push(pg);
  const ps = pages(join(parts)).slice(2);
  check(ps.length === 3 && ps.every((p) => p.segments <= 255), `255 segments close a page (${ps.length} pages, at most ${Math.max(...ps.map((p) => p.segments))} segments)`);
  check(ps[ps.length - 1].granule === 600 * 120, 'and the granule is the samples (no pre-skip given)');
  check(ps.flatMap((p) => p.packets).length === 600, 'all 600 packets');
}
{
  // The end cuts the audio shorter than the packets span, by more than one
  // page: nothing is written past it.
  const w = new OggOpus({ channels: 1, preSkip: 312 });
  const parts = [w.header()];
  for (let i = 0; i < 150; i++) {
    const p = new Uint8Array(40);
    p[0] = 0xf8;
    for (const pg of w.add(p)) parts.push(pg);
  }
  for (const pg of w.finish(100000)) parts.push(pg);
  const ps = pages(join(parts)).slice(2);
  check(ps.every((p) => p.granule <= 312 + 100000) && ps[ps.length - 1].granule === 312 + 100000, 'the end clamps a held page\'s granule past it');
  check(ps.every((p, i) => i === 0 || p.granule >= ps[i - 1].granule), 'and they still never go back');
}
{
  // Packets wholly past the end are left out: an encoder that flushes a
  // packet or two of silence after the audio leaves less than one packet
  // of trimming.
  const w = new OggOpus({ channels: 1, preSkip: 312 });
  const parts = [w.header()];
  for (let i = 0; i < 120; i++) {
    const p = new Uint8Array(30);
    p[0] = 0xf8;
    for (const pg of w.add(p)) parts.push(pg);
  }
  const total = 100 * 960 + 100; // 101 packets hold it with the pre-skip: 312 + 96100 = 96412 -> 101 packets
  for (const pg of w.finish(total)) parts.push(pg);
  const ps = pages(join(parts)).slice(2);
  const n = ps.flatMap((p) => p.packets).length;
  check(n === 101, `packets wholly past the end are dropped (${n} kept of 120, 101 hold the audio)`);
  check(ps[ps.length - 1].granule === 312 + total && ps[ps.length - 1].granule > (n - 1) * 960 && ps[ps.length - 1].granule <= n * 960, 'and the end trims less than one packet');
}
{
  const w = new OggOpus({ channels: 1, preSkip: 312 });
  const parts = [w.header(), ...w.finish(0)];
  const ps = pages(join(parts));
  check(ps.length === 3 && ps[2].flags === 4 && ps[2].packets.length === 0, 'a file with no audio is the headers and an empty last page');
  let threw = false;
  try { w.add(new Uint8Array([0xf8])); } catch { threw = true; }
  check(threw, 'a finished file takes no more');
  threw = false;
  try { new OggOpus({ channels: 6, preSkip: 0 }); } catch { threw = true; }
  check(threw, 'six channels are refused (family 0 is mono or stereo)');
  threw = false;
  try { new OggOpus({ channels: 1, preSkip: 0 }).add(new Uint8Array([0xfb, 0])); } catch { threw = true; }
  check(threw, 'a packet that is not one is refused');
}
{
  const one = page(0, 2 ** 40 + 5, 7, 9, [Uint8Array.of(1, 2, 3)]);
  const p = pages(one)[0];
  check(p.granule === 2 ** 40 + 5 && p.sequence === 9 && p.serial === 7, 'a granule past 32 bits is written in full');
  const exact = page(0, 0, 1, 0, [new Uint8Array(255)]);
  check(pages(exact)[0].packets[0].length === 255 && exact[26] === 2 && exact[28] === 0, 'a packet of exactly 255 bytes ends with a 0 segment');
  check(GRANULE_RATE === 48000, 'granules count at 48 kHz');
}

// -------------------------------------------------- real packets, real players

const has = (cmd) => spawnSync('sh', ['-c', `command -v ${cmd}`]).status === 0;
if (has('ffmpeg') && has('ffprobe') && spawnSync('ffmpeg', ['-hide_banner', '-encoders'], { encoding: 'utf8' }).stdout.includes('libopus')) {
  console.log('ogg: libopus\'s own packets in this writer\'s file, read by ffprobe and ffmpeg');
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'dlogg-'));
  const run = (cmd, args) => spawnSync(cmd, args, { encoding: 'utf8' });
  let spareTotal = null;
  for (const [channels, bitrate] of [[1, '64k'], [2, '96k']]) {
    const src = path.join(dir, `src${channels}.opus`);
    const made = run('ffmpeg', ['-v', 'error', '-y', '-f', 'lavfi', '-i', `sine=frequency=440:sample_rate=48000:duration=5.37`, '-af', channels === 2 ? 'pan=stereo|c0=c0|c1=0.5*c0' : 'anull', '-c:a', 'libopus', '-b:a', bitrate, src]);
    if (made.status !== 0) {
      check(false, `ffmpeg could not make a source: ${made.stderr}`);
      continue;
    }
    const sp = pages(new Uint8Array(fs.readFileSync(src)));
    const hd = sp[0].packets[0];
    const preSkip = new DataView(hd.buffer, hd.byteOffset).getUint16(10, true);
    const packets = sp.slice(2).flatMap((p) => p.packets);
    // The audio's true length: the source's last granule less its pre-skip.
    const whole = sp[sp.length - 1].granule - preSkip;
    // The whole, and the audio ending a little short of the last packet (a
    // player may be asked to cut less than one packet).
    for (const total of [whole, whole - 1, whole - 500]) {
      const w = new OggOpus({ channels, preSkip, serial: 0x1234 });
      const parts = [w.header()];
      for (const p of packets) for (const pg of w.add(p)) parts.push(pg);
      for (const pg of w.finish(total)) parts.push(pg);
      const file = path.join(dir, `out${channels}-${total}.opus`);
      fs.writeFileSync(file, join(parts));
      if (channels === 2) spareTotal = total;
      const probe = run('ffprobe', ['-v', 'warning', '-show_entries', 'format=duration:stream=codec_name,channels,sample_rate', '-of', 'default=nw=1', file]);
      const seconds = Number(/duration=([\d.]+)/.exec(probe.stdout)?.[1]);
      check(probe.status === 0 && !probe.stderr.trim() && /codec_name=opus/.test(probe.stdout) && new RegExp(`channels=${channels}`).test(probe.stdout),
        `${channels} ch, ${total} samples: ffprobe reads it with no warnings (${probe.stderr.trim()})`);
      check(Math.abs(seconds - (total + preSkip) / 48000) < 1.5e-6, `${channels} ch, ${total} samples: the container's duration is ${seconds} s (ffprobe counts the pre-skip: ${(total + preSkip) / 48000} s expected)`);
      const raw = spawnSync('ffmpeg', ['-v', 'warning', '-i', file, '-f', 's16le', '-'], { maxBuffer: 1 << 28 });
      check(raw.status === 0 && !String(raw.stderr).trim() && raw.stdout.length === total * 2 * channels, `${channels} ch, ${total} samples: ffmpeg decodes exactly ${total} frames, no warnings (${raw.stdout.length / 2 / channels}; ${String(raw.stderr).trim()})`);
      // The same packets in the source file decode to the same samples, up to the cut.
      const ref = spawnSync('ffmpeg', ['-v', 'error', '-i', src, '-f', 's16le', '-'], { maxBuffer: 1 << 28 });
      check(Buffer.compare(raw.stdout, ref.stdout.subarray(0, raw.stdout.length)) === 0, `${channels} ch, ${total} samples: the samples are the source's, to the sample`);
    }
  }
  if (has('opusinfo') && spareTotal != null) {
    const info = run('opusinfo', [path.join(dir, `out2-${spareTotal}.opus`)]);
    check(info.status === 0 && !/warning|error|invalid/i.test(info.stdout + info.stderr), `opusinfo has no complaint (${(info.stdout + info.stderr).split('\n').filter((l) => /warning|error|invalid/i.test(l)).join('; ')})`);
  }
  fs.rmSync(dir, { recursive: true, force: true });
} else {
  console.log('ogg: no ffmpeg with libopus here: the real-packet checks are skipped');
}

console.log(failures ? `\nogg: ${failures} of ${checks} checks failed` : `\nogg: all ${checks} checks passed`);
process.exit(failures ? 1 : 0);
