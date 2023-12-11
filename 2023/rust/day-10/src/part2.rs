use core::panic;
use std::str::FromStr;

use crate::custom_error::AocError;
use strum::{Display, EnumString};

use glam::IVec2;

#[derive(Debug, Display, EnumString, Eq, PartialEq, Clone)]
enum Tile {
    #[strum(serialize = ".")]
    Floor(IVec2),

    #[strum(serialize = "|")]
    VerticalPipe(IVec2),

    #[strum(serialize = "-")]
    HorizontalPipe(IVec2),

    #[strum(serialize = "L")]
    NorthEastPipe(IVec2),

    #[strum(serialize = "J")]
    NorthWestPipe(IVec2),

    #[strum(serialize = "7")]
    SouthWestPipe(IVec2),

    #[strum(serialize = "F")]
    SouthEastPipe(IVec2),

    #[strum(serialize = "S")]
    Start(IVec2),
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
struct Polygon {
    vertices: Vec<IVec2>,
}

impl Polygon {
    // This implementation only counts points inside the polygon, not on the edges
    fn contains(&self, point: &IVec2) -> bool {
        let mut count = 0;
        let mut j = self.vertices.len() - 1;

        for i in 0..self.vertices.len() {
            let vertex1 = &self.vertices[i];
            let vertex2 = &self.vertices[j];

            // Check if the point is exactly on a vertex
            if point == vertex1 || point == vertex2 {
                return false;
            }

            if (vertex1.y > point.y) != (vertex2.y > point.y)
                && (point.x
                    < (vertex2.x - vertex1.x) * (point.y - vertex1.y) / (vertex2.y - vertex1.y)
                        + vertex1.x)
            {
                count += 1;
            }

            j = i;
        }

        count % 2 != 0
    }
}

#[derive(Debug)]
struct Player {
    position: IVec2,
    prev_position: Option<IVec2>,
    direction: Option<Direction>,
    steps: usize,
    trail: Vec<IVec2>,
}

impl Player {
    fn new(position: IVec2) -> Self {
        Self {
            position,
            prev_position: None,
            steps: 0,
            direction: None,
            trail: vec![position],
        }
    }

    fn travel(&mut self, map: &Map) {
        loop {
            self.advance(map);
            if self.has_completed() {
                break;
            }
        }
    }

    fn has_completed(&self) -> bool {
        self.trail.first().expect("No trail found") == &self.position && self.steps > 0
    }
    fn get_next_position(&self, direction: &Direction) -> IVec2 {
        match direction {
            Direction::North => self.position - IVec2::new(0, 1),
            Direction::South => self.position + IVec2::new(0, 1),
            Direction::East => self.position + IVec2::new(1, 0),
            Direction::West => self.position - IVec2::new(1, 0),
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
        let new_position = self.get_next_position(direction);
        let tile = map.get_tile(&new_position).expect("No tile found");
        let from_dir = self.get_from_dir(direction);

        self.prev_position = Some(self.position);
        self.position = new_position;
        self.direction = map.get_exit_dir(tile, &from_dir);
        self.steps += 1;
        self.trail.push(self.position)
    }

    fn get_possible_directions(&self, map: &Map) -> Vec<Direction> {
        let mut directions = vec![];

        let mut possible_directions = Vec::new();

        if self.position.x > 0 {
            directions.push(Direction::West);
        }
        if self.position.x < map.width as i32 - 1 {
            directions.push(Direction::East);
        }
        if self.position.y > 0 {
            directions.push(Direction::North);
        }
        if self.position.y < map.height as i32 - 1 {
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
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    starting_position: IVec2,
}

impl Map {
    fn get_tile(&self, position: &IVec2) -> Option<&Tile> {
        if position.x >= self.width as i32 || position.y >= self.height as i32 {
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
        let mut starting_position: Option<IVec2> = None;
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
                                pos.x = x as i32;
                                pos.y = y as i32;
                            }
                            Tile::Start(pos) => {
                                pos.x = x as i32;
                                pos.y = y as i32;
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
fn get_trail(map: &Map) -> Vec<IVec2> {
    let mut player = Player::new(map.starting_position);
    let starting_directions = player.get_possible_directions(map);
    let first_direction = starting_directions
        .first()
        .expect("No starting direction found");

    player.set_direction(*first_direction);
    player.travel(map);
    player.trail
}

fn get_inner_tile_count(map: &Map, trail: &[IVec2]) -> usize {
    let polygon = Polygon {
        vertices: trail.to_owned(),
    };

    // Loop throug all x and y tiles

    let tiles = map
        .tiles
        .iter()
        .enumerate()
        .map(|(y, row)| {
            let count = row
                .iter()
                .enumerate()
                .map(|(x, _)| IVec2::new(x as i32, y as i32))
                .filter(|pos| polygon.contains(pos))
                .count();

            count
        })
        .sum();

    tiles
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let map = Map::from_str(input)?;
    let trail = get_trail(&map);

    Ok(get_inner_tile_count(&map, &trail).to_string())
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[rstest]
    #[case(
        "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        "4"
    )]
    #[case(
        ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        "8"
    )]
    #[case(
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        "10"
    )]
    #[test_log::test]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
