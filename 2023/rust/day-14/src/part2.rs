use crate::custom_error::AocError;
use std::hash::{Hash, Hasher};
use std::{collections::HashMap, str::FromStr};

use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString, Eq, PartialEq, Clone, Copy, Hash)]
enum Tile {
    #[strum(serialize = "O")]
    Rock,
    #[strum(serialize = "#")]
    CubeRock,
    #[strum(serialize = ".")]
    Empty,
}

#[derive(Eq, PartialEq, Debug)]
struct MatrixWrapper<T>(Vec<Vec<T>>);

impl<T: Hash> Hash for MatrixWrapper<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for row in &self.0 {
            for item in row {
                item.hash(state);
            }
        }
    }
}

impl<T> MatrixWrapper<T> {
    fn new(matrix: Vec<Vec<T>>) -> Self {
        MatrixWrapper(matrix)
    }
}

#[derive(Debug)]
struct ReflectorDish {
    tiles: Vec<Vec<Tile>>,
}

impl ReflectorDish {
    fn tilt_north(&mut self) {
        let mut new_tiles = vec![vec![Tile::Empty; self.tiles[0].len()]; self.tiles.len()];

        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                match tile {
                    Tile::Rock => {
                        //let new_y = avail_position.get(&x);
                        let mut new_y = y;
                        // check previous y position entries
                        let mut available_y = y;
                        loop {
                            if new_y == 0 {
                                break;
                            }
                            new_y -= 1;
                            if new_tiles[new_y][x] == Tile::CubeRock
                                || new_tiles[new_y][x] == Tile::Rock
                            {
                                break;
                            }
                            if new_tiles[new_y][x] == Tile::Empty {
                                available_y = new_y;
                            }
                        }

                        new_tiles[available_y][x] = Tile::Rock;
                        if available_y != y {
                            new_tiles[y][x] = Tile::Empty;
                        }
                    }
                    Tile::CubeRock => {
                        new_tiles[y][x] = Tile::CubeRock;
                    }
                    Tile::Empty => {
                        new_tiles[y][x] = Tile::Empty;
                    }
                }
            }
        }

        self.tiles = new_tiles;
    }

    fn rotate_left(&mut self) {
        let rows = self.tiles.len();
        let cols = self.tiles[0].len();
        let mut new_tiles = vec![vec![Tile::Empty; rows]; cols];

        for i in 0..rows {
            for j in 0..cols {
                new_tiles[cols - 1 - j][i] = self.tiles[i][j];
            }
        }

        self.tiles = new_tiles;
    }

    fn rotate_right(&mut self) {
        let rows = self.tiles.len();
        let cols = self.tiles[0].len();
        let mut new_tiles = vec![vec![Tile::Empty; rows]; cols];

        for i in 0..rows {
            for j in 0..cols {
                new_tiles[j][rows - 1 - i] = self.tiles[i][j];
            }
        }

        self.tiles = new_tiles;
    }

    fn cycle(&mut self, cycles: usize) {
        let mut cache: HashMap<MatrixWrapper<Tile>, (usize, Vec<Vec<Tile>>)> = HashMap::new();
        let mut cycle = 0;

        loop {
            cycle += 1;
            let tiles = MatrixWrapper::new(self.tiles.clone());

            // Check cached values and skip ahead if possible
            if let Some((cached_cycle, new_tiles)) = cache.get(&tiles) {
                let cycle_length = cycle - cached_cycle;
                let remaining_cycles = cycles - cycle;
                let full_cycles_to_skip = remaining_cycles / cycle_length;
                cycle += full_cycles_to_skip * cycle_length;

                self.tiles = new_tiles.clone();
                if cycle >= cycles {
                    break;
                }

                continue;
            }

            // North
            self.tilt_north();
            // West
            self.rotate_right();
            self.tilt_north();
            // South
            self.rotate_right();
            self.tilt_north();
            // East
            self.rotate_right();
            self.tilt_north();
            // Back to North
            self.rotate_right();

            let new_tiles = self.tiles.clone();
            cache.insert(tiles, (cycle, new_tiles));
        }
    }

    fn calculate_load(&self) -> usize {
        self.tiles
            .iter()
            .rev()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .filter(|tile| **tile == Tile::Rock)
                    .map(|_| (i + 1))
                    .sum::<usize>()
            })
            .sum()
    }
}

impl FromStr for ReflectorDish {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<Tile>())
                    .collect::<Result<Vec<Tile>, _>>()
            })
            .collect::<Result<Vec<Vec<Tile>>, _>>()
            .expect("Failed to parse tiles");

        Ok(Self { tiles })
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut dish = input
        .parse::<ReflectorDish>()
        .expect("Failed to parse dish");
    dish.cycle(1_000_000_000);
    Ok(dish.calculate_load().to_string())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        "64"
    )]
    #[test_log::test]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
