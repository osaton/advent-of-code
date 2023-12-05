use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

use crate::custom_error::AocError;

#[derive(Debug)]
struct SeedRange {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

struct SeedMap {
    ranges: Vec<SeedRange>,
    name: String,
}

impl SeedRange {
    fn new(destination_range_start: u64, source_range_start: u64, range_length: u64) -> Self {
        Self {
            destination_range_start,
            source_range_start,
            range_length,
        }
    }

    fn get_destination(&self, seed: u64) -> u64 {
        let mut result = seed;

        if seed >= self.source_range_start && seed <= self.source_range_start + self.range_length {
            result = self.destination_range_start + (seed - self.source_range_start);
        }
        result
    }
}

impl SeedMap {
    fn get_destination(&self, source_value: u64) -> u64 {
        let result = self.ranges.iter().find_map(|range| {
            let result = range.get_destination(source_value);
            if result != source_value {
                Some(result)
            } else {
                None
            }
        });

        if let Some(result) = result {
            result
        } else {
            source_value
        }
    }
}

impl FromStr for SeedMap {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split('\n');
        let _name = lines.next().unwrap();

        let ranges = lines
            .map(|line| {
                // Parse each line into a SeedRange
                line.parse::<SeedRange>().unwrap()
            })
            .collect::<Vec<SeedRange>>();
        Ok(Self {
            ranges,
            name: _name.to_owned(),
        })
    }
}

impl FromStr for SeedRange {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        let destination_range_start = parts[0].parse::<u64>().unwrap();
        let source_range_start = parts[1].parse::<u64>().unwrap();
        let range_length = parts[2].parse::<u64>().unwrap();

        Ok(Self::new(
            destination_range_start,
            source_range_start,
            range_length,
        ))
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mut parts = _input.split("\n\n");
    let seeds = parts
        .next()
        .unwrap()
        .split("seeds:")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|number| number.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let rest = parts.collect::<Vec<&str>>();

    let maps = parse_ranges(rest);

    let mut destinations: HashMap<u64, u64> = HashMap::new();
    seeds.iter().for_each(|&seed_value| {
        // tota
        maps.iter().for_each(|seed_map| {
            let current_value = destinations.get(&seed_value).unwrap_or(&seed_value);
            destinations.insert(seed_value, seed_map.get_destination(*current_value));
        });
    });

    let result = destinations.values().sorted().next().unwrap();
    Ok(result.to_string())
}

fn parse_ranges(parts: Vec<&str>) -> Vec<SeedMap> {
    parts
        .iter()
        .map(|&part| part.parse::<SeedMap>().unwrap())
        .collect::<Vec<SeedMap>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!("35", process(input)?);
        Ok(())
    }
}
