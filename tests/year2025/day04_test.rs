use aoc::year2025::day04::*;

const INPUT: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

#[test]
fn part1_test() {
    assert_eq!(part1(&INPUT), 13.into());
}

#[test]
fn part2_test() {
    assert_eq!(part2(&INPUT), 43.into());
}
