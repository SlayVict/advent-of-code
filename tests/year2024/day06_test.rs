use aoc::year2024::day06::*;

const INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

#[test]
fn part1_test() {
    assert_eq!(part1(&INPUT), 41.into());
}

#[test]
fn part2_test() {
    assert_eq!(part2(&INPUT), 6.into());
}

const SIMPLE_LOOP: &str = "\
.#...
....#
.....
.....
#^...
...#.";

#[test]
fn loop_detection_test() {
    let grid = parse(SIMPLE_LOOP);
    let looping = loop_detection(grid);
    assert!(looping);
}
