import java.util.Base64

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

val appId = providers.gradleProperty("driftloom.applicationId").get()
val appName = providers.gradleProperty("driftloom.appName").get()

// The ABIs the core is built for (session 26): phones and tablets (arm64-v8a,
// armeabi-v7a) and the emulator (x86_64). -Pdriftloom.abis=x86_64 builds one,
// for a quicker local build.
val abis = providers.gradleProperty("driftloom.abis").get().split(',').map { it.trim() }.filter { it.isNotEmpty() }

base.archivesName.set(appName.lowercase())

android {
    // The Kotlin package of the code, not the app's id: it stays when the
    // id changes at the rename (session 31a).
    namespace = "app.loops"
    compileSdk = 36
    // The NDK the core's libraries are linked with (android/native/build.sh),
    // pinned: the same version builds the same bytes.
    ndkVersion = providers.gradleProperty("driftloom.ndkVersion").get()

    defaultConfig {
        applicationId = appId
        // 27 (Android 8.1): AAudio, the native audio path (session 26), came
        // in 26 but is only dependable from 27 (it is where Oboe itself
        // starts using it), and a WebView that current runs on 8.1 and up.
        // Below that the page would need a fallback the project has retired.
        minSdk = 27
        targetSdk = 36
        versionCode = providers.gradleProperty("driftloom.versionCode").get().toInt()
        versionName = providers.gradleProperty("driftloom.versionName").get()

        // The instrumented tests (src/androidTest): the core native, on the
        // emulator (session 26).
        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        // The golden loop the debug screen and the tests render again: the
        // first line of core/golden/seeds.hashes, which every target makes
        // and tools/seeds.sh holds to js/dlplayer.wasm. Read from the file,
        // so a sound change that moves it moves this too.
        buildConfigField("String", "GOLDEN_LINE", "\"${goldenLine()}\"")
    }

    buildFeatures {
        buildConfig = true
    }

    // One APK for each ABI, and a universal one with all three (the one to
    // install when unsure). Each carries only its own libdriftloom.so.
    splits {
        abi {
            isEnable = true
            reset()
            include(*abis.toTypedArray())
            isUniversalApk = true
        }
    }

    // Test builds are debug-signed. The key is a throwaway one, kept as text
    // (test-signing/README.md) so each build can be installed over the last
    // and the tree holds no binary but the Gradle wrapper's jar. It signs
    // nothing a listener should trust: a release is signed by another key.
    signingConfigs {
        getByName("debug") {
            val store = layout.buildDirectory.file("test-signing/debug.keystore").get().asFile
            if (!store.exists()) {
                store.parentFile.mkdirs()
                val text = file("../test-signing/debug.keystore.base64").readText()
                store.writeBytes(Base64.getMimeDecoder().decode(text))
            }
            storeFile = store
            storePassword = "android"
            keyAlias = "androiddebugkey"
            keyPassword = "android"
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = true
            isShrinkResources = true
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"))
            // The release is built the same on any machine (session 29, item
            // 107e: docs/notes/107-reproducible.md). AGP 8.3 and later stamp the
            // git commit into the APK unless told not to.
            vcsInfo.include = false
        }
    }

    // Google's encrypted dependency list, which Android Gradle Plugin adds to
    // an APK's signing block and to its bundle: a blob F-Droid refuses (it is
    // closed, and it differs from build to build of the same source).
    dependenciesInfo {
        includeInApk = false
        includeInBundle = false
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    packaging {
        // Nothing here needs the debug symbols' or Kotlin's metadata files.
        resources.excludes += setOf("DebugProbesKt.bin", "kotlin-tooling-metadata.json", "META-INF/*.version")
    }

    // The app's own files, as assets: see bundleApp below.
    sourceSets.getByName("main").assets.srcDir(layout.buildDirectory.dir("generated/bundled-site"))
    // The core's libraries, built from source: see buildNative below.
    sourceSets.getByName("main").jniLibs.srcDir(layout.buildDirectory.dir("generated/native"))
}

kotlin {
    compilerOptions { jvmTarget.set(org.jetbrains.kotlin.gradle.dsl.JvmTarget.JVM_17) }
}

dependencies {
    // WebViewAssetLoader: serves the bundled files to the WebView from one
    // local https origin, so ES modules, the worklet and the .wasm load.
    // Apache-2.0, from Google's Maven; nothing else is depended on.
    implementation("androidx.webkit:webkit:1.14.0")

    // The instrumented tests only (never in the APK): AndroidX Test's runner
    // and JUnit 4 (Apache-2.0 and EPL-1.0, from Google's Maven and Maven
    // Central).
    androidTestImplementation("androidx.test:runner:1.6.2")
    androidTestImplementation("androidx.test.ext:junit:1.2.1")
    androidTestImplementation("junit:junit:4.13.2")
}

// ---- The app's files, from the repo, not committed twice -------------------
//
// The site is plain files (wrangler.jsonc serves the repo root), and sw.js's
// FILES is the list of those the app needs: the one list. This task reads it
// and copies exactly those files, at their own paths, to assets/www/, which
// the page is served from (BundledSite.kt). A new file the page needs goes
// into sw.js's list as it always did, and into the app by that.
val siteRoot: File = rootProject.projectDir.parentFile

fun siteFiles(): List<String> {
    val text = File(siteRoot, "sw.js").readText()
    val block = Regex("""const FILES = \[(.*?)\n\];""", RegexOption.DOT_MATCHES_ALL)
        .find(text)?.groupValues?.get(1)
        ?: throw GradleException("sw.js: no `const FILES = [...]` list found")
    val entries = block.lines()
        .filterNot { it.trim().startsWith("//") }
        .flatMap { Regex("""'\./([^']*)'""").findAll(it).map { m -> m.groupValues[1] }.toList() }
        .map { if (it.isEmpty()) "index.html" else it }
    if (entries.size < 20) throw GradleException("sw.js: only ${entries.size} files listed; the parse is wrong")
    return entries
}

val bundleApp by tasks.registering(Sync::class) {
    group = "driftloom"
    description = "Copies the app's files (sw.js's list) from the repo into assets/www."
    val files = siteFiles()
    inputs.file(File(siteRoot, "sw.js"))
    into(layout.buildDirectory.dir("generated/bundled-site/www"))
    doFirst {
        val missing = files.filterNot { File(siteRoot, it).isFile }
        if (missing.isNotEmpty()) throw GradleException("sw.js lists files that are not in the repo: $missing")
    }
    for (rel in files) {
        from(File(siteRoot, rel)) {
            val dir = rel.substringBeforeLast('/', "")
            if (dir.isNotEmpty()) into(dir)
        }
    }
}

tasks.matching { it.name.startsWith("merge") && it.name.endsWith("Assets") }.configureEach {
    dependsOn(bundleApp)
}

// ---- The core, native, from the same source as the web's module ----------
//
// Session 26 (queue item 107b): android/native is a small crate over the core
// (core/, as a path dependency) that holds what Android needs: the AAudio
// stream, the lock-free queue and the JNI bridge. build.sh builds it with
// cargo for each ABI with the pinned NDK's linkers, into generated/native,
// which the APK's jniLibs come from. Nothing prebuilt is in the tree.
val nativeDir: File = File(siteRoot, "android/native")
val nativeOut = layout.buildDirectory.dir("generated/native")

val buildNative by tasks.registering(Exec::class) {
    group = "driftloom"
    description = "Builds libdriftloom.so for each ABI (android/native/build.sh)."
    val ndk = androidComponents.sdkComponents.ndkDirectory
    inputs.dir(File(siteRoot, "core/src"))
    inputs.dir(File(siteRoot, "data"))
    inputs.files(File(siteRoot, "core/Cargo.toml"), File(siteRoot, "core/Cargo.lock"), File(siteRoot, "core/build.rs"))
    inputs.dir(File(nativeDir, "src"))
    inputs.files(
        File(nativeDir, "Cargo.toml"), File(nativeDir, "Cargo.lock"), File(nativeDir, "rust-toolchain.toml"),
        File(nativeDir, "build.sh"), File(nativeDir, "abi-check.c"),
    )
    inputs.property("abis", abis)
    outputs.dir(nativeOut)
    workingDir(nativeDir)
    doFirst {
        delete(nativeOut)
        commandLine(listOf("sh", "build.sh", nativeOut.get().asFile.path, ndk.get().asFile.path) + abis)
    }
}

tasks.matching { it.name.startsWith("merge") && it.name.endsWith("JniLibFolders") }.configureEach {
    dependsOn(buildNative)
}

// The golden line (BuildConfig.GOLDEN_LINE): the first loop of
// core/golden/seeds.hashes, without its number.
fun goldenLine(): String {
    // From the repo root itself: this runs while `android {}` is read, before
    // siteRoot below is set.
    val line = File(rootProject.projectDir.parentFile, "core/golden/seeds.hashes").readLines().first { it.isNotBlank() && !it.startsWith("#") }
    return line.substringAfter(' ')
}
