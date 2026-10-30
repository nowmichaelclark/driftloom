// The generation data's text format, and its reader (queue item 78).
//
// Profiles, vibes, moods, the voice table and the choice tables are plain
// text in data/, one format, read the same way by the JavaScript here and by
// the Rust generator (phase 2). Nothing here depends on a library, and the
// format needs nothing a `no_std` parser lacks: split lines, split on
// spaces and commas, parse a number.
//
// The format. Plain ASCII; one thing at a time:
//
//   # a comment runs from # to the end of the line, anywhere on a line
//   [kind name]                  starts a block (a profile, a vibe, ...)
//   key = token token, token     an entry of the block
//       more tokens              a line that starts with a space or a tab
//                                continues the entry above it
//
// A value is rows separated by commas; a row is tokens separated by spaces.
// A token is any run of printable ASCII but space, comma and '#'. A key is
// letters, digits, '_' and '.' (a dot names a part: `tone.warmth`). Entries
// keep their order, and a key may not repeat in a block. What a token means
// is up to the reader of that key, with these shared readings:
//
//   number   decimal ('0.05', '-3', '1e-3'), read as the one nearest double,
//            as any JavaScript literal is; or a ratio of two integers
//            ('1/3'), their quotient as one division, for a value that is a
//            third and not 0.3333
//   word     the token as it stands ('dorian', 'Dust', '3/4')
//   flag     'true' or 'false'
//
// So `scales = dorian 3, aeolian 3` is two rows of a word and a number, a
// weighted pool (the weights are relative), and `bpm = 61 88` one row of two
// numbers, a range. The readers in js/characters.js, js/vibes.js,
// js/moods.js, js/choices.js and js/synth.js say, per key, which they
// expect, and fail with the file, block and key when something is not.
//
// The numbers parse to exactly the doubles the old tables in code held, and
// a value that was computed (a ratio) stays one: the generation goldens
// (tools/gen-golden.mjs) hold the loops to that.

const FILES = ['characters', 'vibes', 'moods', 'voices', 'choices'];

// ------------------------------------------------------------- parse

/**
 * Parse a data file. Returns its blocks, in order: `{ kind, name, entries }`,
 * `entries` a list of `[key, rows]` (rows: lists of token strings). Throws,
 * naming `file` and the line, on anything malformed.
 */
export function parseData(text, file = 'data') {
  const blocks = [];
  let block = null;
  let entry = null;
  const fail = (n, why) => { throw new Error(`${file}:${n}: ${why}`); };
  const lines = text.split('\n');
  for (let i = 0; i < lines.length; i++) {
    const n = i + 1;
    let line = lines[i];
    if (line.endsWith('\r')) line = line.slice(0, -1);
    for (let k = 0; k < line.length; k++) {
      const c = line.charCodeAt(k);
      if (c > 126 || (c < 32 && c !== 9)) fail(n, 'plain printable ASCII only');
    }
    const hash = line.indexOf('#');
    if (hash >= 0) line = line.slice(0, hash);
    if (!line.trim()) continue;
    if (line[0] === ' ' || line[0] === '\t') {
      if (!entry) fail(n, 'a continuation with nothing to continue');
      entry.text += ' ' + line.trim();
      continue;
    }
    const head = /^\[([A-Za-z0-9_-]+) ([A-Za-z0-9_-]+)\]\s*$/.exec(line);
    if (head) {
      block = { kind: head[1], name: head[2], entries: [], line: n };
      blocks.push(block);
      entry = null;
      continue;
    }
    const m = /^([A-Za-z0-9_.]+)\s*=(.*)$/.exec(line);
    if (!m) fail(n, `not a block, an entry or a comment: ${line.trim()}`);
    if (!block) fail(n, 'an entry before any [kind name] block');
    if (block.entries.some((e) => e.key === m[1])) fail(n, `'${m[1]}' twice in [${block.kind} ${block.name}]`);
    entry = { key: m[1], text: m[2].trim(), line: n };
    block.entries.push(entry);
  }
  for (const b of blocks) {
    b.entries = b.entries.map((e) => [e.key, e.text.split(',').map((r) => r.trim()).filter(Boolean).map((r) => r.split(/\s+/))]);
  }
  return blocks;
}

// ------------------------------------------------------------- readings

const NUMBER = /^-?(\d+\.?\d*|\.\d+)(e[-+]?\d+)?$/i;
const RATIO = /^(-?\d+)\/(\d+)$/;

/** A token as a number (see the format above). Throws on anything else. */
export function number(token) {
  const q = RATIO.exec(token);
  if (q) return Number(q[1]) / Number(q[2]);
  if (!NUMBER.test(token)) throw new Error(`'${token}' is not a number`);
  return Number(token);
}

