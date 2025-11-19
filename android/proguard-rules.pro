# Add project specific ProGuard rules here.
# You can control the set of applied configuration files using the
# proguardFiles setting in build.gradle.

# Keep Tauri plugin classes
-keep class com.plugin.telephony.** { *; }

# Keep annotations
-keepattributes *Annotation*

# Keep native methods
-keepclasseswithmembernames class * {
    native <methods>;
}

# Keep Tauri annotations
-keep @app.tauri.annotation.TauriPlugin class * { *; }
-keep @app.tauri.annotation.Command class * { *; }
-keep @app.tauri.annotation.InvokeArg class * { *; }
