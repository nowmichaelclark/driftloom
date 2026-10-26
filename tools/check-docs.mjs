#!/usr/bin/env node
// The live docs stay small (docs/BRAIN.md, "The docs stay small"): each role
// loads only its own docs, and the claude.ai Project syncs a few of them, so
// every one has a size limit. Over a limit, the fix is to move what is
// finished into docs/archive/, not to raise the limit.
//
//   node tools/check-docs.mjs          check the limits (CI runs this)
//   node tools/check-docs.mjs --list   print the size of every live doc too
//
// A KB is 1024 bytes. Exits 1 if a file is over its limit or missing.

import { readFileSync, existsSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const KB = 1024;
// File -> limit in KB. docs/README.md is the map of who reads what.
const LIMITS = {
  'AGENTS.md': 6,
  'core/AGENTS.md': 10,
  'docs/QUEUE.md': 80,
  'docs/BRAIN.md': 50,
  'docs/DECISIONS.md': 40,
};
const LIVE = ['CLAUDE.md', 'README.md', 'ROADMAP.md', 'docs/README.md', 'docs/DONE.md', 'docs/CORE.md',
  'docs/EARS.md', 'docs/MOODS.md', 'docs/VIBES.md', 'docs/ALBUMS.md', 'docs/PAINTER.md'];

const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const size = (file) => readFileSync(join(root, file)).length;
const kb = (bytes) => (bytes / KB).toFixed(1).padStart(6) + ' KB';

let failed = false;
for (const [file, limit] of Object.entries(LIMITS)) {
  if (!existsSync(join(root, file))) {
    console.log(`MISSING  ${file}`);
    failed = true;
    continue;
  }
  const bytes = size(file);
  const over = bytes > limit * KB;
  if (over) failed = true;
  console.log(`${over ? 'OVER   ' : 'ok     '} ${file.padEnd(20)} ${kb(bytes)}  (limit ${limit} KB)${over ? '  over by ' + kb(bytes - limit * KB).trim() : ''}`);
}
if (process.argv.includes('--list')) {
  console.log('');
  for (const file of LIVE) if (existsSync(join(root, file))) console.log(`         ${file.padEnd(20)} ${kb(size(file))}`);
}
if (failed) {
  console.error('\nA live doc is over its limit: archive the finished parts (docs/BRAIN.md, "The docs stay small").');
  process.exit(1);
}
