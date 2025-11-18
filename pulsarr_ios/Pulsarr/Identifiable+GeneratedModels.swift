import Foundation
import OpenAPIClient


extension RatingSystem: @retroactive Identifiable {
    public var id: Int { ratingSystemId }
}

extension UserDTO: @retroactive Identifiable {
    public var id: Int { pulsarrUserId }
}

extension GroupDTO: @retroactive Identifiable {
    public var id: Int { pulsarrGroupId }
}
