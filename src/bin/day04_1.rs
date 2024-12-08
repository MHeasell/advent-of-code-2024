use std::{
    fs::{self},
    iter,
};

use aoc24::{
    direction::{EightWayDirection, EIGHT_WAY_DIRECTIONS},
    grid::Grid,
    position::Position,
};

#[derive(Debug)]
struct Input {
    grid: Grid<char>,
}

fn parse_input(s: &str) -> Input {
    let lines = s.lines().map(|l| l.to_string()).collect::<Vec<_>>();
    let grid = Grid::from_strings(&lines);
    Input { grid }
}

fn solve(input: &Input) -> usize {
    input
        .grid
        .pos_iter()
        .map(|pos| count_xmases(&input.grid, pos))
        .sum()
}

fn count_xmases(grid: &Grid<char>, pos: Position) -> usize {
    EIGHT_WAY_DIRECTIONS
        .into_iter()
        .filter(|d| {
            let pos_iter = get_pos_iter(pos, *d);
            let str: String = pos_iter
                .map_while(|p| grid.try_get_pos(&p))
                .take(4)
                .collect();
            str == "XMAS"
        })
        .count()
}

fn get_pos_iter(pos: Position, d: EightWayDirection) -> impl Iterator<Item = Position> {
    iter::successors(Some(pos), move |p| Some(p.move_in_direction8(d)))
}

fn main() {
    let input_str = fs::read_to_string("data/day04/input").unwrap();
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
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 18);
    }
}
