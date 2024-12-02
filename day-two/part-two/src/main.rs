use std::fs;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
    #[arg(long)]
    min_change: i32,
    #[arg(short, long)]
    max_change: i32,
}

#[derive(PartialEq)]
enum Trend {
    Increase,
    Decrease,
    Chaos,
}

fn get_trend(report: &[i32]) -> Trend {
    let diff = report.windows(2).fold(0, |acc, window| {
        if window[0] < window[1] {
            acc + 1
        } else if window[0] > window[1] {
            acc - 1
        } else {
            acc
        }
    });

    if diff > 0 {
        Trend::Increase
    } else if diff < 0 {
        Trend::Decrease
    } else {
        Trend::Chaos
    }
}

fn is_safe_report(report: &[i32], min_change: i32, max_change: i32, tolerance: i32) -> bool {
    let trend = get_trend(report);
    let mut anomalies_count = 0;

    let compare_levels = |level1: i32, level2: i32| {
        let compare_trend = match trend {
            Trend::Increase => level1 < level2,
            Trend::Decrease => level1 > level2,
            Trend::Chaos => false,
        };
        ((level1.abs_diff(level2) as i32) >= min_change
            || (level1.abs_diff(level2) as i32) <= max_change)
            && compare_trend
    };

    let mut i = 1;
    while i < report.len() {
        let regular = compare_levels(report[i - 1], report[i]);
        let anomaly = i < report.len() - 1 && compare_levels(report[i - 1], report[i + 1]);

        match (regular, anomaly) {
            (true, true) => {
                if !compare_levels(report[i], report[i + 1]) {
                    anomalies_count += 1;
                    // i += 1;
                }
            }
            (true, false) => (),
            (false, true) => {
                anomalies_count += 1;
                i += 1;
            }
            (false, false) => anomalies_count += 1,
        }

        i += 1;
    }

    if anomalies_count > tolerance {
        false
    } else {
        true
    }
}

fn main() {
    let args = Args::parse();
    let mut reports: Vec<Vec<i32>> = vec![];

    for line in fs::read_to_string(&args.input).unwrap().lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        reports.push(split.iter().map(|s| s.parse::<i32>().unwrap()).collect());
    }

    let result = reports
        .iter()
        .filter(|report| is_safe_report(report, args.min_change, args.max_change, 1))
        .count();

    println!("result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test76421safe() {
        let report = vec![7, 6, 4, 2, 1];
        assert!(is_safe_report(&report, 1, 3, 1));
    }
    #[test]
    fn test12789unsafe() {
        let report = vec![1, 2, 7, 8, 9];
        assert!(!is_safe_report(&report, 1, 3, 1));
    }
    #[test]
    fn test97621unsafe() {
        let report = vec![9, 7, 6, 2, 1];
        assert!(!is_safe_report(&report, 1, 3, 1));
    }
    #[test]
    fn test13245safe() {
        let report = vec![1, 3, 2, 4, 5];
        assert!(is_safe_report(&report, 1, 3, 1));
    }
    #[test]
    fn test86441safe() {
        let report = vec![8, 6, 4, 4, 1];
        assert!(is_safe_report(&report, 1, 3, 1));
    }
    #[test]
    fn test13679safe() {
        let report = vec![1, 3, 6, 7, 9];
        assert!(is_safe_report(&report, 1, 3, 1));
    }
    #[test]
    fn test93679safe() {
        let report = vec![9, 3, 6, 7, 9];
        assert!(is_safe_report(&report, 1, 3, 1));
    }
    #[test]
    fn test16421safe() {
        let report = vec![1, 6, 4, 2, 1];
        assert!(is_safe_report(&report, 1, 3, 1));
    }
}
