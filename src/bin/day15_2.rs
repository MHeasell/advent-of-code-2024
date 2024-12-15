use std::{
    fs::{self},
    time::Instant,
};

use aoc24::{direction::Direction, grid::Grid, position::Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BoxSide {
    Left,
    Right,
}
impl BoxSide {
    fn opposite_dir(&self) -> Direction {
        match self {
            BoxSide::Left => Direction::Right,
            BoxSide::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Box(BoxSide),
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
            .into_iter()
            .map(|l| {
                l.chars()
                    .flat_map(|c| {
                        let (a, b) = translate_char(c);
                        [a, b]
                    })
                    .map(|c| parse_char(c).unwrap())
                    .collect()
            })
            .collect::<Vec<_>>(),
    );

    let instructions = groups
        .next()
        .unwrap()
        .into_iter()
        .flat_map(|l| l.chars().map(|c| parse_instruction(c).unwrap()))
        .collect();

    Input { grid, instructions }
}

fn translate_char(c: char) -> (char, char) {
    match c {
        '#' => ('#', '#'),
        '.' => ('.', '.'),
        '@' => ('@', '.'),
        'O' => ('[', ']'),
        _ => panic!("invalid char {}", c),
    }
}

fn parse_char(c: char) -> Option<Cell> {
    match c {
        '#' => Some(Cell::Wall),
        '.' => Some(Cell::Floor),
        '@' => Some(Cell::Robot),
        '[' => Some(Cell::Box(BoxSide::Left)),
        ']' => Some(Cell::Box(BoxSide::Right)),
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
    if can_push_into(g, next_pos, d) {
        push_into(g, next_pos, d);
        *pos = next_pos;
    }
}

fn push_into(g: &mut Grid<Cell>, pos: Position, d: Direction) {
    match *g.get_pos(&pos) {
        Cell::Wall => panic!("hit wall while pushing"),
        Cell::Floor | Cell::Robot => {
            // do nothing
        }
        Cell::Box(side) => {
            match d {
                Direction::Right | Direction::Left => {
                    // Need to push both units of the two-wide box.
                    let next_pos = pos.move_in_direction(d);
                    let next_next_pos = next_pos.move_in_direction(d);

                    push_into(g, next_next_pos, d);

                    g.set_pos(&next_next_pos, *g.get_pos(&next_pos));
                    g.set_pos(&next_pos, *g.get_pos(&pos));
                    g.set_pos(&pos, Cell::Floor);
                }
                Direction::Up | Direction::Down => {
                    // Need to push both sides of the box.
                    let other_side_pos = pos.move_in_direction(side.opposite_dir());
                    let next_pos = pos.move_in_direction(d);
                    let next_other_side_pos = other_side_pos.move_in_direction(d);

                    push_into(g, next_pos, d);

                    g.set_pos(&next_pos, *g.get_pos(&pos));
                    g.set_pos(&pos, Cell::Floor);

                    push_into(g, next_other_side_pos, d);

                    g.set_pos(&next_other_side_pos, *g.get_pos(&other_side_pos));
                    g.set_pos(&other_side_pos, Cell::Floor);
                }
            }
        }
    }
}

fn can_push_into(g: &mut Grid<Cell>, pos: Position, d: Direction) -> bool {
    let cell = g.get_pos(&pos);
    match cell {
        Cell::Wall => false,
        Cell::Floor | Cell::Robot => true,
        Cell::Box(side) => match d {
            Direction::Up | Direction::Down => {
                // Boxes are two wide, so pushing up or down can mean that a box
                // may push on two other boxes. Check both sides.
                let other_side_pos = pos.move_in_direction(side.opposite_dir());
                can_push_into(g, pos.move_in_direction(d), d)
                    && can_push_into(g, other_side_pos.move_in_direction(d), d)
            }
            Direction::Left | Direction::Right => {
                // Boxes are two wide and one tall, so pushing horizontally is
                // easy, so simply skip two cells ahead.
                can_push_into(g, pos.move_in_direction(d).move_in_direction(d), d)
            }
        },
    }
}

fn calc_gps_sum(g: &Grid<Cell>) -> i64 {
    g.enumerate()
        .filter(|(_, c)| **c == Cell::Box(BoxSide::Left))
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

        assert_eq!(answer, 9021);
    }
}
