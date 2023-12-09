use crate::custom_error::AocError;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
struct Element {
    id: String,
    left: String,
    right: String,
    last: bool,
}

impl Element {
    fn new(id: String, left: String, right: String) -> Self {
        Self {
            id,
            left,
            right,
            last: false,
        }
    }

    fn set_last(&mut self, last: bool) {
        self.last = last;
    }

    fn get_node(&self, order: char) -> &str {
        match order {
            'L' => &self.left,
            'R' => &self.right,
            _ => panic!("Invalid order"),
        }
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

    let orders = parts
        .next()
        .expect("No chars found")
        .chars()
        .collect::<Vec<char>>();

    let mut elements: HashMap<String, Element> = HashMap::new();
    let it = parts
        .next()
        .expect("No elements found")
        .split('\n')
        .collect::<Vec<&str>>();

    let mut starting_element: Option<Element> = None;
    if let Some((last, items)) = it.split_last() {
        for (i, &item) in items.iter().enumerate() {
            if i == 0 {
                starting_element = Some(Element::from_str(item)?);
                let el = Element::from_str(item)?;
                elements.insert(el.id.to_owned(), el);
                continue;
            }
            let el = Element::from_str(item)?;
            elements.insert(el.id.to_owned(), el);
        }
        let mut el = Element::from_str(last)?;
        el.set_last(true);
        elements.insert(el.id.to_owned(), el);
    }

    let mut current_id = "AAA".to_string();
    let mut steps: u64 = 0;

    let mut seeking: bool = true;

    let mut total_iterations: u64 = 0;
    while seeking {
        total_iterations += 1;
        if total_iterations % 10000 == 0 {
            println!("steps: {}", steps);
        }
        orders.iter().enumerate().find_map(|(i, &order)| {
            steps += 1;

            if i == 0 && steps == 1 {
                if let Some(el) = &starting_element {
                    current_id = el.get_node(order).to_string();
                }
                return None;
            }
            let el = elements.get(&current_id).expect("Failed to get element");

            current_id = el.get_node(order).to_string();

            if el.id == current_id {
                println!("Loop detected: {} {}", el.id, current_id);
            }
            let el2 = elements.get(&current_id).expect("Failed to get element");

            if el2.last {
                seeking = false;
                return Some(steps);
            }

            None
        });
    }

    Ok(steps.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_single_run() -> miette::Result<()> {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("2", process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn test_loop_orders() -> miette::Result<()> {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
