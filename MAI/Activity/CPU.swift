//
//  CPU.swift
//  MAI
//
//  Created by Curtis Tarr on 01/05/2022.
//

import Foundation

class CPU: NSObject {
    private static var REGEX = try! NSRegularExpression(pattern: "CPU usage: (.*)% user, (.*)% sys, (.*)% idle")
    
    private var user: Float
    private var system: Float
    private var idle: Float
    
    override init() {
        self.user = 0.0
        self.system = 0.0
        self.idle = 0.0
        
        let topOutput = runTop()
        
        if let match = CPU.REGEX.matches(in: topOutput, range: NSRange(topOutput.startIndex..., in: topOutput)).last {
            if let userRange = Range(match.range(at: 1), in: topOutput) {
                self.user = (String(topOutput[userRange]) as NSString).floatValue
            }
            
            if let systemRange = Range(match.range(at: 2), in: topOutput) {
                self.system = (String(topOutput[systemRange]) as NSString).floatValue
            }
            
            if let idleRange = Range(match.range(at: 3), in: topOutput) {
                self.idle = (String(topOutput[idleRange]) as NSString).floatValue
            }
        }
    }
    
    public func getUsage() -> Float {
        return user + system
    }
}

private func runTop() -> String {
    do {
        let process = Process()
        let pipe = Pipe()
        
        process.standardOutput = pipe
        process.standardError = pipe
        process.arguments = ["-l", "2", "-s", "1", "-n", "0"]
        process.launchPath = "/usr/bin/top"
        try process.run()
        process.waitUntilExit()
        
        let data = pipe.fileHandleForReading.readDataToEndOfFile()
        let output = String(data: data, encoding: .utf8)!
        
        return output
    } catch {
        print("Error running top")
        exit(0)
    }
}
