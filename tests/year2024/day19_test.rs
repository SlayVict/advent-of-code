use aoc::year2024::day19::*;

const INPUT: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

#[test]
fn part1_test() {
    assert_eq!(part1(&INPUT), 6.into());
}

#[test]
fn part2_test() {
    assert_eq!(part2(&INPUT), 16.into());
}
