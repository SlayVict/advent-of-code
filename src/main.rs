use aoc::utils::parse::*;
use aoc::*;
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};
use utils::answers::Answer;

macro_rules! solution {
    ($year:tt, $day:tt) => {{
        let year = stringify!($year);
        let day = stringify!($day);
        let path = Path::new("input")
            .join(year)
            .join(day)
            .with_extension("txt");

        let wrapper = |data: String| {
            use $year::$day::*;

            let part1 = part1(&data);
            let part2 = part2(&data);

            (part1, part2)
        };

        Solution {
            year: year.unsigned(),
            day: day.unsigned(),
            path,
            wrapper,
        }
    }};
}

fn main() {
    let solutions = vec![
        solution!(year2024, day01),
        solution!(year2024, day02),
        solution!(year2024, day03),
    ];

    for Solution {
        year,
        day,
        path,
        wrapper,
    } in solutions
    {
        let data = read_to_string(path).unwrap();

        let (part1, part2) = wrapper(data);
        println!("{}/{}", year, day);
        println!("part1: {}", part1);
        println!("part2: {}", part2);
        println!();
    }
}

struct Solution {
    year: u32,
    day: u32,
    path: PathBuf,
    wrapper: fn(String) -> (Answer, Answer),
}
