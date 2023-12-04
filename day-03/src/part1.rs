use std::{collections::HashSet, error::Error};

use regex::Regex;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    char_ind: u32,
    line: u32,
}

fn first_pass(input: &str) -> HashSet<Point> {
    let mut symbol_set: HashSet<Point> = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '.' => {
                    // Insert the default symbol (e.g., "default_value") into the hashmap
                }
                _ => {
                    let point = Point {
                        char_ind: x as u32,
                        line: i as u32,
                    };
                    symbol_set.insert(point);
                }
            };
        }
    }
    return symbol_set;
}

fn parse_line(input: &str, symbol_set: &HashSet<Point>) -> u32 {
    let mut count = 0;
    let regex_pattern = r"(\d+)";
    let reg = Regex::new(regex_pattern).expect("Expect regex to build pattern");
    for (i, line) in input.lines().enumerate() {
        'inner: for capture in reg.captures_iter(line) {
            let mat = capture.get(0).expect("Expect at least one match");
            let val = mat.as_str().parse::<u32>().expect("Expect to parse to u32");
            let start = mat.start() as u32;
            let end = start + mat.len() as u32;
            // check before start
            if start > 0
                && symbol_set.contains(&Point {
                    char_ind: start - 1,
                    line: i as u32,
                })
            {
                count += val;
                continue 'inner;
            }
            // check after end
            if end < (line.len() as u32)
                && symbol_set.contains(&Point {
                    char_ind: start + mat.len() as u32,
                    line: i as u32,
                })
            {
                count += val;
                continue 'inner;
            }
            // check top left
            if start > 0 && i > 0 {
                if symbol_set.contains(&Point {
                    char_ind: start - 1,
                    line: i as u32 - 1,
                }) {
                    count += val;
                    continue 'inner;
                }
            }
            // check bottom left
            if start > 0 && i < input.lines().count() {
                if symbol_set.contains(&Point {
                    char_ind: start - 1,
                    line: i as u32 + 1,
                }) {
                    count += val;
                    continue 'inner;
                }
            }
            // check top right
            if end < (line.len() as u32) && i > 0 {
                if symbol_set.contains(&Point {
                    char_ind: start + mat.len() as u32,
                    line: i as u32 - 1,
                }) {
                    count += val;
                    continue 'inner;
                }
            }
            // check bottom right
            if end < (line.len() as u32) && i < input.lines().count() {
                if symbol_set.contains(&Point {
                    char_ind: start + mat.len() as u32,
                    line: i as u32 + 1,
                }) {
                    count += val;
                    continue 'inner;
                }
            }
            // check above
            if i > 0 {
                let mut start_pos = start;
                while start_pos < end as u32 {
                    if symbol_set.contains(&Point {
                        char_ind: start_pos,
                        line: i as u32 - 1,
                    }) {
                        count += val;
                        continue 'inner;
                    }
                    start_pos += 1;
                }
            }
            //check below
            if i < input.lines().count() {
                let mut start_pos = start;
                while start_pos < end as u32 {
                    if symbol_set.contains(&Point {
                        char_ind: start_pos,
                        line: i as u32 + 1,
                    }) {
                        count += val;
                        continue 'inner;
                    }
                    start_pos += 1;
                }
            }
        }
    }
    return count;
}

fn process(input: &str) -> Result<u32, Box<dyn Error>> {
    let symbol_map = first_pass(input);
    let output = parse_line(input, &symbol_map);
    Ok(output)
}

fn main() {
    let text = include_str!("../input1.txt");
    match process(text) {
        Ok(num) => println!("{}", num),
        Err(err) => println!("{}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_symbol_set() {
        let symbol_set = first_pass(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert!(symbol_set.contains(&Point {
            char_ind: 3,
            line: 1
        }));
    }

    #[test]
    fn test_symbol_set_bad() {
        let symbol_set = first_pass(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert!(!symbol_set.contains(&Point {
            char_ind: 0,
            line: 1
        }));
    }
}
