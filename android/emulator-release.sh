#!/bin/sh
# Installs a RELEASE APK (signed with the test key: the F-Droid build or the
# release workflow's, signed for this test only) on a headless emulator and
# checks that it is the real app and plays: not debuggable, the page loads, Play
# is pressed, the media session says playing, the process lives on and logcat
# holds no crash (the release is minified by R8, which the debug builds the
# other emulator tests drive are not; a name it renamed that the native core
# or the page's bridge looks up would show here). A release has no DevTools
# port, so the page is read through Android's accessibility tree (uiautomator).
# Queue item 107e; CI's `plays` job (.github/workflows/fdroid.yml).
#
#   android/emulator-release.sh path/to/release-signed.apk [screenshot.png]

set -eu

apk=${1:?usage: android/emulator-release.sh APK [SCREENSHOT]}
shot=${2:-}
here=$(cd "$(dirname "$0")" && pwd)
pkg=$(sed -n 's/^driftloom\.applicationId=//p' "$here/gradle.properties")
. "$here/emulator-lib.sh"
boot_emulator

fail() { failure="$*"; echo "FAIL $*" >&2; exit 1; }

adb install -r -g "$apk"
flags=$(adb shell dumpsys package "$pkg" | grep -m1 -E '^\s*flags=' || true)
echo "package flags: $flags"
case "$flags" in *DEBUGGABLE*) fail "the APK is debuggable: it is not a release build" ;; esac
echo "ok   the APK is not debuggable"

adb logcat -c
adb shell am start -W -n "$pkg/app.loops.MainActivity" >/dev/null

# The accessibility tree, as text: the page's buttons are in it by their
# labels (as text or content-desc). Prints "x y" for the center of the node whose label is $1.
center() {
  adb shell uiautomator dump /sdcard/ui.xml >/dev/null 2>&1 || return 1
  adb exec-out cat /sdcard/ui.xml | node -e '
    const xml = require("fs").readFileSync(0, "utf8");
    const want = process.argv[1];
    for (const m of xml.matchAll(/<node [^>]*>/g)) {
      const n = m[0];
      // A button is in the tree with its label as the text.
      if (!n.includes(`text="${want}"`) && !n.includes(`content-desc="${want}"`)) continue;
      const b = /bounds="\[(\d+),(\d+)\]\[(\d+),(\d+)\]"/.exec(n);
      if (b) { console.log(`${(+b[1] + +b[3]) >> 1} ${(+b[2] + +b[4]) >> 1}`); process.exit(0); }
    }
    process.exit(1);' "$1"
}
wait_for() { # what, label
  n=0
  until pos=$(center "$2"); do
    n=$((n + 1))
    if [ "$n" -gt 40 ]; then
      adb exec-out screencap -p >"${shot:-/tmp/release-fail.png}" || true
      tree=$(adb exec-out cat /sdcard/ui.xml 2>/dev/null | tr '>' '\n' | grep -o -E '(text|content-desc)="[^"]+"' | sort -u | head -40 | tr '\n' ';' || true)
      fail "the page never showed \"$2\" ($1); the accessibility tree held: $tree"
    fi
    sleep 3
  done
  echo "ok   $1 (\"$2\" at $pos)"
}
wait_for "the page loads in the app" "Play"
set -- $pos
adb shell input tap "$1" "$2"
wait_for "Play was pressed and the button became Pause" "Pause"

playing() { adb shell dumpsys media_session | grep -c 'state=PlaybackState {state=3' || true; }
n=0
until [ "$(playing)" -ge 1 ]; do
  n=$((n + 1))
  [ "$n" -gt 20 ] && { adb shell dumpsys media_session | head -60 >&2; fail "the media session never said playing"; }
  sleep 2
done
echo "ok   the media session says playing"
sleep 20
[ "$(playing)" -ge 1 ] || fail "it stopped playing within 20 s"
adb shell pidof "$pkg" >/dev/null || fail "the app's process is gone"
echo "ok   still playing, process alive, 20 s later"

if [ -n "$shot" ]; then adb exec-out screencap -p >"$shot" && echo "screenshot: $shot"; fi
bad=$(adb logcat -d | grep -E "FATAL EXCEPTION|UnsatisfiedLinkError|ClassNotFoundException|NoSuchMethodError|NoSuchFieldError|Process $pkg.* has died" || true)
[ -z "$bad" ] || { echo "$bad" >&2; fail "logcat holds a crash"; }
echo "ok   no crash in logcat"
echo "the release APK plays"
