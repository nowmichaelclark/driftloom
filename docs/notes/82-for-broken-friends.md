# 82: for Broken Friends: a loop from a seed, in the core

For the game's sessions (`rust/driftloom-godot/` in the game's repo), from
the hands' item 82 (PR #296). The game pins the core at f654fdb; this is what
a repin brings and what to refresh.

## What is new: the player

The game can now play a friend's theme from a seed, with no JavaScript and
no note lists: `dlcore::player::Player` makes the loop with the core's own
generator and plays it with the app's sequencer, wandering pass after pass as
the app does. The same seed and vibes make the same samples on every target,
bit for bit, and the same as the app's own engine in Chromium.

The calls the game uses, in Rust (the extension links the core directly):

```rust
use dlcore::Core;
use dlcore::generator::Layer;
use dlcore::player::{Player, Request};

// A core is 17.4 MB and a player 1.5 MB (64-bit): keep both off the stack.
// Box::new(Core::new()) builds on the stack first in a debug build, so make
// them on a thread with room, or keep them in a static, as the game keeps
// its core today.
let mut core = Box::new(Core::new());
let mut player = Box::new(Player::new());
player.start(&mut core, 48_000.0, 2, false)?;  // rate, channels (2: stereo), the full mix (false: lite's, the game's on phones)

// A friend's theme: its seed, and its vibes by name; since item 85 with
// weights (the friend's mood, the time of day) and a tempo, and since 85b
// with pins ("organ=on", "arc.period=240": the named settings), as text or
// built call by call. A refusal is a short plain line, never a panic.
let request = Request::parse("peaceful=0.7,restful=0.3,bpm=70-80").unwrap_or_default();
player.load(friend_seed, &request);            // made over the next blocks, then it starts

// The audio thread, every callback, any number of frames:
player.render(&mut core, &mut [&mut left, &mut right])?;

// The app's controls:
player.set_wander(1.0);                        // 0 plays the loop as composed; the app's default is 1
player.set_mute(&mut *core, Layer::Drums, true);
player.set_tempo(&mut *core, 72.0);
player.set_volume(&mut *core, 0.85);
player.stop();                                 // pause; player.resume(0.0) goes on
```

- **A change of loop** is another `load`: the new loop is made a stage a block
  (a few blocks, up to about a hundred), the old one's notes play out, and the
  new one starts. No core reset, so the second core the game keeps for a
  switch is no longer needed for that; the crossfade at a musical moment is
  item 86.
- **The browser**, with no audio thread: `player.ready(&mut *core)` makes the
  loop at once (about 0.5 ms natively, 10 ms at most, for a loop with a tuned
  kick), then `render` as far ahead as the game wants, in chunks of any
  size. Rendering ahead pass by pass properly is item 87.
- **Costs** (measured on the cloud machine item 82 was built on, 44.1 kHz,
  where a block of 128 frames has 2.9 ms): natively the worst block of the
  player and the core together is 0.45 ms (16%), all of it the core's audio;
  a load's worst stage adds 0.14 ms (5%), and making loops is 0.4% of a
  minute's render. In wasm the worst block while a loop is made is 0.28 ms
  (10%), audio included. On every target a load's worst stage costs less than
  the loop's own worst audio block, so a load never makes the worst block;
  on the Moto G Power the audio itself is what to watch.
- **The C header**, `core/include/dlcore.h`, has the same calls for any other
  host (`dl_player_start`, `dl_player_load`, `dl_player_render`, ...);
  `core/examples/c/seed.c` renders a seed to a WAV through it alone.
- **Checking it in the game:** `core/golden/seeds.hashes` keeps the corpus's
  first 80 loops (45 s at 44.1 kHz, full, mono, wander 1), a line each as
  `dlrender --seed N` prints it; `out=` is FNV-1a 32 over the samples' float
  words, `hash=` FNV-1a 64 over their bytes. The lite and stereo renders of
  the first 40 are in `test/core-baseline.json`, `player` part.

## What changed since f654fdb

The host calls the game makes today keep their signatures (`Core::new`,
`start(rate, channels)`, `key`, `note`, `render(out)`, `event`, `set`,
`route`, `param`). Added since the pin:

- **Stereo** (items 64-66): `start(rate, 2)` renders stereo, `set_layout`
  switches while playing (a 0.25 s fade), `lanes()` and `layout()` say which;
  the space recipes (`SPACE_*` params, `space_range`).
- **Sounds and settings** (items 63b, 67-73, 76c): the arc's shapes
  (`ARC_SHAPE`, `ARC_FLOOR`, `ARC_DIP`), the tuned kick and its choke,
  subDuck's route, padSweep, the echo time, and `bass_lift(db)`.
- **The generator** (items 77-81): `dlcore::generator::Generator`, a loop's
  spec, layers, choices, load and wandered passes from a seed.
- **The player** (item 82): `dlcore::player`, above; `Core::render_with`.

## The hashes to refresh

Every golden note list was rewritten since the pin (they carry the newer
blocks' calls and settings, items 63b-76d), and every one but `13-dust`
sounds different, so its hash moved. The out hashes, the same on every
target (`core/golden/*.hashes`):

| list | at f654fdb | now |
|---|---|---|
| 01-glade | adaef6e905e48db7 | c34e89da744479c1 |
| 02-wayfare | 386142de7464b696 | 612cdc588f2cf869 |
| 03-tide | dbf85772fc109bf1 | 69a18beb1f62572b |
| 04-vapor | 2cb8773358694cb0 | 82f1d8a6d5e691e4 |
| 05-clockwork | 62f1c208c1214043 | 1eb3d695f19f8b7d |
| 06-undertow | 609f03013cfed015 | e7087c7a98282b5c |
| 07-dust | c4048c74a17e8159 | 9a355a9011f9becf |
| 08-bloom | dcfb2895db8290e1 | 24bf79e2056e0250 |
| 09-shrine | 1fda45628401a29e | 044d15ac4712944c |
| 10-shrine | f655fa0fcbe7b33a | e22e7d1bc0984bff |
| 11-haven | 0d6595d49fab8200 | 0f974bb825dac4e1 |
| 12-vapor | 0618a0ceecd72918 | eae99636c94d7cdc |
| 13-dust | e5e147c871f529d7 | e5e147c871f529d7 (the same) |
| 14-bloom | 263af2a2eaf6a921 | 4fcb603b4bea49ce |
| 15-wayfare | 63b5910a39889bd5 | 9c279f88619513dc |
| 16-grove | cd306c904078c6b9 | 037aa059f8be63f7 |
| 17-bloom | 9e3bd6f83fd2e5b9 | c58dea35b1b89eb5 |
| 18-shatter | 578e315f082d0de3 | 59c59ae13ec841d3 |
| 19-glade | d4c212c3e964a22e | 0300abf1bff30607 |
| 20-tide | 3979b20914c93f49 | 51b0706a9e5f7d86 |
| busy-cinder | c3bd822e87f275a4 | f04e065b7ad33d66 |

Copy the lists themselves too (`core/golden/*.dlnotes`, all 21). The stereo
hashes are new: `*.stereo.hashes`, one line a list, each side apart. Item 82 changes none of these: the player is
new code beside the core, and the app's `.wasm` is the same bytes.
