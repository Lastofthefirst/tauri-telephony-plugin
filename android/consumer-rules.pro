# Consumer ProGuard rules for library

# Keep plugin public API
-keep public class com.plugin.telephony.TelephonyPlugin { *; }

# Keep receivers
-keep public class com.plugin.telephony.SMSReceiver { *; }
-keep public class com.plugin.telephony.PhoneStateReceiver { *; }
