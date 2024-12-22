use std::{
    fs::{self},
    iter,
    time::Instant,
};

use aoc24::{direction::DIRECTIONS, grid::Grid, position::Position};

#[derive(Debug)]
struct Input {
    grid: Grid<Cell>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Floor,
    Start,
    End,
}

fn parse_char(c: char) -> Cell {
    match c {
        '#' => Cell::Wall,
        '.' => Cell::Floor,
        'S' => Cell::Start,
        'E' => Cell::End,
        _ => panic!("illegal char {}", c),
    }
}

fn parse_input(s: &str) -> Input {
    let lines = s
        .lines()
        .map(|l| l.chars().map(parse_char).collect())
        .collect::<Vec<_>>();
    let grid = Grid::from_vecs(&lines);
    Input { grid }
}

fn solve(input: &Input) -> usize {
    let path = get_path(&input.grid);
    enumerate_cheats(&path, 20)
        .filter(|(i, _)| *i >= 100)
        .count()
}

fn enumerate_cheats(
    path_vec: &[Position],
    n: usize,
) -> impl Iterator<Item = (usize, (Position, Position))> + '_ {
    // Cheats must start and end on the path so we'll look at all pairs
    // of points on the path and keep only those that are viable cheats.
    iter_pairs_with_positions(path_vec).filter_map(move |((i, &p1), (j, &p2))| {
        let bridge_distance = p1.manhattan_distance(&p2) as usize;
        if bridge_distance > n {
            return None;
        }
        let skipped_steps = j - i;
        if skipped_steps <= bridge_distance {
            return None;
        }
        Some((skipped_steps - bridge_distance, (p1, p2)))
    })
}

/// Iterates forward pairs with their positions in the slice.
/// i.e. for the slice [a,b,c], yields ((0,a),(1,b)), ((0,a),(2,c)), ((1,b),(2,c))
fn iter_pairs_with_positions<T>(slice: &[T]) -> impl Iterator<Item = ((usize, &T), (usize, &T))> {
    slice.iter().enumerate().flat_map(|(i, p1)| {
        slice[(i + 1)..]
            .iter()
            .enumerate()
            .map(move |(j, p2)| ((i, p1), (i + j + 1, p2)))
    })
}

fn get_path(grid: &Grid<Cell>) -> Vec<Position> {
    let start = grid.position(|c| *c == Cell::Start).unwrap();

    // We rely on the assumption that there is only a single path through the maze.
    // We'll just walk through that single path from start to end.
    iter::successors(Some((start, None)), |(p, from_dir)| {
        DIRECTIONS
            .into_iter()
            .filter(|d| Some(*d) != *from_dir)
            .map(|d| (p.move_in_direction(d), Some(d.reverse())))
            .find(|(p, _)| *grid.get_pos(p) != Cell::Wall)
    })
    .map(|(p, _)| p)
    .collect()
}

fn main() {
    let start_time = Instant::now();
    let input_str = fs::read_to_string("data/day20/input").unwrap();
    let input = parse_input(&input_str);

    let answer = solve(&input);

    println!("Answer: {}", answer);
    println!("Elapsed time: {}ms", start_time.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_solve1() {
        let input_str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";
        let input = parse_input(&input_str);

        let mut cheat_counts = HashMap::new();
        let path = get_path(&input.grid);
        for (i, _) in enumerate_cheats(&path, 20) {
            cheat_counts
                .entry(i)
                .and_modify(|e| {
                    *e += 1;
                })
                .or_insert(1);
        }

        assert_eq!(cheat_counts[&50], 32);
        assert_eq!(cheat_counts[&52], 31);
        assert_eq!(cheat_counts[&54], 29);
        assert_eq!(cheat_counts[&56], 39);
        assert_eq!(cheat_counts[&58], 25);
        assert_eq!(cheat_counts[&60], 23);
        assert_eq!(cheat_counts[&62], 20);
        assert_eq!(cheat_counts[&64], 19);
        assert_eq!(cheat_counts[&66], 12);
        assert_eq!(cheat_counts[&68], 14);
        assert_eq!(cheat_counts[&70], 12);
        assert_eq!(cheat_counts[&72], 22);
        assert_eq!(cheat_counts[&74], 4);
        assert_eq!(cheat_counts[&76], 3);
    }
}
