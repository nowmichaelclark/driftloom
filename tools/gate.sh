#!/bin/sh
# The fast gate (queue item 103): the checks that hold a merge until the app
# is 1.0 and on F-Droid, in parts that CI runs side by side
# (.github/workflows/core.yml) and a session runs here before it pushes.
#
#   tools/gate.sh                 every part, one after another
#   tools/gate.sh core            the committed .wasm files are their source;
#                                 the core's tests; clippy pedantic at zero
#   tools/gate.sh sketch          the same tests and clippy, sketch build
#   tools/gate.sh node            the Node tests, the generation goldens in
#                                 Node, and the balance lock on the Rust
#                                 generator
#   tools/gate.sh baseline [PARTS]
#                                 the core against its baseline ("nothing
#                                 moved"); PARTS as baseline.mjs --only takes
#                                 them, so CI can run them side by side
#   tools/gate.sh docs            the live docs' sizes
#
# What is not here runs nightly (.github/workflows/nightly.yml) and never
# holds a merge: every target's golden renders, Endless for 24 hours, the
# generation goldens in Chromium, rendering ahead's sweep, the other sample
# rates and the balance lock's own corpus. A red nightly is fixed by the next
# session that sees it.
#
# The baseline needs Playwright's Chromium (tools/baseline.mjs says how).

set -eu

root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root"

core() {
  tools/build-core.sh --check
  (cd core && cargo test --locked)
  (cd core && rustup component add clippy >/dev/null 2>&1 || true
   cargo clippy --all-targets --locked -- -W clippy::pedantic -D warnings
   cargo clippy --lib --locked --features capi -- -W clippy::pedantic -D warnings)
}

sketch() {
  (cd core && cargo test --locked --features sketch)
  (cd core && rustup component add clippy >/dev/null 2>&1 || true
   cargo clippy --all-targets --locked --features sketch -- -W clippy::pedantic -D warnings)
}

node_part() {
  for t in test/*.test.mjs; do
    echo "gate: node $t"
    node "$t"
  done
  node tools/gen-golden.mjs --check
  tools/gen-core.sh wasm32-unknown-unknown
  node tools/stats.mjs --check --core
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

case "${1:-all}" in
  core) core ;;
  sketch) sketch ;;
  node) node_part ;;
  baseline) baseline "${2:-}" ;;
  docs) docs ;;
  all) docs; core; sketch; node_part; baseline ;;
  *) echo "usage: tools/gate.sh [core | sketch | node | baseline [PARTS] | docs]" >&2; exit 2 ;;
esac
echo "gate: ${1:-all} passed"
