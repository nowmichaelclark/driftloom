package app.loops

import android.graphics.Bitmap
import android.os.Handler
import android.os.HandlerThread
import android.util.Log

/**
 * What the page has told the app about what is playing and what is being
 * saved (session 28, queue item 107d): the one place the service, the bridge
 * and the activity read it. The page sets it through [AppBridge]; the
 * service ([PlaybackService]) shows it (the notification, the lock screen's
 * session) and holds what the app needs to go on with the screen off (the
 * foreground service, the wake lock, the audio focus) only while it says so.
 */
object Playback {
    /** The page's player is playing. */
    @Volatile var playing = false
    /** The loop's name, and the line under it (its tempo). */
    @Volatile var name = ""
    @Volatile var subtitle = ""
    /** The loop's own cover, as the page draws it (js/cover.js), or null before the first. */
    @Volatile var cover: Bitmap? = null

    /** A file is being made and saved (js/export.js), with its name and how far it is, 0 to 100. */
    @Volatile var exporting = false
    @Volatile var exportName = ""
    @Volatile var exportPercent = 0

    /**
     * Gives an action ("play", "pause", "next", "previous") to the page, which does what its own button
     * does; false if there is no page to give it to. Set by [AppBridge] while a page is there.
     */
    @Volatile var toPage: ((String) -> Boolean)? = null

    /** When the service paused the music for a call (the clock's, in ms): the page's own pause then is the listener's. */
    @Volatile var focusPausedAt = 0L

    /** The listener paused while a call had the music paused: it is not to start again when the call ends. */
    @Volatile var cancelResume = false

    /** Whether the service has anything to hold on for. */
    val busy get() = playing || exporting

    /** Nothing playing, nothing saving, nothing to show (the page went, or the app is closing). */
    fun reset() {
        playing = false
        exporting = false
        exportName = ""
        exportPercent = 0
        name = ""
        subtitle = ""
        cover = null
    }
}

/**
 * Keeps the native stream running only while something needs it (session 28:
 * nothing runs when paused). It runs while the page is playing and for a
 * ring-out after a pause (what was played rings out: the reverb's tail), and
 * a stream that was stopped starts again at the next Play, or at any message
 * the page sends the core in the meantime (a loop picked while paused, a
 * probe), so a command is never left in the queue with nothing to take it.
 *
 * Every call here is queued to a thread of its own: opening a stream takes
 * tens of milliseconds, and the UI thread is not to spend them.
 */
object StreamKeeper {
    /** How long the stream runs on after a pause: the longest tail the core leaves (docs/CORE.md). */
    const val RING_OUT_MS = 8_000L

    private val thread = HandlerThread("dl-stream").apply { start() }
    private val handler = Handler(thread.looper)

    @Volatile private var wanted = false
    private val stop = Runnable {
        if (!wanted) {
            NativeCore.stop()
            Log.i(TAG, "stream stopped (nothing plays)")
        }
    }

    /** The page plays (true) or has paused (false). */
    fun playing(on: Boolean) {
        wanted = on
        handler.post {
            handler.removeCallbacks(stop)
            if (on) start() else handler.postDelayed(stop, RING_OUT_MS)
        }
    }

    /**
     * The page sent the core a message while paused: the stream is there to take it, for a ring-out's time.
     * Never before the engine exists (the page's own `init` makes it, at its own quality).
     */
    fun touched() {
        if (wanted) return
        handler.post {
            if (wanted || !NativeCore.figures().engine) return@post
            handler.removeCallbacks(stop)
            if (!NativeCore.figures().running) start()
            handler.postDelayed(stop, RING_OUT_MS)
        }
    }

    /** The activity is closing: the stream goes now. */
    fun close() {
        wanted = false
        handler.post {
            handler.removeCallbacks(stop)
            NativeCore.stop()
        }
    }

    private fun start() {
        if (NativeCore.figures().running) return
        // The engine exists once the page has played (its `init` makes it); before that the page's own start
        // opens the stream, and this has nothing to restart.
        if (NativeCore.figures().engine) {
            val why = NativeCore.start(true)
            Log.i(TAG, if (why == null) "stream started" else "stream did not start: $why")
        }
    }

    private const val TAG = "DriftloomStream"
}
