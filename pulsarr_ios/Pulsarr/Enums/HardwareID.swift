// HardwareID.swift

import Foundation

enum HardwareID {
    private static let keychainKey = "pulsarr.hardware.id"

    static func current() -> String {
        // Try to read an existing device ID from Keychain
        if let existing = KeychainHelper.retrieve(key: keychainKey), !existing.isEmpty {
            return existing
        }
        // Generate a new UUID, save it, and return it
        let newID = UUID().uuidString
        KeychainHelper.save(key: keychainKey, value: newID)
        return newID
    }
}
