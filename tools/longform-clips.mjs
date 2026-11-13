#!/usr/bin/env node
// The long pieces' listening set (queue item 102e, session 15): excerpts of
// about 40 s where something happens in a piece (its start, B's entry, the
// peak, the return, a breath, a close the length slider made, the close and
// its landing; a wander's moves; Endless's first twenty minutes), one
// 64-bar piece whole, and blind pairs (the same seed and length before
// item 102 and now, level-matched, named honestly before and after: brain
// B3 makes them blind on its page). Each excerpt's place is read from the
// piece's map (`tools/gen-lib.mjs`'s `pieceMap`), not written in by hand,
// so the set can be made again after session 16's tuning.
//
//   node tools/longform-clips.mjs OUTDIR [--before PATH] [--only NAME]
//
// OUTDIR gets NAME.opus (stereo, 96 kbps, ffmpeg's libopus) for each clip,
// and index.md and index.json: each clip's piece (seed and request), where
// in it, how long, and one line on what to listen for. `--before` is a
// dlrender built at the commit before session 12 (dfe78c3: the plan as
// code, nothing of it heard), for the pairs' before; without it the pairs
// are left out. Renders are the player's (`dlrender --seed`, at the app's
// wander amount 1, 44.1 kHz, stereo), the tools' build for the maps
// (`tools/build-core.sh --tools`) and the release dlrender (built first).

