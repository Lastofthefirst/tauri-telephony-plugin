use crate::{
    error::{Error, Result},
    models::*,
};
use tauri::{AppHandle, Runtime, State};

#[cfg(target_os = "android")]
use crate::android::TelephonyManager as PlatformManager;

#[cfg(target_os = "ios")]
use crate::ios::TelephonyManager as PlatformManager;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
struct PlatformManager;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
impl PlatformManager {
    fn make_call(&self, _phone_number: String) -> Result<()> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }

    fn answer_call(&self, _call_id: String) -> Result<()> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }

    fn reject_call(&self, _call_id: String) -> Result<()> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }

    fn end_call(&self, _call_id: String) -> Result<()> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }

    fn get_call_state(&self) -> Result<CallState> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }

    fn send_sms(&self, _recipient: String, _message: String) -> Result<String> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }

    fn get_sms_messages(&self, _options: MessageQueryOptions) -> Result<Vec<SMSMessage>> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }

    fn delete_sms(&self, _message_id: String) -> Result<()> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }

    fn get_call_logs(&self, _options: CallLogQueryOptions) -> Result<Vec<CallLog>> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }

    fn get_contacts(&self) -> Result<Vec<Contact>> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }

    fn request_permissions(&self, _permissions: Vec<Permission>) -> Result<PermissionStatus> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }

    fn check_permissions(&self, _permissions: Vec<Permission>) -> Result<PermissionStatus> {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }
}

/// Make a phone call to the specified number.
#[tauri::command]
pub async fn make_call<R: Runtime>(
    _app: AppHandle<R>,
    phone_number: String,
) -> Result<()> {
    validate_phone_number(&phone_number)?;

    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let manager = PlatformManager::new();
        manager.make_call(phone_number)
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let manager = PlatformManager;
        manager.make_call(phone_number)
    }
}

/// Answer an incoming call.
#[tauri::command]
pub async fn answer_call<R: Runtime>(
    _app: AppHandle<R>,
    call_id: String,
) -> Result<()> {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let manager = PlatformManager::new();
        manager.answer_call(call_id)
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let manager = PlatformManager;
        manager.answer_call(call_id)
    }
}

/// Reject an incoming call.
#[tauri::command]
pub async fn reject_call<R: Runtime>(
    _app: AppHandle<R>,
    call_id: String,
) -> Result<()> {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let manager = PlatformManager::new();
        manager.reject_call(call_id)
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let manager = PlatformManager;
        manager.reject_call(call_id)
    }
}

/// End an active call.
#[tauri::command]
pub async fn end_call<R: Runtime>(
    _app: AppHandle<R>,
    call_id: String,
) -> Result<()> {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let manager = PlatformManager::new();
        manager.end_call(call_id)
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let manager = PlatformManager;
        manager.end_call(call_id)
    }
}

/// Get the current call state.
#[tauri::command]
pub async fn get_call_state<R: Runtime>(_app: AppHandle<R>) -> Result<CallState> {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let manager = PlatformManager::new();
        manager.get_call_state()
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let manager = PlatformManager;
        manager.get_call_state()
    }
}

/// Send an SMS message.
#[tauri::command]
pub async fn send_sms<R: Runtime>(
    _app: AppHandle<R>,
    options: SMSOptions,
) -> Result<String> {
    validate_phone_number(&options.recipient)?;

    if options.message.is_empty() {
        return Err(Error::InvalidMessage("Message cannot be empty".to_string()));
    }

    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let manager = PlatformManager::new();
        manager.send_sms(options.recipient, options.message)
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let manager = PlatformManager;
        manager.send_sms(options.recipient, options.message)
    }
}

/// Get SMS messages from the device.
#[tauri::command]
pub async fn get_sms_messages<R: Runtime>(
    _app: AppHandle<R>,
    options: Option<MessageQueryOptions>,
) -> Result<Vec<SMSMessage>> {
    let options = options.unwrap_or_default();

    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let manager = PlatformManager::new();
        manager.get_sms_messages(options)
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let manager = PlatformManager;
        manager.get_sms_messages(options)
    }
}

/// Delete an SMS message.
#[tauri::command]
pub async fn delete_sms<R: Runtime>(
    _app: AppHandle<R>,
    message_id: String,
) -> Result<()> {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let manager = PlatformManager::new();
        manager.delete_sms(message_id)
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let manager = PlatformManager;
        manager.delete_sms(message_id)
    }
}

/// Get call logs from the device.
#[tauri::command]
pub async fn get_call_logs<R: Runtime>(
    _app: AppHandle<R>,
    options: Option<CallLogQueryOptions>,
) -> Result<Vec<CallLog>> {
    let options = options.unwrap_or_default();

    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let manager = PlatformManager::new();
        manager.get_call_logs(options)
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let manager = PlatformManager;
        manager.get_call_logs(options)
    }
}

/// Get contacts from the device.
#[tauri::command]
pub async fn get_contacts<R: Runtime>(_app: AppHandle<R>) -> Result<Vec<Contact>> {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let manager = PlatformManager::new();
        manager.get_contacts()
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let manager = PlatformManager;
        manager.get_contacts()
    }
}

/// Request telephony permissions from the user.
#[tauri::command]
pub async fn request_permissions<R: Runtime>(
    _app: AppHandle<R>,
    permissions: Vec<Permission>,
) -> Result<PermissionStatus> {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let manager = PlatformManager::new();
        manager.request_permissions(permissions)
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let manager = PlatformManager;
        manager.request_permissions(permissions)
    }
}

/// Check the status of telephony permissions.
#[tauri::command]
pub async fn check_permissions<R: Runtime>(
    _app: AppHandle<R>,
    permissions: Vec<Permission>,
) -> Result<PermissionStatus> {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let manager = PlatformManager::new();
        manager.check_permissions(permissions)
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let manager = PlatformManager;
        manager.check_permissions(permissions)
    }
}

/// Validate a phone number format.
fn validate_phone_number(phone_number: &str) -> Result<()> {
    if phone_number.is_empty() {
        return Err(Error::InvalidPhoneNumber("Phone number cannot be empty".to_string()));
    }

    // Basic validation - should contain only digits, +, -, (, ), and spaces
    let valid_chars = phone_number.chars().all(|c| {
        c.is_ascii_digit() || c == '+' || c == '-' || c == '(' || c == ')' || c == ' '
    });

    if !valid_chars {
        return Err(Error::InvalidPhoneNumber(
            "Phone number contains invalid characters".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_phone_number() {
        assert!(validate_phone_number("+1234567890").is_ok());
        assert!(validate_phone_number("123-456-7890").is_ok());
        assert!(validate_phone_number("(123) 456-7890").is_ok());
        assert!(validate_phone_number("").is_err());
        assert!(validate_phone_number("abc123").is_err());
    }
}
