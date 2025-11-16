// MyGroupsPickerView.swift
//
// A reusable picker component for selecting from user's groups

import SwiftUI
import OpenAPIClient

struct MyGroupsPickerView: View {
    @Binding var selectedGroup: GroupDTO?
    @State private var groups: [GroupDTO] = []
    @State private var isLoading = false
    @State private var errorMessage: String?
    
    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            // Picker Menu
            Menu {
                if isLoading {
                    HStack {
                        ProgressView()
                        Text("Loading...")
                    }
                } else if groups.isEmpty {
                    Text("No groups available")
                        .foregroundStyle(.secondary)
                    
                    Divider()
                    
                    Button {
                        Task { await loadGroups() }
                    } label: {
                        Label("Refresh", systemImage: "arrow.clockwise")
                    }
                } else {
                    // Clear selection option
                    Button {
                        selectedGroup = nil
                    } label: {
                        HStack {
                            Text("None")
                            if selectedGroup == nil {
                                Image(systemName: "checkmark")
                                    .foregroundStyle(.blue)
                            }
                        }
                    }
                    
                    Divider()
                    
                    // Group options
                    ForEach(groups, id: \.pulsarrGroupId) { group in
                        Button {
                            withAnimation(.easeInOut(duration: 0.2)) {
                                selectedGroup = group
                            }
                        } label: {
                            HStack {
                                VStack(alignment: .leading, spacing: 2) {
                                    Text(group.name)
                                    HStack(spacing: 4) {
                                        Image(systemName: privacyIcon(for: group))
                                            .font(.caption2)
                                        Text(group.privacyType.capitalized)
                                            .font(.caption2)
                                    }
                                    .foregroundStyle(.secondary)
                                }
                                Spacer()
                                if selectedGroup?.pulsarrGroupId == group.pulsarrGroupId {
                                    Image(systemName: "checkmark")
                                        .foregroundStyle(.blue)
                                }
                            }
                        }
                    }
                    
                    Divider()
                    
                    Button {
                        Task { await loadGroups() }
                    } label: {
                        Label("Refresh", systemImage: "arrow.clockwise")
                    }
                }
            } label: {
                HStack {
                    VStack(alignment: .leading, spacing: 4) {
                        Text("My Groups")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                        
                        Text(selectedGroup?.name ?? "Select a group")
                            .foregroundStyle(selectedGroup == nil ? .secondary : .primary)
                    }
                    
                    Spacer()
                    
                    if isLoading {
                        ProgressView()
                            .controlSize(.small)
                    } else {
                        Image(systemName: "chevron.up.chevron.down")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                    }
                }
                .padding(.vertical, 8)
            }
            .disabled(isLoading)
            
            // Selected Group Detail
            if let selected = selectedGroup {
                SelectedGroupCard(group: selected)
                    .transition(.scale.combined(with: .opacity))
            }
            
            // Error Message
            if let error = errorMessage {
                Label(error, systemImage: "exclamationmark.triangle.fill")
                    .font(.caption)
                    .foregroundStyle(.red)
            }
        }
        .task {
            await loadGroups()
        }
    }
    
    // MARK: - Private Methods
    
    private func loadGroups() async {
        guard let apiKey = AuthService.shared.sessionKey, !apiKey.isEmpty else {
            errorMessage = "Not authenticated"
            return
        }
        
        isLoading = true
        errorMessage = nil
        defer { isLoading = false }
        
        do {
            let fetchedGroups = try await withCheckedThrowingContinuation { (continuation: CheckedContinuation<[GroupDTO], Error>) in
                GroupAPI.myGroups(pulsarrApiKey: apiKey) { data, error in
                    if let err = error {
                        continuation.resume(throwing: err)
                    } else if let groups = data {
                        continuation.resume(returning: groups)
                    } else {
                        continuation.resume(returning: [])
                    }
                }
            }
            
            groups = fetchedGroups
        } catch {
            errorMessage = "Failed to load groups"
            print("❌ Error loading groups: \(error.localizedDescription)")
        }
    }
    
    private func privacyIcon(for group: GroupDTO) -> String {
        group.privacyType.lowercased() == "private" ? "lock.fill" : "globe"
    }
}

// MARK: - Supporting Views

private struct SelectedGroupCard: View {
    let group: GroupDTO
    
    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: iconName)
                .font(.title2)
                .foregroundStyle(.blue)
                .frame(width: 40, height: 40)
                .background(Color.blue.opacity(0.1))
                .clipShape(Circle())
            
            VStack(alignment: .leading, spacing: 4) {
                Text(group.name)
                    .font(.subheadline)
                    .fontWeight(.medium)
                
                HStack(spacing: 8) {
                    Label(group.privacyType.capitalized, systemImage: privacyIcon)
                        .font(.caption)
                        .foregroundStyle(.secondary)
                    
                    Text("•")
                        .foregroundStyle(.secondary)
                        .font(.caption)
                    
                    Text("ID: \(group.pulsarrGroupId)")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
            }
            
            Spacer()
            
            Button {
                // Could add navigation to group detail here
            } label: {
                Image(systemName: "chevron.right")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
        }
        .padding()
        .background(Color(.systemBackground))
        .cornerRadius(12)
        .overlay(
            RoundedRectangle(cornerRadius: 12)
                .stroke(Color(.systemGray4), lineWidth: 1)
        )
    }
    
    private var iconName: String {
        group.privacyType.lowercased() == "private" ? "lock.circle.fill" : "globe"
    }
    
    private var privacyIcon: String {
        group.privacyType.lowercased() == "private" ? "lock.fill" : "globe"
    }
}

// MARK: - Preview

#Preview("Empty State") {
    Form {
        Section {
            MyGroupsPickerView(selectedGroup: .constant(nil))
        }
    }
}

#Preview("With Selection") {
    @Previewable @State var selectedGroup: GroupDTO? = GroupDTO(
        pulsarrGroupId: 1,
        ratingSystemId: 1,
        name: "My Awesome Group",
        privacyType: "public"
    )
    
    Form {
        Section {
            MyGroupsPickerView(selectedGroup: $selectedGroup)
        }
    }
}
