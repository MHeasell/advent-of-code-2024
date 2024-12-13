use std::{
    fs::{self},
    time::Instant,
};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Input {
    games: Vec<Game>,
}

#[derive(Debug)]
struct Game {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

lazy_static! {
    static ref LINE_A_REGEX: Regex = Regex::new(r"^Button A: X\+([0-9]+), Y\+([0-9]+)$").unwrap();
    static ref LINE_B_REGEX: Regex = Regex::new(r"^Button B: X\+([0-9]+), Y\+([0-9]+)$").unwrap();
    static ref PRIZE_REGEX: Regex = Regex::new(r"^Prize: X=([0-9]+), Y=([0-9]+)$").unwrap();
}

fn parse_input(s: &str) -> Input {
    let games = s
        .lines()
        .collect::<Vec<_>>()
        .split(|l| l.is_empty())
        .map(|lines| {
            let a = LINE_A_REGEX.captures(lines[0]).unwrap();
            let b = LINE_B_REGEX.captures(lines[1]).unwrap();
            let prize = PRIZE_REGEX.captures(lines[2]).unwrap();
            Game {
                a: (a[1].parse().unwrap(), a[2].parse().unwrap()),
                b: (b[1].parse().unwrap(), b[2].parse().unwrap()),
                prize: (prize[1].parse().unwrap(), prize[2].parse().unwrap()),
            }
        })
        .collect::<Vec<_>>();
    Input { games }
}

fn solve(input: &Input) -> i64 {
    input
        .games
        .iter()
        .map(|game| {
            calc_pushes(game)
                .map(|(a_pushes, b_pushes)| (3 * a_pushes) + b_pushes)
                .unwrap_or(0)
        })
        .sum()
}

#[allow(non_snake_case)]
fn calc_pushes(game: &Game) -> Option<(i64, i64)> {
    /*
    Let X be the target X value and Y the target Y value.
    Let I be the number of times we press button A.
    Let J be the number of times we press button B.
    Let a be the X delta for pressing A, let b be the X delta for pressing B.
    Let c be the Y delta for pressing A, let d be the Y delta for pressing B.

    We have these equations:

    X = (I*a) + (J*b)
    Y = (I*c) + (J*d)

    solve for I and J.

    X = (I*a) + (J*b)
    X - (J*b) = I*a     # subtracted J*b
    (X - (J*b))/a = I   # divided both sides by a

    We have I. Now substitute into other equation:

    Y = (I*c) + (J*d)
    Y = (((X - (J*b))/a)*c) + (J*d)    # substituted I
    Y = ((X/a - (J*b)/a)*c) + (J*d)    # distributed a
    Y = (X*c)/a - (J*b*c)/a + J*d      # distributed c
    Y - (X*c)/a = - (J*b*c)/a + J*d    # subtracted (X*c)/a
    Y - (X*c)/a = J*d - (J*b*c)/a      # reordered RHS terms
    Y - (X*c)/a = J(d - (b*c)/a)       # factored out J
    (Y - (X*c)/a)/(d - (b*c)/a) = J    # divided both sides by (d - (b*c)/a)
    (Y*a - X*c)/(d*a - b*c) = J        # multiplied top and bottom of LHS fraction by a

    We now have J.
    We can compute J, then use J to compute I.
     */

    let X = game.prize.0;
    let Y = game.prize.1;
    let a = game.a.0;
    let b = game.b.0;
    let c = game.a.1;
    let d = game.b.1;

    let J = (Y * a - X * c) / (d * a - b * c);
    let J_solvable = ((Y * a - X * c) % (d * a - b * c)) == 0;
    if !J_solvable {
        return None;
    }

    let I = ((X) - (J * b)) / a;
    let I_solvable = (((X) - (J * b)) % a) == 0;
    if !I_solvable {
        return None;
    }

    Some((I, J))
}

fn main() {
    let start_time = Instant::now();
    let input_str = fs::read_to_string("data/day13/input").unwrap();
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
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 480);
    }
}
