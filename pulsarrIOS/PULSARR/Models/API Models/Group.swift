import Foundation

struct Group: Identifiable, Codable {
    let id: Int
    let name: String
    let privacyType: String
    let ratingSystemId: Int

    enum CodingKeys: String, CodingKey {
        case id = "pulsarr_group_id"
        case name
        case privacyType = "privacy_type"
        case ratingSystemId = "rating_system_id"
    }
}
