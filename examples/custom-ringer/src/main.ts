import {
  makeCall,
  sendSMS,
  getCallLogs,
  getSMSMessages,
  getContacts,
  requestPermissions,
  checkPermissions,
  answerCall,
  rejectCall,
  onIncomingCall,
  onCallStateChanged,
  onSMSReceived,
  onSMSSent,
  Permission,
  CallLog,
  SMSMessage,
  Contact,
  IncomingCallEvent,
} from '../../../webview-src/index';

// State
let currentCallId: string | null = null;

// Initialize
document.addEventListener('DOMContentLoaded', async () => {
  setupTabs();
  setupEventListeners();
  await loadPermissions();
  setupTelephonyEvents();
});

// Tab Management
function setupTabs() {
  const tabs = document.querySelectorAll('.tab');
  const tabContents = document.querySelectorAll('.tab-content');

  tabs.forEach((tab) => {
    tab.addEventListener('click', () => {
      const tabName = (tab as HTMLElement).dataset.tab;

      tabs.forEach((t) => t.classList.remove('active'));
      tabContents.forEach((tc) => tc.classList.remove('active'));

      tab.classList.add('active');
      document.getElementById(tabName!)?.classList.add('active');
    });
  });
}

// Event Listeners
function setupEventListeners() {
  // Calls
  document.getElementById('makeCallBtn')?.addEventListener('click', handleMakeCall);
  document.getElementById('loadCallLogsBtn')?.addEventListener('click', handleLoadCallLogs);
  document.getElementById('answerCallBtn')?.addEventListener('click', handleAnswerCall);
  document.getElementById('rejectCallBtn')?.addEventListener('click', handleRejectCall);

  // SMS
  document.getElementById('sendSMSBtn')?.addEventListener('click', handleSendSMS);
  document.getElementById('loadMessagesBtn')?.addEventListener('click', handleLoadMessages);

  // Contacts
  document.getElementById('loadContactsBtn')?.addEventListener('click', handleLoadContacts);

  // Permissions
  document
    .getElementById('requestPermissionsBtn')
    ?.addEventListener('click', handleRequestPermissions);
}

// Call Handlers
async function handleMakeCall() {
  const input = document.getElementById('callNumber') as HTMLInputElement;
  const phoneNumber = input.value.trim();

  if (!phoneNumber) {
    showStatus('calls', 'Please enter a phone number', 'error');
    return;
  }

  try {
    await makeCall(phoneNumber);
    showStatus('calls', `Calling ${phoneNumber}...`, 'success');
    input.value = '';
  } catch (error) {
    showStatus('calls', `Failed to make call: ${error}`, 'error');
  }
}

async function handleLoadCallLogs() {
  try {
    const logs = await getCallLogs({ limit: 50 });
    displayCallLogs(logs);
    showStatus('calls', `Loaded ${logs.length} call logs`, 'success');
  } catch (error) {
    showStatus('calls', `Failed to load call logs: ${error}`, 'error');
  }
}

async function handleAnswerCall() {
  if (!currentCallId) return;

  try {
    await answerCall(currentCallId);
    hideIncomingCallOverlay();
    showStatus('calls', 'Call answered', 'success');
  } catch (error) {
    showStatus('calls', `Failed to answer call: ${error}`, 'error');
  }
}

async function handleRejectCall() {
  if (!currentCallId) return;

  try {
    await rejectCall(currentCallId);
    hideIncomingCallOverlay();
    showStatus('calls', 'Call rejected', 'info');
  } catch (error) {
    showStatus('calls', `Failed to reject call: ${error}`, 'error');
  }
}

// SMS Handlers
async function handleSendSMS() {
  const numberInput = document.getElementById('smsNumber') as HTMLInputElement;
  const messageInput = document.getElementById('smsMessage') as HTMLTextAreaElement;

  const recipient = numberInput.value.trim();
  const message = messageInput.value.trim();

  if (!recipient || !message) {
    showStatus('messages', 'Please enter both recipient and message', 'error');
    return;
  }

  try {
    await sendSMS({ recipient, message });
    showStatus('messages', `SMS sent to ${recipient}`, 'success');
    numberInput.value = '';
    messageInput.value = '';
  } catch (error) {
    showStatus('messages', `Failed to send SMS: ${error}`, 'error');
  }
}

