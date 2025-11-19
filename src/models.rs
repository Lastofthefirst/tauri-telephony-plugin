use serde::{Deserialize, Serialize};

/// Represents the state of a phone call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CallState {
    Idle,
    Ringing,
    Offhook,
    Dialing,
    Active,
    Holding,
    Disconnected,
    Connecting,
}

/// Represents the type of a call in the call log.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CallType {
    Incoming,
    Outgoing,
    Missed,
    Rejected,
    Blocked,
    Voicemail,
}

/// Represents a phone call.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Call {
    pub id: String,
    pub phone_number: String,
    pub state: CallState,
    pub start_time: Option<i64>,
    pub duration: Option<u32>,
    pub contact_name: Option<String>,
}

/// Options for making a call.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MakeCallOptions {
    pub phone_number: String,
}

/// A call log entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallLog {
    pub id: String,
    pub phone_number: String,
    pub call_type: CallType,
    pub timestamp: i64,
    pub duration: u32,
    pub contact_name: Option<String>,
}

/// Options for querying call logs.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CallLogQueryOptions {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub call_type: Option<CallType>,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
}

/// Options for sending an SMS.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SMSOptions {
    pub recipient: String,
    pub message: String,
}

/// Represents an SMS message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SMSMessage {
    pub id: String,
    pub thread_id: String,
    pub sender: String,
    pub recipient: Option<String>,
    pub body: String,
    pub timestamp: i64,
    pub is_read: bool,
    pub message_type: MessageType,
}

/// Type of SMS message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MessageType {
    Inbox,
    Sent,
    Draft,
    Outbox,
    Failed,
    Queued,
}

/// Options for querying SMS messages.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MessageQueryOptions {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub thread_id: Option<String>,
    pub message_type: Option<MessageType>,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    pub unread_only: Option<bool>,
}

/// Represents a contact from the device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub id: String,
    pub name: String,
    pub phone_numbers: Vec<PhoneNumber>,
    pub email_addresses: Vec<String>,
}

/// A phone number associated with a contact.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhoneNumber {
    pub number: String,
    pub number_type: PhoneNumberType,
}

/// Type of phone number.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PhoneNumberType {
    Mobile,
    Home,
    Work,
    Other,
    Main,
    FaxWork,
    FaxHome,
    Pager,
}

/// Telephony permissions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Permission {
    ReadPhoneState,
    CallPhone,
    ReadCallLog,
    WriteCallLog,
    SendSms,
    ReceiveSms,
    ReadSms,
    ReadContacts,
    AnswerPhoneCalls,
}

/// Status of a permission.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PermissionState {
    Granted,
    Denied,
    Prompt,
}

/// Permission status response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionStatus {
    pub permissions: std::collections::HashMap<Permission, PermissionState>,
}

/// Event payload for incoming calls.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomingCallEvent {
    pub call_id: String,
    pub phone_number: String,
    pub contact_name: Option<String>,
    pub timestamp: i64,
}

/// Event payload for call state changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallStateEvent {
    pub call_id: String,
    pub state: CallState,
    pub phone_number: String,
}

/// Event payload for received SMS.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SMSReceivedEvent {
    pub message_id: String,
    pub sender: String,
    pub body: String,
    pub timestamp: i64,
}

/// Event payload for sent SMS.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SMSSentEvent {
    pub message_id: String,
    pub recipient: String,
    pub success: bool,
    pub error: Option<String>,
}
