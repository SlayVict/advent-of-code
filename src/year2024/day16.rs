use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

use crate::utils::{
    answers::Answer,
    direction::{Direction, ORTHOGONAL},
    grid::Grid,
    point::Point,
};

#[derive(Debug, PartialEq, Eq)]
struct Node {
    point: Point<i32>,
    weight: Option<u32>,
    direction: Direction,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1(input: &str) -> Answer {
    let str_map = Grid::parse(input);
    let mut prices = str_map.same_size_with(None);
    let start = str_map.find(b'S').unwrap();
    let end = str_map.find(b'E').unwrap();

    prices[start] = Some(0);

    let mut heap: BinaryHeap<Reverse<Node>> = BinaryHeap::new();

    heap.push(Reverse(Node {
        point: start,
        weight: Some(0),
        direction: Direction::Right,
    }));

    while let Some(Reverse(Node {
        point,
        weight,
        direction,
    })) = heap.pop()
    {
        for d in ORTHOGONAL {
            let next = point + d.into();
            let mut weight = weight.unwrap() + 1;
            if direction != d {
                weight += 1000;
            }
            if str_map.contains(next) && str_map[next] != b'#' {
                if prices[next].is_none() || prices[next].unwrap() > weight {
                    prices[next] = Some(weight);
                    heap.push(Reverse(Node {
                        point: next,
                        weight: Some(weight),
                        direction: d,
                    }));
                }
            }
        }
    }
    prices[end].unwrap().into()
}

pub fn part2(input: &str) -> Answer {
    let str_map = Grid::parse(input);
    let start = str_map.find(b'S').unwrap();
    let end = str_map.find(b'E').unwrap();

    let mut seen = HashMap::new();

    let mut todo: BinaryHeap<Reverse<Node>> = BinaryHeap::new();

    todo.push(Reverse(Node {
        point: start,
        weight: Some(0),
        direction: Direction::Right,
    }));

    seen.insert((start, Direction::Right), 0);
    let mut lovest = u32::MAX;

    while let Some(Reverse(Node {
        point,
        weight: cost,
        direction,
    })) = todo.pop()
    {
        if point == end {
            lovest = lovest.min(cost.unwrap());
            continue;
        }

        let left = direction.counter_clockwise();
        let right = direction.clockwise();
        let next = [
            (point + direction.into(), direction, cost.unwrap() + 1),
            (point, left, cost.unwrap() + 1000),
            (point, right, cost.unwrap() + 1000),
        ];
        for (next_point, next_direction, cost) in next {
            if str_map[next_point] != b'#' {
                let key = (next_point, next_direction);
                let existing = *seen.get(&key).unwrap_or(&u32::MAX);
                if existing > cost {
                    todo.push(Reverse(Node {
                        point: next_point,
                        weight: Some(cost),
                        direction: next_direction,
                    }));
                    seen.insert(key, cost);
                }
            }
        }
    }

    let mut todo = VecDeque::new();
    let mut path = HashSet::new();

    for direction in ORTHOGONAL {
        if let Some(&cost) = seen.get(&(end, direction)) {
            if cost == lovest {
                todo.push_back((end, direction, lovest));
            }
        }
    }

    while let Some((point, direction, cost)) = todo.pop_front() {
        path.insert(point);

        if point == start {
            continue;
        }
        let left = direction.counter_clockwise();
        let right = direction.clockwise();
        let next = [
            (point - direction.into(), direction, cost - 1),
            (point, left, cost - 1000),
            (point, right, cost - 1000),
        ];
        for (next_position, next_direction, next_cost) in next {
            let key = (next_position, next_direction);

            if let Some(&amount) = seen.get(&key) {
                if amount == next_cost {
                    todo.push_back((next_position, next_direction, next_cost));
                }
            }
        }
    }

    path.len().into()
}
