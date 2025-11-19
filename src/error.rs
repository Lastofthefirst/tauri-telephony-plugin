use serde::{Deserialize, Serialize};
use std::fmt;

/// Result type for telephony operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in telephony operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid phone number: {0}")]
    InvalidPhoneNumber(String),

    #[error("Invalid message: {0}")]
    InvalidMessage(String),

    #[error("Call not found: {0}")]
    CallNotFound(String),

    #[error("Message not found: {0}")]
    MessageNotFound(String),

    #[error("Operation failed: {0}")]
    OperationFailed(String),

    #[error("Platform not supported: {0}")]
    PlatformNotSupported(String),

    #[error("Failed to serialize/deserialize: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Error {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Error::Unknown(s))
    }
}

// Implement conversion from tauri::Error if needed
impl From<tauri::Error> for Error {
    fn from(err: tauri::Error) -> Self {
        Error::Unknown(err.to_string())
    }
}
