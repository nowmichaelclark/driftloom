# Boots a headless AOSP emulator for the CI scripts (emulator-smoke.sh,
# emulator-native.sh); sourced, not run. After `boot_emulator [FLAGS...]`
# the emulator is up, adb sees it, and a trap stops it (with the app's lines
# from logcat) when the calling script exits. FLAGS are extra emulator
# options: the smoke test passes -no-audio; the native test passes none, so
# the emulator plays into the host's sound server (CI's is a null sink).
#
# The image is an AOSP one ("default", not "google_apis"): no Google
# services anywhere, the emulator's included. Needs ANDROID_HOME (or
# ANDROID_SDK_ROOT) with cmdline-tools, and KVM; it installs the emulator and
# the image if they are not there.

sdk=${ANDROID_HOME:-${ANDROID_SDK_ROOT:?set ANDROID_HOME}}
api=35
image="system-images;android-$api;default;x86_64"
avd=driftloom-ci
sdkmanager="$sdk/cmdline-tools/latest/bin/sdkmanager"
export PATH="$sdk/platform-tools:$sdk/emulator:$PATH"

boot_emulator() {
  [ -e /dev/kvm ] || { echo "no /dev/kvm: the emulator would be far too slow" >&2; exit 2; }

  yes | "$sdkmanager" --licenses >/dev/null 2>&1 || true
  "$sdkmanager" "emulator" "platform-tools" "$image" >/dev/null

  export ANDROID_AVD_HOME="${ANDROID_AVD_HOME:-$HOME/.android/avd}"
  mkdir -p "$ANDROID_AVD_HOME"
  echo no | "$sdk/cmdline-tools/latest/bin/avdmanager" create avd --force -n "$avd" -k "$image"
  emulator -list-avds | grep -x "$avd" || { echo "the AVD $avd was not made (in $ANDROID_AVD_HOME)" >&2; ls -la "$ANDROID_AVD_HOME" >&2; exit 1; }

  emulator_log=$(mktemp)
  emulator -avd "$avd" -no-window "$@" -no-boot-anim -no-snapshot -no-metrics \
    -gpu swiftshader_indirect -accel on -memory 4096 -cores 3 >"$emulator_log" 2>&1 &
  emu=$!
  trap stop_emulator EXIT

  echo "waiting for the emulator to boot"
  n=0
  until [ "$(adb shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" = 1 ]; do
    n=$((n + 1))
    if ! kill -0 "$emu" 2>/dev/null; then
      echo "the emulator stopped:" >&2; tail -60 "$emulator_log" >&2; exit 1
    fi
    if [ "$n" -gt 120 ]; then
      echo "the emulator did not boot in ten minutes:" >&2; tail -60 "$emulator_log" >&2; adb devices >&2 || true; exit 1
    fi
    [ "$n" = 3 ] && { echo "emulator log so far:"; tail -20 "$emulator_log"; adb devices; }
    sleep 5
  done
  echo "booted: $(adb shell getprop ro.build.version.release | tr -d '\r') (API $(adb shell getprop ro.build.version.sdk | tr -d '\r')), WebView $(adb shell dumpsys webviewupdate | grep -m1 'Current WebView package' | tr -d '\r')"
  adb shell settings put global window_animation_scale 0 || true
  adb shell settings put global transition_animation_scale 0 || true
  adb shell settings put global animator_duration_scale 0 || true
}

stop_emulator() {
  timeout 20 adb logcat -d -t 400 2>/dev/null | grep -E "chromium|WebView|AndroidRuntime|DEBUG|driftloom|app.loops|AAudio" | tail -80 || true
  # What the calling script said went wrong, after the log, where it is read.
  [ -z "${failure:-}" ] || echo "FAILED: $failure"
  kill "$emu" 2>/dev/null || true
}
