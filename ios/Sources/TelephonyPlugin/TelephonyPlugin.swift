import Foundation
import Tauri
import UIKit
import Contacts
import ContactsUI
import MessageUI
import CallKit

@objc(TelephonyPlugin)
class TelephonyPlugin: Plugin {

    private var callObserver: CXCallObserver?
    private var contactStore = CNContactStore()

    override init() {
        super.init()
        setupCallObserver()
    }

    // MARK: - Call Management

    @objc func makeCall(_ invoke: Invoke) {
        guard let args = invoke.parseArgs(MakeCallArgs.self) else {
            invoke.reject("Invalid arguments")
            return
        }

        let phoneNumber = args.phoneNumber.replacingOccurrences(of: " ", with: "")

        guard let url = URL(string: "tel://\(phoneNumber)") else {
            invoke.reject("Invalid phone number")
            return
        }

        if UIApplication.shared.canOpenURL(url) {
            UIApplication.shared.open(url, options: [:]) { success in
                if success {
                    invoke.resolve()
                } else {
                    invoke.reject("Failed to initiate call")
                }
            }
        } else {
            invoke.reject("Device cannot make calls")
        }
    }

    @objc func answerCall(_ invoke: Invoke) {
        // Note: iOS doesn't provide an API to programmatically answer calls
        // This would require deep system integration not available to third-party apps
        invoke.reject("Answering calls programmatically is not supported on iOS")
    }

    @objc func rejectCall(_ invoke: Invoke) {
        // Note: iOS doesn't provide an API to programmatically reject calls
        invoke.reject("Rejecting calls programmatically is not supported on iOS")
    }

    @objc func endCall(_ invoke: Invoke) {
        // Note: iOS doesn't provide an API to programmatically end calls
        invoke.reject("Ending calls programmatically is not supported on iOS")
    }

    @objc func getCallState(_ invoke: Invoke) {
        guard let callObserver = callObserver else {
            invoke.reject("Call observer not initialized")
            return
        }

        let hasCalls = !callObserver.calls.isEmpty
        let state: String

        if hasCalls {
            // Check if any call is active
            let hasActiveCalls = callObserver.calls.contains { !$0.hasEnded }
            state = hasActiveCalls ? "ACTIVE" : "IDLE"
        } else {
            state = "IDLE"
        }

        var result = JSObject()
        result["state"] = state
        invoke.resolve(result)
    }

    // MARK: - SMS Management

    @objc func sendSMS(_ invoke: Invoke) {
        guard let args = invoke.parseArgs(SMSArgs.self) else {
            invoke.reject("Invalid arguments")
            return
        }

        // Note: iOS requires user interaction to send SMS
        // We'll compose the message but the user must confirm

        if MFMessageComposeViewController.canSendText() {
            DispatchQueue.main.async {
                let messageVC = MFMessageComposeViewController()
                messageVC.body = args.message
                messageVC.recipients = [args.recipient]
                messageVC.messageComposeDelegate = self

                if let rootVC = UIApplication.shared.windows.first?.rootViewController {
                    rootVC.present(messageVC, animated: true)

                    // Generate message ID
                    let messageId = "sms_\(UUID().uuidString)"
                    var result = JSObject()
                    result["messageId"] = messageId
                    invoke.resolve(result)
                } else {
                    invoke.reject("Could not present message composer")
                }
            }
        } else {
            invoke.reject("Device cannot send SMS")
        }
    }

    @objc func getSMSMessages(_ invoke: Invoke) {
        // Note: iOS does not provide API access to read SMS messages
        // This is a privacy restriction by Apple
        invoke.reject("Reading SMS messages is not supported on iOS due to privacy restrictions")
    }

    @objc func deleteSMS(_ invoke: Invoke) {
        // Note: iOS does not allow apps to delete SMS messages
        invoke.reject("Deleting SMS messages is not supported on iOS")
    }

    // MARK: - Call Logs

    @objc func getCallLogs(_ invoke: Invoke) {
        // Note: iOS does not provide direct API access to call history
        // Apps can only access their own CallKit call history
        invoke.reject("Direct access to call logs is not supported on iOS")
    }

    // MARK: - Contacts

    @objc func getContacts(_ invoke: Invoke) {
        let status = CNContactStore.authorizationStatus(for: .contacts)

        guard status == .authorized else {
            invoke.reject("Contacts permission not granted")
            return
        }

        do {
            let keysToFetch = [
                CNContactGivenNameKey,
                CNContactFamilyNameKey,
                CNContactPhoneNumbersKey,
                CNContactEmailAddressesKey
            ] as [CNKeyDescriptor]

            let request = CNContactFetchRequest(keysToFetch: keysToFetch)
            var contactsArray: [[String: Any]] = []

            try contactStore.enumerateContacts(with: request) { contact, _ in
                var contactDict: [String: Any] = [:]
                contactDict["id"] = contact.identifier
                contactDict["name"] = "\(contact.givenName) \(contact.familyName)".trimmingCharacters(in: .whitespaces)

                // Phone numbers
                var phoneNumbers: [[String: String]] = []
                for phoneNumber in contact.phoneNumbers {
                    phoneNumbers.append([
                        "number": phoneNumber.value.stringValue,
                        "numberType": self.mapPhoneNumberType(phoneNumber.label)
                    ])
                }
                contactDict["phoneNumbers"] = phoneNumbers

                // Email addresses
                var emails: [String] = []
                for email in contact.emailAddresses {
                    emails.append(email.value as String)
                }
                contactDict["emailAddresses"] = emails

                contactsArray.append(contactDict)
            }

            var result = JSObject()
            result["contacts"] = contactsArray
            invoke.resolve(result)
        } catch {
            invoke.reject("Failed to fetch contacts: \(error.localizedDescription)")
        }
    }

