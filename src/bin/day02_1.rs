use std::fs::{self};

#[derive(Debug)]
struct Input {
    reports: Vec<Vec<i32>>,
}

fn parse_input(s: &str) -> Input {
    let lines: Vec<Vec<i32>> = s
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();
    Input { reports: lines }
}

fn solve(input: &Input) -> usize {
    input
        .reports
        .iter()
        .filter(|x| (all_increasing(x) || all_decreasing(x)) && differ_in_range(x))
        .count()
}

fn differ_in_range(xs: &[i32]) -> bool {
    xs.windows(2).all(|w| {
        let (a, b) = (w[0], w[1]);
        let diff = a.abs_diff(b);
        diff >= 1 && diff <= 3
    })
}

fn all_decreasing(xs: &[i32]) -> bool {
    xs.is_sorted_by(|a, b| a > b)
}

fn all_increasing(xs: &[i32]) -> bool {
    xs.is_sorted_by(|a, b| a < b)
}

fn main() {
    let input_str = fs::read_to_string("data/day02/input").unwrap();
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
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 2);
    }
}