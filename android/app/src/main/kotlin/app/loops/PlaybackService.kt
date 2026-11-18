package app.loops

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.app.Service
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.content.pm.ServiceInfo
import android.graphics.drawable.Icon
import android.media.AudioAttributes
import android.media.AudioFocusRequest
import android.media.AudioManager
import android.media.MediaMetadata
import android.media.session.MediaSession
import android.media.session.PlaybackState
import android.os.Build
import android.os.Handler
import android.os.IBinder
import android.os.Looper
import android.os.PowerManager
import android.os.SystemClock
import android.util.Log

/**
 * The foreground service that lets the music play on with the screen off and
 * the app in the background (session 28, queue item 107d). The core plays on
 * its own thread, natively, and needs nothing from the page to go on
 * (docs/CORE.md, promise 1); this holds what Android asks of an app that
 * plays in the background, and only while it plays:
 *
 *  - the foreground service of the media playback kind, with its notification
 *    (a media style one, with the loop's own cover);
 *  - a [MediaSession], so the lock screen, the notification, the headset's
 *    buttons and the Bluetooth transport reach Play, Pause, Next and Previous;
 *  - audio focus: a transient loss (a call) pauses, and the music returns
 *    with the focus; a lasting loss pauses for good; another app's short
 *    sound ducks it, the system doing that for a stream like this one;
 *  - the headphones pulled (`ACTION_AUDIO_BECOMING_NOISY`): paused;
 *  - a partial wake lock, held only while the music plays or a file is being
 *    saved, so the page's work (the next loop's choice, an export) is not put
 *    to sleep with the screen.
 *
 * Paused, the service holds nothing: no wake lock, no focus, no stream
 * ([StreamKeeper]); its notification stays (it can be swiped away) with a Play
 * button for fifteen minutes, then goes. What happens when a button is
 * pressed is the page's: the action goes to it ([Playback.toPage]) and it
 * does what its own button does, so the page, the notification and the
 * lock screen are never out of step. With no page there is nothing to play:
 * the service stops.
 */
class PlaybackService : Service() {
    private val main = Handler(Looper.getMainLooper())
    private lateinit var session: MediaSession
    private lateinit var notes: NotificationManager
    private lateinit var audio: AudioManager
    private lateinit var power: PowerManager
    private var wake: PowerManager.WakeLock? = null
    private var foreground = false
    private var focus: AudioFocusRequest? = null
    private var haveFocus = false
    private var resumeOnFocus = false
    private var noisy = false

    // Back to the lock screen's and the notification's buttons: what the page does.
    private val callback = object : MediaSession.Callback() {
        override fun onPlay() = act(ACTION_PLAY)
        override fun onPause() = act(ACTION_PAUSE)
        override fun onStop() = act(ACTION_PAUSE)
        override fun onSkipToNext() = act(ACTION_NEXT)
        override fun onSkipToPrevious() = act(ACTION_PREVIOUS)
    }

    private val focusChanged = AudioManager.OnAudioFocusChangeListener { change ->
        when (change) {
            AudioManager.AUDIOFOCUS_LOSS -> {
                // Another app took the audio for good: pause, and don't take it back unasked.
                resumeOnFocus = false
                pauseNow()
                dropFocus()
            }
            AudioManager.AUDIOFOCUS_LOSS_TRANSIENT -> {
                // A call, an alarm: pause, and play again when it is over.
                if (Playback.playing) {
                    resumeOnFocus = true
                    pauseNow()
                }
            }
            AudioManager.AUDIOFOCUS_GAIN -> if (resumeOnFocus) {
                resumeOnFocus = false
                act(ACTION_PLAY)
            }
            // AUDIOFOCUS_LOSS_TRANSIENT_CAN_DUCK is never sent: the request does not ask to pause when ducked,
            // so the system lowers the stream's volume itself and restores it.
        }
    }

    private val becomingNoisy = object : BroadcastReceiver() {
        override fun onReceive(context: Context, intent: Intent) {
            // The headphones came out (or the Bluetooth went): the speaker must not suddenly play for the room.
            if (intent.action == AudioManager.ACTION_AUDIO_BECOMING_NOISY && Playback.playing) {
                resumeOnFocus = false
                pauseNow()
            }
        }
    }

    private val idleStop = Runnable { shutdown() }
    private val heartbeat = object : Runnable {
        override fun run() {
            // A test build says once a minute what the audio thread is doing, so a run with the screen off can
            // be read afterward (android/background.mjs).
            if (BuildConfig.DEBUG && Playback.playing) {
                Log.i(TAG, "heartbeat ${NativeCore.figures()}")
            }
            if (wake?.isHeld == true) holdWake()
            main.postDelayed(this, HEARTBEAT_MS)
        }
    }

