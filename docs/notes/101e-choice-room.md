# 101e: the choices' room is full

Hands session 06, 2026-11-15, for session 07 (and the brain).

colorTone (#396) is the sixteenth per-loop choice. Two fixed sizes hold
exactly sixteen:

- `CHOICES_MAX = 16` (`core/src/generator/render.rs`): the list of a loop's
  options. A seventeenth choice would be dropped from the list (`lost()`, which
  the tests hold at 0).
- `CHOICE_PINS = 16` (`core/src/generator/request.rs` and `js/pins.js`): the
  request's pin numbers 0 to 15 are the choices, by their order in
  `data/choices.txt`; the settings' pins start at 16 (`SETTING_PINS`), so a
  seventeenth choice would take the number of `arc.shape`.

So a batch 6 candidate built as a new choice from here on (planingSevenths,
lowBell, dropSpotlight, twang) needs the room made first. Raising
`CHOICES_MAX` costs nothing outside the player's memory. Moving `SETTING_PINS`
renumbers every setting's and the space's pins, which a share code or a
written request that pins a setting may carry: that is the codes' question
(queue item 105, session 22's codes version 2). The conservative paths, for
the session that needs one:

1. Build the candidate as an option of an existing choice where it is one
   (Mikey's rule allows it: today's behavior stays the first option).
2. Or raise `CHOICES_MAX` and give the new choices pin numbers past the space's
   (a second range for choices), so no existing number moves.

Nothing here was changed in #396.
