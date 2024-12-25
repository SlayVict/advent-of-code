use aoc::year2024::day22::*;

const INPUT: &str = "\
1
10
100
2024";

#[test]
fn part1_test() {
    assert_eq!(part1(&INPUT), 37327623.into());
}

#[test]
fn part2_test() {
    assert_eq!(part2(&INPUT), 24.into());
}
