use std::{
    fs::{self},
    iter,
    time::Instant,
};

#[derive(Debug)]
struct Input {
    lines: Vec<u64>,
}

fn parse_input(s: &str) -> Input {
    let lines = s.lines().map(|l| l.parse().unwrap()).collect::<Vec<_>>();
    Input { lines }
}

fn solve(input: &Input) -> u64 {
    input
        .lines
        .iter()
        .map(|l| iter_secret_nums(*l).skip(1).take(2000).last().unwrap())
        .sum()
}

fn iter_secret_nums(l: u64) -> impl Iterator<Item = u64> {
    iter::successors(Some(l), |n| Some(next_num(*n)))
}

fn next_num(mut n: u64) -> u64 {
    n = prune(mix(n, n * 64));
    n = prune(mix(n, n / 32));
    n = prune(mix(n, n * 2048));
    n
}

fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

fn prune(n: u64) -> u64 {
    n % 16777216
}

fn main() {
    let start_time = Instant::now();
    let input_str = fs::read_to_string("data/day22/input").unwrap();
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
1
10
100
2024
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 37327623);
    }

    #[test]
    fn test_iter_secret_nums() {
        let initial = 123;
        let v = iter_secret_nums(initial)
            .skip(1)
            .take(10)
            .collect::<Vec<_>>();
        assert_eq!(
            v,
            vec![
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254,
            ]
        )
    }
}
