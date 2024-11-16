//
//  ReviewCardViewModel.swift
//  PULSARR
//
//  Created by Ethan Hart on 10/6/24.
//

import Foundation

class ReviewCardViewModel: ObservableObject {
    @Published var albumTitle : String
    @Published var albumArists : String
    @Published var review : String
    @Published var rating : Int
    @Published var userId : Int
    @Published var groupId : Int
    
    init() {
        loadAlbumReviews()
    }
    
    func loadAlbumReviews() {
        // Dummy JSON data embedded as a string
        let jsonString = """
        [
            {
                "id": 1,
                "title": "Echoes of Silence",
                "description": "A hauntingly beautiful exploration of modern life.",
                "rating": 85
            },
            {
                "id": 2,
                "title": "The Dark Side of the Moon",
                "description": "A masterpiece that delves deep into the human psyche.",
                "rating": 95
            },
            {
                "id": 3,
                "title": "Abbey Road",
                "description": "Timeless tracks that have defined a generation.",
                "rating": 90
            },
            {
                "id": 4,
                "title": "Thriller",
                "description": "The best-selling album of all time with unforgettable hits.",
                "rating": 98
            },
            {
                "id": 5,
                "title": "Back in Black",
                "description": "An electrifying collection of rock anthems.",
                "rating": 92
            }
        ]
        """
        
        // Convert JSON string to Data
        guard let data = jsonString.data(using: .utf8) else {
            print("Failed to convert JSON string to Data.")
            return
        }
        
        // Decode JSON data
        let decoder = JSONDecoder()
        do {
            let reviews = try decoder.decode([AlbumReview].self, from: data)
            DispatchQueue.main.async {
                self.albumReviews = reviews
            }
        } catch {
            print("Error decoding JSON: \(error)")
        }
    }
}
