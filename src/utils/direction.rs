use super::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn clockwise(&self) -> Self {
        match self {
            Direction::Up => Self::Right,
            Direction::Right => Self::Down,
            Direction::Down => Self::Left,
            Direction::Left => Self::Up,
        }
    }

    pub fn counter_clockwise(&self) -> Self {
        match self {
            Direction::Up => Self::Left,
            Direction::Right => Self::Up,
            Direction::Down => Self::Right,
            Direction::Left => Self::Down,
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => panic!("Unkown direction character {c}"),
        }
    }
}

impl From<Direction> for char {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        }
    }
}

impl From<Direction> for Point<i32> {
    fn from(d: Direction) -> Self {
        match d {
            Direction::Up => Point::new(0, -1),
            Direction::Right => Point::new(1, 0),
            Direction::Down => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
        }
    }
}

impl From<Point<i32>> for Direction {
    fn from(p: Point<i32>) -> Self {
        match p {
            Point { x, y } if x == 0 && y < 0 => Self::Up,
            Point { x, y } if x == 0 && y > 0 => Self::Down,
            Point { x, y } if x > 0 && y == 0 => Self::Right,
            Point { x, y } if x < 0 && y == 0 => Self::Left,
            p => panic!("Unkown direction point {p:?}"),
        }
    }
}

pub const ORTHOGONAL: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];
