use std::fs::{self};

fn solve(input: &str) -> u32 {
    let lines = input.lines();

    let (mut list_a, mut list_b): (Vec<_>, Vec<_>) = lines
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

    list_a.sort();
    list_b.sort();

    let val: u32 = list_a
        .into_iter()
        .zip(list_b)
        .map(|(a, b)| a.abs_diff(b))
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
        assert_eq!(solve(input), 11)
    }
}
