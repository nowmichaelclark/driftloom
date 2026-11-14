#!/bin/sh
# What the project's tree must be (queue item 107a; F-Droid refuses a
# repository with prebuilt binaries):
#   1. Every file tracked under android/ is text, except the Gradle wrapper's
#      jar.
#   2. That jar is the one Gradle publishes for the version the wrapper
#      names: its SHA-256 is compared with services.gradle.org's, and the
#      wrapper's properties carry the distribution's own SHA-256.
#
#   android/check-tree.sh

set -eu
cd "$(dirname "$0")/.."

jar=android/gradle/wrapper/gradle-wrapper.jar
props=android/gradle/wrapper/gradle-wrapper.properties

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
git ls-files android | sort >"$tmp/tracked"
git grep -I --name-only -e '' -- android | sort >"$tmp/text" || true
binaries=$(comm -23 "$tmp/tracked" "$tmp/text")
other=$(echo "$binaries" | grep -v -x "$jar" | sed '/^$/d' || true)
if [ -n "$other" ]; then
  echo "binary files in android/ (only the Gradle wrapper's jar may be):" >&2
  echo "$other" >&2
  exit 1
fi
echo "ok   android/ holds no binary but the Gradle wrapper's jar"

version=$(sed -n 's|^distributionUrl=.*gradle-\(.*\)-bin\.zip$|\1|p' "$props")
[ -n "$version" ] || { echo "no Gradle version in $props" >&2; exit 1; }
have=$(sha256sum "$jar" | cut -d' ' -f1)
want=$(curl -fsSL "https://services.gradle.org/distributions/gradle-$version-wrapper.jar.sha256" | tr -d '[:space:]')
if [ "$have" != "$want" ]; then
  echo "the wrapper jar is not Gradle $version's: $have, published $want" >&2
  exit 1
fi
echo "ok   gradle-wrapper.jar is Gradle $version's (sha256 $have)"

grep -q '^distributionSha256Sum=[0-9a-f]\{64\}$' "$props" || { echo "$props has no distributionSha256Sum" >&2; exit 1; }
dist=$(sed -n 's|^distributionSha256Sum=||p' "$props")
published=$(curl -fsSL "https://services.gradle.org/distributions/gradle-$version-bin.zip.sha256" | tr -d '[:space:]')
[ "$dist" = "$published" ] || { echo "the distribution's checksum is not the published one" >&2; exit 1; }
echo "ok   the wrapper pins Gradle $version's distribution (sha256 $dist)"
