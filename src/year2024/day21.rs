//thank to https://raw.githubusercontent.com/fdumontmd/adventofcode/refs/heads/master/2024/day-21/src/main.rs
use std::{cmp::Reverse, collections::BinaryHeap, fmt::Display};

use crate::utils::{answers::Answer, direction::ORTHOGONAL, grid::Grid, point::Point};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
enum Button {
    Up,
    Right,
    Down,
    Left,
    Press,
}

impl Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.as_char();
        write!(f, "{c}")
    }
}

impl TryFrom<char> for Button {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '^' => Button::Up,
            '>' => Button::Right,
            'v' => Button::Down,
            '<' => Button::Left,
            'A' => Button::Press,
            _ => return Err(format!("'{value}' not a valid button")),
        })
    }
}

impl TryFrom<u8> for Button {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'^' => Button::Up,
            b'>' => Button::Right,
            b'v' => Button::Down,
            b'<' => Button::Left,
            b'A' => Button::Press,
            _ => return Err(format!("'{value}' not a valid button")),
        })
    }
}

impl Button {
    fn from_movement(from: Point<i32>, to: Point<i32>) -> Self {
        match (to.x.cmp(&from.x), to.y.cmp(&from.y)) {
            (std::cmp::Ordering::Less, _) => Button::Left,
            (std::cmp::Ordering::Greater, _) => Button::Right,
            (_, std::cmp::Ordering::Less) => Button::Up,
            (_, std::cmp::Ordering::Greater) => Button::Down,
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => Button::Press,
        }
    }

    fn as_char(&self) -> char {
        match self {
            Button::Up => '^',
            Button::Right => '>',
            Button::Down => 'v',
            Button::Left => '<',
            Button::Press => 'A',
        }
    }

    fn as_byte(&self) -> u8 {
        match self {
            Button::Up => b'^',
            Button::Right => b'>',
            Button::Down => b'v',
            Button::Left => b'<',
            Button::Press => b'A',
        }
    }
}

const NUMERIC_PAD: &str = "789
456
123
X0A";

const DIRECTIONAL_PAD: &str = "X^A
<v>";

// what's the cost of pressing a button on the directional pad with `level` levels of indirection
// (level 0 == me)
fn build_press_costs(grid: &Grid<u8>, level: i32) -> [[usize; 5]; 5] {
    use Button::*;
    if level == 0 {
        // uniform of 1 cost for user
        [[1; 5]; 5]
    } else {
        let mut current_press_costs = [[0; 5]; 5];
        let previous_press_costs = build_press_costs(grid, level - 1);

        for from in [Up, Right, Down, Left, Press] {
            // seen does not guarantee that each button will be evaluated just once; just that
            // we'll stop evaluating them quickly
            let mut seen = [false; 5];
            let from_pos = grid.find(from.as_byte()).unwrap();
            let mut queue = BinaryHeap::new();
            queue.push((Reverse(0), from_pos, Press));
            while let Some((Reverse(cost), pos, button)) = queue.pop() {
                let cd = Button::try_from(grid[pos]).unwrap();
                if cost > 0 && button == Press {
                    // first cost will be optimal, so if we have one don't update
                    if current_press_costs[from as usize][cd as usize] == 0 {
                        current_press_costs[from as usize][cd as usize] = cost;
                    }
                } else {
                    queue.push((
                        Reverse(cost + previous_press_costs[button as usize][Press as usize]),
                        pos,
                        Press,
                    ));
                }
                seen[cd as usize] = true;
                for direction in ORTHOGONAL {
                    let n = pos + direction.into();
                    if !grid.contains(n) {
                        continue;
                    }
                    if grid[n] == b'X' {
                        continue;
                    }
                    let d = Button::try_from(grid[n]).unwrap();
                    if seen[d as usize] {
                        continue;
                    }
                    let next_button = Button::from_movement(pos, n);
                    queue.push((
                        Reverse(cost + previous_press_costs[button as usize][next_button as usize]),
                        n,
                        next_button,
                    ));
                }
            }
        }
        current_press_costs
    }
}

// compute shortest path between two numerical buttons taking into account
// the cost of pressing the directional buttons
fn shortest_path(grid: &Grid<u8>, press_costs: [[usize; 5]; 5], from_n: char, to_n: char) -> usize {
    // let from = grid.idx_to_pos(grid.iter().position(|c| *c == from_n).unwrap());
    // let to = grid.idx_to_pos(grid.iter().position(|c| *c == to_n).unwrap());

    let from = grid.find(from_n as u8).unwrap();
    let to = grid.find(to_n as u8).unwrap();

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), from, Button::Press));

    while let Some((Reverse(cost), pos, button)) = queue.pop() {
        if pos == to {
            if button == Button::Press {
                return cost;
            } else {
                queue.push((
                    Reverse(cost + press_costs[button as usize][Button::Press as usize]),
                    pos,
                    Button::Press,
                ));
            }
        } else {
            for direction in ORTHOGONAL {
                let n = pos + direction.into();
                if !grid.contains(n) {
                    continue;
                }
                if grid[n] == b'X' {
                    continue;
                }
                let nd = Button::from_movement(pos, n);
                queue.push((
                    Reverse(cost + press_costs[button as usize][nd as usize]),
                    n,
                    nd,
                ));
            }
        }
    }

    unreachable!()
}

fn part1_one_line(grid: &Grid<u8>, press_costs: [[usize; 5]; 5], line: &str) -> usize {
    // not good but let's get this working first
    let moves = format!("A{line}");
    let steps = moves
        .as_bytes()
        .windows(2)
        .map(|w| shortest_path(grid, press_costs, w[0] as char, w[1] as char))
        .sum::<usize>();

    if let Some(n) = line.strip_suffix("A") {
        let n: usize = n.parse().unwrap();
        steps * n
    } else {
        panic!("wrong input {line}")
    }
}

pub fn part1(input: &str) -> Answer {
    let grid = Grid::parse(DIRECTIONAL_PAD);
    let press_costs = build_press_costs(&grid, 2);
    let grid = Grid::parse(NUMERIC_PAD);
    input
        .lines()
        .map(|line| part1_one_line(&grid, press_costs, line))
        .sum::<usize>()
        .into()
}

pub fn part2(input: &str) -> Answer {
    let grid = Grid::parse(DIRECTIONAL_PAD);
    let press_costs = build_press_costs(&grid, 25);
    let grid = Grid::parse(NUMERIC_PAD);
    input
        .lines()
        .map(|line| part1_one_line(&grid, press_costs, line))
        .sum::<usize>()
        .into()
}
