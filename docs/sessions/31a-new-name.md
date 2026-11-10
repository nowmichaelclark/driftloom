# Session 31a: the new name (before 1.0)

Written by brain B1 (2026-11-11). Mikey renames the app at 1.0, with a repo
rename and its own domain, and the testers must not see it before then
(`docs/DECISIONS.md`, 2026-11-11). So the name, the domain and the Android
release id are not in this public repo: **Mikey pastes them into the
opening message** (the brain keeps them in the Driftloom Project's private
docs). Everything before this session keeps the old name and writes it from
one place, so this is a short list of edits.

- **Model:** Opus 5.5. **Effort:** high.
- **Start after:** brain session B5's "go", the domain bought and attached
  to the Worker (`docs/notes/for-mikey.md`, Accounts), and session 29.
- **Queue item:** item 108's rename.

Paste everything in the block as the session's first message, with the
three lines at its top filled in.

````text
The new name: ____
The domain: ____
The Android release id: ____

Hands session 31a on the road to Driftloom's 1.0 (Opus 5.5, high): the new name.

Read first: AGENTS.md; docs/sessions/README.md (the order, and "Rules for every session": they are part of this prompt); docs/QUEUE.md (status, standing rules, merge policy, items 107 and 108); docs/DECISIONS.md (2026-11-11, the new name); docs/notes/for-mikey.md; wrangler.jsonc; sw.js; js/storage.js (the Library and the backup); the Android project's id, name and label; the fastlane metadata and the F-Droid recipe (session 29).

Check your place: in docs/sessions/README.md every row above session 31a should be ticked, apart from rows the table says may run meanwhile. If one is not, stop and say so. If this file has a "Left" list at its top, an earlier session with this prompt stopped partway: start from that list.

The new name is a surprise for the testers until 1.0: pushing to main deploys, so this session's PR is built, checked and left open, and is merged only by session 31, right before its own (an exception to the merge policy, for this reason alone). Nothing about the new name goes into a commit on main, an issue, or another PR before then.

The work, one PR:

- The visible name everywhere it shows: the page's title and its one name constant, the manifest (name, short name, the icon's text if any), the card and About, the link preview, README.md and CHANGELOG.md, the Android app's name and label, its release id (applicationId for release builds; test builds keep the working id), fastlane's title and descriptions, the F-Droid recipe's id and name. Identifiers inside the code (dlcore, dl_ exports, js/dlplayer.wasm, the dlrender and dlgen tools) stay: renaming them would move nothing a listener sees.
- The site on the new domain (wrangler.jsonc's custom domain, as Cloudflare's docs say for a Worker), and the old address answering every path with a redirect to the same path on the new one, the link's code kept (it is in the fragment or query, as session 24 chose: check it survives).
- The old address's service worker steps aside: testers have the app installed at the old address, so a last version of sw.js there must stop serving the cached app and send them on (and the installed app's start page says where it went).
- The testers' Library: storage is per address, so their saved loops and albums would not follow. Carry them: the old address offers, once, to move the Library to the new one (the backup, handed over by a link or a file, as js/storage.js's backup and restore already do), and the new address takes it. Nothing is lost if they decline: the old backup still works.
- For Mikey, in docs/notes/for-mikey.md, step by step: before session 31 merges, check the new domain answers; after it, rename the repo on GitHub (the old URL redirects), check that Cloudflare's build still runs for the renamed repo, and re-link it if not.

Yardsticks: grep shows the old visible name only in docs/archive, history, and the identifiers listed above; a fresh browser on the new domain plays; the old address redirects every path with the code kept; an installed app at the old address (Chromium, scripted) is sent on and its Library moves across; the Android release build shows the new name and id, the test build keeps the working id; the F-Droid recipe passes fdroidserver's lint; the gate green on the branch.

When you finish: don't merge; tick session 31a's row in docs/sessions/README.md in the same PR, and end with a plain summary for Mikey: the PR number, what he does on GitHub and Cloudflare, and that session 31 merges it.
````
