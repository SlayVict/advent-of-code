use std::collections::HashMap;

use crate::utils::{answers::Answer, parse, point::Point};

struct Dimention {
    width: i32,
    height: i32,
}

impl Dimention {
    fn contain(&self, point: Point) -> bool {
        if point.x < 0 || point.x >= self.width {
            return false;
        }
        if point.y < 0 || point.y >= self.height {
            return false;
        }
        true
    }
}

pub fn part1(input: &str) -> Answer {
    let (map, dm) = parse(input);

    Answer::InProgress
}

pub fn part2(input: &str) -> Answer {
    Answer::InProgress
}

fn parse(input: &str) -> (HashMap<char, Vec<Point>>, Dimention) {
    let width: i32 = input.lines().next().unwrap().chars().count() as i32;
    let height: i32 = input.lines().count() as i32;
    let dm = Dimention { width, height };

    let mut map: HashMap<char, Vec<Point>> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                (*map.entry(char).or_default()).push(Point::new(x as i32, y as i32));
            }
        }
    }

    (map, dm)
}
