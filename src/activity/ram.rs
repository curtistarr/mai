use std::str::FromStr;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

static BYTES_TO_MB: f32 = 1048576.0;
static MB_TO_GB: f32 = 1024.0;

#[derive(PartialEq, Debug)]
pub struct RAM {
    pub active_mb: f32,
    pub wired_mb: f32,
    pub compressed_mb: f32,
}

impl RAM {
    pub fn get_gb_used(&self) -> f32 {
        return (self.active_mb + self.wired_mb + self.compressed_mb) / MB_TO_GB;
    }
}

impl FromStr for RAM {
    type Err = regex::Error;

    fn from_str(vm_stat_output_string: &str) -> Result<Self, Self::Err> {
        let parsed_output = parse_output(vm_stat_output_string);

        let page_size = parsed_output["page_size"].parse::<f32>().unwrap();
        let active_mb = (parsed_output["active"].parse::<f32>().unwrap() * page_size) / BYTES_TO_MB;
        let wired_mb = (parsed_output["wired"].parse::<f32>().unwrap() * page_size) / BYTES_TO_MB;
        let compressed_mb = (parsed_output["compressed"].parse::<f32>().unwrap() * page_size) / BYTES_TO_MB;

        Ok(RAM { active_mb, wired_mb, compressed_mb })
    }
}

fn parse_output(vm_stat_output_string: &str) -> Captures {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"\(page size of (?P<page_size>\d+) bytes\)(.|\n)+Pages active:\s+(?P<active>\d+)\.(.|\n)+Pages wired down:\s+(?P<wired>\d+)\.(.|\n)+Pages occupied by compressor:\s+(?P<compressed>\d+)\.").unwrap();
    }
    return REGEX.captures(vm_stat_output_string).unwrap();
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_from_str() {
        let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_path.push("resources/test/vm_stat-output.txt");
        let vm_stat_output_string = fs::read_to_string(file_path).unwrap();

        let ram = RAM::from_str(&*vm_stat_output_string).unwrap();

        assert_eq!(ram, RAM { active_mb: 2188.0469, wired_mb: 1304.1094, compressed_mb: 1917.4688 });
    }

    #[test]
    fn test_get_gb_used() {
        let ram = RAM { active_mb: 2188.0469, wired_mb: 1304.1094, compressed_mb: 1917.4688 };

        assert_eq!(ram.get_gb_used(), 5.282837)
    }
}