use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Telephony<R>> {
    Ok(Telephony(app.clone()))
}

/// Access to the telephony APIs.
pub struct Telephony<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Telephony<R> {
    pub fn make_call(&self, phone_number: &str) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("make_call", MakeCallOptions {
                phone_number: phone_number.to_string(),
            })
            .map_err(Into::into)
    }

    pub fn answer_call(&self, call_id: &str) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("answer_call", CallIdArgs {
                call_id: call_id.to_string(),
            })
            .map_err(Into::into)
    }

    pub fn reject_call(&self, call_id: &str) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("reject_call", CallIdArgs {
                call_id: call_id.to_string(),
            })
            .map_err(Into::into)
    }

    pub fn end_call(&self, call_id: &str) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("end_call", CallIdArgs {
                call_id: call_id.to_string(),
            })
            .map_err(Into::into)
    }

    pub fn get_call_state(&self) -> crate::Result<CallState> {
        self.0
            .run_mobile_plugin::<()>("get_call_state", ())
            .map_err(Into::into)
    }

    pub fn send_sms(&self, recipient: &str, message: &str) -> crate::Result<String> {
        self.0
            .run_mobile_plugin("send_sms", SMSOptions {
                recipient: recipient.to_string(),
                message: message.to_string(),
            })
            .map_err(Into::into)
    }

    pub fn get_sms_messages(&self, options: MessageQueryOptions) -> crate::Result<Vec<SMSMessage>> {
        self.0
            .run_mobile_plugin("get_sms_messages", options)
            .map_err(Into::into)
    }

    pub fn delete_sms(&self, message_id: &str) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("delete_sms", MessageIdArgs {
                message_id: message_id.to_string(),
            })
            .map_err(Into::into)
    }

    pub fn get_call_logs(&self, options: CallLogQueryOptions) -> crate::Result<Vec<CallLog>> {
        self.0
            .run_mobile_plugin("get_call_logs", options)
            .map_err(Into::into)
    }

    pub fn get_contacts(&self) -> crate::Result<Vec<Contact>> {
        self.0
            .run_mobile_plugin::<()>("get_contacts", ())
            .map_err(Into::into)
    }

    pub fn request_permissions(&self, permissions: Vec<Permission>) -> crate::Result<PermissionStatus> {
        self.0
            .run_mobile_plugin("request_permissions", PermissionsArgs { permissions })
            .map_err(Into::into)
    }

    pub fn check_permissions(&self, permissions: Vec<Permission>) -> crate::Result<PermissionStatus> {
        self.0
            .run_mobile_plugin("check_permissions", PermissionsArgs { permissions })
            .map_err(Into::into)
    }
}

// Argument structs for mobile IPC
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MakeCallOptions {
    phone_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CallIdArgs {
    call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessageIdArgs {
    message_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PermissionsArgs {
    permissions: Vec<Permission>,
}
