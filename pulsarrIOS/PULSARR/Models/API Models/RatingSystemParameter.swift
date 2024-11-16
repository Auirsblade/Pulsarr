import Foundation

struct RatingSystemParameter: Identifiable, Codable {
    let id: Int
    let name: String
    let parameterRatingMax: String
    let ratingSystemId: Int

    enum CodingKeys: String, CodingKey {
        case id = "rating_system_parameter_id"
        case name
        case parameterRatingMax = "parameter_rating_max"
        case ratingSystemId = "rating_system_id"
    }
}
