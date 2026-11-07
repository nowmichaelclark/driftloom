# 93: the player's module, at 637 KB and not 300 KB

For the brain. Item 93's aim was a `js/dlplayer.wasm` within 50 KB of
`js/dlcore.wasm`'s 252 KB. The module is 637 KB now (it was 1,938,596
bytes). Making `Player::new()` zeros did what it could; the aim cannot be
met without a decision.

## What was done

`Player::new()` is all zero bytes (test: `a_new_player_is_all_zeros`), so the
1.5 MB of the player is `.bss`, not stored. What took it there:

- `List` (the generator's fixed lists) holds `MaybeUninit` slots, zero past
  the length; a filler note with a name and a pointer was most of the bytes.
  Nothing reads a slot past the length, so nothing can see the change.
- `Maybe<T>` for the `Option`s whose `None` Rust wrote as a spare value (the
  pending changes, the budget's holds, the kick fit's walks, the wander's
  ghost and floor drop); `Word` for the five `&'static str` fields that
  started as `""`; `Biased` for the numbers that start at a default (the
  wander, the volume, the echo, and the like), stored XOR their default so
  every value reads back exactly; `Length` starts at 1 and `Handover` at
  `RingOut`, so their `None` and default are zero; `Cycles` replaces an
  `Option<[Option<f64>; 5]>`. `Request`'s weights start at 0, read only
  where the vibe is asked.

## What the rest of the module is

| Part | Bytes |
| --- | ---: |
| code (the core's blocks and mix: about 245 KB, as `dlcore.wasm`) | 547,676 |
| data (the generator's tables, names, vibes, profiles) | 86,630 |
| the rest | about 2,400 |

The generator and the player are about 300 KB of code. The largest: the
sequencer's `schedule_step` 31 KB, `character::blend` 21 KB, `new_spec` 20 KB,
`gen_harmony` 19 KB, `Render::part` 17 KB, `gen_drums` 13 KB, `gen_melody`
10 KB, the synth's `voice` 10 KB. About 37 KB is the standard library's: float
printing for the canonical text (`format_shortest`, `format_exact`, 11 KB),
an unstable sort (5 KB), panic and format machinery.

Size by compiler setting (scratch builds, not committed), raw and gzipped:
`opt-level` 3 (today) 637 KB and 215 KB, 2 is 593 KB and 205 KB, `s` 480 KB
and 174 KB, `z` 450 KB and 166 KB. None reaches 300 KB raw, and `s` would
slow the voices and the mix (the costs rule: 5%), for 41 KB gzipped.

## On the wire

Zeros compress to nothing, so gzip hides most of this: the module was 222 KB
gzipped before and is 215 KB now (the app's core: 91 KB). The saving is in
the file's size on disk and in the browser's cache, not in a first visit's
transfer. What a first visit pays is the 215 KB for the player's module.

## One module for both

Yes. `dlplayer.wasm` exports everything `dlcore.wasm` does (all 34 calls the
worklet makes) and 33 more; the worklet would load it and call the same
names. Its memory is 16.8 MB against 15.5 MB (257 pages against 237). So the
page loads one module of 215 KB gzipped, not 91 KB for the core and later
215 KB for the player: item 94b's switch, and `js/dlcore.wasm` goes when the
JavaScript synth does (item 97). This needs no change to the core.

## For a decision

Is 215 KB gzipped for the app's first visit acceptable? If it is not: (a) the
player module at `opt-level` `s` (480 KB, 174 KB gzipped), with the voices
and the mix measured for what it costs; (b) leave the canonical text (the
generator's JSON for the goldens, `canon::number` and the standard library's
float printing it pulls in, 11 KB of the 37 KB above) out of the app's build
with a feature, since only the golden tools read it (not measured past
that). Neither reaches 300 KB. I took neither; both are for the brain.
