package app.loops

import android.content.Context
import android.view.View
import android.webkit.WebView

/**
 * The app's WebView, which the page keeps running in while it has work to do
 * with the screen off (session 28, queue item 107d).
 *
 * The core plays on its own thread and needs nothing from the page, but the
 * lock screen's and the notification's buttons are the page's (Next and
 * Previous are its history, its albums, its choice of the next loop), and so
 * is an export's Worker and the hand-over at the end of a track. A WebView
 * whose window is not shown is a hidden page to Chromium, which slows its
 * timers to one a second and, after some minutes, to one a minute: measured on
 * the emulator, a 100 ms timer in the page never fired in a minute after ten
 * minutes with the screen off, and a button pressed then did nothing. Playing
 * audio of its own is what exempts a page, and this page plays none (the core
 * is native).
 *
 * So while the app plays or saves ([Playback.busy]) this tells the WebView its
 * window and view are shown, whatever they are, and so the page runs as it
 * does in front. Nothing is drawn: a window not shown has no surface to draw
 * to, and the WebView draws only when asked to (the emulator's CPU figure with
 * the screen off is in the test's summary). Paused and idle, the page is
 * hidden as any page is (the real visibility is kept and given back).
 */
class AppWebView(context: Context) : WebView(context) {
    private var windowVisibility = View.VISIBLE
    private var viewVisibility = View.VISIBLE

    override fun onWindowVisibilityChanged(visibility: Int) {
        windowVisibility = visibility
        super.onWindowVisibilityChanged(shown(visibility))
    }

    override fun onVisibilityChanged(changedView: View, visibility: Int) {
        if (changedView === this) viewVisibility = visibility
        super.onVisibilityChanged(changedView, shown(visibility))
    }

    /** The rule moved (playing or saving began or ended): tell the WebView again, as it is now. */
    fun refreshVisibility() {
        try {
            onWindowVisibilityChanged(windowVisibility)
            onVisibilityChanged(this, viewVisibility)
        } catch (_: RuntimeException) {
            // A WebView being torn down has nothing to show.
        }
    }

    // A window whose activity has stopped is GONE to its views (Home, the screen off): measured, the page was
    // hidden and its timers stopped for minutes with the rule that left GONE alone. While there is work to
    // do, every state is shown.
    private fun shown(visibility: Int): Int = if (Playback.busy) View.VISIBLE else visibility
}
