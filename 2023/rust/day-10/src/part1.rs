use core::panic;
use std::str::FromStr;

use crate::custom_error::AocError;
use strum::{Display, EnumString};

use glam::UVec2;

#[derive(Debug, Display, EnumString, Eq, PartialEq, Clone)]
enum Tile {
    #[strum(serialize = ".")]
    Floor(UVec2),

    #[strum(serialize = "|")]
    VerticalPipe(UVec2),

    #[strum(serialize = "-")]
    HorizontalPipe(UVec2),

    #[strum(serialize = "L")]
    NorthEastPipe(UVec2),

    #[strum(serialize = "J")]
    NorthWestPipe(UVec2),

    #[strum(serialize = "7")]
    SouthWestPipe(UVec2),

    #[strum(serialize = "F")]
    SouthEastPipe(UVec2),

    #[strum(serialize = "S")]
    Start(UVec2),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
    Any,
}

#[derive(Debug)]
struct Player {
    position: UVec2,
    prev_position: Option<UVec2>,
    direction: Option<Direction>,
    id: usize,
    steps: usize,
}

impl Player {
    fn new(position: UVec2, id: usize) -> Self {
        Self {
            position,
            prev_position: None,
            id,
            steps: 0,
            direction: None,
        }
    }

    fn is_at_start(&self, map: &Map) -> bool {
        let tile = map.get_tile(&self.position).expect("No tile found");
        matches!(tile, Tile::Start(_))
    }
    fn get_next_position(&self, direction: &Direction) -> UVec2 {
        match direction {
            Direction::North => self.position - UVec2::new(0, 1),
            Direction::South => self.position + UVec2::new(0, 1),
            Direction::East => self.position + UVec2::new(1, 0),
            Direction::West => self.position - UVec2::new(1, 0),
            _ => panic!("Invalid direction"),
        }
    }

    fn get_from_dir(&self, direction: &Direction) -> Direction {
        match direction {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            _ => panic!("Invalid direction"),
        }
    }

    fn set_direction(&mut self, direction: Direction) {
        self.direction = Some(direction);
    }

    fn advance(&mut self, map: &Map) {
        if let Some(direction) = &self.direction.as_mut().cloned() {
            self.move_player(direction, map);
        } else {
            panic!("No direction set");
        }
    }

    fn move_player(&mut self, direction: &Direction, map: &Map) {
        // Implement somehow: If we're on a tile, E.g. NorthEastPipe, we can only exit from North or East
        self.prev_position = Some(self.position);

        let new_position = self.get_next_position(direction);
        self.position = new_position;

        let tile = map.get_tile(&self.position).expect("No tile found");

        let from_dir = self.get_from_dir(direction);

        self.direction = map.get_exit_dir(tile, &from_dir);
        self.steps += 1;
    }

    fn get_possible_directions(&self, map: &Map) -> Vec<Direction> {
        let mut directions = vec![];

        let mut possible_directions = Vec::new();

        if self.position.x > 0 {
            directions.push(Direction::West);
        }
        if self.position.x < map.width as u32 - 1 {
            directions.push(Direction::East);
        }
        if self.position.y > 0 {
            directions.push(Direction::North);
        }
        if self.position.y < map.height as u32 - 1 {
            directions.push(Direction::South);
        }

        for direction in directions {
            if self.can_move(&direction, map) {
                possible_directions.push(direction);
            }
        }
        possible_directions
    }

    fn can_move(&self, direction: &Direction, map: &Map) -> bool {
        let tile = map.get_tile(&self.position).expect("No tile found");
        let position = self.get_next_position(direction);
        let next_tile = map.get_tile(&position).expect("No tile found");

        let from_dir = self.get_from_dir(direction);

        // Do we have an exit facing this direction
        let can_exit = map.is_entry_point(tile, direction);
        if !can_exit {
            return false;
        }

        // Can't move backwards
        if self.prev_position == Some(position) {
            return false;
        }

        // Can we enter the next tile from this direction
        map.is_entry_point(next_tile, &from_dir)
    }

    fn get_farthest_distance(&self) -> usize {
        self.steps / 2
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    starting_position: UVec2,
}

impl Map {
    fn get_tile(&self, position: &UVec2) -> Option<&Tile> {
        if position.x >= self.width as u32 || position.y >= self.height as u32 {
            return None;
        }
        self.tiles
            .get(position.y as usize)
            .and_then(|row| row.get(position.x as usize))
    }

    fn get_exit_dir(&self, tile: &Tile, from: &Direction) -> Option<Direction> {
        match tile {
            Tile::Floor(_) => None,
            Tile::VerticalPipe(_) => match from {
                Direction::North => Some(Direction::South),
                Direction::South => Some(Direction::North),
                _ => None,
            },
            Tile::HorizontalPipe(_) => match from {
                Direction::East => Some(Direction::West),
                Direction::West => Some(Direction::East),
                _ => None,
            },
            Tile::NorthEastPipe(_) => match from {
                Direction::North => Some(Direction::East),
                Direction::East => Some(Direction::North),
                _ => None,
            },
            Tile::NorthWestPipe(_) => match from {
                Direction::North => Some(Direction::West),
                Direction::West => Some(Direction::North),
                _ => None,
            },
            Tile::SouthWestPipe(_) => match from {
                Direction::South => Some(Direction::West),
                Direction::West => Some(Direction::South),
                _ => None,
            },
            Tile::SouthEastPipe(_) => match from {
                Direction::South => Some(Direction::East),
                Direction::East => Some(Direction::South),
                _ => None,
            },
            Tile::Start(_) => Some(Direction::Any),
        }
    }

