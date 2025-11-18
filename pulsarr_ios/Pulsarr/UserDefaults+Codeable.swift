// UserDefaults+Codable.swift

import Foundation

/// Convenience for storing any `Codable` in UserDefaults under a key.
extension UserDefaults {
    func setCodable<T: Codable>(_ codable: T, forKey key: String) {
        let encoder = JSONEncoder()
        if let data = try? encoder.encode(codable) {
            set(data, forKey: key)
        }
    }

    func codableValue<T: Codable>(forKey key: String, as type: T.Type) -> T? {
        guard let data = data(forKey: key) else { return nil }
        let decoder = JSONDecoder()
        return try? decoder.decode(type, from: data)
    }
}
