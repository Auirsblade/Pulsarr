import Foundation

/// Enum representing possible API-related errors with more context.
enum APIError: Error, LocalizedError {
    case invalidResponse
    case serverError(statusCode: Int)
    case decodingError
    case unknown
    
    var errorDescription: String? {
        switch self {
        case .invalidResponse:
            return "Invalid response from the server."
        case .serverError(let statusCode):
            return "Server returned an error with status code \(statusCode)."
        case .decodingError:
            return "Failed to decode the response."
        case .unknown:
            return "An unknown error occurred."
        }
    }
}

/// Service responsible for handling all API calls.
class APIService {
    static let shared = APIService()
    public init() {}
    
    /// Base URL for the API. Replace with your actual base URL.
    private let baseURL = URL(string: "http://0.0.0.0:3003")!
    
    // MARK: - User Endpoints
    
    /// Fetches a PulsarrUser by ID.
    /// - Parameters:
    ///   - id: The user ID.
    ///   - completion: Completion handler with Result containing PulsarrUser or an Error.
    func getPulsarrUser(by id: Int, completion: @escaping (Result<User, Error>) -> Void) {
        let url = baseURL.appendingPathComponent("/user/\(id)")
        URLSession.shared.fetchData(from: url, completion: completion)
    }
    
    /// Adds a new PulsarrUser.
    /// - Parameters:
    ///   - user: The PulsarrUser to add.
    ///   - completion: Completion handler with Result containing Bool or an Error.
    func addPulsarrUser(_ user: User, completion: @escaping (Result<Bool, Error>) -> Void) {
        let url = baseURL.appendingPathComponent("/user/")
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        
        do {
            let jsonData = try JSONEncoder().encode(user)
            request.httpBody = jsonData
        } catch {
            completion(.failure(error))
            return
        }
        
        // Perform the network request
        URLSession.shared.dataTask(with: request) { data, response, error in
            // Handle error scenarios
            if let error = error {
                completion(.failure(error))
                return
            }
            
            // Ensure a valid HTTP response
            guard let httpResponse = response as? HTTPURLResponse else {
                completion(.failure(APIError.invalidResponse))
                return
            }
            
            // Check for successful status codes (200-299)
            guard (200...299).contains(httpResponse.statusCode) else {
                completion(.failure(APIError.serverError(statusCode: httpResponse.statusCode)))
                return
            }
            
            // Decode the response as Bool
            guard let data = data else {
                completion(.failure(APIError.invalidResponse))
                return
            }
            
            do {
                let success = try JSONDecoder().decode(Bool.self, from: data)
                completion(.success(success))
            } catch {
                completion(.failure(APIError.decodingError))
            }
        }.resume()
    }
    
    // MARK: - Group Endpoints
    
    /// Fetches a PulsarrGroup by ID.
    /// - Parameters:
    ///   - id: The group ID.
    ///   - completion: Completion handler with Result containing PulsarrGroup or an Error.
    func getPulsarrGroup(by id: Int, completion: @escaping (Result<Group, Error>) -> Void) {
        let url = baseURL.appendingPathComponent("/group/\(id)")
        URLSession.shared.fetchData(from: url, completion: completion)
    }
    
    /// Adds a new PulsarrGroup.
    /// - Parameters:
    ///   - group: The PulsarrGroup to add.
    ///   - completion: Completion handler with Result containing Bool or an Error.
    func addPulsarrGroup(_ group: Group, completion: @escaping (Result<Bool, Error>) -> Void) {
        let url = baseURL.appendingPathComponent("/group/")
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        
        do {
            let jsonData = try JSONEncoder().encode(group)
            request.httpBody = jsonData
        } catch {
            completion(.failure(error))
            return
        }
        
        // Perform the network request
        URLSession.shared.dataTask(with: request) { data, response, error in
            // Handle error scenarios
            if let error = error {
                completion(.failure(error))
                return
            }
            
            // Ensure a valid HTTP response
            guard let httpResponse = response as? HTTPURLResponse else {
                completion(.failure(APIError.invalidResponse))
                return
            }
            
            // Check for successful status codes (200-299)
            guard (200...299).contains(httpResponse.statusCode) else {
                completion(.failure(APIError.serverError(statusCode: httpResponse.statusCode)))
                return
            }
            
            // Decode the response as Bool
            guard let data = data else {
                completion(.failure(APIError.invalidResponse))
                return
            }
            
            do {
                let success = try JSONDecoder().decode(Bool.self, from: data)
                completion(.success(success))
            } catch {
                completion(.failure(APIError.decodingError))
            }
        }.resume()
    }
    
    // MARK: - Rating System Endpoints
    
    /// Fetches a RatingSystem by ID.
    /// - Parameters:
    ///   - id: The rating system ID.
    ///   - completion: Completion handler with Result containing RatingSystem or an Error.
    func getRatingSystem(by id: Int, completion: @escaping (Result<RatingSystem, Error>) -> Void) {
        let url = baseURL.appendingPathComponent("/rating-system/\(id)")
        URLSession.shared.fetchData(from: url, completion: completion)
    }
    
    /// Fetches all rating types.
    /// - Parameter completion: Completion handler with Result containing an array of Strings or an Error.
    func getRatingTypes(completion: @escaping (Result<[String], Error>) -> Void) {
        let url = baseURL.appendingPathComponent("/rating-system/ratingTypes")
        URLSession.shared.fetchData(from: url, completion: completion)
    }
    
    /// Fetches a RatingSystemParameter by ID.
    /// - Parameters:
    ///   - id: The rating system parameter ID.
    ///   - completion: Completion handler with Result containing RatingSystemParameter or an Error.
    func getRatingSystemParameter(by id: Int, completion: @escaping (Result<RatingSystemParameter, Error>) -> Void) {
        let url = baseURL.appendingPathComponent("/rating-system_parameter/\(id)")
        URLSession.shared.fetchData(from: url, completion: completion)
    }
    
    /// Fetches a Rating by ID.
    /// - Parameters:
    ///   - id: The rating ID.
    ///   - completion: Completion handler with Result containing Rating or an Error.
    func getRating(by id: Int, completion: @escaping (Result<Rating, Error>) -> Void) {
        let url = baseURL.appendingPathComponent("/rating/\(id)")
        URLSession.shared.fetchData(from: url, completion: completion)
    }
    
    /// Fetches a RatingDetail by ID.
    /// - Parameters:
    ///   - id: The rating detail ID.
    ///   - completion: Completion handler with Result containing RatingDetail or an Error.
    func getRatingDetail(by id: Int, completion: @escaping (Result<RatingDetail, Error>) -> Void) {
        let url = baseURL.appendingPathComponent("/rating-detail/\(id)")
        URLSession.shared.fetchData(from: url, completion: completion)
    }
    
    // Implement other API calls as needed following the above pattern.
}
