use aoc::year2024::day09::*;

const INPUT: &str = "2333133121414131402";
const INPUT2: &str = "23331331214141314020";

#[test]
fn part1_test() {
    assert_eq!(part1(&INPUT), 1928.into());
}
#[test]
fn part1_test2() {
    assert_eq!(part1(&INPUT2), 1928.into());
}

#[test]
fn part2_test() {
    assert_eq!(part2(&INPUT), 2858.into());
}
