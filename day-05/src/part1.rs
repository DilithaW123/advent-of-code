use std::collections::HashSet;

fn parse_seeds(line: &str) -> HashSet<u64> {
    let seeds: HashSet<u64> = line
        .split(" ")
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();
    return seeds;
}

fn parse_map(input: &str, validation_set: &HashSet<u64>) -> HashSet<u64> {
    let mut line_iter = input.lines();
    //skip header line
    line_iter.next();
    let mut result_set: HashSet<u64> = HashSet::new();
    let mut seen: HashSet<u64> = HashSet::new();
    let max = validation_set.iter().max().unwrap();
    line_iter.enumerate().for_each(|(_, line)| {
        let nums: Vec<u64> = line
            .split(" ")
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();
        if nums.len() != 3 {
            panic!("Invalid line: {}", line);
        }
        let (a, b, c) = (nums[0], nums[1], nums[2]);
        if a > *max {
            return;
        }
        for val in validation_set.iter() {
            // if value is in range of b and b + c then val - b + a will give the exact match
            // within the range of a and a + c
            if b <= *val && *val <= b + c {
                result_set.insert(val - b + a);
                seen.insert(*val);
            }
        }
    });
    validation_set.difference(&seen).for_each(|x| {
        result_set.insert(*x);
    });

    return result_set;
}

fn process(input: &str) -> u64 {
    let seeds = parse_seeds(input.lines().next().unwrap());
    let chunks = input.split("\n\n");
    let mut validation_set = seeds;
    for chunk in chunks {
        let result_set = parse_map(chunk, &validation_set);
        validation_set = result_set.clone();
    }
    // get min of validation_set
    let min = validation_set.iter().min().unwrap();
    return *min;
}

fn main() {
    println!("Hello, world!");
    let text = include_str!("../input1.txt");
    let val = process(text);
    println!("{}", val);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seeds() {
        let line = "seeds: 79 14 55 13";
        let seeds = parse_seeds(line);
        assert_eq!(seeds.len(), 4);
    }

    #[test]
    fn test_parse_seeds1() {
        let line = "seeds: 79 14 55 13";
        let seeds = parse_seeds(line);
        assert_eq!(seeds.contains(&79), true);
    }

    #[test]
    fn test_parse_map1() {
        let line = "seeds: 79 14 55 13";
        let seeds = parse_seeds(line);
        let result_map = parse_map(
            "seed-to-soil map:
50 98 2
52 50 48",
            &seeds,
        );
        println!("{:?}", result_map);
        assert!(result_map.contains(&57));
    }

    #[test]
    fn test_parse_map2() {
        let line = "seeds: 79 14 55 13";
        let seeds = parse_seeds(line);
        let result_map = parse_map(
            "seed-to-soil map:
50 98 2
52 50 48",
            &seeds,
        );
        println!("{:?}", result_map);
        assert_eq!(result_map.len(), 4);
    }

    #[test]
    fn test_process() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let val = process(input);
        assert_eq!(val, 35);
    }
}
