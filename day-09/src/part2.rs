fn process_history(parent_vec: &Vec<i64>) -> i64 {
    let mut child_vec: Vec<i64> = Vec::new();
    let mut only_zeroes = true;
    for i in 0..(parent_vec.len() - 1) {
        let val = parent_vec[i + 1] - parent_vec[i];
        if val != 0 {
            only_zeroes = false;
        }
        child_vec.push(val);
    }
    if only_zeroes {
        return parent_vec.first().unwrap().clone();
    }
    let val = parent_vec.first().unwrap() - process_history(&child_vec);
    return val;
}

fn process(input: &str) -> i64 {
    let mut val = 0;
    for line in input.lines() {
        let nums = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        val += process_history(&nums);
    }
    return val;
}

fn main() {
    let text = include_str!("../input1.txt");
    let val = process(text);
    println!("{}", val);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_history() {
        let num = process_history(&vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(-3, num);
    }

    #[test]
    fn test_process() {
        let text = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let val = process(text);
        assert_eq!(2, val);
    }
}
