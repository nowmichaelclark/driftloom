# Albums out for listening

Blind albums the brain has sent Mikey and not yet heard back on. Each
key is derivable from the album code itself (decode it with
`share.js decodeAlbum` and read the specs), so no key is written here.

**The codes are version 2** (session 23, v184): each album below was
converted from its version 1 code and checked to play the same notes. Each
track's length of play (the slider) travels with its album again (session
24b, v189): the six albums whose version 1 codes set it (43 tracks, every
Session 26 album, Phase 1 check, Heartwarming? and Joyful?) were written
again with it, checked to play the same notes for the same passes; the
others never set it, so their tracks play on until moved on, as they did.

## "Batch 6 Album" (brain B2, 2026-11-17, v177; items 101 and 84b) -- heard 2026-11-17

Page: <https://claude.ai/artifact/3zgBGFPzidGMt3wr1SQwzP> (private to its
owner; verdicts in its database, collection `verdicts`, one document a card,
a pair and `free`; the blind key in `brain/key`, which the page never shows).
About 28 minutes, headphones. Rendered by the brain with `tools/clips.mjs`
(the core's `dlrender`, stereo, wander 1, corpus seed 20261009), Opus 96 kbps.

- **Labeled, Keep / Tune / Drop (15 cards, 44 clips, about 22 minutes):**
  batch 6's sounds, wideSweep's and dottedPulse's two versions on cards of
  their own, from main at v177 (686543d), each on the loops `docs/DONE.md`
  names (#402's five far-end loops for deepDuck, fast and slowOpening). Each
  card's first loop has a With / Without switch that keeps the moment; the
  Without is the same request with the sound pinned to today's
  (`subDuck=today`, `tuning.cents=0`, `lowBell.kind=today`, ...). Under way A
  a batch 6 draw pinned off frees a change the list then fills, so that one
  is pinned off too (wen-glui `arc=today`, weith-yo `organ=today`): checked,
  every Without differs from its With in the sound alone. Card ids:
  `deepDuck`, `wideSweep-wide`, `wideSweep-fast`, `slowOpening`,
  `tuningOffset`, `padWow`, `widePad`, `tripletKit`, `dottedPulse-pulse`,
  `dottedPulse-kick332`, `colorTone`, `planingSevenths`, `lowBell`,
  `dropSpotlight`, `twang`.
- **Blind, 84b against the build before (7 pairs, about 6 minutes):** the six
  movers #404 names (shu-san, glai-thel, youn-ti, theim-rem, fum-doun,
  fo-roan, 28 s from 0:00, glai-thel from 0:01) and a control, thuing-huim
  (corpus #5), bit for bit the same on both builds; v176 (380ca7f) against
  v177, each pair matched to its quieter side's integrated loudness (gains
  0.00 to 0.14 dB: 84b moves the loops' loudness that little). Order and X / Y
  by a fixed seed, kept in `brain/key`; written here once he has answered.
  Pick X, Y or Same.
- **Heard 2026-11-17 (Mikey, the page's verdicts, read by B-parallel):**
  every card Keep (wideSweep wide in words, its card left blank) but twang,
  Tune: "I only liked the soft version. The bright and slow sounded off to
  me." **The key, now he has answered** (X / Y, his pick, so which build he
  chose): 1 glai-thel X=v177, X (84b); 2 fo-roan X=v177, X (84b); 3 shu-san
  X=v176, Y (84b); 4 the control (thuing-huim) Same; 5 fum-doun X=v176, Same;
  6 youn-ti X=v176, Y (84b); 7 theim-rem X=v177, X (84b). 84b chosen in 5 of
  6, never the build before, and the control heard as Same.

## "Long Pieces Album" (brain B3, 2026-11-14, v164; item 102) -- heard 2026-11-14

Page: <https://claude.ai/artifact/5G7YcuFc7iRvCNz8JtT4VD> (private to its
owner; verdicts in its database, collection `verdicts`, one document a pair,
a piece, a question and `free`; the blind key in `brain/key`, readable by the
owner only). Excerpts, since the app can't take a journey or jump yet:
session 15's set from the branch `long-clips` (`tools/longform-clips.mjs`,
main at v164; stereo Opus 96 kbps; the index on the branch has every clip's
piece, place and line), 33 clips, about 24 minutes. Ten pieces, each a card
of its clips and one pick (Keep, Too much, Too little, Wrong, a note), in
this order: three blind pairs, tha-vol whole, brum-yei and fei-no (256),
moan-teil and loa-yer (1,024), the wander (glor-ti, mu-meth, teth-hu), the
Endless arcs (brum-yei), gluith-vuisk's question; then four questions (the
arc felt, the joins, the wander's pace, the endings) and a free note.