/** A token as a flag. */
export function flag(token) {
  if (token === 'true') return true;
  if (token === 'false') return false;
  throw new Error(`'${token}' is not true or false`);
}

const one = (rows) => {
  if (rows.length !== 1 || rows[0].length !== 1) throw new Error('expected one token');
  return rows[0][0];
};
const oneRow = (rows) => {
  if (rows.length !== 1) throw new Error('expected one row');
  return rows[0];
};

/** The readers, by what a key holds. Each takes the entry's rows. */
export const READ = {
  /** One number. */
  num: (rows) => number(one(rows)),
  /** One word. */
  word: (rows) => one(rows),
  /** One flag. */
  flag: (rows) => flag(one(rows)),
  /** A range: one row of two numbers. */
  range: (rows) => {
    const r = oneRow(rows);
    if (r.length !== 2) throw new Error('expected two numbers');
    return [number(r[0]), number(r[1])];
  },
  /** A list of words, in one row. */
  words: (rows) => (rows.length ? oneRow(rows).slice() : []),
  /** A weighted pool of words: rows of `word number`, as [word, number]. */
  pool: (rows) => rows.map((r) => {
    if (r.length !== 2) throw new Error('expected a word and a number');
    return [r[0], number(r[1])];
  }),
  /** A weighted pool of numbers: rows of `number number`, as [number, number]. */
  numpool: (rows) => rows.map((r) => {
    if (r.length !== 2) throw new Error('expected two numbers');
    return [number(r[0]), number(r[1])];
  }),
  /** A table of numbers by word: rows of `word number`, as { word: number }. */
  table: (rows) => Object.fromEntries(READ.pool(rows)),
  /** A table of numbers by number: rows of `number number`, as { number: number }. */
  numtable: (rows) => Object.fromEntries(READ.numpool(rows)),
  /** Rows of numbers. */
  grid: (rows) => rows.map((r) => r.map(number)),
};

/** The block's entries as a Map of key to rows. */
export function entriesOf(block) {
  return new Map(block.entries);
}

/**
 * Read a block against a schema `{ key: reader }` (reader a name in READ or
 * a function of the rows), into `{ key: value }` in the block's own order.
 * A key with a dot (`tone.warmth`) reads into a part of an object, made at
 * the key's first appearance: its schema entry is the whole key, or
 * `'tone.*'` for any part. A key the schema doesn't know throws,
 * naming file, block and key: a typo never goes quiet. `where` names the
 * file for the message.
 */
export function readBlock(block, schema, where = 'data') {
  const out = {};
  const label = `${where}: [${block.kind} ${block.name}]`;
  for (const [key, rows] of block.entries) {
    const dot = key.indexOf('.');
    const head = dot < 0 ? key : key.slice(0, dot);
    const part = dot < 0 ? '' : key.slice(dot + 1);
    const reader = schema[key] || (dot < 0 ? undefined : schema[`${head}.*`]);
    if (!reader) throw new Error(`${label} has no key '${key}'`);
    try {
      const value = (typeof reader === 'function' ? reader : READ[reader])(rows);
      if (dot < 0) out[head] = value;
      else (out[head] ??= {})[part] = value;
    } catch (err) {
      throw new Error(`${label} ${key}: ${err.message}`);
    }
  }
  return out;
}

// ------------------------------------------------------------- loading

const IN_NODE = typeof window === 'undefined' && typeof process !== 'undefined' && !!(process.versions && process.versions.node);

/**
 * The text of a data file (data/NAME.txt): read from disk in Node, fetched
 * in a browser (the page at start; sw.js caches the files). Resolved against
 * this module, so it works wherever the app is served from.
 */
async function readText(name) {
  const url = new URL(`../data/${name}.txt`, import.meta.url);
  if (IN_NODE) {
    const { readFile } = await import('node:fs/promises');
    return readFile(url, 'utf8');
  }
  const res = await fetch(url);
  if (!res.ok) throw new Error(`data/${name}.txt: ${res.status}`);
  return res.text();
}

/** Every data file, parsed: `{ characters: [blocks], vibes: [...], ... }`. */
export const DATA = Object.fromEntries(await Promise.all(FILES.map(
  async (name) => [name, parseData(await readText(name), `data/${name}.txt`)],
)));

/** The blocks of a file of one kind. */
export function blocksOf(file, kind) {
  return DATA[file].filter((b) => b.kind === kind);
}

/** The one block of a file of a kind and name. Throws if missing. */
export function blockOf(file, kind, name) {
  const b = DATA[file].find((x) => x.kind === kind && x.name === name);
  if (!b) throw new Error(`data/${file}.txt has no [${kind} ${name}]`);
  return b;
}

export { FILES };
