package com.plugin.telephony

import android.Manifest
import android.app.Activity
import android.content.ContentResolver
import android.content.Intent
import android.content.pm.PackageManager
import android.database.Cursor
import android.net.Uri
import android.provider.CallLog
import android.provider.ContactsContract
import android.provider.Telephony
import android.telecom.TelecomManager
import android.telephony.SmsManager
import android.telephony.TelephonyManager
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import org.json.JSONArray
import org.json.JSONObject
import java.text.SimpleDateFormat
import java.util.*

@InvokeArg
class MakeCallArgs {
    var phoneNumber: String = ""
}

@InvokeArg
class CallIdArgs {
    var callId: String = ""
}

@InvokeArg
class SMSArgs {
    var recipient: String = ""
    var message: String = ""
}

@InvokeArg
class MessageQueryArgs {
    var limit: Int? = null
    var offset: Int? = null
    var threadId: String? = null
    var messageType: String? = null
    var startDate: Long? = null
    var endDate: Long? = null
    var unreadOnly: Boolean? = null
}

@InvokeArg
class CallLogQueryArgs {
    var limit: Int? = null
    var offset: Int? = null
    var callType: String? = null
    var startDate: Long? = null
    var endDate: Long? = null
}

@InvokeArg
class MessageIdArgs {
    var messageId: String = ""
}

@InvokeArg
class PermissionsArgs {
    var permissions: Array<String> = arrayOf()
}

@TauriPlugin
class TelephonyPlugin(private val activity: Activity) : Plugin(activity) {

    private val telephonyManager: TelephonyManager by lazy {
        activity.getSystemService(Activity.TELEPHONY_SERVICE) as TelephonyManager
    }

    private val telecomManager: TelecomManager by lazy {
        activity.getSystemService(Activity.TELECOM_SERVICE) as TelecomManager
    }

    private val contentResolver: ContentResolver by lazy {
        activity.contentResolver
    }

    companion object {
        private const val PERMISSION_REQUEST_CODE = 1001

        // Permission mappings
        private val PERMISSION_MAP = mapOf(
            "READ_PHONE_STATE" to Manifest.permission.READ_PHONE_STATE,
            "CALL_PHONE" to Manifest.permission.CALL_PHONE,
            "READ_CALL_LOG" to Manifest.permission.READ_CALL_LOG,
            "WRITE_CALL_LOG" to Manifest.permission.WRITE_CALL_LOG,
            "SEND_SMS" to Manifest.permission.SEND_SMS,
            "RECEIVE_SMS" to Manifest.permission.RECEIVE_SMS,
            "READ_SMS" to Manifest.permission.READ_SMS,
            "READ_CONTACTS" to Manifest.permission.READ_CONTACTS,
            "ANSWER_PHONE_CALLS" to Manifest.permission.ANSWER_PHONE_CALLS
        )
    }

    @Command
    fun makeCall(invoke: Invoke) {
        val args = invoke.parseArgs(MakeCallArgs::class.java)

        if (!hasPermission(Manifest.permission.CALL_PHONE)) {
            invoke.reject("Permission denied: CALL_PHONE permission is required")
            return
        }

        try {
            val intent = Intent(Intent.ACTION_CALL).apply {
                data = Uri.parse("tel:${args.phoneNumber}")
            }
            activity.startActivity(intent)
            invoke.resolve()
        } catch (e: Exception) {
            invoke.reject("Failed to make call: ${e.message}")
        }
    }

