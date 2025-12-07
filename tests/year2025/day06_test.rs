use aoc::year2025::day06::*;

const INPUT: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +";

#[test]
fn part1_test() {
    assert_eq!(part1(&INPUT), 4277556.into());
}

#[test]
fn part2_test() {
    assert_eq!(part2(&INPUT), 3263827.into());
}
