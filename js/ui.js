import { LAYERS, LAYER_LABELS, STEPS_PER_BAR, loopBars } from './request.js';
import { CHARACTERS, resolveKey } from './characters.js';
import { NOTE_NAMES, SCALES } from './theory.js';
import { moodWord, pointWord } from './moods.js';
import { appLink } from './app.js';

export const el = (id) => document.getElementById(id);

// ------------------------------------------------------------- readout

// The grid is rebuilt whenever the metre changes, because a bar of 6/8 is
// twelve cells, not sixteen with four dead ones on the end.
export function buildPlayhead(steps = STEPS_PER_BAR) {
  const host = el('playhead');
  host.innerHTML = '';
  host.style.gridTemplateColumns = `repeat(${steps}, 1fr)`;
  for (let i = 0; i < steps; i++) {
    const light = document.createElement('i');
    if (i % 4 === 0) light.className = 'beat';
    host.appendChild(light);
  }
  return host.children;
}

const METER = { 12: '6/8', 16: '4/4', 20: '5/4', 14: '7/8' };

// Both the profile blend and the feeling are mixtures, so show them as
// mixtures. Percentages are more honest than one adjective standing in for a
// loop that is 60% one thing and 30% another.
function asMixParts(weights, nameOf, max = 3) {
  return Object.entries(weights)
    .filter(([, w]) => w >= 0.05)
    .sort((a, b) => b[1] - a[1])
    .slice(0, max)
    .map(([k, w]) => `${Math.round(w * 100)}% ${nameOf(k)}`);
}

function mixParts(spec) {
  if (!spec.mix) {
    const c = CHARACTERS[resolveKey(spec.character || 'dust')];
    return [c ? c.label : 'Dust'];
  }
  const named = {};
  for (const [k, w] of Object.entries(spec.mix)) {
    const key = resolveKey(k);
    named[key] = (named[key] || 0) + w;
  }
  const parts = asMixParts(named, (k) => CHARACTERS[k].label);
  return parts.length ? parts : ['Dust'];
}

function feelParts(spec) {
  // A loop drawn with vibes (queue item 38b) shows what it was steered by,
  // weights included, in place of the mood line. Steered, not measured:
  // the honest share of the sound comes with the dials (items 34-35).
  if (spec.vibes && Object.keys(spec.vibes).length) {
    const parts = asMixParts(spec.vibes, (k) => k);
    if (parts.length) return parts;
  }
  if (spec.feelMix && Object.keys(spec.feelMix).length) {
    return asMixParts(spec.feelMix, moodWord);
  }
  // Saves from before feeling became a mixture.
  return [pointWord(spec.feel || { lift: spec.mood ?? 0.5, energy: 0.5, warmth: 0.6 })];
}

// The card's details are facts, each on a line of its own and each whole:
// the blend, the feeling, the key, the tempo, the meter and the bars. The
// card is as tall as its art and as wide as its column, so a loop with a
// lot to say shows what fits and leaves the rest off, whole, in a fixed
// order of what matters least (session 30d, item 112); a tap on the details
// shows the rest in the same place, and a quiet mark says there is more.
// Every fact is always in the page for a screen reader (`#loopFacts`).
//
// A fact is atoms (the tempo and the loop's shape word; the meter and the
// bars), each with a rank: what is left off first has the lowest. A fact
// whose atoms fit side by side is one line; if they don't they are lines of
// their own. In that order of rank:
//   key, tempo, bars, meter, the first style, the album playing, the first
//   feeling, the second style, the second feeling, the shape word, the
//   third style, the third feeling, the pass count.
// The first three are the loop (never off before the rest), the last is
// what goes first.
const RANK = {
  key: 100, tempo: 99, bars: 98, meter: 97, mix: [96, 93, 90], album: 95,
  feel: [94, 92, 89], shape: 91, pass: 80,
};

// An atom may carry `alts`: shorter words for the same fact, used only where
// the column is too narrow for the full text on its own (a fact is never
// cut; it is said in fewer words).
const atom = (text, rank, alts = []) => ({ text, rank, alts, shown: text });
// "40% Undertow" shortened to "Undertow".
const bare = (part) => [part.replace(/^\d+% /, '')];