    @Command
    fun answerCall(invoke: Invoke) {
        val args = invoke.parseArgs(CallIdArgs::class.java)

        if (!hasPermission(Manifest.permission.ANSWER_PHONE_CALLS)) {
            invoke.reject("Permission denied: ANSWER_PHONE_CALLS permission is required")
            return
        }

        try {
            if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.O) {
                val success = telecomManager.acceptRingingCall()
                if (success) {
                    invoke.resolve()
                } else {
                    invoke.reject("Failed to answer call")
                }
            } else {
                invoke.reject("Answer call requires Android 8.0 or higher")
            }
        } catch (e: Exception) {
            invoke.reject("Failed to answer call: ${e.message}")
        }
    }

    @Command
    fun rejectCall(invoke: Invoke) {
        val args = invoke.parseArgs(CallIdArgs::class.java)

        if (!hasPermission(Manifest.permission.ANSWER_PHONE_CALLS)) {
            invoke.reject("Permission denied: ANSWER_PHONE_CALLS permission is required")
            return
        }

        try {
            if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.P) {
                val success = telecomManager.endCall()
                if (success) {
                    invoke.resolve()
                } else {
                    invoke.reject("Failed to reject call")
                }
            } else {
                invoke.reject("Reject call requires Android 9.0 or higher")
            }
        } catch (e: Exception) {
            invoke.reject("Failed to reject call: ${e.message}")
        }
    }

    @Command
    fun endCall(invoke: Invoke) {
        val args = invoke.parseArgs(CallIdArgs::class.java)

        if (!hasPermission(Manifest.permission.ANSWER_PHONE_CALLS)) {
            invoke.reject("Permission denied: ANSWER_PHONE_CALLS permission is required")
            return
        }

        try {
            if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.P) {
                val success = telecomManager.endCall()
                if (success) {
                    invoke.resolve()
                } else {
                    invoke.reject("Failed to end call")
                }
            } else {
                invoke.reject("End call requires Android 9.0 or higher")
            }
        } catch (e: Exception) {
            invoke.reject("Failed to end call: ${e.message}")
        }
    }

    @Command
    fun getCallState(invoke: Invoke) {
        if (!hasPermission(Manifest.permission.READ_PHONE_STATE)) {
            invoke.reject("Permission denied: READ_PHONE_STATE permission is required")
            return
        }

        try {
            val state = when (telephonyManager.callState) {
                TelephonyManager.CALL_STATE_IDLE -> "IDLE"
                TelephonyManager.CALL_STATE_RINGING -> "RINGING"
                TelephonyManager.CALL_STATE_OFFHOOK -> "OFFHOOK"
                else -> "IDLE"
            }

            val result = JSObject()
            result.put("state", state)
            invoke.resolve(result)
        } catch (e: Exception) {
            invoke.reject("Failed to get call state: ${e.message}")
        }
    }

    @Command
    fun sendSMS(invoke: Invoke) {
        val args = invoke.parseArgs(SMSArgs::class.java)

        if (!hasPermission(Manifest.permission.SEND_SMS)) {
            invoke.reject("Permission denied: SEND_SMS permission is required")
            return
        }

        try {
            val smsManager = SmsManager.getDefault()
            val parts = smsManager.divideMessage(args.message)

            if (parts.size > 1) {
                smsManager.sendMultipartTextMessage(
                    args.recipient,
                    null,
                    parts,
                    null,
                    null
                )
            } else {
                smsManager.sendTextMessage(
                    args.recipient,
                    null,
                    args.message,
                    null,
                    null
                )
            }

            val result = JSObject()
            result.put("messageId", "sms_${UUID.randomUUID()}")
            invoke.resolve(result)

            // Emit event
            trigger("telephony://sms-sent", JSONObject().apply {
                put("recipient", args.recipient)
                put("success", true)
            })
        } catch (e: Exception) {
            invoke.reject("Failed to send SMS: ${e.message}")

            // Emit error event
            trigger("telephony://sms-sent", JSONObject().apply {
                put("recipient", args.recipient)
                put("success", false)
                put("error", e.message)
            })
        }
    }

    @Command
    fun getSMSMessages(invoke: Invoke) {
        val args = invoke.parseArgs(MessageQueryArgs::class.java)

        if (!hasPermission(Manifest.permission.READ_SMS)) {
            invoke.reject("Permission denied: READ_SMS permission is required")
            return
        }

        try {
            val messages = JSONArray()
            val uri = Telephony.Sms.CONTENT_URI
            val projection = arrayOf(
                Telephony.Sms._ID,
                Telephony.Sms.THREAD_ID,
                Telephony.Sms.ADDRESS,
                Telephony.Sms.BODY,
                Telephony.Sms.DATE,
                Telephony.Sms.READ,
                Telephony.Sms.TYPE
            )

            var selection: String? = null
            val selectionArgs = mutableListOf<String>()

            // Build selection query
            val conditions = mutableListOf<String>()

            args.threadId?.let {
                conditions.add("${Telephony.Sms.THREAD_ID} = ?")
                selectionArgs.add(it)
            }

            args.startDate?.let {
                conditions.add("${Telephony.Sms.DATE} >= ?")
                selectionArgs.add(it.toString())
            }

            args.endDate?.let {
                conditions.add("${Telephony.Sms.DATE} <= ?")
                selectionArgs.add(it.toString())
            }

            args.unreadOnly?.let {
                if (it) {
                    conditions.add("${Telephony.Sms.READ} = ?")
                    selectionArgs.add("0")
                }
            }

            if (conditions.isNotEmpty()) {
                selection = conditions.joinToString(" AND ")
            }

            val sortOrder = "${Telephony.Sms.DATE} DESC"
            val limit = args.limit ?: 100
            val offset = args.offset ?: 0

            val cursor: Cursor? = contentResolver.query(
                uri,
                projection,
                selection,
                if (selectionArgs.isEmpty()) null else selectionArgs.toTypedArray(),
                "$sortOrder LIMIT $limit OFFSET $offset"
            )

            cursor?.use {
                val idIndex = it.getColumnIndex(Telephony.Sms._ID)
                val threadIdIndex = it.getColumnIndex(Telephony.Sms.THREAD_ID)
                val addressIndex = it.getColumnIndex(Telephony.Sms.ADDRESS)
                val bodyIndex = it.getColumnIndex(Telephony.Sms.BODY)
                val dateIndex = it.getColumnIndex(Telephony.Sms.DATE)
                val readIndex = it.getColumnIndex(Telephony.Sms.READ)
                val typeIndex = it.getColumnIndex(Telephony.Sms.TYPE)

                while (it.moveToNext()) {
                    val message = JSONObject().apply {
                        put("id", it.getString(idIndex))
                        put("threadId", it.getString(threadIdIndex))
                        put("sender", it.getString(addressIndex))
                        put("body", it.getString(bodyIndex))
                        put("timestamp", it.getLong(dateIndex))
                        put("isRead", it.getInt(readIndex) == 1)
                        put("messageType", mapSmsType(it.getInt(typeIndex)))
                    }
                    messages.put(message)
                }
            }

            val result = JSObject()
            result.put("messages", messages)
            invoke.resolve(result)
        } catch (e: Exception) {
            invoke.reject("Failed to get SMS messages: ${e.message}")
        }
    }

    @Command
    fun deleteSMS(invoke: Invoke) {
        val args = invoke.parseArgs(MessageIdArgs::class.java)

        if (!hasPermission(Manifest.permission.WRITE_SMS)) {
            invoke.reject("Permission denied: WRITE_SMS permission is required")
            return
        }

        try {
            val uri = Uri.parse("content://sms/${args.messageId}")
            val deleted = contentResolver.delete(uri, null, null)

            if (deleted > 0) {
                invoke.resolve()
            } else {
                invoke.reject("Message not found")
            }
        } catch (e: Exception) {
            invoke.reject("Failed to delete SMS: ${e.message}")
        }
    }

    @Command
    fun getCallLogs(invoke: Invoke) {
        val args = invoke.parseArgs(CallLogQueryArgs::class.java)

        if (!hasPermission(Manifest.permission.READ_CALL_LOG)) {
            invoke.reject("Permission denied: READ_CALL_LOG permission is required")
            return
        }

        try {
            val logs = JSONArray()
            val projection = arrayOf(
                CallLog.Calls._ID,
                CallLog.Calls.NUMBER,
                CallLog.Calls.TYPE,
                CallLog.Calls.DATE,
                CallLog.Calls.DURATION,
                CallLog.Calls.CACHED_NAME
            )

            var selection: String? = null
            val selectionArgs = mutableListOf<String>()

            // Build selection query
            val conditions = mutableListOf<String>()

            args.callType?.let {
                val type = when (it) {
                    "INCOMING" -> CallLog.Calls.INCOMING_TYPE
                    "OUTGOING" -> CallLog.Calls.OUTGOING_TYPE
                    "MISSED" -> CallLog.Calls.MISSED_TYPE
                    "REJECTED" -> CallLog.Calls.REJECTED_TYPE
                    "BLOCKED" -> CallLog.Calls.BLOCKED_TYPE
                    "VOICEMAIL" -> CallLog.Calls.VOICEMAIL_TYPE
                    else -> null
                }

                type?.let { t ->
                    conditions.add("${CallLog.Calls.TYPE} = ?")
                    selectionArgs.add(t.toString())
                }
            }

            args.startDate?.let {
                conditions.add("${CallLog.Calls.DATE} >= ?")
                selectionArgs.add(it.toString())
            }

            args.endDate?.let {
                conditions.add("${CallLog.Calls.DATE} <= ?")
                selectionArgs.add(it.toString())
            }

            if (conditions.isNotEmpty()) {
                selection = conditions.joinToString(" AND ")
            }

            val sortOrder = "${CallLog.Calls.DATE} DESC"
            val limit = args.limit ?: 100
            val offset = args.offset ?: 0

            val cursor: Cursor? = contentResolver.query(
                CallLog.Calls.CONTENT_URI,
                projection,
                selection,
                if (selectionArgs.isEmpty()) null else selectionArgs.toTypedArray(),
                "$sortOrder LIMIT $limit OFFSET $offset"
            )

            cursor?.use {
                val idIndex = it.getColumnIndex(CallLog.Calls._ID)
                val numberIndex = it.getColumnIndex(CallLog.Calls.NUMBER)
                val typeIndex = it.getColumnIndex(CallLog.Calls.TYPE)
                val dateIndex = it.getColumnIndex(CallLog.Calls.DATE)
                val durationIndex = it.getColumnIndex(CallLog.Calls.DURATION)
                val nameIndex = it.getColumnIndex(CallLog.Calls.CACHED_NAME)

                while (it.moveToNext()) {
                    val log = JSONObject().apply {
                        put("id", it.getString(idIndex))
                        put("phoneNumber", it.getString(numberIndex) ?: "Unknown")
                        put("callType", mapCallType(it.getInt(typeIndex)))
                        put("timestamp", it.getLong(dateIndex))
                        put("duration", it.getInt(durationIndex))

                        val name = it.getString(nameIndex)
                        if (name != null) {
                            put("contactName", name)
                        }
                    }
                    logs.put(log)
                }
            }

            val result = JSObject()
            result.put("logs", logs)
            invoke.resolve(result)
        } catch (e: Exception) {
            invoke.reject("Failed to get call logs: ${e.message}")
        }
    }

    @Command
    fun getContacts(invoke: Invoke) {
        if (!hasPermission(Manifest.permission.READ_CONTACTS)) {
            invoke.reject("Permission denied: READ_CONTACTS permission is required")
            return
        }

        try {
            val contacts = JSONArray()
            val cursor = contentResolver.query(
                ContactsContract.Contacts.CONTENT_URI,
                null,
                null,
                null,
                ContactsContract.Contacts.DISPLAY_NAME + " ASC"
            )

            cursor?.use {
                val idIndex = it.getColumnIndex(ContactsContract.Contacts._ID)
                val nameIndex = it.getColumnIndex(ContactsContract.Contacts.DISPLAY_NAME)
                val hasPhoneIndex = it.getColumnIndex(ContactsContract.Contacts.HAS_PHONE_NUMBER)

                while (it.moveToNext()) {
                    val id = it.getString(idIndex)
                    val name = it.getString(nameIndex)
                    val hasPhone = it.getInt(hasPhoneIndex) > 0

                    if (hasPhone) {
                        val phoneNumbers = getPhoneNumbers(id)

                        if (phoneNumbers.length() > 0) {
                            val contact = JSONObject().apply {
                                put("id", id)
                                put("name", name)
                                put("phoneNumbers", phoneNumbers)
                                put("emailAddresses", JSONArray()) // Could be extended
                            }
                            contacts.put(contact)
                        }
                    }
                }
            }

            val result = JSObject()
            result.put("contacts", contacts)
            invoke.resolve(result)
        } catch (e: Exception) {
            invoke.reject("Failed to get contacts: ${e.message}")
        }
    }

    @Command
    fun requestPermissions(invoke: Invoke) {
        val args = invoke.parseArgs(PermissionsArgs::class.java)

        val androidPermissions = args.permissions.mapNotNull { PERMISSION_MAP[it] }.toTypedArray()

        if (androidPermissions.isEmpty()) {
            invoke.reject("No valid permissions requested")
            return
        }

        ActivityCompat.requestPermissions(activity, androidPermissions, PERMISSION_REQUEST_CODE)

        // Note: In a real implementation, you'd need to handle the result in onRequestPermissionsResult
        // For now, we'll return the current status
        checkPermissionsInternal(invoke, args.permissions)
    }

    @Command
    fun checkPermissions(invoke: Invoke) {
        val args = invoke.parseArgs(PermissionsArgs::class.java)
        checkPermissionsInternal(invoke, args.permissions)
    }

    // Helper methods

    private fun checkPermissionsInternal(invoke: Invoke, permissions: Array<String>) {
        try {
            val result = JSObject()
            val permissionsObj = JSONObject()

            for (permission in permissions) {
                val androidPermission = PERMISSION_MAP[permission]
                if (androidPermission != null) {
                    val state = when {
                        ContextCompat.checkSelfPermission(activity, androidPermission) ==
                            PackageManager.PERMISSION_GRANTED -> "GRANTED"
                        ActivityCompat.shouldShowRequestPermissionRationale(activity, androidPermission) ->
                            "DENIED"
                        else -> "PROMPT"
                    }
                    permissionsObj.put(permission, state)
                }
            }

            result.put("permissions", permissionsObj)
            invoke.resolve(result)
        } catch (e: Exception) {
            invoke.reject("Failed to check permissions: ${e.message}")
        }
    }

    private fun hasPermission(permission: String): Boolean {
        return ContextCompat.checkSelfPermission(activity, permission) ==
            PackageManager.PERMISSION_GRANTED
    }

    private fun getPhoneNumbers(contactId: String): JSONArray {
        val phoneNumbers = JSONArray()
        val cursor = contentResolver.query(
            ContactsContract.CommonDataKinds.Phone.CONTENT_URI,
            null,
            "${ContactsContract.CommonDataKinds.Phone.CONTACT_ID} = ?",
            arrayOf(contactId),
            null
        )

        cursor?.use {
            val numberIndex = it.getColumnIndex(ContactsContract.CommonDataKinds.Phone.NUMBER)
            val typeIndex = it.getColumnIndex(ContactsContract.CommonDataKinds.Phone.TYPE)

            while (it.moveToNext()) {
                val number = it.getString(numberIndex)
                val type = it.getInt(typeIndex)

                val phoneNumber = JSONObject().apply {
                    put("number", number)
                    put("numberType", mapPhoneNumberType(type))
                }
                phoneNumbers.put(phoneNumber)
            }
        }

        return phoneNumbers
    }

    private fun mapCallType(type: Int): String {
        return when (type) {
            CallLog.Calls.INCOMING_TYPE -> "INCOMING"
            CallLog.Calls.OUTGOING_TYPE -> "OUTGOING"
            CallLog.Calls.MISSED_TYPE -> "MISSED"
            CallLog.Calls.REJECTED_TYPE -> "REJECTED"
            CallLog.Calls.BLOCKED_TYPE -> "BLOCKED"
            CallLog.Calls.VOICEMAIL_TYPE -> "VOICEMAIL"
            else -> "INCOMING"
        }
    }

    private fun mapSmsType(type: Int): String {
        return when (type) {
            Telephony.Sms.MESSAGE_TYPE_INBOX -> "INBOX"
            Telephony.Sms.MESSAGE_TYPE_SENT -> "SENT"
            Telephony.Sms.MESSAGE_TYPE_DRAFT -> "DRAFT"
            Telephony.Sms.MESSAGE_TYPE_OUTBOX -> "OUTBOX"
            Telephony.Sms.MESSAGE_TYPE_FAILED -> "FAILED"
            Telephony.Sms.MESSAGE_TYPE_QUEUED -> "QUEUED"
            else -> "INBOX"
        }
    }

    private fun mapPhoneNumberType(type: Int): String {
        return when (type) {
            ContactsContract.CommonDataKinds.Phone.TYPE_MOBILE -> "MOBILE"
            ContactsContract.CommonDataKinds.Phone.TYPE_HOME -> "HOME"
            ContactsContract.CommonDataKinds.Phone.TYPE_WORK -> "WORK"
            ContactsContract.CommonDataKinds.Phone.TYPE_MAIN -> "MAIN"
            ContactsContract.CommonDataKinds.Phone.TYPE_FAX_WORK -> "FAX_WORK"
            ContactsContract.CommonDataKinds.Phone.TYPE_FAX_HOME -> "FAX_HOME"
            ContactsContract.CommonDataKinds.Phone.TYPE_PAGER -> "PAGER"
            else -> "OTHER"
        }
    }
}
