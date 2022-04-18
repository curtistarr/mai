use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct RAM {
    pub used: u32,
    pub wired: u32,
    pub unused: u32,
}

impl FromStr for RAM {
    type Err = std::num::ParseIntError;

    // Parses RAM usage from TOP into an instance of RAM
    // format = 'PhysMem: 7515M used (1430M wired), 135M unused.'
    fn from_str(top_ram_usage: &str) -> Result<Self, Self::Err> {
        let mut usages = top_ram_usage.split_whitespace()
            .filter(|s| s.ends_with("M"))
            .map(|s| s.strip_prefix("(").unwrap_or(s))
            .map(|s| s.strip_suffix("M").unwrap())
            .map(|s| s.parse::<u32>().unwrap());

        let used = usages.next().unwrap();
        let wired = usages.next().unwrap();
        let unused = usages.next().unwrap();

        Ok(RAM { used, wired, unused })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let activity = "PhysMem: 7515M used (1430M wired), 135M unused.";
        let ram = RAM::from_str(activity).unwrap();

        assert_eq!(ram, RAM { used: 7515, wired: 1430, unused: 135 });
    }
}