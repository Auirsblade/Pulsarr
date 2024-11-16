import Foundation

struct Rating: Identifiable, Codable {
    let id: Int
    let comments: String
    let pulsarrUserId: Int
    let pulsarrGroupId: Int
    let ratingSystemId: Int
    let ratingValue: String

    enum CodingKeys: String, CodingKey {
        case id = "rating_id"
        case comments
        case pulsarrUserId = "pulsarr_user_id"
        case pulsarrGroupId = "pulsarr_group_id"
        case ratingSystemId = "rating_system_id"
        case ratingValue = "rating_value"
    }
}
