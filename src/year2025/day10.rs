use microlp::{LinearExpr, OptimizationDirection, Problem};
use std::usize;

use crate::utils::{answers::Answer, parse::ParseOps};

#[derive(Debug)]
struct Machine {
    leds: usize,
    btns: Vec<usize>,
    jolts: Vec<usize>,
}

pub fn part1(input: &str) -> Answer {
    let machines = parse(input);

    machines
        .iter()
        .map(|machine| {
            (0..2usize.pow(machine.btns.len() as _)).fold(usize::MAX, |min, mask| {
                let n = mask.count_ones() as usize;
                if n < min
                    && machine
                        .btns
                        .iter()
                        .enumerate()
                        .filter(|(ix, _)| mask & (1 << ix) != 0)
                        .fold(machine.leds, |acc, (_, &b)| acc ^ b)
                        == 0
                {
                    min.min(n)
                } else {
                    min
                }
            })
        })
        .sum::<usize>()
        .into()

    // Answer::InProgress
}

pub fn part2(input: &str) -> Answer {
    let machines = parse(input);

    let a: usize = machines
        .iter()
        .map(|machine| {
            let mut problem = Problem::new(OptimizationDirection::Minimize);
            let max = machine.jolts.iter().copied().max().unwrap();
            let vars = (0..machine.btns.len())
                .map(|_| problem.add_integer_var(1.0, (0, max as i32)))
                .collect::<Vec<_>>();
            for (i, &n) in machine.jolts.iter().enumerate() {
                problem.add_constraint(
                    machine
                        .btns
                        .iter()
                        .zip(&vars)
                        .filter(|&(mask, _)| mask & (1 << i) != 0)
                        .fold(LinearExpr::empty(), |mut ex, (_, &var)| {
                            ex.add(var, 1.0);
                            ex
                        }),
                    microlp::ComparisonOp::Eq,
                    n as f64,
                );
            }
            problem.solve().unwrap().objective().round() as usize
        })
        .sum();

    a.into()
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            let leds = parts[0]
                .bytes()
                .skip(1)
                .rev()
                .skip(1)
                .fold(0, |acc, c| (acc << 1) | (c == b'#') as usize);
            let joltage = parts.last().unwrap().iter_unsigned().collect();
            let buttons = parts
                .iter()
                .skip(1)
                .filter(|s| s.starts_with('('))
                .map(|s| s.iter_unsigned::<usize>().fold(0, |acc, n| acc | (1 << n)))
                .collect();
            Machine {
                leds,
                btns: buttons,
                jolts: joltage,
            }
        })
        .collect()
}
