use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::error::Error;
use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Telephony<R>> {
    Ok(Telephony(std::marker::PhantomData))
}

/// Access to the telephony APIs.
pub struct Telephony<R: Runtime>(std::marker::PhantomData<R>);

impl<R: Runtime> Telephony<R> {
    pub fn make_call(&self, _phone_number: &str) -> crate::Result<()> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }

    pub fn answer_call(&self, _call_id: &str) -> crate::Result<()> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }

    pub fn reject_call(&self, _call_id: &str) -> crate::Result<()> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }

    pub fn end_call(&self, _call_id: &str) -> crate::Result<()> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }

    pub fn get_call_state(&self) -> crate::Result<CallState> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }

    pub fn send_sms(&self, _recipient: &str, _message: &str) -> crate::Result<String> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }

    pub fn get_sms_messages(&self, _options: MessageQueryOptions) -> crate::Result<Vec<SMSMessage>> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }

    pub fn delete_sms(&self, _message_id: &str) -> crate::Result<()> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }

    pub fn get_call_logs(&self, _options: CallLogQueryOptions) -> crate::Result<Vec<CallLog>> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }

    pub fn get_contacts(&self) -> crate::Result<Vec<Contact>> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }

    pub fn request_permissions(&self, _permissions: Vec<Permission>) -> crate::Result<PermissionStatus> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }

    pub fn check_permissions(&self, _permissions: Vec<Permission>) -> crate::Result<PermissionStatus> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }
}
