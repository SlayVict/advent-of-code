use crate::utils::{answers::Answer, iters::ChunkOps, parse::*};
use std::cmp::Ordering::{self, *};

fn is_allowed_line(line: &[u32], after: &[[Ordering; 100]; 100]) -> bool {
    line.iter().enumerate().all(|(i, &page)| {
        line[i + 1..]
            .iter()
            .all(|&a| after[page as usize][a as usize] == Greater)
    })
}

pub fn part1(input: &str) -> Answer {
    let (after, updates) = parse(input);

    updates
        .iter()
        .filter(|&line| is_allowed_line(line, &after))
        .map(|line| line[line.len() / 2])
        .sum::<u32>()
        .into()
}

pub fn part2(input: &str) -> Answer {
    let (after, updates) = parse(input);

    updates
        .iter()
        .filter(|&line| !is_allowed_line(line, &after))
        .map(|line| {
            let mut l = line.clone();
            l.select_nth_unstable_by(line.len() / 2, |&from, &to| {
                after[from as usize][to as usize]
            });
            l[line.len() / 2]
        })
        .sum::<u32>()
        .into()
}

fn parse(input: &str) -> ([[Ordering; 100]; 100], Vec<Vec<u32>>) {
    let (edges_str, update_order_str) = input.split_once("\n\n").unwrap();

    let mut after = [[Greater; 100]; 100];

    edges_str
        .iter_unsigned::<u32>()
        .chunk::<2>()
        .for_each(|[l, r]| after[r as usize][l as usize] = Less);

    let updates: Vec<Vec<u32>> = update_order_str
        .lines()
        .map(|line| line.iter_unsigned::<u32>().collect())
        .collect();

    (after, updates)
}
