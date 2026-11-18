#!/bin/sh
# The reproducibility check (queue item 107e; docs/notes/107-reproducible.md):
# the same commit's release APKs, built on two machines (a/ and b/, the
# release job's), must be the same bytes, and so must F-Droid's build of it
# (f/, the build job's). What differs, if anything, is printed entry by entry
# and with diffoscope, and written to the job's summary. Exits 1 when a and b
# differ; F-Droid's build differing from a is reported and fails too (the
# signature copying F-Droid verifies a signed APK with needs it identical).
#
#   a/<name>-<abi>-release-unsigned.apk  b/ the same  f/<id>_<versionCode>.apk
#
# Run from the directory that holds a/, b/ and (optionally) f/.

set -u
summary=${GITHUB_STEP_SUMMARY:-/dev/null}
status=0

entries() { # name crc size of every entry, sorted
  unzip -v "$1" | awk '$7 ~ /^[0-9a-f]{8}$/ { print $8, $7, $1 }' | sort
}

differ() { # X Y: explain a difference
  entries "$1" >/tmp/entries-x; entries "$2" >/tmp/entries-y
  echo "entries that differ (name, CRC-32, size):"
  diff /tmp/entries-x /tmp/entries-y | head -60 || true
  if diff -q /tmp/entries-x /tmp/entries-y >/dev/null; then
    echo "every entry's bytes are the same: the difference is in the zip's own layout (order, dates, extra fields)"
  fi
  if command -v diffoscope >/dev/null 2>&1; then
    diffoscope --text-color=never --max-report-size 40000 "$1" "$2" 2>&1 | head -150 || true
  fi
}

sha() { sha256sum "$1" | cut -c1-16; }

compare() { # label X Y
  if cmp -s "$2" "$3"; then
    echo "SAME  $1: $(sha "$2") ($(wc -c <"$2") bytes)"
    echo "| $1 | same | \`$(sha "$2")\` |" >>"$summary"
  else
    echo "DIFF  $1: $(sha "$2") ($(wc -c <"$2") bytes) against $(sha "$3") ($(wc -c <"$3") bytes)"
    echo "| $1 | **differ** | \`$(sha "$2")\` against \`$(sha "$3")\` |" >>"$summary"
    differ "$2" "$3"
    return 1
  fi
}

command -v diffoscope >/dev/null 2>&1 || sudo apt-get install -y -qq diffoscope-minimal >/dev/null 2>&1 || true

{ echo "### Reproducible builds"; echo; echo "| APK | result | sha-256 |"; echo "|---|---|---|"; } >>"$summary"
for x in a/*-release-unsigned.apk; do
  name=$(basename "$x")
  compare "$name, machine a against machine b" "$x" "b/$name" || status=1
done
f=$(ls f/*.apk 2>/dev/null | head -1 || true)
if [ -n "$f" ]; then
  compare "universal, machine a against F-Droid's build" a/*-universal-release-unsigned.apk "$f" || status=1
else
  echo "no F-Droid build to compare with"
fi
exit $status
