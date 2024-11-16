import Foundation

extension URLSession {
    /// Fetches and decodes data from the given URL into the specified Decodable type.
    /// - Parameters:
    ///   - url: The URL to fetch data from.
    ///   - completion: Completion handler with Result containing decoded data or an Error.
    func fetchData<T: Decodable>(from url: URL, completion: @escaping (Result<T, Error>) -> Void) {
        self.dataTask(with: url) { data, response, error in
            // Handle error scenarios
            if let error = error {
                completion(.failure(error))
                return
            }
            
            // Ensure data is received
            guard let data = data else {
                completion(.failure(NetworkError.noData))
                return
            }
            
            // Attempt to decode the data
            do {
                let decodedData = try JSONDecoder().decode(T.self, from: data)
                completion(.success(decodedData))
            } catch let decoderError {
                completion(.failure(decoderError))
            }
        }.resume()
    }
}

/// Defines possible network-related errors.
enum NetworkError: Error, LocalizedError {
    case noData
    
    var errorDescription: String? {
        switch self {
        case .noData:
            return "No data received from the server."
        }
    }
}
