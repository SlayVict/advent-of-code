use std::ops::{Add, AddAssign};

use rayon::prelude::*;

use crate::utils::answers::Answer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Wall,
    Empty,
    Visited(u8),
    Player(Direction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '.' => Self::Empty,
            'X' => Self::Visited(0b1111),
            '^' | '<' | 'v' | '>' => Self::Player(Direction::from(c)),
            _ => panic!("Unkown map character {c}"),
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

impl Direction {
    fn rotate_clockwise(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl From<Direction> for u8 {
    fn from(d: Direction) -> Self {
        match d {
            Direction::Up => 0b1000,
            Direction::Right => 0b0100,
            Direction::Down => 0b0010,
            Direction::Left => 0b0001,
        }
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

type Grid = Vec<Vec<Cell>>;

trait GridOps {
    fn copy(&self) -> Self;
    fn positions(&self, predicate: impl Fn(Cell) -> bool) -> Vec<Point>;
    fn set(&mut self, _: Point, cell: Cell);
    fn at(&self, point: Point) -> Option<Cell>;

    fn player_position(&self) -> Option<Point>;

    fn count(&self, predicate: impl Fn(Cell) -> bool) -> u32;
}

impl GridOps for Grid {
    fn at(&self, Point { x, y }: Point) -> Option<Cell> {
        if x < 0 || y < 0 {
            return None;
        }
        let (x, y) = (x as usize, y as usize);
        self.get(y).and_then(|row| row.get(x).copied())
    }

    fn set(&mut self, Point { x, y }: Point, cell: Cell) {
        if x < 0 || y < 0 {
            return;
        }
        let (x, y) = (x as usize, y as usize);

        self[y][x] = cell;
    }

    fn count(&self, predicate: impl Fn(Cell) -> bool) -> u32 {
        self.iter().flatten().filter(|f| predicate(**f)).count() as u32
    }

    fn player_position(&self) -> Option<Point> {
        for y in 0..self.len() {
            for x in 0..self[y].len() {
                if let Cell::Player(_) = self.at(Point::new(x as i32, y as i32))? {
                    return Some(Point::new(x as i32, y as i32));
                }
            }
        }
        None
    }

    fn positions(&self, predicate: impl Fn(Cell) -> bool) -> Vec<Point> {
        let mut positions = Vec::new();
        for y in 0..self.len() {
            for x in 0..self[y].len() {
                if predicate(self.at(Point::new(x as i32, y as i32)).unwrap()) {
                    positions.push(Point::new(x as i32, y as i32));
                }
            }
        }
        positions
    }

    fn copy(&self) -> Self {
        self.iter()
            .map(|row| row.iter().copied().collect())
            .collect()
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<Direction> for Point {
    fn from(d: Direction) -> Self {
        match d {
            Direction::Up => Point::new(0, -1),
            Direction::Right => Point::new(1, 0),
            Direction::Down => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
        }
    }
}

fn calculate_default_path(grid: &Grid) -> Grid {
    let mut grid = grid.copy();
    let mut player_position = grid.player_position().unwrap();

    while let Some(Cell::Player(direction)) = grid.at(player_position) {
        let next_position = player_position + direction.into();
        match grid.at(next_position) {
            Some(Cell::Empty) => {
                grid.set(player_position, Cell::Visited(direction.into()));
                grid.set(next_position, Cell::Player(direction));
                player_position = next_position;
            }
            Some(Cell::Visited(d)) => {
                grid.set(player_position, Cell::Visited(d | (u8::from(direction))));
                grid.set(next_position, Cell::Player(direction));
                player_position = next_position;
            }
            Some(Cell::Wall) => {
                let new_direction = direction.rotate_clockwise();
                grid.set(player_position, Cell::Player(new_direction));
            }
            _ => {
                grid.set(player_position, Cell::Visited(direction.into()));
            }
        }
    }
    grid
}

#[derive(Clone)]
struct Player {
    position: Option<Point>,
    direction: Direction,
}

pub fn loop_detection(grid: &mut Grid, player: &mut Player) -> bool {
    while let Some(position) = player.position {
        let next_position = position + player.direction.into();
        match grid.at(next_position) {
            Some(Cell::Empty) => {
                grid.set(position, Cell::Visited(player.direction.into()));
                player.position = Some(next_position);
            }
            Some(Cell::Visited(_)) => {
                let mut d: u8 = 0;
                if let Cell::Visited(dddd) = grid.at(position).unwrap() {
                    d = dddd;
                };
                if d & (u8::from(player.direction)) != 0 {
                    return true;
                }
                grid.set(position, Cell::Visited(d | (u8::from(player.direction))));
                player.position = Some(next_position);
            }
            Some(Cell::Wall) => {
                let new_direction = player.direction.rotate_clockwise();
                player.direction = new_direction;
            }
            _ => {
                grid.set(position, Cell::Visited(player.direction.into()));
                player.position = None;
            }
        }
    }
    false
}

pub fn part1(input: &str) -> Answer {
    let grid = parse(input);
    calculate_default_path(&grid)
        .count(|c| matches!(c, Cell::Visited(_)))
        .into()
}

pub fn part2(input: &str) -> Answer {
    let grid = parse(input);
    let player_position = grid.player_position().unwrap();
    let player_cell = grid.at(player_position).unwrap();
    let player_direction = match player_cell {
        Cell::Player(direction) => direction,
        _ => panic!(),
    };

    let player = Player {
        position: Some(player_position),
        direction: player_direction,
    };

    let solved = calculate_default_path(&grid);
    let solved_positions = solved.positions(|c| matches!(c, Cell::Visited(_)));
    let solved_positions: Vec<_> = solved_positions
        .iter()
        .filter(|&p| *p != player_position)
        .collect();

    solved_positions
        .par_iter()
        .map(|&p| {
            let mut grid = grid.copy();
            grid.set(*p, Cell::Wall);
            loop_detection(&mut grid, &mut player.clone())
        })
        .filter(|&b| b)
        .count()
        .into()
}

pub fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|l| l.chars().map(Cell::from).collect())
        .collect()
}
