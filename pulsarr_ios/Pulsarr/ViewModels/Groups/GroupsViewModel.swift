// GroupsViewModel.swift

import Foundation
import OpenAPIClient

@MainActor
final class GroupsViewModel: ObservableObject {
    @Published var groups: [GroupDTO] = []
    @Published var errorMessage: String?
    @Published var isLoading: Bool = false
    
    private let getRequest = GetRequest(takeSize: 50)
    
    func fetchAllGroups() async {
        isLoading = true
        errorMessage = nil
        
        defer { isLoading = false }
        
        do {
            groups = try await GroupService.shared.getAllGroups(getRequest: getRequest)
        } catch {
            errorMessage = "Failed to load groups: \(error.localizedDescription)"
            print("Error fetching groups:", error)
        }
    }
}
