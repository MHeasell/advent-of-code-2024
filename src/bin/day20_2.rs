use std::{
    collections::HashMap,
    fs::{self},
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
    enumerate_cheats(&input, 20)
        .filter(|(i, _)| *i >= 100)
        .count()
}

fn find_bridges(grid: &Grid<Cell>, p: Position, n: usize) -> impl Iterator<Item = Position> + '_ {
    // We can just enumerate every floor position in range of the start here,
    // regardless of whether it goes through a wall. Anything that doesn't go
    // through a wall at some point won't save any time and will be discarded
    // later.

    // There is definitely a smarter way to iterate, but I already have this
    // flood fill function on hand so it's quick to implement.
    let ends = flood_fill(p, |q| {
        if q.manhattan_distance(&p) as usize == n {
            return vec![].into_iter();
        }
        DIRECTIONS
            .iter()
            .map(|&d| q.move_in_direction(d))
            .filter(|p| grid.is_in_bounds(p))
            .collect::<Vec<_>>()
            .into_iter()
    });
    ends.into_iter().filter(|p| *grid.get_pos(p) != Cell::Wall)
}

fn enumerate_cheats(
    input: &Input,
    n: usize,
) -> impl Iterator<Item = (usize, (Position, Position))> + '_ {
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
        .flat_map(move |(p, _)| find_bridges(&input.grid, p, n).map(move |b| (p, b)))
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
        for (i, _) in enumerate_cheats(&input, 20) {
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
