// Links, the album from one seed and the link preview (queue item 105c;
// session 24), through the page's own modules: a link is the address with
// the code after "#c=" and reads back to the code; a pasted link is its
// code; an album of one seed of 12 tracks has a code under 20 characters
// and the same 12 tracks every time it is read; and the preview's tags in
// index.html are whole, and the name and address in them, in the manifest
// and in js/main.js are one.
// Run with:  node test/preview.test.mjs

import fs from 'node:fs';
import { linkOf, codeOfLink, encodeSong, decodeSong, encodeSeedAlbum, decodeAlbum, codeKind } from '../js/share.js';
import { newSpec } from '../js/gen.js';

let failures = 0;
function check(label, condition, detail = '') {
  console.log(condition ? `  pass  ${label}` : `  FAIL  ${label} ${detail}`);
  if (!condition) failures++;
}
const read = (f) => fs.readFileSync(new URL(`../${f}`, import.meta.url), 'utf8');

console.log('\nA link');
{
  const spec = newSpec(31337, ['rainy', 'mellow']);
  const code = encodeSong(spec);
  const link = linkOf('https://driftloom.nowmichaelclark.workers.dev/?x=1#old', code);
  check('the code sits in the fragment, never in the path or a query', link === `https://driftloom.nowmichaelclark.workers.dev/#c=${code}`, link);
  check('the link reads back to the code', codeOfLink(link) === code);
  check('a bare code is its own', codeOfLink(` ${code} `) === code);
  check('a link pasted among words is read from its #c=', codeOfLink(`listen: ${link}`) === code);
  check('the code opens the same loop', decodeSong(codeOfLink(link)).seed === 31337);
  const flip = code.slice(0, 4) + (code[4] === 'A' ? 'B' : 'A') + code.slice(5);
  check('a code with a character changed is no code', codeKind(flip) === null);
  check('a cut-short code is no code', codeKind(code.slice(0, 4)) === null);
  check('an empty link is no code', codeKind(codeOfLink('https://x/#c=')) === null);
}

console.log('\nAn album from one seed');
{
  const request = { vibes: ['rainy'], tempo: null, length: null };
  const code = encodeSeedAlbum('', 12, request, 90210);
  check(`12 tracks have a code under 20 characters (${code.length})`, code.length < 20);
  check('it is an album', codeKind(code) === 'album');
  const a = decodeAlbum(code);
  const b = decodeAlbum(code);
  check('it reads as 12 tracks', a.specs.length === 12);
  check('the same 12 tracks every time (a second device)', a.specs.every((s, i) => s.seed === b.specs[i].seed && encodeSong(s) === encodeSong(b.specs[i])));
  check('the tracks differ from each other', new Set(a.specs.map((s) => s.seed)).size === 12);
  const again = encodeSeedAlbum('', 12, request, 90210);
  check('the same seed and picks, the same code', again === code);
  const other = encodeSeedAlbum('', 12, request, 90211);
  check('another seed, another album', other !== code);
  check('the picks are shared: every track leans on the vibe asked', a.specs.every((s) => s.vibes && Object.keys(s.vibes).includes('rainy')));
  check('a long album is as short', encodeSeedAlbum('', 48, request, 5).length < 20);
}

console.log('\nThe preview');
{
  const html = read('index.html');
  const meta = (attr, name) => {
    const m = html.match(new RegExp(`<meta ${attr}="${name}" content="([^"]*)"`));
    return m ? m[1] : null;
  };
  const man = JSON.parse(read('manifest.webmanifest'));
  const main = read('js/main.js');
  const appName = main.match(/const APP_NAME = '([^']+)'/)[1];
  const appUrl = main.match(/const APP_URL = '([^']+)'/)[1];
  const title = html.match(/<title>([^<]*)<\/title>/)[1];
  check('the title, the manifest and APP_NAME are one name', title === appName && man.name === appName && man.short_name === appName, `${title} / ${man.name} / ${appName}`);
  check('og:site_name and og:title are the name', meta('property', 'og:site_name') === appName && meta('property', 'og:title') === appName);
  check('og:url and the canonical link are APP_URL', meta('property', 'og:url') === appUrl && html.includes(`<link rel="canonical" href="${appUrl}">`));
  check('the description is the page\'s and the manifest\'s', meta('property', 'og:description') === meta('name', 'description') && meta('name', 'description') === man.description);
  const image = meta('property', 'og:image');
  check('og:image is an absolute https address under APP_URL', !!image && image.startsWith(appUrl) && image.startsWith('https://'), image);
  const file = image && image.slice(appUrl.length);
  check('the picture is a file the site ships', !!file && fs.existsSync(new URL(`../${file}`, import.meta.url)), file);
  const png = file && fs.readFileSync(new URL(`../${file}`, import.meta.url));
  const w = png && png.readUInt32BE(16);
  const h = png && png.readUInt32BE(20);
  check('its size is as the tags say', String(w) === meta('property', 'og:image:width') && String(h) === meta('property', 'og:image:height'), `${w}x${h}`);
  check('og:image:alt is there', !!meta('property', 'og:image:alt'));
  check('the card is a summary with the same title, description and picture', meta('name', 'twitter:card') === 'summary' && meta('name', 'twitter:title') === appName && meta('name', 'twitter:image') === image);
  check('og:type is website', meta('property', 'og:type') === 'website');
  check('the picture is no larger than 8 MB (Discord\'s limit)', png.length < 8 * 1024 * 1024);
}

if (failures) {
  console.log(`\n${failures} failed`);
  process.exit(1);
}
console.log('\nall passed');
