# Session 25: the Android app: the project and its CI build (107a)

**Changed by brain B1 (2026-11-11):** the package name answered: a working id for test builds; the release id and the new name come at session 31a (Mikey renames the app at 1.0, a surprise for the testers).

- **Model:** Sonnet 5.5. **Effort:** high.
- **Start after:** session 01 at the earliest; before session 26. It may run alongside other sessions: it only adds android/ and a workflow.
- **Queue item:** item 107.

Paste everything in the block as the session's first message.

````text
Hands session 25 on the road to Driftloom's 1.0 (Sonnet 5.5, high): the Android app: the project and its CI build (107a).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 107); docs/CORE.md ("the platforms"; promises 1, 2 and 4); docs/DECISIONS.md ("Platforms and games"); docs/PAINTER.md ("Platforms"); wrangler.jsonc and .assetsignore (which files are the app); sw.js (the list of the app's files).

Check your place: in docs/sessions/README.md every row above session 25 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

The work, one PR: an Android project in android/ that builds in CI, and nothing more. No Google services, nothing F-Droid's inclusion rules refuse (read them: no proprietary dependencies, no prebuilt binaries in the repo, no trackers; Gradle wrapper pinned and verified).

- A minimal app with the working id io.github.nowmichaelclark.driftloom for test builds (the release id and name are set by session 31a: docs/DECISIONS.md, 2026-11-11; keep the id, the app's name and its label each in one Gradle property or resource so 31a changes one line each; test builds are debug-signed, so the release could never update over them anyway), min SDK chosen from what AAudio and a current WebView need (say why), Kotlin, one activity with a WebView that loads the app's own files bundled as assets (copied from the repo at build time by a Gradle task from the same list sw.js keeps, not committed twice), served to the WebView from a local origin (WebViewAssetLoader) so ES modules, the worklet and the .wasm load.
- For now the page plays as it does in a browser (its own worklet in the WebView): that proves the shell. The native audio is session 26's.
- A workflow (.github/workflows/android.yml): builds a debug APK on GitHub's runner on every PR that touches android/ or the app's files, and uploads it as an artifact Mikey can install. Not part of the merge gate yet.

Yardsticks: the APK builds from a clean checkout in CI (the workflow's run linked in the PR); it installs and starts on an emulator in CI (a smoke test: the page loads, Play is pressed, the page's Diagnostics shows the core's player with no error), headless; the APK's size; the tree has no binary but the Gradle wrapper jar (checked against its published checksum). In for-mikey.md: where to download the APK.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 25's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
