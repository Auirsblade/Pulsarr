// SettingsView.swift

import SwiftUI

struct SettingsView: View {
    @Binding var isSignedIn: Bool
    @State private var showingSignOutAlert = false
    
    var body: some View {
        List {
            Section {
                Button(role: .destructive) {
                    showingSignOutAlert = true
                } label: {
                    HStack {
                        Image(systemName: "rectangle.portrait.and.arrow.right")
                            .foregroundColor(.red)
                        Text("Sign Out")
                    }
                }
            }
        }
        .navigationTitle("Settings")
        .alert("Sign Out", isPresented: $showingSignOutAlert) {
            Button("Cancel", role: .cancel) { }
            Button("Sign Out", role: .destructive) {
                signOut()
            }
        } message: {
            Text("Are you sure you want to sign out?")
        }
    }
    
    private func signOut() {
        AuthService.shared.signOut()
        isSignedIn = false
    }
}

#Preview {
    NavigationStack {
        SettingsView(isSignedIn: .constant(true))
    }
}
