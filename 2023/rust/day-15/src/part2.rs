use crate::custom_error::AocError;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Operation {
    Remove,
    Upsert(u32),
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    lens_box: u32,
    operation: Operation,
}

impl Lens {}

impl FromStr for Lens {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(?<label>\w*)(?<operation>=|-)?(?<focal_length>\d*)$")
            .expect("Failed to compile regex");

        let caps = re.captures(s).expect("Failed to capture");

        let label = caps["label"].to_owned();
        let lens_box = label.chars().fold(0, |acc, c| {
            let ascii_value = c as u8;
            (acc + ascii_value as u32) * 17 % 256
        });
        if &caps["operation"] == "=" {
            let focal_length = caps["focal_length"]
                .parse::<u32>()
                .expect("Failed to parse focal length");
            return Ok(Self {
                label,
                lens_box,
                operation: Operation::Upsert(focal_length),
            });
        }

        Ok(Self {
            label,
            lens_box,
            operation: Operation::Remove,
        })
    }
}
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut positions: HashMap<String, (Lens, Option<usize>)> = HashMap::new();
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];

    input.split(',').fold(0, |acc, s| {
        let lens = s.parse::<Lens>().expect("Failed to parse lens");
        let mut to_update = Vec::new();

        match positions.entry(lens.label.clone()) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                let (existing_lens, index) = entry.get_mut();

                match lens.operation {
                    Operation::Remove => {
                        //dbg!("removing", &lens.label);
                        if let Some(i) = *index {
                            boxes[lens.lens_box as usize].remove(i);

                            for lens in &boxes[lens.lens_box as usize][i..] {
                                to_update = boxes[lens.lens_box as usize][i..]
                                    .iter()
                                    .map(|l| l.label.clone())
                                    .collect();
                            }
                        }
                        entry.remove();
                    }
                    Operation::Upsert(_) => {
                        if let Some(i) = *index {
                            boxes[lens.lens_box as usize][i] = lens.clone();
                        } else {
                            // Handle the case when index is None if needed
                        }
                        *existing_lens = lens.clone();
                    }
                }
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                match lens.operation {
                    Operation::Remove => { /* Do nothing or handle if needed */ }
                    Operation::Upsert(_) => {
                        //dbg!("inserting", &lens.label, &focal_length);
                        boxes[lens.lens_box as usize].push(lens.clone());
                        let new_index = boxes[lens.lens_box as usize].len() - 1;
                        entry.insert((lens.clone(), Some(new_index)));
                    }
                }
            }
        }

        for label in to_update {
            let lens = positions.get_mut(&label).expect("Failed to get lens");
            let (_, index) = lens;
            if let Some(i) = index {
                *i -= 1;
            }
        }

        acc + lens.lens_box
    });

    let sum = boxes
        .iter()
        .enumerate()
        .map(|(box_number, slots)| {
            slots
                .iter()
                .enumerate()
                .map(|(i, lens)| {
                    let focal_length = match lens.operation {
                        Operation::Upsert(focal_length) => focal_length,
                        Operation::Remove => 0,
                    };
                    focal_length * (i as u32 + 1)
                })
                .sum::<u32>()
                * (box_number as u32 + 1)
        })
        .sum::<u32>();
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("145", process(input)?);
        Ok(())
    }
}