// The facts of a loop, in the order the card lays them out.
function loopFacts(spec, pattern, extra) {
  const meter = METER[spec.stepsPerBar || 16] || `${spec.stepsPerBar}/16`;
  const shape = [];
  if (pattern && pattern.form) shape.push('airy');
  if (spec.cycles) shape.push('drifting');
  const facts = [];
  mixParts(spec).forEach((m, i) => facts.push({ cls: 'mix', atoms: [atom(m, RANK.mix[i], bare(m))] }));
  feelParts(spec).forEach((f, i) => facts.push({ cls: 'feel', atoms: [atom(f, RANK.feel[i], bare(f))] }));
  facts.push({ pack: true, atoms: [atom(`${NOTE_NAMES[spec.root]} ${SCALES[spec.scale].label}`, RANK.key)] });
  const tempo = [atom(`${spec.bpm} bpm`, RANK.tempo)];
  if (shape.length) tempo.push(atom(shape.join(' · '), RANK.shape));
  facts.push({ pack: true, atoms: tempo });
  facts.push({ pack: true, atoms: [atom(meter, RANK.meter), atom(barsText(spec), RANK.bars)] });
  if (extra.album) facts.push({ cls: 'album-line', atoms: [atom(extra.album, RANK.album, albumAlts(extra.album))] });
  if (extra.pass) facts.push({ cls: 'faint', atoms: [atom(extra.pass, RANK.pass)] });
  return facts;
}

// "Rainy Day Mix 1/3": the title alone, then the place alone.
function albumAlts(line) {
  const m = /^(.*) (\d+\/\d+)$/.exec(line);
  return m ? [m[1], m[2]] : [];
}

// The card's words for a loop: `extra` adds the album being played and the
// pass count when a track length is set; both are what someone glancing at
// the card would otherwise have to go looking for.
export function renderReadout(spec, pattern, extra = {}) {
  setName(spec.name);
  setFacts(loopFacts(spec, pattern, extra));
  const bpm = el('bpmVal');
  if (bpm) bpm.textContent = spec.bpm;
}

// The card turned over to show an album: its art is drawn by the caller,
// this fills in the words.
export function renderAlbumCard(title, specs, extra = {}) {
  setName(title);
  const facts = [{ cls: 'mix', atoms: [atom(`${specs.length} ${specs.length === 1 ? 'track' : 'tracks'}`, 99)] }];
  if (specs.length) {
    // The album's blend is the average of its loops' blends.
    const named = {};
    for (const spec of specs) {
      const mix = spec.mix || { [spec.character || 'dust']: 1 };
      for (const [k, w] of Object.entries(mix)) {
        const key = resolveKey(k);
        named[key] = (named[key] || 0) + w / specs.length;
      }
    }
    asMixParts(named, (k) => (CHARACTERS[k] ? CHARACTERS[k].label : k), 2)
      .forEach((part, i) => facts.push({ cls: 'feel', atoms: [atom(part, 94 - 2 * i, bare(part))] }));
    const bpms = specs.map((s) => s.bpm);
    const lo = Math.min(...bpms);
    const hi = Math.max(...bpms);
    facts.push({ atoms: [atom(lo === hi ? `${lo} bpm` : `${lo}-${hi} bpm`, 98)] });
    const secs = specs.reduce((t, s) => t + passSeconds(s), 0);
    facts.push({ atoms: [atom(Number.isFinite(secs) ? `~${formatTime(secs)} once through` : 'endless', 97)] });
  }
  if (extra.playing) facts.push({ cls: 'album-line', atoms: [atom(extra.playing, 96)] });
  setFacts(facts);
}

// How many bars the loop is, as the card shows it (queue item 88b): the
// number the Bars picker asked for ("33 bars", "1,024 bars"), or Endless;
// else the loop's own.
function barsText(spec) {
  const n = loopBars(spec);
  if (!Number.isFinite(n)) return 'endless';
  return `${String(n).replace(/\B(?=(\d{3})+(?!\d))/g, ',')} ${n === 1 ? 'bar' : 'bars'}`;
}

