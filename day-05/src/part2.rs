use std::{collections::HashSet, ops::Range};

// INCOMPLETE

fn parse_seeds(line: &str) -> HashSet<Range<u64>> {
    let seeds: Vec<u64> = line
        .split(" ")
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();
    let mut seed_range: HashSet<Range<u64>> = HashSet::new();
    let mut i = 0;
    while i < seeds.len() - 1 {
        let start = seeds[i] as u64;
        let end = start + seeds[i + 1] - 1 as u64;
        seed_range.insert(start..end);
        i += 2;
    }
    return seed_range;
}

fn parse_map(input: &str, validation_vec: &HashSet<Range<u64>>) -> HashSet<Range<u64>> {
    let mut line_iter = input.lines();
    //skip header line
    line_iter.next();
    let mut result_set: HashSet<Range<u64>> = HashSet::new();
    let mut unseen: HashSet<Range<u64>> = HashSet::new();
    for line in line_iter {
        println!("line: {}", line);
        let nums: Vec<u64> = line
            .split(" ")
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();
        if nums.len() != 3 {
            panic!("Invalid line: {}", line);
        }
        let (a, b, c) = (nums[0], nums[1], nums[2]);
        let brange = b..(b + c - 1);
        let arange = a..(a + c - 1);
        //print a b c
        println!("a: {}, b: {}, c: {}", a, b, c);
        for range in validation_vec {
            // check if subset
            if range.start == brange.start {
                if range.end == brange.end {
                    result_set.insert(arange.clone());
                } else if range.end > brange.end {
                    result_set.insert(arange.clone());
                    unseen.insert(brange.end + 1..range.end);
                } else if range.end < brange.end {
                    result_set.insert(arange.start..arange.start + (range.start - range.end));
                }
            } else if range.start < brange.start && range.end > brange.start {
                unseen.insert(range.start..brange.start - 1);
                if range.end == brange.end {
                    result_set.insert(arange.start..arange.end);
                } else if range.end > brange.end {
                    result_set.insert(arange.start..arange.end);
                    unseen.insert(brange.end + 1..range.end);
                } else if range.end < brange.end {
                    result_set.insert(arange.start..arange.start + (range.end - range.start));
                }
            } else if range.start > brange.start && range.start < brange.end {
                if range.end == brange.end {
                    result_set.insert(arange.start + (range.start - brange.start)..arange.end);
                } else if range.end > brange.end {
                    result_set.insert(arange.start + (range.start - brange.start)..arange.end);
                    unseen.insert(brange.end + 1..range.end);
                } else if range.end < brange.end {
                    result_set.insert(
                        arange.start + (range.start - brange.start)
                            ..arange.start + (range.end - range.start),
                    );
                }
            } else {
                println!("unseen");
            }
        }
    }
    println!("result_set: {:?}", result_set);
    unseen.into_iter().for_each(|x| {
        result_set.insert(x);
    });
    return result_set;
}

fn process(input: &str) -> u64 {
    let seeds = parse_seeds(input.lines().next().unwrap());
    let mut chunks = input.split("\n\n");
    chunks.next();
    let mut validation_set = seeds;
    for chunk in chunks {
        let result_set = parse_map(chunk, &validation_set);
        validation_set = result_set;
    }
    println!("{:?}", validation_set);
    let min = validation_set.iter().min_by_key(|x| x.start).unwrap();
    return min.start;
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
        assert_eq!(val, 46);
    }
}
