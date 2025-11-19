package com.plugin.telephony

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.provider.Telephony
import android.util.Log

/**
 * Broadcast receiver for incoming SMS messages.
 * Emits events to the Tauri app when messages are received.
 */
class SMSReceiver : BroadcastReceiver() {

    override fun onReceive(context: Context?, intent: Intent?) {
        if (intent?.action == Telephony.Sms.Intents.SMS_RECEIVED_ACTION) {
            val messages = Telephony.Sms.Intents.getMessagesFromIntent(intent)

            for (smsMessage in messages) {
                val sender = smsMessage.displayOriginatingAddress
                val body = smsMessage.messageBody
                val timestamp = smsMessage.timestampMillis

                Log.d(TAG, "SMS received from: $sender")

                // TODO: Emit event to Tauri app
                // This would require a reference to the Tauri app instance
                // which should be set up when the plugin is initialized

                // For now, we'll broadcast locally
                context?.let {
                    val localIntent = Intent(ACTION_SMS_RECEIVED).apply {
                        putExtra(EXTRA_SENDER, sender)
                        putExtra(EXTRA_BODY, body)
                        putExtra(EXTRA_TIMESTAMP, timestamp)
                    }
                    it.sendBroadcast(localIntent)
                }
            }
        }
    }

    companion object {
        private const val TAG = "SMSReceiver"
        const val ACTION_SMS_RECEIVED = "com.plugin.telephony.SMS_RECEIVED"
        const val EXTRA_SENDER = "sender"
        const val EXTRA_BODY = "body"
        const val EXTRA_TIMESTAMP = "timestamp"
    }
}