// A piece's length for a Library list (queue item 106b): its bars and, where
// a length was asked, its duration at its tempo, from "4 bars · 12s" to
// "32,768 bars · 27h 18m"; "endless" for Endless.
export function lengthLine(spec) {
  if (!spec) return '';
  const n = loopBars(spec);
  if (!Number.isFinite(n)) return 'endless';
  const bars = `${String(n).replace(/\B(?=(\d{3})+(?!\d))/g, ',')}\u00a0${n === 1 ? 'bar' : 'bars'}`;
  if (spec.length == null) return bars;
  // Past a day it stays in hours ("30h"): a 32,768-bar piece is about a day.
  const secs = passSeconds(spec);
  return `${bars} · ${secs >= 86400 ? `${Math.round(secs / 3600)}h` : formatTime(secs)}`;
}

// One pass of the loop, in seconds: every bar it is (a long loop's every
// section; Infinity for Endless).
export function passSeconds(spec) {
  return loopBars(spec) * (spec.stepsPerBar || 16) * (60 / spec.bpm / 4);
}

// One bar, in seconds.
export function barSeconds(spec) {
  return (spec.stepsPerBar || 16) * (60 / spec.bpm / 4);
}

// Track length is chosen as a time, which is how anyone thinks of it, and
// stored as passes (a track ends on a whole loop), worked out at this tempo
// when the slider moves; a time shorter than one pass (a long loop's pass
// can be hours) as bars, and the track ends at the first phrase line after
// them (queue item 88b; `trackLength`). The stops are curated: fine at the
// short end, where people actually set it, and reaching a whole day at the
// top. Here, beside the passes and bars they come to, so a test walks them
// (session 24b: an album's code carries each).
export const TRACK_TIMES = [
  0, 60, 120, 180, 300, 600, 900, 1200, 1800, 2700,
  3600, 5400, 7200, 10800, 14400, 21600, 28800, 43200, 86400,
];

// The slider's time, `secs` (0 for off), as the spec keeps it: whole passes
// where it is a pass or longer, as ever; shorter than a pass, the bars it
// comes to, rounded up (queue item 88b: the track ends at the first phrase
// line after them, its notes fading).
export function trackLength(spec, secs) {
  spec.playFor = null;
  delete spec.playBars;
  if (!secs) return;
  const pass = passSeconds(spec);
  if (secs >= pass) spec.playFor = Math.max(1, Math.round(secs / pass));
  else spec.playBars = Math.max(1, Math.ceil(secs / barSeconds(spec) - 1e-9));
}

export function formatTime(secs) {
  if (secs < 90) return `${Math.round(secs)}s`;
  if (secs < 5400) return `${Math.round(secs / 60)}m`;
  if (secs < 86400) {
    const h = Math.floor(secs / 3600);
    const m = Math.round((secs % 3600) / 60);
    return m ? `${h}h ${m}m` : `${h}h`;
  }
  const d = Math.floor(secs / 86400);
  const h = Math.round((secs % 86400) / 3600);
  return h ? `${d}d ${h}h` : `${d}d`;
}

// The name is as big as fits on one line of the card, down to a floor;
// only a name too long even at the floor wraps. Breaking "mun-dain" at its
// hyphen to keep a size reads worse than a slightly smaller name.
export function setName(name) {
  const h = el('loopName');
  h.textContent = name;
  fitName();
}

export function fitName() {
  const h = el('loopName');
  if (h.hidden || !h.clientWidth) return;
  h.classList.remove('wrap', 'break');
  h.style.fontSize = '';
  const max = parseFloat(getComputedStyle(h).fontSize) || 32;
  let size = max;
  const set = (px) => { size = px; h.style.fontSize = `${px}px`; };
  // One line, as large as fits, down to a size that still reads as a title.
  const oneLine = () => h.scrollWidth <= h.clientWidth + 1;
  while (!oneLine() && size > NAME_ONE_LINE_MIN) set(size - 1);
  if (oneLine()) return;
  // Otherwise two lines, as large as lets both fit. Names are capped at
  // NAME_MAX characters, so this always lands well above the floor; the
  // CSS clamps to two lines in case an old, longer name does not.
  h.classList.add('wrap');
  set(max);
  const twoLines = () => h.scrollWidth <= h.clientWidth + 1
    && h.scrollHeight <= Math.ceil(size * 1.1 * 2) + 2;
  while (!twoLines() && size > NAME_TWO_LINE_MIN) set(size - 1);
  // One unbroken word too long even at the floor: only then split it.
  h.classList.toggle('break', !twoLines());
}

