use crate::custom_error::AocError;

use itertools::Itertools;
use std::{str::FromStr, string::ToString};
use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString, Eq, PartialEq, Clone, Copy)]
enum Item {
    #[strum(serialize = ".")]
    Ash,
    #[strum(serialize = "#")]
    Rock,
}

struct Pattern {
    items: Vec<Vec<Item>>,
}

impl Pattern {
    fn find_mirror_position(&self, rows: &[String]) -> Option<u32> {
        rows.iter()
            .enumerate()
            .tuple_windows()
            .filter(|((_, a), (_, b))| {
                a == b || a.chars().zip(b.chars()).filter(|(a, b)| a != b).count() <= 1
            })
            .find_map(|((i, _), (j, _))| {
                let lines_prev = (rows[0..=i]).iter().map(|line| line.chars()).rev();
                let lines_next = (rows[j..]).iter().map(|line| line.chars());

                (lines_next
                    .flatten()
                    .zip(lines_prev.flatten())
                    .filter(|(a, b)| a != b)
                    .count()
                    == 1)
                    //.all(|(a, b)| a == b || a.chars().zip(b.chars()).filter(|(a, b)| a != b).count() <= 1)
                    .then_some(i as u32 + 1)
            })
    }
    fn find_mirror(&self) -> (Option<u32>, Option<u32>) {
        let (rows, columns) = self.stringified_items();

        let row_position = self.find_mirror_position(&rows);
        let column_position = self.find_mirror_position(&columns);
        (column_position, row_position)
    }

    fn summarize(&self) -> u32 {
        let (columns, rows) = self.find_mirror();

        if let Some(count) = rows {
            return count * 100;
        }

        if let Some(count) = columns {
            return count;
        }

        0
    }

    fn stringified_items(&self) -> (Vec<String>, Vec<String>) {
        let rows = self
            .items
            .iter()
            .map(|row| {
                row.iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>();
        let columns = (0..self.items[0].len())
            .map(|x| {
                (0..self.items.len())
                    .map(|y| self.items[y][x].to_string())
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>();

        (rows, columns)
    }
}

impl FromStr for Pattern {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<Item>().expect("invalid item"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(Self { items })
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let res = input
        .split("\n\n")
        .map(|s| {
            let pattern = s.parse::<Pattern>().unwrap();
            pattern.summarize()
        })
        .sum::<u32>();
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!("400", process(input)?);
        Ok(())
    }
}