    // MARK: - Permissions

    @objc func requestPermissions(_ invoke: Invoke) {
        guard let args = invoke.parseArgs(PermissionsArgs.self) else {
            invoke.reject("Invalid arguments")
            return
        }

        var permissionStatus: [String: String] = [:]
        let dispatchGroup = DispatchGroup()

        for permission in args.permissions {
            if permission == "READ_CONTACTS" {
                dispatchGroup.enter()
                contactStore.requestAccess(for: .contacts) { granted, error in
                    permissionStatus[permission] = granted ? "GRANTED" : "DENIED"
                    dispatchGroup.leave()
                }
            } else {
                // For other permissions, check current status
                permissionStatus[permission] = self.checkPermissionStatus(permission)
            }
        }

        dispatchGroup.notify(queue: .main) {
            var result = JSObject()
            result["permissions"] = permissionStatus
            invoke.resolve(result)
        }
    }

    @objc func checkPermissions(_ invoke: Invoke) {
        guard let args = invoke.parseArgs(PermissionsArgs.self) else {
            invoke.reject("Invalid arguments")
            return
        }

        var permissionStatus: [String: String] = [:]

        for permission in args.permissions {
            permissionStatus[permission] = checkPermissionStatus(permission)
        }

        var result = JSObject()
        result["permissions"] = permissionStatus
        invoke.resolve(result)
    }

    // MARK: - Helper Methods

    private func setupCallObserver() {
        callObserver = CXCallObserver()
        callObserver?.setDelegate(self, queue: nil)
    }

    private func checkPermissionStatus(_ permission: String) -> String {
        switch permission {
        case "READ_CONTACTS":
            let status = CNContactStore.authorizationStatus(for: .contacts)
            switch status {
            case .authorized:
                return "GRANTED"
            case .denied, .restricted:
                return "DENIED"
            case .notDetermined:
                return "PROMPT"
            @unknown default:
                return "PROMPT"
            }
        default:
            // Most telephony permissions don't have a direct check on iOS
            return "GRANTED"
        }
    }

    private func mapPhoneNumberType(_ label: String?) -> String {
        guard let label = label else { return "OTHER" }

        switch label {
        case CNLabelPhoneNumberMobile:
            return "MOBILE"
        case CNLabelHome:
            return "HOME"
        case CNLabelWork:
            return "WORK"
        case CNLabelPhoneNumberMain:
            return "MAIN"
        case CNLabelPhoneNumberHomeFax:
            return "FAX_HOME"
        case CNLabelPhoneNumberWorkFax:
            return "FAX_WORK"
        case CNLabelPhoneNumberPager:
            return "PAGER"
        default:
            return "OTHER"
        }
    }
}

// MARK: - CXCallObserverDelegate

extension TelephonyPlugin: CXCallObserverDelegate {
    func callObserver(_ callObserver: CXCallObserver, callChanged call: CXCall) {
        var eventData: [String: Any] = [:]
        eventData["callId"] = call.uuid.uuidString

        if call.hasConnected {
            eventData["state"] = "ACTIVE"
            trigger("telephony://call-state-changed", data: eventData)
        } else if call.hasEnded {
            eventData["state"] = "DISCONNECTED"
            trigger("telephony://call-state-changed", data: eventData)
        } else if call.isOutgoing {
            eventData["state"] = "DIALING"
            trigger("telephony://call-state-changed", data: eventData)
        }
    }
}

// MARK: - MFMessageComposeViewControllerDelegate

extension TelephonyPlugin: MFMessageComposeViewControllerDelegate {
    func messageComposeViewController(_ controller: MFMessageComposeViewController,
                                     didFinishWith result: MessageComposeResult) {
        controller.dismiss(animated: true)

        var eventData: [String: Any] = [:]

        switch result {
        case .sent:
            eventData["success"] = true
            trigger("telephony://sms-sent", data: eventData)
        case .failed:
            eventData["success"] = false
            eventData["error"] = "Failed to send message"
            trigger("telephony://sms-sent", data: eventData)
        case .cancelled:
            eventData["success"] = false
            eventData["error"] = "Message cancelled by user"
            trigger("telephony://sms-sent", data: eventData)
        @unknown default:
            break
        }
    }
}

// MARK: - Argument Models

struct MakeCallArgs: Decodable {
    let phoneNumber: String
}

struct CallIdArgs: Decodable {
    let callId: String
}

struct SMSArgs: Decodable {
    let recipient: String
    let message: String
}

struct MessageQueryArgs: Decodable {
    let limit: Int?
    let offset: Int?
    let threadId: String?
    let messageType: String?
    let startDate: Int64?
    let endDate: Int64?
    let unreadOnly: Bool?
}

struct CallLogQueryArgs: Decodable {
    let limit: Int?
    let offset: Int?
    let callType: String?
    let startDate: Int64?
    let endDate: Int64?
}

struct MessageIdArgs: Decodable {
    let messageId: String
}

struct PermissionsArgs: Decodable {
    let permissions: [String]
}
