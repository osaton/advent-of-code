use std::collections::{HashMap, HashSet};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mut card_copies: HashMap<u32, u32> = HashMap::new();
    let result = _input
        .lines()
        .map(|line| {
            let mut parts = line.split(':');

            let card_id = parts
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

            let copy_count = *card_copies.get(&card_id).unwrap_or(&0);
            let mut target_card = 1;
            winning_numbers.iter().for_each(|number| {
                if our_numbers.contains(number) {
                    let copies = card_copies.get(&(card_id + target_card)).unwrap_or(&0);
                    card_copies.insert(card_id + target_card, copies + 1 + copy_count);
                    target_card += 1;
                }
            });

            // copies + original
            copy_count + 1
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
        assert_eq!("30", process(input)?);
        Ok(())
    }
}
