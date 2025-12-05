use std::{
    collections::VecDeque,
    fmt::Display,
    i32::{self, MAX, MIN},
};

use crate::utils::{
    answers::Answer,
    direction::ORTHOGONAL,
    grid::Grid,
    parse::ParseByte,
    point::{Point, DIAGONAL},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    None,
    Roll(i32),
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::None => write!(f, "."),
            Cell::Roll(a) if *a >= 4 => write!(f, "@"),
            Cell::Roll(_) => write!(f, "x"),
        }
    }
}

impl From<u8> for Cell {
    fn from(byte: u8) -> Self {
        match byte {
            b'.' => Cell::None,
            b'@' => Cell::Roll(MAX),
            b'x' => Cell::Roll(MIN),
            _ => panic!("Invalid cell byte"),
        }
    }
}

impl Display for Grid<Cell> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                let cell = self[Point::new(j, i)];
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part1(input: &str) -> Answer {
    let mut input = parse(input);

    count_naighbors(&mut input);

    let removable = input
        .iter()
        .filter(|&(_, cell)| {
            if let Cell::Roll(n) = cell {
                *n < 4
            } else {
                false
            }
        })
        .count();

    // println!("{input}");
    removable.into()
}

pub fn part2(input: &str) -> Answer {
    let mut input = parse(input);

    count_naighbors(&mut input);

    let mut removable = 0;

    let mut to_check = VecDeque::new();

    for i in 0..(input.width * input.height) {
        let point = Point::new(i % input.width, i / input.width);
        let cell = input[point];
        if let Cell::Roll(n) = cell {
            if n < 4 {
                to_check.push_back(point);
            }
        }
    }

    while let Some(point) = to_check.pop_front() {
        let Cell::Roll(n) = input[point] else {
            continue;
        };
        if n >= 4 {
            continue;
        }

        input[point] = Cell::None;
        removable += 1;

        for direction in DIAGONAL {
            let neighbor = point + direction;
            if !input.contains(neighbor) {
                continue;
            }
            if let Cell::Roll(n) = input[neighbor] {
                to_check.push_back(neighbor);
                input[neighbor] = Cell::Roll(n - 1);
            }
        }
    }

    // println!("{input}");
    removable.into()
}

fn parse(input: &str) -> Grid<Cell> {
    let input = Grid::parse(input);
    let mut grid = input.same_size_with(Cell::None);
    for i in 0..input.height {
        for j in 0..input.width {
            let point = Point::new(j, i);
            let cell = input[point];
            grid[point] = cell.into();
        }
    }
    grid
}

fn count_naighbors(grid: &mut Grid<Cell>) {
    for i in 0..(grid.width * grid.height) {
        let point = Point::new(i % grid.width, i / grid.width);
        let cell = grid[point];

        if let Cell::None = cell {
            continue;
        }

        let roll_count: i32 = DIAGONAL
            .iter()
            .map(|&direction| {
                let neighbor = point + direction;
                if !grid.contains(neighbor) {
                    return 0;
                }
                if let Cell::Roll(_) = grid[neighbor] {
                    1
                } else {
                    0
                }
            })
            .sum();

        grid[point] = Cell::Roll(roll_count);
    }
}
