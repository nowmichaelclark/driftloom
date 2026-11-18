package app.loops

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.media.AudioAttributes
import android.media.AudioFocusRequest
import android.media.AudioManager

/**
 * Debug builds only (session 28): stands in for a phone call on the emulator,
 * which has no call to take. A call takes the audio focus for a while (a
 * transient loss for whatever plays) and gives it back when it ends:
 *
 *   adb shell am broadcast -n ID/app.loops.CallSimulator -a app.loops.debug.CALL_START
 *   adb shell am broadcast -n ID/app.loops.CallSimulator -a app.loops.debug.CALL_END
 *
 * android/background.mjs does it, and checks the music pauses and returns.
 */
class CallSimulator : BroadcastReceiver() {
    override fun onReceive(context: Context, intent: Intent) {
        val audio = context.getSystemService(AudioManager::class.java)
        when (intent.action) {
            START -> {
                val request = AudioFocusRequest.Builder(AudioManager.AUDIOFOCUS_GAIN_TRANSIENT)
                    .setAudioAttributes(
                        AudioAttributes.Builder()
                            .setUsage(AudioAttributes.USAGE_VOICE_COMMUNICATION)
                            .setContentType(AudioAttributes.CONTENT_TYPE_SPEECH)
                            .build(),
                    )
                    .setOnAudioFocusChangeListener { }
                    .build()
                held = request
                audio.requestAudioFocus(request)
            }
            END -> {
                held?.let { audio.abandonAudioFocusRequest(it) }
                held = null
            }
        }
    }

    companion object {
        const val START = "app.loops.debug.CALL_START"
        const val END = "app.loops.debug.CALL_END"
        private var held: AudioFocusRequest? = null
    }
}
