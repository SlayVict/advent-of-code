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

#[derive(PartialEq)]
enum Trend {
    Increase,
    Decrease,
}

fn get_trend(level1: u32, level2: u32) -> Trend {
    if level1 > level2 {
        Trend::Decrease
    } else {
        Trend::Increase
    }
}

fn _is_safe_helper(report: &[u32], min_change: u32, max_change: u32, skip: Option<usize>) -> bool {
    let new_report: Vec<u32> = if let Some(skip) = skip {
        report
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != skip)
            .map(|(_, &x)| x)
            .collect()
    } else {
        report.into()
    };

    let trend = get_trend(new_report[0], new_report[1]);

    for i in 1..new_report.len() {
        if !((min_change..=max_change).contains(&new_report[i - 1].abs_diff(new_report[i]))
            && trend == get_trend(new_report[i - 1], new_report[i]))
        {
            if let Some(_) = skip {
                return false;
            }
            return _is_safe_helper(report, min_change, max_change, Some(i.saturating_sub(2)))
                || _is_safe_helper(report, min_change, max_change, Some(i.saturating_sub(1)))
                || _is_safe_helper(report, min_change, max_change, Some(i));
        }
    }
    true
}

fn is_safe_report(report: &[u32], min_change: u32, max_change: u32) -> bool {
    _is_safe_helper(report, min_change, max_change, None)
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
