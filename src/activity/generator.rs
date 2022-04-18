use std::process::{Command, Output};
use std::str::FromStr;

use crate::activity::cpu::CPU;
use crate::activity::ram::RAM;

pub fn get_activity() -> String {
    return parse_top_to_activity(run_top());
}

fn run_top() -> Output {
    return Command::new("top")
        .arg("-l")
        .arg("1")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute top: {}", e) });
}

fn parse_top_to_activity(top: Output) -> String {
    let output_string = String::from_utf8(top.stdout).unwrap();

    let lines = output_string.split("\n");

    let mut cpu_found = false;
    let mut ram_found = false;

    let mut cpu = CPU { user: 0.00, system: 0.00, idle: 0.00 };
    let mut ram = RAM { used: 0, wired: 0, unused: 0 };

    for s in lines {
        if cpu_found && ram_found {
            break;
        }

        if s.starts_with("CPU usage") {
            cpu = CPU::from_str(s).unwrap();
            cpu_found = true;
        } else if s.starts_with("PhysMem") {
            ram = RAM::from_str(s).unwrap();
            ram_found = true;
        }
    }

    return format!("{:.2}% {}MB", cpu.user + cpu.system, ram.used);
}