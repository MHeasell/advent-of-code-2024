use std::fs::{self};

use aoc24::{direction::EightWayDirection, grid::Grid, position::Position};

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
    // Could speed up slightly by not iterating the edges
    // where the X is guaranteed to go out of bounds.
    input
        .grid
        .pos_iter()
        .filter(|pos| is_x_mas(&input.grid, *pos))
        .count()
}

fn is_x_mas(grid: &Grid<char>, pos: Position) -> bool {
    if grid.try_get_pos(&pos) != Some(&'A') {
        return false;
    }

    let has_first_diag = match (
        grid.try_get_pos(&pos.move_in_direction8(EightWayDirection::UpLeft)),
        grid.try_get_pos(&pos.move_in_direction8(EightWayDirection::DownRight)),
    ) {
        (Some(&'M'), Some(&'S')) | (Some(&'S'), Some(&'M')) => true,
        _ => false,
    };

    if !has_first_diag {
        return false;
    }

    let has_second_diag = match (
        grid.try_get_pos(&pos.move_in_direction8(EightWayDirection::UpRight)),
        grid.try_get_pos(&pos.move_in_direction8(EightWayDirection::DownLeft)),
    ) {
        (Some(&'M'), Some(&'S')) | (Some(&'S'), Some(&'M')) => true,
        _ => false,
    };

    if !has_second_diag {
        return false;
    }

    true
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

        assert_eq!(answer, 9);
    }
}
