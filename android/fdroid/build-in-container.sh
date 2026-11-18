#!/bin/bash
# Builds the app the way F-Droid's buildserver does (queue item 107e): inside
# F-Droid's own buildserver container, with fdroidserver's `fdroid build
# --on-server`, from the recipe render.mjs made, from a clean clone of the
# commit. Modelled on the `fdroid build` job of fdroiddata's CI
# (https://gitlab.com/fdroid/fdroiddata/-/blob/master/.gitlab-ci.yml), which
# every merge request to F-Droid's recipes passes. CI's `build` job runs it:
#
#   docker run --rm -v "$PWD:/src:ro" -v "$OUT:/out" \
#     -e APP_ID=... -e VERSION_CODE=... -e RECIPE=/src/... \
#     registry.gitlab.com/fdroid/fdroidserver:buildserver-trixie \
#     bash /src/android/fdroid/build-in-container.sh
#
# In: /src, the repository (a git checkout holding the commit, read only);
# RECIPE, the rendered recipe (its Repo is /home/vagrant/src, where this clones
# /src to); APP_ID, VERSION_CODE; NDK, the NDK's full version (installed here
# if the image lacks it, and said so: F-Droid's admins would have to). Out:
# /out/<id>_<versionCode>.apk (the unsigned release APK) and /out/build.log.

set -eux
: "${APP_ID:?}" "${VERSION_CODE:?}" "${RECIPE:?}" "${NDK:?}"
# F-Droid's CI uses this fdroidserver (a commit of its master); pinned the same.
FDROIDSERVER_COMMIT=${FDROIDSERVER_COMMIT:-a35fdfddd9c66823987a410566a6101186e39c84}
FDROIDSERVER_SINCE=${FDROIDSERVER_SINCE:-2026-07-13}

source /etc/profile.d/bsenv.sh
export ANDROID_HOME=${ANDROID_HOME:-/opt/android-sdk}
export DEBIAN_FRONTEND=noninteractive
home_vagrant=/home/vagrant

echo "::group::the image"
java -version 2>&1 || true
ls "$ANDROID_HOME" "$ANDROID_HOME/ndk" "$ANDROID_HOME/platforms" "$ANDROID_HOME/build-tools" 2>&1 || true
ls /opt/gradle/versions 2>&1 || true
echo "::endgroup::"

apt-get update -qq
apt-get install -y -qq sudo git curl ca-certificates >/dev/null

# What a build needs of the SDK, if the image does not hold it already: said
# plainly, since F-Droid's own buildserver would need the same (the NDK is
# named in the recipe; the platform and build tools Gradle fetches itself
# there, which this stands in for).
sdkmanager=$(command -v sdkmanager || echo "$ANDROID_HOME/cmdline-tools/latest/bin/sdkmanager")
need=()
[ -d "$ANDROID_HOME/ndk/$NDK" ] || { echo "NOTE: NDK $NDK is not in the image; installing it from Google's repository"; need+=("ndk;$NDK"); }
[ -d "$ANDROID_HOME/platforms/android-36" ] || need+=("platforms;android-36")
[ -d "$ANDROID_HOME/build-tools/36.0.0" ] || need+=("build-tools;36.0.0")
if [ ${#need[@]} -gt 0 ]; then
  yes | "$sdkmanager" --licenses >/dev/null || true
  "$sdkmanager" "${need[@]}"
fi
chmod -R a+rX "$ANDROID_HOME"

# fdroidserver, as fdroiddata's CI gets it.
rm -rf "$fdroidserver"
git clone -q --shallow-since="$FDROIDSERVER_SINCE" https://gitlab.com/fdroid/fdroidserver.git "$fdroidserver"
git -C "$fdroidserver" checkout -q -B master "$FDROIDSERVER_COMMIT"
export PATH="$fdroidserver:$PATH" PYTHONPATH="$fdroidserver:$fdroidserver/examples" PYTHONUNBUFFERED=true

# The source: a clean clone of the commit, as F-Droid would make from the
# repository's address (the recipe's Repo points here).
git config --global --add safe.directory '*'
rm -rf "$home_vagrant/src"
git clone -q /src "$home_vagrant/src"

# fdroiddata's layout, in the build user's home.
cd "$home_vagrant"
mkdir -p metadata srclibs build logs tmp unsigned .android .gradle
cp "$RECIPE" "metadata/$APP_ID.yml"
curl -fsSL https://gitlab.com/fdroid/fdroiddata/-/raw/master/srclibs/rustup.yml -o srclibs/rustup.yml
printf 'sdk_path: %s\ngradle_version_dir: /opt/gradle/versions\n' "$ANDROID_HOME" > config.yml
cp -f "$fdroidserver/buildserver/gradle.properties" .gradle/ 2>/dev/null || true
cp /root/.gitconfig "$home_vagrant/.gitconfig" 2>/dev/null || true
chown -R vagrant:vagrant "$home_vagrant"

# `fdroid build --on-server` expects sudo, runs the recipe's sudo commands,
# then removes it, as the buildserver VM does.
export GRADLE_USER_HOME=$home_vagrant/.gradle
fdroid() {
  sudo --preserve-env --user vagrant \
    env PATH="$fdroidserver:$PATH" PYTHONPATH="$fdroidserver:$fdroidserver/examples" PYTHONUNBUFFERED=true \
    TERM="${TERM:-dumb}" HOME="$home_vagrant" GRADLE_USER_HOME="$GRADLE_USER_HOME" \
    fdroid "$@"
}
fdroid fetchsrclibs "$APP_ID:$VERSION_CODE" --verbose
(unset CI; fdroid build --verbose --test --refresh-scanner --on-server --no-tarball "$APP_ID:$VERSION_CODE") 2>&1 | tee /out/build.log

ls -l tmp
cp "tmp/${APP_ID}_${VERSION_CODE}.apk" /out/
ls -l /out
