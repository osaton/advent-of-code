use std::collections::HashSet;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let result = _input
        .lines()
        .map(|line| {
            let mut parts = line.split(':');

            let _card_id = parts
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let sections = parts.next().unwrap().split('|').collect::<Vec<&str>>();

            let winning_numbers = sections[0]
                .split(' ')
                .filter(|number| !number.is_empty())
                .map(|number| number.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let our_numbers = sections[1]
                .split(' ')
                .filter(|number| !number.is_empty())
                .map(|number| number.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();

            let mut total = 0;
            let mut hits = 0;
            winning_numbers.iter().for_each(|number| {
                let hit = our_numbers.contains(number);

                if hit {
                    let addition = if hits == 0 { 1 } else { 2_u32.pow(hits - 1) };
                    total += addition;
                    hits += 1;
                }
            });
            total
        })
        .sum::<u32>();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("13", process(input)?);
        Ok(())
    }
}
