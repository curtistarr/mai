use std::process::{Command, Output};
use std::str::FromStr;

use crate::activity::cpu::CPU;
use crate::activity::ram::RAM;

pub fn get_activity() -> String {
    let top_output = run_top();
    let output_string = String::from_utf8(top_output.stdout).unwrap();
    return parse_output_to_activity(output_string);
}

fn run_top() -> Output {
    return Command::new("top")
        .arg("-l")
        .arg("1")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute top: {}", e) });
}

fn parse_output_to_activity(output_string: String) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_top_to_activity() {
        let output = String::from("CPU usage: 8.82% user, 14.70% sys, 76.47% idle\nPhysMem: 7515M used (1430M wired), 135M unused.");
        let activity = parse_output_to_activity(output);

        assert_eq!(activity, String::from("23.52% 7515MB"));
    }
}