use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

pub use models::*;

mod commands;
mod error;
mod models;

#[cfg(desktop)]
mod desktop;

#[cfg(mobile)]
mod mobile;

pub use error::{Error, Result};

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_telephony);

/// Initializes the telephony plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("telephony")
        .invoke_handler(tauri::generate_handler![
            commands::make_call,
            commands::answer_call,
            commands::reject_call,
            commands::end_call,
            commands::get_call_state,
            commands::send_sms,
            commands::get_sms_messages,
            commands::delete_sms,
            commands::get_call_logs,
            commands::get_contacts,
            commands::request_permissions,
            commands::check_permissions,
        ])
        .setup(|app, api| {
            #[cfg(target_os = "android")]
            {
                api.register_android_plugin("com.plugin.telephony", "TelephonyPlugin")?;
            }
            #[cfg(target_os = "ios")]
            {
                api.register_ios_plugin(init_plugin_telephony)?;
            }
            Ok(())
        })
        .build()
}
