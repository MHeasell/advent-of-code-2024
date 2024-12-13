use std::{
    collections::HashSet,
    fs::{self},
    hash::Hash,
    time::Instant,
};

use aoc24::grid::Grid;

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
        let mut perimeter = 0;
        flood_fill(&mut seen, pos, |p| {
            let val = *input.grid.get_pos(p);
            let neighbours: Vec<_> = input
                .grid
                .neighbours(*p)
                .filter_map(|(p, v)| (*v == val).then_some(p))
                .collect();
            area += 1;
            perimeter += 4 - neighbours.len();
            neighbours.into_iter()
        });
        total += area * perimeter;
    }

    total
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
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 1930);
    }
}
