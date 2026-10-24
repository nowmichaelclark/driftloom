#!/bin/sh
# The golden renders on one target (queue item 42): render every note list
# in core/golden/ on TARGET and compare the hashes with the ones kept for it.
#
#   tools/golden.sh [TARGET]            render, compare with core/golden/TARGET.hashes
#   tools/golden.sh --update [TARGET]   render, and rewrite that file
#   tools/golden.sh --wav DIR [TARGET]  also write each list's output to DIR/NAME.wav
#   tools/golden.sh --strict [TARGET]   also fail if the hashes differ from wasm32's
#   tools/golden.sh --flush [TARGET]    render with the processor flushing subnormal
#                                       numbers to zero (x86-64 and aarch64): the
#                                       core never makes one, so nothing may change
#   tools/golden.sh --report            compare every kept target with wasm32's
#   tools/golden.sh --blocks [all] [TARGET]
#                                       also render the lists the way a host with its own
#                                       block size does (1, 64, 128, 441 and 1,024 frames a
#                                       call, and a different number every call): the
#                                       speaker output must be the plain render's, to the
#                                       bit. Three short lists, or all of them (queue item 45)
#   tools/golden.sh --rates [--update] [TARGET]
#                                       render the lists in core/golden/rates.txt at the
#                                       sample rates it names and compare with
#                                       core/golden/rates.hashes, one file for every target:
#                                       the same code makes the same samples at any rate,
#                                       everywhere (queue item 45)
#   tools/golden.sh --stereo ...        any of the above on the stereo render (queue item 64):
#                                       both sides hashed, kept in core/golden/TARGET.stereo.hashes
#                                       (and rates.stereo.hashes) beside the mono files
#   tools/golden.sh --diff DIR_A DIR_B  how the WAVs written by two --wav runs differ,
#                                       list by list: how many samples, where the first
#                                       and the largest difference are, and how far
#                                       under the signal the difference lies
#
# TARGET is a Rust target triple, or wasm32-unknown-unknown (the app's
# engine, rendered in Node through the same exports the worklet calls); it
# defaults to this machine's. The others are built with the pinned
# toolchain and run directly (x86_64) or under QEMU's user mode, as
# core/.cargo/config.toml says. Each target is held to its own file, and
# since the core carries its own math (queue item 43) every file must equal
# wasm32's: --strict, which CI runs, fails on any difference. Rendering
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
flush=''
report=no
wav=''
target=''
blocks=''
rates=no
stereo=''
kind=''
while [ $# -gt 0 ]; do
  case "$1" in
    --update) update=yes ;;
    --stereo) stereo=--stereo; kind=.stereo ;;
    --strict) strict=yes ;;
    --flush) flush=--flush ;;
    --report) report=yes ;;
    --blocks) blocks=short; [ "${2:-}" != all ] || { blocks=all; shift; } ;;
    --rates) rates=yes ;;
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
  base="$golden/$wasm$kind.hashes"
  [ -f "$base" ] || { echo "golden: $base is missing; run tools/golden.sh --update $wasm" >&2; exit 1; }
  grep -v '^#' "$base" > "$tmp.base"
  total=$(wc -l < "$tmp.base" | tr -d ' ')
  for file in "$golden"/*-*.hashes; do
    name=$(basename "$file" .hashes)
    # One layout at a time: mono's files, or (with --stereo) stereo's.
    case "$name" in *.stereo) [ -n "$kind" ] || continue; name=${name%.stereo} ;; *) [ -z "$kind" ] || continue ;; esac
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

# The sizes a host's audio callback might ask for: one frame, a power of two
# below the core's own block, the core's block, a size that is nobody's
# multiple, a big one, and a different one every call.
sizes=1,64,128,441,1024,random

if [ "$rates" = yes ]; then
  # Which lists, at which rates (core/golden/rates.txt); one hash file for
  # every target, since they must all make the same samples.
  list_names=$(sed -n 's/^lists //p' "$golden/rates.txt")
  rate_list=$(sed -n 's/^rates //p' "$golden/rates.txt")
  echo "golden: rendering at $rate_list Hz on $target" >&2
  : > "$tmp"
  for rate in $rate_list; do
    files=''
    if [ "$target" = "$wasm" ]; then prefix="$golden"; else prefix=golden; fi
    for name in $list_names; do files="$files $prefix/$name.dlnotes"; done
    if [ "$target" = "$wasm" ]; then
      # Above 96 kHz the committed .wasm (the web build) has no room: the
      # wide build, made for this, renders it.
      if [ "$rate" -gt 96000 ]; then
        [ -f "$root/core/target/dlcore-wide.wasm" ] || "$root/tools/build-core.sh" --wide >&2
        module="$root/core/target/dlcore-wide.wasm"
      else
        module="$root/js/dlcore.wasm"
      fi
      (cd "$root" && node tools/golden-wasm.mjs --twice $stereo --wasm "$module" --rate "$rate" $files) >> "$tmp"
    else
      (cd "$root/core" && cargo run -q --release --locked --example dlrender --target "$target" -- --twice $flush $stereo --rate "$rate" $files) >> "$tmp"
    fi
  done
  file="$golden/rates$kind.hashes"
  if [ "$update" = yes ]; then
    {
      echo "# The golden lists named in rates.txt at the sample rates it names (tools/golden.sh $stereo --rates --update)."
      echo "# One line a list and rate, in the golden hashes' form. The same on every target."
      cat "$tmp"
    } > "$file"
    echo "golden: wrote core/golden/rates$kind.hashes ($(wc -l < "$tmp" | tr -d ' ') renders)" >&2
    exit 0
  fi
  grep -v '^#' "$file" > "$tmp.kept"
  if cmp -s "$tmp" "$tmp.kept"; then
    echo "golden: $target: all $(wc -l < "$tmp" | tr -d ' ') renders at other rates match their hashes"
    exit 0
  fi
  echo "golden: $target: the renders at other rates moved"
  differing "$tmp.kept" "$tmp" | sed 's/^/  /'
  echo "  If the sound moved on purpose, run  tools/golden.sh --rates --update  and say so in the PR." >&2
  exit 1
fi

if [ -n "$blocks" ]; then
  if [ "$blocks" = all ]; then
    names=$(cd "$golden" && ls *.dlnotes | sed 's/\.dlnotes$//')
  else
    names="14-bloom 16-grove 19-glade"
  fi
  files=''
  if [ "$target" = "$wasm" ]; then prefix="$golden"; else prefix=golden; fi
  for name in $names; do files="$files $prefix/$name.dlnotes"; done
  echo "golden: $target: rendering $(echo $names | wc -w | tr -d ' ') lists in calls of $sizes frames" >&2
  if [ "$target" = "$wasm" ]; then
    (cd "$root" && node tools/golden-wasm.mjs $stereo --blocks "$sizes" $files) > /dev/null
  else
    (cd "$root/core" && cargo run -q --release --locked --example dlrender --target "$target" -- $stereo --blocks "$sizes" $files) > /dev/null
  fi
  echo "golden: $target: the speaker output is the same at every block size"
  exit 0
fi
echo "golden: rendering on $target" >&2
if [ "$target" = "$wasm" ]; then
  [ -z "$flush" ] || { echo "golden: --flush is for native targets" >&2; exit 2; }
  if [ -n "$wav" ]; then node "$root/tools/golden-wasm.mjs" --twice $stereo --wav "$wav" > "$tmp"; else node "$root/tools/golden-wasm.mjs" --twice $stereo > "$tmp"; fi
else
  cd "$root/core"
  # One list at a time with --wav, since the host writes one file.
  if [ -n "$wav" ]; then
    mkdir -p "$wav"
    for list in golden/*.dlnotes; do
      name=$(basename "$list" .dlnotes)
      cargo run -q --release --locked --example dlrender --target "$target" -- --twice $flush $stereo --wav "$wav/$name.wav" "$list" >> "$tmp"
    done
  else
    cargo run -q --release --locked --example dlrender --target "$target" -- --twice $flush $stereo golden/*.dlnotes > "$tmp"
  fi
fi

file="$golden/$target$kind.hashes"
if [ "$update" = yes ]; then
  [ -z "$flush" ] || { echo "golden: --update keeps plain renders; drop --flush" >&2; exit 2; }
  {
    echo "# The golden hashes on $target (tools/golden.sh $stereo --update $target)."
    echo "# One line a note list: its frames, then FNV-1a 64 of the output, of each channel"
    echo "# as it went into the mix, and of the signal at the bus compressor."
    [ -z "$stereo" ] || echo "# The stereo render: each side of the output and of the bus apart."
    cat "$tmp"
  } > "$file"
  echo "golden: wrote core/golden/$target$kind.hashes ($(wc -l < "$tmp" | tr -d ' ') lists)" >&2
  exit 0
fi

[ -f "$file" ] || { echo "golden: core/golden/$target$kind.hashes does not exist; run tools/golden.sh $stereo --update $target" >&2; exit 1; }
grep -v '^#' "$file" > "$tmp.kept"
status=0
if cmp -s "$tmp" "$tmp.kept"; then
  echo "golden: $target: all $(wc -l < "$tmp" | tr -d ' ') lists match their hashes"
else
  echo "golden: $target: the renders moved"
  differing "$tmp.kept" "$tmp" | sed 's/^/  /'
  echo "  If the sound moved on purpose, run  tools/golden.sh $stereo --update $target  and say so in the PR." >&2
  status=1
fi

# And against the app's own engine, for the record (and, with --strict, as
# a failure).
if [ "$target" != "$wasm" ] && [ -f "$golden/$wasm$kind.hashes" ]; then
  grep -v '^#' "$golden/$wasm$kind.hashes" > "$tmp.wasm"
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
