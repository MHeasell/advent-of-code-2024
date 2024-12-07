use std::{
    collections::HashMap,
    fs::{self},
};

fn solve(input: &str) -> i32 {
    let lines = input.lines();

    let (list_a, list_b): (Vec<_>, Vec<_>) = lines
        .map(|l| {
            let [a, b]: [i32; 2] = l
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            (a, b)
        })
        .unzip();

    let freqs = {
        let mut freqs = HashMap::new();
        for x in list_b {
            freqs.entry(x).and_modify(|e| *e += 1).or_insert(1);
        }
        freqs
    };

    let val: i32 = list_a
        .into_iter()
        .map(|x| {
            let freq = freqs.get(&x).copied().unwrap_or(0);
            x * freq
        })
        .sum();

    val
}

fn main() {
    println!(
        "{}",
        solve(&fs::read_to_string("data/day01/input").unwrap())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = "\
3   4
4   3
2   5
1   3
3   9
3   3
";
        assert_eq!(solve(input), 31)
    }
}
