// The generation goldens' text and hashes (queue item 77): the canonical
// text the core's generator writes its parts in, read back here, the hash
// taken over it, and the file's cases. tools/gen-golden.mjs documents the
// file, the text and the cases; tools/gen-lib.mjs reads the parts from the
// core. Plain JavaScript with no imports, so anything may read it.

// ------------------------------------------------------- canonical text

const F = new Float64Array(1);
const U = new Uint32Array(F.buffer);
const LE = new Uint8Array(new Uint16Array([1]).buffer)[0] === 1;
const hex8 = (n) => (n >>> 0).toString(16).padStart(8, '0');

function number(x) {
  if (Number.isInteger(x) && Math.abs(x) <= 2 ** 53) return Object.is(x, -0) ? '-0' : String(x);
  if (x !== x) return 'nan';
  if (x === Infinity) return 'inf';
  if (x === -Infinity) return '-inf';
  F[0] = x;
  return `x${hex8(U[LE ? 1 : 0])}${hex8(U[LE ? 0 : 1])}`;
}

/** The canonical text of a value (tools/gen-golden.mjs, "The canonical text"). */
export function canonical(v) {
  if (v === null) return 'null';
  if (v === undefined) return 'undef';
  switch (typeof v) {
    case 'number': return number(v);
    case 'boolean': return v ? 'true' : 'false';
    case 'string':
      if (!/^[\x20-\x7e]*$/.test(v)) throw new Error(`not printable ASCII: ${JSON.stringify(v)}`);
      return `"${v.replace(/[\\"]/g, (c) => `\\${c}`)}"`;
    case 'object': {
      if (Array.isArray(v)) return `[${v.map(canonical).join(',')}]`;
      if (Object.getPrototypeOf(v) !== Object.prototype) throw new Error(`not a plain object: ${v.constructor && v.constructor.name}`);
      const keys = Object.keys(v).filter((k) => v[k] !== undefined).sort();
      return `{${keys.map((k) => `${k}:${canonical(v[k])}`).join(',')}}`;
    }
    default: throw new Error(`can't write a ${typeof v}`);
  }
}

/** FNV-1a, 64 bits, over a string of ASCII, as 16 hex digits. */
export function fnv1a64(text) {
  // The state in two 32-bit halves; the prime is 2^40 + 0x1b3, so a step
  // is the state times 0x1b3, plus its low half shifted into the high one
  // by 8 (40 - 32). Every product stays below 2^53, so it is exact.
  let hi = 0xcbf29ce4;
  let lo = 0x84222325;
  for (let i = 0; i < text.length; i++) {
    const c = text.charCodeAt(i);
    if (c > 0x7f) throw new Error('not ASCII');
    lo = (lo ^ c) >>> 0;
    const l = lo * 0x1b3;
    const carry = Math.floor(l / 4294967296);
    hi = (hi * 0x1b3 + carry + ((lo << 8) >>> 0)) >>> 0;
    lo = l >>> 0;
  }
  return hex8(hi) + hex8(lo);
}

export const hashOf = (v) => fnv1a64(canonical(v));

// ------------------------------------------------------------ the cases

/**
 * The cases of a goldens file's text, in its order: { set, index, name,
 * seed, request (text, or null for nothing asked), want: { spec, ...,
 * passes: [..] } (the hashes on the line; empty for a line with none yet) }.
 * The file is the list of cases: a new one is a line of its first three
 * fields, and `node tools/gen-golden.mjs` writes its hashes.
 */
export function casesOf(text) {
  const out = [];
  for (const line of text.split('\n')) {
    if (!line || line.startsWith('#')) continue;
    const [name, seed, asked, ...fields] = line.split(' ');
    const want = Object.fromEntries(fields.map((f) => [f.slice(0, f.indexOf('=')), f.slice(f.indexOf('=') + 1)]));
    if (want.passes) want.passes = want.passes.split(',');
    const at = name.indexOf(':');
    out.push({ set: name.slice(0, at), index: name.slice(at + 1), name, seed: Number(seed) >>> 0, request: asked === '-' ? null : asked, want });
  }
  return out;
}

/** The file's line for a case and its hashes ({ spec, ..., passes: [..] }). */
export function lineOf(c, h) {
  const named = Object.entries(h).filter(([k]) => k !== 'passes').map(([k, v]) => `${k}=${v}`);
  return [c.name, c.seed >>> 0, c.request ?? '-', ...named, `passes=${h.passes.join(',')}`].join(' ');
}

// ------------------------------------------------------- reading it back

/**
 * A canonical text read back into a value (the inverse of `canonical`, for
 * tools that read what the Rust generator made: `tools/stats.mjs --core`).
 * An object's keys come back in the text's (sorted) order.
 */
export function parseCanonical(text) {
  let i = 0;
  const fail = (why) => { throw new Error(`canonical text, at ${i}: ${why}`); };
  const word = () => {
    const start = i;
    while (i < text.length && /[A-Za-z0-9_.+-]/.test(text[i])) i++;
    return text.slice(start, i);
  };
  const value = () => {
    const c = text[i];
    if (c === '{') {
      i++;
      const out = {};
      if (text[i] === '}') { i++; return out; }
      for (;;) {
        const k = word();
        if (text[i++] !== ':') fail('expected :');
        out[k] = value();
        if (text[i] === ',') { i++; continue; }
        if (text[i++] !== '}') fail('expected }');
        return out;
      }
    }
    if (c === '[') {
      i++;
      const out = [];
      if (text[i] === ']') { i++; return out; }
      for (;;) {
        out.push(value());
        if (text[i] === ',') { i++; continue; }
        if (text[i++] !== ']') fail('expected ]');
        return out;
      }
    }
    if (c === '"') {
      let s = '';
      i++;
      while (text[i] !== '"') {
        if (i >= text.length) fail('an unended string');
        if (text[i] === '\\') i++;
        s += text[i++];
      }
      i++;
      return s;
    }
    const w = word();
    switch (w) {
      case 'null': return null;
      case 'undef': return undefined;
      case 'true': return true;
      case 'false': return false;
      case 'nan': return NaN;
      case 'inf': return Infinity;
      case '-inf': return -Infinity;
      case '-0': return -0;
      default:
        if (/^x[0-9a-f]{16}$/.test(w)) {
          U[LE ? 1 : 0] = parseInt(w.slice(1, 9), 16);
          U[LE ? 0 : 1] = parseInt(w.slice(9), 16);
          return F[0];
        }
        if (/^-?\d+$/.test(w)) return Number(w);
        return fail(`not a value: ${w}`);
    }
  };
  const v = value();
  if (i !== text.length) fail('text after the value');
  return v;
}
