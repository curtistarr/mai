//
//  Main.swift
//  MAI
//
//  Created by Curtis Tarr on 30/04/2022.
//

import SwiftUI

@main
struct Main: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self)
    private var appDelegate
    
    var body: some Scene {
        WindowGroup {
            EmptyView()
                .frame(width: .zero)
        }
    }
}
