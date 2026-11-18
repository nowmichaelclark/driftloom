package app.loops

import android.Manifest
import android.app.Activity
import android.content.ActivityNotFoundException
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.graphics.BitmapFactory
import android.net.Uri
import android.os.Build
import android.os.Handler
import android.os.Looper
import android.os.SystemClock
import android.util.Base64
import android.util.Log
import android.webkit.ValueCallback
import androidx.webkit.JavaScriptReplyProxy
import androidx.webkit.WebViewCompat
import org.json.JSONObject
import java.io.IOException
import java.util.concurrent.Executors

/**
 * What the page asks of the app that a browser would do itself (session 28,
 * queue item 107d): a second bridge beside the core's ([PageBridge]), the
 * object `DriftloomApp` in the page (js/app.js is its end). Plain text both
 * ways, JSON:
 *
 * From the page:
 *  - `hello`;
 *  - `media` { playing, name, subtitle }: what is playing, for the service,
 *    its notification and the lock screen; `cover` { data }: the loop's own
 *    cover as a base64 PNG, drawn by the page (js/cover.js);
 *  - `export` { on, name, percent }: a file is being made, for the
 *    notification and the wake lock;
 *  - `save` { id, name, mime, need, seek }: Android's own picker for a new
 *    file; then `save-write`, `save-header`, `save-close` and `save-abort`
 *    { id, data } (base64), `save-view` and `save-share` for the file saved.
 *
 * To the page: `action` { action } (play, pause, next, previous: what a
 * button on the lock screen or the notification does) and `reply` { id, ok,
 * ... } to each request that has an id.
 *
 * Also here, since they are the same kind of thing: the restore file's picker
 * (the page's `<input type=file>`) and the one question the app asks, whether
 * it may show its notification (Android 13 and later), at the first Play, with
 * the app in front.
 */
class AppBridge(private val activity: Activity, private val view: AppWebView) {
    private val main = Handler(Looper.getMainLooper())

    // Files are written on a thread of their own, one call at a time, in the order they came.
    private val io = Executors.newSingleThreadExecutor { r -> Thread(r, "dl-save") }

    /** The page's end of the bridge: the UI thread's only. */
    private var proxy: JavaScriptReplyProxy? = null

    private var pending: PendingSave? = null
    private var file: SaveFile? = null
    private var saved: Saved? = null
    private var chooser: ValueCallback<Array<Uri>>? = null
    private var lastExportNote = 0L

    /** Whether the activity is in front (the permission's dialog needs it). */
    @Volatile var resumed = false

    private class PendingSave(val id: Int, val mime: String, val need: Long, val seek: Boolean)
    private class Saved(val uri: Uri, val mime: String)

    /** Put `DriftloomApp` in the app's own page (before it loads). */
    fun attach() {
        WebViewCompat.addWebMessageListener(view, NAME, setOf("https://${BundledSite.HOST}")) { _, message, _, isMainFrame, reply ->
            if (!isMainFrame) return@addWebMessageListener
            proxy = reply
            val text = message.data ?: return@addWebMessageListener
            if (isFileCall(text)) {
                // A megabyte of base64: read on the writer's thread, never the UI's.
                io.execute {
                    val m = try { JSONObject(text) } catch (_: Exception) { return@execute }
                    writeTo(m.optInt("id"), m.optString("type"), m.optString("data"))
                }
                return@addWebMessageListener
            }
            val m = try { JSONObject(text) } catch (_: Exception) { return@addWebMessageListener }
            handle(m)
        }
        Playback.toPage = { action ->
            if (proxy == null) {
                false
            } else {
                main.post {
                    Log.i(PlaybackService.TAG, "action $action: posted to the page")
                    proxy?.postMessage("""{"type":"action","action":"$action"}""")
                }
                true
            }
        }
    }

