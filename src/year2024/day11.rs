use std::collections::HashMap;

use crate::utils::{answers::Answer, parse::ParseOps};

fn count(stone: u128, blinks: u128, cache: &mut HashMap<(u128, u128), u128>) -> u128 {
    if blinks == 0 {
        return 1;
    }
    let key = (stone, blinks);
    if let Some(&val) = cache.get(&key) {
        return val;
    }

    let next = if stone == 0 {
        count(1, blinks - 1, cache)
    } else {
        let digits = stone.ilog10() + 1;
        if digits % 2 == 0 {
            let power = 10_u128.pow(digits / 2);
            count(stone / power, blinks - 1, cache) + count(stone % power, blinks - 1, cache)
        } else {
            count(stone * 2024, blinks - 1, cache)
        }
    };

    cache.insert(key, next);
    next
}

pub fn part1(input: &str) -> Answer {
    let stones = parse(input);
    let mut map = HashMap::new();

    stones
        .iter()
        .map(|stone| count(*stone, 25, &mut map))
        .sum::<u128>()
        .to_string()
        .into()
}

pub fn part2(input: &str) -> Answer {
    let stones = parse(input);
    let mut map = HashMap::new();

    stones
        .iter()
        .map(|stone| count(*stone, 75, &mut map))
        .sum::<u128>()
        .to_string()
        .into()
}

fn parse(input: &str) -> Vec<u128> {
    input.iter_unsigned::<u128>().collect()
}
