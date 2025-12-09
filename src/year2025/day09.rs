use std::collections::{HashMap, VecDeque};

use num::{traits::NumAssign, Integer};

use crate::utils::{answers::*, grid::Grid, iters::ChunkOps, parse::*, point::*};

const OUTSIDE: i64 = 0;
const INSIDE: i64 = 1;
const UNKNOWN: i64 = 2;

pub fn part1(input: &str) -> Answer {
    let tiles = parse(input);

    let mut area = 0;
    for (i, tile1) in tiles.iter().enumerate() {
        for (j, tile2) in tiles.iter().enumerate().skip(i + 1) {
            let dx = tile1.x.abs_diff(tile2.x) + 1;
            let dy = tile1.y.abs_diff(tile2.y) + 1;
            area = area.max(dx * dy);
        }
    }

    area.into()
}

pub fn part2(input: &str) -> Answer {
    let tiles = parse(input);
    let size = tiles.len();
    let shrink_x = shrink(&tiles, 0);
    let shrink_y = shrink(&tiles, 1);
    let shrunk: Vec<_> = tiles
        .iter()
        .map(|&point| Point::new(shrink_x[&point.x], shrink_y[&point.y]))
        .collect();

    let mut area = 0;
    let mut todo = VecDeque::from([ORIGIN]);
    let mut grid = Grid::new(shrink_x.len() as i32, shrink_y.len() as i32, UNKNOWN);

    for i in 0..size {
        let (p1, p2) = minmax(&shrunk[i], &shrunk[(i + 1) % size]);

        for x in p1.x..p2.x + 1 {
            for y in p1.y..p2.y + 1 {
                grid[Point::new(x, y)] = INSIDE;
            }
        }
    }

    while let Some(point) = todo.pop_front() {
        for next in ORTHOGONAL.map(|o| point + o) {
            if grid.contains(next) && grid[next] == UNKNOWN {
                grid[next] = OUTSIDE;
                todo.push_back(next);
            }
        }
    }

    for y in 1..grid.height {
        for x in 1..grid.width {
            let point = Point::new(x, y);
            let value = i64::from(grid[point] != OUTSIDE);
            grid[point] = value + grid[point + UP] + grid[point + LEFT] - grid[point + UP + LEFT];
        }
    }

    for i in 0..size {
        for j in i + 1..size {
            let (p1, p2) = minmax(&shrunk[i], &shrunk[j]);

            let expected = (p2.x - p1.x + 1) as i64 * (p2.y - p1.y + 1) as i64;
            let actual = grid[Point::new(p2.x, p2.y)]
                - grid[Point::new(p1.x - 1, p2.y)]
                - grid[Point::new(p2.x, p1.y - 1)]
                + grid[Point::new(p1.x - 1, p1.y - 1)];

            if expected == actual {
                let p1 = tiles[i];
                let p2 = tiles[j];
                let dx = p1.x.abs_diff(p2.x) + 1;
                let dy = p1.y.abs_diff(p2.y) + 1;
                area = area.max(dx * dy);
            }
        }
    }

    area.into()
}

fn shrink(tiles: &[Point<u64>], index: usize) -> HashMap<u64, i32> {
    let mut axis: Vec<_> = tiles
        .iter()
        .map(|tile| match index {
            0 => tile.x,
            _ => tile.y,
        })
        .collect();
    axis.push(u64::MIN);
    axis.push(u64::MAX);
    axis.sort_unstable();
    axis.dedup();
    axis.iter()
        .enumerate()
        .map(|(i, &n)| (n, i as i32))
        .collect()
}

#[inline]
fn minmax<T>(point1: &Point<T>, point2: &Point<T>) -> (Point<T>, Point<T>)
where
    T: Integer + Copy + Ord + NumAssign,
{
    let min_x = point1.x.min(point2.x);
    let min_y = point1.y.min(point2.y);
    let max_x = point1.x.max(point2.x);
    let max_y = point1.y.max(point2.y);
    (Point::new(min_x, min_y), Point::new(max_x, max_y))
}

fn parse(input: &str) -> Vec<Point<u64>> {
    input
        .iter_unsigned()
        .chunk::<2>()
        .map(|chunk| Point::new(chunk[0], chunk[1]))
        .collect()
}
