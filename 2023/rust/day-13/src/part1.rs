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
            .filter(|((_, a), (_, b))| a == b)
            .find_map(|((i, _), (j, _))| {
                let lines_prev = (rows[0..=i]).iter().rev();
                let lines_next = (rows[j..]).iter();

                lines_next
                    .zip(lines_prev)
                    .all(|(a, b)| a == b)
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
        let mut rows = Vec::new();
        let mut columns = Vec::new();
        for row in self.items.iter() {
            let mut row_string = String::new();
            for item in row.iter() {
                row_string.push_str(&item.to_string());
            }
            rows.push(row_string);
        }

        let columns_len = self.items[0].len();
        let rows_len = self.items.len();

        for x in 0..columns_len {
            let mut column_string = String::new();
            for y in 0..rows_len {
                column_string.push_str(&self.items[y][x].to_string());
            }
            columns.push(column_string);
        }
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
        assert_eq!("405", process(input)?);
        Ok(())
    }
}
