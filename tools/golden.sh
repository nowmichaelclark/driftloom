#!/bin/sh
# The golden renders on one target (queue item 42): render every note list
# in core/golden/ on TARGET and compare the hashes with the ones kept for it.
#
#   tools/golden.sh [TARGET]            render, compare with core/golden/TARGET.hashes
#   tools/golden.sh --update [TARGET]   render, and rewrite that file
#   tools/golden.sh --wav DIR [TARGET]  also write each list's output to DIR/NAME.wav
#   tools/golden.sh --strict [TARGET]   also fail if the hashes differ from wasm32's
#   tools/golden.sh --report            compare every kept target with wasm32's
#   tools/golden.sh --diff DIR_A DIR_B  how the WAVs written by two --wav runs differ,
#                                       list by list: how many samples, where the first
#                                       and the largest difference are, and how far
#                                       under the signal the difference lies
#
# TARGET is a Rust target triple, or wasm32-unknown-unknown (the app's
# engine, rendered in Node through the same exports the worklet calls); it
# defaults to this machine's. The others are built with the pinned
# toolchain and run directly (x86_64) or under QEMU's user mode, as
# core/.cargo/config.toml says. Each target is held to its own file: the
# hashes of one target may differ from another's until the core carries its
# own math (queue item 43), from which --strict is what CI runs. Rendering
# twice in one process must give the same hashes on every target; that is
# checked on every run.
#
# The hashes are FNV-1a 64 over each signal's samples (see the native host,
# core/examples/dlrender/notelist.rs). One line a list: its name, its
# length in frames, and the hash of the output, of each channel as it went
# into the mix, and of what reached the bus compressor.

set -eu

root=$(cd "$(dirname "$0")/.." && pwd)
golden="$root/core/golden"
if [ "${1:-}" = --diff ]; then
  a=${2:?--diff needs two directories}
  b=${3:?--diff needs two directories}
  cd "$root/core"
  for file in "$a"/*.wav; do
    name=$(basename "$file" .wav)
    printf '%-14s ' "$name"
    cargo run -q --release --locked --example dlrender -- --compare "$file" "$b/$name.wav"
  done
  exit 0
fi

update=no
strict=no
report=no
wav=''
target=''
while [ $# -gt 0 ]; do
  case "$1" in
    --update) update=yes ;;
    --strict) strict=yes ;;
    --report) report=yes ;;
    --wav) shift; wav=${1:?--wav needs a directory} ;;
    -h|--help) sed -n '2,/^$/p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
    -*) echo "golden: no such option: $1" >&2; exit 2 ;;
    *) target=$1 ;;
  esac
  shift
done

wasm=wasm32-unknown-unknown
tmp=$(mktemp)
trap 'rm -f "$tmp" "$tmp".*' EXIT

# The signals' names in the hash lines, for saying which differ.
differing() {
  # $1 and $2: two hash files. Prints, for each list that differs, its name
  # and the signals that do not match.
  awk '
    FNR == NR { for (i = 3; i <= NF; i++) a[$1 " " i] = $i; names[$1] = 1; heads[$1] = $0; next }
    { seen[$1] = 1
      n = split(heads[$1], h, " ")
      if (!($1 in names)) { print $1 ": not in the first file"; next }
      bad = ""
      for (i = 3; i <= NF; i++) if (a[$1 " " i] != $i) { split($i, kv, "="); bad = bad " " kv[1] }
      if (bad != "") print $1 ":" bad }
    END { for (n in names) if (!(n in seen)) print n ": not in the second file" }
  ' "$1" "$2" | sort
}

if [ "$report" = yes ]; then
  base="$golden/$wasm.hashes"
  [ -f "$base" ] || { echo "golden: $base is missing; run tools/golden.sh --update $wasm" >&2; exit 1; }
  grep -v '^#' "$base" > "$tmp.base"
  total=$(wc -l < "$tmp.base" | tr -d ' ')
  for file in "$golden"/*.hashes; do
    name=$(basename "$file" .hashes)
    [ "$name" = "$wasm" ] && continue
    grep -v '^#' "$file" > "$tmp.other"
    bad=$(differing "$tmp.base" "$tmp.other")
    count=$(printf '%s' "$bad" | grep -c . || true)
    echo "$name: $((total - count)) of $total lists match $wasm"
    if [ -n "$bad" ]; then printf '%s\n' "$bad" | sed 's/^/    /'; fi
  done
  exit 0
fi

if [ -z "$target" ]; then
  target=$(rustc -vV | sed -n 's/^host: //p')
fi
echo "golden: rendering on $target" >&2
if [ "$target" = "$wasm" ]; then
  if [ -n "$wav" ]; then node "$root/tools/golden-wasm.mjs" --twice --wav "$wav" > "$tmp"; else node "$root/tools/golden-wasm.mjs" --twice > "$tmp"; fi
else
  cd "$root/core"
  # One list at a time with --wav, since the host writes one file.
  if [ -n "$wav" ]; then
    mkdir -p "$wav"
    for list in golden/*.dlnotes; do
      name=$(basename "$list" .dlnotes)
      cargo run -q --release --locked --example dlrender --target "$target" -- --twice --wav "$wav/$name.wav" "$list" >> "$tmp"
    done
  else
    cargo run -q --release --locked --example dlrender --target "$target" -- --twice golden/*.dlnotes > "$tmp"
  fi
fi

file="$golden/$target.hashes"
if [ "$update" = yes ]; then
  {
    echo "# The golden hashes on $target (tools/golden.sh --update $target)."
    echo "# One line a note list: its frames, then FNV-1a 64 of the output, of each channel"
    echo "# as it went into the mix, and of the signal at the bus compressor."
    cat "$tmp"
  } > "$file"
  echo "golden: wrote core/golden/$target.hashes ($(wc -l < "$tmp" | tr -d ' ') lists)" >&2
  exit 0
fi

[ -f "$file" ] || { echo "golden: core/golden/$target.hashes does not exist; run tools/golden.sh --update $target" >&2; exit 1; }
grep -v '^#' "$file" > "$tmp.kept"
status=0
if cmp -s "$tmp" "$tmp.kept"; then
  echo "golden: $target: all $(wc -l < "$tmp" | tr -d ' ') lists match their hashes"
else
  echo "golden: $target: the renders moved"
  differing "$tmp.kept" "$tmp" | sed 's/^/  /'
  echo "  If the sound moved on purpose, run  tools/golden.sh --update $target  and say so in the PR." >&2
  status=1
fi

# And against the app's own engine, for the record (and, with --strict, as
# a failure).
if [ "$target" != "$wasm" ] && [ -f "$golden/$wasm.hashes" ]; then
  grep -v '^#' "$golden/$wasm.hashes" > "$tmp.wasm"
  bad=$(differing "$tmp.wasm" "$tmp")
  count=$(printf '%s' "$bad" | grep -c . || true)
  total=$(wc -l < "$tmp" | tr -d ' ')
  echo "golden: $target against $wasm: $((total - count)) of $total lists match"
  if [ -n "$bad" ]; then
    printf '%s\n' "$bad" | sed 's/^/  /'
    [ "$strict" = yes ] && status=1
  fi
fi
exit $status
