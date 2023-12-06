use crate::custom_error::AocError;
use tracing::{debug, error, info, warn};

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    fn get_ways_to_win(&self) -> u64 {
        let mut ways = 0;
        for i in 0..self.time {
            let speed = i;
            let time = self.time - speed;
            let distance = speed * time;

            if distance > self.distance {
                ways += 1;
            }
        }

        ways
    }
}

fn parse_line(line: Option<&str>) -> Result<u64, &str> {
    line.ok_or("No line found")
        .and_then(|line| line.split(':').last().ok_or("Missing ':'"))
        .map(|nums| nums.split_whitespace().collect::<String>())
        .and_then(|s| s.parse::<u64>().map_err(|_| "Failed to parse"))
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mut parts = _input.lines();
    let time = parse_line(parts.next()).expect("Failed to parse time");
    let distance = parse_line(parts.next()).expect("Failed to parse time");

    let race = Race::new(time, distance);
    Ok(race.get_ways_to_win().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("71503", process(input)?);
        Ok(())
    }
}
