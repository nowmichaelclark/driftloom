#!/bin/sh
# What is in the release APK (queue item 107e): the anti-features check and
# the list of what ships.
#
#   android/fdroid/check-apk.sh APK DEPENDENCIES.txt
#
# 1. The permissions are exactly the ones AndroidManifest.xml declares and
#    docs/notes/107-licenses.md names: no network, no location, no storage.
# 2. No tracker, no ad or analytics library, no Google Play Services: none of
#    their classes are in the code. (F-Droid's anti-features: Ads, Tracking,
#    NonFreeNet, NonFreeDep, NonFreeAdd, NonFreeAssets, KnownVuln: none apply.)
# 3. The APK holds the app's own files (assets/www from sw.js's list), the
#    core's library for each ABI, the code and the resources, and nothing else.
# 4. Every library in the release's runtime classpath (the dependencies' text)
#    is in android/LICENSES.md with its license, and nothing is listed there
#    that is not in the APK.

set -eu
apk=${1:?usage: check-apk.sh APK DEPENDENCIES.txt}
deps=${2:?usage: check-apk.sh APK DEPENDENCIES.txt}
here=$(cd "$(dirname "$0")" && pwd)
root=$here/../..
failed=0
ok() { echo "ok   $*"; }
bad() { echo "FAIL $*"; failed=1; }

aapt2=$(ls "${ANDROID_HOME:-/opt/android-sdk}"/build-tools/*/aapt2 2>/dev/null | sort -V | tail -1)

# 1. permissions
want="android.permission.FOREGROUND_SERVICE android.permission.FOREGROUND_SERVICE_DATA_SYNC android.permission.FOREGROUND_SERVICE_MEDIA_PLAYBACK android.permission.POST_NOTIFICATIONS android.permission.WAKE_LOCK"
have=$("$aapt2" dump permissions "$apk" | sed -n "s/^uses-permission: name='\(.*\)'.*/\1/p" | sort | tr '\n' ' ' | sed 's/ $//')
# the app's own signature-level permission, which AndroidX adds for its receivers, is the app's id
have=$(echo "$have" | tr ' ' '\n' | grep -v '\.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION$' | tr '\n' ' ' | sed 's/ $//')
[ "$have" = "$want" ] && ok "permissions: $have" || bad "permissions are [$have], expected [$want]"
echo "$have" | grep -q 'INTERNET' && bad "the APK asks for the internet" || ok "no INTERNET permission"

# 2. classes of trackers, ads and Google's services
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
unzip -q -o "$apk" 'classes*.dex' -d "$tmp"
strings -n 8 "$tmp"/classes*.dex >"$tmp/strings"
found=$(grep -E 'Lcom/google/android/gms/|Lcom/google/firebase/|Lcom/crashlytics|Lio/fabric|Lio/sentry|Lcom/facebook/|Lcom/appsflyer|Lcom/adjust/|Lcom/mixpanel|Lcom/amplitude|Lcom/google/android/play/core|Lcom/google/android/ump|Lcom/google/ads|Lcom/applovin|Lcom/unity3d/ads|Lcom/ironsource|Lcom/flurry|Lcom/segment|Lcom/microsoft/appcenter|Lcom/bugsnag|Lcom/onesignal' "$tmp/strings" | head -5 || true)
[ -z "$found" ] && ok "no tracker, ad or Google Play Services class in the code" || bad "classes of a tracker or Google's services: $found"
# the page's own network use: none (the manifest has no permission, and the WebView blocks it)

# 3. what the APK holds
unzip -Z1 "$apk" | sed -E 's,^(assets/www)/.*,\1/,; s,^res/.*,res/,; s,^lib/([^/]+)/(.*),lib/\1/\2,; s,^classes[0-9]*\.dex$,classes.dex,; s,^(META-INF/).*,\1,; s,^kotlin/.*,kotlin/,' | sort -u >"$tmp/kinds"
cat "$tmp/kinds"
extra=$(grep -v -x -E 'assets/www/|res/|lib/(arm64-v8a|armeabi-v7a|x86_64)/libdriftloom\.so|classes\.dex|META-INF/|kotlin/|AndroidManifest\.xml|resources\.arsc|assets/dexopt/baseline\.prof(m)?|DebugProbesKt\.bin|androidsupportmultidexversion\.txt' "$tmp/kinds" || true)
[ -z "$extra" ] && ok "the APK holds the app's files, the core's library, the code and the resources" || bad "other entries in the APK: $extra"
unzip -Z1 "$apk" | grep '^lib/' | grep -v 'libdriftloom\.so$' && bad "a native library besides the core's" || ok "the only native library is libdriftloom.so (built from this repository's source)"
# the app's files are the list sw.js keeps
n=$(unzip -Z1 "$apk" | grep -c '^assets/www/' || true)
want_n=$(node -e '
  const t = require("fs").readFileSync(process.argv[1] + "/sw.js", "utf8");
  const b = /const FILES = \[(.*?)\n\];/s.exec(t)[1].split("\n").filter((l) => !l.trim().startsWith("//"));
  console.log(b.flatMap((l) => [...l.matchAll(/\x27\.\/([^\x27]*)\x27/g)]).length);' "$root")
[ "$n" = "$want_n" ] && ok "assets/www holds the $n files sw.js lists" || bad "assets/www holds $n files; sw.js lists $want_n"

# 4. the libraries and their licenses
sed -n 's/^.*--- \([^ :]*:[^ :]*\):\([^ ]*\).*/\1/p' "$deps" | sort -u >"$tmp/libs"
echo "libraries in the release's runtime classpath:"; cat "$tmp/libs"
for lib in $(cat "$tmp/libs"); do
  grep -q "\`$lib\`" "$root/android/LICENSES.md" && ok "licensed: $lib" || bad "$lib is not in android/LICENSES.md"
done
for lib in $(sed -n 's/^| `\([^`]*:[^`]*\)`.*/\1/p' "$root/android/LICENSES.md" | grep -v '^$'); do
  grep -qx "$lib" "$tmp/libs" || bad "android/LICENSES.md lists $lib, which the release does not use"
done
exit $failed
