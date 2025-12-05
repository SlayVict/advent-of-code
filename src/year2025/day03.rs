use std::mem::replace;

use crate::utils::{answers::Answer, grid::Grid, parse::ParseByte, point::Point};

pub fn part1(input: &str) -> Answer {
    let input = parse(input);
    solve::<2>(&input).into()
}

pub fn part2(input: &str) -> Answer {
    let input = parse(input);
    solve::<12>(&input).into()
}

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn solve<const N: usize>(input: &[&str]) -> u64 {
    let mut batteries = [0; N];

    let mut result = 0;

    for &bank in input {
        let left = bank.len() - N;
        batteries.copy_from_slice(&bank.as_bytes()[left..]);

        for mut next in bank[..left].bytes().rev() {
            for battery in &mut batteries {
                if next < *battery {
                    break;
                }
                next = replace(battery, next);
            }
        }
        result += batteries
            .iter()
            .fold(0, |joltage, &b| 10 * joltage + (b - b'0') as u64)
    }

    result
}