    private fun handle(m: JSONObject) {
        when (m.optString("type")) {
            "hello" -> proxy?.postMessage("""{"type":"hello","sdk":${Build.VERSION.SDK_INT}}""")
            "media" -> {
                val was = Playback.playing
                Playback.playing = m.optBoolean("playing")
                if (m.has("name")) Playback.name = m.optString("name")
                if (m.has("subtitle")) Playback.subtitle = m.optString("subtitle")
                if (Playback.playing && !was) askNotifications()
                // Already paused, and the page says paused a while after the call paused it: the listener's own
                // Pause, which the call's ending must not undo.
                if (!Playback.playing && !was && SystemClock.elapsedRealtime() - Playback.focusPausedAt > 2000) {
                    Playback.cancelResume = true
                }
                PlaybackService.sync(activity)
                // The page runs as in front while the app plays or saves, and is hidden when it doesn't.
                view.refreshVisibility()
            }
            "cover" -> {
                val data = m.optString("data")
                io.execute {
                    val bytes = try { Base64.decode(data, Base64.DEFAULT) } catch (_: IllegalArgumentException) { return@execute }
                    val bitmap = BitmapFactory.decodeByteArray(bytes, 0, bytes.size) ?: return@execute
                    Playback.cover = bitmap
                    main.post { PlaybackService.sync(activity) }
                }
            }
            "export" -> {
                val on = m.optBoolean("on")
                val percent = m.optInt("percent")
                val was = Playback.exporting
                Playback.exporting = on
                Playback.exportName = m.optString("name")
                Playback.exportPercent = percent
                if (on && !was) askNotifications()
                // The notification's bar: about once a second, and at the start and the end.
                val now = System.currentTimeMillis()
                if (on != was || now - lastExportNote >= 1000) {
                    lastExportNote = now
                    PlaybackService.sync(activity)
                }
                if (on != was) view.refreshVisibility()
            }
            "save" -> chooseNewFile(m)
            "save-view" -> sendSaved(open = true, id = m.optInt("id"))
            "save-share" -> sendSaved(open = false, id = m.optInt("id"))
        }
    }

    // ------------------------------------------------------------ saving

    /** A call on the open file: write, header, close or abort (js/app.js puts `type` first). */
    private fun isFileCall(text: String): Boolean =
        text.startsWith("{\"type\":\"save-") && text.length > 14 && text[14] in "whca"

    /** Android's own picker for a new file; the answer comes to [onResult]. */
    private fun chooseNewFile(m: JSONObject) {
        val id = m.optInt("id")
        // One at a time: a file left open (the page went mid-export) is let go first.
        io.execute { file?.discard(); file = null }
        val mime = m.optString("mime").ifEmpty { "application/octet-stream" }
        pending = PendingSave(id, mime, m.optLong("need"), m.optBoolean("seek"))
        val intent = Intent(Intent.ACTION_CREATE_DOCUMENT)
            .addCategory(Intent.CATEGORY_OPENABLE)
            .setType(mime)
            .putExtra(Intent.EXTRA_TITLE, m.optString("name", "driftloom"))
        try {
            @Suppress("DEPRECATION")
            activity.startActivityForResult(intent, REQ_SAVE)
        } catch (_: ActivityNotFoundException) {
            pending = null
            reply(id, false, "why" to "picker")
        }
    }

    /** The picker's answer: the file is opened, its space and its way of being written checked, and the page told. */
    private fun savePicked(resultCode: Int, data: Intent?) {
        val p = pending ?: return
        pending = null
        val uri = data?.data
        if (resultCode != Activity.RESULT_OK || uri == null) {
            reply(p.id, false, "cancelled" to true)
            return
        }
        io.execute {
            val f = try {
                SaveFile(activity.contentResolver, uri, p.mime)
            } catch (e: Exception) {
                SaveFile.remove(activity.contentResolver, uri)
                reply(p.id, false, "why" to "open", "message" to (e.message ?: ""))
                return@execute
            }
            when {
                p.need > 0 && f.free in 0 until p.need -> {
                    f.discard()
                    reply(p.id, false, "why" to "space", "free" to f.free)
                }
                p.seek && !f.seekable -> {
                    f.discard()
                    reply(p.id, false, "why" to "seek")
                }
                else -> {
                    file = f
                    saved = Saved(uri, p.mime)
                    reply(p.id, true, "name" to f.name, "free" to f.free, "seekable" to f.seekable)
                }
            }
        }
    }

    /** One call on the open file (the I/O thread's). */
    private fun writeTo(id: Int, kind: String, data: String) {
        val f = file
        if (f == null) {
            // An abort with nothing open is done already.
            reply(id, kind == "save-abort", "why" to "none")
            return
        }
        try {
            when (kind) {
                "save-write" -> f.write(Base64.decode(data, Base64.DEFAULT))
                "save-header" -> f.header(Base64.decode(data, Base64.DEFAULT))
                "save-close" -> {
                    f.finish()
                    file = null
                }
                else -> {
                    f.discard()
                    file = null
                    saved = null
                }
            }
            reply(id, true, "bytes" to f.bytes)
        } catch (e: IOException) {
            f.discard()
            file = null
            saved = null
            reply(id, false, "why" to (if (SaveFile.isFull(e)) "space" else "write"), "message" to (e.message ?: ""))
        } catch (e: IllegalArgumentException) {
            f.discard()
            file = null
            saved = null
            reply(id, false, "why" to "write", "message" to (e.message ?: ""))
        }
    }

