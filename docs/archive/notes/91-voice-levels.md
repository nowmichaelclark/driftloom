# 91: voices pinned onto profiles that never draw them, and their levels

Item 91 (voices and layers, #321) passes every check it names. This note is
the one thing the brief asked to be said: **which combinations needed care.**
Nothing is blocked; it is for the brain to read, and to decide the one
question at the end.

## What was measured

`node tools/measure.mjs --voice all` (K-weighted, each voice against its
layer's median, 0.4 and 0.8 velocity, two octaves) and the corpus report with
a voice pinned (`--request voice.melody=piano`, 40 loops, `per-layer dry
level`), against the catalog's:

| pinned | the layer's LUFS over 40 loops | catalog's |
|---|---|---|
| melody piano | -32.2 | -33.3 |
| melody harp | -31.7 | -33.3 |
| melody moog | -34.3 | -33.3 |
| melody panflute | -34.2 | -33.3 |
| melody saw or analoglead | -33.3 | -33.3 |
| chords piano | -36.6 | -36.7 |
| chords moogpad or analogpad | -36.4 | -36.7 |
| chords stab | **-47.1** | -36.7 |
| bass pluckbass | -27.3 | -29.0 |
| bass moogbass | -27.7 | -29.0 |
| bass fifths | -29.2 | -29.0 |
| texture waves | -42.1 | -44.1 |
| drums kit hand | -22.0 | -22.1 |

No layer is louder than its median allows by more than the catalog's own
voices are (the one loud voice is the texture's swell, +4.0 LU, the same as
when a profile draws it). Where a pinned voice is quiet it is quiet by the
amount it always is.

## The voices past 3 LU from their layer's median

These are the voices `--voice all` already flags, and the profiles that draw
them (undertow's stab, for one) are levelled around them. Pinned onto another
profile they play at that same measured level, which is the brief ("plays at
its measured level"), but they sit well under (or over) the rest of the loop:

- melody: stab -11.7, tubular -5.3, templebell -3.0 (hum -7.7 is the choir's
  and is not offered)
- chords: stab -10.9, nylon -4.9, tubular -4.7 (hum is the choir's)
- texture: swell +4.0, drops -28.2, wind -8.9

`chords stab` is the starkest: the keys layer lands at -47 LUFS, 10 LU under
the catalog's mean, so that loop's chords are nearly inaudible under the
melody (+13.9 LU). undertow's own loops with the stab are as quiet.

## Decision for the brain

Keep them in the offered lists as they are (what was built), or leave the
quiet ones out of `VOICE_ORDER` until a Painter or ears session decides
(append-only: leaving one out now and adding it later costs nothing, the
other way round moves share codes), or level each for pinned use (a core
change, an item of its own). The conservative choice, for Mikey's ears, is
the first: they are the catalog's own voices at the catalog's own levels, and
the picker the design does not have yet may well hide the extremes.
