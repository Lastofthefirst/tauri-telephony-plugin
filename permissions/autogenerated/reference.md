# Telephony Plugin Permissions

This directory contains auto-generated permission definitions for the Tauri Telephony Plugin.

## Available Permissions

### Call Management Permissions

- `telephony:allow-make-call` - Allows making phone calls
- `telephony:allow-answer-call` - Allows answering incoming calls
- `telephony:allow-reject-call` - Allows rejecting incoming calls
- `telephony:allow-end-call` - Allows ending active calls
- `telephony:allow-get-call-state` - Allows reading the current call state
- `telephony:allow-get-call-logs` - Allows accessing call logs

### SMS Permissions

- `telephony:allow-send-sms` - Allows sending SMS messages
- `telephony:allow-get-sms-messages` - Allows reading SMS messages
- `telephony:allow-delete-sms` - Allows deleting SMS messages

### Contact Permissions

- `telephony:allow-get-contacts` - Allows accessing device contacts

### Permission Management

- `telephony:allow-request-permissions` - Allows requesting telephony permissions
- `telephony:allow-check-permissions` - Allows checking permission status

## Usage

To use these permissions in your Tauri application, add them to your `tauri.conf.json`:

```json
{
  "plugins": {
    "telephony": {
      "permissions": [
        "telephony:allow-make-call",
        "telephony:allow-send-sms",
        "telephony:allow-get-contacts"
      ]
    }
  }
}
```

Or grant all permissions:

```json
{
  "plugins": {
    "telephony": {
      "permissions": ["telephony:default"]
    }
  }
}
```
