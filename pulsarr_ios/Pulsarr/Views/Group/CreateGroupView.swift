// CreateGroupView.swift

import SwiftUI
import OpenAPIClient

struct CreateGroupView: View {
    var onGroupCreated: () -> Void
    @Environment(\.dismiss) private var dismiss
    
    @State private var groupName: String = ""
    @State private var privacyType: String = "public"
    @State private var isCreating: Bool = false
    @State private var errorMessage: String?
    
    // Rating System states
    @State private var ratingSystems: [RatingSystem] = []
    @State private var selectedRatingSystemId: Int = 1
    @State private var isLoadingRatingSystems: Bool = false
    
    var body: some View {
        Form {
            Section {
                VStack(alignment: .leading, spacing: 24) {
                    // Group Name Field
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Group Name")
                            .font(.subheadline)
                            .foregroundStyle(.secondary)
                        TextField("Enter group name", text: $groupName)
                            .textFieldStyle(.plain)
                            .padding()
                            .background(Color(.systemGray6))
                            .cornerRadius(10)
                    }
                    
                    // Privacy Type Field
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Privacy Type")
                            .font(.subheadline)
                            .foregroundStyle(.secondary)
                        
                        Picker("Privacy Type", selection: $privacyType) {
                            Text("Public").tag("public")
                            Text("Private").tag("private")
                        }
                        .pickerStyle(.segmented)
                        .frame(height: 44)
                    }
                    
                    // Rating System Picker
                    if isLoadingRatingSystems {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("Rating System")
                                .font(.subheadline)
                                .foregroundStyle(.secondary)
                            HStack {
                                Text("Loading...")
                                    .foregroundStyle(.secondary)
                                Spacer()
                                ProgressView()
                            }
                            .padding()
                            .background(Color(.systemGray6))
                            .cornerRadius(10)
                        }
                    } else if ratingSystems.isEmpty {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("Rating System")
                                .font(.subheadline)
                                .foregroundStyle(.secondary)
                            Text("No rating systems available")
                                .foregroundStyle(.secondary)
                                .padding()
                                .frame(maxWidth: .infinity, alignment: .leading)
                                .background(Color(.systemGray6))
                                .cornerRadius(10)
                        }
                    } else {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("Rating System")
                                .font(.subheadline)
                                .foregroundStyle(.secondary)
                            
                            Menu {
                                ForEach(ratingSystems, id: \.ratingSystemId) { ratingSystem in
                                    Button {
                                        selectedRatingSystemId = ratingSystem.ratingSystemId
                                    } label: {
                                        HStack {
                                            Text(ratingSystem.name)
                                            if selectedRatingSystemId == ratingSystem.ratingSystemId {
                                                Spacer()
                                                Image(systemName: "checkmark")
                                            }
                                        }
                                    }
                                }
                            } label: {
                                HStack {
                                    if let selectedSystem = ratingSystems.first(where: { $0.ratingSystemId == selectedRatingSystemId }) {
                                        Text(selectedSystem.name)
                                    } else {
                                        Text("Select Rating System")
                                            .foregroundStyle(.secondary)
                                    }
                                    Spacer()
                                    Image(systemName: "chevron.up.chevron.down")
                                        .font(.caption)
                                        .foregroundStyle(.secondary)
                                }
                                .padding()
                                .background(Color(.systemGray6))
                                .cornerRadius(10)
                            }
                            .buttonStyle(.plain)
                        }
                    }
                }
                .padding(.vertical, 8)
            } header: {
                Text("Group Details")
                    .font(.headline)
                    .foregroundStyle(.primary)
                    .textCase(nil)
            }
            
            if let error = errorMessage {
                Section {
                    Text(error)
                        .foregroundColor(.red)
                        .font(.footnote)
                }
            }
            
            Section {
                Button {
                    Task {
                        await createGroup()
                    }
                } label: {
                    if isCreating {
                        HStack {
                            ProgressView()
                            Text("Creating...")
                        }
                    } else {
                        Text("Create Group")
                            .fontWeight(.semibold)
                    }
                }
                .frame(maxWidth: .infinity)
                .disabled(groupName.isEmpty || isCreating)
            }
        }
        .navigationTitle("Create Group")
        .navigationBarTitleDisplayMode(.inline)
        .toolbar {
            ToolbarItem(placement: .navigationBarLeading) {
                Button("Cancel") {
                    dismiss()
                }
            }
        }
        .task {
            await loadRatingSystems()
        }
    }
    
    private func createGroup() async {
        errorMessage = nil
        guard !groupName.trimmingCharacters(in: .whitespaces).isEmpty else {
            errorMessage = "Group name cannot be empty."
            return
        }
        
        isCreating = true
        defer { isCreating = false }
        
        do {
            // Get the API key
            guard let apiKey = AuthService.shared.sessionKey, !apiKey.isEmpty else {
                errorMessage = "Not signed in."
                return
            }
            
            // Create a GroupDTO with placeholder values
            // Note: You may need to adjust these values based on your requirements
            let newGroup = GroupDTO(
                pulsarrGroupId: 0, // Will be assigned by the server
                ratingSystemId: selectedRatingSystemId, // Use selected rating system
                name: groupName,
                privacyType: privacyType
            )
            
            // Call the API
            let _ = try await withCheckedThrowingContinuation { (continuation: CheckedContinuation<GroupDTO, Error>) in
                GroupAPI.createGroup(
                    pulsarrApiKey: apiKey,
                    groupDTO: newGroup
                ) { data, error in
                    if let err = error {
                        continuation.resume(throwing: err)
                    } else if let group = data {
                        continuation.resume(returning: group)
                    } else {
                        continuation.resume(throwing: NSError(
                            domain: "CreateGroup",
                            code: -1,
                            userInfo: [NSLocalizedDescriptionKey: "No data returned"]
                        ))
                    }
                }
            }
            
            // Success!
            onGroupCreated()
        } catch {
            errorMessage = error.localizedDescription
        }
    }
    
    private func loadRatingSystems() async {
        isLoadingRatingSystems = true
        defer { isLoadingRatingSystems = false }
        
        do {
            var systems = try await withCheckedThrowingContinuation { (continuation: CheckedContinuation<[RatingSystem], Error>) in
                RatingSystemAPI.getAllRatingSystems { data, error in
                    if let error = error {
                        continuation.resume(throwing: error)
                    } else if let data = data {
                        continuation.resume(returning: data)
                    } else {
                        continuation.resume(throwing: NSError(
                            domain: "RatingSystem",
                            code: -1,
                            userInfo: [NSLocalizedDescriptionKey: "No data returned"]
                        ))
                    }
                }
            }
            
            systems.sort { $0.ratingSystemId < $1.ratingSystemId }
            ratingSystems = systems
            
            // Set default selection to first rating system if available
            if let firstSystem = systems.first {
                selectedRatingSystemId = firstSystem.ratingSystemId
            }
        } catch {
            errorMessage = "Failed to load rating systems: \(error.localizedDescription)"
        }
    }
}

#Preview {
    NavigationStack {
        CreateGroupView(onGroupCreated: {})
    }
}