    override fun onCreate() {
        super.onCreate()
        alive = true
        notes = getSystemService(NotificationManager::class.java)
        audio = getSystemService(AudioManager::class.java)
        power = getSystemService(PowerManager::class.java)
        notes.createNotificationChannel(
            NotificationChannel(CHANNEL, getString(R.string.channel_playback), NotificationManager.IMPORTANCE_LOW).apply {
                description = getString(R.string.channel_playback_about)
                setShowBadge(false)
            },
        )
        session = MediaSession(this, "Driftloom").apply {
            setCallback(callback, main)
            setSessionActivity(openApp())
        }
        main.postDelayed(heartbeat, HEARTBEAT_MS)
    }

    override fun onBind(intent: Intent?): IBinder? = null

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        val action = intent?.action ?: ""
        when (action) {
            ACTION_PLAY, ACTION_PAUSE, ACTION_NEXT, ACTION_PREVIOUS -> act(action)
            ACTION_STOP -> shutdown()
            else -> refresh()
        }
        return START_NOT_STICKY
    }

    /** The user's swipe from the recents: the page goes with the task, and so does the music. */
    override fun onTaskRemoved(rootIntent: Intent?) {
        shutdown()
    }

    override fun onDestroy() {
        main.removeCallbacksAndMessages(null)
        release()
        if (this::session.isInitialized) {
            session.isActive = false
            session.release()
        }
        alive = false
        super.onDestroy()
    }

    // ------------------------------------------------------------- actions

    /** A button was pressed: the page does it. With no page the service has nothing to play, and ends. */
    private fun act(action: String) {
        // The listener's own Pause (the notification's, the lock screen's, a headset's) is final: a call's end
        // does not start the music again.
        if (action == ACTION_PAUSE) resumeOnFocus = false
        val name = when (action) {
            ACTION_PLAY -> "play"
            ACTION_PAUSE -> "pause"
            ACTION_NEXT -> "next"
            else -> "previous"
        }
        val given = Playback.toPage?.invoke(name) == true
        Log.i(TAG, "action $name: ${if (given) "given to the page" else "no page"}")
        if (!given) shutdown()
    }

    /**
     * Silence at once, without waiting for the page: the core stops deciding steps, and the page is told so its
     * button and its player agree (its own pause does the same again, harmlessly).
     */
    private fun pauseNow() {
        Playback.focusPausedAt = SystemClock.elapsedRealtime()
        NativeCore.pause()
        Playback.playing = false
        Playback.toPage?.invoke("pause")
        refresh()
    }

    // --------------------------------------------------------------- state

    /** Show what [Playback] says, and hold what it needs held (and nothing else). */
    private fun refresh() {
        if (Playback.cancelResume) {
            Playback.cancelResume = false
            resumeOnFocus = false
        }
        val playing = Playback.playing
        val busy = Playback.busy
        session.isActive = true
        session.setMetadata(
            MediaMetadata.Builder()
                .putString(MediaMetadata.METADATA_KEY_TITLE, Playback.name.ifEmpty { getString(R.string.app_name) })
                .putString(MediaMetadata.METADATA_KEY_ARTIST, Playback.subtitle)
                .putString(MediaMetadata.METADATA_KEY_ALBUM, getString(R.string.app_name))
                .apply { Playback.cover?.let { putBitmap(MediaMetadata.METADATA_KEY_ALBUM_ART, it) } }
                .build(),
        )
        session.setPlaybackState(
            PlaybackState.Builder()
                .setActions(
                    PlaybackState.ACTION_PLAY or PlaybackState.ACTION_PAUSE or PlaybackState.ACTION_PLAY_PAUSE or
                        PlaybackState.ACTION_SKIP_TO_NEXT or PlaybackState.ACTION_SKIP_TO_PREVIOUS or PlaybackState.ACTION_STOP,
                )
                .setState(
                    if (playing) PlaybackState.STATE_PLAYING else PlaybackState.STATE_PAUSED,
                    PlaybackState.PLAYBACK_POSITION_UNKNOWN,
                    if (playing) 1f else 0f,
                )
                .build(),
        )

        if (playing) {
            if (!takeFocus()) {
                // Not given the audio (a call is on): stay paused; the page is told.
                resumeOnFocus = true
                pauseNow()
                return
            }
            watchNoisy(true)
        } else {
            watchNoisy(false)
            if (!resumeOnFocus) dropFocus()
        }
        StreamKeeper.playing(playing)

        if (busy) holdWake() else dropWake()

        val note = notification()
        if (busy) {
            main.removeCallbacks(idleStop)
            if (!foreground) {
                try {
                    if (Build.VERSION.SDK_INT >= 29) {
                        val type = if (playing) ServiceInfo.FOREGROUND_SERVICE_TYPE_MEDIA_PLAYBACK else ServiceInfo.FOREGROUND_SERVICE_TYPE_DATA_SYNC
                        startForeground(NOTE_ID, note, type)
                    } else {
                        startForeground(NOTE_ID, note)
                    }
                    foreground = true
                } catch (e: RuntimeException) {
                    // Not allowed to start from where the app is (Android 12 and later): the music plays on, and
                    // the service goes on as it is.
                    Log.w(TAG, "no foreground service: ${e.message}")
                    notes.notify(NOTE_ID, note)
                }
            } else if (Build.VERSION.SDK_INT >= 29) {
                // Playing and saving change which kind it is.
                try {
                    val type = if (playing) ServiceInfo.FOREGROUND_SERVICE_TYPE_MEDIA_PLAYBACK else ServiceInfo.FOREGROUND_SERVICE_TYPE_DATA_SYNC
                    startForeground(NOTE_ID, note, type)
                } catch (e: RuntimeException) {
                    notes.notify(NOTE_ID, note)
                }
            } else {
                notes.notify(NOTE_ID, note)
            }
        } else {
            // Paused: the service is no longer foreground; the notification stays, with Play, and can be
            // swiped away. After a while with nothing to do the service ends.
            if (foreground) {
                stopForeground(STOP_FOREGROUND_DETACH)
                foreground = false
            }
            notes.notify(NOTE_ID, note)
            main.removeCallbacks(idleStop)
            main.postDelayed(idleStop, IDLE_MS)
        }
    }

    /** The notification for what is on now. */
    private fun notification(): Notification {
        val playing = Playback.playing
        val exportOnly = Playback.exporting && !playing && Playback.name.isEmpty()
        val b = Notification.Builder(this, CHANNEL)
            .setSmallIcon(R.drawable.ic_stat_driftloom)
            .setContentIntent(openApp())
            .setVisibility(Notification.VISIBILITY_PUBLIC)
            .setOnlyAlertOnce(true)
            .setShowWhen(false)
            .setCategory(if (Playback.exporting && !playing) Notification.CATEGORY_PROGRESS else Notification.CATEGORY_TRANSPORT)
            .setOngoing(playing || Playback.exporting)
        if (exportOnly) {
            b.setContentTitle(getString(R.string.saving, Playback.exportName))
                .setProgress(100, Playback.exportPercent, Playback.exportPercent <= 0)
            return b.build()
        }
        b.setContentTitle(Playback.name.ifEmpty { getString(R.string.app_name) })
            .setContentText(Playback.subtitle)
            .apply { Playback.cover?.let { setLargeIcon(it) } }
            .addAction(action(R.drawable.ic_media_previous, R.string.previous, ACTION_PREVIOUS, 1))
            .addAction(
                if (playing) action(R.drawable.ic_media_pause, R.string.pause, ACTION_PAUSE, 2)
                else action(R.drawable.ic_media_play, R.string.play, ACTION_PLAY, 3),
            )
            .addAction(action(R.drawable.ic_media_next, R.string.next, ACTION_NEXT, 4))
            .setStyle(Notification.MediaStyle().setMediaSession(session.sessionToken).setShowActionsInCompactView(0, 1, 2))
        if (Playback.exporting) {
            b.setSubText(getString(R.string.saving_percent, Playback.exportPercent))
        }
        return b.build()
    }

    private fun action(icon: Int, label: Int, action: String, code: Int): Notification.Action =
        Notification.Action.Builder(Icon.createWithResource(this, icon), getString(label), service(action, code)).build()

    private fun service(action: String, code: Int): PendingIntent =
        PendingIntent.getService(
            this, code, Intent(this, PlaybackService::class.java).setAction(action),
            PendingIntent.FLAG_IMMUTABLE or PendingIntent.FLAG_UPDATE_CURRENT,
        )

    private fun openApp(): PendingIntent =
        PendingIntent.getActivity(
            this, 0,
            Intent(this, MainActivity::class.java).addFlags(Intent.FLAG_ACTIVITY_SINGLE_TOP or Intent.FLAG_ACTIVITY_REORDER_TO_FRONT),
            PendingIntent.FLAG_IMMUTABLE or PendingIntent.FLAG_UPDATE_CURRENT,
        )

    // --------------------------------------------------------------- focus

    private fun takeFocus(): Boolean {
        if (haveFocus) return true
        val request = focus ?: AudioFocusRequest.Builder(AudioManager.AUDIOFOCUS_GAIN)
            .setAudioAttributes(
                AudioAttributes.Builder()
                    .setUsage(AudioAttributes.USAGE_MEDIA)
                    .setContentType(AudioAttributes.CONTENT_TYPE_MUSIC)
                    .build(),
            )
            .setOnAudioFocusChangeListener(focusChanged, main)
            .build()
            .also { focus = it }
        haveFocus = audio.requestAudioFocus(request) == AudioManager.AUDIOFOCUS_REQUEST_GRANTED
        return haveFocus
    }

    private fun dropFocus() {
        if (!haveFocus) return
        focus?.let { audio.abandonAudioFocusRequest(it) }
        haveFocus = false
    }

    private fun watchNoisy(on: Boolean) {
        if (on == noisy) return
        noisy = on
        if (on) registerReceiver(becomingNoisy, IntentFilter(AudioManager.ACTION_AUDIO_BECOMING_NOISY))
        else unregisterReceiver(becomingNoisy)
    }

    // ----------------------------------------------------------- wake lock

    // Held while the music plays or a file is saved, with a timeout it renews (the heartbeat) while that goes
    // on, so a stuck app can't hold the phone awake for good.
    private fun holdWake() {
        val lock = wake ?: power.newWakeLock(PowerManager.PARTIAL_WAKE_LOCK, "Driftloom:playing").also {
            it.setReferenceCounted(false)
            wake = it
        }
        lock.acquire(WAKE_MS)
    }

    private fun dropWake() {
        wake?.let { if (it.isHeld) it.release() }
    }

    // ------------------------------------------------------------ the end

    /** Everything let go: no page, nothing playing, nothing saved, or the task removed. */
    private fun shutdown() {
        resumeOnFocus = false
        Playback.playing = false
        Playback.exporting = false
        release()
        stopForeground(STOP_FOREGROUND_REMOVE)
        foreground = false
        notes.cancel(NOTE_ID)
        stopSelf()
    }

    private fun release() {
        main.removeCallbacks(idleStop)
        if (noisy) {
            noisy = false
            try { unregisterReceiver(becomingNoisy) } catch (_: IllegalArgumentException) { }
        }
        dropFocus()
        dropWake()
        StreamKeeper.playing(false)
    }

    companion object {
        const val ACTION_PLAY = "app.loops.action.PLAY"
        const val ACTION_PAUSE = "app.loops.action.PAUSE"
        const val ACTION_NEXT = "app.loops.action.NEXT"
        const val ACTION_PREVIOUS = "app.loops.action.PREVIOUS"
        const val ACTION_STOP = "app.loops.action.STOP"
        const val ACTION_REFRESH = "app.loops.action.REFRESH"

        const val TAG = "DriftloomService"
        private const val CHANNEL = "playback"
        private const val NOTE_ID = 1
        /** Paused, the notification and the session stay this long. */
        private const val IDLE_MS = 15 * 60 * 1000L
        private const val HEARTBEAT_MS = 60 * 1000L
        /** The wake lock's timeout, renewed every heartbeat while it is wanted. */
        private const val WAKE_MS = 3 * 60 * 1000L

        /** The service is running (so a change while paused can be shown without starting one). */
        @Volatile var alive = false

        /** Tell the service what [Playback] says now: starting it if something is playing or being saved. */
        fun sync(context: Context) {
            val intent = Intent(context, PlaybackService::class.java).setAction(ACTION_REFRESH)
            try {
                if (Playback.busy) context.startForegroundService(intent)
                else if (alive) context.startService(intent)
            } catch (e: RuntimeException) {
                // Started from the background where Android won't have it: the music plays on without it.
                Log.w(TAG, "the service could not be started: ${e.message}")
            }
        }

        /** The page is gone: the service goes. */
        fun end(context: Context) {
            if (!alive) return
            try {
                context.startService(Intent(context, PlaybackService::class.java).setAction(ACTION_STOP))
            } catch (e: RuntimeException) {
                Log.w(TAG, "the service could not be stopped: ${e.message}")
            }
        }
    }
}
