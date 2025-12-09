use std::collections::{HashSet, VecDeque};

use crate::utils::{
    answers::Answer,
    iters::ChunkOps,
    parse::ParseOps,
    point::{Point, DOWN, LEFT, RIGHT, UP},
};

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

    let size = tiles.iter().fold((0, 0), |(max_x, max_y), tile| {
        (max_x.max(tile.x), max_y.max(tile.y))
    });
    let size = (size.0 + 2, size.1 + 2);

    let mut set = HashSet::new();
    let mut void = HashSet::new();

    let mut cursor = *tiles.last().unwrap();

    for tile in tiles.iter() {
        let direction = match (cursor.x, cursor.y, tile.x, tile.y) {
            (x1, y1, x2, y2) if x1 < x2 && y1 == y2 => RIGHT,
            (x1, y1, x2, y2) if x1 > x2 && y1 == y2 => LEFT,
            (x1, y1, x2, y2) if x1 == x2 && y1 < y2 => DOWN,
            (x1, y1, x2, y2) if x1 == x2 && y1 > y2 => UP,
            _ => unreachable!(),
        };

        while cursor != *tile {
            set.insert(cursor);
            match direction {
                RIGHT => cursor = Point::new(cursor.x + 1, cursor.y),
                LEFT => cursor = Point::new(cursor.x - 1, cursor.y),
                DOWN => cursor = Point::new(cursor.x, cursor.y + 1),
                UP => cursor = Point::new(cursor.x, cursor.y - 1),
                _ => unreachable!(),
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(Point::new(0u64, 0u64));
    queue.push_back(Point::new(size.0, size.1));
    void.insert(Point::new(0u64, 0u64));
    void.insert(Point::new(size.0, size.1));

    while let Some(point) = queue.pop_front() {
        for direction in [RIGHT, LEFT, DOWN, UP] {
            let new_point = match direction {
                RIGHT => Point::new(point.x + 1, point.y),
                LEFT => Point::new(point.x.saturating_sub(1), point.y),
                DOWN => Point::new(point.x, point.y + 1),
                UP => Point::new(point.x, point.y.saturating_sub(1)),
                _ => unreachable!(),
            };
            if !void.contains(&new_point)
                && !set.contains(&new_point)
                && new_point.x <= size.0
                && new_point.y <= size.1
            {
                queue.push_back(new_point);
                void.insert(new_point);
            }
        }
    }

    let mut area = 0;
    for (i, tile1) in tiles.iter().enumerate() {
        for (j, tile2) in tiles.iter().enumerate().skip(i + 1) {
            if !is_valid(tile1, tile2, &void) {
                continue;
            }
            let dx = tile1.x.abs_diff(tile2.x) + 1;
            let dy = tile1.y.abs_diff(tile2.y) + 1;
            println!("{:>2}: {:?} {:?}", dx * dy, tile1, tile2);
            area = area.max(dx * dy);
        }
    }

    // let mut vv: Vec<_> = void.into_iter().collect();
    // vv.sort_unstable();
    // for v in &vv {
    //     println!("{:?}", v);
    // }

    area.into()
}

fn is_valid(point1: &Point<u64>, point2: &Point<u64>, void: &HashSet<Point<u64>>) -> bool {
    let right_x = point1.x.max(point2.x);
    let left_x = point1.x.min(point2.x);
    let top_y = point1.y.max(point2.y);
    let bottom_y = point1.y.min(point2.y);

    if right_x == left_x || top_y == bottom_y {
        return true;
    }

    for x in left_x..=right_x {
        if void.contains(&Point::new(x, top_y)) || void.contains(&Point::new(x, bottom_y)) {
            return false;
        }
    }

    for y in bottom_y..=top_y {
        if void.contains(&Point::new(left_x, y)) || void.contains(&Point::new(right_x, y)) {
            return false;
        }
    }

    true
}

fn min_max(point1: &Point<u64>, point2: &Point<u64>) -> (Point<u64>, Point<u64>) {
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
