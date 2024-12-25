use std::collections::HashSet;

use crate::utils::answers::Answer;

pub fn part1(input: &str) -> Answer {
    let (locks, keys) = parse(input);

    let mut count = 0;
    for lock in locks {
        for key in &keys {
            if key.iter().zip(lock.iter()).all(|(k, l)| k >= l) {
                count += 1;
            }
        }
    }
    count.into()
}

pub fn part2(input: &str) -> Answer {
    Answer::InProgress
}

fn parse(input: &str) -> (Vec<[usize; 5]>, Vec<[usize; 5]>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for shape in input.split("\n\n") {
        let mut column = [0; 5];
        let shape_c = shape.bytes().next().unwrap();
        for (i, line) in shape.lines().enumerate() {
            for (col, c) in line.bytes().enumerate() {
                if c == shape_c {
                    column[col] += 1;
                }
            }
        }

        match shape_c {
            b'#' => locks.push(column),
            b'.' => keys.push(column),
            _ => panic!(),
        };
    }

    (locks, keys)
}
