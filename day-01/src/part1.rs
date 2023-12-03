use std::error::Error;

fn process(input: &str) -> Result<u32, Box<dyn Error>> {
    let output = input
        .lines()
        .map(|line| {
            let mut filtered_chars = line.chars().filter_map(|c| c.to_digit(10));
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
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        ) {
            Ok(result) => assert_eq!(result, 142),
            Err(error) => panic!("Error: {}", error),
        }
    }
}