    /** Open the file saved (the app that plays it), or share it. */
    private fun sendSaved(open: Boolean, id: Int) {
        val s = saved
        if (s == null) {
            reply(id, false, "why" to "none")
            return
        }
        val intent = if (open) {
            Intent(Intent.ACTION_VIEW).setDataAndType(s.uri, s.mime).addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
        } else {
            val send = Intent(Intent.ACTION_SEND).setType(s.mime).putExtra(Intent.EXTRA_STREAM, s.uri)
                .addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
            Intent.createChooser(send, null).addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
        }
        try {
            activity.startActivity(intent)
            reply(id, true)
        } catch (_: ActivityNotFoundException) {
            reply(id, false, "why" to "nothing opens it")
        }
    }

    // ----------------------------------------------------- the restore file

    /** The page's file input (restore a backup): Android's picker. */
    fun chooseFile(callback: ValueCallback<Array<Uri>>, types: Array<String>): Boolean {
        chooser?.onReceiveValue(null)
        chooser = callback
        val intent = Intent(Intent.ACTION_OPEN_DOCUMENT)
            .addCategory(Intent.CATEGORY_OPENABLE)
            .setType("*/*")
        val known = types.filter { it.isNotBlank() && it.contains('/') }
        if (known.isNotEmpty()) intent.putExtra(Intent.EXTRA_MIME_TYPES, known.toTypedArray())
        return try {
            @Suppress("DEPRECATION")
            activity.startActivityForResult(intent, REQ_OPEN)
            true
        } catch (_: ActivityNotFoundException) {
            chooser = null
            callback.onReceiveValue(null)
            false
        }
    }

    // ------------------------------------------------------- notifications

    /**
     * Android 13 and later: the notification needs the listener's yes. Asked once, at the first Play (or the first
     * export) with the app in front; refused, the app plays as it did, and the lock screen's controls still work.
     */
    private fun askNotifications() {
        if (Build.VERSION.SDK_INT < 33 || !resumed) return
        if (activity.checkSelfPermission(Manifest.permission.POST_NOTIFICATIONS) == PackageManager.PERMISSION_GRANTED) return
        val prefs = activity.getSharedPreferences("app", Context.MODE_PRIVATE)
        if (prefs.getBoolean(ASKED, false)) return
        prefs.edit().putBoolean(ASKED, true).apply()
        activity.requestPermissions(arrayOf(Manifest.permission.POST_NOTIFICATIONS), REQ_NOTES)
    }

    // ------------------------------------------------------ the activity's

    fun onResult(requestCode: Int, resultCode: Int, data: Intent?) {
        when (requestCode) {
            REQ_SAVE -> savePicked(resultCode, data)
            REQ_OPEN -> {
                val callback = chooser ?: return
                chooser = null
                val uri = data?.data
                callback.onReceiveValue(if (resultCode == Activity.RESULT_OK && uri != null) arrayOf(uri) else null)
            }
        }
    }

    fun onPermissions(requestCode: Int) {
        if (requestCode == REQ_NOTES) PlaybackService.sync(activity)
    }

    /** The page is going (a reload, the activity closing): nothing it started goes on. */
    fun pageGone() {
        proxy = null
        pending = null
        Playback.reset()
        view.refreshVisibility()
        io.execute { file?.discard(); file = null; saved = null }
        PlaybackService.end(activity)
    }

    fun close() {
        Playback.toPage = null
        pageGone()
        chooser?.onReceiveValue(null)
        chooser = null
        io.shutdown()
    }

    private fun reply(id: Int, ok: Boolean, vararg more: Pair<String, Any>) {
        val o = JSONObject().put("type", "reply").put("id", id).put("ok", ok)
        for ((k, v) in more) o.put(k, v)
        val text = o.toString()
        main.post { proxy?.postMessage(text) }
    }

    companion object {
        /** The object's name in the page (js/app.js reads it). */
        const val NAME = "DriftloomApp"

        const val REQ_SAVE = 41
        const val REQ_OPEN = 42
        const val REQ_NOTES = 43
        private const val ASKED = "asked_notifications"
    }
}
