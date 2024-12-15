use super::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
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