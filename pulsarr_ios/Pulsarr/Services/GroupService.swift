// GroupService.swift

import Foundation
import OpenAPIClient   // Contains GroupAPI, PulsarrGroup, GetRequest

final class GroupService {
    static let shared = GroupService()

    private init() { }

    /// Fetches all groups via POST /group/.
    /// - Parameter getRequest: a `GetRequest` object (e.g. paging/filter parameters).
    /// - Throws if not signed in or if the network call fails.
    /// - Returns: an array of `PulsarrGroup` on success.
    func getAllGroups(getRequest: GetRequest) async throws -> [GroupDTO] {
        // Ensure we have a valid API key
        guard let apiKey = AuthService.shared.sessionKey, !apiKey.isEmpty else {
            throw NSError(
                domain: "GroupService",
                code: 401,
                userInfo: [NSLocalizedDescriptionKey: "Not signed in"]
            )
        }

        return try await withCheckedThrowingContinuation { continuation in
            GroupAPI.getAllGroups(
                pulsarrApiKey: apiKey,
                getRequest: getRequest
            ) { data, error in
                if let err = error {
                    continuation.resume(throwing: err)
                } else if let groups = data {
                    continuation.resume(returning: groups)
                } else {
                    continuation.resume(throwing: NSError(
                        domain: "GroupService",
                        code: -1,
                        userInfo: [NSLocalizedDescriptionKey: "No data returned"]
                    ))
                }
            }
        }
    }
}
