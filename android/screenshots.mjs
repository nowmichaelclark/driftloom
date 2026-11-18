// The F-Droid listing's phone screenshots (queue item 107e), taken from the
// app itself on an emulator: the debug build's page is driven through the
// WebView's DevTools port (android/cdp.mjs) to each screen a listener sees,
// and the emulator's own screen is captured (`adb exec-out screencap`, so a
// screenshot is what is on the glass: status bar, navigation bar and all).
//
//   node android/screenshots.mjs OUT_DIR       writes OUT_DIR/1.png, 2.png, ...
//
// The numbers are the order the listing shows them in
// (fastlane/metadata/android/en-US/images/phoneScreenshots/). Retake them with
// the `screenshots` job of .github/workflows/android.yml whenever the screens
// or the app's name change (session 31a), and commit the artifact's files.

import { execFileSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { AppPage, appId, until } from './cdp.mjs';

const out = process.argv[2];
if (!out) { console.error('usage: node android/screenshots.mjs OUT_DIR'); process.exit(2); }
fs.mkdirSync(out, { recursive: true });
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const adb = (...a) => execFileSync('adb', a, { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] });

// A quiet status bar (the system UI's demo mode): noon, full battery, no
// notifications. Best effort: an image without it is as true.
function demoMode() {
  const send = (...extra) => { try { adb('shell', 'am', 'broadcast', '-a', 'com.android.systemui.demo', ...extra); } catch { /* not every image has it */ } };
  try { adb('shell', 'settings', 'put', 'global', 'sysui_demo_allowed', '1'); } catch { /* ignore */ }
  send('-e', 'command', 'enter');
  send('-e', 'command', 'clock', '-e', 'hhmm', '1200');
  send('-e', 'command', 'battery', '-e', 'level', '100', '-e', 'plugged', 'false');
  send('-e', 'command', 'network', '-e', 'wifi', 'hide', '-e', 'mobile', 'hide');
  send('-e', 'command', 'notifications', '-e', 'visible', 'false');
}

function shot(n) {
  const file = path.join(out, `${n}.png`);
  fs.writeFileSync(file, execFileSync('adb', ['exec-out', 'screencap', '-p'], { maxBuffer: 64 * 1024 * 1024 }));
  console.log(`screenshot ${n}: ${file} (${fs.statSync(file).size} bytes)`);
}

const page = await AppPage.open({ pkg: appId(), ms: 90000, log: console.log });
const ev = (expression) => page.evaluate(expression, undefined, { gesture: true });
const click = (id) => ev(`document.getElementById('${id}').click()`);
const shown = (view) => until(`the ${view} view`, () => ev(`!document.getElementById('${view}').hidden`), 20000);
// The bottom row's Now Playing key toggles (it goes back to the last edit view
// from Now Playing itself): press it only when Now Playing is not shown.
const goNow = async () => {
  if (await ev("document.getElementById('viewNow').hidden")) await click('nowBtn');
  await shown('viewNow');
};
let failed = 0;
async function screen(n, name, steps) {
  try {
    await steps();
    await sleep(1500);
    shot(n);
  } catch (e) {
    failed++;
    console.log(`FAIL ${name}: ${e.message}`);
  }
}

try {
  await until('the page to be built', () => ev("document.readyState === 'complete' && !!document.getElementById('playBtn')"), 90000);
  demoMode();

  // 1. Now Playing: a loop playing, a few seconds in.
  await screen(1, 'Now playing', async () => {
    await click('playBtn');
    await until('the player to play', () => ev("document.getElementById('playBtn').getAttribute('aria-label') === 'Pause'"), 60000);
    await sleep(6000);
    await goNow();
  });
  // The bottom-right key opens the last of Sound and Layers, and switches
  // between them from there: press it until the one wanted is shown.
  const edit = async (view) => {
    await goNow();
    for (let i = 0; i < 2; i++) {
      await click('viewBtn');
      await sleep(600);
      if (await ev(`!document.getElementById('${view}').hidden`)) return;
    }
    await shown(view);
  };
  // 2. Layers: what is playing, drawn as it plays.
  await screen(2, 'Layers', async () => {
    await edit('viewLayers');
    await sleep(5000);
  });
  // 3. Sound: the loop's sliders.
  await screen(3, 'Sound', async () => {
    await edit('viewSounds');
  });
  // 4. The vibe panel: what new loops lean toward.
  await screen(4, 'Vibe', async () => {
    await goNow();
    await click('vibesBtn');
    await shown('viewVibe');
  });
  // 5. The Library.
  await screen(5, 'Library', async () => {
    await click('libraryBtn');
    await shown('viewLibrary');
  });
  // 6. Save and share.
  await screen(6, 'Save and share', async () => {
    await click('saveBtn');
    await shown('viewSave');
  });
} finally {
  page.close();
}
const taken = fs.readdirSync(out).filter((f) => /^\d+\.png$/.test(f)).length;
console.log(`${taken} screenshot(s) taken, ${failed} failed`);
process.exit(taken >= 2 && failed === 0 ? 0 : 1);
