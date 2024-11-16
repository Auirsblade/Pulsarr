import Foundation

struct RatingSystem: Identifiable, Codable {
    let id: Int
    let masterRatingType: String
    let name: String
    let ratingMax: String

    enum CodingKeys: String, CodingKey {
        case id = "rating_system_id"
        case masterRatingType = "master_rating_type"
        case name
        case ratingMax = "rating_max"
    }
}
