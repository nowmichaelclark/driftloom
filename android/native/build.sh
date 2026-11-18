#!/bin/sh
# The core for Android (queue item 107b): libdriftloom.so for each ABI asked,
# built by cargo from this crate (and the core beside it, the same source as
# the web's module) with the NDK's linkers. What the app's Gradle build runs
# (android/app/build.gradle.kts, task buildNative); it runs alone too.
#
#   android/native/build.sh OUT_DIR NDK_DIR [ABI ...]
#
# ABI is arm64-v8a, armeabi-v7a or x86_64 (all three by default); each
# library lands in OUT_DIR/ABI/. NDK_DIR is the NDK the Gradle build names
# (driftloom.ndkVersion in android/gradle.properties). The compiler is the
# one rust-toolchain.toml pins, with these targets; cargo is found on PATH,
# in $CARGO, or in ~/.cargo/bin.
#
# Before each build the NDK's clang checks abi-check.c: the JNI slots and the
# AAudio numbers and shapes written by hand in src/ against the NDK's own
# headers. Paths are remapped so the library holds no machine's directories
# (reproducible builds, F-Droid's), and the sources are built from a copy at
# one fixed place: cargo's hash of a path dependency (the core, outside this
# crate's folder) includes where it is, the hash orders the library's
# symbols, and so the same commit built in two checkouts was two different
# libraries (docs/notes/107-reproducible.md). DRIFTLOOM_NATIVE_STAGE names
# another place; it must be the same on every machine that is to agree.

set -eu

out=${1:?usage: build.sh OUT_DIR NDK_DIR [ABI ...]}
ndk=${2:?usage: build.sh OUT_DIR NDK_DIR [ABI ...]}
shift 2
[ $# -gt 0 ] || set -- arm64-v8a armeabi-v7a x86_64

here=$(cd "$(dirname "$0")" && pwd)
src=$(cd "$here/../.." && pwd)
# The copy: this crate, the core and the data its build script reads, kept
# with their dates so cargo's own rebuild check still works; the build's
# target folder stays in it between builds.
root=${DRIFTLOOM_NATIVE_STAGE:-/tmp/driftloom-native-stage}
here=$root/android/native
mkdir -p "$root/android"
for part in core data android/native LICENSE; do
  if [ "$part" = android/native ]; then
    mkdir -p "$here"
    for f in "$src/android/native"/* "$src/android/native"/.[!.]*; do
      [ -e "$f" ] || continue
      [ "$(basename "$f")" = target ] && continue
      rm -rf "$here/$(basename "$f")"; cp -Rp "$f" "$here/"
    done
  else
    rm -rf "${root:?}/$part"; cp -Rp "$src/$part" "$root/$part"
  fi
done
rm -rf "$root/core/target"
api=27

case "$(uname -s)" in
  Linux) host=linux-x86_64 ;;
  Darwin) host=darwin-x86_64 ;;
  *) echo "build.sh: no NDK toolchain known for $(uname -s)" >&2; exit 2 ;;
esac
bin="$ndk/toolchains/llvm/prebuilt/$host/bin"
[ -d "$bin" ] || { echo "build.sh: no NDK toolchain at $bin" >&2; exit 2; }

cargo=${CARGO:-}
if [ -z "$cargo" ]; then
  if command -v cargo >/dev/null 2>&1; then cargo=cargo; else cargo="$HOME/.cargo/bin/cargo"; fi
fi
cargo_home=${CARGO_HOME:-$HOME/.cargo}

for abi in "$@"; do
  case "$abi" in
    arm64-v8a) target=aarch64-linux-android; clang=aarch64-linux-android$api-clang; var=AARCH64_LINUX_ANDROID ;;
    armeabi-v7a) target=armv7-linux-androideabi; clang=armv7a-linux-androideabi$api-clang; var=ARMV7_LINUX_ANDROIDEABI ;;
    x86_64) target=x86_64-linux-android; clang=x86_64-linux-android$api-clang; var=X86_64_LINUX_ANDROID ;;
    *) echo "build.sh: no such ABI: $abi" >&2; exit 2 ;;
  esac
  "$bin/$clang" -std=c11 -Wall -Werror -Wno-unused-const-variable -fsyntax-only "$here/abi-check.c"
  # 16 KB pages: Android 15's devices may use them, and a library aligned
  # to 4 KB would not load there (the NDK's default from r28; said anyway).
  flags="--remap-path-prefix=$root=/driftloom --remap-path-prefix=$cargo_home=/cargo -C link-arg=-Wl,-z,max-page-size=16384"
  # From this folder, so rustup takes its rust-toolchain.toml.
  (cd "$here" && env "CARGO_TARGET_DIR=$here/target" "CARGO_TARGET_${var}_LINKER=$bin/$clang" "CARGO_TARGET_${var}_RUSTFLAGS=$flags" \
    "$cargo" build --release --locked --target "$target")
  mkdir -p "$out/$abi"
  cp "$here/target/$target/release/libdriftloom.so" "$out/$abi/libdriftloom.so"
  echo "ok   $abi: $(wc -c <"$out/$abi/libdriftloom.so") bytes"
done
