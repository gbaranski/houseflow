//
//  Client.swift
//  houseflow
//
//  Created by Grzegorz Baranski on 08/02/2022.
//

import Foundation

struct HouseflowLoginRequest: Codable {
    var email: String
    var verificationCode: String?
    
    enum CodingKeys: String, CodingKey {
        case email
        case verificationCode = "verification-code"
    }
}

struct HouseflowLoggedIn: Codable {
    var accessToken: String
    var refreshToken: String
}

enum HouseflowLoginResponse: Codable {
    case loggedIn(HouseflowLoggedIn)
    case verificationCodeSent
}

extension HouseflowLoginResponse {
    enum CodingKeys: String, CodingKey {
        case loggedIn = "logged-in"
        case verificationCodeSent = "verification-code-sent"
    }
}

@MainActor
class HouseflowClient: ObservableObject {
    let url = URL(string: "http://192.168.1.193:6001/auth/login")!
    
    struct ServerError: Error, LocalizedError, Codable {
        var error: String
        var description: String
        
        enum CodingKeys: String, CodingKey {
            case error
            case description = "error-description"
        }
    }
    
    
    func login(request: HouseflowLoginRequest) async
    throws -> HouseflowLoginResponse {
        let body = try JSONEncoder().encode(request)
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.httpBody = body
        let (data, response) = try await URLSession.shared.data(for: request)
        guard (response as? HTTPURLResponse)?.statusCode == 200 else {
            let error = try JSONDecoder().decode(ServerError.self, from: data)
            throw error
        }
        let loginResponse = try JSONDecoder().decode(HouseflowLoginResponse.self, from: data)
        return loginResponse
    }
    
}
