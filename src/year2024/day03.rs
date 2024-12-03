use std::result;

use regex::Regex;

use crate::utils::answers::Answer;

pub fn part1(input: &str) -> Answer {
    let re = Regex::new(r"(?x)mul\((\d{0,4}),(\d{0,4})\)").unwrap();
    re.captures_iter(&input)
        .map(|c| c.extract())
        .map(|(_, [n1, n2])| n1.parse::<u32>().unwrap() * n2.parse::<u32>().unwrap())
        .sum::<u32>()
        .into()
}

pub fn part2(input: &str) -> Answer {
    let input = fold_lines(&input);
    let input = format!("do(){input}don't()");
    let mul_regex = Regex::new(r"(?x)mul\((\d{0,4}),(\d{0,4})\)").unwrap();
    let dodont_regex = Regex::new(r"(?xU)(?:do\(\))(.+)(?:don't\(\))").unwrap();

    let do_parts = dodont_regex
        .captures_iter(&input)
        .map(|c| c.extract())
        .map(|(_, [do_part])| do_part);

    do_parts
        .map(|part| {
            mul_regex
                .captures_iter(part)
                .map(|c| c.extract())
                .map(|(_, [n1, n2])| n1.parse::<u32>().unwrap() * n2.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sum::<u32>()
        .into()
}

fn fold_lines(input: &str) -> String {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .join("")
}
