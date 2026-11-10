# Session 29: the Android app: F-Droid's metadata, recipe and icons (107e)

**Changed by brain B1 (2026-11-11):** the recipe's id and name are filled in by session 31a (the new name at 1.0), and the submission waits until after 1.0, since fdroiddata's merge request is public.

- **Model:** Sonnet 5.5. **Effort:** high.
- **Start after:** session 28.
- **Queue item:** item 107.

Paste everything in the block as the session's first message.

````text
Hands session 29 on the road to Driftloom's 1.0 (Sonnet 5.5, high): the Android app: F-Droid's metadata, recipe and icons (107e).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 107); docs/CORE.md ("the platforms"; promises 1, 2 and 4); docs/DECISIONS.md ("Platforms and games"); docs/PAINTER.md ("Platforms"); F-Droid's own docs, read now, not from memory (the inclusion policy, the build metadata reference, reproducible builds, fastlane's metadata layout); sessions 25 to 28's PRs; js/main.js (item 100's APP_LINK).

Check your place: in docs/sessions/README.md every row above session 29 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

The work, one PR, and a note for Mikey:

- fastlane metadata in the repo (fastlane/metadata/android/en-US/: title, short and full description in plain words, changelogs/<versionCode>.txt, the icon, a feature graphic and phone screenshots taken from the app in CI's emulator). The app's adaptive icon with its monochrome layer, from the app's existing artwork; a Painter may replace it.
- An F-Droid build recipe (the metadata YAML fdroiddata wants), for the release id session 31a sets (write it so the id and the name are one value each: 31a fills them in), tested by building the app from a clean checkout with F-Droid's own tooling in CI (fdroidserver's build, or its documented container), including the Rust toolchain and the NDK the recipe must install from source-available places; the versions pinned.
- Reproducible-build notes (docs/notes/107-reproducible.md): what makes the APK byte-for-byte the same on two machines (pinned toolchains, the path remapping the core already uses for its .wasm, timestamps, the order of files), and a CI job that builds it twice and compares. If it is not yet reproducible, say exactly what differs.
- The licenses of everything in the APK listed (the repo is MIT; the fonts and any other asset checked), and an anti-features check: none expected.
- The version name and code scheme (1.0 is session 31's to set), and the release workflow: a tag builds an unsigned release APK as an artifact.
- For Mikey (docs/notes/for-mikey.md, step by step in plain words; the submission itself comes after session 31, since the merge request to fdroiddata is public and carries the new name): the signing choice (F-Droid signs it, or his own key with reproducible builds: what each means for him, and the recommendation), and how to submit (the merge request to fdroiddata, with the recipe's text ready to paste; the Requests for Packaging route if he prefers). Do not create accounts or submit anything.
- Item 100's link: set APP_LINK to the F-Droid listing's address it will have (packages/<id>), behind the fact that it 404s until the listing is live: leave it empty and put one line in for-mikey.md to set it when the listing exists.

Yardsticks: F-Droid's tooling builds the APK from a clean checkout in CI; the two-builds comparison's result; the metadata passes fdroidserver's lint; the screenshots are the app's own.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 29's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
