# Driftloom for Android

Queue item 107. This folder is the Android app: one activity, one WebView,
the page's own files inside the APK, served to it from a local origin, and the
core built natively for Android's own audio. Session 25 (107a) made the shell
and its CI build; session 26 (107b) the core native on AAudio, with a debug
screen and tests; session 27 (107c) the page talking to it, so the app plays
on the native core and the WebView plays no audio of its own; session 28 (107d) the screen
off, the notification and saving an export; session 29 (107e) F-Droid's
listing, build recipe, release workflow and checks (the sections "F-Droid",
"Versions and releases" and "Licenses" below).

## What is here

- `app/src/main/kotlin/app/loops/MainActivity.kt`: the WebView. No network
  (`blockNetworkLoads`, no permission in the manifest), no Safe Browsing, no
  metrics; a link out of the app opens in the listener's browser.
- `.../BundledSite.kt`: the `WebViewAssetLoader` handler, with each file's
  type spelled out (a module must be `text/javascript`, a `.wasm`
  `application/wasm`). The origin is `https://appassets.androidplatform.net/`;
  the page's saved loops live under it, so it never changes.
- `app/build.gradle.kts`: the `bundleApp` task copies the app's files into the
  APK from the repo root, taking the list from `sw.js`'s `FILES` (the one list;
  nothing is committed twice). A file the page needs goes into `sw.js` as ever.
  `speed.html` and `rank.html` are not on it, so they are not in the app.
