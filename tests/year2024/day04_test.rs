use aoc::year2024::day04::*;

const INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

#[test]
fn part1_test() {
    assert_eq!(part1(&INPUT), 18.into());
}

#[test]
fn part2_test() {
    assert_eq!(part2(&INPUT), 9.into());
}
