// SignInView.swift

import SwiftUI
import OpenAPIClient

struct SignInView: View {
    var onSignInSuccess: () -> Void  // callback to flip isSignedIn

    @State private var username: String = ""
    @State private var password: String = ""
    @State private var errorMessage: String? = nil
    @State private var isSigningIn: Bool = false
    @State private var isShowingSignUp: Bool = false

    var body: some View {
        VStack(spacing: 24) {
            Text("Welcome to Pulsarr")
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
                Task { await handleSignIn() }
            } label: {
                if isSigningIn {
                    ProgressView()
                        .progressViewStyle(.circular)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color.blue)
                        .foregroundColor(.white)
                        .cornerRadius(8)
                } else {
                    Text("Sign In")
                        .font(.headline)
                        .foregroundColor(.white)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color.blue)
                        .cornerRadius(8)
                }
                
            }
            .disabled(isSigningIn || username.isEmpty || password.isEmpty)

            Button {
                isShowingSignUp = true
            } label: {
                Text("Don't have an account? Sign Up")
                    .font(.subheadline)
                    .foregroundColor(.blue)
            }
            .padding(.top, 8)
            
            Spacer()
        }
        .padding(.horizontal, 32)
        .navigationDestination(isPresented: $isShowingSignUp) {
            SignUpView(onSignInSuccess: onSignInSuccess)
        }
    }

    private func handleSignIn() async {
        errorMessage = nil
        guard !username.trimmingCharacters(in: .whitespaces).isEmpty,
              !password.isEmpty else {
            errorMessage = "Username and password cannot be empty."
            return
        }

        isSigningIn = true
        defer { isSigningIn = false }

        do {
            // Build hwKey from persisted device identifier + username
            let deviceID = HardwareID.current();

            let req = SignInRequest(username: username, password: password, hwKey: deviceID)
            let response = try await AuthService.shared.signIn(req)

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
