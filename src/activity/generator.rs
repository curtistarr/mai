use std::process::{Command, Output};
use std::str::FromStr;

use crate::activity::cpu::CPU;
use crate::activity::ram::RAM;

pub fn get_activity() -> String {
    let top_output = run_top();
    let vm_stat_output = run_vm_stat();

    let top_output_string = String::from_utf8(top_output.stdout).unwrap();
    let vm_stat_output_string = String::from_utf8(vm_stat_output.stdout).unwrap();

    let cpu = CPU::from_str(&*top_output_string).unwrap();
    let ram = RAM::from_str(&*vm_stat_output_string).unwrap();

    return format!("{:.2}% {:.2}GB", cpu.get_usage(), ram.get_gb_used());
}

fn run_top() -> Output {
    return Command::new("top")
        .arg("-l")
        .arg("1")
        .arg("-s")
        .arg("0")
        .arg("-n")
        .arg("0")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute top: {}", e) });
}

fn run_vm_stat() -> Output {
    return Command::new("vm_stat")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute vm_stat: {}", e) });
}