const NAME_ONE_LINE_MIN = 21;
const NAME_TWO_LINE_MIN = 14;

// Refit when the card changes width (rotation, a resized window, fonts
// arriving late), not just when the name changes.
if (typeof ResizeObserver !== 'undefined') {
  let lastWidth = 0;
  const ro = new ResizeObserver((entries) => {
    const w = entries[0].contentRect.width;
    if (Math.abs(w - lastWidth) < 1) return;
    lastWidth = w;
    fitCard();
  });
  const watch = () => {
    const host = el('loopName');
    if (host && host.parentElement) ro.observe(host.parentElement);
  };
  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', watch);
  else watch();
}
// Late fonts change what a line measures.
if (typeof document !== 'undefined' && document.fonts && document.fonts.ready) {
  document.fonts.ready.then(() => fitCard());
}

// What the card is showing: its facts, the page of them shown (the rest
// are a tap away), and a signature so a card drawn again for the same loop
// stays on the page it was on.
const card = { facts: [], sig: '', page: 0, pages: 1 };
const SEP = ' \u00b7 ';
const GUTTER = 14;
const MAX_PAGES = 12;
const DOT_PITCH = 10; // a page's dot (6px) and its gap

function setFacts(facts) {
  const sig = facts.map((f) => f.atoms.map((a) => a.text).join('|')).join('\n');
  if (sig !== card.sig) card.page = 0;
  card.sig = sig;
  card.facts = facts;
  // For a screen reader: every fact, whatever is shown.
  const all = el('loopFacts');
  if (all) {
    all.innerHTML = '';
    for (const f of facts) {
      const li = document.createElement('li');
      li.textContent = f.atoms.map((a) => a.text).join(SEP);
      all.appendChild(li);
    }
  }
  layoutDetails();
}

// How wide each atom is set in the card's own type (the style of its line),
// and the separator.
function measureAtoms(host, facts) {
  const probes = [];
  const make = (text, cls) => {
    const li = document.createElement('li');
    li.className = cls || '';
    li.textContent = text;
    li.style.cssText = 'position:absolute;visibility:hidden;white-space:pre;left:0;top:0;';
    host.appendChild(li);
    probes.push(li);
    return li;
  };
  const items = [];
  for (const f of facts) {
    for (const a of f.atoms) {
      items.push([a, make(a.text, f.cls)]);
      a.altProbes = a.alts.map((t) => make(t, f.cls));
    }
  }
  const sep = make(SEP, '');
  const tall = make('Ag', '');
  const out = { sep: 0, line: 0 };
  for (const [a, li] of items) {
    a.w0 = li.getBoundingClientRect().width + 0.5;
    a.altW = a.altProbes.map((p) => p.getBoundingClientRect().width + 0.5);
    delete a.altProbes;
  }
  out.sep = sep.getBoundingClientRect().width;
  out.line = tall.getBoundingClientRect().height;
  for (const li of probes) li.remove();
  return out;
}

// The lines `keep` makes: a fact is a line when its atoms fit side by side,
// else a line to each atom; where height is short (`packed`), neighbouring
// facts of the tail (key, tempo, meter and bars) share a line too.
function compose(facts, keep, width, sepW, packed) {
  const lines = [];
  for (const f of facts) {
    const atoms = f.atoms.filter((a) => keep.has(a));
    if (!atoms.length) continue;
    const joined = atoms.reduce((t, a) => t + a.w, 0) + sepW * (atoms.length - 1);
    const groups = joined <= width ? [atoms] : atoms.map((a) => [a]);
    for (const g of groups) {
      const w = g.reduce((t, a) => t + a.w, 0) + sepW * (g.length - 1);
      const prev = lines[lines.length - 1];
      if (packed && f.pack && prev && prev.pack && prev.w + sepW + w <= width) {
        prev.atoms.push(...g);
        prev.w += sepW + w;
      } else {
        lines.push({ cls: f.cls, pack: f.pack, atoms: g, w });
      }
    }
  }
  return lines;
}

