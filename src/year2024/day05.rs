use std::{collections::HashMap, iter::empty};

use crate::utils::{answers::Answer, iters::ChunkOps, parse::*};

// type Graph = HashMap<u32, Vec<u32>>;
// type UpdateOrders = Vec<Vec<u32>>;

fn is_allowed_line(line: &[u32], after: &[[bool; 100]; 100]) -> bool {
    line.iter().enumerate().all(|(i, &page)| {
        line[i + 1..]
            .iter()
            .all(|&a| after[page as usize][a as usize])
    })
}

fn fix_line(line: &Vec<u32>, after: &[[bool; 100]; 100]) -> Vec<u32> {
    let mut todo: Vec<u32> = line.clone();
    loop {
        let Some(error_index) = todo.iter().enumerate().position(|(i, &page)| {
            !todo[i + 1..]
                .iter()
                .all(|&a| after[page as usize][a as usize])
        }) else {
            return todo;
        };

        let reason = error_index
            + 1
            + todo[error_index + 1..]
                .iter()
                .position(|&a| !after[todo[error_index] as usize][a as usize])
                .unwrap();
        todo.swap(error_index, reason);
    }
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
        .map(|line| fix_line(line, &after))
        .map(|line| line[line.len() / 2])
        .sum::<u32>()
        .into()
}

fn parse(input: &str) -> ([[bool; 100]; 100], Vec<Vec<u32>>) {
    let (edges_str, update_order_str) = input.split_once("\n\n").unwrap();

    let mut after = [[true; 100]; 100];

    edges_str
        .iter_unsigned::<u32>()
        .chunk::<2>()
        .for_each(|[l, r]| after[r as usize][l as usize] = false);

    let updates: Vec<Vec<u32>> = update_order_str
        .lines()
        .map(|line| line.iter_unsigned::<u32>().collect())
        .collect();

    (after, updates)
}
