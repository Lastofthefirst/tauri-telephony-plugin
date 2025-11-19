fn main() {
    tauri_plugin::Builder::new(&["android", "ios"])
        .android_path("android")
        .ios_path("ios")
        .build();
}
