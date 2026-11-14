import java.util.Base64

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

val appId = providers.gradleProperty("driftloom.applicationId").get()
val appName = providers.gradleProperty("driftloom.appName").get()

base.archivesName.set(appName.lowercase())

android {
    // The Kotlin package of the code, not the app's id: it stays when the
    // id changes at the rename (session 31a).
    namespace = "app.loops"
    compileSdk = 36

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
        }
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
}

kotlin {
    compilerOptions { jvmTarget.set(org.jetbrains.kotlin.gradle.dsl.JvmTarget.JVM_17) }
}

dependencies {
    // WebViewAssetLoader: serves the bundled files to the WebView from one
    // local https origin, so ES modules, the worklet and the .wasm load.
    // Apache-2.0, from Google's Maven; nothing else is depended on.
    implementation("androidx.webkit:webkit:1.14.0")
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
