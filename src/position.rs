use std::fmt::Debug;

use crate::direction::{Direction, EightWayDirection};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pos({}, {})", self.x, self.y)
    }
}

impl Position {
    pub fn new(x: i64, y: i64) -> Position {
        Position { x, y }
    }
    pub fn manhattan_distance(&self, other: &Position) -> u64 {
        let delta_x = other.x.abs_diff(self.x);
        let delta_y = other.y.abs_diff(self.y);
        delta_x + delta_y
    }

    pub fn move_in_direction(&self, direction: Direction) -> Position {
        match direction {
            Direction::Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    pub fn d_offset(direction: Direction) -> Position {
        match direction {
            Direction::Up => Position { x: 0, y: -1 },
            Direction::Right => Position { x: 1, y: 0 },
            Direction::Down => Position { x: 0, y: 1 },
            Direction::Left => Position { x: -1, y: 0 },
        }
    }

    pub fn move_in_direction8(&self, direction: EightWayDirection) -> Position {
        match direction {
            EightWayDirection::Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            EightWayDirection::UpRight => Position {
                x: self.x + 1,
                y: self.y - 1,
            },
            EightWayDirection::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
            EightWayDirection::DownRight => Position {
                x: self.x + 1,
                y: self.y + 1,
            },
            EightWayDirection::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            EightWayDirection::DownLeft => Position {
                x: self.x - 1,
                y: self.y + 1,
            },
            EightWayDirection::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            EightWayDirection::UpLeft => Position {
                x: self.x - 1,
                y: self.y - 1,
            },
        }
    }
}

pub fn pos(x: i64, y: i64) -> Position {
    Position { x, y }
}

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Position {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Position {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<i64> for Position {
    type Output = Position;
    fn mul(self, rhs: i64) -> Self::Output {
        Position {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
