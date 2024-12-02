use std::fs;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
    #[arg(long)]
    min_change: u32,
    #[arg(short, long)]
    max_change: u32,
}

fn is_safe_report(report: &[u32], min_change: u32, max_change: u32) -> bool {
    let mut up = true;
    let mut down = true;
    let mut range = true;

    for w in report.windows(2) {
        up &= w[1] > w[0];
        down &= w[1] < w[0];
        range &= (min_change..=max_change).contains(&w[1].abs_diff(w[0]));
    }

    (up ^ down) && range
}

fn main() {
    let args = Args::parse();
    let mut reports: Vec<Vec<u32>> = vec![];

    for line in fs::read_to_string(&args.input).unwrap().lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        reports.push(split.iter().map(|s| s.parse::<u32>().unwrap()).collect());
    }

    let result = reports
        .iter()
        .filter(|report| is_safe_report(report, args.min_change, args.max_change))
        .count();

    println!("result: {result}");
}
