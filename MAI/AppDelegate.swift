//
//  AppDelegate.swift
//  MAI
//
//  Created by Curtis Tarr on 01/05/2022.
//

import Cocoa
import SwiftUI

class AppDelegate: NSObject, NSApplicationDelegate {
    private var statusBar: NSStatusBar?
    private var statusItem: NSStatusItem?
    
    func applicationDidFinishLaunching(_ notification: Notification) {
        statusBar = NSStatusBar.system
        statusItem = statusBar?.statusItem(withLength: NSStatusItem.variableLength)
        createMenu()
        startActivityThread()
    }
    
    private func createMenu() {
        if let statusBarButton = statusItem?.button {
            statusBarButton.title = "MAI"
            
            let mainMenu = NSMenu()
            
            let maiItem = NSMenuItem()
            maiItem.title = "MAI"
            maiItem.isEnabled = false
            mainMenu.addItem(maiItem)
            
            mainMenu.addItem(.separator())
            
            let quitItem = NSMenuItem()
            quitItem.title = "Quit"
            quitItem.action = #selector(self.terminate(_:))
            mainMenu.addItem(quitItem)
            
            statusItem?.menu = mainMenu
        }
    }
    
    @objc private func terminate(_ sender: Any?) {
        NSApplication.shared.terminate(self)
    }
    
    private func startActivityThread() {
        let queue = DispatchQueue(label: "activity-thread")
        
        queue.async {
            while (true) {
                let cpu = CPU()
                let ram = RAM()
                self.updateTitle(cpu: cpu, ram: ram)
                sleep(1)
            }
        }
    }
    
    private func updateTitle(cpu: CPU, ram: RAM) {
        DispatchQueue.main.async {
            let title = String(format: "%.2f%@ %.2fGB", cpu.getUsage(), "%", ram.getUsedInGB())
            self.statusItem?.button?.title = title
        }
    }
}
