use std::{collections::VecDeque, default, fmt::Display};

use tokio::time;

use crate::utils::{
    answers::Answer,
    grid::{print_grid, Grid},
    point::{Point, DOWN, LEFT, RIGHT},
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Cell {
    Source,
    Void,
    Beam,
    Splitter,
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Cell::Void,
            b'|' => Cell::Beam,
            b'^' => Cell::Splitter,
            b'S' => Cell::Source,
            _ => panic!("Invalid cell type"),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Source => write!(f, "S"),
            Cell::Void => write!(f, "."),
            Cell::Beam => write!(f, "|"),
            Cell::Splitter => write!(f, "^"),
        }
    }
}

pub fn part1(input: &str) -> Answer {
    let (mut map, src) = parse(input);
    let mut queue = VecDeque::new();
    queue.push_back(src + DOWN);

    let mut splits = 0;

    while let Some(point) = queue.pop_front() {
        if !map.contains(point) {
            continue;
        }
        let cell = map[point];
        match cell {
            Cell::Void => {
                map[point] = Cell::Beam;
                queue.push_back(point + DOWN);
            }
            Cell::Splitter => {
                // map[point] = Cell::Beam;
                queue.push_back(point + RIGHT);
                queue.push_back(point + LEFT);
                splits += 1;
            }
            _ => {}
        }
    }

    splits.into()
}

pub fn part2(input: &str) -> Answer {
    let (mut map, src) = parse(input);

    let mut timelines = vec![0; map.width as usize];
    timelines[src.x as usize] = 1;

    for y in 2..map.height {
        let line = (y) / 2;
        for x in (src.x - line)..(src.x + line + 1) {
            let count = timelines[x as usize];

            let cell = map[Point::new(x, y)];

            if count > 0 && cell == Cell::Splitter {
                timelines[x as usize] = 0;
                timelines[(x - 1) as usize] += count;
                timelines[(x + 1) as usize] += count;
            }
        }
    }

    timelines.iter().sum::<u64>().into()
}

fn parse(input: &str) -> (Grid<Cell>, Point<i32>) {
    let input = Grid::parse(input);
    let mut map = input.same_size_with(Cell::Void);

    let mut source = Point::new(0, 0);
    for (point, char) in input.iter() {
        map[point] = Cell::from(*char);
        if *char == b'S' {
            source = point;
        }
    }

    (map, source)
}
