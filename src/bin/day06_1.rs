use std::{
    collections::HashSet,
    fs::{self},
};

use aoc24::{direction::Direction, grid::Grid};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TerrainType {
    Free,
    Blocked,
    Guard,
}

#[derive(Debug)]
struct Input {
    grid: Grid<TerrainType>,
}

fn parse_input(s: &str) -> Input {
    let lines = s
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => TerrainType::Free,
                    '#' => TerrainType::Blocked,
                    '^' => TerrainType::Guard,
                    _ => panic!("unexpected symbol {}", c),
                })
                .collect()
        })
        .collect::<Vec<_>>();
    let grid = Grid::from_vecs(&lines);
    Input { grid }
}

fn solve(input: &Input) -> usize {
    let mut pos = input.grid.position(|e| *e == TerrainType::Guard).unwrap();
    let mut seen = HashSet::new();

    let mut facing_dir = Direction::Up;
    loop {
        seen.insert(pos);
        let candidate_pos = pos.move_in_direction(facing_dir);
        match input.grid.try_get_pos(&candidate_pos) {
            Some(TerrainType::Blocked) => {
                facing_dir = facing_dir.rotate_cw();
            }
            Some(TerrainType::Free) | Some(TerrainType::Guard) => {
                pos = candidate_pos;
            }
            None => {
                break;
            }
        };
    }

    seen.len()
}

fn main() {
    let input_str = fs::read_to_string("data/day06/input").unwrap();
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
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 41);
    }
}
