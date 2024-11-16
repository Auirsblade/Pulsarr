import Foundation

struct User: Identifiable, Codable {
    let id: Int
    let name: String

    enum CodingKeys: String, CodingKey {
        case id = "pulsarr_user_id"
        case name
    }
}
