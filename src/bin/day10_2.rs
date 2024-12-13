use std::{
    fs::{self},
    time::Instant,
};

use aoc24::{grid::Grid, position::Position};

#[derive(Debug)]
struct Input {
    heightmap: Grid<u8>,
}

fn parse_input(s: &str) -> Input {
    let lines = s
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap_or(100).try_into().unwrap())
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
        .map(|(p, _)| get_rating(&input.heightmap, p))
        .sum()
}

fn get_rating(grid: &Grid<u8>, start_pos: Position) -> usize {
    let val = *grid.get_pos(&start_pos);
    if val == 9 {
        return 1;
    }

    grid.neighbours(start_pos)
        .filter(|(_, v)| **v == val + 1)
        .map(|(p, _)| get_rating(grid, p))
        .sum()
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
.....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 3);
    }

    #[test]
    fn test_solve2() {
        let input_str = "\
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 13);
    }

    #[test]
    fn test_solve3() {
        let input_str = "\
012345
123456
234567
345678
4.6789
56789.
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 227);
    }

    #[test]
    fn test_solve4() {
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

        assert_eq!(answer, 81);
    }
}
