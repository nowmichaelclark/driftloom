# The test builds' signing key

`debug.keystore.base64` is a throwaway key (the keystore, in text), made with
`keytool -genkeypair -alias androiddebugkey -storepass android -keypass android
-keyalg RSA -keysize 2048 -validity 36500 -storetype PKCS12`. `app/build.gradle.kts`
decodes it into `app/build/` and signs the **debug** build with it.

Why a committed key: a debug key made fresh on each CI runner is different every
time, and Android refuses to install an APK over one signed by another key, so
every test build would have to be uninstalled first (and the listener's loops
with it). One shared test key lets each build update the last.

What it is not: a secret, or a thing to trust. Anyone can sign an APK with it.
It signs test builds only. The release is signed by a different key (F-Droid's,
or Mikey's: session 29 and `docs/notes/for-mikey.md`), which is never in the
tree. To go back to a per-machine key, delete this folder's `.base64` and the
`signingConfigs` block.