// The facts as pages of whole lines: the first holds what matters most that
// fits; each tap shows the next page of what was left off. A fact wider
// than the column on its own can be on no page (the screen reader has it).
function paginate(facts, width, maxLines, sepW) {
  let rest = new Set();
  for (const f of facts) for (const a of f.atoms) if (a.w <= width) rest.add(a);
  const pages = [];
  while (rest.size && pages.length < MAX_PAGES) {
    const keep = new Set(rest);
    let lines;
    for (;;) {
      lines = compose(card.facts, keep, width, sepW, false);
      // Lines of the tail share a line only when the card is so short that
      // even the loop's own facts would not otherwise fit.
      if (lines.length > maxLines && [...keep].every((a) => a.rank >= RANK.meter)) {
        lines = compose(card.facts, keep, width, sepW, true);
      }
      if (lines.length <= maxLines || keep.size <= 1) break;
      let least = null;
      for (const a of keep) if (!least || a.rank <= least.rank) least = a;
      keep.delete(least);
    }
    if (lines.length > maxLines) break;
    pages.push(lines);
    rest = new Set([...rest].filter((a) => !keep.has(a)));
  }
  return pages;
}

// Each fact in its full words where they fit the column, else in its
// shortest words that do (a fact too wide even so is on no page).
function fitAndPaginate(width, maxLines, sepW) {
  for (const f of card.facts) {
    for (const a of f.atoms) {
      a.shown = a.text;
      a.w = a.w0;
      if (a.w0 > width) {
        const i = a.altW.findIndex((w) => w <= width);
        if (i >= 0) { a.shown = a.alts[i]; a.w = a.altW[i]; }
      }
    }
  }
  return paginate(card.facts, width, maxLines, sepW);
}

function layoutDetails() {
  const host = el('loopDetail');
  const wrap = host && host.parentElement;
  if (!wrap || !card.facts.length) return;
  const col = wrap.parentElement;
  if (!wrap.clientWidth || !col.clientHeight) return;
  const name = el('loopName');
  const gap = parseFloat(getComputedStyle(col).rowGap) || 0;
  const used = (name.hidden ? 0 : name.offsetHeight) + gap;
  wrap.classList.remove('paged');
  const m = measureAtoms(host, card.facts);
  const maxLines = Math.floor((col.clientHeight - used + 1) / (m.line || 16));
  let pages = fitAndPaginate(host.clientWidth, maxLines, m.sep);
  if (pages.length > 1) {
    // The mark for "more" takes a strip of its own at the side.
    wrap.classList.add('paged');
    pages = fitAndPaginate(host.clientWidth, maxLines, m.sep);
  }
  card.pages = Math.max(1, pages.length);
  if (card.page >= card.pages) card.page = 0;
  host.innerHTML = '';
  for (const line of pages[card.page] || []) {
    const li = document.createElement('li');
    li.textContent = line.atoms.map((a) => a.shown).join(SEP);
    if (line.cls) li.className = line.cls;
    host.appendChild(li);
  }
  paintMore();
}

// The quiet mark: a dot a page, the page shown filled; the details are
// a button where there is more to show.
function paintMore() {
  const more = el('metaMore');
  if (!more) return;
  const dots = el('metaDots');
  more.hidden = card.pages < 2;
  dots.innerHTML = '';
  dots.classList.remove('track');
  if (card.pages < 2) return;
  more.dataset.pages = String(card.pages);
  // Where the dots do not all fit the details' height, the mark is a thin
  // track with a thumb on the page shown, so it is never cut off.
  const room = el('metaWrap').clientHeight - 2;
  if (card.pages * DOT_PITCH - 4 > room) {
    dots.classList.add('track');
    const thumb = document.createElement('i');
    thumb.style.top = `${(card.page / card.pages) * 100}%`;
    thumb.style.height = `${100 / card.pages}%`;
    dots.appendChild(thumb);
  } else {
    for (let i = 0; i < card.pages; i++) {
      const d = document.createElement('i');
      if (i === card.page) d.className = 'on';
      dots.appendChild(d);
    }
  }
  more.setAttribute('aria-label', `More details, ${card.page + 1} of ${card.pages}`);
}

// A tap on the details: the next page of them, the last back to the first.
export function moreDetails() {
  if (card.pages < 2) return;
  card.page = (card.page + 1) % card.pages;
  layoutDetails();
}

export function fitCard() {
  fitName();
  layoutDetails();
}

// -------------------------------------------------------------- layers

