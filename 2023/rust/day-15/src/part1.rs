use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let sum = input.split(',').fold(0, |acc, s| {
        let block = s.chars().fold(0, |acc, c| {
            let ascii_value = c as u8;
            (acc + ascii_value as i32) * 17 % 256
        });
        acc + block
    });
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("1320", process(input)?);
        Ok(())
    }
}
