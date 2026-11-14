#!/bin/sh
# The fast gate (queue item 103): the checks that hold a merge until the app
# is 1.0 and on F-Droid, in parts that CI runs side by side
# (.github/workflows/core.yml) and a session runs here before it pushes.
#
#   tools/gate.sh                 every part, one after another
#   tools/gate.sh core            the committed .wasm is its source; the
#                                 core's tests; clippy pedantic at zero
#   tools/gate.sh sketch          the same tests and clippy, sketch build
#   tools/gate.sh node            the Node tests, the generation goldens and
#                                 the balance lock, on the core's generator
#                                 (the tools' build: it makes it first)
#   tools/gate.sh baseline [PARTS]
#                                 the core against its baseline ("nothing
#                                 moved"); PARTS as baseline.mjs --only takes
#                                 them, so CI can run them side by side
#   tools/gate.sh flac            the FLAC writer on real renders against
#                                 the reference decoder (queue item 104a,
#                                 tools/flac.mjs; needs flac or ffmpeg)
#   tools/gate.sh docs            the live docs' sizes
#
# What is not here runs nightly (.github/workflows/nightly.yml) and never
# holds a merge: every target's golden renders, Endless for 24 hours,
# rendering ahead's sweep and the other sample rates. A red nightly is fixed
# by the next session that sees it.
#
# Nothing here needs a browser: the baseline plays the player in Node (queue
# item 96).

set -eu

root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root"

core() {
  tools/build-core.sh --check
  (cd core && cargo test --locked)
  (cd core && rustup component add clippy >/dev/null 2>&1 || true
   cargo clippy --all-targets --locked -- -W clippy::pedantic -D warnings
   cargo clippy --lib --locked --features capi -- -W clippy::pedantic -D warnings
   cargo clippy --lib --locked --features tools -- -W clippy::pedantic -D warnings)
}

sketch() {
  (cd core && cargo test --locked --features sketch)
  (cd core && rustup component add clippy >/dev/null 2>&1 || true
   cargo clippy --all-targets --locked --features sketch -- -W clippy::pedantic -D warnings)
}

node_part() {
  # The tools' build: the generator's parts, for the tests and tools that
  # read them (tools/gen-lib.mjs).
  tools/build-core.sh --tools
  for t in test/*.test.mjs; do
    echo "gate: node $t"
    node "$t"
  done
  node tools/gen-golden.mjs --check
  node tools/stats.mjs --check
}

baseline() {
  if [ -n "${1:-}" ]; then
    node tools/baseline.mjs --only "$1"
  else
    node tools/baseline.mjs
  fi
}

docs() {
  node tools/check-docs.mjs
}

flac_part() {
  node tools/flac.mjs
}

case "${1:-all}" in
  core) core ;;
  sketch) sketch ;;
  node) node_part ;;
  baseline) baseline "${2:-}" ;;
  docs) docs ;;
  flac) flac_part ;;
  all) docs; core; sketch; node_part; flac_part; baseline ;;
  *) echo "usage: tools/gate.sh [core | sketch | node | baseline [PARTS] | flac | docs]" >&2; exit 2 ;;
esac
echo "gate: ${1:-all} passed"
