//
//  ContentView.swift
//  Shared
//
//  Created by Grzegorz Baranski on 08/02/2022.
//

import SwiftUI

struct LoginView: View {
    @Environment(\.dismiss) var dismiss
    @EnvironmentObject var client: HouseflowClient;
    
    @State var email: String = ""
    
    @State var showAlert: Bool = false
    
    @State var error: HouseflowClient.ServerError? = nil
    
    var body: some View {
        NavigationView {
            Form {
                TextField(text: $email, prompt: Text("example@icloud.com")) {
                    Text("Email")
                }
                .textContentType(.emailAddress)
                .keyboardType(.emailAddress)
                .disableAutocorrection(true)
            }
            .navigationBarTitle(Text("Sheet View"), displayMode: .inline)
            .navigationBarItems(
                leading: Button(action: {
                    print("Dismissing sheet view...")
                    dismiss()
                }) {
                    Text("Cancel").bold()
                },
                trailing: Button(action: {
                    print("logging in with \(email)")
                    Task {
                        let request = HouseflowLoginRequest(email: email, verificationCode: nil)
                        do {
                            let response = try await client.login(request: request)
                            print(response)
                        } catch let error as HouseflowClient.ServerError {
                            self.error = error
                            self.showAlert = true
                        }
                    }
                }) {
                    Text("Next").bold()
                })
        }.alert(isPresented: $showAlert, error: error) {_ in
            Button("Ok") {
                self.showAlert = false
                // Handle acknowledgement.
            }
        } message: {error in
            Text(error.localizedDescription)
        }
    }
    
}


struct ContentView: View {
    @State private var showingLoginView = false
    @StateObject var client = HouseflowClient()
    
    var body: some View {
        HStack{
            Button("Sign in") {
                showingLoginView.toggle()
            }
            .sheet(isPresented: $showingLoginView) {
                LoginView()
            }
            
            Button("Register") {}
            .disabled(true)
        }
        .buttonStyle(.bordered)
        .environmentObject(client)
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