import { execFileSync, spawnSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { pieceMap } from './gen-lib.mjs';

const ROOT = path.join(path.dirname(fileURLToPath(import.meta.url)), '..');
const DLRENDER = path.join(ROOT, 'core', 'target', 'release', 'examples', 'dlrender');
const args = process.argv.slice(2);
const value = (name) => (args.includes(name) ? args[args.indexOf(name) + 1] : null);
const OUT = args.find((a, i) => !a.startsWith('--') && !['--before', '--only'].includes(args[i - 1]));
if (!OUT) {
  console.error('usage: node tools/longform-clips.mjs OUTDIR [--before PATH] [--only NAME]');
  process.exit(2);
}
const BEFORE = value('--before');
const ONLY = value('--only');
fs.mkdirSync(OUT, { recursive: true });
if (process.env.LONGFORM_BUILD !== 'no') execFileSync('cargo', ['build', '--release', '--locked', '--example', 'dlrender'], { cwd: path.join(ROOT, 'core'), stdio: 'ignore' });

/** A piece's units with their times: each unit's start in seconds from the piece's first step. */
function timed(seed, request, sections) {
  const { form, units } = pieceMap(seed, request, 0, sections);
  const bar = form.unitSeconds / form.unitBars;
  let t = 0;
  for (const u of units) {
    u.t = t;
    t += u.bars * bar;
  }
  return { form, units, bar, end: t };
}
/** The first unit after `after` seconds that `pick` takes. */
const first = (p, pick, after = 0) => p.units.find((u) => u.t >= after && pick(u));
const role = (name) => (u) => u.role === name;
const mmss = (t) => `${Math.floor(t / 60)}:${String(Math.floor(t % 60)).padStart(2, '0')}`;

// The pieces (session 15's choice; `docs/notes/for-mikey.md` says how they
// were chosen), each with its excerpts: a name, where it starts (a
// function of the piece's timed map), how long, and what to listen for.
const PIECES = [
  {
    key: 'tha-vol-64', seed: 2728346932, request: 'bars=64,journey=arc', title: 'tha-vol, 64 bars, an arc (the small form), whole',
    clips: (p) => [{ name: 'tha-vol-64-whole', at: 0, len: p.end + 6, tail: true, what: 'the whole small form: the opening thin, a fill into the theme, the lift with the tune doubled an octave up, the floor out as it falls, the bass on the home fifth before theme A returns answered, a beat of the tune alone into the close, the close thinning, the landing on the tonic chord alone, ringing' }],
  },
  {
    key: 'brum-yei-256', seed: 1151594917, request: 'bars=256,journey=arc', title: 'brum-yei, 256 bars, an arc in two movements',
    clips: (p) => {
      const b = first(p, role('second'));
      const peak = first(p, role('peak'));
      const ret = first(p, role('return'));
      const breath = first(p, role('breath'));
      const close = first(p, role('close'));
      return [
        { name: 'brum-yei-256-start', at: 0, len: 40, what: 'the opening: thin and a little dark, the kit softer; a fill into the theme' },
        { name: 'brum-yei-256-b-enters', at: b.t - 20, len: 40, what: `a chord both keys share held across the line, then theme B, another tune in the same voice, a fourth up (${mmss(b.t)})` },
        { name: 'brum-yei-256-peak', at: peak.t - 15, len: 45, what: `the development into the peak (${mmss(peak.t)}), the tune doubled an octave up; then the floor out for a beat as it falls into the release` },
        { name: 'brum-yei-256-return', at: ret.t - 22, len: 40, what: `the release, the bass holding the home fifth, then theme A home, an octave away (${mmss(ret.t)})` },
        { name: 'brum-yei-256-breath', at: breath.t - 5, len: 45, what: `the breath between the two movements (${mmss(breath.t)}): the tune at its thinnest, not gone; the second movement's opening after it` },
        { name: 'brum-yei-256-close', at: close.t - 10, len: p.end - close.t + 16, tail: true, what: `the close (${mmss(close.t)}): thinning to the chords and the tune, the landing on the tonic, ringing out` },
      ];
    },
  },
  {
    key: 'fei-no-256', seed: 2158762586, request: 'bars=256,journey=arc', title: 'fei-no, 256 bars, one arc',
    clips: (p) => {
      const b = first(p, role('second'));
      const ret = first(p, role('return'));
      return [
        { name: 'fei-no-256-b-enters', at: b.t - 20, len: 40, what: `theme B in the relative key (${mmss(b.t)}): every note two steps of the mode down; the join across the line is a chord both keys share` },
        { name: 'fei-no-256-return', at: ret.t - 15, len: 40, what: `theme A home, stretched (${mmss(ret.t)}): each phrase's first half at half speed over held chords` },
      ];
    },
  },
  {
    key: 'fei-no-256-stopped', seed: 2158762586, request: 'bars=256,journey=arc', track: 'bars=120', title: 'fei-no, 256 bars, the length slider at 120 bars',
    clips: (p) => {
      const stop = 120 * p.bar;
      return [{ name: 'fei-no-256-slider-close', at: stop - 30, len: 36, what: `the length slider stops the piece at bar 120 (${mmss(stop)}), in the middle of its arc: the last phrases thin to a close and land on the tonic, ringing out (before session 15, a one-second fade); the next loop follows, as in the app` }];
    },
  },
  {
    key: 'moan-teil-1024', seed: 2060199680, request: 'bars=1024,journey=arc', title: 'moan-teil, 1,024 bars, five movements of four kinds',
    clips: (p) => {
      const last = p.units.at(-1).part;
      const look = first(p, (u) => u.part === last && u.role === 'development');
      const close = p.units.find((u) => u.role === 'close');
      const interlude = first(p, (u) => u.movement === 'interlude' && u.role === 'turning');
      return [
        ...(interlude ? [{ name: 'moan-teil-1024-interlude', at: interlude.t - 15, len: 40, what: `the interlude, a movement in the wander's way: one thing moves (${mmss(interlude.t)}), nothing else` }] : []),
        { name: 'moan-teil-1024-look-back', at: look.t, len: 45, what: `the finale looks back (${mmss(look.t)}): fragments of the earlier movements' second themes, through the related keys, a chord held across each key's line` },
        { name: 'moan-teil-1024-close', at: close.t - 10, len: p.end - close.t + 16, tail: true, what: `the whole piece's close (${mmss(close.t)}) after 34 minutes, landing on the tonic` },
      ];
    },
  },
  {
    key: 'loa-yer-1024', seed: 2975389383, request: 'bars=1024,adventurous=0.8', title: 'loa-yer, 1,024 bars, adventurous: the vibe draws the arc (no journey asked)',
    clips: (p) => {
      const peak = first(p, role('peak'));
      return [
        { name: 'loa-yer-1024-start', at: 0, len: 40, what: 'an arc the vibe chose: the opening held back' },
        { name: 'loa-yer-1024-peak', at: peak.t - 20, len: 40, what: `the first movement's peak (${mmss(peak.t)}): brighter and fuller, the kit's shimmer in, never loud` },
      ];
    },
  },
  {
    key: 'gluith-vuisk-64', seed: 1068550449, request: 'bars=64,journey=arc', title: "gluith-vuisk, 64 bars, an arc over batch 6's slow opening",
    clips: () => [{ name: 'gluith-vuisk-64-start', at: 0, len: 90, what: "a question for your ears, not a showcase: the loop's own slow opening keeps the kit out for 74 s and the bass for 100 s, so the arc's opening and theme are the chords and the tune alone; does the start run too long thin?" }],
  },
  {
    key: 'glor-ti-endless', seed: 7, request: 'bars=endless,journey=wander', title: 'glor-ti, Endless, the wander: its first twenty minutes',
    clips: (p) => {
      const moves = p.units.filter((u) => u.role === 'turning' && u.t < 20 * 60);
      return moves.slice(0, 5).map((u, i) => ({ name: `glor-ti-endless-move-${i + 1}`, at: u.t - 15, len: 40, what: `${mmss(u.t)}: ${u.changed.filter((c) => c !== 'role').join(', ')} moves, alone and slowly (${['layers', 'warmth', 'space', 'density'].some((c) => u.changed.includes(c)) ? 'a glide' : 'on the line, the layer that moves taking a breath before it'})` }));
    },
  },
  {
    key: 'glor-ti-endless-stopped', seed: 7, request: 'bars=endless,journey=wander', track: 'bars=100', title: 'glor-ti, Endless, the length slider at 100 bars',
    clips: (p) => {
      const stop = 100 * p.bar;
      return [{ name: 'glor-ti-endless-slider-close', at: stop - 30, len: 36, what: `Endless stopped by the length slider at bar 100 (${mmss(stop)}): the wander closes softly and lands on the tonic, ringing out, where a fade was` }];
    },
  },
  {
    key: 'mu-meth-endless', seed: 1086007177, request: 'bars=endless,journey=wander', title: 'mu-meth, Endless, the wander',
    clips: (p) => {
      const key = first(p, (u) => u.changed.includes('key'));
      return [{ name: 'mu-meth-endless-key', at: key.t - 22, len: 40, what: `the wander's rare key move (${mmss(key.t)}): the whole loop a fourth up, a chord both keys share held across the line` }];
    },
  },
  {
    key: 'teth-hu-endless', seed: 706141505, request: 'bars=endless,still=0.8', title: 'teth-hu, Endless, still: the vibe draws the wander (no journey asked)',
    clips: (p) => {
      const move = first(p, role('turning'), 60);
      return [
        { name: 'teth-hu-endless-start', at: 0, len: 40, what: 'a wander the vibe chose, at 40 bpm: long held states' },
        { name: 'teth-hu-endless-move', at: move.t - 15, len: 45, what: `its first move after a minute (${mmss(move.t)}): ${move.changed.filter((c) => c !== 'role').join(', ')}` },
      ];
    },
  },
  {
    key: 'brum-yei-endless', seed: 1151594917, request: 'bars=endless,journey=arc', title: 'brum-yei, Endless, arcs in turn',
    clips: (p) => {
      const breath = first(p, role('breath'));
      const gentle = first(p, (u) => u.movement === 'gentle' && u.role === 'second');
      return [
        { name: 'brum-yei-endless-breath', at: breath.t - 10, len: 45, what: `Endless's first arc ending in a breath, not a close (${mmss(breath.t)}), then the next movement, a gentle one, beginning` },
        ...(gentle ? [{ name: 'brum-yei-endless-gentle', at: gentle.t - 15, len: 40, what: `the gentle movement's own second theme (${mmss(gentle.t)}): the full form held low, no build to speak of` }] : []),
      ];
    },
  },
];

// The blind pairs: the same seed and request, rendered before item 102
// (dfe78c3: the loop's wandered sections) and now.
const PAIRS = [
  { name: 'pair-1', seed: 1151594917, request: 'bars=256,journey=arc', at: 55, len: 45, what: 'brum-yei, 256 bars, from 0:55: the build into the second theme now, the loop wandering before' },
  { name: 'pair-2', seed: 7, request: 'bars=endless,journey=wander', at: 320, len: 45, what: 'glor-ti, Endless, from 5:20: the kit fading out over 24 s now, the loop wandering before' },
];

const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'longform-clips-'));
const render = (bin, seed, request, seconds, track) => {
  const file = path.join(tmp, `${seed}-${Math.random().toString(36).slice(2)}.wav`);
  execFileSync(bin, ['--seed', String(seed), '--request', request, '--seconds', String(Math.ceil(seconds)), '--stereo', '--wander', '1', ...(track ? ['--track', track] : []), '--wav', file], { stdio: 'ignore' });
  return file;
};
/** The integrated loudness of a stretch of a WAV, by ffmpeg's EBU R128 meter. */
const loudness = (file, at, len) => {
  const r = spawnSync('ffmpeg', ['-hide_banner', '-nostats', '-ss', String(at), '-t', String(len), '-i', file, '-af', 'ebur128', '-f', 'null', '-'], { encoding: 'utf8' });
  const m = [...(r.stderr ?? '').matchAll(/I:\s+(-?[\d.]+) LUFS/g)].at(-1);
  return m ? Number(m[1]) : NaN;
};
/** An excerpt to Opus: a short fade in, and out unless it ends ringing (`tail`), at `gain` dB. */
const cut = (file, name, at, len, { tail = false, gain = 0 } = {}) => {
  const fades = [`afade=t=in:st=0:d=${at > 0 ? 0.6 : 0.01}`, ...(tail ? [] : [`afade=t=out:st=${Math.max(0, len - 1.5)}:d=1.5`])];
  if (gain) fades.push(`volume=${gain.toFixed(2)}dB`);
  const out = path.join(OUT, `${name}.opus`);
  execFileSync('ffmpeg', ['-hide_banner', '-loglevel', 'error', '-y', '-ss', String(Math.max(0, at)), '-t', String(len), '-i', file, '-af', fades.join(','), '-c:a', 'libopus', '-b:a', '96k', out]);
  return out;
};

