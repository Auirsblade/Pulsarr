// GroupService.swift

import Foundation
import OpenAPIClient   // Contains GroupAPI, PulsarrGroup, GetRequest

final class UserService {
    static let shared = UserService()

    private init() { }

    /// Fetches the current user.
    /// - Throws if not signed in or if the network call fails.
    /// - Returns: a `UserDTO` on success.
    func getCurrentUser() async throws -> UserDTO {
        // Ensure we have a valid API key
        guard let apiKey = AuthService.shared.sessionKey, !apiKey.isEmpty else {
            throw NSError(
                domain: "UserService",
                code: 401,
                userInfo: [NSLocalizedDescriptionKey: "Not signed in"]
            )
        }

        return try await withCheckedThrowingContinuation { continuation in
            UserAPI.getCurrentUser(
                pulsarrApiKey: apiKey
            ) { data, error in
                if let err = error {
                    continuation.resume(throwing: err)
                } else if let user = data {
                    continuation.resume(returning: user)
                } else {
                    continuation.resume(throwing: NSError(
                        domain: "UserService",
                        code: -1,
                        userInfo: [NSLocalizedDescriptionKey: "No data returned"]
                    ))
                }
            }
        }
    }
}