- `native/`: the core for Android (session 26), a small crate of its own over
  `core/` (a path dependency: the core stays `no_std` with no dependencies):
  `src/engine.rs` (the core and its player, the lock-free queue the commands
  come by, the queue back its reports go by, and `fill`, the audio thread's
  one call), `src/bridge.rs` (the page's messages as commands, and the
  reports back as the worklet's messages; `src/json.rs` reads them),
  `src/aaudio.rs` (the stream), `src/jni.rs` (five JVM calls, by hand),
  `src/watch.rs` (what a phone can say about a crackle, for Diagnostics),
  `src/lib.rs` (the bridge's functions).
  `build.sh` builds `libdriftloom.so` for each ABI; `abi-check.c` holds the
  numbers written by hand there to the NDK's own headers at every build.
- `app/src/main/kotlin/app/loops/NativeCore.kt`: the bridge's Kotlin side
  (start and stop the stream, load a seed with a request, play, pause, next,
  volume, the counts, an offline render; the page's `send`, `drain` and
  `pageGone`).
- `.../PageBridge.kt` (session 27): puts `DriftloomNative` in the page (a
  WebView message listener for the app's origin) and carries text both ways
  on a thread of its own: each message from the page into the core's bridge,
  and every 25 ms what is due back (`js/native.js` is the page's end).
- `.../Playback.kt` (session 28): what the page has told the app (what is
  playing, the loop's name and own cover, a file being saved), and the
  `StreamKeeper`, which keeps the native stream running only while something
  needs it (playing, or the ring-out after a pause, or a message to the core).
- `.../PlaybackService.kt`: the foreground service (media playback; data sync
  while a file is saved with the music paused) with its notification, the
  `MediaSession` (play, pause, next, previous; the loop's name and its cover),
  audio focus, the headphones-pulled pause and a partial wake lock held only
  while playing or saving. A button's action goes to the page, which does what
  its own button does; with no page the service ends.
- `.../AppBridge.kt` (the page's `DriftloomApp`, `js/app.js` its end): the
  page's media state and cover, Android's own picker for a file to save
  (`SaveFile.kt`, written as it is made, free space from the platform's
  `fstatvfs`), the restore file's picker, and the one notification question.
- `background.mjs`, `emulator-background.sh`: the screen-off test (CI's
  `background` job), with the debug build's `CallSimulator` standing in for a
  call; it presses Play after two minutes paused with the screen off.
- `burn.mjs`, `emulator-burn.sh` (session 30g): the "lighter on processor"
  rule on the native core, a slower phone made with `?burn=` and swept
  (CI's `burn` job, by hand only: `only=burn`).
- `app/src/debug/`: the **Native audio test**, a second launcher icon in debug
  builds only (`NativeTestActivity.kt`): "Play the test" plays the golden loop
  natively and shows underruns, the busiest callback against its deadline, and
  late notes; "Check the samples" renders it offline and says whether its hash
  is the web's.
- `app/src/androidTest/`: the instrumented tests (`NativeCoreTest.kt`): the
  samples against the golden line and js/dlplayer.wasm's, a device change, and
  five minutes of play with no underrun and nothing late.
- `fdroid/` (session 29): everything F-Droid needs and CI's `fdroid` workflow
  runs: `recipe.yml.in` and `render.mjs` (the build recipe, filled from
  `gradle.properties`), `build-in-container.sh` (the build in F-Droid's own
  buildserver container), `check-metadata.mjs` (the listing's files and the
  version scheme), `check-apk.sh` (permissions, trackers, what the APK holds,
  the licenses list), `compare-apks.sh` (two builds, byte for byte).
  `emulator-release.sh` plays a release APK on an emulator;
  `screenshots.mjs`, `emulator-screenshots.sh` take the listing's screenshots.
  `../fastlane/` holds the listing itself.
- `smoke.mjs`, `emulator-smoke.sh`: the smoke test (the page plays on the
  native core; its main thread blocked 5 s, the music plays on).
  `emulator-rows.sh`: `tools/rows.mjs --app`, every row of item 94a's table
  that makes sense in the app. `cdp.mjs`: the WebView's DevTools client both
  use, which finds the app's process and page as they are now on every
  connect. `emulator-native.sh`: the instrumented tests on an emulator with
  audio. `emulator-lib.sh`: the boot they share. `check-tree.sh`: no
  binary in the tree but the Gradle wrapper's jar, which is checked against
  Gradle's published checksum. `test-signing/`: the test builds' key.

## Build

```bash
cd android && ./gradlew assembleDebug   # app/build/outputs/apk/debug/driftloom-*-debug.apk
```

One APK for each ABI (arm64-v8a for phones, armeabi-v7a for older 32-bit
ones, x86_64 for the emulator) and `driftloom-universal-debug.apk` with all
three. `-Pdriftloom.abis=x86_64` builds one ABI, quicker.

Needs JDK 17 or newer and the Android SDK (`ANDROID_HOME`, or `sdk.dir` in
`android/local.properties`): platform 36, build-tools 36.0.0 and the NDK that
`driftloom.ndkVersion` names (28.2.13676358, r28c); and Rust through rustup
(`android/native/rust-toolchain.toml` pins the core's 1.99.0 with Android's
three targets, which rustup fetches). The `buildNative` task runs
`native/build.sh`, so a clean checkout builds everything from source. The Gradle
wrapper pins Gradle 8.14.3 by checksum. Android Gradle Plugin 8.13.2, Kotlin
2.3.21, and one library, `androidx.webkit` (Apache-2.0, Google's Maven), which
is `WebViewAssetLoader`. No Google services, no other repository.

`cd android/native && cargo test` runs the native crate's own tests on a
computer (the queue, the engine's path against the golden hashes, nothing
allocated in `fill`); `DL_WASM_LINE` adds a line from `tools/player-wasm.mjs`.

`node android/smoke.mjs --cdp URL` runs the smoke test against any DevTools
endpoint (how it was tried on desktop Chromium); `android/emulator-smoke.sh
APK` boots an AOSP emulator (needs KVM) and runs it on that, as
`android/emulator-rows.sh APK --long` does the rows. Both need a sound server
for the emulator (CI's is PulseAudio with a null sink). The workflow's
`smoke_runs` input (run it by hand) boots that many emulators for the smoke
test at once.

## The rename (session 31a): one line each

1. The id: `driftloom.applicationId` in `gradle.properties`.
2. The name: `driftloom.appName` in `gradle.properties` (the APK's file name).
3. The launcher label: `app_name` in `app/src/main/res/values/strings.xml`.

The Kotlin package (`app.loops`, the `namespace`) is not the id and stays.

## Choices

- **Min SDK 27** (Android 8.1). AAudio, the native audio path (26), arrived in
  API 26 but is only dependable from 27, which is where Oboe itself starts
  using it; and a WebView current enough for the page's modules, worklet and
  WebAssembly runs on 8.1 and up. Target and compile SDK 36.
- **Test builds are debug builds with a shared throwaway key**
  (`test-signing/README.md`), so each installs over the last (CI numbers them
  by run) and the app can be inspected over USB. The release gets another key.
- **AAudio directly, not Oboe** (session 26). Oboe is C++ over AAudio (and
  OpenSL ES below 8.1): CMake, the NDK's C++ library, and its source vendored
  as a submodule or Google's prebuilt package, which F-Droid would not take.
  AAudio is part of the system from 8.0, the NDK carries its header and stub,
  and the app uses a couple of dozen of its calls; Oboe's fallback below 8.1
  the app's floor doesn't need, and its reopening on a device change is a few
  lines in `native/src/aaudio.rs`. So the native side is Rust and the NDK's
  linker, nothing else.
- **The stream:** float, stereo, the device's own rate, the power-saving mode
  (session 30g, item 115: Android's deep buffer, a music player's path) in
  callbacks of 1,024 frames, as Chrome opens for the site's
  `latencyHint: 'playback'`; Diagnostics shows what the system gave, shared
  with other apps (exclusive is lower latency and more often trouble; the
  conservative choice), AAudio's own buffer size (until 30g the low-latency
  path, 192 frames a callback on the moto g: `docs/notes/115-crackles.md`). A device change (headphones in or out)
  closes the stream and opens another at the same rate; the loop plays on.
- **Nothing allocated or locked in the callback** (`docs/CORE.md`, promise 2):
  the commands come by a lock-free queue of fixed slots, the counts go back
  in atomics and the name by a sequence lock; `fill_allocates_nothing` counts
  allocations inside it over half a minute of callbacks with a load, a Next, a
  pause and a play.
- **One engine a process**, made at the first start at the device's rate and
  kept: the stream stops and starts around it.
- **The page on the native core** (session 27): one path, two transports.
  `js/core.js`'s `CoreHost` posts the worklet's very messages, starting with
  `init` (what a worklet's options carry: the quality, the layout), and hears
  the worklet's messages back, batched as the worklet batches them (events
  every 50 ms, or at once with one that is not a step; the cost every two
  seconds of audio). The page's context is a stand-in on the core's clock
  (`js/native.js`); the page still reads a loop's record and notes from its
  own module (`js/gen.js`), which is not audio. A new page's player (`init`)
  lets every voice go and builds the mix afresh at its quality on the same
  engine, so the quality switch is the browser's: the old fades, the new one
  takes the loop from its top. The bridge's text is read on the app's bridge
  thread, never the audio thread: the commands reach it as fixed slots, the
  reports leave it the same way, and `fill` still allocates nothing
  (`fill_allocates_nothing` runs every command the page sends). Too-slow
  (item 100) can't show in the app. A WebView without the message listener
  (older than 2020's) plays the page as a browser does. No service worker in
  the app: Diagnostics says `sw: none (the app holds its own files)`.
- **The screen off** (session 28): the page's `MediaBridge` (`js/media.js`)
  tells the app what plays; the service holds the rest. Paused, nothing is
  held: no wake lock, no focus, the stream closed after an eight-second
  ring-out; the notification stays with Play for fifteen minutes, then goes. A
  transient focus loss (a call) pauses and returns with the focus; a lasting
  one pauses for good; another app's short sound ducks the music (the system
  does that, as the request does not ask to pause when ducked). The page holds
  a Web Lock while it plays, so Chromium does not put it on its slow timer
  with the screen off. Back with music playing leaves the app like Home does
  (before Android 12 it would close the page, and the music with it); a swipe
  from the recents ends both. Permissions are the foreground service's
  (media playback, data sync), the notification (Android 13 and later: asked
  once, at the first Play with the app in front; refused, the music and the
  lock screen's controls work all the same), and the wake lock; no network.
- **Saving a file** (session 28): the export's bytes go from its Worker
  through the page to the app a megabyte at a time (base64 in the bridge's
  text; one piece at a time, each acknowledged), straight into the file the
  listener chose in Android's picker. FLAC's header is written again at the
  end, so a place that only streams (a cloud folder) is refused for Lossless
  with a plain line; Small is written front to back and goes anywhere. A MIDI
  file and the backup go through the same picker (`ui.download`). An album
  (item 121): one zip through the same picker (its headers written again at
  their places, so it needs a place that seeks), or a file a track into a
  folder chosen in Android's own folder picker (`save-folder`, then
  `save-in` a track, each written as a picked file is).
- **Workers** (session 28): a Worker's own requests (its imports, a `fetch`)
  are never served by `shouldInterceptRequest` (measured on the emulator,
  Chromium 124: the Worker's script loads, a Blob module Worker loads, one that
  imports a file of the app fails), so `js/worker-blob.js` makes the export's
  Worker from Blob URLs the page builds. The cover Worker (`js/covers.js`)
  fails the same way and falls back to drawing on the main thread, as it
  always did in the app; giving it the same loader is a follow-up.
- **The page while the screen is off** (session 28): measured, a WebView whose
  window is not shown is hidden to Chromium and its timers stopped for minutes
  (a 50 ms interval last ran 3 minutes before; the lock screen's buttons never
  reached the page). `AppWebView` tells the WebView it is shown while the app
  plays or saves; the page then runs as in front, the emulator's CPU for the
  app being 0% with the screen off.
- **The icon** (item 120): made from the one source, `icons/icon.svg`, by
  `node tools/icons.mjs` (with the web app's icons and the listing's), as
  vector drawables (text, as android/ must be): the adaptive icon's
  background, foreground and monochrome layers (the 108 dp layer, its 72 dp
  middle the icon itself) and the notification's small icon, the monochrome
  drawing at 24 dp (`drawable/ic_stat_driftloom.xml`). The script checks the
  safe zones, and that each drawable draws what the SVG draws.
- **Not yet:** the system bars' color following the page's theme (a Painter's
  choice).

## Versions and releases

**The scheme** (session 29). A release is `major.minor` or `major.minor.patch`,
and its version code is `major * 10000 + minor * 100 + patch`: 1.0 is 10000,
1.0.1 is 10001, 1.2 is 10200, 1.2.3 is 10203. Both live in `gradle.properties`
(`driftloom.versionName`, `driftloom.versionCode`), once; the tag is
`v<versionName>` (`v1.0`); and the changelog for the code is
`fastlane/metadata/android/en-US/changelogs/<versionCode>.txt` (F-Droid shows it
as "what's new"). A test build's name ends in `-test` and CI numbers its code by
run, so each installs over the last. `fdroid/check-metadata.mjs` holds all of
this (CI runs it; the release workflow runs it on the tag). Session 31 sets 1.0.
Room is left for per-ABI codes later (a factor of ten, F-Droid's `VercodeOperation`),
should the universal APK ever be too big.

**A release:** change the three values (name, code, changelog), merge, then

```sh
git tag v1.0 && git push origin v1.0
```

The `android-release` workflow checks the tag against `gradle.properties`,
checks that `js/dlplayer.wasm` is what `core/` builds, builds the unsigned
release APKs (one for each ABI and the universal one) and keeps them, with
their checksums, as the run's artifact. Nothing is signed or published by CI,
and no key is stored there: `docs/notes/for-mikey.md` ("Signing") says what is
done with the artifact.

## F-Droid

F-Droid builds the app itself, from the tagged commit, with its own tooling, and
signs it (or, with a reproducible build, publishes the APK you signed).

- **The listing** is `fastlane/metadata/android/en-US/`: the title (the app's
  name: the one place a listing holds it, so the rename is one file and the
  screenshots), the short and full descriptions, the changelogs, the icon, the
  feature graphic and the phone screenshots (taken from the app by the
  `screenshots` job of `android.yml`: download its artifact into
  `images/phoneScreenshots/`).
- **The recipe** (`fdroid/recipe.yml.in`, filled by `fdroid/render.mjs`): `node
  android/fdroid/render.mjs --commit <the tag's commit> --repo-url <the public
  repository's address>` prints the text of fdroiddata's `metadata/<id>.yml`.
  The id, the name and the version come from `gradle.properties`, so the rename
  (31a) changes nothing in it. `--signed-by <sha256>` adds the lines for an APK
  signed with your own key.
- **What it does**: builds from `android/` with Gradle's `assembleRelease` (the
  universal APK); F-Droid refuses what is prebuilt, so it deletes the committed
  `js/dlplayer.wasm` before its scan and builds it again (`tools/build-core.sh`,
  the same bytes: `--check` proves it), deletes the research archives and a
  `package.json` without a lock file, and installs the Rust compiler the
  repository pins with rustup's own script (the `rustup` source library at a
  pinned release). The NDK is the pinned r28c.
- **Tested in CI** (`.github/workflows/fdroid.yml`): `check-metadata.mjs`;
  fdroidserver's `lint` and `rewritemeta` on the recipe, in both forms; the
  update check finding a new tag; **the build in F-Droid's own buildserver
  container** (`build-in-container.sh`, modelled on the `fdroid build` job of
  fdroiddata's CI: `fdroid build --on-server` from a clean clone, with its
  scanner); that APK on an emulator, test-signed (`emulator-release.sh`: not
  debuggable, the page loads, Play plays, nothing crashes in logcat, which is
  what R8's renaming could break); and two builds compared byte for byte
  (`docs/notes/107-reproducible.md`).
- **Release builds are R8-minified** and carry no Google dependency blob and no
  VCS stamp (`app/build.gradle.kts`).

## Licenses

`LICENSES.md` lists everything in the APK with its license; the repository is
MIT. `fdroid/check-apk.sh` (CI) fails when a library is in the release and not
in that list, when the list holds one that is not in the APK, when the APK asks
for a permission beyond the five the manifest names (none is the network), or
when it holds a tracker's or Google Play Services' classes. F-Droid's
anti-features (Ads, Tracking, NonFreeNet, NonFreeDep, NonFreeAdd,
NonFreeAssets, KnownVuln): none apply.
