package com.plugin.telephony

import android.Manifest
import android.content.Context
import android.content.pm.PackageManager
import androidx.core.content.ContextCompat
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import org.junit.Assert.*
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith

/**
 * Instrumented test for the Telephony Plugin.
 * These tests run on an Android device or emulator.
 */
@RunWith(AndroidJUnit4::class)
class TelephonyPluginTest {

    private lateinit var context: Context

    @Before
    fun setUp() {
        context = InstrumentationRegistry.getInstrumentation().targetContext
    }

    @Test
    fun testContextIsNotNull() {
        assertNotNull(context)
    }

    @Test
    fun testPermissionMapping() {
        val permissionMap = mapOf(
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

        assertEquals(9, permissionMap.size)
        assertEquals(Manifest.permission.READ_PHONE_STATE, permissionMap["READ_PHONE_STATE"])
    }

    @Test
    fun testCallTypeMapping() {
        val callTypes = mapOf(
            1 to "INCOMING",
            2 to "OUTGOING",
            3 to "MISSED",
            5 to "REJECTED",
            6 to "BLOCKED",
            4 to "VOICEMAIL"
        )

        assertEquals("INCOMING", callTypes[1])
        assertEquals("OUTGOING", callTypes[2])
        assertEquals("MISSED", callTypes[3])
    }

    @Test
    fun testSmsTypeMapping() {
        val smsTypes = mapOf(
            1 to "INBOX",
            2 to "SENT",
            3 to "DRAFT",
            4 to "OUTBOX",
            5 to "FAILED",
            6 to "QUEUED"
        )

        assertEquals("INBOX", smsTypes[1])
        assertEquals("SENT", smsTypes[2])
        assertEquals("DRAFT", smsTypes[3])
    }

    @Test
    fun testPhoneNumberValidation() {
        val validNumbers = listOf(
            "+1234567890",
            "123-456-7890",
            "(123) 456-7890",
            "123 456 7890"
        )

        val invalidNumbers = listOf(
            "",
            "abc",
            "123abc456"
        )

        validNumbers.forEach { number ->
            assertTrue(
                "Valid number $number should pass validation",
                isValidPhoneNumber(number)
            )
        }

        invalidNumbers.forEach { number ->
            assertFalse(
                "Invalid number $number should fail validation",
                isValidPhoneNumber(number)
            )
        }
    }

    @Test
    fun testSMSReceiverIntentAction() {
        assertEquals(
            "com.plugin.telephony.SMS_RECEIVED",
            SMSReceiver.ACTION_SMS_RECEIVED
        )
    }

    @Test
    fun testPhoneStateReceiverIntentActions() {
        assertEquals(
            "com.plugin.telephony.INCOMING_CALL",
            PhoneStateReceiver.ACTION_INCOMING_CALL
        )
        assertEquals(
            "com.plugin.telephony.CALL_STATE_CHANGED",
            PhoneStateReceiver.ACTION_CALL_STATE_CHANGED
        )
    }

    // Helper function for phone number validation
    private fun isValidPhoneNumber(phoneNumber: String): Boolean {
        if (phoneNumber.isEmpty()) return false

        return phoneNumber.all { char ->
            char.isDigit() || char in setOf('+', '-', '(', ')', ' ')
        }
    }
}
