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
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mut parts = _input.split('\n');
    let times = parts
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|number| number.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let distances = parts
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|number| number.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

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
