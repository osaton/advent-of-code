use glam::IVec2;
use pathfinding::prelude::dijkstra;
use std::{collections::HashMap, str::FromStr};

use crate::custom_error::AocError;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug)]
struct City {
    grid: HashMap<IVec2, u32>,
    height: i32,
    width: i32,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Pos {
    position: IVec2,
    direction: Option<Direction>,
    steps_in_direction: u32,
}

impl Pos {
    fn new(x: i32, y: i32, direction: Option<Direction>, steps_in_direction: u32) -> Self {
        let position = IVec2::new(x, y);

        Self {
            position,
            direction,
            steps_in_direction,
        }
    }

    fn opposite_direction(&self, direction: Direction) -> bool {
        match self.direction {
            Some(Direction::Up) => direction == Direction::Down,
            Some(Direction::Down) => direction == Direction::Up,
            Some(Direction::Left) => direction == Direction::Right,
            Some(Direction::Right) => direction == Direction::Left,
            None => false,
        }
    }

    fn can_move(&self, direction: Direction) -> bool {
        let max_moves = 3;

        if self.opposite_direction(direction) {
            return false;
        }

        if self.direction == Some(direction) && self.steps_in_direction >= max_moves {
            return false;
        }

        if self.steps_in_direction < max_moves {
            return true;
        }

        // let next = self.position + *next;
        // let last = self.previous[self.previous.len() - 3];
        // let previous = self.previous[self.previous.len() - 2];
        // // Can't move back to previous position
        // if next == previous {
        //     return false;
        // }

        // if next.x == last.x && (next.y - last.y >= max_moves || next.y - last.y <= -max_moves) {
        //     return false;
        // }

        // if next.y == last.y && (next.x - last.x >= max_moves || next.x - last.x <= -max_moves) {
        //     return false;
        // }

        // if diff.x >= max_moves
        //     || diff.y >= max_moves
        //     || diff.x <= -max_moves
        //     || diff.y <= -max_moves
        // {
        //     return false;
        // }

        true
    }

    fn step(&self, direction: Direction) -> Self {
        let position = match direction {
            Direction::Up => self.position + IVec2::new(0, -1),
            Direction::Down => self.position + IVec2::new(0, 1),
            Direction::Left => self.position + IVec2::new(-1, 0),
            Direction::Right => self.position + IVec2::new(1, 0),
        };

        let steps_in_direction = if self.direction == Some(direction) {
            self.steps_in_direction + 1
        } else {
            1
        };

        Self {
            position,
            direction: Some(direction),
            steps_in_direction,
        }
    }
}
impl City {
    fn find_cheapest_path(&self) -> (Vec<Pos>, u32) {
        let start = IVec2::new(0, 0);
        let last_column = self.grid.keys().max_by_key(|k| k.x).unwrap().x;
        let last_row = self.grid.keys().max_by_key(|k| k.y).unwrap().y;
        let end = IVec2::new(last_column, last_row);
        let pos = Pos::new(start.x, start.y, None, 0);

        dijkstra(&pos, |p| self.neighbors(p), |p| p.position == end).expect("Failed to find path")
    }

