use std::collections::HashMap;

use itertools::Itertools;

use crate::custom_error::AocError;

#[derive(Debug)]
struct Gear {
    value1: Option<u32>,
    value2: Option<u32>,
}

impl Gear {
    fn new() -> Self {
        Self {
            value1: None::<u32>,
            value2: None::<u32>,
        }
    }

    fn set_value(&mut self, value: u32) {
        if self.value1.is_none() {
            self.value1 = Some(value);
            return;
        }
        if self.value2.is_none() {
            self.value2 = Some(value);
        }
    }

    fn get_gear(&self) -> Option<u32> {
        if self.value1.is_none() || self.value2.is_none() {
            return None;
        }

        return Some(self.value1.unwrap() * self.value2.unwrap());
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mut gears = HashMap::new();
    _input.lines().enumerate().for_each(|(y, line)| {
        (0..line.len() - 1).for_each(|x| {
            let char = line.chars().nth(x).unwrap();
            if char == '*' {
                gears.insert((y, x), Gear::new());
            }
        });
    });

    let result = _input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let mut digits = vec![];
            let mut start_pos: Option<(usize, usize)> = None;
            let mut total = 0;
            for x in 0..=line.len() + 1 {
                let char = line.chars().nth(x);
                if char.is_some() && char.unwrap().is_numeric() {
                    if start_pos.is_none() {
                        start_pos = Some((y, x));
                    }
                    digits.push(char.unwrap().to_string());
                } else if !digits.is_empty() {
                    let number = digits.join("").parse::<u32>().unwrap();

                    if let Some(gear) =
                        find_gear(start_pos.unwrap(), (y, x - 1), number, &mut gears)
                    {
                        total += gear;
                    }
                    digits = vec![];
                    start_pos = None;
                }
            }
            total
        })
        .sum::<u32>();
    Ok(result.to_string())
}
#[tracing::instrument]
fn find_gear(
    start: (usize, usize),
    end: (usize, usize),
    number: u32,
    gears: &mut HashMap<(usize, usize), Gear>,
) -> Option<u32> {
    let previous_line = if start.0 == 0 { 0 } else { start.0 - 1 };

    for y in previous_line..=end.0 + 1 {
        let previous_column = if start.1 == 0 { 0 } else { start.1 - 1 };

        let range = if y == start.0 {
            // We can skip the actual number position
            vec![previous_column, end.1 + 1]
        } else {
            (previous_column..=end.1 + 1).collect_vec()
        };
        for x in range {
            if let Some(gear) = gears.get_mut(&(y, x)) {
                gear.set_value(number);
                return gear.get_gear();
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!("467835", process(input)?);
        Ok(())
    }
}
