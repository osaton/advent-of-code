use nom::character::complete::u32;

use crate::custom_error::AocError;

#[derive(Debug)]
struct CubeGame {
    id: u32,
    red: u32,
    blue: u32,
    green: u32,
}

impl CubeGame {
    fn new(id: u32) -> Self {
        Self {
            id,
            red: 0,
            blue: 0,
            green: 0,
        }
    }

    fn set_red(&mut self, red: u32) {
        if self.red > red {
            return;
        }
        self.red = red;
    }

    fn set_blue(&mut self, blue: u32) {
        if self.blue > blue {
            return;
        }
        self.blue = blue;
    }

    fn set_green(&mut self, green: u32) {
        if self.green > green {
            return;
        }
        self.green = green;
    }

    fn can_play(&self, max_red: u32, max_blue: u32, max_green: u32) -> bool {
        self.red <= max_red && self.blue <= max_blue && self.green <= max_green
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let result: u32 = _input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(": ");
            let game = parts.next().unwrap();
            let id = game.split(' ').last().unwrap().parse::<u32>().unwrap();

            let stats = parts.next().unwrap();
            let draws = stats.split(';').collect::<Vec<&str>>();

            let mut cube = CubeGame::new(id);
            for draw in draws {
                let parts = draw.trim().split(", ").collect::<Vec<&str>>();

                for part in parts {
                    let mut color_count = part.split(' ');
                    let count = color_count.next().unwrap().parse::<u32>().unwrap();
                    let color = color_count.next().unwrap();

                    if color == "red" {
                        cube.set_red(count);
                    } else if color == "blue" {
                        cube.set_blue(count);
                    } else if color == "green" {
                        cube.set_green(count);
                    }
                }
            }

            if cube.can_play(12, 14, 13) {
                Some(cube.id)
            } else {
                None
            }
        })
        .sum::<u32>();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
