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
        .filter(|x| is_safe_report_with_tolerance(x))
        .count()
}

fn is_safe_report(x: &[i32]) -> bool {
    (all_increasing(x) || all_decreasing(x)) && differ_in_range(x)
}

fn is_safe_report_with_tolerance(x: &[i32]) -> bool {
    if is_safe_report(x) {
        return true;
    }

    // This is really slow but the lists are really short (<10 elems)
    // so it doesn't matter an awful lot.
    (0..x.len()).any(|i| {
        let mut x2 = x.to_vec();
        x2.remove(i);
        is_safe_report(&x2)
    })
}

fn differ_in_range(xs: &[i32]) -> bool {
    xs.windows(2).all(|w| {
        let (a, b) = (w[0], w[1]);
        let diff = a.abs_diff(b);
        (1..=3).contains(&diff)
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

        assert_eq!(answer, 4);
    }
}
