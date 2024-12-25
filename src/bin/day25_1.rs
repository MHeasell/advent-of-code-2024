use aoc24::grid::Grid;
use std::{
    fs::{self},
    time::Instant,
};

#[derive(Debug)]
struct Input {
    grids: Vec<Grid<char>>,
}

fn parse_input(s: &str) -> Input {
    let lines = s.lines().map(|l| l.to_owned()).collect::<Vec<_>>();
    let grids = lines
        .split(|l| l.is_empty())
        .map(Grid::from_strings)
        .collect();

    Input { grids }
}

fn solve(input: &Input) -> usize {
    let (locks, keys): (Vec<_>, Vec<_>) = input
        .grids
        .iter()
        .map(|g| {
            if g.cols().all(|c| *g.get(c, 0) == '#') {
                // top all filled in, it's a lock
                let heights = g
                    .cols()
                    .map(|c| {
                        (0..g.height())
                            .rev()
                            .find(|r| *g.get(c, *r) == '#')
                            .unwrap()
                    })
                    .collect::<Vec<_>>();
                (Some(heights), None)
            } else {
                // otherwise it's a key
                let heights = g
                    .cols()
                    .map(|c| {
                        let h = (0..g.height())
                            .rev()
                            .find(|r| *g.get(c, *r) == '.')
                            .unwrap();
                        g.height() - 2 - h
                    })
                    .collect::<Vec<_>>();
                (None, Some(heights))
            }
        })
        .unzip();

    locks
        .iter()
        .flatten()
        .map(|l| {
            keys.iter()
                .flatten()
                .filter(|k| lock_key_matches(l, k))
                .count()
        })
        .sum()
}

fn lock_key_matches(l: &[usize], k: &[usize]) -> bool {
    l.iter().zip(k).all(|(a, b)| (*a + *b) <= 5)
}

fn main() {
    let start_time = Instant::now();
    let input_str = fs::read_to_string("data/day25/input").unwrap();
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
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 3);
    }
}
