package app.loops

import android.app.Activity
import android.content.ActivityNotFoundException
import android.content.Intent
import android.content.pm.ApplicationInfo
import android.graphics.Color
import android.os.Build
import android.os.Bundle
import android.view.ViewGroup
import android.view.WindowInsets
import android.webkit.RenderProcessGoneDetail
import android.webkit.WebResourceRequest
import android.webkit.WebResourceResponse
import android.webkit.WebView
import android.webkit.WebViewClient
import android.widget.FrameLayout
import androidx.webkit.WebViewAssetLoader

/**
 * The whole app for now: one WebView with the page in it, as it plays in a
 * browser (its own worklet, in the WebView). Session 26 puts the core's
 * native audio under it; this is the shell.
 */
class MainActivity : Activity() {

    private var web: WebView? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // A debuggable (test) build can be inspected from a computer over
        // USB, which is what the emulator smoke test does. A release cannot.
        if (applicationInfo.flags and ApplicationInfo.FLAG_DEBUGGABLE != 0) {
            WebView.setWebContentsDebuggingEnabled(true)
        }

        val root = FrameLayout(this)
        root.setBackgroundColor(Color.parseColor("#160830"))
        // From Android 15 the window runs under the system bars: the page is
        // kept inside them (and above the keyboard), and the bars show the
        // theme's dark behind them.
        root.setOnApplyWindowInsetsListener { v, insets ->
            if (Build.VERSION.SDK_INT >= 30) {
                val i = insets.getInsets(
                    WindowInsets.Type.systemBars() or WindowInsets.Type.displayCutout() or WindowInsets.Type.ime()
                )
                v.setPadding(i.left, i.top, i.right, i.bottom)
                WindowInsets.CONSUMED
            } else {
                @Suppress("DEPRECATION")
                v.setPadding(
                    insets.systemWindowInsetLeft, insets.systemWindowInsetTop,
                    insets.systemWindowInsetRight, insets.systemWindowInsetBottom,
                )
                @Suppress("DEPRECATION")
                insets.consumeSystemWindowInsets()
            }
        }
        setContentView(root)
        attachPage(root)
    }

    private fun attachPage(root: ViewGroup) {
        val loader = WebViewAssetLoader.Builder()
            .setDomain(BundledSite.HOST)
            .addPathHandler("/", BundledSite(this))
            .build()

        val view = WebView(this)
        view.settings.apply {
            javaScriptEnabled = true
            // The page keeps the loops, the library and the preferences in
            // local storage.
            domStorageEnabled = true
            allowFileAccess = false
            allowContentAccess = false
            // The app has no permissions, so no network: the page is its own
            // files and nothing else.
            blockNetworkLoads = true
            @Suppress("DEPRECATION")
            safeBrowsingEnabled = false
        }
        view.setBackgroundColor(Color.parseColor("#160830"))
        view.webViewClient = object : WebViewClient() {
            override fun shouldInterceptRequest(v: WebView, request: WebResourceRequest): WebResourceResponse? =
                loader.shouldInterceptRequest(request.url)

            override fun shouldOverrideUrlLoading(v: WebView, request: WebResourceRequest): Boolean {
                if (request.url.host == BundledSite.HOST) return false
                // A link out of the app opens in whatever the listener uses.
                try {
                    startActivity(Intent(Intent.ACTION_VIEW, request.url))
                } catch (_: ActivityNotFoundException) {
                }
                return true
            }

            // WebView's renderer was killed (memory): the page is rebuilt.
            override fun onRenderProcessGone(v: WebView, detail: RenderProcessGoneDetail): Boolean {
                (v.parent as? ViewGroup)?.removeView(v)
                v.destroy()
                web = null
                recreate()
                return true
            }
        }
        root.addView(view, FrameLayout.LayoutParams(ViewGroup.LayoutParams.MATCH_PARENT, ViewGroup.LayoutParams.MATCH_PARENT))
        web = view
        view.loadUrl(BundledSite.START)
    }

    override fun onDestroy() {
        web?.let { (it.parent as? ViewGroup)?.removeView(it); it.destroy() }
        web = null
        super.onDestroy()
    }
}
