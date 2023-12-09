use crate::custom_error::AocError;

#[tracing::instrument]
pub fn get_previous_in_sequence(sequence: &Vec<i64>) -> i64 {
    // Just reverse and it works for finding the previous number
    let mut previous_differences: Vec<i64> = sequence.iter().rev().copied().collect();
    let last_number = sequence.first().expect("Failed to get last number");
    let mut last_numbers = vec![];

    let number = loop {
        let differences = previous_differences
            .iter()
            .zip(previous_differences.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect::<Vec<i64>>();

        last_numbers.push(*differences.last().expect("Failed to get last number"));

        // All zeros means we've found the number
        if differences.iter().all(|&x| x == 0) {
            break last_numbers.iter().sum::<i64>() + last_number;
        }

        previous_differences = differences;
    };
    number
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let lines = _input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|s| s.parse::<i64>().expect("Failed to parse a number"))
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let sum = lines.iter().map(get_previous_in_sequence).sum::<i64>();
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!("2", process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn test_arithmetic_sequence() {
        assert_eq!(5, get_previous_in_sequence(&vec![10, 13, 16, 21, 30, 45]));
        assert_eq!(-3, get_previous_in_sequence(&vec![0, 3, 6, 9, 12, 15]));
        assert_eq!(0, get_previous_in_sequence(&vec![1, 3, 6, 10, 15, 21]));
    }
}
