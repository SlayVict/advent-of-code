use aoc::year2025::day03::*;

const INPUT: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

#[test]
fn part1_test() {
    assert_eq!(part1(&INPUT), 357.into());
}

#[test]
fn part2_test() {
    assert_eq!(part2(&INPUT), 3121910778619u64.into());
}
