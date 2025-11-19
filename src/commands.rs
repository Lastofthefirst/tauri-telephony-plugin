use crate::{
    error::{Error, Result},
    models::*,
};
use tauri::{AppHandle, Runtime};

/// Make a phone call to the specified number.
#[tauri::command]
pub async fn make_call<R: Runtime>(
    app: AppHandle<R>,
    phone_number: String,
) -> Result<()> {
    validate_phone_number(&phone_number)?;

    #[cfg(mobile)]
    {
        app.run_mobile_plugin("make_call", phone_number)
            .map_err(Into::into)
    }

    #[cfg(desktop)]
    {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }
}

/// Answer an incoming call.
#[tauri::command]
pub async fn answer_call<R: Runtime>(
    app: AppHandle<R>,
    call_id: String,
) -> Result<()> {
    #[cfg(mobile)]
    {
        app.run_mobile_plugin("answer_call", call_id)
            .map_err(Into::into)
    }

    #[cfg(desktop)]
    {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }
}

/// Reject an incoming call.
#[tauri::command]
pub async fn reject_call<R: Runtime>(
    app: AppHandle<R>,
    call_id: String,
) -> Result<()> {
    #[cfg(mobile)]
    {
        app.run_mobile_plugin("reject_call", call_id)
            .map_err(Into::into)
    }

    #[cfg(desktop)]
    {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }
}

/// End an active call.
#[tauri::command]
pub async fn end_call<R: Runtime>(
    app: AppHandle<R>,
    call_id: String,
) -> Result<()> {
    #[cfg(mobile)]
    {
        app.run_mobile_plugin("end_call", call_id)
            .map_err(Into::into)
    }

    #[cfg(desktop)]
    {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }
}

/// Get the current call state.
#[tauri::command]
pub async fn get_call_state<R: Runtime>(app: AppHandle<R>) -> Result<CallState> {
    #[cfg(mobile)]
    {
        app.run_mobile_plugin::<()>("get_call_state", ())
            .map_err(Into::into)
    }

    #[cfg(desktop)]
    {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }
}

/// Send an SMS message.
#[tauri::command]
pub async fn send_sms<R: Runtime>(
    app: AppHandle<R>,
    options: SMSOptions,
) -> Result<String> {
    validate_phone_number(&options.recipient)?;

    if options.message.is_empty() {
        return Err(Error::InvalidMessage("Message cannot be empty".to_string()));
    }

    #[cfg(mobile)]
    {
        app.run_mobile_plugin("send_sms", options)
            .map_err(Into::into)
    }

    #[cfg(desktop)]
    {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }
}

/// Get SMS messages from the device.
#[tauri::command]
pub async fn get_sms_messages<R: Runtime>(
    app: AppHandle<R>,
    options: Option<MessageQueryOptions>,
) -> Result<Vec<SMSMessage>> {
    let options = options.unwrap_or_default();

    #[cfg(mobile)]
    {
        app.run_mobile_plugin("get_sms_messages", options)
            .map_err(Into::into)
    }

    #[cfg(desktop)]
    {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }
}

/// Delete an SMS message.
#[tauri::command]
pub async fn delete_sms<R: Runtime>(
    app: AppHandle<R>,
    message_id: String,
) -> Result<()> {
    #[cfg(mobile)]
    {
        app.run_mobile_plugin("delete_sms", message_id)
            .map_err(Into::into)
    }

    #[cfg(desktop)]
    {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }
}

/// Get call logs from the device.
#[tauri::command]
pub async fn get_call_logs<R: Runtime>(
    app: AppHandle<R>,
    options: Option<CallLogQueryOptions>,
) -> Result<Vec<CallLog>> {
    let options = options.unwrap_or_default();

    #[cfg(mobile)]
    {
        app.run_mobile_plugin("get_call_logs", options)
            .map_err(Into::into)
    }

    #[cfg(desktop)]
    {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }
}

/// Get contacts from the device.
#[tauri::command]
pub async fn get_contacts<R: Runtime>(app: AppHandle<R>) -> Result<Vec<Contact>> {
    #[cfg(mobile)]
    {
        app.run_mobile_plugin::<()>("get_contacts", ())
            .map_err(Into::into)
    }

    #[cfg(desktop)]
    {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }
}

/// Request telephony permissions from the user.
#[tauri::command]
pub async fn request_permissions<R: Runtime>(
    app: AppHandle<R>,
    permissions: Vec<Permission>,
) -> Result<PermissionStatus> {
    #[cfg(mobile)]
    {
        app.run_mobile_plugin("request_permissions", permissions)
            .map_err(Into::into)
    }

    #[cfg(desktop)]
    {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
    }
}

/// Check the status of telephony permissions.
#[tauri::command]
pub async fn check_permissions<R: Runtime>(
    app: AppHandle<R>,
    permissions: Vec<Permission>,
) -> Result<PermissionStatus> {
    #[cfg(mobile)]
    {
        app.run_mobile_plugin("check_permissions", permissions)
            .map_err(Into::into)
    }

    #[cfg(desktop)]
    {
        Err(Error::PlatformNotSupported(
            "Telephony is only supported on Android and iOS".to_string(),
        ))
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
