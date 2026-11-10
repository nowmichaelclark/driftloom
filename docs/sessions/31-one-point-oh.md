# Session 31: 1.0 (item 108)

- **Model:** Sonnet 5.5. **Effort:** medium.
- **Start after:** brain session B5, and Mikey's go-ahead written in docs/notes/for-mikey.md or the queue.
- **Queue item:** item 108.

Paste everything in the block as the session's first message.

````text
Hands session 31 on the road to Driftloom's 1.0 (Sonnet 5.5, medium): 1.0 (item 108).

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, item 108); docs/notes/for-mikey.md (it should be empty or answered: if it is not, stop and list what is open); CHANGELOG.md; session 29's PR (the version scheme, the release workflow, the fastlane changelog).

Check your place: in docs/sessions/README.md every row above session 31 should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

If the brain's release check (B5) has not written "go" in the queue's status, or for-mikey.md still has open lines that the brain marked as blocking, stop and say so.

The work, one PR and a tag:

- The version named 1.0 everywhere it shows: the app (BUILD stays the cache's own counter; add or set the listener-facing version line), the APK's versionName and versionCode, the fastlane changelog for that code, CHANGELOG.md's 1.0 entry in plain words.
- Codes frozen: the code format's version recorded as 1.0's in core/README.md, and a test that fails if a version 2 code made today ever decodes differently (a file of golden codes with what they decode to, as the generation goldens are kept).
- The nightly's checks back as merge gates (item 103: every target's golden renders and the rest hold a merge again from here): move them, or require them, in the workflows, and update the merge policy's text in the queue and AGENTS.md.
- Tag the release as session 29's workflow expects; check the workflow's APK. Do not submit to F-Droid or touch Mikey's accounts: the last line of for-mikey.md says what he does next.

Yardsticks: the whole gate and the full nightly green on the release commit; the APK built from the tag by F-Droid's tooling matches CI's (or the known differences, listed); the site at the release commit plays, and its Diagnostics shows the version.

When you finish: add the merged PRs' lines to docs/DONE.md, keep the queue's status line current, tick session 31's row in docs/sessions/README.md, and end with a plain summary for Mikey: what merged (PR numbers), what to listen for or check, and what you added to docs/notes/for-mikey.md. Do this session's work and stop: the next session has its own prompt.
````
