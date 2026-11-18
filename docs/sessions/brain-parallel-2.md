# Brain session B-parallel 2: the chains' ends and Mikey's seven, two at a time

Written by brain B-parallel (2026-11-18), at Mikey's request, when that session grew long: it ran 17 to 23, 26 and 27, and handed over with 24 and 28 at their ends. This one picks up the same work under the same rules, up to the point where 30e is next.

- **Model:** Opus 5.5. **Effort:** high.
- **Where:** a Claude Code session (the desktop app's Code tab) on the repo's own folder, not a worktree: it starts the hands sessions itself, each in its own worktree.
- **Start after:** brain B-parallel (its State in `docs/BRAIN.md` says what it left running).

Paste everything in the block as the session's first message.

````text
Brain session B-parallel 2 for Driftloom's road to 1.0 (Opus 5.5, high), in Claude Code: finish the chains and start Mikey's seven, two hands sessions at a time, and oversee them.

Start as docs/BRAIN.md says ("Start here": brain B-parallel's bullet first, its State is where you pick up). Then read docs/sessions/README.md (the order from 17 on: every row's After and Alongside, "Mikey's seven", and "Rules for every session"), docs/QUEUE.md (status, the road to 1.0, items 105, 107 and 109 to 114, the merge policy), docs/notes/for-mikey.md, docs/DECISIONS.md's lines of 2026-11-17 and 2026-11-18, and the prompts still to come (24b, 29, 30a to 30d; brain-4 and brain-5 for where you stop). You are the brain: you verify, plan, keep the docs true and merge docs-only changes (Mikey, 2026-11-18: docs-only fixes merge without asking, once CI is green); you never write app or core code.

1. Pick up what B-parallel left. Its State names each hands session still open (24, 28) with its PR. Read each PR and its CI. A session that is idle with its PR green needs a message to merge it; a session stopped at the usage limit needs a message to carry on (when Mikey says "try again"); a session waiting on Mikey's answer is his to answer, not yours.

2. Run the rows that are left, in their order, at most two hands sessions at once on this machine (about 7 GB; it crashed once on 2026-11-18; never two heavy local runs: tools/gate.sh, tools/longform.mjs, tools/baseline.mjs, a gradle build with an emulator; CI runs those). The rows: 24b after 24 (an album's code carries how each track plays; before B4, since the format freezes at 1.0); 29 after 28 (F-Droid); 30b now (it needed only 27); 30d after 23 (done); 30a after 24; 30c after 21, 24 and 28. Check each row's After and Alongside in the README before starting it. Start each with start_session, use_worktree true, the prompt's block from its file as the whole brief, the model and effort its file names, and a background naming who runs alongside and the rules below. Prefer the critical path: 24 then 24b (B4 waits on them), and 28 then 29 (Android's end).

3. Oversee. When a session reports, check its PR against its prompt: CI green on its last commit, the yardsticks, the Done line, the README row ticked with every cell's `|` kept (session 21's tick dropped one). Where a claim matters, measure it yourself, one heavy run at a time, or read it from CI. After any PR that moves what is generated or played, look at the next nightly (`.github/workflows/nightly.yml`): B-parallel missed that #414 left the stored render hashes stale, and the nightly stayed red a day.

4. B4 is not yours. When 18, 21, 24 and 24b have merged, tell Mikey that brain B4 (docs/sessions/brain-4-codes.md, a claude.ai Project chat) can start; it may run while Android's and the seven's sessions go on. Stop when 29 and 30a to 30d have merged: 30e, 30f and 30 are the next brain's or Mikey's, in the table's order. Stop earlier if Mikey's usage runs low; leave every running session's state in docs/BRAIN.md's State.

What B-parallel learned (docs/BRAIN.md has the detail):
- A session ends its turn while its CI runs, even when told to wait, and an idle session can't merge: run your own background wait on its PR (until its checks finish or it merges), then message it to merge.
- The app starts a session only on Mikey's own words in your chat. From a turn that a session's report or a background event began, propose it instead (initiation "own_initiative"): he clicks the card or says "start it".
- Usage limits stop sessions mid-turn: when Mikey says "try again", message each to carry on from where it stopped.
- Two PRs taking the same version: tell both up front; the second merges main in, takes main's js/dlplayer.wasm and BUILD/CACHE, rebuilds, takes the next number.
- From a worktree, `gh pr create` needs `--repo nowmichaelclark/driftloom --head <branch>`. Commits are authored `Mikey <162073748+nowmichaelclark@users.noreply.github.com>` with `git -c user.name=... -c user.email=...`; if a session's own permission check refuses that, it is Mikey's to answer in that session.
- The new name: never write it, its domain or the Android release id into this public repo (Mikey pastes them into 31a). It is already in three files by mistake (`docs/DECISIONS.md`'s export line of 2026-11-17, `docs/sessions/21-export-small.md`, `test/ogg.test.mjs`); B-parallel asked Mikey whether to replace them with a placeholder: follow his answer.
- The Android emulator smoke job was fixed in #420; a branch cut before it may still flake: one re-run, and it isn't part of the gate.

Always, before you end: Mikey's answers as dated lines in docs/DECISIONS.md, answered lines out of for-mikey.md; docs/BRAIN.md's State current (which sessions ran, merged, are open, and why); the archive pass and node tools/check-docs.mjs. Tell Mikey in a few plain lines: what merged, what you verified, what waits on him, and what is still running.
````
