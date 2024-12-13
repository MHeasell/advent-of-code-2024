use std::{
    fs::{self},
    iter,
    time::Instant,
};

#[derive(Debug)]
struct Input {
    stones: Vec<u64>,
}

fn parse_input(s: &str) -> Input {
    let line = s
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .next()
        .unwrap();
    Input { stones: line }
}

fn solve(input: Input) -> usize {
    let num_blinks = 25;

    iter::successors(Some(input.stones), |stones| {
        Some(blink(stones.iter().copied()).collect())
    })
    .take(num_blinks + 1) // also count initial state
    .last()
    .unwrap()
    .len()
}

fn blink(stones: impl Iterator<Item = u64>) -> impl Iterator<Item = u64> {
    stones.flat_map(|stone| {
        let (left, right) = blink_stone(stone);
        [Some(left), right].into_iter().flatten()
    })
}

fn blink_stone(s: u64) -> (u64, Option<u64>) {
    if s == 0 {
        (1, None)
    } else if let Some((left, right)) = split_digits(s) {
        (left, Some(right))
    } else {
        (s * 2024, None)
    }
}

fn split_digits(s: u64) -> Option<(u64, u64)> {
    let str = s.to_string();
    if str.len() % 2 != 0 {
        return None;
    }

    let half_len = str.len() / 2;
    let left = str[..half_len].parse().unwrap();
    let right = str[half_len..].parse().unwrap();
    Some((left, right))
}

fn main() {
    let start_time = Instant::now();
    let input_str = fs::read_to_string("data/day11/input").unwrap();
    let input = parse_input(&input_str);

    let answer = solve(input);

    println!("Answer: {}", answer);
    println!("Elapsed time: {}ms", start_time.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input_str = "\
125 17
";
        let input = parse_input(&input_str);
        let answer = solve(input);

        assert_eq!(answer, 55312);
    }
}
