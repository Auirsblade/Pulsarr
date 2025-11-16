// SignUpView.swift

import SwiftUI
import OpenAPIClient

struct SignUpView: View {
    var onSignInSuccess: () -> Void  // callback to flip isSignedIn

    @State private var username: String = ""
    @State private var email: String = ""
    @State private var password: String = ""
    @State private var errorMessage: String? = nil
    @State private var isSigningUp: Bool = false

    var body: some View {
        VStack(spacing: 24) {
            Text("Sign Up")
                .font(.largeTitle).bold()
                .padding(.top, 40)

            // Username
            VStack(alignment: .leading, spacing: 8) {
                Text("Username")
                    .font(.subheadline)
                    .foregroundColor(.gray)
                TextField("Username", text: $username)
                    .autocapitalization(.none)
                    .disableAutocorrection(true)
                    .padding(12)
                    .overlay(
                        RoundedRectangle(cornerRadius: 8)
                            .stroke(Color.gray.opacity(0.5))
                    )
            }
            
            // Email
            VStack(alignment: .leading, spacing: 8) {
                Text("Email")
                    .font(.subheadline)
                    .foregroundColor(.gray)
                TextField("Enter your email", text: $email)
                    .keyboardType(.emailAddress) // Configures the keyboard for email input
                    .autocapitalization(.none) // Prevents automatic capitalization
                    .disableAutocorrection(true) // Disables autocorrection
                    .padding(12)
                    .overlay(
                        RoundedRectangle(cornerRadius: 8)
                            .stroke(Color.gray.opacity(0.5))
                    )
            }

            // Password
            VStack(alignment: .leading, spacing: 8) {
                Text("Password")
                    .font(.subheadline)
                    .foregroundColor(.gray)
                SecureField("Password", text: $password)
                    .padding(12)
                    .overlay(
                        RoundedRectangle(cornerRadius: 8)
                            .stroke(Color.gray.opacity(0.5))
                    )
            }

            // Error
            if let error = errorMessage {
                Text(error)
                    .foregroundColor(.red)
                    .font(.footnote)
                    .multilineTextAlignment(.center)
                    .padding(.horizontal)
            }

            // Sign In Button
            Button {
                Task { await handleSignUp() }
            } label: {
                Text("Sign In")
                    .font(.headline)
                    .foregroundColor(.white)
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(Color.blue)
                    .cornerRadius(8)
                
            }
            .disabled(username.isEmpty || password.isEmpty || email.isEmpty)

            Spacer()
        }
        .padding(.horizontal, 32)
    }

    private func handleSignUp() async {
        errorMessage = nil
        guard !username.trimmingCharacters(in: .whitespaces).isEmpty,
              !password.isEmpty, !email.isEmpty else {
            errorMessage = "Username, Email, and Password must be filled."
            return
        }

        do {
            let req = UserDTO(pulsarrUserId: 0, name: username, email: email, password: password);
            let response = try await AuthService.shared.addUserAndSignIn(req)

            // Check that a valid key was returned
            if !response.pulsarrApiKey.isEmpty {
                onSignInSuccess()
            } else {
                errorMessage = "Invalid credentials."
            }
        } catch {
            errorMessage = error.localizedDescription
        }
    }
}
