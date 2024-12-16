use std::{
    fs::{self},
    time::Instant,
};

use aoc24::{direction::Direction, grid::Grid, position::Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Box,
    Floor,
    Robot,
}

#[derive(Debug)]
struct Input {
    grid: Grid<Cell>,
    instructions: Vec<Direction>,
}

fn parse_input(s: &str) -> Input {
    let lines = s.lines().map(|l| l.to_string()).collect::<Vec<_>>();
    let mut groups = lines.split(|l| l.is_empty());

    let grid = Grid::from_vecs(
        &groups
            .next()
            .unwrap()
            .iter()
            .map(|l| l.chars().map(|c| parse_char(c).unwrap()).collect())
            .collect::<Vec<_>>(),
    );

    let instructions = groups
        .next()
        .unwrap()
        .iter()
        .flat_map(|l| l.chars().map(|c| parse_instruction(c).unwrap()))
        .collect();

    Input { grid, instructions }
}

fn parse_char(c: char) -> Option<Cell> {
    match c {
        '#' => Some(Cell::Wall),
        '.' => Some(Cell::Floor),
        '@' => Some(Cell::Robot),
        'O' => Some(Cell::Box),
        _ => None,
    }
}

fn parse_instruction(c: char) -> Option<Direction> {
    match c {
        '<' => Some(Direction::Left),
        '^' => Some(Direction::Up),
        '>' => Some(Direction::Right),
        'v' => Some(Direction::Down),
        _ => None,
    }
}

fn solve(input: &Input) -> i64 {
    let mut working_grid = input.grid.clone();
    let mut robot_pos = input.grid.position(|x| x == &Cell::Robot).unwrap();
    for i in &input.instructions {
        apply_instruction(&mut working_grid, &mut robot_pos, *i);
    }

    calc_gps_sum(&working_grid)
}

fn apply_instruction(g: &mut Grid<Cell>, pos: &mut Position, d: Direction) {
    let next_pos = pos.move_in_direction(d);
    let cell_ahead = g.get_pos(&next_pos);
    match cell_ahead {
        Cell::Wall => {
            // do nothing
        }
        Cell::Box => {
            if push_box(g, next_pos, d) {
                *pos = next_pos;
            }
        }
        Cell::Floor | Cell::Robot => {
            *pos = next_pos;
        }
    }
}

fn push_box(g: &mut Grid<Cell>, pos: Position, d: Direction) -> bool {
    assert_eq!(g.get_pos(&pos), &Cell::Box);
    let next_pos = pos.move_in_direction(d);

    g.try_get_pos(&next_pos)
        .copied()
        .map(|cell| match cell {
            Cell::Wall => false,
            Cell::Box => {
                if push_box(g, next_pos, d) {
                    g.set_pos(&next_pos, Cell::Box);
                    g.set_pos(&pos, Cell::Floor);
                    true
                } else {
                    false
                }
            }
            Cell::Floor | Cell::Robot => {
                g.set_pos(&next_pos, Cell::Box);
                g.set_pos(&pos, Cell::Floor);
                true
            }
        })
        .unwrap_or(false)
}

fn calc_gps_sum(g: &Grid<Cell>) -> i64 {
    g.enumerate()
        .filter(|(_, c)| c == &&Cell::Box)
        .map(|(p, _)| (100 * p.y) + p.x)
        .sum()
}

fn main() {
    let start_time = Instant::now();
    let input_str = fs::read_to_string("data/day15/input").unwrap();
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
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 2028);
    }

    #[test]
    fn test_solve2() {
        let input_str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^

";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 10092);
    }
}
