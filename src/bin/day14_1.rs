use std::{
    fs::{self},
    ops::{Add, Mul},
    time::Instant,
};

use aoc24::position::Position;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Input {
    robots: Vec<Robot>,
}

#[derive(Debug)]
struct Robot {
    pos: Position,
    vel: Velocity,
}

#[derive(Debug, Clone, Copy)]
struct Velocity {
    x: i64,
    y: i64,
}
impl Velocity {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Mul<i64> for Velocity {
    type Output = Velocity;

    fn mul(self, rhs: i64) -> Self::Output {
        Velocity::new(self.x * rhs, self.y * rhs)
    }
}

impl Add<Velocity> for Position {
    type Output = Position;

    fn add(self, rhs: Velocity) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

lazy_static! {
    static ref LINE_REGEX: Regex =
        Regex::new(r"^p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)$").unwrap();
}

fn parse_input(s: &str) -> Input {
    let robots = s
        .lines()
        .map(|l| {
            let captures = LINE_REGEX.captures(l).unwrap();
            Robot {
                pos: Position::new(captures[1].parse().unwrap(), captures[2].parse().unwrap()),
                vel: Velocity::new(captures[3].parse().unwrap(), captures[4].parse().unwrap()),
            }
        })
        .collect::<Vec<_>>();
    Input { robots }
}

fn solve(input: &Input, width: i64, height: i64) -> usize {
    let mut quadrants = [0; 4];
    let rect = Rect::new(width, height);

    input
        .robots
        .iter()
        .filter_map(|robot| {
            let pos = rect.pos_at_t(robot, 100);
            rect.get_quadrant(pos)
        })
        .for_each(|idx| {
            quadrants[idx] += 1;
        });

    quadrants.into_iter().product()
}

struct Rect {
    width: i64,
    height: i64,
}

impl Rect {
    fn new(width: i64, height: i64) -> Self {
        Self { width, height }
    }

    fn pos_at_t(&self, robot: &Robot, time: i64) -> Position {
        let pos = robot.pos + (robot.vel * time);
        Position::new(pos.x.rem_euclid(self.width), pos.y.rem_euclid(self.height))
    }

    fn get_quadrant(&self, pos: Position) -> Option<usize> {
        if self.width % 2 != 0 && pos.x == self.width / 2 {
            return None;
        }
        if self.height % 2 != 0 && pos.y == self.height / 2 {
            return None;
        }

        let x = usize::from(pos.x < self.width / 2);
        let y = usize::from(pos.y < self.height / 2);
        Some((y * 2) + x)
    }
}

fn main() {
    let start_time = Instant::now();
    let input_str = fs::read_to_string("data/day14/input").unwrap();
    let input = parse_input(&input_str);

    let answer = solve(&input, 101, 103);

    println!("Answer: {}", answer);
    println!("Elapsed time: {}ms", start_time.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input_str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";
        let input = parse_input(&input_str);
        let answer = solve(&input, 11, 7);

        assert_eq!(answer, 12);
    }
}
