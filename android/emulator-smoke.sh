#!/bin/sh
# Installs an APK on a headless emulator and runs the smoke test on it
# (android/smoke.mjs): the app starts, its page loads, Play is pressed, and
# Diagnostics shows the core's player. What CI's `smoke` job runs; it runs on
# any Linux machine with KVM, the Android SDK and Node 22.
#
#   android/emulator-smoke.sh path/to/app-debug.apk [screenshot.png]
#
# The image is an AOSP one ("default", not "google_apis"): no Google
# services anywhere, the emulator's included. Needs ANDROID_HOME (or
# ANDROID_SDK_ROOT) with cmdline-tools; it installs the emulator and the
# image if they are not there.

set -eu

apk=${1:?usage: android/emulator-smoke.sh APK [SCREENSHOT]}
shot=${2:-}
here=$(cd "$(dirname "$0")" && pwd)
sdk=${ANDROID_HOME:-${ANDROID_SDK_ROOT:?set ANDROID_HOME}}
api=35
image="system-images;android-$api;default;x86_64"
avd=driftloom-ci
sdkmanager="$sdk/cmdline-tools/latest/bin/sdkmanager"
export PATH="$sdk/platform-tools:$sdk/emulator:$PATH"

[ -e /dev/kvm ] || { echo "no /dev/kvm: the emulator would be far too slow" >&2; exit 2; }

yes | "$sdkmanager" --licenses >/dev/null 2>&1 || true
"$sdkmanager" "emulator" "platform-tools" "$image" >/dev/null

echo no | "$sdk/cmdline-tools/latest/bin/avdmanager" create avd --force -n "$avd" -k "$image" -d pixel_6 >/dev/null

log=$(mktemp)
emulator -avd "$avd" -no-window -no-audio -no-boot-anim -no-snapshot -no-metrics \
  -gpu swiftshader_indirect -accel on -memory 4096 -cores 3 >"$log" 2>&1 &
emu=$!
cleanup() {
  adb logcat -d -t 400 2>/dev/null | grep -E "chromium|WebView|AndroidRuntime|DEBUG|driftloom|app.loops" | tail -80 || true
  kill "$emu" 2>/dev/null || true
}
trap cleanup EXIT

echo "waiting for the emulator to boot"
n=0
until [ "$(adb shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" = 1 ]; do
  n=$((n + 1))
  if ! kill -0 "$emu" 2>/dev/null; then
    echo "the emulator stopped:" >&2; tail -60 "$log" >&2; exit 1
  fi
  if [ "$n" -gt 120 ]; then
    echo "the emulator did not boot in ten minutes:" >&2; tail -60 "$log" >&2; adb devices >&2 || true; exit 1
  fi
  [ "$n" = 3 ] && { echo "emulator log so far:"; tail -20 "$log"; adb devices; }
  sleep 5
done
echo "booted: $(adb shell getprop ro.build.version.release | tr -d '\r') (API $(adb shell getprop ro.build.version.sdk | tr -d '\r')), WebView $(adb shell dumpsys webviewupdate | grep -m1 'Current WebView package' | tr -d '\r')"
adb shell settings put global window_animation_scale 0 || true
adb shell settings put global transition_animation_scale 0 || true
adb shell settings put global animator_duration_scale 0 || true

adb install -r "$apk"
set -- node "$here/smoke.mjs"
[ -n "$shot" ] && set -- "$@" --shot "$shot"
"$@"
