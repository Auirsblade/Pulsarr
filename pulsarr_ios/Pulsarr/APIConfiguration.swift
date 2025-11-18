// APIConfiguration.swift
//
// Environment-based API configuration layer
//

import Foundation
import OpenAPIClient

/// Defines the available API environments
public enum APIEnvironment: String, CaseIterable {
    case development = "dev"
    case staging = "staging"
    case production = "prod"
    
    /// The base URL for this environment
    var baseURL: String {
        switch self {
        case .development:
            return "https://dev.pulsarr-music.com"
        case .staging:
            return "https://staging.pulsarr-music.com"
        case .production:
            return "https://pulsarr-music.com"
        }
    }
    
    /// Human-readable name for the environment
    var displayName: String {
        switch self {
        case .development:
            return "Development"
        case .staging:
            return "Staging"
        case .production:
            return "Production"
        }
    }
}

/// Configuration manager for the OpenAPI client
public final class APIConfiguration {
    
    /// The current environment
    public static var current: APIEnvironment = {
        // Automatically detect environment based on build configuration
        #if DEBUG
        return .development
        #elseif STAGING
        return .staging
        #else
        return .production
        #endif
    }()
    
    /// Configure the OpenAPI client with the current environment
    public static func configure(environment: APIEnvironment? = nil) {
        if let environment = environment {
            current = environment
        }
        
        // Update the OpenAPI client base path
        OpenAPIClientAPI.basePath = current.baseURL
        
        print("API configured for \(current.displayName) environment: \(current.baseURL)")
    }
    
    /// Configure with a custom base URL (useful for testing or local development)
    public static func configure(customBaseURL: String) {
        OpenAPIClientAPI.basePath = customBaseURL
        print("API configured with custom base URL: \(customBaseURL)")
    }
}

// MARK: - OpenAPIClientAPI Extension

extension OpenAPIClientAPI {
    
    /// Configure the API client for the specified environment
    public static func configure(for environment: APIEnvironment) {
        APIConfiguration.configure(environment: environment)
    }
    
    /// The current environment (read-only access)
    public static var environment: APIEnvironment {
        return APIConfiguration.current
    }
    
    /// Check if running in production
    public static var isProduction: Bool {
        return APIConfiguration.current == .production
    }
    
    /// Check if running in development
    public static var isDevelopment: Bool {
        return APIConfiguration.current == .development
    }
}

// MARK: - RequestBuilder Extension

extension RequestBuilder {
    
    /// Creates a new request builder with environment-aware URL
    public convenience init(
        method: String,
        path: String, // Use path instead of full URL
        parameters: [String: Any]? = nil,
        headers: [String: String] = [:],
        requiresAuthentication: Bool = false
    ) {
        // Construct full URL using current base path
        let baseURL = OpenAPIClientAPI.basePath
        let fullURL = path.hasPrefix("http") ? path : "\(baseURL)\(path)"
        
        self.init(
            method: method,
            URLString: fullURL,
            parameters: parameters,
            headers: headers,
            requiresAuthentication: requiresAuthentication
        )
    }
}
