#!/usr/bin/env node
// Renders F-Droid's build recipe (recipe.yml.in) for the checkout it is run in
// (queue item 107e). The id, the name and the version come from
// android/gradle.properties (one value each; session 31a changes them there);
// the commit and the repository's address from git.
//
//   node android/fdroid/render.mjs                   print the recipe
//   node android/fdroid/render.mjs --out DIR         write DIR/<id>.yml
//   --commit SHA        the commit the build is of (default: HEAD)
//   --repo-url URL      the repository's public https address (default: origin's)
//   --repo-git PATH|URL what F-Droid clones (default: the address and .git; a
//                       local path is how CI tests the recipe on a checkout)
//   --signed-by SHA256  the lowercase hex SHA-256 of the signing certificate
//                       of the APK you sign yourself: adds Binaries and
//                       AllowedAPKSigningKeys, so F-Droid publishes your signed
//                       APK once its own build matches it byte for byte
//                       (docs/notes/for-mikey.md, "Signing")
//
// The result is plain text; check it with `fdroid lint` and `fdroid
// rewritemeta` (CI does).

import { execFileSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const argv = process.argv.slice(2);
const opt = (name, fallback = '') => {
  const i = argv.indexOf(name);
  return i >= 0 && argv[i + 1] ? argv[i + 1] : fallback;
};
const git = (...args) => execFileSync('git', args, { cwd: here, encoding: 'utf8' }).trim();
const fail = (message) => { console.error(`render.mjs: ${message}`); process.exit(1); };

const props = Object.fromEntries(
  readFileSync(join(here, '..', 'gradle.properties'), 'utf8').split('\n')
    .filter((l) => /^[\w.]+=/.test(l)).map((l) => [l.slice(0, l.indexOf('=')), l.slice(l.indexOf('=') + 1).trim()]));
const need = (key) => props[key] || fail(`android/gradle.properties has no ${key}`);
const appId = need('driftloom.applicationId');
const appName = need('driftloom.appName');
const versionName = need('driftloom.versionName');
const versionCode = need('driftloom.versionCode');

const commit = opt('--commit') || git('rev-parse', 'HEAD');
if (!/^[0-9a-f]{40}$/.test(commit)) fail(`not a full commit hash: ${commit}`);

// origin's address as a public https one (ssh and token forms rewritten).
let repoUrl = opt('--repo-url');
if (!repoUrl) {
  const origin = git('remote', 'get-url', 'origin');
  const m = /(?:github\.com[:/])([^/]+\/[^/]+?)(?:\.git)?\/?$/.exec(origin);
  repoUrl = m ? `https://github.com/${m[1]}` : origin;
}
repoUrl = repoUrl.replace(/\.git$/, '').replace(/\/$/, '');
const repoGit = opt('--repo-git', `${repoUrl}.git`);

let binaries = '';
let allowed = '';
const signedBy = opt('--signed-by');
if (signedBy) {
  if (!/^[0-9a-f]{64}$/.test(signedBy)) fail('--signed-by is the signing certificate\'s SHA-256, 64 lowercase hex digits');
  // The APK you upload to the release (docs/notes/for-mikey.md): <name>-<versionName>.apk,
  // the universal one, signed with apksigner.
  binaries = `Binaries: ${repoUrl}/releases/download/v%v/${appName.toLowerCase()}-%v.apk\n`;
  allowed = `AllowedAPKSigningKeys: ${signedBy}\n`;
}

// The template's comments explain it to us; `fdroid rewritemeta` would strip
// them from the recipe, so the recipe has none.
let text = readFileSync(join(here, 'recipe.yml.in'), 'utf8')
  .split('\n').filter((l) => !l.trim().startsWith('#')).join('\n');
const values = {
  '@APP_NAME@': appName, '@REPO_URL@': repoUrl, '@REPO_GIT@': repoGit, '@COMMIT@': commit,
  '@VERSION_NAME@': versionName, '@VERSION_CODE@': versionCode,
};
text = text.replace('@BINARIES@\n', `${binaries}\n`).replace('@ALLOWED@\n', allowed && `${allowed}\n`);
for (const [k, v] of Object.entries(values)) text = text.split(k).join(v);
if (/@[A-Z_]+@/.test(text)) fail(`a value is left unfilled: ${/@[A-Z_]+@/.exec(text)[0]}`);

const out = opt('--out');
if (out) {
  mkdirSync(out, { recursive: true });
  writeFileSync(join(out, `${appId}.yml`), text);
  console.error(`render.mjs: wrote ${join(out, `${appId}.yml`)} (version ${versionName}, code ${versionCode}, commit ${commit.slice(0, 10)})`);
} else {
  process.stdout.write(text);
}
