// CreateGroupViewWithPicker.swift
//
// Example of CreateGroupView using the reusable MyGroupsPickerView

import SwiftUI
import OpenAPIClient

struct CreateGroupViewWithPicker: View {
    var onGroupCreated: () -> Void
    @Environment(\.dismiss) private var dismiss
    
    @State private var groupName: String = ""
    @State private var privacyType: String = "public"
    @State private var isCreating: Bool = false
    @State private var errorMessage: String?
    
    // Selected group from the picker
    @State private var selectedExistingGroup: GroupDTO?
    
    var body: some View {
        Form {
            // Browse existing groups
            Section {
                MyGroupsPickerView(selectedGroup: $selectedExistingGroup)
            } header: {
                Text("Browse My Groups")
            } footer: {
                Text("View and select from your existing groups, or create a new one below.")
            }
            
            // Create new group
            Section("Create New Group") {
                TextField("Group Name", text: $groupName)
                
                Picker("Privacy Type", selection: $privacyType) {
                    Text("Public").tag("public")
                    Text("Private").tag("private")
                }
            }
            
            // Error message
            if let error = errorMessage {
                Section {
                    Label(error, systemImage: "exclamationmark.triangle.fill")
                        .foregroundColor(.red)
                        .font(.footnote)
                }
            }
            
            // Create button
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
        .navigationTitle("Groups")
        .navigationBarTitleDisplayMode(.inline)
        .toolbar {
            ToolbarItem(placement: .navigationBarLeading) {
                Button("Cancel") {
                    dismiss()
                }
            }
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
            guard let apiKey = AuthService.shared.sessionKey, !apiKey.isEmpty else {
                errorMessage = "Not signed in."
                return
            }
            
            let newGroup = GroupDTO(
                pulsarrGroupId: 0,
                ratingSystemId: 1,
                name: groupName,
                privacyType: privacyType
            )
            
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
            
            onGroupCreated()
        } catch {
            errorMessage = error.localizedDescription
        }
    }
}

#Preview {
    NavigationStack {
        CreateGroupViewWithPicker(onGroupCreated: {})
    }
}
