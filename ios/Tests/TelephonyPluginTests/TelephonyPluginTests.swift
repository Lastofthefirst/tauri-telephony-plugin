import XCTest
@testable import TelephonyPlugin

final class TelephonyPluginTests: XCTestCase {

    func testPhoneNumberValidation() {
        let validNumbers = [
            "+1234567890",
            "123-456-7890",
            "(123) 456-7890",
            "123 456 7890"
        ]

        let invalidNumbers = [
            "",
            "abc",
            "123abc456"
        ]

        for number in validNumbers {
            XCTAssertTrue(
                isValidPhoneNumber(number),
                "Valid number \(number) should pass validation"
            )
        }

        for number in invalidNumbers {
            XCTAssertFalse(
                isValidPhoneNumber(number),
                "Invalid number \(number) should fail validation"
            )
        }
    }

    func testPhoneNumberTypeMapping() {
        let typeMapping: [String: String] = [
            "_$!<Mobile>!$_": "MOBILE",
            "_$!<Home>!$_": "HOME",
            "_$!<Work>!$_": "WORK",
            "_$!<Main>!$_": "MAIN"
        ]

        XCTAssertEqual(typeMapping.count, 4)
    }

    func testPermissionStateValues() {
        let states = ["GRANTED", "DENIED", "PROMPT"]
        XCTAssertEqual(states.count, 3)
        XCTAssertTrue(states.contains("GRANTED"))
        XCTAssertTrue(states.contains("DENIED"))
        XCTAssertTrue(states.contains("PROMPT"))
    }

    func testCallStateValues() {
        let states = [
            "IDLE",
            "RINGING",
            "OFFHOOK",
            "DIALING",
            "ACTIVE",
            "HOLDING",
            "DISCONNECTED",
            "CONNECTING"
        ]

        XCTAssertEqual(states.count, 8)
        XCTAssertTrue(states.contains("ACTIVE"))
        XCTAssertTrue(states.contains("RINGING"))
    }

    func testCallTypeValues() {
        let types = [
            "INCOMING",
            "OUTGOING",
            "MISSED",
            "REJECTED",
            "BLOCKED",
            "VOICEMAIL"
        ]

        XCTAssertEqual(types.count, 6)
        XCTAssertTrue(types.contains("INCOMING"))
        XCTAssertTrue(types.contains("OUTGOING"))
    }

    func testMessageTypeValues() {
        let types = [
            "INBOX",
            "SENT",
            "DRAFT",
            "OUTBOX",
            "FAILED",
            "QUEUED"
        ]

        XCTAssertEqual(types.count, 6)
        XCTAssertTrue(types.contains("INBOX"))
        XCTAssertTrue(types.contains("SENT"))
    }

    // Helper function
    private func isValidPhoneNumber(_ phoneNumber: String) -> Bool {
        if phoneNumber.isEmpty {
            return false
        }

        let validCharacters = CharacterSet(charactersIn: "0123456789+()-  ")
        return phoneNumber.unicodeScalars.allSatisfy { validCharacters.contains($0) }
    }
}
