# 101h: how many changes a loop gets, with all fourteen in

Hands session 09, 2026-11-16, for Mikey (and the brain). Nothing here is
built: it is a proposal, because each way of restoring the rule changes the
choices of about half the loops that exist today.

## What moved

Mikey's rule (2026-10-18): most loops get one or two of the changes. The
list of choices still draws its changes as it did (`changes = 0 15, 1 45, 2
30, 3 10`), but batch 6 added nine draws beside it, each on a stream of its
own and "not counted among a loop's changes" by the data's own words: five
choices drawn alone (padWow, widePad, tripletKit, dottedPulse, colorTone)
and four settings (tuningOffset, planingSevenths, lowBell with dropSpotlight,
twang). Together they add about one change a loop. The other four batch
sounds (deepDuck, wideSweep, slowOpening and dropSpotlight's bell) are
options or kinds of a draw that was already there and add none.

Over 10,000 loops (corpus seed 1, as `stats.mjs` draws it; every option
other than today's, and each of the four settings when it plays, counted as
a change):

| Changes a loop gets | 0 | 1 | 2 | 3 | 4 | 5+ | one or two | three or more |
|---|---|---|---|---|---|---|---|---|
| Before batch 6 (de2fc1f, v155) | 14.5% | 45.8% | 29.5% | 10.2% | | | **75.3%** | 10.2% |
| Now (v174, all fourteen in) | 4.8% | 21.5% | 30.7% | 24.6% | 12.8% | 5.6% | **52.2%** | 43.1% |

Still "most", barely, but three or more went from 1 loop in 10 to 4 in 10:
the batch's note that "three is rare enough that a loop never feels like a
demo of the batch" no longer holds. (`node tools/stats.mjs --choices` counts
the list's choices, the five alone among them, but not the four settings:
it reads 65.0% one or two.)

## The ways back, measured on the same 10,000 loops

| Way | 0 | 1 | 2 | 3+ | one or two | Loops whose sound changes |
|---|---|---|---|---|---|---|
| **A. The nine count against the changes a loop drew** (the list fills only what they leave: a loop that drew two changes and has twang gets one from the list) | 4.8% | 39.4% | 39.4% | 16.4% | **78.8%** | 56.6% (half the list's changes go, the batch's none) |
| B. No loop over two: the list gives way where the nine leave less | 4.8% | 21.5% | 66.8% | 7.0% | 88.2% | 42.0% |
| C. The nine's shares halved (0.15 to 0.075) | 8.7% | 31.6% | 33.5% | 26.2% | 65.1% | 41.2% (half the batch's loops lose it) |
| `changes` weights alone | can't restore it: the nine alone give 33.6% none, 39.2% one, 20.2% two, 7.0% three or more | | | | | |

**The hands' recommendation: A.** It keeps the shape Mikey approved for the
list (a few loops with none, most with one or two, three rare) and every
batch 6 sound at its approved share; what gives way is the oldest changes,
and only in loops that already have their share. It is the only way that
needs code, not data: the list's draw must know the nine before it draws,
and two of them (lowBell, twang) depend on what the loop plays, so they
would be counted where they are drawn, before their eligibility (a loop
that drew twang but has a busy kit would still spend a change on it), or
the draw reordered. B is the smallest edit (a cap, one line of data and a
few in the draw) but makes two changes the norm. C trades batch 6's
presence for the old list.

Each changes the choices of 41 to 57% of the loops that exist today (their
codes would play differently), so it is Mikey's call, not the hands'. Until
he says, nothing moves; the generation goldens, the baseline and the
balance lock are as #400 left them.
