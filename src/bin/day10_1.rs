use std::{
    fs::{self},
    time::Instant,
};

use aoc24::{algorithms::flood_fill, grid::Grid, position::Position};

#[derive(Debug)]
struct Input {
    heightmap: Grid<u8>,
}

fn parse_input(s: &str) -> Input {
    let lines = s
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect::<Vec<_>>();
    let grid = Grid::from_vecs(&lines);
    Input { heightmap: grid }
}

fn solve(input: &Input) -> usize {
    input
        .heightmap
        .enumerate()
        .filter(|(_, val)| **val == 0)
        .map(|(p, _)| get_score(&input.heightmap, p))
        .sum()
}

fn get_score(grid: &Grid<u8>, start_pos: Position) -> usize {
    let mut count = 0;
    flood_fill(start_pos, |p| {
        let val = *grid.get_pos(p);
        if val == 9 {
            count += 1;
        }
        grid.neighbours(*p)
            .filter_map(move |(p, n)| (*n == val + 1).then_some(p))
    });
    count
}

fn main() {
    let start_time = Instant::now();
    let input_str = fs::read_to_string("data/day10/input").unwrap();
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
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 36);
    }
}
