use aoc::year2025::day01::*;

const INPUT: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

#[test]
fn part1_test() {
    assert_eq!(part1(&INPUT), 3.into());
}

#[test]
fn part2_test() {
    assert_eq!(part2(&INPUT), 6.into());
}
