use tracing_subscriber::fmt::format;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let result: u32 = _input
        .lines()
        .map(|line| {
            line.find(char::is_numeric)
                .map(|first| {
                    line.rfind(char::is_numeric)
                        .map(|last| {
                            let first = line[first..].chars().next().unwrap_or('0');
                            let last = line[last..].chars().next().unwrap_or('0');
                            let addition = format!("{}{}", first, last);
                            addition.parse::<u32>().unwrap_or(0)
                        })
                        .unwrap_or(first.try_into().unwrap())
                })
                .unwrap_or(0)
        })
        .sum();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
