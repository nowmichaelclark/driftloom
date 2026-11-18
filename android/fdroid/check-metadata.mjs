#!/usr/bin/env node
// What the listing's files and the version must be (queue item 107e), checked
// without F-Droid's tooling (CI also runs fdroidserver's lint on the recipe):
//
//   node android/fdroid/check-metadata.mjs            the files and the version
//   node android/fdroid/check-metadata.mjs --tag v1.0 and the tag a release is built from
//
// 1. fastlane/metadata/android/en-US/ (F-Droid's layout): a title of up to 50
//    characters, a short description of up to 80, a full description of up to
//    4000 holding only the HTML F-Droid allows, changelogs named by version
//    code and under 500 characters, the icon (512 x 512), the feature graphic
//    (1024 x 500) and at least two phone screenshots, all PNG.
// 2. The app's name is written once, in title.txt (and in gradle.properties
//    and strings.xml: session 31a's three lines): no other listing file holds it.
// 3. The version scheme (android/README.md): versionCode = major * 10000 +
//    minor * 100 + patch, for the versionName "major.minor" or
//    "major.minor.patch". A release (a versionName without "-test") has its
//    changelog, and is the tag it is built from.

import { existsSync, readFileSync, readdirSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..', '..');
const argv = process.argv.slice(2);
const tag = argv.includes('--tag') ? argv[argv.indexOf('--tag') + 1] : '';
const dir = join(root, 'fastlane/metadata/android/en-US');
let failed = false;
const check = (ok, what) => {
  console.log(`${ok ? 'ok  ' : 'FAIL'} ${what}`);
  if (!ok) failed = true;
};
const read = (f) => readFileSync(join(dir, f), 'utf8');
const chars = (s) => [...s].length;

const props = Object.fromEntries(
  readFileSync(join(root, 'android/gradle.properties'), 'utf8').split('\n')
    .filter((l) => /^[\w.]+=/.test(l)).map((l) => [l.slice(0, l.indexOf('=')), l.slice(l.indexOf('=') + 1).trim()]));
const appName = props['driftloom.appName'];
const versionName = props['driftloom.versionName'];
const versionCode = Number(props['driftloom.versionCode']);

// ---- the text
const title = read('title.txt');
const short = read('short_description.txt');
const full = read('full_description.txt');
check(chars(title.trim()) > 0 && chars(title) <= 50, `title.txt: ${chars(title)} characters (up to 50)`);
check(!title.includes('\n'), 'title.txt is one line with no newline at its end');
check(title === appName, `title.txt is the app's name, "${appName}" (android/gradle.properties's driftloom.appName)`);
check(chars(short) <= 80 && chars(short) > 0, `short_description.txt: ${chars(short)} characters (up to 80)`);
check(chars(full) <= 4000 && chars(full) > 0, `full_description.txt: ${chars(full)} characters (up to 4000)`);
const allowed = new Set(['b', 'big', 'blockquote', 'br', 'cite', 'em', 'i', 'li', 'ol', 'small', 'strike', 'strong', 'sub', 'sup', 'tt', 'u', 'ul']);
const tags = [...full.matchAll(/<\/?([a-zA-Z][a-zA-Z0-9]*)/g)].map((m) => m[1].toLowerCase());
const bad = tags.filter((t) => !allowed.has(t));
check(bad.length === 0, `full_description.txt holds only the HTML F-Droid allows${bad.length ? ` (not: ${[...new Set(bad)].join(', ')})` : ''}`);
check(/^[\x09\x0a\x20-\x7e]*$/.test(short + full + title), 'the three texts are plain ASCII');

// ---- the changelogs
const logs = readdirSync(join(dir, 'changelogs'));
check(logs.length > 0 && logs.every((f) => /^[1-9]\d*\.txt$/.test(f)), `changelogs/ is named by version code (${logs.join(', ')})`);
for (const f of logs) check(chars(read(`changelogs/${f}`)) <= 500, `changelogs/${f}: ${chars(read(`changelogs/${f}`))} characters (up to 500)`);

// ---- the graphics
function png(file) {
  const b = readFileSync(join(dir, file));
  if (b.length < 24 || b.readUInt32BE(0) !== 0x89504e47) return null;
  return { w: b.readUInt32BE(16), h: b.readUInt32BE(20), bytes: b.length };
}
const icon = png('images/icon.png');
check(icon && icon.w === 512 && icon.h === 512, `images/icon.png is 512 x 512 (${icon ? `${icon.w} x ${icon.h}` : 'missing'})`);
const feature = png('images/featureGraphic.png');
check(feature && feature.w === 1024 && feature.h === 500, `images/featureGraphic.png is 1024 x 500 (${feature ? `${feature.w} x ${feature.h}` : 'missing'})`);
const shots = existsSync(join(dir, 'images/phoneScreenshots')) ? readdirSync(join(dir, 'images/phoneScreenshots')).sort() : [];
check(shots.length >= 2 && shots.every((f, i) => f === `${i + 1}.png`), `images/phoneScreenshots/ is 1.png, 2.png, ...: ${shots.length} (at least two)`);
for (const f of shots) {
  const p = png(`images/phoneScreenshots/${f}`);
  check(p && p.w >= 320 && p.h >= 320 && p.w <= 3840 && p.h <= 3840, `phoneScreenshots/${f} is ${p ? `${p.w} x ${p.h}` : 'not a PNG'}`);
}

// ---- the name, once
const listing = ['short_description.txt', 'full_description.txt', ...logs.map((f) => `changelogs/${f}`)];
const named = listing.filter((f) => read(f).toLowerCase().includes(appName.toLowerCase()));
check(named.length === 0, `the app's name is written only in title.txt${named.length ? ` (also in: ${named.join(', ')})` : ''}`);

// ---- the version
const m = /^(\d+)\.(\d+)(?:\.(\d+))?(-test)?$/.exec(versionName);
check(!!m, `versionName "${versionName}" is major.minor or major.minor.patch (a test build's ends in -test)`);
if (m) {
  const want = Number(m[1]) * 10000 + Number(m[2]) * 100 + Number(m[3] || 0);
  const release = !m[4];
  if (release) {
    check(versionCode === want, `versionCode ${versionCode} is ${want} for ${versionName}`);
    check(logs.includes(`${versionCode}.txt`), `changelogs/${versionCode}.txt exists`);
  } else {
    console.log(`note a test version (${versionName}, code ${versionCode}): the release checks wait for 1.0`);
  }
  if (tag) {
    check(release, 'the tag is of a release version, not a test one');
    check(tag === `v${versionName}`, `the tag ${tag} is v${versionName} (android/gradle.properties's driftloom.versionName)`);
  }
}
process.exit(failed ? 1 : 0);
