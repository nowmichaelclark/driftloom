package app.loops

import android.content.Context
import android.webkit.WebResourceResponse
import androidx.webkit.WebViewAssetLoader
import java.io.IOException

/**
 * Serves the app's own files (assets/www, copied from the repo by the
 * `bundleApp` task) to the WebView, under one local https origin.
 *
 * WebViewAssetLoader's own AssetsPathHandler guesses a file's type from its
 * name, and the page needs them exact: a module script served as text/plain
 * does not run, a .wasm needs application/wasm to be compiled while it
 * streams, and the audio worklet is a module too. So the types are listed.
 */
class BundledSite(private val context: Context) : WebViewAssetLoader.PathHandler {

    override fun handle(path: String): WebResourceResponse? {
        val name = path.trimStart('/').ifEmpty { "index.html" }
        // The path is the page's to choose: never let it climb out of www.
        if (name.split('/').any { it == ".." || it.isEmpty() }) return notFound()
        return try {
            val stream = context.assets.open("www/$name")
            WebResourceResponse(typeOf(name), encodingOf(name), stream)
        } catch (e: IOException) {
            notFound()
        }
    }

    private fun notFound() = WebResourceResponse(
        "text/plain", "utf-8", 404, "Not Found", emptyMap(), "Not found".byteInputStream(),
    )

    companion object {
        /** The origin the page lives at: its saved loops are stored under
         *  it, so it never changes once listeners have used it. */
        const val HOST = "appassets.androidplatform.net"
        const val START = "https://$HOST/"

        private val TYPES = mapOf(
            "html" to "text/html",
            "js" to "text/javascript",
            "mjs" to "text/javascript",
            "css" to "text/css",
            "txt" to "text/plain",
            "json" to "application/json",
            "webmanifest" to "application/manifest+json",
            "wasm" to "application/wasm",
            "png" to "image/png",
            "woff2" to "font/woff2",
            "flac" to "audio/flac",
            "wav" to "audio/wav",
        )

        fun typeOf(name: String): String =
            TYPES[name.substringAfterLast('.', "").lowercase()] ?: "application/octet-stream"

        private fun encodingOf(name: String): String? =
            if (typeOf(name).startsWith("text/") || typeOf(name).endsWith("json")) "utf-8" else null
    }
}
