# Item 83: the sketch voice is not built

**From:** the hands (Sonnet 5.5), 2026-10-31. **Needs:** a decision.

Item 83 names three places a sketch can play: a channel's own block, a voice
for a layer, a send. A channel's own block and a send are built (a send is a
channel block with `Channel::SENDS`; the sketch file chooses). A **voice for a
layer** is not.

**Why:** a voice is played by number from the registry's `voices` list
(`blocks/mod.rs`), whose lines take no `#[cfg]`: the number, the slot enum
(`Held`), the pool and the dispatch `match`es are all written from the list.
Letting one line carry `#[cfg(feature = "sketch")]` means threading an
attribute through about ten expansions of the macro, in the file every block
goes through. That is a change to the core's registry the item did not ask
for, and a slip there moves the app's `.wasm` (which the item's yardstick
holds byte for byte).

**What works meanwhile:** a sketch instrument as a channel source on the
layer's channel (it follows the layer's events, as the organ and the wash do),
heard in `tools/clips.mjs` with `"sketch": true`.

**Options:** (1) leave it: sketches of a new instrument stay channel sources
until a sketch needs a voice's note-by-note play; (2) queue a small item to
give the `voices` list the attribute, with `tools/build-core.sh --check` as
its yardstick. I suggest (1) until an ears session asks.
