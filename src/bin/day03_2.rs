use std::fs::{self};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum Cmd {
    Do,
    Dont,
    Mul(i32, i32),
}

#[derive(Debug)]
struct Input {
    cmds: Vec<Cmd>,
}

lazy_static! {
    static ref CMD_REGEX: Regex =
        Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|(do\(\))|(don't\(\))").unwrap();
}

fn parse_input(s: &str) -> Input {
    // You could be a bit faster by just skipping sections of text
    // enclosed in a dont..do block, instead of parsing everything.
    // This will do though.
    let cmds: Vec<Cmd> = CMD_REGEX
        .captures_iter(s)
        .map(|c| {
            if c.get(3).is_some() {
                return Cmd::Do;
            }
            if c.get(4).is_some() {
                return Cmd::Dont;
            }
            Cmd::Mul(c[1].parse().unwrap(), c[2].parse().unwrap())
        })
        .collect();

    Input { cmds }
}

fn solve(input: &Input) -> i32 {
    input
        .cmds
        .iter()
        .scan(true, |enabled, x| {
            let instruction = match x {
                Cmd::Do => {
                    *enabled = true;
                    None
                }
                Cmd::Dont => {
                    *enabled = false;
                    None
                }
                Cmd::Mul(a, b) if *enabled => Some((*a, *b)),
                _ => None,
            };
            Some(instruction)
        })
        .flatten()
        .map(|(a, b)| a * b)
        .sum()
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
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 48);
    }
}
