# Brain session B-parallel: sessions 17 onward, two at a time

Written by brain B2 (2026-11-17), at Mikey's request: from 17 on the road
splits into four chains that barely touch each other, so this brain runs
them two at a time and oversees them, in place of starting each hands
session by hand.

- **Model:** Opus 5.5. **Effort:** high.
- **Where:** a Claude Code session (the desktop app's Code tab) on the repo's
  own folder, not a worktree and not a claude.ai chat: it starts the hands
  sessions itself, each in its own worktree.
- **Start after:** brain B2 (it may run while Mikey listens to the Batch 6
  Album).

Paste everything in the block as the session's first message.

````text
Brain session B-parallel for Driftloom's road to 1.0 (Opus 5.5, high), in Claude Code: run sessions 17 onward two at a time, as chains, and oversee them.

Start as docs/BRAIN.md says ("Start here", brain B2's bullet first). Then read docs/sessions/README.md (the order, the rules for every session, what is ticked), docs/QUEUE.md (status, the road to 1.0, merge policy), docs/notes/for-mikey.md, and the prompts still to come in docs/sessions/ (17 onward, brain-4, brain-5). You are the brain: you verify, plan, keep the docs true and merge docs-only changes; you never write app or core code. Hands sessions do the work from their own prompts.

1. Make the plan parallel, as one docs-only PR you merge yourself when CI is green. The road from 17 on is four chains: long pieces 17 -> 18; export 20 -> 21; codes 22 -> 23 -> 24 -> B4; Android 26 -> 27 -> (Mikey installs 27's APK; the work goes on) -> 28 -> 29. Then 30, B5, 31a, 31 in order. Check each dependency by reading the prompts and the code, not from this list. My reading: 22 needs nothing from 17 to 21 (a code carries no jump and no export), so its "Start after: session 21" becomes "after B2"; 27 waits for 17 (both change js/worklet.js and js/player.js); 23 never runs alongside 18 or 20 (the same app files). Rewrite the README's order table as the chains, and mark which rows may run alongside which, so each prompt's "check your place" passes for a session the plan allows. Correct each prompt's "Start after", saying what you changed and why at the top of its file. Add to "Rules for every session": before merging, merge the latest main into your branch (force-pushes are refused on Mikey's machine); where `js/dlplayer.wasm` or BUILD/CACHE conflict, take main's, rebuild with tools/build-core.sh, take the next version number, and let CI run again on that last commit.

2. Run the chains. Start hands sessions with start_session, each in its own worktree (use_worktree true), the prompt's block from its file as the session's whole brief, the model and effort its file names. At most two hands sessions at once on this machine (about 7 GB; never two local heavy runs: gate.sh, longform.mjs, baseline.mjs; CI runs those). A third only when it is app-only and light (18, 21 or 24). Your own checks stay light while two are running. Start the longest chains first (Android, then codes), and fill the second place with 17 -> 18 or 20 -> 21. Never start two sessions whose files overlap beyond the shared spots (BUILD/CACHE, js/dlplayer.wasm, core/src/wasm.rs, core/include/dlcore.h, docs/DONE.md, the queue's status line, the README's table). Merging stays each hands session's job, under the merge policy.

3. Oversee. When a session reports, check its PR against what its prompt asked: CI green on the last commit, the prompt's yardsticks, the Done line, the README row ticked. Where a claim matters, measure it yourself, as B2 did, one heavy run at a time: the module matches its source, the goldens, the balance lock, what didn't move. Answer a blocked session or ask Mikey. Then start the next session in that chain. What needs Mikey's ears, phone or accounts goes in docs/notes/for-mikey.md as one line, and the chains don't wait for him, except where a row says so (the APK check before 28; the signing choice and the domain before 31a).

4. Stop at B4. When 24 has merged, B4's prompt (docs/sessions/brain-4-codes.md) is the next brain session's, not yours. End there, or earlier if Mikey's usage runs low; leave every running session's state in docs/BRAIN.md's State.

Always, before you end: Mikey's answers as dated lines in docs/DECISIONS.md, answered lines out of for-mikey.md; docs/BRAIN.md's State current (which sessions ran, merged, are open, and why); the archive pass and node tools/check-docs.mjs. Tell Mikey in a few plain lines: what merged, what you verified, what waits on him, and what is still running.
````
