use std::{
    collections::HashSet,
    fs::{self},
    iter,
    time::Instant,
};

use aoc24::{algorithms::detect_loop, direction::Direction, grid::Grid, position::Position};

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
    let start_pos = input.grid.position(|e| *e == TerrainType::Guard).unwrap();
    let start_facing_dir = Direction::Up;

    let mut obstacle_candidate_positions = HashSet::new();
    for obstacle_pos in
        iter_path(&input.grid, start_pos, start_facing_dir, None).map(|(pos, _)| pos)
    {
        if obstacle_pos != start_pos
            && !obstacle_candidate_positions.contains(&obstacle_pos)
            && is_valid_obstacle_candidate(&input.grid, start_pos, start_facing_dir, obstacle_pos)
        {
            obstacle_candidate_positions.insert(obstacle_pos);
        }
    }

    obstacle_candidate_positions.len()
}

fn iter_path(
    grid: &Grid<TerrainType>,
    start_pos: Position,
    start_facing_dir: Direction,
    extra_obstacle: Option<Position>,
) -> impl Iterator<Item = (Position, Direction)> + Clone + '_ {
    iter::successors(Some((start_pos, start_facing_dir)), move |prev| {
        let candidate_pos = prev.0.move_in_direction(prev.1);
        (extra_obstacle == Some(candidate_pos))
            .then_some(&TerrainType::Blocked)
            .or_else(|| grid.try_get_pos(&candidate_pos))
            .map(|terrain| match terrain {
                TerrainType::Blocked => (prev.0, prev.1.rotate_cw()),
                TerrainType::Free | TerrainType::Guard => (candidate_pos, prev.1),
            })
    })
}

fn is_valid_obstacle_candidate(
    grid: &Grid<TerrainType>,
    pos: Position,
    facing_dir: Direction,
    obstacle_pos: Position,
) -> bool {
    match grid.try_get_pos(&obstacle_pos) {
        Some(TerrainType::Free) | Some(TerrainType::Guard) => {
            detect_loop(&iter_path(grid, pos, facing_dir, Some(obstacle_pos))).is_some()
        }
        Some(TerrainType::Blocked) | None => false,
    }
}

fn main() {
    let start_time = Instant::now();
    let input_str = fs::read_to_string("data/day06/input").unwrap();
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

        assert_eq!(answer, 6);
    }
}
