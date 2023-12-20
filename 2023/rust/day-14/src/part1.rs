use std::{collections::HashMap, str::FromStr};

use crate::custom_error::AocError;

use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString, Eq, PartialEq, Clone, Copy)]
enum Tile {
    #[strum(serialize = "O")]
    Rock,
    #[strum(serialize = "#")]
    CubeRock,
    #[strum(serialize = ".")]
    Empty,
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
    dish.tilt_north();
    Ok(dish.calculate_load().to_string())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        ".
.
O", "3"
    )]
    #[case(
        "..
##
OO", "2"
    )]
    #[case(
        "..
OO
##
..
OO",
        "14"
    )]
    #[case(
        "..
.O
..
OO
OO",
        "21"
    )]
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
        "136"
    )]
    #[test_log::test]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
