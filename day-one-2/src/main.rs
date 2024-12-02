use std::fs;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();

    for line in fs::read_to_string(&args.input).unwrap().lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        left.push(split[0].parse().unwrap());
        right.push(split[1].parse().unwrap());
    }

    let min = *right.iter().min().clone().unwrap();
    let max = *right.iter().max().clone().unwrap();

    let mut table = vec![0usize; max - min + 1];

    for i in right.iter() {
        table[*i - min] += 1;
    }

    let result: usize = left
        .iter()
        .filter(|l| **l >= min && **l <= max)
        .map(|l| *l * table[*l - min])
        .sum();

    println!("result: {result}");
}
