# The Android app's reproducible build (item 107e, session 29)

The same commit must build to the same APK bytes on any machine: that is what
lets F-Droid publish an APK signed with Mikey's own key (`for-mikey.md`), and
it is what F-Droid's checks hold a recipe to. CI's `fdroid` workflow proves it
on every Android change (the job `reproducible`).

## What is pinned and what is removed

- **Tools:** JDK 21, Gradle 8.14.3 (the wrapper), AGP 8.13.2, NDK r28c
  (28.2.13676358), Rust 1.99.0 (`rust-toolchain.toml`, installed from rustup
  1.28.2 in the recipe), the buildserver image's own toolchain for the rest.
- **Nothing stamped:** `vcsInfo` and `dependenciesInfo` are off in the release
  build (no commit id, no encrypted dependency blob); the entries' dates are
  fixed by AGP; no build date is written anywhere.
- **Paths out of the native library:** `android/native/build.sh` remaps its
  paths (`--remap-path-prefix`) and builds from a copy at one fixed place
  (`/tmp/driftloom-native-stage`). See below for why the copy.
- **The web module rebuilt from source:** the recipe deletes the committed
  `js/dlplayer.wasm` (F-Droid's scanner refuses binaries) and `tools/build-core.sh`
  builds it again, with the same remapping.

## What differed, and the fix

First result (the commit before the fix): the release APKs built on two
machines, in different directories (`driftloom` and
`some/other/deeper/place/checkout`) and time zones (UTC and Asia/Bangkok),
differed, in `lib/<abi>/libdriftloom.so` only. The ARM library's `.text` and
every later section moved by 12 to 16 bytes; the x86_64 library had the same
bytes in another order. No string in the library held a path.

Cause, found by building twice on the host target: cargo hashes a dependency's
package id into the crate's metadata, and for a path dependency inside the
workspace it hashes the path relative to the workspace root, but the core
(`../../core`) is outside the crate's folder, so the absolute path went into
the hash. The hash is in every symbol's name and orders the library's symbols
and code; so where the checkout is changed the library. `--remap-path-prefix`
does not touch it, as it only rewrites strings.

Fix: `build.sh` copies the crate, the core and the data its build script reads
to a fixed folder and builds there (dates kept, so cargo's own rebuild check
still works). Built from two checkouts of the same commit on the host target:
the same sha-256. `DRIFTLOOM_NATIVE_STAGE` names another place; every machine
that is to agree must use the same one.

## The check

`android/fdroid/compare-apks.sh` (the job's last step) compares, with `cmp`:
each ABI's release APK from machine a against machine b, then the universal
APK against the one F-Droid's own tooling built from a clean clone (the job
`build`). When bytes differ it prints the entries (name, CRC-32, size) that
differ, the strings and sections of a differing library, and diffoscope's
report, and fails the job.

**Result** (commit 3a91fe1, CI run 44389330525; the commit before it failed
with the differences above): every release APK built on machine a is the same
bytes as the one built on machine b, in other directories and another time
zone, and the universal one is the same bytes as F-Droid's own build of it from
a clean clone.

| APK | a against b | sha-256 (first 16) | bytes |
|---|---|---|---|
| arm64-v8a | same | `6db04a55a97f8257` | 2282394 |
| armeabi-v7a | same | `a09f90f0714b7516` | 2004556 |
| x86_64 | same | `1cbfc948f4548592` | 2631695 |
| universal | same | `d974e11f7c0d2b3f` | 5335207 |
| universal, a against F-Droid's build | same | `d974e11f7c0d2b3f` | 5335207 |

What this does not cover: a different machine type or a newer base image (the
runners are `ubuntu-24.04`, F-Droid's build is in its buildserver image, and
both agreed), and a new Rust, NDK, AGP or JDK, which move the bytes on their
own: bump them together and let this job say whether they still agree.

## Who signs

A reproducible build is what makes "my own key" work: F-Droid builds from
source, checks that its unsigned APK is the same bytes as the one in the
developer's signature's APK, and publishes the developer's. If the check ever
fails, F-Droid refuses the update, and the fallback is the one signing choice
that needs no reproducibility: F-Droid signs with its own key (the app then
can't be updated from any other source, and Mikey's own APKs from the release
workflow can't install over it). `for-mikey.md` has the choice.
