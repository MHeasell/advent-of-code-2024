use std::fs::{self};

#[derive(Debug)]
struct Input {
    rules: Vec<(i32, i32)>,
    updates: Vec<Vec<i32>>,
}

fn parse_input(s: &str) -> Input {
    let mut lines = s.lines().map(|l| l.to_string());

    let rules: Vec<_> = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            l.split_once('|')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap()
        })
        .collect();

    let updates: Vec<Vec<_>> = lines
        .map(|l| l.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    Input { rules, updates }
}

fn solve(input: &Input) -> i32 {
    // We could speed up by putting the rules into some data structure
    // so lookup is faster, but this will do for now.
    input
        .updates
        .iter()
        .filter_map(|u| fix_order(&input.rules, u))
        .map(|u| u[u.len() / 2])
        .sum()
}

fn fix_order(rules: &[(i32, i32)], u: &[i32]) -> Option<Vec<i32>> {
    let mut buf = Vec::new();
    let mut broken = false;
    for page in u {
        let idx = rules
            .iter()
            .filter(|(before, _)| *before == *page)
            .filter_map(|(_, after)| buf.iter().position(|p| p == after))
            .min();
        broken |= idx.is_some();
        buf.insert(idx.unwrap_or(buf.len()), *page);
    }

    broken.then_some(buf)
}

fn main() {
    let input_str = fs::read_to_string("data/day05/input").unwrap();
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
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 123);
    }
}