    fn neighbors(&self, pos: &Pos) -> Vec<(Pos, u32)> {
        let mut neighbors = Vec::new();

        if pos.can_move(Direction::Up) {
            let up = pos.step(Direction::Up);

            if let Some(up_value) = self.grid.get(&up.position) {
                neighbors.push((up, *up_value));
            }
        }

        if pos.can_move(Direction::Down) {
            let down = pos.step(Direction::Down);

            if let Some(down_value) = self.grid.get(&down.position) {
                neighbors.push((down, *down_value));
            }
        }

        if pos.can_move(Direction::Left) {
            let left = pos.step(Direction::Left);

            if let Some(left_value) = self.grid.get(&left.position) {
                neighbors.push((left, *left_value));
            }
        }

        if pos.can_move(Direction::Right) {
            let right = pos.step(Direction::Right);
            if let Some(right_value) = self.grid.get(&right.position) {
                neighbors.push((right, *right_value));
            }
        }

        // let up = Pos::new(position.x, position.y - 1, pos.direction, pos.steps_in_direction);
        // let down = Pos::new(position.x, position.y + 1, &pos.previous);
        // let left = Pos::new(position.x - 1, position.y, &pos.previous);
        // let right = Pos::new(position.x + 1, position.y, &pos.previous);

        // if pos.can_move(&IVec2::new(0, -1)) {
        //     if let Some(up_value) = self.grid.get(&up.position) {
        //         neighbors.push((up, *up_value));
        //     }
        // }

        // if pos.can_move(&IVec2::new(0, 1)) {
        //     if let Some(down_value) = self.grid.get(&down.position) {
        //         neighbors.push((down, *down_value));
        //     }
        // }

        // if pos.can_move(&IVec2::new(-1, 0)) {
        //     if let Some(left_value) = self.grid.get(&left.position) {
        //         neighbors.push((left, *left_value));
        //     }
        // }

        // if pos.can_move(&IVec2::new(1, 0)) {
        //     if let Some(right_value) = self.grid.get(&right.position) {
        //         neighbors.push((right, *right_value));
        //     }
        // }

        neighbors
    }

    fn print_path(&self, path: &Vec<Pos>) {
        let mut grid = vec![vec!['.'; self.width as usize]; self.height as usize];
        for pos in path {
            grid[pos.position.y as usize][pos.position.x as usize] = '#';
        }

        let grid_str = grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile| tile.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n");

        println!("{}", grid_str);
    }
}

impl FromStr for City {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let position = IVec2::new(x as i32, y as i32);
                let value = c.to_digit(10).expect("Failed to parse digit");
                grid.insert(position, value);
            }
        }
        let width = grid.keys().max_by_key(|k| k.x).unwrap().x + 1;
        let height = grid.keys().max_by_key(|k| k.y).unwrap().y + 1;
        Ok(Self {
            grid,
            height,
            width,
        })
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let city = input.parse::<City>()?;
    let (_path, cost) = city.find_cheapest_path();

    Ok(cost.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!("102", process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn test_city_grid() -> miette::Result<()> {
        let input = "123
456
789";
        let city = input.parse::<City>()?;
        assert_eq!(city.grid.len(), 9);
        assert_eq!(city.grid[&IVec2::new(0, 0)], 1);
        assert_eq!(city.grid[&IVec2::new(1, 0)], 2);
        assert_eq!(city.grid[&IVec2::new(2, 0)], 3);
        assert_eq!(city.grid[&IVec2::new(0, 1)], 4);
        assert_eq!(city.grid[&IVec2::new(1, 1)], 5);
        assert_eq!(city.grid[&IVec2::new(2, 1)], 6);
        assert_eq!(city.grid[&IVec2::new(0, 2)], 7);
        assert_eq!(city.grid[&IVec2::new(1, 2)], 8);
        assert_eq!(city.grid[&IVec2::new(2, 2)], 9);
        Ok(())
    }

    #[rstest]
    #[case(
        "123
456
789",
        20
    )]
    #[case(
        "123
456
119",
        15
    )]
    #[case(
        "1111111
1119111",
        9
    )]
    #[case(
        "11111
22222
33333
44444
55555",
        19
    )]
    fn test_find_cheapest_path(#[case] input: &str, #[case] expected: u32) -> miette::Result<()> {
        let city = input.parse::<City>()?;

        let (path, cost) = city.find_cheapest_path();

        city.print_path(&path);
        assert_eq!(expected, cost);
        Ok(())
    }

    #[test_log::test]
    fn test_pos_can_move() -> miette::Result<()> {
        let pos = Pos::new(0, 2, None, 0);
        assert!(pos.can_move(Direction::Right));

        let pos = Pos::new(0, 3, Some(Direction::Right), 3);
        assert!(!pos.can_move(Direction::Right));
        assert!(!pos.can_move(Direction::Left));
        assert!(pos.can_move(Direction::Down));
        assert!(pos.can_move(Direction::Up));
        Ok(())
    }
}
