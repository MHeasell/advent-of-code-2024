use std::{
    fs::{self},
    time::Instant,
};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Input {
    lines: Vec<(i64, Vec<i64>)>,
}

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(r"^([0-9]+): ([0-9 ]+)$").unwrap();
}

fn parse_input(s: &str) -> Input {
    let lines: Vec<(i64, Vec<i64>)> = s
        .lines()
        .map(|l| {
            let captures = LINE_REGEX.captures(l).unwrap();
            (
                captures[1].parse().unwrap(),
                captures[2]
                    .split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            )
        })
        .collect::<Vec<_>>();
    Input { lines }
}

fn solve(input: &Input) -> i64 {
    input
        .lines
        .iter()
        .filter(|(test_val, nums)| can_make(*test_val, nums))
        .map(|(test_val, _)| test_val)
        .sum()
}

fn can_make(target_val: i64, nums: &[i64]) -> bool {
    // We'll go backwards from the target number to the inputs, right to left.
    let Some((&last_num, rest)) = nums.split_last() else {
        // If we have no more nums left, we either reduced the target val exactly
        // and got to 0, in which case we succeeded, or there is still a number
        // left to reduce, in which case we failed.
        return target_val == 0;
    };

    if target_val % last_num == 0 {
        // Remainder is zero so we could have multiplied
        if can_make(target_val / last_num, rest) {
            return true;
        }
    }

    if target_val >= last_num {
        // target is equal or larger so we could have added
        if can_make(target_val - last_num, rest) {
            return true;
        }
    }

    false
}

fn main() {
    let start_time = Instant::now();
    let input_str = fs::read_to_string("data/day07/input").unwrap();
    let input = parse_input(&input_str);

    let answer = solve(&input);

    println!("Answer: {}", answer);
    println!("Elapsed time: {}ms", start_time.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input_str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 3749);
    }
}
