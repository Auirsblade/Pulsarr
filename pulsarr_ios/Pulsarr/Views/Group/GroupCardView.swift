// GroupCardView.swift

import SwiftUI
import OpenAPIClient

struct GroupCardView: View {
    let group: GroupDTO
    
    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            // Group name and privacy badge
            HStack {
                Text(group.name)
                    .font(.title3)
                    .fontWeight(.semibold)
                    .foregroundColor(.primary)
                
                Spacer()
                
                // Privacy badge
                Text(group.privacyType.capitalized)
                    .font(.caption)
                    .fontWeight(.medium)
                    .foregroundColor(.white)
                    .padding(.horizontal, 8)
                    .padding(.vertical, 4)
                    .background(privacyColor(for: group.privacyType))
                    .cornerRadius(6)
            }
            
            // Rating system info (if available)
            if let ratingSystem = group.ratingSystem {
                HStack(spacing: 4) {
                    Image(systemName: "star.fill")
                        .font(.caption)
                        .foregroundColor(.orange)
                    Text(ratingSystem.name ?? "Custom Rating")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                }
            }
            
            // Creation date (if available)
            if let creationDate = group.creationDate {
                HStack(spacing: 4) {
                    Image(systemName: "calendar")
                        .font(.caption)
                        .foregroundColor(.gray)
                    Text("Created: \(formatDate(creationDate))")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            }
        }
        .padding()
        .frame(maxWidth: .infinity, alignment: .leading)
        .background(Color(.systemBackground))
        .cornerRadius(12)
        .shadow(color: Color.black.opacity(0.05), radius: 5, x: 0, y: 2)
    }
    
    private func privacyColor(for privacyType: String) -> Color {
        switch privacyType.lowercased() {
        case "public":
            return .green
        case "private":
            return .red
        default:
            return .blue
        }
    }
    
    private func formatDate(_ dateString: String) -> String {
        let inputFormatter = DateFormatter()
        inputFormatter.dateFormat = "yyyy-MM-dd'T'HH:mm:ss.SSSSSS"
        inputFormatter.locale = Locale(identifier: "en_US_POSIX")
        
        if let date = inputFormatter.date(from: dateString) {
            let displayFormatter = DateFormatter()
            displayFormatter.dateFormat = "yyyy-MM-dd"
            return displayFormatter.string(from: date)
        }
        
        // Fallback to original string if parsing fails
        return dateString
    }
}

#Preview {
    GroupCardView(
        group: GroupDTO(
            pulsarrGroupId: 1,
            ratingSystemId: 1,
            name: "My Group",
            privacyType: "public",
            creationDate: ISO8601DateFormatter().string(from: Date())
        )
    )
    .padding()
}
