#!/usr/bin/env node
// Every icon the app uses, made from one source, icons/icon.svg (item 120):
// the page's favicons, the web app's icons (192, 512, the maskable 512),
// the apple-touch-icon, Android's adaptive icon (its background, foreground
// and monochrome layers) and the status bar's icon, and the F-Droid
// listing's icon and feature graphic. A change to the icon is an edit to
// icons/icon.svg and a run of this script; commit what it writes.
//
//   node tools/icons.mjs                  write every icon, after the checks
//   node tools/icons.mjs --check          the checks only (nothing written)
//   node tools/icons.mjs --sheet out.png  also write a contact sheet: every
//                                         size, launchers on light and dark,
//                                         the themed icon, the listing
//
// The web's and the listing's icons are PNGs. Android's are vector
// drawables (XML: android/ holds no binary, android/check-tree.sh), which
// this script writes from the SVG itself, so the SVG keeps to what they can
// draw (its header says what). The contact sheet draws Android's tiles from
// those XML files, read back, not from the SVG.
//
// The checks, each a failure (exit 1):
//   - what icons/icon.svg marks with class="key" (the moon, the stream, its
//     first ring) lies inside the maskable icon's safe zone (the circle of
//     40% of its size, the W3C's) and inside Android's adaptive-icon safe
//     zone (the 66 dp circle of the 108 dp layer);
//   - the whole monochrome layer lies inside Android's safe zone;
//   - each Android layer, read back from its XML, draws what the SVG draws.
//
// Needs ImageMagick 7 (`magick`) with librsvg, which renders the SVG; the
// same version writes the same bytes. Nothing else: Node 22, no packages.

import { readFileSync, writeFileSync, mkdtempSync, rmSync, mkdirSync } from 'node:fs';
import { execFileSync, spawnSync } from 'node:child_process';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const SOURCE = join(root, 'icons/icon.svg');
const svg = readFileSync(SOURCE, 'utf8');
const args = process.argv.slice(2);
const checkOnly = args.includes('--check');
const sheetAt = args.includes('--sheet') ? args[args.indexOf('--sheet') + 1] : null;
const tmp = mkdtempSync(join(tmpdir(), 'icons-'));
const at = (p) => join(root, p);
const RES = 'android/app/src/main/res';
const LISTING = 'fastlane/metadata/android/en-US/images';

// Crops of the scene, in the source's units ([x, y, width, height]). The
// icon is 0 0 512 512; the scene is drawn out to -128..640 each way.
const ICON = [0, 0, 512, 512];
const MASKABLE = [-32, -32, 576, 576]; // a little room past the icon
const ADAPTIVE = [-128, -128, 768, 768]; // the 108 dp layer: its 72 dp is the icon
const FEATURE = [-128, 52, 768, 375]; // 1024 x 500, from the moon down to the pool
const STATUS = [40, 38, 440, 440]; // the monochrome drawing, filling 24 dp

// Android's drawables: the file, the group of the SVG it draws, the crop
// and its size in dp.
const ANDROID = [
  ['ic_launcher_background', 'background', ADAPTIVE, 108],
  ['ic_launcher_foreground', 'foreground', ADAPTIVE, 108],
  ['ic_launcher_monochrome', 'monochrome', ADAPTIVE, 108],
  ['ic_stat_driftloom', 'monochrome', STATUS, 24],
];

// ---- Rendering the SVG ----

// What each picture shows of the source's three groups.
const SHOW = {
  full: '#monochrome{display:none}',
  background: '#foreground,#monochrome{display:none}',
  foreground: '#background,#monochrome{display:none}',
  monochrome: '#background,#foreground{display:none} #monochrome{display:inline}',
  key: '#background,#monochrome{display:none} #foreground *{visibility:hidden} #foreground .key{visibility:visible}',
};

function variant(crop, w, h, show) {
  const s = svg.replace('viewBox="0 0 512 512"', `viewBox="${crop.join(' ')}" width="${w}" height="${h}"`);
  if (s === svg) throw new Error('icons/icon.svg: its root must say viewBox="0 0 512 512"');
  return s.replace(/<svg[^>]*>/, (open) => `${open}\n<style>${SHOW[show]}</style>`);
}

