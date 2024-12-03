use std::fs;

use clap::Parser;
use regex::Regex;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();
    let re = Regex::new(r"(?x)mul\((\d{0,4}),(\d{0,4})\)").unwrap();

    let input = fs::read_to_string(&args.input).unwrap();

    let mut sum = 0;

    for (_, [n1, n2]) in re.captures_iter(&input).map(|c| c.extract()) {
        println!("mul({n1}, {n2})");
        sum += n1.parse::<u32>().unwrap() * n2.parse::<u32>().unwrap();
    }
    println!("result: {sum}");
}
