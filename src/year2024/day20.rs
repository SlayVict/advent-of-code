use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    i32,
};

use colored::Colorize;

use crate::utils::{answers::Answer, direction::ORTHOGONAL, grid::Grid, point::Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct QueueNode {
    point: Point<i32>,
    cost: i32,
}

impl QueueNode {
    fn new(point: Point<i32>, cost: i32) -> Self {
        Self { point, cost }
    }
}

impl Ord for QueueNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for QueueNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn print_map(layout: &Grid<u8>, distances: &Grid<i32>) {
    for y in 0..layout.height {
        for x in 0..layout.width {
            let point = Point::new(x, y);
            let distance = distances[point];
            let c = match layout[point] {
                b'S' => format!(
                    "{}",
                    format!("{:>3}", distance.to_string()).as_str().green()
                ),
                b'E' => format!("{}", format!("{:>3}", distance.to_string()).as_str().red()),
                b'#' => "  #".to_string(),
                b'.' => format!("{:>3}", distance.to_string()),
                _ => "  #".to_string(),
            };
            print!("{c} ");
        }
        println!();
    }
}

fn get_manhatten_distance(dist: i32) -> Vec<Point<i32>> {
    let mut points = HashSet::new();

    for x in 0..=dist {
        let y = dist - x;
        points.insert(Point::new(x, y));
        points.insert(Point::new(-x, y));
        points.insert(Point::new(-x, -y));
        points.insert(Point::new(x, -y));
    }

    points.into_iter().collect()
}

pub fn part1(input: &str) -> Answer {
    let map = Grid::parse(input);
    let start = map.find(b'S').unwrap();
    let end = map.find(b'E').unwrap();

    let mut distances = map.same_size_with(i32::MAX);
    let mut queue = BinaryHeap::new();
    queue.push(QueueNode::new(end, 0));
    distances[end] = 0;

    while let Some(QueueNode { point, cost }) = queue.pop() {
        for direction in ORTHOGONAL {
            let next = point + direction.into();
            if map.contains(next) && map[next] != b'#' && distances[next] > cost + 1 {
                distances[next] = cost + 1;
                queue.push(QueueNode::new(next, cost + 1));
            }
        }
    }

    // print_map(&map, &distances);

    let cheats = get_manhatten_distance(2);
    let mut hashmap = HashMap::new();
    let mut count = 0;

    let mut dfs_visited = map.same_size_with(i32::MAX);
    let mut queue = BinaryHeap::new();
    queue.push(QueueNode::new(start, 0));
    dfs_visited[start] = 0;

    while let Some(QueueNode { point, cost }) = queue.pop() {
        for direction in ORTHOGONAL {
            let next = point + direction.into();
            if map.contains(next) && map[next] != b'#' && dfs_visited[next] > cost + 1 {
                dfs_visited[next] = cost + 1;
                queue.push(QueueNode::new(next, cost + 1));
            }
        }
        let cost = distances[point];
        if cost == i32::MAX {
            continue;
        }

        for &cheat in cheats.iter() {
            let next = point + cheat;
            if !distances.contains(next) {
                continue;
            }

            let next = distances[next];

            if next == i32::MAX {
                continue;
            }

            let cheat_savings = next - cost - 2;

            if cheat_savings > 0 {
                // println!("{cost}, {next}, {point:?}");
                *hashmap.entry(cheat_savings).or_insert(0) += 1
            }

            if cheat_savings >= 100 {
                count += 1;
            }
        }
    }

    // let mut sorted_hashmap: Vec<_> = hashmap.into_iter().collect();
    // sorted_hashmap.sort_by_key(|item| item.0);
    //
    // for (saving, count) in sorted_hashmap {
    //     println!("{count}: {saving}");
    // }

    count.into()
}

pub fn part2(input: &str) -> Answer {
    let map = Grid::parse(input);
    let start = map.find(b'S').unwrap();
    let end = map.find(b'E').unwrap();

    let mut distances = map.same_size_with(i32::MAX);
    let mut queue = BinaryHeap::new();
    queue.push(QueueNode::new(end, 0));
    distances[end] = 0;

    while let Some(QueueNode { point, cost }) = queue.pop() {
        for direction in ORTHOGONAL {
            let next = point + direction.into();
            if map.contains(next) && map[next] != b'#' && distances[next] > cost + 1 {
                distances[next] = cost + 1;
                queue.push(QueueNode::new(next, cost + 1));
            }
        }
    }

    // print_map(&map, &distances);

    let cheats: Vec<_> = (2..=20).map(get_manhatten_distance).flatten().collect();
    let mut hashmap = HashMap::new();
    let mut count = 0;

    let mut dfs_visited = map.same_size_with(i32::MAX);
    let mut queue = BinaryHeap::new();
    queue.push(QueueNode::new(start, 0));
    dfs_visited[start] = 0;

    while let Some(QueueNode { point, cost }) = queue.pop() {
        for direction in ORTHOGONAL {
            let next = point + direction.into();
            if map.contains(next) && map[next] != b'#' && dfs_visited[next] > cost + 1 {
                dfs_visited[next] = cost + 1;
                queue.push(QueueNode::new(next, cost + 1));
            }
        }
        let cost = distances[point];
        if cost == i32::MAX {
            continue;
        }

        for &cheat in cheats.iter() {
            let next = point + cheat;
            if !distances.contains(next) {
                continue;
            }

            let next = distances[next];

            if next == i32::MAX {
                continue;
            }

            let cheat_savings = next - cost - cheat.x.abs() - cheat.y.abs();

            if cheat_savings > 0 {
                // println!("{cost}, {next}, {point:?}");
                *hashmap.entry(cheat_savings).or_insert(0) += 1
            }

            if cheat_savings >= 100 {
                count += 1;
            }
        }
    }

    // let mut sorted_hashmap: Vec<_> = hashmap.into_iter().collect();
    // sorted_hashmap.sort_by_key(|item| item.0);
    //
    // for (saving, count) in sorted_hashmap {
    //     println!("{count}: {saving}");
    // }

    count.into()
}
