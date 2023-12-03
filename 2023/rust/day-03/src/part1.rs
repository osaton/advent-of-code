use std::collections::HashMap;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mut symbols = HashMap::new();
    _input.lines().enumerate().for_each(|(y, line)| {
        (0..line.len() - 1).for_each(|x| {
            let char = line.chars().nth(x).unwrap();
            if !char.is_numeric() && char != '.' {
                symbols.insert(format!("{}:{}", y, x), char);
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

                    if touches_symbol(start_pos.unwrap(), (y, x - 1), &symbols) {
                        total += number;
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
fn touches_symbol(
    start: (usize, usize),
    end: (usize, usize),
    symbols: &HashMap<String, char>,
) -> bool {
    let previous_line = if start.0 == 0 { 0 } else { start.0 - 1 };

    for y in previous_line..=end.0 + 1 {
        let previous_column = if start.1 == 0 { 0 } else { start.1 - 1 };
        for x in previous_column..=end.1 + 1 {
            if let Some(_symbol) = symbols.get(&format!("{}:{}", y, x)) {
                return true;
            }
        }
    }
    return false;
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

        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