const index = [];
for (const piece of PIECES) {
  if (ONLY && !piece.key.startsWith(ONLY)) continue;
  const head = pieceMap(piece.seed, piece.request, 0, 1).form;
  // The sections read: a finite piece's every one, Endless's first half hour.
  const sections = head.sections ?? Math.ceil((40 * 60) / (head.unitSeconds * head.sectionBars / head.unitBars));
  const p = timed(piece.seed, piece.request, sections);
  const clips = piece.clips(p);
  const upto = Math.max(...clips.map((c) => c.at + c.len)) + 2;
  // (The first step falls 0.05 s or so into a render: the excerpts keep it.)
  const wav = render(DLRENDER, piece.seed, piece.request, upto, piece.track);
  for (const c of clips) {
    const at = Math.max(0, c.at);
    cut(wav, c.name, at, c.len, c);
    index.push({ name: c.name, file: `${c.name}.opus`, piece: piece.title, seed: piece.seed, request: piece.request, track: piece.track ?? null, from: mmss(at), seconds: Math.round(c.len), what: c.what, lufs: Number(loudness(wav, at, c.len).toFixed(1)) });
    console.log(`${c.name}: ${mmss(at)} for ${Math.round(c.len)} s`);
  }
  fs.rmSync(wav, { force: true });
}
if (BEFORE && !ONLY) {
  for (const pair of PAIRS) {
    const now = render(DLRENDER, pair.seed, pair.request, pair.at + pair.len + 2);
    const then = render(BEFORE, pair.seed, pair.request, pair.at + pair.len + 2);
    const [ln, lt] = [loudness(now, pair.at, pair.len), loudness(then, pair.at, pair.len)];
    // Level-matched to the quieter, so neither is pushed toward clipping.
    const target = Math.min(ln, lt);
    for (const [side, wav, l] of [['after', now, ln], ['before', then, lt]]) {
      const name = `${pair.name}-${side}`;
      cut(wav, name, pair.at, pair.len, { gain: target - l });
      index.push({ name, file: `${name}.opus`, piece: pair.what, seed: pair.seed, request: pair.request, track: null, from: mmss(pair.at), seconds: pair.len, what: side === 'after' ? 'now (session 15): the same seed and length as its pair' : 'before item 102 (dfe78c3): the same seed and length as its pair', lufs: Number(target.toFixed(1)), gain: Number((target - l).toFixed(2)) });
      fs.rmSync(wav, { force: true });
      console.log(`${name}: ${mmss(pair.at)} for ${pair.len} s, ${(target - l).toFixed(2)} dB to ${target.toFixed(1)} LUFS`);
    }
  }
}
fs.rmSync(tmp, { recursive: true, force: true });
const total = index.reduce((s, c) => s + c.seconds, 0);
fs.writeFileSync(path.join(OUT, 'index.json'), `${JSON.stringify(index, null, 2)}\n`);
const md = ['# The long pieces: the listening set (102e, session 15)', '', `${index.length} clips, ${mmss(total)} in all. Stereo Opus, 96 kbps, the app's wander amount (1), 44.1 kHz. Made by \`tools/longform-clips.mjs\`.`, '', '| Clip | Piece | Request | From | Seconds | Listen for |', '|---|---|---|---|---|---|', ...index.map((c) => `| ${c.file} | ${c.piece} | \`${c.seed} ${c.request}${c.track ? ` track ${c.track}` : ''}\` | ${c.from} | ${c.seconds} | ${c.what} |`), ''];
fs.writeFileSync(path.join(OUT, 'index.md'), md.join('\n'));
console.log(`longform-clips: ${index.length} clips, ${mmss(total)} in all, in ${OUT}`);
