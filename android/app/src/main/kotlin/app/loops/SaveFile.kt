package app.loops

import android.content.ContentResolver
import android.net.Uri
import android.os.ParcelFileDescriptor
import android.provider.DocumentsContract
import android.provider.OpenableColumns
import android.system.ErrnoException
import android.system.Os
import android.system.OsConstants
import java.io.FileOutputStream
import java.io.IOException
import java.nio.ByteBuffer

/**
 * A file the listener chose through Android's own picker (the Storage Access
 * Framework's ACTION_CREATE_DOCUMENT), written as it is made (session 28,
 * queue item 107d): the page's export sends its bytes a megabyte at a time
 * (js/export-worker.js's relay sink) and each goes straight to the file, so
 * memory stays the same for a minute or three hours and the file never exists
 * twice on the disk. The place may be anywhere a document provider offers:
 * this device's own storage seeks (FLAC's header is written again at the
 * end, as the web's file handles do); a provider that streams (a cloud
 * folder) can't, so it takes only what is written front to back (Opus).
 *
 * Free space is the platform's own answer: `fstatvfs` on the file's
 * descriptor, which is the file system the file lives on, wherever the
 * picker put it. A place that can't say (a stream) says -1 and the write
 * itself tells if the disk fills.
 */
class SaveFile(private val resolver: ContentResolver, val uri: Uri, val mime: String) {
    private val pfd: ParcelFileDescriptor = resolver.openFileDescriptor(uri, "wt")
        ?: throw IOException("the place would not open")
    private val out = FileOutputStream(pfd.fileDescriptor)

    /** The bytes written so far. */
    var bytes = 0L
        private set

    /** Whether the place can be written at a position (the header's second write). */
    val seekable: Boolean = try {
        Os.lseek(pfd.fileDescriptor, 0, OsConstants.SEEK_CUR)
        true
    } catch (_: ErrnoException) {
        false
    }

    /** Bytes free on the file system the file lives on, or -1 if the place can't say. */
    val free: Long = try {
        val st = Os.fstatvfs(pfd.fileDescriptor)
        st.f_bavail * st.f_frsize
    } catch (_: ErrnoException) {
        -1L
    }

    /** The name the provider gave the file (it may have changed the one asked for). */
    val name: String = resolver.query(uri, arrayOf(OpenableColumns.DISPLAY_NAME), null, null, null)?.use {
        if (it.moveToFirst()) it.getString(0) else null
    } ?: uri.lastPathSegment.orEmpty()

    fun write(data: ByteArray) {
        out.write(data)
        bytes += data.size
    }

    /** The file's first bytes again (FLAC's header, once the length and the checksum are known). */
    fun header(data: ByteArray) {
        out.channel.write(ByteBuffer.wrap(data), 0)
    }

    /** The file is complete: flushed to the disk and closed. */
    fun finish() {
        out.flush()
        try { Os.fsync(pfd.fileDescriptor) } catch (_: ErrnoException) { }
        out.close()
        pfd.close()
    }

    /** Nothing is to be kept: closed, and the file the picker made removed. */
    fun discard() {
        try { out.close() } catch (_: IOException) { }
        try { pfd.close() } catch (_: IOException) { }
        remove(resolver, uri)
    }

    companion object {
        /** Remove a document, as far as its provider lets us. */
        fun remove(resolver: ContentResolver, uri: Uri) {
            try {
                if (!DocumentsContract.deleteDocument(resolver, uri)) resolver.delete(uri, null, null)
            } catch (_: Exception) {
                // Not ours to remove, or already gone: nothing more to do.
            }
        }

        /** Whether a failed write was the disk filling. */
        fun isFull(e: IOException): Boolean =
            (e.message ?: "").let { it.contains("ENOSPC") || it.contains("No space", ignoreCase = true) }
    }
}
