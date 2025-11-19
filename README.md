# Tauri Plugin Telephony

A robust and comprehensive Tauri v2 plugin that provides telephony functionality for mobile applications, enabling developers to build custom ringer apps and SMS applications on Android and iOS.

## Features

### Call Management
- 📞 **Incoming Call Handling**: Detect, answer, and reject incoming calls
- 📱 **Outgoing Calls**: Initiate phone calls programmatically
- 🔊 **Call State Monitoring**: Track call states (ringing, active, ended, etc.)
- 🎯 **Custom Ringer**: Build custom call screening and ringer UIs
- 📋 **Call Log Access**: Read and manage call history

### SMS/MMS Messaging
- 💬 **Send SMS**: Send text messages programmatically
- 📨 **Receive SMS**: Listen for incoming messages
- 📖 **Read Messages**: Access SMS inbox and conversation threads
- 🗑️ **Delete Messages**: Manage message storage
- 📎 **MMS Support**: Send and receive multimedia messages (Android)

### Additional Features
- 👥 **Contact Integration**: Access device contacts
- 🔔 **Permission Management**: Request and check telephony permissions
- 🔄 **Event System**: Real-time events for calls, messages, and state changes
- 🛡️ **Error Handling**: Comprehensive error types with detailed messages
- 🧪 **E2E Tested**: Full test coverage with integration tests

## Platform Support

| Feature | Android | iOS |
|---------|---------|-----|
| Incoming Calls | ✅ | ✅ |
| Outgoing Calls | ✅ | ✅ |
| Call State | ✅ | ✅ |
| SMS Send | ✅ | ✅ |
| SMS Receive | ✅ | ✅ |
| MMS | ✅ | ⚠️ Limited |
| Call Logs | ✅ | ⚠️ Limited |
| Contacts | ✅ | ✅ |

## Installation

```bash
npm install tauri-plugin-telephony
# or
yarn add tauri-plugin-telephony
# or
pnpm add tauri-plugin-telephony
```

## Setup

### Rust (`src-tauri/src/main.rs`)

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_telephony::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### JavaScript/TypeScript

```typescript
import { Telephony } from 'tauri-plugin-telephony';

// Initialize telephony
const telephony = new Telephony();
```

## Usage Examples

### Making a Phone Call

```typescript
import { makeCall } from 'tauri-plugin-telephony';

// Make a call
await makeCall('+1234567890');
```

### Sending an SMS

```typescript
import { sendSMS } from 'tauri-plugin-telephony';

// Send a text message
await sendSMS({
  recipient: '+1234567890',
  message: 'Hello from Tauri!'
});
```

### Listening for Incoming Calls

```typescript
import { listen } from '@tauri-apps/api/event';

// Listen for incoming call events
const unlisten = await listen('telephony://incoming-call', (event) => {
  console.log('Incoming call from:', event.payload.phoneNumber);
  console.log('Call state:', event.payload.state);
});

// Later, stop listening
unlisten();
```

### Monitoring SMS Messages

```typescript
import { listen } from '@tauri-apps/api/event';

// Listen for new SMS messages
const unlisten = await listen('telephony://sms-received', (event) => {
  console.log('New message from:', event.payload.sender);
  console.log('Message:', event.payload.body);
  console.log('Timestamp:', event.payload.timestamp);
});
```

### Accessing Call Logs

```typescript
import { getCallLogs } from 'tauri-plugin-telephony';

// Get recent call logs
const logs = await getCallLogs({ limit: 50 });

logs.forEach(log => {
  console.log(`${log.type} - ${log.phoneNumber} - ${log.duration}s`);
});
```

### Reading SMS Messages

```typescript
import { getSMSMessages } from 'tauri-plugin-telephony';

// Get recent messages
const messages = await getSMSMessages({ limit: 100 });

messages.forEach(msg => {
  console.log(`${msg.sender}: ${msg.body}`);
});
```

## Permissions

### Android (`src-tauri/gen/android/app/src/main/AndroidManifest.xml`)

```xml
<uses-permission android:name="android.permission.READ_PHONE_STATE" />
<uses-permission android:name="android.permission.CALL_PHONE" />
<uses-permission android:name="android.permission.READ_CALL_LOG" />
<uses-permission android:name="android.permission.WRITE_CALL_LOG" />
<uses-permission android:name="android.permission.SEND_SMS" />
<uses-permission android:name="android.permission.RECEIVE_SMS" />
<uses-permission android:name="android.permission.READ_SMS" />
<uses-permission android:name="android.permission.READ_CONTACTS" />
<uses-permission android:name="android.permission.ANSWER_PHONE_CALLS" />
```

### iOS (`src-tauri/gen/apple/Info.plist`)

```xml
<key>NSContactsUsageDescription</key>
<string>This app needs access to contacts for telephony features</string>
<key>NSTelephonyUsageDescription</key>
<string>This app needs access to make phone calls</string>
```

## API Reference

### Commands

#### `makeCall(phoneNumber: string): Promise<void>`
Initiate a phone call to the specified number.

#### `answerCall(callId: string): Promise<void>`
Answer an incoming call.

#### `rejectCall(callId: string): Promise<void>`
Reject an incoming call.

#### `endCall(callId: string): Promise<void>`
End an active call.

#### `sendSMS(options: SMSOptions): Promise<void>`
Send an SMS message.

#### `getSMSMessages(options?: MessageQueryOptions): Promise<SMSMessage[]>`
Retrieve SMS messages from the device.

#### `getCallLogs(options?: CallLogQueryOptions): Promise<CallLog[]>`
Retrieve call log entries.

#### `requestPermissions(permissions: Permission[]): Promise<PermissionStatus>`
Request telephony permissions from the user.

#### `checkPermissions(permissions: Permission[]): Promise<PermissionStatus>`
Check the current status of telephony permissions.

### Events

#### `telephony://incoming-call`
Fired when an incoming call is detected.

#### `telephony://call-state-changed`
Fired when the call state changes.

#### `telephony://sms-received`
Fired when a new SMS is received.

#### `telephony://sms-sent`
Fired when an SMS is successfully sent.

## Building a Custom Ringer App

See the [example application](./examples/custom-ringer/) for a complete implementation of a custom ringer app that:
- Intercepts incoming calls
- Displays custom UI for call screening
- Allows answer/reject with custom buttons
- Shows caller ID and contact information

## Error Handling

All commands return proper errors that can be caught and handled:

```typescript
try {
  await makeCall('+1234567890');
} catch (error) {
  if (error.includes('PERMISSION_DENIED')) {
    console.error('Please grant phone permissions');
  } else if (error.includes('INVALID_NUMBER')) {
    console.error('Invalid phone number format');
  }
}
```

## Contributing

Contributions are welcome! Please read our [Contributing Guide](./CONTRIBUTING.md) for details.

## License

MIT OR Apache-2.0
