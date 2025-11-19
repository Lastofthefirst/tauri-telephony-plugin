const COMMANDS: &[&str] = &[
    "make_call",
    "answer_call",
    "reject_call",
    "end_call",
    "get_call_state",
    "send_sms",
    "get_sms_messages",
    "delete_sms",
    "get_call_logs",
    "get_contacts",
    "request_permissions",
    "check_permissions",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
