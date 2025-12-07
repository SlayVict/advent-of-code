use crate::utils::{answers::Answer, parse::ParseOps};

type Range = [u64; 2];

trait Contains {
    fn contains(&self, value: u64) -> bool;
}

impl Contains for Range {
    fn contains(&self, value: u64) -> bool {
        self[0] <= value && value <= self[1]
    }
}

pub fn part1(input: &str) -> Answer {
    let (ranges, ids) = parse(input);

    let mut fresh = 0;
    for id in ids {
        let in_range = ranges.iter().any(|range| range.contains(id));

        fresh += (in_range) as u64;
    }

    fresh.into()
}

pub fn part2(input: &str) -> Answer {
    let (ranges, ids) = parse(input);

    ranges
        .iter()
        .map(|range| range[1] - range[0] + 1)
        .sum::<u64>()
        .into()
}

fn parse(input: &str) -> (Vec<Range>, Vec<u64>) {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();

    let ranges = prefix
        .lines()
        .map(|map| {
            let (start, end) = map.split_once('-').unwrap();
            // println!("({start}-{end})");
            [start.parse().unwrap(), end.parse().unwrap()]
        })
        .collect();

    let ids = suffix.iter_unsigned().collect();

    (merge_ranges(ranges), ids)
}

fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    ranges.sort_by_key(|range| range[0]);

    let mut result = vec![ranges[0]];

    for i in 1..ranges.len() {
        let last = result.last_mut().unwrap();
        let curr = ranges[i];
        if curr[0] <= last[1] {
            last[1] = last[1].max(curr[1]);
        } else {
            result.push(curr);
        }
    }

    // for range in ranges {
    //     println!("{:?}", range);
    // }

    result
}
