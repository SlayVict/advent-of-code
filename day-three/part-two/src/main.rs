use std::fs;

use clap::Parser;
use regex::Regex;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

fn fold_lines(input: &str) -> String {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .join("")
}

fn main() {
    let args = Args::parse();
    let mul_regex = Regex::new(r"(?x)mul\((\d{0,4}),(\d{0,4})\)").unwrap();
    let dodont_regex = Regex::new(r"(?xU)(?:do\(\))(.+)(?:don't\(\))").unwrap();

    let input = fs::read_to_string(&args.input).unwrap();
    let input = fold_lines(&input);
    let input = format!("do(){input}don't()");

    let mut sum = 0;

    for (_, [do_part]) in dodont_regex.captures_iter(&input).map(|c| c.extract()) {
        for (_, [n1, n2]) in mul_regex.captures_iter(do_part).map(|c| c.extract()) {
            println!("mul({n1}, {n2})");
            sum += n1.parse::<u32>().unwrap() * n2.parse::<u32>().unwrap();
        }
    }
    println!("result: {sum}");
}
