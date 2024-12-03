use crate::utils::answers::Answer;

pub fn part1(input: &str) -> Answer {
    let (mut left, mut right) = parse(input);
    left.sort_unstable();
    right.sort_unstable();

    let result: u32 = left
        .into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| l.abs_diff(r))
        .sum();

    result.into()
}

pub fn part2(input: &str) -> Answer {
    let (left, right) = parse(input);

    let min = *right.iter().min().clone().unwrap();
    let max = *right.iter().max().clone().unwrap();

    let mut table = vec![0u32; (max - min + 1) as usize];

    for i in right.iter() {
        table[(*i - min) as usize] += 1;
    }

    let result: u32 = left
        .iter()
        .filter(|&&l| l >= min && l <= max)
        .map(|&l| l * table[(l - min) as usize])
        .sum();

    result.into()
}

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in input.lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        left.push(split[0].parse().unwrap());
        right.push(split[1].parse().unwrap());
    }

    (left, right)
}
