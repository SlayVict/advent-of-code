use std::collections::{HashMap, HashSet};

use crate::utils::{answers::Answer, point::Point};

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

    let mut antinodes = HashSet::new();

    for (_, antenas) in map {
        for (i, &antena1) in antenas.iter().enumerate() {
            for &antena2 in antenas[..i].iter().chain(antenas[i + 1..].iter()) {
                let dif = antena2 - antena1;
                let antinode = antena2 + dif;

                if dm.contain(antinode) {
                    antinodes.insert(antinode);
                }
            }
        }
    }
    antinodes.len().into()
}

pub fn part2(input: &str) -> Answer {
    let (map, dm) = parse(input);

    let mut antinodes = HashSet::new();

    for (_, antenas) in map {
        for (i, &antena1) in antenas.iter().enumerate() {
            for &antena2 in antenas[..i].iter().chain(antenas[i + 1..].iter()) {
                let dif = antena2 - antena1;
                let mut antinode = antena2;
                while dm.contain(antinode) {
                    antinodes.insert(antinode);
                    antinode = antinode + dif;
                }
            }
        }
    }
    antinodes.len().into()
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
