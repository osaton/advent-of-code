use crate::custom_error::AocError;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;
use strum::{Display, EnumString};

use glam::IVec2;

#[derive(Debug, Display, EnumString, Eq, PartialEq, Clone)]
enum Item {
    #[strum(serialize = "#")]
    Galaxy(IVec2),
    #[strum(serialize = ".")]
    Empty(IVec2),
}

#[derive(Debug)]
struct Universe {
    map: Vec<Vec<Item>>,
    galaxies: Vec<IVec2>,
    width: usize,
    height: usize,
}

impl Universe {
    fn expand_universe(&mut self, expansion: usize) {
        let mut row_set: HashSet<i32> = HashSet::new();
        let mut col_set: HashSet<i32> = HashSet::new();

        for y in 0..self.height {
            row_set.insert(y as i32);
        }
        for x in 0..self.width {
            col_set.insert(x as i32);
        }

        self.galaxies.iter().for_each(|galaxy| {
            if row_set.contains(&galaxy.y) {
                row_set.remove(&galaxy.y);
            }
            if col_set.contains(&galaxy.x) {
                col_set.remove(&galaxy.x);
            }
        });

        let mut new_rows = row_set.into_iter().collect::<Vec<i32>>();
        new_rows.sort();
        let mut new_cols = col_set.into_iter().collect::<Vec<i32>>();
        new_cols.sort();

        // Add new rows
        for (y_addition, y) in new_rows.into_iter().enumerate() {
            // let mut new_row = vec![];
            // for x in 0..self.width {
            //     new_row.push(Item::Empty(IVec2::new(x as i32, y)));
            // }

            // for _ in 0..expansion {
            //     new_row.push(Item::Empty(IVec2::new(self.width as i32, y)));
            // }
            // self.map.insert(y as usize + y_addition, new_row);

            // Update galaxy positions
            self.galaxies.iter_mut().for_each(|galaxy| {
                if galaxy.y > y + (y_addition * expansion) as i32 {
                    galaxy.y += expansion as i32;
                }
            });
        }

        // Add new cols
        for (x_addition, x) in new_cols.into_iter().enumerate() {
            // Update galaxy positions
            self.galaxies.iter_mut().for_each(|galaxy| {
                if galaxy.x > x + (x_addition * expansion) as i32 {
                    galaxy.x += expansion as i32;
                }
            });
        }
    }

    fn calculate_distance(&self, a: &IVec2, b: &IVec2) -> i32 {
        (a.x - b.x).abs() + (a.y - b.y).abs()
    }

    fn sum_of_distances(&mut self) -> u64 {
        let mut distances = vec![];

        let it = self.galaxies.clone().into_iter().enumerate();

        for (i, galaxy) in it {
            let mut galaxy_distances = vec![];
            // We can skip all the ones we've already calculated.
            // We only need to calculate each distance once.
            for j in (i + 1)..self.galaxies.len() {
                let other_galaxy = &self.galaxies[j];
                galaxy_distances.push(self.calculate_distance(&galaxy, other_galaxy) as u64);
            }
            distances.push(galaxy_distances);
        }

        distances.iter().flatten().sum::<u64>()
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut parts = vec![];
        for row in &self.map {
            let mut result = String::new();
            for item in row {
                result.push_str(&format!("{}", item));
            }
            parts.push(result);
        }
        write!(f, "{}", parts.join("\n"))
    }
}

impl FromStr for Universe {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut galaxies = vec![];
        let map = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let mut item = c.to_string().parse::<Item>().expect("Failed to parse item");
                        match &mut item {
                            Item::Galaxy(pos) => {
                                pos.x = x as i32;
                                pos.y = y as i32;
                                galaxies.push(*pos);
                            }
                            Item::Empty(pos) => {
                                pos.x = x as i32;
                                pos.y = y as i32;
                            }
                        }
                        item
                    })
                    .collect::<Vec<Item>>()
            })
            .collect::<Vec<Vec<Item>>>();

        let width = map[0].len();
        let height = map.len();

        Ok(Self {
            map,
            width,
            height,
            galaxies,
        })
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut universe = Universe::from_str(input)?;
    universe.expand_universe(999_999);

    Ok(universe.sum_of_distances().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!("82000210", process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn test_calculate_distance() -> miette::Result<()> {
        let mut universe = Universe::from_str(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        )?;
        universe.expand_universe(1);

        assert_eq!(
            2,
            universe.calculate_distance(&IVec2::new(0, 0), &IVec2::new(2, 0))
        );
        assert_eq!(
            15,
            universe.calculate_distance(&IVec2::new(4, 0), &IVec2::new(9, 10))
        );
        assert_eq!(
            17,
            universe.calculate_distance(&IVec2::new(0, 2), &IVec2::new(12, 7))
        );
        Ok(())
    }

    #[test_log::test]
    fn test_sum_of_distances() -> miette::Result<()> {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let mut universe = Universe::from_str(input)?;
        universe.expand_universe(1);

        assert_eq!(374, universe.sum_of_distances());

        let mut universe = Universe::from_str(input)?;
        universe.expand_universe(9);

        assert_eq!(1030, universe.sum_of_distances());

        let mut universe = Universe::from_str(input)?;
        universe.expand_universe(99);

        assert_eq!(8410, universe.sum_of_distances());

        Ok(())
    }
}
