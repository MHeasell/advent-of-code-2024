use std::{
    collections::{HashMap, HashSet},
    fs::{self},
    time::Instant,
};

use aoc24::{grid::Grid, position::Position};

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
    let station_groups = {
        let mut groups = HashMap::<char, Vec<Position>>::new();
        for (pos, c) in input.grid.enumerate().filter(|(_, c)| **c != '.') {
            groups
                .entry(*c)
                .and_modify(|v| {
                    v.push(pos);
                })
                .or_insert_with(|| vec![pos]);
        }
        groups
    };

    let antinode_locations: HashSet<Position> = station_groups
        .values()
        .flat_map(|stations| gen_antinodes(stations))
        .filter(|p| input.grid.is_in_bounds(p))
        .collect();

    antinode_locations.len()
}

fn gen_antinodes(stations: &[Position]) -> impl Iterator<Item = Position> + '_ {
    let pairs = stations
        .iter()
        .enumerate()
        .flat_map(|(i, p)| stations[i + 1..].iter().map(move |q| (p, q)));

    pairs.flat_map(|(p, q)| {
        let dist_x = q.x - p.x;
        let dist_y = q.y - p.y;
        let antinode_1 = Position::new(q.x + dist_x, q.y + dist_y);
        let antinode_2 = Position::new(p.x - dist_x, p.y - dist_y);
        [antinode_1, antinode_2]
    })
}

fn main() {
    let start_time = Instant::now();
    let input_str = fs::read_to_string("data/day08/input").unwrap();
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
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 14);
    }
}
