use std::str::FromStr;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

#[derive(PartialEq, Debug)]
pub struct CPU {
    pub user: f32,
    pub system: f32,
    pub idle: f32,
}

impl CPU {
    pub fn get_usage(&self) -> f32 {
        return self.user + self.system;
    }
}

impl FromStr for CPU {
    type Err = regex::Error;

    fn from_str(top_output_string: &str) -> Result<Self, Self::Err> {
        let parsed_output = parse_output(top_output_string);

        let user = parsed_output["user"].parse::<f32>().unwrap();
        let system = parsed_output["sys"].parse::<f32>().unwrap();
        let idle = parsed_output["idle"].parse::<f32>().unwrap();

        Ok(CPU { user, system, idle })
    }
}

fn parse_output(vm_stat_output_string: &str) -> Captures {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"CPU usage: (?P<user>.*)% user, (?P<sys>.*)% sys, (?P<idle>.*)% idle").unwrap();
    }
    return REGEX.captures_iter(vm_stat_output_string).last().unwrap();
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_from_str() {
        let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_path.push("resources/test/top-output.txt");
        let top_output_string = fs::read_to_string(file_path).unwrap();

        let cpu = CPU::from_str(&*top_output_string).unwrap();

        assert_eq!(cpu, CPU { user: 8.67, system: 8.8, idle: 83.23 });
    }

    #[test]
    fn test_get_usage() {
        let cpu = CPU { user: 3.9, system: 11.34, idle: 85.56 };

        assert_eq!(cpu.get_usage(), 15.24)
    }
}