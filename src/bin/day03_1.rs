use std::fs::{self};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Input {
    muls: Vec<(i32, i32)>,
}

lazy_static! {
    static ref MUL_REGEX: Regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
}

fn parse_input(s: &str) -> Input {
    let muls: Vec<(i32, i32)> = MUL_REGEX
        .captures_iter(s)
        .map(|c| (c[1].parse().unwrap(), c[2].parse().unwrap()))
        .collect();

    Input { muls }
}

fn solve(input: &Input) -> i32 {
    input.muls.iter().map(|(a, b)| a * b).sum()
}

fn main() {
    let input_str = fs::read_to_string("data/day03/input").unwrap();
    let input = parse_input(&input_str);

    let answer = solve(&input);

    println!("{}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input_str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 161);
    }
}
