use aoc::year2024::day01::*;

const INPUT: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

#[test]
fn part1_test() {
    assert_eq!(part1(&INPUT), 11.into());
}

#[test]
fn part2_test() {
    assert_eq!(part2(&INPUT), 31.into());
}
