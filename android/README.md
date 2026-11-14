# Driftloom for Android

Queue item 107. This folder is the Android app: one activity, one WebView,
the page's own files inside the APK, served to it from a local origin. Session
25 (107a) made the shell and its CI build; the core's native audio is session
26, the page talking to it session 27, the background service and exports 28,
F-Droid's metadata 29 (`docs/sessions/`).

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
- `smoke.mjs`, `emulator-smoke.sh`: the smoke test. `check-tree.sh`: no
  binary in the tree but the Gradle wrapper's jar, which is checked against
  Gradle's published checksum. `test-signing/`: the test builds' key.

## Build

```bash
cd android && ./gradlew assembleDebug   # app/build/outputs/apk/debug/driftloom-debug.apk
```

Needs JDK 17 or newer and the Android SDK (`ANDROID_HOME`, or `sdk.dir` in
`android/local.properties`): platform 36 and build-tools 36.0.0. The Gradle
wrapper pins Gradle 8.14.3 by checksum. Android Gradle Plugin 8.13.2, Kotlin
2.3.21, and one library, `androidx.webkit` (Apache-2.0, Google's Maven), which
is `WebViewAssetLoader`. No Google services, no other repository.

`node android/smoke.mjs --cdp URL` runs the smoke test against any DevTools
endpoint (how it was tried on desktop Chromium); `android/emulator-smoke.sh
APK` boots an AOSP emulator (needs KVM) and runs it on that.

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
- **For now the page plays as in a browser** (its own worklet in the WebView).
  Its `sw.js` is not in the app, so Diagnostics says `sw: failed` there: noise
  to be silenced when the page next changes (session 27).
- **Not yet:** the screen off and a notification (28), saving an export and the
  restore file picker (28), the system bars' color following the page's theme
  (a Painter's choice), the launcher icon (a placeholder in plain vectors).
