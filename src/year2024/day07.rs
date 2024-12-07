use crate::utils::{answers::Answer, parse::ParseOps};
use rayon::prelude::*;

fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| {
            let nums: Vec<_> = line.iter_signed::<i64>().collect();
            (nums[0], nums[1..].into())
        })
        .collect()
}

pub fn part1(input: &str) -> Answer {
    fn is_possible(target: i64, nums: &[i64]) -> bool {
        match nums.len() {
            0 => false,
            1 => nums[0] == target,
            2.. => {
                let last = nums[nums.len() - 1];
                let rest = &nums[..nums.len() - 1];

                if target % last == 0 && is_possible(target / last, rest) {
                    return true;
                }

                if target > last && is_possible(target - last, rest) {
                    return true;
                }

                false
            }
        }
    }

    parse(input)
        .par_iter()
        .filter(|&(des, nums)| is_possible(*des as i64, nums))
        .map(|(des, _)| des)
        .sum::<i64>()
        .into()
}

pub fn part2(input: &str) -> Answer {
    fn is_possible(target: i64, nums: &[i64]) -> bool {
        match nums.len() {
            0 => false,
            1 => nums[0] == target,
            2.. => {
                let last = nums[nums.len() - 1];
                let rest = &nums[..nums.len() - 1];

                let round = (1..)
                    .map(|i| (1..=i).map(|_| 10).product())
                    .filter(|r| last / r == 0)
                    .next();

                if round.is_some()
                    && target % round.unwrap() == last
                    && is_possible(target / round.unwrap(), rest)
                {
                    return true;
                }

                if target % last == 0 && is_possible(target / last, rest) {
                    return true;
                }

                if target > last && is_possible(target - last, rest) {
                    return true;
                }

                false
            }
        }
    }

    parse(input)
        .par_iter()
        .filter(|&(des, nums)| is_possible(*des as i64, nums))
        .map(|(des, _)| des)
        .sum::<i64>()
        .into()
}
