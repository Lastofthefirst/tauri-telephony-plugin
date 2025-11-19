use tauri_plugin_telephony::{
    models::*, CallLogQueryOptions, MessageQueryOptions, Permission,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_state_serialization() {
        let state = CallState::Ringing;
        let serialized = serde_json::to_string(&state).unwrap();
        assert_eq!(serialized, "\"RINGING\"");

        let deserialized: CallState = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, CallState::Ringing);
    }

    #[test]
    fn test_call_type_serialization() {
        let call_type = CallType::Incoming;
        let serialized = serde_json::to_string(&call_type).unwrap();
        assert_eq!(serialized, "\"INCOMING\"");

        let deserialized: CallType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, CallType::Incoming);
    }

    #[test]
    fn test_message_type_serialization() {
        let msg_type = MessageType::Inbox;
        let serialized = serde_json::to_string(&msg_type).unwrap();
        assert_eq!(serialized, "\"INBOX\"");

        let deserialized: MessageType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, MessageType::Inbox);
    }

    #[test]
    fn test_permission_serialization() {
        let permission = Permission::ReadPhoneState;
        let serialized = serde_json::to_string(&permission).unwrap();
        assert_eq!(serialized, "\"READ_PHONE_STATE\"");

        let deserialized: Permission = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, Permission::ReadPhoneState);
    }

    #[test]
    fn test_call_log_query_options() {
        let options = CallLogQueryOptions {
            limit: Some(50),
            offset: Some(0),
            call_type: Some(CallType::Incoming),
            start_date: Some(1234567890),
            end_date: Some(1234567900),
        };

        let serialized = serde_json::to_string(&options).unwrap();
        let deserialized: CallLogQueryOptions = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.limit, Some(50));
        assert_eq!(deserialized.call_type, Some(CallType::Incoming));
    }

    #[test]
    fn test_message_query_options() {
        let options = MessageQueryOptions {
            limit: Some(100),
            offset: Some(0),
            thread_id: Some("thread_123".to_string()),
            message_type: Some(MessageType::Inbox),
            start_date: None,
            end_date: None,
            unread_only: Some(true),
        };

        let serialized = serde_json::to_string(&options).unwrap();
        let deserialized: MessageQueryOptions = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.limit, Some(100));
        assert_eq!(deserialized.unread_only, Some(true));
        assert_eq!(deserialized.thread_id, Some("thread_123".to_string()));
    }

    #[test]
    fn test_sms_message_structure() {
        let message = SMSMessage {
            id: "msg_123".to_string(),
            thread_id: "thread_456".to_string(),
            sender: "+1234567890".to_string(),
            recipient: Some("+0987654321".to_string()),
            body: "Hello, World!".to_string(),
            timestamp: 1234567890,
            is_read: false,
            message_type: MessageType::Inbox,
        };

        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: SMSMessage = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.id, "msg_123");
        assert_eq!(deserialized.body, "Hello, World!");
        assert!(!deserialized.is_read);
    }

    #[test]
    fn test_call_log_structure() {
        let log = CallLog {
            id: "call_123".to_string(),
            phone_number: "+1234567890".to_string(),
            call_type: CallType::Outgoing,
            timestamp: 1234567890,
            duration: 120,
            contact_name: Some("John Doe".to_string()),
        };

        let serialized = serde_json::to_string(&log).unwrap();
        let deserialized: CallLog = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.id, "call_123");
        assert_eq!(deserialized.duration, 120);
        assert_eq!(deserialized.call_type, CallType::Outgoing);
    }

    #[test]
    fn test_contact_structure() {
        let contact = Contact {
            id: "contact_123".to_string(),
            name: "Jane Smith".to_string(),
            phone_numbers: vec![
                PhoneNumber {
                    number: "+1234567890".to_string(),
                    number_type: PhoneNumberType::Mobile,
                },
                PhoneNumber {
                    number: "+0987654321".to_string(),
                    number_type: PhoneNumberType::Home,
                },
            ],
            email_addresses: vec!["jane@example.com".to_string()],
        };

        let serialized = serde_json::to_string(&contact).unwrap();
        let deserialized: Contact = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.name, "Jane Smith");
        assert_eq!(deserialized.phone_numbers.len(), 2);
        assert_eq!(deserialized.email_addresses.len(), 1);
    }

    #[test]
    fn test_permission_status() {
        use std::collections::HashMap;

        let mut permissions = HashMap::new();
        permissions.insert(Permission::ReadPhoneState, PermissionState::Granted);
        permissions.insert(Permission::CallPhone, PermissionState::Denied);
        permissions.insert(Permission::SendSms, PermissionState::Prompt);

        let status = PermissionStatus { permissions };

        let serialized = serde_json::to_string(&status).unwrap();
        let deserialized: PermissionStatus = serde_json::from_str(&serialized).unwrap();

        assert_eq!(
            deserialized.permissions.get(&Permission::ReadPhoneState),
            Some(&PermissionState::Granted)
        );
        assert_eq!(
            deserialized.permissions.get(&Permission::CallPhone),
            Some(&PermissionState::Denied)
        );
    }

    #[test]
    fn test_incoming_call_event() {
        let event = IncomingCallEvent {
            call_id: "call_123".to_string(),
            phone_number: "+1234567890".to_string(),
            contact_name: Some("John Doe".to_string()),
            timestamp: 1234567890,
        };

        let serialized = serde_json::to_string(&event).unwrap();
        let deserialized: IncomingCallEvent = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.call_id, "call_123");
        assert_eq!(deserialized.contact_name, Some("John Doe".to_string()));
    }

    #[test]
    fn test_call_state_event() {
        let event = CallStateEvent {
            call_id: "call_123".to_string(),
            state: CallState::Active,
            phone_number: "+1234567890".to_string(),
        };

        let serialized = serde_json::to_string(&event).unwrap();
        let deserialized: CallStateEvent = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.state, CallState::Active);
    }

    #[test]
    fn test_sms_received_event() {
        let event = SMSReceivedEvent {
            message_id: "msg_123".to_string(),
            sender: "+1234567890".to_string(),
            body: "Hello!".to_string(),
            timestamp: 1234567890,
        };

        let serialized = serde_json::to_string(&event).unwrap();
        let deserialized: SMSReceivedEvent = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.body, "Hello!");
    }

    #[test]
    fn test_sms_sent_event() {
        let event = SMSSentEvent {
            message_id: "msg_123".to_string(),
            recipient: "+1234567890".to_string(),
            success: true,
            error: None,
        };

        let serialized = serde_json::to_string(&event).unwrap();
        let deserialized: SMSSentEvent = serde_json::from_str(&serialized).unwrap();

        assert!(deserialized.success);
        assert_eq!(deserialized.error, None);

        let event_fail = SMSSentEvent {
            message_id: "msg_456".to_string(),
            recipient: "+1234567890".to_string(),
            success: false,
            error: Some("Network error".to_string()),
        };

        let serialized = serde_json::to_string(&event_fail).unwrap();
        let deserialized: SMSSentEvent = serde_json::from_str(&serialized).unwrap();

        assert!(!deserialized.success);
        assert_eq!(deserialized.error, Some("Network error".to_string()));
    }
}
