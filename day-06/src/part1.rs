fn parse_games(input: &str) -> (Vec<u32>, Vec<u32>) {
    let lines = input.lines().collect::<Vec<_>>();
    let time_line = lines[0].strip_prefix("Time: ").unwrap().trim();
    let distance_line = lines[1].strip_prefix("Distance: ").unwrap().trim();
    let times: Vec<u32> = time_line
        .split_ascii_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let distances: Vec<u32> = distance_line
        .split_ascii_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    return (times, distances);
}

fn process(input: &str) -> u32 {
    let (times, distances) = parse_games(input);
    let mut time_vecs: Vec<u32> = Vec::new();
    for (time, distance) in times.iter().zip(distances.iter()) {
        let mut range = 0;
        for hold_time in 0..*time {
            let dist = hold_time * (time - hold_time);
            if dist > *distance {
                range += 1;
            }
        }
        time_vecs.push(range);
    }
    return time_vecs.iter().product();
}

fn main() {
    let text = include_str!("../input1.txt");
    let result = process(text);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_games() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let (times, distances) = parse_games(input);
        assert_eq!(times, vec![7, 15, 30]);
        assert_eq!(distances, vec![9, 40, 200]);
    }

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = process(input);
        assert_eq!(288, result);
    }
}
