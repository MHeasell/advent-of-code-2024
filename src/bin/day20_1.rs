use std::{
    collections::HashMap,
    fs::{self},
    iter,
    time::Instant,
};

use aoc24::{algorithms::flood_fill, direction::DIRECTIONS, grid::Grid, position::Position};

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
    enumerate_cheats(input).filter(|(i, _)| *i >= 100).count()
}

fn find_bridges(grid: &Grid<Cell>, p: Position) -> impl Iterator<Item = Position> + '_ {
    DIRECTIONS.iter().filter_map(move |d| {
        let it = iter::successors(Some(p), |&p| Some(p.move_in_direction(*d)));
        it.skip(1)
            .take(2)
            .find(|p| grid.try_get_pos(p).is_some_and(|c| *c != Cell::Wall))
    })
}

fn enumerate_cheats(input: &Input) -> impl Iterator<Item = (usize, (Position, Position))> + '_ {
    let start = input.grid.position(|c| *c == Cell::Start).unwrap();

    let mut path = HashMap::<Position, usize>::new();
    let mut count = 0;
    flood_fill(start, |&p| {
        path.insert(p, count);
        count += 1;
        DIRECTIONS
            .iter()
            .map(move |&d| p.move_in_direction(d))
            .filter(|p| input.grid.try_get_pos(p).is_some_and(|&c| c != Cell::Wall))
    });

    input
        .grid
        .enumerate()
        .filter(|(_, c)| match c {
            Cell::Wall => false,
            Cell::Floor => true,
            Cell::Start => true,
            Cell::End => false,
        })
        .flat_map(|(p, _)| find_bridges(&input.grid, p).map(move |b| (p, b)))
        .filter_map(move |(start, end)| {
            let start_step = path[&start];
            let end_step = path[&end];
            let bridge_distance = start.manhattan_distance(&end) as usize;
            if end_step <= start_step {
                return None;
            }
            let skipped_steps = end_step - start_step;
            if skipped_steps <= bridge_distance {
                return None;
            }
            Some((skipped_steps - bridge_distance, (start, end)))
        })
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
        for (i, _) in enumerate_cheats(&input) {
            cheat_counts
                .entry(i)
                .and_modify(|e| {
                    *e += 1;
                })
                .or_insert(1);
        }

        assert_eq!(cheat_counts[&2], 14);
        assert_eq!(cheat_counts[&4], 14);
        assert_eq!(cheat_counts[&6], 2);
        assert_eq!(cheat_counts[&8], 4);
        assert_eq!(cheat_counts[&10], 2);
        assert_eq!(cheat_counts[&12], 3);
        assert_eq!(cheat_counts[&20], 1);
        assert_eq!(cheat_counts[&36], 1);
        assert_eq!(cheat_counts[&38], 1);
        assert_eq!(cheat_counts[&40], 1);
        assert_eq!(cheat_counts[&64], 1);
    }
}
