use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
    fmt::Display,
};

use crate::utils::{
    answers::Answer, direction::ORTHOGONAL, grid::Grid, iters::ChunkOps, parse::ParseOps,
    point::Point,
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

pub fn part2(input: &str) -> Answer {
    let (points, dimensions, count) = parse(input);
    let mut grid = Grid::new(dimensions.0 as i32, dimensions.1 as i32, Memory::Unknown);

    let mut corupted_count = count;
    for &ele in points.iter().take(count) {
        grid[ele] = Memory::Corropted;
    }
    let mut last_corrupted: Point<i32>;
    let mut queue = BinaryHeap::new();
    loop {
        queue.clear();

        last_corrupted = points[corupted_count];
        grid[last_corrupted] = Memory::Corropted;
        corupted_count += 1;

        for y in 0..grid.height {
            for x in 0..grid.width {
                if let Memory::Distance(_) = grid[Point::new(x, y)] {
                    grid[Point::new(x, y)] = Memory::Unknown;
                }
            }
        }

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
        if last == Memory::Unknown {
            return format!("{},{}", last_corrupted.x, last_corrupted.y).into();
        }
    }
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
