#!/usr/bin/env node
// Share codes, version 2 (queue item 105), from the command line, on the
// core's own reader and writer (`core/src/code.rs`, the tools' build:
// `tools/build-core.sh --tools`). For the brain's albums and for looking
// inside a code; core/README.md, "Share codes", is the format.
//
//   node tools/codes.mjs loop SEED [REQUEST] [--mutes N] [--salt N] [--name T]
//                                   a loop's code (REQUEST as `dlrender
//                                   --request` reads it: "glacial,bars=64";
//                                   a name of its own, any script, emoji)
//   node tools/codes.mjs read CODE  a loop's seed, mutes, salt and name (and
//                                   whether it reads), or an album's tracks
//   node tools/codes.mjs album [--name T] CODE...
//                                   an album of loops' codes, in order, each
//                                   with how long it plays before the next
//                                   after it where it has one (session 24b,
//                                   a time since 30c: CODE:t5025 1:23:45, i
//                                   and o after it for its fades, CODE:i on
//                                   and on fading in)
//   node tools/codes.mjs seed-album SEED COUNT [REQUEST] [--name T]
//                                   an album of COUNT tracks drawn from SEED,
//                                   sharing REQUEST's picks
//
// As a module: `loopCode`, `readLoop`, `albumCode`, `seedAlbumCode`,
// `albumTracks`, `nameRefusal` (test/codes.test.mjs holds them to
// core/golden/share-codes.txt).

import { open } from './gen-lib.mjs';

let x = null;
const enc = new TextEncoder();
const dec = new TextDecoder();

function core() {
  if (!x) x = open();
  return x;
}

function put(bytes) {
  const c = core();
  new Uint8Array(c.memory.buffer, c.dl_code_input(), bytes.length).set(bytes);
  return bytes.length;
}

function text(len) {
  const c = core();
  return dec.decode(new Uint8Array(c.memory.buffer, c.dl_code_text(), len));
}

function refused() {
  const n = core().dl_code_refusal();
  return new Error(n ? text(n) : 'refused');
}

function ask(request) {
  const c = core();
  const t = enc.encode(request || '');
  new Uint8Array(c.memory.buffer, c.dl_tools_input(), t.length).set(t);
  if (!c.dl_tools_ask(t.length)) throw new Error(`the request is refused: ${request}`);
}

/** A loop's code: `seed`, `request` as text, `mutes` (a bit a layer), `salt` and its own `name`. */
export function loopCode(seed, request = '', mutes = 0, salt = 0, name = '') {
  ask(request);
  const n = core().dl_code_write(seed >>> 0, mutes, salt >>> 0, put(enc.encode(name)));
  if (!n) throw refused();
  return text(n);
}

/** A loop's code read: { seed, mutes, salt, name, request } (the request as text), or a refusal thrown. */
export function readLoop(code) {
  const c = core();
  if (!c.dl_code_read(put(enc.encode(code)))) throw refused();
  const out = { seed: c.dl_code_seed() >>> 0, mutes: c.dl_code_mutes(), salt: c.dl_code_salt() >>> 0 };
  out.name = text(c.dl_code_name());
  out.request = text(c.dl_code_request());
  return out;
}

/** Why a code can't carry `name` ("a name a code can't carry"), or null. */
export function nameRefusal(name) {
  return core().dl_code_name_ok(put(enc.encode(name))) ? null : refused().message;
}

/** The album of loops' `codes` (each with its pace after it where it has one: `CODE:t5025o`), named `name` (or none). */
export function albumCode(codes, name = '') {
  const t = enc.encode(name);
  const n = core().dl_code_album_write(put(new Uint8Array([...t, ...enc.encode(codes.join('.'))])), t.length);
  if (!n) throw refused();
  return text(n);
}

/** The album of `count` tracks drawn from `seed`, sharing `request`'s picks. */
export function seedAlbumCode(seed, count, request = '', name = '') {
  ask(request);
  const n = core().dl_code_album_seed(seed >>> 0, count, put(enc.encode(name)));
  if (!n) throw refused();
  return text(n);
}

/** A pace, [kind, count] as the player takes it (0 on and on, 3 a time, each with its fades' bits), as text: '', 'i', 't5025', 't120o'. */
export function paceText([kind, count]) {
  if ((kind & 3) === 3) return `t${count}${kind & 4 ? 'i' : ''}${kind & 8 ? 'o' : ''}`;
  return kind & 4 ? 'i' : '';
}

/**
 * An album's { name, tracks, paces, paced }: each track a loop's code, its
 * pace ([kind, count]), and the two as text (`CODE:t5025o`, or the code alone
 * where it plays on and on), as `albumCode` takes them.
 */
export function albumTracks(code) {
  const c = core();
  const len = put(enc.encode(code));
  const count = c.dl_code_album(len);
  if (!count) throw refused();
  const mem = new Uint8Array(c.memory.buffer, c.dl_code_text(), 64);
  const name = dec.decode(mem.slice(0, mem.indexOf(0)));
  const tracks = [];
  const paces = [];
  for (let i = 0; i < count; i++) {
    const n = c.dl_code_track(len, i);
    if (!n) throw refused();
    tracks.push(text(n));
    const kind = c.dl_code_pace(len, i);
    if (!kind) throw refused();
    paces.push([kind - 1, c.dl_code_pace_count() >>> 0]);
  }
  const paced = tracks.map((t, i) => (paces[i][0] ? `${t}:${paceText(paces[i])}` : t));
  return { name, tracks, paces, paced };
}

function main(argv) {
  const flag = (name) => {
    const i = argv.indexOf(name);
    if (i < 0) return null;
    const v = argv[i + 1];
    argv.splice(i, 2);
    return v;
  };
  const name = flag('--name') || '';
  const mutes = Number(flag('--mutes') || 0);
  const salt = Number(flag('--salt') || 0);
  const [what, ...rest] = argv;
  if (what === 'loop' && rest.length >= 1) {
    console.log(loopCode(Number(rest[0]), rest[1] || '', mutes, salt, name));
  } else if (what === 'read' && rest.length === 1) {
    const kind = (() => {
      put(enc.encode(rest[0]));
      return core().dl_code_kind(rest[0].length);
    })();
    if (kind === 2) {
      const a = albumTracks(rest[0]);
      if (a.name) console.log(`name: ${a.name}`);
      a.tracks.forEach((t, i) => {
        const l = readLoop(t);
        const pace = paceText(a.paces[i]);
        console.log(`${t}  seed ${l.seed}${pace ? `  plays ${a.paces[i][1]} ${a.paces[i][0] === 1 ? 'passes' : 'bars'}` : ''}`);
      });
    } else {
      console.log(JSON.stringify(readLoop(rest[0])));
    }
  } else if (what === 'album' && rest.length >= 1) {
    console.log(albumCode(rest, name));
  } else if (what === 'seed-album' && rest.length >= 2) {
    console.log(seedAlbumCode(Number(rest[0]), Number(rest[1]), rest[2] || '', name));
  } else {
    console.error('usage: node tools/codes.mjs loop SEED [REQUEST] | read CODE | album [--name T] CODE... | seed-album SEED COUNT [REQUEST] [--name T]');
    process.exit(2);
  }
}

if (import.meta.url === `file://${process.argv[1]}`) {
  try {
    main(process.argv.slice(2));
  } catch (e) {
    console.error(`codes: ${e.message}`);
    process.exit(1);
  }
}
