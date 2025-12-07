use std::io::Error;

use crate::utils::{
    answers::Answer,
    parse::{ParseByte, ParseOps},
};

#[derive(Debug, Clone, Copy)]
enum Operation {
    Sum,
    Multiply,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "+" => Operation::Sum,
            "*" => Operation::Multiply,
            a => panic!("Invalid operation with \"{a:?}\""),
        }
    }
}

impl TryFrom<u8> for Operation {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'+' => Ok(Operation::Sum),
            b'*' => Ok(Operation::Multiply),
            a => Err(()),
        }
    }
}

pub fn part1(input: &str) -> Answer {
    let (numbers, operations) = parse(input);

    let mut result = 0;

    for (i, operation) in operations.iter().enumerate() {
        let mut curr = 0;
        if let Operation::Multiply = operation {
            curr = 1;
        }

        for j in 0..numbers.len() {
            let num = numbers[j][i];
            match operation {
                Operation::Sum => curr += num,
                Operation::Multiply => curr *= num,
            }
        }
        result += curr;
    }

    result.into()
}

pub fn part2(input: &str) -> Answer {
    let (numbers, operations) = parse_2(input);

    numbers
        .iter()
        .zip(operations)
        .map(|(nums, operation)| {
            let init = match operation {
                Operation::Multiply => 1,
                Operation::Sum => 0,
            };

            let f = match operation {
                Operation::Multiply => |a, b| a * b,
                Operation::Sum => |a, b| a + b,
            };

            nums.iter().fold(init, f)
        })
        .sum::<u64>()
        .into()
}

fn parse(input: &str) -> (Vec<Vec<u64>>, Vec<Operation>) {
    let lines = input.lines().collect::<Vec<_>>();
    let operations = lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(Operation::from)
        .collect();

    let numbers = lines[..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    (numbers, operations)
}

fn parse_2(input: &str) -> (Vec<Vec<u64>>, Vec<Operation>) {
    let lines = input
        .lines()
        .map(|line| line.bytes().collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let mut numbers = vec![];
    let mut operations = vec![];

    let max_len = lines.iter().map(|line| line.len()).max().unwrap();

    let mut curr = 0;
    for i in 0..max_len {
        let op_char = lines.last().unwrap().get(i);
        if let Some(&op_char) = op_char {
            let op = Operation::try_from(op_char).ok();
            if let Some(op) = op {
                operations.push(op);
                numbers.push(Vec::new());
            }
        }

        let last = numbers.last_mut();
        let Some(last) = last else {
            continue;
        };

        let mut curr = 0;

        let mut flag = false;
        for line in lines.iter().take(lines.len() - 1) {
            let digit = line.get(i);
            if let Some(b'0'..=b'9') = digit {
            } else {
                continue;
            }
            let digit = *digit.unwrap();
            flag = true;
            let digit = digit.to_decimal() as u64;
            curr = curr * 10 + digit;
        }
        if flag {
            last.push(curr);
        }
    }

    // println!("{:?}\n\n", numbers);
    // println!("{:?}\n\n", operations);

    (numbers, operations)
}
