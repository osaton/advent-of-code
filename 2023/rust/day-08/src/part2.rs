use crate::custom_error::AocError;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone)]
struct Element {
    id: String,
    left: String,
    right: String,
}

impl Element {
    fn new(id: String, left: String, right: String) -> Self {
        Self { id, left, right }
    }
}

impl FromStr for Element {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(?P<id>\w+)\s=\s\((?P<left>\w+),\s(?P<right>\w+)\)$")
            .expect("Failed to compile regex");

        let caps = re.captures(s).expect("Failed to capture");

        Ok(Self::new(
            caps["id"].to_owned(),
            caps["left"].to_owned(),
            caps["right"].to_owned(),
        ))
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mut parts = _input.split("\n\n");

    let orders = parts.next().expect("No chars found");

    let mut elements: HashMap<String, Element> = HashMap::new();
    let it = parts
        .next()
        .expect("No elements found")
        .split('\n')
        .collect::<Vec<&str>>();

    for &item in it.iter() {
        let el = Element::from_str(item)?;
        elements.insert(el.id.to_owned(), el);
    }

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    fn lcm(a: usize, b: usize) -> usize {
        a / gcd(a, b) * b
    }

    let number = elements
        .keys()
        .filter(|&id| id.ends_with('Z'))
        .map(|mut element| {
            return orders
                .chars()
                .cycle()
                .position(|instruction| {
                    element = match instruction {
                        'L' => &elements.get(element).unwrap().left,
                        'R' => &elements.get(element).unwrap().right,
                        _ => unreachable!(),
                    };
                    element.ends_with('Z')
                })
                .expect("Failed to find position")
                + 1;
        })
        .reduce(lcm)
        .expect("Failed to reduce");

    Ok(number.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!("7", process(input)?);
        Ok(())
    }
}
