#!/bin/sh
# Loops from their seeds on one target (queue item 82): the corpus's first 80
# loops (corpus seed 1, as tools/stats.mjs and tools/measure.mjs draw it)
# played by the core's player, each to a WAV, and their hashes compared with
# the ones kept in core/golden/seeds.hashes: every target must make them all.
#
#   tools/seeds.sh [TARGET]             render, compare with core/golden/seeds.hashes
#   tools/seeds.sh --update             render on this machine, and rewrite it
#   tools/seeds.sh --first N [TARGET]   only the first N loops (QEMU's targets)
#   tools/seeds.sh --wav DIR [TARGET]   keep the WAVs in DIR (else a temporary one)
#   tools/seeds.sh --browser [TARGET]   the baseline's 40 loops, full and lite, mono
#                                       and stereo (160 renders), against the
#                                       browser's: test/core-baseline.json's `player`
#                                       part, which tools/baseline.mjs renders in
#                                       Chromium with the app's own engine
#   tools/seeds.sh --c [--first N]      the C host (core/examples/c/seed.c, built
#                                       against core/include/dlcore.h alone) against
#                                       the same hashes, and its WAV against
#                                       dlrender's, byte for byte
#
# Each loop plays 45 s at 44.1 kHz, full quality, mono, wandering as the app's
# does (amount 1): dlrender --corpus 1:80, its lines kept as it prints them
# (the loop's name, how it played, FNV-1a 32 over the samples' words and
# FNV-1a 64 over their bytes, which are the WAV's). TARGET is a Rust target
# triple, run directly (x86_64) or under QEMU's user mode as
# core/.cargo/config.toml says, or wasm32-unknown-unknown: js/dlplayer.wasm in
# Node (tools/player-wasm.mjs), the module the app plays with ?engine=player,
# which prints the same lines but writes no WAVs. It defaults to this
# machine's.

set -eu

root=$(cd "$(dirname "$0")/.." && pwd)
kept="$root/core/golden/seeds.hashes"
wasm=wasm32-unknown-unknown
update=no
first=80
wav=''
browser=no
chost=no
target=''
while [ $# -gt 0 ]; do
  case "$1" in
    --update) update=yes ;;
    --first) shift; first=${1:?--first needs a number} ;;
    --wav) shift; wav=${1:?--wav needs a directory} ;;
    --browser) browser=yes ;;
    --c) chost=yes ;;
    -h|--help) sed -n '2,/^$/p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
    -*) echo "seeds: no such option: $1" >&2; exit 2 ;;
    *) target=$1 ;;
  esac
  shift
done
[ -n "$target" ] || target=$(rustc -vV | sed -n 's/^host: //p')

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

# dlrender's lines for `$@` on the target.
render() {
  if [ "$target" = "$wasm" ]; then
    (cd "$root" && node tools/player-wasm.mjs "$@")
  else
    (cd "$root/core" && cargo run -q --release --locked --example dlrender --target "$target" -- "$@")
  fi
}

if [ "$browser" = yes ]; then
  # The four ways, against the browser's hashes: every render the same, to
  # the bit, or the loop and the way that differ.
  : > "$tmp/lines"
  for way in '' '--stereo' '--lite' '--lite --stereo'; do
    echo "seeds: the baseline's 40 loops on $target ${way:-(full, mono)}" >&2
    # shellcheck disable=SC2086 # the way's words, split on purpose
    render --corpus 1:40 $way >> "$tmp/lines"
  done
  node --input-type=module -e '
    import fs from "node:fs";
    const kept = JSON.parse(fs.readFileSync(process.argv[1], "utf8")).player;
    if (!kept) { console.error("seeds: test/core-baseline.json has no player part; run  node tools/baseline.mjs --update --only player"); process.exit(1); }
    const bySeed = new Map();
    for (const q of ["full", "lite"]) for (const [name, e] of Object.entries(kept[q])) bySeed.set(`${e.seed} ${q}`, { name, ...e });
    let same = 0; const bad = [];
    for (const line of fs.readFileSync(process.argv[2], "utf8").split("\n").filter(Boolean)) {
      const f = Object.fromEntries(line.split(" ").filter((w) => w.includes("=")).map((w) => w.split("=")));
      const e = bySeed.get(`${f.seed} ${f.quality}`);
      const want = !e ? null : f.layout === "stereo" ? `left=${e.left} right=${e.right}` : `out=${e.out}`;
      const got = f.layout === "stereo" ? `left=${f.left} right=${f.right}` : `out=${f.out}`;
      if (want === got) same++; else bad.push(`  ${f.name} ${f.quality} ${f.layout}: ${got}, the browser ${want || "(none)"}`);
    }
    console.log(`seeds: ${process.argv[3]}: ${same} of ${same + bad.length} renders equal the browser'"'"'s, bit for bit`);
    if (bad.length) { console.log(bad.join("\n")); process.exit(1); }
  ' "$root/test/core-baseline.json" "$tmp/lines" "$target"
  exit 0
