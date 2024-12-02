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
    Chaos,
}

fn get_trend(arr: &[u32]) -> Trend {
    let mut trend: Option<Trend> = None;
    for i in 1..arr.len() {
        if arr[i] > arr[i - 1] {
            if trend == Some(Trend::Decrease) {
                return Trend::Chaos;
            }
            trend = Some(Trend::Increase);
        } else if arr[i] < arr[i - 1] {
            if trend == Some(Trend::Increase) {
                return Trend::Chaos;
            }
            trend = Some(Trend::Decrease);
        }
    }
    trend.unwrap()
}

fn is_safe_report(report: &[u32], min_cahnge: u32, max_change: u32) -> bool {
    let trend = get_trend(report);
    if trend == Trend::Chaos {
        return false;
    }
    for i in 1..report.len() {
        if report[i].abs_diff(report[i - 1]) > max_change
            || report[i].abs_diff(report[i - 1]) < min_cahnge
        {
            return false;
        }
    }
    true
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
