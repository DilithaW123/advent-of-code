use std::{collections::HashMap, error::Error};

use regex::Regex;

fn retrieve_winners(line: &str) -> HashMap<&str, u32> {
    let mut winners: HashMap<&str, u32> = HashMap::new();
    let pipe_ind = line.find("|").expect("No pipe found");
    let line = &line[0..pipe_ind];
    let reg = Regex::new(r"(\d+)").unwrap();
    let mut captures = reg.captures_iter(&line);
    _ = captures.next().unwrap();
    for cap in captures {
        let s = cap.get(0).unwrap().as_str();
        winners.insert(s, 0);
    }
    winners
}

fn retrieve_points(line: &str, winners: &mut HashMap<&str, u32>) -> u32 {
    let pipe_ind = line.find("|").expect("No pipe found");
    let line = &line[pipe_ind + 1..];
    let reg = Regex::new(r"(\d+)").unwrap();
    let captures = reg.captures_iter(&line);
    let mut points = 0;
    for cap in captures {
        let s = cap.get(0).unwrap().as_str();
        if winners.contains_key(s) {
            println!("{}", s);
            if points == 0 {
                points += 1;
            } else {
                points = points * 2;
            }
        }
    }
    println!("{}", points);
    points
}

fn process(line: &str) -> Result<u32, Box<dyn Error>> {
    let mut points = 0;
    for line in line.lines() {
        let mut winners = retrieve_winners(line);
        points += retrieve_points(line, &mut winners);
    }
    Ok(points)
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
    fn test_check_winners() {
        let hashmap = retrieve_winners("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        println!("{}", hashmap.get("48").unwrap());
        assert!(hashmap.contains_key("41"));
    }

    #[test]
    fn test_process() {
        let text = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(13, process(text).unwrap());
    }
}
