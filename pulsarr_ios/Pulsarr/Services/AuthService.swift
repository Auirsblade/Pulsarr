// AuthService.swift

import Foundation
import OpenAPIClient   // Contains SignInRequest, SignInResponse, AuthAPI, UserDTO, etc.

final class AuthService {
    static let shared = AuthService()

    private let sessionKeyKeychainKey = "sessionKey"
    private let currentUserDefaultsKey = "currentUser"

    /// The user’s session key (cached in Keychain).
    private(set) var sessionKey: String? = nil

    /// The currently signed-in user’s DTO (cached in UserDefaults).
    private var currentUser: UserDTO? = nil

    private init() {
        // On initialization, try to load a persisted session key:
        if let savedKey = KeychainHelper.retrieve(key: sessionKeyKeychainKey),
           !savedKey.isEmpty {
            sessionKey = savedKey
        }

        // Try to load a persisted UserDTO from UserDefaults:
        if let savedUser: UserDTO = UserDefaults.standard.codableValue(
            forKey: currentUserDefaultsKey,
            as: UserDTO.self
        ) {
            currentUser = savedUser
        }
    }

    // MARK: — Authentication

    /// Adds a new user to the system.
    /// - Parameter userDTO: The user information to create
    /// - Returns: The created UserDTO with server-assigned ID
    /// - Throws: If the API request fails
    func addUser(_ userDTO: UserDTO) async throws -> UserDTO {
        return try await withCheckedThrowingContinuation { continuation in
            UserAPI.addUser(
                userDTO: userDTO,
                apiResponseQueue: OpenAPIClientAPI.apiResponseQueue
            ) { data, error in
                if let err = error {
                    continuation.resume(throwing: err)
                    return
                }
                
                guard let createdUser = data else {
                    continuation.resume(throwing: NSError(
                        domain: "AuthService",
                        code: -1,
                        userInfo: [NSLocalizedDescriptionKey: "No user data returned"]
                    ))
                    return
                }
                
                continuation.resume(returning: createdUser)
            }
        }
    }
    
    /// Creates a new user and immediately signs them in.
    /// - Parameter userDTO: The user information to create (must include password)
    /// - Returns: The sign-in response containing the API key and user data
    /// - Throws: If user creation or sign-in fails
    func addUserAndSignIn(_ userDTO: UserDTO) async throws -> SignInResponse {
        // First, create the user
        let createdUser = try await addUser(userDTO)
        
        // Build the sign-in request using the created user's credentials
        let deviceID = HardwareID.current()
        let signInRequest = SignInRequest(
            username: createdUser.name,
            password: userDTO.password, // Use original password since it's not returned from server
            hwKey: deviceID
        )
        
        // Sign in with the new user's credentials
        return try await signIn(signInRequest)
    }
    
    /// Signs in with the given credentials.
    /// On success, stores the returned API key (Keychain) and user DTO (UserDefaults).
    func signIn(_ request: SignInRequest) async throws -> SignInResponse {
        return try await withCheckedThrowingContinuation { continuation in
            AuthAPI.signin(
                signInRequest: request,
                apiResponseQueue: OpenAPIClientAPI.apiResponseQueue
            ) { data, error in
                if let err = error {
                    continuation.resume(throwing: err)
                    return
                }

                guard let response = data else {
                    continuation.resume(throwing: NSError(
                        domain: "AuthService",
                        code: -1,
                        userInfo: [NSLocalizedDescriptionKey: "No data returned"]
                    ))
                    return
                }

                // MARK: 1) Save the session key
                if !response.pulsarrApiKey.isEmpty {
                    let key = response.pulsarrApiKey
                    
                    KeychainHelper.save(key: self.sessionKeyKeychainKey, value: key)
                    self.sessionKey = key
                }

                // MARK: 2) Save the UserDTO
                let user = response.user
                UserDefaults.standard.setCodable(user, forKey: self.currentUserDefaultsKey)
                self.currentUser = user

                continuation.resume(returning: response)
            }
        }
    }

    /// Signs out: clears Keychain + UserDefaults, forgets current session and user.
    func signOut() {
        // Delete the session key
        KeychainHelper.delete(key: sessionKeyKeychainKey)
        sessionKey = nil

        // Delete the saved user
        UserDefaults.standard.removeObject(forKey: currentUserDefaultsKey)
        currentUser = nil
    }

    // MARK: — Current User Access

    /// Returns the currently signed-in user, if any.
    /// If none exists, returns a “default” UserDTO (does NOT persist that default).
    func getCurrentUser() -> UserDTO {
        if let existing = currentUser {
            return existing
        }
        // Create a default guest user (adjust fields to match your UserDTO initializer)
        let guest = UserDTO(
            pulsarrUserId: 0,
            name: "Guest",
            email: "",
            password: ""
        )
        currentUser = guest
        return guest
    }

    /// Returns true if there is a non-empty session key in memory (or in Keychain).
    var isSignedIn: Bool {
        if let key = sessionKey, !key.isEmpty {
            return true
        }
        return false
    }
}
