use aoc::year2024::day10::*;

const INPUT: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

#[test]
fn part1_test() {
    assert_eq!(part1(&INPUT), 36.into());
}

#[test]
fn part2_test() {
    assert_eq!(part2(&INPUT), 81.into());
}
