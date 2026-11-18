#!/bin/sh
# Boots an emulator, installs a debug APK and takes the listing's phone
# screenshots from the app (android/screenshots.mjs). CI's `screenshots` job.
#
#   android/emulator-screenshots.sh path/to/app-universal-debug.apk OUT_DIR

set -eu
apk=${1:?usage: android/emulator-screenshots.sh APK OUT_DIR}
out=${2:?usage: android/emulator-screenshots.sh APK OUT_DIR}
here=$(cd "$(dirname "$0")" && pwd)
. "$here/emulator-lib.sh"
boot_emulator
adb install -r -g "$apk"
# A phone's shape: the default AVD is a 1080 x 1920 phone; say what it is.
adb shell wm size
adb shell wm density
node "$here/screenshots.mjs" "$out"
