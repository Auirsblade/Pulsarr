import Foundation

struct RatingDetail: Identifiable, Codable {
    let id: Int
    let ratingId: Int
    let ratingSystemParameterId: Int
    let ratingValue: String

    enum CodingKeys: String, CodingKey {
        case id = "rating_detail_id"
        case ratingId = "rating_id"
        case ratingSystemParameterId = "rating_system_parameter_id"
        case ratingValue = "rating_value"
    }
}
