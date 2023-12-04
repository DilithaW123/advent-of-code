use std::{collections::HashMap, error::Error};

use regex::Regex;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    char_ind: u32,
    line: u32,
}

fn first_pass(input: &str) -> HashMap<Point, Vec<u32>> {
    let mut symbol_set: HashMap<Point, Vec<u32>> = HashMap::new();
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
                    symbol_set.insert(point, vec![]);
                }
            };
        }
    }
    return symbol_set;
}

fn parse_line(input: &str, symbol_set: &mut HashMap<Point, Vec<u32>>) -> u32 {
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
            if start > 0 {
                let left = Point {
                    char_ind: start - 1,
                    line: i as u32,
                };
                if symbol_set.contains_key(&left) {
                    symbol_set.get_mut(&left).unwrap().push(val);
                }
            }

            let end_point = Point {
                char_ind: end,
                line: i as u32,
            };
            if end < (line.len() as u32) && symbol_set.contains_key(&end_point) {
                symbol_set.get_mut(&end_point).unwrap().push(val);
            }

            if start > 0 && i > 0 {
                let top_left = Point {
                    char_ind: start - 1,
                    line: i as u32 - 1,
                };
                if symbol_set.contains_key(&top_left) {
                    symbol_set.get_mut(&top_left).unwrap().push(val);
                }
            }

            if start > 0 && i < input.lines().count() {
                let bot_left = Point {
                    char_ind: start - 1,
                    line: i as u32 + 1,
                };
                if symbol_set.contains_key(&bot_left) {
                    symbol_set.get_mut(&bot_left).unwrap().push(val);
                }
            }

            if end < (line.len() as u32) && i > 0 {
                let top_right = Point {
                    char_ind: start + mat.len() as u32,
                    line: i as u32 - 1,
                };
                if symbol_set.contains_key(&top_right) {
                    symbol_set.get_mut(&top_right).unwrap().push(val);
                }
            }

            let bot_right = Point {
                char_ind: start + mat.len() as u32,
                line: i as u32 + 1,
            };
            if end < (line.len() as u32) && i < input.lines().count() {
                if symbol_set.contains_key(&bot_right) {
                    symbol_set.get_mut(&bot_right).unwrap().push(val);
                }
            }

            // check above
            if i > 0 {
                let mut start_pos = start;
                while start_pos < end as u32 {
                    let above_point = Point {
                        char_ind: start_pos,
                        line: i as u32 - 1,
                    };
                    if symbol_set.contains_key(&above_point) {
                        symbol_set.get_mut(&above_point).unwrap().push(val);
                    }
                    start_pos += 1;
                }
            }
            //check below
            if i < input.lines().count() {
                let mut start_pos = start;
                while start_pos < end as u32 {
                    let below_point = Point {
                        char_ind: start_pos,
                        line: i as u32 + 1,
                    };
                    if symbol_set.contains_key(&below_point) {
                        symbol_set.get_mut(&below_point).unwrap().push(val);
                    }
                    start_pos += 1;
                }
            }
        }
    }
    for (point, vec) in symbol_set.iter() {
        if vec.len() == 2 {
            count += vec[0] * vec[1];
        }
    }
    return count;
}

fn process(input: &str) -> Result<u32, Box<dyn Error>> {
    let mut symbol_map = first_pass(input);
    let output = parse_line(input, &mut symbol_map);
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
}
