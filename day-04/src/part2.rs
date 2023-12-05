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
            points += 1;
        }
    }
    points
}

fn process(line: &str) -> Result<u32, Box<dyn Error>> {
    let mut points = 0;
    let mut card_map: HashMap<u32, u32> = HashMap::new();
    for (i, line) in line.lines().enumerate() {
        let card_num = i as u32 + 1;
        let mut winners = retrieve_winners(line);
        let cards_won = retrieve_points(line, &mut winners);
        let multiplier: u32;
        // if doesn't exist multiplier is 1
        // if exists, multiplier is value + 1
        match card_map.get_mut(&card_num) {
            Some(v) => {
                *v += 1;
                multiplier = *v;
            }
            None => {
                multiplier = 1;
                card_map.insert(card_num, 1);
            }
        }
        if cards_won == 0 {
            continue;
        }
        for j in 1..cards_won + 1 {
            let new_card_num = j + card_num as u32;
            match card_map.get_mut(&new_card_num) {
                Some(v) => {
                    *v += multiplier;
                }
                None => {
                    card_map.insert(new_card_num, multiplier);
                }
            }
        }
    }
    for (_, v) in card_map {
        points += v;
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
        assert_eq!(30, process(text).unwrap());
    }
}
