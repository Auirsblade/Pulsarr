// PulsarrApp.swift

import SwiftUI

@main
struct PulsarrApp: App {
    @State private var isSignedIn: Bool = AuthService.shared.isSignedIn

    init() {
        // Configure API environment on app launch
        APIConfiguration.configure()
    }

    var body: some Scene {
        WindowGroup {
            if isSignedIn {
                TabView {
                    NavigationStack {
                        Text("Feed") // Replace with your ProfileView
                            .navigationTitle("Feed")
                    }
                    .tabItem {
                        Label("Feed", systemImage: "square.stack.fill")
                    }
                    NavigationStack {
                        GroupsView()
                    }
                    .tabItem {
                        Label("Groups", systemImage: "person.3.fill")
                    }
                    
                    NavigationStack {
                        Text("Profile") // Replace with your ProfileView
                            .navigationTitle("Profile")
                    }
                    .tabItem {
                        Label("Profile", systemImage: "person.fill")
                    }
                    
                    NavigationStack {
                        SettingsView(isSignedIn: $isSignedIn)
                    }
                    .tabItem {
                        Label("Settings", systemImage: "gearshape.fill")
                    }
                }
            } else {
                NavigationStack {
                    SignInView {
                        isSignedIn = true
                    }
                }
            }
        }
    }
}