fi

if [ "$chost" = yes ]; then
  # The C host, built against the header alone: its line for each loop is
  # dlrender's, and its WAV is dlrender's, byte for byte.
  cd "$root/core"
  cargo rustc -q --release --locked --lib --features capi --crate-type staticlib
  cc -O2 -std=c11 -Wall -Wextra -Werror -pedantic -Iinclude examples/c/seed.c target/release/libdlcore.a -lm -o "$tmp/seed"
  grep -v '^#' "$kept" | head -n "$first" > "$tmp/kept"
  : > "$tmp/lines"
  while read -r index line; do
    s=$(printf '%s\n' "$line" | sed 's/^seed=\([0-9]*\) .*/\1/')
    printf '%s %s\n' "$index" "$("$tmp/seed" "$s")" >> "$tmp/lines"
  done < "$tmp/kept"
  if ! cmp -s "$tmp/lines" "$tmp/kept"; then
    echo "seeds: the C host's renders differ from the kept hashes:" >&2
    diff "$tmp/kept" "$tmp/lines" | sed 's/^/  /' >&2
    exit 1
  fi
  s=$(head -n 1 "$tmp/kept" | sed 's/^[0-9]* seed=\([0-9]*\) .*/\1/')
  "$tmp/seed" "$s" --stereo --seconds 10 --wav "$tmp/c.wav" > /dev/null
  cargo run -q --release --locked --example dlrender -- --seed "$s" --stereo --seconds 10 --wav "$tmp/rust.wav" > /dev/null
  cmp "$tmp/c.wav" "$tmp/rust.wav"
  echo "seeds: the C host, through dlcore.h alone: all $(wc -l < "$tmp/kept" | tr -d ' ') loops equal their hashes, and its WAV is dlrender's"
  exit 0
fi

echo "seeds: the corpus's first $first loops from their seeds on $target" >&2
if [ "$target" = "$wasm" ]; then
  render --corpus "1:$first" > "$tmp/lines"
else
  dir=${wav:-$tmp/wav}
  render --corpus "1:$first" --wav "$dir" > "$tmp/lines"
  echo "seeds: wrote $(ls "$dir"/*.wav | wc -l | tr -d ' ') WAVs into ${wav:-a temporary directory}" >&2
fi

if [ "$update" = yes ]; then
  [ "$first" = 80 ] || { echo "seeds: --update keeps all 80; drop --first" >&2; exit 2; }
  {
    echo "# The corpus's first 80 loops (corpus seed 1) played from their seeds by the core's"
    echo "# player (tools/seeds.sh --update): 45 s at 44.1 kHz, full, mono, wander 1. One line"
    echo "# a loop, as dlrender --corpus 1:80 prints it. The same on every target."
    cat "$tmp/lines"
  } > "$kept"
  echo "seeds: wrote core/golden/seeds.hashes ($(wc -l < "$tmp/lines" | tr -d ' ') loops)" >&2
  exit 0
fi

grep -v '^#' "$kept" | head -n "$first" > "$tmp/kept"
if cmp -s "$tmp/lines" "$tmp/kept"; then
  echo "seeds: $target: all $first loops equal their hashes"
  exit 0
fi
echo "seeds: $target: the renders moved"
diff "$tmp/kept" "$tmp/lines" | sed 's/^/  /'
echo "  If the sound moved on purpose, run  tools/seeds.sh --update  and say so in the PR." >&2
exit 1
