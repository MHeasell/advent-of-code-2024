#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn rotate_cw(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn rotate_ccw(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    pub fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
        }
    }

    pub fn reflect_around_y(&self) -> Direction {
        match self {
            Direction::Up => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Down,
        }
    }

    pub fn reflect_around_x(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Left,
            Direction::Right => Direction::Right,
            Direction::Down => Direction::Up,
        }
    }
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EightWayDirection {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl EightWayDirection {
    pub fn rotate_cw(&self) -> EightWayDirection {
        match self {
            EightWayDirection::Up => EightWayDirection::UpRight,
            EightWayDirection::UpRight => EightWayDirection::Right,
            EightWayDirection::Right => EightWayDirection::DownRight,
            EightWayDirection::DownRight => EightWayDirection::Down,
            EightWayDirection::Down => EightWayDirection::DownLeft,
            EightWayDirection::DownLeft => EightWayDirection::Left,
            EightWayDirection::Left => EightWayDirection::UpLeft,
            EightWayDirection::UpLeft => EightWayDirection::Up,
        }
    }

    pub fn rotate_ccw(&self) -> EightWayDirection {
        match self {
            EightWayDirection::Up => EightWayDirection::UpLeft,
            EightWayDirection::UpLeft => EightWayDirection::Left,
            EightWayDirection::Left => EightWayDirection::DownLeft,
            EightWayDirection::DownLeft => EightWayDirection::Down,
            EightWayDirection::Down => EightWayDirection::DownRight,
            EightWayDirection::DownRight => EightWayDirection::Right,
            EightWayDirection::Right => EightWayDirection::UpRight,
            EightWayDirection::UpRight => EightWayDirection::Up,
        }
    }

    pub fn reverse(&self) -> EightWayDirection {
        match self {
            EightWayDirection::Up => EightWayDirection::Down,
            EightWayDirection::Left => EightWayDirection::Right,
            EightWayDirection::Right => EightWayDirection::Left,
            EightWayDirection::Down => EightWayDirection::Up,
            EightWayDirection::UpLeft => EightWayDirection::DownRight,
            EightWayDirection::UpRight => EightWayDirection::DownLeft,
            EightWayDirection::DownLeft => EightWayDirection::UpRight,
            EightWayDirection::DownRight => EightWayDirection::UpLeft,
        }
    }
}

pub const EIGHT_WAY_DIRECTIONS: [EightWayDirection; 8] = [
    EightWayDirection::Up,
    EightWayDirection::UpRight,
    EightWayDirection::Right,
    EightWayDirection::DownRight,
    EightWayDirection::Down,
    EightWayDirection::DownLeft,
    EightWayDirection::Left,
    EightWayDirection::UpLeft,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_rotate_cw() {
        assert_eq!(Direction::Up.rotate_cw(), Direction::Right);
        for d in DIRECTIONS {
            assert_eq!(d.rotate_cw().rotate_cw().rotate_cw().rotate_cw(), d);
        }
    }

    #[test]
    fn test_direction_rotate_ccw() {
        assert_eq!(Direction::Up.rotate_ccw(), Direction::Left);
        for d in DIRECTIONS {
            assert_eq!(d.rotate_ccw().rotate_ccw().rotate_ccw().rotate_ccw(), d);
        }
    }

    #[test]
    fn test_direction_reverse() {
        assert_eq!(Direction::Up.reverse(), Direction::Down);
        for d in DIRECTIONS {
            assert_eq!(d.reverse().reverse(), d);
        }
    }

    #[test]
    fn test_direction8_rotate_cw() {
        assert_eq!(
            EightWayDirection::Up.rotate_cw(),
            EightWayDirection::UpRight
        );
        for d in EIGHT_WAY_DIRECTIONS {
            assert_eq!(
                d.rotate_cw()
                    .rotate_cw()
                    .rotate_cw()
                    .rotate_cw()
                    .rotate_cw()
                    .rotate_cw()
                    .rotate_cw()
                    .rotate_cw(),
                d
            );
        }
    }

    #[test]
    fn test_direction8_rotate_ccw() {
        assert_eq!(
            EightWayDirection::Up.rotate_ccw(),
            EightWayDirection::UpLeft
        );
        for d in EIGHT_WAY_DIRECTIONS {
            assert_eq!(
                d.rotate_ccw()
                    .rotate_ccw()
                    .rotate_ccw()
                    .rotate_ccw()
                    .rotate_ccw()
                    .rotate_ccw()
                    .rotate_ccw()
                    .rotate_ccw(),
                d
            );
        }
    }

    #[test]
    fn test_direction8_reverse() {
        assert_eq!(
            EightWayDirection::UpLeft.reverse(),
            EightWayDirection::DownRight
        );
        for d in EIGHT_WAY_DIRECTIONS {
            assert_eq!(d.reverse().reverse(), d);
        }
    }
}
