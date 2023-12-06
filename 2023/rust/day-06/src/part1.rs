use crate::custom_error::AocError;
use tracing::{debug, error, info, warn};

struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn new(time: u32, distance: u32) -> Self {
        Self { time, distance }
    }

    fn get_ways_to_win(&self) -> u32 {
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

#[tracing::instrument]
fn parse_line(line: Option<&str>) -> Result<Vec<u32>, &str> {
    line.ok_or("No line found")
        .and_then(|line| line.split(':').last().ok_or("Missing ':'"))
        .map(|nums| {
            nums.split_whitespace()
                .map(|s| s.parse::<u32>().map_err(|e| "Failed to parse"))
        })
        .and_then(|nums| nums.collect::<Result<Vec<u32>, &str>>())
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mut parts = _input.split('\n');
    let times = parse_line(parts.next()).expect("Failed to parse times");
    let distances = parse_line(parts.next()).expect("Failed to parse distances");

    let res = times
        .into_iter()
        .zip(distances.into_iter())
        .map(|pair| {
            let race = Race::new(pair.0, pair.1);
            race.get_ways_to_win()
        })
        .product::<u32>();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("288", process(input)?);
        Ok(())
    }
}
