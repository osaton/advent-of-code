use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let result: u32 = _input
        .lines()
        .map(|line| {
            let mut it = (0..line.len()).filter_map(|index: usize| {
                let reduced_line = &line[index..];
                let result = if reduced_line.starts_with("one") {
                    Some(1)
                } else if reduced_line.starts_with("two") {
                    Some(2)
                } else if reduced_line.starts_with("three") {
                    Some(3)
                } else if reduced_line.starts_with("four") {
                    Some(4)
                } else if reduced_line.starts_with("five") {
                    Some(5)
                } else if reduced_line.starts_with("six") {
                    Some(6)
                } else if reduced_line.starts_with("seven") {
                    Some(7)
                } else if reduced_line.starts_with("eight") {
                    Some(8)
                } else if reduced_line.starts_with("nine") {
                    Some(9)
                } else {
                    reduced_line.chars().next().unwrap().to_digit(10)
                };

                result
            });

            let first = it.next().expect("should be a number");

            match it.last() {
                Some(num) => first * 10 + num,
                None => first * 10 + first,
            }
        })
        .sum();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