// Each layer shows one bar at sixteenth resolution, as a row of dots (the
// v17 design): a quiet guide dot on every step, a note where one sounds, a
// bigger note for an accent. The playhead's column gets a halo on every
// layer that is on. Showing the whole loop at once would squash a four-bar
// pattern into slivers on a phone, so the rows follow the playhead bar by
// bar instead.
let columns = [];

export function buildLayers(handlers, steps = STEPS_PER_BAR) {
  const host = el('layerList');
  host.innerHTML = '';
  const cells = {};
  columns = Array.from({ length: steps }, () => []);

  for (const layer of LAYERS) {
    const label = LAYER_LABELS[layer];
    const row = document.createElement('div');
    row.className = 'layer';
    row.dataset.layer = layer;

    const name = document.createElement('div');
    name.className = 'lbl';
    name.id = `lbl-${layer}`;
    name.textContent = label;

    const grid = document.createElement('div');
    grid.className = 'pattern';
    grid.setAttribute('role', 'img');
    grid.setAttribute('aria-label', `${label} pattern`);
    grid.style.gridTemplateColumns = `repeat(${steps}, 1fr)`;
    const dots = [];
    for (let i = 0; i < steps; i++) {
      const d = document.createElement('span');
      grid.appendChild(d);
      dots.push(d);
      columns[i].push(d);
    }
    cells[layer] = dots;

    const reroll = document.createElement('button');
    reroll.type = 'button';
    reroll.className = 'btn-roll';
    reroll.textContent = 'roll';
    reroll.setAttribute('aria-label', `Roll ${label}`);
    reroll.addEventListener('click', () => handlers.onReroll(layer));

    const mute = document.createElement('button');
    mute.type = 'button';
    mute.className = 'btn-on';
    mute.setAttribute('role', 'switch');
    mute.setAttribute('aria-checked', 'true');
    mute.setAttribute('aria-labelledby', name.id);
    mute.innerHTML = '<span aria-hidden="true">on</span>';
    mute.addEventListener('click', () => handlers.onMute(layer));

    row.append(name, grid, reroll, mute);
    host.appendChild(row);
  }
  lastCursor = -1;
  return cells;
}

function stepsForBar(pattern, layer, bar) {
  const spb = pattern.stepsPerBar || STEPS_PER_BAR;
  const out = new Array(spb).fill(0);
  // With polymeter a layer wraps on its own cycle, so ask where it actually
  // is rather than assuming it shares the pattern's bar lines.
  const cycle = (pattern.cycles && pattern.cycles[layer]) || pattern.totalSteps;
  const start = (bar * spb) % cycle;
  for (const e of pattern.tracks[layer]) {
    if (!e.vel) continue;
    const idx = ((e.step - start) % cycle + cycle) % cycle;
    if (idx >= spb) continue;
    out[idx] = Math.max(out[idx], e.vel);
  }
  return out;
}

// The layers' grid as composed (a loop not playing, or before the core's
// first events): each layer's loudest note on each step of bar `bar`.
export function renderGrids(cells, pattern, bar, mutes) {
  const loud = LAYERS.map((layer) => stepsForBar(pattern, layer, bar));
  renderRows(cells, { loud, share: null }, mutes);
}

// How bright a dot is drawn for the share of its level it is heard at
// (session 30b): its own at a full share, dimmer as a long piece's plan, a
// late arrival or a fade has it softer, never under SHADE_LEAST (still a
// note). In tenths, so a gliding level writes a dot's style only when it
// moves a tenth. For Mikey's eyes.
const SHADE_LEAST = 0.3;
function shadeOf(share) {
  if (share == null || !(share < 0.95)) return '';
  return String(Math.round(10 * Math.max(SHADE_LEAST, share)) / 10);
}

