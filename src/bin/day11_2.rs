use std::{
    collections::HashMap,
    fs::{self},
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

fn solve(input: Input, num_blinks: u64) -> u64 {
    let mut memo = MemoizedState::new();

    input
        .stones
        .into_iter()
        .map(|s| memo.stones_after(s, num_blinks))
        .sum()
}

struct MemoizedState {
    cache: HashMap<(u64, u64), u64>,
}

impl MemoizedState {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn stones_after(&mut self, s: u64, blinks: u64) -> u64 {
        if blinks == 0 {
            return 1;
        }
        if let Some(n) = self.cache.get(&(s, blinks)) {
            return *n;
        }

        let (s1, s2) = blink_stone(s);
        let result = self.stones_after(s1, blinks - 1)
            + s2.map(|s2| self.stones_after(s2, blinks - 1)).unwrap_or(0);
        self.cache.insert((s, blinks), result);
        result
    }
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

    let answer = solve(input, 75);

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
        let answer = solve(input, 25);

        assert_eq!(answer, 55312);
    }
}
