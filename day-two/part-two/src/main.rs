use std::fs;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

fn delta(l: i32, r: i32) -> i32 {
    let delta = r - l;
    if (1..=3).contains(&delta.abs()) {
        delta.signum()
    } else {
        0
    }
}

fn score(levels: &[i32]) -> i32 {
    levels.windows(2).map(|w| delta(w[0], w[1])).sum()
}

fn is_safe(levels: &[i32]) -> bool {
    let score = score(levels);
    let size = levels.len() as i32;

    if score.abs() == size - 1 {
        return true;
    }

    for i in 0..size as usize {
        let mut score = score;
        if i > 0 {
            score -= delta(levels[i - 1], levels[i]);
        }
        if i < size as usize - 1 {
            score -= delta(levels[i], levels[i + 1]);
        }
        if i > 0 && i < size as usize - 1 {
            score += delta(levels[i - 1], levels[i + 1]);
        }
        if score.abs() == size - 2 {
            return true;
        }
    }

    false
}

fn main() {
    let args = Args::parse();
    let mut reports: Vec<Vec<i32>> = vec![];

    for line in fs::read_to_string(&args.input).unwrap().lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        reports.push(split.iter().map(|s| s.parse::<i32>().unwrap()).collect());
    }

    let result = reports.iter().filter(|report| is_safe(report)).count();

    println!("result: {result}");
}
