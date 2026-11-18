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
# A phone's shape: the AVD from the plain system image is 320 x 640 at 160 dpi,
# a small thing; a current phone is 1080 x 2400 at 420 dpi (about 411 dp wide),
# which is what the screenshots show. The activity takes the change in place.
adb shell wm size 1080x2400
adb shell wm density 420
sleep 3
adb shell wm size
adb shell wm density
node "$here/screenshots.mjs" "$out"