// The layers drawn from `rows` (js/heard.js's HeardSteps.bar: each layer's
// loudness on each step of the bar, 0 for none, and the share it is heard
// at, in the layers' order; `share` null for the grid as composed): a dot
// for a note, a bigger one for an accent (as loud as heard), dimmer as its
// share is lower.
export function renderRows(cells, rows, mutes) {
  LAYERS.forEach((layer, l) => {
    const vels = rows.loud[l] || [];
    const shares = rows.share ? rows.share[l] : null;
    const dots = cells[layer];
    let notes = 0;
    for (let i = 0; i < dots.length; i++) {
      const v = vels[i] || 0;
      if (v) notes++;
      const on = !!v;
      const big = v > 0.55;
      const shade = on && shares ? shadeOf(shares[i]) : '';
      // Only the dots that change. Writing a class, even the same one,
      // marks the element for style and paint.
      if (dots[i].classList.contains('on') !== on) dots[i].classList.toggle('on', on);
      if (dots[i].classList.contains('big') !== big) dots[i].classList.toggle('big', big);
      if ((dots[i].dataset.shade || '') !== shade) {
        if (shade) {
          dots[i].dataset.shade = shade;
          dots[i].style.setProperty('--heard', shade);
        } else {
          delete dots[i].dataset.shade;
          dots[i].style.removeProperty('--heard');
        }
      }
    }
    const row = document.querySelector(`.layer[data-layer="${layer}"]`);
    const off = !!mutes[layer];
    if (row.classList.contains('dim') !== off) row.classList.toggle('dim', off);
    const sw = row.querySelector('.btn-on');
    const want = off ? 'false' : 'true';
    if (sw.getAttribute('aria-checked') !== want) {
      sw.setAttribute('aria-checked', want);
      sw.firstChild.textContent = off ? 'off' : 'on';
    }
    const pat = row.querySelector('.pattern');
    const label = `${LAYER_LABELS[layer]} pattern: ${notes} ${notes === 1 ? 'note' : 'notes'} in ${dots.length} steps`;
    if (pat.getAttribute('aria-label') !== label) pat.setAttribute('aria-label', label);
  });
}

let lastCursor = -1;

export function resetCursor() {
  lastCursor = -1;
}

// Only the column being left and the column being entered change: the lit
// cell on the card, and the halo on each layer's dot in that column. The
// lit colour and glow are pseudo-elements and shadows switched by class,
// with no fade, so a step is a handful of class writes.
export function moveCursor(lights, cells, index) {
  if (index === lastCursor) return;
  if (lastCursor >= 0) {
    if (lights[lastCursor]) lights[lastCursor].classList.remove('on');
    if (columnsLive && columns[lastCursor]) for (const d of columns[lastCursor]) d.classList.remove('at');
  }
  if (index >= 0) {
    if (lights[index]) lights[index].classList.add('on');
    if (columnsLive && columns[index]) for (const d of columns[index]) d.classList.add('at');
  }
  lastCursor = index;
}

// The layers' column follows the music only while Layers is on screen
// (queue item 40: nothing animates in a view that is hidden). Shown
// again, it is put where the cursor is.
let columnsLive = true;
export function setColumnsLive(on) {
  if (on === columnsLive) return;
  columnsLive = on;
  if (lastCursor >= 0 && columns[lastCursor]) for (const d of columns[lastCursor]) d.classList.toggle('at', on);
}

// --------------------------------------------------------------- misc

// A plain line in the info card's place (queue item 97; Mikey, 2026-11-07):
// what a browser that cannot run the core is told, or one too slow to play
// it smoothly (queue item 100), with a second sentence that is a link
// (`link`: { text, href }) where there is one. `null` brings the card back.
export function plainLine(text, link = null) {
  const line = el('plainLine');
  line.textContent = text || '';
  if (text && link && link.href) {
    const a = document.createElement('a');
    a.href = link.href;
    a.textContent = link.text;
    a.rel = 'noopener';
    line.append(' ', a);
  }
  line.hidden = !text;
  el('card').classList.toggle('plain', !!text);
}

let toastTimer = null;
export function toast(message) {
  const t = el('toast');
  t.textContent = message;
  t.classList.add('show');
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => t.classList.remove('show'), 2200);
}