let n = 0;
function rasterize(svgText, w, h, out, format = 'PNG32') {
  const file = join(tmp, `v${n++}.svg`);
  writeFileSync(file, svgText);
  mkdirSync(dirname(out), { recursive: true });
  execFileSync('magick', ['-background', 'none', file, '-strip', '-define', 'png:exclude-chunks=date,time', `${format}:${out}`]);
  const [pw, ph] = execFileSync('magick', ['identify', '-format', '%w %h', out]).toString().split(' ').map(Number);
  if (pw !== w || ph !== h) throw new Error(`${out}: ${pw} x ${ph}, not ${w} x ${h}`);
  return out;
}
const render = (crop, w, h, show, out, format) => rasterize(variant(crop, w, h, show), w, h, out, format);

// ---- Android's vector drawables, from the SVG ----

// A small reader of XML: elements and their attributes (no text content).
function parseXml(text) {
  const top = { name: '#doc', attrs: {}, children: [] };
  const stack = [top];
  const tag = /<(\/?)([\w:-]+)((?:\s+[\w:-]+="[^"]*")*)\s*(\/?)>/g;
  for (const [, close, name, attrText, selfClose] of text.replace(/<!--[\s\S]*?-->/g, '').matchAll(tag)) {
    if (close) { stack.pop(); continue; }
    const node = { name, attrs: Object.fromEntries([...attrText.matchAll(/([\w:-]+)="([^"]*)"/g)].map((a) => [a[1], a[2]])), children: [] };
    stack.at(-1).children.push(node);
    if (!selfClose) stack.push(node);
  }
  return top.children[0];
}

const num = (x) => String(+(+x).toFixed(4));
function argb(color, alpha = 1) {
  let c = color.replace('#', '');
  if (c.length === 3) c = [...c].map((d) => d + d).join('');
  if (!/^[0-9a-fA-F]{6}$/.test(c)) throw new Error(`icons/icon.svg: a color Android can't take: ${color}`);
  return `#${Math.round(alpha * 255).toString(16).padStart(2, '0')}${c}`.toUpperCase();
}

// A shape's outline as path data, absolute commands only.
function pathOf(node, why) {
  const a = node.attrs;
  const v = (k, d = 0) => (a[k] === undefined ? d : +a[k]);
  if (node.name === 'path') {
    if (/[a-df-z]/.test(a.d)) throw new Error(`icons/icon.svg: ${why}: relative path commands; use absolute ones`);
    return a.d.replace(/\s+/g, ' ').trim();
  }
  if (node.name === 'circle' || node.name === 'ellipse') {
    const [cx, cy] = [v('cx'), v('cy')];
    const [rx, ry] = node.name === 'circle' ? [v('r'), v('r')] : [v('rx'), v('ry')];
    return `M${num(cx - rx)},${num(cy)} A${num(rx)},${num(ry)} 0 1 0 ${num(cx + rx)},${num(cy)} A${num(rx)},${num(ry)} 0 1 0 ${num(cx - rx)},${num(cy)} Z`;
  }
  if (node.name === 'rect') {
    const [x, y, w, h] = [v('x'), v('y'), v('width'), v('height')];
    const r = Math.min(v('rx', v('ry')), w / 2, h / 2);
    if (!r) return `M${num(x)},${num(y)} H${num(x + w)} V${num(y + h)} H${num(x)} Z`;
    const A = (tx, ty) => `A${num(r)},${num(r)} 0 0 1 ${num(tx)},${num(ty)}`;
    return `M${num(x + r)},${num(y)} H${num(x + w - r)} ${A(x + w, y + r)} V${num(y + h - r)} ${A(x + w - r, y + h)} H${num(x + r)} ${A(x, y + h - r)} V${num(y + r)} ${A(x + r, y)} Z`;
  }
  throw new Error(`icons/icon.svg: ${why}: <${node.name}> has no Android drawing`);
}

const doc = parseXml(svg);
const defs = {};
(function collect(node) {
  if (node.attrs.id) defs[node.attrs.id] = node;
  node.children.forEach(collect);
})(doc);
const ref = (value) => /^url\(#([\w-]+)\)$/.exec(value)?.[1];

function gradient(attr, id, shape) {
  const g = defs[id];
  const stops = g.children.filter((s) => s.name === 'stop').map((s) =>
    `<item android:offset="${num(s.attrs.offset)}" android:color="${argb(s.attrs['stop-color'], s.attrs['stop-opacity'] ?? 1)}"/>`);
  let kind;
  if (g.name === 'linearGradient') {
    if (g.attrs.gradientUnits !== 'userSpaceOnUse') throw new Error(`icons/icon.svg: #${id}: a linear gradient needs gradientUnits="userSpaceOnUse"`);
    const a = g.attrs;
    kind = `android:type="linear" android:startX="${num(a.x1)}" android:startY="${num(a.y1)}" android:endX="${num(a.x2)}" android:endY="${num(a.y2)}"`;
  } else if (g.name === 'radialGradient') {
    if (shape.name !== 'circle' || g.attrs.gradientUnits || g.attrs.cx || g.attrs.r) throw new Error(`icons/icon.svg: #${id}: a radial gradient only on a circle, filling it`);
    const s = shape.attrs;
    kind = `android:type="radial" android:centerX="${num(s.cx)}" android:centerY="${num(s.cy)}" android:gradientRadius="${num(s.r)}"`;
  } else throw new Error(`icons/icon.svg: #${id} is no gradient`);
  return `<aapt:attr name="${attr}"><gradient ${kind}>${stops.join('')}</gradient></aapt:attr>`;
}

function transformGroups(t, why) {
  if (!t) return [];
  const out = [];
  const rest = t.replace(/(translate|scale)\(([^)]*)\)/g, (_, f, list) => {
    const [x, y] = list.trim().split(/[\s,]+/).map(Number);
    out.push(f === 'translate'
      ? `android:translateX="${num(x)}" android:translateY="${num(y ?? 0)}"`
      : `android:scaleX="${num(x)}" android:scaleY="${num(y ?? x)}"`);
    return '';
  });
  if (rest.trim()) throw new Error(`icons/icon.svg: ${why}: a transform Android can't take: ${t}`);
  return out;
}

function walk(node, style, out, pad) {
  for (const c of node.children) {
    const a = c.attrs;
    const why = a.id ? `#${a.id}` : `<${c.name}>`;
    const st = { ...style };
    for (const k of ['fill', 'stroke', 'stroke-width', 'stroke-linecap']) if (a[k] !== undefined) st[k] = a[k];
    if (c.name === 'g') {
      if (a.opacity) throw new Error(`icons/icon.svg: ${why}: opacity on a group; put it on its shapes`);
      const groups = transformGroups(a.transform, why);
      const clip = ref(a['clip-path'] ?? '');
      if (clip) groups.push('');
      let p = pad;
      groups.forEach((g, i) => {
        out.push(`${p}<group${g ? ` ${g}` : ''}>`);
        p += '  ';
        if (clip && i === groups.length - 1) out.push(`${p}<clip-path android:pathData="${defs[clip].children.map((k) => pathOf(k, `#${clip}`)).join(' ')}"/>`);
      });
      walk(c, st, out, p);
      groups.forEach(() => { p = p.slice(2); out.push(`${p}</group>`); });
      continue;
    }
    const opacity = a.opacity === undefined ? 1 : +a.opacity;
    const attrs = [`android:pathData="${pathOf(c, why)}"`];
    const inner = [];
    if (st.fill !== 'none') {
      const id = ref(st.fill);
      if (id) inner.push(gradient('android:fillColor', id, c)); else attrs.push(`android:fillColor="${argb(st.fill)}"`);
      if (opacity < 1) attrs.push(`android:fillAlpha="${num(opacity)}"`);
    }
    if (st.stroke && st.stroke !== 'none') {
      const id = ref(st.stroke);
      if (id) inner.push(gradient('android:strokeColor', id, c)); else attrs.push(`android:strokeColor="${argb(st.stroke)}"`);
      attrs.push(`android:strokeWidth="${num(st['stroke-width'] ?? 1)}"`);
      if (st['stroke-linecap']) attrs.push(`android:strokeLineCap="${st['stroke-linecap']}"`);
      if (opacity < 1) attrs.push(`android:strokeAlpha="${num(opacity)}"`);
    }
    const open = `${pad}<path\n${attrs.map((x) => `${pad}    ${x}`).join('\n')}`;
    out.push(inner.length ? `${open}>\n${inner.map((x) => `${pad}  ${x}`).join('\n')}\n${pad}</path>` : `${open} />`);
  }
}

function vector(show, crop, dp) {
  const group = doc.children.find((c) => c.name === 'g' && c.attrs.id === show);
  const out = [];
  const outer = [`android:translateX="${num(-crop[0])}" android:translateY="${num(-crop[1])}"`, ...transformGroups(group.attrs.transform, `#${show}`)];
  let pad = '    ';
  for (const g of outer) { out.push(`${pad}<group ${g}>`); pad += '  '; }
  walk(group, { fill: group.attrs.fill ?? '#000' }, out, pad);
  for (let i = 0; i < outer.length; i++) { pad = pad.slice(2); out.push(`${pad}</group>`); }
  return `<?xml version="1.0" encoding="utf-8"?>
<!-- Made by tools/icons.mjs from icons/icon.svg (its #${show}): edit that, run the script, never this file. -->
<vector xmlns:android="http://schemas.android.com/apk/res/android"
    xmlns:aapt="http://schemas.android.com/aapt"
    android:width="${dp}dp"
    android:height="${dp}dp"
    android:viewportWidth="${num(crop[2])}"
    android:viewportHeight="${num(crop[3])}">
${out.join('\n')}
</vector>
`;
}

// A vector drawable read back into SVG, to draw what Android will.
function vectorToSvg(xml, px) {
  const v = parseXml(xml);
  const defsOut = [];
  let k = 0;
  const color = (c) => { const h = c.replace('#', ''); return [`#${h.slice(2)}`, parseInt(h.slice(0, 2), 16) / 255]; };
  const paint = (node, attr) => {
    const g = node.children.find((c) => c.name === 'aapt:attr' && c.attrs.name === attr)?.children[0];
    if (!g) return node.attrs[attr] ? color(node.attrs[attr]) : null;
    const id = `g${k++}`;
    const stops = g.children.map((s) => { const [c, o] = color(s.attrs['android:color']); return `<stop offset="${s.attrs['android:offset']}" stop-color="${c}" stop-opacity="${o}"/>`; }).join('');
    const a = g.attrs;
    defsOut.push(a['android:type'] === 'radial'
      ? `<radialGradient id="${id}" gradientUnits="userSpaceOnUse" cx="${a['android:centerX']}" cy="${a['android:centerY']}" r="${a['android:gradientRadius']}">${stops}</radialGradient>`
      : `<linearGradient id="${id}" gradientUnits="userSpaceOnUse" x1="${a['android:startX']}" y1="${a['android:startY']}" x2="${a['android:endX']}" y2="${a['android:endY']}">${stops}</linearGradient>`);
    return [`url(#${id})`, 1];
  };
  const body = (node) => node.children.map((c) => {
    const a = c.attrs;
    if (c.name === 'group') {
      const t = `translate(${a['android:translateX'] ?? 0} ${a['android:translateY'] ?? 0}) scale(${a['android:scaleX'] ?? 1} ${a['android:scaleY'] ?? 1})`;
      const clip = c.children.find((x) => x.name === 'clip-path');
      let cp = '';
      if (clip) { const id = `c${k++}`; defsOut.push(`<clipPath id="${id}"><path d="${clip.attrs['android:pathData']}"/></clipPath>`); cp = ` clip-path="url(#${id})"`; }
      return `<g transform="${t}"><g${cp}>${body(c)}</g></g>`;
    }
    if (c.name !== 'path') return '';
    const fill = paint(c, 'android:fillColor');
    const stroke = paint(c, 'android:strokeColor');
    const fa = (fill?.[1] ?? 1) * (+(a['android:fillAlpha'] ?? 1));
    const sa = (stroke?.[1] ?? 1) * (+(a['android:strokeAlpha'] ?? 1));
    return `<path d="${a['android:pathData']}" fill="${fill ? fill[0] : 'none'}" fill-opacity="${fa}"${stroke ? ` stroke="${stroke[0]}" stroke-opacity="${sa}" stroke-width="${a['android:strokeWidth'] ?? 0}" stroke-linecap="${a['android:strokeLineCap'] ?? 'butt'}"` : ''}/>`;
  }).join('\n');
  const content = body(v);
  return `<svg xmlns="http://www.w3.org/2000/svg" width="${px}" height="${px}" viewBox="0 0 ${v.attrs['android:viewportWidth']} ${v.attrs['android:viewportHeight']}"><defs>${defsOut.join('')}</defs>${content}</svg>`;
}

// ---- The checks ----

// The largest alpha of `png` outside the circle of `fraction` of its size,
// centered: 0 when all of it is inside.
function outside(png, S, fraction) {
  const c = S / 2;
  return Number(execFileSync('magick', [png, '-alpha', 'extract',
    '(', '-size', `${S}x${S}`, 'xc:white', '-fill', 'black', '-draw', `circle ${c},${c} ${c},${c + fraction * S}`, ')',
    '-compose', 'multiply', '-composite', '-format', '%[fx:maxima]', 'info:']).toString());
}
const shown = (crop, show) => render(crop, 512, 512, show, join(tmp, `check${n}.png`));
// The root-mean-square difference of two pictures, 0 to 1.
function compare(a, b) {
  const r = spawnSync('magick', ['compare', '-metric', 'RMSE', a, b, 'null:'], { encoding: 'utf8' });
  const m = /\(([\d.e-]+)\)/.exec(r.stderr);
  if (!m) throw new Error(`magick compare: ${r.stderr}`);
  return Number(m[1]);
}

const ANDROID_SAFE = 33 / 108; // the 66 dp circle's radius, of the 108 dp layer
const xml = Object.fromEntries(ANDROID.map(([file, show, crop, dp]) => [file, vector(show, crop, dp)]));
const checks = [
  ['the key inside the maskable icon\'s safe zone (40%)', outside(shown(MASKABLE, 'key'), 512, 0.4) <= 0.02],
  ['the key inside Android\'s safe zone (66 of 108 dp)', outside(shown(ADAPTIVE, 'key'), 512, ANDROID_SAFE) <= 0.02],
  ['the monochrome layer inside Android\'s safe zone', outside(shown(ADAPTIVE, 'monochrome'), 512, ANDROID_SAFE) <= 0.02],
  ...ANDROID.map(([file, show, crop]) => {
    const back = rasterize(vectorToSvg(xml[file], 512), 512, 512, join(tmp, `${file}-back.png`));
    const d = compare(shown(crop, show), back);
    return [`${file}.xml draws what the SVG draws (RMSE ${(d * 100).toFixed(2)}%)`, d < 0.01];
  }),
];
let failed = 0;
for (const [what, ok] of checks) {
  if (!ok) failed++;
  console.log(`${ok ? 'ok  ' : 'FAIL'} ${what}`);
}
if (failed || checkOnly) { rmSync(tmp, { recursive: true }); process.exit(failed ? 1 : 0); }

// ---- The icons ----

const written = [
  render(ICON, 32, 32, 'full', at('icons/favicon-32.png')),
  render(ICON, 180, 180, 'full', at('icons/apple-touch-icon.png'), 'PNG24'),
  render(ICON, 192, 192, 'full', at('icons/icon-192.png'), 'PNG24'),
  render(ICON, 512, 512, 'full', at('icons/icon-512.png'), 'PNG24'),
  render(MASKABLE, 512, 512, 'full', at('icons/maskable-512.png'), 'PNG24'),
  render(ICON, 512, 512, 'full', at(`${LISTING}/icon.png`), 'PNG24'),
  render(FEATURE, 1024, 500, 'full', at(`${LISTING}/featureGraphic.png`), 'PNG24'),
];
for (const [file] of ANDROID) {
  const out = at(`${RES}/drawable/${file}.xml`);
  writeFileSync(out, xml[file]);
  written.push(out);
}
console.log(`wrote ${written.length} icons`);

if (sheetAt) sheet(sheetAt);
rmSync(tmp, { recursive: true });

// ---- The contact sheet: rows of labeled tiles ----

function sheet(out) {
  const t = (name) => join(tmp, `${name}.png`);
  const mk = (...a) => execFileSync('magick', a.flat());
  const LIGHT = '#e9e6ef';
  const DARK = '#1f1f24';
  const tile = (img, w, h, bg, label, name) => {
    mk(['-size', `${w}x${h}`, `xc:${bg}`, '(', img, '-resize', `${w - 8}x${h - 8}>`, ')', '-gravity', 'center', '-composite',
      '+size', '-background', '#fafafa', '-fill', '#444', '-pointsize', '13', `label:${label}`, '-gravity', 'center', '-append',
      '-bordercolor', '#fafafa', '-border', '8', t(name)]);
    return t(name);
  };
  // Android's layers as Android will draw them: from the XML, read back.
  const drawn = (file, px) => rasterize(vectorToSvg(readFileSync(at(`${RES}/drawable/${file}.xml`), 'utf8'), px), px, px, t(`${file}-${px}`));
  mk([drawn('ic_launcher_background', 432), drawn('ic_launcher_foreground', 432), '-composite', '-gravity', 'center', '-crop', '288x288+0+0', '+repage', t('launch')]);
  const masked = (shape, size, name) => {
    const r = shape === 'circle' ? size / 2 : size * 0.3;
    mk([t('launch'), '-resize', `${size}x${size}`,
      '(', '-size', `${size}x${size}`, 'xc:none', '-fill', 'white', '-draw', `roundrectangle 0,0 ${size - 1},${size - 1} ${r},${r}`, ')',
      '-compose', 'DstIn', '-composite', t(name)]);
    return t(name);
  };
  const mono = drawn('ic_launcher_monochrome', 432);
  const themed = (fg, bg, size, name) => {
    mk([mono, '-gravity', 'center', '-crop', '288x288+0+0', '+repage', '-resize', `${size}x${size}`, '-fill', fg, '-colorize', '100', t(`${name}-m`)]);
    mk(['-size', `${size}x${size}`, 'xc:none', '-fill', bg, '-draw', `circle ${size / 2},${size / 2} ${size / 2},0`, t(`${name}-m`), '-compose', 'over', '-composite', t(name)]);
    return t(name);
  };
  const icon = (p) => at(`icons/${p}`);
  mk([icon('favicon-32.png'), '-resize', '16x16', t('fav16')]);
  mk([t('fav16'), '-filter', 'point', '-resize', '64x64', t('fav16x4')]);
  mk([icon('maskable-512.png'), '-resize', '256x256', '-fill', 'none', '-stroke', '#ffffff', '-strokewidth', '1.5',
    '-draw', 'circle 128,128 128,25.6', t('mask')]);
  const status = drawn('ic_stat_driftloom', 24);
  mk([drawn('ic_stat_driftloom', 96), t('statx4')]);
  const rows = [
    [tile(t('fav16'), 40, 40, LIGHT, 'favicon 16', 'a1'), tile(t('fav16x4'), 72, 72, LIGHT, '16, enlarged', 'a2'),
      tile(icon('favicon-32.png'), 48, 48, LIGHT, 'favicon 32', 'a3'), tile(icon('apple-touch-icon.png'), 196, 196, LIGHT, 'apple-touch 180', 'a4'),
      tile(icon('icon-192.png'), 208, 208, LIGHT, 'icon 192', 'a5'), tile(t('mask'), 272, 272, LIGHT, 'maskable 512, its safe zone', 'a6')],
    [tile(masked('circle', 96, 'c96'), 120, 120, LIGHT, 'launcher, light', 'b1'), tile(masked('squircle', 96, 's96'), 120, 120, LIGHT, 'rounded', 'b2'),
      tile(masked('circle', 48, 'c48'), 72, 72, LIGHT, '48 dp at 1x', 'b3'),
      tile(t('c96'), 120, 120, DARK, 'launcher, dark', 'b4'), tile(t('s96'), 120, 120, DARK, 'rounded', 'b5'), tile(t('c48'), 72, 72, DARK, '48 dp at 1x', 'b6'),
      tile(themed('#2d3a5c', '#d6e0ff', 48, 'th1'), 72, 72, '#f4f4f8', 'themed 48, light', 'b7'),
      tile(themed('#d6e0ff', '#2d3550', 48, 'th2'), 72, 72, '#121318', 'themed 48, dark', 'b8'),
      tile(status, 40, 40, '#202124', 'status 24', 'b9'), tile(t('statx4'), 112, 112, '#202124', 'status, enlarged', 'b10')],
    [tile(at(`${LISTING}/icon.png`), 264, 264, LIGHT, 'listing icon 512', 'c1'),
      tile(at(`${LISTING}/featureGraphic.png`), 640, 320, LIGHT, 'feature graphic 1024 x 500', 'c2')],
  ].map((tiles, i) => {
    mk(['-background', '#fafafa', ...tiles, '-gravity', 'south', '+append', t(`row${i}`)]);
    return t(`row${i}`);
  });
  mk(['-background', '#fafafa', ...rows, '-gravity', 'west', '-append', '-bordercolor', '#fafafa', '-border', '12', '+repage', out]);
  console.log(`wrote the contact sheet: ${out}`);
}