async function handleLoadMessages() {
  try {
    const messages = await getSMSMessages({ limit: 50 });
    displayMessages(messages);
    showStatus('messages', `Loaded ${messages.length} messages`, 'success');
  } catch (error) {
    showStatus('messages', `Failed to load messages: ${error}`, 'error');
  }
}

// Contact Handlers
async function handleLoadContacts() {
  try {
    const contacts = await getContacts();
    displayContacts(contacts);
    showStatus('contacts', `Loaded ${contacts.length} contacts`, 'success');
  } catch (error) {
    showStatus('contacts', `Failed to load contacts: ${error}`, 'error');
  }
}

// Permission Handlers
async function handleRequestPermissions() {
  const allPermissions = [
    Permission.READ_PHONE_STATE,
    Permission.CALL_PHONE,
    Permission.READ_CALL_LOG,
    Permission.WRITE_CALL_LOG,
    Permission.SEND_SMS,
    Permission.RECEIVE_SMS,
    Permission.READ_SMS,
    Permission.READ_CONTACTS,
    Permission.ANSWER_PHONE_CALLS,
  ];

  try {
    await requestPermissions(allPermissions);
    await loadPermissions();
    showStatus('permissions', 'Permissions requested', 'success');
  } catch (error) {
    showStatus('permissions', `Failed to request permissions: ${error}`, 'error');
  }
}

async function loadPermissions() {
  const allPermissions = [
    Permission.READ_PHONE_STATE,
    Permission.CALL_PHONE,
    Permission.READ_CALL_LOG,
    Permission.WRITE_CALL_LOG,
    Permission.SEND_SMS,
    Permission.RECEIVE_SMS,
    Permission.READ_SMS,
    Permission.READ_CONTACTS,
    Permission.ANSWER_PHONE_CALLS,
  ];

  try {
    const status = await checkPermissions(allPermissions);
    displayPermissions(status.permissions);
  } catch (error) {
    console.error('Failed to load permissions:', error);
  }
}

// Telephony Events
function setupTelephonyEvents() {
  // Incoming call
  onIncomingCall((event: IncomingCallEvent) => {
    console.log('Incoming call:', event);
    currentCallId = event.callId;
    showIncomingCallOverlay(event.phoneNumber, event.contactName);
  });

  // Call state changed
  onCallStateChanged((event) => {
    console.log('Call state changed:', event);
    if (event.state === 'DISCONNECTED' || event.state === 'IDLE') {
      hideIncomingCallOverlay();
      currentCallId = null;
    }
  });

  // SMS received
  onSMSReceived((event) => {
    console.log('SMS received:', event);
    showStatus('messages', `New message from ${event.sender}`, 'info');

    // Show notification
    if ('Notification' in window && Notification.permission === 'granted') {
      new Notification('New SMS', {
        body: `${event.sender}: ${event.body}`,
      });
    }
  });

  // SMS sent
  onSMSSent((event) => {
    console.log('SMS sent:', event);
    if (event.success) {
      showStatus('messages', `SMS delivered to ${event.recipient}`, 'success');
    } else {
      showStatus('messages', `Failed to send SMS: ${event.error}`, 'error');
    }
  });
}

// Display Functions
function displayCallLogs(logs: CallLog[]) {
  const container = document.getElementById('callLogsList');
  if (!container) return;

  if (logs.length === 0) {
    container.innerHTML = '<p style="text-align: center; color: #999;">No call logs found</p>';
    return;
  }

  container.innerHTML = logs
    .map(
      (log) => `
    <div class="list-item">
      <div class="info">
        <div class="name">${log.contactName || log.phoneNumber}</div>
        <div class="details">${log.callType} • ${formatDuration(log.duration)}</div>
        <div class="time">${formatTimestamp(log.timestamp)}</div>
      </div>
    </div>
  `
    )
    .join('');
}

