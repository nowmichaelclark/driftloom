# The docs: a map

Which doc is for what, and who reads it. Every role loads only its own live
docs; the history is in `docs/archive/`, which nothing loads by default.
Sizes are limited and checked in CI (`node tools/check-docs.mjs`).

## Start here, by role

| Role | Starts at | Then |
|---|---|---|
| Any agent | `AGENTS.md` (the root) | the role's start below |
| Brain | `docs/BRAIN.md`, "Start here" | `docs/DECISIONS.md`, `docs/QUEUE.md`, `ROADMAP.md` |
| Hands | `docs/QUEUE.md` | `core/AGENTS.md` for `core/`; `docs/notes/` for open notes |
| Ears | `docs/ears/sessions.md` | `docs/ears/plan.md`, `docs/EARS.md`, `tools/ears/README.md` |
| Painter | `docs/PAINTER.md` | `design/README.md`, `design/next-picks.md` |

## The live docs

| Doc | What it is | Read by |
|---|---|---|
| `AGENTS.md`, `CLAUDE.md` | What Driftloom is, the roles, the rules that hold everywhere, build and test. `CLAUDE.md` imports it. | everyone |
| `core/AGENTS.md` | The core's standing rules, and the rules for sound items. | hands (core) |
| `docs/QUEUE.md` | The work queue: status, standing rules, the open items, known issues, the merge policy. | hands, brain |
| `docs/DONE.md` | One line for each merged PR since #277. | hands (to add), brain |
| `docs/notes/` | One file for each note from the hands (`<item>-<name>.md`). | brain |
| `docs/BRAIN.md` | How the work is organized, getting oriented, "Start here", listening albums, methods, tools. | brain |
| `docs/DECISIONS.md` | What Mikey has decided: one dated line each, by topic. | brain, hands |
| `ROADMAP.md` | What is still ahead, what was decided against, the contribution rules. | brain |
| `docs/CORE.md` | The core's charter: ten promises, each with its measure, and the phases. | brain, hands (core) |
| `docs/PAINTER.md`, `design/` | What there is to paint; the design spec and the picks' brief. | Painter |
| `docs/EARS.md`, `docs/ears/`, `tools/ears/` | What the ears sessions found and the yardsticks; their plan, reports, album cards and tools. `docs/ears/README.md` indexes the folder. | ears, brain |
| `docs/MOODS.md`, `docs/VIBES.md` | The mood words, and the vibe recipes (the source is `js/vibes.js`). | brain |
| `docs/ALBUMS.md` | Listening albums out for Mikey. | brain |
| `README.md`, `core/README.md`, `core/golden/README.md` | The public front page; how a host drives the core and how to add a block; the golden renders. | everyone; core developers |
| `Mikey's Thoughts.md` | His. | Mikey |

## The archive (`docs/archive/`)

Out of the Project sync. Open one when a question needs it.

| File | What it holds |
|---|---|
| `queue-items.md` | The full text of every finished item, with an index; an item number like "item 46" means the text there. Also the queue's old status notes. |
| `notes.md`, `notes/` | The hands' notes: everything before 2026-10-26 in `notes.md`; closed notes from `docs/notes/` after. |
| `done.md` | Done lines before #277. |
| `brain-log.md` | The brain's log and the early History, newest first. |
| `decisions-reasoning.md` | The long reasoning behind the short lines in `docs/DECISIONS.md`. |
| `roadmap-shipped.md` | The roadmap items that shipped or closed. |
| `engineering-notes.md` | How Driftloom works inside (it was the README's body): feeling, measurement, the core, the voices. |
| `perf-baseline.md` | The performance baseline (v54) and what came of it. |
| `session26-clips.md` | The brain's clip renderer from Session 26 (forced choices, a moved constant), for item 83. |

Archived text keeps the pointers it was written with; where one names a
heading that moved, this map and the headings above say where it went.
