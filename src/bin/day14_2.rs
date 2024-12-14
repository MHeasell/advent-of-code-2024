use std::{
    collections::HashSet,
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
    let rect = Rect::new(width, height);

    for i in 0..10000 {
        let lookup: HashSet<Position> = input
            .robots
            .iter()
            .map(|robot| rect.pos_at_t(robot, i))
            .collect();

        // hack: let's assume that this christmas tree drawing will have 20 robots
        // in a horizontal line somewhere.
        if rect.find_robots_row(&lookup) {
            rect.draw(&lookup);
            return i as usize;
        }
    }
    panic!("did not find image");
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

    fn find_robots_row(&self, lookup: &HashSet<Position>) -> bool {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if lookup.contains(&Position::new(x, y)) {
                    count += 1;
                } else {
                    count = 0;
                };

                if count == 20 {
                    return true;
                }
            }
        }

        false
    }

    fn draw(&self, lookup: &HashSet<Position>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = if lookup.contains(&Position::new(x, y)) {
                    '#'
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!();
        }
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
