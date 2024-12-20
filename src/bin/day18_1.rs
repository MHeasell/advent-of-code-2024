use std::{
    collections::HashSet,
    fs::{self},
    time::Instant,
};

use aoc24::{
    algorithms::dijkstra_search,
    direction::DIRECTIONS,
    position::{pos, Position},
};

#[derive(Debug)]
struct Input {
    coords: Vec<Position>,
}

fn parse_input(s: &str) -> Input {
    let coords = s
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            Position::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect::<Vec<_>>();
    Input { coords }
}

fn solve(input: &Input, w: i64, h: i64, n: usize) -> i64 {
    let obstacles: HashSet<_> = input.coords.iter().copied().take(n).collect();
    dijkstra_search(
        &[pos(0, 0)],
        |c| {
            DIRECTIONS
                .iter()
                .map(|d| c.move_in_direction(*d))
                .filter(|c| {
                    (0..w).contains(&c.x) && (0..h).contains(&c.y) && !obstacles.contains(c)
                })
                .map(|c| (c, 1))
                .collect()
        },
        |c| *c == pos(w - 1, h - 1),
    )
    .unwrap()
}

fn main() {
    let start_time = Instant::now();
    let input_str = fs::read_to_string("data/day18/input").unwrap();
    let input = parse_input(&input_str);

    let answer = solve(&input, 71, 71, 1024);

    println!("Answer: {}", answer);
    println!("Elapsed time: {}ms", start_time.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input_str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";
        let input = parse_input(&input_str);
        let answer = solve(&input, 7, 7, 12);

        assert_eq!(answer, 22);
    }
}
