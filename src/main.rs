#![feature(duration_millis_float)]
use aoc::utils::parse::*;
use aoc::*;
use std::time::{Duration, Instant};
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

            let instant = Instant::now();
            let part1 = part1(&data);
            let part1time = instant.elapsed();
            let instant = Instant::now();
            let part2 = part2(&data);
            let part2time = instant.elapsed();
            // let part2 = 0.into();

            (part1, part2, part1time, part2time)
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
        solution!(year2025, day01),
        solution!(year2025, day02),
        solution!(year2025, day03),
        solution!(year2025, day04),
        // solution!(year2025, day05),
    ];

    for Solution {
        year,
        day,
        path,
        wrapper,
    } in solutions
    {
        let data = read_to_string(path).unwrap();

        let (part1, part2, part1time, part2time) = wrapper(data);
        println!("{}/{}", year, day);
        println!("part1: {} ({}ms)", part1, part1time.as_millis_f32());
        println!("part2: {} ({}ms)", part2, part2time.as_millis_f32());
        println!();
    }
}

struct Solution {
    year: u32,
    day: u32,
    path: PathBuf,
    wrapper: fn(String) -> (Answer, Answer, Duration, Duration),
}
