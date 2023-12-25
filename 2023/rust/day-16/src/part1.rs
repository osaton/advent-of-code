use crate::custom_error::AocError;
use glam::IVec2;
use std::{str::FromStr, string::ToString};
use strum::{Display, EnumString, EnumVariantNames, IntoStaticStr};

#[derive(Display, Debug, EnumString, EnumVariantNames, IntoStaticStr, Clone, Copy)]
enum Tile {
    #[strum(serialize = ".")]
    Empty(IVec2),
    #[strum(serialize = "-")]
    HorizontalSplitter(IVec2),
    #[strum(serialize = "|")]
    VerticalSplitter(IVec2),
    #[strum(serialize = "/")]
    ForwardSlashMirror(IVec2),
    #[strum(serialize = r"\")]
    BackSlashMirror(IVec2),
    #[strum(serialize = "#")]
    Visited(IVec2),
}

#[derive(Debug)]
struct Contraption {
    grid: Vec<Vec<Tile>>,
}

impl Contraption {
    fn gather_beam_path(&self, start: IVec2, direction: IVec2) -> Vec<IVec2> {
        let mut path = Vec::new();
        let mut stack = vec![(start, direction)];
        let mut visited = std::collections::HashSet::new();

        while let Some((current, direction)) = stack.pop() {
            if !visited.insert((current, direction)) {
                continue;
            }

            let mut next = current + direction;
            while next.x >= 0
                && next.y >= 0
                && next.x < self.grid[0].len() as i32
                && next.y < self.grid.len() as i32
            {
                path.push(next);

                match self.tile(next) {
                    Some(Tile::Empty(_)) => {
                        next += direction;
                    }
                    Some(Tile::HorizontalSplitter(_)) => {
                        if direction.x == 0 {
                            stack.push((next, IVec2::new(-1, 0)));
                            stack.push((next, IVec2::new(1, 0)));
                            break;
                        } else {
                            next += direction;
                        }
                    }
                    Some(Tile::VerticalSplitter(_)) => {
                        if direction.y == 0 {
                            stack.push((next, IVec2::new(0, -1)));
                            stack.push((next, IVec2::new(0, 1)));
                            break;
                        } else {
                            next += direction;
                        }
                    }
                    Some(Tile::ForwardSlashMirror(_)) => {
                        let new_direction = match direction {
                            IVec2 { x: 1, y: 0 } => IVec2::new(0, -1), // Right to Up
                            IVec2 { x: -1, y: 0 } => IVec2::new(0, 1), // Left to Down
                            IVec2 { x: 0, y: 1 } => IVec2::new(-1, 0), // Down to Left
                            IVec2 { x: 0, y: -1 } => IVec2::new(1, 0), // Up to Right
                            _ => direction,
                        };
                        stack.push((next, new_direction));
                        break;
                    }
                    Some(Tile::BackSlashMirror(_)) => {
                        let new_direction = match direction {
                            IVec2 { x: 1, y: 0 } => IVec2::new(0, 1),   // Right to Down
                            IVec2 { x: -1, y: 0 } => IVec2::new(0, -1), // Left to Up
                            IVec2 { x: 0, y: 1 } => IVec2::new(1, 0),   // Down to Right
                            IVec2 { x: 0, y: -1 } => IVec2::new(-1, 0), // Up to Left
                            _ => direction,
                        };
                        stack.push((next, new_direction));
                        break;
                    }

                    None => break,
                    _ => break,
                }
            }
        }
        path
    }

    fn count_positions(&self, path: &[IVec2]) -> usize {
        let mut positions = Vec::new();
        for position in path {
            if !positions.contains(position) {
                positions.push(*position);
            }
        }
        positions.len()
    }

    #[allow(dead_code)]
    fn draw_path(&self, path: &[IVec2]) -> String {
        let mut grid = self.grid.clone();
        for position in path {
            grid[position.y as usize][position.x as usize] = Tile::Visited(*position);
        }

        grid.iter()
            .map(|row| {
                row.iter()
                    .map(|tile| tile.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn tile(&self, position: IVec2) -> Option<&Tile> {
        self.grid
            .get(position.y as usize)
            .and_then(|row| row.get(position.x as usize))
    }
}
impl FromStr for Contraption {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let tile = c.to_string().parse::<Tile>().expect("Failed to parse tile");

                        match tile {
                            Tile::Empty(_) => Tile::Empty(IVec2::new(x as i32, y as i32)),
                            Tile::HorizontalSplitter(_) => {
                                Tile::HorizontalSplitter(IVec2::new(x as i32, y as i32))
                            }
                            Tile::VerticalSplitter(_) => {
                                Tile::VerticalSplitter(IVec2::new(x as i32, y as i32))
                            }
                            Tile::ForwardSlashMirror(_) => {
                                Tile::ForwardSlashMirror(IVec2::new(x as i32, y as i32))
                            }
                            Tile::BackSlashMirror(_) => {
                                Tile::BackSlashMirror(IVec2::new(x as i32, y as i32))
                            }
                            _ => tile,
                        }
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect::<Vec<Vec<Tile>>>();

        Ok(Self { grid })
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let contraption = input.parse::<Contraption>()?;
    let path = contraption.gather_beam_path(IVec2::new(-1, 0), IVec2::new(1, 0));
    let positions = contraption.count_positions(&path);
    //println!("{}", &contraption.draw_path(&path));
    Ok(positions.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!("46", process(input)?);
        Ok(())
    }
}
