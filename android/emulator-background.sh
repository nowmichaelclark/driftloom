#!/bin/sh
# Installs an APK on a headless emulator and runs the screen-off test on it
# (android/background.mjs): the music plays on with the screen off for ten
# minutes, the notification's and the lock screen's controls work, a call
# (simulated) pauses it and gives it back, nothing runs when it is paused, and
# an export finishes with the screen off into a file chosen in Android's own
# picker. What CI's `background` job runs; it runs on any Linux machine with
# KVM, the Android SDK and Node 22, and a sound server for the emulator's audio
# (CI's is PulseAudio with a null sink), as emulator-native.sh's does.
#
#   android/emulator-background.sh path/to/app-debug.apk [SUMMARY.md] [MINUTES]

set -eu

apk=${1:?usage: android/emulator-background.sh APK [SUMMARY] [MINUTES]}
summary=${2:-}
minutes=${3:-10}
here=$(cd "$(dirname "$0")" && pwd)
. "$here/emulator-lib.sh"
boot_emulator

adb install -r "$apk"
# The picker's first run offers its welcome and a notification of its own;
# the app's notification is asked once and refused here (a test of the app
# without it: the music and the controls must work).
adb shell pm revoke "$(sed -n 's/^driftloom\.applicationId=//p' "$here/gradle.properties")" android.permission.POST_NOTIFICATIONS 2>/dev/null || true
set -- node "$here/background.mjs" --minutes "$minutes"
[ -n "$summary" ] && set -- "$@" --summary "$summary"
"$@"