The pairs, A and B by `random.Random(1102)`: pair 1 is session 15's
(brum-yei 256 bars from 0:55, now against dfe78c3); pair 2 the brain's own
(glor-ti Endless from 5:50, across the kit's 24-s fade, the same two builds,
level-matched to the quieter as session 15's script does); pair 3 a control
(session 15's pair 2, from 5:20: the same music both sides within -29 dB, the
fade starting in its last 7 s). Measured beside it: `docs/notes/102-b3-measured.md`.
Heard 2026-11-14, so the key is written here: pair 1, B was now (he chose
**B**); pair 2, A was now (**Same**); pair 3, the control (**Same**). Every
piece **Keep**; the arc "Good enough for now"; joins Welcome; the wander
Right; endings Real endings (`docs/notes/102-mikey.md`). `long-clips` deleted.

## "Phase 3 Control" (brain B1, 2026-11-11, v149 against v153) -- out

Item 98's control: the same album code in the app before phase 3 and in
today's, labeled (not blind: the brain's renders already answer the
question, so this is his check, not the test). It should sound the same.

- **Before** (v149, the page's Engine and Synth on the core):
  <https://f280ce1a-driftloom.nowmichaelclark.workers.dev>
- **Now** (v153, the core's player, no JavaScript synth):
  <https://driftloom.nowmichaelclark.workers.dev>
- Code (Paste a code, in **Now**; the **Before** build reads only version 1
  codes, so its code is in `docs/archive/notes/98-phase3-control-v1.md`):

```
FAGwBbtlgKASA83-t2lr0-kFfSBy78io-7ys8MfSC-r_m7ylqvyCfyBLNV-qBSA6tu78jjphnIfSBqwi88tnzmgLfSCs_6ng91uy6iBhB0BcJ-P0BSA278uwt2o2liChBSBq1q15u7olvJfSCk-wng4z5_thBhBwBKky18BSA2lt1hgk-p6JfSB-1v1pxszulKfSCglohg0o0qyFfwB-QwGEASA20lp7109xqMfSBw9rvm8kzytLfSCo8hvm3i-_gKfyBSCJMEDSA6omigjvzi4JfSBqwi2j07hotBdSC2-1vg6r9k1lChBePUGhhc2UgMyBDb250cm9syy
```

Tracks: tui-doask, sith-fai, shai-soung, soath-wa, le-ra, glur-va (corpus
20261009, loops 3, 2, 25, 28, 10 and 29 counting from 1; nothing asked). The brain rendered 30
corpus loops both ways (v149 in Chromium through its own Engine and Synth on
the core; v153's player in Node; 45 s, full, stereo, wander 1, 48 kHz): every
name the same, every start the same sample; 28 null at -82 to -90 dB (float
rounding); tui-doask and soath-wa differ after their silent bars (from 31.1 s
and 27.3 s), 25 to 50 dB under the music and fading over about ten seconds,
loudness within 0.08 dB: a phase or a held state, not a level. Session 01
found it (#361, v155): the width ceiling read a rest's zero mid as "far too
wide" and pulled the side down, in both apps; after the fix the brain's
renders of both loops match v149 at -88 dB, the other 28 unchanged (noar-te
-82 dB). One word a loop from Mikey.

## "Phase 2 Blind Album" (brain, 2026-11-07, v128 against v149) -- heard 2026-11-07

Page: <https://claude.ai/artifact/Ako2hvWjfw2kXvRWe6TWxC> (private to its
owner; verdicts in its database, collection `verdicts`; the key in
`brain/key`). The same code in the app before phase 2 (v128) and today's
(v149), each rendered in Chromium by its own tree's Synth and Engine on the
core, 45 s, full, stereo. Heard, so the key is written here:

| pair | loop | key | heard |
|---|---|---|---|
| 1 | veisk-lain | null (both new) | Same |
| 2 | keil-ruith | A new; chords up an octave (gap to the tune 16.6 to 4.6) | A (new) |
| 3 | veisk-lain | A new; notes unchanged | Same |
| 4 | ke-the | B new; chords up (11.2 to -0.8) | B (new) |
| 5 | fu-glur | B new; chords down (5 to 17) | A (old) |
| 6 | glui-shith | B new; chords down (-2 to 10) | A (old) |
| 7 | thei-bri | B new; chords up (16 to 4) | Same ("they sound different for sure but I like the sound of both") |
| 8 | reisk-foa | A new; chords up (17.1 to 5.1) | A (new) |
| 9 | toask-ha | B new; notes unchanged, the busiest chords (85e) | Same |
| 10 | sung-hal | A new; chords down (5.1 to 17.1) | A (new; "only a slight difference, but I might not") |
| 11 | douth-roa | A new; notes unchanged | Same |

Gap: the melody's mean pitch minus the chords', in semitones. Also on the
page, not blind: the stab chords (broum-hang and no-moun, undertow's own;
toask-ha with `voice.chords=stab`): "Fine" all three. The in-app checklist
for bars and length (88b) was left blank.

## "Full Room Album" (item 85e, hands, 2026-11-01, on v139) -- out

Labeled, not blind: six loops at full quality, each before (`main`, v138)
and after (85e: full quality lets a layer's old notes go as lite does, the
ones already releasing first). Page:
<https://claude.ai/artifact/NmfNUM1hj6buNUHKZzG9of> (private to its owner;
keep / tune / drop and a note per loop, saved in its database, collection
`verdicts`, one document per loop). Rendered by `dlrender`, stereo, 40 s,
the wander on.

- Busy chords: their-roask (hollow), ri-houng (halcyon): chords about 2.5 LU
  up where they now come in.
- Slow: theith-yaing (thaw, 53 bpm; 124 notes let go), dith-shuisk (thaw):
  listen for long tails let go.
- Shatter: tei-har (213 let go), neir-luin asked for 180 bpm.

## "Lite Room Album" (item 85c, hands, 2026-10-31, on v135) -- out

Labeled, not blind: six loops on the lite mix, each before (today's lite,
`main`) and after (85c: the budget lets a layer's oldest notes go, over a
0.05 s fade, instead of refusing the new one). Page:
<https://claude.ai/artifact/XwAWYWFdVi55KdGyccjjEW> (private to its owner;
keep / tune / drop and a note per loop, saved in its database, collection
`verdicts`, one document per loop). Rendered by `dlrender`, stereo, 40 s,
the wander on.

- Busy chords: toask-ha (tide, 96 bpm), hai-waim (glade, 89).
- Slow: theith-yaing (thaw, 53): listen for long tails cut short.
- Shatter: meil-sheir (145), voung-kuing (148), neir-luin asked for
  `bpm=180`; before, lite lost 36-58% of their tunes, after none.

For Mikey's ears: the fade's length, and whether full quality should do
the same (today it refuses as before).

## Session 26 (2026-10-26, on v123): the sound batch -- heard 2026-10-29

**Heard (Mikey, 2026-10-29), every verdict from the page's database:**
- **Keep, everything.** New sounds: frame drum, longChords, glass, dubStab,
  the organ, the wash, both echo times. Mixing ideas: floorDrop, tunedKick,
  subDuck, lateFloor, pedalBass. Flagged: tei-har's glass, the organ after a
  rest, the deep arc, the dip arc ("almost has a bit of a lynchian vibe to
  it; just a bit"), padSweep. Clips: undertow #45's long late floor, thaw
  #71's padSweep, cinder #93's stutter, vapor #4's long drop.
- **floorDrop** (undertow #45): "I didn't actually hear the kick drop out
  unless I'm crazy, but I like the idea for sure." (One bar at 129 bpm, 1.9
  s, under the wash.)
- **lateFloor, with a caveat:** "the bass can come in later too in relation
  to the kit. It doesn't need a set come in time though. I like when things
  are weighted or determined by vibe or somewhat random." Queue item 76d.
- **dust #69's pedal:** "very little difference between with and without."
  Kept as built (his keep on pedalBass; the refused notes weren't heard).
- **The bass level:** +3 dB on dust #7 and noan-nou, today's on dust #42 (no
  kit). Queue item 76c: +3 dB with a kick, today's without.
- **Stereo against mono:** "same" on pairs 1, 2, 3, 5 and 6 and on the null
  (7); pair 4 (thaw #82) "just barely x sounds nicer but about the same
  here", and X was the stereo render there. Key, read back: X stereo in
  pairs 1, 2, 4 and 5, mono in 3 and 6. "Maybe my ears are getting older?"
  More likely the design: the approved baseline keeps the side 21-25 dB
  under the mid on these loops, steady, so the fold sounds like mono by
  construction. Stereo stays the default (it costs nothing he can hear);
  a wider, moving space is the later, experimental option the charter
  already names.

Everything is on one private listening page, "Session 26 Listening"
(https://claude.ai/artifact/4hof83esqPq6qFbWb1NTmK, Mikey's own), where his
verdicts save as he taps (its database, collection `verdicts`, one document per
track; the brain reads them back). Three albums play in the app (codes below);
four sets of clips play on the page, rendered by the brain through the app's
own Engine and Synth on the core in headless Chromium (stereo, wander on, 48
kHz, Opus 160 kb/s), because a code can't carry them:

- **The bass level** (item 56; labeled): dust #7, dust #42 and noan-nou, each
  at today's level, +3 dB and +5.5 dB (`LIFT_DB` in `core/src/blocks/bass.rs`,
  set in a scratch build). The whole loop rises with the bass (dust #7
  -18.5/-17.6/-16.3 LUFS, dust #42 -25.0/-22.6/-20.2, noan-nou
  -24.9/-23.7/-22.1); the page says so. His pick sets `LIFT_DB`.
- **Stereo against mono** (blind, 30 s a pair): glade #57, vapor #30 from 0:05,
  haven #56, thaw #82, grove #31, wayfare #23; pair 7 is tide #52 mono on both
  sides (the null, his test-retest noise). Key, for the brain: in pair *n*
  (1-6), X is the stereo render when the first hex digit of
  SHA-256("S26-AB-*n*") is even. The guards on all six (fold within 0.00 LU,
  side under mid below 120 Hz -28.6 dB or lower, correlation median 0.98-1.00,
  5th percentile 0.93 or more, balance within 0.12 dB): every one passes.
- **Forced, or deep into a loop** (labeled): undertow #45 with lateFloor forced
  (kit 0:30, bass 0:34: his ask); dust #69 0:50-1:20 without and with pedalBass
  forced (60 -> 66 refused); thaw #71 with padSweep forced; cinder #93
  1:48-2:10 (the stutter at 1:55); vapor #4 1:00-1:50 (the long drop at 1:08.6).

Corpus loops are "profile #n" of corpus seed 20261009 (the ears' and the Done
lines'); tei-har and noan-nou are the baseline's (corpus seed 1). Each track's
length is set in its code (`playFor`), so the album moves on by itself.

### S26 New sounds

- frame drum: tide #52, 4 x 7.8 s = 31 s; draws: none
- longChords: grove #47, 2 x 18.3 s = 37 s; draws: floorDrop:shapes
- glass: thaw #58, 5 x 8.9 s = 44 s; draws: glass
- dubStab: bloom #37, 5 x 6.4 s = 32 s; draws: dubStab
- chapel organ: haven #56, 3 x 15.5 s = 46 s; draws: organ
- wash: vapor #30, 1 x 71.1 s = 71 s; draws: wash
- echoTime: half beat: thaw #10, 1 x 57.3 s = 57 s; draws: none
- echoTime: one beat: haven #38, 2 x 24.9 s = 50 s; draws: floorDrop:shapes

```
FAIyBnRvonASA4-191h19_iBdSBwn81xn39l9LfSC6tunr64pr1EfWH2BiV2Y-CSAi0gprzjzo0LfSBo6yuwo0zj5gChBSCqhqowkpuu9kBhBWD0BRY8itASAy4jmwh66rlbhBSBwn81xn39l9LfSC-wnkwjt875ShBWJyBH2FBzBSAiv4nr_6-9hGfSBor6uujp-x_IfSC0786krn0g5FfWJ1B1MHzgASAor6ygshw_yzChBSBk-w7k1r-lyIfSCuuzjw5oy2gQhBWFvBIN6ytBSA8hv0hx9-x0FfSB0glktkz9gtBdSCqwigoxo1kLfyBJgm7vBSA6tunr64pr1EfSBuuzjw5oy2gQfSC63-1g-2i6whDjB0BMVVlyCSAq1q15u7olvJfSBuuz7sm0zq9IfSCgloxgz2mi0sCjBWDWBeOUzI2IE5ldyBzb3VuZHMeo
```

### S26 Mixing ideas

- floorDrop: undertow #45, 2 x 29.8 s = 60 s; draws: wash
- tunedKick (long boom): dust #32, 2 x 21.1 s = 42 s; draws: tunedKick:long
- subDuck: grove #31, 2 x 16.6 s = 33 s; draws: wash subDuck
- lateFloor: undertow #6, 2 x 17.8 s = 36 s; draws: lateFloor
- pedalBass: undertow #8, 2 x 15.9 s = 32 s; draws: pedalBass

```
FAFwBXpKi6ASA078u20xz2qJfSB-r_iwvkoxnNfSCohq0xus0o-EfwBQ8_-nDSA-r_iwvkoxnNfSBiv478ol-zzJfSCor6uujp-x_IfyBLjgyGBSA25ti42ozw5KfSBoyx8s16yijNfSC8hvogn643zlBhBwBtesCJDSAmyxo7rwzsxJfSBi0gprzjzo0LfSCiv4z51w_nwCfmBEbirVBSA63-154_yqoMfSBk-wv2-197jMfSCEBWDeQUzI2IE1peGluZyBpZGVhcwMJ
```

### S26 Flagged

- tei-har's glass: tei-har, 2 x 30.0 s = 60 s; draws: glass wash lateFloor
- organ after a rest: clockwork #27, 1 x 36.2 s = 36 s; draws: organ
- arc: deep: bloom #79, 17 x 12.1 s = 206 s; draws: arc
- arc: dip: dust #40, 18 x 5.9 s = 107 s; draws: arc
- padSweep: dust #77, 7 x 6.8 s = 47 s; draws: pedalBass padSweep

```
FAF0BEeJfLBSAo6yuwo0zj5gChBSBk5ou2qtpxxGfSC83-t2lr0-kFfWDwBKky18BSA2lt1hgk-p6JfSB-1v1pxszulKfSCglohg0o0qyFf1BDuHdOASA2ltxw1nxrqtChBSBmtpv-q89tiLfSCwi00xzup7qGfWhBzBteKB5CSAuz78s68n1vOfSB0glyujik1hBdSCwi0oj94ox8JfWjB2B3dR2bCSAw4ji4xm-9sJfSBkloxgz2mi0sChBSCokjxwwl849rChBWNWBeLUzI2IEZsYWdnZWQgw
```

## "Phase 1 check" (2026-10-18, blind A/B, v92 against v99) -- heard 2026-10-18: "everything sounds the same on both versions"

Core phase 1 (items 42-45) was meant to change nothing a listener hears:
the core's own math (sound moved under -225 dB), the random draws moved
into the core (different draws from the same ranges: a noise offset, a
bell's detune, a singer's scoop), any rate and block size (nothing at 48
kHz). Nine loops, one pass each (so "Let the loop wander" never comes
in), chosen from the corpus to lean on every kind of draw that moved: 1
pan flute over drums and wind; 2 whistle with the drop texture; 3 temple
bells over an analog lead; 4 fiddle and accordion over the hand kit; 5
hum and tubular bells; 6 soft pads; 7 ocarina and prepared piano over
drums; 8 flute over drums; 9 choir with waves. About 3.5 minutes a link.

Played on two commit previews, the same album pasted on each (each link
keeps its own Library): **X** https://6e1cdb44-driftloom.nowmichaelclark.workers.dev
and **Y** https://3769be13-driftloom.nowmichaelclark.workers.dev. One is
v92 (before phase 1), the other v99 (today's main). Per track, one word:
**same**, or **X** / **Y** for the one that sounds nicer, or
**different** if they differ and neither is nicer; and any click or
dropout. The screens differ a little between the links (Painter 6); best
ignored. Diagnostics shows the build: look only after rating.

Key, for the brain: X is v92 when the first hex digit of the SHA-256 of
the album code below is even, v99 when odd. One track is a null, the
same audio on both links to the bit (its seed is 12-vapor's in
`core/golden/codes.txt`): an answer other than "same" there is the
test-retest noise.

```
FAJwBTTjPoASA6omigjvzi4JfSB20lp7109xqMfSC0gloz1lp15HfyB0AhWsCSA-1v1pxszulKfSB0glyujik1hBdSC8m3pg7itimrBhBmBS8a9mDSAs_6rnj1u9tBdSB-1v1pxszulKfSCIB0BtoMQABSAm35tg8--0p9BhBSB6y205k3-_1GfSCqmypw4xi5vqBhBxBIgCmtBSA2v97sr2o9pKfSBo351p2uohyLfSCiqwigoxo1kLhB0B7hArLCSA0glogi4jlnkBhBSB2v9jw-qnptRfSC63-1g-2i6whDjB0BedVrLBSA2turw2-gp7yBhBSB078u20xz2qJfSCwxs0gls58x6CjBoBCT8JoDSAs_6nzwj0itGfSB4m3xw6pm-2uChBSCEBxBLBBHVASAs6yu2vv-j-HfSBqwiymy5uwWdSC0q1qgqps6rvBhBWBeNUGhhc2UgMSBjaGVjaw_l
```

## "Heartwarming?" and "Joyful?" (2026-10-11, on v85 or later) -- the first word albums; heard 2026-10-11

Mikey's idea for the words (`docs/MOODS.md`, "A better test: word
albums"). Eight loops each, about 40 seconds a track. The brain picked
most of them as its guess at the word and mixed in a couple it expects
not to fit. **For each track, say: fits, doesn't fit, or not sure.** If a
track fits a different word better, name it if one comes to mind
(optional). Both words almost never fit in the ranking test, so "none of
them" is a useful answer too.

**Heard (Mikey, 2026-10-11).** Heartwarming: 1 not really; 2 yes,
definitely; 3 no; 4 yeah, mostly; 5 yes and no; 6 yes; 7 yes; 8 "it almost
starts to feel like it will be heartwarming, but it's really not."
Joyful: 1 yeah, kind of; 2 a bit, but not really; 3 no; 4 yeah, kind of;
5 yes; 6 not really, "a hint of joy though"; 7 no; 8 unanswered. What the
brain took from them is in `docs/VIBES.md` (the first recipes).

Heartwarming?

```
FAI1BMINAAASA2v9v-0goz7NfSB4tuzg2l6ks2ChBSC6y2kgrg838UhBWDzBWyMAAASA0gl8k_vorrLfSB2turw2-gp7yBhBSCqr6igtz9nxMhB3B8DNAAASA8hvogn643zlBhBSB4tuzg2l6ks2ChBSCq1qlg1km91XhBWNyBnsMAAASA-m3pzuwyy4OfSBo8hvm3i-_gKfSCokjxwwl849rChByBtxMAAASA-r_iwvkoxnNfSB6y2oruh-1nKfSC6tunr64pr1EfWD0BnENAAASA0gl8k_vorrLfSBukjpzksot_LfSC0glogi4jlnkBhBWH2BQINAAASAk-wng4z5_thBhBSB2skzgxjly_0ChBSCyi0owprjhqmBhBwBYzMAAASAyskvuorzksKfSBo8hvm3i-_gKfSCk5ou2qtpxxGfWFeNSGVhcnR3YXJtaW5nPw5h
```

Joyful?

```
FAIzBGFNAAASAglohg0o0qyFfSB6y2oruh-1nKfSCuuzjw5oy2gQhBWJ1BTuMAAASAmwiswgjru01BhBSBiv4r_xs54iBdSCy78mguvv60ehBWJyBc8MAAASAy20hoqz-hqHfSBiv4vuyv9plNfSCysknr1204oDfWH0B_ENAAASA0i0wgpy886pChBSB0gloz1lp15HfSC8hv0hx9-x0FfWJ1ByDNAAASA2ltxw1nxrqtChBSBkloxgz2mi0sChBSC6tujg33ntqPhByB8JNAAASAyxsorp_oj7IfSB4m3xw6pm-2uChBSC6tunr64pr1EfwBUJNAAASA25ti42ozw5KfSB8m31hlmz8mLfSC8hv0hx9-x0FfzBHDNAAASAq1qholxpv9FfSB68mv25zop3KfSCi0glgwixqpWhBWFWDeHSm95ZnVsPwVi
```

## "Moods" (2026-09-28, on v64 or later) -- retired 2026-09-28

Retired before it was heard: the word-ranking test (queue item 19) asks
the same things faster. Kept for the record.

Eight short loops (8 bars or fewer), each led at 80%+ by one mood: two
joyful, two happy, two peaceful, two reflective, shuffled. Key: each
spec's leading `feelMix` entry. Changed 2026-09-28: describing each
track freely felt too abstract, so the test is now **most and least**.
Listen to all eight, then for each word give two track numbers, the
track that is most like it and the one least like it ("none" is fine):
happy, lively, warm, crisp, floating, bouncy, twinkling, intimate,
quirky. Uses: whether the mood steering (item 16) makes happy sound happy,
and Mikey's picks become the first data for the word regions (roadmap 19,
maps and clouds in `docs/MOODS.md`).

```
FAIzBX6YoDCSA43-tw-vp-_9BhBSBuprqz4083xRfSCi73mwr-kx-dhB0BSscZPBSA2-1vg6r9k1lChBSBglopjn9y21MfSC0xsswllghh3BhB0B_f-JKASA0ltpg2g4v5pBhBSB-r_iwvkoxnNfSCgloxgz2mi0sCjBxBySNUmDSAu4j2p7w9z-MfSBmyxo7rwzsxJfSCq8hnwwg6jrfhBwBDIbsBCSAglohg0o0qyFfSB6tu78jjphnIfSCqwi056y068DfxBNVTZPASAysknr1204oDfSBy20pr9n9ttOfSCi2vlw31wmsYhB0BgA9CzBSA2skzgxjly_0ChBSBohqwgkwnquoChBSC4-191h19_iBdzB_n-5SBSAquzzw42kui3ChBSBqwiymy5uwWdSC2-1vg6r9k1lChBeFTW9vZHMHS
```

## "Rust check" (2026-10-05, on v68 or later) -- heard 2026-10-07: no difference (Firefox, desktop)

Six loops that lean on the ported voices: kalimba (tracks 1 and 4),
fiddle (2 and 5), pad (3 and 6). Played twice, once on the main site and
once with `?engine=rust`. Mikey listens for any difference, clicks or
dropouts, and checks Diagnostics reads `engine: rust  late: 0` on the
Rust pass. Not blind: the URL and Diagnostics show the engine (see
State).

```
FAGyB1gAAAASAg73i47qojmMfSBmtp7shy-3wHfSCilopwzvtmjpBhBzB8QAAAASA4m3xw6pm-2uChBSB0gloz1lp15HfSC60llwyz7z_WhBxBzTAAAASAi0g15p5zyiIfSBiv478ol-zzJfSCi73mwr-kx-dhByBhXAAAASA6y205k3-_1GfSByskjgy1y69NfSCgloxgz2mi0sCjBzBDzAAAASA2-1vg6r9k1lChBSBk5ou2qtpxxGfSC60llwyz7z_WhB0B6wAAAASAk-wng4z5_thBhBSBy78io-7ys8MfSCor6ygshw_yzCjBeKUnVzdCBjaGVjawnI
```

## "Rust, all voices" (2026-10-09, on v77 or later) -- heard 2026-10-11: "it sounded great!"

Every voice now plays from the Rust core behind `?engine=rust` (queue
items 23-28). Ten short loops, one per family of newly ported sounds:
1 hum; 2 choir (vowel, three singers); 3 keys over a full kit; 4 moog
lead; 5 pan flute; 6 fiddle with strummed nylon; 7 music box and harp;
8 prepared piano and kalimba; 9 accordion over the hand kit (frame, tap,
jingle); 10 the waves texture. Bass and drums are spread across them.
Played twice: once on the main site, once with `?engine=rust`. Listen
for any difference, clicks or dropouts; on the Rust pass Diagnostics
should read `engine: rust  late: 0  fallback: 0`. **Since v81 (items
29-31) the Rust pass also runs the whole mix in the core** -- reverb,
echo, wobble, saturator, both compressors -- so this one album now
checks everything; no second album is needed. Labeled, not blind:
the URL and Diagnostics show the engine. Best heard on the phone.

```
FAKyBazFAAASAyxs05_0ptpFfSB6tuvutto34LfSC04jughh0n2-BhB0BJN_DAASAquzzw42kui3ChBSBs6yio5595vLfSCs_6ng91uy6iBhBwBr3DAAASAyxs88ypo5sMfSBi5oio03onjKfSCqr6685--7tFf0BwIZAAASA2-1vg6r9k1lChBSBsuzv-v-ygvMfSCqmypw4xi5vqBhBwBaiBCAASAu4j2p7w9z-MfSB0786krn0g5FfSC2v9zp4hqxmDfxB4ZFCAASAqwiork9zwuHfSB2v91zxq3-WdSCk-wng4z5_thBhBzB0ZRAAASAokjxwwl849rChBSB6j-80rl956OfSCy4jmwh66rlbhBwBOAdAAASA2v97sr2o9pKfSB2lt1hgk-p6JfSCiv4nr_6-9hGfyB-77AAASAkohiwg-o5hJfSB8m31hlmz8mLfSCi-w3g3hs3voDjByBnmLAAASAor6uujp-x_IfSBohq80hhz0hMfSC2skzgxjly_0ChBeQUnVzdCwgYWxsIHZvaWNlcw1H
```

## "Depth two" (2026-09-28, on v64 or later) -- heard 2026-10-11

| track | key (`developmentOf`) | heard |
|---|---|---|
| 1 | simple | goes somewhere |
| 2 | developing (A4 V4 B4 R4) | goes somewhere |
| 3 | simple | loops ("not a bad thing, nice to have long loops that sound like they're looping sometimes") |
| 4 | developing | goes somewhere |
| 5 | developing | goes somewhere |
| 6 | developing | goes somewhere |

The 16-bar form works: every developing loop went somewhere (the first
Depth album: one of three). Key read back from the code on 2026-10-11.


Six 16-bar loops, four developing and two simple, shuffled. Key:
`developmentOf(spec)`. Rated "goes somewhere", "loops" or "too busy", as
the first depth album was. Tests item 17's new 16-bar form (brain's
measure: 8.8 distinct melody bars developing, 6.7 simple).

```
FAG1BbmJDgCSAquzzw42kui3ChBSB04jughh0n2-BhBSCq1qlg1km91XhByBQJ9yFASA63-154_yqoMfSBk5o259ho90NfSC4tuzg2l6ks2ChBwBcAC1GBSAqr6igtz9nxMfSB63-154_yqoMfSC2v9n7hspn4GfzBcDPwSASAohqwgkwnquoChBSByxswu8znv-PfSCiqwigoxo1kLhByBApuRnBSA43-tw-vp-_9BhBSBuuz7sm0zq9IfSCqr6685--7tFf0BMPQMGDSAohqwgkwnquoChBSB0ltpzpu9_rNfSC6-1nw6kkpkiBhBeJRGVwdGggdHdvdW
```
