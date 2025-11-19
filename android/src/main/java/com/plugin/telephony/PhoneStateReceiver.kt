package com.plugin.telephony

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.telephony.TelephonyManager
import android.util.Log

/**
 * Broadcast receiver for phone state changes.
 * Emits events to the Tauri app when call states change.
 */
class PhoneStateReceiver : BroadcastReceiver() {

    override fun onReceive(context: Context?, intent: Intent?) {
        if (intent?.action == TelephonyManager.ACTION_PHONE_STATE_CHANGED) {
            val state = intent.getStringExtra(TelephonyManager.EXTRA_STATE)
            val phoneNumber = intent.getStringExtra(TelephonyManager.EXTRA_INCOMING_NUMBER)

            Log.d(TAG, "Phone state changed: $state, number: $phoneNumber")

            when (state) {
                TelephonyManager.EXTRA_STATE_RINGING -> {
                    // Incoming call
                    phoneNumber?.let {
                        Log.d(TAG, "Incoming call from: $it")

                        context?.let { ctx ->
                            val localIntent = Intent(ACTION_INCOMING_CALL).apply {
                                putExtra(EXTRA_PHONE_NUMBER, it)
                                putExtra(EXTRA_STATE, "RINGING")
                            }
                            ctx.sendBroadcast(localIntent)
                        }
                    }
                }
                TelephonyManager.EXTRA_STATE_OFFHOOK -> {
                    // Call answered
                    Log.d(TAG, "Call answered")

                    context?.let { ctx ->
                        val localIntent = Intent(ACTION_CALL_STATE_CHANGED).apply {
                            putExtra(EXTRA_STATE, "OFFHOOK")
                        }
                        ctx.sendBroadcast(localIntent)
                    }
                }
                TelephonyManager.EXTRA_STATE_IDLE -> {
                    // Call ended
                    Log.d(TAG, "Call ended")

                    context?.let { ctx ->
                        val localIntent = Intent(ACTION_CALL_STATE_CHANGED).apply {
                            putExtra(EXTRA_STATE, "IDLE")
                        }
                        ctx.sendBroadcast(localIntent)
                    }
                }
            }
        }
    }

    companion object {
        private const val TAG = "PhoneStateReceiver"
        const val ACTION_INCOMING_CALL = "com.plugin.telephony.INCOMING_CALL"
        const val ACTION_CALL_STATE_CHANGED = "com.plugin.telephony.CALL_STATE_CHANGED"
        const val EXTRA_PHONE_NUMBER = "phoneNumber"
        const val EXTRA_STATE = "state"
    }
}
