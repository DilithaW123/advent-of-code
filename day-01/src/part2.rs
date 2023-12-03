use std::error::Error;

// How to fix case of eighttwothree?
// Instead of replacing eightwo with eigh2
// We can replace it with eight2o
// Replace two with t2o
// That way we don't skip characters before & after two
fn replace_word(line: &str) -> String {
    let new_line = line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
    new_line
}

fn process(input: &str) -> Result<u32, Box<dyn Error>> {
    let output = input
        .lines()
        .map(|line| {
            let replaced = replace_word(line);
            // println!("{replaced}");
            let mut filtered_chars = replaced.chars().filter_map(|c| c.to_digit(10));
            let first = filtered_chars
                .next()
                .expect("Expect at least one digit in each line ");
            match filtered_chars.last() {
                Some(num) => first * 10 + num,
                None => first * 10 + first,
            }
        })
        .sum::<u32>();
    Ok(output)
}

fn main() {
    println!("Hello, world!");
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
    fn test_process() {
        match process(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        ) {
            Ok(result) => assert_eq!(result, 281),
            Err(error) => panic!("Error: {}", error),
        }
    }
}
