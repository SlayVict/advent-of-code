use aoc::year2025::day05::*;

const INPUT: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

#[test]
fn part1_test() {
    assert_eq!(part1(&INPUT), 3.into());
}

#[test]
fn part2_test() {
    assert_eq!(part2(&INPUT), 14.into());
}
