//
//  RAM.swift
//  MAI
//
//  Created by Curtis Tarr on 01/05/2022.
//

import Foundation

class RAM: NSObject {
    private static var REGEX = try! NSRegularExpression(pattern: #"\(page size of (\d+) bytes\)(.|\n)+Pages active:\s+(\d+)\.(.|\n)+Pages wired down:\s+(\d+)\.(.|\n)+Pages occupied by compressor:\s+(\d+)\."#)
    private static var BYTES_TO_MB = 1048576 as Float;
    private static var MB_TO_GB = 1024 as Float;
    
    private var activeMb: Float
    private var wiredMb: Float
    private var compressedMb: Float
    
    override init() {
        var pages: Float = 0
        self.activeMb = 0
        self.wiredMb = 0
        self.compressedMb = 0
        
        let vmStatOuput = runVmStat()
        
        if let match = RAM.REGEX.firstMatch(in: vmStatOuput, range: NSRange(vmStatOuput.startIndex..., in: vmStatOuput)) {
            if let pagesRange = Range(match.range(at: 1), in: vmStatOuput) {
                pages = (String(vmStatOuput[pagesRange]) as NSString).floatValue
            }
            
            if let activeRange = Range(match.range(at: 3), in: vmStatOuput) {
                self.activeMb = ((String(vmStatOuput[activeRange]) as NSString).floatValue * pages) / RAM.BYTES_TO_MB
            }
            
            if let wiredRange = Range(match.range(at: 5), in: vmStatOuput) {
                self.wiredMb = ((String(vmStatOuput[wiredRange]) as NSString).floatValue * pages) / RAM.BYTES_TO_MB
            }
            
            if let compressedRange = Range(match.range(at: 7), in: vmStatOuput) {
                self.compressedMb = ((String(vmStatOuput[compressedRange]) as NSString).floatValue * pages) / RAM.BYTES_TO_MB
            }
        }
    }
    
    public func getUsedInGB() -> Float {
        return (activeMb + wiredMb + compressedMb) / RAM.MB_TO_GB
    }
}

private func runVmStat() -> String {
    do {
        let process = Process()
        let pipe = Pipe()
        
        process.standardOutput = pipe
        process.standardError = pipe
        process.launchPath = "/usr/bin/vm_stat"
        try process.run()
        process.waitUntilExit()
        
        let data = pipe.fileHandleForReading.readDataToEndOfFile()
        let output = String(data: data, encoding: .utf8)!
        
        return output
    } catch {
        print("Error running vm_stat")
        exit(0)
    }
}
