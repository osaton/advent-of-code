use crate::custom_error::AocError;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;
use strum::{Display, EnumString, VariantNames};

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
    fn expand_universe(&mut self) {
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

        let row_len = row_set.len();
        let col_len = col_set.len();

        let mut new_rows = row_set.into_iter().collect::<Vec<i32>>();
        new_rows.sort();
        let mut new_cols = col_set.into_iter().collect::<Vec<i32>>();
        new_cols.sort();

        // Add new rows
        for (y_addition, y) in new_rows.into_iter().enumerate() {
            let mut new_row = vec![];
            for x in 0..self.width {
                new_row.push(Item::Empty(IVec2::new(x as i32, y)));
            }
            self.map.insert(y as usize + y_addition, new_row);

            // Update galaxy positions
            self.galaxies.iter_mut().for_each(|galaxy| {
                if galaxy.y > y + y_addition as i32 {
                    galaxy.y += 1;
                }
            });
        }

        self.height += row_len;
        self.width += col_len;

        // Add new cols
        for (x_addition, x) in new_cols.into_iter().enumerate() {
            for y in 0..self.height {
                self.map[y].insert(
                    x as usize + x_addition,
                    Item::Empty(IVec2::new(x, y as i32)),
                );
            }
            // Update galaxy positions
            self.galaxies.iter_mut().for_each(|galaxy| {
                if galaxy.x > x + x_addition as i32 {
                    galaxy.x += 1;
                }
            });
        }
    }

    fn calculate_distance(&self, a: &IVec2, b: &IVec2) -> i32 {
        (a.x - b.x).abs() + (a.y - b.y).abs()
    }

    fn sum_of_distances(&mut self) -> i32 {
        let mut distances = vec![];

        let it = self.galaxies.clone().into_iter().enumerate();

        for (i, galaxy) in it {
            let mut galaxy_distances = vec![];
            // We can skip all the ones we've already calculated.
            // We only need to calculate each distance once.
            for j in (i + 1)..self.galaxies.len() {
                let other_galaxy = &self.galaxies[j];
                galaxy_distances.push(self.calculate_distance(&galaxy, other_galaxy));
            }
            distances.push(galaxy_distances);
        }

        distances.iter().flatten().sum::<i32>()
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
    universe.expand_universe();

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
        assert_eq!("374", process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn test_expand_universe() -> miette::Result<()> {
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

        universe.expand_universe();

        assert_eq!(
            "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......",
            universe.to_string()
        );
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
        universe.expand_universe();

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
        universe.expand_universe();

        assert_eq!(374, universe.sum_of_distances());
        Ok(())
    }
}