export function download(blob, filename) {
  // Loop names are user-editable, and a slash or colon in one either breaks
  // the download or silently writes somewhere unexpected.
  const safe = filename
    .replace(/[\\/:*?"<>|\u0000-\u001f]/g, '-')
    .replace(/\s+/g, ' ')
    .replace(/^[.\s]+/, '')
    .slice(0, 120) || 'driftloom.dat';
  // The Android app has no downloads: the file goes through its own picker
  // (js/app.js, session 28).
  const app = appLink();
  if (app) {
    app.saveBlob(blob, safe).then((r) => toast(r.name ? `Saved ${r.name}` : r.why === 'cancelled' ? 'Nothing saved' : 'Could not save that file'));
    return;
  }
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = safe;
  document.body.appendChild(a);
  a.click();
  a.remove();
  setTimeout(() => URL.revokeObjectURL(url), 4000);
}

// --------------------------------------------------------------- icons

// Line icons on a 24-unit grid, drawn in the button's text color so the
// theme decides how they look, not the icon. Anything marked `solid` is
// filled instead of stroked. The transport and view keys are the v17
// design's own glyphs.
const ICONS = {
  prev: '<polygon class="solid" points="19 20 9 12 19 4 19 20"/><line x1="5" y1="19" x2="5" y2="5"/>',
  next: '<polygon class="solid" points="5 4 15 12 5 20 5 4"/><line x1="19" y1="5" x2="19" y2="19"/>',
  play: '<polygon class="solid" stroke-width="1" points="7 4 20 12 7 20"/>',
  pause: '<rect class="solid" stroke-width="1" x="6.5" y="4" width="4" height="16" rx="1"/>'
    + '<rect class="solid" stroke-width="1" x="13.5" y="4" width="4" height="16" rx="1"/>',
  // Books on a shelf, one leaning.
  library: '<path d="M6 4v15"/><path d="M10.5 4v15"/><path d="M15 5.2 17.6 19"/><path d="M4 20h16"/>',
  // A bookmark with an arrow out of it: save, or send it on.
  saveShare: '<path d="M17.5 21 12 17 6.5 21V5a2 2 0 0 1 2-2h7a2 2 0 0 1 2 2z"/><path d="M12 15V9"/><path d="m9.2 11.8 2.8-2.8 2.8 2.8"/>',
  // A note whose flag is a wave: what is playing now.
  now: '<circle cx="6.5" cy="17" r="2.6"/><path d="M9.1 17V5.5"/><path d="M9.1 5.5c1.5-2 3-2 4.5 0s3 2 4.5 0"/>',
  nowMark: '<circle class="solid" cx="6.5" cy="17" r="2.6"/><path d="M9.1 17V5.5"/><path d="M9.1 5.5c1.5-2 3-2 4.5 0s3 2 4.5 0"/>',
  check: '<path d="m5 12.5 4.5 4.5L19 7.5"/>',
  close: '<path d="M18 6 6 18"/><path d="m6 6 12 12"/>',
  plus: '<path d="M12 5v14"/><path d="M5 12h14"/>',
  bookmark: '<path d="M18 21l-6-4.5L6 21V5a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2z"/>',
  // Three joined dots: the share mark phones already use.
  share: '<circle cx="18" cy="5" r="2.6"/><circle cx="6" cy="12" r="2.6"/><circle cx="18" cy="19" r="2.6"/>'
    + '<path d="m8.3 13.3 7.4 4.4"/><path d="m15.7 6.3-7.4 4.4"/>',
  // Three sliders: the sound controls.
  sounds: '<line x1="4" y1="6" x2="20" y2="6"/><line x1="4" y1="12" x2="20" y2="12"/><line x1="4" y1="18" x2="20" y2="18"/>'
    + '<circle class="solid" cx="9" cy="6" r="2"/><circle class="solid" cx="15" cy="12" r="2"/><circle class="solid" cx="7" cy="18" r="2"/>',
  layers: '<path d="M12 3.5 21 8l-9 4.5L3 8z"/><path d="m3 12.5 9 4.5 9-4.5"/><path d="m3 16.5 9 4.5 9-4.5"/>',
  left: '<path d="m14.5 6-6 6 6 6"/>',
  right: '<path d="m9.5 6 6 6-6 6"/>',
};

export function icon(name) {
  const body = ICONS[name].replace(/class="solid"/g, 'fill="currentColor"');
  return `<svg viewBox="0 0 24 24" aria-hidden="true" focusable="false" fill="none" stroke="currentColor"
    stroke-width="2" stroke-linecap="round" stroke-linejoin="round">${body}</svg>`;
}

// Only rewrites when the icon actually changes, since it is called on every
// load and every play/stop.
export function setIcon(button, name) {
  if (button.dataset.icon === name) return;
  button.dataset.icon = name;
  button.innerHTML = icon(name);
}
