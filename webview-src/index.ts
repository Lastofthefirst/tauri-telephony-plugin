import { invoke } from '@tauri-apps/api/core';
import { listen, UnlistenFn } from '@tauri-apps/api/event';

// Types

export enum CallState {
  IDLE = 'IDLE',
  RINGING = 'RINGING',
  OFFHOOK = 'OFFHOOK',
  DIALING = 'DIALING',
  ACTIVE = 'ACTIVE',
  HOLDING = 'HOLDING',
  DISCONNECTED = 'DISCONNECTED',
  CONNECTING = 'CONNECTING',
}

export enum CallType {
  INCOMING = 'INCOMING',
  OUTGOING = 'OUTGOING',
  MISSED = 'MISSED',
  REJECTED = 'REJECTED',
  BLOCKED = 'BLOCKED',
  VOICEMAIL = 'VOICEMAIL',
}

export enum MessageType {
  INBOX = 'INBOX',
  SENT = 'SENT',
  DRAFT = 'DRAFT',
  OUTBOX = 'OUTBOX',
  FAILED = 'FAILED',
  QUEUED = 'QUEUED',
}

export enum PhoneNumberType {
  MOBILE = 'MOBILE',
  HOME = 'HOME',
  WORK = 'WORK',
  OTHER = 'OTHER',
  MAIN = 'MAIN',
  FAX_WORK = 'FAX_WORK',
  FAX_HOME = 'FAX_HOME',
  PAGER = 'PAGER',
}

export enum Permission {
  READ_PHONE_STATE = 'READ_PHONE_STATE',
  CALL_PHONE = 'CALL_PHONE',
  READ_CALL_LOG = 'READ_CALL_LOG',
  WRITE_CALL_LOG = 'WRITE_CALL_LOG',
  SEND_SMS = 'SEND_SMS',
  RECEIVE_SMS = 'RECEIVE_SMS',
  READ_SMS = 'READ_SMS',
  READ_CONTACTS = 'READ_CONTACTS',
  ANSWER_PHONE_CALLS = 'ANSWER_PHONE_CALLS',
}

export enum PermissionState {
  GRANTED = 'GRANTED',
  DENIED = 'DENIED',
  PROMPT = 'PROMPT',
}

export interface Call {
  id: string;
  phoneNumber: string;
  state: CallState;
  startTime?: number;
  duration?: number;
  contactName?: string;
}

export interface CallLog {
  id: string;
  phoneNumber: string;
  callType: CallType;
  timestamp: number;
  duration: number;
  contactName?: string;
}

export interface CallLogQueryOptions {
  limit?: number;
  offset?: number;
  callType?: CallType;
  startDate?: number;
  endDate?: number;
}

export interface SMSOptions {
  recipient: string;
  message: string;
}

export interface SMSMessage {
  id: string;
  threadId: string;
  sender: string;
  recipient?: string;
  body: string;
  timestamp: number;
  isRead: boolean;
  messageType: MessageType;
}

export interface MessageQueryOptions {
  limit?: number;
  offset?: number;
  threadId?: string;
  messageType?: MessageType;
  startDate?: number;
  endDate?: number;
  unreadOnly?: boolean;
}

export interface Contact {
  id: string;
  name: string;
  phoneNumbers: PhoneNumber[];
  emailAddresses: string[];
}

export interface PhoneNumber {
  number: string;
  numberType: PhoneNumberType;
}

export interface PermissionStatus {
  permissions: Record<Permission, PermissionState>;
}

export interface IncomingCallEvent {
  callId: string;
  phoneNumber: string;
  contactName?: string;
  timestamp: number;
}

export interface CallStateEvent {
  callId: string;
  state: CallState;
  phoneNumber: string;
}

export interface SMSReceivedEvent {
  messageId: string;
  sender: string;
  body: string;
  timestamp: number;
}

export interface SMSSentEvent {
  messageId: string;
  recipient: string;
  success: boolean;
  error?: string;
}

// Call Management Functions

/**
 * Make a phone call to the specified number.
 * Requires CALL_PHONE permission on Android.
 *
 * @param phoneNumber - The phone number to call
 * @throws If permission is denied or call fails
 */
export async function makeCall(phoneNumber: string): Promise<void> {
  return invoke('plugin:telephony|make_call', { phoneNumber });
}

/**
 * Answer an incoming call.
 * Requires ANSWER_PHONE_CALLS permission on Android.
 * Note: Limited support on iOS.
 *
 * @param callId - The ID of the call to answer
 * @throws If permission is denied or operation fails
 */
export async function answerCall(callId: string): Promise<void> {
  return invoke('plugin:telephony|answer_call', { callId });
}

/**
 * Reject an incoming call.
 * Requires ANSWER_PHONE_CALLS permission on Android.
 * Note: Limited support on iOS.
 *
 * @param callId - The ID of the call to reject
 * @throws If permission is denied or operation fails
 */
export async function rejectCall(callId: string): Promise<void> {
  return invoke('plugin:telephony|reject_call', { callId });
}

/**
 * End an active call.
 * Requires ANSWER_PHONE_CALLS permission on Android.
 * Note: Limited support on iOS.
 *
 * @param callId - The ID of the call to end
 * @throws If permission is denied or operation fails
 */
export async function endCall(callId: string): Promise<void> {
  return invoke('plugin:telephony|end_call', { callId });
}