function displayMessages(messages: SMSMessage[]) {
  const container = document.getElementById('messagesList');
  if (!container) return;

  if (messages.length === 0) {
    container.innerHTML = '<p style="text-align: center; color: #999;">No messages found</p>';
    return;
  }

  container.innerHTML = messages
    .map(
      (msg) => `
    <div class="list-item">
      <div class="info">
        <div class="name">${msg.sender}</div>
        <div class="details">${msg.body}</div>
        <div class="time">${formatTimestamp(msg.timestamp)}</div>
      </div>
    </div>
  `
    )
    .join('');
}

function displayContacts(contacts: Contact[]) {
  const container = document.getElementById('contactsList');
  if (!container) return;

  if (contacts.length === 0) {
    container.innerHTML = '<p style="text-align: center; color: #999;">No contacts found</p>';
    return;
  }

  container.innerHTML = contacts
    .map(
      (contact) => `
    <div class="list-item">
      <div class="info">
        <div class="name">${contact.name}</div>
        <div class="details">
          ${contact.phoneNumbers.map((p) => p.number).join(', ')}
        </div>
      </div>
    </div>
  `
    )
    .join('');
}

function displayPermissions(permissions: Record<Permission, string>) {
  const container = document.getElementById('permissionsList');
  if (!container) return;

  container.innerHTML = Object.entries(permissions)
    .map(
      ([permission, state]) => `
    <div class="permission-item">
      <span>${formatPermissionName(permission)}</span>
      <span class="permission-badge ${state.toLowerCase()}">${state}</span>
    </div>
  `
    )
    .join('');
}

// UI Helpers
function showIncomingCallOverlay(phoneNumber: string, contactName?: string) {
  const overlay = document.getElementById('incomingCallOverlay');
  const numberEl = document.getElementById('incomingNumber');
  const contactEl = document.getElementById('incomingContact');

  if (overlay && numberEl && contactEl) {
    numberEl.textContent = phoneNumber;
    contactEl.textContent = contactName || '';
    overlay.classList.add('active');
  }
}

function hideIncomingCallOverlay() {
  const overlay = document.getElementById('incomingCallOverlay');
  overlay?.classList.remove('active');
}

function showStatus(tabId: string, message: string, type: 'info' | 'success' | 'error') {
  const tab = document.getElementById(tabId);
  if (!tab) return;

  // Remove existing status
  const existingStatus = tab.querySelector('.status');
  existingStatus?.remove();

  // Add new status
  const status = document.createElement('div');
  status.className = `status ${type}`;
  status.textContent = message;
  tab.insertBefore(status, tab.firstChild);

  // Auto-remove after 5 seconds
  setTimeout(() => status.remove(), 5000);
}

// Format Helpers
function formatDuration(seconds: number): string {
  if (seconds < 60) {
    return `${seconds}s`;
  }
  const minutes = Math.floor(seconds / 60);
  const remainingSeconds = seconds % 60;
  return `${minutes}m ${remainingSeconds}s`;
}

function formatTimestamp(timestamp: number): string {
  const date = new Date(timestamp);
  const now = new Date();
  const diff = now.getTime() - date.getTime();

  // Less than 24 hours
  if (diff < 86400000) {
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  }

  // Less than 7 days
  if (diff < 604800000) {
    return date.toLocaleDateString([], { weekday: 'short', hour: '2-digit', minute: '2-digit' });
  }

  // Older
  return date.toLocaleDateString([], {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
}

function formatPermissionName(permission: string): string {
  return permission
    .split('_')
    .map((word) => word.charAt(0) + word.slice(1).toLowerCase())
    .join(' ');
}

// Request notification permission
if ('Notification' in window && Notification.permission === 'default') {
  Notification.requestPermission();
}
