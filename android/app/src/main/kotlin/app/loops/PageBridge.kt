package app.loops

import android.os.Handler
import android.os.HandlerThread
import android.os.Looper
import android.webkit.WebView
import androidx.webkit.JavaScriptReplyProxy
import androidx.webkit.WebViewCompat
import androidx.webkit.WebViewFeature

/**
 * The page talks to the native core (session 27, queue item 107c). The page's
 * player (js/core.js's CoreHost) posts the worklet's messages to an object the
 * WebView puts in the page, `DriftloomNative` (js/native.js), and hears the
 * worklet's messages back from it: here they go to the core's bridge
 * (android/native/src/bridge.rs) and back. The WebView plays no audio of its
 * own; the core plays on Android's, at high priority, and needs nothing from
 * the page to go on (docs/CORE.md, promise 1).
 *
 * One thread of its own carries every call into the core (a message in, and
 * every 25 ms what is due back out), so the UI thread only hands text over;
 * the reply proxy is the UI thread's, as the WebView asks.
 */
class PageBridge(private val view: WebView) {
    private val main = Handler(Looper.getMainLooper())
    private val thread = HandlerThread("dl-bridge").apply { start() }
    private val bridge = Handler(thread.looper)

    /** The page's end of the bridge: the UI thread's only. */
    private var proxy: JavaScriptReplyProxy? = null

    private val drain = object : Runnable {
        override fun run() {
            NativeCore.drain()?.let { post(it) }
            bridge.postDelayed(this, DRAIN_MS)
        }
    }

    /** Put `DriftloomNative` in the app's own page (before it loads), and start draining. */
    fun attach() {
        WebViewCompat.addWebMessageListener(view, NAME, setOf("https://${BundledSite.HOST}")) { _, message, _, isMainFrame, reply ->
            if (!isMainFrame) return@addWebMessageListener
            proxy = reply
            val text = message.data ?: return@addWebMessageListener
            // Anything sent to the core while paused finds the stream there to take it (session 28).
            StreamKeeper.touched()
            bridge.post { NativeCore.send(text)?.let { post(it) } }
        }
        bridge.post(drain)
    }

    /** A batch for the page, in order, from the bridge thread. */
    private fun post(text: String) {
        main.post { proxy?.postMessage(text) }
    }

    /** The page is going (a reload, the activity closing): its player is let go. */
    fun pageGone() {
        proxy = null
        bridge.post { NativeCore.pageGone() }
    }

    /** The activity is gone: the page's player let go and the stream closed. */
    fun close() {
        proxy = null
        bridge.removeCallbacksAndMessages(null)
        StreamKeeper.close()
        bridge.post {
            NativeCore.pageGone()
            NativeCore.stop()
        }
        thread.quitSafely()
    }

    companion object {
        /** The object's name in the page (js/native.js reads it). */
        const val NAME = "DriftloomNative"

        /** How often what is due goes back to the page: the worklet posts every 50 ms of audio. */
        const val DRAIN_MS = 25L

        /** Whether this WebView can carry the bridge; without it the page plays as in a browser. */
        fun supported(): Boolean = WebViewFeature.isFeatureSupported(WebViewFeature.WEB_MESSAGE_LISTENER)
    }
}
