// GroupsView.swift

import SwiftUI
import OpenAPIClient

struct GroupsView: View {
    @StateObject private var viewModel = GroupsViewModel()
    @State private var isShowingCreateGroup = false
    
    var body: some View {
        ZStack {
            Color(.systemGroupedBackground)
                .ignoresSafeArea()
            
            VStack(spacing: 0) {
                // Main Content
                if viewModel.isLoading && viewModel.groups.isEmpty {
                    // Loading state
                    VStack {
                        ProgressView()
                            .scaleEffect(1.5)
                        Text("Loading groups...")
                            .foregroundColor(.secondary)
                            .padding(.top)
                    }
                    .frame(maxHeight: .infinity)
                } else if viewModel.groups.isEmpty {
                    // Empty state
                    VStack(spacing: 16) {
                        Image(systemName: "person.3.fill")
                            .font(.system(size: 60))
                            .foregroundColor(.gray)
                        
                        Text("No Groups Yet")
                            .font(.title2)
                            .fontWeight(.semibold)
                        
                        Text("Create your first group to get started!")
                            .font(.subheadline)
                            .foregroundColor(.secondary)
                            .multilineTextAlignment(.center)
                        
                        Button {
                            isShowingCreateGroup = true
                        } label: {
                            Label("Create Group", systemImage: "plus.circle.fill")
                                .font(.headline)
                                .foregroundColor(.white)
                                .padding()
                                .background(Color.blue)
                                .cornerRadius(12)
                        }
                        .padding(.top, 8)
                    }
                    .padding()
                    .frame(maxHeight: .infinity)
                } else {
                    // Groups list
                    ScrollView {
                        LazyVStack(spacing: 16) {
                            ForEach(viewModel.groups) { group in
                                GroupCardView(group: group)
                            }
                        }
                        .padding()
                    }
                }
                
                // Error message
                if let error = viewModel.errorMessage {
                    Text(error)
                        .foregroundColor(.white)
                        .padding()
                        .background(Color.red)
                        .cornerRadius(8)
                        .padding()
                }
            }
        }
        .navigationTitle("Groups")
        .navigationBarTitleDisplayMode(.large)
        .toolbar {
            ToolbarItem(placement: .navigationBarTrailing) {
                Button {
                    isShowingCreateGroup = true
                } label: {
                    Image(systemName: "plus.circle.fill")
                        .font(.title3)
                }
            }
        }
        .sheet(isPresented: $isShowingCreateGroup) {
            NavigationStack {
                CreateGroupView(onGroupCreated: {
                    isShowingCreateGroup = false
                    Task {
                        await viewModel.fetchAllGroups()
                    }
                })
            }
        }
        .task {
            await viewModel.fetchAllGroups()
        }
        .refreshable {
            await viewModel.fetchAllGroups()
        }
    }
}

#Preview {
    NavigationStack {
        GroupsView()
    }
}
