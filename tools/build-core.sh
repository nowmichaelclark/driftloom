#!/bin/sh
# Build the Rust core (core/) to WebAssembly and copy it to js/dlcore.wasm,
# and the player's build of it to js/dlplayer.wasm (queue item 82).
#
#   tools/build-core.sh          build, and replace js/dlcore.wasm and
#                                js/dlplayer.wasm
#   tools/build-core.sh --check  build, and fail if either differs from its
#                                build by a single byte
#   tools/build-core.sh --wide   build the 192 kHz variant, to
#                                core/target/dlcore-wide.wasm (not committed:
#                                the tools that render goldens above 96 kHz
#                                in Node read it)
#
# The built file is committed (roadmap 16): the site stays plain files with
# no build step, and the bytes that were checked are the bytes that ship.
# --check is what proves the committed file is its source. It runs on every
# PR (.github/workflows/core.yml), and it rests on the build being
# reproducible: core/rust-toolchain.toml pins the exact compiler, the
# crate has no dependencies, and the paths below are remapped so the file
# does not depend on where the repository or the toolchain sits.
#
# Needs rustup; the pinned toolchain and its wasm32 target install
# themselves on first use.

set -eu

root=$(cd "$(dirname "$0")/.." && pwd)
check=no
wide=no
case "${1:-}" in
  --check) check=yes ;;
  --wide) wide=yes ;;
  '') ;;
  *) echo "usage: tools/build-core.sh [--check | --wide]" >&2; exit 2 ;;
esac

cd "$root/core"
cargo_home=${CARGO_HOME:-$HOME/.cargo}
rustup_home=${RUSTUP_HOME:-$HOME/.rustup}
RUSTFLAGS="--remap-path-prefix=$root/core=/dlcore --remap-path-prefix=$cargo_home=/cargo --remap-path-prefix=$rustup_home=/rustup"
export RUSTFLAGS
# The library with its WebAssembly exports (the `web` feature), as a cdylib:
# the crate itself is a plain no_std library (core/Cargo.toml says why). It
# is built as an rlib in the same run, as it always was: that keeps its
# functions public, so LLVM does not fold every stage into one giant
# function (and Cargo leaves out LTO, which it does for a crate that is an
# rlib too). Built as a cdylib alone, the same source renders about 15%
# slower in V8 (measured by instruction count on the golden renders).
#
# The web build leaves the `wide` feature off: a browser's context runs at 96
# kHz at most, and the app keeps the memory it has (core/Cargo.toml).
if [ "$wide" = yes ]; then
  cargo rustc --release --locked --target wasm32-unknown-unknown --features web --crate-type cdylib,rlib
  cp "$root/core/target/wasm32-unknown-unknown/release/dlcore.wasm" "$root/core/target/dlcore-wide.wasm"
  echo "build-core: wrote core/target/dlcore-wide.wasm ($(wc -c < "$root/core/target/dlcore-wide.wasm" | tr -d ' ') bytes)"
  exit 0
fi
# Two modules: the app's (the `web` feature), and the player's (queue item
# 82: the `player` feature, the generator and the player with their exports),
# which the app plays only with ?engine=player, for testing, until phase 3.
# Each is built and kept beside the other; the app's stays the same bytes
# whatever the player does.
status=0
for module in dlcore:web dlplayer:player; do
  name=${module%%:*}
  feature=${module#*:}
  cargo rustc --release --locked --target wasm32-unknown-unknown --no-default-features --features "$feature" --crate-type cdylib,rlib

  built="$root/core/target/wasm32-unknown-unknown/release/dlcore.wasm"
  committed="$root/js/$name.wasm"
  size=$(wc -c < "$built" | tr -d ' ')

  if [ "$check" = yes ]; then
    if [ ! -f "$committed" ]; then
      echo "build-core: js/$name.wasm is missing; run tools/build-core.sh and commit it" >&2
      status=1
    elif ! cmp -s "$built" "$committed"; then
      echo "build-core: js/$name.wasm does not match a build of core/ ($size bytes built," >&2
      echo "  $(wc -c < "$committed" | tr -d ' ') committed). Run tools/build-core.sh and commit the result." >&2
      status=1
    else
      echo "build-core: js/$name.wasm matches its source ($size bytes)"
    fi
  else
    cp "$built" "$committed"
    echo "build-core: wrote js/$name.wasm ($size bytes)"
  fi
done
exit $status