    fn is_entry_point(&self, next: &Tile, from: &Direction) -> bool {
        match next {
            Tile::Floor(_) => false,
            Tile::VerticalPipe(_) => matches!(from, Direction::North | Direction::South),
            Tile::HorizontalPipe(_) => matches!(from, Direction::East | Direction::West),
            Tile::NorthEastPipe(_) => matches!(from, Direction::North | Direction::East),
            Tile::NorthWestPipe(_) => matches!(from, Direction::North | Direction::West),
            Tile::SouthWestPipe(_) => matches!(from, Direction::South | Direction::West),
            Tile::SouthEastPipe(_) => matches!(from, Direction::South | Direction::East),
            Tile::Start(_) => true,
        }
    }
}

impl FromStr for Map {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut starting_position: Option<UVec2> = None;
        let tiles = s
            .split('\n')
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let mut tile = c.to_string().parse::<Tile>().expect("Failed to parse tile");
                        match &mut tile {
                            Tile::Floor(pos)
                            | Tile::VerticalPipe(pos)
                            | Tile::HorizontalPipe(pos)
                            | Tile::NorthEastPipe(pos)
                            | Tile::NorthWestPipe(pos)
                            | Tile::SouthWestPipe(pos)
                            | Tile::SouthEastPipe(pos) => {
                                pos.x = x as u32;
                                pos.y = y as u32;
                            }
                            Tile::Start(pos) => {
                                pos.x = x as u32;
                                pos.y = y as u32;
                                starting_position = Some(*pos);
                            }
                        }
                        tile
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect::<Vec<Vec<Tile>>>();

        let width = tiles[0].len();
        let height = tiles.len();

        Ok(Self {
            tiles,
            width,
            height,
            starting_position: starting_position.expect("No starting position found"),
        })
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let map = Map::from_str(input)?;
    let mut player = Player::new(map.starting_position, 1);

    let starting_directions = player.get_possible_directions(&map);

    let first_direction = starting_directions
        .first()
        .expect("No starting direction found");

    player.set_direction(*first_direction);

    let steps = loop {
        player.advance(&map);
        if player.is_at_start(&map) {
            break player.get_farthest_distance();
        }
    };

    Ok(steps.to_string())
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[rstest]
    #[case(
        "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        "4"
    )]
    #[case(
        "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        "8"
    )]
    #[test_log::test]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn test_moving() -> miette::Result<()> {
        let map = Map::from_str(
            "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        )?;

        let mut player = Player::new(map.starting_position, 1);
        assert!(!player.can_move(&Direction::West, &map));
        assert!(!player.can_move(&Direction::North, &map));
        assert!(player.can_move(&Direction::East, &map));
        assert!(player.can_move(&Direction::South, &map));
        player.move_player(&Direction::South, &map);
        assert!(player.can_move(&Direction::South, &map));
        assert!(!player.can_move(&Direction::East, &map));
        assert!(!player.can_move(&Direction::West, &map));
        assert!(!player.can_move(&Direction::North, &map));
        player.move_player(&Direction::South, &map);
        assert!(player.can_move(&Direction::East, &map));
        assert!(!player.can_move(&Direction::South, &map));
        assert!(!player.can_move(&Direction::North, &map));
        assert!(!player.can_move(&Direction::West, &map));
        player.move_player(&Direction::East, &map);
        assert!(player.can_move(&Direction::East, &map));
        assert!(!player.can_move(&Direction::South, &map));
        assert!(!player.can_move(&Direction::North, &map));
        assert!(!player.can_move(&Direction::West, &map));
        player.move_player(&Direction::East, &map);
        assert!(!player.can_move(&Direction::East, &map));
        assert!(!player.can_move(&Direction::South, &map));
        assert!(player.can_move(&Direction::North, &map));
        assert!(!player.can_move(&Direction::West, &map));
        player.move_player(&Direction::North, &map);
        assert!(!player.can_move(&Direction::East, &map));
        assert!(!player.can_move(&Direction::South, &map));
        assert!(player.can_move(&Direction::North, &map));
        assert!(!player.can_move(&Direction::West, &map));
        player.move_player(&Direction::North, &map);
        assert!(!player.can_move(&Direction::East, &map));
        assert!(!player.can_move(&Direction::South, &map));
        assert!(!player.can_move(&Direction::North, &map));
        assert!(player.can_move(&Direction::West, &map));
        player.move_player(&Direction::West, &map);
        assert!(!player.can_move(&Direction::East, &map));
        assert!(!player.can_move(&Direction::South, &map));
        assert!(!player.can_move(&Direction::North, &map));
        assert!(player.can_move(&Direction::West, &map));
        player.move_player(&Direction::West, &map);
        assert!(player.is_at_start(&map));

        Ok(())
    }
}
