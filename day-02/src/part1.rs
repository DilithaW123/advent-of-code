use std::error::Error;

use regex::Regex;

#[derive(Debug)]
struct Cube {
    red_count: u32,
    green_count: u32,
    blue_count: u32,
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.red_count == other.red_count
            && self.blue_count == other.blue_count
            && self.green_count == other.green_count
    }
}

fn parse_game(text: &str) -> Cube {
    let regex_pattern = r"(?:(\d+) (green|red|blue))";
    let find_reg = Regex::new(regex_pattern).expect("Expected regex to be built");
    let mut cube_count = Cube {
        red_count: 0,
        blue_count: 0,
        green_count: 0,
    };
    for found_match in find_reg.find_iter(text) {
        let string_split: Vec<&str> = found_match.as_str().split(' ').collect();
        let count = string_split
            .get(0)
            .expect("Expect split to work")
            .parse::<u32>()
            .expect("Expected number to parse");
        let color = string_split.get(1).expect("Expect a color").to_owned();
        match color {
            "green" => {
                if count > cube_count.green_count {
                    cube_count.green_count = count
                }
            }
            "red" => {
                if count > cube_count.red_count {
                    cube_count.red_count = count
                }
            }
            "blue" => {
                if count > cube_count.blue_count {
                    cube_count.blue_count = count
                }
            }
            _ => panic!("Unknown str"),
        }
    }

    cube_count
}

fn process(text: &str, red_max: u32, green_max: u32, blue_max: u32) -> Result<u32, Box<dyn Error>> {
    let mut output: u32 = 0;
    for (i, line) in text.lines().enumerate() {
        let result_cube = parse_game(line);
        if result_cube.blue_count <= blue_max
            && result_cube.green_count <= green_max
            && result_cube.red_count <= red_max
        {
            output += (i + 1) as u32
        }
    }
    Ok(output)
}

fn main() {
    let text = include_str!("../input1.txt");
    match process(text, 12, 13, 14) {
        Ok(num) => println!("{}", num),
        Err(err) => println!("{}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            8,
            process(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
                12,
                13,
                14,
            )?
        );
        Ok(())
    }

    #[test]
    fn test_parse_game() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            Cube {
                green_count: 13,
                blue_count: 6,
                red_count: 20
            },
            parse_game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")
        );
        Ok(())
    }
}
