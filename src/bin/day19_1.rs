use std::{
    collections::HashMap,
    fs::{self},
    time::Instant,
};

#[derive(Debug)]
struct Input {
    towels: Vec<String>,
    designs: Vec<String>,
}

fn parse_input(s: &str) -> Input {
    let mut lines = s.lines();
    let towels = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();
    assert_eq!(lines.next(), Some(""));
    let designs = lines.map(|l| l.to_owned()).collect::<Vec<_>>();
    Input { towels, designs }
}

fn solve(input: &Input) -> usize {
    input
        .designs
        .iter()
        .filter(|d| is_possible(d, &input.towels))
        .count()
}

fn is_possible(d: &str, towels: &[String]) -> bool {
    is_possible_inner(&mut HashMap::new(), d, towels)
}

fn is_possible_inner(cache: &mut HashMap<String, bool>, d: &str, towels: &[String]) -> bool {
    if d.is_empty() {
        return true;
    }

    if let Some(b) = cache.get(d) {
        return *b;
    }

    let result = towels
        .iter()
        .filter_map(|t| d.strip_prefix(t))
        .any(|d| is_possible_inner(cache, d, towels));

    cache.insert(d.to_owned(), result);

    result
}

fn main() {
    let start_time = Instant::now();
    let input_str = fs::read_to_string("data/day19/input").unwrap();
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
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 6);
    }
}
