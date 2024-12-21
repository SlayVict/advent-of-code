use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use num::{traits::NumAssign, Integer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point<T: Integer> {
    pub x: T,
    pub y: T,
}
impl<T> Point<T>
where
    T: Integer + Copy + NumAssign,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> AddAssign for Point<T>
where
    T: Integer + Copy + NumAssign,
{
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T> Add for Point<T>
where
    T: Integer + Copy + NumAssign,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> SubAssign for Point<T>
where
    T: Integer + Copy + NumAssign,
{
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T> Sub for Point<T>
where
    T: Integer + Copy + NumAssign,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul<T> for Point<T>
where
    T: Integer + Copy + NumAssign,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> MulAssign<T> for Point<T>
where
    T: Integer + Copy + NumAssign,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T> Div<T> for Point<T>
where
    T: Integer + Copy + NumAssign,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> DivAssign<T> for Point<T>
where
    T: Integer + Copy + NumAssign,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T> Rem<T> for Point<T>
where
    T: Integer + Copy + NumAssign,
{
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        Self {
            x: self.x % rhs,
            y: self.y % rhs,
        }
    }
}

impl<T> RemAssign<T> for Point<T>
where
    T: Integer + Copy + NumAssign,
{
    fn rem_assign(&mut self, rhs: T) {
        self.x %= rhs;
        self.y %= rhs;
    }
}

impl<T> From<(T, T)> for Point<T>
where
    T: Integer + Copy + NumAssign,
{
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

pub const ORIGIN: Point<i32> = Point { x: 0, y: 0 };
pub const UP: Point<i32> = Point { x: 0, y: -1 };
pub const DOWN: Point<i32> = Point { x: 0, y: 1 };
pub const LEFT: Point<i32> = Point { x: -1, y: 0 };
pub const RIGHT: Point<i32> = Point { x: 1, y: 0 };
// pub const ORTHOGONAL: [Point<i32>; 4] = [UP, DOWN, LEFT, RIGHT];
// Left to right and top to bottom.
pub const DIAGONAL: [Point<i32>; 8] = [
    Point { x: -1, y: -1 },
    UP,
    Point { x: 1, y: -1 },
    LEFT,
    RIGHT,
    Point { x: -1, y: 1 },
    DOWN,
    Point { x: 1, y: 1 },
];
