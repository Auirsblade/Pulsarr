//
//  pulsarrIOSApp.swift
//  pulsarrIOS
//
//  Created by Ethan Hart on 3/23/24.
//

import SwiftUI

@main
struct pulsarrIOSApp: App {
    let persistenceController = PersistenceController.shared

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environment(\.managedObjectContext, persistenceController.container.viewContext)
        }
    }
}
