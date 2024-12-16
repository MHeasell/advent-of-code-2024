use std::{
    fs::{self},
    time::Instant,
};

use aoc24::{algorithms::dijkstra_search, direction::Direction, grid::Grid};

#[derive(Debug)]
struct Input {
    grid: Grid<char>,
}

fn parse_input(s: &str) -> Input {
    let lines = s.lines().map(|l| l.to_string()).collect::<Vec<_>>();
    let grid = Grid::from_strings(&lines);
    Input { grid }
}

fn solve(input: &Input) -> i64 {
    let start = input.grid.position(|c| *c == 'S').unwrap();
    dijkstra_search(
        &[(start, Direction::Right)],
        |(pos, dir)| {
            let forward_pos = pos.move_in_direction(*dir);
            let forward_cell = input.grid.get_pos(&forward_pos);
            [
                Some(((*pos, dir.rotate_cw()), 1000)),
                Some(((*pos, dir.rotate_ccw()), 1000)),
                (*forward_cell != '#').then_some(((forward_pos, *dir), 1)),
            ]
            .into_iter()
            .flatten()
            .collect()
        },
        |(pos, _)| *input.grid.get_pos(pos) == 'E',
    )
    .unwrap()
}

fn main() {
    let start_time = Instant::now();
    let input_str = fs::read_to_string("data/day16/input").unwrap();
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
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 7036);
    }

    #[test]
    fn test_solve2() {
        let input_str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 11048);
    }
}
