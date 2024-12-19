use colored::Colorize;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
    fmt::Display,
};

use crate::utils::{
    answers::Answer,
    direction::ORTHOGONAL,
    grid::Grid,
    iters::ChunkOps,
    parse::ParseOps,
    point::{Point, DIAGONAL},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Memory {
    Unknown,
    Distance(u32),
    Corropted,
}

impl Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Memory::Unknown => write!(f, "."),
            Memory::Distance(d) => write!(f, "{}", d),
            Memory::Corropted => write!(f, "#"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    point: Point<i32>,
    cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}

fn print_memory(grid: &Grid<Memory>) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            print!("{:<3} ", grid[Point::new(x, y)]);
        }
        println!();
    }
}

pub fn part1(input: &str) -> Answer {
    let (points, dimensions, count) = parse(input);
    let mut grid = Grid::new(dimensions.0 as i32, dimensions.1 as i32, Memory::Unknown);

    for ele in points.into_iter().take(count) {
        grid[ele] = Memory::Corropted;
    }

    let mut queue = BinaryHeap::new();

    queue.push(Reverse(Node {
        point: Point::new(0, 0),
        cost: 0,
    }));
    grid[Point::new(0, 0)] = Memory::Distance(0);

    while let Some(Reverse(Node { point, cost })) = queue.pop() {
        for direction in ORTHOGONAL {
            let next = point + direction.into();
            if !grid.contains(next) {
                continue;
            }
            let next_cost = cost + 1;
            match grid[next] {
                Memory::Unknown => {
                    grid[next] = Memory::Distance(next_cost);
                    queue.push(Reverse(Node {
                        point: next,
                        cost: next_cost,
                    }));
                }
                Memory::Distance(current) => {
                    if next_cost < current {
                        grid[next] = Memory::Distance(next_cost);
                        queue.push(Reverse(Node {
                            point: next,
                            cost: next_cost,
                        }));
                    }
                }
                Memory::Corropted => (),
            }
        }
    }

    let last = grid[Point::new(grid.width - 1, grid.height - 1)];

    match last {
        Memory::Distance(distance) => distance.into(),
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    WallUnknown,
    WallNorth,
    WallSouth,
    WallBreaker,
}

fn print_cells(grid: &Grid<Cell>) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            print!("{} ", grid[Point::new(x, y)]);
        }
        println!();
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::WallUnknown => write!(f, "#"),
            Cell::WallNorth => write!(f, "^"),
            Cell::WallSouth => write!(f, "v"),
            Cell::WallBreaker => write!(f, "{}", "X".red()),
        }
    }
}

pub fn part2(input: &str) -> Answer {
    let (points, dimensions, count) = parse(input);
    let mut grid = Grid::new(dimensions.0 as i32, dimensions.1 as i32, Cell::Empty);

    let mut todo = VecDeque::new();
    for (i, point) in points.into_iter().enumerate() {
        todo.push_back(point);
        while let Some(point) = todo.pop_front() {
            let mut cell = match point {
                Point { x, y: 0 } if x != 0 => Cell::WallNorth,
                Point { x, y } if x != grid.width - 1 && y == grid.height - 1 => Cell::WallSouth,
                Point { x: 0, y } if y != 0 => Cell::WallSouth,
                Point { x, y } if x == grid.width - 1 && y != grid.height - 1 => Cell::WallNorth,
                _ => Cell::WallUnknown,
            };

            for vec in DIAGONAL {
                let next = point + vec;
                if !grid.contains(next) {
                    continue;
                }
                let next = grid[next];
                cell = match (cell, next) {
                    (Cell::WallUnknown, Cell::WallNorth) => Cell::WallNorth,
                    (Cell::WallUnknown, Cell::WallSouth) => Cell::WallSouth,
                    (Cell::WallNorth, Cell::WallSouth) => {
                        // grid[point] = Cell::WallBreaker;
                        // print_cells(&grid);
                        return format!("{},{}", point.x, point.y).into();
                    }
                    (Cell::WallSouth, Cell::WallNorth) => {
                        // grid[point] = Cell::WallBreaker;
                        // print_cells(&grid);
                        return format!("{},{}", point.x, point.y).into();
                    }
                    _ => cell,
                };
            }
            grid[point] = cell;
            if cell == Cell::WallNorth || cell == Cell::WallSouth {
                for vec in DIAGONAL {
                    if !grid.contains(point + vec) || grid[point + vec] != Cell::WallUnknown {
                        continue;
                    }
                    todo.push_back(point + vec);
                }
            }
        }
    }
    0.into()
}

fn parse(input: &str) -> (Vec<Point<i32>>, (u32, u32), usize) {
    let points: Vec<_> = input
        .iter_unsigned::<u32>()
        .chunk::<2>()
        .map(|[x, y]| Point::new(x as i32, y as i32))
        .collect();
    let (dimensions, count) = if points.iter().any(|p| p.x > 6 || p.y > 6) {
        ((71, 71), 1024)
    } else {
        ((7, 7), 12)
    };

    (points, dimensions, count)
}
