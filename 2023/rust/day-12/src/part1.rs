use std::str::FromStr;

use crate::custom_error::AocError;
use strum::{Display, EnumString, VariantNames};

#[derive(Debug, Display, EnumString, Eq, PartialEq, Clone)]
enum Spring {
    #[strum(serialize = ".")]
    Operational,
    #[strum(serialize = "#")]
    Damaged,
    #[strum(serialize = "?")]
    Unknown,
}

struct Row {
    groups: Vec<usize>,
    springs: Vec<Spring>,
}

impl FromStr for Row {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let springs = parts
            .next()
            .unwrap()
            .chars()
            .map(|c| c.to_string().parse::<Spring>().expect("invalid spring"))
            .collect::<Vec<_>>();
        let groups = parts
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();

        Ok(Self { groups, springs })
    }
}

impl Row {
    fn count_combos(&mut self) -> u64 {
        // Simplify inner logic by adding a dummy spring to the front
        self.springs.push(Spring::Operational);
        let mut cache = vec![vec![None; self.springs.len()]; self.groups.len()];
        self.count_combos_inner(&self.springs, &self.groups, &mut cache)
    }

    fn count_combos_inner(
        &self,
        springs: &[Spring],
        groups: &[usize],
        cache: &mut Vec<Vec<Option<u64>>>,
    ) -> u64 {
        if groups.is_empty() {
            if springs.contains(&Spring::Damaged) {
                // Too many damaged springs
                return 0;
            } else {
                return 1;
            }
        }

        let mut total = 0;
        let group_size = groups[0];

        // If there are more groups than springs, there's no way to fit them all
        // We also need to have room for operational springs between groups
        if springs.len() < groups.iter().sum::<usize>() + groups.len() {
            return 0;
        }

        if let Some(cached) = cache[groups.len() - 1][springs.len() - 1] {
            return cached;
        }

        // Assume operational and check next position
        if springs[0] != Spring::Damaged {
            total += self.count_combos_inner(&springs[1..], groups, cache);
        }

        // Assume damaged and check next group
        if !springs[..group_size].contains(&Spring::Operational)
            && springs[group_size] != Spring::Damaged
        {
            total += self.count_combos_inner(&springs[group_size + 1..], &groups[1..], cache);
        }

        cache[groups.len() - 1][springs.len() - 1] = Some(total);

        total
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mut total = 0;
    for line in _input.lines() {
        let mut row = line.parse::<Row>()?;
        total += row.count_combos();
    }
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1";
        assert_eq!("21", process(input)?);

        let input = "???. 1";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