/**
 * Get the current call state.
 * Requires READ_PHONE_STATE permission on Android.
 *
 * @returns The current call state
 * @throws If permission is denied
 */
export async function getCallState(): Promise<CallState> {
  return invoke('plugin:telephony|get_call_state');
}

// SMS Functions

/**
 * Send an SMS message.
 * Requires SEND_SMS permission on Android.
 * Note: On iOS, requires user interaction to confirm.
 *
 * @param options - SMS options including recipient and message
 * @returns The message ID
 * @throws If permission is denied or send fails
 */
export async function sendSMS(options: SMSOptions): Promise<string> {
  return invoke('plugin:telephony|send_sms', { options });
}

/**
 * Get SMS messages from the device.
 * Requires READ_SMS permission on Android.
 * Note: Not supported on iOS due to privacy restrictions.
 *
 * @param options - Query options for filtering messages
 * @returns Array of SMS messages
 * @throws If permission is denied or platform not supported
 */
export async function getSMSMessages(
  options?: MessageQueryOptions
): Promise<SMSMessage[]> {
  return invoke('plugin:telephony|get_sms_messages', { options });
}

/**
 * Delete an SMS message.
 * Requires WRITE_SMS permission on Android.
 * Note: Not supported on iOS.
 *
 * @param messageId - The ID of the message to delete
 * @throws If permission is denied or platform not supported
 */
export async function deleteSMS(messageId: string): Promise<void> {
  return invoke('plugin:telephony|delete_sms', { messageId });
}

// Call Log Functions

/**
 * Get call logs from the device.
 * Requires READ_CALL_LOG permission on Android.
 * Note: Limited support on iOS.
 *
 * @param options - Query options for filtering call logs
 * @returns Array of call log entries
 * @throws If permission is denied
 */
export async function getCallLogs(
  options?: CallLogQueryOptions
): Promise<CallLog[]> {
  return invoke('plugin:telephony|get_call_logs', { options });
}

// Contact Functions

/**
 * Get contacts from the device.
 * Requires READ_CONTACTS permission.
 *
 * @returns Array of contacts
 * @throws If permission is denied
 */
export async function getContacts(): Promise<Contact[]> {
  return invoke('plugin:telephony|get_contacts');
}

// Permission Functions

/**
 * Request telephony permissions from the user.
 *
 * @param permissions - Array of permissions to request
 * @returns Permission status for each requested permission
 */
export async function requestPermissions(
  permissions: Permission[]
): Promise<PermissionStatus> {
  return invoke('plugin:telephony|request_permissions', { permissions });
}

/**
 * Check the current status of telephony permissions.
 *
 * @param permissions - Array of permissions to check
 * @returns Permission status for each permission
 */
export async function checkPermissions(
  permissions: Permission[]
): Promise<PermissionStatus> {
  return invoke('plugin:telephony|check_permissions', { permissions });
}

// Event Listeners

/**
 * Listen for incoming call events.
 *
 * @param handler - Callback function to handle incoming call events
 * @returns Function to unlisten
 */
export async function onIncomingCall(
  handler: (event: IncomingCallEvent) => void
): Promise<UnlistenFn> {
  return listen<IncomingCallEvent>('telephony://incoming-call', (event) => {
    handler(event.payload);
  });
}

/**
 * Listen for call state change events.
 *
 * @param handler - Callback function to handle call state changes
 * @returns Function to unlisten
 */
export async function onCallStateChanged(
  handler: (event: CallStateEvent) => void
): Promise<UnlistenFn> {
  return listen<CallStateEvent>('telephony://call-state-changed', (event) => {
    handler(event.payload);
  });
}

/**
 * Listen for received SMS events.
 *
 * @param handler - Callback function to handle received SMS
 * @returns Function to unlisten
 */
export async function onSMSReceived(
  handler: (event: SMSReceivedEvent) => void
): Promise<UnlistenFn> {
  return listen<SMSReceivedEvent>('telephony://sms-received', (event) => {
    handler(event.payload);
  });
}

/**
 * Listen for SMS sent events.
 *
 * @param handler - Callback function to handle sent SMS status
 * @returns Function to unlisten
 */
export async function onSMSSent(
  handler: (event: SMSSentEvent) => void
): Promise<UnlistenFn> {
  return listen<SMSSentEvent>('telephony://sms-sent', (event) => {
    handler(event.payload);
  });
}

// Main Telephony class for convenience

export class Telephony {
  // Call Management
  makeCall = makeCall;
  answerCall = answerCall;
  rejectCall = rejectCall;
  endCall = endCall;
  getCallState = getCallState;

  // SMS
  sendSMS = sendSMS;
  getSMSMessages = getSMSMessages;
  deleteSMS = deleteSMS;

  // Call Logs
  getCallLogs = getCallLogs;

  // Contacts
  getContacts = getContacts;

  // Permissions
  requestPermissions = requestPermissions;
  checkPermissions = checkPermissions;

  // Events
  onIncomingCall = onIncomingCall;
  onCallStateChanged = onCallStateChanged;
  onSMSReceived = onSMSReceived;
  onSMSSent = onSMSSent;
}

// Default export
export default {
  Telephony,
  makeCall,
  answerCall,
  rejectCall,
  endCall,
  getCallState,
  sendSMS,
  getSMSMessages,
  deleteSMS,
  getCallLogs,
  getContacts,
  requestPermissions,
  checkPermissions,
  onIncomingCall,
  onCallStateChanged,
  onSMSReceived,
  onSMSSent,
};
