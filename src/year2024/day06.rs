// use std::{
// process::Command,
// thread,
// time::Duration,
// };

use rayon::prelude::*;

use crate::utils::{answers::Answer, direction::Direction, point::Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Wall,
    Empty,
    Visited(u8),
    Player(Direction),
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

impl From<Cell> for char {
    fn from(value: Cell) -> Self {
        match value {
            Cell::Wall => '#',
            Cell::Empty => '.',
            Cell::Visited(_) => 'X',
            Cell::Player(direction) => direction.into(),
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

type Grid = Vec<Vec<Cell>>;

trait GridOps {
    fn copy(&self) -> Self;
    fn positions(&self, predicate: impl Fn(Cell) -> bool) -> Vec<Point<i32>>;
    fn set(&mut self, _: Point<i32>, cell: Cell);
    fn at(&self, point: Point<i32>) -> Option<Cell>;

    fn player_position(&self) -> Option<Point<i32>>;

    fn count(&self, predicate: impl Fn(Cell) -> bool) -> u32;
}

impl GridOps for Grid {
    fn at(&self, Point { x, y }: Point<i32>) -> Option<Cell> {
        if x < 0 || y < 0 {
            return None;
        }
        let (x, y) = (x as usize, y as usize);
        self.get(y).and_then(|row| row.get(x).copied())
    }

    fn set(&mut self, Point { x, y }: Point<i32>, cell: Cell) {
        if x < 0 || y < 0 {
            return;
        }
        let (x, y) = (x as usize, y as usize);

        self[y][x] = cell;
    }

    fn count(&self, predicate: impl Fn(Cell) -> bool) -> u32 {
        self.iter().flatten().filter(|f| predicate(**f)).count() as u32
    }

    fn player_position(&self) -> Option<Point<i32>> {
        for y in 0..self.len() {
            for x in 0..self[y].len() {
                if let Cell::Player(_) = self.at(Point::new(x as i32, y as i32))? {
                    return Some(Point::new(x as i32, y as i32));
                }
            }
        }
        None
    }

    fn positions(&self, predicate: impl Fn(Cell) -> bool) -> Vec<Point<i32>> {
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

// fn clear_console() {
//     if cfg!(target_os = "windows") {
//         Command::new("cmd").args(&["/c", "cls"]).status().unwrap();
//     } else {
//         Command::new("clear").status().unwrap();
//     }
// }
//
// fn move_to_the_top_left() {
//     print!("\x1B[1;1H");
// }
//
// fn displaygrid(grid: &Grid) {
//     let gridStr: String = grid
//         .iter()
//         .map(|line| line.iter().map(|&c| char::from(c)).collect::<String>())
//         .collect::<Vec<_>>()
//         .join("\n");
//     println!("{gridStr}\n");
// }

fn calculate_default_path(grid: &Grid) -> Grid {
    let mut grid = grid.copy();
    let mut player_position = grid.player_position().unwrap();

    // let delay = Duration::from_millis(10);
    // clear_console();
    // displaygrid(&grid);
    // thread::sleep(delay);

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

        // move_to_the_top_left();
        // displaygrid(&grid);
        // thread::sleep(delay);
    }
    grid
}

#[derive(Clone)]
pub struct Player {
    position: Option<Point<i32>>,
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
