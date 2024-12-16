use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::{self},
    hash::Hash,
    time::Instant,
};

use aoc24::{
    algorithms::{flood_fill2, priority_queue_insert},
    direction::{Direction, DIRECTIONS},
    grid::Grid,
    position::Position,
};

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
    let start = input.grid.position(|c| *c == 'S').unwrap();
    let end = input.grid.position(|c| *c == 'E').unwrap();

    let costs = dijkstra_search(
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
        |(pos, _)| *pos == end,
    )
    .unwrap();

    // Walk back from the end using the costs map to find all the states that
    // are on a shortest path.
    let end_states: Vec<_> = DIRECTIONS
        .into_iter()
        .map(|d| (end, d))
        .filter(|s| costs.contains_key(s))
        .collect();
    let states = flood_fill2(&end_states, |end| {
        let our_cost = *costs.get(end).unwrap();
        [
            ((end.0, end.1.rotate_cw()), 1000),
            ((end.0, end.1.rotate_ccw()), 1000),
            ((end.0.move_in_direction(end.1.reverse()), end.1), 1),
        ]
        .into_iter()
        .filter_map(|(v, move_cost)| {
            let ancestor_cost = costs.get(&v)?;
            let diff = our_cost - *ancestor_cost;
            (diff == move_cost).then_some(v)
        })
        .collect()
    });

    // Count the unique positions within the states.
    let positions: HashSet<Position> = states.into_iter().map(|(p, _)| p).collect();
    positions.len()
}

// Dijkstra search with a few modifications:
//
// 1. Keeps track of the cost to each vertex in the closed set.
//
// 2. Continues searching until all vertices with equal cost to a goal have been
//    added to the closed set.
//
// 3. Returns the closed set at the end.
//
// This will allow our caller to walk back the path and count all the vertices
// in all the shortest paths.
pub fn dijkstra_search<T, Succ, GPred>(
    start: &[T],
    get_successors: Succ,
    is_goal: GPred,
) -> Option<HashMap<T, i64>>
where
    T: Hash + Eq + Copy,
    Succ: Fn(&T) -> Vec<(T, i64)>,
    GPred: Fn(&T) -> bool,
{
    let mut open_list = VecDeque::<(T, i64)>::new();
    let mut closed_set = HashMap::<T, i64>::new();

    let mut goal_cost = None;

    for s in start {
        open_list.push_back((*s, 0));
    }

    while let Some((value, cost)) = open_list.pop_front() {
        if goal_cost.is_some_and(|goal_cost| cost > goal_cost) {
            return Some(closed_set);
        }
        if is_goal(&value) {
            goal_cost = Some(cost);
        }

        closed_set.insert(value, cost);

        for (successor_val, successor_cost) in get_successors(&value) {
            if closed_set.contains_key(&successor_val) {
                continue;
            }

            priority_queue_insert(&mut open_list, successor_val, cost + successor_cost);
        }
    }

    None
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

        assert_eq!(answer, 45);
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

        assert_eq!(answer, 64);
    }
}
