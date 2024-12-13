use std::{
    collections::HashSet,
    fs::{self},
    hash::Hash,
    time::Instant,
};

use aoc24::{direction::Direction, grid::Grid, position::Position};

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
    let mut seen = HashSet::new();
    let mut total = 0;
    for pos in input.grid.pos_iter() {
        if seen.contains(&pos) {
            continue;
        }
        let mut area = 0;
        let mut num_sides = 0;
        flood_fill(&mut seen, pos, |p| {
            let val = *input.grid.get_pos(&p);
            let neighbours: Vec<_> = input
                .grid
                .neighbours(*p)
                .filter_map(|(p, v)| (*v == val).then_some(p))
                .collect();
            area += 1;
            num_sides += count_corners(&input.grid, *p);
            neighbours.into_iter()
        });
        total += area * num_sides;
    }

    total
}

// counting corners is a lot easier than counting sides,
// and the number of corners always equals the number of sides.
fn count_corners(grid: &Grid<char>, p: Position) -> usize {
    // each of the 4 corners of our cell can either be part of a convex corner
    // of the shape or a concave corner of the shape.

    let val = *grid.get_pos(&p);
    let is_diff_region = |p: Position| grid.try_get_pos(&p).map(|v| *v != val).unwrap_or(true);
    let is_convex_corner = |vert: Direction, horiz: Direction| {
        is_diff_region(p.move_in_direction(vert)) && is_diff_region(p.move_in_direction(horiz))
    };
    let is_concave_corner = |vert: Direction, horiz: Direction| {
        !is_diff_region(p.move_in_direction(vert))
            && !is_diff_region(p.move_in_direction(horiz))
            && is_diff_region(p.move_in_direction(vert).move_in_direction(horiz))
    };

    [Direction::Up, Direction::Down]
        .into_iter()
        .flat_map(|v| {
            [Direction::Left, Direction::Right]
                .into_iter()
                .map(move |h| (v, h))
        })
        .flat_map(|(v, h)| {
            [
                usize::from(is_convex_corner(v, h)),
                usize::from(is_concave_corner(v, h)),
            ]
        })
        .sum()
}

// flood fill but you provide the seen set
pub fn flood_fill<T, I, F>(seen: &mut HashSet<T>, start: T, mut succ: F)
where
    T: Eq + Hash + Copy,
    I: Iterator<Item = T>,
    F: FnMut(&T) -> I,
{
    let mut stack = vec![start];
    seen.insert(start);

    while let Some(elem) = stack.pop() {
        let neighbours = succ(&elem);
        for n in neighbours {
            if seen.insert(n) {
                stack.push(n);
            }
        }
    }
}

fn main() {
    let start_time = Instant::now();
    let input_str = fs::read_to_string("data/day12/input").unwrap();
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
AAAA
BBCD
BBCC
EEEC
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 80);
    }

    #[test]
    fn test_solve2() {
        let input_str = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 236);
    }
    #[test]
    fn test_solve3() {
        let input_str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 368);
    }
}
